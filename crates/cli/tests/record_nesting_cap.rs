//! The record's own fence for M49 Increment 5 — the four docs stop stating a nesting cap
//! the tool does not enforce (T2).
//!
//! T1 made [`engine::schema::MAX_NESTING_DEPTH`] a **derived** number: it is computed as
//! `(engine::address::MAX_FRAGMENT_HOPS − 1) / 2`, the deepest nesting level whose *leaf
//! write* still fits the address fragment hop budget (a leaf write at depth `D` costs
//! `2D + 1` hops, because the address chain alternates an item id and the nested-section id
//! that declares the next block). Before it, three ceilings shipped and all three differed —
//! the loader admitted **4**, `add-item` reached **3**, a leaf write reached **2**, and
//! `doc schema` advertised the difference as six addresses `Address::parse` refuses
//! (`DECISIONS.md` → 2026-08-29 M49 Increment 5 (T1)).
//!
//! **The retired fact: *repeatable nesting is capped at four levels by H6*.** The record
//! stated it in four live homes — `design/structural-grammar.md` (→ Repetition, and its
//! Open-questions echo), `design/changelog.md` (engine work #1, and the *supports up to 4
//! nesting levels* line under *What stays deferred*), `implementation/parsing.md` (the
//! heading-depth ceiling's derivation, whose `schema.rs:584` citation was stale as well as
//! wrong), and `implementation/decisions-pending.md` (the graduated multi-level-repetition
//! entry, and the nesting-beyond-2 trigger). Both halves of the fact are now false: the
//! **number** is 2, not 4, and the **reason** is the address hop budget, not the `H6` render
//! ceiling — which is the *other*, now non-binding, cap.
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The
//! behaviour it describes is driven through the real binary by the manufactured-shape arms
//! in `crates/cli/tests/doc_read_surface.rs` (every address the projection advertises parses
//! and round-trips at the cap; a pack one level deeper is refused at pack-load) and by
//! `crates/engine/src/schema.rs`'s own re-derived cap tests, which is why this file asserts
//! only that the record says what those suites drive.
//!
//! **A *fact* sweep, not a phrasing sweep** — the `record_item_slot_kind.rs` /
//! `record_set_splice_retired.rs` precedent. The retired fact is a **conjunction**: a line
//! that names the nesting-cap subject *and* states either the retired **number** (arm 2) or
//! the retired **reason** (arm 3). Keying on the conjunction rather than on a byte-form is
//! what makes the sweep unmaskable — a future pass cannot re-state the retired fact in fresh
//! words. Naming `H6` at all is deliberately **not** the trigger: the new prose names it, as
//! the render ceiling the address budget now runs out ahead of. What arm 3 refuses is `H6`
//! presented as *the* cap on nesting.
//!
//! **Two files are deliberately out of the sweep**, by name and with the reason —
//! [`EXCLUDED_HISTORICAL`]. Arm 6 fences that exclusion structurally so it stays a stated
//! decision rather than an oversight, without asserting the historical text itself (which
//! would pin a retired fact as expected output).

use std::fs;
use std::path::{Path, PathBuf};

use engine::schema::MAX_NESTING_DEPTH;

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

/// The four **live** record files the retired fact is swept over.
const SWEPT: &[&str] = &[
    "design/structural-grammar.md",
    "design/changelog.md",
    "implementation/parsing.md",
    "implementation/decisions-pending.md",
];

/// The dated, append-only logs held **out** of the sweep, each with why. They record what was
/// *built*, including M22's own cap and M49's entry retiring it, so rewriting them would
/// falsify the history rather than correct the record (the Increment-4-T3 discipline).
const EXCLUDED_HISTORICAL: &[(&str, &str)] = &[
    (
        "DECISIONS.md",
        "the dated decision log — its M22 entries record the H6/4-level cap as the call that \
         was made, and its 2026-08-29 entry quotes the retired number in order to retire it",
    ),
    (
        "implementation/roadmap.md",
        "Milestone 22's decomposition — a record of the scope that was built at the time, not \
         a statement of current truth",
    ),
];

/// The falsified statements, each with the file it lives in and why it is now false. Named
/// with the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/structural-grammar.md",
        "recursively, capped at **four nesting levels**",
        "the design of record for Repetition — the sentence every other home echoes",
    ),
    (
        "design/structural-grammar.md",
        "capped at four nesting levels (`H6`)",
        "the Open-questions echo, which carried both halves of the retired fact in six words",
    ),
    (
        "design/changelog.md",
        "capped at **`######` (H6) → max 4 nesting levels**",
        "engine work #1 — the lift's own statement of the cap it built",
    ),
    (
        "design/changelog.md",
        "the engine supports up to 4 nesting levels",
        "the *What stays deferred* line, which told a reader two more levels were available",
    ),
    (
        "implementation/parsing.md",
        "`MAX_NESTING_DEPTH = 4`, `schema.rs:584`",
        "the heading-depth ceiling's derivation — wrong in the number *and* stale in the \
         citation, and the constant is now computed rather than written down at any line",
    ),
    (
        "implementation/decisions-pending.md",
        "capped H6/4 levels",
        "the graduated multi-level-repetition entry",
    ),
    (
        "implementation/decisions-pending.md",
        "general recursion (up to 4 levels)",
        "the nesting-beyond-2 trigger, whose own condition depends on the real ceiling",
    ),
];

/// The subject: the repeatable **nesting cap**, however the record names it.
const NESTING_SUBJECT: &[&str] = &[
    "nesting level",
    "nesting depth",
    "nesting-depth",
    "MAX_NESTING_DEPTH",
    "nest a repeatable",
    "repeatable nesting",
    "Nesting is bounded",
    "caps nesting",
    "repeatable level",
];

/// The retired **number** — four, stated as the cap. A line naming the nesting subject and
/// carrying one of these states a ceiling the loader no longer admits.
const RETIRED_NUMBER: &[&str] = &[
    "four nesting levels",
    "4 nesting levels",
    "four levels",
    "4 levels",
    "max 4",
    "up to 4",
    "MAX_NESTING_DEPTH = 4",
];

/// The retired **reason** — `H6` presented as *the* cap on nesting. Naming `H6` as the render
/// ceiling is fine and the new prose does it; claiming it is what bounds nesting is not.
const RETIRED_REASON: &[&str] = &[
    "H6 cap",
    "capped at H6",
    "capped at `H6`",
    "capped H6",
    "(`H6`)",
    "(H6)",
    "H6 →",
    "H6) →",
];

/// The binding constraint, however a home names it — every swept file must state the reason
/// beside the number, not the number alone.
const BINDING_REASON: &[&str] = &["address hop budget", "hop budget", "MAX_FRAGMENT_HOPS"];

/// The derivation's **arithmetic**, which belongs in exactly one home.
const ARITHMETIC: &[&str] = &["2D + 1", "(MAX_FRAGMENT_HOPS"];

/// The one home the arithmetic is stated in; the other three name the reason and point here.
const ARITHMETIC_HOME: &str = "design/structural-grammar.md";

/// **Arm 1 — nothing states the falsified statements, byte for byte.**
#[test]
fn the_record_no_longer_states_the_retired_cap() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — the retired *number*, swept over each file's lines rather than one phrasing.**
#[test]
fn the_nesting_subject_is_never_capped_at_the_retired_number() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, line) in body.lines().enumerate() {
            let subject = NESTING_SUBJECT.iter().any(|s| line.contains(s));
            let retired = RETIRED_NUMBER.iter().any(|c| line.contains(c));
            assert!(
                !(subject && retired),
                "{file}:{} caps repeatable nesting at the retired number — whatever words carry \
                 it. The loader admits {MAX_NESTING_DEPTH} (M49 Inc-5 T1: \
                 `engine::schema::MAX_NESTING_DEPTH`, derived from the address hop budget); \
                 state the history in the dated logs, not here.\n  {line}",
                n + 1,
            );
        }
    }
}

/// **Arm 3 — the retired *reason*: `H6` presented as the cap on nesting.**
#[test]
fn the_nesting_cap_is_never_attributed_to_the_heading_ceiling() {
    for file in SWEPT {
        let body = read_doc(file);
        for (n, line) in body.lines().enumerate() {
            let subject = NESTING_SUBJECT.iter().any(|s| line.contains(s));
            let retired = RETIRED_REASON.iter().any(|c| line.contains(c));
            assert!(
                !(subject && retired),
                "{file}:{} makes the `H6` heading ceiling the reason nesting is capped — the \
                 address hop budget runs out first, and H6 is the other, now non-binding, cap \
                 (M49 Inc-5 T1). Naming H6 as the render ceiling is fine; naming it as *the* \
                 cap is the retired fact.\n  {line}",
                n + 1,
            );
        }
    }
}

/// **Arm 4 — the shipped number is stated in each home, and read from the constant.**
///
/// The needle is *generated* from [`MAX_NESTING_DEPTH`], so the day the derived constant moves
/// — because the address grammar widened — every home that failed to follow it reddens here
/// instead of quietly going stale, which is the whole failure mode this task exists to close.
#[test]
fn every_home_states_the_shipped_number() {
    let stated = format!("{MAX_NESTING_DEPTH} nesting levels");
    for file in SWEPT {
        let body = read_doc(file);
        assert!(
            body.contains(&stated),
            "{file} must state the shipped cap as `{stated}` — the number \
             `engine::schema::MAX_NESTING_DEPTH` computes today",
        );
    }
}

/// **Arm 5 — and states its reason, so the number is checkable rather than memorized.**
#[test]
fn every_home_states_the_binding_reason() {
    for file in SWEPT {
        let body = read_doc(file);
        assert!(
            BINDING_REASON.iter().any(|r| body.contains(r)),
            "{file} states the cap without its reason — the binding constraint is the address \
             hop budget (`engine::address::MAX_FRAGMENT_HOPS`), not the heading grammar. A \
             number with no reason is what let three ceilings drift apart.",
        );
    }
}

/// **Arm 6 — the derivation's arithmetic lives in exactly one home.**
///
/// Cross-reference, never restate: the other three name the reason and point at Repetition.
#[test]
fn the_arithmetic_is_stated_in_one_home() {
    for file in SWEPT {
        let body = read_doc(file);
        let carries = ARITHMETIC.iter().any(|a| body.contains(a));
        if *file == ARITHMETIC_HOME {
            assert!(
                carries,
                "{file} is the derivation's home ({ARITHMETIC_HOME} → Repetition) and must \
                 carry the arithmetic the number falls out of",
            );
        } else {
            assert!(
                !carries,
                "{file} restates the derivation's arithmetic — it belongs in \
                 {ARITHMETIC_HOME} → Repetition alone, which the other homes cross-reference",
            );
        }
    }
}

/// **Arm 7 — the exclusion is a stated decision, fenced structurally.**
///
/// The dated logs are named with their reason and are provably not in the swept set, so a
/// later pass cannot quietly widen or narrow the sweep. Their *content* is deliberately not
/// asserted: pinning the retired sentence as expected output is how a record sweep turns into
/// a fence that protects the thing it retired.
#[test]
fn the_historical_logs_are_excluded_by_name_with_a_reason() {
    for (file, why) in EXCLUDED_HISTORICAL {
        assert!(
            !SWEPT.contains(file),
            "{file} is excluded from the sweep ({why}) and must not also be swept",
        );
        assert!(
            repo_root().join(file).is_file(),
            "the excluded log {file} must exist — an exclusion naming a missing file is a \
             stale carve-out",
        );
        assert!(
            !why.is_empty(),
            "{file} must carry the reason it is excluded, in this file",
        );
    }
}
