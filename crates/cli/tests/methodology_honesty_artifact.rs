//! T4 — the honesty artifact: the authored methodology steps match the
//! `design/self-hosting.md` three-way determinism sort + named-trigger list
//! (M12 Inc 1; `design/self-hosting.md` → The honest caveat, The three-way
//! determinism cut, The named dialect-extension trigger).
//!
//! The encode's honesty is part of the Deliverable, and the acceptance bar cuts
//! **both ways** (self-hosting.md → success bar #2's two FAIL exemplars). This is
//! the **falsifiable check** that the authored steps stay on the side the
//! forcing-function sorts them to. It reads the *authored* step files + the doc
//! and asserts:
//!
//!   (a) **Hollowing absent** (FAIL exemplar (a)). A judgment slot must not be
//!       mechanized into a lint/checklist, and a portable prose gate must not
//!       hardcode a toolchain. Concretely: `gate.yaml` carries no hardcoded
//!       `cargo`/`pnpm`/`npm` gate command (it is portable prose), `implement.yaml`
//!       carries no ordering-enforcement structure — it is pure prose, explicitly
//!       handing the test-first ordering back to the agent — and `scope.yaml` stays
//!       judgment prose (restate + stop-and-check the human), not a gap-count lint or
//!       a scored checklist (the scope step is named in FAIL exemplar (a) directly).
//!
//!   (b) **Prose-wash-without-naming absent** (FAIL exemplar (b)). Every
//!       *prose-washed mechanizable* the authored steps actually surface — the
//!       test-first (red-before-green) ordering and the gate command — must appear
//!       in self-hosting.md's named-trigger list, not be silently dropped. The
//!       implement step prose-washes test-first ordering → trigger #1 (the
//!       ordering/assertion gate) must be named; the gate step prose-washes the
//!       gate command → trigger #3 (the portable gate-command knob) must be named.
//!
//! These are structural assertions over the committed pack data + doc, with **zero
//! `crates/*/src` changes** — if a later edit mechanizes a judgment step, or ships
//! a prose-wash the doc does not name, this test goes red.
//!
//! M16 Inc 4 T3 EXTENDS the hollowing guard to the **planning** phase steps
//! (`design/methodology-docs.md` → Build-time honesty watch, review finding A-3:
//! "the encoded planning/completion steps must keep their gap-detection / settle /
//! verdict phases as *pure agent-judgment prose* — a step that emits a
//! `gap-count ≥ N` lint or a structured checklist the agent must satisfy has
//! *hollowed* the judgment"). The planning loop's judgment phases — `settle`,
//! `detect-gaps`, `plan-review` — each carry a human-judgment handback and must
//! stay free of a numeric-threshold lint. A future edit that mechanizes any of
//! them into a `gap-count ≥ N` / scored gate goes red here.

use std::fs;
use std::path::{Path, PathBuf};

/// `<root>/packs/methodology` — `CARGO_MANIFEST_DIR` is `<root>/crates/cli`.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// `<root>/design/self-hosting.md` — the locked honesty spec.
fn self_hosting_doc() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("design")
        .join("self-hosting.md")
}

fn step_text(name: &str) -> String {
    let path = methodology_pack_tree().join("steps").join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn hollowing_absent_gate_is_portable_prose_and_implement_carries_no_ordering_structure() {
    // FAIL exemplar (a): a judgment slot mechanized, or the portable gate hardcoding
    // a toolchain. The gate step must read principle-first (graduation pass part 1)
    // and the implement step must keep test-first ordering as the agent's prose.

    // (a.i) The gate step must NOT hardcode a concrete gate command — it is portable
    // prose ("run your project's own gate"). Hardcoding jigc's own `cargo` stack (or
    // any specific toolchain) would prose-wash *and* break portability.
    let gate = step_text("gate.yaml").to_lowercase();
    for forbidden in ["cargo", "pnpm", "npm ", "npx", "yarn"] {
        assert!(
            !gate.contains(forbidden),
            "gate.yaml must be portable prose — it must NOT hardcode the `{}` toolchain \
             (self-hosting.md → graduation pass part 1: the gate step must not hardcode `cargo`); \
             got:\n{}",
            forbidden.trim(),
            step_text("gate.yaml"),
        );
    }

    // (a.ii) The implement step must carry no ordering-ENFORCEMENT structure: it is
    // pure prose handing test-first back to the agent. The honest tell is that the
    // step explicitly says nothing enforces the ordering — i.e. it does NOT claim a
    // mechanical gate it does not have, and it does NOT ship a structured
    // checklist/lint the agent must satisfy. The absence of an enforcement claim is
    // the falsifiable signal that the mechanizable was prose-washed honestly.
    let implement = step_text("implement.yaml");
    let implement_lc = implement.to_lowercase();
    assert!(
        implement_lc.contains("nothing here enforces") || implement_lc.contains("yours to police"),
        "implement.yaml must hand the test-first ordering back to the agent as prose, \
         explicitly stating nothing enforces it (self-hosting.md → sort table Red row: \
         prose-washed, not falsely presented as a structural TDD gate); got:\n{implement}",
    );
}

#[test]
fn scope_step_is_judgment_prose_not_a_hollowed_checklist() {
    // FAIL exemplar (a), scope coverage. self-hosting.md success-bar #2 FAIL exemplar
    // (a) names the scope step explicitly ("the scope step emits a gap-count >= N lint
    // or a structured checklist"), but the hollowing guard above only covers gate.yaml
    // + implement.yaml. The scope step is the milestone's primary judgment slot
    // (restate the intent, name a done-criterion, stop-and-check the human on drift —
    // irreducibly the agent's), so the falsifiable check must guard it too: a future
    // edit that mechanizes scope into a scored checklist must go red here.
    let scope = step_text("scope.yaml");
    let scope_lc = scope.to_lowercase();

    // (positive) the judgment handback is present — restate in your own words + defer to
    // the human on scope drift. If scope were hollowed into a CLI-scored gate, this
    // human-judgment handback would be gone.
    assert!(
        scope_lc.contains("your own words") && scope_lc.contains("stop and check"),
        "scope.yaml must stay judgment prose — restate the intent in your own words and \
         stop-and-check the human on scope drift (self-hosting.md → sort table Scope rows: \
         irreducibly the agent's); got:\n{scope}",
    );

    // (negative) no hollowing into a gap-count / numeric-threshold lint or a scored
    // checklist (self-hosting.md → success bar #2 FAIL exemplar (a)).
    for forbidden in ["gap-count", "gap count", ">=", "checklist"] {
        assert!(
            !scope_lc.contains(forbidden),
            "scope.yaml must not be hollowed into a structured `{forbidden}` lint/checklist \
             — the scope judgment stays the agent's (self-hosting.md → bar #2 FAIL exemplar (a)); \
             got:\n{scope}",
        );
    }
}

#[test]
fn planning_judgment_phases_stay_prose_not_a_gap_count_or_scored_checklist() {
    // FAIL exemplar (a) for the planning loop (methodology-docs.md → Build-time
    // honesty watch, A-3). The settle / detect-gaps / plan-review phases are the
    // milestone's irreducible judgment slots — gap-detection, the human-gated
    // settle, and the independent adversarial review. Each must stay judgment prose
    // (a human-judgment handback present) and must NOT be mechanized into a
    // `gap-count >= N` lint or a scored checklist the agent must satisfy. A future
    // edit that hollows any of them goes red here.

    // The numeric-threshold / scoring tokens that mark a hollowed judgment slot. We
    // do NOT forbid the bare word "checklist": detect-gaps DISAVOWS it ("judgment
    // work, not a checklist to tick"), and forbidding the disavowal would false-red.
    // The honest tell is a numeric gate (`gap-count >= N`), which never appears in
    // honest judgment prose; that token set is the falsifiable edge.
    let forbidden_mechanizers = ["gap-count", "gap count", ">=", "\u{2265}"];

    // Collapse whitespace runs (incl. the prose's hard line-wraps) to single spaces
    // so a handback phrase split across a wrap still matches its words in order.
    let normalize = |s: &str| {
        s.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    // (settle) the human-gated checkpoint phase: the human owns the gate.
    let settle = step_text("settle.yaml");
    let settle_lc = normalize(&settle);
    assert!(
        settle_lc.contains("human owns") && settle_lc.contains("judgment"),
        "settle.yaml must stay the human-in-the-loop judgment gate — the human owns it and \
         the judgment calls are surfaced, not silently decided (methodology-docs.md → A-3: \
         the settle phase stays pure agent-judgment prose); got:\n{settle}",
    );

    // (detect-gaps) the adversarial gap pass: explicitly judgment work, not a checklist.
    let detect = step_text("detect-gaps.yaml");
    let detect_lc = normalize(&detect);
    assert!(
        detect_lc.contains("judgment work") && detect_lc.contains("not a checklist"),
        "detect-gaps.yaml must stay judgment prose — it is judgment work, NOT a checklist to \
         tick (methodology-docs.md → A-3: gap-detection stays pure agent-judgment prose); \
         got:\n{detect}",
    );

    // (plan-review) the independent adversarial review: the human owns each finding.
    let review = step_text("plan-review.yaml");
    let review_lc = normalize(&review);
    assert!(
        review_lc.contains("human owns") && review_lc.contains("adversari"),
        "plan-review.yaml must stay the independent adversarial-judgment phase — the human \
         owns accepting or rejecting each finding (methodology-docs.md → A-3: the review/verdict \
         phase stays pure agent-judgment prose); got:\n{review}",
    );

    // (negative) none of the three judgment phases may carry a numeric-threshold lint
    // or scored gate — that is the FAIL exemplar (a) hollowing.
    for (name, text) in [
        ("settle.yaml", &settle),
        ("detect-gaps.yaml", &detect),
        ("plan-review.yaml", &review),
    ] {
        let lc = text.to_lowercase();
        for forbidden in forbidden_mechanizers {
            assert!(
                !lc.contains(forbidden),
                "{name} must not be hollowed into a numeric-threshold `{forbidden}` lint / scored \
                 gate — the planning judgment stays the agent's (methodology-docs.md → A-3: a step \
                 that emits a `gap-count >= N` lint has hollowed the judgment); got:\n{text}",
            );
        }
    }
}

#[test]
fn every_authored_prose_wash_is_named_in_the_self_hosting_trigger_list() {
    // FAIL exemplar (b): an authored prose-wash that the milestone record does NOT
    // name as a dialect-extension trigger. We read the doc's named-trigger section
    // and assert each mechanizable the authored steps surface appears there.
    let doc = fs::read_to_string(self_hosting_doc()).expect("read self-hosting.md");

    // Isolate the named-trigger list section so an incidental mention elsewhere does
    // not count as "named". The section runs from its heading to the next `## `.
    let trigger_heading = "## The named dialect-extension trigger";
    let start = doc
        .find(trigger_heading)
        .expect("self-hosting.md must carry the named dialect-extension trigger section");
    let rest = &doc[start..];
    let end = rest[trigger_heading.len()..]
        .find("\n## ")
        .map(|i| trigger_heading.len() + i)
        .unwrap_or(rest.len());
    let triggers = rest[..end].to_lowercase();

    // The implement step prose-washes test-first (red-before-green) ordering → the
    // ordering/assertion gate must be named (trigger #1, the headline).
    assert!(
        triggers.contains("ordering") && triggers.contains("assert"),
        "the authored implement.yaml prose-washes test-first ordering, so self-hosting.md's \
         named-trigger list MUST name an ordering/assertion gate (trigger #1); \
         section:\n{}",
        &rest[..end],
    );

    // The gate step prose-washes the concrete gate command → the portable
    // gate-command knob must be named (trigger #3).
    assert!(
        triggers.contains("gate-command knob") || triggers.contains("gate command knob"),
        "the authored gate.yaml prose-washes the concrete gate command, so self-hosting.md's \
         named-trigger list MUST name a portable gate-command knob (trigger #3); \
         section:\n{}",
        &rest[..end],
    );
}
