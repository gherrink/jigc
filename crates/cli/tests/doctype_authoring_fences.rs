//! The registration checklist names **every pack-load fence** a new doctype has to
//! clear (M49 Increment 11, T9 — E1).
//!
//! [`implementation/doctype-authoring.md`](../../../implementation/doctype-authoring.md)
//! opens *"a doctype's obligations were never listed in one place. This is that
//! list."* At this suite's discovery it named **none** of the fences that can refuse
//! a pack load — zero hits for `stated-at`, `staged-read-back`, `states-constraints`,
//! `suppressed` or `{{schema:` — so an author following it to the letter could ship a
//! doctype whose author step blocks **every door of the binary** at load, with the
//! checklist's own opening claim standing.
//!
//! **The subject is derived from the code, never listed here.** A grep for `assert_`
//! is not a fence ([`dev-workflow.md`](../../../implementation/dev-workflow.md) — *a
//! grep is not a fence*): it keys on a naming habit, so a fence named `check_…` or
//! `verify_…` joins the pack-load path invisibly and the checklist's claim quietly
//! becomes false again — which is the very defect this row exists to fix. The
//! derivation keys on **what the members are**:
//!
//! > a **pack-load fence** is a top-level function of `crates/cli/src/pack.rs` whose
//! > return type is `anyhow::Result<()>` — a function whose only outcomes are *pass*
//! > and *refuse*, carrying no value a caller could use — that is **reachable from
//! > [`make_pack`]**, the pack-source factory every door goes through.
//!
//! Both legs are load-bearing. The return shape alone would sweep in helpers that
//! merely happen to be fallible; reachability alone would sweep in every reader on
//! the factory path. Together they are exactly *the things that can refuse a pack
//! load*, whatever they are called.
//!
//! Every derived member must be **disposed** by a row: what it demands and how to
//! satisfy it (`**Satisfy:**`), or out with a reason (`**Out:**`) — and every row
//! states the condition under which the fence fires (`**Fires:**`), because most of
//! them are conditional and a checklist of eight unconditional obligations would be
//! its own law-1 lie (a machine-maintained doctype with no author step clears three
//! of them by construction).
//!
//! Set equality, both directions: a fence with no row reddens, and a row naming a
//! fence the code no longer carries reddens too.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The pack-source factory — the reachability root. Every door in the binary
/// assembles its pack through it, so a refusal reachable from here is a refusal at
/// every door.
const FACTORY: &str = "make_pack";

/// The module the fence family lives in.
const PACK_SRC: &str = "crates/cli/src/pack.rs";

/// The checklist under test.
const CHECKLIST: &str = "implementation/doctype-authoring.md";

/// The heading whose bullets are the fence rows.
const FENCE_HEADING: &str = "## Pack-load fences — what refuses a load";

/// The return type that marks a pass-or-refuse function.
const REFUSAL_RETURN: &str = "-> anyhow::Result<()>";

/// This repo — the checklist and the source are both checked in, so the derivation
/// runs against the checkout rather than a fabricated tree.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

/// One top-level `fn` item of a source file: its name, its signature (everything up
/// to the body's opening brace) and its body text.
struct Item {
    name: String,
    signature: String,
    body: String,
}

/// Every **top-level** `fn` item of a rustfmt-formatted source file.
///
/// Both boundaries are rustfmt-guaranteed and the gate runs `cargo fmt --check`, so
/// they hold by construction rather than by luck: a top-level item starts at column
/// 0, and its closing brace is the next line that is exactly `}`. Items nested in a
/// `mod`, an `impl` or a `#[cfg(test)]` block are indented and therefore excluded —
/// which is what we want: a test helper is not a production fence.
fn top_level_fns(src: &str) -> Vec<Item> {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(rest) = line
            .strip_prefix("fn ")
            .or_else(|| line.strip_prefix("pub fn "))
            .or_else(|| line.strip_prefix("pub(crate) fn "))
        else {
            continue;
        };
        let name: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if name.is_empty() {
            continue;
        }
        let end = (i..lines.len())
            .find(|j| lines[*j] == "}")
            .unwrap_or_else(|| panic!("the top-level `fn {name}` closes at column 0"));
        let item = lines[i..=end].join("\n");
        let brace = item.find('{').unwrap_or(item.len());
        out.push(Item {
            name,
            signature: item[..brace]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
            body: item[brace.min(item.len())..].to_owned(),
        });
    }
    out
}

/// The pack-load fence set, derived: pass-or-refuse functions reachable from the
/// factory. Returns each fence's name mapped to its body, so an arm can look at what
/// a member actually does rather than at what it is called.
fn derived_fences() -> BTreeMap<String, String> {
    let items = top_level_fns(&read(PACK_SRC));
    let by_name: BTreeMap<&str, &Item> = items.iter().map(|it| (it.name.as_str(), it)).collect();
    let factory = by_name.get(FACTORY).unwrap_or_else(|| {
        panic!("{PACK_SRC} defines the pack-source factory `{FACTORY}` at the top level")
    });

    // Pass-or-refuse candidates: nothing to hand back, so the whole point of the call
    // is whether it returns at all.
    let candidates: BTreeSet<&str> = items
        .iter()
        .filter(|it| it.signature.contains(REFUSAL_RETURN))
        .map(|it| it.name.as_str())
        .collect();

    // Reachability from the factory, transitively through the candidates themselves
    // (`assert_when_shape` is reached only via the front-matter sweep).
    let mut reached = BTreeMap::new();
    let mut frontier = vec![factory.body.as_str()];
    while let Some(body) = frontier.pop() {
        for name in &candidates {
            if reached.contains_key(*name) || !body.contains(&format!("{name}(")) {
                continue;
            }
            let item = by_name[*name];
            reached.insert((*name).to_owned(), item.body.clone());
            frontier.push(item.body.as_str());
        }
    }
    assert!(
        !reached.is_empty(),
        "the derivation found no pack-load fence at all in {PACK_SRC} — the extractor is broken, \
         and a fence set that cannot be non-empty cannot redden either"
    );
    reached
}

/// The checklist's fence section, as its bullet rows.
fn fence_rows() -> Vec<String> {
    let doc = read(CHECKLIST);
    let (_, after) = doc.split_once(FENCE_HEADING).unwrap_or_else(|| {
        panic!(
            "{CHECKLIST} carries no `{FENCE_HEADING}` section — the pack-load fences are the \
             obligations most able to block every door of the binary, and the file opens by \
             claiming to be the list of a doctype's obligations"
        )
    });
    let section = after.split("\n## ").next().expect("a first section body");
    section
        .lines()
        .filter(|line| line.starts_with("- [ ] "))
        .map(str::to_owned)
        .collect()
}

/// Every fence identifier a row names, keyed to its row. A row names exactly one.
fn rows_by_fence(fences: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for row in fence_rows() {
        let named: Vec<&String> = fences
            .keys()
            .filter(|name| row.contains(&format!("`{name}`")))
            .collect();
        assert!(
            named.len() <= 1,
            "one row of {CHECKLIST} names {} fences ({named:?}) — a row disposes one fence, \
             so the checklist cannot say which obligation it discharged:\n{row}",
            named.len(),
        );
        if let Some(name) = named.first() {
            let prior = out.insert((*name).clone(), row.clone());
            assert!(
                prior.is_none(),
                "{CHECKLIST} carries two rows for `{name}` — the reader is given two answers"
            );
        }
    }
    out
}

/// Every pack-load fence is disposed by a row, and no row disposes a fence the code
/// no longer carries.
#[test]
fn every_pack_load_fence_carries_a_checklist_row() {
    let fences = derived_fences();
    let rows = rows_by_fence(&fences);

    let derived: BTreeSet<&String> = fences.keys().collect();
    let disposed: BTreeSet<&String> = rows.keys().collect();

    let undisposed: Vec<&&String> = derived.difference(&disposed).collect();
    assert!(
        undisposed.is_empty(),
        "{CHECKLIST} disposes no row for the pack-load fence(s) {undisposed:?} — each can refuse \
         a pack load at EVERY door, so a doctype whose author step trips one ships a binary that \
         will not start, while the file's own opening claim (\"This is that list\") stands. Add a \
         row naming what the fence demands and how to satisfy it, or say why it is out for a \
         doctype author.\nderived from {PACK_SRC}: {derived:?}"
    );

    let stale: Vec<&&String> = disposed.difference(&derived).collect();
    assert!(
        stale.is_empty(),
        "{CHECKLIST} carries a row for {stale:?}, which {PACK_SRC} no longer reaches from \
         `{FACTORY}` as a pass-or-refuse function — the row instructs an author to satisfy a \
         fence that is gone"
    );

    // The checklist's rows also carry the section's own bullets that name no fence
    // (the section intro's caveats); the count below is only the disposal set.
    assert_eq!(
        derived.len(),
        rows.len(),
        "the derived fence set and the disposal set must match one-for-one"
    );
}

/// A row says **when** the fence fires and **how** to satisfy it — or why it is out.
///
/// Most of the family is conditional (a singleton's author template, a step that
/// solicits a write, a workflow off the router catalog), so a row that states an
/// obligation without its condition would tell a machine-maintained doctype's author
/// to satisfy three fences that cannot fire for them — a lie in the shape this wave
/// exists to close.
#[test]
fn every_fence_row_states_its_condition_and_its_discharge() {
    let fences = derived_fences();
    for (name, row) in rows_by_fence(&fences) {
        assert!(
            row.contains("**Fires:**"),
            "the `{name}` row states no `**Fires:**` condition — most of this family is \
             conditional, and an unconditional-looking row sends an author to satisfy a fence \
             that cannot fire for their doctype:\n{row}"
        );
        assert!(
            row.contains("**Satisfy:**") || row.contains("**Out:**"),
            "the `{name}` row neither says how to satisfy the fence (`**Satisfy:**`) nor \
             disposes it with a reason (`**Out:**`) — naming a fence without a discharge leaves \
             the author exactly where the missing section did:\n{row}"
        );
    }
}
