//! The ledger's own fence for M46 Increment 5 — entry 7 stops reading as an open
//! disposition and records what discharged it (T3).
//!
//! Entry 7 (*the item-slot repair verb*) was **retired** at the M46 razor, not deferred:
//! driving the binary showed a shipped verb chain already repairs a pre-guard corpus, so
//! the missing thing was never a verb. What the retirement left open was a *route* — and
//! the roadmap posed it as a fork: *"either the conflict-block route names the migrate
//! entry, or a sanctioned way to drop the stale baseline first."* The row still read
//! *"What ships is the **route, with two arms**"*, which is a disposition in the future
//! tense over behaviour that has since landed.
//!
//! This suite fences the discharge:
//!
//!   * the open-disposition sentence is **gone**, not annotated around;
//!   * the row names `jigc unmanage` and **both** shipped clauses (T2), the T1 control
//!     that proves the *verb* was never the gap, and the **falsifying datum** that closed
//!     the fork on its second arm — quoted, because a fork closed against its own record
//!     is a basis-has-changed rebuttal, not an override (`DECISIONS.md` → 2026-08-09);
//!   * **no other entry's keying moves.** The M46 disposition table's twelve rows are
//!     pinned by their first two cells, so a discharge cannot quietly re-key a neighbour.
//!
//! These are ledger-content assertions by nature — the deliverable *is* the record. The
//! behaviour it records is driven through the real binary in
//! `crates/cli/tests/pre_guard_repair_route.rs`, which is why this file asserts only that
//! the ledger says what that suite drives.

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

/// The **M46** disposition table only — entry keys repeat across the M48 table above it,
/// so a fence over the whole file would pin the wrong row.
fn m46_table(body: &str) -> String {
    let marker = "> | # | M46 disposition | Basis (exercised unless marked) |\n";
    let start = body
        .find(marker)
        .expect("the ledger must carry the M46 disposition table header");
    let rest = &body[start + marker.len()..];
    let end = rest.find("\n>\n").unwrap_or(rest.len());
    rest[..end].to_string()
}

fn row<'a>(table: &'a str, key: &str) -> &'a str {
    let prefix = format!("> | {key} |");
    table
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("the M46 table must carry a row keyed `{key}`"))
}

/// **The open disposition is gone.** The row said what *would* ship; it now says what did.
#[test]
fn entry_seven_no_longer_reads_as_an_open_disposition() {
    let body = read_ledger();
    assert_eq!(
        body.matches("What ships is the **route, with two arms**")
            .count(),
        0,
        "{LEDGER} still states entry 7's disposition in the future tense — the route \
         shipped at M46 Increment 5, so the sentence is a record of the past or it is a lie",
    );
}

/// **The discharge names what discharged it** — the verb the route reaches for, both
/// clauses, the control that kept the retirement true, and the datum that closed the fork.
#[test]
fn entry_seven_carries_the_discharge_and_what_shipped() {
    let body = read_ledger();
    let table = m46_table(&body);
    let row = row(&table, "7 item-slot repair verb");

    for (needle, why) in [
        (
            "DISCHARGED",
            "the discharge mark itself — a retired entry with shipped behaviour behind it \
             is discharged, not still open",
        ),
        (
            "`jigc unmanage <source>`",
            "the shipped verb the migration clause routes at, with the substituted-path \
             shape the route actually prints",
        ),
        (
            "(a)",
            "clause (a) — the migration arm, keyed on the task's own recorded source path",
        ),
        (
            "(b)",
            "clause (b) — the general clause gaining the out-of-band sanction its sibling \
             already carried",
        ),
        (
            "`write.wrong-shape`",
            "T1's standing control: `jigc doc add-item` is still refused at the offending \
             address, which is what proves the *verb* was never what was missing",
        ),
        (
            "`reconciliation.conflict-block`",
            "the falsifying datum, quoted — the migrate chain is where the block fires",
        ),
        (
            "basis-has-changed rebuttal, not an override",
            "the fork closed against its own recorded arm, named as the rebuttal shape \
             this repo requires rather than a silent swap",
        ),
    ] {
        assert!(
            row.contains(needle),
            "entry 7's row must carry `{needle}` — {why}\nrow was:\n{row}",
        );
    }
}

/// **No other entry's keying moves.** Each row is pinned by `| <key> | <disposition> |`,
/// so amending one entry cannot re-key, re-dispose, or drop a neighbour.
#[test]
fn no_other_entrys_keying_moves() {
    const PINNED: &[(&str, &str)] = &[
        ("1 gate-command / evidence", "**RE-KEYED**"),
        (
            "2 checkpoint / `planning-record`",
            "**RE-KEYED, and taken off the schema window**",
        ),
        ("3 acknowledged-findings ledger", "**RETIRED as posed**"),
        ("4 managed-doc read-side / search", "**RE-KEYED, post-1.0**"),
        (
            "5 Pest / non-Rust is-a-test tier",
            "**SPLIT — the message ships, the tier RE-KEYED**",
        ),
        ("6 symbol-mention-sweep", "**RE-KEYED**"),
        ("8 file-state introspection", "**RETIRED**"),
        (
            "9 store locking / lost updates",
            "**IN — admitted at the falsified predicate**",
        ),
        (
            "10 durable \"was provisioned\"",
            "**RETIRED — residue empty, driven both arms**",
        ),
        (
            "11 dirty-worktree guard",
            "**RE-CUT, and the re-cut ships**",
        ),
        ("12 `setup --format json` `hook_file`", "**DISCHARGED**"),
    ];

    let body = read_ledger();
    let table = m46_table(&body);

    for (key, disposition) in PINNED {
        let cells = format!("> | {key} | {disposition} |");
        assert_eq!(
            table.matches(&cells).count(),
            1,
            "the M46 table must still key entry `{key}` at `{disposition}`, exactly once — \
             entry 7's discharge moves entry 7 and nothing else",
        );
    }

    assert_eq!(
        table.lines().filter(|l| l.starts_with("> | ")).count(),
        12,
        "the M46 table must still hold exactly twelve entry rows — no entry is added or \
         dropped by a discharge",
    );

    // Entry 7 keeps its key and its retirement verdict; only the discharge is new.
    let seven = row(&table, "7 item-slot repair verb");
    assert!(
        seven.starts_with(
            "> | 7 item-slot repair verb | **RETIRED — the verb is not what is missing**"
        ),
        "entry 7 keeps its key and the verdict the razor reached — the discharge records \
         what shipped, it does not re-adjudicate\nrow was:\n{seven}",
    );
}
