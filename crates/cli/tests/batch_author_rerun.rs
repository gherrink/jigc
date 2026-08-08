//! M47 Increment 11, T1 — **the one-shot clause dies on every surface that carried
//! it** (RC-alpha4 B2, the *refuted* repro block; `DECISIONS.md` → 2026-08-08 the
//! Increment 11 plan halt).
//!
//! Fourteen shipped surfaces — thirteen pack steps in **both** packs plus
//! `jigc doc author --help` — stated an ordering rule the binary has never enforced:
//! *"run it INSTEAD of the create + per-entry verbs, never after them — an
//! already-staged doc rejects a second create."* Since M45 fork 2's copy-on-write, a
//! `create` over a doc the task already staged is handed back **as found** and acks
//! `already existed — copied in for update` (`engine::state::create_gated`), and
//! `author` re-runs in the same task. The trial's worker read the clause as *"one
//! shot, no iteration"* and declined to iterate a record they were unhappy with — a
//! law-1 lie that cost real quality.
//!
//! What this suite pins, each over its class **axis** rather than the reported repro:
//!
//!   * **the behaviour** — `create` → `author` → `create` → `author` over **one**
//!     identity on **one** task, all exit 0 through the real binary, with the second
//!     `create` acking the copy-in and the second `author` landing its prose. This is
//!     the fact the clause contradicted, so it is what the deleted sentence owed.
//!   * **the composed surface** — **every** workflow of **both** packs, enumerated
//!     from the loaded registries (never a hand list), composed through
//!     `jigc workflow <id> --preview`: none may claim a repeat `create`/`author` is
//!     refused. A new workflow, or a new pack, joins this sweep with no edit here.
//!   * **the help surface** — the same fence over `jigc doc author --help` and its
//!     parent `jigc doc --help`, the two help texts the clause lived on.
//!
//! **Swept by the sentence, never by a count** — the fence is a set of claim
//! wordings, and a non-vacuity arm asserts the batch note still composes into ≥8
//! workflows, so a sweep that silently stopped seeing the note cannot pass green.

use crate::support;

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};

use support::trial_corpus::{State, TrialCorpus};

/// Wordings that claim a repeat `create` / `author` is refused. Each is a form the
/// class has actually taken: the first four are the shipped clause's own tokens (in
/// both its `per-leaf` and `per-entry` wrappings, help and pack alike), the last two
/// are how the trial's worker read it back (`RC-alpha4/feedback-P3.md` §2 —
/// *"author may be run only once per task"*), so a re-statement in the reader's own
/// words is fenced too.
const REJECTION_CLAIMS: [&str; 6] = [
    "never after",
    "rejects a second create",
    "rejects the second create",
    "already-staged doc rejects",
    "already staged doc rejects",
    "only once per task",
];

/// The batch note's surviving opening — the selector that proves this sweep sees the
/// paragraph it fences, rather than passing because it found nothing at all.
const BATCH_NOTE: &str = "the batch alternative to the whole sequence above";

/// The count of workflows the batch note composes into. Eight at HEAD (`planning`,
/// `record-decision`, `completion`, `do-research`, `park-idea`, `record-change`,
/// `plan`, `architecture-documentation`); asserted as a floor, so adding a
/// batch-authoring workflow never reddens this while removing the whole note does.
const BATCH_NOTE_WORKFLOWS_FLOOR: usize = 8;

/// Whitespace-collapsed, ASCII-case-folded view — pack prose is hard wrapped and
/// clap re-wraps help, so a phrase assertion must be wrap- and case-insensitive.
fn normalized(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// Assert `text` (already normalized) states no rejection-of-a-repeat claim.
fn assert_no_rejection_claim(surface: &str, text: &str) {
    for claim in REJECTION_CLAIMS {
        assert!(
            !text.contains(claim),
            "{surface} claims a repeat `create`/`author` is refused (\"{claim}\") — \
             the binary refuses neither: a `create` over a doc the task already \
             staged acks `already existed — copied in for update`, and `author` \
             re-runs. Surface text:\n{text}"
        );
    }
}

/// Every workflow id of both embedded packs, read from the loaded registries — the
/// CWD-free `EmbeddedPack` constructors, never `make_pack()`.
fn all_workflow_ids() -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for (pack_name, pack) in [
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ] {
        for id in pack.list(PackResourceKind::Workflows) {
            out.push((pack_name, id.as_str().to_string()));
        }
    }
    out
}

/// The fact the deleted clause contradicted, through the real binary: the four-call
/// sequence the help said would break runs clean over one identity in one task, and
/// the second `create` acks the copy-in rather than routing away.
#[test]
fn create_author_create_author_all_succeed_on_one_task() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "settle the cache strategy");

    // 1 — the fresh create.
    let first_create = corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        &task,
    ]);
    assert!(
        !first_create.contains("already existed"),
        "the FIRST create mints fresh, so it must not ack a copy-in; got:\n{first_create}"
    );
    let address = first_create
        .lines()
        .next()
        .expect("the create ack names the address")
        .split_whitespace()
        .next()
        .expect("the ack's first token is the address")
        .to_string();

    // 2 — `author` over the doc `create` just staged: the exact call the clause said
    // would be refused.
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD,
    );

    // 3 — a second `create` over that staged copy: exit 0, acking the copy-in.
    let second_create = corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        &task,
    ]);
    assert!(
        second_create.contains("already existed — copied in for update"),
        "the SECOND create over the staged copy must ack the copy-in rather than \
         reject; got:\n{second_create}"
    );
    assert!(
        !second_create.contains("serial-collision"),
        "the second create must not route away with `create.serial-collision`; \
         got:\n{second_create}"
    );

    // 4 — `author` again, revising the prose the third call left untouched.
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD_REVISED,
    );

    // The staged doc carries the LAST author's prose — the four calls composed, none
    // of them lost.
    let staged = corpus.jigc_ok(&["doc", "show", &address, "--task", &task]);
    assert!(
        staged.contains("Colder reads, revised."),
        "the second author's prose must be the staged truth; got:\n{staged}"
    );
}

/// The composed surface, swept over **every** workflow of both packs.
#[test]
fn no_composed_workflow_claims_a_repeat_create_is_refused() {
    let corpus = TrialCorpus::build(State::Fresh);
    let mut carrying_the_note = Vec::new();
    for (pack, id) in all_workflow_ids() {
        // A `creates-task: false` member refuses the preview and prints its refusal on
        // stderr, so both streams are fenced — the clause could live on either.
        let out = corpus.jigc(&["workflow", &id, "--preview"]);
        let composed = normalized(&format!(
            "{}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        ));
        assert_no_rejection_claim(&format!("the composed `{pack}:{id}` preview"), &composed);
        if composed.contains(BATCH_NOTE) {
            carrying_the_note.push(format!("{pack}:{id}"));
        }
    }
    assert!(
        carrying_the_note.len() >= BATCH_NOTE_WORKFLOWS_FLOOR,
        "only {} composed workflow(s) carry the batch-authoring note ({}) — fewer than \
         the {BATCH_NOTE_WORKFLOWS_FLOOR} that carry it at HEAD, so this sweep is no \
         longer looking at the paragraph it fences",
        carrying_the_note.len(),
        carrying_the_note.join(", "),
    );
}

/// The help surface — where the clause was quoted back from in the trial.
#[test]
fn no_doc_help_surface_claims_a_repeat_create_is_refused() {
    let corpus = TrialCorpus::build(State::Fresh);
    for args in [
        ["doc", "author", "--help"].as_slice(),
        ["doc", "--help"].as_slice(),
    ] {
        let help = normalized(&corpus.jigc_ok(args));
        assert_no_rejection_claim(&format!("`jigc {}`", args.join(" ")), &help);
    }
    // Non-vacuous: the batch verb's help still describes the verb it fences.
    let help = normalized(&corpus.jigc_ok(&["doc", "author", "--help"]));
    assert!(
        help.contains("batch"),
        "`jigc doc author --help` must still describe the batch verb; got:\n{help}"
    );
}

const ADR_PAYLOAD: &str = "\
title: Cache strategy
sections:
  - id: context
    set:
      context: |-
        <<Forces around caching.>>
  - id: decision
    set:
      decision: |-
        <<Use a write-through cache.>>
  - id: consequences
    set:
      consequences: |-
        <<Colder reads.>>
";

const ADR_PAYLOAD_REVISED: &str = "\
title: Cache strategy
sections:
  - id: context
    set:
      context: |-
        <<Forces around caching, revised.>>
  - id: decision
    set:
      decision: |-
        <<Use a write-through cache, revised.>>
  - id: consequences
    set:
      consequences: |-
        <<Colder reads, revised.>>
";
