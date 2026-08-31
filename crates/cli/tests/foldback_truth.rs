//! The fold-back's own fence — the three outward-facing docs state the world the
//! wave actually built (M47 Increment 11, T7).
//!
//! Three claims, each asserted against the shipped file rather than trusted:
//!
//!   1. **MIGRATING.md carries the hook-rejection gate.** The wave swept the
//!      survivable-rejection frame across the whole nine-door committing axis
//!      ([design/finalize.md](../../../design/finalize.md) → 6. Commit, *the frame is the
//!      whole committing family's*; `DECISIONS.md` → 2026-08-03 M47 Inc 3 T7), and no
//!      user-facing doc said so: a human meeting a rejecting `pre-commit` hook could not
//!      tell a recoverable rejection from a destroyed task. *Reconciling and backing out*
//!      is the chapter that ladder lives in, so the gate joins it — **in meeting order**,
//!      which is what the contiguity check below fences: the ladder is numbered
//!      `1..n` with no gap, so inserting one gate cannot silently leave two `5.`s.
//!   2. **QUICKSTART.md cross-refs it.** The quickstart's finalize section is where a
//!      cold reader meets the commit boundary; it points at the gate rather than
//!      restating it (CLAUDE.md → *Cross-reference, never restate*).
//!   3. **CLAUDE.md's project-state paragraph names the wave whose claim is still moving,
//!      and claims exactly what has been reached — no more, and no less.** At a close
//!      increment the wave is **built, not audited**, and the fence requires those words
//!      while forbidding a `VERDICT` citation: the doc may not claim a verdict nobody has
//!      reached. When the audit lands the fence goes red — which is the fence working, not
//!      failing — and it **inverts** rather than relaxing: the stale bound becomes itself
//!      the law-1 lie, the completed claim is required, and the cited verdict artifact must
//!      actually exist. It has run in both directions at every wave since M47, and at each
//!      new wave's close it is **re-aimed rather than duplicated**, so the suite carries one
//!      live pin rather than one dead pin per wave — it now points at M49.
//!
//! A fourth claim joined at M48 Increment 12 (T1):
//!
//!   4. **The pack-step count is stated once, and says which binary it was measured on.**
//!      The pre-1.0 trial's headline discoverability fact — *of N pack step files exactly
//!      one names `jigc doc show`, and it is not an authoring step* — reached **eleven
//!      sites across seven files** as a bare `69`, and 69 was never the number. It is what
//!      the finding's own repro command printed: `ls crates/cli/pack/steps/
//!      packs/methodology/steps/ | wc -l` counts `ls`'s two directory headers and the
//!      blank line between them, so a tree of **66** step `*.yaml` — what the trial's HEAD
//!      `8979f16` carried, and therefore what 1.0.0-rc.10 shipped — reported 66 + 3. The
//!      finding survives its denominator intact; only the denominator was wrong.
//!
//!      A bare count also re-falsifies itself on the next step file the pack gains (HEAD
//!      is already **67**), so the surviving statement carries its **measurement point** —
//!      the pre-1.0.0 trial's 1.0.0-rc.10 — and lives in **one** home: the rc.11 charter
//!      row in `implementation/decisions-pending.md`. The live docs that restated it now
//!      cross-reference that home instead (CLAUDE.md → *Cross-reference, never restate*).
//!
//!      The **dated records** — `DECISIONS.md` and the two `RC-pre-1.0/` artifacts — are
//!      not rewritten and do not outsource their numbers: a trial record that stated
//!      someone else's measurement would stop being a record. They keep their own figure,
//!      corrected, inside the file's own **dated correction bracket** (`DECISIONS.md`'s
//!      `[Corrected <date> …]` convention), because the reader must see the basis *and*
//!      what falsified it — and that bracket names the home, so landing on the record
//!      still reaches the count that is current.
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The
//! behaviour the prose describes is proven elsewhere, through the real binary:
//! `crates/cli/tests/commit_rejected_axis.rs` drives every committing door under a
//! rejecting hook and re-runs the argv it printed.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read_doc(name: &str) -> String {
    let path = repo_root().join(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("{name} must exist at the repo root: {path:?}"))
}

/// The body of a `## <heading>` section — up to the next `## ` heading or EOF.
fn section<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("## {heading}\n");
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

/// The numbered gates of a `1. `-style ladder, in file order, as `(number, text)`.
/// A gate's text runs to the next numbered line, so a wrapped gate is read whole.
fn numbered_items(section_body: &str) -> Vec<(usize, String)> {
    let mut items: Vec<(usize, String)> = Vec::new();
    for line in section_body.lines() {
        let numbered = line
            .split_once(". ")
            .and_then(|(n, rest)| n.parse::<usize>().ok().map(|n| (n, rest.to_string())));
        match numbered {
            Some((n, rest)) => items.push((n, rest)),
            None => {
                if let Some((_, text)) = items.last_mut() {
                    text.push('\n');
                    text.push_str(line);
                }
            }
        }
    }
    items
}

#[test]
fn migrating_carries_the_hook_rejection_gate_in_meeting_order() {
    let body = read_doc("MIGRATING.md");
    let chapter = section(&body, "Reconciling and backing out");
    let gates = numbered_items(chapter);
    assert!(
        !gates.is_empty(),
        "*Reconciling and backing out* must be a numbered ladder of gates",
    );

    // The ladder is `1..n` with no gap or repeat — inserting a gate renumbers the rest.
    let numbering: Vec<usize> = gates.iter().map(|(n, _)| *n).collect();
    let expected: Vec<usize> = (1..=gates.len()).collect();
    assert_eq!(
        numbering, expected,
        "the gates must be numbered 1..n in meeting order, with no gap or repeat",
    );

    let hook_gates: Vec<&(usize, String)> = gates
        .iter()
        .filter(|(_, text)| {
            let lower = text.to_lowercase();
            lower.contains("hook") && lower.contains("reject")
        })
        .collect();
    assert_eq!(
        hook_gates.len(),
        1,
        "exactly one gate must own the hook rejection; found {} in:\n{chapter}",
        hook_gates.len(),
    );

    let (_, gate) = hook_gates[0];
    let lower = gate.to_lowercase();
    for owed in [
        // the recovery, named as the action the reader takes
        "re-run",
        // the two doors whose surviving state differs — the frame is per-door
        "jigc task finalize",
        "jigc migrate-corpus",
    ] {
        assert!(
            lower.contains(&owed.to_lowercase()),
            "the hook-rejection gate must name `{owed}`; gate reads:\n{gate}",
        );
    }
    assert!(
        lower.contains("no commit") || lower.contains("nothing was committed"),
        "the hook-rejection gate must say no commit was made; gate reads:\n{gate}",
    );
}

#[test]
fn quickstart_cross_refs_the_hook_rejection_gate() {
    let body = read_doc("QUICKSTART.md");
    let lower = body.to_lowercase();
    assert!(
        lower.contains("hook") && lower.contains("reject"),
        "QUICKSTART.md's finalize section must name a hook that rejects the commit",
    );
    assert!(
        body.contains("[MIGRATING.md](MIGRATING.md) → Reconciling and backing out"),
        "QUICKSTART.md must cross-ref MIGRATING.md → Reconciling and backing out \
         rather than restating the ladder",
    );
}

/// The span of one milestone's claim inside the project-state paragraph: from its
/// `**M<nn> —` marker to the next bolded milestone marker (or the paragraph's end).
fn milestone_span<'a>(body: &'a str, marker: &str) -> &'a str {
    let start = body
        .find(marker)
        .unwrap_or_else(|| panic!("CLAUDE.md's project state must carry `{marker}`"));
    let rest = &body[start + marker.len()..];
    let end = rest
        .match_indices("**M")
        .find(|(i, _)| {
            rest[i + 3..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit())
        })
        .map(|(i, _)| i)
        .unwrap_or(rest.len());
    &rest[..end]
}

/// **The fence follows the wave whose claim is still moving.** It was written at M47
/// Increment 11's fold-back over `**M47 —`, requiring the bound *built, not audited* and
/// forbidding a verdict citation; when M47's audit landed it went red, which is the fence
/// working rather than failing, and it was reconciled to the inverse direction — a
/// completed wave cites what its audit found and may not say *audited clean* over three LOW
/// findings. Both directions are the same rule: **the paragraph may say no more about the
/// audit than the audit found.** M48 moved it forward, then flipped it again at that
/// wave's completion fold-back; M46 moved it forward at its Increment 10 close, and M49
/// Increment 1 flipped it, one wave late, when M46's audit had already run.
///
/// At M49 Increment 12 it moves forward once more, to the wave this build closes. M46's
/// claim is settled prose now — its audit ran, its verdict is persisted, and nothing in
/// this build can move it — while M49's claim is the one a fold-back can overstate, and the
/// overstatement available *today* is the premature one: the completion audit, its persisted
/// verdict and the version bump + install are the milestone-completion workflow's next acts
/// ([roadmap.md](../../../implementation/roadmap.md) → Milestone 49 Increment 12, whose
/// grouped scope says *built and installed after the audit fixes, not before*). So the arm
/// is **re-aimed, not duplicated**, a third time: the assertions invert back to their
/// pre-audit direction and follow the marker, rather than accumulating one dead pin per
/// wave.
///
/// The owed **version bump** is deliberately outside this fence. It is discharged as a
/// named obligation rather than performed here, and it carries **no numeral**: the roadmap's
/// M49 section names no target version and the 1.0.0 call is the human's, so asserting a
/// version string would be this fence choosing it.
#[test]
fn claude_md_names_m49_and_claims_only_the_build() {
    let body = read_doc("CLAUDE.md");
    // The project-state paragraph is a single line; the sections that follow it (build /
    // lint / test, quickstart, code architecture) are not milestone claims, and M49 is the
    // last marker in the paragraph — so the span is bounded at the paragraph's own end
    // rather than running to EOF and forbidding these words to the whole file.
    let span = milestone_span(&body, "**M49 —")
        .split('\n')
        .next()
        .expect("splitting a str always yields at least one part");

    for owed in [
        "implementation/roadmap.md",
        "Milestone 49",
        "flow 50",
        "flow50_acceptance.rs",
    ] {
        assert!(
            span.contains(owed),
            "the M49 project-state claim must name `{owed}`; it reads:\n{span}",
        );
    }

    // The bound is stated, not merely implied by an absence: a reader must be able to see
    // that the audit is owed, and a paragraph that simply omits the word cannot say so.
    assert!(
        span.contains("built, not audited"),
        "M49 is built and not yet audited, and the claim must say so in those words:\n{span}",
    );
    for forbidden in [
        // No audit has run, so every one of these claims a verdict nobody reached.
        "audited clean",
        "audit is CLEAN",
        "audit ran clean",
        // The verdict artifact does not exist yet; a link to one would be a law-1 lie.
        "VERDICT",
    ] {
        assert!(
            !span.contains(forbidden),
            "the M49 claim may not claim an audit that has not run, but contains \
             `{forbidden}`:\n{span}",
        );
    }
}

// ---------------------------------------------------------------------------
// 4. The pack-step count — one home, and it names its measurement point.
// ---------------------------------------------------------------------------

/// Every home the pre-1.0 trial's pack-step-count claim reached, enumerated from the
/// M48 Settle's own site list plus `implementation/roadmap.md`, which was already
/// correct and must stay so. Enumerated rather than globbed: a glob over the repo
/// would sweep in the goldens and this suite's own prose.
const STEP_COUNT_HOMES: [&str; 8] = [
    "implementation/decisions-pending.md",
    "CLAUDE.md",
    "design/surface-contract.md",
    "DECISIONS.md",
    "implementation/roadmap.md",
    "completions/artifacts/M48/handover.md",
    "completions/artifacts/RC-pre-1.0/trial-record.md",
    "completions/artifacts/RC-pre-1.0/findings-verification.md",
];

/// The single home the count is allowed to live in — the rc.11 charter row.
const STEP_COUNT_HOME: &str = "implementation/decisions-pending.md";

/// The **live** docs that restated F1's count and now cross-reference the charter
/// instead — present-tense prose, which is where *cross-reference, never restate* binds.
///
/// `implementation/roadmap.md` is deliberately not here: its M48 row is the milestone's
/// own claim, already stated at 66 before this task, and owned by the record repair (T6).
const STEP_COUNT_RESTATEMENTS: [&str; 3] = [
    "CLAUDE.md",
    "design/surface-contract.md",
    "completions/artifacts/M48/handover.md",
];

/// The **dated records**, which state their own measurement rather than pointing at
/// someone else's — a trial record that outsourced its numbers would stop being a record.
/// They carry the correction instead: a dated bracket that keeps the falsified figure
/// visible beside what falsified it, and names the home the live count now lives in.
///
/// `DECISIONS.md` is a record in the same sense, and also carries unrelated step-file
/// censuses in its build entries (F1's owe-set *30 of 66*, the batch-author-note's *13*)
/// that a blanket no-count rule would forbid for no gain.
const STEP_COUNT_RECORDS: [&str; 3] = [
    "DECISIONS.md",
    "completions/artifacts/RC-pre-1.0/trial-record.md",
    "completions/artifacts/RC-pre-1.0/findings-verification.md",
];

/// The smallest byte index `>= i` that is a char boundary of `s` (or `s.len()`).
fn char_ceil(s: &str, mut i: usize) -> usize {
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i.min(s.len())
}

/// Byte spans of the file's **dated** correction brackets — `[Corrected YYYY-MM-DD …]**`,
/// the convention `DECISIONS.md` uses to keep a falsified basis visible beside what
/// falsified it. An undated `[Corrected …]` is not one: the date is the whole point.
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
        if !dated {
            continue;
        }
        if let Some(close) = rest.find("]**") {
            spans.push((start, after + close + "]**".len()));
        }
    }
    spans
}

/// Every `(offset, value)` at which `body` states a number as a count of pack **step
/// files**.
///
/// Tight by construction: the number must govern a `step file(s)` phrase within a short
/// reach that crosses no other number. That admits every shape the claim was written in
/// (`of 69 pack step files`, `1 of 69 step files`, `(1/69 step files)`, `**66** pack step
/// files`) while excluding the many `…md:69` *line* references the record also carries.
fn step_count_claims(body: &str) -> Vec<(usize, usize)> {
    let bytes = body.as_bytes();
    let mut hits = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        // Not the line half of a `path.md:69` / `1.69` reference.
        if body[..start]
            .chars()
            .next_back()
            .is_some_and(|c| c == ':' || c == '.')
        {
            continue;
        }
        let tail = &body[i..];
        let window = tail[..char_ceil(tail, 40.min(tail.len()))].to_lowercase();
        let Some(phrase) = window.find("step file") else {
            continue;
        };
        if window[..phrase].chars().any(|c| c.is_ascii_digit()) {
            continue;
        }
        if let Ok(value) = body[start..i].parse::<usize>() {
            hits.push((start, value));
        }
    }
    hits
}

/// The count claims of `body` that sit outside every dated correction bracket — the
/// statements the file makes in its own present-tense voice.
fn uncorrected_step_count_claims(body: &str) -> Vec<(usize, usize)> {
    let brackets = dated_correction_spans(body);
    step_count_claims(body)
        .into_iter()
        .filter(|(at, _)| !brackets.iter().any(|(s, e)| at >= s && at < e))
        .collect()
}

fn line_of(body: &str, at: usize) -> usize {
    body[..at].matches('\n').count() + 1
}

/// Whether `text` cross-references the one home the live count lives in. Both halves are
/// required: the file alone is a big document, the heading alone is not addressable.
fn points_at_the_home(text: &str) -> bool {
    text.contains("decisions-pending.md") && text.contains("The rc.11 wave")
}

#[test]
fn the_stale_pack_step_count_survives_only_inside_a_dated_correction() {
    let mut bare = Vec::new();
    for home in STEP_COUNT_HOMES {
        let body = read_doc(home);
        for (at, _) in uncorrected_step_count_claims(&body)
            .into_iter()
            .filter(|(_, value)| *value == 69)
        {
            bare.push(format!("{home}:{}", line_of(&body, at)));
        }
    }
    assert!(
        bare.is_empty(),
        "`69` pack step files was never the number — it is `ls <dir> <dir> | wc -l` \
         counting its own two directory headers and blank separator over a tree of 66, \
         which is what 1.0.0-rc.10 shipped. A dated record keeps its basis inside a \
         `[Corrected …]` bracket; a live doc is corrected. Bare at: {bare:?}",
    );
}

#[test]
fn the_pack_step_count_is_stated_once_and_names_its_measurement_point() {
    // The live docs state no count of their own — they point at the home.
    let mut restated = Vec::new();
    for home in STEP_COUNT_RESTATEMENTS {
        let body = read_doc(home);
        for (at, value) in uncorrected_step_count_claims(&body) {
            restated.push(format!("{home}:{} (`{value}`)", line_of(&body, at)));
        }
        assert!(
            points_at_the_home(&body),
            "{home} drops the count, so it must point at the home that keeps it \
             (`implementation/decisions-pending.md` → *The rc.11 wave*)",
        );
    }
    assert!(
        restated.is_empty(),
        "the F1 pack-step count lives in one home ({STEP_COUNT_HOME}) and is \
         cross-referenced, never restated (CLAUDE.md → *Cross-reference, never \
         restate*); restated at: {restated:?}",
    );

    // A dated record keeps its own measurement, corrected — and its correction names the
    // home, so a reader who lands on the record still reaches the count that is current.
    for record in STEP_COUNT_RECORDS {
        let body = read_doc(record);
        let brackets = dated_correction_spans(&body);
        assert!(
            brackets
                .iter()
                .any(|(s, e)| points_at_the_home(&body[*s..*e])),
            "{record} corrects the count in its own voice, so one of its dated \
             correction brackets must name the home that now keeps it \
             (`implementation/decisions-pending.md` → *The rc.11 wave*)",
        );
    }

    let body = read_doc(STEP_COUNT_HOME);
    let charter = section(&body, "Before planning — milestone-keyed deferrals");
    let start = charter
        .find("### The rc.11 wave")
        .expect("decisions-pending.md must carry the rc.11 charter");
    let rest = &charter[start..];
    let end = rest[1..]
        .find("\n### ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    let rc11 = &rest[..end];

    let rows: Vec<&str> = rc11
        .lines()
        .filter(|line| step_count_claims(line).iter().any(|(_, n)| *n == 66))
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "the rc.11 charter must state the pack-step count as 66, exactly once; \
         found {} row(s) in:\n{rc11}",
        rows.len(),
    );

    // A bare count re-falsifies itself on the next step file the pack gains (the tree is
    // already at 67), so the surviving statement carries the binary it was measured on.
    let row = rows[0];
    for owed in ["1.0.0-rc.10", "pre-1.0.0 trial"] {
        assert!(
            row.contains(owed),
            "the count must name its measurement point (`{owed}`); the row reads:\n{row}",
        );
    }
}
