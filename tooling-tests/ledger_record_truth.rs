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
//! (`tooling-tests/ledger_entry_seven_discharged.rs`):
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

// ---------------------------------------------------------------------------------------
// T3 — no entry is left keyed at the fired M46 Settle, or resting on a count.
//
// The M46 Settle's own opening clause is a promise about this file: *"Every entry leaves
// this ledger; none stays deferred on a count."* At HEAD the promise was unkept — the
// Settle fired on 2026-08-18 and ten live items still routed their future to it, so the
// next planner reading a trigger would be sent to a gate that has already closed. That is
// the same rot entry 12's stale bound was struck for, one altitude up: a record that
// describes a decision nobody can still take.
//
// The arm below enumerates the ledger's whole M46 surface — the twelve numbered capability
// entries, the two `(D)` blocks that route to the Settle, and the four OUT-set items the
// Settle re-keyed by name (`razor-ledger.md` §3) — and asserts two things of each:
//
//   * it **records its M46 disposition** where the entry itself is read, not only in the
//     disposition table above it (a planner sizing work reads the entry's prose);
//   * **no forward key it carries names the M46 Settle** — every `Trigger:` /
//     `If it returns:` / `Re-opening condition:` clause routes somewhere still reachable.
//
// The forward key is read from its marker to the end of its line, so a re-key cannot be
// written and then walked back later in the same sentence.
//
// One deliberate exclusion: the M47 manifest-hash fence entry keeps a spent
// `"Trigger: the M46 Settle"` **inside quoted original text it explicitly labels spent**
// — that is a dated record of what was written, and rewriting it would be the
// falsification this suite exists to prevent. It is out of the enumerated set for that
// reason, not by oversight.

/// Where an enumerated item's text lives in the ledger.
enum Block {
    /// A numbered capability entry, taken with its `**Re-counted …**` continuation lines —
    /// a re-key written on the continuation must count, and a stale one there must redden.
    Numbered(u32),
    /// A whole ledger line, found by its leading text.
    Line(&'static str),
    /// One ` · `-separated segment of the M46 Settle's refused-and-re-keyed paragraph.
    /// The disposition is stated once for the paragraph; the forward key is per item.
    Refused(&'static str),
}

struct Item {
    /// What the failure message calls this row.
    name: &'static str,
    block: Block,
    /// The M46 disposition the entry's own text must record.
    disposition: &'static [&'static str],
}

/// The markers that open a forward key. `New trigger:` is caught by `trigger:`.
const KEY_MARKERS: [&str; 4] = [
    "Trigger:",
    "trigger:",
    "If it returns:",
    "Re-opening condition:",
];

const FIRED: &str = "M46 Settle";

/// The line that opens the M46 Settle's refused-and-re-keyed paragraph.
const REFUSED_LEAD: &str =
    "> **Refused with measured evidence behind them, re-keyed rather than dropped:**";

fn line_starting<'a>(body: &'a str, prefix: &str) -> &'a str {
    body.lines()
        .find(|line| line.starts_with(prefix))
        .unwrap_or_else(|| panic!("{LEDGER} must carry a line starting `{prefix}`"))
}

/// The capability-wave section only. The ledger carries several unrelated numbered lists
/// (the M48 Settle agenda, the surface-contract laws, the rc.7 forks), so a bare `N. **`
/// scan over the whole file reads the wrong list and silently fences nothing.
fn capability_wave(body: &str) -> &str {
    const HEADING: &str = "### The capability wave (M46)";
    let start = body
        .find(HEADING)
        .unwrap_or_else(|| panic!("{LEDGER} must carry the `{HEADING}` section"));
    let rest = &body[start + HEADING.len()..];
    let end = rest
        .find("\n### ")
        .map(|i| start + HEADING.len() + i)
        .unwrap_or(body.len());
    &body[start..end]
}

/// A numbered entry plus every line up to the next numbered entry — its re-counts included.
fn numbered_block(body: &str, number: u32) -> String {
    let body = capability_wave(body);
    let start = format!("{number}. **");
    let end = if number == 12 {
        "**Planning inputs from".to_string()
    } else {
        format!("{}. **", number + 1)
    };
    let mut taking = false;
    let mut out = Vec::new();
    for line in body.lines() {
        if line.starts_with(&start) {
            taking = true;
        } else if taking && line.starts_with(&end) {
            break;
        }
        if taking {
            out.push(line);
        }
    }
    assert!(
        !out.is_empty(),
        "{LEDGER} must carry a numbered capability entry `{start}…`",
    );
    out.join("\n")
}

/// `(where the disposition is read, where the forward keys are read)`, or the reason the
/// item has no text to read at all — a missing item is one failure among the others, not a
/// panic that hides the rest of the enumeration.
fn scopes(body: &str, block: &Block) -> Result<(String, String), String> {
    match block {
        Block::Numbered(n) => {
            let b = numbered_block(body, *n);
            Ok((b.clone(), b))
        }
        Block::Line(prefix) => {
            let l = line_starting(body, prefix).to_string();
            Ok((l.clone(), l))
        }
        Block::Refused(anchor) => {
            let paragraph = line_starting(body, REFUSED_LEAD);
            match paragraph.split(" · ").find(|seg| seg.contains(anchor)) {
                Some(segment) => Ok((paragraph.to_string(), segment.to_string())),
                None => Err(format!(
                    "the M46 Settle's refused paragraph does not name `{anchor}` — an \
                     OUT-set item the razor disposed by name has to be readable here, or \
                     the boundary's stated cost is stated nowhere a planner looks"
                )),
            }
        }
    }
}

/// Every forward key in `scope`, each read from its marker to the end of its line.
fn forward_keys(scope: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for line in scope.lines() {
        let mut from = 0usize;
        while from < line.len() {
            let hit = KEY_MARKERS
                .iter()
                .filter_map(|m| line[from..].find(m).map(|i| from + i))
                .min();
            match hit {
                Some(i) => {
                    out.push(&line[i..]);
                    from = i + 1;
                }
                None => break,
            }
        }
    }
    out
}

/// **The wave's claim, turned on the planning record.** Every M46 item states its own
/// disposition, and routes forward to something that can still happen.
#[test]
fn no_m46_entry_is_keyed_at_the_fired_settle_or_rests_on_a_count() {
    const ITEMS: &[Item] = &[
        Item {
            name: "entry 1 — gate-command / evidence rung",
            block: Block::Numbered(1),
            disposition: &["RE-KEYED"],
        },
        Item {
            name: "entry 2 — checkpoint / planning-record",
            block: Block::Numbered(2),
            disposition: &["RE-KEYED, and taken off the schema window"],
        },
        Item {
            name: "entry 3 — acknowledged-findings ledger",
            block: Block::Numbered(3),
            disposition: &["RETIRED as posed"],
        },
        Item {
            name: "entry 4 — managed-doc read-side / search",
            block: Block::Numbered(4),
            disposition: &["RE-KEYED, post-1.0"],
        },
        Item {
            name: "entry 5 — Pest / non-Rust is-a-test tier",
            block: Block::Numbered(5),
            disposition: &["SPLIT — the message ships, the tier RE-KEYED"],
        },
        Item {
            name: "entry 6 — symbol-mention-sweep",
            block: Block::Numbered(6),
            disposition: &["RE-KEYED"],
        },
        Item {
            name: "entry 7 — item-slot repair verb",
            block: Block::Numbered(7),
            disposition: &["RETIRED — the verb is not what is missing", "DISCHARGED"],
        },
        Item {
            name: "entry 8 — file-state introspection",
            block: Block::Numbered(8),
            disposition: &["RETIRED"],
        },
        Item {
            name: "entry 9 — store locking / lost updates",
            block: Block::Numbered(9),
            disposition: &["at the falsified predicate", "DISCHARGED"],
        },
        Item {
            name: "entry 10 — durable \"was provisioned\"",
            block: Block::Numbered(10),
            disposition: &["RETIRED — residue empty, driven both arms"],
        },
        Item {
            name: "entry 11 — dirty-worktree guard",
            block: Block::Numbered(11),
            disposition: &["DISPOSED at M46 Increment 2"],
        },
        Item {
            name: "entry 12 — `setup --format json` `hook_file`",
            block: Block::Numbered(12),
            disposition: &["DISCHARGED"],
        },
        Item {
            name: "(D) the M46 demand-counter re-count",
            block: Block::Line("- **(D) The M46 demand-counter re-count"),
            disposition: &["DISCHARGED"],
        },
        Item {
            name: "(D) the frozen methodology-doctype schema bump",
            block: Block::Line("> **(D) The frozen methodology-doctype schema bump"),
            disposition: &["CLOSED — take neither"],
        },
        Item {
            name: "OUT — `milestone finalize --dry-run`",
            block: Block::Refused("`milestone finalize --dry-run`"),
            disposition: &["re-keyed rather than dropped"],
        },
        Item {
            name: "OUT — the `unadopted-instance` cap/collapse",
            block: Block::Refused("`unadopted-instance`"),
            disposition: &["re-keyed rather than dropped"],
        },
        Item {
            name: "OUT — N-6, the text renderer dropping `location.address`",
            block: Block::Refused("**N-6**"),
            disposition: &["re-keyed rather than dropped"],
        },
        Item {
            name: "OUT — S12, the repo publishing floor",
            block: Block::Refused("**S12**"),
            disposition: &["**S12**", "discharged by citation"],
        },
    ];

    let body = read_ledger();
    let mut failures: Vec<String> = Vec::new();

    for item in ITEMS {
        let (disposition_scope, key_scope) = match scopes(&body, &item.block) {
            Ok(scopes) => scopes,
            Err(why) => {
                failures.push(format!("{}: {why}", item.name));
                continue;
            }
        };

        for needle in item.disposition {
            if !disposition_scope.contains(needle) {
                failures.push(format!(
                    "{}: its own text does not record the M46 disposition `{needle}` — the \
                     table above is what a planner *scans*, the entry is what a planner \
                     *reads*, and only one of them is quoted back into a scope brief",
                    item.name,
                ));
            }
        }

        let keys = forward_keys(&key_scope);
        if keys.is_empty() {
            failures.push(format!(
                "{}: carries no forward key at all — a ledger entry with no `Trigger:` / \
                 `If it returns:` / `Re-opening condition:` is a deferral with no way back, \
                 which is the failure this file's own preamble names",
                item.name,
            ));
        }
        for key in keys {
            if key.contains(FIRED) {
                failures.push(format!(
                    "{}: routes forward to the **fired** M46 Settle — it closed 2026-08-18, \
                     so this key can never fire again and the entry is stranded, not \
                     deferred\nkey was: {key}",
                    item.name,
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "the M46 Settle promised *\"Every entry leaves this ledger; none stays deferred on \
         a count\"* — {} item(s) in {LEDGER} still contradict it:\n\n{}",
        failures.len(),
        failures.join("\n\n"),
    );
}

// ---------------------------------------------------------------------------------------
// T4 — the M48 adjudication table's falsified rows are re-disposed *in place*, with the
// falsifying datum quoted.
//
// The M48 table is a **dated record**: it says what was decided on 2026-08-13, against the
// baseline exercised that day. Five of its twelve rows were later falsified by driving the
// binary at the M46 razor — 5, 7, 9, 11 at their stated predicate, and 10 re-based off an
// argument onto measurement — and four of the five were disposed *the other way*. Left
// bare, the table reads as live guidance: a planner scanning it finds `stay deferred |
// untouched` against work that has since shipped.
//
// The repair is **not** a rewrite. `DECISIONS.md`'s own convention for a dated record whose
// basis has moved is a dated correction bracket — the falsified basis stays visible beside
// what falsified it (`DECISIONS.md:998`, `:2376`, `:2771`, `:4706`) — and M47 named the
// shape a correction has to take when a call is re-taken against its own record: *"a
// basis-has-changed rebuttal, not an override"* (`DECISIONS.md` → 2026-08-09). So the M48
// disposition cells stay **byte-exact**, and each falsified row's basis cell gains a dated
// bracket that (a) names the datum that falsified it, quoted rather than paraphrased, and
// (b) names the M46 row that re-disposed it, so the reader lands on the live disposition
// instead of inferring one.
//
// The seven untouched rows are pinned by their first two cells, and the table by its row
// count, so a correction cannot quietly re-key, re-dispose or drop a neighbour — the same
// guard `ledger_entry_seven_discharged::no_other_entrys_keying_moves` puts on the M46 table
// below, which keys off the *M46* header and must stay green through this edit.
//
// These are ledger-content assertions by nature. The behaviour each quoted datum records is
// driven elsewhere — `pre_guard_repair_route.rs` (row 7), `file_state_merge_hand_off.rs` +
// `reconciliation_baseline_contrast.rs` (row 9), `milestone_path_subject.rs` (row 10),
// `milestone_teardown_loss.rs` (row 11) — which is why this file asserts only that the
// record says what those suites hold.

/// The **M48** adjudication table only. It sits *above* the M46 table and repeats its entry
/// keys, so it is found by its own header row — `Basis`, where M46's reads `Basis (exercised
/// unless marked)`.
fn m48_table(body: &str) -> String {
    let marker = "> | # | Disposition | Basis |\n";
    let start = body
        .find(marker)
        .expect("the ledger must carry the M48 adjudication table header");
    let rest = &body[start + marker.len()..];
    let end = rest.find("\n>\n").unwrap_or(rest.len());
    rest[..end].to_string()
}

fn m48_row<'a>(table: &'a str, key: &str) -> &'a str {
    let prefix = format!("> | {key} |");
    table
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("the M48 table must carry a row keyed `{key}`"))
}

/// The row's **dated** correction bracket — `**[Corrected YYYY-MM-DD …**]**`, the
/// convention `DECISIONS.md` uses to keep a falsified basis visible beside what falsified
/// it. An undated `[Corrected …]` is not one: the date is what makes it a record of when
/// the basis moved rather than an unattributed edit.
fn dated_bracket(row: &str) -> Option<&str> {
    const OPEN: &str = "**[Corrected ";
    let start = row.find(OPEN)?;
    let after = start + OPEN.len();
    let rest = &row[after..];
    let dated = rest.len() >= 10
        && rest.as_bytes()[..10].iter().enumerate().all(|(i, b)| {
            if i == 4 || i == 7 {
                *b == b'-'
            } else {
                b.is_ascii_digit()
            }
        });
    if !dated {
        return None;
    }
    let close = rest.find("**]**")?;
    Some(&row[start..after + close + "**]**".len()])
}

/// One falsified M48 row: where it is, which M46 row now governs it, and the datum that
/// moved it.
struct ReDisposed {
    /// The M48 row's key cell, verbatim.
    key: &'static str,
    /// The M46 row that re-disposed it — key and disposition, both quoted into the bracket
    /// so the reader lands on the live call instead of inferring it.
    m46_key: &'static str,
    m46_disposition: &'static str,
    /// `(quoted datum, why it has to be here)`.
    datum: &'static [(&'static str, &'static str)],
}

/// **The falsified rows are re-disposed, and each names what falsified it.**
#[test]
fn m48_falsified_rows_carry_a_dated_falsification_naming_the_m46_row() {
    const RE_DISPOSED: &[ReDisposed] = &[
        ReDisposed {
            key: "5 Pest is-a-test tier",
            m46_key: "5 Pest / non-Rust is-a-test tier",
            m46_disposition: "SPLIT — the message ships, the tier RE-KEYED",
            datum: &[
                (
                    "three frameworks by three parties",
                    "the reproduction that refuted *untouched* — the row's whole basis was \
                     that nothing had moved",
                ),
                (
                    "is absent from",
                    "the blocking message's own words, quoted — it asserts an absence the \
                     indexed blob disproves",
                ),
                (
                    "indexed blob",
                    "where the symbol actually is, which is what makes the message false \
                     rather than merely unhelpful",
                ),
            ],
        },
        ReDisposed {
            key: "7 item-slot repair verb",
            m46_key: "7 item-slot repair verb",
            m46_disposition: "RETIRED — the verb is not what is missing",
            datum: &[
                (
                    "`migrate --as`",
                    "the head of the shipped chain that already repairs a pre-guard corpus \
                     — the motive the row called *largely gone* was gone entirely",
                ),
                (
                    "only from a fresh clone",
                    "the bound that made the retirement a *route* question rather than a \
                     verb one",
                ),
            ],
        },
        ReDisposed {
            key: "9 store locking",
            m46_key: "9 store locking / lost updates",
            m46_disposition: "IN — admitted at the falsified predicate",
            datum: &[
                (
                    "`reconciliation.conflict-block`",
                    "the guard a lost baseline silently switches off — which is corruption, \
                     not the contention the row rested on",
                ),
                (
                    "5/5",
                    "the driven rate, quoted rather than described as *observed*",
                ),
                (
                    "exit 0",
                    "the exit code every losing process returned — the silence is the defect",
                ),
            ],
        },
        ReDisposed {
            key: "10 durable \"was provisioned\"",
            m46_key: "10 durable \"was provisioned\"",
            m46_disposition: "RETIRED — residue empty, driven both arms",
            datum: &[
                (
                    "`cp -R`",
                    "the first driven arm — it falls to the path-subject fix, so the row's \
                     *motive drained* is now measured rather than argued",
                ),
                (
                    "zero stored state",
                    "what the arm needed from a durable record: nothing",
                ),
                (
                    "exit 3",
                    "the fresh-clone arm's refusal, which the row's argument never reached",
                ),
            ],
        },
        ReDisposed {
            key: "11 dirty-worktree guard",
            m46_key: "11 dirty-worktree guard",
            m46_disposition: "RE-CUT, and the re-cut ships",
            datum: &[
                (
                    "for the remainder it narrates",
                    "the exclusion predicate itself, quoted — the deferral rested on it",
                ),
                (
                    "enumerated one path while destroying two",
                    "the finalize that falsified it, driven",
                ),
            ],
        },
    ];

    let body = read_ledger();
    let table = m48_table(&body);

    for item in RE_DISPOSED {
        let row = m48_row(&table, item.key);
        let Some(bracket) = dated_bracket(row) else {
            panic!(
                "M48 row `{}` carries no **dated** correction bracket. Its basis was \
                 falsified by driving the binary at the M46 razor, and the M48 table is a \
                 dated record: the disposition stays as written and the correction lands \
                 beside it in a `**[Corrected YYYY-MM-DD …**]**` bracket \
                 (`DECISIONS.md`'s own convention), never as a silent swap\nrow was:\n{row}",
                item.key,
            );
        };

        assert!(
            bracket.contains("falsif"),
            "M48 row `{}`'s bracket must say the basis was **falsified** — a basis-has-\
             changed rebuttal names what it rebuts, it does not merely add a \
             note\nbracket was:\n{bracket}",
            item.key,
        );

        for (needle, why) in item.datum {
            assert!(
                bracket.contains(needle),
                "M48 row `{}`'s bracket must quote `{needle}` — {why}\nbracket was:\n{bracket}",
                item.key,
            );
        }

        for (needle, why) in [
            (
                item.m46_key,
                "the M46 row that re-disposed this entry — a correction that does not name \
                 its home leaves the reader with two dispositions and no order between them",
            ),
            (
                item.m46_disposition,
                "the live disposition, quoted, so the table a planner scans lands them on \
                 the call that is still standing",
            ),
        ] {
            assert!(
                bracket.contains(needle),
                "M48 row `{}`'s bracket must name `{needle}` — {why}\nbracket was:\n{bracket}",
                item.key,
            );
        }
    }
}

/// **The M48 dispositions stay as written, and no neighbour moves.** Every row is pinned by
/// `| <key> | <disposition> |`: the five corrected rows keep the call they recorded on
/// 2026-08-13 (the correction is a bracket in the *basis* cell, never a rewritten verdict),
/// and the other seven are untouched.
#[test]
fn no_m48_rows_keying_or_disposition_moves() {
    const PINNED: &[(&str, &str)] = &[
        (
            "1 gate-command / evidence",
            "**stay deferred**, count unmoved",
        ),
        (
            "2 checkpoint / gate-record",
            "**stay deferred, re-counted at 7**",
        ),
        (
            "3 acknowledged-findings ledger",
            "**drain first** — disposition unchanged, **new counted datum**",
        ),
        (
            "4 managed-doc read-side",
            "**SPLIT** — own-work subclass **closed by F1**; **search subclass stays deferred**",
        ),
        ("5 Pest is-a-test tier", "**stay deferred**"),
        ("6 symbol-mention-sweep", "**stay deferred**"),
        ("7 item-slot repair verb", "**stay deferred**"),
        ("8 file-state introspection", "**stay deferred**"),
        ("9 store locking", "**stay deferred**"),
        (
            "10 durable \"was provisioned\"",
            "**stay deferred, motive drained**",
        ),
        ("11 dirty-worktree guard", "**PARTIALLY ABSORBED**"),
        ("12 `setup --format json` `hook_file`", "**ABSORBED**"),
    ];

    let body = read_ledger();
    let table = m48_table(&body);

    for (key, disposition) in PINNED {
        let cells = format!("> | {key} | {disposition} |");
        assert_eq!(
            table.matches(&cells).count(),
            1,
            "the M48 table must still key entry `{key}` at `{disposition}`, exactly once — \
             the M48 adjudication is a dated record of what was decided on 2026-08-13, and a \
             falsified basis is corrected beside it, never swapped out from under it",
        );
    }

    assert_eq!(
        table.lines().filter(|l| l.starts_with("> | ")).count(),
        12,
        "the M48 table must still hold exactly twelve rows — a correction records what \
         moved, it does not add or drop an entry",
    );
}

/// **The seven uncorrected rows stay bare.** A dated bracket marks a row whose basis was
/// driven and found false; putting one on a row nobody re-drove would make the mark
/// meaningless, which is how a convention stops carrying information.
#[test]
fn the_seven_unfalsified_m48_rows_carry_no_correction() {
    const UNTOUCHED: [&str; 7] = [
        "1 gate-command / evidence",
        "2 checkpoint / gate-record",
        "3 acknowledged-findings ledger",
        "4 managed-doc read-side",
        "6 symbol-mention-sweep",
        "8 file-state introspection",
        "12 `setup --format json` `hook_file`",
    ];

    let body = read_ledger();
    let table = m48_table(&body);

    for key in UNTOUCHED {
        let row = m48_row(&table, key);
        assert!(
            dated_bracket(row).is_none(),
            "M48 row `{key}` carries a dated correction bracket, but no driven datum \
             falsified it at the M46 razor — the mark says *this basis was tested and found \
             false*, so it has to stay rare enough to mean that\nrow was:\n{row}",
        );
    }
}
