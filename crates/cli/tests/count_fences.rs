//! **The count fences** — a numeral this repo states about a set its own code can move is
//! asserted against that set (M51 Increment 7, T3; `completions/artifacts/M51/settle-record.md`
//! → D8 and §11; the charter's EC-14).
//!
//! Every subject takes one shape, the mold `crates/cli/tests/doctype_map_versions.rs`
//! established: **read the registry, assert the prose**.
//!
//!   1. **The committing-door count.** [`COMMITTING_DOORS`] gained its tenth member at M49
//!      Increment 2 T3 (`jigc task discard` of a milestone sub-task), and **five prose homes
//!      kept saying nine** — `design/worked-examples.md`'s flow-47 arm-1 axis line among
//!      them, and `crates/cli/tests/flow47_acceptance.rs`, which contradicted itself four
//!      lines apart (`:18` *"10 doors since M49 Inc 2 T3"*, `:21` *"all nine pairwise
//!      distinct"*). The suites iterate the table, so they all passed: **only the prose
//!      lied, and nothing read it.**
//!   2. **The error-code mirror.** `design/surface-contract.md` → *The error-code namespace*
//!      declares itself an exact mirror of [`ERROR_CODE_REGISTRY`] — *"member-for-member —
//!      **eleven**"* — and `invocation_log.rs`'s own registry test hardcodes `11` **and names
//!      that file in its assert message**, while no test had ever opened it. Accurate today;
//!      unfenced, which is the first home's defect one door earlier.
//!   3. **The manifest-listed doctype totals** (T6). `design/corpus-migration.md` → *The
//!      freeze* argues the whole-file-shadow decision from how many doctypes a
//!      refusal-by-name would have cost, and counted the methodology manifest at ten after
//!      `planning-record` joined it at M49.
//!   4. **The write-miss axis' two sizes** (T6). `CLAUDE.md`'s M50 paragraph stated one
//!      numeral for two different tables — the cross's coordinates and the rows that
//!      witness them — and it was neither table's.
//!
//! **What is fenced and what is corrected — the residue is stated, not papered over.** A
//! count over a set the code can move is fenced *here*. A **historical** count — a record of
//! what a past wave drove, or of a red state on a past binary — is **dated-bracketed rather
//! than re-pinned**: re-pinning it to today would make the record lie about the measurement
//! it carries. The bracket is the shipped `[Corrected YYYY-MM-DD …]**` convention
//! `crates/cli/tests/foldback_truth.rs` already fences the pack-step count with; it is
//! re-implemented here rather than shared because the two suites are sibling `mod`s of one
//! group root with no home for a private helper between them.
//!
//! **The detector's bound, declared.** Two claim shapes are read anywhere in a fenced home —
//! *N committing doors* and *N-door … axis* — because they name their own subject. The
//! looser shapes (*N doors*, *N pairwise distinct*, *N distinct identities*) are read **only
//! inside a prose unit that names `COMMITTING_DOORS`**, since *doors* is a word this repo
//! uses for a dozen different registries. `CLAUDE.md` therefore takes the explicit shapes
//! only: its project-state record is one ~40 000-character paragraph, so a unit-scoped rule
//! has no unit there — it would read every door count in the repo's history as a claim about
//! this axis. A count stated in some third wording is outside this probe.

use crate::support;
use cli::invocation_log::{COMMITTING_DOORS, ERROR_CODE_REGISTRY};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root is reachable from the cli crate")
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

/// The doc of record for the error-code namespace.
const MIRROR: &str = "design/surface-contract.md";

/// The sentence in [`MIRROR`] that declares the mirror and states its size.
const MIRROR_CLAIM: &str = "mirrors the registry member-for-member";

/// How a fenced home's looser claim shapes are scoped.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Scope {
    /// Only the two self-naming shapes are read — the home has no usable prose unit.
    ExplicitOnly,
    /// The looser shapes are read too, inside any unit naming the registry.
    WithRegistryUnits,
}

/// What this suite owes a home.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Disposition {
    /// Every claim must state the registry's count or sit in a dated correction.
    Fenced,
    /// Deferred to a named increment, keyed on the stale literal that increment corrects.
    OwedAt {
        increment: &'static str,
        literal: &'static str,
    },
}

struct Home {
    path: &'static str,
    scope: Scope,
    disposition: Disposition,
}

/// **Every home that states the committing-door count**, enumerated rather than globbed on
/// `foldback_truth.rs`' reason: a sweep of every `.md` would reach the dated records, whose
/// job is to state the world as it was.
///
/// `MIGRATING.md` is **deferred, not exempt**. It is `include_str!`'d into the installed
/// `SKILL.md`, so any guide byte moves `jigc-body-blake3` and engages M48's refuse-to-clobber
/// path; the decomposition lands every guide byte this wave owes in one batch at Increment 9
/// (`implementation/roadmap.md` → Milestone 51, Increment 9), and this row's second leg
/// asserts the stale literal is **still stale**, so that correction reddens this suite and
/// forces the row to be re-dispositioned in the same commit.
const HOMES: &[Home] = &[
    Home {
        path: "CLAUDE.md",
        scope: Scope::ExplicitOnly,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "implementation/decisions-pending.md",
        scope: Scope::ExplicitOnly,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "design/worked-examples.md",
        scope: Scope::WithRegistryUnits,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "crates/cli/tests/flow47_acceptance.rs",
        scope: Scope::WithRegistryUnits,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "crates/cli/tests/commit_rejected_axis.rs",
        scope: Scope::WithRegistryUnits,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "crates/cli/tests/pinned_facts.rs",
        scope: Scope::ExplicitOnly,
        disposition: Disposition::Fenced,
    },
    Home {
        path: "MIGRATING.md",
        scope: Scope::ExplicitOnly,
        disposition: Disposition::OwedAt {
            increment: "Increment 9",
            literal: "All nine committing doors carry the frame",
        },
    },
];

/// The number words this repo spells counts in, plus the digit form. One table, both
/// directions — a fence that could read a numeral it cannot write would report its own
/// expectation in a spelling the doc never uses.
const NUMBER_WORDS: [&str; 21] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
];

fn number_word(n: usize) -> String {
    NUMBER_WORDS
        .get(n)
        .map(|w| (*w).to_string())
        .unwrap_or_else(|| n.to_string())
}

/// A numeral at `at`, in either spelling, returned with the byte it ends at.
fn number_at(chars: &[char], at: usize) -> Option<(usize, usize)> {
    if chars[at].is_ascii_digit() {
        let mut end = at;
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }
        let value: String = chars[at..end].iter().collect();
        return value.parse().ok().map(|n| (n, end));
    }
    let lower: String = chars[at..chars.len().min(at + 12)]
        .iter()
        .flat_map(|c| c.to_lowercase())
        .collect();
    NUMBER_WORDS
        .iter()
        .enumerate()
        .filter(|(_, word)| lower.starts_with(*word))
        .filter(|(_, word)| {
            chars
                .get(at + word.chars().count())
                .is_none_or(|c| !c.is_alphanumeric())
        })
        // Longest first, so `nineteen` is never read as `nine`.
        .max_by_key(|(_, word)| word.len())
        .map(|(value, word)| (value, at + word.chars().count()))
}

/// The file's prose with its markdown emphasis, its code ticks and its Rust comment markers
/// removed, plus the original byte offset of every retained character. Claims routinely wrap
/// a line (`the same nine` / `//! doors, …`) and routinely carry emphasis mid-phrase
/// (`the other **eight** committing doors`), so a line-at-a-time or raw-bytes reader would
/// miss exactly the statements that have gone stale.
fn normalize(body: &str) -> (Vec<char>, Vec<usize>) {
    let mut text = Vec::new();
    let mut map = Vec::new();
    for (offset, line) in line_spans(body) {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let mut at = offset + indent;
        let mut rest = trimmed;
        for marker in ["//!", "///", "//"] {
            if let Some(stripped) = rest.strip_prefix(marker) {
                at += marker.len();
                rest = stripped;
                break;
            }
        }
        for (i, c) in rest.char_indices() {
            if c == '*' || c == '`' {
                continue;
            }
            text.push(c);
            map.push(at + i);
        }
        text.push(' ');
        map.push(offset + line.len());
    }
    (text, map)
}

/// `(offset, line)` for every line of `body`, the newline excluded.
fn line_spans(body: &str) -> Vec<(usize, &str)> {
    let mut spans = Vec::new();
    let mut at = 0usize;
    for line in body.split('\n') {
        spans.push((at, line));
        at += line.len() + 1;
    }
    spans
}

/// The body's **prose units**: maximal runs of lines that are not blank once the comment
/// marker is stripped. A markdown paragraph and a Rust doc-comment block are both one unit,
/// which is the granularity at which a claim and the registry it is about sit together.
fn units(body: &str) -> Vec<(usize, usize)> {
    let mut units = Vec::new();
    let mut open: Option<(usize, usize)> = None;
    for (offset, line) in line_spans(body) {
        let content = line
            .trim_start()
            .trim_start_matches("//!")
            .trim_start_matches("///")
            .trim_start_matches("//")
            .trim();
        if content.is_empty() {
            if let Some(unit) = open.take() {
                units.push(unit);
            }
        } else {
            let end = offset + line.len();
            open = Some(open.map_or((offset, end), |(start, _)| (start, end)));
        }
    }
    units.extend(open);
    units
}

/// Byte spans of the body's **dated** correction brackets — `[Corrected YYYY-MM-DD …]**`, the
/// convention `DECISIONS.md` uses to keep a superseded figure visible beside what superseded
/// it. An undated `[Corrected …]` is not one: the date is the whole point.
fn dated_correction_spans(body: &str) -> Vec<(usize, usize)> {
    const OPEN: &str = "[Corrected ";
    let mut spans = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = body[from..].find(OPEN) {
        let start = from + rel;
        let after = start + OPEN.len();
        from = after;
        let rest = &body[after..];
        let dated = rest.len() >= 10
            && rest.as_bytes()[..10].iter().enumerate().all(|(i, b)| {
                if i == 4 || i == 7 {
                    *b == b'-'
                } else {
                    b.is_ascii_digit()
                }
            });
        if dated && let Some(close) = rest.find("]**") {
            spans.push((start, after + close + "]**".len()));
        }
    }
    spans
}

/// One stated count of the committing-door axis.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Claim {
    value: usize,
    /// The phrase as the file writes it, normalized — what the failure message quotes.
    phrase: String,
    /// Byte offset into the original file.
    at: usize,
}

/// Whether `text[at..]` begins with `word` as a whole word.
fn word_at(text: &[char], at: usize, word: &str) -> Option<usize> {
    let needle: Vec<char> = word.chars().collect();
    if !text[at..].starts_with(needle.as_slice()) {
        return None;
    }
    let end = at + needle.len();
    text.get(end)
        .is_none_or(|c| !c.is_alphanumeric())
        .then_some(end)
}

/// Skip the run of spaces and hyphens joining a numeral to its noun.
fn skip_joiners(text: &[char], mut at: usize) -> usize {
    while text.get(at).is_some_and(|c| *c == ' ' || *c == '-') {
        at += 1;
    }
    at
}

/// Every committing-door count `text` states, under the scope's rules.
///
/// Three shapes are read, and the plural is required throughout: *one door's fixture* and
/// *exactly one door framed its rejection* are not claims about the size of a ten-member set,
/// and a fence that read them would report a defect at every singular sentence in the tree.
fn claims(text: &[char], map: &[usize], scope: Scope) -> Vec<Claim> {
    let names_registry = text
        .windows("COMMITTING_DOORS".len())
        .any(|w| w.iter().collect::<String>() == "COMMITTING_DOORS");
    let loose = scope == Scope::WithRegistryUnits && names_registry;
    let mut found = Vec::new();
    let mut at = 0usize;
    while at < text.len() {
        if at > 0 && text[at - 1].is_alphanumeric() {
            at += 1;
            continue;
        }
        let Some((value, after_number)) = number_at(text, at) else {
            at += 1;
            continue;
        };
        let mut end = None;
        // (a) `N committing doors` — self-naming, read in every home.
        let head = skip_joiners(text, after_number);
        if let Some(after) = word_at(text, head, "committing") {
            end = word_at(text, skip_joiners(text, after), "doors");
        }
        // (b) `N-door … axis` — the compound form, also self-naming.
        if end.is_none()
            && text.get(after_number) == Some(&'-')
            && let Some(after) = word_at(text, after_number + 1, "door")
        {
            let window: String = text[after..text.len().min(after + 30)].iter().collect();
            if window.contains("axis") {
                end = Some(after);
            }
        }
        // (c) the loose shapes, inside a unit that names the registry.
        if end.is_none() && loose {
            end = word_at(text, head, "doors")
                .or_else(|| word_at(text, head, "pairwise distinct"))
                .or_else(|| word_at(text, head, "distinct identities"));
        }
        let Some(end) = end else {
            at = after_number;
            continue;
        };
        found.push(Claim {
            value,
            phrase: text[at..end].iter().collect(),
            at: map[at],
        });
        at = end;
    }
    found
}

/// Every claim a home states, partitioned into *all* of them and the ones the home states in
/// its **own present-tense voice** — those outside every dated correction bracket.
///
/// Both halves are load-bearing. The second is what must be current; the first is what proves
/// the home still speaks about this axis at all, so a home whose last statement is deleted
/// reddens instead of silently passing an empty fence.
fn claims_of(home: &Home) -> (Vec<Claim>, Vec<Claim>) {
    let body = read(home.path);
    let brackets = dated_correction_spans(&body);
    let mut all = Vec::new();
    for (start, end) in units(&body) {
        let (text, map) = normalize(&body[start..end]);
        let absolute: Vec<usize> = map.iter().map(|at| start + at).collect();
        all.extend(claims(&text, &absolute, home.scope));
    }
    let uncorrected = all
        .iter()
        .filter(|claim| {
            !brackets
                .iter()
                .any(|(s, e)| claim.at >= *s && claim.at < *e)
        })
        .cloned()
        .collect();
    (all, uncorrected)
}

fn line_of(body: &str, at: usize) -> usize {
    body[..at].matches('\n').count() + 1
}

/// The headline: every home that states the size of the committing-door axis states the size
/// the registry carries.
#[test]
fn every_door_count_home_states_the_registry_count() {
    let expected = COMMITTING_DOORS.len();
    let mut stale = Vec::new();
    for home in HOMES {
        if home.disposition != Disposition::Fenced {
            continue;
        }
        let body = read(home.path);
        let (all, uncorrected) = claims_of(home);
        assert!(
            !all.is_empty(),
            "{} states no committing-door count any more, in its own voice or in a dated \
             bracket — either the sentence moved (re-key the home) or the home stopped \
             stating it, in which case it is fencing nothing",
            home.path,
        );
        for claim in uncorrected.into_iter().filter(|c| c.value != expected) {
            stale.push(format!(
                "{}:{} says `{}` — `COMMITTING_DOORS` carries {expected}",
                home.path,
                line_of(&body, claim.at),
                claim.phrase.trim(),
            ));
        }
    }
    assert!(
        stale.is_empty(),
        "the committing-door axis is a code-side table (`cli::invocation_log::COMMITTING_DOORS`) \
         and these homes state a size it does not carry. A count that is *historical* — what a \
         past wave drove, or a red state on a past binary — takes a dated `[Corrected …]**` \
         bracket instead of today's number, so the record keeps stating its own measurement:\n{}",
        stale.join("\n"),
    );
}

/// The fence is a property of its predicate, not of today's bytes: it must **catch** a stale
/// count, **spare** a dated one, and **ignore** the singular and the other registries' doors.
#[test]
fn a_stale_count_is_caught_a_dated_one_is_spared_and_a_singular_is_ignored() {
    let scan = |text: &str, scope: Scope| -> Vec<usize> {
        let (chars, map) = normalize(text);
        claims(&chars, &map, scope)
            .into_iter()
            .map(|c| c.value)
            .collect()
    };

    assert_eq!(
        scan(
            "All **nine** committing doors carry the frame.",
            Scope::ExplicitOnly
        ),
        vec![9],
        "the self-naming shape is read in every home, emphasis and all",
    );
    assert_eq!(
        scan(
            "swept over the whole nine-door committing axis",
            Scope::ExplicitOnly
        ),
        vec![9],
        "the compound shape is read too — it is how the record writes the same claim",
    );
    assert_eq!(
        scan(
            "the axis: the code-side `COMMITTING_DOORS` table — 9 doors, 9 distinct identities",
            Scope::WithRegistryUnits,
        ),
        vec![9, 9],
        "the loose shapes are read inside a unit that names the registry",
    );
    assert_eq!(
        scan(
            "the write path reaches all 25 doors of the corpus",
            Scope::WithRegistryUnits
        ),
        Vec::<usize>::new(),
        "another registry's door count is not this axis' — without the registry named, the \
         loose shape is not a claim",
    );
    assert_eq!(
        scan(
            "Build one door's fixture off `COMMITTING_DOORS`, one door at a time",
            Scope::WithRegistryUnits,
        ),
        Vec::<usize>::new(),
        "the singular is never a statement of the set's size",
    );
    assert_eq!(
        scan("nineteen committing doors", Scope::ExplicitOnly),
        vec![19],
        "`nineteen` must not be read as `nine`",
    );

    // And the bracket spares only a *dated* correction.
    let dated = "the whole **[Corrected 2026-09-15: nine-door axis]** frame";
    assert_eq!(dated_correction_spans(dated).len(), 1);
    assert!(
        dated_correction_spans("the whole **[Corrected at review: nine-door axis]** frame")
            .is_empty(),
        "an undated correction is not one — the date is the whole point",
    );
}

/// The guide's correction is deferred to the single guide batch, and the deferral is
/// bookkeeping rather than an exemption: the stale literal must still be there, and still be
/// stale, so Increment 9's edit reddens this suite.
#[test]
fn the_guide_door_count_is_owed_at_the_single_guide_batch() {
    let mut checked = 0usize;
    for home in HOMES {
        let Disposition::OwedAt { increment, literal } = home.disposition else {
            continue;
        };
        checked += 1;
        let body = read(home.path);
        assert!(
            body.contains(literal),
            "{} is excluded from the live fence because its correction lands in {increment}'s \
             single guide batch, keyed on the literal `{literal}`. That literal is gone — so \
             either the correction landed (lift the exclusion and fence the guide live) or the \
             sentence moved (re-key the row).",
            home.path,
        );
        let (_, uncorrected) = claims_of(home);
        assert!(
            uncorrected
                .iter()
                .any(|c| c.value != COMMITTING_DOORS.len()),
            "{}'s owed literal must still state a count the registry does not carry — the \
             deferral is bookkeeping, not a blanket exemption; found {uncorrected:?}",
            home.path,
        );
    }
    assert_eq!(
        checked, 1,
        "exactly one home is deferred to the guide batch"
    );
}

/// The mirror table lists [`ERROR_CODE_REGISTRY`] member-for-member, in order — the claim
/// `design/surface-contract.md` makes about itself, and the one `invocation_log.rs`'s registry
/// test has named in its assert message since M47 without any test opening the file.
#[test]
fn the_error_code_mirror_lists_the_registry_member_for_member() {
    let body = read(MIRROR);
    let mirrored = mirror_identities(&body);
    assert_eq!(
        mirrored,
        ERROR_CODE_REGISTRY.to_vec(),
        "{MIRROR} → The error-code namespace declares itself an exact mirror of \
         `cli::invocation_log::ERROR_CODE_REGISTRY`; a member minted, retired or reordered in \
         the registry moves the table in the same commit",
    );
}

/// …and the size it states is the size it lists.
#[test]
fn the_error_code_mirror_states_the_registry_member_count() {
    let body = read(MIRROR);
    let at = body
        .find(MIRROR_CLAIM)
        .unwrap_or_else(|| panic!("{MIRROR} must declare the mirror: `{MIRROR_CLAIM}`"));
    let tail: Vec<char> = body[at + MIRROR_CLAIM.len()..]
        .chars()
        .take_while(|c| *c != ':')
        .collect();
    let stated = (0..tail.len())
        .filter(|i| *i == 0 || !tail[i - 1].is_alphanumeric())
        .find_map(|i| number_at(&tail, i))
        .map(|(value, _)| value);
    assert_eq!(
        stated,
        Some(ERROR_CODE_REGISTRY.len()),
        "{MIRROR}'s mirror sentence states `{}` — the registry has {} members ({})",
        tail.iter().collect::<String>().trim(),
        ERROR_CODE_REGISTRY.len(),
        number_word(ERROR_CODE_REGISTRY.len()),
    );
}

/// The identity column of the mirror table, in document order.
fn mirror_identities(body: &str) -> Vec<&str> {
    let at = body
        .find(MIRROR_CLAIM)
        .unwrap_or_else(|| panic!("{MIRROR} must declare the mirror: `{MIRROR_CLAIM}`"));
    let rows: Vec<&str> = body[at..]
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .collect();
    assert!(
        rows.len() > 2,
        "{MIRROR}: the mirror sentence is not followed by a table",
    );
    rows[2..]
        .iter()
        .map(|row| {
            row.trim_start_matches('|')
                .split('|')
                .next()
                .expect("a row has a first cell")
                .trim()
                .trim_matches('`')
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The residue (M51 Increment 7, T6) — §11's rule, applied member by member.
// ---------------------------------------------------------------------------
//
// §11 partitions the counts this repo states into three, and only the first is fenceable:
//
//   * **A count over a set the code can move** is fenced against that set. Two more join
//     here — the manifest-listed doctype totals and the write-miss axis' size.
//   * **A count over a set the code cannot move** is corrected in place and left
//     unfenced, because there is nothing to fence it against. `design/doc-read-surface.md`
//     → *The version/posture map* headed **five regimes** over a six-row table: a
//     version/posture *regime* is a judgment about a surface, not a registry, so the
//     heading, its opening sentence and its *owns three of the five* clause are corrected,
//     and `design/command-output-contract.md`'s two cross-references stop restating the
//     figure at all — an unfenceable count is best carried in one home, not two.
//   * **A historical count** — what a past wave drove — is **dated-bracketed, never
//     re-pinned**: re-pinning it would make the record lie about its own measurement.
//     Both `schema-manifest.yaml` headers stated *all 16 doctype hashes* of M47 Increment
//     1's genesis re-pin; the dev header keeps that figure inside a dated bracket, and the
//     methodology header — which restated the dev header's history verbatim — now
//     cross-references it (CLAUDE.md → *Cross-reference, never restate*).

/// The dev pack's frozen-set manifest.
const DEV_MANIFEST: &str = "crates/cli/pack/config/schema-manifest.yaml";
/// The methodology pack's, which ends its freeze exemption (M40 A1).
const METHODOLOGY_MANIFEST: &str = "packs/methodology/config/schema-manifest.yaml";
/// The doc that states how many doctypes the two of them list between them.
const MANIFEST_CLAIM_HOME: &str = "design/corpus-migration.md";
/// The phrase that opens the claim. Anchored on the *sentence*, not on a numeral, so a
/// stale figure reddens on its value rather than by the anchor going missing.
const MANIFEST_CLAIM_ANCHOR: &str = "would have deleted a documented capability for";

/// Every doctype a manifest lists, in document order.
///
/// Read as lines rather than parsed: the file is a hand-edited artifact whose `- type:`
/// column is exactly the thing being counted, and a YAML round-trip here would add a
/// dependency to read one field.
fn manifest_types(rel: &str) -> Vec<String> {
    let body = read(rel);
    let listed = body
        .find("\ndoctypes:\n")
        .map(|at| at + "\ndoctypes:\n".len())
        .unwrap_or_else(|| panic!("{rel} must carry a `doctypes:` list"));
    body[listed..]
        .lines()
        .take_while(|line| line.starts_with("  - ") || line.starts_with("    "))
        .filter_map(|line| line.trim().strip_prefix("- type:"))
        .map(|ty| ty.trim().to_string())
        .collect()
}

/// The first `limit` numerals `line` states, in either spelling, in order.
fn numbers_in(line: &str, limit: usize) -> Vec<usize> {
    let (text, _) = normalize(line);
    let mut found = Vec::new();
    let mut at = 0usize;
    while at < text.len() && found.len() < limit {
        if at > 0 && text[at - 1].is_alphanumeric() {
            at += 1;
            continue;
        }
        match number_at(&text, at) {
            Some((value, end)) => {
                found.push(value);
                at = end;
            }
            None => at += 1,
        }
    }
    found
}

/// The line of `body` carrying `anchor`.
fn line_carrying<'a>(body: &'a str, anchor: &str, rel: &str) -> &'a str {
    body.lines()
        .find(|line| line.contains(anchor))
        .unwrap_or_else(|| panic!("{rel} must carry the claim anchored at `{anchor}`"))
}

/// The manifest-listed doctype counts are the manifests' own — both ratios, the total
/// across the two packs, and the number of **distinct type names** that total spans.
///
/// The two halves of each ratio are the same number by construction: pack-load asserts the
/// declared set and the shipped set are exactly equal, so *N of N are manifest-listed* is
/// one figure stated twice, and the manifest is where it lives.
#[test]
fn the_manifest_listed_doctype_counts_are_the_manifests_own() {
    let dev = manifest_types(DEV_MANIFEST);
    let methodology = manifest_types(METHODOLOGY_MANIFEST);
    let distinct: std::collections::BTreeSet<&String> = dev.iter().chain(&methodology).collect();
    let expected = vec![
        dev.len(),
        dev.len(),
        methodology.len(),
        methodology.len(),
        dev.len() + methodology.len(),
        distinct.len(),
    ];

    let body = read(MANIFEST_CLAIM_HOME);
    let line = line_carrying(&body, MANIFEST_CLAIM_ANCHOR, MANIFEST_CLAIM_HOME);
    let at = line
        .find(MANIFEST_CLAIM_ANCHOR)
        .expect("the anchor is on the line it was found by");
    let stated = numbers_in(&line[at..], expected.len());

    assert_eq!(
        stated,
        expected,
        "{MANIFEST_CLAIM_HOME} → *Hashed, not refused by name* states the size of a set \
         the two shipped manifests carry: {} dev ({dev:?}) + {} methodology \
         ({methodology:?}) = {} entries over {} distinct type names. The sentence must \
         state, in order, each ratio's two halves, the total and the distinct count.",
        dev.len(),
        methodology.len(),
        dev.len() + methodology.len(),
        distinct.len(),
    );
}

/// Where the write-miss axis' rows live …
const AXIS_ROWS: &str = "crates/cli/tests/support/write_miss_cells.rs";
/// … and where the cross they are witnesses of lives. It is a `const` private to another
/// test group (`g_finalize`), so it is counted from its own source rather than `use`d —
/// the declared bound of this half of the fence.
const AXIS_CROSS: &str = "crates/cli/tests/write_miss_shape_axis.rs";
/// The one doc that states either size outside the two files above.
const AXIS_CLAIM_HOME: &str = "CLAUDE.md";

/// The number of `MissShape` rows `write_miss_shape_axis.rs` declares in `MISS_SHAPES`,
/// read from the source of the table itself.
fn cross_coordinates() -> usize {
    let body = read(AXIS_CROSS);
    let at = body
        .find("\nconst MISS_SHAPES:")
        .unwrap_or_else(|| panic!("{AXIS_CROSS} must declare `MISS_SHAPES`"));
    let rows = body[at..]
        .lines()
        .take_while(|line| *line != "];")
        .filter(|line| line.trim_end() == "    MissShape {")
        .count();
    assert!(rows > 0, "{AXIS_CROSS}: `MISS_SHAPES` parsed as empty");
    rows
}

/// Every count of the write-miss axis `text` states — a numeral governing `CELLS rows` or
/// `MISS_SHAPES coordinates`, the two shapes the record writes them in.
fn axis_claims(text: &[char], map: &[usize]) -> Vec<(Claim, &'static str)> {
    let mut found = Vec::new();
    let mut at = 0usize;
    while at < text.len() {
        if at > 0 && text[at - 1].is_alphanumeric() {
            at += 1;
            continue;
        }
        let Some((value, after_number)) = number_at(text, at) else {
            at += 1;
            continue;
        };
        let head = skip_joiners(text, after_number);
        let hit = word_at(text, head, "CELLS")
            .and_then(|after| word_at(text, skip_joiners(text, after), "rows"))
            .map(|end| (end, "CELLS"))
            .or_else(|| {
                word_at(text, head, "MISS_SHAPES")
                    .and_then(|after| word_at(text, skip_joiners(text, after), "coordinates"))
                    .map(|end| (end, "MISS_SHAPES"))
            });
        let Some((end, subject)) = hit else {
            at = after_number;
            continue;
        };
        found.push((
            Claim {
                value,
                phrase: text[at..end].iter().collect(),
                at: map[at],
            },
            subject,
        ));
        at = end;
    }
    found
}

/// The write-miss axis has two sizes and they count different things — the **cross**'s
/// `(verb, shape, leading-hop declaredness)` coordinates and the **rows** that witness
/// them. `CLAUDE.md`'s M50 paragraph stated one numeral for both, and it was neither:
/// *thirteen* is `write_miss_shape_axis.rs`'s count of the address-shape column's own new
/// item-addressing rows, read as the size of the whole axis.
#[test]
fn the_write_miss_axis_sizes_are_the_axis_tables_own() {
    let rows = support::write_miss_cells::CELLS.len();
    let cross = cross_coordinates();
    let body = read(AXIS_CLAIM_HOME);
    let (text, map) = normalize(&body);
    let claims = axis_claims(&text, &map);

    let mut stale = Vec::new();
    for (claim, subject) in &claims {
        let expected = if *subject == "CELLS" { rows } else { cross };
        if claim.value != expected {
            stale.push(format!(
                "{AXIS_CLAIM_HOME}:{} says `{}` — {subject} carries {expected} ({})",
                line_of(&body, claim.at),
                claim.phrase.trim(),
                number_word(expected),
            ));
        }
    }
    assert!(
        stale.is_empty(),
        "the write-miss axis is two code-side tables — `{AXIS_ROWS}`'s `CELLS` ({rows} \
         rows) and `{AXIS_CROSS}`'s `MISS_SHAPES` ({cross} coordinates) — and each numeral \
         must name the one it counts:\n{}",
        stale.join("\n"),
    );
    assert_eq!(
        claims.len(),
        2,
        "{AXIS_CLAIM_HOME} must state both sizes, each naming its own table; found {:?}",
        claims
            .iter()
            .map(|(c, s)| format!("{} ({s})", c.phrase.trim()))
            .collect::<Vec<_>>(),
    );
}
