//! **Pinned fact** (RC-alpha4 [findings-verification.md](../../../../completions/artifacts/RC-alpha4/findings-verification.md)
//! → **C1**): *"`doc show` dumps the whole document; the set-slot-symmetric address
//! doesn't work"* — **REFUTED as a capability gap** (the confirmed half is the
//! *route* the failure prints, fixed at M47 Inc 10 T4 and pinned there).
//!
//! The trial's P2 concluded no address-scoped read existed and fell back to
//! `sed -n '290,320p'` over the managed file — the AGENT.md violation the wave's
//! read rule exists to prevent. The refutation: **every documented depth resolves**
//! ([doc-read-surface.md](../../../../design/doc-read-surface.md) → The slice
//! grammar), and the address P2 actually ran elided the **section** segment, putting
//! an item id in the section position. That is not a read/write asymmetry: the
//! identical elision fails on the **write** side too, which is the negative control
//! that makes this a fact rather than a happy path.
//!
//! The repro block, made standing:
//!
//! ```yaml
//! claim: "doc show has no address-scoped read; the set-slot-symmetric address does not work"
//! verdict: REFUTED (capability half)
//! setup: [ { fixture: committed-singletons } ]
//! repro:
//!   - ["jigc","doc","show","roadmap:roadmap"]                                  # whole doc
//!   - ["jigc","doc","show","roadmap:roadmap#milestones"]                       # repeatable section
//!   - ["jigc","doc","show","roadmap:roadmap#meta"]                             # fields-only section
//!   - ["jigc","doc","show","roadmap:roadmap#meta/schema-version"]              # field leaf
//!   - ["jigc","doc","show","roadmap:roadmap#milestones/<id>"]                  # item
//!   - ["jigc","doc","show","roadmap:roadmap#milestones/<id>/proves"]           # item leaf
//!   - ["jigc","doc","show","changelog:changelog#releases/<r>/changes/<g>"]     # nested item
//!   - ["jigc","doc","show","changelog:changelog#releases/<r>/changes/<g>/notes"] # nested leaf
//!   - ["jigc","doc","show","roadmap#milestones/<id>"]                          # bare singleton + fragment
//! expect:
//!   exit: 0 for every depth; stdout carries the addressed node alone
//! negative-control:
//!   - ["jigc","doc","show","roadmap:roadmap#<id>"]        # section elided → exit 1, store.no-such-section
//!   - ["jigc","doc","set-slot","roadmap:roadmap#<id>", …] # the SAME elision, exit non-zero on the WRITE side
//! ```
//!
//! **Fact drift, not contract drift.** `doc_read_surface.rs` pins the *contract* —
//! the address grammar as a registry-derived projection↔write parity property. This
//! pins the *trial claim*: the specific depths P2 believed unreachable, read over the
//! trial-shaped corpus, with the elision that actually failed shown to fail on both
//! sides (pinning.md §3).

use crate::support::trial_corpus::{State, TrialCorpus};
use serde_json::Value;

/// A served read: exit 0 with a non-empty stdout carrying the addressed node.
fn show(corpus: &TrialCorpus, address: &str) -> String {
    let out = corpus.jigc(&["doc", "show", address]);
    assert!(
        out.status.success(),
        "REFUTED (C1): `doc show {address}` must resolve — every documented depth is \
         addressable; status {}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        !stdout.trim().is_empty(),
        "`doc show {address}` served an EMPTY node at exit 0 — the wrong-node-exit-0 \
         the read surface forbids by name",
    );
    stdout
}

/// The whole doc as the pinned json contract projects it — the source of every id
/// this suite addresses, so no test-side reconstruction of the slug rule happens.
fn show_json(corpus: &TrialCorpus, address: &str) -> Value {
    let stdout = corpus.jigc_ok(&["doc", "show", address, "--format", "json"]);
    serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!("`doc show {address} --format json` parses ({e}); got:\n{stdout}")
    })
}

/// The first item id under `section`, read out of the doc's own json projection.
fn first_item_id(doc: &Value, section: &str) -> String {
    doc["sections"][section][0]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("`sections.{section}[0].id` is a string; got:\n{doc:#}"))
        .to_string()
}

/// **The pin.** Every depth the slice grammar documents resolves over the
/// trial-shaped corpus — whole-doc · repeatable section · fields-only section · field
/// leaf · item · item leaf · nested item · nested leaf — plus the bare-singleton
/// alias, which is byte-identical to its type-qualified twin.
#[test]
fn every_documented_address_depth_resolves() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);

    let roadmap = show_json(&corpus, "roadmap:roadmap");
    let milestone = first_item_id(&roadmap, "milestones");
    let changelog = show_json(&corpus, "changelog:changelog");
    let release = first_item_id(&changelog, "releases");
    let group = changelog["sections"]["releases"][0]["changes"][0]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the nested change-group's id is a string; got:\n{changelog:#}"))
        .to_string();

    // (1) whole-doc — the only address that "dumps the whole document", by design.
    let whole = show(&corpus, "roadmap:roadmap");
    assert!(
        whole.contains("M-Alpha"),
        "the whole-doc read carries the milestone; got:\n{whole}",
    );

    // (2) `#section`, all three section kinds the grammar's row 2 covers.
    let repeatable = show(&corpus, "roadmap:roadmap#milestones");
    assert!(
        repeatable.contains("M-Alpha") && !repeatable.contains("# Roadmap"),
        "the repeatable-section slice serves its items and NOT the whole doc \
         (P2's complaint); got:\n{repeatable}",
    );
    let slot = show(&corpus, "vision:vision#thesis");
    assert!(
        slot.contains("deterministic CLI"),
        "the slot-section slice serves its prose; got:\n{slot}",
    );
    let fields_only = show(&corpus, "roadmap:roadmap#meta");
    assert!(
        fields_only.contains("schema-version"),
        "the fields-only (header) section slice serves its fields (M42); got:\n{fields_only}",
    );

    // (3) `#section/<leaf>` — a leaf inside a NON-repeatable section, the depth the
    //     tool's own findings emit. Parity with the whole-doc projection is what
    //     makes this "the right node" rather than "some bytes at exit 0".
    let field_leaf = show(&corpus, "roadmap:roadmap#meta/schema-version");
    assert_eq!(
        field_leaf.trim(),
        roadmap["fields"]["schema-version"]
            .as_str()
            .expect("the stamp is projected as a string"),
        "the field-leaf slice serves the same value the whole-doc projection carries",
    );

    // (4) + (5) item and item leaf.
    let item = show(&corpus, &format!("roadmap:roadmap#milestones/{milestone}"));
    assert!(
        item.contains("M-Alpha") && item.contains(&format!("{{#{milestone}}}")),
        "the item slice re-heads with the `{{#id}}` anchor that IS its address id; got:\n{item}",
    );
    let item_leaf = show(
        &corpus,
        &format!("roadmap:roadmap#milestones/{milestone}/proves"),
    );
    assert_eq!(
        item_leaf.trim(),
        "That the composed loop lands one task end to end.",
        "the item-leaf slice serves that leaf's prose alone",
    );

    // (6) + (7) the nested depths (M40) — a nested repeatable's member and its leaf.
    let nested = show(
        &corpus,
        &format!("changelog:changelog#releases/{release}/changes/{group}"),
    );
    assert!(
        nested.contains("the trial-shaped fixture builder"),
        "the nested-item slice serves the group's own notes; got:\n{nested}",
    );
    let nested_leaf = show(
        &corpus,
        &format!("changelog:changelog#releases/{release}/changes/{group}/notes"),
    );
    assert_eq!(
        nested_leaf.trim(),
        "- the trial-shaped fixture builder",
        "the nested-leaf slice serves the leaf alone",
    );

    // (8) The bare-singleton alias resolves at the same depths, byte-identically.
    assert_eq!(
        show(&corpus, &format!("roadmap#milestones/{milestone}")),
        item,
        "a bare singleton address + fragment is the same read as its type-qualified twin",
    );
}

/// **The negative control that makes the pin a fact.** The address P2 actually ran
/// elides the section segment (`#<item>` where the grammar is `#<section>/<item>`).
/// It fails on the read side — and the **identical** elision fails on the write side,
/// so the claimed read/write asymmetry does not exist. The correctly-formed write
/// address on the same doc is accepted in the same task, which is what keeps this a
/// statement about the *address* rather than about a refusing surface.
#[test]
fn the_section_eliding_address_fails_identically_on_both_sides() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let roadmap = show_json(&corpus, "roadmap:roadmap");
    let milestone = first_item_id(&roadmap, "milestones");
    let elided = format!("roadmap:roadmap#{milestone}");

    // Read side: the item id in the section position names no section.
    let read = corpus.jigc(&["doc", "show", &elided]);
    assert!(
        !read.status.success(),
        "the section-eliding read must fail — it is the address that was wrong, not \
         the surface; stdout:\n{}",
        String::from_utf8_lossy(&read.stdout),
    );
    let read_err = String::from_utf8(read.stderr).expect("utf-8 stderr");
    assert!(
        read_err.contains("store.no-such-section"),
        "the elided read fails as a MISSING SECTION (the diagnosis P2 never got); \
         got:\n{read_err}",
    );

    // Write side: the same elision, the same refusal — no asymmetry to explain.
    let task = corpus.start_workflow("planning", "probe the elided address");
    let write = corpus.jigc_stdin(
        &[
            "doc",
            "set-slot",
            &elided,
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "prose that must never land",
    );
    assert!(
        !write.status.success(),
        "the SAME elided address must fail on the write side too — that is the whole \
         refutation of the claimed asymmetry; stdout:\n{}",
        String::from_utf8_lossy(&write.stdout),
    );

    // ...while the correctly-formed write address lands in that same task, so the
    // refusal above is about the address and nothing else.
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("roadmap:roadmap#milestones/{milestone}/proves"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "That the composed loop lands one task end to end, restated.",
    );
}
