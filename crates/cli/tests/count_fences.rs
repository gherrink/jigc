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
//!   4. **The write-miss axis' two sizes** (T6). The record's M50 span (then still in
//!      `CLAUDE.md`) stated one numeral for two different tables — the cross's
//!      coordinates and the rows that witness them — and it was neither table's.
//!   5. **The record-only door set** (M52 Increment 5, T11) — one axis stated three
//!      different sizes inside one tree, and every suite passed because every suite
//!      iterated the doors and nothing read the prose.
//!   6. **The unswept path-text remainder** (M52 Increment 10, T10). `UNSWEPT_PRODUCERS`
//!      is the one bound M50's law-1 sweep declared rather than closed, and the two
//!      records that carry its size disagreed with each other **and** with the table —
//!      **79** in one place, **60** in two others, against a table that has summed
//!      neither since the day it was minted.
//!
//! **The `design/` part-docs are inside the fence from M52 Increment 10 on.** Until then
//! `HOMES` reached `CLAUDE.md`, `implementation/decisions-pending.md`, one design doc and
//! three test files, so a count a part-doc stated was unfenced prose by construction — the
//! gap `completions/artifacts/M52/gap-docs.md` → B11 found. A part-doc joins the list when
//! it states a fenced count; the ones that state none are not homes, and a subject with no
//! home at all is named as such rather than given an empty fence (`DESTROYING_DOORS`, whose
//! size this wave moved from four to six, is stated by no `design/` sentence — every design
//! mention of it is a *subset* claim or a singular, both of which this fence's predicate
//! deliberately ignores).
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
//! uses for a dozen different registries. [`RECORD`] therefore takes the explicit shapes
//! only: the project-state record is one ~156 000-character paragraph, so a unit-scoped rule
//! has no unit there — it would read every door count in the repo's history as a claim about
//! this axis. A count stated in some third wording is outside this probe.
//!
//! **The record's home moved on 2026-09-27, and four homes here moved with it.** Every
//! numeral this suite reads out of the project-state record — the committing-door axis, the
//! write-miss axis' two sizes, the unswept path-text remainder and `ROLLBACK_POPULATIONS`'
//! row count — was written inside that one paragraph, and the paragraph left `CLAUDE.md`
//! for [`RECORD`] byte-for-byte (`crates/cli/tests/foldback_truth.rs`' module doc carries
//! the move and its link rule). So each home is re-keyed onto the file the sentence is
//! actually in; none of the four facts is stated in the guidance `CLAUDE.md` retained, so
//! none of them stays.

use crate::support;
use cli::invocation_log::{COMMITTING_DOORS, ERROR_CODE_REGISTRY};
use cli::rollback::ROLLBACK_POPULATIONS;
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

/// The running project-state record — the home of every milestone fold-back, and the file
/// four of this suite's subjects state their size in. It left `CLAUDE.md` on 2026-09-27
/// (module doc, *The record's home moved*).
const RECORD: &str = "implementation/project-history.md";

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

struct Home {
    path: &'static str,
    scope: Scope,
}

/// **Every home that states the committing-door count**, enumerated rather than globbed on
/// `foldback_truth.rs`' reason: a sweep of every `.md` would reach the dated records, whose
/// job is to state the world as it was.
///
/// `MIGRATING.md` was **deferred, not exempt**, for the length of one increment: it is
/// `include_str!`'d into the installed `SKILL.md`, so any guide byte moves
/// `jigc-body-blake3` and engages M48's refuse-to-clobber path, and the decomposition
/// landed every guide byte this wave owed in **one** batch at Increment 9 T12
/// (`implementation/roadmap.md` → Milestone 51, Increment 9). That batch landed —
/// `MIGRATING.md` now states **ten** — so the deferral and its bookkeeping arm are
/// **discharged** and the guide is fenced live here, like every other home.
const HOMES: &[Home] = &[
    Home {
        path: RECORD,
        scope: Scope::ExplicitOnly,
    },
    Home {
        path: "implementation/decisions-pending.md",
        scope: Scope::ExplicitOnly,
    },
    Home {
        path: "design/worked-examples.md",
        scope: Scope::WithRegistryUnits,
    },
    Home {
        path: "crates/cli/tests/flow47_acceptance.rs",
        scope: Scope::WithRegistryUnits,
    },
    Home {
        path: "crates/cli/tests/commit_rejected_axis.rs",
        scope: Scope::WithRegistryUnits,
    },
    Home {
        path: "crates/cli/tests/pinned_facts.rs",
        scope: Scope::ExplicitOnly,
    },
    Home {
        path: "MIGRATING.md",
        scope: Scope::ExplicitOnly,
    },
    // The two `design/` part-docs that state this axis' size (M52 Increment 10, T10).
    // Both name the registry in the sentence that counts it, which is why the count is
    // followable at all — and why the self-naming shape (d) below reads them in any scope.
    Home {
        path: "design/command-output-contract.md",
        scope: Scope::WithRegistryUnits,
    },
    Home {
        path: "design/surface-contract.md",
        scope: Scope::WithRegistryUnits,
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
        // (d) `N COMMITTING_DOORS …` — the registry's own name as the noun's qualifier, so
        // the claim carries its subject and its list in one phrase. Self-naming, and
        // therefore read in every home, like (a) and (b).
        if end.is_none() {
            end = word_at(text, head, "COMMITTING_DOORS");
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
    assert_eq!(
        scan(
            "the ten `COMMITTING_DOORS` rejections move from `{error}`",
            Scope::ExplicitOnly,
        ),
        vec![10],
        "a numeral qualifying the registry's own name is self-naming — read in every home",
    );
    assert_eq!(
        scan(
            "`COMMITTING_DOORS` gained its tenth member at M49",
            Scope::ExplicitOnly
        ),
        Vec::<usize>::new(),
        "an ordinal naming one member is not a statement of the set's size",
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
const DEV_MANIFEST: &str = cli::pack_path!(dev, "config/schema-manifest.yaml");
/// The methodology pack's, which ends its freeze exemption (M40 A1).
const METHODOLOGY_MANIFEST: &str = cli::pack_path!(methodology, "config/schema-manifest.yaml");
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
const AXIS_CLAIM_HOME: &str = RECORD;

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
/// them. The record's M50 span (then still in `CLAUDE.md`) stated one numeral for both,
/// and it was neither:
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

// ---------------------------------------------------------------------------
// The record-only door set (M52 Increment 5, T11) — the third subject, same mold.
// ---------------------------------------------------------------------------
//
// G-4 found this one axis stated **three** different sizes inside one tree — four in
// `design/finalize.md`'s rollback row, five in the code, six in the Settle's own draft — and
// every suite passed, because every suite iterated the doors and nothing read the prose. The
// set is now `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row, so the prose is
// fenced against it here rather than re-counted by hand a fourth time.
//
// **The shape is strictly adjacent, and that is the discriminating half.** Only an
// unqualified `<N> record-only doors` is a claim about the whole set. *`four` **milestone**
// `record-only doors`* is a different and **true** claim — the four milestone ops share one
// `ConflictDoor` while a sub-task `jigc task discard` carries its own — so a fence that read a
// qualified form would report a defect at the one sentence that draws the split correctly
// (`design/validation.md` → The M52 registrations; `cli::rollback::MILESTONE_DOOR`).

/// The `milestone-record` population's doors — the enumeration `design/finalize.md` carried as
/// prose until this increment, and the subject every home below states the size of.
fn record_only_doors() -> &'static [&'static [&'static str]] {
    ROLLBACK_POPULATIONS
        .iter()
        .find(|row| row.id == "milestone-record")
        .map(|row| row.doors)
        .expect("`ROLLBACK_POPULATIONS` carries the `milestone-record` population")
}

/// Every design doc that states the size of the record-only door set, enumerated on [`HOMES`]'
/// reason: a sweep of every `.md` would reach the dated records, whose job is to state the
/// world as it was.
///
/// Each home must **also** name the registry, so the number a reader finds is followable to
/// the list rather than being a second list one edit away from disagreeing with it.
const RECORD_DOOR_HOMES: &[&str] = &[
    "design/finalize.md",
    "design/reconciliation.md",
    "design/team-ready-state.md",
];

/// The registry a [`RECORD_DOOR_HOMES`] member must point at.
const RECORD_DOOR_REGISTRY: &str = "ROLLBACK_POPULATIONS";

/// Every unqualified `<N> record-only doors` claim `text` states.
fn record_only_door_claims(text: &[char], map: &[usize]) -> Vec<Claim> {
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
        let end = word_at(text, skip_joiners(text, after_number), "record-only")
            .and_then(|after| word_at(text, skip_joiners(text, after), "doors"));
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

/// Every claim a record-only-door home states, partitioned exactly as [`claims_of`] does —
/// all of them, and the ones outside every dated correction bracket.
fn record_door_claims_of(path: &str) -> (Vec<Claim>, Vec<Claim>) {
    let body = read(path);
    let brackets = dated_correction_spans(&body);
    let mut all = Vec::new();
    for (start, end) in units(&body) {
        let (text, map) = normalize(&body[start..end]);
        let absolute: Vec<usize> = map.iter().map(|at| start + at).collect();
        all.extend(record_only_door_claims(&text, &absolute));
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

/// Every home that states the size of the record-only door set states the size the registry
/// carries — and names the registry, so the count is followable to the list.
#[test]
fn every_record_only_door_home_states_the_registrys_door_count() {
    let expected = record_only_doors().len();
    let mut stale = Vec::new();
    for path in RECORD_DOOR_HOMES {
        let body = read(path);
        let (all, uncorrected) = record_door_claims_of(path);
        assert!(
            !all.is_empty(),
            "{path} states no record-only-door count any more, in its own voice or in a dated \
             bracket — either the sentence moved (re-key the home) or the home stopped stating \
             it, in which case it is fencing nothing",
        );
        assert!(
            body.contains(RECORD_DOOR_REGISTRY),
            "{path} states the size of the record-only door set without naming \
             `cli::rollback::{RECORD_DOOR_REGISTRY}` — a count whose list a reader cannot reach \
             is the second list this fence exists to stop",
        );
        for claim in uncorrected.into_iter().filter(|c| c.value != expected) {
            stale.push(format!(
                "{path}:{} says `{}` — the `milestone-record` row carries {expected} doors",
                line_of(&body, claim.at),
                claim.phrase.trim(),
            ));
        }
    }
    assert!(
        stale.is_empty(),
        "the record-only door set is a code-side row (`cli::rollback::ROLLBACK_POPULATIONS` → \
         `milestone-record`) and these homes state a size it does not carry. A count that is \
         *historical* — what a past wave's cell said — takes a dated `[Corrected …]**` bracket \
         instead of today's number:\n{}",
        stale.join("\n"),
    );
}

/// The fence is a property of its predicate: it must **catch** a stale count, **spare** a
/// dated one, and **ignore** both the singular and the qualified sub-set claim.
#[test]
fn a_qualified_record_only_door_claim_is_not_a_claim_about_the_whole_set() {
    let scan = |text: &str| -> Vec<usize> {
        let (chars, map) = normalize(text);
        record_only_door_claims(&chars, &map)
            .into_iter()
            .map(|c| c.value)
            .collect()
    };

    assert_eq!(
        scan("the commit one of **the five record-only doors** lands is rejected"),
        vec![5],
        "the unqualified shape is the claim, emphasis and all",
    );
    assert_eq!(
        scan("the **four milestone record-only doors** raise `milestone.rollback-conflict`"),
        Vec::<usize>::new(),
        "the milestone ops are a real sub-set with their own shared code — a true claim, and \
         not a claim about the size of the whole set",
    );
    assert_eq!(
        scan("a record-only door captures the record's pre-image before writing"),
        Vec::<usize>::new(),
        "the singular is never a statement of the set's size",
    );
    assert_eq!(
        scan("the write path reaches all 25 doors of the corpus"),
        Vec::<usize>::new(),
        "another registry's door count is not this set's",
    );
}

// ---------------------------------------------------------------------------
// The unswept path-text remainder (M52 Increment 10, T10) — the fourth subject.
// ---------------------------------------------------------------------------
//
// `UNSWEPT_PRODUCERS` is the bound M50's law-1 sweep **declared** rather than closed: the
// production `.display()` sites, file by file, that still compose a host path, each with the
// reason it is not in `GUARDED_SRC` yet. `repo_relative_paths.rs` already holds every *row*
// to its own source (`the_unswept_remainder_is_counted_not_described`) — what nothing held
// was the **prose** that states the table's size, and the three sentences that state it
// disagreed with the table and with each other.
//
// **The scoping rule is proximity, not the prose unit.** The project-state record is one
// ~156 000-character paragraph, so a unit-scoped rule has no unit there — and a bare
// `<N> producers` is a shape this repo writes about a dozen different producer sets (the same
// bullet that states this bound also says *"the class is **three** producers, not one"* about
// an entirely different one). So a numeral is read as a claim about **this** table only when
// the table's own name stands within [`UNSWEPT_NAMING_WINDOW`] characters after the noun it
// governs. That is also the shape a reader needs: a count whose list is not named beside it
// is the second list this whole file exists to stop.

/// The file that declares the remainder table. It is a `const` private to another test group
/// (`g_surface`), so it is counted from its own source — the same declared bound the
/// write-miss cross carries above.
const UNSWEPT_TABLE: &str = "crates/cli/tests/repo_relative_paths.rs";

/// The table's name, which a home must state beside its numeral.
const UNSWEPT_REGISTRY: &str = "UNSWEPT_PRODUCERS";

/// How far after the counted noun the table's name may stand and still be its subject.
const UNSWEPT_NAMING_WINDOW: usize = 80;

/// Every home that states the size of the remainder, enumerated on [`HOMES`]' reason.
///
/// `DECISIONS.md` and `implementation/roadmap.md` name the table and are deliberately **not**
/// homes: they are dated records, whose job is to state the world as it was on the day of the
/// entry. [`RECORD`] is a home because its fold-back speaks in the present tense about what
/// a wave left standing — and when that is a past measurement, the dated-bracket rule below
/// is how it keeps saying so.
const UNSWEPT_HOMES: &[&str] = &[RECORD, "implementation/decisions-pending.md"];

/// Which of the table's two sizes a claim counts.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UnsweptSubject {
    /// The production `.display()` sites, summed over every row.
    Producers,
    /// The rows themselves — one per file.
    Files,
}

impl UnsweptSubject {
    fn noun(self) -> &'static str {
        match self {
            UnsweptSubject::Producers => "producers",
            UnsweptSubject::Files => "files",
        }
    }
}

/// `(file, production sites)` for every row of `UNSWEPT_PRODUCERS`, in declaration order.
///
/// Read as lines for [`manifest_types`]' reason: the rows are a hand-edited literal whose
/// path-and-count columns are exactly the thing being counted, and parsing Rust to read two
/// fields would buy nothing the source text does not already say.
fn unswept_rows() -> Vec<(String, usize)> {
    let body = read(UNSWEPT_TABLE);
    let at = body
        .find(&format!("\nconst {UNSWEPT_REGISTRY}:"))
        .unwrap_or_else(|| panic!("{UNSWEPT_TABLE} must declare `{UNSWEPT_REGISTRY}`"));
    let mut rows = Vec::new();
    let mut file: Option<String> = None;
    for line in body[at..].lines().skip(1).take_while(|line| *line != "];") {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('"')
            && let Some(name) = rest.strip_suffix("\",")
            && name.starts_with("crates/")
        {
            file = Some(name.to_string());
        } else if let Some(count) = trimmed.strip_suffix(',')
            && let Ok(count) = count.parse::<usize>()
            && let Some(file) = file.take()
        {
            rows.push((file, count));
        }
    }
    assert!(
        !rows.is_empty(),
        "{UNSWEPT_TABLE}: `{UNSWEPT_REGISTRY}` parsed as empty — the reader is keyed on the \
         literal's shape, so a reformat of the table re-keys this function",
    );
    rows
}

/// Every claim about the remainder's size `text` states — a numeral governing `producers` or
/// `files`, with [`UNSWEPT_REGISTRY`] standing within [`UNSWEPT_NAMING_WINDOW`] characters
/// after that noun.
///
/// Up to three words may sit between the numeral and its noun (*60 remaining producers*), and
/// the plural is required throughout, on [`claims`]' reason: a singular is never a statement
/// of a set's size.
fn unswept_claims(text: &[char], map: &[usize]) -> Vec<(Claim, UnsweptSubject)> {
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
        let Some((end, subject)) = governed_noun(text, after_number) else {
            at = after_number;
            continue;
        };
        let window: String = text[end..text.len().min(end + UNSWEPT_NAMING_WINDOW)]
            .iter()
            .collect();
        if !window.contains(UNSWEPT_REGISTRY) {
            at = end;
            continue;
        }
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

/// The counted noun a numeral ending at `from` governs, with the byte it ends at — scanning
/// past at most three intervening adjective words, and never past a second numeral.
fn governed_noun(text: &[char], from: usize) -> Option<(usize, UnsweptSubject)> {
    let mut at = from;
    for _ in 0..4 {
        at = skip_joiners(text, at);
        for subject in [UnsweptSubject::Producers, UnsweptSubject::Files] {
            if let Some(end) = word_at(text, at, subject.noun()) {
                return Some((end, subject));
            }
        }
        let word_end = (at..text.len())
            .find(|i| !text[*i].is_alphabetic() && text[*i] != '-')
            .unwrap_or(text.len());
        if word_end == at {
            return None;
        }
        at = word_end;
    }
    None
}

/// A home's remainder claims, each carrying the size it counts.
type UnsweptClaims = Vec<(Claim, UnsweptSubject)>;

/// Every claim a remainder home states, partitioned exactly as [`claims_of`] does.
fn unswept_claims_of(path: &str) -> (UnsweptClaims, UnsweptClaims) {
    let body = read(path);
    let brackets = dated_correction_spans(&body);
    let mut all = Vec::new();
    for (start, end) in units(&body) {
        let (text, map) = normalize(&body[start..end]);
        let absolute: Vec<usize> = map.iter().map(|at| start + at).collect();
        all.extend(unswept_claims(&text, &absolute));
    }
    let uncorrected = all
        .iter()
        .filter(|(claim, _)| {
            !brackets
                .iter()
                .any(|(s, e)| claim.at >= *s && claim.at < *e)
        })
        .cloned()
        .collect();
    (all, uncorrected)
}

/// Every home that states the size of the unswept remainder states the size the table
/// carries — its production-site total and its row count both.
#[test]
fn every_unswept_remainder_home_states_the_tables_own_size() {
    let rows = unswept_rows();
    let producers: usize = rows.iter().map(|(_, count)| count).sum();
    let files = rows.len();

    let mut stale = Vec::new();
    for path in UNSWEPT_HOMES {
        let body = read(path);
        let (all, uncorrected) = unswept_claims_of(path);
        assert!(
            !all.is_empty(),
            "{path} states no size of the unswept remainder any more, in its own voice or in \
             a dated bracket — either the sentence moved (re-key the home) or it stopped \
             naming `{UNSWEPT_REGISTRY}` beside its numeral, in which case the count is no \
             longer followable to the list and this fence is reading nothing",
        );
        for (claim, subject) in uncorrected {
            let expected = match subject {
                UnsweptSubject::Producers => producers,
                UnsweptSubject::Files => files,
            };
            if claim.value != expected {
                stale.push(format!(
                    "{path}:{} says `{}` — the table carries {expected} {} ({})",
                    line_of(&body, claim.at),
                    claim.phrase.trim(),
                    subject.noun(),
                    number_word(expected),
                ));
            }
        }
    }
    assert!(
        stale.is_empty(),
        "the unswept remainder is a code-side table (`{UNSWEPT_TABLE}`'s \
         `{UNSWEPT_REGISTRY}`) carrying {producers} production sites across {files} files, \
         and these homes state a size it does not carry. A count that is *historical* — what \
         a past wave left standing — takes a dated `[Corrected …]**` bracket instead of \
         today's number, so the record keeps stating its own measurement:\n{}",
        stale.join("\n"),
    );
}

/// The fence is a property of its predicate: the table's name must stand beside the numeral,
/// the plural is required, and another producer set's count is not this one's.
#[test]
fn an_unswept_claim_is_read_only_where_the_table_is_named_beside_it() {
    let scan = |text: &str| -> Vec<(usize, UnsweptSubject)> {
        let (chars, map) = normalize(text);
        unswept_claims(&chars, &map)
            .into_iter()
            .map(|(claim, subject)| (claim.value, subject))
            .collect()
    };

    assert_eq!(
        scan("**74 producers across ten files** are countable in `UNSWEPT_PRODUCERS` today"),
        vec![(74, UnsweptSubject::Producers), (10, UnsweptSubject::Files)],
        "both sizes are read, emphasis and all, when the table stands beside them",
    );
    assert_eq!(
        scan("with 60 remaining producers left countable in an `UNSWEPT_PRODUCERS` table"),
        vec![(60, UnsweptSubject::Producers)],
        "an adjective between the numeral and its noun does not hide the claim",
    );
    assert_eq!(
        scan("driven at T2 the class is **three** producers, not one — `store.not-found`"),
        Vec::<(usize, UnsweptSubject)>::new(),
        "another producer set's count is not this table's — without the table named beside \
         it, a bare `<N> producers` is not a claim about this bound",
    );
    assert_eq!(
        scan("one producer in `UNSWEPT_PRODUCERS` still composes a host path"),
        Vec::<(usize, UnsweptSubject)>::new(),
        "the singular is never a statement of the set's size",
    );
    assert_eq!(
        scan("79 producers across 11 files, each mechanically counted in `UNSWEPT_PRODUCERS`"),
        vec![(79, UnsweptSubject::Producers), (11, UnsweptSubject::Files)],
        "the window reaches across the whole claim, not only its last noun",
    );
}

// ---------------------------------------------------------------------------
// B11's remaining two counts (M52 Increment 10, T10) — the reject arms, and the
// worktree-shaped doors that refuse.
// ---------------------------------------------------------------------------
//
// `completions/artifacts/M52/gap-docs.md` → B11 listed seven counts this wave would move and
// no `design/` part-doc inside the fence to catch any of them. Three closed in earlier
// increments (the record-only door set above, the posture family in
// `crates/cli/tests/posture_member_inventory.rs`, `reconciliation.md`'s *exactly four*
// re-baselining sites, struck); a fourth — the **four `doc show` projections** — was a
// *code-side* claim (`ArmShape::ArrayOf`'s doc-comment: *"`jigc task list` is the surface's
// one array"*) against a locked doc that was right, and it is closed in the code rather than
// fenced in prose: the comment now names the two array shapes `doc show` answers with, so
// there is no prose count left to hold. Three were left: the unswept remainder above, and
// these two — both **true** at HEAD and held by nothing, which is the state one edit away
// from B1/B2.
//
// **A phrase, not a loose shape.** Each is read as a numeral governing a fixed word sequence
// — *`<N>` reject arms*, *`<N>` doors standing where* — because both sets have a name the
// prose already uses, and a phrase specific enough to name its own subject needs no unit or
// proximity scoping (the bound [`claims`] declares for *doors*).

/// Every `<numeral> <phrase>` claim `text` states, the phrase matched as whole words joined
/// by runs of spaces and hyphens. The plural lives in the caller's phrase, on [`claims`]'
/// reason: a singular is never a statement of a set's size.
fn phrase_claims(text: &[char], map: &[usize], phrase: &[&str]) -> Vec<Claim> {
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
        let mut cursor = Some(after_number);
        for word in phrase {
            cursor = cursor.and_then(|from| word_at(text, skip_joiners(text, from), word));
        }
        let Some(end) = cursor else {
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

/// Every claim a home states for `phrase`, partitioned exactly as [`claims_of`] does — all of
/// them, and the ones outside every dated correction bracket.
fn phrase_claims_of(path: &str, phrase: &[&str]) -> (Vec<Claim>, Vec<Claim>) {
    let body = read(path);
    let brackets = dated_correction_spans(&body);
    let mut all = Vec::new();
    for (start, end) in units(&body) {
        let (text, map) = normalize(&body[start..end]);
        let absolute: Vec<usize> = map.iter().map(|at| start + at).collect();
        all.extend(phrase_claims(&text, &absolute, phrase));
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

/// Assert every home states `expected` for `phrase`, and that each still states it at all.
fn assert_phrase_count(homes: &[&str], phrase: &[&str], expected: usize, subject: &str) {
    let spelled = phrase.join(" ");
    let mut stale = Vec::new();
    for path in homes {
        let body = read(path);
        let (all, uncorrected) = phrase_claims_of(path, phrase);
        assert!(
            !all.is_empty(),
            "{path} states no `<N> {spelled}` count any more, in its own voice or in a dated \
             bracket — either the sentence moved (re-key the home) or the home stopped \
             stating it, in which case it is fencing nothing",
        );
        for claim in uncorrected.into_iter().filter(|c| c.value != expected) {
            stale.push(format!(
                "{path}:{} says `{}` — {subject} carries {expected} ({})",
                line_of(&body, claim.at),
                claim.phrase.trim(),
                number_word(expected),
            ));
        }
    }
    assert!(
        stale.is_empty(),
        "{subject} is a code-side set and these homes state a size it does not carry. A count \
         that is *historical* takes a dated `[Corrected …]**` bracket instead of today's \
         number:\n{}",
        stale.join("\n"),
    );
}

/// The homes that state how many reject arms the envelope has.
const REJECT_ARM_HOMES: &[&str] = &[
    "design/command-output-contract.md",
    "design/surface-contract.md",
    "design/validation.md",
    "implementation/decisions-pending.md",
];

/// The doc of record for the arms — the one home the followability rule binds.
///
/// It is deliberately **not** every home: *cross-reference, never restate* (CLAUDE.md) puts
/// the declaration in one place and lets the others cite its size, so requiring all four to
/// name the registry would be asking for the second list this file exists to stop.
const REJECT_ARM_DECLARING_HOME: &str = "design/command-output-contract.md";

/// The **two reject arms** are the rows of `cli::render::ENVELOPE_ARMS` that no single leaf
/// owns, and M52 Increment 1 made that count true by deleting the third shape the binary
/// emitted and this doc never declared (`render::setup_block`'s bare root `Finding`). True
/// and held by nothing until here: the number is a hand count of a table `cli` exports.
#[test]
fn the_reject_arm_count_is_the_envelopes_own() {
    let arms = cli::render::ENVELOPE_ARMS
        .iter()
        .filter(|arm| arm.arm.starts_with("Reject::"))
        .count();
    assert!(
        read(REJECT_ARM_DECLARING_HOME).contains("ENVELOPE_ARMS"),
        "{REJECT_ARM_DECLARING_HOME} declares the reject arms and must name \
         `cli::render::ENVELOPE_ARMS`, so the count is followable to the rows",
    );
    assert_phrase_count(
        REJECT_ARM_HOMES,
        &["reject", "arms"],
        arms,
        "the cross-cutting reject arms of `cli::render::ENVELOPE_ARMS`",
    );
}

/// The home that states how many worktree-shaped doors refuse.
const WORKTREE_REFUSAL_HOME: &str = "design/team-ready-state.md";

/// The **worktree-shaped doors that refuse** — `cli::milestone::WORKTREE_DOORS` filtered on
/// its own disposition, which is how the design doc scopes the claim: *every door that
/// removes a worktree-shaped path under `.jigc/worktrees/`* answers for what it removes, and
/// the ones standing where no commit has carried those bytes refuse.
///
/// The subset is load-bearing and is why this reads `WORKTREE_DOORS` rather than
/// `DESTROYING_DOORS`: the registry carries six members since M52 Increment 4, of which four
/// stand at a worktree path and three of those refuse — so a fence keyed on the whole table
/// would report a defect at the one sentence that draws the split correctly.
#[test]
fn the_refusing_worktree_door_count_is_the_registrys_own() {
    let refusing = cli::milestone::WORKTREE_DOORS
        .iter()
        .filter(|door| matches!(door.disposition, cli::milestone::Disposition::Refuse { .. }))
        .count();
    assert!(
        read(WORKTREE_REFUSAL_HOME).contains("WORKTREE_DOORS"),
        "{WORKTREE_REFUSAL_HOME} states how many worktree-shaped doors refuse and must name \
         `cli::milestone::WORKTREE_DOORS`, so the count is followable to the list — the \
         enclosing sentence already scopes itself to that subset in prose",
    );
    assert_phrase_count(
        &[WORKTREE_REFUSAL_HOME],
        &["doors", "standing", "where"],
        refusing,
        "the refusing members of `cli::milestone::WORKTREE_DOORS`",
    );
}

// ---------------------------------------------------------------------------
// The rollback registry's own row count (M52 Increment 11 fix) — the same class one
// layer out: the registry other fences here read, unfenced about its own size.
// ---------------------------------------------------------------------------
//
// `cli::rollback::ROLLBACK_POPULATIONS` is the registry two of the fences above already
// read — and nothing read the prose that states **its** size. It was minted **twelve**
// rows at M52 Increment 5 / T1 and became **eleven** at T9 (`9720620e`), when
// `rename-head-restore` was deleted rather than re-worded: one discipline is one row, so
// `rollback_rename`'s HEAD-sourced arm folded back into the door's single population. The
// source header carries that strike with its arithmetic (`crates/cli/src/rollback.rs` →
// *The row count is eleven*: 9 + 1 + 1, every addend cited), and both
// `crates/cli/tests/rollback_population_registry.rs` and flow 53's arm 1 iterate the rows
// — so **every suite stayed green while this wave's own fold-back, written two days
// later, still said twelve**, in the one paragraph a reader of this repo reads first.
//
// **The phrase names its own subject**, so this needs neither a prose unit nor a proximity
// window: `<N> ROLLBACK_POPULATIONS rows` carries the numeral and the list in one breath,
// which is also the followability the homes above have to be asserted into separately. A
// sub-set's row count (*both `MintedSet` rows*, *the two rows that have no restore site*)
// is deliberately unreadable by it — the phrase, not the noun, is the subject.

/// The home that states the registry's size in the **present tense**.
///
/// `DECISIONS.md` and `implementation/roadmap.md` state it too and are deliberately **not**
/// homes, on [`UNSWEPT_HOMES`]' reason: they are dated records, and a superseded figure
/// stays visible there beside the correction that superseded it — which is exactly the
/// chain the roadmap's Increment 5 row now carries (ten → twelve → eleven).
const ROLLBACK_ROW_HOME: &str = RECORD;

/// The registry's row count is the registry's own.
#[test]
fn the_rollback_population_row_count_is_the_registrys_own() {
    assert_phrase_count(
        &[ROLLBACK_ROW_HOME],
        &["ROLLBACK_POPULATIONS", "rows"],
        ROLLBACK_POPULATIONS.len(),
        "`cli::rollback::ROLLBACK_POPULATIONS`",
    );
}

/// The three phrase fences are properties of one predicate: whole words, a numeral in
/// either spelling, and no match without one.
#[test]
fn a_phrase_claim_needs_its_numeral_and_its_whole_phrase() {
    let scan = |text: &str, phrase: &[&str]| -> Vec<usize> {
        let (chars, map) = normalize(text);
        phrase_claims(&chars, &map, phrase)
            .into_iter()
            .map(|c| c.value)
            .collect()
    };

    assert_eq!(
        scan(
            "**The two reject arms** — the only rows no single leaf owns",
            &["reject", "arms"]
        ),
        vec![2],
        "the phrase is read through emphasis, like every other shape here",
    );
    assert_eq!(
        scan(
            "both reject arms are pinned and neither moved a key",
            &["reject", "arms"]
        ),
        Vec::<usize>::new(),
        "`both` is not a numeral — a fence that read it would have to guess a size",
    );
    assert_eq!(
        scan("the cross-cutting reject arms", &["reject", "arms"]),
        Vec::<usize>::new(),
        "without a numeral there is no count to check",
    );
    assert_eq!(
        scan(
            "The three doors standing where no commit has carried those bytes refuse",
            &["doors", "standing", "where"],
        ),
        vec![3],
        "the whole word sequence is what names the subject",
    );
    assert_eq!(
        scan(
            "the four doors this table carries",
            &["doors", "standing", "where"]
        ),
        Vec::<usize>::new(),
        "another door count is not this one — the phrase, not the noun, is the subject",
    );
    assert_eq!(
        scan(
            "twelve `ROLLBACK_POPULATIONS` rows — `FileCas` · `MintedSet` · `DoorGuard`",
            &["ROLLBACK_POPULATIONS", "rows"],
        ),
        vec![12],
        "the registry's own name inside the phrase is what makes the claim self-naming, \
         code ticks and all — this is the sentence that shipped stale",
    );
    assert_eq!(
        scan(
            "both `MintedSet` rows, and the two rows that have no restore site at all",
            &["ROLLBACK_POPULATIONS", "rows"],
        ),
        Vec::<usize>::new(),
        "a sub-set's row count is not the registry's size",
    );
}
