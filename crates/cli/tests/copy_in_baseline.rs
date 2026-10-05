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
//! The suite iterates the class, not the reported instance: both committing doors × a
//! placement and a location doctype × every way the key is absent at the first write × the
//! two orders of the hand edit, every `doc` write leaf as the first touch, and the nine
//! cells the new writer brings (the planning's advocacy, `advocate-promote-1`): raw-bytes
//! hash · the sweep's key · the save-lock timeout · concurrent first writes · a
//! non-conformant doc · the adoption stated at the write door · the store sweep's reading ·
//! two tasks on one doc · a task with no record.
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

/// **The declared order keeps landing.** A hand edit made *before* the task's first write is
/// carried by the copy-in, so the door lands both sides in one commit
/// (`design/storage.md` → Concurrent writers, *What none of this buys*) — at both doors, for
/// both kinds, whichever way the key was absent.
#[test]
fn a_hand_edit_before_the_first_write_still_lands_merged() {
    let base = baselined_corpus();
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

/// **Cell 5 — a non-conformant doc is never baselined**, and the door still does not
/// overwrite it. A hand edit that breaks conformance (a malformed `date`) sits on disk when
/// the task first writes the doc: the copy-in records nothing and adopts nothing, and the
/// door blocks on the base-pin backstop instead of advising *fix the file* and then
/// overwriting it. The same break made *after* the first write blocks on the recorded
/// baseline.
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
    let staged = |corpus: &TrialCorpus, task: &str| {
        corpus
            .repo()
            .join(".jigc/tasks")
            .join(task)
            .join("docs")
            .join(format!("{}.md", kind.address()))
    };

    for before_the_first_write in [true, false] {
        let what = format!("broken before the first write: {before_the_first_write}");
        let corpus = base.copy_state();
        corpus.fresh_clone_shape();
        let task = Door::Task.mint(&corpus);
        if before_the_first_write {
            break_it(&corpus);
            // The write itself is refused — the doc it copied in does not conform, so the
            // splice has nothing sound to land in — but the copy-in ran first: the doc is
            // staged, and that is what makes the door's sweep read it *touched*.
            let out = corpus.jigc_stdin(
                &[
                    "doc",
                    "set-slot",
                    &kind.slot(),
                    "--from-file",
                    "-",
                    "--task",
                    &task,
                    "--format",
                    "json",
                ],
                &format!("{TASK_PROSE}\n"),
            );
            assert!(
                staged(&corpus, &task).is_file(),
                "{what}: the premise — the copy-in staged the doc; {}",
                text(&out),
            );
            assert_eq!(
                recorded(&corpus, kind.home()),
                None,
                "{what}: a non-conformant doc is not baselined at the copy-in",
            );
            assert!(
                !String::from_utf8_lossy(&out.stdout).contains(ADOPT)
                    && !String::from_utf8_lossy(&out.stderr).contains(ADOPT),
                "{what}: and the write claims no adoption; {}",
                text(&out),
            );
        } else {
            kind.first_write(&corpus, &task);
            break_it(&corpus);
        }
        let disk = read(&corpus.repo(), kind.home());
        let head_before = head(&corpus);
        let out = Door::Task.finalize(&corpus, &task);
        assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
    }
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

/// **Cell 9 — a task with a staged doc and no record gets the backstop.** That is every task
/// a binary older than the copy-in baseline copied in, and every key lost after the copy-in.
/// The base pin decides: bytes off the pin block, in either order of the hand edit (the
/// staged copy cannot say which side it carries); bytes at the pin land. And `jigc unmanage`
/// after a block no longer switches the guard off.
#[test]
fn a_staged_doc_with_no_record_is_decided_by_the_base_pin() {
    let base = baselined_corpus();
    #[derive(Clone, Copy, Debug)]
    enum Edit {
        None,
        BeforeTheCopyIn,
        AfterTheCopyIn,
    }
    for door in Door::ALL {
        for kind in Kind::ALL {
            for edit in [Edit::None, Edit::BeforeTheCopyIn, Edit::AfterTheCopyIn] {
                let what = format!("{door:?}/{kind:?}/{edit:?}");
                let corpus = base.copy_state();
                let task = door.mint(&corpus);
                if matches!(edit, Edit::BeforeTheCopyIn) {
                    kind.hand_edit(&corpus);
                }
                kind.first_write(&corpus, &task);
                if matches!(edit, Edit::AfterTheCopyIn) {
                    kind.hand_edit(&corpus);
                }
                forget(&corpus, kind.home());
                let disk = read(&corpus.repo(), kind.home());
                let head_before = head(&corpus);
                let out = door.finalize(&corpus, &task);
                match edit {
                    Edit::None => assert_eq!(
                        out.status.code(),
                        Some(0),
                        "{what}: bytes at the pin land; {}",
                        text(&out),
                    ),
                    Edit::BeforeTheCopyIn | Edit::AfterTheCopyIn => {
                        assert_blocked(&corpus, &out, kind.home(), &head_before, &disk, &what);
                    }
                }
            }
        }
    }

    // `unmanage` after the block: the recorded baseline blocked, the key is dropped, and the
    // backstop blocks the same finalize again.
    for kind in Kind::ALL {
        let what = format!("{kind:?}: unmanage after the block");
        let corpus = base.copy_state();
        let task = Door::Task.mint(&corpus);
        kind.first_write(&corpus, &task);
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
}

impl Conversion {
    const ALL: [Conversion; 5] = [
        Conversion::AutocrlfTrue,
        Conversion::AutocrlfInput,
        Conversion::EolCrlf,
        Conversion::TextAuto,
        Conversion::Filter,
    ];

    /// The keyword the filter cell expands.
    const KEYWORD: &'static str = "$Rev$";
    const EXPANDED: &'static str = "$Rev: 42 $";

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
            Conversion::AutocrlfTrue | Conversion::EolCrlf | Conversion::TextAuto => assert!(
                doc.contains("\r\n"),
                "{self:?}: the premise — the doc is checked out with CRLF endings",
            ),
            Conversion::AutocrlfInput => assert!(!doc.contains('\r'), "{self:?}: premise"),
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
                if matches!(
                    conversion,
                    Conversion::AutocrlfTrue | Conversion::EolCrlf | Conversion::TextAuto
                ) {
                    assert!(
                        doc.contains("\r\n") && doc.replace("\r\n", "").contains('\n'),
                        "{conversion:?}: the premise — jigc's in-place write left a MIXED file",
                    );
                }

                // 2. The next task, the key held.
                let task = mint_task(&corpus, "second pass over the vision");
                write(&task, "SECOND-PASS which domains earn a pack.");
                lands(&task, "a second task, the key held");

                // 3. The key lost after the copy-in — the backstop's own arm.
                let task = mint_task(&corpus, "third pass over the vision");
                write(&task, "THIRD-PASS which domains earn a pack.");
                corpus.fresh_clone_shape();
                assert_eq!(recorded(&corpus, home), None, "{conversion:?}: premise");
                lands(&task, "a third task, the key lost after its copy-in");
                let landed = corpus.git(&["show", &format!("HEAD:{home}")]);
                assert!(
                    landed.contains("THIRD-PASS") && !landed.contains('\r'),
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
}
