//! The illustrative disclaimer says what it disclaims (M49 Increment 11, T12).
//!
//! Fourteen of the twenty-eight `design/` part-docs open with a doc-level
//! *"Notation is illustrative"* header. The header names the thing it disclaims —
//! *notation* — but never says what that leaves standing, so a reader is free to
//! read it as disclaiming the whole document. One did: M47's D1 was refused with
//! *"No rule. The near-miss citation is killed by `Notation is illustrative`."* A
//! razor whose leg 1 is *a rule stated in a locked artifact and violated at HEAD*
//! then decides by **which doc an item happens to cite** — the same rule refuses in
//! `storage.md` and passes in `surface-contract.md`.
//!
//! The fix is a scope, stated **once** and referenced from every header, never
//! restated fourteen times ([`CLAUDE.md`](../CLAUDE.md) → *How we work
//! together* → *What "Notation is illustrative" disclaims*). This suite fences both
//! halves of that: the reference is present wherever the disclaimer is, and the rule
//! itself is stated in exactly one home.
//!
//! **The subject is derived on the concept, not on the string.** A grep for the
//! common phrasing misses `structural-grammar.md:5`, which reads *"Notation **below**
//! is illustrative"* — and a future doc may word it differently again. What makes a
//! disclaimer *doc-level* is **position, not wording**: it sits in the document's
//! header region, before the first `##` section, where it governs everything that
//! follows. So the derivation is:
//!
//! > a **doc-level illustrative disclaimer** is the word *illustrative* appearing in
//! > a `design/*.md` file's **header region** — every line preceding its first `## `
//! > heading.
//!
//! That boundary is what discriminates the two objects the increment must keep
//! apart. `changelog.md:34` and `self-hosting.md:123` also say *illustrative*, but
//! both sit inside a section and caveat **one example**; they are not doc-level
//! claims and they correctly stay as they are. `validation.md:148` is the same shape
//! a third time. Position separates them with no list to maintain.
//!
//! The second test keys on the **rule**, not on a header. The rule has three parts —
//! the **object** it governs (a *disclaimer*), what that object **removes**
//! (*illustrative* notation), and what it **leaves standing** (a *citable* rule) — and
//! a paragraph naming all three is stating the rule rather than using a header or
//! discussing something adjacent. Each pair alone is a false positive already present
//! in the corpus: the fourteen references name the object and the removal but never
//! citability, and `worked-examples.md` pairs *illustrative* with `doc-code`'s
//! unrelated sense of a *citable* code unit without ever mentioning a disclaimer.
//!
//! `DECISIONS.md` is out of the scan for the same reason `completions/` is: it is the
//! **dated log of what was decided**, explicitly not current truth (`CLAUDE.md` holds
//! that), so its planning entry for this very row quotes the criterion verbatim. A
//! fence that forced that entry to be rewritten would be falsifying a record to stay
//! green. The bound is stated rather than hidden: an append-only log can carry a stale
//! copy of the rule, and nothing here catches that — it is not a home a reader is
//! routed to for current truth.

use crate::support::root_walk;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The corpus the disclaimer governs.
const DESIGN_DIR: &str = "design";

/// The single home of the rule.
const HOME: &str = "CLAUDE.md";

/// The link every disclaimered doc carries, resolved from inside `design/`.
const HOME_REFERENCE: &str = "../CLAUDE.md#how-we-work-together";

/// The word that marks a disclaimer.
const DISCLAIMED_MARKER: &str = "illustrative";

/// The word that marks what the disclaimer leaves standing.
const SURVIVES_MARKER: &str = "citable";

/// The word that marks the thing being scoped — a statement *about* a disclaimer,
/// rather than a use of one.
const OBJECT_MARKER: &str = "disclaim";

/// The living documentation set. `completions/` is an append-only archive of past
/// trial and milestone artifacts: those quote whatever the docs said at the time, so
/// fencing them would redden on history rather than on rot.
const LIVING_DOC_ROOTS: &[&str] = &["", "design", "implementation", "ideas"];

/// Excluded from the living set for the same reason as `completions/` — the dated log
/// of what was decided, not a home for current truth.
const DATED_LOG: &str = "DECISIONS.md";

/// This repo — every document under test is checked in, so the derivation runs
/// against the checkout rather than a fabricated tree.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn markdown_files(dir: &Path) -> Vec<PathBuf> {
    root_walk::files_in(dir, root_walk::ext("md"))
}

/// Every line of `src` preceding its first `## ` heading — the header region, where a
/// claim governs the whole document rather than one section.
fn header_region(src: &str) -> String {
    src.lines()
        .take_while(|l| !l.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every `design/*.md` whose header region carries a doc-level illustrative
/// disclaimer, by file name.
fn disclaimered_design_docs(root: &Path) -> BTreeSet<String> {
    markdown_files(&root.join(DESIGN_DIR))
        .into_iter()
        .filter(|p| {
            let src = fs::read_to_string(p).expect("a readable design doc");
            header_region(&src)
                .to_lowercase()
                .contains(DISCLAIMED_MARKER)
        })
        .map(|p| {
            p.file_name()
                .expect("a named file")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

#[test]
fn every_doc_level_illustrative_disclaimer_carries_the_scoping_reference() {
    let root = repo_root();
    let disclaimered = disclaimered_design_docs(&root);

    // Guard the derivation itself: an empty subject would make this suite pass by
    // matching nothing, which is exactly the failure it exists to prevent.
    assert!(
        !disclaimered.is_empty(),
        "the derivation matched no doc-level illustrative disclaimer at all — \
         the header-region boundary or the marker word has drifted"
    );

    let unscoped: Vec<&String> = disclaimered
        .iter()
        .filter(|name| {
            let src = fs::read_to_string(root.join(DESIGN_DIR).join(name))
                .expect("a readable design doc");
            !header_region(&src).contains(HOME_REFERENCE)
        })
        .collect();

    assert!(
        unscoped.is_empty(),
        "{} design doc(s) open with a doc-level illustrative disclaimer that never \
         says what it disclaims — each must link the one home, `{HOME_REFERENCE}`, \
         rather than restate the rule: {unscoped:?}",
        unscoped.len()
    );
}

#[test]
fn the_scoping_rule_is_stated_in_exactly_one_home() {
    let root = repo_root();

    let mut homes: Vec<String> = Vec::new();
    for dir in LIVING_DOC_ROOTS {
        let dir = if dir.is_empty() {
            root.clone()
        } else {
            root.join(dir)
        };
        if !dir.is_dir() {
            continue;
        }
        for path in markdown_files(&dir) {
            if path.file_name().is_some_and(|n| n == DATED_LOG) {
                continue;
            }
            let src = fs::read_to_string(&path).expect("a readable document");
            let statements = src
                .split("\n\n")
                .filter(|para| {
                    let p = para.to_lowercase();
                    p.contains(OBJECT_MARKER)
                        && p.contains(DISCLAIMED_MARKER)
                        && p.contains(SURVIVES_MARKER)
                })
                .count();
            let rel = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            for _ in 0..statements {
                homes.push(rel.clone());
            }
        }
    }

    assert_eq!(
        homes,
        vec![HOME.to_owned()],
        "the scope of the illustrative disclaimer is a rule with one home: it must be \
         stated in `{HOME}` and nowhere else, and every other doc must reference it \
         instead of restating it"
    );
}
