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
//!      live pin rather than one dead pin per wave — it now points at M50.
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
//!      A bare count also re-falsifies itself on the next step file the pack gains, so the
//!      surviving statement carries its **measurement point** — the pre-1.0.0 trial's
//!      1.0.0-rc.10 — and lives in **one** home: the rc.11 charter row in
//!      `implementation/decisions-pending.md`. The live docs that restated it now
//!      cross-reference that home instead (CLAUDE.md → *Cross-reference, never restate*).
//!
//!      **The fence itself became an instance of the class it was built for, and is
//!      re-keyed (M51 Increment 7, T6).** It banned the numeral `69` outright; the pack has
//!      since grown to exactly that many step files, so the one sentence stating the
//!      present tree truthfully was the one sentence it forbade. It now refuses a figure
//!      other than 66 **attached to the rc.10 measurement point**, which is the claim that
//!      was ever false, and lets a count naming its own measurement point through.
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

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use cli::render::ManifestKind;

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
/// wave's completion fold-back; M46 moved it forward at its Increment 10 close, M49
/// Increment 1 flipped it, one wave late, when M46's audit had already run, M49 Increment
/// 12 moved it forward again, and the M49 completion fold-back flipped it a fourth time.
///
/// At M50 Increment 13 it moves forward once more, to the wave this build closes. M49's
/// claim is settled prose now — its audit ran, its verdict is persisted, `1.0.0-rc.13` is
/// built, and nothing in this build can move any of it — while M50's claim is the one a
/// fold-back can overstate, and the overstatement available *today* is the premature one:
/// the completion audit, its persisted verdict and the version bump + install are the
/// milestone-completion workflow's next acts
/// ([milestone-completion-workflow.md](../../../implementation/milestone-completion-workflow.md);
/// [roadmap.md](../../../implementation/roadmap.md) → Milestone 50 Increment 13, whose
/// grouped scope discharges the wave's obligations and stops short of adjudicating them).
/// So the arm is **re-aimed, not duplicated**, a fourth time: the assertions invert back to
/// their pre-audit direction and follow the marker, rather than accumulating one dead pin
/// per wave.
///
/// The owed **version bump** is deliberately outside this fence, exactly as it was at M49's
/// build close. It is discharged as a named obligation rather than performed here, and it
/// carries **no numeral**: the roadmap's M50 section names no target version and the 1.0.0
/// call is the human's, so asserting a version string would be this fence choosing it.
///
/// **Inverted 2026-09-09, late — and the lateness is the finding.** The audit landed at
/// `95c79be6`, which rewrote CLAUDE.md's M50 paragraph to `built + audited` with a
/// `VERDICT` citation and touched fifteen files — **not** this one. The fence went red on
/// the spot, exactly as its module doc says it must, and stayed red through two further
/// commits while [the RC-rc14 handover](../../../completions/artifacts/RC-rc14/handover.md)
/// recorded *"Gate at HEAD: PASS · 3341 passed / 0 failed … re-run after the version bump
/// and the golden regen"* at that same sha. So the fence caught the record overstating
/// itself, and then the record overstated the fence. The inversion is performed here, at
/// the trial that verified the handover; what it now asserts is the post-audit direction
/// the module doc always specified — the completed claim required, the stale bound
/// forbidden, and **the cited verdict artifact checked against the filesystem** rather
/// than accepted as a string, because an unchecked citation is how this got here.
#[test]
fn claude_md_names_m50_and_claims_exactly_what_the_audit_reached() {
    let body = read_doc("CLAUDE.md");
    // The project-state paragraph is a single line; the sections that follow it (build /
    // lint / test, quickstart, code architecture) are not milestone claims, and M50 is the
    // last marker in the paragraph — so the span is bounded at the paragraph's own end
    // rather than running to EOF and forbidding these words to the whole file.
    let span = milestone_span(&body, "**M50 —")
        .split('\n')
        .next()
        .expect("splitting a str always yields at least one part");

    for owed in [
        "implementation/roadmap.md",
        "Milestone 50",
        "flow 51",
        "flow51_acceptance.rs",
    ] {
        assert!(
            span.contains(owed),
            "the M50 project-state claim must name `{owed}`; it reads:\n{span}",
        );
    }

    // INVERTED 2026-09-09, which is this fence doing its job rather than failing at it.
    // The audit ran, so the pre-audit bound is now itself the law-1 lie: a paragraph
    // still saying *built, not audited* would understate a verdict that exists.
    assert!(
        span.contains("built + audited"),
        "M50's audit has run, and the claim must say so in those words:\n{span}",
    );
    for forbidden in [
        // The stale bound, now the overstatement's mirror: it claims LESS than happened.
        "built, not audited",
        // The 1.0.0 call is the human's and was deliberately not taken at this wave
        // (`completions/artifacts/M50/VERDICT.md` -> What is next). A paragraph claiming
        // it would be the overstatement available *today*, exactly as a premature audit
        // claim was the one available before.
        "1.0.0 is called",
        "1.0.0 shipped",
    ] {
        assert!(
            !span.contains(forbidden),
            "the M50 claim may not say `{forbidden}`:\n{span}",
        );
    }

    // The citation is checked against the filesystem, not merely required as a string.
    // A link to a verdict that does not exist is the same law-1 lie the pre-audit
    // direction forbade, wearing the opposite costume — and the failure this whole
    // inversion is owed to was a claim nobody drove.
    assert!(
        span.contains("VERDICT"),
        "the audited M50 claim must cite its persisted verdict:\n{span}",
    );
    let verdict = repo_root().join("completions/artifacts/M50/VERDICT.md");
    assert!(
        verdict.is_file(),
        "the M50 claim cites a verdict artifact that does not exist at {}",
        verdict.display(),
    );
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

/// The **measurement point** the falsified figure was attached to: the pre-1.0.0 trial,
/// run on 1.0.0-rc.10. A prose unit naming either token is stating *that* measurement,
/// whatever number it carries.
const RC10_MEASUREMENT: [&str; 2] = ["1.0.0-rc.10", "pre-1.0.0 trial"];

/// What the tree actually carried at that measurement point.
const RC10_STEP_COUNT: usize = 66;

/// The **line** around `at`, which is the granularity at which a count and the measurement
/// point it is attached to sit together in this repo: a markdown paragraph here is one very
/// long line, the rc.11 charter states its count and both measurement-point tokens on one
/// row, and `DECISIONS.md`'s build entries are one claim per bullet line.
///
/// **Declared bound:** a claim whose measurement point is named in a *neighbouring*
/// sentence is outside this probe. The looser granularity was tried and is wrong here —
/// over a run of `DECISIONS.md` bullets it read the M47 batch-author note's unrelated
/// *13 step files* as a statement about 1.0.0-rc.10, which is the false-positive shape that
/// makes a fence get relaxed rather than obeyed.
fn enclosing_line(body: &str, at: usize) -> &str {
    let start = body[..at].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = body[at..].find('\n').map(|i| at + i).unwrap_or(body.len());
    &body[start..end]
}

/// The claims this suite refuses, as a **pure predicate** over a body, so the fence can be
/// asserted over fixtures rather than only over today's tree.
///
/// **Keyed on the measurement point, not on the numeral (M51 Increment 7, T6).** The first
/// form of this fence banned a bare `69` outright, on the ground that 69 was never the
/// number — and then the pack grew, the tree reached 69 step files, and the fence built to
/// stop a stale count from shipping became an instance of its own class: it now forbids the
/// one sentence that would be *true*. What is false is not the numeral; it is attaching any
/// figure but [`RC10_STEP_COUNT`] to [`RC10_MEASUREMENT`]. So that is what is refused, and a
/// present-tense count measured on today's tree passes — as it must, since a count that
/// names its own measurement point is exactly what this suite asks every home for.
fn stale_pack_step_claims(body: &str) -> Vec<(usize, usize)> {
    uncorrected_step_count_claims(body)
        .into_iter()
        .filter(|(at, value)| {
            *value != RC10_STEP_COUNT
                && RC10_MEASUREMENT
                    .iter()
                    .any(|token| enclosing_line(body, *at).contains(token))
        })
        .collect()
}

/// The re-key, asserted over the predicate rather than over today's bytes: the fence must
/// **spare** a true present-tense count and **catch** the falsified rc.10 denominator — and
/// the two fixtures carry the same numeral, so nothing but the measurement point can
/// separate them.
#[test]
fn the_step_count_fence_keys_on_the_measurement_point_not_the_numeral() {
    let present = "The pack ships 69 step files at HEAD (2026-09-15).";
    assert!(
        stale_pack_step_claims(present).is_empty(),
        "a TRUE present-tense count carrying its own measurement point is not the \
         falsified claim — banning the numeral bans the truth",
    );
    let stale = "Of 69 pack step files at 1.0.0-rc.10, exactly one names `jigc doc show`.";
    assert_eq!(
        stale_pack_step_claims(stale).len(),
        1,
        "`69` attached to the pre-1.0.0 trial's 1.0.0-rc.10 is the falsified claim",
    );
    let bracketed = "Of **[Corrected 2026-08-15: 69 step files]** at 1.0.0-rc.10, one names it.";
    assert!(
        stale_pack_step_claims(bracketed).is_empty(),
        "a dated record keeps its own falsified basis visible inside the bracket",
    );
}

#[test]
fn the_stale_pack_step_count_survives_only_inside_a_dated_correction() {
    let mut bare = Vec::new();
    for home in STEP_COUNT_HOMES {
        let body = read_doc(home);
        for (at, _) in stale_pack_step_claims(&body) {
            bare.push(format!("{home}:{}", line_of(&body, at)));
        }
    }
    assert!(
        bare.is_empty(),
        "these homes attach a pack-step count other than {RC10_STEP_COUNT} to the \
         pre-1.0.0 trial's 1.0.0-rc.10. The denominator that reached eleven sites was \
         `ls <dir> <dir> | wc -l` counting its own two directory headers and blank \
         separator over a tree of {RC10_STEP_COUNT}, which is what 1.0.0-rc.10 shipped. A \
         dated record keeps its basis inside a `[Corrected …]` bracket; a live doc is \
         corrected. Misattributed at: {bare:?}",
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

    // A bare count re-falsifies itself on the next step file the pack gains, so the
    // surviving statement carries the binary it was measured on.
    let row = rows[0];
    for owed in ["1.0.0-rc.10", "pre-1.0.0 trial"] {
        assert!(
            row.contains(owed),
            "the count must name its measurement point (`{owed}`); the row reads:\n{row}",
        );
    }
}

// ---------------------------------------------------------------------------
// 5. The shipped guides describe the destroying door as it behaves.
// ---------------------------------------------------------------------------

/// **The guides are a product surface, not documentation** (M50 Increment 13 / T4;
/// [settle-record.md](../../../completions/artifacts/M50/settle-record.md) → *Docs that
/// move*, "priced, not assumed"). `crates/cli/src/setup.rs` `include_str!`s them into
/// `.claude/skills/jigc/SKILL.md`, the artifact `jigc setup` installs and owns, so their
/// bytes reach every adopter's repo and editing them moves `jigc-body-blake3`.
///
/// Both stated `jigc task discard` as unconditionally safe — *"it removes only the working
/// area … no commit is made"* — which stopped being the whole truth when M50 Increment 3
/// gave the door the shared staged-prose guard: it now **refuses** while the task's area
/// stages docs no commit has a copy of (`task-discard.staged-prose`), and `--force` is the
/// single consent that removes them anyway. A guide that describes the door without naming
/// the refusal sends a reader at a command that will not run, and one that names the
/// refusal without the consent leaves them with no exit — so both tokens are owed wherever
/// the door is named.
///
/// **The subject is derived, not hand-listed.** The set is read out of `setup.rs`'s own
/// `include_str!` sites, so a third guide joining the shipped artifact joins this fence
/// with no test edit — and a guide leaving it stops being asserted here for the same
/// reason. The predicate iterates **every** unit of each guide that names the door, not the
/// one the finding reported: a second paragraph describing the discard door owes the same
/// two facts as the first.
///
/// The behaviour itself is pinned through the real binary elsewhere —
/// `crates/cli/tests/staged_prose_consent_axis.rs` drives both doors that carry the guard,
/// their codes and their `--force` consent. This arm asserts only that the shipped prose
/// says what that suite proves.
const GUIDE_INCLUDE_PREFIX: &str = "include_str!(\"../../../";

/// Every guide `setup.rs` embeds into the installed artifact, in declaration order.
fn shipped_guides() -> Vec<String> {
    let setup = fs::read_to_string(repo_root().join("crates/cli/src/setup.rs"))
        .expect("crates/cli/src/setup.rs must be readable");
    let mut guides = Vec::new();
    for (at, _) in setup.match_indices(GUIDE_INCLUDE_PREFIX) {
        let tail = &setup[at + GUIDE_INCLUDE_PREFIX.len()..];
        let Some(end) = tail.find('"') else { continue };
        guides.push(tail[..end].to_string());
    }
    assert!(
        !guides.is_empty(),
        "setup.rs must `include_str!` the guides it ships — the subject of this fence is \
         read from those sites, never hand-listed",
    );
    guides
}

/// The doc's units of prose: a blank-line paragraph, and every list item within one as its
/// own unit — a numbered back-out ladder is a single blank-line block, so paragraph
/// splitting alone would let one gate's honesty be paid for by its neighbour's.
fn prose_units(body: &str) -> Vec<String> {
    let mut units: Vec<String> = Vec::new();
    let mut current = String::new();
    let starts_item = |line: &str| {
        let t = line.trim_start();
        t.starts_with("- ")
            || t.starts_with("* ")
            || t.split_once(". ")
                .is_some_and(|(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
    };
    for line in body.lines() {
        if line.trim().is_empty() || starts_item(line) {
            if !current.trim().is_empty() {
                units.push(std::mem::take(&mut current));
            } else {
                current.clear();
            }
            if line.trim().is_empty() {
                continue;
            }
        }
        current.push_str(line);
        current.push('\n');
    }
    if !current.trim().is_empty() {
        units.push(current);
    }
    units
}

#[test]
fn the_shipped_guides_name_the_discard_refusal_and_its_consent() {
    let mut checked = 0usize;
    for guide in shipped_guides() {
        let body = read_doc(&guide);
        let describing: Vec<String> = prose_units(&body)
            .into_iter()
            .filter(|unit| unit.contains("jigc task discard"))
            .collect();
        assert!(
            !describing.is_empty(),
            "{guide} ships to every adopter and must describe how to back a task out; \
             no unit of it names `jigc task discard`",
        );
        for unit in describing {
            for owed in [
                // the refusal, by the identity the door actually prints
                "task-discard.staged-prose",
                // the single consent, so the reader is not left without an exit
                "--force",
            ] {
                assert!(
                    unit.contains(owed),
                    "{guide} describes `jigc task discard` without naming `{owed}` — the \
                     door refuses over staged docs and `--force` is its one consent; the \
                     unit reads:\n{unit}",
                );
            }
            checked += 1;
        }
    }
    assert!(
        checked >= 2,
        "both shipped guides describe the door; only {checked} unit(s) were checked",
    );
}

// ---------------------------------------------------------------------------
// 6. The manifest vocabulary is whole wherever it is spoken.
// ---------------------------------------------------------------------------

// **The finalize manifest's tag vocabulary is PARTITIONED, and a flat owe-set would mint
// a fresh law-1 lie** (M51 Increment 7 / T1; `completions/artifacts/M51/settle-record.md`
// → D8; the charter's EC-22).
//
// [`ManifestKind`] has six members, and `manifest_line`'s own doc-comment states the split
// the prose has to respect: `Untracked` never reaches the **included** path — it tags only
// files the commit left out. So the two docs of record are saying two different true
// things, and both are correct as written: `design/finalize.md` names the **committed
// set**'s five (`promoted` / `modified` / `deleted` / `added` / `carried-over`), and
// `design/command-output-contract.md` names the **JSON `kind`** value space's six. A fence
// owing all six everywhere would force `untracked` into the committed-set sentence — a new
// falsehood, shipped by the fence built to stop falsehoods.
//
// Hence the mint is [`ManifestKind::ALL`] **plus** the in-commit/left-out partition as
// code-side data ([`ManifestKind::in_commit`], an exhaustive match, so a seventh member
// cannot compile until it is classified), and the owe-set is **chosen by which vocabulary
// the unit is speaking** — read off the partition itself, never hand-assigned per home.
//
// The claim this closes: a prose home that enumerates the vocabulary enumerates *all* of
// it, so the value a driver will meet on the wire cannot be missing from the sentence that
// teaches the vocabulary. `MIGRATING.md`'s four-tag list has been short by `added` since
// M30 shipped the kind, and nothing was watching.
/// Which of the manifest's two vocabularies a prose unit is speaking.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Vocabulary {
    /// The **committed set**: the tags a path *in the commit* can carry. Five members.
    Committed,
    /// The whole JSON `kind` value space — the committed set plus the left-out-only tag.
    /// Six members.
    Whole,
}

impl Vocabulary {
    /// The tags a unit speaking this vocabulary owes — derived from
    /// [`ManifestKind::in_commit`] over [`ManifestKind::ALL`], never a second hand list.
    fn owed(self) -> Vec<&'static str> {
        ManifestKind::ALL
            .iter()
            .filter(|kind| self == Vocabulary::Whole || kind.in_commit())
            .map(|kind| kind.tag())
            .collect()
    }
}

/// The tags that only ever label a **left-out** path — the partition's other side, read
/// from the same predicate rather than restated.
fn left_out_only_tags() -> BTreeSet<&'static str> {
    ManifestKind::ALL
        .iter()
        .filter(|kind| !kind.in_commit())
        .map(|kind| kind.tag())
        .collect()
}

/// Whether `unit` names `tag` as a whole token. `-` counts as a word character, so
/// `carried-over` is one token and `added` does not match inside `added-optional-field`.
fn names_tag(unit: &str, tag: &str) -> bool {
    let boundary = |c: Option<char>| !c.is_some_and(|c| c.is_alphanumeric() || c == '-');
    unit.match_indices(tag).any(|(at, _)| {
        boundary(unit[..at].chars().next_back()) && boundary(unit[at + tag.len()..].chars().next())
    })
}

/// Every manifest tag `unit` names.
fn tags_named(unit: &str) -> BTreeSet<&'static str> {
    ManifestKind::ALL
        .iter()
        .map(|kind| kind.tag())
        .filter(|tag| names_tag(unit, tag))
        .collect()
}

/// The vocabulary a prose unit is **enumerating**, or `None` when it is not enumerating one.
///
/// **A bounded detector, and the bound is stated.** A unit enumerates the vocabulary when
/// it names the **manifest** — both vocabularies belong to that one surface — *and* three
/// or more of its tags. Three rather than one because every tag is also an ordinary English
/// word these docs use constantly (*"the promoted docs"*, *"a deleted tree"*, *"added at
/// M43"*), so a one-tag trigger would fence prose that is not making the claim; every real
/// enumeration in the tree names four or more. A unit that enumerates the vocabulary in
/// some other wording than *manifest* is outside this probe — the residue is named, not
/// papered over.
///
/// **Which vocabulary is read off the partition, not hand-assigned.** A unit naming a
/// left-out-only kind is speaking about the left-out list as well as the commit set, so it
/// owes the whole of [`ManifestKind::ALL`]; a unit naming only in-commit kinds is
/// describing the commit set, and owes exactly that side of the partition.
fn vocabulary_spoken(unit: &str) -> Option<Vocabulary> {
    if !unit.contains("manifest") {
        return None;
    }
    let named = tags_named(unit);
    if named.len() < 3 {
        return None;
    }
    let left_out = left_out_only_tags();
    Some(if named.iter().any(|tag| left_out.contains(tag)) {
        Vocabulary::Whole
    } else {
        Vocabulary::Committed
    })
}

/// The tags a unit's **own** vocabulary owes and the unit does not name. Empty for a unit
/// that is not enumerating one.
fn missing_tags(unit: &str) -> Vec<&'static str> {
    match vocabulary_spoken(unit) {
        None => Vec::new(),
        Some(vocabulary) => vocabulary
            .owed()
            .into_iter()
            .filter(|tag| !names_tag(unit, tag))
            .collect(),
    }
}

/// The docs of record for the two vocabularies — `finalize.md` owns the committed set,
/// `command-output-contract.md` the JSON `kind` value space. Enumerated rather than
/// globbed, on `STEP_COUNT_HOMES`' reason: a sweep of every `.md` would reach the dated
/// records, whose job is to state the world as it was.
const VOCABULARY_HOMES: [&str; 2] = ["design/finalize.md", "design/command-output-contract.md"];

/// **The shipped guides are deferred, not exempt — and the deferral names its landing.**
///
/// Both guides are `include_str!`'d into the installed `SKILL.md` (`shipped_guides` above),
/// so *any* guide byte moves `jigc-body-blake3` and engages M48's refuse-to-clobber path.
/// The decomposition therefore lands every guide byte this wave owes in **one batch, one
/// hash move** (`implementation/roadmap.md` → Milestone 51, Increment 9 — *the law-1 surface
/// batch and the single guide hash move*, which names EC-22, this correction, among its
/// rows). Editing `MIGRATING.md` here would ship the second hash move that decomposition
/// exists to prevent.
///
/// So the exclusion is **scoped to the whole shipped-guide set** — no guide byte moves
/// before the batch, so a third guide joining the installed artifact joins the deferral
/// too, which is what the set equality below makes checkable — and the stale literal the
/// batch will correct is asserted **still present**, so Increment 9's correction reddens
/// this suite and forces the exclusion to be lifted in the same commit. This is the
/// claim-3 inversion idiom this suite already runs in both directions.
const GUIDE_BATCH: &str = "Increment 9";

/// The stale literals the guide batch will correct: `(guide, literal)`. Today exactly one —
/// `MIGRATING.md`'s four-tag list, short by `added`.
const OWED_AT_GUIDE_BATCH: [(&str, &str); 1] = [(
    "MIGRATING.md",
    "(`promoted` / `modified` / `deleted` / `carried-over`)",
)];

#[test]
fn every_prose_unit_that_enumerates_the_manifest_vocabulary_names_all_of_it() {
    let mut checked = 0usize;
    let mut short = Vec::new();
    for home in VOCABULARY_HOMES {
        let body = read_doc(home);
        for unit in prose_units(&body) {
            let Some(vocabulary) = vocabulary_spoken(&unit) else {
                continue;
            };
            checked += 1;
            let missing = missing_tags(&unit);
            if !missing.is_empty() {
                short.push(format!(
                    "{home} (speaking {vocabulary:?}) omits {missing:?} from:\n{unit}"
                ));
            }
        }
    }
    assert!(
        short.is_empty(),
        "a home that enumerates the manifest vocabulary enumerates all of the vocabulary \
         it is speaking — the committed set's five, or the JSON `kind` space's six; \
         short at: {short:#?}",
    );
    assert_eq!(
        checked,
        VOCABULARY_HOMES.len(),
        "each doc of record must enumerate its vocabulary exactly once — if a home stopped \
         stating it, the contract moved and this fence stopped fencing anything",
    );
}

/// The fence is a property of the predicate, not of today's bytes: it must **catch** a unit
/// that names a strict subset of the vocabulary it is speaking. The fixture is the shipped
/// defect — `MIGRATING.md`'s four-tag list — so the arm also states, executably, what the
/// guide batch is owed for.
#[test]
fn a_unit_naming_a_strict_subset_of_its_vocabulary_is_caught() {
    let short = "It prints the manifest the finalize *would* commit — each path tagged by \
                 how it enters (`promoted` / `modified` / `deleted` / `carried-over`).";
    assert_eq!(
        vocabulary_spoken(short),
        Some(Vocabulary::Committed),
        "a unit naming no left-out-only tag is describing the commit set",
    );
    assert_eq!(
        missing_tags(short),
        vec!["added"],
        "the committed set's fifth tag is missing and the fence must say so",
    );

    // The same sentence completed is clean — the fence owes the partition, never all six.
    let whole = short.replace("`deleted`", "`deleted` / `added`");
    assert_eq!(
        missing_tags(&whole),
        Vec::<&str>::new(),
        "a committed-set enumeration owes the committed set, not `untracked` as well",
    );

    // And a unit that names the left-out side owes the whole value space.
    let json = "the manifest JSON tags each path with a `kind` (`promoted` / `modified` / \
                `deleted` / `added` / `untracked`)";
    assert_eq!(vocabulary_spoken(json), Some(Vocabulary::Whole));
    assert_eq!(
        missing_tags(json),
        vec!["carried-over"],
        "a unit naming the JSON value space owes every value, `carried-over` included",
    );

    // Ordinary prose that merely uses the words is not an enumeration.
    let prose = "the manifest names the promoted docs and the modified sources";
    assert_eq!(vocabulary_spoken(prose), None);
}

#[test]
fn the_guide_vocabulary_correction_is_owed_at_the_single_guide_batch() {
    let deferred: BTreeSet<String> = OWED_AT_GUIDE_BATCH
        .iter()
        .map(|(guide, _)| (*guide).to_string())
        .chain(
            // A guide with nothing stale still may not move before the batch, so the
            // deferral's subject is the guide set, never the one stale file.
            shipped_guides()
                .into_iter()
                .filter(|guide| !guide.contains("MIGRATING")),
        )
        .collect();
    let shipped: BTreeSet<String> = shipped_guides().into_iter().collect();
    assert_eq!(
        deferred, shipped,
        "the deferral covers every shipped guide — the batch is one hash move, so a guide \
         joining the installed artifact joins {GUIDE_BATCH}'s batch with it",
    );

    for (guide, literal) in OWED_AT_GUIDE_BATCH {
        let body = read_doc(guide);
        assert!(
            body.contains(literal),
            "{guide} is excluded from the live fence because its correction lands in \
             {GUIDE_BATCH}'s single guide batch, keyed on the literal `{literal}`. That \
             literal is gone — so either the correction landed (lift the exclusion and \
             fence the guide live) or the sentence moved (re-key the row).",
        );
        // The exclusion is load-bearing only while the literal is genuinely short.
        let stale: Vec<String> = prose_units(&body)
            .into_iter()
            .filter(|unit| unit.contains(literal))
            .filter(|unit| !missing_tags(unit).is_empty())
            .collect();
        assert_eq!(
            stale.len(),
            1,
            "{guide}'s owed literal must sit in exactly one unit the fence would redden; \
             found {} — the deferral is bookkeeping, not a blanket exemption",
            stale.len(),
        );
    }
}

/// `ManifestKind::ALL` is a registry only if it holds every variant the enum declares — an
/// exhaustive `match` forces a seventh member to be *classified*, not to *join the array*,
/// and an escaped member would silently shrink every owe-set above. Read from the source,
/// which also fences `tag()` against serde's `rename_all = "kebab-case"`: the two spellings
/// reach the same wire and a divergence is invisible at runtime.
#[test]
fn manifest_kind_all_holds_every_variant_the_enum_declares() {
    let source = fs::read_to_string(repo_root().join("crates/cli/src/render.rs"))
        .expect("crates/cli/src/render.rs must be readable");
    const HEAD: &str = "pub enum ManifestKind {";
    let at = source
        .find(HEAD)
        .expect("render.rs must declare `pub enum ManifestKind`");
    let body = &source[at + HEAD.len()..];
    let end = body.find('}').expect("the enum block must close");
    let declared: BTreeSet<String> = body[..end]
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| line.trim_end_matches(',').to_string())
        .map(|variant| {
            let mut kebab = String::new();
            for (i, c) in variant.chars().enumerate() {
                if c.is_ascii_uppercase() && i > 0 {
                    kebab.push('-');
                }
                kebab.extend(c.to_lowercase());
            }
            kebab
        })
        .collect();
    let registered: BTreeSet<String> = ManifestKind::ALL
        .iter()
        .map(|kind| kind.tag().to_string())
        .collect();
    assert_eq!(
        declared, registered,
        "`ManifestKind::ALL` must hold every declared variant, and `tag()` must spell each \
         one exactly as serde's `rename_all = \"kebab-case\"` puts it on the wire",
    );
}

// ---------------------------------------------------------------------------
// 7. The fold-back names the version `Cargo.toml` carries.
// ---------------------------------------------------------------------------

// **The version-stamp confirmation has been prose for six waves, and prose caught the owed
// bump unshipped in five of them** (M51 Increment 7 / T7;
// [settle-record.md](../../../completions/artifacts/M51/settle-record.md) → **D11** and its
// amendment **§15**; [charter.md](../../../completions/artifacts/M51/charter.md) → EC-10).
// The failure has one shape every time: the fold-back paragraph states a version as *built
// and installed* while `Cargo.toml` still carries the one before it, and the two homes are
// only ever compared by a human who remembers to look.
//
// **This is a narrowing of a recorded refusal, with that refusal's rationale engaged — not
// an override.** The claim-3 arm above declines the version assertion **by name**: *"it
// carries **no numeral**: the roadmap's M50 section names no target version and the 1.0.0
// call is the human's, so asserting a version string would be this fence choosing it."* That
// rationale holds for choosing a **numeral**, and is **silent on comparing two homes**. So
// the fence below chooses nothing: it reads the version out of `Cargo.toml` and asserts that
// the fold-back's own claim names *that* version, whatever it is. The refusal's live half
// survives intact — **whether** the bump has happened is still not asserted here, which is
// why the claim is a conditional (see the bound below).
//
// **Two bounds, written here rather than left in the record.**
//
//   * **Razor leg 2 would refuse this fence on its face.** The wave's razor asks that a
//     change be *right for any adopter — if the only beneficiary is our layout, our naming
//     or our process, it is refused*, and a fence over **this repo's** fold-back paragraph
//     benefits this repo's process only; it is the exact ground on which the charter
//     excludes the harness-surface wave. **The admission rests on the human's boundary
//     decision** — the wave takes every ledger row, EC-10 included — **not on the razor**,
//     and **no future wave may cite D11 as a razor precedent** (§15).
//   * **The pack-step home is refused with its ground.** The obvious alternative — have
//     `completion.yaml` tell the agent to check the version — was refused: that step ships
//     into **every adopter's repo**, and the version it would name is jigc's own, a law-1 lie
//     for every reader who is not this repo (`re-verify.yaml` already models the right
//     register). A new pack step would also move the pack-step count and make the count
//     fence's cell above disappear by accident.
//
// **What "the version the fold-back names" means, stated as a grammar.** CLAUDE.md's
// project-state paragraph names **every** version this project has ever shipped, so *any
// version token* is the wrong subject. The subject is the paragraph's **built-and-installed
// claim**: a version token immediately followed by the words the fold-back has used at every
// wave — `built and installed` / `built + installed`, optionally through an `is`, across the
// markup that decorates it. A trial sentence naming the binary it ran on
// (*"`1.0.0-rc.13` built from `979baca`"*) is not that claim and is not bound, which is the
// distinction the historical paragraph actually turns on.
//
// **Declared bound:** the claim is a **conditional**, and it binds the **newest** wave's span
// only. A fold-back that names no version at all passes — the presence of the bump is the
// milestone-completion workflow's obligation and is precisely what the recorded refusal
// declined to choose — and a historical span keeps the version *it* shipped, since that
// claim was true when it was written. A claim phrased with the token **after** the words
// (*"we built and installed `x`"*) is outside the grammar; the non-vacuity leg below is what
// catches a wholesale drift of the shipped phrasing.

/// The workspace version — the second home, read from `[workspace.package]` rather than from
/// any crate's inherited `version.workspace = true`.
fn workspace_version() -> String {
    let manifest = fs::read_to_string(repo_root().join("Cargo.toml"))
        .expect("the workspace Cargo.toml must be readable");
    let at = manifest
        .find("[workspace.package]")
        .expect("Cargo.toml must declare `[workspace.package]`");
    let section = &manifest[at..];
    let end = section[1..]
        .find("\n[")
        .map(|i| i + 1)
        .unwrap_or(section.len());
    for line in section[..end].lines() {
        if let Some(rest) = line.trim().strip_prefix("version") {
            let rest = rest.trim_start();
            if let Some(rest) = rest.strip_prefix('=') {
                return rest.trim().trim_matches('"').to_string();
            }
        }
    }
    panic!("`[workspace.package]` must carry a `version = \"…\"` key");
}

/// Byte spans of every semver-shaped token in `text` — `<major>.<minor>.<patch>` with an
/// optional `-<pre-release>`. Deliberately loose about what surrounds it: the discrimination
/// is done by [`states_built_and_installed`], on what the token is *claimed to be*.
fn version_tokens(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        // Not the tail of a longer number, nor the patch half of a token already read.
        if text[..i]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_ascii_digit() || c == '.')
        {
            i += 1;
            continue;
        }
        let start = i;
        let mut dots = 0usize;
        let mut end = i;
        while end < bytes.len() {
            let b = bytes[end];
            if b.is_ascii_digit() {
                end += 1;
            } else if b == b'.'
                && dots < 2
                && end + 1 < bytes.len()
                && bytes[end + 1].is_ascii_digit()
            {
                dots += 1;
                end += 1;
            } else {
                break;
            }
        }
        if dots < 2 {
            i = end.max(start + 1);
            continue;
        }
        // The optional pre-release: `-rc.15`, `-gate`, …
        if end < bytes.len() && bytes[end] == b'-' {
            let mut pre = end + 1;
            while pre < bytes.len() && (bytes[pre].is_ascii_alphanumeric() || bytes[pre] == b'.') {
                pre += 1;
            }
            // A trailing `.` belongs to the sentence, not to the version.
            while pre > end + 1 && bytes[pre - 1] == b'.' {
                pre -= 1;
            }
            if pre > end + 1 {
                end = pre;
            }
        }
        tokens.push((start, end));
        i = end;
    }
    tokens
}

/// Whether the text immediately following a version token states the fold-back's
/// built-and-installed claim about it — the grammar stated in this section's opening comment.
fn states_built_and_installed(tail: &str) -> bool {
    /// The markup a claim is decorated with — backticks, emphasis, and the whitespace
    /// between. Never punctuation: an em dash or a comma between the token and the words
    /// would make it a different sentence.
    fn undecorated(text: &str) -> &str {
        text.trim_start_matches(|c: char| c == '`' || c == '*' || c.is_whitespace())
    }

    let rest = undecorated(tail);
    // `**1.0.0-rc.10 is built and installed**` and `**`1.0.0-rc.14` built and installed**`
    // are the two shipped spellings of one claim.
    let rest = undecorated(rest.strip_prefix("is").unwrap_or(rest));
    let Some(rest) = rest.strip_prefix("built") else {
        return false;
    };
    let rest = undecorated(rest);
    // `built and installed` / `built + installed` — but never `built from <sha>`, which is a
    // trial's provenance and binds nothing.
    let Some(rest) = rest.strip_prefix("and").or_else(|| rest.strip_prefix('+')) else {
        return false;
    };
    undecorated(rest).starts_with("installed")
}

/// Every version `text` names as **built and installed**, as `(offset, version)`.
fn built_and_installed_versions(text: &str) -> Vec<(usize, String)> {
    version_tokens(text)
        .into_iter()
        .filter(|(_, end)| states_built_and_installed(&text[*end..]))
        .map(|(start, end)| (start, text[start..end].to_string()))
        .collect()
}

/// The newest wave's marker in CLAUDE.md's project-state paragraph — the claim a fold-back
/// is still able to move. Derived rather than pinned, so the fence follows the paragraph the
/// way the paragraph grows: each wave appends its own claim at the end.
fn newest_milestone_marker(body: &str) -> String {
    let mut newest: Option<String> = None;
    for (at, _) in body.match_indices("**M") {
        let rest = &body[at + "**M".len()..];
        let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() {
            continue;
        }
        if rest[digits.len()..].starts_with(" —") {
            newest = Some(format!("**M{digits} —"));
        }
    }
    newest.expect("CLAUDE.md's project state must carry at least one `**M<nn> —` marker")
}

/// The newest wave's claim: its span within the project-state paragraph, which is one line.
fn newest_milestone_claim(body: &str) -> &str {
    let marker = newest_milestone_marker(body);
    milestone_span(body, &marker)
        .split('\n')
        .next()
        .expect("splitting a str always yields at least one part")
}

/// **The comparison, as a pure function of `(body, version)`** — so the fence is asserted over
/// fixtures as well as over today's bytes. Every version the newest wave's claim states as
/// built and installed, that is not the version `Cargo.toml` carries.
fn foldback_version_mismatches(body: &str, version: &str) -> Vec<String> {
    let claim = newest_milestone_claim(body);
    built_and_installed_versions(claim)
        .into_iter()
        .filter(|(_, named)| named != version)
        .map(|(at, named)| format!("names `{named}` at offset {at} of the newest wave's claim"))
        .collect()
}

#[test]
fn the_foldback_names_the_version_cargo_toml_carries() {
    let body = read_doc("CLAUDE.md");
    let version = workspace_version();

    let mismatches = foldback_version_mismatches(&body, &version);
    assert!(
        mismatches.is_empty(),
        "CLAUDE.md's newest wave claims a version built and installed that `Cargo.toml` does \
         not carry (`{version}`): {mismatches:?}. Either the bump is owed — the failure five \
         consecutive waves needed a human to notice — or the fold-back is naming the wrong \
         binary.",
    );

    // Non-vacuity, scoped to the paragraph rather than to the newest span: a wave that has
    // not yet bumped names no version at all, which is allowed, but the *shipped phrasing*
    // must stay recognisable — a wholesale reword would leave this fence asserting nothing.
    let marker = newest_milestone_marker(&body);
    let paragraph = body
        .lines()
        .find(|line| line.contains(&marker))
        .expect("the project-state paragraph must be one line of CLAUDE.md");
    assert!(
        !built_and_installed_versions(paragraph).is_empty(),
        "no built-and-installed claim is recognisable anywhere in the project-state \
         paragraph — the phrasing this fence reads moved, and the fence stopped fencing",
    );
}

/// The red step this fence was written from: the comparison is a function of `(body, version)`
/// and must **reject** a fold-back naming a version other than the one it is handed. Asserted
/// over fixtures, never over today's bytes — the claim is GREEN at HEAD, so today's tree
/// cannot demonstrate the fence bites.
#[test]
fn the_version_comparison_rejects_a_foldback_naming_another_version() {
    // Two waves, exactly as the paragraph carries them: a settled one, and the newest.
    let body = "\
**M50 — the last wave — is complete** (… **`1.0.0-rc.14` built and installed after the fixes, \
not before**, with the goldens regenerated). **M51 — the count wave — is complete** (… \
**`1.0.0-rc.15` built and installed after the fixes, not before**). The trial ran on \
`1.0.0-rc.14` built from `21ffc0d4`.

## Build / lint / test
";

    // The fence bites when the two homes disagree …
    assert_eq!(
        foldback_version_mismatches(body, "1.0.0-rc.14").len(),
        1,
        "a fold-back naming `1.0.0-rc.15` as built and installed while `Cargo.toml` carries \
         `1.0.0-rc.14` is exactly the failure five consecutive waves shipped",
    );
    // … and passes when they agree, so it is the comparison biting and not the numeral.
    assert!(
        foldback_version_mismatches(body, "1.0.0-rc.15").is_empty(),
        "the fence compares two homes; it chooses no numeral",
    );

    // The settled wave keeps the version it shipped — only the newest claim is bound, which
    // is what keeps `1.0.0-rc.14` true where M50 wrote it while the tree carries rc.15.
    let newest = newest_milestone_claim(body);
    assert!(
        newest.contains("the count wave") && !newest.contains("the last wave"),
        "the subject is the newest wave's claim alone; it reads:\n{newest}",
    );

    // A trial sentence naming the binary it ran on is not a built-and-installed claim, which
    // is what keeps the paragraph's dozen historical version tokens out of the subject.
    let trial = "The pre-v1 trial ran on `1.0.0-rc.13` built from `979baca` — not the \
                 host-installed rc.13, which predates it.";
    assert!(
        built_and_installed_versions(trial).is_empty(),
        "`built from <sha>` is a trial's provenance, not the fold-back's claim",
    );

    // Both shipped spellings of the claim are read, through the markup that decorates them.
    for spelling in [
        "**`1.0.0-rc.15` built and installed after the fixes**",
        "**1.0.0-rc.15 built + installed** (the version-stamp confirmation)",
        "and `1.0.0-rc.15` is built and installed — after the audit's fixes, not before**",
    ] {
        assert_eq!(
            built_and_installed_versions(spelling)
                .into_iter()
                .map(|(_, v)| v)
                .collect::<Vec<_>>(),
            vec!["1.0.0-rc.15".to_string()],
            "the shipped claim spelling must be read: {spelling}",
        );
    }
}
