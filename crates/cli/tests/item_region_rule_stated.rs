//! The item leaf boundary's rule is written where it lives, and the false universal
//! cannot come back (M49 Increment 1, T7).
//!
//! Increment 1 made the item's own leaf region **schema-keyed** — *the first deeper
//! heading that is neither a declared slot sub-label of this item's template nor an
//! anchored nested item* — at one implementation (`parse::is_item_slot_sub_label`) that
//! both seams ask. The code carries the rule; this suite fences the two places a reader
//! meets it *before* the code:
//!
//!   1. **`crates/engine/src/parse.rs` no longer asserts the universal it replaced.**
//!      Two comments claimed, of a fixed depth, that *"`####`+ is slot-internal
//!      structure"* — false at three constructs (a multi-slot sub-label, a nested item
//!      head, and a reserved-depth violation), and the sentence a later reader would have
//!      taken as the boundary rule. A third states it **parameterized on `reserved_max`**
//!      ([`is_reserved_depth`]) and is true as written, so the fence is *"the claim is
//!      made only where it names the bound it is relative to"*, never *"the phrase is
//!      banned"* — a rule that would have deleted the true statement along with the false
//!      ones.
//!
//!      Scoped to that exact phrase on purpose. The sibling phrase *"slot-internal
//!      content"* appears in test docs over **concrete fixtures** (a depth-1 multi-slot
//!      golden, where `#####` genuinely is opaque), where a fixed depth is a fact about
//!      that fixture rather than a universal — so widening the fence would redden true
//!      statements.
//!
//!   2. **`implementation/parsing.md` states the rule, names the sites that enforce it,
//!      and carries D4's declared bound verbatim.** The line citations are checked
//!      against the source rather than trusted: each must land on a line that calls the
//!      boundary, inside the function the doc names. A doc that cites a line number is
//!      making a checkable claim, and this repo has watched those rot (M48's own
//!      derivation called `rename.rs` untouched off a stale read).
//!
//! The *behaviour* the prose describes is proven elsewhere, through the real binary:
//! `crates/cli/tests/item_region_boundary.rs` and
//! `crates/cli/tests/item_region_shape_space.rs`.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{rel} must be readable: {e}"))
}

/// Contiguous runs of comment lines (`//` / `///`), each with the 1-based line number
/// it starts at — the unit a claim is made in.
fn comment_blocks(source: &str) -> Vec<(usize, String)> {
    let mut blocks: Vec<(usize, String)> = Vec::new();
    let mut current: Option<(usize, Vec<&str>)> = None;
    for (i, line) in source.lines().enumerate() {
        if line.trim_start().starts_with("//") {
            current.get_or_insert((i + 1, Vec::new())).1.push(line);
        } else if let Some((start, lines)) = current.take() {
            blocks.push((start, lines.join("\n")));
        }
    }
    if let Some((start, lines)) = current {
        blocks.push((start, lines.join("\n")));
    }
    blocks
}

/// Whitespace-collapsed text, so a claim carried verbatim across two files with
/// different line wrapping still compares equal.
fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The body of a `## <heading>` section of a markdown doc — up to the next `## ` or EOF.
fn section<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("\n## {heading}\n");
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("the doc must carry a `## {heading}` section"))
        + marker.len();
    let rest = &body[start..];
    match rest.find("\n## ") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// Claim 1: the *"is slot-internal structure"* universal is asserted only where it is
/// parameterized on the bound it is relative to (`reserved_max`).
#[test]
fn parse_rs_states_slot_internal_structure_only_where_parameterized() {
    let source = read("crates/engine/src/parse.rs");
    let offenders: Vec<usize> = comment_blocks(&source)
        .into_iter()
        .filter(|(_, block)| {
            block.contains("slot-internal structure") && !block.contains("reserved_max")
        })
        .map(|(line, _)| line)
        .collect();
    assert!(
        offenders.is_empty(),
        "a comment claims a depth is `slot-internal structure` without naming the bound \
         it is relative to (`reserved_max`) — the universal M49 Increment 1 replaced: \
         what a deeper heading *is* (declared slot sub-label / nested item head / \
         reserved-depth violation / slot prose) is decided against the schema, never by \
         depth alone. Comment blocks starting at lines: {offenders:?}"
    );
    // The true, parameterized statement must still be there — a fence that passes
    // because the whole subject was deleted has fenced nothing.
    assert!(
        source.contains("slot-internal structure"),
        "`is_reserved_depth`'s parameterized statement of the rule must survive"
    );
}

/// Claim 2a: `implementation/parsing.md` states the schema-keyed boundary rule.
#[test]
fn parsing_md_states_the_schema_keyed_boundary_rule() {
    let doc = read("implementation/parsing.md");
    let body = section(&doc, "The item's own leaf region");
    let flat_body = flat(body);
    for fragment in [
        "neither a declared slot sub-label of this item's template nor an anchored nested item",
        "`{#id}` anchor",
        "is_item_slot_sub_label",
    ] {
        assert!(
            flat_body.contains(&flat(fragment)),
            "the item-leaf-region section must state the rule — missing: {fragment:?}"
        );
    }
}

/// Claim 2b: every `path:line` the section cites resolves — the cited line calls the
/// boundary, inside the function the doc names beside it.
#[test]
fn parsing_md_line_citations_resolve_to_the_enforcement_sites() {
    let doc = read("implementation/parsing.md");
    let body = section(&doc, "The item's own leaf region");

    // Citations are written as ``  `write.rs:1356` (`unset_item_field`)  ``.
    let mut sites: Vec<(String, usize, String)> = Vec::new();
    for prefix in ["`write.rs:", "`parse.rs:"] {
        let file = prefix.trim_start_matches('`').trim_end_matches(':');
        let mut rest = body;
        while let Some(at) = rest.find(prefix) {
            rest = &rest[at + prefix.len()..];
            let close = rest.find('`').expect("a citation closes its backtick");
            let line: usize = rest[..close]
                .parse()
                .unwrap_or_else(|_| panic!("`{file}:<line>` must cite a line number"));
            let after = &rest[close + 1..];
            let open = after.find("(`").unwrap_or_else(|| {
                panic!("`{file}:{line}` must name the function it sits in: (`<fn>`)")
            });
            let name_rest = &after[open + 2..];
            let name_end = name_rest.find('`').expect("the function name closes");
            sites.push((file.to_string(), line, name_rest[..name_end].to_string()));
        }
    }
    // The four sites the boundary is *keyed on*. A further caller may cite itself here
    // (the validate-after guard consumes the same region rather than deciding it), so the
    // fence is a superset check plus per-citation resolution — never an exact count that
    // would forbid naming one.
    for keyed in [
        "unset_item_field",
        "set_item_field",
        "insert_item_field",
        "parse_items",
    ] {
        assert!(
            sites.iter().any(|(_, _, name)| name == keyed),
            "the section must name the site `{keyed}`, got: {sites:?}"
        );
    }

    for (file, line, func) in sites {
        let source = read(&format!("crates/engine/src/{file}"));
        let lines: Vec<&str> = source.lines().collect();
        let text = lines
            .get(line - 1)
            .unwrap_or_else(|| panic!("{file} has no line {line}"));
        assert!(
            text.contains("item_own_leaf_region(") || text.contains("first_nested_heading("),
            "{file}:{line} is cited as an enforcement site but does not ask the \
             boundary — the citation has rotted: {text:?}"
        );
        let enclosing = lines[..line - 1]
            .iter()
            .rev()
            .find_map(|l| {
                let l = l.trim_start();
                for kw in ["pub(crate) fn ", "pub fn ", "fn "] {
                    if let Some(rest) = l.strip_prefix(kw) {
                        return Some(rest.split('(').next().unwrap_or_default().to_string());
                    }
                }
                None
            })
            .unwrap_or_else(|| panic!("{file}:{line} sits in no function"));
        assert_eq!(
            enclosing, func,
            "{file}:{line} is cited as `{func}` but sits in `{enclosing}`"
        );
    }
}

/// Claim 2c: D4's declared bound is carried **verbatim** — read out of the locked
/// artifact and compared, so it cannot drift into a softer paraphrase.
#[test]
fn parsing_md_carries_the_declared_bound_verbatim() {
    let settle = read("completions/artifacts/M49/settle-record.md");
    let marker = "**Declared bound, carried:**";
    let start = settle
        .find(marker)
        .expect("the M49 settle record states D4's declared bound");
    let bound = &settle[start..];
    let end = bound
        .find("\n\n")
        .expect("the declared bound is one paragraph");
    let bound = flat(&bound[..end]);

    let doc = flat(&read("implementation/parsing.md"));
    assert!(
        doc.contains(&bound),
        "implementation/parsing.md must carry D4's declared bound verbatim \
         (`completions/artifacts/M49/settle-record.md` → D4), got neither it nor a \
         wrapping variant of it. The bound: {bound:?}"
    );
}
