//! M51 Increment 1 / T4 — **a step source is read as a source**, and the `--from-file`
//! handoff family takes its explicit disposition
//! (`completions/artifacts/M51/settle-record.md` → §2 *"`file` and `from_file` get a SOURCE
//! rule (S11)"* · the §10 mold; `design/overrides.md` → Authoring deltas;
//! `design/write-commands.md` → Content handoff).
//!
//! `jigc config insert-step` / `replace-step` took their `<file>` argument as an opaque
//! token: `std::fs::read(file)`, `file_stem()`, and the bytes landed verbatim at
//! `.jigc/config/steps/<stem>.yaml` — an in-repo, committable file that **composes into the
//! step text `jigc start` hands the agent**. Driven at `dddc11a5`, all three of these exited
//! **0**:
//!
//! ```text
//! jigc config insert-step --workflow single-task --after implement .git/config
//! jigc config replace-step 'workflow:single-task#implement' .git/config
//! jigc config insert-step --workflow single-task --after implement .jigc/scratch/wb.yaml
//! ```
//!
//! The first two copied this repository's **git config** into `.jigc/config/steps/config.yaml`
//! (`baseline-tokens.md` §2d drove the composed harm: `repositoryformatversion = 0` rendered
//! into the agent's step text).
//!
//! **The rule is a SOURCE rule, and that is the whole of §2's correction.** D1 part 1 had
//! pre-committed this door to `untrackable_reason` + `is_workbench_root`, which are
//! *destination* predicates and ask the wrong question twice over: the first refuses
//! *"resolves outside the repository root"*, which would kill a shared team steps library at
//! `~/steps/foo.yaml`, and the second refuses every path under `.jigc/` — precisely where
//! these two verbs **write**. So the rule asked here is: **readable · no `.git` component ·
//! not reached through the workbench**, and an **out-of-repo source stays allowed** — the
//! caller names it and the bytes land visibly, copied into a file their next diff shows.
//!
//! Arm (c) is that admitting half, and it is not decoration: it is the cell that fails if the
//! door is ever "fixed" with the destination predicate.
//!
//! **The `--from-file` family carries the registry's stated no-rule-and-why** (the disposition
//! is written at `crate::doc::read_handoff`, one home for its three occurrences). Arm (d) is
//! its regression guard: the `-` sentinel is not a path and must keep reading stdin at all
//! three doors — which is also the reason those doors take no path rule, since a rule there
//! would refuse a spelling while the door's own declared grammar hands the identical bytes
//! through the pipe.

use std::fs;
use std::path::Path;

use crate::support::trial_corpus::{State, TrialCorpus};

/// The one code this door carries (`settle-record.md` → §10's table). One code with the
/// reason in the message — `config.untrackable-root`'s five-reasons-one-code precedent: the
/// operator's fix is the same whichever leg answered, namely name a different source file.
const CODE: &str = "config.step-source-untrackable";

/// The adr payload arm (d) authors — the smallest complete `adr` batch, so the stdin proof at
/// `doc author` is a *successful* author rather than an inference from a later refusal.
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

/// The project layer's step shadow directory — the destination these verbs write, asserted
/// **empty** after every refusal.
fn steps_dir(repo: &Path) -> std::path::PathBuf {
    repo.join(".jigc").join("config").join("steps")
}

/// The project manifest's raw bytes (`""` when it does not exist) — a refusal must append no
/// delta, and the manifest is where a delta would land.
fn manifest(repo: &Path) -> String {
    fs::read_to_string(repo.join(".jigc").join("config").join("manifest.yaml")).unwrap_or_default()
}

/// Assert `out` is the §10 refusal — **exit 1**, [`CODE`], **exactly one** `route:` line — and
/// that the run wrote nothing: no native step file, no appended delta.
///
/// The exit code is asserted as the number rather than as *non-zero*: a refusal surfacing at 3
/// would be a validation verdict (`design/validation.md` → Exit semantics), and this is an
/// operational refusal at the door, before any override exists to have a verdict about.
fn assert_refused(corpus: &TrialCorpus, out: &std::process::Output, before: &str, what: &str) {
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(1),
        "{what} must be refused at exit 1 (the §10 mold); got {}\n--- stdout ---\n{stdout}\
         \n--- stderr ---\n{stderr}",
        out.status,
    );
    assert!(
        stderr.contains(CODE),
        "{what} must name `{CODE}`; got:\n{stderr}",
    );
    let routes: Vec<&str> = stderr
        .lines()
        .filter(|line| line.trim_start().starts_with("route:"))
        .collect();
    assert_eq!(
        routes.len(),
        1,
        "{what} must carry exactly one route line (the route floor); got:\n{stderr}",
    );
    let written: Vec<String> = fs::read_dir(steps_dir(&corpus.repo()))
        .map(|entries| {
            let mut names: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect();
            names.sort();
            names
        })
        .unwrap_or_default();
    assert_eq!(
        written,
        Vec::<String>::new(),
        "{what} must write no native step file — the source is adjudicated BEFORE the write",
    );
    assert_eq!(
        manifest(&corpus.repo()),
        before,
        "{what} must append no delta — a refusal leaves the manifest byte-identical",
    );
}

/// Drive one door over `source`, on its own copy of the built state.
fn drive(corpus: &TrialCorpus, door: &str, source: &str) -> std::process::Output {
    match door {
        "insert-step" => corpus.jigc(&[
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            source,
        ]),
        "replace-step" => corpus.jigc(&[
            "config",
            "replace-step",
            "workflow:single-task#implement",
            source,
        ]),
        other => panic!("unknown door `{other}`"),
    }
}

// ───────────────── (a)+(b) the refusing half, at BOTH occurrences of `file` ─────────────────

/// **The three refusing shapes, at both occurrences of the `file` argument** — the source
/// rule's whole refusing surface, iterated rather than pinned at one door.
///
/// `file` occurs twice in the clap tree (`config insert-step`, `config replace-step`), which is
/// why the M51 registry is keyed by *occurrence* and not by deduplicated argument id (§2). Both
/// occurrences reach the same `fs::read` + `file_stem` + write sequence, so a guard at one of
/// them would close the reported repro and leave the class open at its sibling.
///
/// The third shape is the **symlink**, and it is here because it is the cell that proves the
/// rule is asked of the *bytes* rather than of the spelling: a source rule canonicalizes the
/// **leaf** — the opposite of the retire path's rule, where the leaf is the subject of a later
/// delete — because the read follows the link. That is also how *"not reached through the
/// workbench"* is satisfied rather than merely tested for.
///
/// The workbench cell is a **transient** subtree (`.jigc/state/`), never `.jigc/` whole: its
/// admitting sibling is the next arm.
#[test]
fn a_step_source_in_gits_dir_or_in_the_workbench_is_refused_at_both_doors() {
    let built = TrialCorpus::build(State::Fresh);

    for door in ["insert-step", "replace-step"] {
        for shape in [".git/config", ".jigc/state/wb.yaml", "sneaky.yaml"] {
            let corpus = built.copy_state();
            // The workbench cell needs a real readable file there: the cell is the source's
            // LOCATION, never an unreadable path, so a refusal must not be an I/O error
            // wearing the code's name. `state/` because the leg's subject is the **transient**
            // workbench — the subtrees `.jigc/.gitignore` declares — and not `.jigc/` whole.
            let state = corpus.repo().join(".jigc").join("state");
            fs::create_dir_all(&state).expect("create the workbench state dir");
            fs::write(state.join("wb.yaml"), "Workbench body.\n").expect("write the wb source");
            // …and the symlink cell needs one pointing INTO git's own directory, whose target
            // is a perfectly readable file: what must refuse it is where it lands.
            std::os::unix::fs::symlink(".git/config", corpus.repo().join("sneaky.yaml"))
                .expect("plant the symlink into git's own directory");

            let before = manifest(&corpus.repo());
            let subject = corpus.repo().join(shape);
            let bytes = fs::read(&subject).expect("the cell's source file is readable");

            let out = drive(&corpus, door, shape);
            assert_refused(
                &corpus,
                &out,
                &before,
                &format!("`config {door} … {shape}`"),
            );

            assert_eq!(
                fs::read(&subject).expect("read the source back"),
                bytes,
                "`config {door} … {shape}` must leave the source byte-identical",
            );
        }
    }
}

// ───────────────── (c) the admitting half: an out-of-repo source still lands ─────────────────

/// **A source *outside* the repository still lands, at both doors** — the shipped affordance
/// §2 protected by making the rule a source rule.
///
/// A shared team steps library at `~/steps/foo.yaml` is the stated case: the caller names the
/// file, the bytes are **copied in**, and what lands in the repository is a file their own diff
/// shows. This arm is the one that fails if the door is ever re-guarded with
/// `untrackable_reason`, whose first leg refuses *"resolves outside the repository root"*.
#[test]
fn a_readable_source_outside_the_repository_still_lands_at_both_doors() {
    let built = TrialCorpus::build(State::Fresh);

    for door in ["insert-step", "replace-step"] {
        let corpus = built.copy_state();
        // Outside the repository and outside the corpus: the corpus root holds `repo/` and
        // `home/`, so the library lives beside them under the corpus root's own parent-free
        // scratch. `home/` is a real directory outside the repo, which is exactly the shape
        // `~/steps/foo.yaml` has.
        let library = corpus.home().join("steps");
        fs::create_dir_all(&library).expect("create the team steps library");
        let source = library.join("teamstep.yaml");
        fs::write(&source, "Run the shared team step.\n").expect("write the team step");

        let out = drive(
            &corpus,
            door,
            source.to_str().expect("utf-8 team step path"),
        );
        assert!(
            out.status.success(),
            "`config {door}` must still accept a readable out-of-repo source; got {}\
             \n--- stderr ---\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        assert_eq!(
            fs::read_to_string(steps_dir(&corpus.repo()).join("teamstep.yaml"))
                .expect("the native step file is written"),
            "Run the shared team step.\n",
            "`config {door}` must copy the out-of-repo source bytes in verbatim",
        );
    }
}

// ───────────── (c2) the committed cascade layer is a source, not a transient ─────────────

/// **A source inside `.jigc/config/` still lands** — the admitting half of the workbench leg,
/// and the cell that says what *"not reached through the workbench"* is about.
///
/// The leg's subject is jigc's **transient** working area — the subtrees `.jigc/.gitignore`
/// declares, which `jigc uninstall` removes whole and every task rewrites. `.jigc/config/` is
/// none of that: it is the committed project cascade layer, and it is exactly where these two
/// verbs **write** their shadows, so a source already sitting there is an ordinary one.
///
/// This is not a hypothetical. §2's own argument against the *destination* predicate is that
/// `is_workbench_root` *"refuses any path under `.jigc` — precisely where `insert-step`
/// writes"*, and a shipped e2e scenario has always sourced its step from
/// `.jigc/config/extra.yaml`. A first cut of this rule that refused all of `.jigc/` reddened
/// it, which is how the contradiction surfaced; this arm is what keeps the narrowing from
/// silently regressing back.
#[test]
fn a_source_in_the_committed_cascade_layer_is_not_a_transient_and_still_lands() {
    let corpus = TrialCorpus::build(State::Fresh);
    let source = corpus
        .repo()
        .join(".jigc")
        .join("config")
        .join("house.yaml");
    fs::write(&source, "Run the project lint probe before you finalize.\n")
        .expect("write the source step inside the committed cascade layer");

    let out = drive(&corpus, "insert-step", ".jigc/config/house.yaml");
    assert!(
        out.status.success(),
        "a source in the committed cascade layer must still land; got {}\n--- stderr ---\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        fs::read_to_string(steps_dir(&corpus.repo()).join("house.yaml"))
            .expect("the native step file is written"),
        "Run the project lint probe before you finalize.\n",
        "the cascade-layer source bytes must be copied in verbatim",
    );
}

// ───────────────── (d) the `--from-file` family keeps its `-` sentinel ─────────────────

/// **`--from-file -` still reads stdin at all three `from_file` doors** — the regression guard
/// on the family's stated no-rule disposition.
///
/// `from_file` occurs three times (`config fill`, `doc set-slot`, `doc author`) and carries the
/// `-` stdin sentinel, which is **not a path** — the conditional arm the occurrence-keyed
/// registry exists to distinguish. The disposition itself is written at
/// `crate::doc::read_handoff`; this arm holds the half a machine can check, and it is driven
/// through the real binary at each of the three doors rather than at the shared seam, because
/// the seam is not where a future path rule would be bolted on.
#[test]
fn the_stdin_sentinel_still_reads_stdin_at_all_three_from_file_doors() {
    let corpus = TrialCorpus::build(State::Fresh);

    // 1 — `config fill`: the pack `implement` step ships the `{{fill: extra-guidance}}` point.
    corpus.jigc_stdin_ok(
        &[
            "config",
            "fill",
            "step:implement#extra-guidance",
            "--from-file",
            "-",
        ],
        "House rule from stdin.\n",
    );
    assert_eq!(
        fs::read_to_string(
            corpus
                .repo()
                .join(".jigc")
                .join("config")
                .join("fills")
                .join("extra-guidance.md"),
        )
        .expect("the native fill file is written"),
        "House rule from stdin.\n",
        "`config fill --from-file -` must carry the stdin bytes verbatim",
    );

    // 2 — `doc set-slot`, on the commit doc every task provisions.
    let task = corpus.start_workflow("record-decision", "settle the cache strategy");
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        "settle the cache strategy from stdin",
    );
    let staged = corpus.jigc_ok(&["doc", "show", &format!("commit:{task}"), "--task", &task]);
    assert!(
        staged.contains("settle the cache strategy from stdin"),
        "`doc set-slot --from-file -` must splice the stdin bytes; got:\n{staged}",
    );

    // 3 — `doc author`, the batch payload door.
    corpus.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD,
    );
    let authored = corpus.jigc_ok(&["doc", "list", "--task", &task]);
    assert!(
        authored.contains("cache-strategy"),
        "`doc author --from-file -` must author the payload piped on stdin; got:\n{authored}",
    );
}

// ───────────────── (e) the discriminator the family's disposition rests on ─────────────────

/// **`-` is not stdin at the `file` doors** — the driven fact that makes the split between
/// `file`'s source rule and `from_file`'s stated no-rule a rule rather than a preference.
///
/// The `from_file` family takes **no** path rule, and reason 1 of the disposition
/// (`crate::doc::read_handoff`) is that a rule there would refuse a *spelling* while the
/// door's own declared `-` hands the identical bytes through the pipe. That reason only holds
/// if the `file` doors have no such channel — so it is asserted here rather than assumed: `-`
/// is read as a filename and fails, which is what makes the path token those doors' **only**
/// channel and a refusal there a refusal of the outcome.
#[test]
fn the_stdin_sentinel_is_not_a_channel_at_the_file_doors() {
    let corpus = TrialCorpus::build(State::Fresh);
    let out = corpus.jigc_stdin(
        &[
            "config",
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            "-",
        ],
        "Body piped on stdin.\n",
    );
    assert!(
        !out.status.success(),
        "`-` must not be a stdin channel at `config insert-step`; got {}\n--- stdout ---\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
    );
    assert_eq!(
        fs::read_dir(steps_dir(&corpus.repo()))
            .map(|entries| entries.count())
            .unwrap_or(0),
        0,
        "`-` at `config insert-step` must write no native step file",
    );
}
