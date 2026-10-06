//! M49 Increment 8, T6 — the exit-flip count stops being a hand count, at the three
//! homes the settled rule never reached.
//!
//! `design/validation.md` → *Exit semantics* settled the rule in M46: **point at the
//! table, do not re-count** — *"the count is deliberately **not** restated here … the
//! table **is** the enumeration"* — and `tooling-tests/record_foreign_arm.rs` already
//! pins the byte it retired (`report-only, with three exit-flipping exceptions`) as
//! falsified **in that one file**. The rule never reached the other homes, and by M49
//! every one of them was stating a different wrong number against the same constant:
//!
//!   * `cli::render::STORE_EXIT_FLIPS` has **five** members (`probe-unreliable` ·
//!     `oob-rename` · `unmigrated-corpus` · `ahead-corpus` · `foreign-squatter`);
//!   * `design/command-output-contract.md` said **three** twice — once as a cross-
//!     reference naming a `validation.md` section title that no longer exists, once as
//!     the exit-code taxonomy's row-1 qualifier, which hand-listed three of the five
//!     codes *and* claimed a class (*the sweep could not produce a trustworthy result*)
//!     that two members carry `sweep_untrustworthy: false` against;
//!   * `design/assistant-adapter.md` said **four**, in the very clause whose stated
//!     reason for existing is that *enumerating them in a preloaded paragraph is
//!     content that rots* — the rot happened inside the sentence warning about it;
//!   * and **code itself** carried the stale count twice, in the crate that owns the
//!     exit vocabulary: `ExitClass::Error`'s doc-comment and the shipped `EXIT_CODES`
//!     row's `scope` string, both *"the three store-scope exit flips"*.
//!
//! This suite is the record fence for that sweep, on `record_foreign_arm.rs`' pattern:
//! **every falsified byte is named with the string it carried and asserted absent**, and
//! **every replacement is asserted exactly once** — a correction restated in two homes is
//! the rot this repo's own cross-reference rule exists to prevent.
//!
//! Two further arms carry what a byte list cannot:
//!
//!   * **the numeral fence** (`no_shipped_exit_code_row_or_task_source_counts_the_flips`)
//!     reads the **shipped** `EXIT_CODES` rows rather than their source, so a future row
//!     that re-counts the flips reddens whatever prose it is spelled in;
//!   * **the preload arm** (`the_bootstrap_preload_states_the_class_without_a_count`) is
//!     **green today and pinned, not changed**: `.jigc/AGENT.md` is the one surface that
//!     already obeys the settled rule — it says *a few conditions flip that exit* and
//!     names the closing line for the rest — and it is the surface an agent reads before
//!     anything else, so the property that kept it right is worth a standing test.
//!
//! And the increment's **declaration audit**: T1 and T2 each spent something on a pinned
//! contract (a `code` flip on the `(code, target)` key; an additive top-level key on the
//! `doc show` serve), and `design/command-output-contract.md` → *Evolution posture* is
//! explicit that a spend is a **declaration in its posture home**, never a field that
//! merely ships. The audit asserts each paragraph is where its siblings are.
//!
//! Doc-content assertions by nature — the deliverable *is* the prose and the shipped
//! strings. The exit-flipping behaviour itself is driven through the real binary
//! elsewhere (`crates/cli/tests/exit_codes.rs`, `crates/cli/tests/managed_vs_foreign.rs`,
//! and `cli::adapter`'s `bootstrap_clause_covers_every_store_exit_flip`).

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

/// The `## <heading>` section body — up to the next `## ` heading or EOF.
fn section<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("## {heading}\n");
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("the doc must carry a `## {heading}` section"));
    let rest = &body[start + marker.len()..];
    match rest.find("\n## ") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// The falsified statements — `(file, the bytes it carried, why it is now false)`.
/// Named with the bytes so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/command-output-contract.md",
        "report-only, with three exit-flipping exceptions",
        "a cross-reference naming a `validation.md` section title that M46 already \
         retired — the link's own text carried the count the target had dropped",
    ),
    (
        "design/command-output-contract.md",
        "the three store-scope **exit flips** (`pack-probe-integrity.*`, \
         `reconciliation.rename`, `schema-conformance.schema-version-current`)",
        "the exit-code taxonomy's row-1 qualifier hand-listed three of the five members \
         of `render::STORE_EXIT_FLIPS`",
    ),
    (
        "design/command-output-contract.md",
        "which say *the sweep could not produce a trustworthy result*",
        "false as a universal over the table: `oob-rename` and `foreign-squatter` both \
         carry `sweep_untrustworthy: false` — the sweep worked and is reporting what it \
         found (the same law-1 lie `assistant-adapter.md` already retired one tier down)",
    ),
    (
        "design/assistant-adapter.md",
        "four conditions flip it (an unreliable pack probe · an out-of-band rename · an \
         unmigrated corpus · an above-current stamp)",
        "the count and the enumeration inside the very clause whose stated reason for \
         existing is that enumerating them is content that rots",
    ),
    (
        "design/assistant-adapter.md",
        "true of three of the four",
        "the same hand-count one sentence on — and doubly wrong at five members, three \
         of which carry `sweep_untrustworthy: true`",
    ),
    (
        "design/assistant-adapter.md",
        "reddens when a fifth one would falsify the clause",
        "the fifth member landed at M46 (`foreign-squatter`); the fence is over any \
         further member, not over a particular ordinal",
    ),
    (
        "crates/cli/src/task.rs",
        "the three store-scope exit flips",
        "the crate that owns the exit vocabulary carried the stale count twice — \
         `ExitClass::Error`'s doc-comment and the shipped `EXIT_CODES` row's `scope`",
    ),
];

/// The replacements, each asserted **exactly once** in its file — a correction stated
/// twice is the restatement rot; stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "design/command-output-contract.md",
        "report-only, with the exit-flipping exceptions `render::STORE_EXIT_FLIPS` enumerates",
    ),
    (
        "design/command-output-contract.md",
        "the store-scope **exit flips** the `render::STORE_EXIT_FLIPS` table enumerates",
    ),
    (
        "design/command-output-contract.md",
        "each row carries its own `sweep_untrustworthy` verdict",
    ),
    (
        "design/assistant-adapter.md",
        "a handful of conditions flip it, and enumerating them in a preloaded paragraph \
         is content that rots",
    ),
    (
        "design/assistant-adapter.md",
        "which each member's own `sweep_untrustworthy` verdict decides",
    ),
    (
        "design/assistant-adapter.md",
        "reddens when a further one would falsify the clause",
    ),
    (
        "crates/cli/src/task.rs",
        "and the store-scope exit flips [`crate::render::STORE_EXIT_FLIPS`] enumerates.",
    ),
    (
        "crates/cli/src/task.rs",
        "and the store-scope exit flips `render::STORE_EXIT_FLIPS` enumerates",
    ),
];

/// **The sweep, arm 1 — no home still states a hand count of the store-scope flips.**
#[test]
fn no_home_still_states_the_hand_counted_exit_flips() {
    for (file, needle, why) in FALSIFIED {
        let body = read_file(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **The sweep, arm 2 — each replacement is stated once, in one home.**
#[test]
fn each_replacement_is_stated_exactly_once() {
    for (file, needle) in REPLACEMENTS {
        let body = read_file(file);
        assert_eq!(
            count(&body, needle),
            1,
            "{file} must state `{needle}` exactly once (point at the table, never re-count \
             — and never restate a correction in two homes)",
        );
    }
}

/// Count words and digits — anything that quantifies the flips instead of pointing at the
/// table that enumerates them.
const COUNT_TOKENS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

/// The **qualifier slot**: the three words immediately preceding `at`, lowercased. Three,
/// not a byte window, because the slot this fences is the one that directly quantifies the
/// phrase (*the **three** store-scope exit flips*) — a wider reach reads unrelated prose as
/// a count (`unless **one** of a few conditions flips that exit` is a head noun, not a
/// tally, and the preload is the surface that got this right).
fn qualifier_slot(haystack: &str, at: usize) -> Vec<String> {
    let mut words: Vec<String> = haystack[..at]
        .to_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_string())
        .collect();
    let keep = words.len().saturating_sub(3);
    words.drain(..keep);
    words
}

/// Assert no count token qualifies any occurrence of `phrase` in `subject`.
fn assert_uncounted(subject: &str, phrase: &str, what: &str) {
    let mut from = 0;
    while let Some(rel) = subject[from..].find(phrase) {
        let at = from + rel;
        let words = qualifier_slot(subject, at);
        for token in COUNT_TOKENS {
            assert!(
                !words.iter().any(|w| w == token),
                "{what} quantifies `{phrase}` with `{token}` — the settled rule is point \
                 at `render::STORE_EXIT_FLIPS`, do not re-count (`design/validation.md` → \
                 Exit semantics); the qualifier read: {words:?}",
            );
        }
        from = at + phrase.len();
    }
}

/// **Arm 3 — nothing the exit-code taxonomy *ships* counts the store-scope flips.**
///
/// Read off the shipped [`cli::task::EXIT_CODES`] rows rather than their source text, so a
/// future row that re-counts reddens however its prose is spelled — and over the whole of
/// `task.rs` besides, which is where both stale statements lived (one of them a
/// doc-comment, which no constant can carry).
#[test]
fn no_shipped_exit_code_row_or_task_source_counts_the_flips() {
    for row in cli::task::EXIT_CODES {
        assert_uncounted(
            row.meaning,
            "store-scope exit flip",
            &format!("the shipped EXIT_CODES `{:?}` row's `meaning`", row.class),
        );
        assert_uncounted(
            row.scope,
            "store-scope exit flip",
            &format!("the shipped EXIT_CODES `{:?}` row's `scope`", row.class),
        );
    }

    let source = read_file("crates/cli/src/task.rs");
    assert!(
        source.contains("store-scope exit flip"),
        "task.rs must still name the store-scope exit flips somewhere — this arm fences \
         how they are named, and a vacuous pass would hide their removal",
    );
    assert_uncounted(
        &source,
        "store-scope exit flip",
        "the exit-vocabulary home `crates/cli/src/task.rs`",
    );
}

/// **Arm 4 — the preload states the class without a count. Green today: pinned, not
/// changed.**
///
/// `.jigc/AGENT.md` is the one surface that already obeyed the settled rule, and it is the
/// first thing an agent reads. It says *a few conditions flip that exit* and hands the
/// reader to the sweep's own closing line for which one fired — naming exactly one
/// instance (the above-current stamp) as an example, which is a named instance and not an
/// enumeration. Asserted over the **shipped** body, so a re-enumeration reddens here.
#[test]
fn the_bootstrap_preload_states_the_class_without_a_count() {
    let body = cli::adapter::bootstrap_file();

    assert_eq!(
        count(&body, "unless one of a few conditions flips that exit"),
        1,
        "the preload must scope the report-only stance by the class of conditions, \
         stated once; got:\n{body}",
    );
    assert!(
        body.contains("its closing line names the condition and why"),
        "the preload must hand the reader to the sweep's own closing line for which \
         condition fired — that hand-off is what makes the un-enumerated class usable; \
         got:\n{body}",
    );
    assert_uncounted(&body, "conditions flip", "the `.jigc/AGENT.md` preload");
}

/// **The declaration audit — T1's and T2's spends each have their paragraph in their
/// posture home.**
///
/// Increment 8 spends on two pinned contracts, and `command-output-contract.md` →
/// *Evolution posture* is explicit that an addition to a pinned surface is a
/// **declaration**, never a field that merely ships (`record_foreign_arm.rs` → the arm
/// M46's completion audit added, for the identical omission one wave earlier). This
/// asserts each spend's paragraph sits in the section that homes its siblings.
#[test]
fn each_pinned_contract_spend_is_declared_in_its_posture_home() {
    // T1 — a `code` flip on the `(code, target)` key: the write-side posture home.
    let write = read_file("design/command-output-contract.md");
    let posture = section(&write, "Evolution posture (declared)");
    assert_eq!(
        count(
            posture,
            "**The M49 code flip, on the same authorization and the same bound: the \
             nested-section hop.**"
        ),
        1,
        "the nested-section `code` flip must be declared in the write-side posture home, \
         in its own paragraph — M47's authorization is cited, never silently reused",
    );
    for needle in ["write.unknown-section", "nested_section_undeclared"] {
        assert!(
            posture.contains(needle),
            "T1's declaration must name `{needle}` — a spend states what it converges and \
             on which predicate, or it is not a declaration",
        );
    }

    // T2 — an additive top-level key on the `doc show` serve: the read-side posture home,
    // the section that owns the pinned shape and its window discharge.
    let read = read_file("design/doc-read-surface.md");
    let pinned = section(&read, "The pinned `--format json` contract (1.0)");
    assert_eq!(
        count(
            pinned,
            "**The last spend before the pin (M49, the surface-completion wave).**"
        ),
        1,
        "the `schema-version` additive key must be declared in the read-side posture home, \
         beside the discharge paragraph that closes the window",
    );
    for needle in ["`schema-version`", "additive key", "1.0 pin"] {
        assert!(
            pinned.contains(needle),
            "T2's declaration must name `{needle}` — the key, its kind, and the pin the \
             still-open window closes at",
        );
    }
}
