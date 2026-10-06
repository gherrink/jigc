//! The rc.24 fix pass — **the milestone-record door with no baseline**.
//!
//! `design/team-ready-state.md` → *No-silent-overwrite discipline*: an out-of-band edit to
//! the machine-maintained `milestone-record` is *"detected + conflict-blocked"*, never
//! merged and never clobbered. Through the `(R3, F7)` fix that sentence held only where the
//! gitignored `file-state` cache happened to hold the record's key. With no key — every
//! fresh clone, a deleted cache, a `jigc unmanage` of the record — the record door's
//! reconcile preflight took the `UNKNOWN` arm, adopted whatever was on disk, and the op then
//! wrote the record over it: `jigc milestone add-task` re-rendered a hand line away at exit
//! 0 (worktree 0, `HEAD` 0), and the splicing doors committed the hand edit as if jigc had
//! written it. The `(R3, F7)` base-pin backstop never reached this door, because it passed
//! no pin.
//!
//! The rule (`design/reconciliation.md` → Baseline adoption, the record door's witness):
//! with no recorded hash, the record's blob at `HEAD` is the witness of what jigc last
//! wrote — every record write lands in a commit — so a record git calls modified against
//! it is an edit and the door raises its existing `reconciliation.conflict-block`, adopts
//! nothing and writes nothing. One git calls **unmodified** (an untouched clone, a pulled
//! edit) is adopted exactly as before.
//!
//! **The comparison is git's own** (the completion audit's eol regression): the door first
//! compared bytes, and in a converting checkout an untouched record is not its blob's
//! bytes — the conversion cells below iterate the settings × the byte forms a record takes.
//!
//! The suite iterates the class, not the reported instance: every record-rewriting door ×
//! every way the key is absent × three shapes of the hand edit, each refusal's emitted
//! route run **as printed** from outside the repository, then the same door re-run to a
//! landed result. Real binary throughout, in throwaway corpora.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use engine::file_state::FileStateRecord;

use crate::support;
use crate::support::trial_corpus::{State, TrialCorpus, read};

const MILESTONE_TITLE: &str = "sharpen the docs";
const MILESTONE: &str = "sharpen-the-docs";

/// The record's home, repo-relative — its file-state key.
const RECORD: &str = "docs/milestone-records/sharpen-the-docs.md";

/// The hand line written straight into the record, out of band.
const HAND: &str = "A hand note written into the record HAND-REC-5514.";

const CONFLICT: &str = "reconciliation.conflict-block";

/// A committed two-criteria spec — what `add-from-spec` seeds from.
const SPEC: &str = "\
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

fn text(out: &Output) -> String {
    format!(
        "exit {:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// A live milestone with one sub-task that has staged a doc edit — the state every door on
/// the axis can act on. Returns the corpus and the sub-task's id, read off the binary's ack.
fn live_milestone() -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    // The spec is committed BEFORE the milestone is minted, so it sits behind the base pin.
    let specs = corpus.repo().join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs");
    fs::write(specs.join("rate-limit.md"), SPEC).expect("write the spec");
    corpus.git(&["add", "--", "docs/specs/rate-limit.md"]);
    corpus.git(&["commit", "-q", "-m", "docs: add the rate-limit spec"]);

    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let ack = corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "alpha sharpens a doc"]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
    let sub = rest
        .split_whitespace()
        .next()
        .expect("the sub-task id")
        .to_owned();
    // The sub-task's contribution — what lets `milestone finalize` land rather than refuse
    // on `milestone.zero-contribution`.
    corpus.set_slot(
        "vision:vision#open-questions",
        &sub,
        "Which domains earn a pack, and when.",
    );
    assert!(corpus.repo().join(RECORD).is_file(), "the record landed");
    (corpus, sub)
}

/// A door that **rewrites the committed record in place** — the class axis. `milestone
/// create` is not a member: it mints a record at a home it found free and never opens one
/// that exists (`milestone.record-exists`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Door {
    /// `jigc milestone add-task` — appends an item (re-rendering the prior last one).
    AddTask,
    /// `jigc milestone add-from-spec` — one append per criterion.
    AddFromSpec,
    /// `jigc milestone discard` — settles the header and every live item.
    Discard,
    /// `jigc task discard <sub-task>` — settles one item.
    SubTaskDiscard,
    /// `jigc milestone finalize` — the status flip folded into the boundary commit.
    Finalize,
}

impl Door {
    const ALL: [Door; 5] = [
        Door::AddTask,
        Door::AddFromSpec,
        Door::Discard,
        Door::SubTaskDiscard,
        Door::Finalize,
    ];

    fn argv(self, sub: &str) -> Vec<String> {
        let argv: &[&str] = match self {
            Door::AddTask => &["milestone", "add-task", MILESTONE, "bravo checks the links"],
            Door::AddFromSpec => &["milestone", "add-from-spec", MILESTONE, "spec:rate-limit"],
            Door::Discard => &["milestone", "discard", MILESTONE, "--force"],
            Door::SubTaskDiscard => &["task", "discard", sub, "--force"],
            Door::Finalize => &["milestone", "finalize", MILESTONE],
        };
        argv.iter().map(|word| (*word).to_owned()).collect()
    }

    fn run(self, corpus: &TrialCorpus, sub: &str) -> Output {
        let argv = self.argv(sub);
        let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
        corpus.jigc(&argv)
    }

    /// The mark the door's own write leaves in the committed record.
    fn mark(self) -> &'static str {
        match self {
            Door::AddTask => "bravo-checks-the-links",
            Door::AddFromSpec => "rejects-the-101st-request",
            Door::Discard | Door::SubTaskDiscard => "status: discarded",
            Door::Finalize => "status: joined",
        }
    }
}

/// A way the record has no `file-state` key when the door runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Absent {
    /// The fresh-clone shape: `.jigc/state/` is gitignored, so a clone has no record of
    /// what jigc last wrote.
    CacheGone,
    /// `jigc unmanage <record>` dropped the key.
    Unmanaged,
}

impl Absent {
    const ALL: [Absent; 2] = [Absent::CacheGone, Absent::Unmanaged];

    fn apply(self, corpus: &TrialCorpus) {
        match self {
            Absent::CacheGone => corpus.fresh_clone_shape(),
            Absent::Unmanaged => {
                corpus.jigc_ok(&["unmanage", RECORD]);
            }
        }
        assert_eq!(
            recorded(corpus),
            None,
            "{self:?}: the premise — no file-state key for the record",
        );
    }
}

/// A shape the out-of-band edit takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Edit {
    /// A prose line appended to the record, unstaged — the reported instance.
    Note,
    /// The same line, `git add`ed: the index holds it too, so only a restore from `HEAD`
    /// puts the record back.
    StagedNote,
    /// The machine-set header `status` flipped to another **valid** value — drift a door
    /// that read it would act on (`milestone.terminal`), where the note is only lost.
    StatusFlip,
}

impl Edit {
    const ALL: [Edit; 3] = [Edit::Note, Edit::StagedNote, Edit::StatusFlip];

    fn apply(self, corpus: &TrialCorpus) {
        let path = corpus.repo().join(RECORD);
        let before = fs::read_to_string(&path).expect("read the record");
        let after = match self {
            Edit::Note | Edit::StagedNote => format!("{before}\n{HAND}\n"),
            Edit::StatusFlip => before.replacen("status: active", "status: joined", 1),
        };
        assert_ne!(before, after, "{self:?}: the hand edit changes the record");
        fs::write(&path, after).expect("write the hand edit");
        if self == Edit::StagedNote {
            corpus.git(&["add", "--", RECORD]);
        }
    }
}

/// The recorded hash for the record's key, read through the engine API that owns the store.
fn recorded(corpus: &TrialCorpus) -> Option<String> {
    FileStateRecord::load(&corpus.repo().join(".jigc"))
        .expect("load the file-state record")
        .get(RECORD)
        .map(str::to_owned)
}

fn head(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD"])
}

/// `git <args>` in `repo`, asserting success; stdout verbatim.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// The record as `HEAD` holds it.
fn record_at_head(repo: &Path) -> String {
    git(repo, &["show", &format!("HEAD:{RECORD}")])
}

/// The `route:` line of the conflict finding in a text refusal.
fn conflict_route(stderr: &str, what: &str) -> String {
    let mut lines = stderr.lines().skip_while(|line| !line.contains(CONFLICT));
    assert!(
        lines.next().is_some(),
        "{what}: the refusal is `{CONFLICT}`; got:\n{stderr}",
    );
    lines
        .find_map(|line| line.trim_start().strip_prefix("route:"))
        .unwrap_or_else(|| panic!("{what}: a blocking finding always routes; got:\n{stderr}"))
        .trim()
        .to_owned()
}

/// The one backticked `git …` span a route carries, as emitted.
fn git_span<'a>(route: &'a str, what: &str) -> &'a str {
    let spans: Vec<&str> = route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("git "))
        .collect();
    assert_eq!(
        spans.len(),
        1,
        "{what}: the route names exactly one git command; got: {route}",
    );
    spans[0]
}

/// Run an emitted command line through a real `sh` word split, from `cwd`.
fn run_emitted(command: &str, cwd: &Path, home: &Path) -> Output {
    let argv = support::shell_words(command, cwd, home);
    Command::new(&argv[0])
        .args(&argv[1..])
        .current_dir(cwd)
        .env("HOME", home)
        .output()
        .expect("spawn the emitted command")
}

/// Assert `out` is the record door **blocked** on the conflict at the record: nothing
/// committed, nothing adopted, the record's bytes untouched. Returns the finding's route.
fn assert_blocked(
    repo: &Path,
    out: &Output,
    head_before: &str,
    edited: &str,
    what: &str,
) -> String {
    assert_eq!(out.status.code(), Some(1), "{what}: {}", text(out));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(CONFLICT) && stderr.contains(RECORD),
        "{what}: a `{CONFLICT}` naming the record; {}",
        text(out),
    );
    assert_eq!(head(repo), head_before, "{what}: nothing is committed");
    assert_eq!(
        read(repo, RECORD),
        edited,
        "{what}: the record's bytes are untouched — never merged, never clobbered",
    );
    let route = conflict_route(&stderr, what);
    assert!(
        !route.contains("jigc task discard") && !route.contains('<'),
        "{what}: the record's route names no task verb and carries no placeholder: {route}",
    );
    route
}

/// **The class.** Every door that rewrites the record × every way its key is absent ×
/// every shape of the hand edit: the door refuses under `reconciliation.conflict-block`,
/// commits nothing, adopts nothing and leaves the bytes on disk — and the route it prints
/// works as printed, from outside the repository, after which the same door lands.
#[test]
fn a_hand_edit_to_the_record_blocks_every_record_door_that_has_no_baseline() {
    let (base, sub) = live_milestone();
    for door in Door::ALL {
        for absent in Absent::ALL {
            for edit in Edit::ALL {
                let what = format!("{door:?}/{absent:?}/{edit:?}");
                let corpus = base.copy_state();
                let repo = corpus.repo();
                let committed = read(&repo, RECORD);
                absent.apply(&corpus);
                edit.apply(&corpus);
                let edited = read(&repo, RECORD);
                let head_before = head(&repo);

                let out = door.run(&corpus, &sub);
                let route = assert_blocked(&repo, &out, &head_before, &edited, &what);
                assert_eq!(
                    recorded(&corpus),
                    None,
                    "{what}: a refused door adopts nothing — the hand edit must not become \
                     `what jigc last wrote`",
                );
                assert!(
                    repo.join(".jigc").join("tasks").join(&sub).is_dir(),
                    "{what}: the sub-task's working area is still there",
                );

                // The route, as printed, from a directory that is not the repository.
                let span = git_span(&route, &what);
                let restored = run_emitted(span, &corpus.home(), &corpus.home());
                assert!(
                    restored.status.success(),
                    "{what}: the emitted `{span}` runs as printed; {}",
                    text(&restored),
                );
                assert_eq!(
                    read(&repo, RECORD),
                    committed,
                    "{what}: the route puts the record back to what `HEAD` holds",
                );
                assert_eq!(
                    git(&repo, &["status", "--porcelain", "--", RECORD]),
                    "",
                    "{what}: index and worktree both match `HEAD` after the route",
                );

                // …and the same door, re-run, lands.
                let rerun = door.run(&corpus, &sub);
                assert!(
                    rerun.status.success(),
                    "{what}: the re-run lands; {}",
                    text(&rerun),
                );
                let landed = record_at_head(&repo);
                assert!(
                    landed.contains(door.mark()),
                    "{what}: the door's own write is in the committed record:\n{landed}",
                );
                assert!(
                    !landed.contains(HAND),
                    "{what}: the hand line was never merged into the record",
                );
                assert_ne!(head(&repo), head_before, "{what}: the re-run committed");
            }
        }
    }
}

/// **The reported instance, in a real `git clone`**: no `.jigc/state/` at all, a hand line
/// appended to the record, then `jigc milestone add-task` as the clone's first jigc call.
#[test]
fn a_clone_does_not_re_render_the_record_over_a_hand_line() {
    let (corpus, _sub) = live_milestone();
    let clone = corpus
        .repo()
        .parent()
        .expect("the corpus root")
        .join("clone");
    let origin = corpus.repo();
    git(
        &origin,
        &[
            "clone",
            "-q",
            &origin.display().to_string(),
            &clone.display().to_string(),
        ],
    );
    git(&clone, &["config", "user.email", "mate@example.com"]);
    git(&clone, &["config", "user.name", "Mate"]);
    assert!(
        !clone.join(".jigc").join("state").exists(),
        "the premise: a clone carries no file-state cache",
    );

    let path = clone.join(RECORD);
    let committed = fs::read_to_string(&path).expect("the record is in the clone");
    let edited = format!("{committed}\n{HAND}\n");
    fs::write(&path, &edited).expect("write the hand line");
    let head_before = head(&clone);

    let argv = ["milestone", "add-task", MILESTONE, "bravo checks the links"];
    let out = corpus.jigc_stdin_from(&clone, &argv, "");
    let route = assert_blocked(&clone, &out, &head_before, &edited, "clone/add-task");

    let span = git_span(&route, "clone/add-task");
    let restored = run_emitted(span, &corpus.home(), &corpus.home());
    assert!(restored.status.success(), "{span}: {}", text(&restored));
    assert_eq!(read(&clone, RECORD), committed);

    let rerun = corpus.jigc_stdin_from(&clone, &argv, "");
    assert!(rerun.status.success(), "the re-run lands; {}", text(&rerun));
    let landed = record_at_head(&clone);
    assert!(landed.contains("bravo-checks-the-links"), "{landed}");
    assert!(!landed.contains(HAND), "{landed}");
}

/// **Control — an untouched record is still adopted.** With no key and bytes equal to
/// `HEAD`'s, every door serves exactly as before: the first encounter is not a conflict.
#[test]
fn an_untouched_record_with_no_baseline_still_serves_every_door() {
    let (base, sub) = live_milestone();
    for door in Door::ALL {
        for absent in Absent::ALL {
            let what = format!("{door:?}/{absent:?}");
            let corpus = base.copy_state();
            absent.apply(&corpus);
            let out = door.run(&corpus, &sub);
            assert!(out.status.success(), "{what}: {}", text(&out));
            assert!(
                record_at_head(&corpus.repo()).contains(door.mark()),
                "{what}: the door's write landed",
            );
        }
    }
}

/// **Control — a committed edit is still adopted** (the rc.24 review's `(R3, F4)` shape: a
/// teammate's edit arrives by `git pull`, so `HEAD` moved and the bytes on disk are
/// `HEAD`'s). With no key that is a first encounter of what git holds, not a conflict; this
/// rule must not turn it into a block.
#[test]
fn a_committed_edit_to_the_record_with_no_baseline_is_still_adopted() {
    let (base, sub) = live_milestone();
    for door in Door::ALL {
        for absent in Absent::ALL {
            let what = format!("{door:?}/{absent:?}");
            let corpus = base.copy_state();
            let repo = corpus.repo();
            absent.apply(&corpus);
            Edit::Note.apply(&corpus);
            corpus.git(&["add", "--", RECORD]);
            corpus.git(&["commit", "-q", "-m", "docs: a note on the record"]);
            assert_eq!(
                read(&repo, RECORD),
                record_at_head(&repo),
                "{what}: premise"
            );

            let out = door.run(&corpus, &sub);
            assert!(
                out.status.success(),
                "{what}: bytes equal to `HEAD`'s are adopted, not blocked; {}",
                text(&out),
            );
        }
    }
}

/// **Control — a held baseline keeps its own answer.** Where the cache does hold the
/// record's key, every drift conflict-blocks against *what jigc last wrote*, a committed
/// one included (`design/reconciliation.md` → the record door passes no base pin to the
/// pull-absorb arm). The `HEAD` witness is consulted only where there is no key, so it
/// cannot turn this block into an absorb.
#[test]
fn a_held_baseline_still_blocks_a_committed_edit_at_the_record_door() {
    let (corpus, sub) = live_milestone();
    let repo = corpus.repo();
    assert!(recorded(&corpus).is_some(), "the premise: the key is held");
    Edit::Note.apply(&corpus);
    corpus.git(&["add", "--", RECORD]);
    corpus.git(&["commit", "-q", "-m", "docs: a note on the record"]);
    let edited = read(&repo, RECORD);
    let head_before = head(&repo);

    let out = Door::AddTask.run(&corpus, &sub);
    let route = assert_blocked(&repo, &out, &head_before, &edited, "held/add-task");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("since jigc last wrote it") && route.contains("what jigc last wrote"),
        "the held-baseline presentation is unchanged; {}",
        text(&out),
    );
}

/// A way git converts between the record's blob and its working-tree file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Conversion {
    /// A committed `.gitattributes` holding `*.md text eol=crlf`.
    EolCrlf,
    /// `core.autocrlf=true` — the Git for Windows default.
    AutocrlfTrue,
    /// A committed `.gitattributes` holding `* text=auto`, with `core.eol=crlf`.
    TextAuto,
}

impl Conversion {
    const ALL: [Conversion; 3] = [
        Conversion::EolCrlf,
        Conversion::AutocrlfTrue,
        Conversion::TextAuto,
    ];

    /// Put the conversion in force. Runs before the milestone is minted.
    fn apply(self, corpus: &TrialCorpus) {
        // Mask whatever this machine's ambient config converts.
        corpus.git(&["config", "core.autocrlf", "false"]);
        let commit_attributes = |attributes: &str| {
            fs::write(corpus.repo().join(".gitattributes"), attributes)
                .expect("write .gitattributes");
            corpus.git(&["add", "--", ".gitattributes"]);
            corpus.git(&["commit", "-q", "-m", "chore: attributes"]);
        };
        match self {
            Conversion::EolCrlf => commit_attributes("*.md text eol=crlf\n"),
            Conversion::AutocrlfTrue => {
                corpus.git(&["config", "core.autocrlf", "true"]);
            }
            Conversion::TextAuto => {
                corpus.git(&["config", "core.eol", "crlf"]);
                commit_attributes("* text=auto\n");
            }
        }
    }
}

/// Which byte form the record on disk is in — every one of them a file git calls
/// unmodified against `HEAD`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Form {
    /// As jigc wrote it — `\n` endings, the blob's own bytes.
    Written,
    /// As a checkout writes it — `\r\n` endings.
    CheckedOut,
    /// As a record door leaves a checked-out record: it splices in place, so the lines git
    /// wrote keep `\r\n` and the lines jigc wrote have `\n`. What a clone's first record op
    /// produces, and the form the audit found unanswered for.
    Mixed,
}

impl Form {
    const ALL: [Form; 3] = [Form::Written, Form::CheckedOut, Form::Mixed];

    /// Bring the record at `repo` into this form, from the `Written` one.
    fn apply(self, corpus: &TrialCorpus, repo: &Path, what: &str) {
        let bytes = |repo: &Path| fs::read(repo.join(RECORD)).expect("read the record");
        let crlf = |bytes: &[u8]| bytes.windows(2).filter(|pair| pair == b"\r\n").count();
        let lines = |bytes: &[u8]| bytes.iter().filter(|byte| **byte == b'\n').count();
        if self == Form::Written {
            assert_eq!(crlf(&bytes(repo)), 0, "{what}: jigc wrote `\\n` endings");
            return;
        }
        fs::remove_file(repo.join(RECORD)).expect("remove the record");
        git(repo, &["checkout", "--", RECORD]);
        let record = bytes(repo);
        assert_eq!(
            crlf(&record),
            lines(&record),
            "{what}: the checkout wrote `\\r\\n` on every line",
        );
        if self == Form::Mixed {
            // As in a clone: no key yet, so the door's first write meets the checked-out
            // record as a first encounter and splices it in place.
            corpus.fresh_clone_shape();
            let out = corpus.jigc_stdin_from(
                repo,
                &[
                    "milestone",
                    "add-task",
                    MILESTONE,
                    "an earlier in-place write",
                ],
                "",
            );
            assert!(out.status.success(), "{what}: {}", text(&out));
            let record = bytes(repo);
            assert!(
                crlf(&record) > 0 && crlf(&record) < lines(&record),
                "{what}: the premise — the door's in-place write left a MIXED record",
            );
        }
    }
}

/// **The witness is git's answer, so every form of an untouched record is at `HEAD`** (the
/// rc.24 fix pass's completion audit, the eol regression).
///
/// jigc writes a record with `\n` endings whatever the checkout converts to; a clone's
/// checkout writes the converted form; and a record door then **splices the checked-out
/// file in place**, leaving a mixed one. git calls all three unmodified. The door first
/// compared bytes against the forms it knew — the checked-out one, then the raw blob —
/// which answered for two of the three: driven at `5f5b273a` under `* text eol=crlf`, a
/// clone's second `jigc milestone add-task` conflict-blocked a record nobody had edited
/// once the cache held no key, the route's `git checkout HEAD --` rewrote nothing (git had
/// no modification to undo), and the re-run blocked again. `1.0.0-rc.24` landed. The
/// fixture that pinned this iterated the two known forms and ran one door once, so it never
/// held the form a clone's first write produces.
///
/// So: every conversion × every form × every way the key is absent — the door lands. And
/// beside it, under the same conversions, the refusal the guard is for: a hand note blocks,
/// the route as printed restores the record, and the re-run lands.
#[test]
fn the_record_door_does_not_false_fire_in_a_crlf_checkout() {
    std::thread::scope(|scope| {
        for conversion in Conversion::ALL {
            scope.spawn(move || {
                let base = TrialCorpus::build(State::CommittedSingletons);
                conversion.apply(&base);
                base.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
                base.jigc_ok(&["milestone", "add-task", MILESTONE, "alpha sharpens a doc"]);
                for form in Form::ALL {
                    for absent in Absent::ALL {
                        let what = format!("{conversion:?}/{form:?}/{absent:?}");
                        let corpus = base.copy_state();
                        let repo = corpus.repo();
                        form.apply(&corpus, &repo, &what);
                        assert_eq!(
                            git(&repo, &["status", "--porcelain", "--", RECORD]),
                            "",
                            "{what}: the premise — git calls the record unmodified",
                        );
                        absent.apply(&corpus);

                        let out = Door::AddTask.run(&corpus, "");
                        assert!(
                            out.status.success(),
                            "{what}: an untouched record is at `HEAD` in every form — no \
                             conflict; {}",
                            text(&out),
                        );
                        assert!(
                            record_at_head(&repo).contains(Door::AddTask.mark()),
                            "{what}: the door's write landed",
                        );
                    }
                }

                // The refusal, under the same conversion, over the mixed form: a hand note
                // in the record's own line ending.
                let what = format!("{conversion:?}: a hand note");
                let corpus = base.copy_state();
                let repo = corpus.repo();
                Form::Mixed.apply(&corpus, &repo, &what);
                Absent::CacheGone.apply(&corpus);
                let before = read(&repo, RECORD);
                let edited = format!("{before}\r\n{HAND}\r\n");
                fs::write(repo.join(RECORD), &edited).expect("write the hand note");
                let head_before = head(&repo);
                let out = Door::AddTask.run(&corpus, "");
                let route = assert_blocked(&repo, &out, &head_before, &edited, &what);
                let span = git_span(&route, &what);
                let restored = run_emitted(span, &corpus.home(), &corpus.home());
                assert!(restored.status.success(), "{what}: {}", text(&restored));
                assert!(
                    !read(&repo, RECORD).contains(HAND)
                        && git(&repo, &["status", "--porcelain", "--", RECORD]).is_empty(),
                    "{what}: the route as printed puts the record back to what `HEAD` holds",
                );
                let rerun = Door::AddTask.run(&corpus, "");
                assert!(
                    rerun.status.success(),
                    "{what}: the re-run lands; {}",
                    text(&rerun)
                );
                let landed = record_at_head(&repo);
                assert!(
                    landed.contains(Door::AddTask.mark()) && !landed.contains(HAND),
                    "{what}: the door's write landed and the note was never merged:\n{landed}",
                );
            });
        }
    });
}

/// **The reported instance, in a real `git clone`** made with `core.autocrlf=true`: the
/// clone's first record op lands and leaves the mixed record; the cache is lost; the second
/// op is the one that blocked with no exit.
#[test]
fn a_converting_clone_serves_the_record_door_twice() {
    let (corpus, _sub) = live_milestone();
    let origin = corpus.repo();
    let clone = origin.parent().expect("the corpus root").join("clone");
    git(
        &origin,
        &[
            "clone",
            "-q",
            "--config",
            "core.autocrlf=true",
            &origin.display().to_string(),
            &clone.display().to_string(),
        ],
    );
    git(&clone, &["config", "user.email", "mate@example.com"]);
    git(&clone, &["config", "user.name", "Mate"]);
    assert!(
        read(&clone, RECORD).contains("\r\n"),
        "the premise: the clone's checkout converted the record",
    );
    for (title, mark) in [
        ("bravo checks the links", "bravo-checks-the-links"),
        ("charlie reads the proofs", "charlie-reads-the-proofs"),
    ] {
        let out = corpus.jigc_stdin_from(&clone, &["milestone", "add-task", MILESTONE, title], "");
        assert!(out.status.success(), "`{title}` lands; {}", text(&out));
        assert!(
            record_at_head(&clone).contains(mark),
            "`{title}` is in `HEAD`"
        );
        assert_eq!(git(&clone, &["status", "--porcelain", "--", RECORD]), "");
        // The cache is gone before the next op, as in a container session.
        let state = clone.join(".jigc").join("state").join("file-state.json");
        if state.exists() {
            fs::remove_file(&state).expect("drop the file-state cache");
        }
    }
}

/// A way git itself rewrites the record's working file while jigc holds its hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rewrite {
    /// A switch to a branch that does not carry the record, and back.
    SwitchAndBack,
    /// An edit to the record, stashed: git checks the record out again.
    Stash,
}

impl Rewrite {
    /// The branch `SwitchAndBack` visits — made before the milestone is minted.
    const ELSEWHERE: &'static str = "before-the-milestone";

    fn apply(self, corpus: &TrialCorpus) {
        match self {
            Rewrite::SwitchAndBack => {
                corpus.git(&["switch", "-q", Self::ELSEWHERE]);
                corpus.git(&["switch", "-q", "-"]);
            }
            Rewrite::Stash => {
                Edit::Note.apply(corpus);
                corpus.git(&["stash", "push", "-q", "--", RECORD]);
                corpus.git(&["stash", "drop", "-q"]);
            }
        }
    }
}

/// [`live_milestone`], with a line-ending conversion in force from before the spec's commit
/// and a branch left behind at the commit before the milestone was minted.
fn live_milestone_under(conversion: Option<Conversion>) -> (TrialCorpus, String) {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    match conversion {
        Some(conversion) => conversion.apply(&corpus),
        None => {
            corpus.git(&["config", "core.autocrlf", "false"]);
        }
    }
    let specs = corpus.repo().join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs");
    fs::write(specs.join("rate-limit.md"), SPEC).expect("write the spec");
    corpus.git(&["add", "--", "docs/specs/rate-limit.md"]);
    corpus.git(&["commit", "-q", "-m", "docs: add the rate-limit spec"]);
    corpus.git(&["branch", Rewrite::ELSEWHERE]);

    corpus.jigc_ok(&["milestone", "create", MILESTONE_TITLE]);
    let ack = corpus.jigc_ok(&["milestone", "add-task", MILESTONE, "alpha sharpens a doc"]);
    let (_, rest) = ack
        .split_once("added task:")
        .unwrap_or_else(|| panic!("`milestone add-task` names its task; got:\n{ack}"));
    let sub = rest
        .split_whitespace()
        .next()
        .expect("the sub-task id")
        .to_owned();
    corpus.set_slot(
        "vision:vision#open-questions",
        &sub,
        "Which domains earn a pack, and when.",
    );
    (corpus, sub)
}

/// The one backticked `jigc unmanage …` span a route carries, as emitted.
fn unmanage_span<'a>(route: &'a str, what: &str) -> &'a str {
    let spans: Vec<&str> = route
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.starts_with("jigc unmanage "))
        .collect();
    assert_eq!(
        spans.len(),
        1,
        "{what}: the held-hash refusal names the exit that works where there is no edit to \
         restore — one `jigc unmanage <record>`; got: {route}",
    );
    spans[0]
}

/// Run an emitted `jigc …` span as printed: a real `sh` word split, then this tree's binary.
fn run_emitted_jigc(corpus: &TrialCorpus, span: &str, what: &str) -> Output {
    let argv = support::shell_words(span, &corpus.repo(), &corpus.home());
    assert_eq!(
        argv[0], "jigc",
        "{what}: the span is a jigc command: {span}"
    );
    let args: Vec<&str> = argv[1..].iter().map(String::as_str).collect();
    corpus.jigc(&args)
}

/// **The held-hash refusal names the exit that works where git has no edit to restore**
/// (the human's ruling of 2026-10-06 on the fix pass's item 12).
///
/// While jigc holds the record's hash, the record door compares bytes and never asks git —
/// that is `design/reconciliation.md`'s declared open bound, and it stands. In a checkout
/// that converts line endings git rewrites the record's working file in its converted form
/// whenever it checks the record out (a branch switch and back, a stash), so the door
/// refuses as *edited out of band* over a record git calls unmodified, and the restore the
/// route printed, `git checkout -- <record>`, has nothing to restore. The route now also
/// names `jigc unmanage <record>`: it drops the held hash, the next run compares the record
/// with `HEAD` through git, and lands.
///
/// Every conversion × every door that rewrites the record after a switch and back; a stash
/// under every conversion; each printed command run as printed.
#[test]
fn the_held_hash_refusal_in_a_converting_checkout_names_the_exit_that_works() {
    std::thread::scope(|scope| {
        for conversion in Conversion::ALL {
            scope.spawn(move || {
                let (base, sub) = live_milestone_under(Some(conversion));
                let cells = Door::ALL
                    .into_iter()
                    .map(|door| (door, Rewrite::SwitchAndBack))
                    .chain([(Door::AddTask, Rewrite::Stash)]);
                for (door, rewrite) in cells {
                    let what = format!("{conversion:?}/{door:?}/{rewrite:?}");
                    let corpus = base.copy_state();
                    let repo = corpus.repo();
                    assert!(
                        !read(&repo, RECORD).contains("\r\n"),
                        "{what}: the premise — jigc wrote the record with `\\n` endings",
                    );
                    rewrite.apply(&corpus);
                    let rewritten = read(&repo, RECORD);
                    assert!(
                        rewritten.contains("\r\n") && recorded(&corpus).is_some(),
                        "{what}: the premise — git rewrote the record in its converted form \
                         while jigc held its hash",
                    );
                    assert_eq!(
                        git(&repo, &["status", "--porcelain", "--", RECORD]),
                        "",
                        "{what}: the premise — git calls the record unmodified",
                    );
                    let head_before = head(&repo);

                    let out = door.run(&corpus, &sub);
                    let route = assert_blocked(&repo, &out, &head_before, &rewritten, &what);
                    assert!(
                        String::from_utf8_lossy(&out.stderr).contains("since jigc last wrote it"),
                        "{what}: the held-hash presentation; {}",
                        text(&out),
                    );

                    // The restore the route has always printed: it runs, and there is
                    // nothing for it to restore.
                    let restore = route
                        .split('`')
                        .skip(1)
                        .step_by(2)
                        .find(|span| span.starts_with("git ") && span.contains(" checkout "))
                        .unwrap_or_else(|| panic!("{what}: the route's restore; got: {route}"));
                    let restored = run_emitted(restore, &corpus.home(), &corpus.home());
                    assert!(restored.status.success(), "{what}: {}", text(&restored));
                    assert_eq!(
                        read(&repo, RECORD),
                        rewritten,
                        "{what}: the premise — git has no edit to restore",
                    );

                    // The check the route names for telling the two cases apart, as printed.
                    let status = route
                        .split('`')
                        .skip(1)
                        .step_by(2)
                        .find(|span| span.starts_with("git ") && span.contains(" status "))
                        .unwrap_or_else(|| panic!("{what}: the route's check; got: {route}"));
                    let asked = run_emitted(status, &corpus.home(), &corpus.home());
                    assert!(
                        asked.status.success() && asked.stdout.is_empty(),
                        "{what}: `{status}` prints nothing here; {}",
                        text(&asked),
                    );

                    // The exit, as printed — and the same door, re-run, lands.
                    let exit = unmanage_span(&route, &what);
                    let dropped = run_emitted_jigc(&corpus, exit, &what);
                    assert!(
                        dropped.status.success(),
                        "{what}: `{exit}`; {}",
                        text(&dropped)
                    );
                    assert_eq!(recorded(&corpus), None, "{what}: the held hash is dropped");
                    let rerun = door.run(&corpus, &sub);
                    assert!(
                        rerun.status.success(),
                        "{what}: the re-run lands; {}",
                        text(&rerun),
                    );
                    assert!(
                        record_at_head(&repo).contains(door.mark()),
                        "{what}: the door's own write is in the committed record",
                    );
                    assert_ne!(head(&repo), head_before, "{what}: the re-run committed");
                }
            });
        }
    });
}

/// **Beside it, what must not change.** Without a conversion git's rewrite leaves the bytes
/// jigc wrote, so a switch and back or a stash refuses nothing. And the exit the route now
/// names is no way past a real edit: with the held hash dropped, the door asks git, git
/// calls the record modified, and the door refuses again over the same untouched bytes.
#[test]
fn the_unmanage_exit_neither_fires_without_a_conversion_nor_passes_a_real_edit() {
    let (base, sub) = live_milestone_under(None);
    for rewrite in [Rewrite::SwitchAndBack, Rewrite::Stash] {
        for door in Door::ALL {
            let what = format!("no conversion/{door:?}/{rewrite:?}");
            let corpus = base.copy_state();
            rewrite.apply(&corpus);
            let out = door.run(&corpus, &sub);
            assert!(
                out.status.success(),
                "{what}: git rewrote the bytes jigc wrote — no refusal; {}",
                text(&out),
            );
        }
    }

    for conversion in [None, Some(Conversion::AutocrlfTrue)] {
        let what = format!("{conversion:?}: a real edit");
        let (corpus, sub) = live_milestone_under(conversion);
        let repo = corpus.repo();
        Edit::Note.apply(&corpus);
        let edited = read(&repo, RECORD);
        let head_before = head(&repo);
        let out = Door::AddTask.run(&corpus, &sub);
        let route = assert_blocked(&repo, &out, &head_before, &edited, &what);
        let exit = unmanage_span(&route, &what);
        let dropped = run_emitted_jigc(&corpus, exit, &what);
        assert!(dropped.status.success(), "{what}: {}", text(&dropped));

        let rerun = Door::AddTask.run(&corpus, &sub);
        let route = assert_blocked(&repo, &rerun, &head_before, &edited, &what);
        assert!(
            String::from_utf8_lossy(&rerun.stderr).contains("differs from what `HEAD` holds")
                && route.contains("checkout HEAD -- "),
            "{what}: with the hash dropped the door asks git, and git calls the record \
             modified; {}",
            text(&rerun),
        );
    }
}

/// **The statement, where the rule is described.** The design sentences this fix makes
/// true name what the code now does, and the open question it leaves is stated as open.
#[test]
fn the_design_states_the_record_doors_witness() {
    let root = format!("{}/../..", env!("CARGO_MANIFEST_DIR"));
    let doc = |rel: &str| {
        fs::read_to_string(format!("{root}/{rel}")).unwrap_or_else(|e| panic!("read {rel}: {e}"))
    };
    let reconciliation = doc("design/reconciliation.md");
    for needle in [
        "its witness is `HEAD`",
        "git checkout HEAD -- <record>",
        "The witness answers only where there is no key",
        "A home git holds and the disk does not, with no baseline",
        "The route names that exit itself",
    ] {
        assert!(
            reconciliation.contains(needle),
            "design/reconciliation.md states `{needle}`",
        );
    }
    assert!(
        !reconciliation.contains("The milestone-record door with no baseline"),
        "the open question this fix closes is no longer listed as open",
    );
    assert!(
        doc("design/team-ready-state.md").contains("does not depend on the gitignored cache"),
        "design/team-ready-state.md → No-silent-overwrite discipline states the no-key rule",
    );
    assert!(
        doc("crates/cli/guides/MIGRATING.md").contains("checkout HEAD -- <record>"),
        "MIGRATING.md item 8 states the record's route",
    );
}
