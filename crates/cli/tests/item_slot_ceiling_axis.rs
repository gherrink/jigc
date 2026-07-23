//! **The slot heading-depth ceiling is a per-address fact, derived from
//! `(Schema, section_id, item-chain)`** — [`implementation/parsing.md`] → Slot
//! heading-depth ceiling (the four-row table and its `multi_slot || has_nested`
//! derivation).
//!
//! The statement this suite pins: for *every* doctype × item context the real
//! registry ships, [`engine::write::slot_ceiling`] returns the reserved ceiling and
//! first-allowed depth that context's structure actually implies — `###`-reserved
//! for a plain depth-1 item, `####`-reserved where the item carries multi-slot
//! sub-labels or a nested repeatable. A blanket *"`####` is safe"* is neither always
//! true nor always false, which is exactly why the sweep asserts the pair per
//! address rather than a global constant.
//!
//! **Enumeration comes from the registry, never a hand list**
//! ([`implementation/pinning.md`] §1): the contexts are walked out of the
//! engine-loaded schemas of the composite `[dev ▸ methodology]` pack-set, so a new
//! doctype (or a new nested repeatable in an existing one) joins the sweep the day
//! it lands — and, having no stated expectation, reddens this suite until its
//! ceiling is stated. The expected pairs are the hand-written half on purpose:
//! they are the *claim*, restated from the design table, not a second computation
//! of the code under test.
//!
//! Every repeatable item context is swept, including the **field-only** ones
//! (`commit.trailers`, `completion-record.findings`, `milestone-record.tasks`) and
//! the slotless-but-nested `changelog.releases`: the ceiling is a property of the
//! *address*, and `changelog.releases` is the only `has_nested` witness either pack
//! ships — the shipped half of the row no doctype exercises through a slot.
//!
//! [`implementation/parsing.md`]: ../../../implementation/parsing.md
//! [`implementation/pinning.md`]: ../../../implementation/pinning.md

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, Repeatable, Schema, SectionBody};
use engine::write::slot_ceiling;
use std::collections::{BTreeMap, BTreeSet};

/// The production composition, built the **CWD-free** way (`pinning.md` §1 —
/// `make_pack()` resolves against the process CWD and is a hazard under parallel
/// tests): `[dev ▸ methodology]`, dev highest-precedence.
fn composite() -> CompositePack {
    CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ])
}

/// Every doctype the composite registry ships, loaded through the **CLI** schema
/// loader against the doctype's own **origin** pack (so a pack-declared field type
/// resolves).
fn loaded_schemas(pack: &dyn PackSource) -> BTreeMap<String, Schema> {
    pack.list(PackResourceKind::Schemas)
        .iter()
        .map(|id| {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("read the `{id}` schema: {e}"));
            let origin = pack.origin_pack(PackResourceKind::Schemas, id);
            let schema = load_pack_schema(origin, &bytes)
                .unwrap_or_else(|e| panic!("load the `{id}` schema: {e}"));
            (schema.ty.clone(), schema)
        })
        .collect()
}

/// A placeholder item id. Item ids contribute **depth** to the derivation and
/// nothing else (the walk is schema-side — no document is consulted), so the sweep
/// does not need real anchors to enumerate a context.
const ITEM: &str = "an-item";

/// One enumerated item context: its human-readable address (`changelog.releases/`
/// `changes`) and the section-qualified chain the engine walks.
struct ItemContext {
    address: String,
    section_id: String,
    chain: Vec<String>,
}

/// Walk one doctype's schema for every repeatable item context — each repeatable
/// section, then each nested repeatable inside an item block, recursively.
fn contexts(doctype: &str, schema: &Schema) -> Vec<ItemContext> {
    let mut out = Vec::new();
    for section in &schema.sections {
        let SectionBody::Repeatable { repeatable } = &section.body else {
            continue;
        };
        walk(
            &section.id,
            &format!("{doctype}.{}", section.id),
            vec![ITEM.to_string()],
            repeatable,
            &mut out,
        );
    }
    out
}

/// Emit the context for one repeatable level, then descend into each nested
/// repeatable its item block declares (the chain gains the nested-section id plus
/// one more item hop per level).
fn walk(
    section_id: &str,
    address: &str,
    chain: Vec<String>,
    repeatable: &Repeatable,
    out: &mut Vec<ItemContext>,
) {
    out.push(ItemContext {
        address: address.to_string(),
        section_id: section_id.to_string(),
        chain: chain.clone(),
    });
    for leaf in &repeatable.block {
        if let Leaf::Repeatable {
            id,
            repeatable: inner,
        } = leaf
        {
            let mut deeper = chain.clone();
            deeper.push(id.clone());
            deeper.push(ITEM.to_string());
            walk(section_id, &format!("{address}/{id}"), deeper, inner, out);
        }
    }
}

/// The claim, per address: `(reserved_max, first_allowed)` as ATX level numbers.
/// Restated from `parsing.md`'s table — `###`-reserved everywhere except the two
/// contexts whose structure reaches `####`: `roadmap.milestones` (multi-slot
/// sub-labels) and the `changelog.releases` pair (nested, and the nested level
/// itself at depth 2).
fn expected() -> BTreeMap<&'static str, (usize, usize)> {
    BTreeMap::from([
        ("arch-doc.components", (3, 4)),
        ("changelog.releases", (4, 5)),
        ("changelog.releases/changes", (4, 5)),
        ("changelog.unreleased-changes", (3, 4)),
        ("commit.trailers", (3, 4)),
        ("completion-record.findings", (3, 4)),
        ("decisions-log.entries", (3, 4)),
        ("deferral-ledger.entries", (3, 4)),
        ("milestone-record.tasks", (3, 4)),
        ("prd.requirements", (3, 4)),
        ("roadmap.milestones", (4, 5)),
        ("spec.criteria", (3, 4)),
    ])
}

#[test]
fn every_shipped_item_context_derives_its_own_ceiling() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);
    let all: Vec<ItemContext> = schemas
        .iter()
        .flat_map(|(doctype, schema)| contexts(doctype, schema))
        .collect();

    // The enumerated set is the registry's, and every member has a stated claim —
    // a new doctype or nested repeatable reddens here rather than shipping
    // underived.
    let enumerated: BTreeSet<&str> = all.iter().map(|c| c.address.as_str()).collect();
    let claimed: BTreeSet<&str> = expected().keys().copied().collect();
    assert_eq!(
        enumerated, claimed,
        "every item context the composite registry ships states its ceiling (and vice versa)"
    );

    for context in &all {
        let chain: Vec<&str> = context.chain.iter().map(String::as_str).collect();
        let schema = &schemas[context
            .address
            .split('.')
            .next()
            .expect("the address leads with its doctype")];
        let derived = slot_ceiling(schema, &context.section_id, &chain)
            .unwrap_or_else(|| panic!("the `{}` context resolves", context.address));
        let (reserved_max, first_allowed) = expected()[context.address.as_str()];
        assert_eq!(
            (derived.reserved_max, derived.first_allowed),
            (reserved_max, first_allowed),
            "`{}` reserves through H{reserved_max} and first allows H{first_allowed}",
            context.address
        );
    }
}
