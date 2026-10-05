//! The rc.24 fix pass, `(R3, F7)` — **a task remembers what it copied in**.
//!
//! `design/reconciliation.md` → *What reconciliation does NOT do*: *"No silent discard,
//! ever … There is no door exemption from this sentence."* Through `1.0.0-rc.24` that
//! sentence held only where the gitignored `file-state` cache happened to hold the doc's
//! key. With no key — every fresh clone, CI checkout and container session before its first
//! landed finalize; after `jigc unmanage`; a deleted cache; a doc that is new and untracked —
//! the reconciler's `UNKNOWN` arm adopted whatever was on disk, so a hand edit made **after**
//! a task's first write to that doc was overwritten by the committing door at exit 0 and was
//! in no git object
//! (`completions/artifacts/M55/per-axis-review-rc24/tier1-verification/R3-F7.md`).
//!
//! Two rules close it (`DECISIONS.md` → 2026-10-04, fork `(R3, F7)`):
//!
//! 1. **The copy-in door records the baseline** when the key is absent — the hash of the raw
//!    bytes it copied, under the key the sweep reads. From then on the doc reads `IN_SYNC` or
//!    `DRIFTED` like any baselined doc, and a later hand edit is the shipped
//!    `DRIFTED + TOUCHED → reconciliation.conflict-block`.
//! 2. **The base pin is the backstop** for a key lost after the copy-in (or a task an older
//!    binary copied in): `UNKNOWN` + touched + the pin carries a blob for the path + the
//!    on-disk bytes differ from it → the same conflict-block.
//!
//! 3. **For a doc a task holds, the task's own witness decides** (`DECISIONS.md` →
//!    2026-10-05, option A; the completion audit's reconcile-baseline F1 · F2 · F7 and the
//!    e2e's F5). Rules 1 and 2 put the task's question — *is the file still what I copied
//!    in?* — to a record keyed by path, which every other jigc writer moves and a doc that
//!    does not conform, or that git never committed, is never in. So each copy-in also
//!    writes, into the task's own `docs/provenance.json`, the hash of the bytes it read
//!    (raw, and as git stores them at the doc's home), and both committing doors compare
//!    the file with it. One rule everywhere: an edit before the task's first write is
//!    carried and lands; an edit after it blocks. A working area with no witness — one an
//!    older binary minted — is still decided by rules 1 and 2.
//!
//! The suite iterates the class, not the reported instance: both committing doors × a
//! placement and a location doctype × every way the key is absent at the first write × the
//! two orders of the hand edit, every `doc` write leaf as the first touch, and the nine
//! cells the new writer brings (the planning's advocacy, `advocate-promote-1`): raw-bytes
//! hash · the sweep's key · the save-lock timeout · concurrent first writes · a
//! non-conformant doc · the adoption stated at the write door · the store sweep's reading ·
//! two tasks on one doc · a task with no record. And the crossing cells the audit found
//! skipped, which are where the losses were: a second jigc writer of the key between the
//! hand edit and the holding task's door · an untracked doc that does not conform · an
//! untracked doc whose key is lost after the copy-in · two tasks on one untracked doc ·
//! a doc git checked out again in another line-ending form between the copy-in and the door.
//!
//! Real binary throughout, in throwaway corpora built by the shared fixture builder.

use std::fs;
use std::process::Output;
use std::sync::{Arc, Barrier};

use engine::file_state::{FileStateRecord, hash_bytes};

use crate::support;
use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus, read};

/// The hand line written straight into the file, out of band.
const HAND: &str = "A hand line written out of band HAND-MARK-7731.";

/// The prose the task writes through the CLI.
const TASK_PROSE: &str = "Which domains earn a pack, and when TASK-PROSE-4410.";

const CONFLICT: &str = "reconciliation.conflict-block";
const ADOPT: &str = "file-state.baseline-adopt";

/// The committed ADR the location arms use, at its home.
const ADR_HOME: &str = "docs/decisions/single-node-cache.md";
const ADR: &str = "adr:single-node-cache";

const MILESTONE_TITLE: &str = "sharpen the docs";
const MILESTONE: &str = "sharpen-the-docs";

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The committed singleton set plus one committed ADR, every doc baselined by the finalize
/// that landed it. Each arm takes a copy.
fn baselined_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let task = corpus.start_workflow("single-task", "record the cache decision");
    corpus.jigc_ok(&[
        "doc",
        "create",
        "adr",
        "--title",
        "Single-node cache",
        "--task",
        &task,
    ]);
    for (slot, prose) in [
        ("context", "Session lookups must stay sub-millisecond."),
        ("decision", "A single in-memory node keeps lookups fast."),
        ("consequences", "A cold node loses its sessions."),
    ] {
        corpus.set_slot(&format!("{ADR}#{slot}"), &task, prose);
    }
    corpus.finalize(&task, "cache", "record the cache decision", false);
    assert!(corpus.repo().join(ADR_HOME).is_file(), "the ADR landed");
    corpus
}

/// The doctype kind — the two ways a managed doc has a home.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// A `placement` doctype: `vision`, at the literal repo-root `VISION.md`.
    Placement,
    /// A `location` doctype: `adr`, at `<docs-root>/decisions/<slug>.md`.
    Location,
}

impl Kind {
    const ALL: [Kind; 2] = [Kind::Placement, Kind::Location];

    /// The doc's home, repo-relative — its file-state key.
    fn home(self) -> &'static str {
        match self {
            Kind::Placement => "VISION.md",
            Kind::Location => ADR_HOME,
        }
    }

    /// `<type>:<slug>`.
    fn address(self) -> &'static str {
        match self {
            Kind::Placement => "vision:vision",
            Kind::Location => ADR,
        }
    }

    /// The slot the task writes.
    fn slot(self) -> String {
        match self {
            Kind::Placement => "vision:vision#open-questions".to_owned(),
            Kind::Location => format!("{ADR}#decision"),
        }
    }

    /// The committed line the hand edit lands under — in a slot the task does not write.
    fn anchor(self) -> &'static str {
        match self {
            Kind::Placement => "A deterministic CLI assembles exactly the context a task needs.\n",
            Kind::Location => "A cold node loses its sessions.\n",
        }
    }

    /// Append [`HAND`] under [`Self::anchor`], on disk, uncommitted — a conformant,
    /// prose-only out-of-band edit.
    fn hand_edit(self, corpus: &TrialCorpus) {
        let path = corpus.repo().join(self.home());
        let before = fs::read_to_string(&path).expect("read the doc at its home");
        let after = before.replacen(self.anchor(), &format!("{}{HAND}\n", self.anchor()), 1);
        assert_ne!(before, after, "{self:?}: the hand edit changes the doc");
        fs::write(&path, after).expect("write the hand edit");
    }

    /// The task's first write to the doc — the copy-in. Returns the ack envelope.
    fn first_write(self, corpus: &TrialCorpus, task: &str) -> serde_json::Value {
        let out = corpus.jigc_stdin(
            &[
                "doc",
                "set-slot",
                &self.slot(),
                "--from-file",
                "-",
                "--task",
                task,
                "--format",
                "json",
            ],
            &format!("{TASK_PROSE}\n"),
        );
        let ack: serde_json::Value = stdout_json(&out, &[0], "the first write");
        assert_eq!(
            ack["copied_in"], true,
            "{self:?}: the premise — the first write copies the doc in: {ack}"
        );
        ack
    }
}

/// A way the doc has no `file-state` key when the task first writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Absent {
    /// The fresh-clone shape: `.jigc/state/` is gitignored, so a clone has no record.
    Clone,
    /// `jigc unmanage <home>` dropped the key.
    Unmanaged,
    /// The doc is on disk at its home, untracked, and was never baselined — no key, and no
    /// blob at any base pin either.
    Untracked,
}

impl Absent {
    const ALL: [Absent; 3] = [Absent::Clone, Absent::Unmanaged, Absent::Untracked];

    /// Put the corpus in this state. Runs before the task (or milestone) is minted, so a
    /// commit it makes is behind the base pin.
    fn apply(self, corpus: &TrialCorpus, kind: Kind) {
        match self {
            Absent::Clone => corpus.fresh_clone_shape(),
            Absent::Unmanaged => {
                corpus.jigc_ok(&["unmanage", kind.home()]);
            }
            Absent::Untracked => {
                corpus.git(&["rm", "-q", "--cached", "--", kind.home()]);
                corpus.git(&["commit", "-q", "-m", "chore: untrack the doc"]);
                corpus.fresh_clone_shape();
            }
        }
        assert_eq!(
            recorded(corpus, kind.home()),
            None,
            "{self:?}/{kind:?}: the premise — no file-state key before the first write",
        );
        assert!(
            corpus.repo().join(kind.home()).is_file(),
            "{self:?}/{kind:?}: the premise — the doc is on disk at its home",
        );
    }

    /// Put the doc's bytes back to `original` — *revert the external edit on disk*, the
    /// conflict route's second exit. `git checkout` where git has the bytes; a rewrite of the
    /// saved bytes where it does not.
    fn revert(self, corpus: &TrialCorpus, kind: Kind, original: &str) {
        match self {
            Absent::Clone | Absent::Unmanaged => {
                corpus.git(&["checkout", "--", kind.home()]);
            }
            Absent::Untracked => {
                fs::write(corpus.repo().join(kind.home()), original).expect("restore the doc");
            }
        }
        assert_eq!(read(&corpus.repo(), kind.home()), original);
    }
}

/// A committing door.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Door {
    /// `jigc task finalize <id>`.
    Task,
    /// `jigc milestone finalize <id>`, over one sub-task.
    Milestone,
}

impl Door {
    const ALL: [Door; 2] = [Door::Task, Door::Milestone];

    /// Mint the work unit; returns the task id the `doc` writes are scoped to.
    fn mint(self, corpus: &TrialCorpus) -> String {
        match self {
            Door::Task => {
                let task = corpus.start_workflow("single-task", "sharpen the docs");
                corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
                corpus.set_field(&format!("commit:{task}#scope"), &task, "docs");
                corpus.set_slot(&format!("commit:{task}#summary"), &task, "sharpen the docs");
                task
            }
            Door::Milestone => {
                corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
                let ack =
                    corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "alpha sharpens a doc"]);
                let (_, rest) = ack
                    .split_once("added task:")
                    .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
                rest.split_whitespace()
                    .next()
                    .expect("the sub-task id")
                    .to_owned()
            }
        }
    }

    /// The committing door's argv.
    fn finalize_argv(self, task: &str) -> Vec<String> {
        let argv: &[&str] = match self {
            Door::Task => &["task", "finalize", task, "--format", "json"],
            Door::Milestone => &["milestone", "finalize", MILESTONE, "--format", "json"],
        };
        argv.iter().map(|word| (*word).to_owned()).collect()
    }

    fn finalize(self, corpus: &TrialCorpus, task: &str) -> Output {
        let argv = self.finalize_argv(task);
        let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
        corpus.jigc(&argv)
    }
}

/// The recorded hash for `key`, read through the engine API that owns the record.
fn recorded(corpus: &TrialCorpus, key: &str) -> Option<String> {
    FileStateRecord::load(&corpus.repo().join(".jigc"))
        .expect("load the file-state record")
        .get(key)
        .map(str::to_owned)
}

/// Drop `key` through the engine API — the shape a key lost **after** the copy-in has
/// (a deleted cache, an `unmanage`), and what a task copied in by a binary older than the
/// copy-in baseline looks like: a staged doc with no record.
fn forget(corpus: &TrialCorpus, key: &str) {
    let jigc_root = corpus.repo().join(".jigc");
    let mut record = FileStateRecord::load(&jigc_root).expect("load the record");
    assert!(
        record.forget(key),
        "the premise: `{key}` was recorded before it is dropped"
    );
    record.save(&jigc_root).expect("save the record");
    assert_eq!(recorded(corpus, key), None);
}

/// The raw-byte hash of the file at `rel`.
fn disk_hash(corpus: &TrialCorpus, rel: &str) -> String {
    hash_bytes(&fs::read(corpus.repo().join(rel)).expect("read the doc"))
}

fn head(corpus: &TrialCorpus) -> String {
    corpus.git(&["rev-parse", "HEAD"])
}

/// The findings of a `--format json` envelope. A landed `milestone finalize` carries no
/// `findings` key at all — the same empty set.
fn findings(envelope: &serde_json::Value) -> Vec<serde_json::Value> {
    envelope["findings"].as_array().cloned().unwrap_or_default()
}

/// The findings keyed `(code, target)`.
fn keyed(findings: &[serde_json::Value], code: &str, target: &str) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["key"]["code"] == code && f["key"]["target"] == target)
        .cloned()
        .collect()
}

/// Assert `out` is the door **blocked** on the conflict at `home`, with nothing committed and
/// the doc's bytes untouched; returns the one conflict finding.
fn assert_blocked(
    corpus: &TrialCorpus,
    out: &Output,
    home: &str,
    head_before: &str,
    disk_before: &str,
    what: &str,
) -> serde_json::Value {
    let envelope: serde_json::Value = stdout_json(out, &[3], what);
    let rows = keyed(&findings(&envelope), CONFLICT, home);
    assert_eq!(
        rows.len(),
        1,
        "{what}: one `{CONFLICT}` keyed at `{home}`; {}",
        text(out)
    );
    assert_eq!(rows[0]["severity"], "blocking", "{what}: {}", rows[0]);
    assert_eq!(head(corpus), head_before, "{what}: nothing is committed");
    assert_eq!(
        read(&corpus.repo(), home),
        disk_before,
        "{what}: the bytes at the home are untouched",
    );
    rows[0].clone()
}

/// The backticked `jigc …` spans of a route, as emitted.
fn jigc_spans(route: &str) -> Vec<&str> {
    route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc "))
        .collect()
}

/// Run an emitted `jigc …` command through a real `sh` split.
fn run_emitted(corpus: &TrialCorpus, command: &str) -> Output {
    let argv = support::shell_words(command, &corpus.repo(), &corpus.home());
    assert_eq!(argv.first().map(String::as_str), Some("jigc"), "{command}");
    let args: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    corpus.jigc(&args)
}

/// The task's own record of what it copied in — `docs/provenance.json` → `copied-in`.
fn witness(corpus: &TrialCorpus, task: &str) -> serde_json::Value {
    let manifest = corpus
        .repo()
        .join(".jigc/tasks")
        .join(task)
        .join("docs/provenance.json");
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest).expect("read the manifest"))
            .expect("the manifest is JSON");
    manifest["copied-in"].clone()
}

/// Rewrite `task`'s manifest **without** its `copied-in` member — byte for byte what a
/// binary older than the witness wrote, so the area is one *the previous format minted*.
fn strip_witness(corpus: &TrialCorpus, task: &str) {
    let path = corpus
        .repo()
        .join(".jigc/tasks")
        .join(task)
        .join("docs/provenance.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).expect("read the manifest"))
            .expect("the manifest is JSON");
    assert!(
        manifest
            .as_object_mut()
            .expect("the manifest is an object")
            .remove("copied-in")
            .is_some(),
        "the premise: `{task}` recorded a witness before it is stripped",
    );
    let mut bytes = serde_json::to_string_pretty(&manifest).expect("serialize the manifest");
    bytes.push('\n');
    fs::write(&path, bytes).expect("write the previous-format manifest");
}

/// Write `prose` into `slot` for `task` — a `doc set-slot`, returning the ack envelope.
fn write_slot(corpus: &TrialCorpus, task: &str, slot: &str, prose: &str) -> serde_json::Value {
    let out = corpus.jigc_stdin(
        &[
            "doc",
            "set-slot",
            slot,
            "--from-file",
            "-",
            "--task",
            task,
            "--format",
            "json",
        ],
        &format!("{prose}\n"),
    );
    stdout_json(&out, &[0], &format!("`doc set-slot {slot}`"))
}

/// An ADR that is on disk at its home and **git has never committed** — a hand-started
/// draft. `conformant: false` leaves its required `consequences` slot empty, so it parses
/// and fails the conformance gate: the copy-in records no baseline for it, and there is no
/// blob for it at any pin.
struct Draft;

impl Draft {
    const HOME: &'static str = "docs/decisions/draft-queue.md";
    const ADDRESS: &'static str = "adr:draft-queue";
    /// The line the hand edit lands under — in a slot the task does not write.
    const ANCHOR: &'static str = "Session lookups must stay sub-millisecond.\n";

    /// Write the draft; returns its bytes.
    fn write(corpus: &TrialCorpus, conformant: bool) -> String {
        let committed = read(&corpus.repo(), ADR_HOME);
        let mut draft = committed.replacen("# Single-node cache", "# Draft queue", 1);
        if !conformant {
            draft = draft.replacen("A cold node loses its sessions.\n", "", 1);
        }
        assert_ne!(committed, draft, "the draft is its own doc");
        fs::write(corpus.repo().join(Self::HOME), &draft).expect("write the draft");
        assert_eq!(
            corpus.git(&["ls-files", "--", Self::HOME]),
            "",
            "the premise: git has never seen the draft",
        );
        draft
    }

    /// The slot the task writes — the one the non-conformant draft leaves empty.
    fn slot() -> String {
        format!("{}#consequences", Self::ADDRESS)
    }

    /// Append [`HAND`] under [`Self::ANCHOR`], on disk.
    fn hand_edit(corpus: &TrialCorpus) -> String {
        let path = corpus.repo().join(Self::HOME);
        let before = fs::read_to_string(&path).expect("read the draft");
        let after = before.replacen(Self::ANCHOR, &format!("{}{HAND}\n", Self::ANCHOR), 1);
        assert_ne!(before, after, "the hand edit changes the draft");
        fs::write(&path, &after).expect("write the hand edit");
        after
    }
}

/// **The class.** Both committing doors × both doctype kinds × every way the key is absent
/// at the first write: a hand edit made *after* the task's first write blocks the door at
/// exit 3 under `reconciliation.conflict-block`, commits nothing, and leaves the hand line on
/// disk — and each exit the route names works as printed.
#[test]
fn a_hand_edit_after_the_first_write_blocks_both_committing_doors_and_survives() {
    let base = baselined_corpus();
    for door in Door::ALL {
        for kind in Kind::ALL {
            for absent in Absent::ALL {
                let what = format!("{door:?}/{kind:?}/{absent:?}");
                let corpus = base.copy_state();
                absent.apply(&corpus, kind);
                let original = read(&corpus.repo(), kind.home());
                let task = door.mint(&corpus);

                // The copy-in records what it copied, under the key the sweep reads.
                let ack = kind.first_write(&corpus, &task);
                assert_eq!(
                    recorded(&corpus, kind.home()),
                    Some(hash_bytes(original.as_bytes())),
                    "{what}: the first write records the hash of the raw bytes it copied",
                );
                assert_eq!(
                    keyed(&findings(&ack), ADOPT, kind.home()).len(),
                    1,
                    "{what}: the write that adopted the baseline says so: {ack}",
                );

                kind.hand_edit(&corpus);
                let edited = read(&corpus.repo(), kind.home());
                let head_before = head(&corpus);

                // The task door's two previews say what the door will do.
                if door == Door::Task {
                    for preview in [
                        vec!["task", "validate", task.as_str(), "--format", "json"],
                        vec![
                            "task",
                            "finalize",
                            task.as_str(),
                            "--dry-run",
                            "--format",
                            "json",
                        ],
                    ] {
                        let out = corpus.jigc(&preview);
                        assert_blocked(
                            &corpus,
                            &out,
                            kind.home(),
                            &head_before,
                            &edited,
                            &format!("{what}: `jigc {}`", preview.join(" ")),
                        );
                    }
                }

                let out = door.finalize(&corpus, &task);
                let conflict =
                    assert_blocked(&corpus, &out, kind.home(), &head_before, &edited, &what);
                assert!(
                    edited.contains(HAND) && !edited.contains(TASK_PROSE),
                    "{what}: the hand line is on disk and the task's prose is not",
                );
                let route = conflict["route"].as_str().expect("the conflict routes");

                // Exit one, on a copy: the emitted `jigc …` command, run as printed.
                let spans = jigc_spans(route);
                assert_eq!(spans.len(), 1, "{what}: one `jigc` span in: {route}");
                let aside = corpus.copy_state();
                let ran = run_emitted(&aside, spans[0]);
                assert_eq!(
                    ran.status.code(),
                    Some(0),
                    "{what}: the emitted `{}` runs; {}",
                    spans[0],
                    text(&ran),
                );
                assert_eq!(
                    read(&aside.repo(), kind.home()),
                    edited,
                    "{what}: `{}` leaves the hand edit on disk",
                    spans[0],
                );

                // Exit two: revert the external edit on disk, and the door lands the task.
                assert!(
                    route.contains("revert the external edit on disk"),
                    "{what}: the route names the revert: {route}",
                );
                absent.revert(&corpus, kind, &original);
                let landed = door.finalize(&corpus, &task);
                assert_eq!(
                    landed.status.code(),
                    Some(0),
                    "{what}: with the edit reverted the door lands; {}",
                    text(&landed),
                );
                let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
                assert!(
                    at_head.contains(TASK_PROSE) && !at_head.contains(HAND),
                    "{what}: the landed doc carries the task's prose; got:\n{at_head}",
                );
            }
        }
    }
}

/// **The reported instance, on a real `git clone`** — R3-F7.md §1, byte for byte: a second
/// clone, no other act, a `single-task`, one slot write, a hand line, `task finalize`.
#[test]
fn a_second_clones_first_task_does_not_overwrite_a_hand_edit() {
    let base = TrialCorpus::build(State::CommittedSingletons);
    let root = base.repo().parent().expect("the corpus root").to_path_buf();
    support::branch_and_pull::git_in(&root, &base, &["clone", "-q", "repo", "clone2"]);
    let clone = root.join("clone2");
    support::branch_and_pull::git_in(&clone, &base, &["config", "user.email", "m@example.com"]);
    support::branch_and_pull::git_in(&clone, &base, &["config", "user.name", "Mate"]);
    assert!(
        !clone.join(".jigc/state").exists(),
        "the premise: a clone carries no file-state cache",
    );
    let jigc = |args: &[&str], stdin: &str| base.jigc_stdin_from(&clone, args, stdin);
    let minted = jigc(
        &[
            "start",
            "--workflow",
            "single-task",
            "sharpen the open questions",
        ],
        "",
    );
    assert_eq!(minted.status.code(), Some(0), "{}", text(&minted));
    let task = "sharpen-the-open-questions";
    let wrote = jigc(
        &[
            "doc",
            "set-slot",
            "vision:vision#open-questions",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        &format!("{TASK_PROSE}\n"),
    );
    assert_eq!(wrote.status.code(), Some(0), "{}", text(&wrote));
    for (addr, value) in [("type", "docs"), ("scope", "vision")] {
        let out = jigc(
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#{addr}"),
                "--value",
                value,
                "--task",
                task,
            ],
            "",
        );
        assert_eq!(out.status.code(), Some(0), "{}", text(&out));
    }
    let out = jigc(
        &[
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
            "--task",
            task,
        ],
        "sharpen the open questions\n",
    );
    assert_eq!(out.status.code(), Some(0), "{}", text(&out));

    let vision = clone.join("VISION.md");
    let before = fs::read_to_string(&vision).expect("read the clone's VISION.md");
    fs::write(&vision, format!("{before}\n{HAND}\n")).expect("append the hand line");
    let head_before = support::branch_and_pull::git_in(&clone, &base, &["rev-parse", "HEAD"]);

    let out = jigc(&["task", "finalize", task, "--format", "json"], "");
    let envelope: serde_json::Value = stdout_json(&out, &[3], "the clone's first finalize");
    assert_eq!(
        keyed(&findings(&envelope), CONFLICT, "VISION.md").len(),
        1,
        "the door blocks on the conflict; {}",
        text(&out),
    );
    assert_eq!(
        support::branch_and_pull::git_in(&clone, &base, &["rev-parse", "HEAD"]),
        head_before,
        "nothing is committed",
    );
    assert!(
        fs::read_to_string(&vision)
            .expect("re-read VISION.md")
            .contains(HAND),
        "the hand line is still on disk",
    );
}

/// **The declared order keeps landing — with or without a recorded baseline.** A hand edit
/// made *before* the task's first write is carried by the copy-in, so the door lands both
/// sides in one commit (`design/storage.md` → Concurrent writers, *What none of this buys*)
/// — at both doors, for both kinds, whichever way the key was absent, **and where it was
/// held**. That last cell is the one the task's own witness changed (`DECISIONS.md` →
/// 2026-10-05, the first accepted consequence): through the previous round a held baseline
/// conflict-blocked this order while a clone with no record landed it, so one sequence had
/// two outcomes decided by a gitignored cache. The file is what the task copied in, so it
/// lands — and the door says the baseline moved, under `reconciliation.absorb`, in words
/// that name the staged copy rather than an "external edit".
#[test]
fn a_hand_edit_before_the_first_write_still_lands_merged() {
    let base = baselined_corpus();
    for door in Door::ALL {
        for kind in Kind::ALL {
            // The held cell is engine-identical across kinds; the second kind rides the
            // task door only.
            if door == Door::Milestone && kind == Kind::Location {
                continue;
            }
            let what = format!("{door:?}/{kind:?}/held");
            let corpus = base.copy_state();
            let held = recorded(&corpus, kind.home());
            assert!(held.is_some(), "{what}: the premise — the key is held");
            let task = door.mint(&corpus);
            kind.hand_edit(&corpus);
            kind.first_write(&corpus, &task);
            assert_eq!(
                recorded(&corpus, kind.home()),
                held,
                "{what}: a held key is never moved by the copy-in",
            );
            let out = door.finalize(&corpus, &task);
            let envelope: serde_json::Value = stdout_json(&out, &[0], &what);
            if door == Door::Task {
                let carried = keyed(&findings(&envelope), "reconciliation.absorb", kind.home());
                assert_eq!(carried.len(), 1, "{what}: the move is said; {}", text(&out));
                let said = carried[0]["message"].as_str().unwrap_or_default();
                assert!(
                    said.contains("it is what this work copied in")
                        && !said.contains("external edit"),
                    "{what}: in words true of a doc the task holds: {said}",
                );
            }
            let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
            assert!(
                at_head.contains(HAND) && at_head.contains(TASK_PROSE),
                "{what}: both sides are in the commit; got:\n{at_head}",
            );
        }
    }
    for door in Door::ALL {
        for kind in Kind::ALL {
            for absent in Absent::ALL {
                let what = format!("{door:?}/{kind:?}/{absent:?}");
                let corpus = base.copy_state();
                absent.apply(&corpus, kind);
                let task = door.mint(&corpus);
                kind.hand_edit(&corpus);
                kind.first_write(&corpus, &task);
                assert_eq!(
                    recorded(&corpus, kind.home()),
                    Some(disk_hash(&corpus, kind.home())),
                    "{what}: the copy-in records the bytes it copied — the edited ones",
                );
                let out = door.finalize(&corpus, &task);
                assert_eq!(
                    out.status.code(),
                    Some(0),
                    "{what}: the merge order lands; {}",
                    text(&out),
                );
                let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
                assert!(
                    at_head.contains(HAND) && at_head.contains(TASK_PROSE),
                    "{what}: both sides are in the commit; got:\n{at_head}",
                );
            }
        }
    }
}

/// **A hand edit undone after the copy-in is an edit after it, and blocks** (the rc.24 fix
/// pass's completion audit, F8 and its silent sibling).
///
/// The hand adds a line, the task copies the doc in — carrying the line — and the hand then
/// puts the file back (`git checkout -- <doc>`). The staged copy still holds the line and
/// the file does not, so landing would commit, and write back into the worktree, a line its
/// author had just removed. Through the previous round the door did exactly that: where the
/// copy-in had recorded the baseline it said so in an advisory, and where a baseline was
/// already held the doc read in-sync and it said nothing at all. The file is no longer what
/// the task copied in, which is the one question the door now asks, so it blocks — with the
/// key absent at the copy-in and with it held — and the emitted discard runs.
///
/// A working area the previous format minted has no witness, so it keeps the arm it had and
/// that arm's words: the staged copy lands, and the advisory says a change undone on disk is
/// still in it.
#[test]
fn an_edit_undone_after_the_copy_in_blocks_and_an_older_area_says_what_lands() {
    let base = baselined_corpus();
    let kind = Kind::Placement;
    for held in [false, true] {
        let what = format!("the key held at the copy-in: {held}");
        let corpus = base.copy_state();
        if !held {
            corpus.fresh_clone_shape();
        }
        let task = mint_task(&corpus, "sharpen the questions");
        kind.hand_edit(&corpus);
        kind.first_write(&corpus, &task);
        corpus.git(&["checkout", "--", kind.home()]);
        let disk = read(&corpus.repo(), kind.home());
        assert!(
            !disk.contains(HAND),
            "{what}: the premise — the edit is undone"
        );
        let head_before = head(&corpus);
        let out = Door::Task.finalize(&corpus, &task);
        let conflict = assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
        let route = conflict["route"].as_str().expect("the conflict routes");
        let spans = jigc_spans(route);
        assert_eq!(spans.len(), 1, "{what}: one `jigc` span in: {route}");
        let ran = run_emitted(&corpus, spans[0]);
        assert_eq!(ran.status.code(), Some(0), "{what}: {}", text(&ran));
        assert_eq!(
            read(&corpus.repo(), kind.home()),
            disk,
            "{what}: the file stays"
        );
    }

    // The previous format: no witness, so the pulled-edit arm and its sentence.
    let corpus = base.copy_state();
    corpus.fresh_clone_shape();
    let task = mint_task(&corpus, "sharpen the questions");
    kind.hand_edit(&corpus);
    kind.first_write(&corpus, &task);
    strip_witness(&corpus, &task);
    corpus.git(&["checkout", "--", kind.home()]);
    let out = corpus.jigc(&["task", "finalize", &task]);
    let said = text(&out);
    assert_eq!(out.status.code(), Some(0), "the older area lands; {said}");
    assert!(
        said.contains("reconciliation.absorb")
            && said.contains("the staged copy replaces it when this lands")
            && said.contains("a change undone on disk after the copy-in is still in it"),
        "and the door says what lands; {said}",
    );
    assert!(
        !said.contains("external edit absorbed"),
        "never that an external edit was absorbed; {said}",
    );
    let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
    assert!(
        at_head.contains(HAND) && at_head.contains(TASK_PROSE),
        "which is the staged copy — the undone line and the task's prose; got:\n{at_head}",
    );
}

/// **Zero false fire.** A first task with no hand edit lands, at both doors, however the key
/// was absent — and an *untouched* doc's first encounter is still a plain advisory adoption.
#[test]
fn a_first_task_with_no_hand_edit_lands_and_untouched_docs_still_adopt() {
    let base = baselined_corpus();
    for door in Door::ALL {
        for absent in Absent::ALL {
            let kind = Kind::Placement;
            let what = format!("{door:?}/{absent:?}");
            let corpus = base.copy_state();
            absent.apply(&corpus, kind);
            let task = door.mint(&corpus);
            kind.first_write(&corpus, &task);
            let out = door.finalize(&corpus, &task);
            let envelope: serde_json::Value = stdout_json(&out, &[0], &what);
            let rows = findings(&envelope);
            assert!(
                rows.iter().all(|f| f["severity"] != "blocking"),
                "{what}: nothing blocks; {}",
                text(&out),
            );
            // The clone shape drops every key, so the docs the task never touched are still
            // first encounters at the door's sweep — adopted, advisory, as before.
            if absent == Absent::Clone && door == Door::Task {
                assert_eq!(
                    keyed(&rows, ADOPT, "docs/roadmap.md").len(),
                    1,
                    "{what}: an untouched doc is still adopted on first encounter; {}",
                    text(&out),
                );
                assert!(
                    keyed(&rows, ADOPT, kind.home()).is_empty(),
                    "{what}: the touched doc was adopted at its copy-in, not here; {}",
                    text(&out),
                );
            }
            let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
            assert!(
                at_head.contains(TASK_PROSE),
                "{what}: the task's prose landed"
            );
        }
    }
}

/// One `doc` write leaf as a task's first touch of a committed doc.
struct Leaf {
    /// The leaf's name under `jigc doc` — its `VERB_KINDS` path's second word.
    leaf: &'static str,
    /// What distinguishes two rows of one leaf.
    arm: &'static str,
    workflow: &'static str,
    /// The home the leaf copies in.
    home: &'static str,
    argv: &'static [&'static str],
    stdin: Option<&'static str>,
}

/// Every `doc` write leaf — the six that stage a committed doc through `read_or_copy_in`
/// (`set-field` on both its arms) and the two that mint through the create gate.
const LEAVES: &[Leaf] = &[
    Leaf {
        leaf: "set-slot",
        arm: "",
        workflow: "single-task",
        home: "VISION.md",
        argv: &[
            "set-slot",
            "vision:vision#open-questions",
            "--from-file",
            "-",
        ],
        stdin: Some("A sharper question.\n"),
    },
    Leaf {
        leaf: "set-field",
        arm: "set",
        workflow: "single-task",
        home: ADR_HOME,
        argv: &[
            "set-field",
            "adr:single-node-cache#status",
            "--value",
            "accepted",
        ],
        stdin: None,
    },
    Leaf {
        leaf: "set-field",
        arm: "unset",
        workflow: "single-task",
        home: ADR_HOME,
        argv: &["set-field", "adr:single-node-cache#cites-code", "--unset"],
        stdin: None,
    },
    Leaf {
        leaf: "add-item",
        arm: "",
        workflow: "single-task",
        home: "docs/roadmap.md",
        argv: &[
            "add-item",
            "roadmap:roadmap#milestones",
            "--title",
            "M-Beta",
        ],
        stdin: None,
    },
    Leaf {
        leaf: "remove-item",
        arm: "",
        workflow: "single-task",
        home: "docs/roadmap.md",
        argv: &["remove-item", "roadmap:roadmap#milestones/m-alpha"],
        stdin: None,
    },
    Leaf {
        leaf: "retitle-item",
        arm: "",
        workflow: "single-task",
        home: "docs/roadmap.md",
        argv: &[
            "retitle-item",
            "roadmap:roadmap#milestones/m-alpha",
            "--title",
            "M-Alpha prime",
        ],
        stdin: None,
    },
    Leaf {
        leaf: "rename",
        arm: "",
        workflow: "single-task",
        home: ADR_HOME,
        // A retitle that keeps the slug: a committed identity's slug is frozen in-task.
        argv: &["rename", ADR, "--to", "Single-Node Cache"],
        stdin: None,
    },
    Leaf {
        leaf: "create",
        arm: "",
        workflow: "single-task",
        home: ADR_HOME,
        argv: &["create", "adr", "--title", "Single-node cache"],
        stdin: None,
    },
    Leaf {
        leaf: "author",
        arm: "",
        workflow: "single-task",
        home: ADR_HOME,
        argv: &["author", "adr", "--from-file", "-"],
        stdin: Some(
            "title: Single-node cache\n\
             sections:\n  \
             - id: decision\n    \
             set:\n      \
             decision: |-\n        \
             <<A single node, authored again.>>\n",
        ),
    },
];

/// **Every `doc` write leaf is a copy-in door.** The rows are fenced against
/// `cli::cli::VERB_KINDS`, so a ninth write leaf reddens here until someone says where it
/// stages a committed doc from; each row, run as a task's first touch of a doc with no key,
/// records that doc's raw-byte hash and says so on its ack.
#[test]
fn every_doc_write_leaf_records_the_baseline_on_its_first_touch() {
    let mut declared: Vec<&str> = cli::cli::VERB_KINDS
        .iter()
        .filter(|(path, kind)| {
            path.len() == 2 && path[0] == "doc" && *kind == cli::cli::VerbKind::Write
        })
        .map(|(path, _)| path[1])
        .collect();
    declared.sort_unstable();
    let mut rows: Vec<&str> = LEAVES.iter().map(|row| row.leaf).collect();
    rows.sort_unstable();
    rows.dedup();
    assert_eq!(
        rows, declared,
        "every `doc` write leaf has a row here — a write leaf that can stage a committed doc \
         is a copy-in door",
    );

    let base = baselined_corpus();
    for row in LEAVES {
        let what = format!("doc {} {}", row.leaf, row.arm);
        let corpus = base.copy_state();
        corpus.fresh_clone_shape();
        let raw = disk_hash(&corpus, row.home);
        let task = corpus.start_workflow(row.workflow, "touch one doc");
        let mut argv = vec!["doc"];
        argv.extend(row.argv);
        argv.extend(["--task", task.as_str(), "--format", "json"]);
        let out = corpus.jigc_stdin(&argv, row.stdin.unwrap_or(""));
        let ack: serde_json::Value = stdout_json(&out, &[0], &what);
        assert_eq!(
            recorded(&corpus, row.home),
            Some(raw),
            "{what}: the first touch records the raw-byte hash of `{}`; {}",
            row.home,
            text(&out),
        );
        assert_eq!(
            keyed(&findings(&ack), ADOPT, row.home).len(),
            1,
            "{what}: the ack states the adoption: {ack}",
        );
    }
}

/// **Cell 1 — the hash is of the raw bytes copied, never the canonicalized staged copy.**
/// The copy-in applies the first-touch canonicalization (here: a missing final newline is
/// added), so the staged copy differs from the file. Hashing the staged copy would make every
/// such doc read drifted from the moment it was copied in — and for a doc with no blob at the
/// pin, block its own task.
#[test]
fn the_recorded_hash_is_of_the_raw_bytes_not_the_canonicalized_copy() {
    let base = baselined_corpus();
    let kind = Kind::Location;
    for absent in [Absent::Clone, Absent::Untracked] {
        let what = format!("{absent:?}");
        let corpus = base.copy_state();
        // A doc whose raw bytes the first touch canonicalizes: no final newline.
        let path = corpus.repo().join(kind.home());
        let body = fs::read_to_string(&path).expect("read the ADR");
        let raw = body.trim_end_matches('\n').to_owned();
        assert_ne!(
            raw, body,
            "the premise: the committed ADR ends in a newline"
        );
        fs::write(&path, &raw).expect("strip the final newline");
        corpus.git(&["add", "--", kind.home()]);
        corpus.git(&["commit", "-q", "-m", "docs: no final newline"]);
        absent.apply(&corpus, kind);

        let task = Door::Task.mint(&corpus);
        kind.first_write(&corpus, &task);
        assert_eq!(
            recorded(&corpus, kind.home()),
            Some(hash_bytes(raw.as_bytes())),
            "{what}: the recorded hash is the raw file's",
        );
        let staged = corpus
            .repo()
            .join(".jigc/tasks")
            .join(&task)
            .join("docs")
            .join(format!("{}.md", kind.address()));
        assert!(
            fs::read_to_string(&staged)
                .expect("read the staged copy")
                .ends_with('\n'),
            "{what}: the premise — the staged copy was canonicalized",
        );
        let out = Door::Task.finalize(&corpus, &task);
        let envelope: serde_json::Value = stdout_json(&out, &[0], &what);
        assert!(
            findings(&envelope)
                .iter()
                .all(|f| f["severity"] != "blocking"),
            "{what}: a doc the copy-in canonicalized does not read as drifted; {}",
            text(&out),
        );
    }
}

/// Hold the save lock on the corpus's file-state record from a second fd. Released on drop.
fn hold_save_lock(corpus: &TrialCorpus) -> fs::File {
    let record = FileStateRecord::path_in(&corpus.repo().join(".jigc"));
    fs::create_dir_all(record.parent().expect("the state dir")).expect("mk the state dir");
    let holder = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(engine::state::lock_sibling(&record))
        .expect("open the lock sibling");
    holder.lock().expect("hold the save lock");
    holder
}

/// **Cell 3 — a save-lock timeout fails the write door closed**, at both copy-in sites. A
/// baseline that cannot be recorded is a write that does not happen: the verb exits non-zero
/// naming the lock and the retry, nothing is staged and no key is written — never a staged
/// doc with no record. Released, the same argv lands and records.
#[test]
fn a_save_lock_timeout_fails_the_write_door_closed_at_both_copy_in_sites() {
    let base = baselined_corpus();
    // (leaf argv, the staged doc it would create, the home it copies in)
    let sites: [(&[&str], &str, &str); 2] = [
        (
            &[
                "doc",
                "set-slot",
                "vision:vision#open-questions",
                "--from-file",
                "-",
            ],
            "vision:vision",
            "VISION.md",
        ),
        (
            &["doc", "create", "adr", "--title", "Single-node cache"],
            ADR,
            ADR_HOME,
        ),
    ];
    let corpus = Arc::new(base.copy_state());
    corpus.fresh_clone_shape();
    let tasks: Vec<String> = ["first site", "second site"]
        .iter()
        .map(|intent| corpus.start_workflow("single-task", intent))
        .collect();

    let holder = hold_save_lock(&corpus);
    let started = std::time::Instant::now();
    let workers: Vec<_> = sites
        .iter()
        .zip(&tasks)
        .map(|((argv, _, _), task)| {
            let corpus = Arc::clone(&corpus);
            let mut argv: Vec<String> = argv.iter().map(|word| (*word).to_owned()).collect();
            argv.extend(["--task".to_owned(), task.clone()]);
            std::thread::spawn(move || {
                let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
                corpus.jigc_stdin(&argv, "A sharper question.\n")
            })
        })
        .collect();
    let outs: Vec<Output> = workers
        .into_iter()
        .map(|worker| worker.join().expect("the writer thread"))
        .collect();
    let waited = started.elapsed();
    drop(holder);

    assert!(
        waited >= engine::state::SAVE_LOCK_BUDGET * 9 / 10,
        "the writes waited the budget out ({waited:?})",
    );
    for (((argv, staged, home), task), out) in sites.iter().zip(&tasks).zip(&outs) {
        let what = format!("`jigc {}` behind a held save lock", argv.join(" "));
        assert_ne!(
            out.status.code(),
            Some(0),
            "{what} must fail; {}",
            text(out)
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("file-state.json.lock") && stderr.contains("retry"),
            "{what}: the error names the lock and the retry; {}",
            text(out),
        );
        assert!(
            !corpus
                .repo()
                .join(".jigc/tasks")
                .join(task)
                .join("docs")
                .join(format!("{staged}.md"))
                .exists(),
            "{what}: nothing is staged",
        );
        assert_eq!(recorded(&corpus, home), None, "{what}: no key is written");
    }

    // The route — retry once the holder has finished — works as printed.
    for ((argv, _, home), task) in sites.iter().zip(&tasks) {
        let mut argv: Vec<&str> = argv.to_vec();
        argv.extend(["--task", task.as_str()]);
        let out = corpus.jigc_stdin(&argv, "A sharper question.\n");
        assert_eq!(
            out.status.code(),
            Some(0),
            "the retry lands; {}",
            text(&out)
        );
        assert_eq!(
            recorded(&corpus, home),
            Some(disk_hash(&corpus, home)),
            "the retry records `{home}`",
        );
    }
}

/// **Cell 3b — an unreadable record refuses naming the cache, and its route runs** (the
/// rc.24 fix pass's completion audit). The copy-in door reads the record before it stages,
/// so a record whose bytes do not parse refuses every first write — where `1.0.0-rc.24`,
/// which did not read it there, wrote. The refusal said *"could not read committed
/// `vision:vision#open-questions`: key must be a string at line 1 column 3"*: the wrong
/// file, and no route.
///
/// The error is the record's own, so the class is every reader of it, not the one door the
/// audit drove: both copy-in sites, the two task doors, the store sweep, `ingest` and
/// `unmanage`. Each exits non-zero naming `.jigc/state/file-state.json` and the same route;
/// the write doors stage nothing; no reader "heals" the file on the way past. Then the
/// route is lifted out of the emitted bytes and run through a real shell, from outside the
/// repository — it is an absolute path — and the refused write lands and records.
#[test]
fn an_unreadable_record_refuses_naming_the_cache_and_its_route_runs() {
    const CACHE: &str = ".jigc/state/file-state.json";
    const GARBAGE: &str = "{ not json";
    let corpus = baselined_corpus();
    let task = mint_task(&corpus, "sharpen the questions");
    let record = FileStateRecord::path_in(&corpus.repo().join(".jigc"));
    fs::write(&record, GARBAGE).expect("corrupt the record");

    let set_slot = [
        "doc",
        "set-slot",
        "vision:vision#open-questions",
        "--from-file",
        "-",
        "--task",
        task.as_str(),
    ];
    let doors: [(&str, &[&str]); 7] = [
        ("the edit verbs' copy-in", &set_slot),
        (
            "`doc create`'s copy-in",
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Single-node cache",
                "--task",
                task.as_str(),
            ],
        ),
        ("task validate", &["task", "validate", task.as_str()]),
        ("task finalize", &["task", "finalize", task.as_str()]),
        ("the store sweep", &["validate"]),
        ("ingest", &["ingest"]),
        ("unmanage", &["unmanage", "VISION.md"]),
    ];
    let mut route = None;
    for (what, argv) in doors {
        let out = corpus.jigc_stdin(argv, "A sharper question.\n");
        assert_ne!(
            out.status.code(),
            Some(0),
            "{what} must refuse; {}",
            text(&out)
        );
        let said = text(&out);
        assert!(
            said.contains(&format!("the file-state record `{CACHE}`")),
            "{what}: the refusal names the cache; {said}",
        );
        let emitted = said
            .split_once("route: `")
            .and_then(|(_, rest)| rest.split_once('`'))
            .map(|(command, _)| command.to_owned())
            .unwrap_or_else(|| panic!("{what}: the refusal carries a route; {said}"));
        assert!(emitted.starts_with("rm "), "{what}: {emitted}");
        if let Some(first) = &route {
            assert_eq!(&emitted, first, "{what}: every reader prints one route");
        }
        route = Some(emitted);
        assert_eq!(
            fs::read_to_string(&record).expect("the record is still there"),
            GARBAGE,
            "{what}: a reader does not rewrite the record on its way past",
        );
    }
    let docs = corpus.repo().join(".jigc/tasks").join(&task).join("docs");
    for staged in ["vision:vision.md", &format!("{ADR}.md")] {
        assert!(
            !docs.join(staged).exists(),
            "a refused write stages nothing ({staged})",
        );
    }

    // The route, as printed, through a real shell — from outside the repository.
    let command = route.expect("a route was printed");
    let ran = std::process::Command::new("sh")
        .args(["-c", &command])
        .current_dir(std::env::temp_dir())
        .output()
        .expect("spawn sh");
    assert!(
        ran.status.success(),
        "`{command}` runs as printed; {}",
        text(&ran)
    );
    assert!(!record.exists(), "and it removed the record it names");

    let out = corpus.jigc_stdin(&set_slot, "A sharper question.\n");
    assert_eq!(
        out.status.code(),
        Some(0),
        "the refused write lands; {}",
        text(&out)
    );
    assert_eq!(
        recorded(&corpus, "VISION.md"),
        Some(disk_hash(&corpus, "VISION.md")),
        "and records the doc's baseline, in a record that reads again",
    );
}

/// **Cell 4 — concurrent first writes lose no key.** A fan-out's sub-agents now write one
/// `state/file-state.json` at their first touch, a population that file never had. Seven
/// processes released together — four on distinct docs, three more on one of them — all
/// succeed, and every doc's key is its raw-byte hash.
#[test]
fn concurrent_first_writes_all_record_their_baselines() {
    let base = baselined_corpus();
    let corpus = Arc::new(base.copy_state());
    corpus.fresh_clone_shape();
    // (home, argv, stdin)
    let writes: [(&str, &[&str], &str); 7] = [
        (
            "VISION.md",
            &[
                "doc",
                "set-slot",
                "vision:vision#open-questions",
                "--from-file",
                "-",
            ],
            "One.\n",
        ),
        (
            "VISION.md",
            &[
                "doc",
                "set-slot",
                "vision:vision#invariants",
                "--from-file",
                "-",
            ],
            "Two.\n",
        ),
        (
            "VISION.md",
            &[
                "doc",
                "set-slot",
                "vision:vision#thesis",
                "--from-file",
                "-",
            ],
            "Three.\n",
        ),
        (
            "VISION.md",
            &[
                "doc",
                "set-slot",
                "vision:vision#open-questions",
                "--from-file",
                "-",
            ],
            "Four.\n",
        ),
        (
            ADR_HOME,
            &[
                "doc",
                "set-slot",
                "adr:single-node-cache#decision",
                "--from-file",
                "-",
            ],
            "Five.\n",
        ),
        (
            "docs/roadmap.md",
            &[
                "doc",
                "add-item",
                "roadmap:roadmap#milestones",
                "--title",
                "M-Beta",
            ],
            "",
        ),
        (
            "CHANGELOG.md",
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "fixed",
            ],
            "",
        ),
    ];
    let expected: std::collections::BTreeMap<&str, String> = writes
        .iter()
        .map(|(home, _, _)| (*home, disk_hash(&corpus, home)))
        .collect();
    let tasks: Vec<String> = (0..writes.len())
        .map(|n| corpus.start_workflow("single-task", &format!("concurrent writer {n}")))
        .collect();

    let barrier = Arc::new(Barrier::new(writes.len()));
    let workers: Vec<_> = writes
        .iter()
        .zip(&tasks)
        .map(|((_, argv, stdin), task)| {
            let corpus = Arc::clone(&corpus);
            let barrier = Arc::clone(&barrier);
            let mut argv: Vec<String> = argv.iter().map(|word| (*word).to_owned()).collect();
            argv.extend(["--task".to_owned(), task.clone()]);
            let stdin = (*stdin).to_owned();
            std::thread::spawn(move || {
                let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
                barrier.wait();
                corpus.jigc_stdin(&argv, &stdin)
            })
        })
        .collect();
    for (worker, (_, argv, _)) in workers.into_iter().zip(&writes) {
        let out = worker.join().expect("the writer thread");
        assert_eq!(
            out.status.code(),
            Some(0),
            "`jigc {}` lands under contention; {}",
            argv.join(" "),
            text(&out),
        );
    }
    for (home, hash) in &expected {
        assert_eq!(
            recorded(&corpus, home).as_ref(),
            Some(hash),
            "`{home}` is recorded at its raw-byte hash — no concurrent writer's key was lost",
        );
    }
}

/// **Cell 5 — a non-conformant doc is never baselined, and never overwritten.** A hand edit
/// that breaks conformance (a malformed `date`) and adds a line:
///
/// - made **after** the first write, it blocks the door on the conflict — the file is not
///   what the task copied in;
/// - made **before** it, it is carried into the task like any edit before the first write.
///   The copy-in records no baseline (the per-path rule), but the task's own witness names
///   the broken bytes, so the door does not conflict — and the staged copy, which carries
///   the break, fails the task's own conformance gate with a route that repairs it **in the
///   staged copy, through the CLI**. Run as printed, the route lands the task with the hand
///   line in it. Nothing told the reader to edit the file, and nothing was written over.
///
/// The crossing cell — a doc git has never committed that does not conform when it is copied
/// in — is [`an_untracked_draft_that_does_not_conform_is_never_promoted_over_a_hand_edit`].
#[test]
fn a_non_conformant_doc_is_never_baselined_and_never_overwritten() {
    let base = baselined_corpus();
    let kind = Kind::Location;
    let break_it = |corpus: &TrialCorpus| {
        let path = corpus.repo().join(kind.home());
        let body = fs::read_to_string(&path).expect("read the ADR");
        let date = body
            .lines()
            .find(|line| line.starts_with("date:"))
            .unwrap_or_else(|| panic!("the ADR carries a date field; got:\n{body}"))
            .to_owned();
        let broken = body.replacen(&date, "date: 2026/13/01", 1);
        assert_ne!(body, broken);
        fs::write(&path, format!("{broken}\n{HAND}\n")).expect("break the doc");
    };

    // After the first write: the conflict.
    let corpus = base.copy_state();
    corpus.fresh_clone_shape();
    let task = Door::Task.mint(&corpus);
    kind.first_write(&corpus, &task);
    break_it(&corpus);
    let disk = read(&corpus.repo(), kind.home());
    let head_before = head(&corpus);
    let out = Door::Task.finalize(&corpus, &task);
    assert_blocked(
        &corpus,
        &out,
        kind.home(),
        &head_before,
        &disk,
        "broken after the first write",
    );

    // Before the first write: carried, and repaired in the staged copy.
    let what = "broken before the first write";
    let corpus = base.copy_state();
    corpus.fresh_clone_shape();
    let task = Door::Task.mint(&corpus);
    break_it(&corpus);
    let ack = kind.first_write(&corpus, &task);
    assert_eq!(
        recorded(&corpus, kind.home()),
        None,
        "{what}: a non-conformant doc is not baselined at the copy-in",
    );
    assert!(
        keyed(&findings(&ack), ADOPT, kind.home()).is_empty(),
        "{what}: and the write claims no adoption: {ack}",
    );
    let disk = read(&corpus.repo(), kind.home());
    let head_before = head(&corpus);
    let out = Door::Task.finalize(&corpus, &task);
    let envelope: serde_json::Value = stdout_json(&out, &[3], what);
    let rows = findings(&envelope);
    assert!(
        keyed(&rows, CONFLICT, kind.home()).is_empty(),
        "{what}: the file is what the task copied in — no conflict; {}",
        text(&out),
    );
    let blocking: Vec<&serde_json::Value> = rows
        .iter()
        .filter(|f| f["severity"] == "blocking")
        .collect();
    assert_eq!(blocking.len(), 1, "{what}: one block; {}", text(&out));
    assert_eq!(
        blocking[0]["key"]["code"],
        "schema-conformance.field-value-conformant",
        "{what}: the staged copy's own conformance; {}",
        text(&out),
    );
    assert!(
        !text(&out).contains("yours to hand-edit"),
        "{what}: nothing tells the reader to edit a file the task holds; {}",
        text(&out),
    );
    assert_eq!(head(&corpus), head_before, "{what}: nothing is committed");
    assert_eq!(
        read(&corpus.repo(), kind.home()),
        disk,
        "{what}: the file stays"
    );
    // The route, as printed, with a value.
    let route = blocking[0]["route"].as_str().expect("the block routes");
    let spans = jigc_spans(route);
    assert_eq!(spans.len(), 1, "{what}: one `jigc` span in: {route}");
    let ran = run_emitted(&corpus, &spans[0].replace("<value>", "2026-10-05"));
    assert_eq!(ran.status.code(), Some(0), "{what}: {}", text(&ran));
    let landed = Door::Task.finalize(&corpus, &task);
    assert_eq!(landed.status.code(), Some(0), "{what}: {}", text(&landed));
    let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
    assert!(
        at_head.contains(HAND) && at_head.contains(TASK_PROSE) && at_head.contains("2026-10-05"),
        "{what}: the carried line, the task's prose and the repair landed; got:\n{at_head}",
    );
}

/// **Cell 6 — the adoption is stated where it happens, once.** The write that records the
/// baseline carries `file-state.baseline-adopt` on both surfaces; the task's next write, and
/// a first write in a checkout that already holds the key, carry none.
#[test]
fn the_adoption_is_stated_on_the_write_that_made_it_and_only_there() {
    let base = baselined_corpus();
    let kind = Kind::Placement;

    // Text surface, key absent.
    let corpus = base.copy_state();
    corpus.fresh_clone_shape();
    let task = Door::Task.mint(&corpus);
    let line = corpus.jigc_stdin_ok(
        &[
            "doc",
            "set-slot",
            &kind.slot(),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        &format!("{TASK_PROSE}\n"),
    );
    assert!(
        line.contains(&format!("{ADOPT} — baseline adopted: `{}`", kind.home())),
        "the agent-text ack states the adoption; got:\n{line}",
    );
    // The same task's next write reads the staged copy: nothing is adopted.
    let again = corpus.jigc_stdin(
        &[
            "doc",
            "set-slot",
            "vision:vision#invariants",
            "--from-file",
            "-",
            "--task",
            &task,
            "--format",
            "json",
        ],
        "Still true.\n",
    );
    let again: serde_json::Value = stdout_json(&again, &[0], "the second write");
    assert_eq!(again["copied_in"], false, "{again}");
    assert!(
        keyed(&findings(&again), ADOPT, kind.home()).is_empty(),
        "{again}"
    );

    // Key present: a first write adopts nothing and says nothing.
    let held = base.copy_state();
    let before = recorded(&held, kind.home());
    assert!(before.is_some(), "the premise: the corpus is baselined");
    let task = Door::Task.mint(&held);
    let ack = kind.first_write(&held, &task);
    assert!(
        keyed(&findings(&ack), ADOPT, kind.home()).is_empty(),
        "{ack}"
    );
    assert_eq!(
        recorded(&held, kind.home()),
        before,
        "the held key is untouched"
    );
}

/// The `file-state.*` rows a store sweep reports at `home`, as `(code, severity)`.
fn store_rows(corpus: &TrialCorpus, home: &str) -> (Option<i32>, Vec<(String, String)>) {
    let out = corpus.jigc(&["validate", "--format", "json"]);
    let envelope: serde_json::Value =
        serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}; {}", text(&out)));
    let rows = findings(&envelope)
        .iter()
        .filter(|f| {
            f["key"]["target"] == home
                && f["key"]["code"]
                    .as_str()
                    .is_some_and(|code| code.starts_with("file-state."))
        })
        .map(|f| {
            (
                f["key"]["code"].as_str().unwrap_or_default().to_owned(),
                f["severity"].as_str().unwrap_or_default().to_owned(),
            )
        })
        .collect();
    (out.status.code(), rows)
}

/// **Cell 7 — after a copy-in, a clone reads at store scope as any baselined checkout
/// does.** Before the first write the store sweep reports the doc `file-state.un-baselined`
/// (advisory); after it, nothing; and a hand edit then reads as drift — the same row, at the
/// same severity and exit, a checkout that always held the key reports for the same edit.
#[test]
fn after_a_copy_in_the_store_sweep_reports_drift_as_a_baselined_checkout_does() {
    let base = baselined_corpus();
    let kind = Kind::Placement;

    let control = base.copy_state();
    let task = Door::Task.mint(&control);
    kind.first_write(&control, &task);
    kind.hand_edit(&control);
    let control_rows = store_rows(&control, kind.home());
    assert!(
        control_rows
            .1
            .contains(&("file-state.hash-matches".to_owned(), "blocking".to_owned())),
        "the premise: a baselined checkout reports the hand edit as drift; got {control_rows:?}",
    );

    let clone = base.copy_state();
    clone.fresh_clone_shape();
    assert_eq!(
        store_rows(&clone, kind.home()).1,
        vec![("file-state.un-baselined".to_owned(), "advisory".to_owned())],
        "a clone's store sweep reports the doc un-baselined before any write",
    );
    let task = Door::Task.mint(&clone);
    kind.first_write(&clone, &task);
    assert_eq!(
        store_rows(&clone, kind.home()).1,
        Vec::<(String, String)>::new(),
        "the un-baselined advisory's route came true: the author write baselined the doc",
    );
    kind.hand_edit(&clone);
    assert_eq!(
        store_rows(&clone, kind.home()),
        control_rows,
        "after the copy-in the clone reports the hand edit exactly as the control does",
    );
}

/// **Cell 8 — the record is per path, not per task.** Two open tasks hold one doc; the first
/// one's copy-in left no record (an older binary, or the key was dropped mid-task); a hand
/// edit falls between the two copy-ins. The second task must **not** record the edited bytes
/// — the first would then read in-sync and promote over the edit — so the path stays
/// `UNKNOWN` and the base-pin backstop decides: the first task blocks and the hand line
/// survives.
#[test]
fn a_second_tasks_copy_in_records_nothing_while_another_task_holds_the_doc_unrecorded() {
    let base = baselined_corpus();
    for kind in Kind::ALL {
        let what = format!("{kind:?}");
        let corpus = base.copy_state();
        corpus.fresh_clone_shape();
        let first = Door::Task.mint(&corpus);
        kind.first_write(&corpus, &first);
        forget(&corpus, kind.home());

        kind.hand_edit(&corpus);
        let edited = read(&corpus.repo(), kind.home());

        let second = corpus.start_workflow("single-task", "a second task on the same doc");
        let ack = kind.first_write(&corpus, &second);
        assert_eq!(
            recorded(&corpus, kind.home()),
            None,
            "{what}: the second copy-in records nothing while the first task holds the doc \
             with no record",
        );
        assert!(
            keyed(&findings(&ack), ADOPT, kind.home()).is_empty(),
            "{what}: and claims no adoption: {ack}",
        );

        let head_before = head(&corpus);
        let out = Door::Task.finalize(&corpus, &first);
        assert_blocked(
            &corpus,
            &out,
            kind.home(),
            &head_before,
            &edited,
            &format!("{what}: the first task"),
        );
        assert!(edited.contains(HAND), "{what}: the hand line survives");
    }
}

/// **Cell 9 — a task with a staged doc and no record.** The key is lost after the copy-in
/// (a deleted cache, an `unmanage`), under each order of the hand edit, at both doors:
///
/// - **the task holds its witness** → it decides. No edit lands; an edit *after* the copy-in
///   blocks; an edit *before* it is what the task copied in, so it is carried and lands —
///   the cell the previous round blocked, because the pin could not say which side of the
///   copy-in an edit fell on. The witness can.
/// - **an area the previous format minted** (no `copied-in` member — every task a binary
///   older than the witness copied in) → the base pin decides, as it did: bytes at the pin
///   land, bytes off it block in either order.
///
/// And `jigc unmanage` after a block does not switch the guard off, in either area.
#[test]
fn a_staged_doc_with_no_record_is_decided_by_the_witness_or_else_the_base_pin() {
    let base = baselined_corpus();
    #[derive(Clone, Copy, Debug)]
    enum Edit {
        None,
        BeforeTheCopyIn,
        AfterTheCopyIn,
    }
    for previous_format in [false, true] {
        for door in Door::ALL {
            for kind in Kind::ALL {
                // The previous format's arms are the pin's, unchanged and pinned cell by
                // cell at the engine (`unknown_and_touched_is_decided_by_the_base_pin`);
                // one doctype kind through both doors is the binary-level statement.
                if previous_format && kind == Kind::Location {
                    continue;
                }
                for edit in [Edit::None, Edit::BeforeTheCopyIn, Edit::AfterTheCopyIn] {
                    // …and at the join, the one cell whose outcome the format changes.
                    if previous_format
                        && door == Door::Milestone
                        && !matches!(edit, Edit::BeforeTheCopyIn)
                    {
                        continue;
                    }
                    let what =
                        format!("previous format: {previous_format}/{door:?}/{kind:?}/{edit:?}");
                    let corpus = base.copy_state();
                    let task = door.mint(&corpus);
                    if matches!(edit, Edit::BeforeTheCopyIn) {
                        kind.hand_edit(&corpus);
                    }
                    kind.first_write(&corpus, &task);
                    if matches!(edit, Edit::AfterTheCopyIn) {
                        kind.hand_edit(&corpus);
                    }
                    if previous_format {
                        strip_witness(&corpus, &task);
                    }
                    forget(&corpus, kind.home());
                    let disk = read(&corpus.repo(), kind.home());
                    let head_before = head(&corpus);
                    let out = door.finalize(&corpus, &task);
                    let lands = match edit {
                        Edit::None => true,
                        Edit::BeforeTheCopyIn => !previous_format,
                        Edit::AfterTheCopyIn => false,
                    };
                    if lands {
                        assert_eq!(out.status.code(), Some(0), "{what}: {}", text(&out));
                        let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
                        assert_eq!(
                            at_head.contains(HAND),
                            matches!(edit, Edit::BeforeTheCopyIn),
                            "{what}: an edit before the copy-in is carried; got:\n{at_head}",
                        );
                        assert!(at_head.contains(TASK_PROSE), "{what}: got:\n{at_head}");
                    } else {
                        assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
                    }
                }
            }
        }
    }

    // `unmanage` after the block: the key is dropped, and the same finalize blocks again.
    for previous_format in [false, true] {
        for kind in Kind::ALL {
            if previous_format && kind == Kind::Location {
                continue;
            }
            let what = format!("previous format: {previous_format}/{kind:?}: unmanage");
            let corpus = base.copy_state();
            let task = Door::Task.mint(&corpus);
            kind.first_write(&corpus, &task);
            if previous_format {
                strip_witness(&corpus, &task);
            }
            kind.hand_edit(&corpus);
            let disk = read(&corpus.repo(), kind.home());
            let head_before = head(&corpus);
            let out = Door::Task.finalize(&corpus, &task);
            assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
            corpus.jigc_ok(&["unmanage", kind.home()]);
            assert_eq!(recorded(&corpus, kind.home()), None, "{what}");
            let out = Door::Task.finalize(&corpus, &task);
            assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
        }
    }
}

/// **The crossing cells** (the completion audit's reconcile-baseline F7) — the cells the
/// suite iterated *around*, which is where the audit drove three exit-0 losses of a hand
/// edit made after a task's first write. Each crosses two axes the suite already had:
///
/// | cell | crosses | the finding |
/// |---|---|---|
/// | a second jigc writer of the key, between the edit and the holder's door | the record's writers × the order of the edit | F1 |
/// | an untracked doc that does not conform | no blob at the pin × never baselined | F2 |
/// | an untracked doc whose key is lost after the copy-in | no blob at the pin × key lost | e2e F5 |
/// | two open tasks on one untracked doc | no blob at the pin × staged elsewhere | F2's third cell |
///
/// They are one test because they are one statement — *for a doc a task holds, what that
/// task copied in decides* — over one fixture; each cell is its own function below, with
/// what was driven before the witness existed.
#[test]
fn a_doc_a_task_holds_is_decided_by_what_that_task_copied_in() {
    let base = baselined_corpus();
    another_jigc_writer_of_the_record_never_unblocks_a_hand_edit(&base);
    an_untracked_draft_that_does_not_conform_is_never_promoted_over_a_hand_edit(&base);
    an_untracked_doc_whose_key_is_lost_after_the_copy_in_still_blocks_a_hand_edit(&base);
    two_tasks_on_one_untracked_doc_are_each_decided_by_what_they_copied_in(&base);
}

/// A jigc writer, other than the task holding the doc, that moves the doc's `file-state` key
/// to the bytes on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mover {
    /// An unrelated task's landed `jigc task finalize`: its sweep absorbs the hand edit to a
    /// doc it never touched, and its post-commit persists the moved key.
    UnrelatedFinalize,
    /// `jigc ingest`, which absorbs an out-of-band edit into the baseline by design.
    Ingest,
}

impl Mover {
    const ALL: [Mover; 2] = [Mover::UnrelatedFinalize, Mover::Ingest];

    fn run(self, corpus: &TrialCorpus) {
        match self {
            Mover::UnrelatedFinalize => {
                let other = mint_task(corpus, "an unrelated code change");
                fs::write(corpus.repo().join("unrelated.rs"), "fn unrelated() {}\n")
                    .expect("write the unrelated file");
                corpus.git(&["add", "--", "unrelated.rs"]);
                let out = corpus.jigc(&["task", "finalize", &other]);
                assert_eq!(
                    out.status.code(),
                    Some(0),
                    "the unrelated task lands; {}",
                    text(&out),
                );
            }
            Mover::Ingest => {
                corpus.jigc_ok(&["ingest"]);
            }
        }
    }
}

/// **A second jigc writer between the hand edit and the holding task's door** (the
/// completion audit's reconcile-baseline F1).
///
/// The record is per path, and the copy-in is not its only writer: an unrelated task's
/// finalize sweeps the whole store and absorbs a conformant edit to a doc it never touched,
/// and `jigc ingest` does the same on purpose. Either one, run after the hand edit, moves
/// the key to the edited bytes — so the holding task read the doc `IN_SYNC` and its door
/// promoted the staged copy over the edit at exit 0, the line in no git object. Driven on
/// the previous round's build at both doors.
///
/// The holding task's own witness is untouched by either writer, so the door blocks. Then
/// the route's second exit, as printed: the edit reverted on disk, the door lands.
fn another_jigc_writer_of_the_record_never_unblocks_a_hand_edit(base: &TrialCorpus) {
    let kind = Kind::Placement;
    for door in Door::ALL {
        for mover in Mover::ALL {
            for clone in [false, true] {
                // The clone shape adds nothing the held key does not at the milestone door,
                // and an unrelated commit is not a mover there at all: it moves `HEAD` off
                // the milestone's base, which `finalize.base-mismatch` refuses first.
                if door == Door::Milestone && (clone || mover == Mover::UnrelatedFinalize) {
                    continue;
                }
                // With no key at the copy-in the copy-in records one, and from there the
                // two movers are the same cell; one of them carries the clone shape.
                if clone && mover == Mover::Ingest {
                    continue;
                }
                let what = format!("{door:?}/{mover:?}/clone: {clone}");
                let corpus = base.copy_state();
                if clone {
                    corpus.fresh_clone_shape();
                }
                let task = door.mint(&corpus);
                kind.first_write(&corpus, &task);
                kind.hand_edit(&corpus);
                let edited = read(&corpus.repo(), kind.home());

                mover.run(&corpus);
                assert_eq!(
                    recorded(&corpus, kind.home()),
                    Some(hash_bytes(edited.as_bytes())),
                    "{what}: the premise — the other writer moved the key to the edited bytes",
                );

                let head_before = head(&corpus);
                let out = door.finalize(&corpus, &task);
                assert_blocked(&corpus, &out, kind.home(), &head_before, &edited, &what);
                assert!(edited.contains(HAND), "{what}: the hand line survives");

                corpus.git(&["checkout", "--", kind.home()]);
                let landed = door.finalize(&corpus, &task);
                assert_eq!(landed.status.code(), Some(0), "{what}: {}", text(&landed));
                let at_head = corpus.git(&["show", &format!("HEAD:{}", kind.home())]);
                assert!(
                    at_head.contains(TASK_PROSE) && !at_head.contains(HAND),
                    "{what}: the landed doc carries the task's prose; got:\n{at_head}",
                );
            }
        }
    }
}

/// **An untracked doc that does not conform** (the completion audit's reconcile-baseline F2
/// — the crossing cell cell 5 used to state as open).
///
/// A hand-started draft with an empty required slot is the plainest way to meet a doc with
/// neither a record nor a blob at any pin: it is never baselined (it does not conform) and
/// git has never committed it. A task finishes it through the CLI; a line is then added to
/// the file by hand. Driven on the previous round's build, from one task: the door printed
/// the advisory *"fix the file … this is the one case a managed file is yours to hand-edit"*
/// and then promoted the staged copy over the line, exit 0, the line in no git object.
///
/// The task's witness names the draft as it was copied in, so the door blocks, at both
/// doors, and both exits run: the emitted discard leaves the draft and its line on disk, and
/// with the line taken back out the door lands the finished draft. And the draft nobody
/// edited lands with no sentence inviting an edit to it — `must not refuse` beside
/// `must refuse`.
fn an_untracked_draft_that_does_not_conform_is_never_promoted_over_a_hand_edit(base: &TrialCorpus) {
    for door in Door::ALL {
        for edited in [true, false] {
            let what = format!("{door:?}/hand edit after the copy-in: {edited}");
            let corpus = base.copy_state();
            let draft = Draft::write(&corpus, false);
            let task = door.mint(&corpus);
            let ack = write_slot(&corpus, &task, &Draft::slot(), TASK_PROSE);
            assert_eq!(ack["copied_in"], true, "{what}: the premise: {ack}");
            assert_eq!(
                recorded(&corpus, Draft::HOME),
                None,
                "{what}: the premise — a draft that does not conform is not baselined",
            );
            assert!(
                witness(&corpus, &task)[Draft::ADDRESS]["bytes"] == hash_bytes(draft.as_bytes()),
                "{what}: the task records what it copied in: {}",
                witness(&corpus, &task),
            );

            if !edited {
                let out = door.finalize(&corpus, &task);
                assert_eq!(out.status.code(), Some(0), "{what}: {}", text(&out));
                assert!(
                    !text(&out).contains("yours to hand-edit"),
                    "{what}: no door calls a doc the task holds the reader's to edit; {}",
                    text(&out),
                );
                let at_head = corpus.git(&["show", &format!("HEAD:{}", Draft::HOME)]);
                assert!(at_head.contains(TASK_PROSE), "{what}: got:\n{at_head}");
                continue;
            }

            let on_disk = Draft::hand_edit(&corpus);
            let head_before = head(&corpus);
            let out = door.finalize(&corpus, &task);
            let conflict =
                assert_blocked(&corpus, &out, Draft::HOME, &head_before, &on_disk, &what);
            assert!(
                !text(&out).contains("yours to hand-edit"),
                "{what}: and never the sanction over a doc it then refuses; {}",
                text(&out),
            );
            let route = conflict["route"].as_str().expect("the conflict routes");

            // Exit one, on a copy: the emitted `jigc …` command, run as printed.
            let spans = jigc_spans(route);
            assert_eq!(spans.len(), 1, "{what}: one `jigc` span in: {route}");
            let aside = corpus.copy_state();
            let ran = run_emitted(&aside, spans[0]);
            assert_eq!(ran.status.code(), Some(0), "{what}: {}", text(&ran));
            assert_eq!(
                read(&aside.repo(), Draft::HOME),
                on_disk,
                "{what}: the line stays"
            );

            // Exit two: the edit taken back out, and the door lands the finished draft.
            fs::write(corpus.repo().join(Draft::HOME), &draft).expect("revert the hand edit");
            let landed = door.finalize(&corpus, &task);
            assert_eq!(landed.status.code(), Some(0), "{what}: {}", text(&landed));
            let at_head = corpus.git(&["show", &format!("HEAD:{}", Draft::HOME)]);
            assert!(
                at_head.contains(TASK_PROSE) && !at_head.contains(HAND),
                "{what}: got:\n{at_head}",
            );
        }
    }
}

/// **An untracked doc whose key is lost after the copy-in** (the e2e audit's F5 — the
/// declared no-blob cell).
///
/// The doc conforms, so its copy-in records the baseline; the cache is then removed. With
/// no record and no blob at the pin the previous round's door adopted whatever was on disk
/// and promoted over a hand line at exit 0. Both copy-in sites are driven — an edit leaf,
/// and `doc create` over the occupied home — because they are the two writers of the
/// witness. Untouched, the same sequence lands.
fn an_untracked_doc_whose_key_is_lost_after_the_copy_in_still_blocks_a_hand_edit(
    base: &TrialCorpus,
) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Site {
        EditLeaf,
        Create,
    }
    for (door, site) in [
        (Door::Task, Site::EditLeaf),
        (Door::Task, Site::Create),
        (Door::Milestone, Site::EditLeaf),
    ] {
        for edited in [true, false] {
            // One untouched control: the landing does not depend on the site or the door.
            if !edited && (door, site) != (Door::Task, Site::EditLeaf) {
                continue;
            }
            let what = format!("{door:?}/{site:?}/edited: {edited}");
            let corpus = base.copy_state();
            Draft::write(&corpus, true);
            let task = door.mint(&corpus);
            if site == Site::Create {
                let ack = corpus.jigc_ok(&[
                    "doc",
                    "create",
                    "adr",
                    "--title",
                    "Draft queue",
                    "--task",
                    &task,
                ]);
                assert!(
                    ack.contains("already existed — copied in for update"),
                    "{what}: the premise — `doc create` copies the occupant in: {ack}",
                );
            }
            write_slot(&corpus, &task, &Draft::slot(), TASK_PROSE);
            assert!(
                recorded(&corpus, Draft::HOME).is_some(),
                "{what}: the premise — a conformant draft is baselined at its copy-in",
            );
            assert!(
                witness(&corpus, &task)[Draft::ADDRESS]["bytes"].is_string(),
                "{what}: this copy-in site records the witness: {}",
                witness(&corpus, &task),
            );
            corpus.fresh_clone_shape();
            assert_eq!(
                recorded(&corpus, Draft::HOME),
                None,
                "{what}: the key is gone"
            );

            let head_before = head(&corpus);
            if edited {
                let on_disk = Draft::hand_edit(&corpus);
                let out = door.finalize(&corpus, &task);
                assert_blocked(&corpus, &out, Draft::HOME, &head_before, &on_disk, &what);
            } else {
                let out = door.finalize(&corpus, &task);
                assert_eq!(out.status.code(), Some(0), "{what}: {}", text(&out));
            }
        }
    }
}

/// **Two open tasks on one untracked doc** — the previous round's two-tasks cell (cell 8),
/// on the doc where neither the record nor the pin could answer for the second task.
///
/// The record is per path, so with the key dropped a second task's copy-in records nothing
/// (cell 8), and with no blob at the pin nothing was left to decide either task: a line
/// added after both copy-ins was promoted over by whichever task finalized, exit 0. Each
/// task now holds its own witness, so each is decided by what *it* copied in:
///
/// - a hand edit **after both** copy-ins blocks both tasks;
/// - a hand edit **between** them blocks the first (it never saw the line) and lands the
///   second, which copied the line in and carries it.
fn two_tasks_on_one_untracked_doc_are_each_decided_by_what_they_copied_in(base: &TrialCorpus) {
    for between in [false, true] {
        let what = format!("the hand edit between the two copy-ins: {between}");
        let corpus = base.copy_state();
        Draft::write(&corpus, true);
        let first = Door::Task.mint(&corpus);
        write_slot(
            &corpus,
            &first,
            &Draft::slot(),
            "FIRST-TASK its consequences.",
        );
        // The first task's key is dropped, so the second copy-in records nothing.
        corpus.jigc_ok(&["unmanage", Draft::HOME]);
        if between {
            Draft::hand_edit(&corpus);
        }
        let second = mint_task(&corpus, "a second task on the same draft");
        let ack = write_slot(
            &corpus,
            &second,
            &Draft::slot(),
            "SECOND-TASK its consequences.",
        );
        assert!(
            keyed(&findings(&ack), ADOPT, Draft::HOME).is_empty()
                && recorded(&corpus, Draft::HOME).is_none(),
            "{what}: the premise — the second copy-in records nothing: {ack}",
        );
        if !between {
            Draft::hand_edit(&corpus);
        }
        let on_disk = read(&corpus.repo(), Draft::HOME);
        let head_before = head(&corpus);

        let out = Door::Task.finalize(&corpus, &first);
        assert_blocked(
            &corpus,
            &out,
            Draft::HOME,
            &head_before,
            &on_disk,
            &format!("{what}: the first task"),
        );
        let out = Door::Task.finalize(&corpus, &second);
        if between {
            assert_eq!(
                out.status.code(),
                Some(0),
                "{what}: the second; {}",
                text(&out)
            );
            let at_head = corpus.git(&["show", &format!("HEAD:{}", Draft::HOME)]);
            assert!(
                at_head.contains(HAND) && at_head.contains("SECOND-TASK"),
                "{what}: the second task carries the line it copied in; got:\n{at_head}",
            );
        } else {
            assert_blocked(
                &corpus,
                &out,
                Draft::HOME,
                &head_before,
                &on_disk,
                &format!("{what}: the second task"),
            );
        }
    }
}

/// **The backstop asks git, so a checked-out form is not an edit.** In a checkout whose
/// working files are not the blobs' bytes — `eol=crlf` here; `core.autocrlf` and a smudge
/// filter are the same family — an untouched doc differs from its raw blob in every line
/// ending. Compared byte for byte, every staged doc with no record would block in such a
/// checkout.
#[test]
fn the_backstop_does_not_false_fire_in_a_crlf_checkout() {
    let base = baselined_corpus();
    let kind = Kind::Placement;
    let corpus = base.copy_state();
    fs::write(corpus.repo().join(".gitattributes"), "*.md text eol=crlf\n")
        .expect("write .gitattributes");
    corpus.git(&["add", "--", ".gitattributes"]);
    corpus.git(&["commit", "-q", "-m", "chore: check markdown out as CRLF"]);
    // Re-check the doc out under the attribute.
    fs::remove_file(corpus.repo().join(kind.home())).expect("remove the doc");
    corpus.git(&["checkout", "--", kind.home()]);
    let disk = fs::read(corpus.repo().join(kind.home())).expect("read the doc");
    assert!(
        disk.windows(2).any(|pair| pair == b"\r\n"),
        "the premise: the doc is checked out with CRLF line endings",
    );
    corpus.fresh_clone_shape();

    let task = Door::Task.mint(&corpus);
    let out = corpus.jigc_stdin(
        &[
            "doc",
            "set-slot",
            &kind.slot(),
            "--from-file",
            "-",
            "--task",
            &task,
        ],
        &format!("{TASK_PROSE}\n"),
    );
    assert_eq!(out.status.code(), Some(0), "{}", text(&out));
    // The key is dropped if the copy-in recorded one, so the door reaches the backstop.
    if recorded(&corpus, kind.home()).is_some() {
        forget(&corpus, kind.home());
    }
    let out = Door::Task.finalize(&corpus, &task);
    let envelope: serde_json::Value =
        serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}; {}", text(&out)));
    assert!(
        keyed(&findings(&envelope), CONFLICT, kind.home()).is_empty(),
        "an untouched doc in a CRLF checkout is at its pin — no conflict; {}",
        text(&out),
    );
}

/// **…and neither is the form jigc itself wrote.** A finalize promotes a created doc with
/// `\n` endings whatever the checkout converts to, and git calls that file unmodified.
/// Compared against the checked-out form alone, a doc jigc's own finalize landed would
/// block here, and the route's *revert the edit on disk* would have nothing to revert. (The
/// third form — the mixed file an in-place edit of a checked-out doc leaves — is the cell
/// below; enumerating forms is what missed it, which is why git is asked instead.)
#[test]
fn the_backstop_does_not_false_fire_on_jigcs_own_write_in_a_crlf_checkout() {
    let base = baselined_corpus();
    let kind = Kind::Placement;
    let corpus = base.copy_state();
    fs::write(corpus.repo().join(".gitattributes"), "*.md text eol=crlf\n")
        .expect("write .gitattributes");
    corpus.git(&["add", "--", ".gitattributes"]);
    corpus.git(&["commit", "-q", "-m", "chore: check markdown out as CRLF"]);
    // NOT re-checked out: the doc stays as the finalize that landed it wrote it.
    let disk = fs::read(corpus.repo().join(kind.home())).expect("read the doc");
    assert!(
        !disk.windows(2).any(|pair| pair == b"\r\n"),
        "the premise: the doc is on disk as jigc wrote it, with `\\n` line endings",
    );
    assert_eq!(
        corpus.git(&["status", "--porcelain", "--", kind.home()]),
        "",
        "the premise: git calls the doc unmodified",
    );
    corpus.fresh_clone_shape();

    let task = Door::Task.mint(&corpus);
    kind.first_write(&corpus, &task);
    // The key is dropped, so the door reaches the backstop.
    forget(&corpus, kind.home());
    let out = Door::Task.finalize(&corpus, &task);
    let envelope: serde_json::Value = stdout_json(&out, &[0], "the finalize");
    assert!(
        keyed(&findings(&envelope), CONFLICT, kind.home()).is_empty(),
        "an untouched doc jigc wrote is at its pin — no conflict; {}",
        text(&out),
    );
}

/// A way git converts between a blob and its working-tree file — the axis the completion
/// audit exposed at the committing doors. Each makes an untouched doc's working file
/// something other than its blob's bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Conversion {
    /// `core.autocrlf=true` — the Git for Windows default, and the reported instance.
    AutocrlfTrue,
    /// `core.autocrlf=input` — nothing is converted on the way out; the control that the
    /// setting alone changes nothing.
    AutocrlfInput,
    /// A committed `.gitattributes` holding `*.md text eol=crlf`.
    EolCrlf,
    /// A committed `.gitattributes` holding `* text=auto`, with `core.eol=crlf`.
    TextAuto,
    /// A clean/smudge filter over `*.md`: keyword expansion in the doc's prose.
    Filter,
    /// No conversion at all — the control every other cell is a variation of.
    None,
    /// A **committed blob that itself holds CRLF**, under `core.autocrlf=input`: git leaves
    /// such a blob alone in both directions (its rule for a path whose index entry already
    /// has CRLF), which `git hash-object` on its own does not apply — the configuration
    /// that refused clean repositories at `jigc setup` earlier in this pass.
    CommittedCrlf,
}

impl Conversion {
    const ALL: [Conversion; 7] = [
        Conversion::AutocrlfTrue,
        Conversion::AutocrlfInput,
        Conversion::EolCrlf,
        Conversion::TextAuto,
        Conversion::Filter,
        Conversion::None,
        Conversion::CommittedCrlf,
    ];

    /// The keyword the filter cell expands.
    const KEYWORD: &'static str = "$Rev$";
    const EXPANDED: &'static str = "$Rev: 42 $";

    /// Whether git writes this checkout's working files in a byte form other than the one
    /// jigc's in-place write leaves — where a doc checked out again differs in bytes from
    /// the same doc as a finalize landed it.
    fn rewrites_on_checkout(self) -> bool {
        matches!(
            self,
            Conversion::AutocrlfTrue | Conversion::EolCrlf | Conversion::TextAuto
        )
    }

    /// Put the corpus under this conversion **as a fresh clone would find it**: the setting
    /// in force, every tracked file checked out again through it, and no file-state cache.
    fn apply(self, corpus: &TrialCorpus) {
        let repo = corpus.repo();
        let commit_attributes = |attributes: &str| {
            fs::write(repo.join(".gitattributes"), attributes).expect("write .gitattributes");
            corpus.git(&["add", "--", ".gitattributes"]);
            corpus.git(&["commit", "-q", "-m", "chore: attributes"]);
        };
        // Mask whatever this machine's ambient config converts.
        corpus.git(&["config", "core.autocrlf", "false"]);
        match self {
            Conversion::None => {}
            Conversion::CommittedCrlf => {
                // The blob is committed holding CRLF while nothing converts, and only then
                // does the setting come into force.
                let home = repo.join(Kind::Placement.home());
                let doc = fs::read_to_string(&home).expect("read the doc");
                fs::write(&home, doc.replace('\n', "\r\n")).expect("write the doc as CRLF");
                corpus.git(&["add", "--", Kind::Placement.home()]);
                corpus.git(&["commit", "-q", "-m", "chore: a doc committed with CRLF"]);
                corpus.git(&["config", "core.autocrlf", "input"]);
            }
            Conversion::AutocrlfTrue => {
                corpus.git(&["config", "core.autocrlf", "true"]);
            }
            Conversion::AutocrlfInput => {
                corpus.git(&["config", "core.autocrlf", "input"]);
            }
            Conversion::EolCrlf => commit_attributes("*.md text eol=crlf\n"),
            Conversion::TextAuto => {
                corpus.git(&["config", "core.eol", "crlf"]);
                commit_attributes("* text=auto\n");
            }
            Conversion::Filter => {
                corpus.git(&[
                    "config",
                    "filter.rev.clean",
                    "sed -e 's/[$]Rev: [0-9]* [$]/$Rev$/'",
                ]);
                corpus.git(&[
                    "config",
                    "filter.rev.smudge",
                    "sed -e 's/[$]Rev[$]/$Rev: 42 $/'",
                ]);
                // The keyword goes into the doc's committed prose, behind every base pin.
                let home = repo.join(Kind::Placement.home());
                let doc = fs::read_to_string(&home).expect("read the doc");
                let anchor = Kind::Placement.anchor().trim_end();
                let with_keyword =
                    doc.replacen(anchor, &format!("{anchor} Revision {}.", Self::KEYWORD), 1);
                assert_ne!(doc, with_keyword, "the keyword lands in the doc");
                fs::write(&home, with_keyword).expect("write the keyword");
                corpus.git(&["add", "--", Kind::Placement.home()]);
                commit_attributes("VISION.md filter=rev\n");
            }
        }
        // A clone's checkout: every tracked file written again, through the conversion.
        corpus.git(&["rm", "-r", "-q", "--cached", "."]);
        corpus.git(&["reset", "-q", "--hard"]);
        corpus.fresh_clone_shape();
        assert_eq!(
            corpus.git(&["status", "--porcelain"]),
            "",
            "{self:?}: the premise — a clean checkout",
        );
        let doc = read(&repo, Kind::Placement.home());
        match self {
            Conversion::AutocrlfTrue
            | Conversion::EolCrlf
            | Conversion::TextAuto
            | Conversion::CommittedCrlf => assert!(
                doc.contains("\r\n"),
                "{self:?}: the premise — the doc is checked out with CRLF endings",
            ),
            Conversion::AutocrlfInput | Conversion::None => {
                assert!(!doc.contains('\r'), "{self:?}: premise")
            }
            Conversion::Filter => assert!(
                doc.contains(Self::EXPANDED),
                "{self:?}: the premise — the working file is the smudged form",
            ),
        }
    }
}

/// Mint a `single-task` task under `intent`, with its commit doc authored.
fn mint_task(corpus: &TrialCorpus, intent: &str) -> String {
    let task = corpus.start_workflow("single-task", intent);
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), &task, "docs");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, intent);
    task
}

/// Append [`HAND`] after the anchor line of the placement doc, in the line ending that line
/// already has — an out-of-band edit as an editor on that checkout would save it.
fn hand_edit_in_place(corpus: &TrialCorpus) {
    let path = corpus.repo().join(Kind::Placement.home());
    let before = fs::read_to_string(&path).expect("read the doc");
    let anchor = Kind::Placement.anchor().trim_end();
    let at = before.find(anchor).expect("the anchor line is in the doc");
    let line_end = at + before[at..].find('\n').expect("the anchor line ends") + 1;
    let eol = if before[..line_end].ends_with("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let after = format!("{}{HAND}{eol}{}", &before[..line_end], &before[line_end..]);
    fs::write(&path, after).expect("write the hand edit");
}

/// **A doc nobody edited never blocks a committing door, under any conversion — and one
/// somebody did edit still does** (the rc.24 fix pass's completion audit, the eol
/// regression).
///
/// The regression, driven at `5f5b273a` in a `core.autocrlf=true` clone and under
/// `* text eol=crlf`: `jigc doc set-slot` edits the checked-out CRLF doc in place, so the
/// file a finalize lands is **mixed** — CRLF on the lines git wrote, `\n` on the lines jigc
/// did. `git add` normalizes it and git calls it unmodified. The base-pin backstop compared
/// bytes, against the two forms it knew (the checked-out one, the raw blob), matched
/// neither, and conflict-blocked the next task once the key was gone; the route's
/// `git checkout` rewrote nothing, because git had no modification to undo, and
/// `jigc unmanage` — the exit `1.0.0-rc.24` had — led straight back to the block. The two
/// cells above built *"jigc's own write"* from a doc landed before the conversion existed,
/// which is all `\n` and never mixed.
///
/// So the sequence here is the one a clone lives through, over every conversion: a first
/// task lands; a second lands with the key held (the doc reads drifted at once in such a
/// checkout, since the recorded hash is the blob's — the block `1.0.0-rc.24` raised here is
/// closed by the same answer); a third lands with the key lost after its copy-in; and where
/// the instance was reported, a fourth lands through `jigc unmanage`. Beside them, the
/// refusals the guard is for: a hand edit after the copy-in blocks with the key held and
/// with it gone, the line stays on disk, and reverting it as the route says lands the task.
#[test]
fn an_unedited_doc_never_blocks_under_any_conversion_and_an_edited_one_still_does() {
    let base = baselined_corpus();
    let kind = Kind::Placement;
    let home = kind.home();
    std::thread::scope(|scope| {
        for conversion in Conversion::ALL {
            let corpus = base.copy_state();
            scope.spawn(move || {
                conversion.apply(&corpus);
                let write = |task: &str, prose: &str| {
                    let out = corpus.jigc_stdin(
                        &[
                            "doc",
                            "set-slot",
                            &kind.slot(),
                            "--from-file",
                            "-",
                            "--task",
                            task,
                        ],
                        &format!("{prose}\n"),
                    );
                    assert_eq!(out.status.code(), Some(0), "{conversion:?}: {}", text(&out));
                };
                let lands = |task: &str, what: &str| {
                    let out = Door::Task.finalize(&corpus, task);
                    let envelope: serde_json::Value =
                        stdout_json(&out, &[0], &format!("{conversion:?}: {what}"));
                    assert!(
                        keyed(&findings(&envelope), CONFLICT, home).is_empty(),
                        "{conversion:?}: {what} — no conflict on a doc nobody edited; {}",
                        text(&out),
                    );
                    assert_eq!(
                        corpus.git(&["status", "--porcelain"]),
                        "",
                        "{conversion:?}: {what} leaves the checkout clean",
                    );
                };

                // 1. The clone's first task.
                let task = mint_task(&corpus, "first pass over the vision");
                write(&task, "FIRST-PASS which domains earn a pack.");
                lands(&task, "the clone's first task");
                let doc = read(&corpus.repo(), home);
                if conversion.rewrites_on_checkout() {
                    assert!(
                        doc.contains("\r\n") && doc.replace("\r\n", "").contains('\n'),
                        "{conversion:?}: the premise — jigc's in-place write left a MIXED file",
                    );
                }

                // 2. The next task, the key held.
                let task = mint_task(&corpus, "second pass over the vision");
                write(&task, "SECOND-PASS which domains earn a pack.");
                lands(&task, "a second task, the key held");

                // 2b. **git checks the doc out again between the copy-in and the door**
                //     (`DECISIONS.md` → 2026-10-05, the third accepted consequence). The
                //     task copied in the file as the finalize above left it; git then
                //     writes it in its own form. Nobody edited it, so the task's witness
                //     must not call it edited: the bytes are compared first, and where
                //     they differ git is asked what it stores each side as, at the doc's
                //     home. With the witness, and in an area the previous format minted.
                for previous_format in [false, true] {
                    let what = format!(
                        "a doc git checked out again after the copy-in, previous format: \
                         {previous_format}"
                    );
                    let task = mint_task(&corpus, &format!("re-checkout {previous_format}"));
                    write(
                        &task,
                        &format!("RE-CHECKOUT {previous_format} which domains earn a pack."),
                    );
                    if previous_format {
                        strip_witness(&corpus, &task);
                    }
                    let copied_in = fs::read(corpus.repo().join(home)).expect("read the doc");
                    fs::remove_file(corpus.repo().join(home)).expect("remove the doc");
                    corpus.git(&["checkout", "--", home]);
                    let checked_out = fs::read(corpus.repo().join(home)).expect("read the doc");
                    assert_eq!(
                        copied_in != checked_out,
                        conversion.rewrites_on_checkout(),
                        "{conversion:?}: the premise — whether git's form differs in bytes \
                         from the form the task copied in",
                    );
                    lands(&task, &what);
                }

                // 3. The key lost after the copy-in — the backstop's own arm.
                let task = mint_task(&corpus, "third pass over the vision");
                write(&task, "THIRD-PASS which domains earn a pack.");
                corpus.fresh_clone_shape();
                assert_eq!(recorded(&corpus, home), None, "{conversion:?}: premise");
                lands(&task, "a third task, the key lost after its copy-in");
                let landed = corpus.git(&["show", &format!("HEAD:{home}")]);
                // A blob that already holds CRLF is the one git does not normalize.
                assert!(
                    landed.contains("THIRD-PASS")
                        && (conversion == Conversion::CommittedCrlf || !landed.contains('\r')),
                    "{conversion:?}: what landed is the task's prose, normalized: {landed}",
                );

                // 4. `jigc unmanage` between the copy-in and the door.
                if conversion == Conversion::AutocrlfTrue {
                    let task = mint_task(&corpus, "fourth pass over the vision");
                    write(&task, "FOURTH-PASS which domains earn a pack.");
                    corpus.jigc_ok(&["unmanage", home]);
                    lands(&task, "a fourth task, through `jigc unmanage`");
                }

                // 5. And a hand edit after the copy-in still blocks — key held, then gone.
                let task = mint_task(&corpus, "fifth pass over the vision");
                write(&task, "FIFTH-PASS which domains earn a pack.");
                hand_edit_in_place(&corpus);
                let edited = read(&corpus.repo(), home);
                assert!(edited.contains(HAND), "{conversion:?}: premise");
                let head_before = head(&corpus);
                for key in ["held", "lost"] {
                    if key == "lost" {
                        corpus.fresh_clone_shape();
                    }
                    let out = Door::Task.finalize(&corpus, &task);
                    assert_blocked(
                        &corpus,
                        &out,
                        home,
                        &head_before,
                        &edited,
                        &format!("{conversion:?}: a hand edit, the key {key}"),
                    );
                }
                // The route's second exit — revert the external edit on disk — lands it.
                corpus.git(&["checkout", "--", home]);
                assert!(
                    !read(&corpus.repo(), home).contains(HAND),
                    "{conversion:?}: git reverted the edit it could see",
                );
                lands(&task, "the same task, the edit reverted as the route says");
            });
        }
    });
}

/// **The milestone join reads the same answer.** `jigc milestone finalize` sweeps the
/// committed store against the milestone's base through the same seam, so a doc a sub-task
/// staged in a converting checkout, its key lost, lands there too.
#[test]
fn the_milestone_join_does_not_block_an_unedited_doc_under_conversion() {
    let corpus = baselined_corpus().copy_state();
    let kind = Kind::Placement;
    Conversion::EolCrlf.apply(&corpus);
    // A first task, so the doc on disk is the mixed file an in-place write leaves.
    let task = mint_task(&corpus, "first pass over the vision");
    corpus.set_slot(&kind.slot(), &task, "FIRST-PASS which domains earn a pack.");
    let out = Door::Task.finalize(&corpus, &task);
    assert_eq!(out.status.code(), Some(0), "{}", text(&out));

    let sub = Door::Milestone.mint(&corpus);
    kind.first_write(&corpus, &sub);
    // git checks the doc out again after the sub-task's copy-in: the sub-task's witness
    // names the mixed file, the door meets git's own form, and nobody edited it.
    let copied_in = fs::read(corpus.repo().join(kind.home())).expect("read the doc");
    fs::remove_file(corpus.repo().join(kind.home())).expect("remove the doc");
    corpus.git(&["checkout", "--", kind.home()]);
    assert_ne!(
        copied_in,
        fs::read(corpus.repo().join(kind.home())).expect("read the doc"),
        "the premise: git's form differs in bytes from the form the sub-task copied in",
    );
    corpus.fresh_clone_shape();
    let out = Door::Milestone.finalize(&corpus, &sub);
    let envelope: serde_json::Value = stdout_json(&out, &[0], "the join");
    assert!(
        keyed(&findings(&envelope), CONFLICT, kind.home()).is_empty(),
        "no conflict at the join on a doc nobody edited; {}",
        text(&out),
    );
}

/// **The question is about the file, so a doc the index no longer carries is still decided
/// by its bytes.** `git diff <pin> -- <path>` reads a path with no index entry as *deleted*
/// whatever the working file holds — a statement about the index. Driven while this fix was
/// being built: a task's doc un-tracked with `git rm --cached` after the copy-in, its key
/// lost, nobody editing the file, refused at `jigc task finalize` where `1.0.0-rc.24` lands.
/// So with no index entry the file is compared with the pin over a scratch index: untouched
/// it lands, and a hand edit there still blocks.
#[test]
fn a_doc_the_index_no_longer_carries_is_still_decided_by_the_file() {
    let base = baselined_corpus();
    let kind = Kind::Placement;
    for edited in [false, true] {
        let what = format!("un-tracked after the copy-in, edited: {edited}");
        let corpus = base.copy_state();
        let task = Door::Task.mint(&corpus);
        kind.first_write(&corpus, &task);
        forget(&corpus, kind.home());
        corpus.git(&["rm", "-q", "--cached", "--", kind.home()]);
        if edited {
            kind.hand_edit(&corpus);
        }
        let disk = read(&corpus.repo(), kind.home());
        let head_before = head(&corpus);
        let out = Door::Task.finalize(&corpus, &task);
        if edited {
            assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
        } else {
            let envelope: serde_json::Value = stdout_json(&out, &[0], &what);
            assert!(
                keyed(&findings(&envelope), CONFLICT, kind.home()).is_empty(),
                "{what}: no conflict on a file nobody edited; {}",
                text(&out),
            );
            assert!(
                corpus
                    .git(&["show", &format!("HEAD:{}", kind.home())])
                    .contains(TASK_PROSE),
                "{what}: the task's write landed",
            );
        }
    }
}

/// **The statement, where the rule is described.** The design sentences this fix makes true,
/// and the one it narrows, name what the code now does.
#[test]
fn the_design_states_the_copy_in_baseline_and_its_backstop() {
    let design = |doc: &str| {
        fs::read_to_string(format!("{}/../../design/{doc}", env!("CARGO_MANIFEST_DIR")))
            .unwrap_or_else(|e| panic!("read design/{doc}: {e}"))
    };
    let reconciliation = design("reconciliation.md");
    for needle in [
        "copy-in",
        "base-pin backstop",
        "another open task",
        "fails closed",
    ] {
        assert!(
            reconciliation.contains(needle),
            "design/reconciliation.md states `{needle}`",
        );
    }
    let storage = design("storage.md");
    assert!(
        storage.contains("copy-in") && storage.contains("base-pin backstop"),
        "design/storage.md → Concurrent writers names the copy-in writer and the backstop",
    );
    let validation = design("validation.md");
    assert!(
        validation.contains("after a copy-in"),
        "design/validation.md states what the store sweep reports in a clone after a copy-in",
    );

    // What the two rules could not answer is answered by the task's own witness, and each
    // doc that stated the bound as open now states the rule that closed it — and the one
    // bound that is left, an area minted before the witness existed.
    assert!(
        reconciliation.contains("### What a task copied in")
            && reconciliation.contains("the task's own witness decides")
            && reconciliation.contains(
                "an edit before the task's first write is carried and lands; an edit after \
                 it blocks"
            )
            && reconciliation.contains("git hash-object --path=<home>")
            && reconciliation.contains("An area with no witness is decided as before"),
        "design/reconciliation.md states the witness, its one rule, the form it is compared \
         in and the previous-format bound",
    );
    assert!(
        reconciliation
            .contains("`jigc unmanage` is not an exit from a conflict on a doc a task holds"),
        "design/reconciliation.md states that dropping the baseline is no exit",
    );
    assert!(
        storage.contains("the task now answers itself") && storage.contains("`copied-in`"),
        "design/storage.md names the witness and where it is kept",
    );
    for (doc, body) in [
        ("design/reconciliation.md", reconciliation.as_str()),
        ("design/storage.md", storage.as_str()),
        ("design/validation.md", validation.as_str()),
    ] {
        for claimed_open in [
            "(open — found by the rc.24 fix pass's completion audit)",
            "Both are open",
            "it is not built",
        ] {
            assert!(
                !body.contains(claimed_open),
                "{doc} no longer states the closed cells as open: `{claimed_open}`",
            );
        }
    }
    let guide = fs::read_to_string(format!(
        "{}/guides/MIGRATING.md",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("read the migrating guide");
    assert!(
        guide.contains(
            "an edit you made *before* the task first wrote the doc is in the task's staged \
             copy and lands with the task"
        ) && guide.contains("and an edit you make *after* it blocks")
            && guide.contains("`jigc unmanage` is not a third")
            && !guide.contains("That guard has two bounds")
            && !guide.contains("still writes its staged copy over your edit at exit 0"),
        "the guide the install embeds states the one rule, and no longer the two bounds it \
         closed",
    );
}
