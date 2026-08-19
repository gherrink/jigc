//! The ledger's own fence for M46 Increment 9 — the planning record tells the truth about
//! itself, not only about the code.
//!
//! Entry 12 (*`setup --format json` `hook_file`*) was written at M47 with a **declared
//! bound**: the value was computed and carried, and only the JSON key was withheld,
//! because an additive machine key sat outside that wave's boundary. M48's text/JSON
//! parity sweep shipped the key. The M46 razor re-read the entry and found the bound
//! **stale** — so it is *struck, not carried*: a bound that survives its own discharge is
//! an artefact the next planner sizes work against, exactly the failure this increment
//! exists to close.
//!
//! This suite fences that discharge on the entry-7 precedent
//! (`crates/cli/tests/ledger_entry_seven_discharged.rs`):
//!
//!   * the bound sentence is **gone** — match count 0 over the whole ledger, not annotated
//!     around, not softened in place;
//!   * the numbered entry **names what discharged it**, quoting the driven datum
//!     (`hook_file` *and* `hook_committed` under a `core.hooksPath` install) and the fence
//!     that now holds the key, so the discharge is re-checkable rather than asserted;
//!   * the disposition table's basis cell carries the same citation, while its
//!     disposition cell stays byte-exact — `ledger_entry_seven_discharged::
//!     no_other_entrys_keying_moves` owns the keying, and this suite must not move it.
//!
//! These are ledger-content assertions by nature — the deliverable *is* the record. The
//! behaviour it records is driven through the shipped surfaces by
//! `crates/cli/tests/text_json_parity_axis.rs` (the parity fence over `setup`'s envelope)
//! and `crates/cli/tests/setup.rs` (the envelope's `hook_file` read through the binary),
//! which is why this file asserts only that the ledger says what those suites hold.

use std::fs;
use std::path::{Path, PathBuf};

const LEDGER: &str = "implementation/decisions-pending.md";

fn read_ledger() -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .join(LEDGER);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("{LEDGER} must exist: {path:?}"))
}

/// The numbered entry itself — the prose entry under *the capability wave*, keyed by its
/// leading `N. **`, as distinct from the one-line disposition row in the table above it.
fn numbered_entry(body: &str, number: u32) -> &str {
    let prefix = format!("{number}. **");
    body.lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("the ledger must carry a numbered entry `{prefix}…`"))
}

/// The **M46** disposition row — entry keys repeat across the M48 table above it, so the
/// row is found inside the M46 table only.
fn m46_row<'a>(body: &'a str, key: &str) -> &'a str {
    let marker = "> | # | M46 disposition | Basis (exercised unless marked) |\n";
    let start = body
        .find(marker)
        .expect("the ledger must carry the M46 disposition table header");
    let table = &body[start + marker.len()..];
    let end = table.find("\n>\n").unwrap_or(table.len());
    let prefix = format!("> | {key} |");
    table[..end]
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("the M46 table must carry a row keyed `{key}`"))
}

/// **The stale bound is gone.** Struck, not carried — so the sentence occurs nowhere.
#[test]
fn entry_twelve_bound_sentence_is_struck() {
    let body = read_ledger();
    assert_eq!(
        body.matches("still cannot learn where the hook landed")
            .count(),
        0,
        "{LEDGER} still carries entry 12's declared bound. The key it says is withheld \
         ships (`render.rs`'s install envelope) and is fenced (`text_json_parity_axis.rs`, \
         `tests/setup.rs`), so the sentence is false at HEAD — a discharged entry's stale \
         bound is struck, not carried",
    );
}

/// **The discharge names what discharged it** — the driven datum and the fence, quoted, so
/// the next planner can re-check the claim instead of inheriting it.
#[test]
fn entry_twelve_names_what_discharged_it() {
    let body = read_ledger();
    let entry = numbered_entry(&body, 12);

    for (needle, why) in [
        (
            "DISCHARGED",
            "the discharge mark itself — an entry whose withheld key has shipped is \
             discharged, not still owed to a Settle",
        ),
        (
            "`core.hooksPath`",
            "the install shape the datum was driven under — the shape that produced the \
             bound in the first place",
        ),
        ("`hook_file`", "the key that was withheld and now ships"),
        (
            "`hook_committed`",
            "the second key driven alongside it — the envelope answers *where* and \
             *whether it rides the install commit*, which is the whole of what the bound \
             said a driver could not learn",
        ),
        (
            "\".githooks/pre-commit\"",
            "the driven value, quoted rather than paraphrased",
        ),
        (
            "crates/cli/tests/text_json_parity_axis.rs",
            "the fence that holds the key — parity over `setup`'s envelope, so the key \
             cannot be dropped back out silently",
        ),
    ] {
        assert!(
            entry.contains(needle),
            "entry 12 must carry `{needle}` — {why}\nentry was:\n{entry}",
        );
    }
}

/// **The row's basis cell carries the same citation, and its keying does not move.** The
/// disposition cell is byte-exact `**DISCHARGED**`; `ledger_entry_seven_discharged` pins
/// the twelve rows, and this amendment must leave that pin untouched.
#[test]
fn entry_twelve_row_states_the_basis_without_re_keying() {
    let body = read_ledger();
    let row = m46_row(&body, "12 `setup --format json` `hook_file`");

    assert!(
        row.starts_with("> | 12 `setup --format json` `hook_file` | **DISCHARGED** |"),
        "entry 12 keeps its key and its byte-exact disposition cell — the amendment \
         belongs in the basis cell\nrow was:\n{row}",
    );
    for needle in [
        "`core.hooksPath`",
        "`hook_committed`",
        "crates/cli/tests/text_json_parity_axis.rs",
    ] {
        assert!(
            row.contains(needle),
            "the basis cell must carry `{needle}` — the table is what a planner scans, so \
             the driven datum and its fence live there too\nrow was:\n{row}",
        );
    }
}
