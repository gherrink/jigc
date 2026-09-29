//! M49 Increment 11 / T11 — the `finalize.*` family stops being a hand count.
//!
//! Four homes carried four different numerals for one family, and **every one of them was
//! wrong**: `design/command-output-contract.md` said *nine* three times and *eleven plus one*
//! once, and `implementation/roadmap.md` carried that on as *twelve*. The paragraph that
//! published *eleven* had **already written down the reason** — *"a census keyed on where you
//! expect the members to live will miss the ones that live somewhere else"* — and then
//! re-committed the error by publishing a corrected count from a second census.
//!
//! So the repair is not a fifth number. It is `design/validation.md` → *Exit semantics*'
//! settled rule applied where it never reached — **the table is the enumeration, and its size
//! is stated nowhere** — plus the thing the settled rule presupposes and this family never had:
//! **a stated membership predicate**.
//!
//! > A code is a member of the `finalize.*` family **iff a production (non-`#[cfg(test)]`)
//! > constructor mints it as a `Finding` whose code lies in the `finalize.` namespace.**
//!
//! That predicate is what the two published numbers disagreed about, silently. Under it,
//! `finalize.fan-out` is not a member (a cascade knob key),
//! `finalize.left-out` is not one (a declared contract identifier the M42 settle deliberately
//! left unminted) — and two things no count had right in either direction:
//!
//!   * **`finalize.milestone-sub-task` is a member and sat in no row and inside no count** — it
//!     lives in `crates/engine/src/milestone.rs`, the third file a census scoped to
//!     `finalize.rs` (then to `finalize.rs` + `task.rs`) did not think to open;
//!   * **`finalize.forward-ref-dangling` is not a member and had a row** — no production
//!     producer mints it at all; it survives only as a unit-test fixture in `finding.rs`, so it
//!     was a member of the count and of no code path.
//!
//! The arms:
//!
//!   1. [`the_registry_equals_the_production_producer_set`] — the enumeration is **derived**:
//!      production source is scanned for `Finding` constructors and the `(code, module)` pairs
//!      it mints in the namespace must equal `cli::render::FINALIZE_FAMILY` exactly. A producer
//!      added anywhere — a fourth file, a fourth crate — reddens here.
//!   2. [`no_declared_non_member_has_a_production_producer`] — the other direction: a
//!      `FINALIZE_NON_MEMBERS` entry that ever gains a producer must move.
//!   3. [`the_contract_sub_table_renders_the_registry`] — the design doc's sub-table is checked
//!      against the registry row for row, including each row's declared `target` form.
//!   4. [`no_live_doc_states_a_numeral_for_the_family`] — the count fence, on
//!      `exit_flip_count_record.rs`' `assert_uncounted` idiom.
//!   5. [`the_falsified_counts_are_gone_and_each_replacement_lands_once`] — the record fence, on
//!      `record_foreign_arm.rs`' pattern: every falsified byte named with the string it carried.
//!   6. [`every_finalize_identifier_in_a_live_doc_is_declared_one_way_or_the_other`] — no doc may
//!      name a `finalize.*` identifier that is in neither list, which is what makes a phantom
//!      member like `forward-ref-dangling` impossible to re-add quietly.
//!   7. [`the_ambush_class_finalize_codes_are_declared_one_way_or_the_other`] — green today and
//!      pinned: `crate::pack`'s constraint-token map hand-asserts *"verified against the real
//!      producers"* about four of these identifiers; that claim now rests on the derived set.
//!
//! **Declared bounds.** (a) The source scan is a lexer, not a compiler: it drops whole-line
//! comments and top-level `#[cfg(test)]` items (every one in this tree is at column 0), and it
//! reads `code` **string literals** — a member built from a non-literal code would be invisible
//! to it, and a non-vacuity floor over the constructors examined is what keeps a silent lexer
//! regression from reading as an all-clear. **M52 Increment 1 / T1 met that bound rather than
//! ignoring it**: `finalize.commit-rejected` moved out of `FINALIZE_NON_MEMBERS` when a reject
//! that carries a finding took the findings arm, and its producer is one seam
//! (`cli::render::commit_rejection_finding`) minting whichever code its caller's
//! `COMMITTING_DOORS` row names — invisible to a literal scan. Arm 1 therefore completes the
//! derived set from **that registry**, not from an exception list: a door added there whose
//! identity is in this namespace grows the expected set and reddens the table. (b) The count fence covers the phrases the family is
//! **named** by; a retired numeral quoted inside a sentence that says it was wrong (the design
//! doc's note under the sub-table is exactly that) is deliberately not fenced — the record of the
//! error is the reason the rule is credible. (c) `DECISIONS.md` and `completions/` are outside
//! the live-doc set by declaration: they are dated records of what was believed on their date,
//! and rewriting them would falsify the log rather than repair a claim.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::support::root_walk;
use cli::render::{FINALIZE_FAMILY, FINALIZE_NON_MEMBERS};

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

/// The namespace the predicate is stated over.
const NAMESPACE: &str = "finalize.";

// ---------------------------------------------------------------------------
// The production-source scan
// ---------------------------------------------------------------------------

/// Every `.rs` file under a crate's `src/`, recursively, sorted for a stable diagnostic order.
fn crate_sources(crate_dir: &str) -> Vec<PathBuf> {
    root_walk::files(
        &repo_root().join(crate_dir).join("src"),
        root_walk::ext("rs"),
    )
}

/// The **production** view of a Rust source file: whole-line comments dropped, and every
/// top-level `#[cfg(test)]` item (the line is exactly that attribute at column 0) dropped
/// through its closing `}` at column 0.
///
/// Sound for this tree by inspection — every `#[cfg(test)]` **module** here is a column-0 item,
/// and the one indented occurrence is a `fn` attribute inside an already-production `impl` that
/// constructs no finding. Stated as the bound it is rather than presented as a parser.
fn production_view(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut skipping = false;
    for line in text.lines() {
        if skipping {
            if line == "}" {
                skipping = false;
            }
            continue;
        }
        if line == "#[cfg(test)]" {
            skipping = true;
            continue;
        }
        if line.trim_start().starts_with("//") {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Whether `token` has the shape of a finding code — `<probe>.<check>`, kebab, no whitespace.
fn is_code_shaped(token: &str) -> bool {
    let Some((probe, check)) = token.split_once('.') else {
        return false;
    };
    let ok = |s: &str| {
        !s.is_empty()
            && s.starts_with(|c: char| c.is_ascii_lowercase())
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    };
    ok(probe) && ok(check)
}

/// The string literals in `window`, in order, un-escaped only insofar as a finding code needs
/// (a code carries no escapes).
fn literals(window: &str) -> Vec<String> {
    let bytes = window.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'"' {
                if bytes[j] == b'\\' {
                    j += 1;
                }
                j += 1;
            }
            if j >= bytes.len() {
                break;
            }
            out.push(window[start..j].to_string());
            i = j + 1;
            continue;
        }
        i += 1;
    }
    out
}

/// The `<crate>::<module>` a source path denotes.
fn producer_module(path: &Path) -> String {
    let text = path.to_string_lossy();
    let (crate_name, rest) = if let Some(rest) = text.split("crates/engine/src/").nth(1) {
        ("engine", rest)
    } else if let Some(rest) = text.split("crates/cli/src/").nth(1) {
        ("cli", rest)
    } else {
        panic!("unexpected source path {path:?}")
    };
    let module = rest.trim_end_matches(".rs").replace('/', "::");
    format!("{crate_name}::{module}")
}

/// The three `Finding` constructors: `graded` takes the code second (after a `Severity`),
/// `block` and `blocking` take it first. Either way the code is the **first code-shaped string
/// literal** inside the call, which is what the scan keys on.
const CONSTRUCTORS: [&str; 3] = ["Finding::graded(", "Finding::block(", "Finding::blocking("];

/// Scan production source for the `(code, module)` pairs minted in the `finalize.` namespace,
/// returning them alongside the total number of constructor calls examined (the non-vacuity
/// floor's input).
fn scan_producers() -> (BTreeSet<(String, String)>, usize) {
    let mut found = BTreeSet::new();
    let mut examined = 0usize;
    for path in crate_sources("crates/engine")
        .into_iter()
        .chain(crate_sources("crates/cli"))
    {
        let module = producer_module(&path);
        let text = production_view(&fs::read_to_string(&path).expect("read source"));
        for ctor in CONSTRUCTORS {
            let mut from = 0usize;
            while let Some(rel) = text[from..].find(ctor) {
                let at = from + rel + ctor.len();
                examined += 1;
                let end = text.len().min(at + 400);
                if let Some(code) = literals(&text[at..end])
                    .into_iter()
                    .find(|l| is_code_shaped(l))
                    && code.starts_with(NAMESPACE)
                {
                    found.insert((code, module.clone()));
                }
                from = at;
            }
        }
    }
    (found, examined)
}

/// **Arm 1 — the enumeration is derived, not declared.**
#[test]
fn the_registry_equals_the_production_producer_set() {
    let (mut scanned, examined) = scan_producers();

    // **The second producer *shape*, and why it is derived here rather than scanned (M52
    // Increment 1 / T1).** Every member above is minted by a constructor carrying its code as
    // a **string literal**, which is what the lexer reads. The committing doors' rejections
    // are not: one seam (`cli::render::commit_rejection_finding`) mints them and the code
    // arrives as a parameter, from the door's own
    // [`COMMITTING_DOORS`](cli::invocation_log::COMMITTING_DOORS) row. That is exactly the
    // declared bound (b) in this module's header — a member built from a non-literal code is
    // invisible to the scan — so the set is completed from the **registry that decides
    // membership** instead of from a hand-written exception: every `COMMITTING_DOORS` member
    // whose identity lies in this namespace is minted as a `Finding` by that one seam, so an
    // eleventh door added there grows this set and reddens the registry, which is the property
    // the scan buys for the literal shape.
    for door in cli::invocation_log::COMMITTING_DOORS {
        if door.error_code.starts_with(NAMESPACE) {
            scanned.insert((door.error_code.to_string(), "cli::render".to_string()));
        }
    }

    // **The same shape, one family over (M52 Increment 5 / T2).** A raced rollback's conflict
    // is minted by one generic seam (`cli::rollback`'s `rollback_conflict_finding`) whose code
    // arrives as a parameter, from the door's own
    // [`ROLLBACK_DOORS`](cli::rollback::ROLLBACK_DOORS) row — again invisible to a literal
    // scan, and again completed from the registry that decides membership rather than from a
    // hand-written exception. A door added there whose identity lies in this namespace grows
    // this set and reddens the registry.
    for door in cli::rollback::ROLLBACK_DOORS {
        if door.code.starts_with(NAMESPACE) {
            scanned.insert((door.code.to_string(), "cli::rollback".to_string()));
        }
    }

    // A silent lexer regression would empty `scanned` and read as agreement with an empty
    // registry; the floor makes that impossible to mistake for a clean pass.
    assert!(
        examined >= 100,
        "the scan examined only {examined} `Finding` constructor calls across both crates — \
         the source view likely regressed, so nothing below is trustworthy",
    );

    let declared: BTreeSet<(String, String)> = FINALIZE_FAMILY
        .iter()
        .map(|m| (m.code.to_string(), m.producer.to_string()))
        .collect();

    assert_eq!(
        scanned, declared,
        "`cli::render::FINALIZE_FAMILY` must equal the production producer set under the stated \
         predicate — a code is a member iff a production constructor mints it as a `Finding` in \
         the `{NAMESPACE}` namespace. Scanned: {scanned:?}",
    );

    // One order, so a diff over the table reads as a membership change.
    let codes: Vec<&str> = FINALIZE_FAMILY.iter().map(|m| m.code).collect();
    let mut sorted = codes.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(
        codes, sorted,
        "the registry must be sorted by `code` and carry no duplicate",
    );
}

/// **Arm 2 — the non-member list is the other half of the predicate.**
///
/// Without it the predicate reads as a *namespace*, and a namespace is exactly what let a
/// cascade knob key, an invocation-log error identity and a declared-but-unminted contract
/// identifier be counted as codes (M49 Increment 8's lesson: an exemption is an enumeration,
/// never a namespace prefix).
#[test]
fn no_declared_non_member_has_a_production_producer() {
    let (scanned, _) = scan_producers();
    let minted: BTreeSet<&str> = scanned.iter().map(|(code, _)| code.as_str()).collect();

    assert!(
        !FINALIZE_NON_MEMBERS.is_empty(),
        "the non-member list must not be empty — it is what keeps the predicate from being read \
         as a namespace",
    );
    for (identifier, why) in FINALIZE_NON_MEMBERS {
        assert!(
            identifier.starts_with(NAMESPACE),
            "`{identifier}` is not in the `{NAMESPACE}` namespace, so it needs no entry here",
        );
        assert!(
            !minted.contains(identifier),
            "`{identifier}` is declared a non-member ({why}) but a production constructor now \
             mints it as a `Finding` — move it into `cli::render::FINALIZE_FAMILY`",
        );
        assert!(
            !why.trim().is_empty(),
            "`{identifier}` must carry the reason it is not a member",
        );
    }

    let members: BTreeSet<&str> = FINALIZE_FAMILY.iter().map(|m| m.code).collect();
    for (identifier, _) in FINALIZE_NON_MEMBERS {
        assert!(
            !members.contains(identifier),
            "`{identifier}` is in both lists — it is one or the other",
        );
    }
}

// ---------------------------------------------------------------------------
// The design doc's sub-table
// ---------------------------------------------------------------------------

const CONTRACT: &str = "design/command-output-contract.md";

/// The sub-table's rows as `(code, target cell)`, read off the shipped document.
fn sub_table_rows() -> BTreeMap<String, String> {
    let body = read_file(CONTRACT);
    let header = "| `finalize.*` code | Producer | Subject | `target` |";
    let start = body
        .find(header)
        .expect("the contract must carry the `finalize.*` sub-table with its declared header");
    let mut rows = BTreeMap::new();
    for line in body[start..].lines().skip(2) {
        if !line.starts_with('|') {
            break;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        assert_eq!(cells.len(), 4, "malformed sub-table row: {line}");
        let code = cells[0]
            .split('`')
            .nth(1)
            .unwrap_or_else(|| panic!("row names no code: {line}"));
        rows.insert(format!("{NAMESPACE}{code}"), cells[3].to_string());
    }
    rows
}

/// **Arm 3 — the sub-table renders the registry, row for row.**
#[test]
fn the_contract_sub_table_renders_the_registry() {
    let rows = sub_table_rows();

    let declared: BTreeSet<String> = FINALIZE_FAMILY.iter().map(|m| m.code.to_string()).collect();
    let tabled: BTreeSet<String> = rows.keys().cloned().collect();
    assert_eq!(
        tabled, declared,
        "the `{CONTRACT}` sub-table must carry exactly one row per \
         `cli::render::FINALIZE_FAMILY` member — a row for a code no producer mints is how \
         `finalize.forward-ref-dangling` was counted for two waves",
    );

    for member in FINALIZE_FAMILY {
        let target = &rows[member.code];
        assert!(
            target.contains(member.subject.target_form()),
            "the sub-table row for `{}` must name its declared target form `{}`; it reads: {target}",
            member.code,
            member.subject.target_form(),
        );
    }
}

// ---------------------------------------------------------------------------
// The live-doc set, the count fence, the record fence
// ---------------------------------------------------------------------------

/// Every markdown file that states current truth: `design/`, `implementation/` and the root
/// docs. `DECISIONS.md` and `completions/` are declared out (dated records — see the bounds in
/// the module doc).
fn live_docs() -> Vec<(String, String)> {
    let root = repo_root();
    let rel = |path: PathBuf| {
        path.strip_prefix(&root)
            .expect("a walked path sits under the repo root")
            .to_string_lossy()
            .to_string()
    };
    let mut rels: Vec<String> = ["design", "implementation"]
        .into_iter()
        .flat_map(|tree| root_walk::files(&root.join(tree), root_walk::ext("md")))
        .map(rel)
        .collect();
    rels.extend(
        root_walk::files_in(&root, |path| {
            root_walk::ext("md")(path) && !path.ends_with("DECISIONS.md")
        })
        .into_iter()
        .map(rel),
    );
    assert!(
        rels.len() > 30,
        "the live-doc set collapsed to {} files — the walk regressed",
        rels.len(),
    );
    rels.into_iter()
        .map(|rel| {
            let body = read_file(&rel);
            (rel, body)
        })
        .collect()
}

/// Count words and digits — anything that quantifies the family instead of pointing at the
/// table that enumerates it. Lifted deliberately from `exit_flip_count_record.rs`, whose fence
/// this one is the sibling of: one idiom, two families.
const COUNT_TOKENS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "0", "1", "2", "3", "4", "5", "6", "7",
    "8", "9",
];

/// Punctuation a quantifier cannot bind across. `exit_flip_count_record.rs`' version of this
/// helper takes a flat three-word window, and over a **list** that reads the previous item's
/// words as this item's qualifier: the roadmap's own M49 scope line
/// (*"`module-layout.md:58` and its two siblings · the `finalize.*` family size …"*) trips it on
/// a `two` belonging to a different bullet. So the lookback stops at the nearest clause
/// boundary — a quantifier binds inside its clause or not at all.
const CLAUSE_BOUNDARIES: &[char] = &[
    '\n', '·', '—', '–', ';', ':', '|', '(', ')', '[', ']', '.', '!', '?',
];

/// The up-to-three words immediately preceding `at` **within its clause**, lowercased — the slot
/// a quantifier occupies.
fn qualifier_slot(haystack: &str, at: usize) -> Vec<String> {
    let clause = match haystack[..at].rfind(CLAUSE_BOUNDARIES) {
        Some(cut) => &haystack[cut..at],
        None => &haystack[..at],
    };
    let mut words: Vec<String> = clause
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_string)
        .collect();
    let keep = words.len().saturating_sub(3);
    words.drain(..keep);
    words
}

fn assert_uncounted(subject: &str, phrase: &str, what: &str) {
    let mut from = 0;
    while let Some(rel) = subject[from..].find(phrase) {
        let at = from + rel;
        let words = qualifier_slot(subject, at);
        for token in COUNT_TOKENS {
            assert!(
                !words.iter().any(|w| w == token),
                "{what} quantifies `{phrase}` with `{token}` — the settled rule is point at \
                 `cli::render::FINALIZE_FAMILY`, do not re-count (`design/validation.md` → Exit \
                 semantics); the qualifier read: {words:?}",
            );
        }
        from = at + phrase.len();
    }
}

/// The phrases the family is **named** by. A count token qualifying any of them is a size
/// claim about the family, whatever number it carries.
const FAMILY_PHRASES: &[&str] = &[
    "`finalize.*` code",
    "`finalize.*` codes",
    "`finalize.*` family",
    "`finalize.*` constructors",
    "`finalize.*` members",
    "blocked-finalize code",
    "blocked-finalize codes",
];

/// **Arm 4 — no live doc states a numeral for the family.**
#[test]
fn no_live_doc_states_a_numeral_for_the_family() {
    let docs = live_docs();
    let mut seen = 0usize;
    for (rel, body) in &docs {
        for phrase in FAMILY_PHRASES {
            if body.contains(phrase) {
                seen += 1;
            }
            assert_uncounted(body, phrase, rel);
        }
    }
    assert!(
        seen > 0,
        "no live doc names the family at all — this fence would pass vacuously",
    );
}

/// The falsified statements — `(file, the bytes it carried, why it is now false)`. Named with
/// the bytes so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        CONTRACT,
        "twelve `finalize.*` codes sat outside the key spec",
        "the family's size, stated as twelve in the very sentence explaining that an \
         enumeration is not a boundary",
    ),
    (
        CONTRACT,
        "the **nine** blocked-finalize codes (`finalize.rs`, ten constructors)",
        "the form table's row counted the family twice over — nine codes and ten constructors — \
         and named one file as the family's home",
    ),
    (
        CONTRACT,
        "left the nine `finalize.*` codes",
        "the closure paragraph's own count, one wave after the count it was correcting",
    ),
    (
        CONTRACT,
        "All nine codes emit `location: None` today",
        "the sub-table's lead sentence, stating a size that was wrong when written and stale \
         once M42's plumbing landed the locations",
    ),
    (
        CONTRACT,
        "Not all nine take the work-unit form",
        "a second count in the same sentence pair, and a count of a *subset* besides",
    ),
    (
        CONTRACT,
        "in hand at every one of the ten constructors",
        "the constructor count, which had drifted from the code count it was supposed to \
         explain",
    ),
    (
        CONTRACT,
        "`write.*`, `finalize.forward-ref-dangling` |",
        "the form table's URI-form row claimed a code no production producer mints",
    ),
    (
        CONTRACT,
        "| `forward-ref-dangling` | `finding.rs:467` |",
        "the sub-table's row for that same phantom — it survives only as a unit-test fixture \
         in `finding.rs`, so it was a member of the count and of no code path",
    ),
    (
        CONTRACT,
        "The last two rows are the fifth un-enumerated sibling",
        "the note published *eleven plus one* from a second census, which the roadmap then \
         carried as twelve — and it missed `finalize.milestone-sub-task` in a third file",
    ),
    (
        "implementation/roadmap.md",
        "**`finalize.*` — twelve codes, every one `location: None`**",
        "the fourth number, and the one the contract's own note cited as authoritative",
    ),
    (
        "implementation/roadmap.md",
        "that gap is what let twelve codes sit outside",
        "the same number restated one clause on",
    ),
];

/// The replacements, each asserted **exactly once** in its file — a correction stated twice is
/// the restatement rot this repo's cross-reference rule exists to prevent.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        CONTRACT,
        "the blocked-finalize family `cli::render::FINALIZE_FAMILY` enumerates",
    ),
    (
        CONTRACT,
        "the verdicts are the `subject` column of `cli::render::FINALIZE_FAMILY`",
    ),
    (
        CONTRACT,
        "the table is the enumeration, and its size is stated nowhere",
    ),
    (
        "implementation/roadmap.md",
        "the family is enumerated at `cli::render::FINALIZE_FAMILY` and its size is stated \
         nowhere",
    ),
];

/// **Arm 5 — the record fence.**
#[test]
fn the_falsified_counts_are_gone_and_each_replacement_lands_once() {
    for (file, needle, why) in FALSIFIED {
        let body = read_file(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
    for (file, needle) in REPLACEMENTS {
        let body = read_file(file);
        assert_eq!(
            count(&body, needle),
            1,
            "{file} must state `{needle}` exactly once (point at the registry, never re-count — \
             and never restate a correction in two homes)",
        );
    }
}

// ---------------------------------------------------------------------------
// The identifier fence
// ---------------------------------------------------------------------------

/// File extensions a `finalize.<x>` token is a **filename**, not an identifier.
const EXTENSIONS: &[&str] = &["rs", "md", "yaml", "yml", "toml", "json"];

/// Every `finalize.<kebab>` identifier `body` names — excluding filenames and any token whose
/// `finalize` is the tail of a longer word (`milestone-finalize.chain-commit-rejected` is the
/// milestone family, not this one).
fn named_identifiers(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes = body.as_bytes();
    let mut from = 0usize;
    while let Some(rel) = body[from..].find(NAMESPACE) {
        let at = from + rel;
        from = at + NAMESPACE.len();
        if at > 0 {
            let prev = bytes[at - 1] as char;
            if prev.is_ascii_alphanumeric() || prev == '-' || prev == '_' || prev == '/' {
                continue;
            }
        }
        let tail: String = body[from..]
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
            .collect();
        if tail.is_empty() || EXTENSIONS.contains(&tail.as_str()) {
            continue;
        }
        out.insert(format!("{NAMESPACE}{tail}"));
    }
    out
}

/// **Arm 6 — every `finalize.*` identifier a live doc names is declared, one way or the other.**
///
/// This is what makes a phantom member impossible to re-add quietly: `forward-ref-dangling` sat
/// in the contract's table for two waves with no producer anywhere, and nothing could have
/// caught it, because the only thing checking the table was a person counting rows.
#[test]
fn every_finalize_identifier_in_a_live_doc_is_declared_one_way_or_the_other() {
    let declared: BTreeSet<&str> = FINALIZE_FAMILY
        .iter()
        .map(|m| m.code)
        .chain(FINALIZE_NON_MEMBERS.iter().map(|(id, _)| *id))
        .collect();

    let mut seen = BTreeSet::new();
    for (rel, body) in live_docs() {
        for identifier in named_identifiers(&body) {
            assert!(
                declared.contains(identifier.as_str()),
                "{rel} names `{identifier}`, which is in neither \
                 `cli::render::FINALIZE_FAMILY` nor `cli::render::FINALIZE_NON_MEMBERS` — every \
                 identifier in the `{NAMESPACE}` namespace is a member under the stated \
                 predicate or a declared non-member with its reason",
            );
            seen.insert(identifier);
        }
    }
    assert!(
        seen.len() >= 5,
        "the identifier scan found only {} identifiers across the live docs — it regressed",
        seen.len(),
    );
}

/// **Arm 7 — every fenced `finalize.*` contract identifier is disposed one way or the other,
/// and the split is *derived*.**
///
/// The claim is that `crate::pack`'s constraint-token map cannot name a `finalize.*` identifier
/// this registry has not answered for: each one is either a **minted** member of
/// [`FINALIZE_FAMILY`] or a **declared non-member** of [`FINALIZE_NON_MEMBERS`] carrying its
/// reason, with no third state.
///
/// **The split is read off the consts, not remembered** (the F-10 review's MEDIUM-4). It was
/// pinned at `(3, 1)` — a hand count that stood for a `crate::pack` doc-comment sentence
/// (*"three are minted finding codes … and one is not"*) which no longer exists there, and that
/// moved to `(5, 1)` the moment the amend arm's two contracts joined the fenced set. A numeral
/// here fences nothing the partition does not already fence: the declared-only half **is**
/// `DECLARED_CONTRACT_IDENTIFIERS` narrowed to this namespace, the minted half is the
/// remainder, and both halves being non-empty is what makes the disjunction meaningful.
#[test]
fn the_ambush_class_finalize_codes_are_declared_one_way_or_the_other() {
    let members: BTreeSet<&str> = FINALIZE_FAMILY.iter().map(|m| m.code).collect();
    let non_members: BTreeSet<&str> = FINALIZE_NON_MEMBERS.iter().map(|(id, _)| *id).collect();

    let mut minted = 0usize;
    let mut declared_only = 0usize;
    let mut namespaced = 0usize;
    for (code, _) in cli::pack::CONSTRAINT_REQUIRED_TOKENS {
        if !code.starts_with(NAMESPACE) {
            continue;
        }
        namespaced += 1;
        if members.contains(code) {
            minted += 1;
        } else if non_members.contains(code) {
            declared_only += 1;
        } else {
            panic!(
                "`crate::pack`'s constraint-token map names `{code}`, which is in neither \
                 `cli::render::FINALIZE_FAMILY` nor `cli::render::FINALIZE_NON_MEMBERS`",
            );
        }
    }
    let expected_declared_only = cli::pack::DECLARED_CONTRACT_IDENTIFIERS
        .iter()
        .filter(|(code, _)| code.starts_with(NAMESPACE))
        .count();
    assert_eq!(
        declared_only, expected_declared_only,
        "the declared-only half of the fenced `{NAMESPACE}` set IS \
         `DECLARED_CONTRACT_IDENTIFIERS` narrowed to this namespace — a producer-less identifier \
         the token map fences and that const does not carry would be a contract nothing owns",
    );
    assert_eq!(
        minted + declared_only,
        namespaced,
        "every fenced `{NAMESPACE}` identifier falls in exactly one half",
    );
    assert!(
        minted > 0 && declared_only > 0,
        "the disjunction means nothing unless both halves are represented (got {minted} minted, \
         {declared_only} declared-only)",
    );
}
