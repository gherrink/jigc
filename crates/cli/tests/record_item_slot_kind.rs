//! The record's own fence for M49 Increment 4 — the transform matrix stops presenting an
//! exhaustive table with no row for an added item slot (T3).
//!
//! T1 built [`engine::schema_diff::SchemaChange::AddedItemSlot`] and T2 the rest of
//! `item_field`'s filter (`RemovedItemSlot`, the nested-repeatable leaf naming the backstop
//! kind explicitly, and the slot's own `optional:` delta joining the one shared rule).
//! Before them there was **no legal path from a scalar item field to item prose at any
//! arity**: the delta reached the residual, `jigc migrate-corpus` refused at
//! `migrate-corpus.unclassified-change`, and its route named
//! `crates/engine/src/transform.rs` — a file no adopter can edit — while `jigc validate`
//! routed the operator back the other way (`DECISIONS.md` → 2026-08-29 M49 Increment 4;
//! `completions/artifacts/M49/settle-record.md` → D3(B)).
//!
//! **The retired fact: *an added item slot has no kind and rides the residual*.** The record
//! stated it in three homes — the author's checklist matrix
//! (`implementation/doctype-authoring.md`), the design of record
//! (`design/corpus-migration.md`, twice: the over-build line's unbuilt list and the census's
//! **No silent bump** row), and `diff_item_fields`' doc-comment
//! (`crates/engine/src/schema_diff.rs`, which T2 already rewrote — it is swept here so a
//! sweep stopping at the two `.md` files cannot leave the code contradicting the doc that
//! cross-references it, the `record_set_splice_retired.rs` rule).
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The
//! behaviour it describes is driven through the real binary by
//! `crates/cli/tests/migrate_corpus_item_slot.rs` (the arity × requiredness axis) and
//! `crates/cli/tests/migrate_corpus_item_leaf_residual.rs` (the rest of the filter), which is
//! why this file asserts only that the record says what those suites drive, and keys the new
//! row to them.
//!
//! **A *fact* sweep, not a phrasing sweep** — the `record_set_splice_retired.rs` precedent.
//! The retired fact has two mechanical shapes, and each arm keys on one rather than on a
//! byte-form:
//!
//! - **In the matrix it is structural** (arm 2): the matrix is a table, and the fact lives in
//!   a row whose *change* cell names the item-slot subject while its *kind* cell says
//!   **none**. Reading the columns is what makes the arm unmaskable — the third cell is where
//!   a row's historical `⚠ Pre-M49 …` clause lives, and a clause recording what *used* to be
//!   true is not a statement of current truth (the same carve-out the precedent makes for the
//!   dated logs, applied inside a row).
//! - **Elsewhere it is a conjunction** (arm 3): a line that names the item-slot subject *and*
//!   claims, in the present tense, that nothing classifies it. The unbuilt vocabulary is the
//!   one these files actually use.
//!
//! **Two files are deliberately out of the sweep**: `DECISIONS.md` and
//! `implementation/roadmap.md`, both dated append-only logs whose whole purpose is to record
//! what was true when written — including M49's own entries, which quote the retired fact in
//! order to retire it.

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

/// The falsified statements, each with the file it lives in and why it is now false. Named
/// with the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "implementation/doctype-authoring.md",
        "| A **per-item** Framing-A prose handoff (author prose per repeatable item) | — **none** |",
        "the matrix row an author reads *before* bumping a frozen schema — the nearest row to \
         the change T1 built, and it mandated the opposite conclusion",
    ),
    (
        "design/corpus-migration.md",
        "and the **per-item Framing-A prose handoff**",
        "the over-build line's unbuilt list — the design home the matrix cross-references",
    ),
    (
        "design/corpus-migration.md",
        "· per-item prose handoff)",
        "the property census's **No silent bump** row, which enumerates the unbuilt kinds the \
         backstop stands in for",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "a nested-repeatable edit is its own unbuilt kind and rides the backstop's residual",
        "`diff_item_fields`' doc-comment, which blessed the residual against \
         `corpus-migration.md`'s already-settled *the kind must exist rather than ride the \
         backstop* — retired by T2, swept here so it stays retired",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "Non-`Field` leaves (slots, nested repeatables) are left unclassified",
        "the same doc-comment's first sentence, which stated the filter's drop as the design",
    ),
];

/// The three record files the fact is swept over.
const SWEPT: &[&str] = &[
    "design/corpus-migration.md",
    "implementation/doctype-authoring.md",
    "crates/engine/src/schema_diff.rs",
];

/// The subject: an item block's **slot** leaf — per-item prose — however the record names it.
const ITEM_SLOT_SUBJECT: &[&str] = &[
    "per-item Framing-A prose handoff",
    "per-item prose handoff",
    "prose per repeatable item",
    "slot to a repeatable item block",
    "Non-`Field` leaves",
];

/// The present-tense claim that nothing classifies it — the vocabulary these files use to say
/// *unbuilt*. A past-tense record of what the classifier **used to** do is not one of these.
const UNBUILT_CLAIM: &[&str] = &[
    "rides the backstop's residual",
    "rides the residual",
    "are left unclassified",
    "No driver today",
    "build the kind first",
];

/// **Arm 1 — nothing states the falsified statements, byte for byte.**
#[test]
fn the_record_no_longer_states_that_an_item_slot_has_no_kind() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — the matrix, read as a table: no item-slot row has *none* in its kind column.**
///
/// The structural half of the sweep. A row's third cell carries its `⚠ Pre-M49 …` history, so
/// keying on the *kind* cell is what separates "this change has no kind" (retired) from "this
/// change had no kind until M49" (the record doing its job).
#[test]
fn no_matrix_row_naming_an_item_slot_says_its_kind_is_none() {
    let body = read_doc("implementation/doctype-authoring.md");
    for (n, line) in body.lines().enumerate() {
        let cells: Vec<&str> = line.trim().split('|').map(str::trim).collect();
        // A matrix row: `| change | kind | what happens |` → leading + trailing empties.
        if cells.len() != 5 || !cells[0].is_empty() {
            continue;
        }
        let (change, kind) = (cells[1], cells[2]);
        if !ITEM_SLOT_SUBJECT.iter().any(|s| change.contains(s)) {
            continue;
        }
        assert!(
            !kind.contains("none"),
            "doctype-authoring.md:{} gives an item-slot row no transform kind — the retired \
             fact, whatever words the third cell carries. `AddedItemSlot` ships (M49 Inc-4 \
             T1).\n  change: {change}\n  kind:   {kind}",
            n + 1,
        );
    }
}

/// **Arm 3 — the fact, swept over each file's lines rather than over one phrasing.**
///
/// The item-slot subject is never named inside a present-tense unbuilt claim. This is what
/// makes the sweep unmaskable: a future pass cannot re-state the retired fact in fresh words,
/// and it cannot survive by living in a home arm 1 does not name.
#[test]
fn the_item_slot_subject_is_never_inside_an_unbuilt_claim() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, line) in body.lines().enumerate() {
            let subject = ITEM_SLOT_SUBJECT.iter().any(|s| line.contains(s));
            let unbuilt = UNBUILT_CLAIM.iter().any(|c| line.contains(c));
            assert!(
                !(subject && unbuilt),
                "{file}:{} says an item block's slot leaf has no kind — the retired fact, \
                 whatever words carry it. `AddedItemSlot` and `RemovedItemSlot` ship (M49 \
                 Inc-4 T1/T2); state the history in the past tense if it is history.\n  {line}",
                n + 1,
            );
        }
    }
}

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot, and a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "implementation/doctype-authoring.md",
        "| Add a **slot** to a repeatable item block",
    ),
    (
        "implementation/doctype-authoring.md",
        "| **Remove** a slot from a repeatable item block |",
    ),
    (
        "design/corpus-migration.md",
        "### The item locus's other leaf — a slot, i.e. item prose (M49)",
    ),
    ("design/corpus-migration.md", "- **`AddedItemSlot`**"),
    ("design/corpus-migration.md", "- **`RemovedItemSlot`**"),
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

/// **Arm 5 — the new row's own claim, keyed to the suite that drives it.**
///
/// The row is the author's surface for a change the binary now expresses, so it must carry
/// what T1 actually shipped — both arities, both requirednesses, the insert-capable primitive,
/// and the doc-authorable route for the required arm — and it must name the suite that drives
/// it, so a reader can check the claim rather than trust it.
#[test]
fn the_new_row_states_what_the_suite_drives() {
    let body = read_doc("implementation/doctype-authoring.md");
    let row = line_with(&body, "| Add a **slot** to a repeatable item block");

    assert!(
        row.contains("`AddedItemSlot` **(M49)**") && row.contains("✅") && !row.contains("⛔"),
        "the row names the kind and classifies: {row}",
    );
    // Each fact the row owes, with the wordings that state it — the arm is about the claim,
    // not about one byte-form of it.
    let owed: &[(&str, &[&str])] = &[
        ("the slotless block's first slot", &["0 → 1"]),
        ("the relabel arity", &["1 → 2"]),
        (
            "…whose fold writes no content bytes, the stamp being the only delta",
            &["zero content bytes", "byte no-op"],
        ),
        (
            "the insert-capable primitive (`set_item_slot` is update-only)",
            &["insert_item_slot"],
        ),
        (
            "the required arm's doc-authorable route",
            &["migrate-corpus.prose-needed", "prose-needed"],
        ),
    ];
    for (fact, wordings) in owed {
        assert!(
            wordings.iter().any(|w| row.contains(w)),
            "the row must state {fact} — what T1 shipped: {row}",
        );
    }

    let suite = "crates/cli/tests/migrate_corpus_item_slot.rs";
    assert!(
        row.contains("migrate_corpus_item_slot"),
        "the row must key its claim to the suite that drives it ({suite}): {row}",
    );
    assert!(
        repo_root().join(suite).is_file(),
        "…and that suite must exist: {suite}",
    );
}

/// **Arm 6 — the two rows T2 made true are re-stated to what it shipped.**
///
/// Both rows were *already* in the matrix and both were wrong about the item locus: the
/// nested-repeatable row implied the delta is only ever reached by the residual (so a mixed
/// bump dropped it silently), and the `OptionalRelaxed` row claimed the slot level at both
/// loci while `item_field` dropped an item block's slot leaf on the floor.
#[test]
fn the_rows_t2_touched_say_what_t2_shipped() {
    let body = read_doc("implementation/doctype-authoring.md");

    // M50 Inc-6 T3 split this row in two: a leaf edit one level down took its own kind, and
    // the M49 fact below — the delta names the backstop kind explicitly — is now the **block**
    // row's (`record_two_loci_retired.rs`).
    let nested = line_with(
        &body,
        "| **Add**, **drop** or **re-key** a nested repeatable *block*",
    );
    assert!(
        nested.contains("⛔") && nested.contains("names itself"),
        "the nested row stays unbuilt but the delta now names the backstop kind explicitly, so \
         it refuses in a mixed bump instead of being dropped: {nested}",
    );
    assert!(
        nested.contains("alongside a classified change"),
        "…and the row says where that matters — the residual fires only over an otherwise-empty \
         diff: {nested}",
    );

    let prose_needing = line_with(&body, "| Add a **required** slot to a *section*,");
    assert!(
        prose_needing.contains("`AddedItemSlot` row above"),
        "the `ProseNeeding` row must not re-absorb the item-block slot T1 took out of it — a \
         required *item* slot emits `AddedItemSlot` and the per-doc gate adjudicates: {prose_needing}",
    );

    let relaxed = line_with(&body, "| **Relax** a slot or field to optional");
    assert!(
        relaxed.contains("M49") && relaxed.contains("slot_flag_change"),
        "the `OptionalRelaxed` row's both-loci-and-the-slot-level claim was false at an item \
         block's slot leaf until T2 gave the rule one home: {relaxed}",
    );
}
