//! **Branch and pull** — the L1 pull and the L2 branch switch as shared fixture acts
//! ([findings-channel.md](../../../../design/findings-channel.md) → §6 L1/L2, §11 Flow E).
//!
//! Moved here from `l1_pull_absorption` and `l2_branch_switch` (both `g_finalize`) at M55
//! Increment 11 / T5, so `flow59_branch_and_pull` (`g_methodology`) drives the same acts in
//! one corpus's history rather than a second copy of them: a `pub(crate)` item in a suite is
//! visible only inside its own group target, and this module is compiled into every one.
//!
//! - **The pull** is real: [`Team::new`] wires the corpus to a local bare `origin` and gives it
//!   a second clone — the teammate — that commits and pushes; [`Team::pull_teammate_edit`]
//!   brings that commit into the corpus by `git pull --ff-only`.
//! - **The branch switch** is [`adr_on_milestone_branch`]: an ADR finalized on
//!   [`MILESTONE_BRANCH`], left checked out, so a `git switch main` removes the file and leaves
//!   its baseline behind.
//! - **The task** is [`start_task`]: a `single-task` with its commit doc filled, ready to
//!   write [`TASK_PROSE`] into `vision#open-questions`.
//! - **The readers** pick rows out of a `--format json` findings envelope: [`rows_at`] by
//!   `(code, address)`, [`rename_rows`] by the ADR's `(code, target)` key.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus, read};

/// The managed doc the teammate edits and the task touches.
pub const VISION: &str = "VISION.md";

/// The line the teammate's commit adds to `VISION.md`'s thesis — conformant prose.
pub const TEAMMATE_LINE: &str = "A teammate's pulled line.";

/// The prose a task writes into `vision#open-questions`.
pub const TASK_PROSE: &str = "Which domains earn a pack, and when.";

/// The informational route the L1 store arm carries — the design's words, verbatim.
pub const LAG_ROUTE: &str = "the baseline lags `HEAD`; absorbed at the next finalize";

/// The ADR the milestone branch creates, at its committed home.
pub const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// The milestone branch the ADR is created on.
pub const MILESTONE_BRANCH: &str = "milestone/x/main";

/// The corpus (clone A) wired to a local bare `origin`, plus clone B — the teammate.
pub struct Team {
    pub corpus: TrialCorpus,
    pub teammate: PathBuf,
}

impl Team {
    /// Build a baselined `[dev ▸ methodology]` corpus ([`State::CommittedSingletons`]) and
    /// give it a teammate.
    pub fn new() -> Self {
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
    pub fn pull_teammate_edit(&self, rel: &str, edit: impl Fn(&str) -> String) {
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
    pub fn pull_vision_edit(&self) {
        self.pull_teammate_edit(VISION, add_teammate_line);
    }

    pub fn repo(&self) -> PathBuf {
        self.corpus.repo()
    }
}

/// Run `git <args>` in `dir` under the corpus's `$HOME`, asserting success.
pub fn git_in(dir: &Path, corpus: &TrialCorpus, args: &[&str]) -> String {
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

/// The teammate's edit: one conformant line appended to the thesis.
pub fn add_teammate_line(vision: &str) -> String {
    vision.replacen(
        "A deterministic CLI assembles exactly the context a task needs.\n",
        &format!(
            "A deterministic CLI assembles exactly the context a task needs.\n{TEAMMATE_LINE}\n"
        ),
        1,
    )
}

/// The commits on `HEAD`.
pub fn commit_count(corpus: &TrialCorpus) -> u32 {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a commit count")
}

/// Mint a `single-task`, fill its commit doc, and return its id.
pub fn start_task(corpus: &TrialCorpus, intent: &str) -> String {
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

/// `jigc validate --format json` over the corpus: the store sweep is report-only, so it
/// exits 0.
pub fn store_sweep(corpus: &TrialCorpus, context: &str) -> serde_json::Value {
    let out = corpus.jigc(&["validate", "--format", "json"]);
    stdout_json(&out, &[0], context)
}

/// Every finding in `envelope` carrying `code` at `address`.
pub fn rows_at<'a>(
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

/// The probes an envelope names as blocking.
pub fn blocking_probes(envelope: &serde_json::Value) -> Vec<&str> {
    envelope["blocking_probes"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `blocking_probes`: {envelope}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect()
}

/// The dangling-baseline route at both scopes (M55 Increment 5 / T1), byte for byte.
pub fn branch_switch_route() -> String {
    format!(
        "nothing on this checkout needs to change — a branch switch left this baseline \
         behind, and {ADR_PATH} lives on a branch this checkout does not carry: switch back \
         to that branch to work on it again"
    )
}

/// Switch `corpus` to a new [`MILESTONE_BRANCH`] and finalize an ADR there through a
/// `single-task`; the branch is left checked out, so the ADR is on disk and baselined.
pub fn adr_on_milestone_branch(corpus: &TrialCorpus) {
    corpus.git(&["switch", "-q", "-c", MILESTONE_BRANCH]);
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
        corpus.set_slot(&format!("adr:single-node-cache#{slot}"), &task, prose);
    }
    corpus.finalize(&task, "cache", "record the cache decision", false);
    assert!(
        corpus.repo().join(ADR_PATH).exists(),
        "the milestone branch's finalize promotes {ADR_PATH}",
    );
}

/// Every `reconciliation.rename` row in `envelope` keyed at the ADR's path.
pub fn rename_rows(envelope: &serde_json::Value) -> Vec<serde_json::Value> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `findings`: {envelope}"))
        .iter()
        .filter(|f| f["key"]["code"] == "reconciliation.rename" && f["key"]["target"] == ADR_PATH)
        .cloned()
        .collect()
}

/// The single advisory branch-switch row in `envelope`, asserted whole: advisory, routed at
/// the branch switch, and naming `jigc unmanage` in neither its message nor its route.
pub fn branch_switch_row(envelope: &serde_json::Value, what: &str) -> serde_json::Value {
    let rows = rename_rows(envelope);
    assert_eq!(
        rows.len(),
        1,
        "{what}: one `reconciliation.rename` row at {ADR_PATH}; got {envelope}"
    );
    let row = rows[0].clone();
    assert_eq!(row["severity"], "advisory", "{what}: advisory: {row}");
    let route = row["route"].as_str().expect("the row carries a route");
    let message = row["message"].as_str().expect("the row carries a message");
    assert!(
        !route.contains("jigc unmanage") && !message.contains("jigc unmanage"),
        "{what}: a branch switch never routes at an index drop: {row}",
    );
    assert_eq!(
        route,
        branch_switch_route(),
        "{what}: routed at the branch switch: {row}"
    );
    row
}
