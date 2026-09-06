//! **The missing-sub-label finding names the depth the writer actually emits** (M50
//! Increment 7, T2 — N30).
//!
//! `parse_item_slots` matches a multi-slot item's `<#…> <Leaf-Title>` sub-labels at
//! `item_level + 1` — the level [`engine::write`]'s `render_item_at` emits them at, and
//! the level [`engine::write::slot_ceiling`] reserves through for a multi-slot template.
//! Its **message**, though, spelled the hashes as a literal `####`, which is right at the
//! shallowest locus and wrong at every deeper one: at a *nested* multi-slot item the
//! writer emits `##### Proves`, so the finding instructed the author to author `####
//! Proves` — a heading at the schema-reserved item depth, which
//! `conformance.item-heading-unanchored` then blocks and `write.slot-heading-depth`
//! refuses on the write path. Following the instruction verbatim left the original
//! finding standing **and** minted a second one: the message made the document worse, not
//! merely un-followable (M45's *statement == constant* rule — `implementation/pinning.md`
//! §3 — applied to a message that instructs an author).
//!
//! **The axis is the recursion, not the reported instance.** The loci are
//! `1..=MAX_NESTING_DEPTH` — the derived nesting cap
//! ([`engine::schema::MAX_NESTING_DEPTH`], itself derived from the address budget) — and
//! the fixture doctype is *generated* from that number with a **multi-slot item block at
//! every level**, so a deeper locus joins the sweep with no edit here. The shipped packs
//! cannot supply this axis: `roadmap.milestones` is the only multi-slot block either pack
//! declares and it does not nest, so locus 2 exists in no shipped corpus (the same
//! manufactured-shape-space reasoning `support::shape_space` records).
//!
//! **Nothing here types a heading level.** Each locus's expected depth comes from
//! `write::slot_ceiling` — for a multi-slot template that is exactly
//! `item_heading_level(depth) + 1`, the expression `render_item_at` computes its
//! `leaf_hashes` from — and the suite first asserts the **committed bytes the binary
//! emitted** carry that heading, so a drift between the two writer-side seams reddens
//! before the message is examined at all.

use crate::support;

use engine::schema::MAX_NESTING_DEPTH;
use support::shape_space::FIXTURE_WORKFLOW;
use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// The section every generated fixture declares, and the address its items hang off.
const SECTION: &str = "findings";

/// The leaf whose sub-label each locus removes — the item template's **second** slot, so
/// its sibling stays present and the item remains a live multi-slot block.
const LEAF: &str = "proves";

/// The leaf id as the writer title-cases it into a sub-heading.
const LEAF_TITLE: &str = "Proves";

/// The committed home of the fixture doctype — a `placement` file, so the path is a
/// constant of the schema rather than a docs-root derivation.
const HOME: &str = "CHANGELOG.md";

/// A doctype nesting `depth` levels of repeatable whose item block is **multi-slot at
/// every level** — generated from the depth, so the fixture follows
/// [`MAX_NESTING_DEPTH`] instead of pinning a shape.
fn multi_slot_nested_schema(depth: usize) -> String {
    let mut yaml = format!(
        "type: changelog
placement: {{ file: {HOME} }}
display-title: Changelog
singleton: true
id-from: title
description: A manufactured findings log whose item block is multi-slot at every nesting level.
usage: the missing-sub-label message needs a multi-slot item block at every locus the recursion reaches.
sections:
  - id: {SECTION}
    repeatable:
      id-from: label
      block:
        - {{ id: label, type: string }}
"
    );
    // Each further level indents by six: `- id: nested` inside the parent block, then its
    // own `repeatable:` / `id-from:` / `block:` (the generator shape `flow50_acceptance`
    // uses for the depth axis).
    let mut indent = 8;
    for level in 1..=depth {
        let pad = " ".repeat(indent);
        for slot in ["statement", LEAF] {
            yaml.push_str(&format!(
                "{pad}- {{ id: {slot}, slot: {{ hint: \"The {slot}.\" }} }}\n"
            ));
        }
        if level < depth {
            yaml.push_str(&format!(
                "{pad}- id: nested\n{pad}  repeatable:\n{pad}    id-from: label\n\
                 {pad}    block:\n{pad}      - {{ id: label, type: string }}\n"
            ));
            indent += 6;
        }
    }
    yaml
}

/// Build the fixture corpus, author one item at **every** locus with both prose leaves
/// filled, and finalize — so the committed `CHANGELOG.md` carries the writer's own
/// sub-labels at every depth. Returns the corpus and the item address the binary emitted
/// at each locus, shallowest first.
fn committed_corpus(pack: &FixturePack) -> (TrialCorpus, Vec<String>) {
    let corpus = TrialCorpus::build_with_pack(State::Fresh, pack);
    let task = corpus.start_workflow("log-finding", "record a finding at every locus");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "changelog",
        "--title",
        "Changelog",
        "--task",
        &task,
    ]);

    let mut addresses = Vec::new();
    let mut section = format!("changelog:changelog#{SECTION}");
    for level in 1..=MAX_NESTING_DEPTH {
        let item = corpus.add_item(&section, &format!("Level {level}"), &task);
        for slot in ["statement", LEAF] {
            corpus.set_slot(
                &format!("{item}/{slot}"),
                &task,
                &format!("The {slot} at level {level}."),
            );
        }
        section = format!("{item}/nested");
        addresses.push(item);
    }
    corpus.finalize(&task, "changelog", "record a finding at every locus", false);
    (corpus, addresses)
}

/// The `(section_id, item-chain)` [`engine::write::slot_ceiling`] takes, read off the
/// address the binary **emitted** rather than reconstructed from the fixture's titles.
fn ceiling_input(address: &str) -> (String, Vec<String>) {
    let fragment = address.split_once('#').expect("an addressed item").1;
    let mut hops = fragment.split('/').map(str::to_owned);
    let section = hops.next().expect("a section hop");
    (section, hops.collect())
}

/// Every `<heading>` the findings surface spells inside backticks — the message's own
/// bytes, never a reconstruction of them.
fn quoted_heading(line: &str) -> Option<String> {
    line.split('`')
        .skip(1)
        .step_by(2)
        .find(|quoted| quoted.starts_with('#'))
        .map(str::to_owned)
}

/// The finding lines `jigc validate` printed carrying `code`.
fn findings<'a>(surface: &'a str, code: &str) -> Vec<&'a str> {
    surface.lines().filter(|line| line.contains(code)).collect()
}

/// **The whole locus axis.** At every nesting locus the recursion reaches, the
/// missing-sub-label message names the depth the writer emits — and authoring the heading
/// it names clears the finding without minting `conformance.item-heading-unanchored`.
#[test]
fn the_missing_sub_label_message_names_the_depth_the_writer_emits_at_every_locus() {
    let yaml = multi_slot_nested_schema(MAX_NESTING_DEPTH);
    let pack = FixturePack::from_dev_pack("item-slot-label-depth");
    pack.write_schema("changelog", &yaml)
        .write_workflow("log-finding", FIXTURE_WORKFLOW);
    let schema = engine::schema::load_schema(yaml.as_bytes()).expect("the fixture schema loads");
    let (corpus, addresses) = committed_corpus(&pack);

    let home = corpus.repo().join(HOME);
    let pristine = std::fs::read_to_string(&home).expect("the committed fixture doc");

    for (index, address) in addresses.iter().enumerate() {
        let locus = index + 1;
        // ── The expected depth, DERIVED from the writer's own computation ───────
        // For a multi-slot template `slot_ceiling` reserves `item_heading_level(depth)
        // + 1` — the very expression `render_item_at` renders its `leaf_hashes` from.
        let (section, chain) = ceiling_input(address);
        let hops: Vec<&str> = chain.iter().map(String::as_str).collect();
        let level = engine::write::slot_ceiling(&schema, &section, &hops)
            .unwrap_or_else(|| {
                panic!("locus {locus}: the emitted chain resolves against the schema")
            })
            .reserved_max;
        let expected = format!("{} {LEAF_TITLE}", "#".repeat(level));

        // ── The two writer-side seams agree, on the committed bytes ─────────────
        let emitted: Vec<usize> = pristine
            .lines()
            .enumerate()
            .filter(|(_, line)| *line == expected)
            .map(|(at, _)| at)
            .collect();
        assert_eq!(
            emitted.len(),
            1,
            "locus {locus}: the binary's own committed bytes must carry exactly one \
             `{expected}` sub-label — the depth `write::slot_ceiling` reserves is the \
             depth `render_item_at` emits.\n--- committed ---\n{pristine}",
        );

        // ── Remove that sub-label out of band; the finding fires ────────────────
        let mut lines: Vec<&str> = pristine.lines().collect();
        let at = emitted[0];
        lines.remove(at);
        std::fs::write(&home, format!("{}\n", lines.join("\n"))).expect("write the malformed doc");

        let surface = corpus.jigc_ok(&["validate"]);
        let raised = findings(&surface, "conformance.item-slot-label-missing");
        assert_eq!(
            raised.len(),
            1,
            "locus {locus}: removing the `{expected}` sub-label must raise exactly one \
             `conformance.item-slot-label-missing`.\n--- validate ---\n{surface}",
        );
        let named = quoted_heading(raised[0]).unwrap_or_else(|| {
            panic!(
                "locus {locus}: the message must name a heading:\n{}",
                raised[0]
            )
        });
        assert_eq!(
            named, expected,
            "locus {locus}: the message must name the heading the writer emits at this \
             depth, not a global `####` — the level is `item_level + 1`, which is \
             `{level}` here.\n--- validate ---\n{surface}",
        );

        // ── Author the heading the message names; the finding clears ────────────
        // The bytes authored are the message's OWN, spliced back verbatim: what an
        // author following the instruction to the letter would write.
        let mut repaired: Vec<&str> = lines.clone();
        repaired.insert(at, &named);
        std::fs::write(&home, format!("{}\n", repaired.join("\n")))
            .expect("write the repaired doc");

        let surface = corpus.jigc_ok(&["validate"]);
        assert!(
            findings(&surface, "conformance.item-slot-label-missing").is_empty(),
            "locus {locus}: authoring `{named}` — the heading the message named — must \
             clear the finding that asked for it.\n--- validate ---\n{surface}",
        );
        assert!(
            findings(&surface, "conformance.item-heading-unanchored").is_empty(),
            "locus {locus}: authoring `{named}` must not mint a SECOND blocking finding \
             — an instruction whose repair the next door refuses makes the document \
             worse, not merely un-followable.\n--- validate ---\n{surface}",
        );

        std::fs::write(&home, &pristine).expect("restore the committed bytes");
    }
}
