//! The schema-diff classifier — a v1→v2 delta over two [`Schema`] values.
//!
//! Step 1 of the deterministic transform (`design/corpus-migration.md` → The
//! deterministic transform): classify each change between an old and a new
//! doc-type [`Schema`] into one of the four kinds the transform driver knows how
//! to apply. The driver (M34 Inc-3 for `added-optional-field`; Inc-2 T3 for the
//! structural kinds) consumes these classifications and emits byte-stable splices
//! over the `write.rs` primitives — **no LLM in the structural path** (the
//! determinism boundary: the CLI owns structure).
//!
//! This module is *classification only* — it compares two already-loaded schemas
//! and never touches an instance. It is presentation-free and domain-empty (the
//! engine invariant). Changes outside the four supported kinds (a removed
//! section/leaf, a type change, a narrowed cardinality — all breaking or
//! data-losing, none a supported transform) are **not** classified here; the
//! conformance detector and the transform gate adjudicate those.

use crate::schema::{Field, Leaf, Schema, SectionBody};
use std::collections::{BTreeMap, HashMap};

/// One classified change between an old and a new [`Schema`], by kind.
///
/// Each variant names the section (and, where applicable, the leaf) it concerns —
/// the address the transform driver splices at. The four kinds mirror
/// `design/corpus-migration.md` → The deterministic transform (1. Schema-diff).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SchemaChange {
    /// A field the transform can place **deterministically**: it is either
    /// `optional` (its absence never blocks) or carries a deterministic value
    /// source (`default`/`set`, e.g. the schema-version stamp's `set` deriver).
    /// The driver branch + the "current active schema version" deriver land in
    /// M34 Inc-3 (the live stamp dogfood — the branch-split); only the
    /// *classification* lives here.
    AddedOptionalField {
        /// The section the field was added to.
        section: String,
        /// The added field's id.
        field: String,
    },

    /// A field present in both schemas whose `card` (forward cardinality) was **widened** —
    /// the new bound admits every value the old one did (`0..1` → `0..*`), so every committed
    /// instance is still conformant and the fold is a **byte no-op**. Fired at **both loci** —
    /// a simple/header section's fields *and* a repeatable item block's (M42: the item locus
    /// classified nothing at all, so the authoring matrix's ✅ for an item-block widen was a
    /// promise the classifier never honoured).
    ///
    /// A `card` delta in the **other** direction is [`Self::NarrowedCardinality`], not this:
    /// direction is classified, never assumed (`design/corpus-migration.md` → The two silent-
    /// classification holes).
    WidenedCardinality {
        /// The section carrying the field.
        section: String,
        /// The field whose cardinality changed.
        field: String,
    },

    /// A field present in both schemas whose `card` was **narrowed** — the new bound is
    /// *tighter* on either end (`0..*` → `0..1`, `0..*` → `1..*`), so a committed instance may
    /// carry more values than it now admits, or lack one it now demands. **Content-affecting,
    /// never a no-op** — and **refused**: [`crate::transform::transform`] surfaces
    /// [`crate::transform::TransformError::Unsupported`] and `jigc migrate-corpus` blocks the
    /// doc with its own route (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T3 — the recorded pick).
    ///
    /// The refusal is the *pick*, not a gap in the classifier: nothing in the system counts a
    /// committed instance's edges against a `card` bound (`schema_conformance` → `check_field`
    /// type-checks each scalar and never counts), so the design's alternative arm — *validate
    /// every committed instance against the new bound* — is net-new validation surface. Refusing
    /// is strictly better than the pre-M42 behaviour, which classified **any** `card` delta as a
    /// widening and folded a narrowing to zero bytes, restamping the corpus **past the gate**.
    NarrowedCardinality {
        /// The section carrying the field.
        section: String,
        /// The field whose cardinality was narrowed.
        field: String,
    },

    /// An **enum widening**: a field present in both schemas whose `of:` member set **grew**
    /// (`new.of ⊇ old.of`, e.g. `[active, joined]` → `[active, joined, discarded]`). Every
    /// committed value is still a declared member, so the fold is a **byte no-op** (the
    /// [`Self::WidenedCardinality`] sibling) and **no map is authored** — the kind *deletes* the
    /// need for identity-map entries rather than growing the authored table.
    ///
    /// Only a **non-superset** `of:` delta (a genuine rename, or a member drop) stays
    /// [`Self::ValueRemapped`]. Pre-M42 *any* `of:` delta classified as a rename, so a widening
    /// emitted `ValueRemapped { map: {} }` and the driver blocked the doc on its first committed
    /// value — with no map to author, because nothing was renamed
    /// (`design/corpus-migration.md` → The classifier's holes).
    EnumWidened {
        /// The section carrying the enum field.
        section: String,
        /// The enum field whose member set grew.
        field: String,
    },

    /// An **enum member rename**: a field present in both schemas whose `enum` `of:`
    /// member set changed (`[D, I]` → `[Decision, Idea]`) — the **first parameterized
    /// transform kind**. Applies to a **simple** field (`write::set_field`) and a
    /// **repeatable-item** field (`write::set_item_field`).
    ///
    /// The old→new `map` is **authored** — a CLI-supplied migration input — because an
    /// enum rename is *unrecoverable from the schema pair alone*: the classifier sees only
    /// *that* the member set changed, never *which* old value maps onto *which* new one. So
    /// the classifier emits this variant carrying an **empty** map (it detects the delta);
    /// the CLI fills the map before the driver folds it (`design/corpus-migration.md` → the
    /// value-remap kind, the structural-auto vs value-semantic-authored distinction). The
    /// determinism *boundary* holds — the map is a deterministic CLI input, not an LLM call.
    ValueRemapped {
        /// The section carrying the enum field.
        section: String,
        /// The enum field whose member set was renamed.
        field: String,
        /// The authored old→new value map (**empty** as emitted by the classifier; the CLI
        /// supplies the mapping before the driver applies it).
        map: BTreeMap<String, String>,
    },

    /// A field added to a **repeatable item block** — the item-block twin of
    /// [`Self::AddedOptionalField`], and the locus where the work-doc family's content
    /// actually lives (9 of 14 persisted doctypes carry a repeatable; `decisions-log` *is* one
    /// repeatable, so before M42 **zero** content changes to it were migratable).
    ///
    /// Emitted for a leaf the conformance gate never asks a doc for — exactly the negation of
    /// [`crate::validate::is_author_required`], [`classify_added_field`]'s predicate; a leaf it
    /// *does* ask for, with no value source, is [`Self::ProseNeeding`] `{ leaf: Some }` instead.
    /// The driver's two arms mirror that split (`design/corpus-migration.md` → The classifier's
    /// holes: `AddedItemField`, the per-item semantics):
    ///
    /// - **`default:` present** → the value is spliced into **every item that lacks the
    ///   bullet**, through [`crate::write::set_item_field_or_insert`] — the **insert-capable**
    ///   primitive (`set_item_field` is *update-only* and refuses an absent bullet, which is every
    ///   item by definition of this kind). A caller-threaded deriver rides this arm: it hands its
    ///   value in *as* a `default` (`with_stamp_default`), and M46 Inc-4 left that untouched.
    /// - **an absence that already conforms** — `optional:`, an optional `ref`, a pack-declared
    ///   type, or a `set:` carrying no `default:` — → a byte **no-op**: an item without the bullet
    ///   **already conforms**, and inventing an empty bullet would fabricate a value. It still
    ///   names itself — the backstop requires that of every real change — and folds to zero bytes
    ///   (the [`Self::WidenedCardinality`] sibling); the derived member is reported as unfilled
    ///   rather than left silent (M46 Inc-4).
    ///
    /// Pre-M42 the item-block loop emitted [`Self::ValueRemapped`] and nothing else, so an added
    /// item field diffed to `[]`: an optional one "migrated by accident" (the stamp flipped and a
    /// declared `default:` silently never landed), a required one was a **permanent mutual dead
    /// end** (`validate` said *run the migration*, `migrate-corpus` said *author the prose*).
    AddedItemField {
        /// The repeatable section whose item block gained the field.
        section: String,
        /// The added field's id.
        field: String,
    },

    /// A **slot leaf** added to a repeatable item block — the leaf-kind twin of
    /// [`Self::AddedItemField`], and the kind that makes *item prose* reachable at all: before
    /// M49 the item-block loop classified `Field` leaves only, so a doctype whose entries needed
    /// a prose leaf had **no legal path at any arity**, and the backstop's route named a file in
    /// the jigc source tree — which is not a bound on an adopter's lock-in cost but the absence
    /// of one (`design/corpus-migration.md` → The classifier's holes;
    /// `completions/artifacts/M49/settle-record.md` → D3(B)).
    ///
    /// **One kind for both requirednesses, and the per-doc gate adjudicates.** The byte work is
    /// identical either way — the item's committed prose moves under the old leaf's
    /// `#### <Leaf-Title>` sub-label and the new leaf mints empty — so splitting it across two
    /// kinds would split one relabel in half. An **optional** added slot leaves every item
    /// conformant and the doc migrates; a **required** one leaves the minted leaf empty, the
    /// conformance gate breaks on it, and `migrate-corpus` routes the doc at
    /// `migrate-corpus.prose-needed` — the doc-authorable Framing-A handoff, never a
    /// build-the-kind route ([`crate::transform::HaltReason::Gate`]).
    ///
    /// The driver folds it through [`crate::write::insert_item_slot`], which reads the committed
    /// prose through the **old** schema (a v1-shaped instance does not parse under a v2
    /// multi-slot template) and writes **zero bytes** below two declared slots, where a slot
    /// renders bare under no sub-heading.
    ///
    /// The kind exists rather than riding the [`Self::Unclassified`] residual for the reason
    /// [`Self::RemovedField`] records: the backstop is a **residual**, so an added slot arriving
    /// alongside any classified change would leave the diff non-empty and be silently dropped.
    AddedItemSlot {
        /// The repeatable section whose item block gained the slot leaf.
        section: String,
        /// The added slot leaf's id.
        leaf: String,
    },

    /// A wholly-new **optional** slot section in `v2` — an added `## Heading` whose
    /// body is a simple `slot: { optional: true }` (the adr `options` shape). The
    /// driver mints the empty `## Heading` at its schema-ordered offset; an empty
    /// optional slot conforms, so no prose is needed (unlike [`Self::ProseNeeding`]).
    /// It is **not** a byte no-op: the v2 writer emits the heading unconditionally, so
    /// a historical doc lacking it is non-canonical until the heading is spliced in
    /// (`design/corpus-migration.md` → The deterministic transform, 2. added-optional-
    /// section).
    AddedOptionalSection {
        /// The added optional slot section's id.
        section: String,
    },

    /// A wholly-new **repeatable** section in `v2` — an added `## Heading` whose body is an
    /// item block. The driver mints the empty `## Heading` at its schema-ordered offset through
    /// the **same block-insert** [`Self::AddedOptionalSection`] uses, and stops: a **zero-item
    /// repeatable conforms** (`schema-conformance.repeatable-populated` is a store *advisory*,
    /// never a conformance break — `design/validation.md` → Repeatable-section conformance), so
    /// no prose is needed and the doc is done (unlike [`Self::ProseNeeding`]).
    ///
    /// It is **not** a byte no-op, for the [`Self::AddedOptionalSection`] reason: the v2 writer
    /// emits every schema section's heading unconditionally, so a historical doc lacking it is
    /// non-canonical until the heading is spliced in. Before M42 this shape classified **nothing**
    /// (`added_section` handled only the simple-slot cases), so it rode the [`Self::Unclassified`]
    /// residual and the migration refused — a doctype could not grow a repeatable section at all
    /// (`design/corpus-migration.md` → The classifier's holes).
    AddedRepeatableSection {
        /// The added repeatable section's id.
        section: String,
    },

    /// A section that was a simple `<<slot>>` becoming a repeatable item-block —
    /// the riskiest net-new transform (the old slot content becomes the default
    /// first item; `design/corpus-migration.md`). Only emitted when the old
    /// section actually carried a slot (the physical prose to promote).
    FixedSlotToRepeatable {
        /// The section promoted from simple-slot to repeatable.
        section: String,
    },

    /// A new **required** slot or field with **no deterministic default** — the
    /// Framing-A escape hatch: the transform mints it empty and routes it to the
    /// agent, which authors the prose; the CLI conformance-gates the result. A
    /// section-level slot is anonymous, so `leaf` is `None` for it and
    /// `Some(<field-id>)` for a required-without-default field.
    ProseNeeding {
        /// The section the new required prose belongs to.
        section: String,
        /// The leaf id for a required field; `None` for a section's own slot.
        leaf: Option<String>,
    },

    /// A **doctype-level home change**: the committed instances' resolved home moved
    /// (`design/storage.md` → Placement; `design/corpus-migration.md` → Relocation). A
    /// **file move, not a content edit** — the bytes are byte-identical at the new home,
    /// so the transform fold is a content no-op (the [`Self::WidenedCardinality`]
    /// sibling); the CLI migrate-corpus arm performs the git-free `fs::rename`. `from` /
    /// `to` are the two schemas' declared homes (`placement.file` else `location`),
    /// auto-derived from the [`Schema`] values alone — engine-pure, no cascade/docs-root
    /// (the determinism boundary: no hand-written move recipe). Classified **outside**
    /// the per-section loop, because `location` / `placement` are `Schema`-level fields
    /// the section-diff never inspects (a pure relocation would otherwise diff to `[]`).
    Relocated {
        /// The v1 home (its declared `placement.file` else `location`).
        from: String,
        /// The v2 home (its declared `placement.file` else `location`).
        to: String,
    },

    /// A **doctype-level `display-title` add/change**: the doc's `# H1` line is rewritten
    /// to the new display text (`# changelog` → `# Changelog`; `design/corpus-migration.md`
    /// → Relocation). A one-line H1 splice, byte-stable otherwise. Classified **outside**
    /// the per-section loop, because `display-title` is a `Schema`-level field the
    /// section-diff never inspects.
    DisplayTitleChanged {
        /// The new H1 display text.
        to: String,
    },

    /// An **`optional` relaxation** on a leaf present in both schemas (`optional: false → true`,
    /// on a section's slot or on a field): every doc conformant under the strict rule is
    /// conformant under the loose one, so the fold is a **byte no-op** (the
    /// [`Self::WidenedCardinality`] sibling — classify → fold to zero bytes → restamp → ship).
    ///
    /// It is a *named kind* rather than a dropped delta because `optional` is **inside** the
    /// conformance-relevant projection (a committed doc's bytes *can* violate requiredness — see
    /// the tightening below), so the empty-diff backstop would otherwise **refuse** it: a
    /// relaxation — the one direction that cannot break a single doc — would be unshippable
    /// (`design/corpus-migration.md` → The structural projection: the two flag deltas get named
    /// kinds).
    ///
    /// The **other** direction (`optional: true → false`, a *tightening*) is not this kind: it can
    /// render a currently-conformant doc non-conformant, so it classifies [`Self::ProseNeeding`]
    /// (or, when the leaf is not thereby author-required, [`Self::PresentationOnly`]).
    OptionalRelaxed {
        /// The section carrying the leaf.
        section: String,
        /// The field id; `None` for a section's own (anonymous) slot.
        leaf: Option<String>,
    },

    /// A leaf **declared in the old schema and dropped in the new one** — at either locus (a
    /// simple/header section's fields, a repeatable item block's). Every committed instance may
    /// still carry the field line, which the new schema no longer declares.
    ///
    /// **Refused, and that is the recorded pick** (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T5;
    /// `design/corpus-migration.md` → The two silent-classification holes, which left the shape
    /// open): [`crate::transform::transform`] surfaces [`crate::transform::TransformError::Unsupported`]
    /// and `jigc migrate-corpus` blocks the doc with its own route. The alternative — a strip arm
    /// splicing the field line away — is deterministic but **destroys the committed values**, a
    /// knowing exception to **No-data-loss**, a *declared* property of this pair (the property
    /// census); no frozen doctype needs a removal, so building the strip is premature generality.
    /// The refusal is **not a one-way door**: a strip arm with a deliberate data-loss opt-in stays
    /// purely additive if a real driver appears.
    ///
    /// The kind exists — rather than leaving the removal to the [`Self::Unclassified`] backstop —
    /// because the backstop is the **residual**: a removal riding *alongside* a classified change
    /// leaves the diff non-empty, so the backstop never fires and the removal would be **silently
    /// dropped**.
    RemovedField {
        /// The section that declared the dropped leaf.
        section: String,
        /// The dropped leaf's field id.
        field: String,
    },

    /// A **slot leaf** the old schema declared in a repeatable item block and the new one
    /// **drops** — the leaf-kind twin of [`Self::RemovedField`], and the [`Self::AddedItemSlot`]
    /// direction the item filter still dropped on the floor after M49 T1.
    ///
    /// **Refused, on [`Self::RemovedField`]'s recorded reasoning and one sharper than it**
    /// (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T5, the pick — *refuse, not strip*): what a strip
    /// arm would splice away here is **authored prose**, not a field line, so the exception to
    /// **No-data-loss** — a declared property of this pair — would be the larger one.
    /// [`crate::transform::transform`] surfaces
    /// [`crate::transform::TransformError::Unsupported`]; `jigc migrate-corpus` blocks the doc
    /// before the fold with a **schema-authoring** route naming the dropped *slot*. Additive to
    /// build later, exactly as the field strip is.
    ///
    /// The kind exists rather than riding the [`Self::Unclassified`] residual for the reason
    /// [`Self::RemovedField`] records: the backstop is a **residual**, so a removal arriving
    /// alongside any classified change leaves the diff non-empty and would be **silently
    /// dropped** — the doc restamped while every item still carries prose under a sub-label the
    /// schema no longer declares.
    RemovedItemSlot {
        /// The repeatable section whose item block dropped the slot leaf.
        section: String,
        /// The dropped slot leaf's id.
        leaf: String,
    },

    /// A change **outside the conformance-relevant structural projection** — a delta a
    /// committed doc's bytes *cannot* violate, so it is a **byte no-op** (the
    /// [`Self::WidenedCardinality`] / [`Self::Relocated`] shape: classify → fold to zero
    /// bytes → restamp → ship). It is emitted rather than dropped because the empty-diff
    /// backstop below requires **every** real change to name itself: without this kind an
    /// authoring-hint typo fix or a `default:` add would be an *unclassified* change and
    /// would be **falsely refused, i.e. unshippable** (`design/corpus-migration.md` → The
    /// structural projection).
    ///
    /// **The name is narrower than the extent** (kept for continuity with the Settle
    /// record): read it as *outside-the-projection* — it covers presentation (`description`
    /// / `usage` / a slot's `hint`) **and** semantics (`default` / `set` / `inverse` /
    /// `inverse-card` / `check` / `title-names-symbol`) alike, because the property that
    /// matters is not "is it cosmetic" but "**can a committed doc's bytes violate it**" —
    /// and for these, they cannot. The enumeration lives in [`erase_out_of_projection`].
    PresentationOnly,

    /// **The empty-diff backstop**: the doctype's conformance-relevant structural projection
    /// **moved**, but no transform kind classified it (`design/corpus-migration.md` → The
    /// empty-diff backstop — no silent bump). An unclassified change is *not* a no-op: the
    /// migration would stamp the corpus at the new version while leaving every instance
    /// non-conformant — **silently**, the exact strand both M41 and M42 paid to learn.
    ///
    /// So it names itself, and the consumer must **refuse**: [`crate::transform::transform`]
    /// surfaces [`crate::transform::TransformError::Unclassified`] (never a silent skip), and
    /// `jigc migrate-corpus` — the only surface that loads the prior snapshot this diff needs
    /// — **refuses the migration** at classification time and routes to *build the transform
    /// kind first*. It is a **migration refusal, not a build error**: a bumped version + a
    /// recomputed hash passes the pack-load freeze assert clean, so the author learns at the
    /// migration run.
    ///
    /// **Bound (by design):** the backstop catches an *otherwise-empty* diff. A projection
    /// move that rides **alongside** a classified kind is not caught here — which is why each
    /// named shape gets its own kind rather than relying on this signal.
    Unclassified,
}

/// Classify every supported change from `v1` to `v2` into a deterministic,
/// document-order list of [`SchemaChange`]s. An identical pair yields an empty
/// diff. Output order follows `v2`'s section/leaf document order (a stable
/// property of the [`Schema`] model), so the same pair always diffs identically.
pub fn schema_diff(v1: &Schema, v2: &Schema) -> Vec<SchemaChange> {
    let mut out = Vec::new();

    // Doctype-level changes first, classified **outside** the per-section loop —
    // `location` / `placement` / `display-title` are `Schema`-level fields the
    // section-diff never inspects, so a pure relocation or display-title change would
    // otherwise diff to `[]` and silently no-op (`design/corpus-migration.md` → 1.
    // Schema-diff: the two M38 doctype-level kinds).
    if let (Some(from), Some(to)) = (resolved_home(v1), resolved_home(v2))
        && from != to
    {
        out.push(SchemaChange::Relocated { from, to });
    }
    if let Some(to) = &v2.display_title
        && v1.display_title.as_deref() != Some(to.as_str())
    {
        out.push(SchemaChange::DisplayTitleChanged { to: to.clone() });
    }

    let old_sections: HashMap<&str, &SectionBody> = v1
        .sections
        .iter()
        .map(|s| (s.id.as_str(), &s.body))
        .collect();

    for section in &v2.sections {
        match old_sections.get(section.id.as_str()) {
            Some(old) => diff_section(&section.id, old, &section.body, &mut out),
            None => added_section(&section.id, &section.body, &mut out),
        }
    }

    // THE RESIDUAL — the empty-diff backstop and its false-refusal guard
    // (`design/corpus-migration.md` → The empty-diff backstop / The structural projection).
    // Reached only when **no kind classified**, because a classified kind already satisfies
    // the backstop's requirement that a real change name itself (the accepted bound: the
    // backstop catches an *otherwise-empty* diff, which is why each named shape carries its
    // own kind). Two cases remain, and they are opposites:
    //
    // - the **conformance-relevant projection moved** with nothing to apply ⇒ an unclassified
    //   change: refuse (an empty diff is NOT a no-op — the caller would stamp-bump a corpus
    //   that fails its own gate);
    // - the projection is **identical** but the schemas differ ⇒ the delta lies entirely
    //   outside the projection (a hint reword, a `default:` add, an `inverse:`/`check:` edit):
    //   a byte no-op that must still be *named*, or the backstop would refuse it and a frozen
    //   doctype could never fix a typo in an authoring hint.
    //
    // Identical schemas fall through both, yielding the empty diff (nothing changed at all).
    if out.is_empty() {
        if erase_out_of_projection(v1) != erase_out_of_projection(v2) {
            out.push(SchemaChange::Unclassified);
        } else if v1 != v2 {
            out.push(SchemaChange::PresentationOnly);
        }
    }
    out
}

/// The schema's **conformance-relevant structural projection** — a clone with every
/// *out-of-projection* key erased, so two schemas' projections compare equal exactly when no
/// committed doc's bytes can tell them apart (`design/corpus-migration.md` → The structural
/// projection: the enumerated table).
///
/// **The enumeration is stated as its complement, deliberately.** The design's table lists the
/// projection (the section set · their ordering · slot presence · requiredness · the field set ·
/// a field's type, incl. a `ref`'s `to:` · enum members · cardinality) *and* the keys outside it
/// (`description` · `usage` · a slot's `hint` · `default` · `set` · `inverse` · `inverse-card` ·
/// `check` · `title-names-symbol`). Erasing the **outside** set — rather than rebuilding the
/// inside one — makes every schema key that exists now or is **added later** part of the
/// projection **by default**: an un-swept new key then refuses loudly (a false *refusal*, which
/// an author sees at the migration run) instead of silently stranding a corpus (a false *ship*,
/// which nobody sees). That is the safe direction of the very failure this backstop exists to
/// close — the un-enumerated sibling (`DECISIONS.md` → 2026-07-13 M42 Inc 5 T1).
fn erase_out_of_projection(schema: &Schema) -> Schema {
    let mut out = schema.clone();
    // Doctype-level authored prose (`describe` projects it; no instance byte carries it).
    out.description = None;
    out.usage = None;
    for section in &mut out.sections {
        match &mut section.body {
            SectionBody::Simple { slot, fields } => {
                if let Some(slot) = slot {
                    slot.hint = None;
                }
                for field in fields {
                    erase_field(field);
                }
            }
            SectionBody::Repeatable { repeatable } => erase_block(&mut repeatable.block),
        }
    }
    out
}

/// Erase the out-of-projection keys of a repeatable item block's leaves — the **second locus**
/// (the item block is where the work-doc family's content lives), recursing into a nested
/// repeatable so no leaf escapes the projection at any depth.
fn erase_block(block: &mut [Leaf]) {
    for leaf in block {
        match leaf {
            Leaf::Slot { slot, .. } => slot.hint = None,
            Leaf::Field(field) => erase_field(field),
            Leaf::Repeatable { repeatable, .. } => erase_block(&mut repeatable.block),
        }
    }
}

/// Erase one field's out-of-projection keys: the mint-time value sources (`default` / `set` — a
/// doc lacking the value already conforms), the derived back-edge metadata (`inverse` /
/// `inverse-card` — a store advisory, never a per-doc conformance break), and the `doc-code`
/// selectors (`check` / `title-names-symbol` — adjudicated against *code*, not the doc's bytes).
/// What survives is exactly what a committed doc's bytes can violate: the id, the type (incl. a
/// `ref`'s `to:`), the enum members, the cardinality, and the requiredness flag.
fn erase_field(field: &mut Field) {
    field.default = None;
    field.set = None;
    field.inverse = None;
    field.inverse_card = None;
    field.check = None;
    field.title_names_symbol = false;
}

/// A doctype's declared home: its `placement.file` literal (a placement doctype) else
/// its `location` directory. `None` for a transient doctype with neither (e.g. `commit`,
/// whose sink is the git message). Engine-pure — the raw schema-declared home, with **no**
/// `docs-root` resolution (that is the CLI's concern; the determinism boundary keeps the
/// engine domain-empty). Representation (dir vs full path) is executor latitude the
/// migrate-corpus move-arm resolves.
fn resolved_home(schema: &Schema) -> Option<String> {
    schema
        .placement
        .as_ref()
        .map(|p| p.file.clone())
        .or_else(|| schema.location.clone())
}

/// Diff a section present in both schemas.
fn diff_section(id: &str, old: &SectionBody, new: &SectionBody, out: &mut Vec<SchemaChange>) {
    match (old, new) {
        // simple → repeatable: only a *promotion of an existing slot* is the
        // supported `fixed-slot→repeatable-with-default` transform.
        (SectionBody::Simple { slot: Some(_), .. }, SectionBody::Repeatable { .. }) => {
            out.push(SchemaChange::FixedSlotToRepeatable {
                section: id.to_owned(),
            });
        }
        (
            SectionBody::Simple {
                slot: old_slot,
                fields: old_fields,
            },
            SectionBody::Simple {
                slot: new_slot,
                fields: new_fields,
            },
        ) => {
            match (old_slot, new_slot) {
                // A newly-introduced required slot needs prose (the slot is anonymous →
                // `leaf: None`). An added *optional* slot leaves existing docs conformant, so it
                // needs no transform.
                (None, Some(s)) if !s.optional => out.push(SchemaChange::ProseNeeding {
                    section: id.to_owned(),
                    leaf: None,
                }),
                // THE SLOT-LEVEL `optional` FLAG DELTA (the sixth hole), through the **one**
                // slot rule both loci now share ([`slot_flag_change`]) — a section's slot is
                // anonymous, so it names no leaf. Before T4 the delta was invisible here and
                // fell through to the backstop's residual; before M49 Inc-4 T2 the *item* locus
                // still was, which is what made the rule worth having one home.
                (Some(old), Some(new)) if old.optional != new.optional => {
                    out.push(slot_flag_change(id, None, new.optional));
                }
                _ => {}
            }
            diff_fields(id, old_fields, new_fields, out);
        }
        // Both repeatable: the item locus, whose whole leaf surface [`diff_item_fields`]
        // classifies — an added or removed field or slot, an existing leaf's `card` / `of` /
        // `optional:` delta, and a nested-repeatable delta named as the backstop kind.
        (
            SectionBody::Repeatable {
                repeatable: old_rep,
            },
            SectionBody::Repeatable {
                repeatable: new_rep,
            },
        ) => {
            diff_item_fields(id, &old_rep.block, &new_rep.block, out);
        }
        // Any other shape change (repeatable→simple, slotless simple→repeatable,
        // removed leaves) is breaking/unsupported — not classified here.
        _ => {}
    }
}

/// Classify the deltas of a **leaf present in both schemas** — the `card` direction and the
/// enum `of:` direction — in document order (`card` first, then `of`). The **one** existing-leaf
/// rule, shared by both loci: a simple/header section's fields ([`diff_fields`]) *and* a
/// repeatable item block's ([`diff_item_fields`]). Pre-M42 the two loops disagreed — the simple
/// one read `card` direction-blind, the item one read neither — which is why the rule lives in
/// one function now (`design/corpus-migration.md` → The two silent-classification holes: *both
/// loops iterate `new`'s leaves and inspect only `card` + `of`*).
fn diff_leaf(section: &str, old: &Field, new: &Field, out: &mut Vec<SchemaChange>) {
    let (id, field) = (section.to_owned(), new.id.clone());
    if old.optional != new.optional {
        out.push(optional_flag_change(section, new));
    }
    if old.card != new.card {
        out.push(if widens_card(old.card.as_deref(), new.card.as_deref()) {
            SchemaChange::WidenedCardinality {
                section: id.clone(),
                field: field.clone(),
            }
        } else {
            SchemaChange::NarrowedCardinality {
                section: id.clone(),
                field: field.clone(),
            }
        });
    }
    if let (Some(old_of), Some(new_of)) = (&old.of, &new.of)
        && old_of != new_of
    {
        // `new.of ⊇ old.of` — every committed value is still a declared member (a widening, and
        // a byte no-op). Anything else (a rename, a member drop) is value-semantic: the CLI
        // authors the old→new map.
        out.push(if old_of.iter().all(|member| new_of.contains(member)) {
            SchemaChange::EnumWidened { section: id, field }
        } else {
            SchemaChange::ValueRemapped {
                section: id,
                field,
                map: BTreeMap::new(),
            }
        });
    }
}

/// Classify an `optional`-flag delta on a **field** present in both schemas — the sixth hole
/// (`design/corpus-migration.md` → The structural projection: *Inside the projection, the two flag
/// deltas get named kinds*). The flag is **inside** the conformance-relevant projection, so a
/// delta here must name itself or the backstop refuses it.
///
/// - **`false → true` (a relaxation)** — every doc conformant under the strict rule is conformant
///   under the loose one: [`SchemaChange::OptionalRelaxed`], a byte no-op.
/// - **`true → false` (a tightening)** — a **real** [`SchemaChange::ProseNeeding`] *iff the leaf
///   thereby becomes author-required*: a committed doc that legitimately omits the field now
///   breaks `required-field-present`, so it routes to the agent (Framing A).
/// - **a tightening on a leaf that is *not* thereby author-required** — a `default:`/`set:`-carrying
///   field, an optional `ref`, a pack-declared type: [`SchemaChange::PresentationOnly`], the
///   out-of-projection no-op kind. **Blocking it would be a false refusal**, and the predicate is
///   not re-derived here: it is [`crate::validate::is_author_required`] — *the very predicate the
///   conformance gate's absent-field arm consults* — so the classifier's verdict and the gate's
///   can never drift (the design's flat "a tighten is prose-needing" rule, refined against the
///   gate that actually adjudicates it).
fn optional_flag_change(section: &str, new: &Field) -> SchemaChange {
    if new.optional {
        SchemaChange::OptionalRelaxed {
            section: section.to_owned(),
            leaf: Some(new.id.clone()),
        }
    } else if crate::validate::is_author_required(new) {
        SchemaChange::ProseNeeding {
            section: section.to_owned(),
            leaf: Some(new.id.clone()),
        }
    } else {
        SchemaChange::PresentationOnly
    }
}

/// Whether the `new` cardinality bound **admits every value** the `old` one did — the byte-no-op
/// direction ([`SchemaChange::WidenedCardinality`]). True iff `new.min ≤ old.min` **and**
/// `new.max ≥ old.max` (with `*` the greatest upper bound); an equal-bounds respelling (`card:`
/// absent — the `"0..1"` default — spelled out explicitly) is therefore a widening, i.e. the byte
/// no-op it in fact is, and never a false refusal.
///
/// Anything else is a **tightening** — a lower bound raised, an upper bound lowered, or a bound
/// this function cannot parse (an unparseable form reads as *not* a widening, so it refuses
/// loudly rather than folding silently — the safe direction of the very failure the backstop
/// exists to close).
fn widens_card(old: Option<&str>, new: Option<&str>) -> bool {
    let (Some((old_min, old_max)), Some((new_min, new_max))) = (card_bounds(old), card_bounds(new))
    else {
        return false;
    };
    let admits_max = match (new_max, old_max) {
        // `*` (unbounded) admits any old upper bound, including another `*`.
        (None, _) => true,
        // A bounded new max cannot admit an unbounded old one.
        (Some(_), None) => false,
        (Some(new_max), Some(old_max)) => new_max >= old_max,
    };
    new_min <= old_min && admits_max
}

/// Parse a declared `card:` into `(min, max)` — `max = None` meaning unbounded (`*`). The four
/// declared forms are `"0..1"` / `"1"` / `"0..*"` / `"1..*"`, and an **absent** `card:` is the
/// `"0..1"` default (`design/document-type-schema.md` → the schema keys; `schema.rs` →
/// `Field::card`). An unparseable form yields `None` — the caller treats that as *not* a
/// widening.
fn card_bounds(card: Option<&str>) -> Option<(u32, Option<u32>)> {
    let raw = card.unwrap_or("0..1");
    let raw = raw.trim();
    match raw.split_once("..") {
        // The bare form (`"1"`) is the closed interval `[n, n]`.
        None => raw.parse::<u32>().ok().map(|n| (n, Some(n))),
        Some((min, max)) => {
            let min = min.trim().parse::<u32>().ok()?;
            let max = match max.trim() {
                "*" => None,
                bounded => Some(bounded.parse::<u32>().ok()?),
            };
            Some((min, max))
        }
    }
}

/// Diff a repeatable section's item-block leaves — **every leaf kind, at both passes**.
///
/// The added pass walks `new`'s document order: an added `Field` leaf
/// ([`classify_added_item_field`] — the item locus of the add rule), an added `Slot` leaf
/// ([`SchemaChange::AddedItemSlot`]), a leaf present in both under the same existing-leaf rule
/// the simple locus applies ([`diff_leaf`] for a field's `card`/`of` direction, and
/// [`slot_flag_change`] for a slot's `optional:` delta — the one rule, at the one place, for
/// both loci). The removed pass then walks `old`'s document order, kind by kind: the fields
/// `new` drops ([`removed_fields`] — the removal rule stays *shared* with the simple locus
/// rather than re-implemented here), then its slots ([`SchemaChange::RemovedItemSlot`]).
///
/// A **nested repeatable** leaf — added, dropped, or reshaped inside the conformance-relevant
/// projection — has no transform kind built for it, so it emits [`SchemaChange::Unclassified`]
/// **explicitly**. That is the whole of its handling and deliberately so (M49 Inc-4 T2): the
/// backstop is a *residual*, which fires only over an otherwise-empty diff, so a nested delta
/// riding alongside any classified change was **silently dropped** and the doc restamped. Naming
/// itself makes it refuse in every diff, and building `AddedNestedRepeatable` stays a separate,
/// still-deferred decision (`completions/artifacts/M49/settle-record.md` → T1-2). The comparison
/// is over the **erased projection** ([`projected_leaf`]), so a hint reword inside a nested block
/// stays the out-of-projection no-op it is instead of becoming a false refusal.
///
/// Nothing is dropped on the floor here any more — every leaf kind, in both directions, either
/// classifies a kind or names the backstop (`design/corpus-migration.md`:188 — *the kind must
/// exist rather than ride the backstop*).
fn diff_item_fields(section: &str, old: &[Leaf], new: &[Leaf], out: &mut Vec<SchemaChange>) {
    let old_fields: Vec<&Field> = old.iter().filter_map(item_field).collect();
    let old_by_id: HashMap<&str, &Field> = old_fields.iter().map(|f| (f.id.as_str(), *f)).collect();
    let old_slots: HashMap<&str, &Leaf> = old
        .iter()
        .filter_map(|leaf| item_slot_id(leaf).map(|id| (id, leaf)))
        .collect();
    let old_nested: HashMap<&str, &Leaf> = old
        .iter()
        .filter_map(|leaf| item_nested_id(leaf).map(|id| (id, leaf)))
        .collect();
    for leaf in new {
        match leaf {
            Leaf::Field(field) => match old_by_id.get(field.id.as_str()) {
                Some(prev) => diff_leaf(section, prev, field.as_ref(), out),
                None => out.push(classify_added_item_field(section, field.as_ref())),
            },
            Leaf::Slot { id, slot } => match old_slots.get(id.as_str()) {
                Some(Leaf::Slot { slot: prev, .. }) => {
                    if prev.optional != slot.optional {
                        out.push(slot_flag_change(section, Some(id.clone()), slot.optional));
                    }
                }
                _ => out.push(SchemaChange::AddedItemSlot {
                    section: section.to_owned(),
                    leaf: id.clone(),
                }),
            },
            Leaf::Repeatable { id, .. } => {
                let unchanged = old_nested
                    .get(id.as_str())
                    .is_some_and(|prev| projected_leaf(prev) == projected_leaf(leaf));
                if !unchanged {
                    out.push(SchemaChange::Unclassified);
                }
            }
        }
    }
    let new_fields: Vec<&Field> = new.iter().filter_map(item_field).collect();
    removed_fields(section, old_fields.into_iter(), new_fields.into_iter(), out);
    for leaf in old {
        match leaf {
            Leaf::Slot { id, .. } if !new.iter().any(|l| item_slot_id(l) == Some(id.as_str())) => {
                out.push(SchemaChange::RemovedItemSlot {
                    section: section.to_owned(),
                    leaf: id.clone(),
                });
            }
            Leaf::Repeatable { id, .. }
                if !new.iter().any(|l| item_nested_id(l) == Some(id.as_str())) =>
            {
                out.push(SchemaChange::Unclassified);
            }
            _ => {}
        }
    }
}

/// Classify an `optional`-flag delta on a **slot** present in both schemas — [`optional_flag_change`]'s
/// leaf-kind twin, and the **one** rule for both loci: a section's own anonymous slot
/// (`leaf: None`) and a repeatable item block's named slot leaf (`leaf: Some(id)`).
///
/// A slot carries **no value source** — no `default:`, no `set:`, no pack-declared type — so
/// [`crate::validate::is_author_required`]'s exemptions cannot apply to one and the field rule's
/// refinement collapses: a **relaxation** is always the byte no-op
/// ([`SchemaChange::OptionalRelaxed`]), a **tightening** is always a real
/// [`SchemaChange::ProseNeeding`] (a doc that legitimately left the slot empty now breaks
/// `required-slot-present`, and the per-doc conformance gate adjudicates it — the Framing-A
/// handoff, never a build instruction). The flag is **inside** the conformance-relevant
/// projection, so either direction must name itself or the backstop refuses it
/// (`design/corpus-migration.md` → The structural projection: the two flag deltas get named
/// kinds).
fn slot_flag_change(section: &str, leaf: Option<String>, new_optional: bool) -> SchemaChange {
    let section = section.to_owned();
    if new_optional {
        SchemaChange::OptionalRelaxed { section, leaf }
    } else {
        SchemaChange::ProseNeeding { section, leaf }
    }
}

/// One leaf with its **out-of-projection keys erased** — [`erase_out_of_projection`]'s per-leaf
/// form, built on [`erase_block`] so the erasure recurses to any nesting depth. Two leaves'
/// projections compare equal exactly when no committed doc's bytes can tell them apart.
fn projected_leaf(leaf: &Leaf) -> Leaf {
    let mut one = vec![leaf.clone()];
    erase_block(&mut one);
    one.remove(0)
}

/// The id of an item block's **nested repeatable** leaf, if this leaf is one — [`item_slot_id`]'s
/// sibling, one leaf-kind over.
fn item_nested_id(leaf: &Leaf) -> Option<&str> {
    match leaf {
        Leaf::Repeatable { id, .. } => Some(id.as_str()),
        _ => None,
    }
}

/// The id of an item block's `Slot` leaf, if this leaf is one — [`item_field`]'s sibling, one
/// leaf-kind over.
fn item_slot_id(leaf: &Leaf) -> Option<&str> {
    match leaf {
        Leaf::Slot { id, .. } => Some(id.as_str()),
        _ => None,
    }
}

/// The `Field` leaf of an item block, if this leaf is one (a slot / nested repeatable is not).
fn item_field(leaf: &Leaf) -> Option<&Field> {
    match leaf {
        Leaf::Field(field) => Some(field.as_ref()),
        _ => None,
    }
}

/// Classify every leaf `old` declares that `new` **drops** as a [`SchemaChange::RemovedField`] —
/// the one removal rule, shared by both loci (a simple/header section's fields and a repeatable
/// item block's), emitted after the `new`-leaf pass in **`old`'s** document order (deterministic:
/// the same pair always diffs identically).
///
/// The removal needs a kind of its own precisely because the [`SchemaChange::Unclassified`]
/// backstop is a **residual**: a removal riding *alongside* any classified change leaves the diff
/// non-empty, so the backstop never fires and the dropped leaf would be **silently ignored** — the
/// doc restamped while it still carries a field line the new schema no longer declares. The driver
/// then **refuses** it (the recorded pick: refuse, not strip — see [`SchemaChange::RemovedField`]).
fn removed_fields<'a>(
    section: &str,
    old: impl Iterator<Item = &'a Field>,
    new: impl Iterator<Item = &'a Field>,
    out: &mut Vec<SchemaChange>,
) {
    let new_ids: Vec<&str> = new.map(|f| f.id.as_str()).collect();
    for field in old {
        if !new_ids.contains(&field.id.as_str()) {
            out.push(SchemaChange::RemovedField {
                section: section.to_owned(),
                field: field.id.clone(),
            });
        }
    }
}

/// Classify the leaves of a section that exists only in `v2` (every leaf is new).
fn added_section(id: &str, new: &SectionBody, out: &mut Vec<SchemaChange>) {
    match new {
        SectionBody::Simple { slot, fields } => {
            match slot {
                // An added **optional** slot section is deterministically mintable (empty
                // slot conforms) — the added-optional-section transform.
                Some(s) if s.optional => out.push(SchemaChange::AddedOptionalSection {
                    section: id.to_owned(),
                }),
                // A new **required** slot needs prose (the slot is anonymous → `leaf: None`).
                Some(_) => out.push(SchemaChange::ProseNeeding {
                    section: id.to_owned(),
                    leaf: None,
                }),
                None => {}
            }
            diff_fields(id, &[], fields, out);
        }
        // A wholly-new **repeatable** section: mint its empty `## Heading` — a zero-item
        // repeatable conforms, so the same block-insert the added-optional-section arm uses
        // carries the doc onto the v2 writer's canonical shape with no prose and no items
        // (`design/corpus-migration.md` → The classifier's holes: `AddedRepeatableSection`).
        // Before M42 this was *"not one of the transform kinds"* — it classified nothing, so
        // the pair rode the [`SchemaChange::Unclassified`] residual and the migration refused.
        // The item block's own leaves are **not** diffed as added fields: they are minted with
        // the (zero) items, not spliced into existing ones — an added leaf on a *pre-existing*
        // item block is [`SchemaChange::AddedOptionalField`]'s item-locus twin, its own kind.
        SectionBody::Repeatable { .. } => out.push(SchemaChange::AddedRepeatableSection {
            section: id.to_owned(),
        }),
    }
}

/// Diff a simple section's field list: classify added fields, the existing-leaf deltas through the
/// shared [`diff_leaf`] rule (matching by field id; field document order is `v2`'s), and — after
/// that pass — the leaves `new` **drops**, through the shared [`removed_fields`] rule.
fn diff_fields(section: &str, old: &[Field], new: &[Field], out: &mut Vec<SchemaChange>) {
    let old_by_id: HashMap<&str, &Field> = old.iter().map(|f| (f.id.as_str(), f)).collect();
    for field in new {
        match old_by_id.get(field.id.as_str()) {
            Some(prev) => diff_leaf(section, prev, field, out),
            None => out.push(classify_added_field(section, field)),
        }
    }
    removed_fields(section, old.iter(), new.iter(), out);
}

/// Classify a field present only in `v2`, on the one rule that governs every added leaf: **a
/// leaf whose absence the conformance gate accepts needs no prose.** That is
/// [`crate::validate::is_author_required`] — *the very predicate the gate's absent-field arm
/// consults* — negated, exactly as [`optional_flag_change`] already consults it for a tightening,
/// so classification and adjudication cannot drift. Not author-required ⇒
/// [`SchemaChange::AddedOptionalField`] (the driver then splices a deterministic value if one
/// exists, else folds to zero bytes); author-required ⇒ [`SchemaChange::ProseNeeding`], which
/// routes to the agent.
///
/// The predicate is **wider than the `optional || default || set` re-derivation it replaces**
/// (M46 Inc-4 T1), and each member it adds is a leaf a committed doc may conformantly omit: an
/// **optional `ref`** and a **pack-declared type**. Classifying those `ProseNeeding` was a
/// permanent dead end — the route says *author the prose, then re-run*, and neither authoring nor
/// re-running can change a leaf the gate never asks for.
fn classify_added_field(section: &str, field: &Field) -> SchemaChange {
    if !crate::validate::is_author_required(field) {
        SchemaChange::AddedOptionalField {
            section: section.to_owned(),
            field: field.id.clone(),
        }
    } else {
        SchemaChange::ProseNeeding {
            section: section.to_owned(),
            leaf: Some(field.id.clone()),
        }
    }
}

/// Classify a leaf present only in `v2`'s **repeatable item block** — the item-locus twin of
/// [`classify_added_field`], consulting the **same** [`crate::validate::is_author_required`] so
/// the two loci cannot drift (the M42 lesson: the two item/simple loops disagreeing is what
/// produced the holes). The **same two arms**, and the driver splits the placeable one
/// (`design/corpus-migration.md` → The classifier's holes: `AddedItemField`):
/// `default:` → the value is spliced into every item lacking it; every other not-author-required
/// shape (`optional:`, an optional `ref`, a pack type, a `set:`-derived leaf nobody threads a
/// value into) → a byte no-op, since an item lacking the bullet already conforms and inventing
/// one would fabricate a value; author-required with no default → [`SchemaChange::ProseNeeding`]
/// `{ leaf: Some }`, which blocks and routes to the agent.
fn classify_added_item_field(section: &str, field: &Field) -> SchemaChange {
    if !crate::validate::is_author_required(field) {
        SchemaChange::AddedItemField {
            section: section.to_owned(),
            field: field.id.clone(),
        }
    } else {
        SchemaChange::ProseNeeding {
            section: section.to_owned(),
            leaf: Some(field.id.clone()),
        }
    }
}

/// One added leaf a fold left **unfilled** — the subject of the migration report's loudness
/// rider ([`unfilled_set_leaves`]).
#[derive(Clone, Copy, Debug)]
pub struct UnfilledSetLeaf<'a> {
    /// The id of the section the leaf was added to, borrowed from the **new** schema (never
    /// from the change list), so the address a caller composes names a section that exists.
    pub section: &'a str,
    /// The declared field itself — the caller routes on its `set:` kind
    /// ([`crate::schema::is_machine_maintained_absolute`]: an absolute nothing may write reads
    /// differently from an author-overridable `on-create`).
    pub field: &'a Field,
    /// Whether the leaf was added to the section's **item block**
    /// ([`SchemaChange::AddedItemField`]) rather than to a simple/header section's `fields`
    /// ([`SchemaChange::AddedOptionalField`]) — the discrimination a caller *must* carry into
    /// what it says, because the two loci have different write addresses: a simple leaf is
    /// written at `#<section>/<field>`, an item leaf only at `#<section>/<item-id>/<field>`.
    /// Kept on the leaf rather than re-derived, because the change kind is the only place the
    /// locus is stated and it is discarded here (M46 completion audit, finding F2: one
    /// locus-blind renderer composed the simple form for both).
    pub item_locus: bool,
}

/// The added leaves of `changes` whose declaration in `to` carries a **`set:` deriver with no
/// `default:`** — exactly what the fold placed no bytes for, at **both** loci
/// ([`SchemaChange::AddedOptionalField`] and [`SchemaChange::AddedItemField`]), in change order.
///
/// **The one derivation, and it exists so there is not a fourth private one** (M46 Inc-4 T2).
/// Since T1 an absence the conformance gate accepts folds to zero bytes rather than blocking the
/// doc — correct, and *silent*: a doctype author who adds a `set:`-bearing field expecting the
/// corpus to carry a value would read `1 migrated` and nothing else, because the doc conforms
/// without it. This names what was left undone, and only that: an `optional:` leaf, an optional
/// `ref` and a pack-declared type all fold to zero bytes too, but none of them *declares a value
/// source*, so there is nothing unfilled about them.
///
/// **Feed it the schema the driver folded with**, not the raw pack shape: a caller that threads a
/// deterministic value in does so **as a `default:`** (the CLI's `with_stamp_default`), and the
/// `default.is_none()` clause is what makes such a leaf — the schema-version stamp — excluded by
/// construction rather than by a second list of exceptions.
pub fn unfilled_set_leaves<'a>(
    to: &'a Schema,
    changes: &[SchemaChange],
) -> Vec<UnfilledSetLeaf<'a>> {
    let mut out = Vec::new();
    for change in changes {
        let (section, field, item_locus) = match change {
            SchemaChange::AddedOptionalField { section, field } => (section, field, false),
            SchemaChange::AddedItemField { section, field } => (section, field, true),
            _ => continue,
        };
        let Some((section, decl)) = declared_added_leaf(to, section, field, item_locus) else {
            continue;
        };
        if decl.set.is_some() && decl.default.is_none() {
            out.push(UnfilledSetLeaf {
                section,
                field: decl,
                item_locus,
            });
        }
    }
    out
}

/// The declaration of an added leaf in the **new** schema, at the locus its change kind names —
/// a simple/header section's `fields` for [`SchemaChange::AddedOptionalField`], a repeatable
/// section's item block for [`SchemaChange::AddedItemField`]. Returns the section's own id
/// alongside, so the borrow is the schema's rather than the change list's. `None` when the
/// section or the leaf is not declared where the kind says it is (a caller pairing a change list
/// with a schema it did not come from).
fn declared_added_leaf<'a>(
    to: &'a Schema,
    section: &str,
    field: &str,
    item_locus: bool,
) -> Option<(&'a str, &'a Field)> {
    let sec = to.sections.iter().find(|s| s.id == section)?;
    let decl = match (&sec.body, item_locus) {
        (SectionBody::Simple { fields, .. }, false) => fields.iter().find(|f| f.id == field),
        (SectionBody::Repeatable { repeatable }, true) => repeatable
            .block
            .iter()
            .filter_map(item_field)
            .find(|f| f.id == field),
        _ => None,
    }?;
    Some((sec.id.as_str(), decl))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::load_schema;

    fn load(yaml: &[u8]) -> Schema {
        load_schema(yaml).expect("fixture schema loads")
    }

    /// An identical pair (a non-trivial schema with a header field group and a
    /// ref) yields an **empty** diff — proof the classifier diffs rather than
    /// always emitting.
    #[test]
    fn identical_schemas_yield_an_empty_diff() {
        let yaml = b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..1\" }
  - id: rationale
    slot: { hint: \"why\" }
";
        let schema = load(yaml);
        assert_eq!(schema_diff(&schema, &schema), vec![]);
    }

    /// `added-optional-field`: a new `optional: true` field in an existing
    /// section classifies to [`SchemaChange::AddedOptionalField`] naming the
    /// section + field.
    #[test]
    fn added_optional_field_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: link, type: string, optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_owned(),
                field: "link".to_owned(),
            }]
        );
    }

    /// A required field that carries a **deterministic default** is *not*
    /// prose-needing (the design's "no deterministic default" exclusion): it
    /// classifies to [`SchemaChange::AddedOptionalField`] — the deterministic-add
    /// branch (the schema-version stamp's `set` deriver rides this path).
    #[test]
    fn added_required_field_with_a_default_is_a_deterministic_add() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: kind, type: enum, of: [a, b], default: a }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalField {
                section: "meta".to_owned(),
                field: "kind".to_owned(),
            }]
        );
    }

    /// `widened-cardinality`: a field present in both whose `card` changed
    /// classifies to [`SchemaChange::WidenedCardinality`] naming the section +
    /// field.
    #[test]
    fn widened_cardinality_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::WidenedCardinality {
                section: "meta".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// `added-optional-section`: a wholly-new section whose body is an **optional**
    /// slot classifies to exactly [`SchemaChange::AddedOptionalSection`] naming the
    /// section — never `WidenedCardinality`, never the empty diff (the adr `options`
    /// v1→v2 shape, synthetically). The neighbours are unchanged, so it is the *only*
    /// change.
    #[test]
    fn added_optional_section_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: options
    slot: { hint: \"options considered\", optional: true }
  - id: consequences
    slot: { hint: \"the consequences\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalSection {
                section: "options".to_owned(),
            }]
        );
    }

    /// `added-repeatable-section`: a **wholly-new** section whose body is a *repeatable*
    /// item block classifies to **exactly** [`SchemaChange::AddedRepeatableSection`] naming
    /// the section — never the empty diff, never the backstop's residual. Red before T6:
    /// `added_section` classified only the simple-slot cases and closed with the fact in its
    /// own comment (*"A wholly-new repeatable section is not one of the transform kinds"*), so
    /// the pair emitted nothing and rode the residual — **refused**, even though a zero-item
    /// repeatable conforms and the block-insert primitive already exists.
    #[test]
    fn a_wholly_new_repeatable_section_classifies_added_repeatable_section() {
        let v1 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: outcome
    slot: { hint: \"the outcome\" }
",
        );
        let v2 = load(
            b"\
type: t
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
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedRepeatableSection {
                section: "tasks".to_owned(),
            }]
        );
    }

    /// `fixed-slot→repeatable-with-default`: a section that was a simple slot
    /// becoming a repeatable item-block classifies to
    /// [`SchemaChange::FixedSlotToRepeatable`] naming the section.
    #[test]
    fn fixed_slot_to_repeatable_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: requirements
    slot: { hint: \"the requirements prose\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: requirements
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one requirement\" } }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::FixedSlotToRepeatable {
                section: "requirements".to_owned(),
            }]
        );
    }

    /// `prose-needing`: a new **required** slot (here a wholly-new section whose
    /// body is a required slot) classifies to [`SchemaChange::ProseNeeding`] with
    /// `leaf: None` (a section's slot is anonymous), naming the section.
    #[test]
    fn prose_needing_required_slot_is_classified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
  - id: rationale
    slot: { hint: \"why this decision\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "rationale".to_owned(),
                leaf: None,
            }]
        );
    }

    /// A required field with **no** default/set is prose-needing, named by its
    /// field id (the `Some(leaf)` arm of the prose-needing kind) — the
    /// counterpart to the deterministic-add case above.
    #[test]
    fn prose_needing_required_field_names_the_leaf() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "meta".to_owned(),
                leaf: Some("owner".to_owned()),
            }]
        );
    }

    // ---- the two M38 doctype-level kinds (classified outside the per-section loop) ----

    /// `relocated`: a **pure home change** (v1 `location:` → v2 `placement:`) with
    /// identical sections classifies to **exactly** `[Relocated{from,to}]`, **not** the
    /// empty diff — proof the doctype-level fields are diffed *outside* the per-section
    /// loop (which never inspects `location`/`placement`), so a relocation cannot
    /// silently no-op. `from`/`to` are the schemas' declared homes, engine-pure.
    #[test]
    fn relocated_pure_home_change_classifies_to_exactly_relocated() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
placement: { file: CHANGELOG.md }
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::Relocated {
                from: "changelog/".to_owned(),
                to: "CHANGELOG.md".to_owned(),
            }]
        );
    }

    /// `display-title-changed`: a `display-title:` add (sections + home unchanged)
    /// classifies to **exactly** `[DisplayTitleChanged{to}]` naming the new H1 text.
    #[test]
    fn display_title_add_classifies_to_exactly_display_title_changed() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
location: changelog/
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::DisplayTitleChanged {
                to: "Changelog".to_owned(),
            }]
        );
    }

    /// A combined `location:` → `placement:` **and** `display-title:` add classifies to
    /// **both** doctype-level kinds, in the fixed doctype-level order (relocation, then
    /// the H1 re-title) — the real `changelog` v1→v2 relocation, synthetically.
    #[test]
    fn combined_relocation_and_display_title_add_classifies_both_kinds() {
        let v1 = load(
            b"\
type: changelog
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        let v2 = load(
            b"\
type: changelog
placement: { file: CHANGELOG.md }
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![
                SchemaChange::Relocated {
                    from: "changelog/".to_owned(),
                    to: "CHANGELOG.md".to_owned(),
                },
                SchemaChange::DisplayTitleChanged {
                    to: "Changelog".to_owned(),
                },
            ]
        );
    }

    // ---- the value-remapped kind (an enum member rename), both field shapes ----

    /// `value-remapped`: an enum `of:` member rename on a **simple** field
    /// (`[open, closed]` → `[pending, resolved]`) classifies to **exactly**
    /// `[ValueRemapped{section, field}]` carrying an **empty** map (the classifier detects
    /// only the delta; the CLI supplies the mapping) — red today (`diff_fields` inspects
    /// only `card`, so the delta classified to `[]`).
    #[test]
    fn enum_rename_on_a_simple_field_classifies_to_value_remapped() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [pending, resolved] }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ValueRemapped {
                section: "meta".to_owned(),
                field: "status".to_owned(),
                map: BTreeMap::new(),
            }]
        );
    }

    /// `value-remapped`: an enum `of:` member rename on a **repeatable-item** field
    /// (the deferral-ledger `kind` shape: `[D, I]` → `[Decision, Idea]`) classifies to
    /// **exactly** `[ValueRemapped{section, field}]` — red today (`diff_section` drops
    /// `(Repeatable, Repeatable)` through the `_ => {}` catch-all, so the delta was `[]`).
    /// The item's `id-from` field is untouched, so it is the *only* change.
    #[test]
    fn enum_rename_on_a_repeatable_item_field_classifies_to_value_remapped() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: reason
      block:
        - { id: reason, type: string }
        - { id: kind, type: enum, of: [D, I] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: reason
      block:
        - { id: reason, type: string }
        - { id: kind, type: enum, of: [Decision, Idea] }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ValueRemapped {
                section: "entries".to_owned(),
                field: "kind".to_owned(),
                map: BTreeMap::new(),
            }]
        );
    }

    /// An **unchanged** repeatable section (an item-block enum field whose members did not
    /// move) still yields the **empty** diff — the new `(Repeatable, Repeatable)` branch
    /// diffs, never emits on mere presence (the inert-when-unchanged guard for the omitting
    /// context, over the item-block path).
    #[test]
    fn unchanged_repeatable_section_yields_the_empty_diff() {
        let s = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: reason
      block:
        - { id: reason, type: string }
        - { id: kind, type: enum, of: [Decision, Idea] }
",
        );
        assert_eq!(schema_diff(&s, &s), vec![]);
    }

    // ---- the structural projection: outside it → `PresentationOnly`; moved-but-unclassified
    // ---- → `Unclassified` (the empty-diff backstop)

    /// A **hint reword** (a slot's authoring guidance) — plus the doctype-level `description:`
    /// / `usage:` prose — classifies to **exactly** `[PresentationOnly]`: the delta is outside
    /// the conformance-relevant projection (a committed doc's bytes cannot violate an authoring
    /// hint), so it must *name itself* and fold to zero bytes. Red today: it diffs to `[]`,
    /// which the backstop would then refuse — making a typo fix in a hint **unshippable**.
    #[test]
    fn a_hint_and_prose_reword_classifies_presentation_only() {
        let v1 = load(
            b"\
type: t
description: \"A thing.\"
usage: \"Reach for it sometimes.\"
sections:
  - id: rationale
    slot: { hint: \"why\" }
",
        );
        let v2 = load(
            b"\
type: t
description: \"A thing, precisely.\"
usage: \"Reach for it when a decision is warranted.\"
sections:
  - id: rationale
    slot: { hint: \"why this decision\" }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// A **`default:` add** on a field present in both schemas classifies to **exactly**
    /// `[PresentationOnly]` — a mint-time value source, not a conformance constraint: a
    /// committed doc lacking the value already conforms, so the change adds no bytes to the
    /// corpus. Red today: `diff_fields` inspects only `card`/`of` for an existing leaf, so
    /// this moved the hash and diffed to `[]`.
    #[test]
    fn a_default_add_on_an_existing_field_classifies_presentation_only() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed], default: open }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// An **`inverse:` / `check:` edit** classifies to **exactly** `[PresentationOnly]`: the
    /// inverse names a *derived back-edge* (a store advisory, never a per-doc conformance
    /// break) and `check:` selects the `doc-code` predicate, adjudicated against **code**, not
    /// the doc's bytes. Neither can make a conformant doc non-conformant.
    #[test]
    fn an_inverse_and_check_edit_classifies_presentation_only() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..1\", inverse: referred-by }
      - { id: anchor, type: string, check: symbol-exists }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..1\", inverse: refers-to-me, inverse-card: \"0..*\" }
      - { id: anchor, type: string, check: criterion-maps-to-test }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// The projection erasure reaches **inside a repeatable item block** too (the second
    /// locus): a hint reword + a `title-names-symbol` flip on an item-block leaf classifies to
    /// **exactly** `[PresentationOnly]`, never the empty diff and never a refusal.
    #[test]
    fn an_out_of_projection_edit_inside_an_item_block_classifies_presentation_only() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: anchor, type: string }
        - { id: note, slot: { hint: \"the note\" } }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: anchor, type: string, title-names-symbol: true }
        - { id: note, slot: { hint: \"one note per entry\" } }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// **The empty-diff backstop.** A pair whose *conformance-relevant* projection moved but
    /// for which **no transform kind exists** — a **removed section** — must **not** diff to
    /// `[]`: it classifies [`SchemaChange::Unclassified`], which refuses the migration instead
    /// of stamp-bumping a corpus that fails its own gate. Red today: the section-diff loops
    /// `v2`'s sections, so a section only `v1` declares is invisible and the pair diffs `[]`.
    #[test]
    fn a_removed_section_does_not_diff_to_empty_it_classifies_unclassified() {
        let v1 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: retired
    slot: { hint: \"a section v2 drops\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: context
    slot: { hint: \"the context\" }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::Unclassified]);
    }

    /// The backstop **does not fire on a change that classifies**: the shipped kinds still
    /// emit exactly themselves (here the flagship `adr` v1→v2 `options` shape, whose projection
    /// genuinely moved) — no `Unclassified`, no stray `PresentationOnly` riding alongside.
    /// The inert-when-classified guard: the backstop is the *residual*, not an extra emission.
    #[test]
    fn a_classified_change_never_also_emits_the_backstop_signal() {
        let v1 = load(
            b"\
type: t
description: \"A thing.\"
sections:
  - id: context
    slot: { hint: \"the context\" }
",
        );
        // The projection moves (an added optional section) **and** an out-of-projection key
        // moves (the description) — the kind classifies, so the residual never fires.
        let v2 = load(
            b"\
type: t
description: \"A thing, reworded.\"
sections:
  - id: context
    slot: { hint: \"the context\" }
  - id: options
    slot: { hint: \"options considered\", optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedOptionalSection {
                section: "options".to_owned(),
            }]
        );
    }

    // ---- the existing-leaf `card` / `of` deltas: classified BY DIRECTION, at BOTH loci ----

    /// An **enum widening** (`[a, b]` → `[a, b, c]`) on a simple field classifies to **exactly**
    /// `[EnumWidened]` — every committed value is still a declared member, so it is a byte
    /// no-op and **no map is authored**. Red before T3: `enum_members_renamed` fired on *any*
    /// `of:` delta, so this emitted `ValueRemapped { map: {} }` and the driver blocked the doc
    /// on its first committed value — with no map to author, because nothing was renamed.
    #[test]
    fn an_enum_widening_classifies_enum_widened_never_a_rename() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [active, joined] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [active, joined, discarded] }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::EnumWidened {
                section: "meta".to_owned(),
                field: "status".to_owned(),
            }]
        );
    }

    /// The same widening **inside a repeatable item block** (the second locus) classifies to
    /// **exactly** `[EnumWidened]` — the item-block leaf diff reads direction too, not only
    /// *that* the members moved.
    #[test]
    fn an_enum_widening_on_an_item_block_field_classifies_enum_widened() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [Decision, Idea] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: kind, type: enum, of: [Decision, Idea, Risk] }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::EnumWidened {
                section: "entries".to_owned(),
                field: "kind".to_owned(),
            }]
        );
    }

    /// A **non-superset** `of:` delta — a genuine member rename (`[D, I]` → `[Decision, Idea]`,
    /// the shipped `deferral-ledger` bump) — still classifies `[ValueRemapped]`: the widening
    /// kind narrows the rename kind's extent, it does not replace it. (The two rename tests
    /// above pin both loci; this one pins that a *member drop* — also a non-superset — lands
    /// here rather than being mistaken for a widening.)
    #[test]
    fn a_dropped_enum_member_is_not_a_widening_and_stays_value_remapped() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed, void] }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: status, type: enum, of: [open, closed] }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ValueRemapped {
                section: "meta".to_owned(),
                field: "status".to_owned(),
                map: BTreeMap::new(),
            }]
        );
    }

    /// A **cardinality narrowing** (`0..*` → `0..1`) on a simple field classifies to **exactly**
    /// `[NarrowedCardinality]` — a content-affecting change (a committed instance may carry more
    /// values than the new bound admits), which the driver refuses. Red before T3: `diff_fields`
    /// fired `WidenedCardinality` on **any** `card` delta, direction-blind, so a narrowing folded
    /// to zero bytes and the doc restamped **past the gate**, silently.
    #[test]
    fn a_card_narrowing_classifies_narrowed_cardinality() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::NarrowedCardinality {
                section: "meta".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// A **minimum tightening** (`0..*` → `1..*`) is a narrowing too: the new bound demands a
    /// value a committed instance may legitimately lack. Any tightening on *either* bound —
    /// never only the upper one — is the refused direction.
    #[test]
    fn a_card_minimum_tightening_classifies_narrowed_cardinality() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"1..*\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::NarrowedCardinality {
                section: "meta".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// A `card` **widen inside a repeatable item block** classifies to **exactly**
    /// `[WidenedCardinality]` — the byte no-op the authoring matrix has always promised. Red
    /// before T3: `diff_item_fields` never reached `card`, so it diffed to `[]` (the promise the
    /// classifier did not honour).
    #[test]
    fn an_item_block_card_widen_classifies_widened_cardinality() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::WidenedCardinality {
                section: "entries".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// A `card` **narrowing inside a repeatable item block** classifies to **exactly**
    /// `[NarrowedCardinality]` — the item locus reads direction as the simple one does. Red
    /// before T3: `[]` (a real break, invisible).
    #[test]
    fn an_item_block_card_narrowing_classifies_narrowed_cardinality() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: rel, type: ref, to: t, card: \"0..*\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::NarrowedCardinality {
                section: "entries".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    /// **The false-refusal guard on the direction rule**: spelling the *default* cardinality out
    /// (`card:` absent — the `"0..1"` default — → an explicit `card: "0..1"`) moves the
    /// schema-hash but changes **no bound**, so it must classify as the byte no-op
    /// (`WidenedCardinality`, the new-⊇-old direction), never as a refused narrowing and never
    /// as the empty diff (which the backstop would then refuse — an unshippable edit).
    #[test]
    fn spelling_out_the_default_card_is_the_byte_no_op_direction() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: t, card: \"0..1\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::WidenedCardinality {
                section: "meta".to_owned(),
                field: "rel".to_owned(),
            }]
        );
    }

    // ---- the `optional` flag deltas (the sixth hole): relax → no-op kind; tighten →
    // ---- `ProseNeeding` iff the leaf thereby becomes author-required

    /// **The relaxation, on a field** (`optional: false → true`): every doc conformant under the
    /// strict rule is conformant under the loose one, so it is a **byte no-op** — but it must
    /// still *name itself*, or the backstop refuses it. Classifies to **exactly**
    /// `[OptionalRelaxed]` naming the section + leaf. Red before T4: the flag is invisible to the
    /// existing-leaf diff, so the pair emitted nothing and fell through to the residual — a
    /// **false refusal** of a change that cannot break a single doc.
    #[test]
    fn an_optional_relax_on_a_field_classifies_optional_relaxed() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string, optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::OptionalRelaxed {
                section: "meta".to_owned(),
                leaf: Some("owner".to_owned()),
            }]
        );
    }

    /// **The relaxation, on a slot** (a section's anonymous slot turned `optional: true`) — the
    /// same no-op kind, with `leaf: None` (the [`SchemaChange::ProseNeeding`] shape).
    #[test]
    fn an_optional_relax_on_a_slot_classifies_optional_relaxed_with_no_leaf() {
        let v1 = load(
            b"\
type: t
sections:
  - id: rationale
    slot: { hint: \"why\" }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: rationale
    slot: { hint: \"why\", optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::OptionalRelaxed {
                section: "rationale".to_owned(),
                leaf: None,
            }]
        );
    }

    /// **The tightening, on a slot** (`optional: true → false`) — the sixth hole. A currently
    /// conformant doc that legitimately left the optional slot empty is now **non-conformant**,
    /// so it is a real [`SchemaChange::ProseNeeding`] (`leaf: None` — a section's slot is
    /// anonymous), routed to the agent. Red before T4: it diffed to nothing and fell through to
    /// the residual, which cannot tell a *prose* need from a missing transform kind.
    #[test]
    fn an_optional_tighten_on_a_slot_classifies_prose_needing() {
        let v1 = load(
            b"\
type: t
sections:
  - id: notes
    slot: { hint: \"notes\", optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: notes
    slot: { hint: \"notes\" }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "notes".to_owned(),
                leaf: None,
            }]
        );
    }

    /// **The tightening, on an author-required field** — the same real `ProseNeeding`, named by
    /// its leaf: a committed doc that legitimately omits the now-required field breaks
    /// `required-field-present`.
    #[test]
    fn an_optional_tighten_on_an_author_required_field_classifies_prose_needing() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string, optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: owner, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "meta".to_owned(),
                leaf: Some("owner".to_owned()),
            }]
        );
    }

    /// **The refinement the flat rule misses**: a tighten on a field carrying a `default:` renders
    /// **no** doc non-conformant — `schema_conformance`'s absent-field arm consults
    /// `validate::is_author_required`, which exempts a defaulted field — so blocking it would be a
    /// **false refusal**. It classifies the out-of-projection no-op kind
    /// ([`SchemaChange::PresentationOnly`]) and ships. The same holds for a `set:`-derived field
    /// (the schema-version stamp's shape).
    #[test]
    fn an_optional_tighten_on_a_defaulted_field_is_a_no_op_never_a_block() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: kind, type: enum, of: [a, b], optional: true, default: a }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: kind, type: enum, of: [a, b], default: a }
",
        );
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// **The second locus**: the flag deltas classify inside a **repeatable item block** too — a
    /// relax on one item field and a tighten on another, in item-block document order. Red before
    /// T4 at this locus as well (`diff_item_fields` reached only `card`/`of`).
    #[test]
    fn the_optional_flag_deltas_classify_inside_a_repeatable_item_block() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: link, type: string }
        - { id: owner, type: string, optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: link, type: string, optional: true }
        - { id: owner, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![
                SchemaChange::OptionalRelaxed {
                    section: "entries".to_owned(),
                    leaf: Some("link".to_owned()),
                },
                SchemaChange::ProseNeeding {
                    section: "entries".to_owned(),
                    leaf: Some("owner".to_owned()),
                },
            ]
        );
    }

    // ---- field removal: a kind of its own (the recorded pick is to REFUSE it, not strip) ----

    /// **A leaf v2 drops classifies `[RemovedField]`** at the simple locus, naming the section +
    /// field. Red before T5: both diff loops iterate **`v2`'s** leaves and match back into `v1`,
    /// so a leaf only `v1` declares is *invisible* — the pair emitted nothing of its own and fell
    /// through to the backstop's residual (and, before the backstop, to `[]` + a silent stamp
    /// bump).
    #[test]
    fn a_removed_field_at_the_simple_locus_classifies_removed_field() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner, type: string, optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::RemovedField {
                section: "meta".to_owned(),
                field: "owner".to_owned(),
            }]
        );
    }

    /// The **second locus**: a leaf dropped from a **repeatable item block** classifies
    /// `[RemovedField]` too — the item-block loop reads removals as the simple one does.
    #[test]
    fn a_removed_field_in_a_repeatable_item_block_classifies_removed_field() {
        let v1 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: owner, type: string, optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::RemovedField {
                section: "entries".to_owned(),
                field: "owner".to_owned(),
            }]
        );
    }

    /// **Why the removal needs a kind of its own, and cannot ride the backstop.** A removal
    /// *accompanied* by a classified change (here an added optional field) leaves the diff
    /// **non-empty**, so the backstop's residual — which fires only when nothing classified —
    /// **never runs**: without its own kind the removal would be silently dropped and the doc
    /// restamped, exactly the strand the backstop exists to close. It names itself, after the
    /// v2-leaf pass, in `v1`'s document order.
    #[test]
    fn a_removal_alongside_a_classified_change_still_names_itself() {
        let v1 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: owner, type: string, optional: true }
",
        );
        let v2 = load(
            b"\
type: t
sections:
  - id: meta
    header: true
    fields:
      - { id: title, type: string }
      - { id: link, type: string, optional: true }
",
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![
                SchemaChange::AddedOptionalField {
                    section: "meta".to_owned(),
                    field: "link".to_owned(),
                },
                SchemaChange::RemovedField {
                    section: "meta".to_owned(),
                    field: "owner".to_owned(),
                },
            ]
        );
    }

    // ---- the added item field (T7): the three arms, mirroring `classify_added_field` ----

    /// **A field added to a repeatable item block, carrying a `default:`**, classifies to
    /// **exactly** `[AddedItemField]` naming the section + field — the deterministic-value arm
    /// (the driver splices the value into every item). Red before T7: `diff_item_fields`
    /// classified only *existing* leaves, so an added one was invisible and the pair diffed to
    /// `[]` — the doc "migrated by accident" (the stamp flipped, the declared default never
    /// landed).
    #[test]
    fn an_added_item_field_with_a_default_classifies_added_item_field() {
        let v1 = load(ledger_v1_yaml());
        let v2 = load(
            b"\
type: t
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
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedItemField {
                section: "entries".to_owned(),
                field: "kind".to_owned(),
            }]
        );
    }

    /// **An added item field that is `optional:` with no default** classifies to **exactly**
    /// `[AddedItemField]` too — it *names itself* (the backstop requires that of every real
    /// change) even though the driver folds it to **zero bytes**: an item without the bullet
    /// already conforms. Red before T7: `[]`.
    #[test]
    fn an_added_optional_item_field_with_no_default_still_classifies_added_item_field() {
        let v1 = load(ledger_v1_yaml());
        let v2 = load(
            b"\
type: t
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
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::AddedItemField {
                section: "entries".to_owned(),
                field: "owner".to_owned(),
            }]
        );
    }

    /// **An added item field that is *required* with no default** classifies to **exactly**
    /// `[ProseNeeding { leaf: Some }]` — the same arm [`classify_added_field`] takes at the simple
    /// locus: no deterministic value exists, so it routes to the agent and the doc blocks. Red
    /// before T7: `[]` — a **permanent mutual dead end** (`validate` said *run the migration*,
    /// `migrate-corpus` said *author the prose*, and neither instruction was actionable).
    #[test]
    fn an_added_required_item_field_with_no_default_classifies_prose_needing() {
        let v1 = load(ledger_v1_yaml());
        let v2 = load(
            b"\
type: t
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
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![SchemaChange::ProseNeeding {
                section: "entries".to_owned(),
                leaf: Some("owner".to_owned()),
            }]
        );
    }

    /// The v1 item block the three added-item-field cases grow a leaf onto (the shipped
    /// `deferral-ledger` shape: an id-from title, a field, a prose slot).
    fn ledger_v1_yaml() -> &'static [u8] {
        b"\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: body, slot: { hint: \"the deferral\" } }
"
    }

    /// The classifier **diffs**, never emits on mere presence: a schema carrying both a
    /// `location:` and a `display-title:`, diffed against itself, yields the empty diff
    /// (neither doctype-level kind fires when nothing changed) — the inert-when-unchanged
    /// guard for the omitting context.
    #[test]
    fn unchanged_home_and_display_title_emit_no_doctype_level_change() {
        let s = load(
            b"\
type: changelog
location: changelog/
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
",
        );
        assert_eq!(schema_diff(&s, &s), vec![]);
    }

    /// The **nested-repeatable base fixture**: an item block carrying a scalar field and a
    /// nested repeatable whose own block carries a slot — the shape the two tests below reshape
    /// in opposite directions (inside the projection, and outside it).
    const NESTED: &str = "\
type: t
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: notes
          repeatable:
            id-from: title
            block:
              - { id: title, type: string }
              - { id: body, slot: { hint: \"HINT\" } }
";

    /// **A hint reword *inside* a nested repeatable stays the out-of-projection no-op it is** —
    /// `[PresentationOnly]`, never the backstop. The nested comparison is over the **erased**
    /// projection, so naming the nested delta cannot turn a change no committed doc's bytes can
    /// violate into a false refusal (the failure direction this classifier is built to avoid).
    #[test]
    fn a_hint_reword_inside_a_nested_repeatable_is_not_a_refusal() {
        let v1 = load(NESTED.replace("HINT", "why").as_bytes());
        let v2 = load(NESTED.replace("HINT", "why, in one line").as_bytes());
        assert_eq!(schema_diff(&v1, &v2), vec![SchemaChange::PresentationOnly]);
    }

    /// **A nested-repeatable delta names itself beside a classified change.** The added optional
    /// item field leaves the diff non-empty, so the [`SchemaChange::Unclassified`] *residual*
    /// never fires — which is exactly why the delta must emit the kind itself, or the doc
    /// restamps at a version whose nested shape it does not carry.
    #[test]
    fn a_nested_repeatable_delta_names_the_backstop_beside_a_classified_change() {
        let v1 = load(NESTED.replace("HINT", "why").as_bytes());
        let v2 = load(
            NESTED
                .replace("HINT", "why")
                .replace(
                    "        - id: notes\n",
                    "        - { id: owner, type: string, optional: true }\n        - id: notes\n",
                )
                .replace(
                    "              - { id: title, type: string }\n",
                    "              - { id: title, type: string }\n              - { id: weight, type: string }\n",
                )
                .as_bytes(),
        );
        assert_eq!(
            schema_diff(&v1, &v2),
            vec![
                SchemaChange::AddedItemField {
                    section: "entries".into(),
                    field: "owner".into(),
                },
                SchemaChange::Unclassified,
            ],
        );
    }
}
