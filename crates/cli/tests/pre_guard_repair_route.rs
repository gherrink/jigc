//! **The pre-guard repair route — the diagnosis names only repairs that run**
//! (M46 Increment 5 / T1; [`implementation/roadmap.md`] → Milestone 46, Increment 5,
//! grouped-scope bullet 1).
//!
//! `conformance.item-heading-unanchored` diagnoses a heading sitting at the depth the
//! schema reserves for item structure with no `{#id}` anchor. The two readings are
//! byte-identical — stray slot prose that broke out of its slot, or a genuinely
//! anchor-less new item — so the message carries **both** repairs. Until M46 the
//! new-item repair was *"mint it with `jigc doc add-item` (which writes the anchor)"*,
//! and that verb **cannot run on the doc the message is about**: the corruption is
//! exactly what stops the parse, so `add-item` at the containing section answers an
//! unrelated `write.wrong-shape` (the M46 scope audit, N-4).
//!
//! The finding has **one producer** (`engine::parse::unanchored_heading_message`) and
//! **three carriers**, each driven here through the real binary against a corpus whose
//! corruption was committed the way a pre-guard corpus carries it — a hand edit at a
//! `###` heading inside item prose:
//!
//! 1. `jigc validate` (store scope) prints it under `conformance.item-heading-unanchored`;
//! 2. a task door prints it as the detail of a **blocking** `reconciliation.conformance-block`
//!    (the `DRIFTED + UNTOUCHED` arm, baseline present);
//! 3. after `jigc unmanage <path>` the same words arrive on the **advisory**
//!    `reconciliation.conformance-block` (the `UNKNOWN` arm, no baseline).
//!
//! The suite drives the **emitted bytes**, never a reconstruction: each repair arm
//! lifts its repair *out of the message the binary printed* — the demote depth from
//! ``demote it to `####` or deeper``, the anchored heading from the backticked form the
//! message shows — and applies exactly that to the file. A test that hand-built the
//! `####` or the `{#id}` would pass while the emitted repair was wrong.
//!
//! [`implementation/roadmap.md`]: ../../../implementation/roadmap.md

use crate::support;

use std::fs;
use support::trial_corpus::{State, TrialCorpus};

/// The committed managed doc the corruption is planted in.
const SPEC_PATH: &str = "docs/specs/rate-limiter.md";

/// The hand-planted heading — at `###`, the depth `spec#criteria` reserves for items,
/// and carrying no `{#id}` anchor.
const CORRUPT_HEADING: &str = "### Rationale for the cap";

/// The prose under the planted heading. It is a complete criterion statement, so the
/// *new-item* reading of the diagnosis is a genuine reading of these bytes: anchored,
/// the heading and this line are a conformant `criteria` item.
const CORRUPT_PROSE: &str = "The cap keeps the gateway responsive.";

/// The finding the store sweep raises over the corrupted doc.
const CODE: &str = "conformance.item-heading-unanchored";

/// The verb the message named until M46 — the string no carrier may print.
const RETIRED_VERB: &str = "jigc doc add-item";

/// A corpus carrying one committed, managed `spec` — then hand-broken and committed
/// again, which is the shape a corpus corrupted before M45's write guard shipped
/// carries at rest.
fn corrupted_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("plan", "spec the rate limiter");
    let id = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "spec",
            "--title",
            "Rate limiter",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#goal"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "Cap bursts at the configured rate.",
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#context"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "The gateway has no limiter today.",
    );
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{id}#criteria"),
            "--title",
            "Burst limit",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{item}/statement"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "A burst beyond the cap is rejected.",
    );
    corpus.finalize(&task, "spec", "spec the rate limiter", false);

    // The hand break, out of band and committed — the CLI never wrote these bytes.
    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the committed spec is readable");
    let broken = body.replace(
        "A burst beyond the cap is rejected.\n",
        &format!("A burst beyond the cap is rejected.\n\n{CORRUPT_HEADING}\n\n{CORRUPT_PROSE}\n"),
    );
    assert_ne!(body, broken, "the hand break must land");
    fs::write(&path, broken).expect("write the hand-broken spec");
    corpus.git(&["add", SPEC_PATH]);
    corpus.git(&[
        "commit",
        "-q",
        "-m",
        "hand-edit the spec (pre-guard corruption)",
    ]);
    corpus
}

/// Both streams of an invocation, so a finding printed to stderr (the blocking read
/// doors) is searched exactly like one printed to stdout.
fn both_streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// The emitted diagnosis, lifted out of a carrier's output from the producer's own
/// first byte — so what is asserted on is the message the engine composed, stripped of
/// whichever carrier's prefix wrapped it.
fn diagnosis(carrier: &str, output: &str) -> String {
    let anchor = format!("`{CORRUPT_HEADING}` sits at");
    let start = output.find(&anchor).unwrap_or_else(|| {
        panic!(
            "the `{carrier}` carrier must print the unanchored-heading diagnosis; got:\n{output}"
        )
    });
    let rest = &output[start..];
    let end = rest.find('\n').unwrap_or(rest.len());
    rest[..end].trim_end().to_string()
}

/// Carrier 1 — `jigc validate` at store scope.
fn store_carrier(corpus: &TrialCorpus) -> String {
    let out = corpus.jigc(&["validate"]);
    let text = both_streams(&out);
    assert!(
        text.contains(CODE),
        "the store sweep must raise `{CODE}`; got:\n{text}"
    );
    text
}

/// Carrier 2 — a task door, baseline present: the **blocking** conformance-block. The
/// task deliberately touches nothing, which is the `DRIFTED + UNTOUCHED` arm.
fn task_door_carrier(corpus: &TrialCorpus, intent: &str) -> String {
    let task = corpus.start_workflow("plan", intent);
    let text = both_streams(&corpus.jigc(&["task", "validate", &task]));
    assert!(
        text.contains("blocking · reconciliation.conformance-block"),
        "a task door must raise the blocking conformance-block; got:\n{text}"
    );
    text
}

/// Carrier 3 — the same task door **after `jigc unmanage`**: no baseline, so the
/// `UNKNOWN` arm's advisory carries the identical words.
fn post_unmanage_carrier(corpus: &TrialCorpus, intent: &str) -> String {
    corpus.jigc_ok(&["unmanage", SPEC_PATH]);
    let task = corpus.start_workflow("plan", intent);
    let text = both_streams(&corpus.jigc(&["task", "validate", &task]));
    assert!(
        text.contains("advisory · reconciliation.conformance-block"),
        "after `unmanage` the same words must arrive as the advisory; got:\n{text}"
    );
    text
}

/// The demote target the message itself names, lifted out of ``demote it to `####` or
/// deeper`` — never the depth this file thinks is right.
fn demote_depth(diagnosis: &str) -> String {
    let (_, rest) = diagnosis
        .split_once("demote it to `")
        .unwrap_or_else(|| panic!("the diagnosis must name a demote depth; got:\n{diagnosis}"));
    let (depth, _) = rest
        .split_once('`')
        .unwrap_or_else(|| panic!("the demote depth must be backticked; got:\n{diagnosis}"));
    assert!(
        depth.len() > 3 && depth.chars().all(|c| c == '#'),
        "the demote depth must be deeper than the reserved `###`; got `{depth}`"
    );
    depth.to_string()
}

/// The anchored heading the message itself shows for the new-item reading, with its
/// `<id>` placeholder filled — the *emitted* repair, not a reconstruction of one.
///
/// This is the arm that was unreachable before M46: the message named a verb, so there
/// was no anchored form to lift, and the extraction below is what reddens.
fn anchored_heading(diagnosis: &str, id: &str) -> String {
    let form = diagnosis
        .split('`')
        .find(|piece| piece.starts_with("###") && piece.contains("{#<id>}"))
        .unwrap_or_else(|| {
            panic!(
                "the new-item repair must be the anchored heading to write at that line \
                 (a backticked `### … {{#<id>}}` form), never a verb to run; got:\n{diagnosis}"
            )
        });
    form.replace("<id>", id)
}

/// Arm (a) — one producer, three carriers, and the retired verb at none of them.
#[test]
fn no_carrier_of_the_diagnosis_names_a_verb_the_state_refuses() {
    let corpus = corrupted_corpus();

    let store = diagnosis("jigc validate", &store_carrier(&corpus));
    let task_door = diagnosis("task door", &task_door_carrier(&corpus, "a second spec"));
    // `unmanage` mutates the corpus, so this carrier runs last.
    let unmanaged = diagnosis(
        "post-unmanage advisory",
        &post_unmanage_carrier(&corpus, "a third spec"),
    );

    for (carrier, text) in [
        ("jigc validate", &store),
        ("task door", &task_door),
        ("post-unmanage advisory", &unmanaged),
    ] {
        assert!(
            !text.contains(RETIRED_VERB),
            "the `{carrier}` carrier still names `{RETIRED_VERB}`, a verb this state \
             refuses (`write.wrong-shape`); got:\n{text}"
        );
    }

    // One producer: whatever the message says, every carrier says the same thing, so a
    // repair proven at one carrier is proven at all three.
    assert_eq!(
        store, task_door,
        "the store and task-door carriers must print one diagnosis"
    );
    assert_eq!(
        store, unmanaged,
        "the post-`unmanage` advisory must print the same diagnosis"
    );
}

/// Arm (b), the slot-prose reading — the demote the message names, driven through the
/// shipped repair chain (`jigc migrate` → `doc author` → review hold → `--approve`) to a
/// clean `jigc validate`.
///
/// The chain runs from the standing point that has one: `unmanage` first, so the file
/// carries no stale baseline (the baseline-present arm is Increment 5 / T2's).
#[test]
fn the_demoted_heading_lands_through_the_shipped_chain() {
    let corpus = corrupted_corpus();
    let depth = demote_depth(&diagnosis(
        "post-unmanage advisory",
        &post_unmanage_carrier(&corpus, "a third spec"),
    ));

    // The repair: exactly the depth the message named, at exactly the line it named.
    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the corrupted spec is readable");
    let demoted_heading = CORRUPT_HEADING.replacen("###", &depth, 1);
    fs::write(&path, body.replace(CORRUPT_HEADING, &demoted_heading))
        .expect("write the demoted heading");

    let minted = corpus.jigc_ok(&["migrate", SPEC_PATH, "--as", "spec"]);
    let task = minted
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("`jigc migrate` mints a task")
        .trim()
        .to_string();
    corpus.jigc_stdin_ok(
        &["doc", "author", "spec", "--from-file", "-", "--task", &task],
        &format!(
            "title: \"Rate limiter\"\n\
             sections:\n\
             \x20 - id: goal\n\
             \x20   set:\n\
             \x20     goal: |-\n\
             \x20       <<Cap bursts at the configured rate.>>\n\
             \x20 - id: context\n\
             \x20   set:\n\
             \x20     context: |-\n\
             \x20       <<The gateway has no limiter today.>>\n\
             \x20 - id: criteria\n\
             \x20   items:\n\
             \x20     - title: \"Burst limit\"\n\
             \x20       set:\n\
             \x20         statement: |-\n\
             \x20           <<A burst beyond the cap is rejected.\n\
             \n\
             \x20           {demoted_heading}\n\
             \n\
             \x20           {CORRUPT_PROSE}>>\n"
        ),
    );

    // The migration review hold, then the one destructive gate.
    let hold = corpus.jigc(&["task", "finalize", &task]);
    assert_eq!(
        hold.status.code(),
        Some(4),
        "a migration finalize holds for review; got:\n{}",
        both_streams(&hold)
    );
    let approved = corpus.jigc(&["task", "finalize", &task, "--approve"]);
    assert!(
        approved.status.success(),
        "the approved migration must land; got:\n{}",
        both_streams(&approved)
    );

    let swept = both_streams(&corpus.jigc(&["validate"]));
    assert!(
        swept.contains("no findings — the committed store validates clean"),
        "the demote repair must leave the store clean; got:\n{swept}"
    );
    // The demoted line survived the rewrite as slot prose — the reading the repair claims.
    let landed = both_streams(&corpus.jigc(&["doc", "show", "spec:rate-limiter"]));
    assert!(
        landed.contains(&demoted_heading),
        "the demoted heading must land as slot prose; got:\n{landed}"
    );
}

/// Arm (b), the new-item reading — the anchored heading the message shows, written at
/// that line, is a conformant out-of-band edit: no task touches the doc, and the
/// reconciler **absorbs** it.
#[test]
fn the_anchored_heading_is_absorbed_and_becomes_a_real_item() {
    let corpus = corrupted_corpus();
    let anchored = anchored_heading(
        &diagnosis("jigc validate", &store_carrier(&corpus)),
        "rationale-for-the-cap",
    );

    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the corrupted spec is readable");
    fs::write(&path, body.replace(CORRUPT_HEADING, &anchored)).expect("write the anchored heading");

    // A task that touches nothing: the `DRIFTED + UNTOUCHED` arm, which is where a
    // conformant external edit is absorbed rather than blocked.
    let task = corpus.start_workflow("plan", "an unrelated spec");
    let door = both_streams(&corpus.jigc(&["task", "validate", &task]));
    assert!(
        door.contains("reconciliation.absorb"),
        "the anchored heading must be absorbed, not blocked; got:\n{door}"
    );
    assert!(
        !door.contains(CODE) && !door.contains("reconciliation.conformance-block"),
        "no carrier may still raise the corruption after the anchor repair; got:\n{door}"
    );

    // The identity the parser was missing now exists: the read surface serves the doc,
    // and the anchored line is a real `criteria` item rather than stray prose.
    let shown = corpus.jigc(&["doc", "show", "spec:rate-limiter"]);
    assert!(
        shown.status.success(),
        "the repaired doc must read back; got:\n{}",
        both_streams(&shown)
    );
    let item = corpus.jigc(&[
        "doc",
        "show",
        "spec:rate-limiter#criteria/rationale-for-the-cap",
    ]);
    assert!(
        item.status.success() && String::from_utf8_lossy(&item.stdout).contains(CORRUPT_PROSE),
        "the anchored heading must address as an item; got:\n{}",
        both_streams(&item)
    );
}

/// Arm (c) — the control that says *why* the clause went, and keeps the decision from
/// being inherited: `jigc doc add-item` at that address still answers an unrelated
/// `write.wrong-shape`. If the verb ever learns to run here, this reddens and the
/// wording is re-taken rather than assumed.
#[test]
fn add_item_at_that_address_still_answers_wrong_shape() {
    let corpus = corrupted_corpus();
    let task = corpus.start_workflow("plan", "repair the spec");
    let out = corpus.jigc(&[
        "doc",
        "add-item",
        "spec:rate-limiter#criteria",
        "--title",
        "Rationale for the cap",
        "--task",
        &task,
    ]);
    let text = both_streams(&out);
    assert_eq!(
        out.status.code(),
        Some(1),
        "`{RETIRED_VERB}` must refuse over the corrupted doc; got:\n{text}"
    );
    assert!(
        text.contains("write.wrong-shape"),
        "the refusal must be the unrelated `write.wrong-shape`, which is what makes \
         naming the verb a lie; got:\n{text}"
    );
}
