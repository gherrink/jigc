//! M51 Increment 5 / T3 — **the `render::milestone` envelope arm is chosen from
//! [`VERB_KINDS`], not from the verb's identity**
//! ([command-output-contract.md](../../../design/command-output-contract.md) → Stream
//! discipline; `completions/artifacts/M51/envelope-key-census.md` §4.3;
//! `implementation/pinning.md` §2 — a contract property suite).
//!
//! `jigc milestone list-tasks` is the milestone surface's one [`VerbKind::Read`] leaf. It
//! shared `render::milestone` with five **write** siblings and therefore shipped
//! `{"hook_output": "", "text": …}` — a commit-hook key on a verb that runs no commit, so
//! no hook can ever speak into it and the value is structurally `""` forever.
//! `hook_output`'s declaration is scoped to *"the landed-commit envelopes … and the
//! milestone record-only op acks"*; a read verb is none of those, so the key is **deleted**
//! rather than declared (the census's disposition, and D5's).
//!
//! **What this suite fences, and why it is a rule rather than a repair.** The arm is
//! selected at the dispatch site by looking the verb's own leaf path up in [`VERB_KINDS`]
//! ([`cli::cli::verb_kind`]) — never by hand-casing `MilestoneCommand::ListTasks` — so the
//! expectations below are **derived** the same way: a door's key set is a pure function of
//! its [`VerbKind`], `Read` → `["text"]` and `Write` → `["hook_output", "text"]`. A
//! seventh milestone verb rendered through `render::milestone` therefore inherits the
//! right arm the day it classifies itself, and a re-classification of an existing one
//! moves its envelope and this fence together.
//!
//! **The axis is the nine milestone leaves of [`VERB_KINDS`]**, not the six doors driven
//! here: [`DOORS`] bijects that set, and the three verbs that render through their own
//! dispatch arm ([`Arm::RendersElsewhere`]) say so with their reason, so a milestone verb
//! cannot join the tree without being answered here.
//!
//! **Non-vacuity is asserted, not assumed.** A derived expectation is worthless if every
//! driven door lands on the same side of it, so [`every_render_milestone_door_emits_the_arm_its_verb_kind_selects`]
//! additionally requires that **both** kinds were observed on real bytes.
//!
//! **The second fence is about the other half of the surface** — the arm a *refusal* takes
//! (M53 Increment 5 / T1, the spike D5 owes):
//! [`a_milestone_refusal_takes_the_flattened_arm_unless_its_code_is_declared`] drives one
//! door on each side of the membership test and reads both declarations, so the relation it
//! states — *flattened is the default here, and the two lists are its exception set* —
//! moves with them rather than being restated. It lives beside the success-envelope fence
//! because both answer one question about one surface: which document a driver reads back.

use crate::support;

use cli::cli::{VERB_KINDS, VerbKind, verb_kind};
use cli::milestone::ENVELOPE_ARM_CODES;
use cli::render::ENVELOPE_OWED_CODES;
use cli::task::{EXIT_ERROR, EXIT_SUCCESS};
use engine::milestone::UNKNOWN_MILESTONE_CODE;
use serde_json::Value;
use std::collections::BTreeSet;
use std::process::Output;
use support::trial_corpus::{State, TrialCorpus};

// ─────────────────────────────── the door table ───────────────────────────────

/// One milestone leaf verb and how its success envelope is rendered.
struct Door {
    /// The leaf path, exactly as [`VERB_KINDS`] spells it.
    path: &'static [&'static str],
    /// Which renderer answers this door — and, for the six that share
    /// `render::milestone`, how to drive it to a real success.
    arm: Arm,
}

/// Which renderer produces a milestone verb's success envelope.
enum Arm {
    /// `render::milestone` — driven here through the real binary, with the expected key
    /// set **derived** from the door's [`VerbKind`] rather than written down.
    RendersMilestone {
        /// The corpus this door's success needs.
        base: Base,
        /// Drive the verb under `--format json` and hand back the raw process output.
        drive: fn(&TrialCorpus) -> Output,
    },
    /// The verb has its own dispatch arm and its own renderer, so `render::milestone`'s
    /// key set says nothing about it. The reason is stated, because an absence from the
    /// driven set is a decision here, not a hole.
    RendersElsewhere(&'static str),
}

/// The corpus a door's success needs — the `format_json_success_axis.rs` idiom, narrowed
/// to the two states the milestone surface reaches.
#[derive(Clone, Copy)]
enum Base {
    /// `jigc setup` only.
    Fresh,
    /// [`Base::Fresh`] plus a **committed** `spec:rate-limiting` carrying one criterion —
    /// what `milestone add-from-spec` enumerates.
    CommittedSpec,
}

/// **Every milestone leaf verb**, bijected against [`VERB_KINDS`] by
/// [`the_door_table_bijects_the_milestone_leaves_of_verb_kinds`].
const DOORS: &[Door] = &[
    Door {
        path: &["milestone", "create"],
        arm: Arm::RendersMilestone {
            base: Base::Fresh,
            drive: |c| json(c, &["milestone", "create", "Cache rework"]),
        },
    },
    Door {
        path: &["milestone", "add-task"],
        arm: Arm::RendersMilestone {
            base: Base::Fresh,
            drive: |c| {
                c.jigc_ok(&["milestone", "create", "Cache rework"]);
                json(
                    c,
                    &[
                        "milestone",
                        "add-task",
                        "cache-rework",
                        "Warm the read cache",
                    ],
                )
            },
        },
    },
    Door {
        path: &["milestone", "add-from-spec"],
        arm: Arm::RendersMilestone {
            base: Base::CommittedSpec,
            drive: |c| {
                c.jigc_ok(&["milestone", "create", "Cache rework"]);
                json(
                    c,
                    &[
                        "milestone",
                        "add-from-spec",
                        "cache-rework",
                        "spec:rate-limiting",
                    ],
                )
            },
        },
    },
    Door {
        path: &["milestone", "list-tasks"],
        arm: Arm::RendersMilestone {
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "list-tasks", "cache-rework"])
            },
        },
    },
    Door {
        path: &["milestone", "provision"],
        arm: Arm::RendersMilestone {
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "provision", "cache-rework"])
            },
        },
    },
    Door {
        path: &["milestone", "execute"],
        arm: Arm::RendersElsewhere(
            "`execute` composes a workflow: it renders through `render::composed`, whose \
             pinned `{task, text}` projection is a different contract entirely",
        ),
    },
    Door {
        path: &["milestone", "join"],
        arm: Arm::RendersElsewhere(
            "`join` reports a `JoinOutcome` (overlay + findings), not a one-line summary, \
             so it has its own dispatch arm and `render::milestone_join`",
        ),
    },
    Door {
        path: &["milestone", "finalize"],
        arm: Arm::RendersElsewhere(
            "`finalize` is the commit boundary: it drives the shared finalize-plan \
             executor and renders through `render::milestone_finalized`",
        ),
    },
    Door {
        path: &["milestone", "discard"],
        arm: Arm::RendersMilestone {
            base: Base::Fresh,
            drive: |c| {
                milestone_with_subtask(c);
                json(c, &["milestone", "discard", "cache-rework"])
            },
        },
    },
];

/// **The arm a [`VerbKind`] selects** — the rule this suite fences, written once.
///
/// A `Write` door may land a record-only commit, so its envelope carries the captured
/// non-blocking hook stream **present-always** (the empty string when no hook spoke). A
/// `Read` door commits nothing, so the key is **absent** — not empty, which would be a
/// claim that a hook could have spoken and did not.
fn expected_keys(kind: VerbKind) -> Vec<&'static str> {
    match kind {
        VerbKind::Read => vec!["text"],
        VerbKind::Write => vec!["hook_output", "text"],
    }
}

// ───────────────────────────── driving helpers ─────────────────────────────

/// Run the verb under test: `jigc --format json <args…>`, raw [`Output`].
fn json(corpus: &TrialCorpus, args: &[&str]) -> Output {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    corpus.jigc(&argv)
}

/// A milestone with one sub-task, minted `--workflow sub-task` (the `flow10` shape).
fn milestone_with_subtask(corpus: &TrialCorpus) {
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "cache-rework",
        "Warm the read cache",
        "--workflow",
        "sub-task",
    ]);
}

/// The two bases, built once and copied per door so a mutating verb never sees a
/// sibling's leftovers.
struct Bases {
    fresh: TrialCorpus,
    committed_spec: TrialCorpus,
}

impl Bases {
    fn build() -> Self {
        let fresh = TrialCorpus::build(State::Fresh);

        let committed_spec = fresh.copy_state();
        let task = committed_spec.start_workflow("plan", "spec the rate limiter");
        committed_spec.jigc_ok(&[
            "doc",
            "create",
            "spec",
            "--title",
            "Rate limiting",
            "--task",
            &task,
        ]);
        committed_spec.set_slot("spec:rate-limiting#goal", &task, "Cap requests per client.");
        committed_spec.set_slot(
            "spec:rate-limiting#context",
            &task,
            "The gateway is unprotected.",
        );
        let criterion =
            committed_spec.add_item("spec:rate-limiting#criteria", "Limits per IP", &task);
        committed_spec.set_slot(
            &format!("{criterion}/statement"),
            &task,
            "A client over the cap is rejected.",
        );
        committed_spec.finalize(&task, "spec", "spec the rate limiter", false);

        Bases {
            fresh,
            committed_spec,
        }
    }

    /// A **copy** of the named base — the per-door working corpus.
    fn corpus(&self, base: Base) -> TrialCorpus {
        match base {
            Base::Fresh => self.fresh.copy_state(),
            Base::CommittedSpec => self.committed_spec.copy_state(),
        }
    }
}

// ─────────────────────────────── the assertions ───────────────────────────────

/// **The arm fence.** Every door rendered by `render::milestone` is driven to a real
/// success through the real binary, and its envelope's key set is exactly the one its
/// [`VerbKind`] selects — `list-tasks` alone on the `Read` side.
#[test]
fn every_render_milestone_door_emits_the_arm_its_verb_kind_selects() {
    let bases = Bases::build();
    let mut kinds_observed: Vec<VerbKind> = Vec::new();

    for door in DOORS {
        let Arm::RendersMilestone { base, drive } = door.arm else {
            continue;
        };
        let label = format!("jigc {}", door.path.join(" "));
        let kind = verb_kind(door.path)
            .unwrap_or_else(|| panic!("`{label}` must name a classified `VERB_KINDS` leaf"));

        let corpus = bases.corpus(base);
        let out = drive(&corpus);
        assert_eq!(
            out.status.code(),
            Some(i32::from(EXIT_SUCCESS)),
            "`{label} --format json` must reach a real success; got {:?}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        let value: Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
            panic!("`{label}`'s envelope is one JSON document ({err}); got:\n{stdout}")
        });
        let mut keys: Vec<&str> = value
            .as_object()
            .unwrap_or_else(|| panic!("`{label}`'s envelope is a JSON object; got:\n{stdout}"))
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();

        assert_eq!(
            keys,
            expected_keys(kind),
            "`{label}` is `{kind:?}`, so its envelope must carry exactly the key set that \
             kind selects — a `Read` door commits nothing, so `hook_output` is absent \
             rather than empty. Got:\n{stdout}",
        );
        if !kinds_observed.contains(&kind) {
            kinds_observed.push(kind);
        }
    }

    // Non-vacuity: a derived expectation proves nothing if every driven door sits on the
    // same side of it.
    assert!(
        kinds_observed.contains(&VerbKind::Read) && kinds_observed.contains(&VerbKind::Write),
        "the driven set must exercise BOTH arms, or the derivation is untested; \
         observed {kinds_observed:?}",
    );
}

/// **The totality arm.** [`DOORS`] bijects the milestone leaves of [`VERB_KINDS`], so a
/// milestone verb added to the tree cannot ship without stating which renderer answers
/// it — and a door deleted here cannot outlive the verb it spoke for.
#[test]
fn the_door_table_bijects_the_milestone_leaves_of_verb_kinds() {
    let declared: BTreeSet<Vec<&str>> = VERB_KINDS
        .iter()
        .filter(|(path, _)| path.first() == Some(&"milestone"))
        .map(|(path, _)| path.to_vec())
        .collect();
    let covered: BTreeSet<Vec<&str>> = DOORS.iter().map(|door| door.path.to_vec()).collect();

    assert_eq!(
        covered, declared,
        "every `milestone` leaf in `VERB_KINDS` needs exactly one row here, and no row may \
         name a verb the tree does not carry",
    );
    assert_eq!(
        declared.len(),
        9,
        "the milestone surface is nine leaf verbs; a simultaneous delete on both sides \
         must stay visible rather than shrinking the axis in silence. Got: {declared:?}",
    );

    // A row that is NOT driven owes a reason — an undriven door with an empty reason is
    // the hole this table exists to refuse.
    for door in DOORS {
        if let Arm::RendersElsewhere(why) = door.arm {
            assert!(
                !why.trim().is_empty(),
                "`jigc {}` is excluded from the driven set, so it owes the reason its \
                 envelope is not `render::milestone`'s",
                door.path.join(" "),
            );
        }
    }
}

// ──────────── the reject arm (M53 Increment 5 / T1 — the owed spike) ────────────

/// **The spike D5 owes** (`completions/artifacts/M53/acceptance-design.md` → *Spikes owed*,
/// row 5; [settle-record.md](../../../completions/artifacts/M53/settle-record.md) → D5,
/// *"which reject arm it rides … relayed both ways"*).
///
/// D5 mints a third `write.unslugable-title` producer at this module's mint doors and
/// requires that **no arm moves** — the new finding rides whichever arm a finding at these
/// doors rides today. Planning relayed both answers (the auditor observed the flattened
/// `{"error": …}`; M52 Increment 1 moved finding-carrying rejects onto the findings arm), so
/// which it is, is a fact about the binary and is driven here before anything is built on it.
///
/// **The relation this fences, stated once:** at this surface's doors the flattened
/// `{"error": …}` is the **default**, and the envelope is taken only by a code some
/// declaration names — [`ENVELOPE_ARM_CODES`] (this module's own list) or
/// [`ENVELOPE_OWED_CODES`] (the standing obligation `render::carrier` asks). Both lists are
/// **read**, never restated, so a code that joins either one moves this expectation with it.
///
/// **Non-vacuity is asserted, not assumed:** one door on each side of the membership test is
/// driven to a real refusal through the real binary, and the two key sets must differ.
///
/// If this ever reds because the *outside* door answers the envelope, the default arm has
/// moved and D5's *"no arm moves"* is unsatisfiable as written — that is a halt to the
/// human, not a test to update.
#[test]
fn a_milestone_refusal_takes_the_flattened_arm_unless_its_code_is_declared() {
    let base = TrialCorpus::build(State::Fresh);

    // ── outside the declared set: `milestone.record-exists` at `jigc milestone create` ──
    let outside = base.copy_state();
    outside.jigc_ok(&["milestone", "create", "Cache rework"]);
    let out = json(&outside, &["milestone", "create", "Cache rework"]);
    let flat = refusal_envelope("jigc milestone create (record exists)", out);
    let flat_keys = keys_of(&flat);
    assert_eq!(
        flat_keys,
        vec!["error"],
        "a re-`create` over a live record must refuse on the FLATTENED arm — got {flat:#}",
    );
    // The code is read off the emitted bytes rather than written down, so a renamed producer
    // cannot leave the membership legs below quietly matching nothing.
    let flat_text = flat["error"].as_str().expect("`error` is a string");
    let outside_code = rendered_code(flat_text);
    assert!(
        !ENVELOPE_ARM_CODES.contains(&outside_code) && !ENVELOPE_OWED_CODES.contains(&outside_code),
        "`{outside_code}` is named by neither declaration, so it must land on the default \
         arm — if it has since joined one, this door's arm moved with it",
    );

    // ── inside it: `milestone.unknown` at `jigc milestone add-task` ──
    let inside = base.copy_state();
    let out = json(
        &inside,
        &[
            "milestone",
            "add-task",
            "no-such-milestone",
            "Warm the read cache",
        ],
    );
    let enveloped = refusal_envelope("jigc milestone add-task (unknown milestone)", out);
    assert!(
        ENVELOPE_ARM_CODES.contains(&UNKNOWN_MILESTONE_CODE),
        "`{UNKNOWN_MILESTONE_CODE}` is this module's declared exception; without that \
         membership the cell below proves nothing about the relation",
    );
    assert_eq!(
        keys_of(&enveloped),
        vec!["findings", "schema_version"],
        "a declared code must refuse on the FINDINGS arm — got {enveloped:#}",
    );
    let finding = &enveloped["findings"][0];
    assert_eq!(
        finding["code"].as_str(),
        Some(UNKNOWN_MILESTONE_CODE),
        "the enveloped refusal projects the declared code — got {enveloped:#}",
    );
    assert_eq!(
        finding["key"]["target"].as_str(),
        Some("milestone:no-such-milestone"),
        "…under the work-unit target form the contract lists it at — got {enveloped:#}",
    );

    // Non-vacuity: the two arms must be different documents, or the membership test above
    // predicted nothing.
    assert_ne!(
        flat_keys,
        keys_of(&enveloped),
        "both doors answered the same key set, so the relation is untested",
    );
}

/// Drive a refusal and hand back its `--format json` document: exit 1 (the operational
/// funnel's code), and the document on **stderr** with stdout empty — the reject arms'
/// stream discipline (`design/command-output-contract.md` → Stream discipline), asserted
/// here because a document read off the wrong stream would make the key-set legs above
/// vacuous.
fn refusal_envelope(label: &str, out: Output) -> Value {
    assert_eq!(
        out.status.code(),
        Some(i32::from(EXIT_ERROR)),
        "`{label}` must refuse at the operational exit; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        stdout.is_empty(),
        "`{label}` refuses, so its document is stderr's and nothing else is written; \
         stdout carried:\n{stdout}",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    serde_json::from_str(&stderr).unwrap_or_else(|err| {
        panic!("`{label}`'s refusal is one JSON document ({err}); got:\n{stderr}")
    })
}

/// The sorted key set of a JSON object envelope.
fn keys_of(value: &Value) -> Vec<&str> {
    let mut keys: Vec<&str> = value
        .as_object()
        .unwrap_or_else(|| panic!("the envelope is a JSON object; got:\n{value:#}"))
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    keys
}

/// The `code` token of a house findings line (`severity · code — message`), read off the
/// flattened arm's own bytes.
fn rendered_code(line: &str) -> &str {
    let (_, after_severity) = line.split_once('·').unwrap_or_else(|| {
        panic!("a flattened refusal carries the house findings line; got:\n{line}")
    });
    let (code, _) = after_severity
        .split_once('—')
        .unwrap_or_else(|| panic!("the findings line separates code from message; got:\n{line}"));
    code.trim()
}
