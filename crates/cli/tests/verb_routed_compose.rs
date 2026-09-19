//! M52 Increment 9 / T2 — **the two compose doors refuse a workflow that is
//! unreachable as composed** (`workflow.verb-routed`; settle-record D9 as amended by
//! §11).
//!
//! ## The class
//!
//! Fourteen shipped workflows are reached through a `jigc` verb of their own, and the
//! `suppressed:` block now says which (`door: <argv string>`, M52 Increment 9 / T1).
//! The door is what binds the input the body reads: `jigc migrate <path> --as adr`
//! stages the foreign source the twelve `migrate-*` bodies rewrite, `jigc milestone
//! execute <id>` binds the milestone whose sub-tasks the fan-out spawns, and `jigc
//! workflow sub-task --task <id>` names the sub-area the body writes into.
//!
//! Composed **by name** instead, every one of them rendered at exit 0 over an input
//! that is not there. Driven at rc.15
//! ([baseline-surfaces](../../../completions/artifacts/M52/baseline-surfaces.md) §2.2):
//! `jigc start --workflow migrate-adr "probe"` minted a task and printed *"Below is
//! the foreign source the CLI staged for you"* over three blank lines, and the walk it
//! opened landed a commit its own composed text called impossible. `jigc workflow
//! milestone-execution --preview` was worse than a dead end — it refused with a route
//! **into** the defect (`jigc start --workflow milestone-execution`), the door that
//! composes the lie.
//!
//! ## What this suite drives
//!
//! The member set is **derived at runtime** from both embedded packs' `door`
//! declarations — never a hand list here — and crossed with the three refusing forms
//! (`start --workflow <m> "<intent>"`, `start --workflow <m>`, `workflow <m>
//! --preview`). Beside them, the arms that must keep working: each declared door
//! itself (`jigc migrate`, `jigc milestone execute`, the sub-task re-entry), the
//! resume, and a non-member control that still composes at both doors — because a
//! refusal that fires one workflow too wide is the same defect pointed the other way.
//!
//! The refusal's own shape is read off the production registry rather than restated:
//! [`ENVELOPE_ARMS`]' cross-cutting `Reject::Findings` row names the top-level key set
//! and the stream, and the key is the **pack-resource** target form
//! (`design/command-output-contract.md` → `workflow-refs.*` — the pack-resource form,
//! keyed at the resource) that a workflow-subject finding takes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::{Command, Output, Stdio};

use cli::pack::EmbeddedPack;
use cli::render::{ArmShape, ENVELOPE_ARMS};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};

use crate::support;
use support::trial_corpus::{State, TrialCorpus};

/// The code every one of these refusals carries — the production constant, so the
/// suite cannot pin a literal the binary has moved off.
const CODE: &str = cli::start::VERB_ROUTED_CODE;

/// The cross-cutting reject arm a refusal carrying a `Finding` renders through.
const REJECT_ARM: &str = "Reject::Findings";

/// The verb-routed member set, **derived**: every workflow either embedded pack ships
/// whose `suppressed:` block declares a `door:`, as `(workflow id, door)`.
///
/// Read through the production loader from the packs the binary itself composes, so a
/// fifteenth member joins this sweep with no edit here — and a member that loses its
/// `door` leaves it the same way.
fn verb_routed_members() -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for pack in [EmbeddedPack::new(), EmbeddedPack::methodology()] {
        for id in pack.list(PackResourceKind::Workflows) {
            let id = id.as_str().to_owned();
            let bytes = pack
                .read(PackResourceKind::Workflows, &ResourceId::from(id.as_str()))
                .unwrap_or_else(|err| panic!("`{id}` must read from its pack: {err:?}"));
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|f| panic!("`{id}` must load: {}", f.message));
            if let Some(door) = def.suppressed.and_then(|s| s.door) {
                assert!(
                    out.insert(id.clone(), door).is_none(),
                    "`{id}` ships in both packs — the derivation would record one door",
                );
            }
        }
    }
    assert!(
        out.len() >= 14,
        "the derived verb-routed set is {} members — a sweep over a collapsed \
         derivation would pass vacuously",
        out.len(),
    );
    out
}

/// The three forms that compose a workflow **named by the caller** — the doors the
/// refusal binds at. Rendered as owned argv so each can be driven verbatim.
fn refusing_forms(member: &str) -> Vec<Vec<String>> {
    vec![
        vec![
            "start".into(),
            "--workflow".into(),
            member.into(),
            "refuse me".into(),
        ],
        vec!["start".into(), "--workflow".into(), member.into()],
        vec!["workflow".into(), member.into(), "--preview".into()],
    ]
}

/// The top-level key set `ENVELOPE_ARMS` declares for the reject-with-findings arm.
fn reject_findings_keys() -> Vec<&'static str> {
    let row = ENVELOPE_ARMS
        .iter()
        .find(|arm| arm.arm == REJECT_ARM && arm.path.is_empty())
        .unwrap_or_else(|| panic!("`{REJECT_ARM}` is a cross-cutting row of `ENVELOPE_ARMS`"));
    match row.shape {
        ArmShape::Object(keys) => {
            let mut keys = keys.to_vec();
            keys.sort_unstable();
            keys
        }
        _ => panic!("`{REJECT_ARM}` declares an object key set"),
    }
}

/// Run `jigc <args>` from `cwd` against `corpus`'s home — the corpus's own runner
/// plus a working directory, which the sub-task re-entry arm needs (a fanned
/// sub-agent re-enters from its provisioned worktree, never the main checkout).
fn jigc_in(corpus: &TrialCorpus, cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", corpus.home())
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("spawn jigc")
}

/// The corpus's whole mintable state: the task ids on disk and the porcelain tree.
/// A refusal moves neither.
fn mint_state(corpus: &TrialCorpus) -> (BTreeSet<String>, String) {
    let tasks = corpus.repo().join(".jigc").join("tasks");
    let ids = std::fs::read_dir(&tasks)
        .map(|entries| {
            entries
                .map(|e| {
                    e.expect("task dir entry")
                        .file_name()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect()
        })
        .unwrap_or_default();
    (ids, corpus.git(&["status", "--porcelain"]))
}

/// **The sweep.** Every derived member × every refusing form: exit 1, one blocking
/// `workflow.verb-routed` keyed at its own pack resource, routed at the declared door
/// argv-for-argv — and nothing minted, on either axis a mint would show.
#[test]
fn every_verb_routed_member_is_refused_at_both_compose_doors() {
    let corpus = TrialCorpus::build(State::Fresh);
    let before = mint_state(&corpus);
    let declared = reject_findings_keys();
    let members = verb_routed_members();
    let mut driven: BTreeSet<(String, usize)> = BTreeSet::new();

    for (member, door) in &members {
        for (form, argv) in refusing_forms(member).into_iter().enumerate() {
            let mut json: Vec<&str> = argv.iter().map(String::as_str).collect();
            let shown = format!("jigc {}", json.join(" "));
            json.extend(["--format", "json"]);

            // The text arm first — the bytes a human reads.
            let text = corpus.jigc(&argv.iter().map(String::as_str).collect::<Vec<_>>());
            assert_eq!(
                text.status.code(),
                Some(1),
                "{shown}: a verb-routed workflow is refused at exit 1; stdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&text.stdout),
                String::from_utf8_lossy(&text.stderr),
            );
            let text_err = String::from_utf8(text.stderr).expect("utf-8 stderr");
            assert!(
                text_err.contains(CODE),
                "{shown}: the printed refusal names `{CODE}`; got:\n{text_err}",
            );
            assert!(
                text_err.contains(&format!("`{door}`")),
                "{shown}: the printed refusal routes at the declared door `{door}`; \
                 got:\n{text_err}",
            );
            assert!(
                text.stdout.is_empty(),
                "{shown}: a refusal composes nothing on stdout; got:\n{}",
                String::from_utf8_lossy(&text.stdout),
            );

            // Then the machine arm — the key a driver branches on.
            let out = corpus.jigc(&json);
            assert_eq!(
                out.status.code(),
                Some(1),
                "{shown} --format json: exit 1; stderr:\n{}",
                String::from_utf8_lossy(&out.stderr),
            );
            assert!(
                out.stdout.is_empty(),
                "{shown} --format json: a reject leaves stdout empty; got:\n{}",
                String::from_utf8_lossy(&out.stdout),
            );
            let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
            let value: serde_json::Value = serde_json::from_str(&stderr).unwrap_or_else(|err| {
                panic!(
                    "{shown} --format json: stderr is the reject envelope ({err}); got:\n{stderr}"
                )
            });
            let mut keys: Vec<&str> = value
                .as_object()
                .unwrap_or_else(|| panic!("{shown}: the envelope is an object; got:\n{stderr}"))
                .keys()
                .map(String::as_str)
                .collect();
            keys.sort_unstable();
            assert_eq!(
                keys, declared,
                "{shown}: the envelope's top-level keys are exactly what `ENVELOPE_ARMS`' \
                 `{REJECT_ARM}` row declares — a code inside a flattened message projects \
                 no key; got:\n{stderr}",
            );
            let findings = value["findings"]
                .as_array()
                .unwrap_or_else(|| panic!("{shown}: `findings` is an array; got:\n{stderr}"));
            assert_eq!(
                findings.len(),
                1,
                "{shown}: one unreachable workflow is one finding; got:\n{stderr}",
            );
            let finding = &findings[0];
            assert_eq!(
                finding["severity"], "blocking",
                "{shown}: the refusal is blocking; got:\n{stderr}",
            );
            assert_eq!(
                finding["key"],
                serde_json::json!({ "code": CODE, "target": format!("workflow:{member}") }),
                "{shown}: the stable key is the code and the PACK RESOURCE the refusal is \
                 about — the target form a workflow-subject finding takes; got:\n{stderr}",
            );
            assert_eq!(
                finding["route"],
                serde_json::json!(format!("`{door}`")),
                "{shown}: the route is the declared door, argv-for-argv — the whole point \
                 of the `door` key; got:\n{stderr}",
            );

            driven.insert((member.clone(), form));
        }
    }

    assert_eq!(
        mint_state(&corpus),
        before,
        "a refused compose mints no task and touches no tracked path",
    );

    let want: BTreeSet<(String, usize)> = members
        .keys()
        .flat_map(|m| (0..3).map(move |form| (m.clone(), form)))
        .collect();
    assert_eq!(
        driven, want,
        "every derived member is driven at every refusing form — a member with no cell \
         is a failure, never a skip",
    );
}

/// **The sibling arms.** Each declared door still binds its own input and composes,
/// and a task already minted through one still resumes — the refusal binds the
/// compose-by-name act, never the doors themselves.
#[test]
fn the_declared_doors_and_the_resume_still_compose() {
    let corpus = TrialCorpus::build(State::Fresh);

    // `jigc migrate <path> --as adr` — the door the twelve `migrate-*` members declare.
    let foreign = corpus.repo().join("foreign-adr.md");
    std::fs::write(
        &foreign,
        "# Foreign ADR\n\n## Status\n\nAccepted\n\n## Context\n\nWhy.\n\n## Decision\n\nDo it.\n\n## Consequences\n\nFine.\n",
    )
    .expect("write the foreign adr");
    corpus.git(&["add", "foreign-adr.md"]);
    corpus.git(&["commit", "-m", "the foreign adr"]);
    let migrated = corpus.jigc_ok(&["migrate", "foreign-adr.md", "--as", "adr"]);
    assert!(
        migrated.contains("task minted:"),
        "`jigc migrate … --as adr` composes `migrate-adr` through its own door; got:\n{migrated}",
    );

    // `jigc milestone execute <milestone-id>` — `milestone-execution`'s door.
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "cache-rework",
        "Warm the read cache",
        "--workflow",
        "sub-task",
    ]);
    let executed = corpus.jigc_ok(&["milestone", "execute", "cache-rework"]);
    assert!(
        executed.contains("Spawn: "),
        "`jigc milestone execute` composes `milestone-execution` with its milestone \
         bound; got:\n{executed}",
    );

    // `jigc workflow sub-task --task <task-id>` — `sub-task`'s door, run from the
    // provisioned worktree, which is where a fanned sub-agent re-enters from (the main
    // checkout's HEAD is ahead of the sub-task's base pin by design).
    corpus.jigc_ok(&["milestone", "provision", "cache-rework"]);
    let worktree = corpus
        .repo()
        .join(".jigc")
        .join("worktrees")
        .join("warm-the-read-cache");
    let reentered = jigc_in(
        &corpus,
        &worktree,
        &["workflow", "sub-task", "--task", "warm-the-read-cache"],
    );
    assert!(
        reentered.status.success(),
        "`jigc workflow sub-task --task <id>` re-enters the sub-area from its worktree; \
         got {:?}\nstderr:\n{}",
        reentered.status,
        String::from_utf8_lossy(&reentered.stderr),
    );

    // `jigc start --task <id>` — the resume, which re-composes a task's own recorded
    // workflow and must not be read as a compose-by-name.
    let migrate_task = migrated
        .lines()
        .find_map(|line| line.strip_prefix("task minted: "))
        .expect("the migrate door prints the id it minted")
        .trim();
    let resumed = corpus.jigc_ok(&["start", "--task", migrate_task]);
    assert!(
        resumed.contains("Migrate the foreign ADR"),
        "`jigc start --task <id>` re-composes the migrate task it resumed; got:\n{resumed}",
    );
}

/// **The control, both directions.** A workflow declaring no `door` still composes at
/// both doors, and its *own* shipped refusal (a `creates-task: true` workflow with no
/// intent) is untouched — the verb-routed check fires on the declaration, never on
/// the shape of the workflow.
#[test]
fn a_workflow_with_no_declared_door_still_composes() {
    let corpus = TrialCorpus::build(State::Fresh);
    assert!(
        !verb_routed_members().contains_key("single-task"),
        "`single-task` is the non-member control — it must declare no door",
    );

    let previewed = corpus.jigc_ok(&["workflow", "single-task", "--preview"]);
    assert!(
        previewed.contains("preview: workflow `single-task`"),
        "the preview door still previews a member-less workflow; got:\n{previewed}",
    );

    let no_intent = corpus.jigc(&["start", "--workflow", "single-task"]);
    let no_intent_err = String::from_utf8(no_intent.stderr).expect("utf-8 stderr");
    assert!(
        !no_intent.status.success() && !no_intent_err.contains(CODE),
        "a `creates-task: true` workflow with no intent keeps its own refusal — never \
         `{CODE}`; got:\n{no_intent_err}",
    );

    let composed = corpus.jigc_ok(&["start", "--workflow", "single-task", "control intent"]);
    assert!(
        composed.contains("task minted: control-intent"),
        "the compose door still mints and composes a member-less workflow; got:\n{composed}",
    );
}
