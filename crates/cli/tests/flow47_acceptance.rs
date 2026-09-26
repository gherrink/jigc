//! **The M47 rc.10-wave done-picture acceptance suite** — the *surface-fundament
//! wave*, driven end-to-end through the **real `jigc` binary**
//! (`design/worked-examples.md` → flow 47; roadmap → Milestone 47, Increment 11).
//!
//! **The claim the wave proves: every surface `jigc` prints about itself is true, and
//! the ones that carry an agent from a cold start are true *first*.** M43 made the
//! surface contract a law; M45 made a fix complete over its class's axis; M47 pays
//! both debts on the surfaces a blind agent actually meets. Increments 1–10 shipped
//! each fix with its own axis suite; this suite is the **composite acceptance** that
//! ties the wave into six done-picture arms over the real binary — **each arm
//! enumerating its axis from a registry or from the class's defining case-set**,
//! never pinning the single repro the trial reported.
//!
//! The six arms, each a `#[test]` over the real binary:
//!
//!   (1) **A rejecting `pre-commit` hook leaves the repo recoverable at every
//!       committing door** — the axis is the code-side
//!       [`cli::invocation_log::COMMITTING_DOORS`] table (11 doors since F-10
//!       joined `jigc task discard` to it): each door exits
//!       non-zero, leaves `HEAD` where it found it, logs **its own** error code (all
//!       eleven pairwise distinct), and the re-run it printed — **lifted verbatim out of
//!       its own frame** — lands at exit 0 once the hook is gone (Inc 2 + 3).
//!
//!   (2) **`jigc task validate` previews the gate rows `finalize` enforces, and scopes
//!       what it does not** — the axis is *the previewed blocking row set itself*, read
//!       off the emitted `--format json`: every row is then met at the committing door,
//!       peeled one at a time until the finalize lands, and the one cause the preview
//!       cannot answer (a *staging-dependent* untracked owner artifact) is shown live —
//!       validate exit 0, finalize exit 3 — with the composed `what's-left:` line
//!       scoping the claim (Inc 4).
//!
//!   (3) **Every write miss routes somewhere that answers** — the write-verb ×
//!       miss-shape matrix, whose verb column is **bijected against the clap tree's
//!       `jigc doc` leaves** so a new write verb reddens this arm: each cell blocks
//!       non-zero with its declared code, persists nothing, and its emitted route argv
//!       is **run verbatim** and must exit 0 (Inc 6).
//!
//!   (4) **`--format json` is one document on one stream** — the axis is every leaf
//!       verb of the clap tree: fixture-free, no leaf verb may split a JSON document
//!       across both streams or mix one with plain text; and on the **success** side
//!       the composite walk's verbs are driven in the stream-hostile
//!       [`State::ChattyHooks`] corpus, where git folds a foreign hook's stdout onto
//!       its own stderr (Inc 7).
//!
//!   (5) **A pack that drops a named contract fact is refused at load** — the axis is
//!       the shipped tree's own (declaring step × declared code × required token)
//!       triples, enumerated from the pack's steps and
//!       [`cli::pack::CONSTRAINT_REQUIRED_TOKENS`]: each token deleted in turn must
//!       block the real binary at pack-load naming step, code and token (Inc 9).
//!
//!   (6) **The cold-start surfaces tell the truth** — `.jigc/AGENT.md`'s store-sweep
//!       clause is checked **against the binary it describes** over a real
//!       above-current-stamp corpus, `jigc setup` names the hooks dir **git resolved**
//!       (not the `.git/hooks` literal it used to assume), and a second `doc author`
//!       lands clean while **no composed workflow of either pack** — enumerated from
//!       the loaded registries — claims otherwise (Inc 10 + Inc 11 T1).
//!
//! **The declared proof split.** Each arm proves the wave's claim at the *done-picture*
//! altitude; the per-fix mechanism clauses stay with the dedicated axis suites and are
//! not re-proven here: the rejection frame's unwrapped `git commit` line, its
//! state-truth sentence and its shell-safe quoting over author-owned prose are
//! `commit_rejected_axis.rs`'s; the owner-artifact cause-by-cause byte-identity
//! between the two doors is `owner_artifact_cause_axis.rs`'s; the full 44-leaf-verb
//! **success** sweep is
//! `format_json_success_axis.rs`'s and the fixture-free reject sweep
//! `machine_output.rs`'s; the methodology-pack seam of the named-fact fence is
//! `stated_at_fence.rs`'s.
//!
//! **Red on `1.0.0-rc.9`**, arm by arm: eight of nine doors printed a bare error with
//! no recoverable frame and logged the *task* door's code; `task validate` exited 0
//! over a plant `finalize` refuses at 3, under five surfaces promising it previewed the
//! whole gate; four matrix cells called an item-id miss a shape question and routed at
//! a schema read that cannot answer it; `task diff` dropped `--format` and printed
//! plain text at exit 0; a step could keep its `states-constraints:` code while its
//! prose lost the fact; and the preload tier stated an unqualified exit-0 the binary's
//! own trailer contradicted while `setup` printed a hooks path that did not exist.
//!
//! Isolation rides the shared [`support::trial_corpus`] substrate (process-unique
//! tempdir, per-repo git identity, `$HOME` repointed, `JIGC_PACK_DIR` scrubbed), so
//! every arm composes the **embedded** `[dev ▸ methodology]` pack-set that ships.

use crate::support;

use clap::CommandFactory;
use cli::cli::Cli;
use cli::invocation_log::COMMITTING_DOORS;
use cli::pack::{CONSTRAINT_REQUIRED_TOKENS, EmbeddedPack};
use engine::packsource::{PackResourceKind, PackSource};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use support::trial_corpus::{State, TrialCorpus};

// ═════════════════════════════════════════════════════════════════════════════
// Shared helpers
// ═════════════════════════════════════════════════════════════════════════════

/// The distinctive bytes the rejecting hook speaks — an appearance in an output
/// stream can only have come from the hook.
const HOOK_MARKER: &str = "policy: FLOW47-REJECTS-EVERY-COMMIT";

/// `EXIT_VALIDATION_BLOCKED` — the task-scope blocking-report exit
/// (`design/command-output-contract.md` → the exit-code table).
const EXIT_BLOCKED: i32 = 3;

/// A throwaway directory that removes itself on drop — for the two arms that need a
/// tree outside a [`TrialCorpus`] (a mutated pack copy, a `core.hooksPath` repo).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow47-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Recursively copy `src` into `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create the copy target");
    for entry in fs::read_dir(src).expect("read the source dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// Stdout of an invocation as UTF-8.
fn stdout_of(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

/// Stderr of an invocation as UTF-8.
fn stderr_of(out: &Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// Whether `s` parses as **exactly one** JSON document — the pinnable form of "this
/// stream carries the document" and, negated, of "this stream carries no JSON at
/// all" (`machine_output.rs`'s predicate, kept local).
fn is_one_json_doc(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}

/// Parse a `--format json` findings envelope off `stdout`, or off `stderr` when the
/// outcome is a reject (stdout empty) — the contract's own discrimination predicate.
fn envelope(out: &Output, what: &str) -> Value {
    let stdout = stdout_of(out);
    let body = if stdout.trim().is_empty() {
        stderr_of(out)
    } else {
        stdout
    };
    serde_json::from_str(&body).unwrap_or_else(|err| {
        panic!(
            "{what} must emit exactly one JSON document ({err}); stdout:\n{}\nstderr:\n{}",
            stdout_of(out),
            stderr_of(out),
        )
    })
}

/// The envelope's findings as a vector (empty when the key is absent).
fn findings(envelope: &Value) -> Vec<Value> {
    envelope["findings"].as_array().cloned().unwrap_or_default()
}

/// The stable `(code, target)` key of one finding, rendered for comparison — the
/// wave's own discriminating finding key (`design/command-output-contract.md`).
fn finding_key(finding: &Value) -> String {
    format!(
        "{}@{}",
        finding["key"]["code"].as_str().unwrap_or("<no code>"),
        finding["key"]["target"].as_str().unwrap_or("<no target>"),
    )
}

/// The blocking findings' stable keys, sorted — a set comparison that is
/// order-invariant by construction.
fn blocking_keys(out: &Output, what: &str) -> BTreeSet<String> {
    findings(&envelope(out, what))
        .iter()
        .filter(|f| f["severity"].as_str() == Some("blocking"))
        .map(finding_key)
        .collect()
}

/// Lift the first backtick-fenced command line **following `after`** out of a printed
/// route or frame, so a route/re-run is executed as the emitted bytes rather than as a
/// test-side reconstruction of it (a rebuilt command can pass while the printed one is
/// broken). Pass an empty `after` to take the text's first fenced span.
fn lift_command(text: &str, after: &str) -> String {
    let start = text
        .find(after)
        .unwrap_or_else(|| panic!("expected `{after}` in:\n{text}"))
        + after.len();
    let rest = &text[start..];
    let open = rest.find('`').unwrap_or_else(|| {
        panic!("expected a backtick-fenced command after `{after}` in:\n{text}")
    });
    let body = &rest[open + 1..];
    let end = body
        .find('`')
        .unwrap_or_else(|| panic!("the fenced command must be closed in:\n{text}"));
    body[..end].to_string()
}

/// Split a printed command line into argv. Every fixture in this suite feeds
/// single-token titles and intents, so no emitted line is quoted; a stray quote pair
/// is stripped defensively rather than silently executed as part of a token. The
/// author-owned-prose quoting axis is `commit_rejected_axis.rs`'s.
fn argv_of(line: &str) -> Vec<String> {
    line.split_whitespace()
        .map(|token| token.trim_matches(|c| c == '\'' || c == '"').to_string())
        .collect()
}

/// Turn a printed `jigc …` command line into the argv to drive, asserting it really
/// is a runnable `jigc` invocation.
fn jigc_argv(line: &str) -> Vec<String> {
    let argv = argv_of(line);
    assert_eq!(
        argv.first().map(String::as_str),
        Some("jigc"),
        "a printed route must be a runnable `jigc …` command; got `{line}`",
    );
    argv[1..].to_vec()
}

/// Run a `jigc` argv (already stripped of the leading `jigc`) against a corpus.
fn run_argv(corpus: &TrialCorpus, argv: &[String]) -> Output {
    let borrowed: Vec<&str> = argv.iter().map(String::as_str).collect();
    corpus.jigc(&borrowed)
}

/// Install an executable `pre-commit` hook that **rejects every commit**, speaking
/// [`HOOK_MARKER`] on stderr (replacing the warn-only one `jigc setup` installed).
fn install_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!("#!/bin/sh\necho '{HOOK_MARKER}' 1>&2\nexit 1\n"),
    )
    .expect("write the rejecting pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// The operator's repair between the refused run and the recovery re-run.
fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// Write the project scalar layer: the invocation-log knob on (the door's error
/// identity is read back from the log), plus an optional commit-model knob.
fn project_scalars(corpus: &TrialCorpus, squash: Option<&str>) {
    let mut manifest = String::from("scalar:\n  invocation-log: true\n");
    if let Some(value) = squash {
        manifest.push_str(&format!("  finalize.fan-out.squash: {value}\n"));
    }
    let dir = corpus.repo().join(".jigc").join("config");
    fs::create_dir_all(&dir).expect("mk the project config dir");
    fs::write(dir.join("manifest.yaml"), manifest).expect("write the project scalar layer");
}

/// The parsed JSONL invocation-log records.
fn log_records(repo: &Path) -> Vec<Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// The **last** logged record whose `argv` is exactly `argv` — the door's own run,
/// never an earlier setup call that happens to share a token.
fn record_for<'a>(records: &'a [Value], argv: &[String]) -> Option<&'a Value> {
    records.iter().rev().find(|record| {
        record["argv"].as_array().is_some_and(|logged| {
            logged.len() == argv.len()
                && logged
                    .iter()
                    .zip(argv)
                    .all(|(value, want)| value.as_str() == Some(want.as_str()))
        })
    })
}

/// A conformant `adr` body, optionally stamped (`None` = the unstamped v0 state
/// `migrate-corpus` lifts).
fn adr_body(title: &str, stamp: Option<u32>) -> String {
    let stamp_line = match stamp {
        Some(version) => format!("schema-version: {version}\n"),
        None => String::new(),
    };
    format!(
        "---\nstatus: accepted\ndate: 2026-07-26\n{stamp_line}---\n\n# {title}\n\n## Context\n\n\
         Session lookups must stay sub-millisecond.\n\n## Options\n\nA distributed cache was \
         weighed and rejected on latency.\n\n## Decision\n\nKeep sessions in one node.\n\n\
         ## Consequences\n\nA cold node loses its sessions.\n"
    )
}

/// Write + commit an `adr` at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(corpus: &TrialCorpus, slug: &str, title: &str, stamp: Option<u32>) {
    let dir = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), adr_body(title, stamp)).expect("write the adr");
    corpus.git(&["add", "."]);
    corpus.git(&["commit", "-q", "-m", "seed adr"]);
}

/// A committed 2-criteria `spec` — the `add-from-spec` seed substrate.
const TWO_CRITERIA_SPEC: &str = "\
# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.
";

/// Stage a doc body + its provenance bit into a sub-task's working area — the
/// merged-doc contribution the milestone boundary promotes, so the two
/// `milestone finalize` arms reach their commit phase instead of the
/// zero-contribution refusal.
fn stage_subtask_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk the sub-task docs area");
    fs::write(docs.join(format!("{address}.md")), body).expect("write the staged body");
    let manifest = docs.join("provenance.json");
    let mut record: Value = match fs::read_to_string(&manifest) {
        Ok(body) => serde_json::from_str(&body).expect("the provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize the manifest"),
    )
    .expect("write the provenance manifest");
}

/// Fill a task's `commit` doc so a finalize renders a clean git message.
fn fill_commit(corpus: &TrialCorpus, task: &str, scope: &str) {
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "feat",
        "--task",
        task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("commit:{task}#scope"),
        "--value",
        scope,
        "--task",
        task,
    ]);
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            task,
        ],
        "survive the rejection\n",
    );
    corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#body"),
            "--from-file",
            "-",
            "--task",
            task,
        ],
        "Driven by the flow-47 acceptance suite.\n",
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 1 — the committing-door axis under a rejecting `pre-commit` hook
// ═════════════════════════════════════════════════════════════════════════════

/// One door's driven rejection: the corpus it ran in and the argv it was driven with.
struct DoorCase {
    corpus: TrialCorpus,
    driven: Vec<String>,
}

fn owned(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| (*arg).to_string()).collect()
}

/// Build one door's fixture off a copy of the `Fresh` base, install the rejecting
/// hook, and return the argv that drives the door.
fn door_case(base: &TrialCorpus, verb: &str) -> DoorCase {
    let corpus = base.copy_state();
    let driven = match verb {
        "jigc task finalize" => {
            project_scalars(&corpus, None);
            let task = corpus.start_workflow("single-task", "harden-the-cache");
            fill_commit(&corpus, &task, "cache");
            fs::write(corpus.repo().join("code.txt"), "the task's work\n").expect("write code.txt");
            corpus.git(&["add", "code.txt"]);
            owned(&["task", "finalize", &task])
        }
        // The amend arm (F-10): the same leaf, minted through `jigc task amend` so the task
        // carries the marker that selects `git commit --amend`. Nothing is staged — the arm
        // refuses over a non-empty index, so a `git add` here would reach the dirty-index
        // gate rather than the hook this flow plants.
        "jigc task finalize (amend)" => {
            project_scalars(&corpus, None);
            corpus.jigc_ok(&["task", "amend", "repair-the-message"]);
            let task = "repair-the-message";
            fill_commit(&corpus, task, "cache");
            owned(&["task", "finalize", task])
        }
        "jigc milestone finalize (squash: true)" | "jigc milestone finalize (squash: false)" => {
            let squash = if verb.ends_with("true)") {
                "true"
            } else {
                "false"
            };
            project_scalars(&corpus, Some(squash));
            corpus.jigc_ok(&["milestone", "create", "Cache-rework"]);
            corpus.jigc_ok(&["milestone", "add-task", "cache-rework", "Area-low"]);
            stage_subtask_doc(
                &corpus.repo(),
                "area-low",
                "adr:eviction-policy",
                &adr_body("Eviction policy", None),
            );
            owned(&["milestone", "finalize", "cache-rework"])
        }
        "jigc rename" => {
            project_scalars(&corpus, None);
            commit_adr(&corpus, "alpha-decision", "Alpha decision", Some(2));
            owned(&["rename", "adr:alpha-decision", "--to", "Beta-decision"])
        }
        "jigc migrate-corpus" => {
            project_scalars(&corpus, None);
            // An unstamped (v0) committed ADR: the stamp add-field migration writes it
            // back and the commit boundary lands it, so the hook genuinely fires.
            commit_adr(&corpus, "alpha-decision", "Alpha decision", None);
            owned(&["migrate-corpus"])
        }
        "jigc milestone create" => {
            project_scalars(&corpus, None);
            owned(&["milestone", "create", "Cache-rework"])
        }
        "jigc milestone add-task" => {
            project_scalars(&corpus, None);
            corpus.jigc_ok(&["milestone", "create", "Cache-rework"]);
            owned(&["milestone", "add-task", "cache-rework", "Area-low"])
        }
        "jigc milestone add-from-spec" => {
            project_scalars(&corpus, None);
            let specs = corpus.repo().join("docs").join("specs");
            fs::create_dir_all(&specs).expect("mk docs/specs/");
            fs::write(specs.join("rate-limit.md"), TWO_CRITERIA_SPEC).expect("write the spec");
            corpus.git(&["add", "."]);
            corpus.git(&["commit", "-q", "-m", "add spec"]);
            corpus.jigc_ok(&["milestone", "create", "Rate-limit"]);
            owned(&[
                "milestone",
                "add-from-spec",
                "rate-limit",
                "spec:rate-limit",
            ])
        }
        "jigc milestone discard" => {
            project_scalars(&corpus, None);
            corpus.jigc_ok(&["milestone", "create", "Cache-rework"]);
            owned(&["milestone", "discard", "cache-rework"])
        }
        // The sub-task abandon path: its record-only settle commit runs BEFORE the working
        // area is removed, so the rejection leaves both the record and the task intact.
        "jigc task discard" => {
            project_scalars(&corpus, None);
            corpus.jigc_ok(&["milestone", "create", "Cache-rework"]);
            corpus.jigc_ok(&["milestone", "add-task", "cache-rework", "Area-low"]);
            owned(&["task", "discard", "area-low"])
        }
        other => panic!(
            "`{other}` is a committing door with no fixture in this flow — the axis is the \
             code-side `COMMITTING_DOORS` table, so a door added there owes its arm here",
        ),
    };
    install_rejecting_hook(&corpus.repo());
    DoorCase { corpus, driven }
}

/// **Arm 1.** The committing-door axis, rejecting side: a hook that refuses every
/// commit leaves the repo **recoverable** at *every* production committing door, and
/// each door is identifiable in the log by **its own** error code.
///
/// The axis is [`COMMITTING_DOORS`] — the one code-side table `ERROR_CODE_REGISTRY` is
/// derived from — so a door added there joins this sweep by construction and a door
/// with no fixture is a hard panic, never a skip. Per door: the run exits non-zero,
/// `HEAD` is exactly where it was, the hook's own complaint reaches the operator, the
/// invocation log carries that door's identity (and the identities are pairwise
/// distinct), and the re-run **lifted verbatim out of the frame the door printed**
/// exits 0 once the hook is removed — so "a re-run after any rejection recovers" is
/// executed, not merely printed.
///
/// The frame's remaining *shape* clauses (git's own rejection line unwrapped, the
/// state-truth sentence, the shell-safe quoting over author-owned prose) are
/// `commit_rejected_axis.rs`'s.
/// Red on rc.9: every committing door but `jigc task finalize` **[Corrected 2026-09-15
/// (M51 Increment 7, T3):** *eight of the nine doors* — the axis as rc.9 shipped it; it is
/// **ten** since M49 Increment 2 T3, and the red-state figure is kept rather than
/// re-pinned.**]** printed a bare error with no recoverability
/// statement and no route, and the two `milestone finalize` arms logged the *task*
/// door's `finalize.commit-rejected` — a lying code on the surface built to stop the
/// log lying.
#[test]
fn every_committing_door_leaves_the_repo_recoverable() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        11,
        "the axis is the code-side committing-door table (`jigc setup`'s install commit \
         is excluded by its recorded `--no-verify` reason)",
    );
    let codes: BTreeSet<&str> = COMMITTING_DOORS
        .iter()
        .map(|door| door.error_code)
        .collect();
    assert_eq!(
        codes.len(),
        COMMITTING_DOORS.len(),
        "one error code per door — a shared identity makes a rejected run unattributable \
         in the log",
    );

    let base = TrialCorpus::build(State::Fresh);
    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = door_case(&base, verb);
        let repo = case.corpus.repo();
        let driven: Vec<&str> = case.driven.iter().map(String::as_str).collect();

        let head_before = case.corpus.git(&["rev-parse", "HEAD"]);
        let rejected = case.corpus.jigc(&driven);
        let stderr = stderr_of(&rejected);

        assert!(
            !rejected.status.success(),
            "[{verb}] a rejected commit must exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
            stdout_of(&rejected),
        );
        assert_eq!(
            case.corpus.git(&["rev-parse", "HEAD"]),
            head_before,
            "[{verb}] a rejected commit must leave HEAD untouched",
        );
        // Recoverable means the operator can act: the hook's own complaint — the one
        // signal naming *what* to fix — reaches them unedited.
        assert!(
            stderr.contains(HOOK_MARKER),
            "[{verb}] the hook's complaint must reach the operator; stderr:\n{stderr}",
        );

        // The log names THIS door — read off the record, so the release build's
        // compiled-out `debug_assert!` cannot hide a wrong or missing identity.
        let records = log_records(&repo);
        let record = record_for(&records, &case.driven).unwrap_or_else(|| {
            panic!("[{verb}] the rejected run must be logged; records:\n{records:#?}")
        });
        assert_eq!(
            record["error_code"].as_str(),
            Some(door.error_code),
            "[{verb}] the log must carry THIS door's identity; got {record}",
        );

        // The recovery claim is executed, not printed: the argv comes out of the
        // emitted frame, and it must land once the operator's repair is made.
        let printed = lift_command(&stderr, "then re-run ");
        let argv = jigc_argv(&printed);
        remove_hook(&repo);
        let recovered = run_argv(&case.corpus, &argv);
        assert!(
            recovered.status.success(),
            "[{verb}] the printed re-run `{printed}` must recover; stdout:\n{}\nstderr:\n{}",
            stdout_of(&recovered),
            stderr_of(&recovered),
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 2 — `task validate` previews the gate rows `finalize` enforces
// ═════════════════════════════════════════════════════════════════════════════

/// The multi-gate fixture: the corpus it stands in, the task under the gate, and the
/// `completion-record` address the binary itself acked (never a test-side rebuild of
/// the slug rule).
struct GateFixture {
    corpus: TrialCorpus,
    task: String,
    record: String,
}

/// A task standing on **two** previewable gate rows at once: a pre-mint staged file
/// (the carryover gate) and an `owner-artifact` pointing outside the owned artifact
/// home (a staging-independent `owned_location_violation` cause). Everything else is
/// authored clean, so the preview's row set is exactly the gate rows under test.
fn gate_fixture() -> GateFixture {
    let corpus = TrialCorpus::build(State::Fresh);
    // The carryover plant: staged BEFORE the task exists.
    fs::write(corpus.repo().join("plant.txt"), "pre-staged\n").expect("write the plant");
    corpus.git(&["add", "plant.txt"]);

    let task = corpus.start_workflow("completion", "close-the-milestone");
    let record = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "completion-record",
            "--title",
            "M47 completion",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("{record}#meta/verdict"),
        "--value",
        "green",
        "--task",
        &task,
    ]);
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("{record}#meta/owner-artifact"),
        "--value",
        "completions/nope.md",
        "--task",
        &task,
    ]);
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{record}#findings"),
            "--title",
            "A finding",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    for (leaf, value) in [
        ("severity", "advisory"),
        ("disposition", "fixed"),
        ("evidence", "the gate suite is green"),
    ] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("{item}/{leaf}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    fill_commit(&corpus, &task, "m47");
    GateFixture {
        corpus,
        task,
        record,
    }
}

/// Repair the cause of one previewed row, dispatched on its **code** — an unknown
/// code is a hard panic, so a row that joins the preview owes its repair here rather
/// than silently shrinking the peel.
fn repair(fixture: &GateFixture, code: &str) {
    match code {
        "finalize.carried-staged" => {
            fixture.corpus.git(&["restore", "--staged", "plant.txt"]);
        }
        "owner-artifact.present" => {
            let artifact = "completions/artifacts/m47/audit.md";
            let path = fixture.corpus.repo().join(artifact);
            fs::create_dir_all(path.parent().expect("artifact dir")).expect("mk artifact dir");
            fs::write(&path, "the audit artifact\n").expect("write the artifact");
            fixture.corpus.git(&["add", artifact]);
            fixture.corpus.jigc_ok(&[
                "doc",
                "set-field",
                &format!("{}#meta/owner-artifact", fixture.record),
                "--value",
                artifact,
                "--task",
                &fixture.task,
            ]);
        }
        other => panic!(
            "`{other}` is previewed by `jigc task validate` but this arm has no repair for it — \
             a row that joins the preview owes its peel step here",
        ),
    }
}

/// **Arm 2.** `jigc task validate` previews the gate rows `jigc task finalize`
/// enforces, and **scopes what it does not**.
///
/// The axis is not a hand list of gates: it is **the previewed blocking row set
/// itself**, read off the emitted `--format json`. Each previewed row is then met at
/// the committing door — the finalize is run, its blocking rows must be a subset of
/// the preview's, and the row is repaired — until the preview is clean and the
/// finalize **lands**. Every previewed row must have been observed blocking a real
/// finalize, so a preview row that no commit door enforces fails here.
///
/// The scoping half is proven live, not read: the one cause the preview *cannot*
/// answer — an owner artifact present but untracked, whose verdict a later stage
/// phase can change — leaves `task validate` at exit 0 while `task finalize` blocks
/// at exit 3, and the composed `what's-left:` line states that scope rather than
/// promising the whole gate. Red on rc.9: `task validate` exited 0 over a plant
/// finalize refuses at 3, under five surfaces claiming it previewed the gate.
#[test]
fn task_validate_previews_every_gate_row_the_commit_door_enforces() {
    let fixture = gate_fixture();
    let corpus = &fixture.corpus;
    let task = fixture.task.as_str();

    let previewed = blocking_keys(
        &corpus.jigc(&["task", "validate", task, "--format", "json"]),
        "`jigc task validate --format json`",
    );
    assert!(
        previewed.len() >= 2,
        "the fixture must stand on at least the carryover and owner-artifact rows; got {previewed:?}",
    );
    for family in ["finalize.carried-staged@", "owner-artifact.present@"] {
        assert!(
            previewed.iter().any(|key| key.starts_with(family)),
            "the preview must carry a `{family}` row; got {previewed:?}",
        );
    }

    // Peel: every finalize refusal's rows come out of the previewed set, and every
    // previewed row is met at the commit door before the finalize lands.
    let mut met: BTreeSet<String> = BTreeSet::new();
    let mut rounds = 0;
    loop {
        let attempt = corpus.jigc(&["task", "finalize", task, "--format", "json"]);
        if attempt.status.success() {
            break;
        }
        assert_eq!(
            attempt.status.code(),
            Some(EXIT_BLOCKED),
            "a gated finalize must exit {EXIT_BLOCKED}; stdout:\n{}\nstderr:\n{}",
            stdout_of(&attempt),
            stderr_of(&attempt),
        );
        let blocked = blocking_keys(&attempt, "`jigc task finalize --format json`");
        assert!(
            blocked.is_subset(&previewed),
            "the commit door blocked on rows the preview never showed: {:?} ⊄ {previewed:?}",
            blocked,
        );
        for key in &blocked {
            let code = key.split('@').next().expect("a keyed finding").to_string();
            repair(&fixture, &code);
            met.insert(key.clone());
        }
        rounds += 1;
        assert!(
            rounds <= previewed.len() + 1,
            "the peel must converge — {rounds} refusals over {} previewed rows",
            previewed.len(),
        );
    }
    assert_eq!(
        met, previewed,
        "every previewed blocking row must have been enforced at the commit door",
    );

    // The scoping half — the one cause the preview cannot answer, live.
    let scoped = TrialCorpus::build(State::Fresh);
    let untracked = untracked_owner_artifact(&scoped);
    let preview = scoped.jigc(&["task", "validate", &untracked, "--format", "json"]);
    assert!(
        preview.status.success(),
        "the untracked cause is staging-dependent and must NOT be previewed; stdout:\n{}\nstderr:\n{}",
        stdout_of(&preview),
        stderr_of(&preview),
    );
    let committing = scoped.jigc(&["task", "finalize", &untracked, "--format", "json"]);
    assert_eq!(
        committing.status.code(),
        Some(EXIT_BLOCKED),
        "the committing door must still block the untracked owner artifact; stdout:\n{}\nstderr:\n{}",
        stdout_of(&committing),
        stderr_of(&committing),
    );
    assert!(
        blocking_keys(&committing, "`jigc task finalize --format json`")
            .iter()
            .any(|key| key.starts_with("owner-artifact.present@")),
        "the committing door must name the owner-artifact gate it blocked on",
    );

    // …and the composed surface states that scope instead of promising the gate.
    let composed = scoped.jigc_ok(&["start", "--task", &untracked]);
    let line = composed
        .lines()
        .find(|line| line.starts_with("what's-left:"))
        .unwrap_or_else(|| panic!("the compose must carry a what's-left line; got:\n{composed}"));
    assert!(
        line.contains("previews part of the finalize gate"),
        "the composed line must scope its claim; got:\n{line}",
    );
    for covered in ["content findings", "carryover", "owner-artifact"] {
        assert!(
            line.contains(covered),
            "the composed line must name `{covered}` as covered; got:\n{line}",
        );
    }
    assert!(
        line.contains("the staged set, promotion and the commit surface at finalize"),
        "the composed line must say where the un-previewed rest surfaces; got:\n{line}",
    );
}

/// A task whose `completion-record` names an artifact that **exists but is
/// gitignored** — the one `owned_location_violation` cause whose verdict a later
/// stage phase can change, and therefore the one the preview refuses to forecast.
/// Returns the task id.
fn untracked_owner_artifact(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("completion", "close-the-second-milestone");
    let record = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "completion-record",
            "--title",
            "M46 completion",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("{record}#meta/verdict"),
        "--value",
        "green",
        "--task",
        &task,
    ]);
    let artifact = "completions/artifacts/m46/audit.md";
    let path = corpus.repo().join(artifact);
    fs::create_dir_all(path.parent().expect("artifact dir")).expect("mk the artifact dir");
    fs::write(&path, "the audit artifact\n").expect("write the artifact");
    fs::write(corpus.repo().join(".gitignore"), "completions/artifacts/\n")
        .expect("write .gitignore");
    corpus.jigc_ok(&[
        "doc",
        "set-field",
        &format!("{record}#meta/owner-artifact"),
        "--value",
        artifact,
        "--task",
        &task,
    ]);
    let item = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{record}#findings"),
            "--title",
            "A finding",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    for (leaf, value) in [
        ("severity", "advisory"),
        ("disposition", "fixed"),
        ("evidence", "the gate suite is green"),
    ] {
        corpus.jigc_ok(&[
            "doc",
            "set-field",
            &format!("{item}/{leaf}"),
            "--value",
            value,
            "--task",
            &task,
        ]);
    }
    fill_commit(corpus, &task, "m46");
    task
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 3 — the write-verb × miss-shape matrix, every route run verbatim
// ═════════════════════════════════════════════════════════════════════════════

/// One cell of the write-verb × miss-shape matrix.
struct Cell {
    /// The `jigc doc` leaf verb the cell exercises — the column bijected against the
    /// clap tree.
    verb: &'static str,
    /// What the cell is, for the assertion messages.
    what: &'static str,
    /// The argv after `jigc`, with `{doc}` standing for the created doc address.
    args: &'static [&'static str],
    /// Optional stdin payload (the `--from-file -` cells).
    stdin: Option<&'static str>,
    /// The finding `code` the reject must carry.
    code: &'static str,
    /// A substring the emitted route's **verbatim run** must reveal: the section's
    /// real item id for an item-id miss, a declared section name for a shape miss.
    reveals: &'static str,
}

/// **The matrix.** Every item-addressing `jigc doc` write verb × the two miss shapes
/// an agent actually produces: an item id that was never minted (`write.not-present`
/// — a corpus question the schema cannot answer) and a section the doctype never
/// declared (`write.unknown-section` — a shape question `doc schema` answers).
const CELLS: &[Cell] = &[
    Cell {
        verb: "set-slot",
        what: "set-slot at a nonexistent item",
        args: &[
            "doc",
            "set-slot",
            "{doc}#releases/9-9-9/changes/added/notes",
            "--from-file",
            "-",
        ],
        stdin: Some("A note.\n"),
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "remove-item",
        what: "remove-item at a nonexistent item",
        args: &["doc", "remove-item", "{doc}#releases/9-9-9"],
        stdin: None,
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "set-field",
        what: "set-field --value at a nonexistent item",
        args: &[
            "doc",
            "set-field",
            "{doc}#releases/9-9-9/link",
            "--value",
            "https://example.test",
        ],
        stdin: None,
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "set-field",
        what: "set-field --unset at a nonexistent item",
        args: &["doc", "set-field", "{doc}#releases/9-9-9/link", "--unset"],
        stdin: None,
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "retitle-item",
        what: "retitle-item at a nonexistent item",
        args: &[
            "doc",
            "retitle-item",
            "{doc}#releases/9-9-9",
            "--title",
            "1.4.0",
        ],
        stdin: None,
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "add-item",
        what: "nested add-item under an absent parent item",
        args: &[
            "doc",
            "add-item",
            "{doc}#releases/9-9-9/changes",
            "--title",
            "Added",
        ],
        stdin: None,
        code: "write.not-present",
        reveals: "1-3-0",
    },
    Cell {
        verb: "remove-item",
        what: "remove-item in an undeclared section",
        args: &["doc", "remove-item", "{doc}#nope/x"],
        stdin: None,
        code: "write.unknown-section",
        reveals: "releases",
    },
    Cell {
        verb: "add-item",
        what: "add-item in an undeclared section",
        args: &["doc", "add-item", "{doc}#nope", "--title", "Z"],
        stdin: None,
        code: "write.unknown-section",
        reveals: "releases",
    },
    Cell {
        verb: "set-field",
        what: "set-field --unset in an undeclared section",
        args: &["doc", "set-field", "{doc}#nope/x/link", "--unset"],
        stdin: None,
        code: "write.unknown-section",
        reveals: "releases",
    },
    Cell {
        verb: "set-slot",
        what: "set-slot in an undeclared section",
        args: &["doc", "set-slot", "{doc}#nope/x/notes", "--from-file", "-"],
        stdin: Some("A note.\n"),
        code: "write.unknown-section",
        reveals: "releases",
    },
    Cell {
        verb: "retitle-item",
        what: "retitle-item in an undeclared section",
        args: &["doc", "retitle-item", "{doc}#nope/x", "--title", "Z"],
        stdin: None,
        code: "write.unknown-section",
        reveals: "releases",
    },
];

/// The `jigc doc` leaf verbs that address **no** item and therefore have no cell in
/// the matrix — declared, so a *new* `doc` verb reddens the bijection below rather
/// than joining this set silently.
///
/// `rename` (M48) joins the declaration rather than the matrix: it addresses a **doc**
/// and refuses a `#fragment` outright, so it has no item-miss to make. Its own axis —
/// the doctype census it splits on — is swept in `crates/cli/tests/doc_rename_in_task.rs`.
const NON_ITEM_ADDRESSING_DOC_VERBS: &[&str] =
    &["create", "author", "show", "schema", "list", "rename"];

/// Every leaf verb under `jigc doc`, walked from the clap `Command` tree.
fn doc_leaf_verbs() -> BTreeSet<String> {
    Cli::command()
        .get_subcommands()
        .find(|sub| sub.get_name() == "doc")
        .expect("the clap tree carries the `doc` verb")
        .get_subcommands()
        .filter(|sub| sub.get_name() != "help")
        .map(|sub| sub.get_name().to_string())
        .collect()
}

/// **Arm 3.** Every write miss routes somewhere that **answers**, and the answer is
/// executed rather than admired.
///
/// The matrix is verb × miss-shape, and its **verb column is bijected against the
/// clap tree's `jigc doc` leaves** — every leaf is either exercised by a cell or
/// declared item-addressing-free, so a write verb added to the surface reddens this
/// arm until it has a row. Per cell, through the real binary: the write blocks
/// non-zero with its declared code, the staged bytes are **byte-identical** across the
/// reject (a refused write persists nothing), and the emitted `route` is **lifted out
/// of the JSON finding and run verbatim** — it must exit 0 and reveal what the agent
/// was missing (the section's live item ids for a corpus miss, the declared sections
/// for a shape miss). Red on rc.9: four cells called an item-id miss a shape question
/// and routed at `doc schema`, which names the shape and never the corpus's real ids —
/// a dead end.
#[test]
fn every_write_miss_routes_somewhere_that_answers() {
    let covered: BTreeSet<String> = CELLS.iter().map(|cell| cell.verb.to_string()).collect();
    let declared: BTreeSet<String> = NON_ITEM_ADDRESSING_DOC_VERBS
        .iter()
        .map(|verb| (*verb).to_string())
        .collect();
    let union: BTreeSet<String> = covered.union(&declared).cloned().collect();
    assert_eq!(
        union,
        doc_leaf_verbs(),
        "every `jigc doc` leaf verb must either carry a matrix row or be declared \
         item-addressing-free — a new write verb owes its cells here",
    );

    let corpus = TrialCorpus::build(State::Fresh);
    let task = corpus.start_workflow("record-change", "cut-a-release");
    let doc = corpus
        .jigc_ok(&[
            "doc",
            "create",
            "changelog",
            "--title",
            "Changelog",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    let release = corpus
        .jigc_ok(&[
            "doc",
            "add-item",
            &format!("{doc}#releases"),
            "--title",
            "1.3.0",
            "--task",
            &task,
        ])
        .trim()
        .to_string();
    corpus.jigc_ok(&[
        "doc",
        "add-item",
        &format!("{release}/changes"),
        "--title",
        "Added",
        "--task",
        &task,
    ]);

    let staged = corpus
        .repo()
        .join(format!(".jigc/tasks/{task}/docs/{doc}.md"));
    assert!(
        staged.is_file(),
        "the staged changelog must exist at {staged:?}",
    );

    for cell in CELLS {
        let before = fs::read(&staged).expect("read the staged doc");
        let mut args: Vec<String> = cell
            .args
            .iter()
            .map(|arg| arg.replace("{doc}", &doc))
            .collect();
        args.extend(owned(&["--task", &task, "--format", "json"]));
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        let out = match cell.stdin {
            Some(payload) => corpus.jigc_stdin(&argv, payload),
            None => corpus.jigc(&argv),
        };

        assert!(
            !out.status.success(),
            "[{}] the write must be refused; stdout:\n{}\nstderr:\n{}",
            cell.what,
            stdout_of(&out),
            stderr_of(&out),
        );
        assert_eq!(
            fs::read(&staged).expect("read the staged doc"),
            before,
            "[{}] a refused write must persist nothing",
            cell.what,
        );

        let found = findings(&envelope(&out, cell.what));
        let finding = found
            .iter()
            .find(|finding| finding["code"].as_str() == Some(cell.code))
            .unwrap_or_else(|| {
                panic!(
                    "[{}] the reject must carry `{}`; got:\n{found:#?}",
                    cell.what, cell.code,
                )
            });
        let route = finding["route"]
            .as_str()
            .unwrap_or_else(|| panic!("[{}] a blocking finding carries a route", cell.what));
        assert!(
            !route.contains("<address>") && !route.contains("<task-id>"),
            "[{}] the route must carry the resolved address and task, never a \
             placeholder; got `{route}`",
            cell.what,
        );

        // The emitted bytes are the contract: run the route as printed.
        let printed = lift_command(route, "");
        let recovery = jigc_argv(&printed);
        let answered = run_argv(&corpus, &recovery);
        assert!(
            answered.status.success(),
            "[{}] the emitted route `{printed}` must exit 0; stdout:\n{}\nstderr:\n{}",
            cell.what,
            stdout_of(&answered),
            stderr_of(&answered),
        );
        assert!(
            stdout_of(&answered).contains(cell.reveals),
            "[{}] the emitted route `{printed}` must reveal `{}`; got:\n{}",
            cell.what,
            cell.reveals,
            stdout_of(&answered),
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 4 — `--format json` is one document, on one stream
// ═════════════════════════════════════════════════════════════════════════════

/// Every **leaf** verb's argv path, walked from the clap `Command` tree — the
/// enumeration seam, so a subcommand added anywhere auto-joins this sweep.
fn leaf_verb_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let mut had_child = false;
        for sub in cmd.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            had_child = true;
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
        if !had_child && !prefix.is_empty() {
            out.push(prefix);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out
}

/// **Arm 4.** Under `--format json` a driver reads **exactly one** JSON document off
/// **one** stream — never a split document, never a document sharing its stream with
/// plain text.
///
/// Two halves, both over the real binary. The **whole-verb axis** is walked from the
/// clap tree and driven fixture-free (a bare non-git, non-project directory, where no
/// verb can succeed): each leaf verb must leave stdout empty with exactly one document
/// on stderr, or take the declared clap carve-out — a usage error clap formats itself
/// before any jigc code reads `--format`, plain text on both streams. The **success**
/// half runs the composite walk's verbs inside [`State::ChattyHooks`], a corpus
/// carrying a foreign `pre-commit` hook that speaks on stdout, which git folds onto
/// its own stderr: the landed finalize must still put exactly one document on stdout
/// and keep the hook relay off it. Red on rc.9: `task diff` dropped `--format` on the
/// floor and printed plain text at exit 0 — zero bytes of JSON on either stream.
///
/// The exhaustive per-verb **success** sweep (all 44 leaves, each driven to a genuine
/// success) is `format_json_success_axis.rs`'s; this arm proves the composite
/// done-picture and the whole-axis stream discipline.
#[test]
fn every_leaf_verb_speaks_exactly_one_json_document() {
    let leaves = leaf_verb_paths();
    assert!(
        leaves.len() >= 44,
        "the clap tree must enumerate the whole verb surface; got {} leaves",
        leaves.len(),
    );

    let bare = TempDir::new("bare");
    let home = TempDir::new("bare-home");
    for path in &leaves {
        let mut args: Vec<&str> = path.iter().map(String::as_str).collect();
        args.extend(["--format", "json"]);
        let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(&args)
            .current_dir(bare.path())
            .env("HOME", home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn jigc");
        let verb = path.join(" ");
        assert!(
            !out.status.success(),
            "[{verb}] no verb can succeed in a bare directory; stdout:\n{}",
            stdout_of(&out),
        );
        let stdout = stdout_of(&out);
        let stderr = stderr_of(&out);
        if stdout.trim().is_empty() && stderr.trim().is_empty() {
            panic!("[{verb}] a refusal must say something on some stream");
        }
        if is_one_json_doc(&stderr) {
            // The reject envelope rides stderr alone.
            assert!(
                stdout.trim().is_empty(),
                "[{verb}] a reject must leave stdout empty; got:\n{stdout}",
            );
        } else {
            // The declared clap carve-out: a usage error, plain text, no envelope.
            assert!(
                !is_one_json_doc(&stdout),
                "[{verb}] a JSON document must not ride stdout while stderr carries \
                 plain text; stdout:\n{stdout}\nstderr:\n{stderr}",
            );
        }
    }

    // The success half, in the stream-hostile corpus.
    let corpus = TrialCorpus::build(State::ChattyHooks);
    let task = corpus.start_workflow("single-task", "tune-the-cache");
    fill_commit(&corpus, &task, "cache");
    fs::write(corpus.repo().join("cache.txt"), "the task's work\n").expect("write cache.txt");
    corpus.git(&["add", "cache.txt"]);
    let mut driven: Vec<Vec<String>> = vec![
        owned(&["workflow", "single-task", "--preview"]),
        owned(&["doc", "list"]),
        owned(&["doc", "schema", "adr"]),
        owned(&["task", "list"]),
        owned(&["task", "diff", &task]),
        owned(&["task", "validate", &task]),
        owned(&["validate"]),
    ];
    // The landed finalize last — it is the invocation the foreign hook actually
    // speaks over, and it ends the task.
    driven.push(owned(&["task", "finalize", &task]));

    for argv in &driven {
        let mut args: Vec<&str> = argv.iter().map(String::as_str).collect();
        args.extend(["--format", "json"]);
        let out = corpus.jigc(&args);
        let verb = argv.join(" ");
        assert!(
            out.status.success(),
            "[{verb}] must succeed in the chatty-hook corpus; stdout:\n{}\nstderr:\n{}",
            stdout_of(&out),
            stderr_of(&out),
        );
        assert!(
            is_one_json_doc(&stdout_of(&out)),
            "[{verb}] stdout must parse as exactly one JSON document; got:\n{}",
            stdout_of(&out),
        );
        assert!(
            !is_one_json_doc(&stderr_of(&out)),
            "[{verb}] stderr must carry no JSON document; got:\n{}",
            stderr_of(&out),
        );
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 5 — a pack that drops a named contract fact is refused at load
// ═════════════════════════════════════════════════════════════════════════════

/// The on-disk dev pack the binary embeds.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The named-fact comparison view: whitespace collapsed, ASCII case folded — the
/// pack-load fence's own normalization, since step prose is hard-wrapped and
/// sentence-cased.
fn normalized(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

/// Split a step file into its front-matter (through the closing `---`) and its body.
fn split_body(text: &str) -> (&str, &str) {
    let rest = text
        .strip_prefix("---\n")
        .expect("a step file opens with front-matter");
    let end = rest.find("\n---\n").expect("front-matter closes");
    text.split_at("---\n".len() + end + "\n---\n".len())
}

/// Delete every occurrence of `token` from a step's body, matching the fence's
/// normalized view (so a fact wrapped across a line break is still deleted).
fn delete_fact(pack: &Path, step: &str, token: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let text = fs::read_to_string(&path).expect("read the copied step");
    let (front, body) = split_body(&text);

    // Walk the body once, building the fence's normalized view plus a **byte-for-byte**
    // map back into `body` (`origin.resize` keeps the map indexable by normalized byte
    // offset, which a per-char push would break the moment prose carries an em dash).
    let mut norm = String::new();
    let mut origin: Vec<usize> = Vec::new();
    let mut pending_space = false;
    for (index, ch) in body.char_indices() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !norm.is_empty() {
            norm.push(' ');
            origin.resize(norm.len(), index);
        }
        pending_space = false;
        norm.push(ch.to_ascii_lowercase());
        origin.resize(norm.len(), index);
    }

    let mut spans = Vec::new();
    let mut from = 0;
    while let Some(hit) = norm[from..].find(token) {
        let start = from + hit;
        let end = start + token.len();
        spans.push((
            origin[start],
            origin.get(end).copied().unwrap_or(body.len()),
        ));
        from = end;
    }
    let mut stripped = body.to_string();
    for (start, end) in spans.into_iter().rev() {
        stripped.replace_range(start..end, " ");
    }
    assert_ne!(
        stripped, body,
        "the shipped `{step}` step must state \"{token}\" for its deletion to bite",
    );
    fs::write(&path, format!("{front}{stripped}")).expect("write the mutated step");
}

/// Every (declaring step × declared code × required token) triple the tree owes —
/// read off the tree's own steps and the code-side map, never a hand list.
fn owed_triples(pack: &Path) -> Vec<(String, String, String)> {
    let mut steps: Vec<PathBuf> = fs::read_dir(pack.join("steps"))
        .expect("read the copied steps dir")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    steps.sort();

    let mut out = Vec::new();
    for path in steps {
        let id = path
            .file_stem()
            .expect("step file stem")
            .to_string_lossy()
            .into_owned();
        let bytes = fs::read(&path).expect("read the copied step");
        let def = engine::compose::load_step_def(&id, &bytes).expect("step front-matter parses");
        for code in &def.states_constraints {
            let Some((_, tokens)) = CONSTRAINT_REQUIRED_TOKENS
                .iter()
                .find(|(fenced, _)| fenced == code)
            else {
                continue;
            };
            for token in *tokens {
                out.push((id.clone(), code.clone(), (*token).to_owned()));
            }
        }
    }
    out
}

/// **Arm 5.** A pack that keeps a `states-constraints:` declaration while its prose
/// loses the fact the declaration buys is **refused at pack-load**, naming step, code
/// and token.
///
/// The axis is the shipped tree's own **(declaring step × declared code × required
/// token)** triples, enumerated from the pack's steps and
/// [`CONSTRAINT_REQUIRED_TOKENS`] — so a step that starts declaring a fenced code
/// joins the sweep by construction. Each token is deleted in turn from a copy of the
/// dev pack and the composing `jigc start` must block; the deletion is asserted to
/// bite, so the sweep also proves the shipped prose really carries every fact the map
/// claims, and the covered code set must equal the map's, so a tree that silently
/// stopped declaring a code cannot pass by having nothing to sweep. Red on rc.9: the
/// review deleted 590 characters of copy-in contract prose from a migrate step, kept
/// its front-matter code, and every fence stayed green.
///
/// The methodology-pack seam of the same fence is `stated_at_fence.rs`'s.
#[test]
fn a_pack_that_drops_a_named_fact_is_refused_at_load() {
    let repo = TempDir::new("fact-repo");
    let home = TempDir::new("fact-home");
    let pack = TempDir::new("fact-pack");
    copy_tree(&embedded_pack_tree(), pack.path());

    // A minimal project the compose can run in.
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README.md");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    // `workflow --preview` composes through the same pack load and mints nothing, so
    // the sweep can run it once per triple without a serial task collision.
    let compose = |pack: &Path| {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(["workflow", "single-task", "--preview"])
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env("JIGC_PACK_DIR", pack)
            .output()
            .expect("spawn jigc")
    };

    // The control: the unmutated tree composes clean, so every failure below is the
    // mutation's and not the fixture's.
    let clean = compose(pack.path());
    assert!(
        clean.status.success(),
        "the unmutated pack must load clean; stdout:\n{}\nstderr:\n{}",
        stdout_of(&clean),
        stderr_of(&clean),
    );

    let triples = owed_triples(pack.path());
    let covered: BTreeSet<&str> = triples.iter().map(|(_, code, _)| code.as_str()).collect();
    let fenced: BTreeSet<&str> = CONSTRAINT_REQUIRED_TOKENS
        .iter()
        .map(|(code, _)| *code)
        .collect();
    assert_eq!(
        covered, fenced,
        "the shipped tree must declare every fenced constraint code for the axis to sweep them",
    );

    for (step, code, token) in triples {
        let path = pack.path().join("steps").join(format!("{step}.yaml"));
        let original = fs::read_to_string(&path).expect("read the step");
        delete_fact(pack.path(), &step, &token);
        let out = compose(pack.path());
        let stderr = stderr_of(&out);
        fs::write(&path, &original).expect("restore the step");

        assert!(
            !out.status.success(),
            "deleting \"{token}\" from `{step}` (declaring `{code}`) must block pack load; \
             stdout:\n{}\nstderr:\n{stderr}",
            stdout_of(&out),
        );
        for named in [format!("`{step}`"), code.clone(), token.clone()] {
            assert!(
                stderr.contains(&named),
                "the refusal must name `{named}`; got:\n{stderr}",
            );
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// Arm 6 — the cold-start surfaces tell the truth
// ═════════════════════════════════════════════════════════════════════════════

/// The condition the preload tier and the store trailer must state **together** — one
/// phrase, two surfaces (`cli::render::AHEAD_STAMP_PHRASE`, private to the crate, so
/// the agreement is asserted on the emitted bytes of both).
const AHEAD_PHRASE: &str = "stamped above this build's schema-version";

/// Wordings that claim a repeat `create` / `author` is refused — the clause M47 T1
/// deleted from 14 surfaces, plus the two forms the trial's worker read it back in.
const REJECTION_CLAIMS: [&str; 4] = [
    "never after",
    "rejects a second create",
    "already-staged doc rejects",
    "only once per task",
];

/// Every workflow id of both embedded packs, read from the loaded registries.
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

/// **Arm 6.** The surfaces that carry an agent from a **cold start** are true first —
/// checked against the binary they describe, never read as prose.
///
/// Three cold-start surfaces, each over its own axis:
///
///   * the **preload tier**: `.jigc/AGENT.md`'s store-sweep clause is scoped by the
///     exit-flipping exception, and the claim is verified **live** — a corpus carrying
///     a doc stamped above this build's schema-version makes `jigc validate` exit
///     non-zero with the same phrase in its closing trailer, so preload and binary
///     cannot state opposite exits for one condition;
///   * the **install summary**: `jigc setup` names the hooks dir **git resolved** — under
///     `core.hooksPath` the printed path must resolve, from the directory setup ran in,
///     to the hook the install actually wrote;
///   * the **authoring order**: `create` → `author` → `create` → `author` over one
///     identity on one task all exit 0, and **no composed workflow of either pack** —
///     enumerated from the loaded registries, never a hand list — claims otherwise.
///
/// Red on rc.9: the preload stated an unqualified exit-0 its own trailer contradicted,
/// `setup` printed a `.git/hooks/pre-commit` literal that named no file under
/// `core.hooksPath`, and 14 surfaces stated a one-shot rule the binary has never
/// enforced.
#[test]
fn the_cold_start_surfaces_tell_the_truth() {
    // ── the preload tier, checked against the binary ──
    let corpus = TrialCorpus::build(State::Fresh);
    let agent_md = fs::read_to_string(corpus.repo().join(".jigc").join("AGENT.md"))
        .expect("`jigc setup` writes the preloaded AGENT.md");
    assert!(
        agent_md.contains("exits 0 even when it surfaces findings"),
        "the preload must still state the report-only stance; got:\n{agent_md}",
    );
    assert!(
        agent_md.contains("unless one of a few conditions flips that exit")
            && agent_md.contains(AHEAD_PHRASE),
        "the preload must scope the report-only stance by the exit-flipping conditions — \
         as the class the whole axis is in, not as a cause true of only some members (the \
         M47 completion audit: `reconciliation.rename` is a sweep that worked, so \
         \"the sweep itself could not be trusted\" excluded a real member); got:\n{agent_md}",
    );

    let decisions = corpus.repo().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");
    fs::write(
        decisions.join("cache-strategy.md"),
        adr_body("Cache strategy", Some(99)),
    )
    .expect("write the ahead-stamped adr");
    corpus.git(&["add", "."]);
    corpus.git(&["commit", "-q", "-m", "an ahead-stamped adr"]);

    let swept = corpus.jigc(&["validate"]);
    assert!(
        !swept.status.success(),
        "an above-current stamp must flip the store sweep's exit — the preload says so; \
         stdout:\n{}\nstderr:\n{}",
        stdout_of(&swept),
        stderr_of(&swept),
    );
    let printed = format!("{}{}", stdout_of(&swept), stderr_of(&swept));
    assert!(
        printed.contains(AHEAD_PHRASE) && printed.contains("exits non-zero"),
        "the sweep's own trailer must state the exception in the preload's words; got:\n{printed}",
    );

    // ── the install summary names the hooks dir git resolved ──
    let hooks_repo = TempDir::new("hooks-repo");
    let hooks_home = TempDir::new("hooks-home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(hooks_repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    git(&["config", "core.hooksPath", "my-hooks"]);
    fs::write(hooks_repo.path().join("README.md"), "hello\n").expect("write README.md");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);

    let installed = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("setup")
        .current_dir(hooks_repo.path())
        .env("HOME", hooks_home.path())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn jigc setup");
    assert!(
        installed.status.success(),
        "`jigc setup` must succeed under core.hooksPath; stderr:\n{}",
        stderr_of(&installed),
    );
    let summary = format!("{}{}", stdout_of(&installed), stderr_of(&installed));
    let line = summary
        .lines()
        .find(|line| line.contains("pre-commit hook →"))
        .unwrap_or_else(|| panic!("the summary must name the installed hook; got:\n{summary}"));
    let printed_path = line
        .split("pre-commit hook →")
        .nth(1)
        .expect("the arrow splits the line")
        .split("   (")
        .next()
        .expect("the annotation follows the path")
        .trim();
    // Resolve it the way its reader would: absolute as-is, relative against the
    // directory `jigc setup` ran in.
    let candidate = if Path::new(printed_path).is_absolute() {
        PathBuf::from(printed_path)
    } else {
        hooks_repo.path().join(printed_path)
    };
    assert!(
        candidate.is_file(),
        "the printed hook path `{printed_path}` must name the file the install wrote; \
         summary:\n{summary}",
    );
    assert_eq!(
        fs::canonicalize(&candidate).expect("canonicalize the printed path"),
        fs::canonicalize(hooks_repo.path().join("my-hooks").join("pre-commit"))
            .expect("canonicalize the resolved hook"),
        "the printed path must resolve to the hook git's own hooks dir received",
    );

    // ── the authoring order repeats, and nothing says it does not ──
    let authoring = TrialCorpus::build(State::Fresh);
    let task = authoring.start_workflow("record-decision", "settle-the-cache-strategy");
    let first = authoring.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        &task,
    ]);
    assert!(
        !first.contains("already existed"),
        "the first create mints fresh; got:\n{first}",
    );
    authoring.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD,
    );
    let second = authoring.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Cache strategy",
        "--task",
        &task,
    ]);
    assert!(
        second.contains("already existed — copied in for update"),
        "the second create over the staged copy must ack the copy-in, not reject; got:\n{second}",
    );
    authoring.jigc_stdin_ok(
        &["doc", "author", "adr", "--from-file", "-", "--task", &task],
        ADR_PAYLOAD_REVISED,
    );
    let staged = authoring.jigc_ok(&["doc", "show", "adr:cache-strategy", "--task", &task]);
    assert!(
        staged.contains("Colder reads, revised."),
        "the second author's prose must be the staged truth; got:\n{staged}",
    );

    let workflows = all_workflow_ids();
    assert!(
        workflows.len() >= 20,
        "both packs' workflow registries must load; got {} ids",
        workflows.len(),
    );
    for (pack, id) in &workflows {
        let composed = authoring.jigc(&["workflow", id, "--preview"]);
        let text = normalized(&format!(
            "{}\n{}",
            stdout_of(&composed),
            stderr_of(&composed)
        ));
        for claim in REJECTION_CLAIMS {
            assert!(
                !text.contains(claim),
                "the composed `{pack}:{id}` preview claims a repeat `create`/`author` is \
                 refused (\"{claim}\") — the binary refuses neither; got:\n{text}",
            );
        }
    }
    for args in [
        ["doc", "author", "--help"].as_slice(),
        ["doc", "--help"].as_slice(),
    ] {
        let help = normalized(&authoring.jigc_ok(args));
        for claim in REJECTION_CLAIMS {
            assert!(
                !help.contains(claim),
                "`jigc {}` claims a repeat is refused (\"{claim}\"); got:\n{help}",
                args.join(" "),
            );
        }
    }
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
