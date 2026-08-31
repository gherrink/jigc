//! The record's own fence for M49 Increment 11 — the stale **reasons** die, each with the
//! datum that falsifies it (T10).
//!
//! Seven statements of record (plus the siblings the sweep found beside them) justified a
//! conclusion with a reason that HEAD falsifies. **In every case the conclusion survives and
//! the reason does not**, so each is rewritten *where it was written* rather than annotated
//! around — the `record_foreign_arm.rs` discipline.
//!
//!   * **`design/doc-read-surface.md`** — *"no shipped doctype declares a **multi-slot**
//!     repeatable"*. `roadmap.milestones` declares `proves` **and** `decomposition` and has
//!     since M16. The fixture's own doc-comment
//!     (`crates/cli/tests/doc_show_item_leaf.rs`) says *dev-pack*; the design doc dropped the
//!     qualifier and generalized a true bound into a false one. It is not a cosmetic slip:
//!     believing the corpus free of the shape is why the **write** side of the same seam —
//!     an item's leaf region ending at the first deeper heading, while a slot sub-label *is*
//!     one — went unswept until M49 Increment 1.
//!   * **`design/methodology-docs.md`** — the roadmap's prose decomposition justified by
//!     *"repeatable-inside-repeatable is **not expressible** … no `Leaf::Repeatable`"*.
//!     `Leaf::Repeatable` shipped at **M22** and `changelog` uses it; the tier is **deferred**
//!     at the M49 Settle (leg 0, with a written trigger and a one-way-door precondition), not
//!     blocked by the engine. Its neighbour at *What stays deferred* is a **fired-trigger row**
//!     the M49 baseline lists as never resurfaced — engaged here rather than left beside a
//!     repaired sibling.
//!   * **`design/methodology-docs.md`, the doctype table** — three running singletons given
//!     folder homes (`location: roadmap/` · `ledger/` · `decisions-log/`) that M38's placement
//!     convention retired. The named site was the roadmap row; the two beside it carry the
//!     identical rot, as does the fixed-slug path list further down, and the **same doc's own
//!     prose** already states the M38 correction — so the table was contradicting its own page.
//!     Swept as the class, not the reported cell.
//!   * **`implementation/module-layout.md` · `design/workflow-dialect.md` (twice) ·
//!     `design/write-commands.md`** — the emitted-format micro-syntax and the on-disk format
//!     called *"an open question"*, both settled **2026-05-28**, while `workflow-dialect.md`
//!     carries a full *Emitted format* section and the on-disk format has been **frozen v1**
//!     since M34. `decisions-pending.md` has carried a cleanup entry owed on exactly this
//!     since Increment 3 of the MVP; it is **discharged**, not re-owed, and it named one of
//!     the four sites.
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. What the prose
//! now claims about the **shipped schemas** is not asserted as prose: arm 5 derives it from the
//! two embedded packs through the production `load_pack_schema` path, so a dev-pack repeatable
//! that gains a second slot, or a `roadmap` that loses one, reddens the doc's sentence at the
//! point the set is decided rather than at the point someone remembers to re-read it.
//!
//! **Stated as one thing, asserted as one thing**: every falsified statement is named with the
//! bytes it carried and asserted absent, and every replacement is asserted **exactly once**,
//! because a correction restated in two homes is the rot this repo's cross-reference rule
//! exists to prevent. Arm 3 adds the *fact* sweep the byte list cannot give — a line naming the
//! subject may not also call it open — with each withdrawal sentence declared and stripped
//! first, since a withdrawal must be free to quote the bytes it retires.
//!
//! **Two files are deliberately out of the sweep, by name and with the reason**
//! ([`EXCLUDED_HISTORICAL`]), and two whole file *kinds* are out by convention:
//! `DECISIONS.md` and `implementation/roadmap.md` are **dated records of what was decided and
//! planned when** — this wave's own entries there state the falsification, and retiring the
//! 2026-06 lines would rewrite history rather than correct a claim of current truth.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use cli::pack::{CompositePack, EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource};
use engine::schema::{Leaf, SectionBody};

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

/// The falsified statements, each with the file it lives in and the datum that kills it.
/// Named with the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/doc-read-surface.md",
        "no shipped doctype declares a **multi-slot** repeatable",
        "`roadmap.milestones` declares `proves` + `decomposition` and has since M16 — the \
         fixture's own doc-comment scopes the bound to the DEV pack, and this dropped the \
         qualifier",
    ),
    (
        "design/methodology-docs.md",
        "repeatable-inside-repeatable is **not expressible**",
        "`Leaf::Repeatable` is declared in `crates/engine/src/schema.rs` and `changelog` \
         uses it — shipped at M22",
    ),
    (
        "design/methodology-docs.md",
        "no `Leaf::Repeatable`",
        "the variant's absence was the stated mechanism of the bound; it has existed since \
         M22",
    ),
    (
        "design/methodology-docs.md",
        "Fires behind the first doctype that genuinely needs repeatable-inside-repeatable",
        "the fired-trigger row: `changelog` was that doctype at M22, and the M49 baseline \
         lists this row among the triggers that fired and were never resurfaced",
    ),
    (
        "design/methodology-docs.md",
        "`location: roadmap/`",
        "the shipped schema is `placement: { file: docs/roadmap.md }` (M38)",
    ),
    (
        "design/methodology-docs.md",
        "`location: ledger/`",
        "the unlisted neighbour, identical class: the shipped schema is \
         `placement: { file: docs/deferral-ledger.md }` (M38)",
    ),
    (
        "design/methodology-docs.md",
        "`location: decisions-log/`",
        "the second unlisted neighbour: the shipped schema is \
         `placement: { file: docs/decisions-log.md }` (M38)",
    ),
    (
        "design/methodology-docs.md",
        "`roadmap/roadmap.md`, `ledger/deferral-ledger.md`, `decisions-log/decisions-log.md`",
        "the fixed-slug path list states the same retired folder homes one paragraph on — a \
         cell-scoped fix would have left the doc contradicting itself twice over",
    ),
    (
        "implementation/module-layout.md",
        "an open question in [workflow-dialect.md](../design/workflow-dialect.md)",
        "the emitted-format micro-syntax was decided 2026-05-28 and `workflow-dialect.md` \
         carries the full `Emitted format` section it points at",
    ),
    (
        "design/workflow-dialect.md",
        "the emitted-format micro-syntax is an open question",
        "the doc declaring it open is the doc that specifies it, three sections down",
    ),
    (
        "design/workflow-dialect.md",
        "exact framing settles with the emitted-format micro-syntax",
        "the sibling site, and its `#open-questions` target no longer carries the item at \
         all — a pointer to an absence",
    ),
    (
        "design/write-commands.md",
        "the on-disk format is an open question",
        "the instance on-disk format is owned by `storage.md`, its parser boundary locked \
         2026-05-28, and the schema-definition format has been frozen v1 since M34",
    ),
    (
        "implementation/decisions-pending.md",
        "Confirm and remove the stale ref.",
        "the owed cleanup is discharged by this task, so the entry states the discharge \
         instead of re-owing it",
    ),
];

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot; a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "design/doc-read-surface.md",
        "no **dev-pack** repeatable declares two slots",
    ),
    (
        "design/doc-read-surface.md",
        "is a shipped multi-slot repeatable, and since M16 the only one in either pack",
    ),
    (
        "design/methodology-docs.md",
        "`placement: { file: docs/roadmap.md }`",
    ),
    (
        "design/methodology-docs.md",
        "`placement: { file: docs/deferral-ledger.md }`",
    ),
    (
        "design/methodology-docs.md",
        "`placement: { file: docs/decisions-log.md }`",
    ),
    (
        "design/methodology-docs.md",
        "the placement homes the table above states",
    ),
    (
        "design/methodology-docs.md",
        "**The bound is a deferral, not a limit",
    ),
    (
        "design/methodology-docs.md",
        "*Trigger:* a **second** independent driver that is not this repo's own planning \
         convenience",
    ),
    (
        "design/methodology-docs.md",
        "**the trigger fired and the capability shipped**",
    ),
    (
        "implementation/module-layout.md",
        "- **Settled dependency**",
    ),
    (
        "design/workflow-dialect.md",
        "the emitted-format micro-syntax is **settled**",
    ),
    (
        "design/workflow-dialect.md",
        "the exact framing is the settled micro-syntax",
    ),
    (
        "design/write-commands.md",
        "the on-disk format is **not** an open question",
    ),
    (
        "implementation/decisions-pending.md",
        "*discharged 2026-08-31 (M49 Increment 11 / T10)*",
    ),
];

/// The **fact** sweep's inputs: `(file, subject token)`. A line naming the subject may not
/// also call it open — no phrasing of it, not just the byte-forms above.
const OPEN_QUESTION_SUBJECTS: &[(&str, &str)] = &[
    ("design/workflow-dialect.md", "micro-syntax"),
    ("implementation/module-layout.md", "micro-syntax"),
    ("design/write-commands.md", "on-disk format"),
];

/// The withdrawal sentences — the one place each retired phrasing may still appear, because a
/// withdrawal must be free to quote the bytes it retires. `(file, sentence)`, each asserted
/// exactly once and stripped before the fact sweep runs.
const WITHDRAWALS: &[(&str, &str)] = &[
    (
        "design/write-commands.md",
        "the on-disk format is **not** an open question",
    ),
    (
        "implementation/module-layout.md",
        "which is **not** an open question",
    ),
];

/// The nesting fact, keyed as a fact: a line naming the nesting subject may not also state
/// the retired non-expressibility mechanism. `(subject, retired claim)` over one file.
const NESTING_SUBJECTS: &[&str] = &[
    "repeatable-inside-repeatable",
    "leaf::repeatable",
    "multi-level repetition",
];
const NESTING_RETIRED_CLAIMS: &[&str] = &["not expressible", "leaf = slot | field"];

/// Files carrying the same falsified reason that stay **out** of the sweep, each with the
/// structural marker that makes the exclusion checkable and the reason it holds.
/// `(file, locus marker, why)`.
const EXCLUDED_HISTORICAL: &[(&str, &str, &str)] = &[
    (
        "design/changelog.md",
        "## The engine work (the four lifts; risk-first, the M13/M16 cold-start discipline)",
        "a PRE-BUILD plan section whose whole tense is the plan's own — one bullet carries \
         four `today …` clauses about the engine as it stood at M22 planning. Repairing one \
         clause would leave the section less coherent than it is; its conclusion (the cap and \
         its derivation) was already corrected in place by M49 Increment 5",
    ),
    (
        "implementation/decisions-pending.md",
        "**GRADUATED — built at M22 (2026-06-14)**",
        "the multi-level-repetition ledger row states `today not expressible` and its own \
         graduation in the SAME row — the ledger's idiom is a row that carries its history, \
         and the reader is not misled by a sentence its own bold clause retires",
    ),
];

/// **Arm 1 — nothing states the falsified reason.**
#[test]
fn the_record_no_longer_states_the_reasons_head_falsifies() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — each replacement is stated once, in one home.**
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

/// **Arm 3 — the *fact* sweep: nothing calls a settled question open, in any phrasing.**
///
/// The byte list above is readable and therefore maskable — a future pass could restate the
/// retired fact in fresh words and walk through it. So the subject is swept line by line, with
/// each declared withdrawal sentence stripped first.
#[test]
fn no_line_still_calls_a_settled_question_open() {
    let mut bodies: BTreeMap<&str, String> = BTreeMap::new();
    for (file, _) in OPEN_QUESTION_SUBJECTS {
        bodies.entry(file).or_insert_with(|| read_doc(file));
    }
    for (file, sentence) in WITHDRAWALS {
        let body = bodies
            .get_mut(file)
            .unwrap_or_else(|| panic!("{file} must be in the fact sweep to declare a withdrawal"));
        assert_eq!(
            count(body, sentence),
            1,
            "{file} must carry the withdrawal `{sentence}` exactly once — a fact is retired \
             where it was written, never annotated around",
        );
        *body = body.replace(sentence, "");
    }
    for (file, subject) in OPEN_QUESTION_SUBJECTS {
        let body = &bodies[file];
        for line in body.lines() {
            let lower = line.to_lowercase();
            assert!(
                !(lower.contains(subject) && lower.contains("open question")),
                "{file} still calls `{subject}` an open question — it was settled \
                 2026-05-28. Offending line:\n{line}",
            );
        }
    }
}

/// **Arm 4 — the nesting bound is never re-stated as a limit of the schema model.**
#[test]
fn the_nesting_bound_is_never_restated_as_a_limit() {
    let body = read_doc("design/methodology-docs.md");
    for line in body.lines() {
        let lower = line.to_lowercase();
        if !NESTING_SUBJECTS.iter().any(|s| lower.contains(s)) {
            continue;
        }
        for claim in NESTING_RETIRED_CLAIMS {
            assert!(
                !lower.contains(claim),
                "methodology-docs.md states the retired mechanism `{claim}` on a nesting \
                 line — `Leaf::Repeatable` shipped at M22; the roadmap tier is DEFERRED, not \
                 unexpressible. Offending line:\n{line}",
            );
        }
    }
}

/// **Arm 5 — the surviving claim is derived from the shipped packs, not from prose.**
///
/// `doc-read-surface.md` now says two checkable things: **no dev-pack repeatable declares two
/// slots**, and **`roadmap.milestones` is the only shipped multi-slot repeatable, in either
/// pack**. Both are read off the real schemas through the production `load_pack_schema` path,
/// so growing the set reddens the sentence at the point membership is decided — the set-fence
/// rule, applied to a doc claim.
#[test]
fn the_multi_slot_claim_is_derived_from_the_shipped_schemas() {
    let dev = EmbeddedPack::new();
    let methodology = EmbeddedPack::methodology();
    let composite = CompositePack::new(vec![
        Box::new(EmbeddedPack::new()),
        Box::new(EmbeddedPack::methodology()),
    ]);

    let dev_multi = multi_slot_blocks(&dev, &composite);
    assert!(
        dev_multi.is_empty(),
        "a dev-pack repeatable now declares two slots ({dev_multi:?}) — \
         `design/doc-read-surface.md`'s surviving bound (*no dev-pack repeatable declares two \
         slots*) is falsified and must be rewritten with the witness that killed it",
    );

    let methodology_multi = multi_slot_blocks(&methodology, &composite);
    assert_eq!(
        methodology_multi,
        vec!["roadmap#milestones".to_string()],
        "`design/doc-read-surface.md` states `roadmap.milestones` is the ONLY shipped \
         multi-slot repeatable in either pack; the shipped schemas now say otherwise",
    );

    // And the citation the prose makes: the fixture's doc-comment is where the correctly
    // scoped bound lives, which is what makes the design doc's generalization a dropped
    // qualifier rather than a fresh mistake.
    assert!(
        read_doc("crates/cli/tests/doc_show_item_leaf.rs").contains("no dev-pack repeatable"),
        "doc_show_item_leaf.rs must still carry the dev-pack-scoped bound the design doc now \
         cites as the correctly scoped original",
    );
}

/// Every `<type>#<section>` (or `<type>#<section>/<nested>`) repeatable block in `pack` that
/// declares **two or more** slots, sorted — the id-ordered enumeration, never iteration order.
fn multi_slot_blocks(pack: &dyn PackSource, types_from: &dyn PackSource) -> Vec<String> {
    let mut found = Vec::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = pack
            .read(PackResourceKind::Schemas, &id)
            .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
        let schema = load_pack_schema(types_from, &bytes)
            .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
        for section in &schema.sections {
            if let SectionBody::Repeatable { repeatable } = &section.body {
                walk_block(
                    &format!("{}#{}", schema.ty, section.id),
                    &repeatable.block,
                    &mut found,
                );
            }
        }
    }
    found.sort();
    found
}

/// Recurse a block, recording it when it declares ≥2 slots, then descending into any nested
/// repeatable so the claim covers every level, not only the first.
fn walk_block(path: &str, block: &[Leaf], found: &mut Vec<String>) {
    let slots = block
        .iter()
        .filter(|leaf| matches!(leaf, Leaf::Slot { .. }))
        .count();
    if slots >= 2 {
        found.push(path.to_string());
    }
    for leaf in block {
        if let Leaf::Repeatable { id, repeatable } = leaf {
            walk_block(&format!("{path}/{id}"), &repeatable.block, found);
        }
    }
}

/// **Arm 6 — the exclusions stay stated decisions, not oversights.**
///
/// Each excluded file must still carry the structural marker the exclusion rests on. If the
/// M22 plan section is rewritten as present-tense record, or the ledger row loses the
/// graduation stamp that makes it self-correcting, the premise is gone and this reddens —
/// forcing re-adjudication rather than letting a silent exclusion stand. The historical text
/// itself is deliberately **not** asserted: that would pin a retired reason as expected output.
#[test]
fn the_historical_exclusions_still_hold_their_premise() {
    for (file, marker, why) in EXCLUDED_HISTORICAL {
        let body = read_doc(file);
        assert_eq!(
            count(&body, marker),
            1,
            "{file} must carry `{marker}` exactly once — the exclusion rests on it: {why}",
        );
    }
}
