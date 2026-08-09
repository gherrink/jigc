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
//!   3. **CLAUDE.md's project-state paragraph names M47 and claims only the build.**
//!      M47 is **built, not audited** at this increment — the completion audit and its
//!      VERDICT are the milestone-completion workflow's next act — so the paragraph must
//!      name the roadmap section and the flow-47 acceptance while making **no** clean-audit
//!      claim. This is the law-1 fence applied to our own record: the doc may not claim a
//!      verdict that has not been reached.
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

#[test]
fn claude_md_names_m47_and_claims_only_the_build() {
    let body = read_doc("CLAUDE.md");
    let span = milestone_span(&body, "**M47 —");

    for owed in [
        "implementation/roadmap.md",
        "Milestone 47",
        "flow 47",
        "flow47_acceptance.rs",
    ] {
        assert!(
            span.contains(owed),
            "the M47 project-state claim must name `{owed}`; it reads:\n{span}",
        );
    }

    // **Reconciled 2026-08-09, not retired.** This fence was written during Increment
    // 11's fold-back to stop the record claiming an audit that had not run: it required
    // the bound `built, not audited` and *forbade* `VERDICT.md`. The audit has since run,
    // so leaving it would have fenced the record into a stale transitional truth — it went
    // red the moment the completion fold-back landed, which is the fence working, not
    // failing. Its **purpose is unchanged**: the M47 claim must not say more about the
    // audit than the audit found. Only the direction inverts — it stopped a premature
    // claim, and now stops an over-claim.
    //
    // The audit was **not** clean: 3 LOW findings were raised, reproduced live and fixed
    // (completions/artifacts/M47/VERDICT.md). So "audited clean" is exactly the overclaim
    // to forbid, and the honest record names its verdict artifact instead.
    assert!(
        span.contains("VERDICT"),
        "a completed audit cites its persisted verdict; the M47 claim names none:\n{span}",
    );
    assert!(
        span.contains("LOW") || span.contains("findings"),
        "the M47 claim must state what the audit FOUND, not merely that it ran:\n{span}",
    );
    for forbidden in [
        // The audit raised 3 LOW findings — any of these would overstate it.
        "audited clean",
        "audit is CLEAN",
        "audit ran clean",
        // The retired transitional bound: true during the build, false after the audit.
        "built, not audited",
    ] {
        assert!(
            !span.contains(forbidden),
            "the M47 claim must not overstate the audit, but contains `{forbidden}`:\n{span}",
        );
    }
}
