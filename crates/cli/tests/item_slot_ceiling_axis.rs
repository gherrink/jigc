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
//! (`commit.trailers`, `milestone-record.tasks`) and the slotless-but-nested
//! `changelog.releases`: the ceiling is a property of the
//! *address*, and `changelog.releases` is the only `has_nested` witness either pack
//! ships — the shipped half of the row no doctype exercises through a slot.
//!
//! **The second sweep drives the shipped verb, not the derivation** (M45 Inc 2
//! T7). `slot_ceiling` returning the right pair proves nothing about what an agent
//! typing `jigc doc set-slot` meets, so the same registry enumeration is walked a
//! second time *through the built binary*: for every doctype × item-slot context,
//! a write carrying an ATX heading at **each** reserved depth is refused and a
//! write at the **first allowed** depth lands. A context is reached through its
//! doctype's own gate-granting `migrate-<doctype>` workflow, itself looked up in
//! the registry rather than hand-mapped; a context that no gate can reach, or one
//! whose item block carries no prose slot, is **named** in [`UNREACHABLE`] /
//! [`SLOTLESS`] with its reason and asserted against the computed set — never
//! silently dropped ([`implementation/increment-workflow.md`] → Validation
//! hardening #4, the M38 below-the-gate mask).
//!
//! [`implementation/parsing.md`]: ../../../implementation/parsing.md
//! [`implementation/pinning.md`]: ../../../implementation/pinning.md
//! [`implementation/increment-workflow.md`]: ../../../implementation/increment-workflow.md

use crate::support;

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::compose::load_workflow_def;
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{FieldType, Leaf, Repeatable, Schema, SectionBody};
use engine::write::slot_ceiling;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use support::trial_corpus::{State, TrialCorpus};

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
    doctype: String,
    section_id: String,
    chain: Vec<String>,
    /// The nested-repeatable leaf ids between the section and this level — empty
    /// for a depth-1 context, `["changes"]` for `changelog.releases/changes`. The
    /// write-verb sweep needs these to *build* the address it writes to.
    nested: Vec<String>,
    /// One item title per hop, derived from that level's `id-from` field: an enum
    /// id-source takes its first declared member, any other takes
    /// [`STRING_ITEM_TITLE`]. Derived, so a new enum-keyed repeatable joins the
    /// sweep without a hand-written title.
    titles: Vec<String>,
    /// The prose-slot leaf ids of this level's item block, in document order. An
    /// empty set makes the context [`SLOTLESS`] — the ceiling still derives, but
    /// there is no slot write to gate.
    slots: Vec<String>,
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
            doctype,
            &section.id,
            &format!("{doctype}.{}", section.id),
            vec![ITEM.to_string()],
            Vec::new(),
            Vec::new(),
            repeatable,
            &mut out,
        );
    }
    out
}

/// Emit the context for one repeatable level, then descend into each nested
/// repeatable its item block declares (the chain gains the nested-section id plus
/// one more item hop per level).
#[allow(clippy::too_many_arguments)]
fn walk(
    doctype: &str,
    section_id: &str,
    address: &str,
    chain: Vec<String>,
    nested: Vec<String>,
    titles: Vec<String>,
    repeatable: &Repeatable,
    out: &mut Vec<ItemContext>,
) {
    let mut titles = titles;
    titles.push(item_title(repeatable));
    out.push(ItemContext {
        address: address.to_string(),
        doctype: doctype.to_string(),
        section_id: section_id.to_string(),
        chain: chain.clone(),
        nested: nested.clone(),
        titles: titles.clone(),
        slots: repeatable
            .block
            .iter()
            .filter_map(|leaf| match leaf {
                Leaf::Slot { id, .. } => Some(id.clone()),
                _ => None,
            })
            .collect(),
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
            let mut deeper_nested = nested.clone();
            deeper_nested.push(id.clone());
            walk(
                doctype,
                section_id,
                &format!("{address}/{id}"),
                deeper,
                deeper_nested,
                titles.clone(),
                inner,
                out,
            );
        }
    }
}

/// The item title `jigc doc add-item` is driven with at one repeatable level,
/// **derived from the level's own `id-from` field** rather than hand-written: an
/// `enum` id-source only accepts a declared member, anything else takes a free
/// string. A repeatable whose id-source is an enum with no members would be a
/// schema defect, so it panics rather than guessing.
fn item_title(repeatable: &Repeatable) -> String {
    for leaf in &repeatable.block {
        let Leaf::Field(field) = leaf else { continue };
        if field.id != repeatable.id_from {
            continue;
        }
        if matches!(field.ty, FieldType::Enum) {
            return field
                .of
                .as_ref()
                .and_then(|members| members.first())
                .unwrap_or_else(|| {
                    panic!(
                        "the `{}` id-source is an enum and must declare members",
                        field.id
                    )
                })
                .clone();
        }
        break;
    }
    STRING_ITEM_TITLE.to_string()
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

// ---------------------------------------------------------------------------
// The shipped-verb sweep (M45 Inc 2 T7)
// ---------------------------------------------------------------------------

/// The doc title every swept instance is created under — for the doctypes whose title
/// is the author's. A singleton's `# H1` is the schema's own (`changelog` pins its via
/// `display-title`), and since M48 a divergent `--title` there is **refused** rather
/// than silently dropped (`write.title-ignored`), so the sweep resolves each doctype's
/// title through [`support::create_title`] instead of passing this to all of them.
const DOC_TITLE: &str = "Axis Sweep";

/// The item title used wherever the level's `id-from` is a free string.
const STRING_ITEM_TITLE: &str = "Axis Item";

/// Enumerated contexts whose item block carries **no prose slot** — the ceiling
/// still derives for them (the first sweep asserts it), but there is no slot write
/// for the shipped verb to gate, so they are excluded *by name and reason* rather
/// than by a filter a reader has to trust.
const SLOTLESS: &[(&str, &str)] = &[
    (
        "changelog.releases",
        "a release item is scalar fields plus the nested `changes` repeatable — a \
         leading prose slot would swallow the nested groups (crates/cli/pack/schemas/\
         changelog.yaml, review finding B1). It is still exercised, as the PARENT \
         the `changelog.releases/changes` context is built under.",
    ),
    (
        "commit.trailers",
        "field-only: a trailer is a key/value pair, no prose.",
    ),
    (
        "milestone-record.tasks",
        "field-only: a joined sub-task row is typed fields, no prose.",
    ),
];

/// Item-slot contexts no shipped workflow can reach through a create gate. **Empty
/// today** — each of the 7 item-slot doctypes has its own `migrate-<doctype>`
/// workflow granting the gate — and asserted empty, so a doctype that ever loses
/// its door reddens here and must be written down with its reason instead of
/// quietly dropping out of the sweep.
const UNREACHABLE: &[(&str, &str)] = &[];

/// The `migrate-*` workflow granting each doctype's create gate, read out of the
/// composite registry — the sweep's door, never a hand-written doctype→workflow
/// map. `jigc migrate <path> --as <doctype>` is the verb that composes it.
fn migrate_gates(pack: &dyn PackSource) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Workflows) {
        if !id.as_str().starts_with("migrate-") {
            continue;
        }
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .unwrap_or_else(|e| panic!("read the `{id}` workflow: {e}"));
        let def = load_workflow_def(&bytes)
            .unwrap_or_else(|e| panic!("load the `{id}` workflow: {}", e.message));
        for gate in &def.allows_create {
            out.insert(gate.doc_type.clone(), id.as_str().to_string());
        }
    }
    out
}

/// Slot prose carrying one ATX heading at `depth` — the corrupting payload.
fn prose_at(depth: usize) -> String {
    format!(
        "Axis prose.\n\n{} Ghost  {{#ghost}}\n\ntrailing prose\n",
        "#".repeat(depth)
    )
}

/// The task id a mint printed, read off the binary's own `task minted: <id>` line.
fn minted(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// Build (or reuse) the item chain a context addresses, driving `jigc doc add-item`
/// once per hop and carrying the address the binary **emitted** forward — never a
/// test-side reconstruction of the slug rule.
fn ensure_item(
    corpus: &TrialCorpus,
    minted_items: &mut BTreeMap<String, String>,
    doc_id: &str,
    task: &str,
    context: &ItemContext,
) -> String {
    let mut address = format!("{doc_id}#{}", context.section_id);
    for (hop, title) in context.titles.iter().enumerate() {
        if hop > 0 {
            address = format!("{address}/{}", context.nested[hop - 1]);
        }
        let key = format!("{address}|{title}");
        address = match minted_items.get(&key) {
            Some(existing) => existing.clone(),
            None => {
                let emitted = corpus
                    .jigc_ok(&[
                        "doc", "add-item", &address, "--title", title, "--task", task,
                    ])
                    .trim()
                    .to_string();
                minted_items.insert(key, emitted.clone());
                emitted
            }
        };
    }
    address
}

/// **The axis, through the door an agent actually uses.**
///
/// For every doctype × item-slot context the composite registry ships, a slot write
/// carrying a heading at *each* reserved depth is refused by the real binary, and a
/// write at the first allowed depth lands. Nothing here is hand-listed: the contexts
/// come from the engine-loaded schemas, the item titles from each level's `id-from`,
/// the create gate from the registry's `migrate-*` workflows — so a new doctype with
/// an item slot is swept the day it lands.
#[test]
fn every_item_slot_context_is_gated_at_every_reserved_depth_through_the_shipped_verb() {
    let pack = composite();
    let schemas = loaded_schemas(&pack);
    let all: Vec<ItemContext> = schemas
        .iter()
        .flat_map(|(doctype, schema)| contexts(doctype, schema))
        .collect();

    // Nothing is dropped silently: the slotless partition is stated with reasons.
    let slotless: BTreeSet<&str> = all
        .iter()
        .filter(|c| c.slots.is_empty())
        .map(|c| c.address.as_str())
        .collect();
    assert_eq!(
        slotless,
        SLOTLESS.iter().map(|(a, _)| *a).collect::<BTreeSet<&str>>(),
        "every slotless item context is named in SLOTLESS with its reason"
    );

    let swept: Vec<&ItemContext> = all.iter().filter(|c| !c.slots.is_empty()).collect();
    let doctypes: BTreeSet<&str> = swept.iter().map(|c| c.doctype.as_str()).collect();

    // …and neither is an unreachable one: a context with no create-gate door is
    // named, never quietly excluded because it is awkward to drive.
    let gates = migrate_gates(&pack);
    let unreachable: BTreeSet<&str> = doctypes
        .iter()
        .copied()
        .filter(|doctype| !gates.contains_key(*doctype))
        .collect();
    assert_eq!(
        unreachable,
        UNREACHABLE
            .iter()
            .map(|(d, _)| *d)
            .collect::<BTreeSet<&str>>(),
        "every item-slot doctype with no gate-granting `migrate-*` workflow is named \
         in UNREACHABLE with its reason"
    );

    let corpus = TrialCorpus::build(State::Fresh);
    fs::create_dir_all(corpus.repo().join("docs")).expect("create the foreign source dir");
    for doctype in &doctypes {
        fs::write(
            corpus.repo().join(format!("docs/legacy-{doctype}.md")),
            format!("# Legacy {doctype}\n\nfree-form prose the migration rewrites.\n"),
        )
        .expect("write a foreign source");
    }
    corpus.git(&["add", "docs"]);
    corpus.git(&["commit", "-q", "-m", "the foreign sources"]);

    for doctype in &doctypes {
        // The door: the doctype's own `migrate-*` workflow, composed by the verb
        // that routes to it. The workflow id is read back from the task the binary
        // minted, so the registry lookup above is proven to be the door taken.
        let task = minted(&corpus.jigc_ok(&[
            "migrate",
            &format!("docs/legacy-{doctype}.md"),
            "--as",
            doctype,
        ]));
        assert_eq!(
            fs::read_to_string(corpus.repo().join(format!(".jigc/tasks/{task}/workflow")))
                .expect("the task records its workflow")
                .trim(),
            gates[*doctype],
            "`jigc migrate --as {doctype}` composes the gate-granting workflow"
        );
        let doc_id = corpus
            .jigc_ok(&[
                "doc",
                "create",
                doctype,
                "--title",
                &support::create_title(doctype, DOC_TITLE),
                "--task",
                &task,
            ])
            .trim()
            .to_string();

        let mut minted_items = BTreeMap::new();
        for context in swept.iter().filter(|c| c.doctype == *doctype) {
            let item = ensure_item(&corpus, &mut minted_items, &doc_id, &task, context);
            let chain: Vec<&str> = context.chain.iter().map(String::as_str).collect();
            let ceiling = slot_ceiling(&schemas[*doctype], &context.section_id, &chain)
                .unwrap_or_else(|| panic!("the `{}` context resolves", context.address));

            for slot in &context.slots {
                let address = format!("{item}/{slot}");
                for depth in 1..=ceiling.reserved_max {
                    let out = corpus.jigc_stdin(
                        &[
                            "doc",
                            "set-slot",
                            &address,
                            "--from-file",
                            "-",
                            "--task",
                            &task,
                        ],
                        &prose_at(depth),
                    );
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    assert!(
                        !out.status.success(),
                        "`{address}` must refuse a heading at H{depth} (reserved through \
                         H{}):\n{stderr}",
                        ceiling.reserved_max,
                    );
                    assert!(
                        stderr.contains("write.slot-heading-depth"),
                        "`{address}` at H{depth} refuses as the ceiling reject, not as \
                         something else:\n{stderr}",
                    );
                }
                corpus.jigc_stdin_ok(
                    &[
                        "doc",
                        "set-slot",
                        &address,
                        "--from-file",
                        "-",
                        "--task",
                        &task,
                    ],
                    &prose_at(ceiling.first_allowed),
                );
            }
        }
    }
}
