//! The planning gate set has **one home**, and the home is fenced (M49 Increment 9 /
//! T7; `completions/artifacts/M49/settle-record.md` → D9 — *"the gate set is 14,
//! settled in `design/methodology-docs.md` as the single home"*).
//!
//! [`design/methodology-docs.md`](../../../design/methodology-docs.md) → *The
//! planning gate-record* carries one table row per gate, and each row's *What it
//! requires recorded* cell is what the shipped `planning-record` schema renders as
//! that gate's `hint:`. The doc's own accretion rule
//! ([`methodology-docs.md`](../../../design/methodology-docs.md) → *Why this caps the
//! prose accretion*) says a new plan-time lesson becomes **a new gate-row**, and the
//! workflow docs keep only the *worked instances* as the why — they never restate the
//! checklist.
//!
//! That rule had already been broken when this fence was written:
//! [`implementation/milestone-planning-workflow.md`](../../../implementation/milestone-planning-workflow.md)
//! → *The plan-time gate-record* listed **thirteen** of the fourteen ids inline,
//! silently omitting `quote-attributed` — a second copy of the checklist, already
//! stale, under a paragraph titled *Accretion discipline*. A prose file that no test
//! reads drifts, so this suite is that rule's fence, on the
//! [`doctype_map_versions`](doctype_map_versions) precedent.
//!
//! **The subject is derived, never listed.** Both arms read the gate ids out of the
//! **shipped `planning-record` schema** — loaded from the real embedded methodology
//! pack through the production [`load_pack_schema`] path — so a gate added, renamed
//! or retired in the schema reddens here until the single home follows. Nothing in
//! this file hand-lists a gate id. (Its sibling `planning_record_schema.rs` fences
//! the other direction — the schema against the table, hints included — so the two
//! together hold the doc and the doctype in lockstep.)
//!
//! **Why a *majority*, not a bare count.** The predicate is calibrated on the two
//! real data points rather than a round number: thirteen ids in one paragraph is a
//! restatement and must be red; the *Settle* paragraph naming five gates as worked
//! instances is exactly what the single-home rule wants **kept** and must be green.
//! So the floor is a majority of the shipped gate set — restating most of the
//! checklist is the failure; citing a handful of gates where their worked instance
//! lives is the point.
//!
//! **Scope: `design/` + `implementation/` + `.claude/`.** The third tree is where the
//! runnable overlays live, and `.claude/commands/milestone-plan.md` is the file whose
//! job is to *force* the gates — the likeliest place for a second copy to grow, and it
//! did: an eleven-of-fourteen restatement, three gates short, standing from 2026-06-25
//! until this scan reached it. Two trees stay deliberately outside. `DECISIONS.md` is
//! history — a dated entry records what a wave decided. `completions/` holds the
//! **filled instances** of the gate-record: a milestone's own `planning-gate-record.md`
//! legitimately answers all fourteen rows, because that is the record itself, not a
//! restatement of the checklist. The rule is against a *second copy of the checklist*,
//! never against history or against the artifacts the checklist produces.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use cli::pack::{EmbeddedPack, load_pack_schema};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

/// The doctype whose body sections *are* the gate set.
const TYPE: &str = "planning-record";

/// The settled single home of the gate set (D9).
const GATE_TABLE_DOC: &str = "design/methodology-docs.md";

/// The heading whose table carries one row per planning gate.
const GATE_TABLE_HEADING: &str = "## The planning gate-record";

/// The gate table's first column header (the gate id).
const GATE_COLUMN: &str = "Gate";

/// The trees the single-home rule governs: the design + implementation prose, and the
/// `.claude/` runnable overlays that drive the planning loop. `DECISIONS.md` is history
/// and `completions/` holds the *filled* gate-records — both deliberately outside (see
/// the module doc's *Scope*).
const GOVERNED_TREES: [&str; 3] = ["design", "implementation", ".claude"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

/// The gate ids, in schema order, read from the **shipped** `planning-record` — the
/// real embedded methodology pack, loaded the production way. Header sections are not
/// gates (the schema-version stamp injects its own front-matter `meta`), so the gate
/// set is the body sections.
fn shipped_gate_ids() -> Vec<String> {
    let pack = EmbeddedPack::methodology();
    let bytes = pack
        .read(
            PackResourceKind::Schemas,
            &ResourceId::from(TYPE.to_string()),
        )
        .unwrap_or_else(|e| panic!("the shipped `{TYPE}` schema reads back: {e}"));
    let schema = load_pack_schema(&pack, &bytes)
        .unwrap_or_else(|e| panic!("the shipped `{TYPE}` schema loads: {e:?}"));
    let gates: Vec<String> = schema
        .sections
        .iter()
        .filter(|s| !s.header)
        .map(|s| s.id.clone())
        .collect();
    assert!(
        !gates.is_empty(),
        "`{TYPE}` ships no body section — the gate set cannot be derived"
    );
    gates
}

/// Split a markdown table row into its trimmed cells.
fn cells(row: &str) -> Vec<&str> {
    row.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

/// The gate table under [`GATE_TABLE_HEADING`], as `(row ids in table order, the
/// table's own lines)`. The lines come back so the paragraph sweep can exempt exactly
/// this block — the one place the set is allowed to appear in full.
fn gate_table() -> (Vec<String>, Vec<String>) {
    let text = read(GATE_TABLE_DOC);
    let mut lines = text
        .lines()
        .skip_while(|l| !l.starts_with(GATE_TABLE_HEADING));
    assert!(
        lines.next().is_some(),
        "{GATE_TABLE_DOC} has no `{GATE_TABLE_HEADING}` section — the gate set's home moved"
    );
    let table: Vec<String> = lines
        .map(str::trim)
        .skip_while(|l| !l.starts_with('|'))
        .take_while(|l| l.starts_with('|'))
        .map(str::to_string)
        .collect();
    assert!(
        table.len() > 2,
        "{GATE_TABLE_DOC}: `{GATE_TABLE_HEADING}` carries no gate rows"
    );

    let header = cells(&table[0]);
    let gate_at = header
        .iter()
        .position(|c| *c == GATE_COLUMN)
        .unwrap_or_else(|| {
            panic!("{GATE_TABLE_DOC}: the gate table has no `{GATE_COLUMN}` column")
        });
    let ids = table[2..]
        .iter()
        .map(|row| cells(row)[gate_at].trim_matches('`').to_string())
        .collect();
    (ids, table)
}

/// Every governed markdown file, repo-relative, depth-first and sorted.
fn governed_docs() -> Vec<String> {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) {
        let mut entries: Vec<PathBuf> = fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", dir.display()))
            .map(|e| e.expect("a readable dir entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(&path, root, out);
            } else if path.extension().is_some_and(|e| e == "md") {
                out.push(
                    path.strip_prefix(root)
                        .expect("a path under the repo root")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let root = repo_root();
    let mut out = Vec::new();
    for tree in GOVERNED_TREES {
        walk(&root.join(tree), &root, &mut out);
    }
    assert!(!out.is_empty(), "the governed trees hold markdown");
    out
}

/// Does `text` name `gate` as a whole token? The ids are hyphenated words, so the
/// boundary must reject a longer identifier while accepting the id inside backticks,
/// italics, a table cell or plain prose — every form the docs actually use.
fn names_gate(text: &str, gate: &str) -> bool {
    let boundary = |c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_');
    let mut rest = text;
    while let Some(at) = rest.find(gate) {
        let before_ok = rest[..at].chars().next_back().is_none_or(boundary);
        let after_ok = rest[at + gate.len()..].chars().next().is_none_or(boundary);
        if before_ok && after_ok {
            return true;
        }
        rest = &rest[at + gate.len()..];
    }
    false
}

/// The blank-line-separated blocks of a markdown file, as `(1-based start line, text)`.
fn paragraphs(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    let mut start = 0usize;
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            if !block.is_empty() {
                out.push((start + 1, block.join("\n")));
                block.clear();
            }
        } else {
            if block.is_empty() {
                start = i;
            }
            block.push(line);
        }
    }
    if !block.is_empty() {
        out.push((start + 1, block.join("\n")));
    }
    out
}

/// Arm 1 — the home holds exactly the shipped gate set. Derived from the schema, so a
/// gate added or retired in `planning-record` reddens until the single home follows.
#[test]
fn the_single_home_lists_exactly_the_shipped_gates() {
    let shipped = shipped_gate_ids();
    let (rows, _) = gate_table();

    assert_eq!(
        rows, shipped,
        "{GATE_TABLE_DOC} → `{GATE_TABLE_HEADING}` must carry one row per shipped `{TYPE}` gate, \
         in schema order — it is the settled single home of the gate set (D9)"
    );
    assert_eq!(
        rows.iter().collect::<BTreeSet<_>>().len(),
        rows.len(),
        "the gate table repeats a row: {rows:?}"
    );
}

/// Arm 2 — nothing outside that table restates the set. The floor is a **majority** of
/// the shipped gates: thirteen-in-one-paragraph is a second copy of the checklist and
/// is red; five named as worked instances is the *why* the single home points at and
/// is green.
#[test]
fn no_paragraph_outside_the_home_restates_the_gate_set() {
    let shipped = shipped_gate_ids();
    let floor = shipped.len() / 2 + 1;
    let (_, table_lines) = gate_table();

    let mut restatements: Vec<String> = Vec::new();
    for doc in governed_docs() {
        let text = read(&doc);
        for (line, para) in paragraphs(&text) {
            // The single home itself — the one block allowed to carry the whole set.
            if doc == GATE_TABLE_DOC
                && para
                    .lines()
                    .map(str::trim)
                    .eq(table_lines.iter().map(|l| l.as_str()))
            {
                continue;
            }
            let named: Vec<&str> = shipped
                .iter()
                .filter(|g| names_gate(&para, g))
                .map(String::as_str)
                .collect();
            if named.len() >= floor {
                restatements.push(format!(
                    "{doc}:{line} names {} of the {} gates {named:?}",
                    named.len(),
                    shipped.len()
                ));
            }
        }
    }

    assert!(
        restatements.is_empty(),
        "the gate set has one home — {GATE_TABLE_DOC} → `{GATE_TABLE_HEADING}`. A paragraph \
         naming {floor} or more of the {} gates is a second copy of the checklist, and a second \
         copy goes stale (this fence was written because one already had, omitting a gate). Cite \
         the gates a passage works through and link the home for the rest:\n  {}",
        shipped.len(),
        restatements.join("\n  ")
    );
}
