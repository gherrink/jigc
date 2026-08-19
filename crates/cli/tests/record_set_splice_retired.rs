//! The record's own fence for M46 Increment 4 — the `set:` splice stops being stated as
//! fact (T4).
//!
//! T1 settled **one** rule at both added-leaf loci: a `default:` places its value; an
//! absence that **already conforms** — `optional:`, an optional `ref`, a pack-declared
//! type, or a `set:` carrying no `default:` — is a byte no-op the run *names*
//! ([`engine::validate::is_author_required`] is the single predicate, and
//! `migrate-corpus.set-field-unfilled` the advisory T2 minted); only an author-required
//! leaf with no value source is `ProseNeeding`.
//!
//! Four statements in the design of record said otherwise, each joining `default:` and
//! `set:` into one placement claim — and they were falsified by the binary *before* T1 as
//! well as after it: the `set:` arm returned `TransformError::Unsupported`, so the doc
//! collected a `prose-needed` route that could not be followed even when followed exactly
//! (the razor ledger's **R5**/**R10**; `DECISIONS.md` → 2026-08-19 M46 Increment 4).
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The
//! behaviour it describes is proven through the real binary and at the engine seam by
//! `crates/cli/tests/migrate_corpus_set_fields.rs` (both loci × the four exempt shapes ×
//! the `default:` and required-with-no-value-source controls), which is why this file
//! asserts only that the record says what that suite drives.
//!
//! **The sweep is a *fact* sweep, not a phrasing sweep** — Increment 3's lesson, applied
//! here rather than re-learned: naming byte-forms is what makes a sweep readable and what
//! makes it maskable, so arm 2 keys on the fact itself. The retired fact has a mechanical
//! shape: **the two mint stories joined into one placement claim.** Any line that puts
//! `` `default:` `` and `` `set:` `` side by side *and* claims something gets placed is
//! stating it, whatever words it uses. Joining them for any other purpose (the projection
//! table, the `PresentationOnly` extent, the tightening refinement, the mint-story
//! checklist) is untouched, because those lines make no placement claim.
//!
//! **Three files are in the sweep, and two are deliberately out.** In: the design of
//! record (`design/corpus-migration.md`), the author's checklist
//! (`implementation/doctype-authoring.md`), and the **`SchemaChange::AddedItemField`
//! doc-comment** (`crates/engine/src/schema_diff.rs`) — which the design doc
//! cross-references, and which stated the identical retired fact, so a sweep stopping at
//! the two `.md` files would have left the code contradicting the doc that points at it.
//! Out: `DECISIONS.md` and `implementation/roadmap.md`. Both are **dated, append-only
//! logs** — a 2026-07-13 entry recording what M42 built, and M46's own entries quoting the
//! retired bytes in order to retire them, are not statements of current truth, and
//! rewriting them would destroy the record this repo keeps them for.

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

/// The falsified statements, each with the file it lives in and why it is now false.
/// Named with the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/corpus-migration.md",
        "**`default:` / `set:` present** → the deterministic value is **spliced into every item**",
        "R5's first cite: the `AddedItemField` per-item semantics. A `set:` with no \
         `default:` places nothing — `is_author_required` exempts it, so the item already \
         conforms without the bullet",
    ),
    (
        "design/corpus-migration.md",
        "splicing only when it carries `default:`/`set:`",
        "R5's second cite (`:264` at the ledger, `:274` at HEAD): the transform-determinism \
         census row, restating the same join",
    ),
    (
        "implementation/doctype-authoring.md",
        "| Add a field carrying `default:` / `set:` | `AddedOptionalField` | ✅ the deterministic value lands |",
        "R10: the matrix row an author reads *before* bumping a frozen schema — the one \
         surface where the claim costs a wasted bump",
    ),
    (
        "implementation/doctype-authoring.md",
        "**carries `default:`/`set:`** → the value is spliced into every item",
        "R10's second cell: the item-block row, which bills itself as a correction of a \
         draft *wrong for the optional case* and was left wrong for the `set:` case",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "**`default:`/`set:` present** → the value is spliced into **every item that lacks the",
        "the `AddedItemField` doc-comment the design doc cross-references — the same fact, \
         in the file that defines the kind",
    ),
];

/// The three record files the fact is swept over. `DECISIONS.md` and the roadmap are out
/// by the module doc's rule: dated logs record what was true then, not what is true now.
const SWEPT: &[&str] = &[
    "design/corpus-migration.md",
    "implementation/doctype-authoring.md",
    "crates/engine/src/schema_diff.rs",
];

/// The join: the two mint stories named as one thing.
const JOINED: &[&str] = &[
    "`default:` / `set:`",
    "`default:`/`set:`",
    "`default`/`set`",
    "`default:`/`set`",
];

/// A claim that something gets **placed**. The retired fact is the conjunction of a join
/// and one of these on the same line.
const PLACEMENT_CLAIM: &[&str] = &[
    "splic",
    "value lands",
    "is placed",
    "lands in",
    "deterministic value",
];

/// **Arm 1 — nothing states the falsified statements, byte for byte.**
#[test]
fn the_record_no_longer_states_the_set_splice() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — the fact, swept over the whole file rather than over one phrasing.**
///
/// The two mint stories are never joined inside a placement claim. This is what makes the
/// sweep unmaskable: a future pass cannot re-state the retired fact in fresh words, and it
/// cannot survive by living in a home arm 1 does not name.
#[test]
fn the_two_mint_stories_are_never_joined_in_a_placement_claim() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, line) in body.lines().enumerate() {
            let joined = JOINED.iter().any(|j| line.contains(j));
            let places = PLACEMENT_CLAIM.iter().any(|v| line.contains(v));
            assert!(
                !(joined && places),
                "{file}:{} joins `default:` and `set:` inside a placement claim — the \
                 retired fact, whatever words carry it. A `set:` with no `default:` places \
                 nothing; state the two arms apart.\n  {line}",
                n + 1,
            );
        }
    }
}

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot, and a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "design/corpus-migration.md",
        "**`default:` present** → the declared value is **spliced into every item**",
    ),
    (
        "design/corpus-migration.md",
        "a **`set:` carrying no `default:`** — → **classified, and a byte no-op.**",
    ),
    (
        "design/corpus-migration.md",
        "**The extension point is untouched — only the nobody-threaded-a-value behaviour changed.**",
    ),
    (
        "design/corpus-migration.md",
        "splicing only when it carries a `default:`",
    ),
    (
        "implementation/doctype-authoring.md",
        "| Add a field carrying `set:` with **no** `default:` |",
    ),
    (
        "implementation/doctype-authoring.md",
        "**carries a `default:`** → the value is spliced into every item",
    ),
    (
        "crates/engine/src/schema_diff.rs",
        "/// - **`default:` present** → the value is spliced into **every item that lacks the",
    ),
];

/// **Arm 3 — each replacement is stated once, in one home.**
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

/// **Arm 4 — the matrix carries the ✅/⛔ split the binary holds.**
///
/// The author's checklist is the surface consulted *before* a frozen bump, so the two mint
/// stories must be two rows with two verdicts, and the ⛔ row must be keyed on the shipped
/// predicate rather than on "no default" — which is what put a `set:`-bearing leaf on the
/// blocking side of a matrix the binary never blocked it on.
#[test]
fn the_matrix_splits_the_two_mint_stories() {
    let body = read_doc("implementation/doctype-authoring.md");

    let defaulted = line_with(&body, "| Add a field carrying `default:` |");
    assert!(
        defaulted.contains("✅") && defaulted.contains("lands"),
        "the `default:` row keeps its ✅ — the declared value is placed: {defaulted}",
    );

    let derived = line_with(
        &body,
        "| Add a field carrying `set:` with **no** `default:` |",
    );
    assert!(
        derived.contains("✅") && derived.contains("byte no-op") && !derived.contains("⛔"),
        "the `set:`-without-`default:` row is a ✅ byte no-op, never a block: {derived}",
    );
    assert!(
        derived.contains("set-field-unfilled"),
        "and it names the advisory that keeps the no-op audible: {derived}",
    );

    let blocking = line_with(&body, "| `ProseNeeding` (`leaf: None` / `leaf: Some`) |");
    assert!(
        blocking.contains("⛔") && blocking.contains("author-required"),
        "the blocking row is keyed on the shipped predicate, not on \"no default\": {blocking}",
    );
    assert!(
        !blocking.contains("required field with no default"),
        "…which is the phrasing that swept a `set:`-bearing leaf onto the blocking side: {blocking}",
    );
}

/// **Arm 5 — the extension point is stated where the design lives.**
///
/// T1 retired *nothing* about a caller threading a deterministic value in; it changed only
/// what happens when nobody does. That distinction lived in a code doc-comment and nowhere
/// a doctype author reads, which is exactly how a no-op gets misread as a capability loss.
#[test]
fn the_design_doc_states_that_the_threaded_value_survives() {
    let body = read_doc("design/corpus-migration.md");
    for needle in ["with_stamp_default", "authored_remap"] {
        assert!(
            body.contains(needle),
            "corpus-migration.md must name `{needle}` — the caller-threaded value is the \
             extension point, and it is untouched",
        );
    }
    assert!(
        read_doc("implementation/doctype-authoring.md").contains("with_stamp_default"),
        "and the author's checklist must say so where the `set:` row is read",
    );
}
