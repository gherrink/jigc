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

use std::collections::BTreeSet;

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

/// The whole clause a batch-author solicit owes wherever a payload item can collide
/// with a committed one: that what you author **appends** beside the entries already
/// there, plus the three facts of the collision — the code an agent greps for, that
/// the WHOLE payload is refused, and the edit-in-place exit. Every fact is the
/// binary's own behaviour, driven end to end by
/// [`a_colliding_payload_item_rejects_the_whole_author_over_a_committed_singleton`]
/// below, so the clause is what happens rather than a phrasing preference.
const WHOLE_CLAUSE: [&str; 4] = [
    "append",
    "write.already-present",
    "whole payload",
    "in place",
];

/// The consequence the clause replaces (M49 Inc 10 T2). Until this commit the
/// pack-load fence *demanded* it of five steps — but a colliding item never doubles,
/// it rejects the whole payload — so a fence bought a law-1 lie. No shipped step of
/// either pack may say it again.
const RETIRED_FALSEHOOD: &str = "would double";

/// Every doctype a step solicits a **batch** `jigc doc author` of, in the two shapes
/// the shipped steps use: the rendered payload skeleton (`{{schema:<T>}}` — the
/// generation seam whose whole point is that the step never hand-writes the payload)
/// and the literal `jigc doc author <T>` command line. A `--help` mention inside a
/// sentence is not a command line, and a leading `-` is never a doctype.
fn batch_author_targets(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        if let Some(ty) = rest[..close].trim().strip_prefix("schema:") {
            out.insert(ty.trim().to_owned());
        }
        rest = &rest[close + 2..];
    }
    for line in body.lines() {
        if let Some(tail) = line.trim().strip_prefix("jigc doc author ")
            && let Some(word) = tail.split_whitespace().next()
            && !word.starts_with('-')
        {
            out.insert(word.to_owned());
        }
    }
    out
}

/// A pack's singleton doctypes whose schema declares ≥1 `repeatable:` section — the
/// only doctypes where a payload item *can* collide with a committed one, read
/// through the production schema load rather than a yaml grep.
fn repeating_singletons(pack: &EmbeddedPack) -> BTreeSet<String> {
    pack.list(PackResourceKind::Schemas)
        .into_iter()
        .filter(|id| {
            let Ok(bytes) = pack.read(PackResourceKind::Schemas, id) else {
                return false;
            };
            let Ok(schema) = cli::pack::load_pack_schema(pack, &bytes) else {
                return false;
            };
            schema.singleton
                && schema.sections.iter().any(|section| {
                    matches!(section.body, engine::schema::SectionBody::Repeatable { .. })
                })
        })
        .map(|id| id.as_str().to_owned())
        .collect()
}

/// The owe-set, **derived from the structural signal the step already renders** —
/// never a phrase. The literal-phrase selector this replaces (*"existing entries are
/// untouched"*) missed dev's `author-change`, which says *"existing **releases** are
/// untouched"*: same clause, different noun, three collision facts absent. Derived,
/// the set is every step of both packs that solicits a batch author of a repeating
/// singleton, so a sibling growing the clause with a third noun is covered by
/// construction.
fn whole_clause_owing_steps() -> Vec<(&'static str, String, String)> {
    let repeating: Vec<(&'static str, BTreeSet<String>)> = embedded_packs()
        .into_iter()
        .map(|(name, pack)| (name, repeating_singletons(&pack)))
        .collect();
    all_steps()
        .into_iter()
        .filter(|(pack, _, body)| {
            let Some((_, types)) = repeating.iter().find(|(name, _)| name == pack) else {
                return false;
            };
            batch_author_targets(body)
                .iter()
                .any(|ty| types.contains(ty))
        })
        .collect()
}

/// The clause, over its whole axis: every step of both packs' whole step trees that
/// solicits a batch author of a repeating singleton states all four facts.
#[test]
fn every_batch_author_solicit_states_the_whole_clause() {
    let owing = whole_clause_owing_steps();
    let members: Vec<String> = owing
        .iter()
        .map(|(pack, id, _)| format!("{pack}:{id}"))
        .collect();
    assert!(
        members.iter().any(|member| member == "dev:author-change"),
        "the derived owe-set must reach dev's `author-change` — the step the retired \
         literal-phrase selector missed; got: {members:?}",
    );

    let mut missing = Vec::new();
    for (pack, id, body) in &owing {
        let body = normalized(body);
        for fact in WHOLE_CLAUSE {
            if !body.contains(fact) {
                missing.push(format!("step `{pack}:{id}` never says \"{fact}\""));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "a step solicits a batch author over a singleton whose items can collide but \
         never states what happens when one does — a payload item whose title mints an \
         id the doc already holds rejects the WHOLE payload (`write.already-present`), \
         nothing staged, and the exit is to edit that item in place:\n  {}\n(owing \
         steps: {})",
        missing.join("\n  "),
        members.join(", "),
    );
}

/// The pack-load fence and this sweep demand **the same clause** — the fence is the
/// half that blocks a pack from shipping the gap, this sweep the half that reaches
/// the literal-command siblings outside the declarer family. Bijected here so the two
/// cannot drift into disagreeing about what the binary does.
#[test]
fn the_pack_load_fence_demands_exactly_the_clause() {
    let fenced: BTreeSet<&str> = cli::pack::COPY_IN_APPEND_TOKENS.into_iter().collect();
    let clause: BTreeSet<&str> = WHOLE_CLAUSE.into_iter().collect();
    assert_eq!(
        fenced, clause,
        "`COPY_IN_APPEND_TOKENS` must demand exactly the clause the binary produces",
    );
}

/// The retired consequence is gone from the shipped prose — swept over both packs'
/// whole step trees, not just the five the fence forced to say it.
#[test]
fn no_shipped_step_states_the_retired_doubling_falsehood() {
    let offenders: Vec<String> = all_steps()
        .iter()
        .filter(|(_, _, body)| normalized(body).contains(RETIRED_FALSEHOOD))
        .map(|(pack, id, _)| format!("{pack}:{id}"))
        .collect();
    assert!(
        offenders.is_empty(),
        "a colliding payload item is refused, not doubled — these steps still say \
         \"{RETIRED_FALSEHOOD}\": {offenders:?}",
    );
}

#[test]
fn the_composed_batch_author_note_states_the_collision_reject() {
    let corpus = TrialCorpus::build(State::Fresh);
    // Two composed doors, both driven through the real binary: the mint-free preview
    // the trial read the note from (its `author-roadmap` step carries the batch-author
    // note), and the `single-task` front door, whose `record-changelog` step is the
    // one the fence forced into the falsehood.
    let doors: [(&str, Vec<&str>); 2] = [
        (
            "workflow planning --preview",
            vec!["workflow", "planning", "--preview"],
        ),
        (
            "start --workflow single-task",
            vec![
                "start",
                "--workflow",
                "single-task",
                "record a user-facing change",
            ],
        ),
    ];
    for (label, args) in doors {
        let composed = normalized(&corpus.jigc_ok(&args));
        for fact in WHOLE_CLAUSE {
            assert!(
                composed.contains(fact),
                "the COMPOSED `{label}` bytes — what an agent reads — must state \
                 \"{fact}\"; got:\n{composed}"
            );
        }
        assert!(
            !composed.contains(RETIRED_FALSEHOOD),
            "the COMPOSED `{label}` bytes must not say \"{RETIRED_FALSEHOOD}\"; \
             got:\n{composed}"
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
