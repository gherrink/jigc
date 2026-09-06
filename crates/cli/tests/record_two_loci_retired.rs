//! The record's own fence for M50 Increment 6 — the migration classifier stops being described
//! as having two loci, and a nested-repeatable **leaf** edit stops being described as having no
//! kind (T3).
//!
//! T1 made the kind space a set ([`engine::schema_diff::SchemaChangeKind`] + `ALL` + an
//! exhaustive `From<&SchemaChange>`) and T2 built the third locus: the flat `section: String`
//! became a [`engine::schema_diff::Locus`] path, `diff_item_fields` **recurses** into a nested
//! `Leaf::Repeatable` instead of comparing it wholesale, and the `kind × locus` disposition is a
//! production table ([`engine::schema_diff::locus_disposition`]) the driver asks before folding.
//! Before them, every one of the kinds the item locus classifies answered
//! `migrate-corpus.unclassified-change` one level down, with a route naming
//! `crates/engine/src/transform.rs` — a file no adopter can edit.
//!
//! **Two retired facts, each with its own mechanical shape.**
//!
//! - **Fact A — *the classifier has two loci*.** The locus count is derived
//!   ([`engine::schema_diff::LOCI`] `= MAX_NESTING_DEPTH + 1`), so *any* unqualified two-count
//!   of a locus set is the retired fact: `both loci`, `two loci`, `either locus`. A statement
//!   genuinely about exactly two of them names **which** two (`loci 1 and 2`, or the loci
//!   themselves) rather than counting them — which is the point, because a count is what goes
//!   stale the day the address grammar grows a hop pair.
//! - **Fact B — *a nested-repeatable leaf edit has no kind*.** A **conjunction**: a statement
//!   naming a nested-repeatable *leaf* edit **and** claiming, in the present tense, that nothing
//!   classifies it. The subject discriminates the retired half from the half that still stands —
//!   a nested repeatable **block** added, dropped or `id-from`-re-keyed genuinely does ride the
//!   backstop, deliberately, and the record must keep saying so.
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The behaviour it
//! describes is driven through the real classifier and the shipped binary by
//! `crates/cli/tests/migrate_locus_axis.rs` (the locus-3 column, kind by kind, plus the
//! `kind × locus` table's own completeness) and `crates/cli/tests/schema_change_kind_registry.rs`
//! (the kind space as a set), which is why this file asserts only that the record says what those
//! suites drive, and keys each replacement to them.
//!
//! **Why `crates/engine/src/schema_diff.rs` is swept at all** — the `record_item_slot_kind.rs`
//! rule: a sweep stopping at the `.md` files leaves the code contradicting the document that
//! cross-references it. It takes fact A's arms (the byte-exact statements and the count ban);
//! it is **out of fact B's conjunction arm**, and for a reason rather than for convenience: at
//! the classifier fact B is no longer prose. `locus_disposition` *is* the statement of which
//! cells classify, [`engine::transform`] consults it before folding, and `migrate_locus_axis.rs`
//! drives the whole column — so a doc-comment there restating the retired fact contradicts a
//! test, not merely a document.
//!
//! **Deliberately out of the sweep**, each for a stated reason:
//!
//! - `DECISIONS.md` and `implementation/roadmap.md` — dated append-only logs whose purpose is to
//!   record what was true when written (the `record_item_slot_kind.rs` carve-out).
//! - `crates/cli/src/migrate_corpus.rs`, `crates/engine/src/write.rs`, `crates/cli/src/pack.rs`,
//!   `crates/engine/src/milestone.rs` — each says *both loci* about a **different subject** (two
//!   write-address shapes for an unfilled-leaf finding; the write path's two splice homes; the
//!   cascade's layers), not about the migration classifier's locus set. A phrase ban keyed on
//!   bytes rather than on the subject would fire on true statements about other things.

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

/// Emphasis markers removed, so a count broken by bold (`over **both** loci`) is read as the
/// claim it is. Only `*` and `_` go — backticks stay, because the vocabularies below key on
/// code spans.
fn unemphasized(s: &str) -> String {
    s.chars().filter(|c| *c != '*' && *c != '_').collect()
}

/// The unit a claim is read in: a markdown **paragraph** is one line, and a Rust doc-comment is
/// a **run** of comment lines joined back into the sentence its author wrote. Joining is what
/// makes the `.rs` half meaningful at all — a comment wrapped at 100 columns splits every claim
/// across lines, so a line-at-a-time scan of it measures the wrap, not the sentence.
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
        "both have **two loci**",
        "the hole census's headline count — the design home every other statement of the locus \
         set descends from",
    ),
    (
        "design/corpus-migration.md",
        "these remain **unbuilt**: nested-repeatable edits",
        "the over-build line's unbuilt list, which named the whole nested shape rather than the \
         block half of it",
    ),
    (
        "design/corpus-migration.md",
        "the unbuilt kinds (nested-repeatable edit",
        "the property census's **No silent bump** row, which enumerates the unbuilt kinds the \
         backstop stands in for",
    ),
    (
        "design/corpus-migration.md",
        "An added, dropped or reshaped nested leaf emits `Unclassified` itself",
        "the M49 section's nested bullet, true when written and retired by T2's recursion",
    ),
    (
        "implementation/doctype-authoring.md",
        "| **Edit a nested repeatable** (e.g. `changelog.releases.changes`) | — **none**",
        "the matrix row an author reads *before* bumping a frozen schema — the one row the \
         change T2 built sits in, and it mandated the opposite conclusion",
    ),
    (
        "design/methodology-docs.md",
        "would freeze that block at its birth shape for every adopter",
        "the one-level bound's deferral rationale, which named the wrong arm: the arm that \
         would freeze a nested block at its birth shape is the **reshape** one T2 built, while \
         the shape that stays unclassified is *adding* the block — which is what the increment \
         tier itself would be",
    ),
    (
        "implementation/decisions-pending.md",
        "freezes that block at its birth shape permanently, for every adopter",
        "the roadmap increment tier's binding precondition — `methodology-docs.md`'s sibling, \
         wrong the same way",
    ),
    (
        "implementation/decisions-pending.md",
        "**DISCHARGED at the M50 Settle, 2026-09-04 (D6).**",
        "the entry covers `AddedNestedRepeatable` **and** a nested arm for the existing \
         transforms; M50 builds the arm and not the kind, so a whole-entry discharge overstates \
         what landed",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "Fired at **both loci**",
        "`WidenedCardinality`'s own doc-comment — the classifier stating its locus set as two \
         inside the file T2 gave a third",
    ),
];

/// The record files the two facts are swept over. `schema_diff.rs` takes fact A only (see the
/// module doc).
const SWEPT: &[&str] = &[
    "design/corpus-migration.md",
    "design/methodology-docs.md",
    "implementation/doctype-authoring.md",
    "implementation/decisions-pending.md",
    "crates/engine/src/schema_diff.rs",
];

/// The four record files fact B is swept over.
const SWEPT_RECORD: &[&str] = &[
    "design/corpus-migration.md",
    "design/methodology-docs.md",
    "implementation/doctype-authoring.md",
    "implementation/decisions-pending.md",
];

/// **Fact A**: a locus set stated as an unqualified two-count, however it is spelled.
const TWO_COUNT: &[&str] = &["both loci", "two loci", "either locus"];

/// **Fact B's subject**: an edit to a **leaf inside** a nested repeatable — the half T2 built.
/// A nested repeatable **block** added, dropped or re-keyed is deliberately *not* here: that
/// shape still rides the backstop, and the record must go on saying so.
const NESTED_LEAF_SUBJECT: &[&str] = &[
    "nested-repeatable leaf",
    "nested-repeatable edit",
    "nested repeatable leaf",
    "leaf inside a nested",
    "leaf inside an existing nested",
    "leaf edit inside",
    "Edit a nested repeatable",
];

/// **Fact B's claim**: the present-tense vocabulary these files use to say *nothing classifies
/// it*. A past-tense record of what the classifier **used to** do is not one of these — the
/// `record_item_slot_kind.rs` rule, so history can stay written down as history.
const UNBUILT_CLAIM: &[&str] = &[
    "has no kind",
    "have no kind",
    "no transform kind",
    "is still unbuilt",
    "are still unbuilt",
    "remain **unbuilt**",
    "stays unclassified",
    "stay unclassified",
    "rides the backstop",
    "ride the backstop",
    "rides the residual",
    "reaches the residual",
    "build the kind first",
    "at its birth shape",
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

/// **Arm 2 — fact A: no swept statement counts a locus set as two.**
///
/// The count is the whole defect: [`engine::schema_diff::LOCI`] is derived from
/// `MAX_NESTING_DEPTH`, so a prose two-count is stale already and goes staler. A statement about
/// exactly two of the three names them.
#[test]
fn no_statement_counts_the_classifier_s_loci_as_two() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, stmt) in statements(file, &body) {
            let flat = unemphasized(&stmt);
            for phrase in TWO_COUNT {
                assert!(
                    !flat.contains(phrase),
                    "{file}:{n} states a locus set as `{phrase}` — the retired fact. The \
                     classifier's loci are `LOCI = MAX_NESTING_DEPTH + 1` (M50 Inc-6 T2); name \
                     which loci you mean (`loci 1 and 2`) rather than counting them.\n  {stmt}",
                );
            }
        }
    }
}

/// **Arm 3 — fact A: a stated locus count is *derived*, never a literal.**
///
/// The Settle's own rule (`completions/artifacts/M50/settle-record.md` → D6): any statement
/// hardcoding *three loci* re-enacts M45's *statement == constant* failure one layer up. A
/// statement giving the count must name the constant it comes from in the same breath.
#[test]
fn a_stated_locus_count_names_the_constant_it_derives_from() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, stmt) in statements(file, &body) {
            let flat = unemphasized(&stmt);
            let literal = ["three loci", "3 loci"].iter().any(|l| flat.contains(l));
            assert!(
                !literal || stmt.contains("MAX_NESTING_DEPTH"),
                "{file}:{n} writes the locus count down as a literal without naming \
                 `MAX_NESTING_DEPTH` — the constant `LOCI` derives it from.\n  {stmt}",
            );
        }
    }
}

/// **Arm 4 — fact B: a nested-repeatable *leaf* edit is never inside an unbuilt claim.**
///
/// The conjunction is what makes this unmaskable in both directions: a future pass cannot
/// re-state the retired fact in fresh words, and it cannot be tripped by the still-true half,
/// because the still-true half's subject is the **block**, not a leaf.
#[test]
fn a_nested_leaf_edit_is_never_inside_an_unbuilt_claim() {
    for file in SWEPT_RECORD {
        let body = read_doc(file);
        for (n, stmt) in statements(file, &body) {
            let subject = NESTED_LEAF_SUBJECT.iter().any(|s| stmt.contains(s));
            let unbuilt = UNBUILT_CLAIM.iter().any(|c| stmt.contains(c));
            assert!(
                !(subject && unbuilt),
                "{file}:{n} says a leaf edit inside a nested repeatable has no kind — the \
                 retired fact, whatever words carry it. The recursion classifies it at its own \
                 locus (M50 Inc-6 T2); the shape that still rides the backstop is the nested \
                 **block** added, dropped or re-keyed, so say *that*, and state history in the \
                 past tense if it is history.\n  {stmt}",
            );
        }
    }
}

/// **Arm 5 — no matrix row naming a nested-leaf edit gives it no kind.**
///
/// The structural half. The matrix is a table, and the fact lives in a row whose *change* cell
/// names the subject while its *kind* cell says **none** — read as columns, because a row's
/// third cell is where its historical `⚠ Pre-M…` clause lives, and a clause recording what
/// *used* to be true is not a statement of current truth.
#[test]
fn no_matrix_row_naming_a_nested_leaf_edit_says_its_kind_is_none() {
    let body = read_doc("implementation/doctype-authoring.md");
    for (n, line) in body.lines().enumerate() {
        let cells: Vec<&str> = line.trim().split('|').map(str::trim).collect();
        // A matrix row: `| change | kind | what happens |` → leading + trailing empties.
        if cells.len() != 5 || !cells[0].is_empty() {
            continue;
        }
        let (change, kind) = (cells[1], cells[2]);
        if !NESTED_LEAF_SUBJECT.iter().any(|s| change.contains(s))
            && !change.contains("leaf inside a nested repeatable")
        {
            continue;
        }
        assert!(
            !kind.contains("none"),
            "doctype-authoring.md:{} gives a nested-leaf-edit row no transform kind — the \
             retired fact, whatever words the third cell carries. The leaf classifies with its \
             own kind at the nested locus (M50 Inc-6 T2).\n  change: {change}\n  kind:   {kind}",
            n + 1,
        );
    }
}

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot, and a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "design/corpus-migration.md",
        "### The third locus — a nested item block's leaves (M50)",
    ),
    (
        "design/corpus-migration.md",
        "- **A leaf edit inside an existing nested block classifies with the leaf's own kind.**",
    ),
    (
        "design/corpus-migration.md",
        "- **What still rides the backstop is the block itself, never its leaves.**",
    ),
    (
        "implementation/doctype-authoring.md",
        "| **Edit a leaf inside a nested repeatable**",
    ),
    (
        "implementation/doctype-authoring.md",
        "| **Add**, **drop** or **re-key** a nested repeatable *block*",
    ),
    (
        "design/methodology-docs.md",
        "adding a nested repeatable block to an existing item block rides the backstop",
    ),
    (
        "implementation/decisions-pending.md",
        "**DISCHARGED IN PART at the M50 Settle, 2026-09-04 (D6) — the nested arm, not the kind.**",
    ),
];

/// **Arm 6 — each replacement is stated once, in one home.**
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

/// **Arm 7 — every replacement statement keys to the suite that drives it.**
///
/// A record claim about behaviour is checkable or it is decoration. Each swept record file makes
/// its new claim by naming the acceptance suite that drives the locus-3 column, and that suite
/// must exist.
#[test]
fn every_swept_record_file_keys_its_claim_to_the_driving_suite() {
    let suite = "crates/cli/tests/migrate_locus_axis.rs";
    assert!(
        repo_root().join(suite).is_file(),
        "the suite the record keys to must exist: {suite}",
    );
    for file in SWEPT_RECORD {
        let body = read_doc(file);
        assert!(
            body.contains("migrate_locus_axis"),
            "{file} states the third locus but names no suite driving it — key the claim to \
             {suite} so a reader can check it rather than trust it",
        );
    }
}

/// **Arm 8 — the new matrix rows say what the suites drive, and no more.**
///
/// The leaf row is the author's surface for a change the binary now classifies, so it must carry
/// what T2 actually shipped: the leaf's own kind at the nested locus, the locus **path** in the
/// refusal, and — honestly — that two `kind × locus` cells are classified but **not yet folded**.
/// The block row must keep saying the half that is still unbuilt.
#[test]
fn the_new_matrix_rows_state_what_shipped() {
    let body = read_doc("implementation/doctype-authoring.md");

    let leaf = line_with(&body, "| **Edit a leaf inside a nested repeatable**");
    assert!(
        leaf.contains("✅") && !leaf.contains("⛔"),
        "the leaf row classifies: {leaf}",
    );
    let owed: &[(&str, &[&str])] = &[
        ("the leaf's own kind at the nested locus", &["nested locus"]),
        ("the locus path the refusal names", &["releases/changes"]),
        // T3 owed this row the two `kind × locus` cells whose byte work was NOT Increment 6's,
        // and the row said so. M50 Increment 7 folded them, so the owed fact inverts rather
        // than lapses: the row must now say every cell at this locus folds, and still name the
        // increment — `record_unbuilt_cells_retired.rs` owns the retirement and this arm keeps
        // the row honest about what replaced it.
        (
            "that no cell at this locus is left half-built",
            &["cell at this locus is left half-built"],
        ),
        ("the increment that folded them", &["Increment 7"]),
        ("the suite that drives the claim", &["migrate_locus_axis"]),
    ];
    for (fact, wordings) in owed {
        assert!(
            wordings.iter().any(|w| leaf.contains(w)),
            "the leaf row must state {fact}: {leaf}",
        );
    }

    let block = line_with(
        &body,
        "| **Add**, **drop** or **re-key** a nested repeatable *block*",
    );
    assert!(
        block.contains("⛔") && block.contains("none"),
        "the block row stays unbuilt — `AddedNestedRepeatable` is not built at this commit: \
         {block}",
    );
    assert!(
        block.contains("alongside a classified change"),
        "…and it keeps M49's half: the delta names the backstop kind explicitly, so it refuses \
         beside a classified change where the residual never fired: {block}",
    );
}

/// **Arm 9 — the ledger's discharge says which half discharged.**
///
/// The `AddedNestedRepeatable` entry covers a **kind** and a **nested arm for the existing
/// transforms**. M50 builds the arm; the kind is not built and the shape it would name still
/// rides the backstop by deliberate design. A whole-entry discharge would retire a trigger that
/// has not fired — the exact failure the ledger exists to prevent, one turn later.
#[test]
fn the_ledger_discharge_names_the_half_that_did_not_land() {
    let body = read_doc("implementation/decisions-pending.md");
    let entry = line_with(&body, "**DISCHARGED IN PART at the M50 Settle");
    for needle in [
        "AddedNestedRepeatable",
        "stays open",
        "trigger unchanged",
        "migrate_locus_axis",
    ] {
        assert!(
            entry.contains(needle),
            "the partial discharge must state `{needle}` — which half landed, which stays open, \
             and what drives the half that did: {entry}",
        );
    }
}
