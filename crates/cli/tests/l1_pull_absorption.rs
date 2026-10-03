//! M55 Increment 4 — **L1 pull absorption, at the task scope (T1) and the store's (T2)**
//! ([findings-channel.md](../../../design/findings-channel.md) → §6, §10, §11 Flow E;
//! [reconciliation.md](../../../design/reconciliation.md) → the `DRIFTED + TOUCHED` row).
//!
//! A teammate's committed edit to a managed doc reaches this clone by `git pull`, which
//! moves the file and leaves the gitignored `file-state` baseline where the last landed
//! finalize put it. Until this task the next task that edited the doc met
//! `DRIFTED + TOUCHED` and was refused `reconciliation.conflict-block`, routed at reverting
//! the teammate's edit. The arm now asks one more fact — the doc's blob at the task's
//! **base pin** — and when the on-disk bytes equal it, the drift predates the task and is
//! absorbed through the untouched arm's whole body.
//!
//! The pull is real: a local bare `origin`, this corpus as clone A, a second clone B that
//! commits and pushes, and clone A that `git pull --ff-only`s. Clone A is a
//! `[dev ▸ methodology]` corpus whose `VISION.md` a landed finalize has baselined
//! ([`State::CommittedSingletons`]).
//!
//! - **(a)** after the pull, a `single-task` editing `vision#open-questions` reads advisory
//!   `reconciliation.absorb` at `VISION.md` from `task validate`, and `task finalize` lands
//!   a commit carrying both the teammate's line and the task's prose;
//! - **(b)** `start` and `task validate` leave `file-state.json` byte-identical (D10 —
//!   pure readers stay pure);
//! - **(c)** the control: an uncommitted edit made after the mint still conflict-blocks;
//! - **(d)** the join: a sub-task's pin is its milestone's, so a pull after
//!   `milestone create` is refused and a pull before it lands, leaving no drift behind;
//! - **(e)** the record door: a pulled edit to the milestone record still conflict-blocks
//!   the next `add-task` (F3 — the door passes no pin).
//!
//! **T2 — the store arm** ([validation.md](../../../design/validation.md) → the reused
//! `hash-matches` id's store-scope route). The same pull, read by `jigc validate`, is a
//! baseline that lags `HEAD`, not an out-of-band edit:
//!
//! - **(f)** after the pull, `jigc validate` exits 0 with the `VISION.md` row advisory,
//!   routed *the baseline lags `HEAD`; absorbed at the next finalize*, and no `file-state`
//!   in `blocking_probes`;
//! - **(g)** after the task that edits it lands, the next sweep has no row for `VISION.md`;
//! - **(h)** the control: a non-conformant edit committed with plain git keeps its
//!   conformance finding and its blocking drift, and is not baselined;
//! - **(i)** an uncommitted edit keeps the blocking drift.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus};

/// The managed doc the teammate edits and the task touches.
const VISION: &str = "VISION.md";

/// The line the teammate's commit adds to `VISION.md`'s thesis — conformant prose.
const TEAMMATE_LINE: &str = "A teammate's pulled line.";

/// The prose the task writes into `vision#open-questions`.
const TASK_PROSE: &str = "Which domains earn a pack, and when.";

/// The file-state record every assertion about baselines reads.
const FILE_STATE: &str = ".jigc/state/file-state.json";

/// Clone A (the corpus) wired to a local bare `origin`, plus clone B — the teammate.
struct Team {
    corpus: TrialCorpus,
    teammate: PathBuf,
}

impl Team {
    /// Build a baselined `[dev ▸ methodology]` corpus and give it a teammate.
    fn new() -> Self {
        let corpus = TrialCorpus::build(State::CommittedSingletons);
        let root = corpus
            .repo()
            .parent()
            .expect("the corpus root")
            .to_path_buf();
        let origin = root.join("origin.git");
        let teammate = root.join("teammate");
        git_in(
            &root,
            &corpus,
            &["clone", "-q", "--bare", "repo", "origin.git"],
        );
        corpus.git(&["remote", "add", "origin", path_str(&origin)]);
        corpus.git(&["fetch", "-q", "origin"]);
        corpus.git(&["branch", "-q", "--set-upstream-to=origin/main", "main"]);
        git_in(&root, &corpus, &["clone", "-q", "origin.git", "teammate"]);
        git_in(
            &teammate,
            &corpus,
            &["config", "user.email", "mate@example.com"],
        );
        git_in(&teammate, &corpus, &["config", "user.name", "Teammate"]);
        Team { corpus, teammate }
    }

    /// The teammate edits `rel` with `edit`, commits and pushes it, and clone A pulls it.
    /// Clone A pushes first, so whatever it committed since (a milestone record) is what
    /// the teammate builds on and the pull is a fast-forward.
    fn pull_teammate_edit(&self, rel: &str, edit: impl Fn(&str) -> String) {
        self.corpus.git(&["push", "-q", "origin", "main"]);
        git_in(&self.teammate, &self.corpus, &["pull", "-q", "--ff-only"]);
        let path = self.teammate.join(rel);
        let before = fs::read_to_string(&path).expect("read the teammate's copy");
        let after = edit(&before);
        assert_ne!(before, after, "the teammate's edit must change {rel}");
        fs::write(&path, after).expect("write the teammate's edit");
        git_in(
            &self.teammate,
            &self.corpus,
            &["commit", "-qam", "docs: teammate edit"],
        );
        git_in(
            &self.teammate,
            &self.corpus,
            &["push", "-q", "origin", "main"],
        );
        self.corpus.git(&["pull", "-q", "--ff-only"]);
        assert_eq!(
            read(&self.corpus.repo(), rel),
            read(&self.teammate, rel),
            "the pull brought the teammate's {rel} into clone A",
        );
    }

    /// The teammate's conformant `VISION.md` edit, pulled.
    fn pull_vision_edit(&self) {
        self.pull_teammate_edit(VISION, add_teammate_line);
    }

    fn repo(&self) -> PathBuf {
        self.corpus.repo()
    }
}

/// Run `git <args>` in `dir` under the corpus's `$HOME`, asserting success.
fn git_in(dir: &Path, corpus: &TrialCorpus, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("HOME", corpus.home())
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {args:?} in {} failed: {}",
        dir.display(),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn path_str(path: &Path) -> &str {
    path.to_str().expect("a UTF-8 temp path")
}

fn read(dir: &Path, rel: &str) -> String {
    fs::read_to_string(dir.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

/// The teammate's edit: one conformant line appended to the thesis.
fn add_teammate_line(vision: &str) -> String {
    vision.replacen(
        "A deterministic CLI assembles exactly the context a task needs.\n",
        &format!(
            "A deterministic CLI assembles exactly the context a task needs.\n{TEAMMATE_LINE}\n"
        ),
        1,
    )
}

/// The `file-state` record's bytes, as a pure reader must leave them.
fn file_state(repo: &Path) -> Vec<u8> {
    fs::read(repo.join(FILE_STATE)).expect("read the file-state record")
}

fn commit_count(corpus: &TrialCorpus) -> u32 {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a commit count")
}

/// Mint a `single-task`, fill its commit doc, and return its id.
fn start_task(corpus: &TrialCorpus, intent: &str) -> String {
    let task = corpus.start_workflow("single-task", intent);
    fill_commit(corpus, &task);
    task
}

fn fill_commit(corpus: &TrialCorpus, task: &str) {
    corpus.set_field(&format!("commit:{task}#type"), task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), task, "vision");
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        task,
        "sharpen the open questions",
    );
}

/// `(code, severity, address)` of every finding in a `--format json` envelope.
fn findings(envelope: &serde_json::Value) -> Vec<(String, String, String)> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `findings`: {envelope}"))
        .iter()
        .map(|f| {
            (
                f["code"].as_str().unwrap_or_default().to_string(),
                f["severity"].as_str().unwrap_or_default().to_string(),
                f["location"]["address"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .collect()
}

fn has_code(findings: &[(String, String, String)], code: &str) -> bool {
    findings.iter().any(|(c, _, _)| c == code)
}

fn rendered(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// **(a)** A pulled `VISION.md` edit no longer wedges the task that edits the doc: `task
/// validate` reads the absorb advisory and no conflict-block, and `task finalize` lands a
/// commit carrying both sides — the teammate's line and the task's prose. Red before M55
/// Increment 4 with `reconciliation.conflict-block`.
#[test]
fn a_pulled_edit_lands_through_the_task_that_edits_the_doc() {
    let team = Team::new();
    let corpus = &team.corpus;
    team.pull_vision_edit();

    let task = start_task(corpus, "sharpen the open questions");
    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);

    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let validated: serde_json::Value = stdout_json(&out, &[0], "task validate after the pull");
    let rows = findings(&validated);
    assert!(
        rows.contains(&(
            "reconciliation.absorb".into(),
            "advisory".into(),
            VISION.into()
        )),
        "the pulled edit reads as an advisory absorb at `{VISION}`; got {rows:?}",
    );
    assert!(
        !has_code(&rows, "reconciliation.conflict-block"),
        "a pulled edit is not a conflict; got {rows:?}",
    );

    let before = commit_count(corpus);
    let out = corpus.jigc(&["task", "finalize", &task]);
    assert_eq!(
        out.status.code(),
        Some(i32::from(cli::task::EXIT_SUCCESS)),
        "finalize lands over the pulled edit:\n{}",
        rendered(&out),
    );
    assert_eq!(commit_count(corpus), before + 1, "one commit lands");
    let landed = corpus.git(&["show", "HEAD:VISION.md"]);
    assert!(
        landed.contains(TEAMMATE_LINE) && landed.contains(TASK_PROSE),
        "the landed doc carries the teammate's line and the task's prose; got:\n{landed}",
    );
}

/// **(b)** D10: `start` and `task validate` are pure readers — the absorb they report is
/// not written to the baseline; only a landed finalize advances it.
#[test]
fn start_and_task_validate_leave_the_baseline_untouched() {
    let team = Team::new();
    let corpus = &team.corpus;
    team.pull_vision_edit();
    let at_pull = file_state(&team.repo());

    let task = corpus.start_workflow("single-task", "sharpen the open questions");
    assert_eq!(
        file_state(&team.repo()),
        at_pull,
        "`start` writes no baseline"
    );

    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);
    let at_touch = file_state(&team.repo());
    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let validated: serde_json::Value =
        stdout_json(&out, &[0, 3], "task validate over the pulled edit");
    assert!(
        has_code(&findings(&validated), "reconciliation.absorb"),
        "the sweep did reach the absorb arm; got {validated}",
    );
    assert_eq!(
        file_state(&team.repo()),
        at_touch,
        "`task validate` reports the absorb and writes no baseline",
    );
}

/// **(c)** The control: an edit made **after** the mint is not at the pin — the task's base
/// predates it — so both sides moved and the finalize is refused, exit 3, nothing
/// committed.
#[test]
fn an_edit_made_during_the_task_still_conflict_blocks() {
    let team = Team::new();
    let corpus = &team.corpus;

    let task = start_task(corpus, "sharpen the open questions");
    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);
    let path = team.repo().join(VISION);
    let edited = add_teammate_line(&read(&team.repo(), VISION));
    fs::write(&path, edited).expect("an uncommitted edit after the mint");

    let before = commit_count(corpus);
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let refused: serde_json::Value = stdout_json(
        &out,
        &[i32::from(cli::task::EXIT_VALIDATION_BLOCKED)],
        "finalize over an edit made during the task",
    );
    assert!(
        findings(&refused).contains(&(
            "reconciliation.conflict-block".into(),
            "blocking".into(),
            VISION.into()
        )),
        "an edit made during the task conflict-blocks at `{VISION}`; got {refused}",
    );
    assert_eq!(
        commit_count(corpus),
        before,
        "a conflict-block commits nothing"
    );
}

/// Mint a milestone with one sub-task that edits `vision#open-questions`; returns the
/// milestone id.
fn milestone_editing_vision(corpus: &TrialCorpus) -> String {
    corpus.jigc_ok(&["milestone", "create", "Vision pass"]);
    let milestone = "vision-pass".to_string();
    let added = corpus.jigc_ok(&[
        "milestone",
        "add-task",
        &milestone,
        "Sharpen the open questions",
    ]);
    let sub = added
        .lines()
        .find_map(|l| l.strip_prefix("added task:"))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("add-task prints the sub-task it added; got:\n{added}"))
        .to_string();
    corpus.set_slot("vision:vision#open-questions", &sub, TASK_PROSE);
    milestone
}

/// **(d)** The join binds the seam to the milestone's base — the pin every sub-task
/// inherits. A pull **after** `milestone create` moved the doc past that pin, so the join
/// refuses it; a pull **before** create is at the pin and lands, and the landed boundary
/// leaves no `VISION.md` drift for the next store sweep.
#[test]
fn the_join_absorbs_a_pull_before_the_milestone_and_refuses_one_after() {
    // A pull after create: the sub-task's pin predates the teammate's edit.
    let after = Team::new();
    let milestone = milestone_editing_vision(&after.corpus);
    after.pull_vision_edit();
    let before = commit_count(&after.corpus);
    let out = after
        .corpus
        .jigc(&["milestone", "finalize", &milestone, "--format", "json"]);
    let refused: serde_json::Value = stdout_json(
        &out,
        &[i32::from(cli::task::EXIT_VALIDATION_BLOCKED)],
        "milestone finalize over a pull after create",
    );
    assert!(
        findings(&refused).contains(&(
            "reconciliation.conflict-block".into(),
            "blocking".into(),
            VISION.into()
        )),
        "a pull after `milestone create` conflict-blocks the join; got {refused}",
    );
    assert_eq!(
        commit_count(&after.corpus),
        before,
        "a refused join commits nothing"
    );

    // A pull before create: the milestone's base already carries the teammate's edit.
    let ahead = Team::new();
    ahead.pull_vision_edit();
    let milestone = milestone_editing_vision(&ahead.corpus);
    let out = ahead.corpus.jigc(&["milestone", "finalize", &milestone]);
    assert_eq!(
        out.status.code(),
        Some(i32::from(cli::task::EXIT_SUCCESS)),
        "a pull before `milestone create` lands at the join:\n{}",
        rendered(&out),
    );
    let landed = ahead.corpus.git(&["show", "HEAD:VISION.md"]);
    assert!(
        landed.contains(TEAMMATE_LINE) && landed.contains(TASK_PROSE),
        "the boundary carries the teammate's line and the sub-task's prose; got:\n{landed}",
    );
    let out = ahead.corpus.jigc(&["validate", "--format", "json"]);
    let swept: serde_json::Value = stdout_json(&out, &[0], "the store sweep after the join");
    assert!(
        !findings(&swept)
            .iter()
            .any(|(code, _, at)| code == "file-state.hash-matches" && at == VISION),
        "the landed boundary re-baselined `{VISION}`; got {swept}",
    );
}

/// **(e)** The record door passes no pin (P3): a teammate's edit to the machine-owned
/// milestone record, pulled, still conflict-blocks the next `add-task` — F3's *"detected +
/// conflict-blocked, not absorbed"*.
#[test]
fn a_pulled_milestone_record_edit_still_conflict_blocks_add_task() {
    let team = Team::new();
    let corpus = &team.corpus;
    corpus.jigc_ok(&["milestone", "create", "Vision pass"]);
    corpus.jigc_ok(&[
        "milestone",
        "add-task",
        "vision-pass",
        "Warm the read cache",
    ]);
    let record = "docs/milestone-records/vision-pass.md";
    team.pull_teammate_edit(record, |body| {
        body.replacen("status: active", "status: joined", 1)
    });
    let pulled = read(&team.repo(), record);

    let before = commit_count(corpus);
    let out = corpus.jigc(&["milestone", "add-task", "vision-pass", "Evict cold entries"]);
    assert!(
        !out.status.success(),
        "add-task over a pulled record edit is refused:\n{}",
        rendered(&out),
    );
    let text = rendered(&out);
    assert!(
        text.contains("reconciliation.conflict-block") && text.contains(record),
        "the refusal is the record's conflict-block; got:\n{text}",
    );
    assert_eq!(
        read(&team.repo(), record),
        pulled,
        "the record is untouched"
    );
    assert_eq!(
        commit_count(corpus),
        before,
        "a refused add-task commits nothing"
    );
}

// ───────── T2 — the store arm: the baseline lags `HEAD` ─────────

/// The informational route the store arm carries — the design's words, verbatim.
const LAG_ROUTE: &str = "the baseline lags `HEAD`; absorbed at the next finalize";

/// `jigc validate --format json` over clone A: the store sweep is report-only, so it exits 0.
fn store_sweep(corpus: &TrialCorpus, context: &str) -> serde_json::Value {
    let out = corpus.jigc(&["validate", "--format", "json"]);
    stdout_json(&out, &[0], context)
}

/// Every finding in `envelope` carrying `code` at `address`.
fn rows_at<'a>(
    envelope: &'a serde_json::Value,
    code: &str,
    address: &str,
) -> Vec<&'a serde_json::Value> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `findings`: {envelope}"))
        .iter()
        .filter(|f| f["code"] == code && f["location"]["address"] == address)
        .collect()
}

fn blocking_probes(envelope: &serde_json::Value) -> Vec<&str> {
    envelope["blocking_probes"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `blocking_probes`: {envelope}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect()
}

/// **(f)** Flow E's L1 half at store scope: after the pull, `jigc validate` exits 0 and the
/// `VISION.md` row is `file-state.hash-matches` at **advisory**, routed *the baseline lags
/// `HEAD`* — a pull, not an out-of-band edit — so `file-state` leaves `blocking_probes`. The
/// sweep stays a pure reader: the baseline is not advanced. Red before M55 Increment 4 / T2
/// with the row blocking and routed at re-authoring the teammate's edit.
#[test]
fn after_a_pull_the_store_sweep_reports_the_lagging_baseline_advisory() {
    let team = Team::new();
    team.pull_vision_edit();
    let at_pull = file_state(&team.repo());

    let swept = store_sweep(&team.corpus, "the store sweep after the pull");
    let rows = rows_at(&swept, "file-state.hash-matches", VISION);
    assert_eq!(rows.len(), 1, "one drift row for `{VISION}`; got {swept}");
    assert_eq!(
        rows[0]["severity"], "advisory",
        "the lag is advisory: {}",
        rows[0]
    );
    assert_eq!(rows[0]["route"], LAG_ROUTE, "the lag route: {}", rows[0]);
    assert!(
        !blocking_probes(&swept).contains(&"file-state"),
        "an advisory lag blocks nothing; got {swept}",
    );
    assert_eq!(
        file_state(&team.repo()),
        at_pull,
        "`jigc validate` reports the lag and writes no baseline",
    );
}

/// **(g)** The advisory's promise is kept: the task that edits the pulled doc lands, and the
/// next `jigc validate` carries no `file-state.hash-matches` row for `VISION.md` — the landed
/// finalize advanced the baseline past the pulled bytes.
#[test]
fn after_the_landed_finalize_the_store_sweep_no_longer_reports_the_lag() {
    let team = Team::new();
    let corpus = &team.corpus;
    team.pull_vision_edit();
    let before = store_sweep(corpus, "the store sweep after the pull");
    assert_eq!(
        rows_at(&before, "file-state.hash-matches", VISION).len(),
        1,
        "precondition: the pull reads as a lagging baseline; got {before}",
    );

    let task = start_task(corpus, "sharpen the open questions");
    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);
    let out = corpus.jigc(&["task", "finalize", &task]);
    assert_eq!(
        out.status.code(),
        Some(i32::from(cli::task::EXIT_SUCCESS)),
        "finalize lands over the pulled edit:\n{}",
        rendered(&out),
    );

    let after = store_sweep(corpus, "the store sweep after the landed finalize");
    assert!(
        rows_at(&after, "file-state.hash-matches", VISION).is_empty(),
        "the landed finalize re-baselined `{VISION}`; got {after}",
    );
}

/// **(h)** The control on the store arm's population: bytes at `HEAD` are not on their own a
/// clean doc. A hand edit that breaks `VISION.md`'s conformance, committed with plain git,
/// keeps its family-5 conformance finding **and** its blocking `hash-matches` — the lag is
/// graded behind the conformance check — and the sweep baselines nothing.
#[test]
fn a_non_conformant_edit_committed_with_plain_git_keeps_blocking() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    let broken = read(&repo, VISION).replacen("## Open Questions\n", "## Open Puzzles\n", 1);
    assert_ne!(
        broken,
        read(&repo, VISION),
        "the fixture's heading is there to break"
    );
    fs::write(repo.join(VISION), broken).expect("write the non-conformant edit");
    corpus.git(&["commit", "-qam", "docs: hand edit"]);
    let at_commit = file_state(&repo);

    let swept = store_sweep(
        &corpus,
        "the store sweep over a committed non-conformant edit",
    );
    let conformance = rows_at(
        &swept,
        "conformance.section-renamed",
        "vision:vision#open-questions",
    );
    assert_eq!(
        conformance.len(),
        1,
        "the conformance finding stands; got {swept}"
    );
    assert_eq!(conformance[0]["severity"], "blocking", "{}", conformance[0]);
    let rows = rows_at(&swept, "file-state.hash-matches", VISION);
    assert_eq!(rows.len(), 1, "one drift row for `{VISION}`; got {swept}");
    assert_eq!(
        rows[0]["severity"], "blocking",
        "not graded a lag: {}",
        rows[0]
    );
    assert_ne!(rows[0]["route"], LAG_ROUTE, "{}", rows[0]);
    assert!(
        blocking_probes(&swept).contains(&"file-state"),
        "the drift still blocks; got {swept}",
    );
    assert_eq!(
        file_state(&repo),
        at_commit,
        "a non-conformant edit is not baselined",
    );
}

/// **(i)** An uncommitted edit is not at `HEAD`: the store sweep keeps today's blocking drift,
/// routed at reviewing the out-of-band edit.
#[test]
fn an_uncommitted_edit_keeps_the_blocking_drift() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    fs::write(repo.join(VISION), add_teammate_line(&read(&repo, VISION)))
        .expect("an uncommitted edit");

    let swept = store_sweep(&corpus, "the store sweep over an uncommitted edit");
    let rows = rows_at(&swept, "file-state.hash-matches", VISION);
    assert_eq!(rows.len(), 1, "one drift row for `{VISION}`; got {swept}");
    assert_eq!(rows[0]["severity"], "blocking", "{}", rows[0]);
    assert_ne!(rows[0]["route"], LAG_ROUTE, "{}", rows[0]);
    assert!(
        blocking_probes(&swept).contains(&"file-state"),
        "the drift blocks; got {swept}",
    );
}
