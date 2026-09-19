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
use crate::schema_diff::{
    Locus, LocusDisposition, SchemaChange, SchemaChangeKind, locus_disposition,
};
use crate::validate::schema_conformance;
use crate::write::{self, GenerateError, ItemSlotError, SpliceError};
use std::collections::BTreeMap;

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
        /// The classified kind's wire name — always a
        /// [`SchemaChangeKind::as_str`] output, never a literal typed here
        /// (`crates/cli/tests/schema_change_kind_registry.rs` fences every construction
        /// site in this module's production source).
        kind: &'static str,
        /// The **locus** the change concerns — `releases` at the section and item loci,
        /// `releases/changes` at the nested one, so a refusal one level down says which
        /// level ([`crate::schema_diff::Locus`]).
        locus: Locus,
    },
    /// The `added-item-slot` fold found an item whose **committed bytes the parse does not
    /// model** — prose after its `<!-- fields -->` group is the reachable instance — so the
    /// whole-item re-render the reshape performs would silently destroy them. **No-data-loss**
    /// is a declared property of this pair (the reason [`SchemaChange::RemovedItemSlot`] is
    /// refused rather than stripped, one arm up), so the fold refuses the doc instead of
    /// rewriting it. Carries the located item, because the repair is a one-item edit.
    UnmodelledItemContent {
        /// The repeatable section the item sits in.
        section: String,
        /// The item's **addressing chain from the section root** — a bare `{#id}` anchor at
        /// the item locus, `1-0-0/changes/added` one level down, because same-anchor nested
        /// items under different parents are legal and an anchor alone would not name one
        /// ([`crate::write::ItemSlotError::UnmodelledContent`]).
        item: String,
    },
    /// A [`SchemaChange::ValueRemapped`] whose field is the block's **`id-from`** — the leaf
    /// the item's identity is derived from — over a document carrying at least one committed
    /// identity the rename orphans.
    ///
    /// The `id-from` leaf is never a committed `- key: value` bullet: the parser consumes it
    /// as the item's heading and slugs the `{#id}` anchor from it, so "remap the value" would
    /// mean *re-mint the item's id* — an identity change, which `jigc doc retitle-item`
    /// refuses unconditionally for an enum id-source and which no migration performs
    /// (`design/storage.md` → Identity). There is no arm to build: the refusal is the verdict.
    ///
    /// **Scoped to the docs the rename actually orphans** ([`crate::validate::IdFromViolation`]
    /// → `NotEnumMember`, the shipped adjudicator the conformance gate itself asks). An
    /// instance whose every committed identity is still a declared member has nothing to
    /// rewrite and migrates, so this never blocks a doc that already conforms.
    IdFromRemap {
        /// The locus whose block declares the field as its `id-from`.
        locus: Locus,
        /// The `id-from` leaf the rename was made on.
        field: String,
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

impl From<ItemSlotError> for TransformError {
    fn from(err: ItemSlotError) -> Self {
        match err {
            ItemSlotError::Generate(err) => TransformError::Generate(err),
            ItemSlotError::UnmodelledContent { section, item } => {
                TransformError::UnmodelledItemContent { section, item }
            }
        }
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
        // THE `kind x locus` CELL, asked before the fold (M50 Increment 6 / T2). A cell whose
        // byte-writing arm is not built at this locus refuses here — naming the kind and the
        // locus path — rather than reaching a per-variant arm that would splice an item-locus
        // primitive at the wrong depth. Every other cell falls through to its own arm, which is
        // where a zero-byte fold, a splice, and a by-design refusal are decided.
        if let Some(locus) = change.locus()
            && locus_disposition(SchemaChangeKind::from(change), locus.index())
                == LocusDisposition::Unbuilt
        {
            return Err(TransformError::Unsupported {
                kind: SchemaChangeKind::from(change).as_str(),
                locus: locus.clone(),
            });
        }
        match change {
            SchemaChange::FixedSlotToRepeatable { locus } => {
                // The default first item's title is the section id (deterministic,
                // domain-empty, always slug-able). The primitive carries the old slot
                // prose verbatim as that item's body.
                let section = locus.section();
                out = write::promote_slot_to_repeatable(
                    old_schema, new_schema, &out, section, section,
                )?;
            }
            SchemaChange::WidenedCardinality { .. } => {
                // Instance-byte identity: the existing value is still valid under the
                // widened cardinality, so no splice is needed.
            }
            SchemaChange::NarrowedCardinality { locus, .. } => {
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
                    kind: SchemaChangeKind::NarrowedCardinality.as_str(),
                    locus: locus.clone(),
                });
            }
            SchemaChange::RemovedField { locus, .. } => {
                // THE REMOVAL PICK — refuse, not strip (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T5;
                // `corpus-migration.md` → The two silent-classification holes, which left the
                // shape open). A strip arm would splice the committed field line away
                // deterministically — and **destroy the committed values**, a knowing exception to
                // **No-data-loss**, a declared property of this pair. No frozen doctype needs a
                // removal, so the strip is premature generality; the *silent* hole is the actual
                // defect, and refusing closes it at zero risk. Additive to build later if a real
                // driver appears.
                return Err(TransformError::Unsupported {
                    kind: SchemaChangeKind::RemovedField.as_str(),
                    locus: locus.clone(),
                });
            }
            SchemaChange::RemovedItemSlot { locus, .. } => {
                // THE REMOVAL PICK, at the item locus — refuse, not strip, on the same recorded
                // reasoning as [`SchemaChange::RemovedField`] and one sharper than it: the bytes
                // a strip arm would splice away here are the item's **authored prose**, so the
                // exception to **No-data-loss** would be the larger one. Refusing closes the
                // silent hole (a removal riding alongside a classified change never reaches the
                // backstop) at zero risk, and leaves the opt-in strip arm additive.
                return Err(TransformError::Unsupported {
                    kind: SchemaChangeKind::RemovedItemSlot.as_str(),
                    locus: locus.clone(),
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
            SchemaChange::AddedOptionalSection { locus } => {
                // Splice the empty `## Heading` slot-section at its schema-ordered home —
                // the same block-insert path the required-slot `ProseNeeding` arm uses,
                // but an **optional** empty slot *conforms* instead of blocking, so this
                // is the migration's final form (not a mint-then-author handoff). The
                // v2 writer emits the heading unconditionally, so a historical doc lacking
                // it is non-canonical; the empty-slot section canonicalizes to exactly the
                // v2 writer shape (byte-stable — `render(parse(out)) == out`; proven by the
                // byte-stability test), leaving every prior section's bytes untouched.
                out = write::generate_section(new_schema, &out, locus.section(), Some(""), &[])?;
            }
            SchemaChange::AddedRepeatableSection { locus } => {
                // Mint the empty `## Heading` at its schema-ordered home through the **same**
                // block-insert the added-optional-section arm uses — the section body's shape
                // (repeatable vs simple) is not the *insert's* concern: with no slot prose and
                // no fields, the generated block is exactly the v2 writer's canonical form for a
                // **zero-item** repeatable (`render_section`'s `Repeatable` arm over an empty
                // item list). A zero-item repeatable **conforms** (`repeatable-populated` is a
                // store advisory, not a conformance break), so this is the migration's final
                // byte-stable form — not a mint-then-author handoff, and the CLI invents no items
                // (`corpus-migration.md` → The classifier's holes: `AddedRepeatableSection`).
                out = write::generate_section(new_schema, &out, locus.section(), None, &[])?;
            }
            SchemaChange::ProseNeeding { locus, leaf: None } => {
                // Framing A — the prose-routing branch (T4). A new **required section
                // slot** has no deterministic default, so the CLI mints it **empty** at
                // its schema-ordered home and stops: the reused conformance gate then
                // blocks on the empty slot (`required-slot-present`) until the agent
                // authors the prose. The CLI owns only placement; it never authors the
                // prose — the determinism boundary (`corpus-migration.md` → Prose
                // routing). The minted block is byte-stable by construction
                // (`generate_section`), and `set_slot` fills it byte-stably once authored.
                out = write::generate_section(new_schema, &out, locus.section(), Some(""), &[])?;
            }
            SchemaChange::ProseNeeding {
                locus,
                leaf: Some(_),
            } => {
                // A new required **field** with no default is also prose-needing, but the
                // transform's prose routing mints a **slot**, not a field
                // (`corpus-migration.md` → Prose routing: "mints the empty slot"). No
                // shipped or reconstructed migration needs the field sub-case, so its
                // driver branch is unbuilt — surfaced, never silently dropped (an un-built
                // branch must block the migration rather than drop the change).
                return Err(TransformError::Unsupported {
                    kind: SchemaChangeKind::ProseNeeding.as_str(),
                    locus: locus.clone(),
                });
            }
            SchemaChange::AddedOptionalField { locus, field } => {
                out = apply_added_field(new_schema, &out, locus, field)?;
            }
            SchemaChange::AddedItemField { locus, field } => {
                out = apply_added_item_field(new_schema, &out, locus, field)?;
            }
            SchemaChange::AddedItemSlot { locus, leaf } => {
                // The item block gains a prose leaf. The primitive re-keys every committed
                // item's slot prose onto the new template's leaf order and mints the added
                // leaf **empty** at its schema-ordered offset — reading through `old_schema`,
                // because a v1-shaped instance does not parse under a v2 multi-slot template
                // (the `promote_slot_to_repeatable` route, which is why both schemas are
                // threaded here). Below two declared slots it writes zero bytes: a lone slot
                // renders bare, under no sub-heading. A **required** added leaf is not refused
                // here — it mints empty and the per-doc conformance gate adjudicates it, so the
                // doc collects the doc-authorable Framing-A route instead of a build
                // instruction (`design/corpus-migration.md` → Prose routing).
                //
                // The primitive takes the change's **locus**, not its section id (M50
                // Increment 7 / T4): the block whose slot layout is reshaped is the one at
                // `locus.nested()`, the committed items are enumerated per parent along that
                // chain, and each is re-rendered at its own nesting depth. Handed the section
                // alone it reshaped the outer block — the wrong items, at the wrong depth.
                out = write::insert_item_slot(
                    old_schema,
                    new_schema,
                    &out,
                    locus.section(),
                    locus.nested(),
                    leaf,
                )?;
            }
            SchemaChange::ValueRemapped { locus, field, map } => {
                // The first **parameterized** transform kind: remap each committed value of
                // the enum `field` through the authored old→new `map` (an enum rename is
                // unrecoverable from the schema pair, so the CLI supplies the map — the
                // determinism boundary holds: a deterministic input, not an LLM call). A
                // committed value the map does not cover blocks loudly (`Unsupported`),
                // never a silent no-op (`corpus-migration.md` → the value-remap kind).
                out = apply_value_remap(new_schema, &out, locus, field, map)?;
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
    locus: &Locus,
    field: &str,
) -> Result<String, TransformError> {
    let section = locus.section();
    let unsupported = || TransformError::Unsupported {
        kind: SchemaChangeKind::AddedOptionalField.as_str(),
        locus: locus.clone(),
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

/// Splice an `added-item-field` change: place the new leaf `field` on every committed item **at
/// the change's own locus** — the repeatable section's own item block, or a nested one — carrying
/// its **deterministic** value.
///
/// The splice runs through the **insert-capable** primitive for the locus's depth — at the item
/// locus [`write::set_item_field_or_insert`], below it the depth-aware
/// [`write::set_nested_item_field_or_insert`] — which is the dispatch
/// [`write::set_item_field_validated`] already makes at the CLI seam, copied here so the migration
/// and the write path cannot disagree about what "nested" means (M50 Increment 7). Either way the
/// primitive generates the absent `- key: value` bullet and adjudicates the value against its
/// declared type before touching bytes. [`write::set_item_field`] is *update-only*: it refuses an
/// absent bullet with `SpliceError::NotPresent`, i.e. on **every item, by definition of this kind**
/// (it is the right primitive for [`SchemaChange::ValueRemapped`], which overwrites an *existing*
/// bullet).
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
    locus: &Locus,
    field: &str,
) -> Result<String, TransformError> {
    let section = locus.section();
    let unsupported = || TransformError::Unsupported {
        kind: SchemaChangeKind::AddedItemField.as_str(),
        locus: locus.clone(),
    };
    let sec = new_schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .ok_or_else(unsupported)?;
    let SectionBody::Repeatable { repeatable } = &sec.body else {
        return Err(unsupported());
    };
    // THE DECLARATION IS RESOLVED AT THE CHANGE'S OWN LOCUS, never by section id alone (M50
    // Increment 6 / T2). A nested leaf whose id also exists in the outer block — `date` on
    // `changelog.releases` is the shipped collision — otherwise resolves to the *outer*
    // declaration and takes its branch: it reads that field's `set:`-without-`default:` no-op,
    // writes nothing, and lets the doc restamp with the declared value never placed.
    let mut block = &repeatable.block;
    for hop in locus.nested() {
        let Some(Leaf::Repeatable { repeatable, .. }) = block
            .iter()
            .find(|leaf| matches!(leaf, Leaf::Repeatable { id, .. } if id == hop))
        else {
            return Err(unsupported());
        };
        block = &repeatable.block;
    }
    let decl = block
        .iter()
        .find_map(|leaf| match leaf {
            Leaf::Field(f) if f.id == field => Some(f.as_ref()),
            _ => None,
        })
        .ok_or_else(unsupported)?;

    let value = match &decl.default {
        Some(default) => default.clone(),
        // The simple locus's rule, verbatim: an absence the conformance gate accepts writes no
        // bytes on any item — at **every** locus, which is why a nested add of a leaf the gate
        // never asks for migrates instead of blocking.
        None if !crate::validate::is_author_required(decl) => return Ok(source.to_string()),
        None => return Err(unsupported()),
    };
    // Collect the addressing chain of every item AT THE CHANGE'S OWN LOCUS from the initial
    // parse, then splice each in turn. Item ids are stable under a field insert (the id-from
    // leaf is untouched), so a fresh write re-locates each item after the prior splice.
    let doc = parse_sections(new_schema, source).map_err(|_| unsupported())?;
    let Some(parsed) = doc.sections.iter().find(|s| s.id == section) else {
        // The instance omits the section — no items, nothing to place.
        return Ok(source.to_string());
    };
    let targets: Vec<Vec<String>> = items_at_locus(parsed, locus)
        .into_iter()
        // An item that already carries the bullet keeps its authored value — the skip that
        // makes the fold idempotent and **No-data-loss** hold, at every locus.
        .filter(|(_, item)| !item.fields.iter().any(|f| f.key == field))
        .map(|(chain, _)| chain)
        .collect();

    let mut out = source.to_string();
    for chain in targets {
        let chain: Vec<&str> = chain.iter().map(String::as_str).collect();
        // The dispatch [`write::set_item_field_validated`] already makes at the CLI seam, so
        // the migration and the write path cannot disagree about what "nested" means: a
        // single-hop chain is a top-level item, a deeper one takes the depth-aware primitive.
        out = match chain.as_slice() {
            [item_id] => {
                write::set_item_field_or_insert(new_schema, &out, section, item_id, field, &value)?
            }
            _ => write::set_nested_item_field_or_insert(
                new_schema, &out, section, &chain, field, &value,
            )?,
        };
    }
    Ok(out)
}

/// Every committed item **at `locus`** inside the already-parsed section `parsed`, paired with
/// the addressing chain the write path takes to reach it.
///
/// The chain is the write path's own address shape: item id, then one nested-section hop per
/// level, then the nested item's id (`["1-0-0", "changes", "added"]`). Descending it **per
/// parent** is what makes same-anchor nested items under different parents legal — two releases
/// may each carry a `#### added`, and addressing one by its anchor alone would write one of them
/// twice and the other never.
///
/// One walk, shared by every item-locus arm ([`apply_added_item_field`], [`apply_value_remap`]),
/// so two kinds cannot come to disagree about which items a locus names.
fn items_at_locus<'a>(
    parsed: &'a crate::parse::ParsedSection,
    locus: &Locus,
) -> Vec<(Vec<String>, &'a crate::parse::ParsedItem)> {
    let mut level: Vec<(Vec<String>, &crate::parse::ParsedItem)> = parsed
        .items
        .iter()
        .map(|item| (vec![item.id.clone()], item))
        .collect();
    for hop in locus.nested() {
        level = level
            .into_iter()
            .flat_map(|(chain, item)| {
                item.items.iter().map(move |nested| {
                    let mut chain = chain.clone();
                    chain.push(hop.clone());
                    chain.push(nested.id.clone());
                    (chain, nested)
                })
            })
            .collect();
    }
    level
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
///
/// **A rename made on the block's own `id-from` is refused before the fold**
/// ([`TransformError::IdFromRemap`]): that leaf is the item's heading rather than a bullet,
/// so remapping it would re-mint the item's `{#id}` — an identity change, not a value
/// rewrite. Refused only over the identities the rename orphans, so an instance carrying
/// none of them still migrates.
fn apply_value_remap(
    schema: &Schema,
    source: &str,
    locus: &Locus,
    field: &str,
    map: &BTreeMap<String, String>,
) -> Result<String, TransformError> {
    fn unsupported(locus: &Locus) -> TransformError {
        TransformError::Unsupported {
            kind: SchemaChangeKind::ValueRemapped.as_str(),
            locus: locus.clone(),
        }
    }
    let section = locus.section();
    let sec = schema
        .sections
        .iter()
        .find(|s| s.id == section)
        .ok_or_else(|| unsupported(locus))?;

    let doc = parse_sections(schema, source).map_err(|_| unsupported(locus))?;
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
            let new_value = remap_value(&f.value, map).ok_or_else(|| unsupported(locus))?;
            Ok(write::set_field(
                schema, source, section, field, &new_value,
            )?)
        }
        SectionBody::Repeatable { repeatable } => {
            // THE `id-from` ROLE, asked before the fold (M52 Increment 7 / T3). The block the
            // change names is resolved at the change's OWN locus — the hop-walk
            // [`apply_added_item_field`] performs for the same reason: a nested leaf whose id
            // also exists in the outer block would otherwise be adjudicated against the wrong
            // declaration.
            //
            // The comment that stood below asserted the converse — *"the enum field is never
            // the id-from — an enum id-from item is reslug-refused"* — and never checked it.
            // It is false for the shipped `changelog` change-group, whose `category` enum IS
            // the block's `id-from` at both item loci; driven, the loop below found no bullet
            // on any item, wrote zero bytes, and handed the byte-identical buffer to the
            // caller's conformance gate, which broke on the heading the rename had just
            // orphaned. So the rule the code performs is stated here instead, and enforced: an
            // `id-from` value is the item's identity, a remap of it is a re-slug, and no
            // migration performs one.
            //
            // **Scoped to the identities the rename orphans**, and scoped by the adjudicator
            // the conformance gate itself asks, so the two can never disagree about which
            // heading is still a member: an instance carrying only declared identities has
            // nothing to rewrite and migrates. The other two verdicts are deliberately not
            // ours — a malformed or non-trailer-shaped heading is a pre-existing doc problem
            // this rename neither caused nor repairs, and the gate owns it.
            let mut block = repeatable;
            for hop in locus.nested() {
                let Some(Leaf::Repeatable {
                    repeatable: inner, ..
                }) = block
                    .block
                    .iter()
                    .find(|leaf| matches!(leaf, Leaf::Repeatable { id, .. } if id == hop))
                else {
                    return Err(unsupported(locus));
                };
                block = inner;
            }
            if block.id_from == field
                && items_at_locus(parsed, locus).iter().any(|(_, item)| {
                    matches!(
                        crate::validate::id_from_enum_violation(block, &item.title, &schema.ty),
                        Some(crate::validate::IdFromViolation::NotEnumMember(_)),
                    )
                })
            {
                return Err(TransformError::IdFromRemap {
                    locus: locus.clone(),
                    field: field.to_string(),
                });
            }

            // Collect (item chain, remapped value) for every item AT THE CHANGE'S OWN LOCUS
            // from the initial parse, then splice each via the present-field item write path.
            // Item ids are stable under a value-span splice — the `id-from` is the one leaf a
            // splice could move, and the guard above has refused it — so a fresh write locates
            // each item after the prior splice.
            //
            // An item that does not carry the bullet is skipped, at every locus: the field is
            // absent in that instance and there is nothing to remap.
            let mut edits: Vec<(Vec<String>, String)> = Vec::new();
            for (chain, item) in items_at_locus(parsed, locus) {
                if let Some(f) = item.fields.iter().find(|f| f.key == field) {
                    let new_value = remap_value(&f.value, map).ok_or_else(|| unsupported(locus))?;
                    edits.push((chain, new_value));
                }
            }
            let mut out = source.to_string();
            for (chain, new_value) in edits {
                let chain: Vec<&str> = chain.iter().map(String::as_str).collect();
                // ONE primitive at every locus — the present-field **value-span** splice,
                // whose depth is carried by the chain. The insert-capable dual
                // ([`write::set_nested_item_field_or_insert`], which
                // [`apply_added_item_field`] must use because its bullet is absent by
                // definition) reaches this same splice first and falls through to an
                // in-group bullet insert; this kind never needs that fall-through, because
                // it overwrites a bullet that is already there.
                out = write::set_item_field(schema, &out, section, &chain, field, &new_value)?;
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
    use crate::schema_diff::{LOCI, schema_diff};
    use crate::validate::schema_conformance;
    use crate::write::{Instance, ItemContent, SectionContent, instance_from_source, render};

    /// A repeatable section's **item block** — locus 2, where every item-locus expectation
    /// below sits. Spelled out rather than `.into()` (which builds the section's own locus, 1)
    /// because the classifier now says which of the two a change was found at.
    fn item_locus(section: &str) -> crate::schema_diff::Locus {
        crate::schema_diff::Locus::at_item_block(section)
    }

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
                locus: "requirements".into()
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
                locus: "meta".into(),
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
                locus: "rationale".into(),
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
                locus: "rationale".into(),
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
                locus: "meta".into(),
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
                locus: "meta".into(),
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
                locus: "meta".into(),
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
                locus: "options".into(),
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
                locus: "tasks".into(),
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
                locus: item_locus("entries"),
                field: "kind".to_string(),
                map: BTreeMap::new(),
            }]
        );
        let map = BTreeMap::from([
            ("D".to_string(), "Decision".to_string()),
            ("I".to_string(), "Idea".to_string()),
        ]);
        let changes = vec![SchemaChange::ValueRemapped {
            locus: item_locus("entries"),
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
                locus: "meta".into(),
                field: "status".to_string(),
                map: BTreeMap::new(),
            }]
        );
        let map = BTreeMap::from([
            ("open".to_string(), "active".to_string()),
            ("closed".to_string(), "archived".to_string()),
        ]);
        let changes = vec![SchemaChange::ValueRemapped {
            locus: "meta".into(),
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
            locus: "meta".into(),
            field: "status".to_string(),
            map,
        }];

        assert_eq!(
            transform(&v1, &v2, &src, &changes),
            Err(TransformError::Unsupported {
                kind: "value-remapped",
                locus: "meta".into(),
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
                locus: "meta".into(),
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
            locus: "body".into(),
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
                locus: "meta".into(),
                field: "derived-from".to_string(),
            }],
            "a narrowing names itself — it is not the widening no-op"
        );

        assert_eq!(
            transform(&v1, &v2, &src, &diff),
            Err(TransformError::Unsupported {
                kind: "narrowed-cardinality",
                locus: "meta".into(),
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
                locus: "meta".into(),
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
                locus: "meta".into(),
                field: "derived-from".to_string(),
            }],
            "a dropped leaf names itself — it is not invisible, and not the backstop's residual"
        );

        assert_eq!(
            transform(&v1, &v2, &src, &diff),
            Err(TransformError::Unsupported {
                kind: "removed-field",
                locus: "meta".into(),
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
            locus: "vision".into(),
            leaf: Some("owner".to_string()),
        }];
        assert_eq!(
            transform(&v1, &v1, &src, &prose_needing_field),
            Err(TransformError::Unsupported {
                kind: "prose-needing",
                locus: "vision".into()
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
                locus: item_locus("entries"),
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
                locus: item_locus("entries"),
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
                locus: item_locus("entries"),
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

    // ---- (g') the same cell-set ONE LOCUS DOWN — `AddedItemField` at the nested item block ----

    /// v1 of a **nested** `deferrals`: each `entries` item carries a `trigger` field and nests
    /// a repeatable `notes` block — the shipped `changelog.releases/changes` shape, which is
    /// the only place the third locus exists in a real corpus. No leading bare-prose slot on
    /// the outer item: a slot there would swallow the nested headings.
    const NESTED_DEFERRALS_V1: &str = "\
type: deferrals
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - id: notes
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
              - { id: body, slot: { hint: \"the note\" } }
";

    /// The block line a cell's added leaf is appended after, per locus: the outer item block's
    /// last field at locus 2, the nested block's last leaf at the deepest one. Two anchors over
    /// **one** schema text, so the two loci differ by nothing but where the leaf lands.
    fn nested_deferrals_anchor(locus: usize) -> &'static str {
        if locus == LOCI {
            "              - { id: body, slot: { hint: \"the note\" } }\n"
        } else {
            "        - { id: trigger, type: string }\n"
        }
    }

    /// The [`crate::schema_diff::Locus`] a leaf appended at `locus` must be classified at.
    fn nested_deferrals_locus(locus: usize) -> crate::schema_diff::Locus {
        if locus == LOCI {
            item_locus("entries").nested_in("notes")
        } else {
            item_locus("entries")
        }
    }

    /// [`NESTED_DEFERRALS_V1`] with the leaf line `decl` appended at `locus`.
    fn nested_deferrals_v2(locus: usize, decl: &str) -> Schema {
        let anchor = nested_deferrals_anchor(locus);
        // The indentation is READ off the anchor, never restated: a literal run of spaces
        // here would drift the day the fixture is re-indented, and land the leaf at the
        // other locus while still asserting this one.
        let indent = &anchor[..anchor.len() - anchor.trim_start().len()];
        let yaml = NESTED_DEFERRALS_V1.replacen(anchor, &format!("{anchor}{indent}- {decl}\n"), 1);
        assert_ne!(
            yaml, NESTED_DEFERRALS_V1,
            "the fixture must declare the locus-{locus} anchor `{anchor}`",
        );
        load_schema(yaml.as_bytes())
            .unwrap_or_else(|err| panic!("the locus-{locus} v2 schema loads: {err}\n{yaml}"))
    }

    /// A canonical nested `deferrals`: **two** entries, each nesting **two** notes — so
    /// *every item at the locus* is a real claim at both loci, and the items at the *other*
    /// locus are the control that catches a splice sprayed at the wrong depth.
    /// `preauthored` puts `(key, value)` on the **first** item at `locus` (the
    /// already-carries fixture), and is rendered under the schema that declares it.
    fn nested_deferrals_doc(
        schema: &Schema,
        locus: usize,
        preauthored: Option<(&str, &str)>,
    ) -> String {
        let bullet = |carried: bool| match preauthored {
            Some((key, value)) if carried => vec![Field {
                key: key.to_string(),
                value: Value::Scalar(value.to_string()),
            }],
            _ => Vec::new(),
        };
        let note = |id: &str, title: &str, body: &str, carried: bool| ItemContent {
            id: id.to_string(),
            title: title.to_string(),
            slot: Some(body.to_string()),
            fields: bullet(carried),
            ..Default::default()
        };
        let entry =
            |id: &str, title: &str, trigger: &str, carried: bool, notes: Vec<ItemContent>| {
                let mut fields = vec![Field {
                    key: "trigger".to_string(),
                    value: Value::Scalar(trigger.to_string()),
                }];
                fields.extend(bullet(carried));
                ItemContent {
                    id: id.to_string(),
                    title: title.to_string(),
                    fields,
                    items: notes,
                    ..Default::default()
                }
            };
        let deep = locus == LOCI;
        let inst = Instance {
            title: "Deferral Ledger".to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items: vec![
                    entry(
                        "the-freeze-exempt-floor",
                        "The freeze-exempt floor",
                        "M39",
                        !deep,
                        vec![
                            note(
                                "the-owed-floor",
                                "The owed floor",
                                "Owed: a detect+route floor.",
                                deep,
                            ),
                            note(
                                "the-route",
                                "The route",
                                "Routed at the migrate door.",
                                false,
                            ),
                        ],
                    ),
                    entry(
                        "the-abandon-path",
                        "The abandon path",
                        "M42",
                        false,
                        vec![
                            note(
                                "the-record",
                                "The record",
                                "Parked: what abandon commits.",
                                false,
                            ),
                            note(
                                "the-gate",
                                "The gate",
                                "Parked: which gate adjudicates.",
                                false,
                            ),
                        ],
                    ),
                ],
                ..Default::default()
            }],
        };
        render(schema, &inst)
    }

    /// Every item at `locus`, in document order, paired with the value of the field `key`
    /// where that item carries it — so *every item* and *no other item* are both readable
    /// off one list.
    fn nested_deferrals_values(
        inst: &Instance,
        locus: usize,
        key: &str,
    ) -> Vec<(String, Option<String>)> {
        let entries = inst
            .sections
            .iter()
            .find(|s| s.id == "entries")
            .expect("entries present");
        let of = |item: &ItemContent| {
            (
                item.id.clone(),
                item.fields
                    .iter()
                    .find(|f| f.key == key)
                    .map(|f| match &f.value {
                        Value::Scalar(v) => v.clone(),
                        other => panic!("a scalar bullet; got {other:?}"),
                    }),
            )
        };
        if locus == LOCI {
            entries
                .items
                .iter()
                .flat_map(|entry| entry.items.iter().map(of))
                .collect()
        } else {
            entries.items.iter().map(of).collect()
        }
    }

    /// What the fold owes for one cell of the `AddedItemField` cell-set.
    enum Cell {
        /// A `default:`-carrying leaf: **every** item at the locus gains the bullet.
        Splices(&'static str),
        /// A leaf whose absence already conforms: **zero** bytes.
        ZeroBytes,
        /// A leaf with no deterministic value that the gate does ask for: the corpus fold
        /// halts and the doc stays byte-identical.
        Blocks,
        /// A `default:`-carrying leaf over a doc where the **first** item at the locus was
        /// hand-authored ahead of the bump: `(authored, default)` — the authored value
        /// survives, the rest get the default, and the fold converges.
        Preserves(&'static str, &'static str),
    }

    /// **The `AddedItemField` cell-set holds one locus down.** (M50 Increment 7 / T3.)
    ///
    /// The set is the one the shipped locus-2 suite above names — the deterministic-value
    /// splice, the conformance-clean zero-byte absence, the required-leaf block, and the
    /// already-carries no-data-loss/idempotency cell — and each cell is driven **at both
    /// loci in the same loop**, over one fixture that differs only in which block the leaf
    /// is appended to. Two claims a per-locus pair of tests could not make: the outcome at
    /// the deepest locus is the *same* outcome, and the items at the **other** locus are
    /// untouched — the control that catches a splice landing at the wrong depth.
    ///
    /// Red at HEAD on the two byte-writing cells: `apply_added_item_field` refused any
    /// nested locus outright (`Unsupported { kind: "added-item-field" }`), because the
    /// primitive it splices through addresses an item at the second locus only.
    #[test]
    fn the_added_item_field_cell_set_holds_one_locus_down() {
        let v1 = load_schema(NESTED_DEFERRALS_V1.as_bytes()).expect("the nested v1 loads");
        let cells: [(&str, &str, Cell); 4] = [
            (
                "defaulted",
                "{ id: kind, type: enum, of: [Decision, Idea], default: Decision }",
                Cell::Splices("Decision"),
            ),
            (
                "optional-no-default",
                "{ id: owner, type: string, optional: true }",
                Cell::ZeroBytes,
            ),
            (
                "required-no-default",
                "{ id: owner, type: string }",
                Cell::Blocks,
            ),
            (
                "already-carries",
                "{ id: kind, type: enum, of: [Decision, Idea], default: Decision }",
                Cell::Preserves("Idea", "Decision"),
            ),
        ];

        for (label, decl, cell) in cells {
            for locus in [2, LOCI] {
                let at = format!("{label} at locus {locus}");
                let v2 = nested_deferrals_v2(locus, decl);
                let src = match cell {
                    Cell::Preserves(authored, _) => {
                        nested_deferrals_doc(&v2, locus, Some(("kind", authored)))
                    }
                    _ => nested_deferrals_doc(&v1, locus, None),
                };

                // The real classifier emits the kind AND the locus; the driver is exercised on
                // the emitted classification, never a hand-built list.
                let diff = schema_diff(&v1, &v2);
                let expected = match cell {
                    Cell::Blocks => SchemaChange::ProseNeeding {
                        locus: nested_deferrals_locus(locus),
                        leaf: Some("owner".to_string()),
                    },
                    _ => SchemaChange::AddedItemField {
                        locus: nested_deferrals_locus(locus),
                        field: match cell {
                            Cell::ZeroBytes => "owner".to_string(),
                            _ => "kind".to_string(),
                        },
                    },
                };
                assert_eq!(diff, vec![expected], "{at}: classification");

                if let Cell::Blocks = cell {
                    let corpus = [CorpusDoc {
                        id: "deferrals-a",
                        old_schema: &v1,
                        new_schema: &v2,
                        source: &src,
                        changes: &diff,
                    }];
                    let result = migrate_corpus(&corpus);
                    assert_eq!(result.halted_at, Some(0), "{at}: the fold halts");
                    assert_eq!(
                        result.docs,
                        vec![DocOutcome::Untouched {
                            id: "deferrals-a".to_string(),
                            v0: src.clone(),
                        }],
                        "{at}: the blocked doc stays byte-identical",
                    );
                    continue;
                }

                let out = transform(&v1, &v2, &src, &diff)
                    .unwrap_or_else(|err| panic!("{at}: the fold must not refuse; got {err:?}"));
                assert_conforms(&v2, &out);
                assert_byte_stable(&v2, &out);
                assert_eq!(
                    transform(&v1, &v2, &src, &diff).expect("re-run"),
                    out,
                    "{at}: the fold is deterministic",
                );
                assert_eq!(
                    transform(&v1, &v2, &out, &diff).expect("re-fold"),
                    out,
                    "{at}: the fold converges",
                );

                let inst = instance_from_source(&v2, &out).expect("re-parse under v2");
                let other = if locus == LOCI { 2 } else { LOCI };
                match cell {
                    Cell::ZeroBytes => {
                        assert_eq!(out, src, "{at}: no item gains a fabricated bullet");
                    }
                    Cell::Splices(value) => {
                        for (id, got) in nested_deferrals_values(&inst, locus, "kind") {
                            assert_eq!(
                                got.as_deref(),
                                Some(value),
                                "{at}: every item at the locus carries the default; {id} does not",
                            );
                        }
                        // The byte claim: undo the added bullet and the source comes back.
                        // An item that carried NO fields gains the `<!-- fields -->` sentinel
                        // with it — a field group cannot exist without one — which is why the
                        // group form is undone first and the bare bullet second.
                        let group = format!("\n<!-- fields -->\n- kind: {value}\n");
                        let bullet = format!("- kind: {value}\n");
                        assert_eq!(
                            out.replace(&group, "").replace(&bullet, ""),
                            src,
                            "{at}: the defaulted field group is the migration's ONLY delta",
                        );
                    }
                    Cell::Preserves(authored, value) => {
                        let seen = nested_deferrals_values(&inst, locus, "kind");
                        assert_eq!(
                            seen.first().and_then(|(_, v)| v.as_deref()),
                            Some(authored),
                            "{at}: the hand-authored value survives; got {seen:?}",
                        );
                        for (id, got) in seen.iter().skip(1) {
                            assert_eq!(
                                got.as_deref(),
                                Some(value),
                                "{at}: the item that lacked the bullet gets the default; {id}",
                            );
                        }
                    }
                    Cell::Blocks => unreachable!("handled above"),
                }

                // The control: a splice at the wrong depth would land here too.
                for (id, got) in nested_deferrals_values(&inst, other, "kind") {
                    assert_eq!(
                        got, None,
                        "{at}: no item at locus {other} may gain the leaf; {id} did",
                    );
                }
            }
        }
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
                    locus: "meta".into(),
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
                    locus: item_locus("entries"),
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

    // ---- (f) `AddedItemSlot` — a repeatable item block gains prose (M49 Inc-4 T1) ----

    /// A `plan` doctype whose `milestones` item block declares **one** slot leaf (`proves`)
    /// beside a `trigger` field — the single-slot bare-prose arity every committed item is in
    /// before the bump.
    fn plan_one_slot() -> Schema {
        load_schema(
            b"\
type: plan
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: proves, slot: { hint: \"what it proves\" } }
        - { id: trigger, type: string }
",
        )
        .expect("plan (one slot) loads")
    }

    /// The same block with a **second, optional** slot leaf appended — the 1→2 arity, the cell
    /// where the sub-labels are minted and the committed bare prose is re-keyed under the v1
    /// leaf's own `#### <Leaf-Title>`.
    fn plan_two_slots(optional: bool) -> Schema {
        let block = if optional {
            "        - { id: detail, slot: { optional: true, hint: \"the detail\" } }\n"
        } else {
            "        - { id: detail, slot: { hint: \"the detail\" } }\n"
        };
        load_schema(
            format!(
                "\
type: plan
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - {{ id: title, type: string }}
        - {{ id: proves, slot: {{ hint: \"what it proves\" }} }}
{block}        - {{ id: trigger, type: string }}
"
            )
            .as_bytes(),
        )
        .expect("plan (two slots) loads")
    }

    /// A canonical `plan` carrying **two** items (so "every item" is a real claim, not a sample
    /// of one), rendered through the writer under `schema` — so the fixture is the shipped
    /// canonical form, never a hand-typed approximation of it.
    fn plan_doc(schema: &Schema, slots: [&[(&str, &str)]; 2]) -> String {
        let item = |id: &str, title: &str, trigger: &str, leaves: &[(&str, &str)]| ItemContent {
            id: id.to_string(),
            title: title.to_string(),
            slot: match leaves {
                [(_, prose)] => Some((*prose).to_string()),
                _ => None,
            },
            slots: match leaves {
                [_] => Vec::new(),
                many => many
                    .iter()
                    .map(|(id, prose)| ((*id).to_string(), (*prose).to_string()))
                    .collect(),
            },
            fields: vec![Field {
                key: "trigger".to_string(),
                value: Value::Scalar(trigger.to_string()),
            }],
            items: Vec::new(),
        };
        render(
            schema,
            &Instance {
                title: "Plan".to_string(),
                sections: vec![SectionContent {
                    id: "milestones".to_string(),
                    items: vec![
                        item("the-loop", "The loop", "M1", slots[0]),
                        item("the-fan-out", "The fan-out", "M2", slots[1]),
                    ],
                    ..Default::default()
                }],
            },
        )
    }

    /// **The classifier seam — an added item slot names itself.** Before M49 the item-block
    /// loop classified `Field` leaves only, so this diffed to the `Unclassified` residual and
    /// the migration refused with a route naming `crates/engine/src/transform.rs` — a file no
    /// adopter can edit.
    #[test]
    fn an_added_item_slot_classifies_at_every_arity() {
        let one = plan_one_slot();
        let two = plan_two_slots(true);
        assert_eq!(
            schema_diff(&one, &two),
            vec![SchemaChange::AddedItemSlot {
                locus: item_locus("milestones"),
                leaf: "detail".to_string(),
            }],
            "1→2: the added slot leaf classifies"
        );

        // 0→1: a slot-less item block gains its first prose leaf.
        let none = load_schema(
            b"\
type: plan
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
",
        )
        .expect("plan (no slot) loads");
        let first = load_schema(
            b"\
type: plan
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: proves, slot: { optional: true, hint: \"what it proves\" } }
        - { id: trigger, type: string }
",
        )
        .expect("plan (first slot) loads");
        assert_eq!(
            schema_diff(&none, &first),
            vec![SchemaChange::AddedItemSlot {
                locus: item_locus("milestones"),
                leaf: "proves".to_string(),
            }],
            "0→1: the added slot leaf classifies too — the arity the report's `detail` needs"
        );

        // 0→1 is a **byte no-op**: a lone slot renders bare, so a slot-less item already
        // carries the v2 bytes.
        let src = plan_doc(&none, [&[], &[]]);
        let out = transform(&none, &first, &src, &schema_diff(&none, &first))
            .expect("the 0→1 fold is a no-op, not a refusal");
        assert_eq!(out, src, "0→1 writes zero bytes");
        assert_conforms(&first, &out);
        assert_byte_stable(&first, &out);
    }

    /// **The driver seam at 1→2 — the committed prose moves under the v1 leaf's sub-label and
    /// the new leaf mints empty.** No prose is invented, none is lost, and every other byte
    /// (the item anchors, the `trigger` bullets, the section heading) survives.
    #[test]
    fn an_added_item_slot_relabels_the_committed_prose_and_mints_the_new_leaf() {
        let one = plan_one_slot();
        let two = plan_two_slots(true);
        let src = plan_doc(
            &one,
            [
                &[("proves", "The loop closes.")],
                &[("proves", "The fan-out joins.")],
            ],
        );
        let diff = schema_diff(&one, &two);
        let out = transform(&one, &two, &src, &diff).expect("the 1→2 fold succeeds");

        // The oracle is the writer's own v2 canonical form for the same content — so a
        // byte-faithful relabel reproduces it exactly, and the empty `detail` is minted at its
        // schema-ordered offset rather than appended anywhere convenient.
        let oracle = plan_doc(
            &two,
            [
                &[("proves", "The loop closes."), ("detail", "")],
                &[("proves", "The fan-out joins."), ("detail", "")],
            ],
        );
        assert_eq!(out, oracle, "the fold lands the canonical v2 bytes");
        assert!(
            out.contains("#### Proves\n\nThe loop closes.\n\n#### Detail"),
            "the committed bare prose rides verbatim under the v1 leaf's sub-label; got:\n{out}"
        );
        assert_conforms(&two, &out);
        assert_byte_stable(&two, &out);

        // Determinism, and the re-run: folding the already-migrated bytes changes nothing.
        assert_eq!(
            transform(&one, &two, &src, &diff).expect("re-run succeeds"),
            out,
            "the item-slot reshape is deterministic"
        );
    }

    /// **The per-item re-run guard.** One doc can hold an item that already carries the added
    /// sub-label beside one that does not — a re-run after a refused commit, or a hand-authored
    /// entry. The already-reshaped item is left **byte-identical**; the other is reshaped.
    /// A whole-change filter cannot express this, which is why the guard lives per item.
    #[test]
    fn an_item_already_carrying_the_added_sub_label_is_left_byte_identical() {
        let one = plan_one_slot();
        let two = plan_two_slots(true);
        // Item 1 already renders both sub-labels (a v2-shaped item); item 2 is still bare.
        let mixed = plan_doc(
            &one,
            [
                &[(
                    "proves",
                    "#### Proves\n\nThe loop closes.\n\n#### Detail\n\nAlready reshaped.",
                )],
                &[("proves", "The fan-out joins.")],
            ],
        );
        let already = mixed
            .split("### The fan-out")
            .next()
            .expect("the first item's bytes")
            .to_string();

        let diff = schema_diff(&one, &two);
        let out = transform(&one, &two, &mixed, &diff).expect("the mixed fold succeeds");
        assert!(
            out.starts_with(&already),
            "the already-reshaped item keeps its bytes; got:\n{out}"
        );
        assert!(
            out.contains("#### Proves\n\nThe fan-out joins.\n\n#### Detail"),
            "the un-reshaped item is reshaped; got:\n{out}"
        );
        assert_conforms(&two, &out);
        assert_byte_stable(&two, &out);
    }

    /// **2→3 — the guard keys on the ADDED leaf, not on "any declared sub-label."** Every item
    /// already carries both of the old shape's sub-labels while still needing the new one, so an
    /// any-sub-label guard would skip exactly the items this change exists to reshape and the
    /// fold's output would fail to parse. Each existing sub-label's bytes are untouched.
    #[test]
    fn a_two_to_three_add_mints_the_new_leaf_with_the_existing_ones_untouched() {
        let two = plan_two_slots(true);
        let three = load_schema(
            b"\
type: plan
sections:
  - id: milestones
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: proves, slot: { hint: \"what it proves\" } }
        - { id: detail, slot: { optional: true, hint: \"the detail\" } }
        - { id: risk, slot: { optional: true, hint: \"the risk\" } }
        - { id: trigger, type: string }
",
        )
        .expect("plan (three slots) loads");
        let src = plan_doc(
            &two,
            [
                &[
                    ("proves", "The loop closes."),
                    ("detail", "Two increments."),
                ],
                &[("proves", "The fan-out joins."), ("detail", "One join.")],
            ],
        );

        let diff = schema_diff(&two, &three);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedItemSlot {
                locus: item_locus("milestones"),
                leaf: "risk".to_string(),
            }]
        );
        let out = transform(&two, &three, &src, &diff).expect("the 2→3 fold succeeds");
        assert_eq!(
            out,
            plan_doc(
                &three,
                [
                    &[
                        ("proves", "The loop closes."),
                        ("detail", "Two increments."),
                        ("risk", ""),
                    ],
                    &[
                        ("proves", "The fan-out joins."),
                        ("detail", "One join."),
                        ("risk", ""),
                    ],
                ]
            ),
            "the new leaf mints empty and every existing sub-label survives byte-for-byte"
        );
        assert_conforms(&three, &out);
        assert_byte_stable(&three, &out);
    }

    /// **A REQUIRED added item slot is adjudicated by the per-doc gate, never refused by the
    /// driver.** The byte work is identical to the optional cell — one relabel, not two kinds —
    /// and the empty required leaf then breaks the conformance gate, so the doc rolls back
    /// byte-identical and the caller renders the doc-authorable Framing-A route
    /// ([`HaltReason::Gate`]) rather than a build instruction.
    #[test]
    fn a_required_added_item_slot_halts_at_the_gate_not_at_the_driver() {
        let one = plan_one_slot();
        let two = plan_two_slots(false);
        let src = plan_doc(
            &one,
            [
                &[("proves", "The loop closes.")],
                &[("proves", "The fan-out joins.")],
            ],
        );
        let diff = schema_diff(&one, &two);
        assert_eq!(
            diff,
            vec![SchemaChange::AddedItemSlot {
                locus: item_locus("milestones"),
                leaf: "detail".to_string(),
            }],
            "requiredness does not split the kind — the relabel is the same byte work"
        );

        let corpus = [CorpusDoc {
            id: "plan-a",
            old_schema: &one,
            new_schema: &two,
            source: &src,
            changes: &diff,
        }];
        let result = migrate_corpus(&corpus);
        assert_eq!(result.halted_at, Some(0));
        assert!(
            matches!(result.halt_reason, Some(HaltReason::Gate(_))),
            "the gate adjudicates it, so the caller's route is the authorable one; got {:?}",
            result.halt_reason
        );
        assert_eq!(
            result.docs,
            vec![DocOutcome::Untouched {
                id: "plan-a".to_string(),
                v0: src.clone(),
            }],
            "the doc rolls back byte-identical — the stamp never moves"
        );
    }

    // ---- (h') the same cell-set ONE LOCUS DOWN — `ValueRemapped` at the nested item block ----

    /// v1 of a **nested** `ledger`: the enum `kind` is declared at **both** item loci — the
    /// outer `entries` block and the nested `notes` one. That is the shipped
    /// `changelog.releases` collision (`date` declared outside and, in the nested-`changes`
    /// reshapes, inside) turned onto the kind this arm folds, and it is what makes the
    /// *other* locus a control: a remap that resolved its declaration by section id, or
    /// sprayed by field key over the whole section, rewrites values it does not own and the
    /// assertions below read it off the bytes.
    ///
    /// The section/nesting shape is [`NESTED_DEFERRALS_V1`]'s (`entries` → `notes`), so
    /// [`nested_deferrals_locus`] and [`nested_deferrals_values`] address this fixture too —
    /// one reader per question, not one per fixture.
    const NESTED_LEDGER_V1: &str = "\
type: ledger
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [D, I] }
        - id: notes
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
              - { id: kind, type: enum, of: [D, I] }
              - { id: body, slot: { hint: \"the note\" } }
";

    /// The `kind` declaration line the rename is made in, per locus — the outer block's at
    /// locus 2, the nested block's at the deepest one. The two differ **only** by
    /// indentation, so the pair of v2 schemas differ by nothing but which block moved.
    fn nested_ledger_anchor(locus: usize) -> &'static str {
        if locus == LOCI {
            "              - { id: kind, type: enum, of: [D, I] }\n"
        } else {
            "        - { id: kind, type: enum, of: [D, I] }\n"
        }
    }

    /// [`NESTED_LEDGER_V1`] with the `kind` enum members **renamed** at `locus` and left
    /// alone at the other one.
    fn nested_ledger_v2(locus: usize) -> Schema {
        let anchor = nested_ledger_anchor(locus);
        let yaml =
            NESTED_LEDGER_V1.replacen(anchor, &anchor.replace("[D, I]", "[Decision, Idea]"), 1);
        assert_ne!(
            yaml, NESTED_LEDGER_V1,
            "the fixture must declare the locus-{locus} anchor `{anchor}`",
        );
        assert_eq!(
            yaml.matches("[Decision, Idea]").count(),
            1,
            "exactly one of the two declarations may move; got:\n{yaml}",
        );
        load_schema(yaml.as_bytes())
            .unwrap_or_else(|err| panic!("the locus-{locus} v2 schema loads: {err}\n{yaml}"))
    }

    /// A canonical nested `ledger`: **two** entries, each carrying its own `kind` and nesting
    /// **two** notes carrying theirs — so *every item at the locus* is a real claim at both
    /// loci, both committed members are exercised at both, and the items at the *other* locus
    /// are the control that catches a splice landing at the wrong depth.
    fn nested_ledger_doc(schema: &Schema) -> String {
        let kind = |v: &str| Field {
            key: "kind".to_string(),
            value: Value::Scalar(v.to_string()),
        };
        let note = |id: &str, title: &str, body: &str, k: &str| ItemContent {
            id: id.to_string(),
            title: title.to_string(),
            slot: Some(body.to_string()),
            fields: vec![kind(k)],
            ..Default::default()
        };
        let entry = |id: &str, title: &str, k: &str, notes: Vec<ItemContent>| ItemContent {
            id: id.to_string(),
            title: title.to_string(),
            fields: vec![kind(k)],
            items: notes,
            ..Default::default()
        };
        let inst = Instance {
            title: "Deferral Ledger".to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items: vec![
                    entry(
                        "cache-the-index",
                        "Cache the index",
                        "D",
                        vec![
                            note(
                                "the-owed-floor",
                                "The owed floor",
                                "Owed: a detect+route floor.",
                                "D",
                            ),
                            note("the-route", "The route", "Routed at the migrate door.", "I"),
                        ],
                    ),
                    entry(
                        "a-plugin-surface",
                        "A plugin surface",
                        "I",
                        vec![
                            note(
                                "the-record",
                                "The record",
                                "Parked: what abandon commits.",
                                "I",
                            ),
                            note(
                                "the-gate",
                                "The gate",
                                "Parked: which gate adjudicates.",
                                "D",
                            ),
                        ],
                    ),
                ],
                ..Default::default()
            }],
        };
        render(schema, &inst)
    }

    /// The authored old→new map covering **both** committed members.
    fn ledger_full_map() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("D".to_string(), "Decision".to_string()),
            ("I".to_string(), "Idea".to_string()),
        ])
    }

    /// Undo the remapped value tokens in `out`. Equality with the source after this is the
    /// whole-document byte claim: the fold moved the value bytes and **nothing else**.
    fn undo_ledger_remap(out: &str) -> String {
        out.replace("kind: Decision", "kind: D")
            .replace("kind: Idea", "kind: I")
    }

    /// **The `ValueRemapped` cell-set holds one locus down.** (M50 Increment 7 / T5.)
    ///
    /// The set is the one the shipped locus-1/locus-2 tests above name — the map that covers
    /// every committed value remaps **every** item byte-faithfully, an **uncovered** value
    /// blocks loudly, and the fold is deterministic — and each cell is driven **at both item
    /// loci in the same loop**, over one fixture whose two v2 schemas differ by nothing but
    /// which block the rename was made in. Two claims a per-locus pair of tests could not
    /// make: the outcome at the deepest locus is the *same* outcome, and the items at the
    /// **other** locus are untouched — the control that catches a splice at the wrong depth.
    ///
    /// Red at HEAD: `locus_disposition(ValueRemapped, 3)` was `Unbuilt`, so `transform`
    /// refused the nested cell before reaching the arm at all.
    #[test]
    fn the_value_remap_cell_set_holds_one_locus_down() {
        let v1 = load_schema(NESTED_LEDGER_V1.as_bytes()).expect("the nested v1 loads");
        let src = nested_ledger_doc(&v1);
        let before = instance_from_source(&v1, &src).expect("the v1 source parses");
        let full = ledger_full_map();

        for locus in [2, LOCI] {
            let v2 = nested_ledger_v2(locus);
            let at = nested_deferrals_locus(locus);
            let other = if locus == LOCI { 2 } else { LOCI };

            // The **real classifier** names the block the rename was made in — the premise
            // the arm below rests on, asserted rather than assumed.
            assert_eq!(
                schema_diff(&v1, &v2),
                vec![SchemaChange::ValueRemapped {
                    locus: at.clone(),
                    field: "kind".to_string(),
                    map: BTreeMap::new(),
                }],
                "locus {locus}: the rename classifies at exactly the block it was made in",
            );

            // The fixture is a real claim at this locus: more than one item, and both
            // committed members present, so a map exercised one way only cannot pass.
            let committed = nested_deferrals_values(&before, locus, "kind");
            assert_eq!(
                committed.len(),
                if locus == LOCI { 4 } else { 2 },
                "locus {locus}: every-item is a claim over more than one item",
            );
            assert_eq!(
                committed
                    .iter()
                    .filter_map(|(_, v)| v.clone())
                    .collect::<std::collections::BTreeSet<_>>(),
                std::collections::BTreeSet::from(["D".to_string(), "I".to_string()]),
                "locus {locus}: both committed members are exercised",
            );

            // ---- cell 1: a covering map remaps every item at the locus, byte-faithfully.
            let changes = vec![SchemaChange::ValueRemapped {
                locus: at.clone(),
                field: "kind".to_string(),
                map: full.clone(),
            }];
            let out = transform(&v1, &v2, &src, &changes)
                .unwrap_or_else(|err| panic!("locus {locus}: the remap folds; got {err:?}"));
            assert_conforms(&v2, &out);
            assert_byte_stable(&v2, &out);
            assert_ne!(
                out, src,
                "locus {locus}: the fold rewrites the committed values",
            );
            // THE WHOLE-DOCUMENT BYTE CLAIM, not the field line's: undoing the value tokens
            // must restore the source byte-for-byte. A primitive that re-renders the item
            // (rather than splicing the value span) can pass a field-line assertion while
            // dropping, re-ordering or re-canonicalizing everything around it.
            assert_eq!(
                undo_ledger_remap(&out),
                src,
                "locus {locus}: the value bytes are the only bytes the fold may move",
            );

            let inst = instance_from_source(&v2, &out).expect("the migrated bytes re-parse");
            assert_eq!(
                nested_deferrals_values(&inst, locus, "kind"),
                committed
                    .iter()
                    .map(|(id, v)| (id.clone(), v.as_ref().map(|v| full[v].clone())))
                    .collect::<Vec<_>>(),
                "locus {locus}: every item at the locus carries its remapped value",
            );
            assert_eq!(
                nested_deferrals_values(&inst, other, "kind"),
                nested_deferrals_values(&before, other, "kind"),
                "locus {locus}: no item at locus {other} is touched",
            );

            // ---- cell 2: an uncovered committed value blocks loudly, naming this locus.
            let partial = BTreeMap::from([("D".to_string(), "Decision".to_string())]);
            assert_eq!(
                transform(
                    &v1,
                    &v2,
                    &src,
                    &[SchemaChange::ValueRemapped {
                        locus: at.clone(),
                        field: "kind".to_string(),
                        map: partial,
                    }]
                ),
                Err(TransformError::Unsupported {
                    kind: "value-remapped",
                    locus: at.clone(),
                }),
                "locus {locus}: a value the authored map does not cover blocks, naming the \
                 locus the map has to be keyed at",
            );

            // ---- cell 3: determinism.
            assert_eq!(
                transform(&v1, &v2, &src, &changes).expect("the re-run folds"),
                out,
                "locus {locus}: the value-remap fold is deterministic",
            );
        }
    }

    /// **A nested item's committed bytes the parse does not model survive the remap.**
    /// (M50 Increment 7 / T5 — the plan's red-step assumption 2, proven rather than trusted.)
    ///
    /// jigc's canonical item order is slots-then-fields, so *"append a sentence to this
    /// note"* lands **after** the `<!-- fields -->` group, where the parse carries nothing —
    /// reachable by hand, and reachable at this locus precisely because a nested block that
    /// declares a field is what a value remap needs. The item-locus arm splices the **value
    /// span**, so those bytes cannot move; a nested arm that re-rendered the whole item from
    /// what the parse modelled would delete them at exit 0, on a *migration*, which is the
    /// No-data-loss property the pair declares.
    #[test]
    fn a_nested_value_remap_keeps_committed_bytes_the_parse_does_not_model() {
        let v1 = load_schema(NESTED_LEDGER_V1.as_bytes()).expect("the nested v1 loads");
        let v2 = nested_ledger_v2(LOCI);
        let clean = nested_ledger_doc(&v1);

        // Hand-append an aside to the FIRST nested item, immediately after its field group
        // and before the next `#### ` heading — the one spot the canonical form leaves for it.
        let head = clean
            .find("{#the-owed-floor}")
            .expect("the first nested item is present");
        let next = head
            + clean[head..]
                .find("\n#### ")
                .expect("a following nested item bounds the first");
        let aside = "A hand-appended aside the schema does not model.\n";
        // Separated from the group by a blank line: glued straight onto it the parser reads
        // it as a malformed field bullet, which is a *malformed* doc, not an unmodelled one.
        let src = format!("{}\n{aside}{}", &clean[..next], &clean[next..]);
        assert!(
            parse_sections(&v1, &src).is_ok(),
            "the aside parses — it is unmodelled, not malformed; got {:?}\nsrc:\n{src}",
            parse_sections(&v1, &src).err(),
        );

        let out = transform(
            &v1,
            &v2,
            &src,
            &[SchemaChange::ValueRemapped {
                locus: nested_deferrals_locus(LOCI),
                field: "kind".to_string(),
                map: ledger_full_map(),
            }],
        )
        .expect("the nested remap folds over a doc carrying unmodelled bytes");

        assert!(
            out.contains(aside),
            "the hand-appended aside must survive the fold; got:\n{out}",
        );
        assert_eq!(
            undo_ledger_remap(&out),
            src,
            "the value bytes are the only bytes the fold may move — including around \
             content the parse does not model",
        );
    }

    // ---- (h'') the same axis crossed with the field's ROLE — the `id-from` cell ----

    /// v1 of a ledger whose item blocks are keyed **by the enum itself**: `id-from: kind` at
    /// both item loci, so each item's heading *is* its committed enum value and its `{#id}`
    /// anchor is slugged from it. That is the shipped `changelog` change-group
    /// (`id-from: category` over `[added, changed, …]`, included at the staging section and
    /// again inside every release), which is why the role is reachable at **both** loci
    /// rather than manufactured for one.
    ///
    /// The members are slug-form on purpose: an `id-from` heading's committed value is
    /// `slug(title)` ([`crate::validate::id_from_enum_violation`]), so a `CamelCase` member
    /// would make the fixture non-conformant rather than give it a member to carry.
    const ID_FROM_LEDGER_V1: &str = "\
type: ledger
sections:
  - id: entries
    repeatable:
      id-from: kind
      block:
        - { id: kind, type: enum, of: [decision, idea] }
        - id: notes
          repeatable:
            id-from: kind
            block:
              - { id: kind, type: enum, of: [decision, idea] }
              - { id: body, slot: { hint: \"the note\" } }
";

    /// The `kind` declaration line the rename is made in, per locus — the outer block's at
    /// locus 2, the nested block's at the deepest one. The two differ **only** by
    /// indentation, exactly as [`nested_ledger_anchor`]'s pair does.
    fn id_from_ledger_anchor(locus: usize) -> &'static str {
        if locus == LOCI {
            "              - { id: kind, type: enum, of: [decision, idea] }\n"
        } else {
            "        - { id: kind, type: enum, of: [decision, idea] }\n"
        }
    }

    /// [`ID_FROM_LEDGER_V1`] with the `kind` enum's `decision` member **renamed** at `locus`
    /// and left alone at the other one. `idea` is deliberately kept: a rename that moved
    /// every member would leave no way to author a doc that carries nothing orphaned, and
    /// that doc is half the claim below.
    fn id_from_ledger_v2(locus: usize) -> Schema {
        let anchor = id_from_ledger_anchor(locus);
        let yaml = ID_FROM_LEDGER_V1.replacen(
            anchor,
            &anchor.replace("[decision, idea]", "[call, idea]"),
            1,
        );
        assert_ne!(
            yaml, ID_FROM_LEDGER_V1,
            "the fixture must declare the locus-{locus} anchor `{anchor}`",
        );
        assert_eq!(
            yaml.matches("[call, idea]").count(),
            1,
            "exactly one of the two declarations may move; got:\n{yaml}",
        );
        load_schema(yaml.as_bytes())
            .unwrap_or_else(|err| panic!("the locus-{locus} v2 schema loads: {err}\n{yaml}"))
    }

    /// An [`ID_FROM_LEDGER_V1`] instance: one outer item per `(kind, nested kinds)` pair,
    /// each item's heading being the committed enum value itself.
    fn id_from_ledger_doc(schema: &Schema, entries: &[(&str, &[&str])]) -> String {
        let item = |kind: &str, notes: Vec<ItemContent>| ItemContent {
            id: crate::slug::slugify(kind),
            title: kind.to_string(),
            items: notes,
            ..Default::default()
        };
        let inst = Instance {
            title: "Ledger".to_string(),
            sections: vec![SectionContent {
                id: "entries".to_string(),
                items: entries
                    .iter()
                    .map(|(kind, notes)| {
                        item(
                            kind,
                            notes
                                .iter()
                                .map(|note| ItemContent {
                                    slot: Some(format!("The {note} note.")),
                                    ..item(note, Vec::new())
                                })
                                .collect(),
                        )
                    })
                    .collect(),
                ..Default::default()
            }],
        };
        render(schema, &inst)
    }

    /// **The `ValueRemapped` axis is the field's ROLE crossed with the locus — and the
    /// `id-from` role refuses.** (M52 Increment 7 / T3; `completions/artifacts/M52` →
    /// settle-record D4.4, gap-findings G-19, baseline-freeze §2.3 rows D3d/D3b and §4 L-4.)
    ///
    /// A block's `id-from` leaf is **not a committed bullet**: the parser consumes it as the
    /// item's heading and the `{#id}` anchor is slugged from it. So this arm's per-item
    /// lookup found nothing on any item, wrote **zero bytes**, and returned the source — and
    /// the caller's conformance gate then broke on the heading the rename had just orphaned,
    /// routing the doc at *"author the new required prose … then re-run"*, which no prose
    /// clears and no re-run changes. The comment that stood here asserted the converse
    /// (*"the enum field is never the id-from"*) rather than checking it.
    ///
    /// The axis is the **role**, not the locus, which is why the plain half runs in the same
    /// loop over the same two loci: a covering map rewrites a committed bullet byte-faithfully
    /// at both, so nothing about the locus explains the id-from outcome.
    ///
    /// The refusal is **scoped to the docs the rename actually orphans**: an instance whose
    /// every committed identity is still a declared member has nothing to rewrite, and
    /// blocking it would dead-end an adopter over a doc that already conforms.
    ///
    /// Red at HEAD: both `id-from` cells returned `Ok(src)` — a silent zero-byte fold — so the
    /// refusal this asserts did not exist.
    #[test]
    fn the_value_remap_axis_discriminates_the_field_role_at_every_item_locus() {
        let plain_v1 = load_schema(NESTED_LEDGER_V1.as_bytes()).expect("the plain v1 loads");
        let plain_src = nested_ledger_doc(&plain_v1);
        let id_from_v1 = load_schema(ID_FROM_LEDGER_V1.as_bytes()).expect("the id-from v1 loads");
        // Every item at BOTH loci carries `decision` — the member each bump renames away — so
        // each locus's cell is a real claim rather than an accident of which item sits where.
        let orphaned = id_from_ledger_doc(
            &id_from_v1,
            &[
                ("decision", &["decision", "idea"]),
                ("idea", &["decision", "idea"]),
            ],
        );
        // …and this one carries `idea` at both loci: the rename touches nothing it holds.
        let conformant = id_from_ledger_doc(&id_from_v1, &[("idea", &["idea"])]);
        let renamed = BTreeMap::from([("decision".to_string(), "call".to_string())]);

        for locus in [2, LOCI] {
            let at = nested_deferrals_locus(locus);

            // ---- the PLAIN role: the enum is a committed bullet, and a covering map
            //      rewrites every one of them byte-faithfully. The control that makes the
            //      cell below a statement about the role and not about the locus.
            let plain_v2 = nested_ledger_v2(locus);
            let plain_out = transform(
                &plain_v1,
                &plain_v2,
                &plain_src,
                &[SchemaChange::ValueRemapped {
                    locus: at.clone(),
                    field: "kind".to_string(),
                    map: ledger_full_map(),
                }],
            )
            .unwrap_or_else(|err| panic!("locus {locus}: the plain remap folds; got {err:?}"));
            assert_conforms(&plain_v2, &plain_out);
            assert_ne!(
                plain_out, plain_src,
                "locus {locus}: the plain fold rewrites the committed values",
            );
            assert_eq!(
                undo_ledger_remap(&plain_out),
                plain_src,
                "locus {locus}: the value bytes are the only bytes the plain fold may move",
            );

            // ---- the ID-FROM role: the same kind, the same locus, and a map that COVERS the
            //      committed value — and it still refuses, because the value is the identity.
            let id_from_v2 = id_from_ledger_v2(locus);
            assert_eq!(
                schema_diff(&id_from_v1, &id_from_v2),
                vec![SchemaChange::ValueRemapped {
                    locus: at.clone(),
                    field: "kind".to_string(),
                    map: BTreeMap::new(),
                }],
                "locus {locus}: the id-from rename classifies at the block it was made in",
            );
            let change = |map: BTreeMap<String, String>| {
                vec![SchemaChange::ValueRemapped {
                    locus: at.clone(),
                    field: "kind".to_string(),
                    map,
                }]
            };
            for (tag, map) in [
                ("a covering map", renamed.clone()),
                ("no authored map", BTreeMap::new()),
            ] {
                assert_eq!(
                    transform(&id_from_v1, &id_from_v2, &orphaned, &change(map)),
                    Err(TransformError::IdFromRemap {
                        locus: at.clone(),
                        field: "kind".to_string(),
                    }),
                    "locus {locus}: with {tag}, an id-from rename refuses for the role — the \
                     map gap is a different question and would not help if it were closed",
                );
            }

            // ---- …and it refuses ONLY what the rename orphans: a doc whose every committed
            //      identity is still a declared member folds to a byte no-op and restamps.
            assert_eq!(
                transform(
                    &id_from_v1,
                    &id_from_v2,
                    &conformant,
                    &change(renamed.clone())
                ),
                Ok(conformant.clone()),
                "locus {locus}: a doc the rename orphans nothing in already conforms",
            );
        }
    }

    /// **A nested item's committed bytes the parse does not model survive the field add.**
    /// (M50 Increment 7 fix — the sibling of
    /// [`a_nested_value_remap_keeps_committed_bytes_the_parse_does_not_model`], on the kind
    /// that actually *inserts* a bullet.)
    ///
    /// The 4-cell `the_added_item_field_cell_set_holds_one_locus_down` loop feeds only
    /// canonical, fully-modelled item bodies, so it could not see this: the nested arm
    /// dispatched to a primitive that re-rendered the item's whole committed region from
    /// what the parse modelled, and a paragraph hand-appended after a nested item's
    /// `<!-- fields -->` group was gone at `Ok` — on a *migration*, at exit 0, with the
    /// deletion committed. That is the **No-data-loss** property this pair declares by name.
    ///
    /// The fixture is the ledger's, because the shape needs a nested block that already
    /// carries a field bullet: only then is there a `<!-- fields -->` group for a hand-append
    /// to land after, which is the one spot the canonical form leaves unmodelled.
    #[test]
    fn a_nested_added_item_field_keeps_committed_bytes_the_parse_does_not_model() {
        let v1 = load_schema(NESTED_LEDGER_V1.as_bytes()).expect("the nested v1 loads");
        // The added leaf joins the NESTED block — the locus this arm reaches, appended after
        // its last declared leaf exactly as `nested_deferrals_v2` does.
        let anchor = nested_deferrals_anchor(LOCI);
        let indent = &anchor[..anchor.len() - anchor.trim_start().len()];
        let yaml = NESTED_LEDGER_V1.replacen(
            anchor,
            &format!("{anchor}{indent}- {{ id: owner, type: string, default: unassigned }}\n"),
            1,
        );
        assert_ne!(
            yaml, NESTED_LEDGER_V1,
            "the fixture must declare the nested anchor `{anchor}`",
        );
        let v2 = load_schema(yaml.as_bytes()).expect("the nested v2 loads");

        // Hand-append an aside to the FIRST nested item, after its field group and before
        // the next `#### ` heading — separated by a blank line, so it is unmodelled rather
        // than a malformed field bullet.
        let clean = nested_ledger_doc(&v1);
        let head = clean
            .find("{#the-owed-floor}")
            .expect("the first nested item is present");
        let next = head
            + clean[head..]
                .find("\n#### ")
                .expect("a following nested item bounds the first");
        let aside = "An aside a human appended by hand.\n";
        let src = format!("{}\n{aside}{}", &clean[..next], &clean[next..]);
        assert!(
            parse_sections(&v1, &src).is_ok(),
            "the aside parses — it is unmodelled, not malformed; got {:?}\nsrc:\n{src}",
            parse_sections(&v1, &src).err(),
        );

        // The real classifier, never a hand-built list.
        let changes = crate::schema_diff::schema_diff(&v1, &v2);
        assert_eq!(
            changes,
            vec![SchemaChange::AddedItemField {
                locus: nested_deferrals_locus(LOCI),
                field: "owner".to_string(),
            }],
            "the fixture pair classifies as the nested added-item-field",
        );

        let out = transform(&v1, &v2, &src, &changes)
            .expect("the nested field add folds over a doc carrying unmodelled bytes");

        assert!(
            out.contains(aside),
            "the hand-appended aside must survive the fold; got:\n{out}",
        );
        assert_eq!(
            out.replace("- owner: unassigned\n", ""),
            src,
            "the generated bullets are the only bytes the fold may add — including around \
             content the parse does not model",
        );
        assert_eq!(
            out.matches("- owner: unassigned").count(),
            4,
            "every nested item at the locus gains the bullet; got:\n{out}",
        );
    }
}
