//! M51 Increment 5 / T4 — **`ConfigAck::Set` names the relocation it performed**
//! (`completions/artifacts/M51/envelope-key-census.md` § 4.4 → EC-4; `roadmap.md` →
//! Milestone 51, Increment 5 → Riders).
//!
//! Both root knobs re-point where every managed doc lives and then **move** the committed
//! docs the re-point strands ([`cli::config`]'s two `route_*_repoint_*` floors). The moves
//! narrate on **stderr** as they happen and land as staged `git mv`s — and until this task
//! the `--format json` ack said nothing about them at all. Reproduced at `2f9f7993` on an
//! `adr`-bearing corpus: `jigc --format json config set docs-root documents` relocated one
//! committed doc, `git status` showed the `R`, and stdout was **byte-identical** to the
//! same invocation over a corpus with nothing to move —
//! `{committed, key, op, value}` either way. A driver that reads only the envelope (the
//! contract surface, the one M51 pins) cannot tell a re-point that moved the corpus from
//! one that moved nothing.
//!
//! **The axis this suite iterates** is the census's own cell set for this arm:
//! `{relocating, non-relocating} × {docs-root, placement-root}` — the *knob* half because
//! every root-knob rule that binds one binds the other (`root_knob_rules.rs`, M50's one
//! home), and the *relocating* half because an envelope key whose empty value is the only
//! one ever driven is a key nothing has proven carries anything. Each cell drives the real
//! binary over its own throwaway corpus.
//!
//! **The fixture is an `adr` on the `docs-root` cells, deliberately.** The first drive of
//! this rider used [`State::CommittedSingletons`] and moved nothing: every doc that state
//! commits is a **placement** doctype (`VISION.md`, `CHANGELOG.md`, `docs/roadmap.md`,
//! `docs/decisions-log.md`), and a placement home bypasses `docs-root` entirely
//! (`design/storage.md` → Placement). So the `docs-root` cells mint a `location` doctype —
//! one committed ADR under `docs/decisions/` — and the `placement-root` cells reuse
//! `committed-singletons`, whose two `docs/`-homed singletons are exactly that knob's
//! subject.
//!
//! **What `relocated` claims, and what it deliberately does not.** It carries the moves
//! that **landed**, as `{from, to}` objects — the pairs git actually staged, asserted here
//! against `git diff --cached --name-status -M` rather than against the binary's own
//! narration. A per-doc failure is **not** a relocation and is not counted: the floors
//! route the operator on stderr (`could not relocate … — move it by hand`), and an ack
//! claiming a move that did not happen is exactly the law-1 lie EC-4 closes.

use crate::support;

use serde_json::Value;
use support::trial_corpus::{State, TrialCorpus};

/// One root knob's two arms — the same `config set <key> <value>` driven over a corpus
/// whose committed docs the re-point strands, and over one where nothing is stranded.
struct Knob {
    /// The knob key, as `jigc config set` spells it.
    key: &'static str,
    /// The value both arms set — a home nothing is at yet, so the *only* difference
    /// between the two arms is what the corpus holds.
    value: &'static str,
    /// The corpus whose committed docs this knob's re-point strands.
    relocating: fn() -> TrialCorpus,
}

/// The `docs-root` cell's corpus: one committed ADR at `docs/decisions/cache-strategy.md`,
/// minted through `record-decision` and landed by a real `jigc task finalize` — a
/// `location` doctype, which is what `docs-root` re-roots.
fn adr_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-decision", "record a cache decision");
    let title = support::create_title("adr", "Cache Strategy");
    corpus.jigc_ok(&["doc", "create", "adr", "--title", &title, "--task", &task]);
    for slot in ["context", "decision", "consequences"] {
        corpus.set_slot(
            &format!("adr:cache-strategy#{slot}"),
            &task,
            &format!("Prose for {slot}.\n"),
        );
    }
    corpus.finalize(&task, "docs", "record the cache decision", false);
    corpus
}

/// The `placement-root` cell's corpus: the committed singleton set, two of whose
/// instances (`docs/roadmap.md`, `docs/decisions-log.md`) declare a home with a leading
/// directory component and so move when the placement root does.
fn singleton_corpus() -> TrialCorpus {
    TrialCorpus::build(State::CommittedSingletons)
}

/// The two root knobs, each with the value its arms set.
const KNOBS: &[Knob] = &[
    Knob {
        key: "docs-root",
        value: "documents",
        relocating: adr_corpus,
    },
    Knob {
        key: "placement-root",
        value: "notes",
        relocating: singleton_corpus,
    },
];

/// Drive `jigc --format json config set <key> <value>` and return its **stdout** — the
/// envelope alone, the stderr narration deliberately left out of the comparison.
fn drive(corpus: &TrialCorpus, knob: &Knob) -> String {
    corpus.jigc_ok(&["--format", "json", "config", "set", knob.key, knob.value])
}

/// The envelope's `relocated` rows as `(from, to)`, sorted — read off the emitted bytes.
fn relocated(stdout: &str, label: &str) -> Vec<(String, String)> {
    let doc: Value = serde_json::from_str(stdout)
        .unwrap_or_else(|err| panic!("{label}: the ack is one JSON document ({err}):\n{stdout}"));
    let rows = doc
        .get("relocated")
        .unwrap_or_else(|| {
            panic!(
                "{label}: the `config set` envelope must carry `relocated` on EVERY arm — the \
                 key is present always, `[]` when nothing moved (M51 Inc 5 / T4, EC-4). \
                 Envelope:\n{doc:#}"
            )
        })
        .as_array()
        .unwrap_or_else(|| panic!("{label}: `relocated` is a list; got:\n{doc:#}"))
        .iter()
        .map(|row| {
            let field = |name: &str| {
                row.get(name)
                    .and_then(Value::as_str)
                    .unwrap_or_else(|| {
                        panic!("{label}: each `relocated` row is `{{from, to}}`; got:\n{row:#}")
                    })
                    .to_string()
            };
            (field("from"), field("to"))
        })
        .collect::<Vec<_>>();
    sorted(rows)
}

/// The renames git actually has staged, as `(from, to)`, sorted — reality, against which
/// the ack's claim is checked.
fn staged_renames(corpus: &TrialCorpus) -> Vec<(String, String)> {
    let rows = corpus
        .git(&["diff", "--cached", "--name-status", "-M"])
        .lines()
        .filter_map(|line| {
            let mut cols = line.split('\t');
            let status = cols.next()?;
            if !status.starts_with('R') {
                return None;
            }
            Some((cols.next()?.to_string(), cols.next()?.to_string()))
        })
        .collect::<Vec<_>>();
    sorted(rows)
}

fn sorted(mut rows: Vec<(String, String)>) -> Vec<(String, String)> {
    rows.sort();
    rows
}

/// **EC-4, inverted.** For each knob, the same `config set` over a corpus it relocates and
/// over one it does not must not produce the same envelope. This is the reproduction the
/// census recorded, asserted as the behaviour it now must not have.
#[test]
fn the_relocating_arm_is_not_byte_identical_to_the_non_relocating_arm() {
    for knob in KNOBS {
        let moving = (knob.relocating)();
        let still = TrialCorpus::build(State::Fresh);

        let moved = drive(&moving, knob);
        let unmoved = drive(&still, knob);

        // Non-vacuity: the "relocating" arm really did relocate, established from git
        // rather than from the envelope under test.
        assert!(
            !staged_renames(&moving).is_empty(),
            "`{}`'s relocating cell must actually stage a move, or the comparison below \
             proves nothing; git staged nothing:\n{}",
            knob.key,
            moving.git(&["status", "--porcelain"]),
        );
        assert!(
            staged_renames(&still).is_empty(),
            "`{}`'s non-relocating cell must stage no move; git staged:\n{}",
            knob.key,
            still.git(&["status", "--porcelain"]),
        );

        assert_ne!(
            moved, unmoved,
            "`jigc --format json config set {} {}` emits the SAME bytes whether it relocated \
             the committed corpus or moved nothing (EC-4) — a driver reading the contract \
             surface cannot tell the two apart:\n{moved}",
            knob.key, knob.value,
        );
    }
}

/// **The ack names what git staged.** `relocated`'s rows are exactly the renames in the
/// index after the re-point — the binary's claim checked against reality, not against its
/// own stderr narration.
#[test]
fn relocated_names_the_from_to_git_actually_staged() {
    for knob in KNOBS {
        let corpus = (knob.relocating)();
        let stdout = drive(&corpus, knob);
        let label = format!("jigc config set {} {}", knob.key, knob.value);

        let claimed = relocated(&stdout, &label);
        let staged = staged_renames(&corpus);
        assert!(
            !claimed.is_empty(),
            "{label}: this cell relocates, so `relocated` must carry its moves; envelope:\n\
             {stdout}",
        );
        assert_eq!(
            claimed,
            staged,
            "{label}: `relocated` must name the moves git actually staged.\n\
             envelope:\n{stdout}\ngit status:\n{}",
            corpus.git(&["status", "--porcelain"]),
        );
    }
}

/// **The key is present always.** On the arm that moves nothing the ack still carries
/// `relocated`, as `[]` — a driver reads *nothing moved* rather than having to infer it
/// from a missing key.
#[test]
fn relocated_is_an_empty_list_when_nothing_moved() {
    for knob in KNOBS {
        let corpus = TrialCorpus::build(State::Fresh);
        let stdout = drive(&corpus, knob);
        let label = format!(
            "jigc config set {} {} (nothing stranded)",
            knob.key, knob.value
        );

        assert!(
            staged_renames(&corpus).is_empty(),
            "{label}: the fixture must strand nothing; git staged:\n{}",
            corpus.git(&["status", "--porcelain"]),
        );
        assert_eq!(
            relocated(&stdout, &label),
            Vec::new(),
            "{label}: `relocated` must be the empty list, not absent and not populated; \
             envelope:\n{stdout}",
        );
    }
}
