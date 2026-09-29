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
//!
//! M16 Inc 5 T3 EXTENDS the hollowing guard to the **completion** phase steps —
//! the second authoring spine (`design/methodology-docs.md` → Build-time honesty
//! watch (A-3) + the single-agent-spine bound). The completion loop's judgment
//! phases are `audit` (the milestone verdict + gap-detection) and `triage` (the
//! human-gated finding gate). Each must stay pure agent-judgment prose: a
//! human-judgment handback present, free of a `gap-count ≥ N` lint / scored
//! checklist. Sharper than planning, `audit` also carries the **single-agent-spine
//! bound** — the genuine independent audit verdict is an *orchestration-level*
//! responsibility above the single-agent spine; the spine records the verdict, it
//! does NOT certify it. A future edit that mechanizes either judgment phase, or that
//! lets `audit` over-claim the verdict as jigc-computed/certified, goes red here.
//!
//! M17 Inc 5 T3 EXTENDS the hollowing guard to the **recording** step —
//! `author-dogfood-record.yaml`, the measured-dogfood transcription spine
//! (`design/measurement.md` → The three-way cut, applied to measurement: "The
//! engine never opines on whether jigc is helping — an engine opinion on the
//! thesis would be the thesis inverted. Facts are counts; the verdict consumes
//! them in a judgment slot"). The `judgment` slot and the `verdict` enum are the
//! measurement's irreducible judgment seam: the verdict is *authored judgment over
//! the counts, never computed*, and the judgment prose stays the agent's. A future
//! edit that hollows the step into a scored checklist / scoring lint, or that
//! flips the verdict to a computed/derived value, goes red here.

use std::fs;
use std::path::{Path, PathBuf};

/// `<root>/packs/methodology` — `CARGO_MANIFEST_DIR` is `<root>/crates/cli`.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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
fn completion_judgment_phases_stay_prose_and_audit_does_not_over_claim_the_verdict() {
    // FAIL exemplar (a) for the completion loop (methodology-docs.md → Build-time
    // honesty watch, A-3 + the single-agent-spine bound). The `audit` phase (the
    // milestone verdict + gap-detection) and the `triage` phase (the human-gated
    // finding gate) are the loop's irreducible judgment slots. Each must stay
    // judgment prose (a human-judgment / orchestration handback present) and must NOT
    // be mechanized into a `gap-count >= N` lint or a scored checklist. Additionally,
    // `audit` must NOT over-claim: the genuine audit verdict stays an
    // orchestration-level responsibility above this single-agent spine — the spine
    // records the verdict, it does not certify it. A future edit that hollows either
    // phase, or that lets `audit` present the verdict as jigc-computed/certified,
    // goes red here.

    // The numeric-threshold / scoring tokens that mark a hollowed judgment slot — the
    // same falsifiable edge as the planning guard. We do NOT forbid the bare word
    // "checklist": audit DISAVOWS it ("judgment work, not a checklist to tick"), and
    // forbidding the disavowal would false-red.
    let forbidden_mechanizers = ["gap-count", "gap count", ">=", "\u{2265}"];

    // Collapse whitespace runs (incl. the prose's hard line-wraps) to single spaces so
    // a handback phrase split across a wrap still matches its words in order.
    let normalize = |s: &str| {
        s.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    // (audit) the milestone verdict + gap-detection phase. Two obligations:
    //   - judgment prose: the verdict and gap-detection are pure agent prose;
    //   - the single-agent-spine bound: the genuine audit is orchestration-level and
    //     the spine does not certify it (audit must say so, not over-claim).
    let audit = step_text("audit.yaml");
    let audit_lc = normalize(&audit);
    assert!(
        audit_lc.contains("judgment work") && audit_lc.contains("not a checklist"),
        "audit.yaml must stay judgment prose — the milestone verdict + gap-detection are \
         judgment work, NOT a checklist to tick (methodology-docs.md → A-3: the verdict \
         phase stays pure agent-judgment prose); got:\n{audit}",
    );
    assert!(
        audit_lc.contains("orchestration-level") && audit_lc.contains("does not certify"),
        "audit.yaml must observe the single-agent-spine bound — the genuine audit verdict is \
         an orchestration-level responsibility above this spine; the spine records the verdict, \
         it does NOT certify it (methodology-docs.md → the single-agent-spine bound: the \
         completion audit verdict stays orchestration-level, never over-claimed); got:\n{audit}",
    );

    // (triage) the human-gated finding gate: the human owns this gate.
    let triage = step_text("triage.yaml");
    let triage_lc = normalize(&triage);
    assert!(
        triage_lc.contains("human owns") && triage_lc.contains("gate"),
        "triage.yaml must stay the human-in-the-loop judgment gate — the human owns the \
         triage gate, the calls are surfaced not silently decided (methodology-docs.md → A-3: \
         the triage phase stays pure agent-judgment prose); got:\n{triage}",
    );

    // (negative) neither judgment phase may carry a numeric-threshold lint or scored gate
    // — that is the FAIL exemplar (a) hollowing.
    for (name, text) in [("audit.yaml", &audit), ("triage.yaml", &triage)] {
        let lc = text.to_lowercase();
        for forbidden in forbidden_mechanizers {
            assert!(
                !lc.contains(forbidden),
                "{name} must not be hollowed into a numeric-threshold `{forbidden}` lint / scored \
                 gate — the completion judgment stays the agent's (methodology-docs.md → A-3: a \
                 step that emits a `gap-count >= N` lint has hollowed the judgment); got:\n{text}",
            );
        }
    }
}

#[test]
fn recording_judgment_stays_prose_and_the_verdict_is_authored_never_computed() {
    // FAIL exemplar (a) for the recording spine (measurement.md → The three-way cut,
    // applied to measurement: the engine never opines on the thesis). The
    // `author-dogfood-record` step transcribes mechanical counts, but the `verdict`
    // enum and the `judgment` slot are the measurement's irreducible judgment seam —
    // an engine-computed verdict, a scoring lint over the counts, or a scored
    // checklist replacing the judgment prose would be the thesis inverted. A future
    // edit that hollows the step that way goes red here.

    // Collapse whitespace runs (incl. the prose's hard line-wraps) to single spaces
    // so a handback phrase split across a wrap still matches its words in order.
    let normalize = |s: &str| {
        s.to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    let record = step_text("author-dogfood-record.yaml");
    let record_lc = normalize(&record);

    // (positive) the verdict-stays-authored disavowal is present — the honest tell
    // that the verdict is judgment over the counts, never derived from them. If the
    // verdict were flipped to a computed/derived value, this disavowal would be gone.
    assert!(
        record_lc.contains("authored judgment over the counts, never computed"),
        "author-dogfood-record.yaml must keep the verdict AUTHORED — judgment over the \
         counts, never computed (measurement.md → the three-way cut: the verdict consumes \
         the facts in a judgment slot; the engine never opines on the thesis); got:\n{record}",
    );

    // (positive) the judgment handback is present — the step hands the agent the
    // judgment slot as prose (what the counts mean), authored via `set-slot`, not a
    // form to fill against a rubric.
    assert!(
        record_lc.contains("what the counts mean") && record.contains("#judgment"),
        "author-dogfood-record.yaml must hand the `judgment` slot back as prose — what the \
         counts mean, authored via set-slot (measurement.md → one prose slot: judgment); \
         got:\n{record}",
    );

    // (negative) no hollowing into a scoring lint / scored checklist / numeric-
    // threshold gate over the counts — the FAIL exemplar (a) hollowing, applied to
    // the recording seam. ("score" also catches "scored"/"scoring".)
    for forbidden in [
        "gap-count",
        "gap count",
        ">=",
        "\u{2265}",
        "checklist",
        "score",
    ] {
        assert!(
            !record_lc.contains(forbidden),
            "author-dogfood-record.yaml must not be hollowed into a `{forbidden}` scoring \
             lint / scored checklist — the recording judgment stays the agent's \
             (measurement.md → facts are counts; the verdict stays judgment over them); \
             got:\n{record}",
        );
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
