//! M47 Increment 10, T7 — **the authoring surfaces state the real write contract**
//! (B1 / P3-2 · P4-2 · N14 · N15).
//!
//! Four authoring surfaces overstated or under-stated what a batch `jigc doc author`
//! does over an already-committed singleton. The shipped prose stated the **append**
//! half only (*"appends the new milestone — existing entries are untouched"*, byte-
//! verified accurate and kept) and left the third behaviour unsaid: a payload item
//! whose title mints an id the doc **already holds** rejects the **whole payload**
//! (`write.already-present`), staging nothing. An agent re-authoring a milestone it
//! had already recorded met that reject with no surface having named it.
//!
//! What this suite pins, each over its own **axis** rather than the reported instance:
//!
//!   * **the clause, not a count** — *every* pack step whose body states the append
//!     half must state the collision reject beside it, swept over both embedded packs'
//!     whole step trees, so a fifth step growing the clause is covered by construction;
//!   * **the composed** step text — the `jigc workflow planning --preview` shape the
//!     trial read it from, driven through the real binary (the emitted bytes an agent
//!     reads, never a reconstruction of the step file);
//!   * **the fact itself** — the three-way behaviour the prose now claims, exercised
//!     end-to-end over a committed singleton: append lands, a colliding title rejects
//!     whole, nothing is staged;
//!   * **N14** — every composed `<<author: …>>` directive names a doctype the read
//!     surface resolves, swept over every workflow of both packs (at HEAD
//!     `project-setup` emitted `<<author: brief#vision>>`, a task **role**, and
//!     `jigc doc schema brief` exits 1 with `store.unknown-type`);
//!   * **P4-2** — a catalog `when:` line that uses the term *documented code* glosses
//!     it, swept over every workflow of both packs.
//!
//! `jigc doc author --help`'s own statement of the same contract, and the N15
//! `Create::task` leak, are pinned in `help_truth.rs` beside the other help-text
//! truths.

use crate::support;

use cli::pack::EmbeddedPack;
use engine::packsource::{PackResourceKind, PackSource};

use support::trial_corpus::{State, TrialCorpus};

/// The two embedded packs, built the CWD-free way (never `make_pack()`, which
/// resolves against the process CWD).
fn embedded_packs() -> Vec<(&'static str, EmbeddedPack)> {
    vec![
        ("dev", EmbeddedPack::new()),
        ("methodology", EmbeddedPack::methodology()),
    ]
}

/// Whitespace-collapsed, ASCII-case-folded view of a body — pack prose is hard
/// wrapped, so a phrase assertion must be wrap-insensitive (the `pack.rs`
/// named-fact fence's own comparison view).
fn normalized(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// Every step of both embedded packs, as `(pack, id, body)`.
fn all_steps() -> Vec<(&'static str, String, String)> {
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Steps) {
            let bytes = pack
                .read(PackResourceKind::Steps, &id)
                .expect("a listed step reads back");
            out.push((
                pack_name,
                id.as_str().to_string(),
                String::from_utf8(bytes).expect("pack resources are UTF-8"),
            ));
        }
    }
    out
}

/// Every workflow of both embedded packs, as `(pack, id, body)`.
fn all_workflows() -> Vec<(&'static str, String, String)> {
    let mut out = Vec::new();
    for (pack_name, pack) in embedded_packs() {
        for id in pack.list(PackResourceKind::Workflows) {
            let bytes = pack
                .read(PackResourceKind::Workflows, &id)
                .expect("a listed workflow reads back");
            out.push((
                pack_name,
                id.as_str().to_string(),
                String::from_utf8(bytes).expect("pack resources are UTF-8"),
            ));
        }
    }
    out
}

/// The append half's marker phrase — the sentence that already shipped, and the
/// selector for the axis this suite sweeps. A step that tells an agent its authored
/// entries land beside the committed ones has taken on the whole contract.
const APPEND_HALF: &str = "existing entries are untouched";

/// The facts the collision half owes wherever the append half is stated: the code an
/// agent greps for, that the WHOLE payload is refused, and the edit-in-place exit.
const COLLISION_FACTS: [&str; 3] = ["write.already-present", "whole payload", "in place"];

#[test]
fn every_step_stating_the_append_half_states_the_collision_reject() {
    let steps = all_steps();
    let mut stating = Vec::new();
    let mut missing = Vec::new();
    for (pack, id, body) in &steps {
        let body = normalized(body);
        if !body.contains(APPEND_HALF) {
            continue;
        }
        stating.push(format!("{pack}:{id}"));
        for fact in COLLISION_FACTS {
            if !body.contains(fact) {
                missing.push(format!("step `{pack}:{id}` never says \"{fact}\""));
            }
        }
    }
    assert!(
        !stating.is_empty(),
        "no shipped step states the append half (\"{APPEND_HALF}\") — the selector \
         drifted, so this sweep is vacuous",
    );
    assert!(
        missing.is_empty(),
        "a step states that authored entries APPEND to the committed ones but never \
         states what happens when one collides — the third behaviour (a payload item \
         whose title mints an id the doc already holds rejects the WHOLE payload, \
         nothing staged) is the one an agent meets by surprise:\n  {}\n(steps stating \
         the append half: {})",
        missing.join("\n  "),
        stating.join(", "),
    );
}

#[test]
fn the_composed_batch_author_note_states_the_collision_reject() {
    let corpus = TrialCorpus::build(State::Fresh);
    // The shape the trial read the note from: a mint-free preview of the planning
    // workflow, whose `author-roadmap` step carries the batch-author note.
    let composed = normalized(&corpus.jigc_ok(&["workflow", "planning", "--preview"]));
    assert!(
        composed.contains(APPEND_HALF),
        "the composed planning preview must keep the append half; got:\n{composed}"
    );
    for fact in COLLISION_FACTS {
        assert!(
            composed.contains(fact),
            "the COMPOSED planning preview — the bytes an agent reads — must state \
             \"{fact}\"; got:\n{composed}"
        );
    }
}

/// The behaviour the prose claims, exercised over a **committed** singleton through
/// the real binary: a new entry appends beside the committed ones, and an entry whose
/// title mints an id the doc already holds rejects the whole payload with nothing
/// staged. A verified fact becomes a test (pinning.md §3), so the sentence and the
/// binary cannot drift apart.
#[test]
fn a_colliding_payload_item_rejects_the_whole_author_over_a_committed_singleton() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    // Read the committed milestone's title off the read surface — never reconstruct
    // it, the fixture owns it.
    let json = corpus.jigc_ok(&[
        "doc",
        "show",
        "roadmap:roadmap#milestones",
        "--format",
        "json",
    ]);
    let items: serde_json::Value = serde_json::from_str(&json).expect("valid json");
    let committed_title = items
        .as_array()
        .and_then(|a| a.first())
        .and_then(|item| item.get("title"))
        .and_then(|t| t.as_str())
        .expect("the committed roadmap carries at least one milestone")
        .to_string();

    let task = corpus.start_workflow("planning", "re-record the committed milestone");
    let staged = corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/roadmap:roadmap.md"));

    let colliding = format!(
        "title: Roadmap\nsections:\n  - id: milestones\n    items:\n      - title: {committed_title}\n        set:\n          proves: |\n            <<Re-proving it.>>\n          decomposition: |\n            <<One increment.>>\n"
    );
    let out = corpus.jigc_stdin(
        &[
            "doc",
            "author",
            "roadmap",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        &colliding,
    );
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        !out.status.success(),
        "authoring an entry the committed roadmap already holds must be refused; \
         stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("write.already-present"),
        "the refusal must carry `write.already-present`; got:\n{stderr}"
    );
    assert!(
        !staged.exists(),
        "the WHOLE payload is rejected — nothing staged — but `{}` exists",
        staged.display(),
    );

    // The append half, over the same committed doc: a fresh title lands beside the
    // committed entry rather than replacing it.
    let appending = "title: Roadmap\nsections:\n  - id: milestones\n    items:\n      - title: A Wholly New Milestone\n        set:\n          proves: |\n            <<It proves the append.>>\n          decomposition: |\n            <<One increment.>>\n";
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "author",
            "roadmap",
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        appending,
    );
    let body = std::fs::read_to_string(&staged).expect("the appended roadmap is staged");
    assert!(
        body.contains(&committed_title) && body.contains("A Wholly New Milestone"),
        "the committed entry must survive the append; got:\n{body}"
    );
}

/// N14 — every composed `<<author: …>>` directive names a doctype the read surface
/// resolves. Swept over **every** workflow of both packs, so the sibling directives
/// that happen to read correctly today (`commit`, `spec`, `arch-doc` — roles whose
/// names coincide with their doctype) are pinned by the same rule that catches
/// `brief`.
#[test]
fn every_composed_author_directive_names_a_resolvable_doctype() {
    let corpus = TrialCorpus::build(State::Fresh);
    let mut seen: std::collections::BTreeSet<String> = Default::default();
    for (pack, id, _body) in all_workflows() {
        let out = corpus.jigc(&["workflow", &id, "--preview"]);
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        for line in stdout.lines() {
            let trimmed = line.trim();
            let Some(inner) = trimmed
                .strip_prefix("<<author:")
                .and_then(|s| s.strip_suffix(">>"))
            else {
                continue;
            };
            let target = inner.trim();
            // The doctype is the address's leading hop: `<type>:<slug>#<leaf>`, or a
            // bare `<type>#<leaf>` when the slug is not yet knowable.
            let doctype = target
                .split('#')
                .next()
                .unwrap_or_default()
                .split(':')
                .next()
                .unwrap_or_default()
                .to_string();
            seen.insert(format!("{pack}:{id} → {target}"));
            let schema = corpus.jigc(&["doc", "schema", &doctype]);
            assert!(
                schema.status.success(),
                "workflow `{pack}:{id}` composes `<<author: {target}>>`, whose doctype \
                 `{doctype}` the read surface does not resolve — the directive names a \
                 task ROLE where every sibling names a doc address; \
                 `jigc doc schema {doctype}` said:\n{}",
                String::from_utf8_lossy(&schema.stderr),
            );
        }
    }
    assert!(
        !seen.is_empty(),
        "no workflow composed an `<<author: …>>` directive — the extraction drifted, \
         so this sweep is vacuous",
    );
}

/// P4-2 — *documented code* is a term of art the catalog `when:` lines used with no
/// definition anywhere the reader of that line could see (`usage:` is a different
/// surface). Swept over every workflow of both packs: a `when:` that uses the term
/// glosses it in the same line.
#[test]
fn every_when_line_using_documented_code_glosses_it() {
    let mut users = Vec::new();
    let mut missing = Vec::new();
    for (pack, id, body) in all_workflows() {
        let Some(when) = body
            .lines()
            .find_map(|line| line.trim_start().strip_prefix("when:"))
        else {
            continue;
        };
        let when = normalized(when);
        if !when.contains("documented code") {
            continue;
        }
        users.push(format!("{pack}:{id}"));
        if !when.contains("a managed doc names") {
            missing.push(format!("`{pack}:{id}`'s when: line: {when}"));
        }
    }
    assert!(
        !users.is_empty(),
        "no catalog `when:` line uses the term \"documented code\" — the selector \
         drifted, so this sweep is vacuous",
    );
    assert!(
        missing.is_empty(),
        "a catalog `when:` line disqualifies a workflow on \"documented code\" without \
         saying what that means — the router catalog prints the `when:` line alone, so \
         the reader has no `usage:` to fall back on:\n  {}",
        missing.join("\n  "),
    );
}
