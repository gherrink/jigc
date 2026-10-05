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

/// The committed `prd` the corrupted spec's `derived-from` names — the referrer edge the
/// arm-1 chain must still resolve at the end.
const PRD_ID: &str = "prd:gateway-limits";

/// The operator's edit, made **after** `jigc migrate` minted — so the source seam the
/// task recorded at mint cannot contain it, and no fidelity diff can show it.
const POST_MINT_EDIT: &str = "Traffic is spiky on release days.";

/// The clause the migration-source route carried when it shipped: it sent the operator
/// to a review of bytes that review never renders.
const FALSE_REVIEW_CLAIM: &str = "fidelity diff is where that replacement is reviewed";

/// The act the conflict route ordered until M46 and the adapter has never sanctioned —
/// `.jigc/AGENT.md`'s routing sentence forbids editing a managed doc directly, so a
/// blocking route whose only in-repo exit is a hand revert is a route out of nothing.
const FORBIDDEN_ACT: &str = "revert the external edit on disk";

/// A corpus carrying one committed, managed `spec` — then hand-broken and committed
/// again, which is the shape a corpus corrupted before M45's write guard shipped
/// carries at rest.
fn corrupted_corpus() -> TrialCorpus {
    corrupted_corpus_with_referrer(false)
}

/// The same corpus, optionally with the corrupted spec carrying a **committed
/// `derived-from` edge** at a committed `prd` — so the repair chain is driven over a doc
/// that is a live referrer, not an isolated leaf. The chain's one escape (`jigc unmanage`)
/// drops that doc's forward edges along with its baseline, so "the referrer still
/// resolves at the end" is the assertion that the re-landed doc rejoined the edge index
/// rather than quietly dangling.
fn corrupted_corpus_with_referrer(referrer: bool) -> TrialCorpus {
    let corpus = clean_corpus_with_referrer(referrer);

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

/// The same corpus **before** the hand break — one committed, conformant, managed `spec`,
/// optionally grounded at a committed `prd`. The corrupted states above are this state plus
/// one out-of-band edit, and the conflict control below needs it un-corrupted (a write to a
/// non-reparseable doc is refused at the write, so a corrupted doc cannot reach
/// `DRIFTED + TOUCHED` at all).
fn clean_corpus_with_referrer(referrer: bool) -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    if referrer {
        build_prd(&corpus);
    }
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
    if referrer {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("{id}#derived-from"),
            "--value",
            PRD_ID,
            "--task",
            &task,
        ]);
    }
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
    corpus
}

/// The committed `prd` [`PRD_ID`] names, authored through its own driving workflow and
/// finalized — so the spec's `derived-from` edge points at a real committed target and the
/// store sweep's `ref-resolves` family has something to resolve.
fn build_prd(corpus: &TrialCorpus) {
    let task = corpus.start_workflow("project-setup", "the gateway limits brief");
    let id = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "prd",
            "--title",
            "Gateway limits",
            "--task",
            &task,
        ])
        .trim_end_matches('\n')
        .to_string();
    assert_eq!(
        id, PRD_ID,
        "the fixture PRD must mint the id the spec names"
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("{id}#vision"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "Bound what the gateway lets through.",
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
        "The gateway is unbounded today.",
    );
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{id}#requirements"),
            "--title",
            "Bound bursts",
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
        "Bursts beyond the cap are bounded.",
    );
    corpus.finalize(&task, "prd", "the gateway limits brief", false);
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

/// **Arm 1 — the repair chain, driven from the repo where the corruption happened**
/// (M46 Increment 5 / T2).
///
/// The shipped chain (`jigc migrate <path> --as <type>` → `doc author` → `finalize
/// --approve`) is what the *other* arms of this suite drive, and they drive it after a
/// `jigc unmanage` — from a standing point with no recorded baseline, which is a fresh
/// clone's. In the repo where the hand edit actually happened the baseline is present, so
/// the on-disk drift plus the migration's own staged rewrite are `DRIFTED + TOUCHED`:
/// finalize blocks at `reconciliation.conflict-block`, at **both** doors, and its route
/// offered exactly two exits — retire the migration, or [`FORBIDDEN_ACT`], the act the
/// adapter's routing sentence forbids over a managed doc.
///
/// So the route now carries a third exit for the one path it is true of: the migration's
/// **own recorded source**. This test drives the whole chain and runs that exit **as
/// printed** — the argv is lifted out of the emitted route text and executed verbatim, so
/// what is proven is the bytes an agent would actually run, not a reconstruction of them.
///
/// The doc under repair is a live **referrer** (`derived-from → prd:gateway-limits`), and
/// the exit drops its forward edges along with its baseline — so the chain ending clean is
/// only half the claim; the edge resolving again at the end is the other half.
#[test]
fn the_conflict_route_names_an_exit_that_runs_where_the_corruption_happened() {
    let corpus = corrupted_corpus_with_referrer(true);

    // The corruption's slot-prose repair, at the depth the emitted diagnosis names — the
    // same lift the sibling arm makes, so the chain starts from the state the diagnosis
    // actually leaves an operator in.
    let depth = demote_depth(&diagnosis("jigc validate", &store_carrier(&corpus)));
    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the corrupted spec is readable");
    let demoted_heading = CORRUPT_HEADING.replacen("###", &depth, 1);
    fs::write(&path, body.replace(CORRUPT_HEADING, &demoted_heading))
        .expect("write the demoted heading");

    // No `unmanage` first: this is the standing point the operator has, baseline and all.
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
             \x20 - id: meta\n\
             \x20   set:\n\
             \x20     derived-from: \"{PRD_ID}\"\n\
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

    // The block, at the door the operator reaches first.
    let blocked = corpus.jigc(&["task", "finalize", &task]);
    let text = both_streams(&blocked);
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "the migration finalize must block on the conflict; got:\n{text}"
    );
    assert!(
        text.contains("blocking · reconciliation.conflict-block"),
        "the block must be the conflict, not the review hold; got:\n{text}"
    );

    // The exit, lifted out of the printed route and run verbatim.
    let argv = route_argv(&text);
    assert!(
        argv.first().map(String::as_str) == Some("jigc"),
        "the route's command span must be a `jigc` argv; got {argv:?}"
    );
    let run: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    let exit = corpus.jigc(&run);
    assert!(
        exit.status.success(),
        "the printed route must run from here; `jigc {}` gave:\n{}",
        run.join(" "),
        both_streams(&exit)
    );

    // The chain completes from exactly where it stopped: the review hold, then the one
    // destructive gate.
    let hold = corpus.jigc(&["task", "finalize", &task]);
    assert_eq!(
        hold.status.code(),
        Some(4),
        "the re-run finalize must reach the migration review hold; got:\n{}",
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
        "the chain must leave the store clean; got:\n{swept}"
    );
    // The referrer edge the exit dropped is back — the re-landed doc rejoined the index
    // rather than quietly dangling under a clean sweep.
    let shown = both_streams(&corpus.jigc(&["doc", "show", "spec:rate-limiter"]));
    assert!(
        shown.contains(&format!("derived-from: {PRD_ID}")),
        "the committed `derived-from` referrer must survive the chain; got:\n{shown}"
    );
    assert!(
        shown.contains(&demoted_heading),
        "the demoted heading must land as slot prose; got:\n{shown}"
    );
}

/// **The negative control**: a task that is *not* migrating this doc gets no such exit.
///
/// The exit drops a file-state baseline — the guard whose absence turns "detected and
/// routed, never silently merged" into an exit-0 merge ([storage.md] → What none of this
/// buys). It is offered because a migration's staged rewrite *is* the replacement of that
/// path, which is the one state where dropping the guard costs nothing the task did not
/// already intend. An ordinary task's conflict has no such standing, so the general route
/// stays what it was — one argv, mechanical, fully substituted.
///
/// [storage.md]: ../../../design/storage.md
#[test]
fn an_ordinary_tasks_conflict_is_offered_no_baseline_drop() {
    let corpus = clean_corpus_with_referrer(false);

    // `TOUCHED`: an ordinary task stages a write to the committed doc.
    let task = corpus.start_workflow("plan", "extend the rate limiter spec");
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            "spec:rate-limiter#goal",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "Cap bursts, and shed load beyond the cap.",
    );
    // `DRIFTED`: a conformant external edit to the same doc. Both sides moved, and neither
    // is a migration.
    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the committed spec is readable");
    let drifted = body.replace(
        "The gateway has no limiter today.",
        "The gateway has no limiter today, and traffic is spiky.",
    );
    assert_ne!(body, drifted, "the external edit must land");
    fs::write(&path, drifted).expect("write the external edit");

    let blocked = corpus.jigc(&["task", "validate", &task]);
    let text = both_streams(&blocked);
    assert!(
        text.contains("blocking · reconciliation.conflict-block"),
        "an ordinary task writing a drifted doc must conflict-block; got:\n{text}"
    );
    let argv = route_argv(&text);
    assert_eq!(
        argv,
        vec!["jigc", "task", "discard", task.as_str(), "--force"],
        "the general route stays the single-argv whole-task discard, with the real id — and \
         since M50 Inc 3 / T2 with the consent that door now requires, because the state \
         printing this route is one whose task stages a write"
    );
    let route = route_text(&text);
    assert!(
        !route.contains("unmanage"),
        "no baseline drop is offered to a task that is not replacing the path; got:\n{route}"
    );
    assert!(
        !route.contains('<'),
        "no unsubstituted placeholder survives on a blocking route; got:\n{route}"
    );
    // The hand revert it still offers is the act the adapter forbids by default — so it
    // carries the sanction that makes it the exception, rather than ordering it bare.
    assert!(
        route.contains(FORBIDDEN_ACT),
        "the general route still offers the revert; got:\n{route}"
    );
    assert!(
        route.contains("out-of-band"),
        "and it must say why that revert is sanctioned here; got:\n{route}"
    );
}

/// **Arm 2 — a migration source edited after the mint is not replaced** (M46 Increment 5,
/// validate→fix; re-taken by the rc.24 fix pass's completion audit).
///
/// Arm 1 plants every external edit **before** `jigc migrate` mints, which is the one
/// arrangement where the mint-time snapshot happens to contain the drift. The state an
/// operator actually reaches is the other one: the migration is minted, the rewrite is
/// authored against the source **as this task recorded it at mint**, and *then* a hand
/// edit lands on that file.
///
/// The fidelity diff cannot review that edit: it renders the recorded source seam
/// (`crates/cli/src/migrate.rs` persists it at mint; `task.rs` renders *that*). At M46 the
/// route was made to stop promising a review it did not get, and it still offered `jigc
/// unmanage` — the exit whose finalize replaced the edit at exit 0, having said so. It no
/// longer does: the door compares the file against what the task recorded, and a source
/// that differs blocks with a presentation of its own, which offers no baseline drop.
///
/// Driven through the real binary, and both printed commands are lifted out of the route
/// and run verbatim: the discard keeps the operator's edit on disk, and the migrate span
/// mints a migration whose recorded source carries it.
#[test]
fn a_migration_source_edited_after_the_mint_is_offered_no_baseline_drop() {
    let corpus = corrupted_corpus();

    // The corruption's slot-prose repair, at the depth the emitted diagnosis names — the
    // same standing point arm 1 starts from, baseline and all.
    let depth = demote_depth(&diagnosis("jigc validate", &store_carrier(&corpus)));
    let path = corpus.repo().join(SPEC_PATH);
    let body = fs::read_to_string(&path).expect("the corrupted spec is readable");
    let demoted_heading = CORRUPT_HEADING.replacen("###", &depth, 1);
    fs::write(&path, body.replace(CORRUPT_HEADING, &demoted_heading))
        .expect("write the demoted heading");

    let minted = corpus.jigc_ok(&["migrate", SPEC_PATH, "--as", "spec"]);
    let task = minted_task(&minted);

    // **After the mint** — the edit the recorded source cannot contain, and the one the
    // operator means to keep.
    let body = fs::read_to_string(&path).expect("the source is readable");
    let edited = body.replace(
        "The gateway has no limiter today.",
        &format!("The gateway has no limiter today. {POST_MINT_EDIT}"),
    );
    assert_ne!(body, edited, "the post-mint edit must land");
    fs::write(&path, edited).expect("write the post-mint edit");

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

    // Both spellings of the committing door refuse, and the edit is still on disk.
    for args in [
        vec!["task", "finalize", task.as_str()],
        vec!["task", "finalize", task.as_str(), "--approve"],
    ] {
        let blocked = corpus.jigc(&args);
        let text = both_streams(&blocked);
        assert_eq!(
            blocked.status.code(),
            Some(3),
            "`jigc {}` must block on the edited source; got:\n{text}",
            args.join(" ")
        );
        assert!(
            text.contains("blocking · reconciliation.conflict-block")
                && text.contains("no longer holds what the migration recorded"),
            "the block is the edited-source conflict; got:\n{text}"
        );
    }
    let text = both_streams(&corpus.jigc(&["task", "finalize", &task, "--approve"]));
    let route = route_text(&text);
    assert!(
        !route.contains("unmanage"),
        "no baseline drop is offered over an edit the rewrite does not carry; got:\n{route}"
    );
    assert!(
        !route.contains(FALSE_REVIEW_CLAIM),
        "and the route still does not send the operator to a review of the on-disk bytes; \
         got:\n{route}"
    );
    assert!(
        fs::read_to_string(&path)
            .expect("the source is readable")
            .contains(POST_MINT_EDIT),
        "a refused finalize leaves the operator's edit where it was"
    );

    // Exit one, as printed: retire the migration, keep the file.
    let spans = route_jigc_spans(&text);
    assert_eq!(
        spans.first().map(Vec::as_slice),
        Some(
            &[
                "jigc".to_string(),
                "task".to_string(),
                "discard".to_string(),
                task.clone(),
                "--force".to_string()
            ][..]
        ),
        "the route leads with the exit that keeps the on-disk bytes, id substituted"
    );
    let run: Vec<&str> = spans[0][1..].iter().map(String::as_str).collect();
    let kept = corpus.jigc(&run);
    assert!(
        kept.status.success(),
        "the preserving exit must run as printed; `jigc {}` gave:\n{}",
        run.join(" "),
        both_streams(&kept)
    );
    assert!(
        fs::read_to_string(&path)
            .expect("the source is readable")
            .contains(POST_MINT_EDIT),
        "the exit the route names as the one that keeps the file must keep it"
    );

    // …then the command it names for migrating the file as it now reads, as printed: the
    // new migration's recorded source carries the edit, so its review will.
    let remigrate = spans
        .iter()
        .find(|argv| argv.get(1).map(String::as_str) == Some("migrate"))
        .unwrap_or_else(|| panic!("the route must name the re-migrate command; got:\n{route}"));
    let run: Vec<&str> = remigrate[1..].iter().map(String::as_str).collect();
    let again = corpus.jigc(&run);
    let composed = both_streams(&again);
    assert!(
        again.status.success(),
        "the re-migrate must run as printed; `jigc {}` gave:\n{composed}",
        run.join(" ")
    );
    assert!(
        composed.contains(POST_MINT_EDIT),
        "the second migration is minted against the file as it now reads; got:\n{composed}"
    );
}

/// The task id `jigc migrate` printed on its `task minted:` line.
fn minted_task(output: &str) -> String {
    output
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("`jigc migrate` mints a task")
        .trim()
        .to_string()
}

/// A foreign, non-conformant file a `vision` migration is minted over.
const FOREIGN_DIRECTION: &str = "# Product Direction\n\nWe build a deterministic context \
                                 compiler.\n\n## Principles\n\nStructure belongs to the CLI; \
                                 prose belongs to the model.\n";

/// The `vision` rewrite of [`FOREIGN_DIRECTION`].
const VISION_PAYLOAD: &str = "title: Vision\n\
     sections:\n\
     \x20 - id: thesis\n\
     \x20   set:\n\
     \x20     thesis: |-\n\
     \x20       <<We build a deterministic context compiler.>>\n\
     \x20 - id: invariants\n\
     \x20   set:\n\
     \x20     invariants: |-\n\
     \x20       <<Structure belongs to the CLI; prose belongs to the model.>>\n\
     \x20 - id: open-questions\n\
     \x20   set:\n\
     \x20     open-questions: |-\n\
     \x20       <<Which domains earn a pack of their own.>>\n";

/// The paragraph a hand adds to a migration's source after `jigc migrate` minted.
const HAND_AFTER_MINT: &str = "A paragraph the human added during the migration.";

/// **A migration's source edited after the mint is never replaced or removed — wherever
/// the source lives** (the rc.24 fix pass's completion audit).
///
/// Arm 2 above drives the one shape the M46 route was written for: a *baselined* source,
/// which conflict-blocks on its recorded hash. The audit drove the dominant one — a
/// foreign file jigc never adopted, which has no recorded hash at any finalize — and the
/// edit was replaced at `jigc task finalize --approve`, exit 0, in no git object, on a
/// review that never showed it. That is one cell of a class the brief did not enumerate:
/// a migration task replaces a **same-path** source and **deletes** a source anywhere
/// else, and neither act compared the file against what the task recorded.
///
/// So this iterates where the source lives — at the doctype's own home (replaced), outside
/// every managed location (retired, and never swept), inside another doctype's managed
/// location (retired, swept as a doc the task did not stage) — and at each: the preview
/// door and both spellings of the committing door refuse, `HEAD` does not move, the edit
/// is still on disk, no baseline drop is offered; and the route's second exit, taken —
/// the edit undone — lets the same task reach its review hold and land.
#[test]
fn a_migration_source_edited_after_the_mint_is_never_replaced_or_removed() {
    for source in [
        "VISION.md",
        "notes/direction.md",
        "docs/decisions/direction.md",
    ] {
        let corpus = TrialCorpus::build(State::Fresh);
        let path = corpus.repo().join(source);
        fs::create_dir_all(path.parent().expect("the source has a parent")).expect("mk the dir");
        fs::write(&path, FOREIGN_DIRECTION).expect("write the foreign source");
        corpus.git(&["add", source]);
        corpus.git(&["commit", "-q", "-m", "add the direction doc"]);

        let task = minted_task(&corpus.jigc_ok(&["migrate", source, "--as", "vision"]));
        corpus.jigc_stdin_ok(
            &[
                "doc",
                "author",
                "vision",
                "--from-file",
                "-",
                "--task",
                &task,
            ],
            VISION_PAYLOAD,
        );
        let head = corpus.git(&["rev-parse", "HEAD"]);

        fs::write(
            &path,
            format!("{FOREIGN_DIRECTION}\n## Pricing\n\n{HAND_AFTER_MINT}\n"),
        )
        .expect("write the post-mint edit");

        for args in [
            vec!["task", "validate", task.as_str()],
            vec!["task", "finalize", task.as_str()],
            vec!["task", "finalize", task.as_str(), "--approve"],
        ] {
            let refused = corpus.jigc(&args);
            let text = both_streams(&refused);
            assert_eq!(
                refused.status.code(),
                Some(3),
                "{source}: `jigc {}` must refuse over the edited source; got:\n{text}",
                args.join(" ")
            );
            assert!(
                text.contains(&format!(
                    "blocking · reconciliation.conflict-block — conflict on `{source}`"
                )),
                "{source}: the refusal names the source; got:\n{text}"
            );
            let route = route_text(&text);
            assert!(
                !route.contains("unmanage"),
                "{source}: no baseline drop is offered; got:\n{route}"
            );
            assert_eq!(
                route_jigc_spans(&text).first().map(|argv| argv.join(" ")),
                Some(format!("jigc task discard {task} --force")),
                "{source}: the route leads with the exit that keeps the file"
            );
        }
        assert_eq!(
            corpus.git(&["rev-parse", "HEAD"]),
            head,
            "{source}: a refused finalize commits nothing"
        );
        assert!(
            fs::read_to_string(&path)
                .expect("the source is still there")
                .contains(HAND_AFTER_MINT),
            "{source}: and the edit is still on disk"
        );

        // The route's other exit: undo the edit, and the same task lands as authored.
        fs::write(&path, FOREIGN_DIRECTION).expect("undo the post-mint edit");
        let hold = corpus.jigc(&["task", "finalize", &task]);
        assert_eq!(
            hold.status.code(),
            Some(4),
            "{source}: with the edit undone the finalize reaches its review hold; got:\n{}",
            both_streams(&hold)
        );
        let landed = corpus.jigc(&["task", "finalize", &task, "--approve"]);
        assert!(
            landed.status.success(),
            "{source}: and the approved migration lands; got:\n{}",
            both_streams(&landed)
        );
        assert!(
            corpus
                .git(&["show", "HEAD:VISION.md"])
                .contains("schema-version"),
            "{source}: the managed vision is what HEAD holds"
        );
    }
}

/// **The comparison never refuses a source nobody edited** — the *must not refuse* cells
/// of the guard above, over git's conversion settings and the fresh-clone shape.
///
/// The guard compares the file against the bytes the task recorded at its mint, and the
/// rc.24 fix pass's own audit is the record of what a hand-rolled byte comparison does in
/// a converting checkout: it refuses a file no hand touched, with an edit-shaped route and
/// no edit to undo. Byte-equal is the answer in every cell where nothing rewrote the file,
/// under any setting — the first six cells. The seventh is the one a byte comparison alone
/// gets wrong: git itself rewrites the source between the mint and the finalize (a
/// re-checkout under `core.autocrlf=true` turns the recorded `\n` file into a `\r\n` one),
/// the bytes differ, and the door asks git whether they are the same content at that path
/// — which they are.
///
/// One corpus, one migration per cell: each cell commits its own foreign file under its
/// own setting, migrates it `--as adr`, and lands — the source retired, the doc at `HEAD`.
/// Not covered: a clean/smudge filter, a submodule, a worktree of a bare repository and a
/// `--separate-git-dir` checkout (a linked worktree the user made cannot commit a migration
/// at all — the mint refuses).
#[test]
fn an_unedited_migration_source_lands_under_every_conversion() {
    struct Cell {
        name: &'static str,
        /// `core.autocrlf` for the cell.
        autocrlf: &'static str,
        /// A `.gitattributes` line committed with the source, if any.
        attributes: Option<&'static str>,
        /// The line ending the source is written and committed with.
        eol: &'static str,
        /// Remove the file and let git check it out again after the mint.
        recheckout: bool,
        /// Empty the gitignored state cache first — what a fresh clone holds.
        fresh_clone: bool,
    }
    let cell = |name, autocrlf, attributes, eol| Cell {
        name,
        autocrlf,
        attributes,
        eol,
        recheckout: false,
        fresh_clone: false,
    };
    let cells = [
        cell("no-conversion", "false", None, "\n"),
        cell("autocrlf-true", "true", None, "\n"),
        cell("autocrlf-input", "input", None, "\n"),
        cell("text-auto", "false", Some("* text=auto"), "\n"),
        cell("crlf-blob", "false", None, "\r\n"),
        Cell {
            fresh_clone: true,
            ..cell("fresh-clone", "false", None, "\n")
        },
        Cell {
            recheckout: true,
            ..cell("autocrlf-true-recheckout", "true", None, "\n")
        },
    ];

    let corpus = TrialCorpus::build(State::Fresh);
    for cell in cells {
        let name = cell.name;
        if cell.fresh_clone {
            corpus.fresh_clone_shape();
        }
        corpus.git(&["config", "core.autocrlf", cell.autocrlf]);
        let source = format!("notes/{name}.md");
        let path = corpus.repo().join(&source);
        fs::create_dir_all(path.parent().expect("a parent")).expect("mk notes/");
        let foreign =
            format!("# Decision {name}\n\nWe considered a queue.\n\nWe will use a table.\n")
                .replace('\n', cell.eol);
        fs::write(&path, &foreign).expect("write the foreign source");
        let attributes = corpus.repo().join(".gitattributes");
        match cell.attributes {
            Some(line) => fs::write(&attributes, format!("{line}\n")).expect("write attributes"),
            None => fs::write(&attributes, "").expect("clear attributes"),
        }
        corpus.git(&["add", "--", &source, ".gitattributes"]);
        corpus.git(&[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            &format!("add {name}"),
        ]);

        let task = minted_task(&corpus.jigc_ok(&["migrate", &source, "--as", "adr"]));
        corpus.jigc_stdin_ok(
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            &format!(
                "title: \"Decision {name}\"\n\
                 sections:\n\
                 \x20 - id: status\n\
                 \x20   set:\n\
                 \x20     status: accepted\n\
                 \x20 - id: context\n\
                 \x20   set:\n\
                 \x20     context: \"<<We considered a queue.>>\"\n\
                 \x20 - id: decision\n\
                 \x20   set:\n\
                 \x20     decision: \"<<We will use a table.>>\"\n\
                 \x20 - id: consequences\n\
                 \x20   set:\n\
                 \x20     consequences: \"<<Operations owns the table.>>\"\n"
            ),
        );

        if cell.recheckout {
            // git rewrites the file, no hand does: under `core.autocrlf=true` the checkout
            // writes `\r\n` where the recorded source holds `\n`.
            fs::remove_file(&path).expect("remove the source");
            corpus.git(&["checkout", "--", &source]);
            assert_ne!(
                fs::read(&path).expect("the source is back"),
                foreign.as_bytes(),
                "{name}: the premise — the re-checkout changed the file's bytes"
            );
        }

        let hold = corpus.jigc(&["task", "finalize", &task]);
        assert_eq!(
            hold.status.code(),
            Some(4),
            "{name}: an unedited source reaches the review hold; got:\n{}",
            both_streams(&hold)
        );
        let landed = corpus.jigc(&["task", "finalize", &task, "--approve"]);
        assert!(
            landed.status.success(),
            "{name}: and lands; got:\n{}",
            both_streams(&landed)
        );
        assert!(
            !path.exists(),
            "{name}: the foreign original is retired with the commit"
        );
        let slug = format!("decision-{name}");
        assert!(
            corpus
                .git(&["show", &format!("HEAD:docs/decisions/{slug}.md")])
                .contains("We will use a table."),
            "{name}: and the managed adr is what HEAD holds"
        );
    }
}

/// Every backticked `` `jigc …` `` command span the conflict-block route prints, in the
/// order printed and split as printed — so an exit named in the route's *tail* is run
/// from the emitted bytes exactly like the leading argv is.
fn route_jigc_spans(output: &str) -> Vec<Vec<String>> {
    route_text(output)
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .map(|span| span.split_whitespace().map(str::to_string).collect())
        .collect()
}

/// The `reconciliation.conflict-block` route line, as printed.
fn route_text(output: &str) -> String {
    let start = output
        .find("reconciliation.conflict-block")
        .unwrap_or_else(|| panic!("the output must carry a conflict-block; got:\n{output}"));
    let rest = &output[start..];
    let route = rest
        .find("route: ")
        .unwrap_or_else(|| panic!("the conflict-block must carry a route; got:\n{output}"));
    let rest = &rest[route + "route: ".len()..];
    let end = rest.find('\n').unwrap_or(rest.len());
    rest[..end].trim_end().to_string()
}

/// The argv of the conflict-block route's **command span** — the backticked run of tokens
/// the route text leads with, split as printed. Nothing is reconstructed: this is what the
/// binary told the operator to type.
fn route_argv(output: &str) -> Vec<String> {
    let route = route_text(output);
    let span = route
        .strip_prefix('`')
        .and_then(|rest| rest.split_once('`'))
        .map(|(argv, _)| argv.to_string())
        .unwrap_or_else(|| {
            panic!("the conflict-block route must lead with a backticked argv; got:\n{route}")
        });
    span.split_whitespace().map(str::to_string).collect()
}
