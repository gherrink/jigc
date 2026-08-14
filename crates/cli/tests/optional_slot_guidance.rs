//! **The optional-slot guidance fence** (M48 Increment 9, T1 — trial finding F7).
//!
//! The reported defect: `adr#options`' hint said *"omit when the call was
//! obvious"*, the agent obliged, and the canonical writer emitted the `## Options`
//! heading anyway — a hollow section shipped into the committed store. The Settle
//! adjudicated this a **law-1 "nothing lies" defect in pack prose, not a format
//! defect** (`completions/artifacts/M48/settle-record.md` → F7): the format is
//! behaving exactly as `design/document-type-schema.md` documents (*"heading is
//! still rendered … optionality governs prose, not heading presence"*), and
//! `schema_diff.rs`'s `AddedOptionalSection` invariant depends on it. The lying
//! link is the guidance, so the guidance is what changes.
//!
//! The property this suite stands for: **no guidance for an optional slot
//! instructs an omission the format does not honour.**
//!
//! The subject is **derived from the loaded [`Schema`] model of both shipped
//! packs** — never a grep over YAML text — so a future optional slot joins the
//! fence for free. It is scoped to the **persisted** doctypes, because the
//! transient `commit` sink genuinely strips an empty body from the git message
//! (`doc show commit:<id> --task` rendering a hollow `## Body` is a declared
//! non-defect, the Settle's own words). That exclusion is **asserted rather than
//! assumed**: the excluded set is computed from `location`/`placement` absence and
//! then checked against the one doctype it may contain.
//!
//! **Declared bound** (roadmap → M48 Increment 9, Grouped scope): the *axis-complete*
//! half of this fence is the derived `optional:` set — a slot cannot gain a lying
//! hint without reddening here. The step-body half is a **bounded omission-vocabulary
//! probe**, not an axis-complete fence: its subject is derived (the guidance paragraph
//! that names the slot, in any step of either pack), but its predicate is a small
//! closed vocabulary, so a novel phrasing can still slip past. A fence that claimed
//! more would be a grep-checklist over prose, which the M42 lesson forbids
//! (`design/surface-contract.md`: *a census cannot enforce a predicate*).

use std::collections::BTreeSet;

use cli::pack::{EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, Schema, SectionBody};

/// The omission vocabulary — the closed set of phrasings that tell an author the
/// section can be left out of the rendered document. Lower-cased before matching.
/// Deliberately small: every member is a phrase that *only* means "this heading
/// will not appear", which is the falsehood F7 names.
const OMISSION_INSTRUCTIONS: [&str; 6] = [
    "omit",
    "leave it out",
    "leave them out",
    "leaving it out",
    "skip it",
    "drop it",
];

/// The offending phrase, when `text` instructs an omission.
fn instructs_omission(text: &str) -> Option<&'static str> {
    let hay = text.to_ascii_lowercase();
    OMISSION_INSTRUCTIONS
        .iter()
        .copied()
        .find(|phrase| hay.contains(phrase))
}

/// One optional slot, as the loaded schema model reports it.
struct OptionalSlot {
    /// The pack the schema shipped from (both packs ship a `commit`).
    pack: &'static str,
    /// The doctype id.
    doctype: String,
    /// `true` when instances persist to a file — a `location:` folder home or a
    /// literal `placement:` home. `false` is the transient sink (`commit`).
    persisted: bool,
    /// The slot's write address, for the failure message.
    address: String,
    /// The slot's own id — the token a step body backticks when it guides the slot.
    slot_id: String,
    /// The authoring hint, when the schema declares one.
    hint: Option<String>,
}

/// The two shipped packs, tagged by origin.
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// Collect every optional slot a loaded schema declares — section slots and the
/// slots inside (possibly nested) repeatable item blocks alike.
fn optional_slots_of(pack: &'static str, schema: &Schema) -> Vec<OptionalSlot> {
    let persisted = schema.location.is_some() || schema.placement.is_some();
    let mut out = Vec::new();

    fn walk_leaves(
        pack: &'static str,
        schema: &Schema,
        persisted: bool,
        path: &str,
        leaves: &[Leaf],
        out: &mut Vec<OptionalSlot>,
    ) {
        for leaf in leaves {
            match leaf {
                Leaf::Slot { id, slot } if slot.optional => out.push(OptionalSlot {
                    pack,
                    doctype: schema.ty.clone(),
                    persisted,
                    address: format!("{path}/{id}"),
                    slot_id: id.clone(),
                    hint: slot.hint.clone(),
                }),
                Leaf::Repeatable { id, repeatable } => walk_leaves(
                    pack,
                    schema,
                    persisted,
                    &format!("{path}/{id}"),
                    &repeatable.block,
                    out,
                ),
                _ => {}
            }
        }
    }

    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple {
                slot: Some(slot), ..
            } if slot.optional => out.push(OptionalSlot {
                pack,
                doctype: schema.ty.clone(),
                persisted,
                address: format!("{}#{}", schema.ty, section.id),
                slot_id: section.id.clone(),
                hint: slot.hint.clone(),
            }),
            SectionBody::Repeatable { repeatable } => walk_leaves(
                pack,
                schema,
                persisted,
                &format!("{}#{}", schema.ty, section.id),
                &repeatable.block,
                &mut out,
            ),
            SectionBody::Simple { .. } => {}
        }
    }
    out
}

/// Every optional slot across both shipped packs, derived from the loaded model.
fn optional_slots() -> Vec<OptionalSlot> {
    let mut out = Vec::new();
    for (name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Schemas) {
            let bytes = pack
                .read(PackResourceKind::Schemas, &id)
                .expect("a listed schema reads back");
            let schema =
                load_pack_schema(&pack, &bytes).expect("a shipped schema parses through the pack");
            out.extend(optional_slots_of(name, &schema));
        }
    }
    out
}

/// Every step body of both packs, tagged `<pack>/<step-id>`.
fn step_bodies() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Steps) {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step reads back");
            let body = String::from_utf8(bytes).expect("a shipped step body is UTF-8");
            out.push((format!("{name}/{}", id.as_str()), body));
        }
    }
    out
}

/// **The exclusion is asserted, not assumed.** The transient sink is derived from
/// the model (`location` and `placement` both absent), and the doctypes that
/// derivation excludes are checked against the one type allowed to be there — so
/// scoping the fence to persisted doctypes cannot silently widen.
#[test]
fn the_transient_exclusion_covers_only_the_commit_sink() {
    let slots = optional_slots();
    let excluded: BTreeSet<String> = slots
        .iter()
        .filter(|s| !s.persisted)
        .map(|s| s.doctype.clone())
        .collect();

    assert!(
        !excluded.is_empty(),
        "the persisted-scoping must actually exclude something — an empty exclusion \
         means the discriminator stopped discriminating and the fence's scope is a \
         no-op claim",
    );
    assert_eq!(
        excluded,
        BTreeSet::from([String::from("commit")]),
        "only the transient `commit` sink may be excluded from the optional-slot \
         guidance fence (its empty body is genuinely stripped from the git message — \
         the Settle's declared non-defect); a new transient doctype with an optional \
         slot must be adjudicated, not silently scoped out",
    );
}

/// **The fence.** Every optional slot on a *persisted* doctype, in either shipped
/// pack, carries a hint that does not instruct an omission the writer will not
/// honour. Red at HEAD on `adr#options`' *"omit when the call was obvious"*.
#[test]
fn no_persisted_optional_slot_hint_instructs_an_omission() {
    let slots = optional_slots();
    let persisted: Vec<&OptionalSlot> = slots.iter().filter(|s| s.persisted).collect();

    assert!(
        !persisted.is_empty(),
        "the derived persisted optional-slot set is empty — the fence would pass \
         vacuously; the traversal or the `optional:` model has drifted",
    );

    for slot in persisted {
        let Some(hint) = slot.hint.as_deref() else {
            continue;
        };
        assert!(
            instructs_omission(hint).is_none(),
            "`{}` (pack `{}`) tells the author to {:?}, but the canonical writer emits \
             every schema section's heading unconditionally — optionality governs prose, \
             not heading presence (`design/document-type-schema.md`). Obeying the hint \
             ships a hollow `{}` section. State what actually happens: the heading \
             renders either way, and an empty optional slot conforms.\nhint: {hint:?}",
            slot.address,
            slot.pack,
            instructs_omission(hint).unwrap_or_default(),
            slot.slot_id,
        );
    }
}

/// **The same property over the step bodies** — the guidance paragraph that names a
/// persisted optional slot, in any step of either pack, must not instruct an
/// omission either. The subject is derived (which paragraph, in which step, is
/// decided by the loaded model's slot ids); the predicate is the bounded vocabulary
/// declared at the top of this file, which is why this half is a probe and not an
/// axis-complete fence.
#[test]
fn no_step_paragraph_guiding_a_persisted_optional_slot_instructs_an_omission() {
    let slots = optional_slots();
    let steps = step_bodies();
    let mut guiding_paragraphs = 0usize;

    for slot in slots.iter().filter(|s| s.persisted) {
        let token = format!("`{}`", slot.slot_id);
        for (step, body) in &steps {
            // A step guides *this* doctype's slot only when it also names the doctype;
            // the backticked id alone is not enough to bind the paragraph to a schema.
            if !body.contains(&slot.doctype) {
                continue;
            }
            for paragraph in body.split("\n\n") {
                if !paragraph.contains(&token) {
                    continue;
                }
                guiding_paragraphs += 1;
                assert!(
                    instructs_omission(paragraph).is_none(),
                    "step `{step}` guides `{}` and tells the author to {:?}, but the \
                     heading renders either way — the omission it promises cannot \
                     happen.\nparagraph:\n{paragraph}",
                    slot.address,
                    instructs_omission(paragraph).unwrap_or_default(),
                );
            }
        }
    }

    assert!(
        guiding_paragraphs > 0,
        "no step paragraph names any persisted optional slot — the probe would pass \
         vacuously; the derivation has drifted from the shipped steps",
    );
}
