//! M51 Increment 5, T2 — the three undeclared constants that duplicate the exit code
//! leave the wire.
//!
//! **The rule this suite enforces is T1's, already in the record**
//! ([command-output-contract.md](../../../design/command-output-contract.md) →
//! *Evolution posture (declared)*): *a constant ships iff it is declared as one and its
//! reason is written down*, and — the pre-pin removal clause — *while the window is open,
//! an undeclared key may be removed, declared in the same motion an addition would be*.
//! What the 1.0 pin freezes is the **declared** set, so a key no paragraph ever claimed
//! would be blessed by omission for the life of `1.x` the moment the pin ships.
//!
//! Three keys at HEAD were that shape, each a literal the producer cannot vary, each
//! duplicating a fact the **exit code** already carries — the retired-`discarded`
//! precedent verbatim (`command-output-contract.md` §2's ⚠ correction: *"the distinction
//! already exists, one layer up, and no key is needed to carry it"*):
//!
//! | key | verb | the constant | the discriminator that already exists |
//! |---|---|---|---|
//! | `installed` | `jigc setup` | `true` | exit **0** + this envelope vs exit 1 + `{error}` |
//! | `uninstalled` | `jigc uninstall` | `true` | the same |
//! | `review` | `jigc task finalize` (the exit-4 hold) | `"pending"` | exit **4**, declared at `command-output-contract.md` → the exit-code taxonomy as *the* review-hold code |
//!
//! **The class is asserted in both directions, which is what makes the rule checkable
//! rather than a one-way licence to delete.** Two constants ship *declared, with their
//! reason*, and they must survive this sweep untouched: `committed: false` on every
//! `ConfigAck` (*"a constant `false` is still a fact worth a key"*) and `findings: []` on
//! `jigc task diff` / `task bind` / `task discard` (*"structurally always empty, and
//! stated so rather than left to look incidental"*). A suite that only asserted the
//! absences would pass just as well over a binary that had deleted those two, and the
//! test would then be *constant?* — which is exactly the test the rule refuses.
//!
//! **What this suite pins, and what it does not.** Its subject is the three *removals*:
//! the key is gone, the envelope's remaining keys are exactly what they were, and the
//! fact the key carried is still readable off the exit code. The general fence — *every
//! arm's driven key set equals its declared key set*, over all 60 arms — is the
//! `EnvelopeArm` registry's (T7), and the three exhaustive key sets below are the
//! **removal's** own witness, not a second copy of that registry: a removal is one-way,
//! so "gone, and nothing else moved" earns its own assertion at the door.
//!
//! Drives the real binary over throwaway [`TrialCorpus`] corpora. The `setup` drive is
//! the idempotent re-run, the shipped precedent `format_json_success_axis.rs` uses —
//! `installed` is an unconditional literal in `render::setup_success`, so first-run and
//! re-run are one code path, not two arms.

use crate::support;

use cli::task::{EXIT_SUCCESS, ExitClass, exit_code_for};
use serde_json::Value;
use std::fs;
use std::process::Output;
use support::trial_corpus::{FOREIGN_VISION_PATH, State, TrialCorpus};

/// The foreign document the migration cell adopts — deliberately non-conformant (an H1
/// the schema does not name, a free-form `## Principles` section), mirroring the fixture
/// builder's own [`State::Migrated`] source so this suite invents no migration shape.
const FOREIGN_VISION: &str = "\
# Product Direction

We build a deterministic context compiler.

## Principles

Structure belongs to the CLI; prose belongs to the model.
";

/// The `doc author` batch payload the `migrate-vision` step's `{{schema:vision}}`
/// skeleton solicits, with the literal `<<…>>` slot markers it requires — the fixture
/// builder's payload, for the same reason.
const MIGRATED_VISION_PAYLOAD: &str = "\
title: Vision
sections:
  - id: thesis
    set:
      thesis: |-
        <<We build a deterministic context compiler.>>
  - id: invariants
    set:
      invariants: |-
        <<Structure belongs to the CLI; prose belongs to the model.>>
  - id: open-questions
    set:
      open-questions: |-
        <<Which domains earn a pack of their own.>>
";

/// The task id a mint printed, read off the binary's `task minted: <id>` line.
fn minted_task(stdout: &str) -> String {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .unwrap_or_else(|| panic!("a mint must print `task minted: <id>`; got:\n{stdout}"))
        .trim()
        .to_string()
}

/// `out`'s stdout as a JSON object, with the exit code asserted against the **table
/// constant** for `class` rather than a hand literal — the `exit_codes.rs` discipline: a
/// code that moved in `cli::task::EXIT_CODES` without moving here is unrepresentable.
#[track_caller]
fn envelope(out: &Output, class: ExitClass, what: &str) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(i32::from(exit_code_for(class))),
        "{what} must exit {}\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
        exit_code_for(class),
    );
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("{what} must emit one JSON document ({e}):\n{stdout}"))
}

/// The object's keys, sorted — the comparable form of "this envelope's key set".
#[track_caller]
fn keys(value: &Value, what: &str) -> Vec<String> {
    let map = value
        .as_object()
        .unwrap_or_else(|| panic!("{what} must be a JSON object; got: {value}"));
    let mut out: Vec<String> = map.keys().cloned().collect();
    out.sort();
    out
}

/// Assert `value`'s key set is exactly `expected` — the removal's own witness: the
/// deleted key is gone *and* nothing else moved with it, in one assertion.
#[track_caller]
fn exactly(value: &Value, expected: &[&str], what: &str) {
    let mut want: Vec<String> = expected.iter().map(|k| (*k).to_string()).collect();
    want.sort();
    assert_eq!(
        keys(value, what),
        want,
        "{what}: the envelope's key set must be exactly its declared siblings \
         (a deleted key back on the wire, or a sibling dropped with it, both land here)\n\
         --- envelope ---\n{value:#}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// The three deletes.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn setup_drops_installed_and_the_exit_code_carries_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let out = corpus.jigc(&["setup", "--format", "json"]);
    let envelope = envelope(&out, ExitClass::Success, "`jigc setup --format json`");

    assert!(
        envelope.get("installed").is_none(),
        "`installed` was a literal `true` the producer could not vary, and exit \
         {EXIT_SUCCESS} + this envelope already says the install happened (exit 1 + \
         `{{error}}` says it did not) — it must be off the wire\n--- envelope ---\n{envelope:#}",
    );
    exactly(
        &envelope,
        &[
            "allowlist_file",
            "findings",
            "guide_file",
            "hook_committed",
            "hook_file",
            "install_commit",
            "line_file",
        ],
        "`jigc setup --format json`",
    );
}

#[test]
fn uninstall_drops_uninstalled_and_the_exit_code_carries_it() {
    let corpus = TrialCorpus::build(State::Fresh);
    let out = corpus.jigc(&["uninstall", "--format", "json"]);
    let envelope = envelope(&out, ExitClass::Success, "`jigc uninstall --format json`");

    assert!(
        envelope.get("uninstalled").is_none(),
        "`uninstalled` was a literal `true`, and exit {EXIT_SUCCESS} + this envelope \
         already says the teardown happened — it must be off the wire\
         \n--- envelope ---\n{envelope:#}",
    );
    exactly(
        &envelope,
        &["allowlist_file", "findings", "line_file", "removed"],
        "`jigc uninstall --format json`",
    );
    // The removal ledger is the key a teardown script actually reads, so "siblings
    // unchanged" reaches inside it: the delete above must not have disturbed a flag.
    exactly(
        envelope.get("removed").expect("`removed` is a sibling"),
        &[
            "allowlist",
            "deny",
            "guide",
            "hook",
            "jigc_dir",
            "precommit",
            "reference",
        ],
        "`jigc uninstall --format json` → `removed`",
    );
}

#[test]
fn the_review_hold_drops_review_and_exit_4_carries_it() {
    let corpus = TrialCorpus::build(State::Fresh);

    // A foreign document committed as ordinary repo furniture, then routed through
    // `jigc migrate … --as vision` — the fixture builder's own `State::Migrated`
    // sequence, stopped one step short of the `--approve` that lands it.
    let foreign = corpus.repo().join(FOREIGN_VISION_PATH);
    fs::create_dir_all(foreign.parent().expect("the foreign source has a parent"))
        .expect("create the foreign source dir");
    fs::write(&foreign, FOREIGN_VISION).expect("write the foreign source");
    corpus.git(&["add", FOREIGN_VISION_PATH]);
    corpus.git(&["commit", "-q", "-m", "add the direction doc"]);

    let task = minted_task(&corpus.jigc_ok(&["migrate", FOREIGN_VISION_PATH, "--as", "vision"]));
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
        MIGRATED_VISION_PAYLOAD,
    );
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), &task, "vision");
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "migrate the direction doc into the managed vision",
    );
    corpus.set_slot(
        &format!("commit:{task}#body"),
        &task,
        "Driven by the envelope-constants suite.",
    );

    // No `--approve`: the human-only review gate, which commits nothing and exits 4.
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let envelope = envelope(
        &out,
        ExitClass::MigrationReview,
        "the exit-4 migration review hold",
    );

    assert!(
        envelope.get("review").is_none(),
        "`review` could only ever hold the string `\"pending\"`, and exit {} is declared \
         as *the* review-hold code — the key duplicated it\n--- envelope ---\n{envelope:#}",
        exit_code_for(ExitClass::MigrationReview),
    );
    exactly(
        &envelope,
        &["retires", "rewrites", "source", "task"],
        "the exit-4 migration review hold",
    );
    // The hold is still a hold: it committed nothing, so the fact the deleted key
    // claimed ("pending") remains true of the state, not only of the exit code.
    assert!(
        corpus.repo().join(FOREIGN_VISION_PATH).exists(),
        "a review hold retires nothing",
    );
    assert!(
        !corpus.repo().join("VISION.md").exists(),
        "a review hold writes no canonical doc",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// The other half of the class: the two constants that ship DECLARED, and therefore
// stay. Without these the rule reads as "delete every constant", which is not it.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn the_declared_constant_committed_false_stays_on_the_config_ack() {
    let corpus = TrialCorpus::build(State::Fresh);
    let out = corpus.jigc(&["config", "set", "docs-root", "docs2", "--format", "json"]);
    let envelope = envelope(&out, ExitClass::Success, "`jigc config set --format json`");

    assert_eq!(
        envelope.get("committed"),
        Some(&Value::Bool(false)),
        "`committed: false` is a DECLARED constant with its reason on the record (a \
         cascade write is never committed for you) — the removal rule reaches undeclared \
         constants only\n--- envelope ---\n{envelope:#}",
    );
}

#[test]
fn the_declared_constant_findings_empty_stays_on_the_task_acks() {
    let corpus = TrialCorpus::build(State::Fresh);
    let task = minted_task(&corpus.jigc_ok(&[
        "start",
        "--workflow",
        "quick-fix",
        "probe the declared constants",
    ]));

    let diff = corpus.jigc(&["task", "diff", &task, "--format", "json"]);
    let diff = envelope(&diff, ExitClass::Success, "`jigc task diff --format json`");
    assert_eq!(
        diff.get("findings"),
        Some(&Value::Array(Vec::new())),
        "`findings: []` is a DECLARED constant with its reason on the record \
         (structurally always empty, stated rather than left to look incidental)\
         \n--- envelope ---\n{diff:#}",
    );

    // `--force` because the task stages a commit doc no commit has a copy of; the ack
    // is the same `TaskAck` either way.
    let discard = corpus.jigc(&["task", "discard", &task, "--force", "--format", "json"]);
    let discard = envelope(
        &discard,
        ExitClass::Success,
        "`jigc task discard --format json`",
    );
    assert_eq!(
        discard.get("findings"),
        Some(&Value::Array(Vec::new())),
        "the same declared constant on the discard ack\n--- envelope ---\n{discard:#}",
    );
}
