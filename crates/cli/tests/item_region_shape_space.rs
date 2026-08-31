//! **The item-region shape space, manufactured** (M49 Increment 1, T6) — the class
//! closed over its shape space rather than over its repro.
//!
//! The axis is `{slotless, single-slot, multi-slot} × {nested, ¬nested} × {insert,
//! update, unset}` — **eighteen cells**, each driven end to end through the real binary
//! (`doc create` → `doc add-item` → `doc set-field` / `doc set-field --unset`) and each
//! asserting **both** faces of the seam the increment repaired: the **bytes** on disk and
//! the pinned 1.0 `doc show --format json` **read-back**. A write that acks at exit 0
//! while the read returns another value is the exact contradiction
//! [`item_region_boundary`](../item_region_boundary.rs) reported; here it is refused at
//! every shape, not at the one that reported it.
//!
//! # This suite's set is a MANUFACTURED SHAPE SPACE, not a registry enumeration
//!
//! M45 and M47 established the axis-iterating pattern as *enumerate the code-side
//! registry*, and every acceptance suite since has read its members from the shipped
//! model. **This one deliberately does not, and the departure is the point.** Across
//! both embedded packs the shipped item blocks populate **two** of this axis's six
//! shapes with the ingredients the eighteen cells need: no shipped block is
//! multi-slot **and** nested, and no shipped **multi-slot** block carries a settable
//! field at all (`roadmap.milestones` is the only multi-slot block either pack ships,
//! and its three leaves are an id-source `title` and two slots). A suite that looped
//! the registry here would drive two cells, go green, and report an axis it never
//! reached — the failure mode the complete-fix contract exists to prevent, arriving
//! *through* the mechanism the contract prescribes
//! (`completions/artifacts/M49/settle-record.md` → *Acceptance — one correction to how
//! the axis is built*; `implementation/pinning.md` §1).
//!
//! So the six shapes are **built**: [`shape_schema`] emits one doctype schema per
//! shape and a [`FixturePack`] — a freeze-exempt copy of the dev pack — carries it into
//! a real corpus. The generator is the axis: a shape is a `(slots, nested)` pair, not a
//! hand-written constant, so no cell can be quietly dropped by editing one fixture.
//!
//! **The departure is fenced rather than asserted.**
//! [`the_registry_cannot_supply_this_axis`] measures the two structural facts the
//! paragraph above rests on, against the loaded schemas of both embedded packs. If a
//! pack ever ships a multi-slot ∧ nested block, or a multi-slot block carrying a real
//! field, that test reddens and this prose gets re-read — a claim about a set, fenced
//! where the set is defined, instead of a census that rots.
//!
//! # The red this suite was written against
//!
//! Driven on the shipped `1.0.0-rc.12` binary (the increment's baseline), **six of the
//! eighteen cells fail** — the whole `multi-slot` column:
//!
//!   * `{multi-slot, ¬nested}`: the second `set-field` appends a *duplicate* bullet at
//!     exit 0, `doc show --format json` returns the **first, stale** value, and the
//!     `--unset` that should clear it is refused `write.not-present`;
//!   * `{multi-slot, nested}`: the nested `add-item` is rejected, and every subsequent
//!     write over the skeleton the binary itself emitted answers `write.wrong-shape` —
//!     the writer acking bytes its own parser refuses.
//!
//! The other twelve pass on rc.12 and still pass here: the fix is complete over the
//! axis **and** inert over the shapes that were already right.
//!
//! # The `####` arms — the anchor discriminator, whose answer is schema-keyed
//!
//! A heading one level deeper than an item is not one thing, and this is where the
//! settle record's declared bound lives: what a `#### <text>` line *means* inside an
//! item depends on what that item's own template reserves at that depth
//! (`design/surface-contract.md` → *a context-dependent rule takes its context as a
//! fence input*). [`deeper_heading_codes`] states the whole answer as a function of the
//! shape, and every shape's two arms drive it:
//!
//! | shape | reserves `####` for | unanchored `#### Stray heading` | `#### Stray heading  {#Bad Id}` |
//! |---|---|---|---|
//! | any **nested** shape | anchored nested items | `conformance.item-heading-unanchored` | `conformance.item-anchor-malformed` |
//! | `{multi-slot, ¬nested}` | the item's own slot sub-labels | `conformance.item-slot-delimiter-shadowed` | *(same — an anchor cannot make a sub-label depth into an item)* |
//! | `{slotless, ¬nested}`, `{single-slot, ¬nested}` | **nothing** | *(no finding — unreserved depth is author prose)* | *(no finding)* |
//!
//! **Declared bound, carried rather than pinned.** In the last row a `####` heading is
//! legal because the schema reserves nothing there — but a *slotless* item models no
//! prose at all, so bytes an out-of-band edit puts inside one survive a read and do not
//! survive the cold-fill re-render `insert_item_field` performs (the sanctioned
//! item-bytes exception, `implementation/parsing.md` → *The item-bytes path is the one
//! sanctioned exception*). That is a **parse-side** question — whether content no leaf
//! models is conformant — not a boundary question, so it is recorded as owed
//! (`DECISIONS.md` → 2026-08-28) rather than pinned here: a standing test over the
//! current behaviour would pin the loss as expected output.

use crate::support;

use cli::pack::{CompositePack, EmbeddedPack};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, Repeatable, Schema, SectionBody};
use std::collections::BTreeSet;
use support::shape_space::{FIXTURE_WORKFLOW, SHAPES, Shape, shape_schema};
use support::trial_corpus::{FixturePack, State, TrialCorpus};

// ============================================================================
// The axis
// ============================================================================

/// The three write ops every shape is driven through, in the order a real corpus meets
/// them: the cold-fill **insert** onto an item with no field group, the surgical
/// **update** of the bullet that insert left, and the **unset** that removes it again.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    Insert,
    Update,
    Unset,
}

const OPS: [Op; 3] = [Op::Insert, Op::Update, Op::Unset];

/// One expectation over a `####` heading: the conformance code the gate must raise and
/// the offending bytes that finding must name back — or `None` where the heading draws
/// no finding at all.
type HeadingVerdict = Option<(&'static str, &'static str)>;

/// What a `####` heading inside an item answers with under `shape`, as the
/// `(unanchored, anchored-but-malformed)` pair.
///
/// This function **is** the declared bound: the anchor discriminator only decides
/// anything where the schema has reserved the depth, so the answer is a function of the
/// item's own template, never of the bytes alone.
fn deeper_heading_codes(shape: Shape) -> (HeadingVerdict, HeadingVerdict) {
    if shape.nested {
        // The depth is the nested item head: an `{#id}` anchor is what makes the line an
        // item, so its absence and its malformation are two different defects — and each
        // names the bytes it read, the heading text in one case and the bad anchor in the
        // other.
        (
            Some(("conformance.item-heading-unanchored", "Stray heading")),
            Some(("conformance.item-anchor-malformed", "{#Bad Id}")),
        )
    } else if shape.slots >= 2 {
        // The depth is the item's own slot-sub-label delimiter. No anchor can turn it
        // into an item, so both arms answer the same shadowing code over the same line.
        (
            Some(("conformance.item-slot-delimiter-shadowed", "Stray heading")),
            Some(("conformance.item-slot-delimiter-shadowed", "Stray heading")),
        )
    } else {
        (None, None)
    }
}

// ============================================================================
// Driving one shape
// ============================================================================

/// The document the whole suite addresses — one manufactured doctype, one section, one
/// item, one settable leaf.
const DOC: &str = "changelog:findings-log";
const SECTION: &str = "changelog:findings-log#findings";

/// Build a fixture corpus for `shape` with its one item minted and every declared prose
/// leaf filled, and return it with the live task id and the item address the binary
/// emitted (both read from real output, never reconstructed).
fn build(shape: Shape, pack: &FixturePack) -> (TrialCorpus, String, String) {
    let corpus = TrialCorpus::build_with_pack(State::Fresh, pack);
    let task = corpus.start_workflow("log-finding", "record a finding");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Findings log",
        "--task",
        &task,
    ]);
    let item = corpus.add_item(SECTION, "First finding", &task);
    for slot in shape.slot_ids() {
        corpus.set_slot(&format!("{item}/{slot}"), &task, &slot_prose(slot));
    }
    if shape.nested {
        let note = corpus.add_item(&format!("{item}/notes"), "First note", &task);
        // The nested note's own required slot: left unfilled it is its own finding, and
        // every cell below asserts the *absence* of findings over this doc.
        corpus.set_slot(&format!("{note}/detail"), &task, NOTE_PROSE);
    }
    (corpus, task, item)
}

/// The prose one declared slot is filled with — distinct per slot, so a cell can assert
/// that *this* leaf survived rather than that some prose did.
fn slot_prose(slot: &str) -> String {
    format!("The {slot}, in prose.")
}

const NOTE_PROSE: &str = "The note's prose.";

/// Every text fragment `shape`'s document must still carry after any of the three ops —
/// the item's own prose leaves, its multi-slot sub-labels, and its nested item with its
/// prose. A write that lands its bullet and eats one of these has not passed its cell.
fn surviving_fragments(shape: Shape) -> Vec<String> {
    let mut out = vec![String::from("### First finding  {#first-finding}")];
    for slot in shape.slot_ids() {
        out.push(slot_prose(slot));
        if shape.slots >= 2 {
            // Multi-slot items label each leaf; a single slot renders as bare prose.
            let mut label = slot.to_string();
            label[..1].make_ascii_uppercase();
            out.push(format!("#### {label}"));
        }
    }
    if shape.nested {
        out.push(String::from("#### First note  {#first-note}"));
        out.push(String::from(NOTE_PROSE));
    }
    out
}

/// The staged file's raw bytes — read off disk, not through a renderer, so a byte claim
/// is a byte claim.
fn staged_bytes(corpus: &TrialCorpus, task: &str) -> String {
    std::fs::read_to_string(
        corpus
            .repo()
            .join(format!(".jigc/tasks/{task}/docs/{DOC}.md")),
    )
    .expect("read the staged copy")
}

/// The `- status:` bullets in the staged bytes. The count is the corruption's
/// byte-level face: two writes must leave one bullet, never two.
fn status_bullets(bytes: &str) -> Vec<&str> {
    bytes
        .lines()
        .filter(|line| line.trim_start().starts_with("- status:"))
        .collect()
}

/// One `jigc` invocation's success bit plus both streams.
fn run(corpus: &TrialCorpus, args: &[&str]) -> (bool, String) {
    let out = corpus.jigc(args);
    (
        out.status.success(),
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        ),
    )
}

/// `jigc task validate <task>`'s findings that name **this** document — the task also
/// carries its own unauthored `commit` doc, whose findings are not this suite's subject.
fn findings_over_the_doc(corpus: &TrialCorpus, task: &str) -> Vec<String> {
    let (_, text) = run(corpus, &["task", "validate", task]);
    text.lines()
        .filter(|line| line.contains(DOC) && !line.trim_start().starts_with("route:"))
        .map(str::to_string)
        .collect()
}

/// Assert the document still parses and the gate raises nothing over it — the invariant
/// every cell shares, checked after every op.
fn assert_doc_is_whole(corpus: &TrialCorpus, task: &str, shape: Shape, op: Op, bytes: &str) {
    let label = shape.label();
    let (ok, text) = run(corpus, &["doc", "show", DOC, "--task", task]);
    assert!(
        ok,
        "[{label}/{op:?}] a doc jigc itself just wrote must read back at exit 0 — a \
         reject here is the writer acking bytes its own parser refuses:\n{text}",
    );
    let findings = findings_over_the_doc(corpus, task);
    assert!(
        findings.is_empty(),
        "[{label}/{op:?}] the gate must raise nothing over the written doc: \
         {findings:?}\n--- staged bytes ---\n{bytes}",
    );
    for fragment in surviving_fragments(shape) {
        assert!(
            bytes.contains(&fragment),
            "[{label}/{op:?}] the write ate content it does not address — {fragment:?} \
             is gone:\n{bytes}",
        );
    }
}

/// Drive one shape through all three cells of its column.
fn drive_shape(shape: Shape) {
    let label = shape.label();
    let pack = FixturePack::from_dev_pack(&format!("shape-{label}"));
    pack.write_schema("changelog", &shape_schema(shape))
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let (corpus, task, item) = build(shape, &pack);
    let address = format!("{item}/status");

    // The state the three cells start from, and the state `unset` must return the
    // document to: `--unset` is a splice-remove, so clause 2 of the round-trip
    // guarantees ("only that target's bytes differ") makes insert-then-unset a byte
    // round trip (`implementation/parsing.md` → Round-trip guarantees).
    let minted = staged_bytes(&corpus, &task);
    assert!(
        status_bullets(&minted).is_empty(),
        "[{label}] precondition: a freshly minted item carries no `status` bullet:\n{minted}",
    );
    // The `####` arms run over this same pre-op state, copied before anything mutates it.
    let unanchored_corpus = corpus.copy_state();
    let malformed_corpus = corpus.copy_state();

    for op in OPS {
        match op {
            Op::Insert => {
                // The cold-fill insert: the item carries no field group yet, so this is
                // the arm that re-renders the item whole.
                let (ok, text) = run(
                    &corpus,
                    &[
                        "doc",
                        "set-field",
                        &address,
                        "--value",
                        "draft",
                        "--task",
                        &task,
                    ],
                );
                assert!(
                    ok,
                    "[{label}/insert] the cold-fill insert must land:\n{text}"
                );
                let bytes = staged_bytes(&corpus, &task);
                assert_eq!(
                    status_bullets(&bytes),
                    vec!["- status: draft"],
                    "[{label}/insert] exactly one bullet, carrying the written value:\n{bytes}",
                );
                assert_eq!(
                    read_json(&corpus, &address, &task),
                    "\"draft\"",
                    "[{label}/insert] the pinned read returns what the write acked",
                );
                assert_doc_is_whole(&corpus, &task, shape, op, &bytes);
            }
            Op::Update => {
                // The surgical update: the bullet is present, so this is the splice arm —
                // and the arm whose failure produced M49's reported corruption (a second
                // bullet, and a read that contradicted its own ack).
                let (ok, text) = run(
                    &corpus,
                    &[
                        "doc",
                        "set-field",
                        &address,
                        "--value",
                        "done",
                        "--task",
                        &task,
                    ],
                );
                assert!(
                    ok,
                    "[{label}/update] the surgical update must land:\n{text}"
                );
                let bytes = staged_bytes(&corpus, &task);
                assert_eq!(
                    status_bullets(&bytes),
                    vec!["- status: done"],
                    "[{label}/update] a second write must EDIT the bullet, never append a \
                     duplicate:\n{bytes}",
                );
                assert_eq!(
                    read_json(&corpus, &address, &task),
                    "\"done\"",
                    "[{label}/update] the pinned read returns the value the LAST write \
                     acked, never the stale first one",
                );
                assert_doc_is_whole(&corpus, &task, shape, op, &bytes);
            }
            Op::Unset => {
                let (ok, text) = run(
                    &corpus,
                    &["doc", "set-field", &address, "--unset", "--task", &task],
                );
                assert!(ok, "[{label}/unset] the removal must land:\n{text}");
                let bytes = staged_bytes(&corpus, &task);
                assert!(
                    status_bullets(&bytes).is_empty(),
                    "[{label}/unset] the bullet is gone:\n{bytes}",
                );
                assert_eq!(
                    bytes, minted,
                    "[{label}/unset] insert-then-unset is a byte round trip: the \
                     splice-remove owns the field group's own lines and nothing else \
                     (round-trip clause 2 — only the target's bytes differ)",
                );
                // The read-back agrees the leaf is gone rather than serving a stale value.
                let (ok, text) = run(
                    &corpus,
                    &["doc", "show", &address, "--task", &task, "--format", "json"],
                );
                assert!(
                    !ok && text.contains("store.no-such-leaf"),
                    "[{label}/unset] the pinned read must report the leaf absent, not \
                     serve the removed value:\n{text}",
                );
                assert_doc_is_whole(&corpus, &task, shape, op, &bytes);
            }
        }
    }

    // ---- the two `####` arms, over the shape's own reserved-depth answer ----
    let (unanchored, malformed) = deeper_heading_codes(shape);
    assert_deeper_heading(
        &unanchored_corpus,
        &task,
        shape,
        "\n#### Stray heading\n",
        unanchored,
    );
    assert_deeper_heading(
        &malformed_corpus,
        &task,
        shape,
        "\n#### Stray heading  {#Bad Id}\n",
        malformed,
    );
}

/// The pinned `--format json` read of one field address, trimmed.
fn read_json(corpus: &TrialCorpus, address: &str, task: &str) -> String {
    corpus
        .jigc_ok(&["doc", "show", address, "--task", task, "--format", "json"])
        .trim()
        .to_string()
}

/// Append `line` to the staged document by hand — the out-of-band edit no verb performs
/// — and assert the gate answers with `expected`, or with nothing at all where the
/// shape reserves nothing at that depth.
fn assert_deeper_heading(
    corpus: &TrialCorpus,
    task: &str,
    shape: Shape,
    line: &str,
    expected: HeadingVerdict,
) {
    let label = shape.label();
    let path = corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/{DOC}.md"));
    let mut bytes = std::fs::read_to_string(&path).expect("read the staged copy");
    bytes.push_str(line);
    std::fs::write(&path, &bytes).expect("write the hand-edited staged copy");

    let findings = findings_over_the_doc(corpus, task);
    match expected {
        Some((code, quoted)) => {
            let matching: Vec<&String> = findings.iter().filter(|f| f.contains(code)).collect();
            assert_eq!(
                matching.len(),
                1,
                "[{label}] `{}` must answer with exactly one `{code}` — the shape \
                 reserves `####`, so the heading is adjudicated, never guessed at: \
                 {findings:?}",
                line.trim(),
            );
            assert!(
                matching[0].contains(quoted),
                "[{label}] the finding names the offending bytes ({quoted:?}) back — a \
                 located conformance code is its own route: {}",
                matching[0],
            );
            // The item's own declared structure still parses around the stray line.
            assert!(
                !findings
                    .iter()
                    .any(|f| f.contains("conformance.item-slot-label-missing")),
                "[{label}] the item's own sub-labels are not re-read as broken: {findings:?}",
            );
        }
        None => assert!(
            findings.iter().all(|f| !f.contains("conformance.")),
            "[{label}] this shape's template reserves nothing at `####`, so a deeper \
             heading is ordinary author prose and the gate must not manufacture a \
             finding over it: {findings:?}",
        ),
    }
}

// ============================================================================
// The eighteen cells — one test per shape, three cells each
// ============================================================================

#[test]
fn slotless_flat_insert_update_unset() {
    drive_shape(SHAPES[0]);
}

#[test]
fn slotless_nested_insert_update_unset() {
    drive_shape(SHAPES[1]);
}

#[test]
fn single_slot_flat_insert_update_unset() {
    drive_shape(SHAPES[2]);
}

#[test]
fn single_slot_nested_insert_update_unset() {
    drive_shape(SHAPES[3]);
}

#[test]
fn multi_slot_flat_insert_update_unset() {
    drive_shape(SHAPES[4]);
}

#[test]
fn multi_slot_nested_insert_update_unset() {
    drive_shape(SHAPES[5]);
}

/// The space is the whole cartesian product, and it is eighteen cells — derived from
/// the two enumerations rather than stated as a number a later edit could falsify.
#[test]
fn the_space_is_the_whole_cartesian_product() {
    let built: BTreeSet<(usize, bool)> = SHAPES.iter().map(|s| (s.slots, s.nested)).collect();
    let expected: BTreeSet<(usize, bool)> = [0usize, 1, 2]
        .into_iter()
        .flat_map(|slots| [false, true].into_iter().map(move |nested| (slots, nested)))
        .collect();
    assert_eq!(
        built, expected,
        "the shape axis is `{{slotless, single-slot, multi-slot}} × {{nested, ¬nested}}` \
         — every pair present exactly once",
    );
    assert_eq!(
        SHAPES.len() * OPS.len(),
        18,
        "the space is eighteen cells: six shapes × three write ops",
    );
    // Each shape has its own test above, so a shape added to the axis without a driver
    // cannot hide behind the count.
    let labels: BTreeSet<String> = SHAPES.iter().map(|s| s.label()).collect();
    assert_eq!(
        labels.len(),
        SHAPES.len(),
        "every shape is distinctly labelled"
    );
}

// ============================================================================
// The fence under the departure
// ============================================================================

/// One shipped item block: where it lives, the leaf its `{#id}` anchor is slugged from,
/// and its leaves.
struct ShippedBlock {
    /// `<doctype>#<section>[/<nested leaf>]…` — the label an assertion prints.
    path: String,
    /// The block's `id-from` leaf id. It is a field, but it is the item's *identity*
    /// rather than a settable value, so the census below excludes it.
    id_from: String,
    leaves: Vec<Leaf>,
}

/// Every item block either embedded pack ships, top-level and nested.
fn shipped_item_blocks() -> Vec<ShippedBlock> {
    let pack = CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ]);
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .expect("a listed schema reads back");
        let schema: Schema =
            cli::pack::load_pack_schema(&pack, &bytes).expect("a shipped schema parses");
        for section in &schema.sections {
            if let SectionBody::Repeatable { repeatable } = &section.body {
                collect_blocks(
                    &format!("{}#{}", schema.ty, section.id),
                    repeatable,
                    &mut out,
                );
            }
        }
    }
    out
}

/// Record `repeatable`'s own block and recurse into every repeatable it nests.
fn collect_blocks(path: &str, repeatable: &Repeatable, out: &mut Vec<ShippedBlock>) {
    out.push(ShippedBlock {
        path: path.to_string(),
        id_from: repeatable.id_from.clone(),
        leaves: repeatable.block.clone(),
    });
    for leaf in &repeatable.block {
        if let Leaf::Repeatable { id, repeatable } = leaf {
            collect_blocks(&format!("{path}/{id}"), repeatable, out);
        }
    }
}

/// **The departure's fence.** The two structural facts that make this axis unreachable
/// from the shipped registry, measured against the loaded schemas of both embedded packs
/// rather than asserted in prose. A pack that grows into the shape space reddens this
/// test, and the module doc above gets re-read instead of rotting.
#[test]
fn the_registry_cannot_supply_this_axis() {
    let blocks = shipped_item_blocks();
    assert!(
        !blocks.is_empty(),
        "the packs ship repeatable sections; an empty census means the walk is broken, \
         not that the fact holds",
    );

    let mut multi_slot_and_nested = Vec::new();
    let mut multi_slot_with_a_field = Vec::new();
    for block in &blocks {
        let slots = block
            .leaves
            .iter()
            .filter(|leaf| matches!(leaf, Leaf::Slot { .. }))
            .count();
        if slots < 2 {
            continue;
        }
        if block
            .leaves
            .iter()
            .any(|leaf| matches!(leaf, Leaf::Repeatable { .. }))
        {
            multi_slot_and_nested.push(block.path.clone());
        }
        if block.leaves.iter().any(|leaf| match leaf {
            Leaf::Field(field) => field.id != block.id_from,
            _ => false,
        }) {
            multi_slot_with_a_field.push(block.path.clone());
        }
    }

    assert!(
        multi_slot_and_nested.is_empty(),
        "a shipped item block is now multi-slot AND nested ({multi_slot_and_nested:?}) — \
         the registry has grown into this suite's shape space, so the manufactured-axis \
         rationale in this module's doc comment must be re-read (and that block's own \
         corpus cells now belong in a registry-iterating suite)",
    );
    assert!(
        multi_slot_with_a_field.is_empty(),
        "a shipped multi-slot item block now carries a settable field \
         ({multi_slot_with_a_field:?}) — the shape whose write path M49 repaired is live \
         in the corpus; re-read this module's doc comment and pin the shipped instance too",
    );
}
