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
//! v0→v1 shape change). With **no** default, the branch folds to a byte no-op for every
//! leaf whose absence the conformance gate already accepts (M46 Inc-4 T1 — the shared
//! [`crate::validate::is_author_required`] predicate: `optional:`, an optional `ref`, a
//! pack-declared type, a `set:`-derived field). The caller-supplied deriver value the
//! CLI threads in (`with_stamp_default`) remains the extension point, arriving as a
//! schema `default`; only the *nobody-threaded-one-in* behaviour changed, from a block
//! no author could clear into the conformant absence it already was.
//!
//! M36 Inc-3 T1 builds the **`added-optional-section` branch**: a wholly-new *optional*
//! slot section is minted **empty** at its schema-ordered home via [`crate::write::generate_section`]
//! (the same block-insert the `prose-needing` slot arm uses), but an empty *optional* slot
//! **conforms** — so this is the migration's final byte-stable form, not a mint-then-author
//! handoff. The empty-slot section canonicalizes to exactly the v2 writer shape (proven by
//! the byte-stability test).
//!
//! One branch stays deferred, surfaced as [`TransformError::Unsupported`] rather than
//! silently skipped — an un-built branch must block, never drop a change: the
//! `prose-needing` **field** sub-case (a new required *field*, not a slot — the transform
//! mints slots, not fields).
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
use crate::finding::Finding;
use crate::parse::parse_sections;
use crate::schema::{Leaf, Schema, SectionBody};
use crate::schema_diff::SchemaChange;
use crate::validate::schema_conformance;
use crate::write::{self, GenerateError, SpliceError};
use std::collections::BTreeMap;

/// The wire name [`TransformError::Unsupported`] carries for a **value remap** whose authored
/// old→new map does not cover a committed value — the one refusal whose repair is a *migration
/// input* (the map the CLI threads in) rather than a transform arm, which is why the CLI routes
/// it differently from its siblings (M46 Inc-4 T3). Named at its construction site so that route
/// cannot drift off the kind it claims to match.
pub const VALUE_REMAP_KIND: &str = "value-remapped";

/// A failure applying a classified diff to one instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransformError {
    /// A structural splice primitive failed (an absent/non-conformant target, an
    /// unslug-able promotion title, …). Carries the underlying [`GenerateError`].
    Generate(GenerateError),
    /// A surgical-splice primitive failed — the `display-title-changed` H1 rewrite over a
    /// doc with no locatable `# H1`. Carries the underlying [`SpliceError`].
    Splice(SpliceError),
    /// A classified change kind the driver **will not apply** — either because its branch is
    /// **not built** (the `prose-needing` **field** sub-case, a new required field: T4 mints
    /// only slots) or because it is **refused by design** (a
    /// [`SchemaChange::NarrowedCardinality`] — content-affecting, and the recorded M42 pick is to
    /// refuse rather than restamp unchecked; a [`SchemaChange::RemovedField`] — the recorded M42
    /// pick is to refuse rather than strip committed values away). Surfaced, never silently
    /// skipped, so neither an un-built branch nor a refused kind can drop a change.
    Unsupported {
        /// The classified kind's wire name.
        kind: &'static str,
        /// The section the change concerns.
        section: String,
    },
    /// The **empty-diff backstop** fired: the schema pair moved its conformance-relevant
    /// structural projection but classified **no transform kind**
    /// ([`SchemaChange::Unclassified`]). There are no bytes to fold — the repair is to
    /// *build the transform kind*, not to migrate — so the driver refuses rather than
    /// folding the doc to a silent no-op-then-restamp (`design/corpus-migration.md` → The
    /// empty-diff backstop). The CLI refuses earlier, at classification; this arm is the
    /// engine-side guarantee that an unclassified change can never be silently ignored.
    Unclassified,
}

impl From<GenerateError> for TransformError {
    fn from(err: GenerateError) -> Self {
        TransformError::Generate(err)
    }
}

impl From<SpliceError> for TransformError {
    fn from(err: SpliceError) -> Self {
        TransformError::Splice(err)
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
            SchemaChange::NarrowedCardinality { section, .. } => {
                // THE NARROWING PICK — refuse (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T3;
                // `corpus-migration.md` → The two silent-classification holes). A narrowing is
                // **content-affecting**: a committed instance may carry more values than the new
                // bound admits (or lack one it now demands), and nothing in the system counts a
                // doc's edges against a `card` bound — so the design's other arm (validate every
                // committed instance against the new bound) is net-new validation surface. The
                // pre-M42 behaviour folded a narrowing to **zero bytes** and restamped the corpus
                // past the gate, silently; refusing is strictly better, and leaves the validating
                // arm's door open.
                return Err(TransformError::Unsupported {
                    kind: "narrowed-cardinality",
                    section: section.clone(),
                });
            }
            SchemaChange::RemovedField { section, .. } => {
                // THE REMOVAL PICK — refuse, not strip (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T5;
                // `corpus-migration.md` → The two silent-classification holes, which left the
                // shape open). A strip arm would splice the committed field line away
                // deterministically — and **destroy the committed values**, a knowing exception to
                // **No-data-loss**, a declared property of this pair. No frozen doctype needs a
                // removal, so the strip is premature generality; the *silent* hole is the actual
                // defect, and refusing closes it at zero risk. Additive to build later if a real
                // driver appears.
                return Err(TransformError::Unsupported {
                    kind: "removed-field",
                    section: section.clone(),
                });
            }
            SchemaChange::OptionalRelaxed { .. } => {
                // Instance-byte identity: a leaf relaxed to `optional: true` admits every doc the
                // strict rule admitted — nothing can have become non-conformant, so there is
                // nothing to splice (the widened-cardinality sibling; `corpus-migration.md` → The
                // structural projection: the two flag deltas get named kinds). The kind exists so
                // the change *names itself* to the backstop instead of being refused.
            }
            SchemaChange::EnumWidened { .. } => {
                // Instance-byte identity: `new.of ⊇ old.of`, so every committed value is still a
                // declared member — nothing to remap, no authored map needed (the
                // widened-cardinality sibling; `corpus-migration.md` → the `EnumWidened` kind).
            }
            SchemaChange::AddedOptionalSection { section } => {
                // Splice the empty `## Heading` slot-section at its schema-ordered home —
                // the same block-insert path the required-slot `ProseNeeding` arm uses,
                // but an **optional** empty slot *conforms* instead of blocking, so this
                // is the migration's final form (not a mint-then-author handoff). The
                // v2 writer emits the heading unconditionally, so a historical doc lacking
                // it is non-canonical; the empty-slot section canonicalizes to exactly the
                // v2 writer shape (byte-stable — `render(parse(out)) == out`; proven by the
                // byte-stability test), leaving every prior section's bytes untouched.
                out = write::generate_section(new_schema, &out, section, Some(""), &[])?;
            }
            SchemaChange::AddedRepeatableSection { section } => {
                // Mint the empty `## Heading` at its schema-ordered home through the **same**
                // block-insert the added-optional-section arm uses — the section body's shape
                // (repeatable vs simple) is not the *insert's* concern: with no slot prose and
                // no fields, the generated block is exactly the v2 writer's canonical form for a
                // **zero-item** repeatable (`render_section`'s `Repeatable` arm over an empty
                // item list). A zero-item repeatable **conforms** (`repeatable-populated` is a
                // store advisory, not a conformance break), so this is the migration's final
                // byte-stable form — not a mint-then-author handoff, and the CLI invents no items
                // (`corpus-migration.md` → The classifier's holes: `AddedRepeatableSection`).
                out = write::generate_section(new_schema, &out, section, None, &[])?;
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
            SchemaChange::AddedItemField { section, field } => {
                out = apply_added_item_field(new_schema, &out, section, field)?;
            }
            SchemaChange::ValueRemapped {
                section,
                field,
                map,
            } => {
                // The first **parameterized** transform kind: remap each committed value of
                // the enum `field` through the authored old→new `map` (an enum rename is
                // unrecoverable from the schema pair, so the CLI supplies the map — the
                // determinism boundary holds: a deterministic input, not an LLM call). A
                // committed value the map does not cover blocks loudly (`Unsupported`),
                // never a silent no-op (`corpus-migration.md` → the value-remap kind).
                out = apply_value_remap(new_schema, &out, section, field, map)?;
            }
            SchemaChange::Relocated { .. } => {
                // A file move, not a content edit: the instance bytes are byte-identical
                // at the new home, so the in-content fold is a **no-op** (the
                // widened-cardinality sibling). The git-free `fs::rename` + file-state
                // re-key are the CLI migrate-corpus move-arm's concern (T2), applied
                // outside this per-doc content transform (`corpus-migration.md` →
                // Relocation).
            }
            SchemaChange::PresentationOnly => {
                // A delta **outside** the conformance-relevant projection (a hint reword, a
                // `default:`/`set:`/`inverse:`/`check:` edit): a committed doc's bytes cannot
                // violate it, so there is nothing to splice — the instance folds **byte-
                // identical** and the doc restamps at the new version (the widened-cardinality
                // sibling: classify, then fold to zero bytes; `corpus-migration.md` → The
                // structural projection).
            }
            SchemaChange::Unclassified => {
                // The backstop: the projection moved with no kind to apply. There are no
                // deterministic bytes to write, and folding it as a no-op would restamp the
                // doc at a version it does not conform to — the silent strand. Refuse.
                return Err(TransformError::Unclassified);
            }
            SchemaChange::DisplayTitleChanged { to } => {
                // Rewrite the `# H1` line to the new display text (`# changelog` →
                // `# Changelog`), byte-stable otherwise — every other byte preserved
                // (`corpus-migration.md` → Relocation: display-title-changed rewrites
                // only the H1). The CLI owns structure; the H1 text is deterministic
                // (the schema-declared display title), never authored.
                out = write::set_title(&out, to)?;
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
/// **With no `default`, placement keys on the same predicate the classifier does** —
/// [`crate::validate::is_author_required`], the conformance gate's own absent-field rule (M46
/// Inc-4 T1). A leaf the gate never asks for (`optional:`, an optional `ref`, a pack-declared
/// type, or a `set:`-derived field) is a byte **no-op**: its absence *already conforms*, so there
/// is nothing deterministic to place and inventing a value would fabricate one. Only an
/// author-required leaf reaching here is [`TransformError::Unsupported`] — and the classifier
/// routes those to `ProseNeeding`, so the arm is the driver's belt-and-braces, never the corpus's
/// dead end.
///
/// **The caller-threaded value stays the extension point — nothing about it is retired.** The
/// CLI's `with_stamp_default` (and `authored_remap`'s sibling shape) hands the deriver's value in
/// **as a schema `default`**, so the stamp still splices through the `Some` arm above. What
/// changed is only the behaviour when **nobody threads a value in**: a `set:`-derived field is
/// then left absent — conformantly — instead of blocking the doc with a route no author can
/// follow. The migration report names what it left unfilled (M46 Inc-4 T2), so the no-op is
/// **quiet, not silent**.
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
        // No deterministic value to place. An absence the conformance gate accepts is a byte
        // no-op — the one predicate the classifier keys on, so the two cannot drift.
        None if !crate::validate::is_author_required(decl) => return Ok(source.to_string()),
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

/// Splice an `added-item-field` change: place the new leaf `field` of the **repeatable** section
/// `section` on every committed item, carrying its **deterministic** value.
///
/// The splice runs through [`write::set_item_field_or_insert`] — the **insert-capable** primitive,
/// which generates the absent `- key: value` bullet (and adjudicates the value against its declared
/// type before touching bytes). [`write::set_item_field`] is *update-only*: it refuses an absent
/// bullet with `SpliceError::NotPresent`, i.e. on **every item, by definition of this kind** (it is
/// the right primitive for [`SchemaChange::ValueRemapped`], which overwrites an *existing* bullet).
///
/// An item that **already carries** the bullet is **skipped**, never overwritten: its value is
/// authored and conformant, so re-writing it would destroy committed data (**No-data-loss**) — and
/// the skip is what makes the fold **idempotent** (the T2 re-run lesson, at the item locus; a
/// whole-change filter in the CLI cannot express it, because one doc can hold items of both kinds).
///
/// **With no `default`, placement keys on [`crate::validate::is_author_required`]**, exactly as
/// [`apply_added_field`] does at the simple locus (M46 Inc-4 T1): a leaf the conformance gate
/// never asks for — `optional:`, an optional `ref`, a pack-declared type, or a `set:`-derived
/// leaf — is a byte **no-op**, because an item lacking the bullet *already conforms* and
/// inventing one would fabricate a value. The caller-threaded deriver value (`with_stamp_default`)
/// is **untouched as an extension point**: it arrives as a schema `default` and still splices
/// through the `Some` arm. Only an author-required leaf is [`TransformError::Unsupported`], and
/// the classifier routes those to `ProseNeeding` before the driver ever sees them.
fn apply_added_item_field(
    new_schema: &Schema,
    source: &str,
    section: &str,
    field: &str,
) -> Result<String, TransformError> {
    let unsupported = || TransformError::Unsupported {
        kind: "added-item-field",
        section: section.to_string(),
    };
    let sec = new_schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .ok_or_else(unsupported)?;
    let SectionBody::Repeatable { repeatable } = &sec.body else {
        return Err(unsupported());
    };
    let decl = repeatable
        .block
        .iter()
        .find_map(|leaf| match leaf {
            Leaf::Field(f) if f.id == field => Some(f.as_ref()),
            _ => None,
        })
        .ok_or_else(unsupported)?;

    let value = match &decl.default {
        Some(default) => default.clone(),
        // The simple locus's rule, verbatim: an absence the conformance gate accepts writes no
        // bytes on any item.
        None if !crate::validate::is_author_required(decl) => return Ok(source.to_string()),
        None => return Err(unsupported()),
    };

    // Collect the target item ids from the initial parse, then splice each in turn. Item ids are
    // stable under a field insert (the id-from leaf is untouched), so a fresh
    // `set_item_field_or_insert` re-locates each item after the prior splice.
    let doc = parse_sections(new_schema, source).map_err(|_| unsupported())?;
    let Some(parsed) = doc.sections.iter().find(|s| s.id == section) else {
        // The instance omits the section — no items, nothing to place.
        return Ok(source.to_string());
    };
    let targets: Vec<String> = parsed
        .items
        .iter()
        .filter(|item| !item.fields.iter().any(|f| f.key == field))
        .map(|item| item.id.clone())
        .collect();

    let mut out = source.to_string();
    for item_id in targets {
        out = write::set_item_field_or_insert(new_schema, &out, section, &item_id, field, &value)?;
    }
    Ok(out)
}

/// Apply a `value-remapped` change: remap every committed value of the enum `field` in
/// `section` through the authored old→new `map`, byte-stably.
///
/// The section may be **simple** (one field value, spliced via [`write::set_field`]) or
/// **repeatable** (one value per item, spliced via [`write::set_item_field`] — the
/// present-field value-span splice). Parse is structural (it reads the field bullet
/// regardless of enum membership), so the doc re-parses cleanly under the v2 schema even
/// while it still carries v1 member values (`corpus-migration.md` → the value-remap kind).
///
/// A committed value the `map` does not cover — or a list-valued enum (out of the narrow
/// bound) — surfaces [`TransformError::Unsupported`]: an uncovered value blocks loudly,
/// never a silent no-op. A field/section the instance omits is a byte no-op (nothing to
/// remap).
fn apply_value_remap(
    schema: &Schema,
    source: &str,
    section: &str,
    field: &str,
    map: &BTreeMap<String, String>,
) -> Result<String, TransformError> {
    fn unsupported(section: &str) -> TransformError {
        TransformError::Unsupported {
            kind: VALUE_REMAP_KIND,
            section: section.to_string(),
        }
    }
    let sec = schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .ok_or_else(|| unsupported(section))?;

    let doc = parse_sections(schema, source).map_err(|_| unsupported(section))?;
    let Some(parsed) = doc.sections.iter().find(|s| s.id == section) else {
        // The instance omits the section — nothing to remap.
        return Ok(source.to_string());
    };

    match &sec.body {
        SectionBody::Simple { .. } => {
            let Some(f) = parsed.fields.iter().find(|f| f.key == field) else {
                // The field is absent in this instance — nothing to remap.
                return Ok(source.to_string());
            };
            let new_value = remap_value(&f.value, map).ok_or_else(|| unsupported(section))?;
            Ok(write::set_field(
                schema, source, section, field, &new_value,
            )?)
        }
        SectionBody::Repeatable { .. } => {
            // Collect (item id, remapped value) from the initial parse, then splice each via
            // the present-field item write path. Item ids are stable under a value-span
            // splice (the enum field is never the id-from — an enum id-from item is
            // reslug-refused), so a fresh `set_item_field` locates each item after the prior
            // splice.
            let mut edits: Vec<(String, String)> = Vec::new();
            for item in &parsed.items {
                if let Some(f) = item.fields.iter().find(|f| f.key == field) {
                    let new_value =
                        remap_value(&f.value, map).ok_or_else(|| unsupported(section))?;
                    edits.push((item.id.clone(), new_value));
                }
            }
            let mut out = source.to_string();
            for (item_id, new_value) in edits {
                out = write::set_item_field(schema, &out, section, &item_id, field, &new_value)?;
            }
            Ok(out)
        }
    }
}

/// Look up a committed scalar value in the authored old→new `map`. An **uncovered** value
/// — or a non-scalar (list-valued) enum, out of the narrow bound — yields `None`, which the
/// driver surfaces as [`TransformError::Unsupported`] (blocks loudly, never a silent
/// no-op).
fn remap_value(value: &Value, map: &BTreeMap<String, String>) -> Option<String> {
    match value {
        Value::Scalar(v) => map.get(v).cloned(),
        Value::List(_) => None,
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
    /// **Why** that doc could not commit — `Some` exactly when [`Self::halted_at`] is, and the
    /// reason a caller renders instead of inventing one (M46 Inc-4 T3). The fold has three
    /// distinct ways to refuse a doc and they need three different repairs; collapsing them into
    /// one outcome is what let `migrate-corpus` print *"author the prose, then re-run"* over a
    /// doc no prose and no re-run could clear.
    pub halt_reason: Option<HaltReason>,
}

/// Why [`migrate_corpus`] could not commit one doc — the halt's own cause, carried out to the
/// caller so the refusal it prints names the repair that exists (M46 Inc-4 T3;
/// `design/command-output-contract.md` → the `migrate-corpus.*` sub-table).
///
/// The three variants are the three points [`try_migrate_doc`] can fail at, in order. Only the
/// last is a break **in the doc** that an author can clear from the doc; the other two are
/// **schema-authoring** gaps — the transform arm, or the migration input it was handed — where
/// nothing written into this doc, and no number of re-runs, changes the outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HaltReason {
    /// The transform driver refused the change or a splice primitive failed: no v2 bytes were
    /// produced at all.
    Transform(TransformError),
    /// The folded bytes do **not parse** under the new schema — carries the failed parse's own
    /// findings, which are what say where and why the buffer broke.
    Parse(Vec<Finding>),
    /// The folded bytes parse but do **not conform**: the per-doc conformance gate's own
    /// findings, verbatim. The Framing-A handoff lives here — a required slot the fold minted
    /// empty is one of the breaks this set can carry, and so is one the doc already had.
    Gate(Vec<Finding>),
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
    let mut halt_reason: Option<HaltReason> = None;
    for (i, doc) in docs.iter().enumerate() {
        let migrated = match halted_at {
            // After the halt: never touched — byte-identical v0.
            Some(_) => None,
            None => match try_migrate_doc(doc) {
                Ok(v2) => Some(v2),
                // The first refusal halts the run, and its cause rides out with it — recorded
                // here, at the one site that has it in hand.
                Err(reason) => {
                    halted_at = Some(i);
                    halt_reason = Some(reason);
                    None
                }
            },
        };
        match migrated {
            Some(v2) => out.push(DocOutcome::Migrated {
                id: doc.id.to_string(),
                v2,
            }),
            None => out.push(DocOutcome::Untouched {
                id: doc.id.to_string(),
                v0: doc.source.to_string(),
            }),
        }
    }
    CorpusMigration {
        docs: out,
        halted_at,
        halt_reason,
    }
}

/// One doc's per-doc transaction: transform into a scratch buffer, then gate on
/// conformance against the v2 schema. Returns the committed v2 bytes on a clean gate, or the
/// [`HaltReason`] that stopped it (rollback — the doc stays v0).
///
/// The three failure points **stay distinguishable** (M46 Inc-4 T3). They used to collapse into
/// one `None`, and the caller had nothing left to render but its most common cause — so a doc
/// refused by an un-built transform arm was told to author prose and re-run, which could never
/// clear it.
fn try_migrate_doc(doc: &CorpusDoc<'_>) -> Result<String, HaltReason> {
    let scratch = transform(doc.old_schema, doc.new_schema, doc.source, doc.changes)
        .map_err(HaltReason::Transform)?;
    let parsed = parse_sections(doc.new_schema, &scratch).map_err(HaltReason::Parse)?;
    let findings = schema_conformance(doc.new_schema, &scratch, &parsed);
    if findings.is_empty() {
        Ok(scratch)
    } else {
        Err(HaltReason::Gate(findings))
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
    use crate::write::{Instance, ItemContent, SectionContent, instance_from_source, render};

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

    // ---- (f) the added-optional-section branch: an added `## Options` optional slot ----

    /// v1: a `dec` doctype with `context` + `consequences` prose slots — the adr shape
    /// before `options` (a leading + trailing slot, so the splice is on a *non-trailing*
    /// section, the byte-fragile case).
    fn dec_v1() -> Schema {
        load_schema(
            b"\
type: dec
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        )
        .expect("dec v1 loads")
    }

    /// v2: an **optional** `options` slot section is inserted between them — the
    /// added-optional-section change (the real adr v1→v2 bump, synthetically).
    fn dec_v2() -> Schema {
        load_schema(
            b"\
type: dec
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: options
    slot: { hint: \"options considered\", optional: true }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        )
        .expect("dec v2 loads")
    }

    /// A canonical v0-shaped `dec` (context + consequences filled), built through
    /// [`render`] so the input is the exact byte-stable form a first-touch-canonicalized
    /// corpus doc has.
    fn dec_v0_doc() -> String {
        let inst = Instance {
            title: "Rate-limit at the gateway".to_string(),
            sections: vec![
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
                    id: "consequences".to_string(),
                    slot: Some("Each service drops its local limiter.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&dec_v1(), &inst)
    }

    #[test]
    fn added_optional_section_canonicalizes_to_the_v2_writer_shape() {
        let v1 = dec_v1();
        let v2 = dec_v2();
        let src = dec_v0_doc();

        // The real classifier emits the change; the driver is exercised on the emitted
        // classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedOptionalSection {
                section: "options".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("added-optional-section transform");

        // The empty `## Options` heading is spliced in between the neighbours.
        assert!(
            out.contains("## Options"),
            "the optional section heading is spliced in; got {out:?}"
        );

        // The byte-stability done-criterion: the transform output is **exactly** the v2
        // writer's canonical shape — (a) `render(parse(out)) == out` under v2, and (c) it
        // conforms (an empty *optional* slot exempts the absent prose). This is the
        // load-bearing assertion the planner flagged: `generate_section(Some(""))` must
        // equal what the writer emits for an empty optional slot.
        assert_byte_stable(&v2, &out);
        assert_conforms(&v2, &out);

        // (b) every prior section value is byte-preserved verbatim; the minted section is
        // an empty slot.
        assert!(
            out.contains("## Context\n\nPer-client limits were enforced ad hoc.\n"),
            "the context block is byte-preserved; got {out:?}"
        );
        assert!(
            out.contains("## Consequences\n\nEach service drops its local limiter.\n"),
            "the consequences block is byte-preserved; got {out:?}"
        );
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let slot_of = |id: &str| {
            inst.sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(
            slot_of("context").as_deref(),
            Some("Per-client limits were enforced ad hoc.")
        );
        assert_eq!(
            slot_of("consequences").as_deref(),
            Some("Each service drops its local limiter.")
        );
        let options = inst
            .sections
            .iter()
            .find(|s| s.id == "options")
            .expect("options section present");
        assert!(
            options.slot.as_deref().unwrap_or("").is_empty(),
            "the minted optional slot is empty (conforms, no prose needed)"
        );

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(
            again, out,
            "added-optional-section transform is deterministic"
        );
    }

    // ---- (f2) the added-repeatable-section branch: a wholly-new repeatable mints its heading ----

    /// v1: a `plan` doctype with `context` + `outcome` prose slots (a leading + trailing
    /// slot, so the splice is on a *non-trailing* section — the byte-fragile case).
    fn plan_v1() -> Schema {
        load_schema(
            b"\
type: plan
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: outcome
    slot: { hint: \"the outcome\" }
",
        )
        .expect("plan v1 loads")
    }

    /// v2: a wholly-new **repeatable** `tasks` section is inserted between them — the
    /// `AddedRepeatableSection` change (the shape the deferred spec lifecycle needs).
    fn plan_v2() -> Schema {
        load_schema(
            b"\
type: plan
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: tasks
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one task\" } }
  - id: outcome
    slot: { hint: \"the outcome\" }
",
        )
        .expect("plan v2 loads")
    }

    /// A canonical v0-shaped `plan` (context + outcome filled), built through [`render`] so
    /// the input is the exact byte-stable form a first-touch-canonicalized corpus doc has.
    fn plan_v0_doc() -> String {
        let inst = Instance {
            title: "Ship the limiter".to_string(),
            sections: vec![
                SectionContent {
                    id: "context".to_string(),
                    slot: Some("Per-client limits were enforced ad hoc.".to_string()),
                    ..Default::default()
                },
                SectionContent {
                    id: "outcome".to_string(),
                    slot: Some("One limiter at the gateway.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&plan_v1(), &inst)
    }

    /// **The added-repeatable-section kind, end-to-end.** A wholly-new repeatable section
    /// classifies through the **real classifier** to exactly `[AddedRepeatableSection]`, and the
    /// driver mints its empty `## Heading` at the schema-ordered offset through the **same
    /// block-insert** `AddedOptionalSection` uses: the output round-trips byte-identical under
    /// v2, **conforms with ZERO items** (a zero-item repeatable conforms — `repeatable-populated`
    /// is a store advisory, not a conformance break), and preserves every prior byte.
    ///
    /// Red before T6: the pair diffed to the backstop's residual (`[Unclassified]`) — the
    /// migration **refused**, so a doctype could not grow a repeatable section at all.
    #[test]
    fn added_repeatable_section_mints_the_empty_heading_and_conforms_with_zero_items() {
        let v1 = plan_v1();
        let v2 = plan_v2();
        let src = plan_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedRepeatableSection {
                section: "tasks".to_string(),
            }],
            "a wholly-new repeatable names itself (never the empty diff, never the backstop)"
        );

        let out = transform(&v1, &v2, &src, &diff).expect("added-repeatable-section transform");

        // The empty `## Tasks` heading is minted between the neighbours, at its schema-ordered
        // offset — and it is the *only* byte added (every prior byte preserved verbatim).
        assert!(
            out.contains("## Tasks"),
            "the repeatable section heading is spliced in; got {out:?}"
        );
        assert!(
            out.contains("## Context\n\nPer-client limits were enforced ad hoc.\n"),
            "the context block is byte-preserved; got {out:?}"
        );
        assert!(
            out.contains("## Outcome\n\nOne limiter at the gateway.\n"),
            "the outcome block is byte-preserved; got {out:?}"
        );
        assert_eq!(
            out.replace("## Tasks\n\n\n", ""),
            src,
            "the mint adds exactly the empty heading block and touches nothing else"
        );

        // The done-criterion: byte-stable under v2 (the minted block is exactly the v2 writer's
        // canonical shape for a zero-item repeatable) and conformant **with zero items**.
        assert_byte_stable(&v2, &out);
        assert_conforms(&v2, &out);
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let tasks = inst
            .sections
            .iter()
            .find(|s| s.id == "tasks")
            .expect("the tasks section is present");
        assert!(
            tasks.items.is_empty(),
            "the minted repeatable carries zero items; got {:?}",
            tasks.items
        );
        let slot_of = |id: &str| {
            inst.sections
                .iter()
                .find(|s| s.id == id)
                .and_then(|s| s.slot.clone())
        };
        assert_eq!(
            slot_of("context").as_deref(),
            Some("Per-client limits were enforced ad hoc.")
        );
        assert_eq!(
            slot_of("outcome").as_deref(),
            Some("One limiter at the gateway.")
        );

        // Determinism, and the corpus fold commits it (so the CLI reaches the restamp).
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "the mint is deterministic");
        let corpus = [CorpusDoc {
            id: "plan-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, None, "a zero-item repeatable conforms");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Migrated {
                id: "plan-a".to_string(),
                v2: out.clone(),
            }],
            "the doc migrates (heading minted) — it does not block"
        );
    }

    // ---- (g) the two M38 doctype-level kinds: relocation + display-title H1 rewrite ----

    /// v1: a `log` doctype at an old folder home (`location: changelog/`) with a
    /// lowercase H1 (the slug).
    fn log_v1() -> Schema {
        load_schema(
            b"\
type: log
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        )
        .expect("log v1 loads")
    }

    /// v2: relocated to a literal root home (`placement: CHANGELOG.md`) **and** given a
    /// `display-title: Changelog` (so `# changelog` → `# Changelog`) — the real changelog
    /// v1→v2 relocation, synthetically.
    fn log_v2() -> Schema {
        load_schema(
            b"\
type: log
placement: { file: CHANGELOG.md }
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        )
        .expect("log v2 loads")
    }

    /// v2 differing from v1 **only** in `display-title` (same home, same sections) — to
    /// isolate the display-title-changed kind.
    fn log_v2_retitle_only() -> Schema {
        load_schema(
            b"\
type: log
location: changelog/
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        )
        .expect("log v2-retitle loads")
    }

    /// v2 differing from v1 **only** in home (`location:` → `placement:`, no
    /// display-title) — to isolate the relocated kind.
    fn log_v2_relocate_only() -> Schema {
        load_schema(
            b"\
type: log
placement: { file: CHANGELOG.md }
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        )
        .expect("log v2-relocate loads")
    }

    /// A canonical v0-shaped `log` with a lowercase (`# changelog`) H1, built through
    /// [`render`] so the input is the exact byte-stable form a first-touch-canonicalized
    /// corpus doc has.
    fn log_v0_doc() -> String {
        let inst = Instance {
            title: "changelog".to_string(),
            sections: vec![SectionContent {
                id: "body".to_string(),
                slot: Some("Released 1.0 with the new limiter.".to_string()),
                ..Default::default()
            }],
        };
        render(&log_v1(), &inst)
    }

    #[test]
    fn display_title_changed_rewrites_only_the_h1() {
        let v1 = log_v1();
        let v2 = log_v2_retitle_only();
        let src = log_v0_doc();
        assert!(
            src.starts_with("# changelog\n"),
            "the v0 H1 is the lowercase slug; got {src:?}"
        );

        // The real classifier emits the display-title change; the driver is exercised on
        // the emitted classification, not a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::DisplayTitleChanged {
                to: "Changelog".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("display-title transform");

        // ONLY the H1 line is rewritten: the output is byte-identical to the source save
        // the single `# changelog` → `# Changelog` flip (every other byte preserved).
        assert_eq!(
            out,
            src.replacen("# changelog", "# Changelog", 1),
            "only the H1 line changes; every other byte is preserved"
        );
        assert!(out.starts_with("# Changelog\n"));

        // (a) round-trips byte-identical under v2, (c) conforms.
        assert_byte_stable(&v2, &out);
        assert_conforms(&v2, &out);

        // (d) determinism: the same diff over the same source rewrites identically.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run");
        assert_eq!(again, out, "display-title rewrite is deterministic");
    }

    #[test]
    fn relocated_only_folds_byte_identical_a_content_no_op() {
        let v1 = log_v1();
        let v2 = log_v2_relocate_only();
        let src = log_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::Relocated {
                from: "changelog/".to_string(),
                to: "CHANGELOG.md".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("relocated transform");

        // A file move, not a content edit: the instance bytes are byte-identical (the
        // widened-cardinality sibling). The fs::rename + file-state re-key are T2's arm.
        assert_eq!(out, src, "relocated folds byte-identical (content no-op)");
        assert_byte_stable(&v2, &out);

        let again = transform(&v1, &v2, &src, &diff).expect("re-run");
        assert_eq!(again, out, "relocated fold is deterministic");
    }

    /// The reconstructed changelog reshape end-to-end: a synthetic committed doctype at
    /// an old home with a lowercase H1 migrates to a new literal home + fixed H1,
    /// byte-faithful. The relocation is a content no-op; the sole content change is the
    /// H1 re-title, so the transform output equals the source with only `# changelog` →
    /// `# Changelog`. (The file move + stamp bump are T2's migrate-corpus move-arm.)
    #[test]
    fn combined_relocation_and_display_title_replays_the_changelog_reshape() {
        let v1 = log_v1();
        let v2 = log_v2();
        let src = log_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![
                SchemaChange::Relocated {
                    from: "changelog/".to_string(),
                    to: "CHANGELOG.md".to_string(),
                },
                SchemaChange::DisplayTitleChanged {
                    to: "Changelog".to_string(),
                },
            ]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("combined transform");

        // Byte-faithful: only the H1 line moved to its fixed display form; the body slot
        // prose is preserved verbatim.
        assert_eq!(
            out,
            src.replacen("# changelog", "# Changelog", 1),
            "the relocation is a content no-op; only the H1 is re-titled"
        );
        assert!(out.starts_with("# Changelog\n"));
        assert!(
            out.contains("Released 1.0 with the new limiter."),
            "the body prose is byte-preserved; got {out:?}"
        );

        assert_byte_stable(&v2, &out);
        assert_conforms(&v2, &out);

        let again = transform(&v1, &v2, &src, &diff).expect("re-run");
        assert_eq!(again, out, "the combined fold is deterministic");
    }

    // ---- (h) the value-remapped kind: an enum member rename, both field shapes ----

    /// v1: a `ledger` with a repeatable `entries` section whose item block carries an
    /// enum `kind` over `[D, I]` (the deferral-ledger shape) plus a `body` slot.
    fn ledger_v1() -> Schema {
        load_schema(
            b"\
type: ledger
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [D, I] }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("ledger v1 loads")
    }

    /// v2: the `kind` enum members are **renamed** to `[Decision, Idea]` — everything
    /// else byte-identical (the deferral-ledger v1→v2 bump, synthetically).
    fn ledger_v2() -> Schema {
        load_schema(
            b"\
type: ledger
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [Decision, Idea] }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("ledger v2 loads")
    }

    /// A canonical v0-shaped `ledger` with two entries — one `kind: D`, one `kind: I` —
    /// built through [`render`] so the input is the exact byte-stable form a
    /// first-touch-canonicalized corpus doc has.
    fn ledger_v0_doc() -> String {
        let inst = Instance {
            title: "Deferral Ledger".to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items: vec![
                    ItemContent {
                        id: "cache-the-index".to_string(),
                        title: "Cache the index".to_string(),
                        slot: Some("Deferred until the store scope lands.".to_string()),
                        fields: vec![Field {
                            key: "kind".to_string(),
                            value: Value::Scalar("D".to_string()),
                        }],
                        ..Default::default()
                    },
                    ItemContent {
                        id: "a-plugin-surface".to_string(),
                        title: "A plugin surface".to_string(),
                        slot: Some("Parked until a real external domain earns it.".to_string()),
                        fields: vec![Field {
                            key: "kind".to_string(),
                            value: Value::Scalar("I".to_string()),
                        }],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
        };
        render(&ledger_v1(), &inst)
    }

    #[test]
    fn value_remap_on_a_repeatable_item_field_remaps_every_item_byte_faithful() {
        let v1 = ledger_v1();
        let v2 = ledger_v2();
        let src = ledger_v0_doc();

        // The real classifier emits the value-remapped change carrying an **empty** map;
        // the driver is exercised on the emitted classification, with the CLI-supplied map
        // threaded in (T3's channel, modeled here).
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::ValueRemapped {
                section: "entries".to_string(),
                field: "kind".to_string(),
                map: BTreeMap::new(),
            }]
        );
        let map = BTreeMap::from([
            ("D".to_string(), "Decision".to_string()),
            ("I".to_string(), "Idea".to_string()),
        ]);
        let changes = vec![SchemaChange::ValueRemapped {
            section: "entries".to_string(),
            field: "kind".to_string(),
            map,
        }];

        let out = transform(&v1, &v2, &src, &changes).expect("value-remap transform succeeds");

        // (c) conforms against v2 (the remapped members are now in the enum), (a)
        // round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) every item's `kind` is remapped; ids/titles/body prose are byte-preserved.
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let entries = inst
            .sections
            .iter()
            .find(|s| s.id == "entries")
            .expect("entries present");
        let kind_of = |item_id: &str| {
            entries
                .items
                .iter()
                .find(|i| i.id == item_id)
                .and_then(|i| i.fields.iter().find(|f| f.key == "kind"))
                .map(|f| f.value.clone())
        };
        assert_eq!(
            kind_of("cache-the-index"),
            Some(Value::Scalar("Decision".to_string()))
        );
        assert_eq!(
            kind_of("a-plugin-surface"),
            Some(Value::Scalar("Idea".to_string()))
        );
        let body_of = |item_id: &str| {
            entries
                .items
                .iter()
                .find(|i| i.id == item_id)
                .and_then(|i| i.slot.clone())
        };
        assert_eq!(
            body_of("cache-the-index").as_deref(),
            Some("Deferred until the store scope lands.")
        );
        assert_eq!(
            body_of("a-plugin-surface").as_deref(),
            Some("Parked until a real external domain earns it.")
        );

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &changes).expect("re-run succeeds");
        assert_eq!(again, out, "value-remap transform is deterministic");
    }

    /// v1: a `note` with a header `meta` carrying an enum `status` over `[open, closed]`.
    fn status_v1() -> Schema {
        load_schema(
            b"\
type: note
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed] }
  - id: body
    slot: { hint: \"the note body\" }
",
        )
        .expect("status v1 loads")
    }

    /// v2: the `status` enum members are **renamed** to `[active, archived]`.
    fn status_v2() -> Schema {
        load_schema(
            b"\
type: note
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [active, archived] }
  - id: body
    slot: { hint: \"the note body\" }
",
        )
        .expect("status v2 loads")
    }

    /// A canonical v0-shaped `note` carrying `status: open` + body prose.
    fn status_v0_doc() -> String {
        let inst = Instance {
            title: "A note".to_string(),
            sections: vec![
                SectionContent {
                    id: "meta".to_string(),
                    fields: vec![Field {
                        key: "status".to_string(),
                        value: Value::Scalar("open".to_string()),
                    }],
                    ..Default::default()
                },
                SectionContent {
                    id: "body".to_string(),
                    slot: Some("The note body, untouched by the remap.".to_string()),
                    ..Default::default()
                },
            ],
        };
        render(&status_v1(), &inst)
    }

    #[test]
    fn value_remap_on_a_simple_field_remaps_the_value_byte_faithful() {
        let v1 = status_v1();
        let v2 = status_v2();
        let src = status_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::ValueRemapped {
                section: "meta".to_string(),
                field: "status".to_string(),
                map: BTreeMap::new(),
            }]
        );
        let map = BTreeMap::from([
            ("open".to_string(), "active".to_string()),
            ("closed".to_string(), "archived".to_string()),
        ]);
        let changes = vec![SchemaChange::ValueRemapped {
            section: "meta".to_string(),
            field: "status".to_string(),
            map,
        }];

        let out = transform(&v1, &v2, &src, &changes).expect("simple value-remap succeeds");

        // (c) conforms against v2, (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) the field value is remapped; the body prose is byte-preserved.
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let status = inst
            .sections
            .iter()
            .find(|s| s.id == "meta")
            .and_then(|s| s.fields.iter().find(|f| f.key == "status"))
            .map(|f| f.value.clone());
        assert_eq!(status, Some(Value::Scalar("active".to_string())));
        let body = inst
            .sections
            .iter()
            .find(|s| s.id == "body")
            .and_then(|s| s.slot.clone());
        assert_eq!(
            body.as_deref(),
            Some("The note body, untouched by the remap.")
        );

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &changes).expect("re-run succeeds");
        assert_eq!(again, out, "simple value-remap is deterministic");
    }

    /// An **uncovered** committed value (the map does not include `open`) surfaces
    /// [`TransformError::Unsupported`] — it blocks loudly, never a silent no-op (a value the
    /// authored map cannot carry must halt the migration, not drop the entry).
    #[test]
    fn value_remap_over_an_uncovered_value_surfaces_unsupported() {
        let v1 = status_v1();
        let v2 = status_v2();
        let src = status_v0_doc(); // carries `status: open`

        // The map covers `closed` but NOT the committed `open` value.
        let map = BTreeMap::from([("closed".to_string(), "archived".to_string())]);
        let changes = vec![SchemaChange::ValueRemapped {
            section: "meta".to_string(),
            field: "status".to_string(),
            map,
        }];

        assert_eq!(
            transform(&v1, &v2, &src, &changes),
            Err(TransformError::Unsupported {
                kind: "value-remapped",
                section: "meta".to_string(),
            }),
            "an uncovered committed value blocks loudly"
        );
    }

    // ---- the direction-classified `of` / `card` deltas: widen folds, narrow refuses ----

    /// **The enum-widening kind, end-to-end.** `[open, closed]` → `[open, closed, blocked]`
    /// classifies through the **real classifier** to exactly `[EnumWidened]` and folds
    /// **byte-identical**: every committed value is still a declared member, so the doc conforms
    /// under v2 untouched and the migration restamps it. Red before T3: the pair classified
    /// `[ValueRemapped { map: {} }]` and the driver **blocked** the doc on its committed `open`
    /// — with no map to author, because nothing was renamed.
    #[test]
    fn enum_widened_folds_byte_identical_and_the_doc_still_conforms() {
        let v1 = status_v1(); // status: enum of [open, closed]
        let v2 = load_schema(
            b"\
type: note
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed, blocked] }
  - id: body
    slot: { hint: \"the note body\" }
",
        )
        .expect("status v2-widened loads");
        let src = status_v0_doc(); // carries `status: open`

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::EnumWidened {
                section: "meta".to_string(),
                field: "status".to_string(),
            }],
            "a widening names itself — it is not a rename, and needs no authored map"
        );

        let out = transform(&v1, &v2, &src, &diff).expect("an enum widening folds cleanly");
        assert_eq!(
            out, src,
            "an enum widening adds no bytes (every value still declared)"
        );
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // The corpus fold commits it (so the CLI reaches the restamp) — never a block.
        let corpus = [CorpusDoc {
            id: "note-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, None, "a byte no-op kind never blocks");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Migrated {
                id: "note-a".to_string(),
                v2: src.clone(),
            }],
            "the doc migrates byte-identical — it restamps, it does not block"
        );

        // Determinism.
        assert_eq!(transform(&v1, &v2, &src, &diff).expect("re-run"), out);
    }

    /// **A fold whose output does not parse halts as a PARSE failure, not as a gate break**
    /// (M46 Inc-4 T3). The three ways one doc can refuse to commit need three different repairs,
    /// so they stay distinguishable all the way out to the caller — and this is the variant no
    /// shipped doctype pair reaches through the binary (it takes a source the *new* schema cannot
    /// parse at all), which is why it is proven here at the seam and
    /// `crates/cli/tests/migrate_corpus_halt_causes.rs` says so rather than implying a
    /// real-binary cell for it.
    ///
    /// The change is a byte no-op (`WidenedCardinality`), so the transform commits its input
    /// unchanged — and that input is missing the required `## Body` heading entirely, so the
    /// post-fold parse rejects it before the conformance gate is ever consulted.
    #[test]
    fn a_fold_whose_output_does_not_parse_under_the_new_schema_halts_as_a_parse_failure() {
        let schema = load_schema(
            b"\
type: note
sections:
  - id: body
    slot: { hint: \"the note\" }
",
        )
        .expect("the note schema loads");
        let src = "\
# A note

prose, and no `## Body` heading at all
"
        .to_string();
        assert!(
            parse_sections(&schema, &src).is_err(),
            "the fixture's whole point is a source this schema cannot parse",
        );

        let changes = [SchemaChange::WidenedCardinality {
            section: "body".to_string(),
            field: "rel".to_string(),
        }];
        assert_eq!(
            transform(&schema, &schema, &src, &changes),
            Ok(src.clone()),
            "a widening folds to zero bytes — the transform itself never fails here",
        );

        let corpus = [CorpusDoc {
            id: "note-a",
            old_schema: &schema,
            new_schema: &schema,
            source: &src,
            changes: &changes,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, Some(0), "the fold halts on the doc");
        let Some(HaltReason::Parse(findings)) = &result.halt_reason else {
            panic!(
                "the halt is a PARSE failure, not a gate break — the two take different repairs; \
                 got: {:?}",
                result.halt_reason
            );
        };
        assert!(
            !findings.is_empty(),
            "the parse's own diagnostics ride along — they are what say where the buffer broke",
        );
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "note-a".to_string(),
                v0: src,
            }],
            "and the doc stays byte-identical v0",
        );
    }

    /// **A cardinality narrowing is REFUSED** (the recorded pick — `DECISIONS.md` → 2026-07-13
    /// M42 Inc-5 T3). `0..*` → `0..1` classifies `[NarrowedCardinality]` and the driver blocks:
    /// nothing in the system counts a committed instance's edges against the new bound, so the
    /// alternative arm (validate every instance) is net-new validation surface — and folding it
    /// as a no-op (the pre-T3 behaviour) restamps a corpus that may violate the bound, **past
    /// the gate**. The corpus fold therefore halts, leaving the doc byte-identical v0.
    #[test]
    fn narrowed_cardinality_refuses_the_transform_and_halts_the_corpus_fold() {
        // The reverse of the widening pair: v2 (0..*) is the OLD shape, v1 (0..1) the new one.
        let v1 = spec_v2();
        let v2 = spec_v1();
        let src = spec_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::NarrowedCardinality {
                section: "meta".to_string(),
                field: "derived-from".to_string(),
            }],
            "a narrowing names itself — it is not the widening no-op"
        );

        assert_eq!(
            transform(&v1, &v2, &src, &diff),
            Err(TransformError::Unsupported {
                kind: "narrowed-cardinality",
                section: "meta".to_string(),
            }),
            "the driver refuses a narrowing (it is content-affecting, never a no-op)"
        );

        let corpus = [CorpusDoc {
            id: "spec-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, Some(0), "the fold halts on the refusal");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "spec-a".to_string(),
                v0: src.clone(),
            }],
            "the doc stays byte-identical v0 — no bytes, no stamp"
        );
    }

    // ---- the deferred branches block, never silently drop ----

    // ---- the structural projection: `PresentationOnly` folds to zero bytes; `Unclassified`
    // ---- refuses (the empty-diff backstop)

    /// A change **outside the conformance-relevant projection** — here a slot hint reword and
    /// a `default:` add on an existing field — classifies `[PresentationOnly]` through the
    /// **real classifier** and folds **byte-identical**: the committed doc is untouched, still
    /// conformant and byte-stable under v2, and the migration goes on to restamp it. Without
    /// this, the backstop would refuse a hint typo fix — an unshippable frozen doctype.
    #[test]
    fn presentation_only_folds_byte_identical_and_the_doc_still_conforms() {
        let v1 = doca_v1();
        // v2: the `derived-from` leaf gains a `default:`, and the body slot's hint is reworded
        // — the projection (section set/order, slot presence, field set/type/of/card/optional)
        // is **identical**.
        let v2 = load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: doca, card: \"0..1\", default: \"doca:root\" }
  - id: body
    slot: { hint: \"the body prose, in full sentences\" }
",
        )
        .expect("doca v2-presentation loads");
        let src = doca_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::PresentationOnly],
            "an out-of-projection delta names itself (never the empty diff)"
        );

        let out = transform(&v1, &v2, &src, &diff).expect("presentation-only transform succeeds");
        assert_eq!(out, src, "a presentation-only change folds to zero bytes");
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // Determinism, and the corpus fold commits it (so the CLI reaches the restamp).
        assert_eq!(transform(&v1, &v2, &src, &diff).expect("re-run"), out);
        let corpus = [CorpusDoc {
            id: "doca-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, None, "a byte no-op kind never blocks");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Migrated {
                id: "doca-a".to_string(),
                v2: src.clone(),
            }],
            "the doc migrates (byte-identical) — it restamps, it does not strand"
        );
    }

    /// **The `optional` relaxation folds byte-identical** (the sixth hole's safe direction). A
    /// leaf tightened-to-loose (`optional: false → true`) cannot make a conformant doc
    /// non-conformant, so the change classifies [`SchemaChange::OptionalRelaxed`] through the
    /// **real classifier** and the driver folds **zero bytes** — the doc migrates and restamps
    /// (never blocks, never strands). Red before T4: the delta was invisible to the existing-leaf
    /// diff and fell through to the backstop's residual, which **refused** it — a legitimate,
    /// doc-safe change made unshippable.
    #[test]
    fn an_optional_relaxation_folds_byte_identical_and_the_doc_migrates() {
        let v1 = doca_v1();
        // v2: `derived-from` relaxes to `optional: true`; nothing else moves.
        let v2 = load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - { id: derived-from, type: ref, to: doca, card: \"0..1\", optional: true }
  - id: body
    slot: { hint: \"the body\" }
",
        )
        .expect("doca v2-relaxed loads");
        let src = doca_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::OptionalRelaxed {
                section: "meta".to_string(),
                leaf: Some("derived-from".to_string()),
            }],
            "the relaxation names itself (never the empty diff, never the backstop)"
        );

        let out = transform(&v1, &v2, &src, &diff).expect("the relaxation transform succeeds");
        assert_eq!(out, src, "an `optional` relaxation folds to zero bytes");
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
        assert_eq!(transform(&v1, &v2, &src, &diff).expect("re-run"), out);

        // The corpus fold commits it — the doc reaches the CLI's restamp, it does not strand.
        let corpus = [CorpusDoc {
            id: "doca-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, None, "a byte no-op kind never blocks");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Migrated {
                id: "doca-a".to_string(),
                v2: src.clone(),
            }],
        );
    }

    /// **The empty-diff backstop, engine side.** A pair whose projection moved with no
    /// expressible kind (a **removed section**) classifies `[Unclassified]`, and the driver
    /// **refuses** it — never a silent fold-to-no-op, which would restamp a doc that fails its
    /// own gate. The corpus fold therefore halts on it, leaving the doc byte-identical v0.
    #[test]
    fn unclassified_refuses_the_transform_and_halts_the_corpus_fold() {
        let v1 = note_v1(); // vision + success slots
        let v2 = load_schema(
            b"\
type: note
sections:
  - id: vision
    slot: { hint: \"the vision\" }
",
        )
        .expect("note v2-removed-section loads");
        let src = note_v0_doc();

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::Unclassified],
            "a projection move with no kind names itself"
        );

        assert_eq!(
            transform(&v1, &v2, &src, &diff),
            Err(TransformError::Unclassified),
            "the driver refuses an unclassified change (it cannot be silently ignored)"
        );

        let corpus = [CorpusDoc {
            id: "note-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, Some(0), "the fold halts on the refusal");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "note-a".to_string(),
                v0: src.clone(),
            }],
            "the doc stays byte-identical v0 — no bytes, no stamp"
        );
    }

    /// **A removed field is REFUSED** (the recorded pick — `DECISIONS.md` → 2026-07-13 M42
    /// Inc-5 T5: *refuse, not strip*). A v2 dropping a declared leaf classifies
    /// `[RemovedField]` through the **real classifier** and the driver blocks: stripping the
    /// committed field line would be a knowing exception to **No-data-loss**, a *declared*
    /// property of this pair, and no frozen doctype needs a removal — so the migration refuses
    /// and the corpus fold halts, leaving the doc byte-identical v0 (its committed value intact).
    ///
    /// Red before T5: the removal was invisible to the classifier (both loops iterate `v2`'s
    /// leaves), so the pair rode the backstop's residual — and a removal *accompanied* by any
    /// other classified change was dropped silently, restamping a doc that keeps a field line the
    /// current schema no longer declares.
    #[test]
    fn removed_field_refuses_the_transform_and_halts_the_corpus_fold() {
        // v1 = doca (a `derived-from` ref + a body slot); v2 drops the ref leaf entirely.
        let v1 = doca_v1();
        let v2 = load_schema(
            b"\
type: doca
sections:
  - id: meta
    header: true
    fields: []
  - id: body
    slot: { hint: \"the body\" }
",
        )
        .expect("doca v2-removed-field loads");
        let src = doca_v0_doc(); // carries `derived-from: doca:other`

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::RemovedField {
                section: "meta".to_string(),
                field: "derived-from".to_string(),
            }],
            "a dropped leaf names itself — it is not invisible, and not the backstop's residual"
        );

        assert_eq!(
            transform(&v1, &v2, &src, &diff),
            Err(TransformError::Unsupported {
                kind: "removed-field",
                section: "meta".to_string(),
            }),
            "the driver refuses a removal (stripping the value would lose committed data)"
        );

        let corpus = [CorpusDoc {
            id: "doca-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, Some(0), "the fold halts on the refusal");
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "doca-a".to_string(),
                v0: src.clone(),
            }],
            "the doc stays byte-identical v0 — the committed value is never destroyed"
        );
    }

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

    // ---- (g) the added-item-field kind: the three arms, at the repeatable locus (T7) ----

    /// v1 of a `deferrals`: one repeatable `entries` section whose item block is an id-from
    /// `title`, a `trigger` field and a `body` prose slot (the shipped `deferral-ledger` shape).
    fn deferrals_v1() -> Schema {
        load_schema(
            b"\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("deferrals v1 loads")
    }

    /// v2 grows the item block a **defaulted** `kind` enum — the deterministic arm: the value is
    /// spliced into every item.
    fn deferrals_v2_defaulted() -> Schema {
        load_schema(
            b"\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: kind, type: enum, of: [Decision, Idea], default: Decision }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("deferrals v2 (defaulted) loads")
    }

    /// A canonical v0-shaped `deferrals` carrying **two** entries (so "every item" is a real claim,
    /// not a sample of one), each with a `trigger` field + body prose. `kind` fields, if any, are
    /// supplied by the caller (the already-carries fixture renders under v2).
    fn deferrals_doc(schema: &Schema, kinds: [Option<&str>; 2]) -> String {
        let item = |id: &str, title: &str, trigger: &str, body: &str, kind: Option<&str>| {
            let mut fields = vec![Field {
                key: "trigger".to_string(),
                value: Value::Scalar(trigger.to_string()),
            }];
            if let Some(kind) = kind {
                fields.push(Field {
                    key: "kind".to_string(),
                    value: Value::Scalar(kind.to_string()),
                });
            }
            ItemContent {
                id: id.to_string(),
                title: title.to_string(),
                slot: Some(body.to_string()),
                fields,
                ..Default::default()
            }
        };
        let inst = Instance {
            title: "Deferral Ledger".to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items: vec![
                    item(
                        "the-freeze-exempt-floor",
                        "The freeze-exempt floor",
                        "M39",
                        "Owed: a detect+route floor for freeze-exempt doctypes.",
                        kinds[0],
                    ),
                    item(
                        "the-abandon-path",
                        "The abandon path",
                        "M42",
                        "Parked: what a milestone's abandon path commits.",
                        kinds[1],
                    ),
                ],
                ..Default::default()
            }],
        };
        render(schema, &inst)
    }

    /// **The deterministic arm.** A `default:`-carrying field added to a repeatable item block
    /// classifies `[AddedItemField]` and the driver splices the value into **every** item —
    /// byte-stable, conformant, every prior byte preserved, deterministic. Red before T7: the
    /// classifier emitted `[]`, so the stamp flipped and the declared default silently never
    /// landed ("migrated by accident").
    #[test]
    fn added_item_field_with_a_default_splices_the_value_into_every_item() {
        let v1 = deferrals_v1();
        let v2 = deferrals_v2_defaulted();
        let src = deferrals_doc(&v1, [None, None]);
        assert!(
            !src.contains("kind:"),
            "the pre-bump doc carries no kind bullet; got:\n{src}"
        );

        // The real classifier emits the kind; the driver is exercised on the emitted
        // classification, never a hand-built list.
        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedItemField {
                section: "entries".to_string(),
                field: "kind".to_string(),
            }]
        );

        let out = transform(&v1, &v2, &src, &diff).expect("added-item-field transform succeeds");

        // (c) conforms against v2, (a) round-trips byte-identical.
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);

        // (b) EVERY item carries the defaulted value; every prior field/slot byte survives.
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let entries = inst
            .sections
            .iter()
            .find(|s| s.id == "entries")
            .expect("entries present");
        assert_eq!(entries.items.len(), 2);
        for item in &entries.items {
            assert_eq!(
                item.fields
                    .iter()
                    .find(|f| f.key == "kind")
                    .map(|f| f.value.clone()),
                Some(Value::Scalar("Decision".to_string())),
                "every item carries the default; item {:?} does not",
                item.id
            );
        }
        assert_eq!(
            out.replace("- kind: Decision\n", ""),
            src,
            "the migration adds the defaulted bullet to each item — nothing else"
        );

        // (d) determinism.
        let again = transform(&v1, &v2, &src, &diff).expect("re-run succeeds");
        assert_eq!(again, out, "the item-field splice is deterministic");
    }

    /// **The optional-no-default arm — classified, and ZERO bytes written.** An item without the
    /// bullet **already conforms**, so there is nothing deterministic to place, and inventing an
    /// empty bullet would fabricate a value. The kind still *names itself* (the backstop requires
    /// that of every real change) and folds byte-identical.
    #[test]
    fn added_optional_item_field_with_no_default_writes_zero_bytes() {
        let v1 = deferrals_v1();
        let v2 = load_schema(
            b"\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: owner, type: string, optional: true }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("deferrals v2 (optional) loads");
        let src = deferrals_doc(&v1, [None, None]);

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedItemField {
                section: "entries".to_string(),
                field: "owner".to_string(),
            }]
        );

        let out =
            transform(&v1, &v2, &src, &diff).expect("the optional arm is a no-op, not a block");
        assert_eq!(
            out, src,
            "an optional item field with no default adds no bytes (no fabricated bullet)"
        );
        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
    }

    /// **The required-no-default arm — the doc blocks.** No deterministic value exists, so the
    /// change classifies `[ProseNeeding { leaf: Some }]` (named, where pre-T7 it diffed to `[]`)
    /// and the per-doc corpus fold refuses the doc, leaving it byte-identical v0 — never a silent
    /// restamp past its own gate.
    #[test]
    fn added_required_item_field_with_no_default_blocks_the_doc() {
        let v1 = deferrals_v1();
        let v2 = load_schema(
            b"\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: owner, type: string }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        )
        .expect("deferrals v2 (required) loads");
        let src = deferrals_doc(&v1, [None, None]);

        let diff = schema_diff(&v1, &v2);
        assert_eq!(
            diff,
            vec![SchemaChange::ProseNeeding {
                section: "entries".to_string(),
                leaf: Some("owner".to_string()),
            }]
        );

        let corpus = [CorpusDoc {
            id: "deferrals-a",
            old_schema: &v1,
            new_schema: &v2,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(
            result.halted_at,
            Some(0),
            "the fold halts on the prose need"
        );
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "deferrals-a".to_string(),
                v0: src.clone(),
            }],
            "the blocked doc stays byte-identical v0"
        );
    }

    /// **An item that already carries the bullet keeps its authored value** (the T2 stranding
    /// class, at the item locus — the fixture every "historical doc lacks it" test misses). An
    /// adopter who hand-authored `kind: Idea` on one entry ahead of the bump keeps it; only the
    /// item that lacks the bullet gets the default. The splice is insert-only per item, so the
    /// fold is **idempotent** and **No-data-loss** holds.
    #[test]
    fn an_item_already_carrying_the_added_field_keeps_its_authored_value() {
        let v1 = deferrals_v1();
        let v2 = deferrals_v2_defaulted();
        // The hand-authored shape: entry 1 already carries `kind: Idea`; entry 2 does not.
        let src = deferrals_doc(&v2, [Some("Idea"), None]);

        let diff = schema_diff(&v1, &v2);
        let out = transform(&v1, &v2, &src, &diff).expect("transform succeeds");

        assert_conforms(&v2, &out);
        assert_byte_stable(&v2, &out);
        let inst = instance_from_source(&v2, &out).expect("v2 re-parse");
        let kind_of = |id: &str| {
            inst.sections
                .iter()
                .find(|s| s.id == "entries")
                .and_then(|s| s.items.iter().find(|i| i.id == id))
                .and_then(|i| i.fields.iter().find(|f| f.key == "kind"))
                .map(|f| f.value.clone())
        };
        assert_eq!(
            kind_of("the-freeze-exempt-floor"),
            Some(Value::Scalar("Idea".to_string())),
            "the authored value survives — the default never overwrites it"
        );
        assert_eq!(
            kind_of("the-abandon-path"),
            Some(Value::Scalar("Decision".to_string())),
            "the item that lacked the bullet gets the default"
        );

        // Idempotent: re-folding the migrated bytes changes nothing (every item now carries the
        // bullet, so there is nothing to insert).
        let again = transform(&v1, &v2, &out, &diff).expect("re-fold succeeds");
        assert_eq!(again, out, "the item-field fold converges");
    }

    // ---- (h) the conformance-clean-absence axis: ONE predicate, at BOTH loci ----

    /// Load an inline v2 schema, resolving the dev pack's one declared field type
    /// (`code-anchor`) so the **pack-typed** arm of the axis below is loadable at all.
    fn load_v2(yaml: &str, label: &str) -> Schema {
        crate::schema::load_schema_with_types(
            yaml.as_bytes(),
            &crate::schema::dev_pack_field_types(),
        )
        .unwrap_or_else(|err| panic!("{label} v2 loads: {err}"))
    }

    /// The **added-leaf axis of conformance-clean absence**, at the *simple* locus: the three
    /// shapes whose absence already conforms for a reason other than `optional: true` — a
    /// `set:`-derived field, an **optional `ref`**, and a **pack-declared type**.
    /// [`crate::validate::is_author_required`] exempts all three, so `schema_conformance` never
    /// asks a committed doc for them: there is nothing deterministic to write and **nothing to
    /// refuse**. Each folds byte-identical and the corpus never halts.
    ///
    /// Red before M46 Inc-4 T1: the classifier's private re-derivation covered only
    /// `optional || default || set`, so the ref and the pack type classified `ProseNeeding`, and
    /// the `set:` arm reached the driver and returned `Unsupported` — three dead ends behind a
    /// route that says *author the prose, then re-run*, where re-running changes nothing.
    ///
    /// The **controls** are the neighbouring tests, unchanged: a `default:`-bearing add still
    /// splices (`added_field_into_an_existing_header_is_byte_stable_and_preserves_values`), and a
    /// required leaf with neither `default:` nor `set:` still classifies `ProseNeeding`
    /// (`added_required_item_field_with_no_default_blocks_the_doc`).
    #[test]
    fn a_conformance_clean_added_field_folds_byte_identical_at_the_simple_locus() {
        let v1 = doca_v1();
        let src = doca_v0_doc();
        for (label, decl, field) in [
            (
                "set-derived",
                "{ id: date, type: date, set: on-create }",
                "date",
            ),
            (
                "optional-ref",
                "{ id: supersedes, type: ref, to: doca, card: \"0..*\" }",
                "supersedes",
            ),
            (
                "pack-typed",
                "{ id: cites-code, type: code-anchor }",
                "cites-code",
            ),
        ] {
            let yaml = format!(
                "\
type: doca
sections:
  - id: meta
    header: true
    fields:
      - {{ id: derived-from, type: ref, to: doca, card: \"0..1\" }}
      - {decl}
  - id: body
    slot: {{ hint: \"the body\" }}
"
            );
            let v2 = load_v2(&yaml, label);

            // The real classifier emits the kind; the driver is exercised on the emitted
            // classification, never a hand-built list.
            let diff = schema_diff(&v1, &v2);
            assert_eq!(
                diff,
                vec![SchemaChange::AddedOptionalField {
                    section: "meta".to_string(),
                    field: field.to_string(),
                }],
                "{label}: an absence that already conforms is placeable, not prose-needing"
            );

            let out = transform(&v1, &v2, &src, &diff)
                .unwrap_or_else(|err| panic!("{label}: the fold must not refuse; got {err:?}"));
            assert_eq!(out, src, "{label}: no value exists to invent — zero bytes");
            assert_conforms(&v2, &out);
            assert_byte_stable(&v2, &out);

            let corpus = [CorpusDoc {
                id: "doca-a",
                old_schema: &v1,
                new_schema: &v2,
                source: &src,
                changes: &diff,
            }];
            let result = migrate_corpus(&corpus);
            assert_eq!(
                result.halted_at, None,
                "{label}: the corpus fold never halts"
            );
            assert_eq!(
                result.docs,
                vec![DocOutcome::Migrated {
                    id: "doca-a".to_string(),
                    v2: src.clone(),
                }],
                "{label}: the doc migrates byte-identical"
            );
        }
    }

    /// The **same axis at the item locus** — the twin loop that produced the M42 holes by
    /// disagreeing with its sibling. The same three shapes, added to a repeatable item block,
    /// fold byte-identical (no fabricated bullet on any item) and never halt the corpus.
    #[test]
    fn a_conformance_clean_added_item_field_folds_byte_identical_at_the_item_locus() {
        let v1 = deferrals_v1();
        let src = deferrals_doc(&v1, [None, None]);
        for (label, decl, field) in [
            (
                "set-derived",
                "{ id: date, type: date, set: on-create }",
                "date",
            ),
            (
                "optional-ref",
                "{ id: supersedes, type: ref, to: deferrals, card: \"0..*\" }",
                "supersedes",
            ),
            (
                "pack-typed",
                "{ id: cites-code, type: code-anchor }",
                "cites-code",
            ),
        ] {
            let yaml = format!(
                "\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - {{ id: title, type: string }}
        - {{ id: trigger, type: string }}
        - {decl}
        - {{ id: body, slot: {{ hint: \"the deferral\" }} }}
"
            );
            let v2 = load_v2(&yaml, label);

            let diff = schema_diff(&v1, &v2);
            assert_eq!(
                diff,
                vec![SchemaChange::AddedItemField {
                    section: "entries".to_string(),
                    field: field.to_string(),
                }],
                "{label}: the item locus classifies exactly as its simple-section twin"
            );

            let out = transform(&v1, &v2, &src, &diff)
                .unwrap_or_else(|err| panic!("{label}: the fold must not refuse; got {err:?}"));
            assert_eq!(
                out, src,
                "{label}: no item gains a fabricated bullet — zero bytes"
            );
            assert_conforms(&v2, &out);
            assert_byte_stable(&v2, &out);

            let corpus = [CorpusDoc {
                id: "deferrals-a",
                old_schema: &v1,
                new_schema: &v2,
                source: &src,
                changes: &diff,
            }];
            let result = migrate_corpus(&corpus);
            assert_eq!(
                result.halted_at, None,
                "{label}: the corpus fold never halts"
            );
            assert_eq!(
                result.docs,
                vec![DocOutcome::Migrated {
                    id: "deferrals-a".to_string(),
                    v2: src.clone(),
                }],
                "{label}: the doc migrates byte-identical"
            );
        }
    }
}
