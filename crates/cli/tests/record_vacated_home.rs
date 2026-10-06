//! M52 Increment 7 / T6 — **the record fence over `design/validation.md`'s vacated-home
//! statements.**
//!
//! Two obligations land in that doc with this task, and both are prose, so both are fenced
//! the way this repo fences prose ([`record_foreign_arm`](../../../tooling-tests/record_foreign_arm.rs)'s
//! pattern): **every falsified byte is named with the string it carried and asserted
//! absent**, and **every replacement is asserted exactly once** — a correction restated in a
//! second home is the rot the cross-reference rule exists to prevent, and a replacement
//! asserted zero times is the swap never landing.
//!
//! **Obligation 1 — the registration.** `schema-conformance.home-vacated` is a new blocking
//! store-scope cause and a new exit-flipping condition, and `validation.md` is the inventory
//! that says what the binary emits. The row must be there, and it must state the one thing a
//! reader meeting two near-identical orphan codes needs: *what separates it from
//! `schema-conformance.orphaned-instance`*.
//!
//! **Obligation 2 — residual 2 stops understating itself.** M51's completion audit narrowed
//! the orphan sweep's subject and wrote two residuals; the second said the unflagged orphan at
//! a root-level placement home *"is that one cell's pre-M51 status quo"*. Driven at this
//! wave's baseline ([baseline-freeze](../../../completions/artifacts/M52/baseline-freeze.md) →
//! §4 L-5; [gap-findings](../../../completions/artifacts/M52/gap-findings.md) → G-18) that is
//! false: under `placement-root: .` **five** methodology doctypes home at the repository root,
//! and with that pack dropped `jigc validate` exits 0 *"validates clean"* over three committed
//! stamped orphans. The line is rewritten into the half this increment closes and the half it
//! does not, with that datum, and with the note that the open half's deferral trigger — *the
//! next `schema-version` bump of any doctype* — cannot fire inside a wave that moves none, so
//! the un-namespaced stamp key crosses the 1.0 pin
//! ([settle-record](../../../completions/artifacts/M52/settle-record.md) → §16).
//!
//! **The kind of set this suite iterates: a hand-written byte list, and it says so.** No
//! registry knows which sentences a doc carries. What keeps it from being a list of literals
//! agreeing with themselves is the last arm, which asks the **binary** whether the membership
//! claim the record just made is true — the statement-==-constant idiom, pointed at the
//! record rather than at a message.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read_file(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("{rel} must exist: {path:?}"))
}

fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

const VALIDATION: &str = "design/validation.md";

/// The falsified statements — `(the bytes it carried, why it is now false)`. Named with the
/// bytes so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str)] = &[
    (
        "which is that one cell's pre-M51 status quo",
        "driven at this wave's baseline (freeze §4 L-5), `placement-root: .` homes five \
         methodology doctypes at the repo root and `jigc validate` exits 0 over three \
         stamped orphans — the residual's size is knob-dependent, and \"that one cell\" \
         understates it",
    ),
    (
        "`schema-conformance.unadopted-instance`, the table's fifth member",
        "an ordinal into `render::STORE_EXIT_FLIPS`, already off by one since M51 inserted \
         `orphaned-instance` ahead of it and off by two once `home-vacated` joins — the \
         settled rule is point at the table, never re-count",
    ),
    (
        "joins `render::STORE_EXIT_FLIPS` as its fifth member",
        "the same ordinal for the same code, one section earlier in the same doc — fixing \
         one occurrence and leaving the other is the restatement rot the exactly-once rule \
         below exists to catch",
    ),
];

/// The replacements, each asserted **exactly once**.
const REPLACEMENTS: &[&str] = &[
    // Obligation 2 — the two halves, stated as two.
    "The residual has two halves, and stating it as one cell was false",
    "The half that closes here",
    "The half that stays open",
    // Obligation 2 — L-5's datum, which is what falsified the old line.
    "`jigc config set placement-root .` homes **five** methodology doctypes at the \
     repository root",
    "three committed stamped orphans",
    // Obligation 2 — §16's crossing-the-pin note.
    "crosses the 1.0 pin",
    // Obligation 1 — the registration row and the distinction a reader meeting both codes
    // needs.
    "| `schema-conformance.home-vacated` |",
    "a still-resolved declared home that holds nothing, where its sibling above names a \
     stamped file no resolved doctype claims",
    // Obligation 1 — the membership, stated by pointing at the table rather than counting it.
    "joins `cli::render`'s `STORE_EXIT_FLIPS`",
    // The precedence decision, with the reason it was taken.
    "loss outranks non-adoption",
];

/// **Arm 1 — no falsified statement survives.**
#[test]
fn no_falsified_vacated_home_statement_survives() {
    let body = read_file(VALIDATION);
    for (needle, why) in FALSIFIED {
        assert_eq!(
            count(&body, needle),
            0,
            "{VALIDATION} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **Arm 2 — each replacement is stated once, in one home.**
#[test]
fn each_vacated_home_replacement_is_stated_exactly_once() {
    let body = read_file(VALIDATION);
    for needle in REPLACEMENTS {
        assert_eq!(
            count(&body, needle),
            1,
            "{VALIDATION} must state `{needle}` exactly once — zero times is the swap never \
             landing, twice is the restatement rot",
        );
    }
}

/// **Arm 3 — the record's membership claim is asked of the binary.**
///
/// The two arms above are a doc agreeing with itself. This one takes the fact the doc just
/// asserted — that this cause joins the store sweep's exit-flip axis — and asks
/// `cli::render::STORE_EXIT_FLIPS` whether it is true, through the member's own witness, so
/// the record cannot claim a membership the table does not carry.
#[test]
fn the_records_membership_claim_is_true_of_the_table() {
    let body = read_file(VALIDATION);
    assert!(
        body.contains(cli::orphan::HOME_VACATED_CODE),
        "{VALIDATION} must register the production code itself ({}), not a paraphrase of it",
        cli::orphan::HOME_VACATED_CODE,
    );
    assert!(
        cli::render::STORE_EXIT_FLIPS
            .iter()
            .any(|flip| (flip.witness)().code == cli::orphan::HOME_VACATED_CODE),
        "the record states this cause joins `render::STORE_EXIT_FLIPS`; the table must carry \
         a member whose own witness is that cause, else the inventory promises an exit the \
         binary does not take",
    );
}
