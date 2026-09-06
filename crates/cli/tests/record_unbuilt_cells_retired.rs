//! The record's own fence for M50 Increment 7 — a `kind × locus` cell at the **third** locus
//! stops being described as awaiting a byte-writing arm, and `LocusDisposition::Unbuilt` stops
//! being described as Increment 7's owed work (T6).
//!
//! Increment 6 classified the third locus and left three byte-writing arms open, so
//! [`engine::schema_diff::locus_disposition`] answered
//! [`Unbuilt`](engine::schema_diff::LocusDisposition::Unbuilt) at three cells and
//! [`engine::transform::transform`] refused them before folding. T3–T5 built all three —
//! [`SchemaChange::AddedItemField`](engine::schema_diff::SchemaChange::AddedItemField)'s
//! `default:`-carrying half, [`AddedItemSlot`](engine::schema_diff::SchemaChange::AddedItemSlot)
//! and [`ValueRemapped`](engine::schema_diff::SchemaChange::ValueRemapped) — so the variant now
//! has **no occupant**, and every home that promised the byte work to a later increment is
//! describing a state that no longer exists.
//!
//! **The retired fact, and the half that must stay sayable.** The retired fact is that a
//! **named** locus-3 cell (or a count of them) awaits an arm. What survives is `Unbuilt` the
//! **mechanism** — the conditional that says what the fold does *should* a future cell be
//! half-built — because the variant is deliberately kept for the next one. So the ban is a
//! **conjunction**: a statement naming one of the three cells (or counting them) **and**
//! claiming, in the present or future tense, that its arm does not exist. A conditional about
//! the mechanism names no cell and trips nothing; a past-tense record of what Increment 6 left
//! open names a cell but carries no present-tense claim, which is the `record_item_slot_kind.rs`
//! carve-out that lets history stay written down as history.
//!
//! **The second retired fact is a *loop*, not a sentence.** `migrate_locus_axis.rs` asserted the
//! empty `Unbuilt` set at **loci 1 and 2 only** and carried the third locus's exemption as a
//! comment. A comment is not a fence: with the arms built, the emptiness is assertable over
//! `1..=LOCI`, and then the *test* is what refuses a half-built cell rather than a reader
//! noticing the comment went stale. That widening is in [`REPLACEMENTS`], keyed by its own
//! source text, because the code half of this sweep is where the claim actually gets enforced.
//!
//! **Why the source files are swept beside the `.md` ones** — the `record_item_slot_kind.rs`
//! rule: a sweep stopping at the documents leaves the code contradicting the document that
//! cross-references it. `schema_diff.rs` owns the variant's own doc-comment, `migrate_corpus.rs`
//! owned a route built around the transient refusal, and `migrate_locus_axis.rs` owns the fence.
//!
//! These are record-content assertions by nature — the deliverable *is* the prose. The behaviour
//! they describe is driven through the real classifier and the shipped binary by
//! `crates/cli/tests/migrate_locus_axis.rs` (the `kind × locus` table's completeness and the
//! locus-3 column), `crates/cli/tests/migrate_corpus_nested_item_slot.rs` (`AddedItemSlot` one
//! locus down) and `crates/cli/tests/migrate_corpus_map_gap.rs` (`ValueRemapped`'s refusal at
//! every item locus), which is why this file asserts only that the record says what those suites
//! drive, and keys each replacement to them.
//!
//! **Deliberately out of the sweep**, each for a stated reason:
//!
//! - `DECISIONS.md` and `implementation/roadmap.md` — dated append-only logs whose purpose is to
//!   record what was true when written (the `record_item_slot_kind.rs` carve-out). The T6 entry
//!   in `DECISIONS.md` is the plan for this task and quotes the state it retires.
//! - `completions/artifacts/M50/` — the planning artifacts, likewise a record of what was
//!   decided when, not a statement of current truth.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read_doc(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("{rel} must exist: {path:?}"))
}

fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// The one line of the file that carries `needle`, panicking if it is not unique.
fn line_with<'a>(body: &'a str, needle: &str) -> &'a str {
    let mut hits = body.lines().filter(|l| l.contains(needle));
    let line = hits
        .next()
        .unwrap_or_else(|| panic!("no line carries `{needle}`"));
    assert!(
        hits.next().is_none(),
        "`{needle}` must identify exactly one line",
    );
    line
}

/// Emphasis markers removed, so a claim broken by bold (`not **yet** folded`) is read as the
/// claim it is. Only `*` and `_` go — backticks stay, because the vocabularies below key on
/// code spans.
fn unemphasized(s: &str) -> String {
    s.chars().filter(|c| *c != '*' && *c != '_').collect()
}

/// The unit a claim is read in: a markdown **paragraph** is one line, and a Rust comment is a
/// **run** of comment lines joined back into the sentence its author wrote. Joining is what makes
/// the `.rs` half meaningful at all — a comment wrapped at 100 columns splits every claim across
/// lines, so a line-at-a-time scan of it measures the wrap, not the sentence.
///
/// (The same reader as `record_two_loci_retired.rs`'s, deliberately: two record fences reading
/// the record in two different units would disagree about what a statement is.)
fn statements(rel: &str, body: &str) -> Vec<(usize, String)> {
    if rel.ends_with(".md") {
        return body
            .lines()
            .enumerate()
            .map(|(n, l)| (n + 1, l.to_owned()))
            .collect();
    }
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut open: Option<(usize, String)> = None;
    for (n, line) in body.lines().enumerate() {
        let t = line.trim_start();
        let comment = t
            .strip_prefix("///")
            .or_else(|| t.strip_prefix("//!"))
            .or_else(|| t.strip_prefix("//"));
        match (comment, open.as_mut()) {
            (Some(text), Some((_, acc))) => {
                acc.push(' ');
                acc.push_str(text.trim());
            }
            (Some(text), None) => open = Some((n + 1, text.trim().to_owned())),
            (None, _) => {
                if let Some(run) = open.take() {
                    out.push(run);
                }
            }
        }
    }
    out.extend(open);
    out
}

/// The falsified statements, each with the file it lives in and why it is now false. Named with
/// the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/corpus-migration.md",
        "Two cells stand there at Increment 6",
        "the design home's own count of the open cells — the statement every other home's \
         version of this fact descends from",
    ),
    (
        "implementation/doctype-authoring.md",
        "cells are classified but not yet folded",
        "the authoring matrix's nested-leaf row: the surface an author reads *before* bumping \
         a frozen schema, and the one that told them two of the cells would refuse",
    ),
    (
        "implementation/decisions-pending.md",
        "refuse by that table until M50 Increment 7 folds them",
        "the ledger's partial-discharge sentence, which dated the fold to a later increment",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "arm at this locus is not built (M50 Increment 7)",
        "the `Unbuilt` variant's own doc-comment, which named a pending increment where it \
         should describe a mechanism — the variant outlives the increment that emptied it",
    ),
    (
        "crates/cli/tests/migrate_locus_axis.rs",
        "for locus in [1, 2]",
        "the emptiness fence itself: it covered two of the three loci and carried the third's \
         exemption as a comment, so the fact it exists to hold was held by a reader",
    ),
    (
        "crates/cli/tests/migrate_locus_axis.rs",
        "the un-built arms this wave",
        "the comment that stood in for the assertion above it",
    ),
    (
        "crates/cli/src/migrate_corpus.rs",
        "built at the item locus only",
        "`halt_finding`'s nested-locus branch — a permanent surface built around a transient \
         refusal (the Settle's declared bound), whose text is false for every caller that can \
         still reach it",
    ),
    (
        "crates/cli/src/migrate_corpus.rs",
        "build the nested-item-block arm",
        "that branch's route, which sent an operator with no jigc workspace to write an arm \
         that exists",
    ),
];

/// The record files the fact is swept over. The three documents plus the three source files that
/// state it in code — a sweep stopping at the `.md` files leaves the code contradicting the
/// document that cross-references it (`record_item_slot_kind.rs`).
const SWEPT: &[&str] = &[
    "design/corpus-migration.md",
    "implementation/doctype-authoring.md",
    "implementation/decisions-pending.md",
    "crates/engine/src/schema_diff.rs",
    "crates/engine/src/transform.rs",
    "crates/cli/src/migrate_corpus.rs",
    "crates/cli/tests/migrate_locus_axis.rs",
];

/// **The subject**: one of the three locus-3 byte-writing cells T3–T5 built, named singly or
/// counted collectively. Two vocabularies are deliberately **absent**, each for its own reason.
///
/// The *mechanism*'s words (`Unbuilt`, "a byte-writing arm", "a half-built cell") stay out
/// because the variant is kept for the next half-built kind, so the record has to go on
/// describing what it does.
///
/// A **bare kind name** (`AddedItemSlot`, `ValueRemapped`) stays out because it cannot
/// discriminate: the kinds are built at loci 1 and 2 and have been for waves, so every true
/// sentence about the shapes that genuinely have no driver — `corpus-migration.md`'s over-build
/// line names `AddedItemSlot` and says in the same breath that section reorder and removal *are
/// not built* — would trip the conjunction. Measured, not predicted: that line is what the first
/// draft of this fence fired on. The retired fact never says the kind, it says **the kind at the
/// third locus**, so the subject carries the locus.
const LOCUS3_CELL_SUBJECT: &[&str] = &[
    "added slot leaf",
    "nested enum value",
    "nested-item-block arm",
    "at the nested locus",
    "at the third locus",
    "into a nested item",
    "one locus down",
    "two cells",
    "Two cells",
    "two byte-writing cells",
    "byte-writing cells are",
];

/// **The claim**: the present- and future-tense vocabulary these homes used to say *the arm does
/// not exist*. A past-tense record of what Increment 6 left open carries none of them, which is
/// what lets the history stay written down as history.
const PENDING_CLAIM: &[&str] = &[
    "not yet folded",
    "not yet built",
    "is not built",
    "are not built",
    "stand there",
    "stands there",
    "refuse by that table",
    "builds the byte work",
    "folds them",
    "at the item locus only",
    "remains to be",
    "remain to be",
    "has yet to be",
    "have yet to be",
    "will be built",
    "will fold",
];

/// **Arm 1 — nothing states the falsified statements, byte for byte.**
#[test]
fn the_record_no_longer_states_the_falsified_lines() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — no swept statement names a locus-3 cell and says its arm does not exist.**
///
/// The conjunction is what makes this unmaskable in both directions: a future pass cannot
/// re-state the retired fact in fresh words, and it cannot be tripped by the half that stands —
/// `Unbuilt` the mechanism names no cell, and the shapes that genuinely still ride the backstop
/// (a nested repeatable **block** added, dropped or `id-from`-re-keyed) are not cells of this
/// table at all.
#[test]
fn no_statement_says_a_locus_3_cell_awaits_its_arm() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, stmt) in statements(file, &body) {
            let flat = unemphasized(&stmt);
            let subject = LOCUS3_CELL_SUBJECT.iter().any(|s| flat.contains(s));
            let pending = PENDING_CLAIM.iter().any(|c| flat.contains(c));
            assert!(
                !(subject && pending),
                "{file}:{n} says a locus-3 `kind × locus` cell is still waiting for its \
                 byte-writing arm — the retired fact, whatever words carry it. All three folded \
                 at M50 Increment 7 (`AddedItemField`'s `default:` half, `AddedItemSlot`, \
                 `ValueRemapped`), driven by `crates/cli/tests/migrate_corpus_nested_item_slot.rs` \
                 and `crates/cli/tests/migrate_corpus_map_gap.rs`. State history in the past \
                 tense if it is history, and describe `Unbuilt` as the mechanism it now is.\n  \
                 {stmt}",
            );
        }
    }
}

/// **Arm 3 — no swept statement owes byte work to M50 Increment 7.**
///
/// The sibling shape of arm 2, keyed on the *increment* rather than on the cell, because the
/// homes phrased the promise both ways — *"both are Increment 7's byte work"*, *"M50 Increment 7
/// builds the byte work"*. `M50` is required in the same statement: `M46 Increment 7` and
/// `M49 Increment 7` are different, landed increments that these same files cite.
#[test]
fn no_statement_owes_byte_work_to_the_increment_that_shipped_it() {
    // `is not built` and its plural are deliberately **out**. The ledger states, in the same
    // markdown line that cites M50 Increment 7, that `AddedNestedRepeatable` *is not built* —
    // the half of the entry that genuinely did not land and that this task is required to leave
    // standing. Keying on it would fire on the one sentence T6 must preserve. The cell-shaped
    // version of that claim is arm 2's, which carries a locus and so can tell the two apart.
    const OWING: &[&str] = &["byte work", "builds the", "will build", "not yet"];
    for file in SWEPT {
        let body = read_doc(file);
        for (n, stmt) in statements(file, &body) {
            let flat = unemphasized(&stmt);
            if !flat.contains("M50 Increment 7") {
                continue;
            }
            for owing in OWING {
                assert!(
                    !flat.contains(owing),
                    "{file}:{n} still describes M50 Increment 7 as work to come (`{owing}`) — it \
                     shipped, and the three byte-writing arms at the third locus fold. Cite it in \
                     the past tense.\n  {stmt}",
                );
            }
        }
    }
}

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot, and a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    ("design/corpus-migration.md", "now has **no occupant**"),
    (
        "implementation/doctype-authoring.md",
        "**No `kind × locus` cell at this locus is left half-built**",
    ),
    (
        "implementation/decisions-pending.md",
        "**fold** since M50 Increment 7",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "No cell reads this at HEAD",
    ),
    (
        "crates/cli/tests/migrate_locus_axis.rs",
        "for locus in 1..=LOCI {",
    ),
    (
        "crates/cli/tests/migrate_locus_axis.rs",
        "no locus has a half-built cell",
    ),
    // One line's worth, deliberately: this arm counts **raw** bytes (a replacement can be code
    // as well as prose — the widened loop below is), and a doc-comment wrapped at 100 columns
    // splits a longer phrase across two lines where no raw count would find it.
    (
        "crates/cli/src/migrate_corpus.rs",
        "for want of a nested arm",
    ),
];

/// **Arm 4 — each replacement is stated once, in one home.**
#[test]
fn each_replacement_is_stated_exactly_once() {
    for (file, needle) in REPLACEMENTS {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            1,
            "{file} must state `{needle}` exactly once (cross-reference, never restate)",
        );
    }
}

/// **Arm 5 — every replacement keys to a suite that drives it.**
///
/// A record claim about behaviour is checkable or it is decoration. The three documents make
/// their new claim by naming a suite that drives the folds one locus down, and each of those
/// suites must exist.
#[test]
fn every_swept_document_keys_its_claim_to_a_driving_suite() {
    const DRIVERS: &[&str] = &[
        "crates/cli/tests/migrate_locus_axis.rs",
        "crates/cli/tests/migrate_corpus_nested_item_slot.rs",
        "crates/cli/tests/migrate_corpus_map_gap.rs",
    ];
    for suite in DRIVERS {
        assert!(
            repo_root().join(suite).is_file(),
            "the suite the record keys to must exist: {suite}",
        );
    }
    for file in SWEPT.iter().filter(|f| f.ends_with(".md")) {
        let body = read_doc(file);
        assert!(
            DRIVERS
                .iter()
                .any(|d| body.contains(d.rsplit('/').next().expect("a file name"))),
            "{file} states that the third locus folds but names no suite driving it — key the \
             claim to one of {DRIVERS:?} so a reader can check it rather than trust it",
        );
    }
}

/// **Arm 6 — the `Unbuilt` variant is documented as a mechanism with no occupant.**
///
/// The variant survives the increment that emptied it, deliberately: the next kind to arrive
/// half-built needs it, and `transform` asking the table before folding is what keeps a
/// half-built cell from splicing at the wrong depth. So its doc-comment has to say **both** that
/// nothing reads it today and what holds that true — otherwise "no occupant" is a claim a reader
/// has to take on faith, which is the shape this whole increment retires.
#[test]
fn the_unbuilt_variant_documents_its_own_emptiness_and_its_fence() {
    let body = read_doc("crates/engine/src/schema_diff.rs");
    let doc = statements("crates/engine/src/schema_diff.rs", &body)
        .into_iter()
        .find(|(_, s)| s.contains("No cell reads this at HEAD"))
        .expect("the `Unbuilt` variant's doc-comment states its emptiness")
        .1;
    for (fact, needle) in [
        ("the fence that holds it", "migrate_locus_axis"),
        (
            "the test that asserts the empty set",
            "every_kind_x_locus_cell_is_dispositioned",
        ),
        ("the range it is asserted over", "1..=LOCI"),
        ("why the variant is kept", "half-built"),
    ] {
        assert!(
            doc.contains(needle),
            "the `Unbuilt` doc-comment must name {fact} (`{needle}`): {doc}",
        );
    }
}

/// **Arm 7 — the matrix's nested-leaf row says the locus folds, and what the fold costs to read.**
///
/// The row is the author's surface for a change the binary now *writes*, so it must carry what
/// T3–T5 shipped rather than only that they shipped: which cells fold, the suites that drive
/// them, and the two facts a reader of a locus-3 refusal needs — the deeper sub-label and the
/// full item chain in the cause.
#[test]
fn the_matrix_leaf_row_states_what_the_folds_shipped() {
    let body = read_doc("implementation/doctype-authoring.md");
    let leaf = line_with(&body, "| **Edit a leaf inside a nested repeatable**");
    let owed: &[(&str, &[&str])] = &[
        // "every cell here **folds**" would be the overclaim: three cells
        // (`NarrowedCardinality` and the two removals) are `Refused` **by design** at this
        // locus, exactly as one level up, and `ProseNeeding`'s field sub-case is un-built at
        // *every* locus rather than at this one. What Increment 7 bought is that no cell is
        // left **half-built**.
        (
            "that no cell at this locus is left half-built",
            &["cell at this locus is left half-built"],
        ),
        ("the three arms by name", &["AddedItemSlot"]),
        (
            "the suites that drive them",
            &["migrate_corpus_nested_item_slot"],
        ),
        ("the depth a nested item's sub-labels render at", &["#####"]),
        (
            "the chain a located refusal names",
            &["releases/1-0-0/changes/added"],
        ),
    ];
    for (fact, wordings) in owed {
        assert!(
            wordings.iter().any(|w| leaf.contains(w)),
            "the leaf row must state {fact}: {leaf}",
        );
    }
}

/// **Arm 8 — the ledger's open half is untouched.**
///
/// T6 corrects the *second* clause's timing, never the first: `AddedNestedRepeatable` — a nested
/// repeatable **block** added to an existing item block — is still not built, sits outside the
/// `kind × locus` table by deliberate design, and the entry stays open on it with its trigger
/// unchanged. Retiring a stale sentence is not licence to close a live deferral beside it.
#[test]
fn the_ledger_entry_stays_open_on_the_kind_that_did_not_land() {
    let body = read_doc("implementation/decisions-pending.md");
    let entry = line_with(&body, "**fold** since M50 Increment 7");
    for needle in [
        "AddedNestedRepeatable",
        "stays open",
        "trigger unchanged",
        "migrate_locus_axis",
    ] {
        assert!(
            entry.contains(needle),
            "the entry must still state `{needle}` — the half that did not land keeps its \
             trigger: {entry}",
        );
    }
}
