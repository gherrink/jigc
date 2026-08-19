//! **The gate-coverage table** — one code-side source for what `jigc task validate`
//! previews and what only `jigc task finalize` decides.
//!
//! The claim `jigc task validate` makes is a *coverage* claim, and it is stated on
//! eight surfaces: the composed `what's-left:` line, both packs' `finalize` steps,
//! the `task` unknown-subcommand tip, the `validate-task` catalog hint `describe`
//! mirrors, [QUICKSTART.md](../../../QUICKSTART.md), and the two design docs that
//! own the split — [command-output-contract.md](../../../design/command-output-contract.md)
//! → The exit-code taxonomy (third clause: *"position, not scope"*) and
//! [finalize.md](../../../design/finalize.md) → 2. Validate. Every one of those
//! enumerations was **hand-written**, so a member joining the previewed set joined
//! it eight times or not at all — the staleness the M46 scope brief names
//! ([DECISIONS.md](../../../DECISIONS.md) → 2026-08-18 M46 planned, F-E: *"the
//! exclusion list becomes **generated rather than hand-listed**"*).
//!
//! This table is that source. Each row carries its **member id**, its canonical
//! **surface fragment**, and the **required token** any surface enumerating its
//! tier must contain. The composed `what's-left:` fragment is *generated* from it
//! ([`whats_left_coverage`]); every other site is *fenced per token*
//! (`crates/cli/tests/gate_coverage_fence.rs`), which is the M48 named-fact-guard
//! shape ([`crate::pack::CONSTRAINT_REQUIRED_TOKENS`]) rather than a verbatim-equality
//! fence — [surface-contract.md](../../../design/surface-contract.md) → the M43 law-3
//! fence-depth **re-open** is exactly this lesson: verbatim equality *"bought
//! unrepresentable drift"* the moment one statement legitimately varied by context,
//! and these five sentences legitimately vary (the tip compresses the carryover gate
//! to *"carryover"*; the catalog hint compresses the whole clause to one line).
//!
//! **Why the later-phase rows carry a tier and not just a reason.** The exclusion is
//! stated at two depths, deliberately: an agent-facing one-liner names the *phases*
//! that decide the rest ([`Tier::LaterSummary`] — the staged set, promotion, the
//! commit), while the two design docs name the *causes* ([`Tier::LaterCause`] — the
//! base pin, the empty-commit guard, `stage-failed`, the untracked `owner-artifact`
//! cause, the commit hook's rejection). A site subscribes to the tiers it enumerates,
//! so neither depth forces the other into a surface it does not belong on.
//!
//! **Declared bound, stated rather than silently narrowed.** The table fences the
//! members every enumerating site must carry; a doc naming one *additional*
//! phase-specific code beside them (`finalize.render-io` and `promote-clobber` in
//! `finalize.md`, the migration review hold in `command-output-contract.md`) is free
//! prose the fence neither requires nor forbids. What it buys is that a member of the
//! *set* cannot join one enumeration and miss the other seven.
//!
//! The **membership** of [`Tier::Previewed`] is owned elsewhere — the checks
//! `TaskArea::preview_gates` actually runs ([validation.md](../../../design/validation.md);
//! [finalize.md](../../../design/finalize.md) → 2. Validate). This table states what the
//! surfaces must say about that membership, and the base pin stays out of the previewed
//! set per the M47 Settle, Decision 1.

/// The depth at which a member is stated on a surface.
///
/// A coverage site subscribes to the tiers it enumerates; the fence then requires
/// every member of those tiers, and nothing else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tier {
    /// The set `jigc task validate` previews — every site that names what the
    /// preview covers must name all of these.
    Previewed,
    /// The later-phase set at the depth an agent-facing surface states it: the
    /// *phases* that decide the rest.
    LaterSummary,
    /// The later-phase set at the depth the design docs state it: the named
    /// *causes*, each with the reason it cannot preview.
    LaterCause,
}

/// Why a member is not previewable at phase 2 — carried as the [`Door`] payload, so
/// a finalize-only row cannot exist without stating its reason.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NotPreviewable {
    /// The state it gates on does not exist yet when phase 2 runs: checking it there
    /// would fire on something `finalize` is about to produce.
    NotYetExistent,
    /// It exists, and is adjudicated at another phase of the same transaction — a
    /// later (or earlier) **position**, never a scope difference.
    LaterPhase,
}

impl NotPreviewable {
    /// The reason as the surfaces state it — so the exclusion list carries *why*,
    /// not merely *that*.
    pub fn stated(self) -> &'static str {
        match self {
            NotPreviewable::NotYetExistent => "does not exist yet when phase 2 runs",
            NotPreviewable::LaterPhase => "decided at another phase of the transaction",
        }
    }
}

/// Which door decides a member.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Door {
    /// Previewed at `jigc task validate` and re-run at `finalize`, same check, same
    /// severity, same exit code.
    Previewed,
    /// Decided only by the real `finalize`, for the carried reason.
    FinalizeOnly(NotPreviewable),
}

/// One member of the finalize transaction's check set, as the surfaces must state it.
#[derive(Clone, Copy, Debug)]
pub struct GateCoverage {
    /// The member's stable id — what a fence failure names.
    pub id: &'static str,
    /// Which door decides it (and, for a finalize-only member, why it cannot preview).
    pub door: Door,
    /// The tiers at which it is stated. A [`Door::Previewed`] member is stated at
    /// exactly [`Tier::Previewed`]; a finalize-only member at one or both later tiers.
    pub tiers: &'static [Tier],
    /// The canonical surface fragment — what [`whats_left_coverage`] renders, and the
    /// prose the compressing sites compress.
    pub fragment: &'static str,
    /// The token a surface enumerating this member's tier must contain, authored in
    /// [`crate::pack::normalized_body`]'s form (lowercase, single-spaced) so a phrase
    /// that wraps across a line or opens a sentence capitalized still matches.
    pub token: &'static str,
}

/// The table. Order is the order the generated fragment renders in.
pub const GATE_COVERAGE: &[GateCoverage] = &[
    GateCoverage {
        id: "content-findings",
        door: Door::Previewed,
        tiers: &[Tier::Previewed],
        fragment: "this task's content findings",
        token: "content findings",
    },
    GateCoverage {
        id: "carryover",
        door: Door::Previewed,
        tiers: &[Tier::Previewed],
        fragment: "the carryover gate",
        token: "carryover",
    },
    GateCoverage {
        id: "owner-artifact-unstaged",
        door: Door::Previewed,
        tiers: &[Tier::Previewed],
        fragment: "the owner-artifact causes that need no staging",
        token: "owner-artifact",
    },
    GateCoverage {
        id: "staged-set",
        door: Door::FinalizeOnly(NotPreviewable::LaterPhase),
        tiers: &[Tier::LaterSummary],
        fragment: "the staged set",
        token: "staged set",
    },
    GateCoverage {
        // Stated at both depths: the one-liners name the phase, `finalize.md` names
        // its failure (`promote-clobber`) — which is why the token is the stem both
        // spellings share.
        id: "promotion",
        door: Door::FinalizeOnly(NotPreviewable::LaterPhase),
        tiers: &[Tier::LaterSummary, Tier::LaterCause],
        fragment: "promotion",
        token: "promot",
    },
    GateCoverage {
        id: "commit-surface",
        door: Door::FinalizeOnly(NotPreviewable::LaterPhase),
        tiers: &[Tier::LaterSummary],
        fragment: "the commit surface at finalize",
        token: "the commit",
    },
    GateCoverage {
        // The preflight's base pin — excluded from the preview by the M47 Settle,
        // Decision 1 (previewing it would flip `task validate` 0 → 3 on a shipped
        // state, against the pinned exit-code taxonomy).
        id: "base-pin",
        door: Door::FinalizeOnly(NotPreviewable::LaterPhase),
        tiers: &[Tier::LaterCause],
        fragment: "the preflight's base pin",
        token: "base pin",
    },
    GateCoverage {
        id: "empty-commit",
        door: Door::FinalizeOnly(NotPreviewable::NotYetExistent),
        tiers: &[Tier::LaterCause],
        fragment: "the empty-commit / nothing-staged guard",
        token: "empty-commit",
    },
    GateCoverage {
        id: "stage-failed",
        door: Door::FinalizeOnly(NotPreviewable::NotYetExistent),
        tiers: &[Tier::LaterCause],
        fragment: "finalize.stage-failed",
        token: "stage-failed",
    },
    GateCoverage {
        id: "owner-artifact-untracked",
        door: Door::FinalizeOnly(NotPreviewable::NotYetExistent),
        tiers: &[Tier::LaterCause],
        fragment: "the untracked owner-artifact cause",
        token: "untracked",
    },
    GateCoverage {
        id: "commit-hook",
        door: Door::FinalizeOnly(NotPreviewable::NotYetExistent),
        tiers: &[Tier::LaterCause],
        fragment: "the commit hook's rejection",
        token: "commit hook",
    },
];

/// The members stated at `tier`, in table order.
pub fn members(tier: Tier) -> impl Iterator<Item = &'static GateCoverage> {
    GATE_COVERAGE
        .iter()
        .filter(move |row| row.tiers.contains(&tier))
}

/// The rows of `rows` whose token is **absent** from `text` — the fence's checker.
///
/// Comparison runs over [`crate::pack::normalized_body`]'s view of `text` (whitespace
/// runs collapsed, ASCII case folded), so a fragment that wraps across a line break or
/// opens a sentence capitalized still matches — it is presentation, not content.
///
/// Taking the rows as an argument rather than reading [`GATE_COVERAGE`] directly is
/// what makes the fence's own redness testable: a synthetic row whose token no site
/// carries must come back reported.
pub fn unmet_in<'a>(
    text: &str,
    rows: impl IntoIterator<Item = &'a GateCoverage>,
) -> Vec<&'a GateCoverage> {
    let normalized = crate::pack::normalized_body(text);
    rows.into_iter()
        .filter(|row| !normalized.contains(row.token))
        .collect()
}

/// The members of `tier` that `text` fails to name.
pub fn unmet(text: &str, tier: Tier) -> Vec<&'static GateCoverage> {
    unmet_in(text, members(tier))
}

/// Join fragments as English prose: `", "` between all but the last, `connector`
/// before it.
fn join(fragments: impl Iterator<Item = &'static str>, connector: &str) -> String {
    let parts: Vec<&str> = fragments.collect();
    match parts.split_last() {
        None => String::new(),
        Some((last, [])) => (*last).to_owned(),
        Some((last, head)) => format!("{}{connector}{last}", head.join(", ")),
    }
}

/// The lead-in every coverage statement opens with on the composed line.
const COVERAGE_LEAD: &str = "previews part of the finalize gate";

/// The composed `what's-left:` line's coverage fragment, generated from the table:
/// the previewed members, then the later-phase phases that decide the rest.
///
/// Generated rather than fenced because this is the one site whose sentence carries
/// the canonical fragments verbatim — the highest-traffic surface (every id-carrying
/// compose), and the one a new member must reach without an author remembering to
/// edit it.
pub fn whats_left_coverage() -> String {
    format!(
        "{COVERAGE_LEAD}: {}; {}",
        join(members(Tier::Previewed).map(|row| row.fragment), ", and "),
        join(members(Tier::LaterSummary).map(|row| row.fragment), " and "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The door and the tiers are two facings of one fact, and they must agree: a
    /// previewed member is stated at exactly `Previewed`, a finalize-only member at
    /// one or both later tiers and never at `Previewed`.
    #[test]
    fn every_row_states_the_door_it_is_decided_at() {
        for row in GATE_COVERAGE {
            let previewed = row.tiers.contains(&Tier::Previewed);
            match row.door {
                Door::Previewed => assert!(
                    previewed && row.tiers.len() == 1,
                    "`{}` is previewed, so it is stated at exactly the previewed tier",
                    row.id,
                ),
                Door::FinalizeOnly(_) => assert!(
                    !previewed && !row.tiers.is_empty(),
                    "`{}` is finalize-only, so it is stated at a later tier and not at the \
                     previewed one",
                    row.id,
                ),
            }
        }
    }

    /// Tokens are compared against the normalized view, so they must be authored in
    /// it — the `crate::pack::constraint_tokens_are_authored_in_normalized_form` rule,
    /// one table over.
    #[test]
    fn tokens_are_authored_in_normalized_form() {
        for row in GATE_COVERAGE {
            assert_eq!(
                crate::pack::normalized_body(row.token),
                row.token,
                "`{}`'s token must be authored lowercase and single-spaced",
                row.id,
            );
        }
    }

    /// A row's token must be findable in its own canonical fragment — otherwise the
    /// generated line would not satisfy the fence it feeds.
    #[test]
    fn every_token_is_carried_by_its_own_fragment() {
        for row in GATE_COVERAGE {
            assert!(
                unmet_in(row.fragment, [row]).is_empty(),
                "`{}`'s fragment does not carry its own token `{}`",
                row.id,
                row.token,
            );
        }
    }

    /// Ids are the fence's failure vocabulary, so they must discriminate.
    #[test]
    fn member_ids_are_unique() {
        let mut ids: Vec<&str> = GATE_COVERAGE.iter().map(|row| row.id).collect();
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(
            before,
            ids.len(),
            "duplicate member id in the coverage table"
        );
    }

    /// The checker reports what is missing — the property the fence rests on, driven
    /// with a synthetic row rather than a shipped one so it stays true of the
    /// *checker*, not of today's table.
    #[test]
    fn the_checker_reports_an_absent_token() {
        const ABSENT: GateCoverage = GateCoverage {
            id: "synthetic-absent",
            door: Door::Previewed,
            tiers: &[Tier::Previewed],
            fragment: "a member no surface names",
            token: "no surface names this",
        };
        let text = "previews part of the finalize gate: this task's content findings";
        let missing = unmet_in(text, [&ABSENT]);
        assert_eq!(
            missing.iter().map(|row| row.id).collect::<Vec<_>>(),
            vec!["synthetic-absent"],
        );
        assert!(unmet_in(ABSENT.fragment, [&ABSENT]).len() == 1);
    }

    /// The generated fragment is the sentence the composed line carried by hand — the
    /// byte-identity this task's whole content is.
    #[test]
    fn the_generated_fragment_is_the_shipped_sentence() {
        assert_eq!(
            whats_left_coverage(),
            "previews part of the finalize gate: this task's content findings, the \
             carryover gate, and the owner-artifact causes that need no staging; the \
             staged set, promotion and the commit surface at finalize",
        );
    }

    /// The join renders English, not a debug list — pinned at the three sizes the
    /// table can reach it at.
    #[test]
    fn the_join_renders_english() {
        assert_eq!(join([].into_iter(), ", and "), "");
        assert_eq!(join(["one"].into_iter(), ", and "), "one");
        assert_eq!(join(["one", "two"].into_iter(), " and "), "one and two");
        assert_eq!(
            join(["one", "two", "three"].into_iter(), ", and "),
            "one, two, and three",
        );
    }
}
