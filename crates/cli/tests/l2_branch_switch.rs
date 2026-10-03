//! M55 Increment 5 / T2 — **L2: the branch switch, and the one route at both scopes**
//! ([findings-channel.md](../../../design/findings-channel.md) → §6 L2, §10, §11 Flow E;
//! [validation.md](../../../design/validation.md) → Exit semantics).
//!
//! Under the branch-per-milestone model a managed doc is created and finalized on a
//! milestone branch, and the landed finalize baselines it in the gitignored `file-state`
//! cache. A `git switch main` removes the file and leaves the baseline: the recorded path is
//! missing, and `HEAD` carries no history for it — nothing was deleted, the doc lives on the
//! branch the checkout left. Task scope has graded that cell advisory since M45; until this
//! task the store sweep graded it the blocking weak deletion and `jigc validate` exited 1,
//! routed at `jigc unmanage` — dropping the identity of a doc alive on its branch.
//!
//! Driven through the real binary over a [`State::CommittedSingletons`] corpus:
//!
//! - **(a)** an ADR finalized on `milestone/x/main`, then `git switch main` → `jigc
//!   validate` exits 0, `report_only: true`, one advisory `reconciliation.rename` at the ADR
//!   path routed at the branch switch; neither its message nor its route names `jigc
//!   unmanage`. Red before this task: exit 1, blocking.
//! - **(b)** a task's `task validate` over the same state carries the same `(code, target)`
//!   key at advisory, message and route byte-identical to (a)'s — one producer, one route.
//! - **(c)** switching back clears the row: the route's promise, driven.
//! - **(d)** the control: a managed doc with history, `git rm`'d and committed, still blocks
//!   — exit 1, the blocking weak finding, the `oob-rename` closing line.

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus};

/// The ADR the milestone branch creates, at its committed home.
const ADR_PATH: &str = "docs/decisions/single-node-cache.md";

/// The milestone branch the ADR is created on.
const MILESTONE_BRANCH: &str = "milestone/x/main";

/// The dangling-baseline route at both scopes (M55 Increment 5 / T1), byte for byte.
fn branch_switch_route() -> String {
    format!(
        "nothing on this checkout needs to change — a branch switch left this baseline \
         behind, and {ADR_PATH} lives on a branch this checkout does not carry: switch back \
         to that branch to work on it again"
    )
}

/// A `[dev ▸ methodology]` corpus whose `milestone/x/main` carries a finalized ADR — the
/// branch is left checked out, so the ADR is on disk and baselined.
fn adr_on_milestone_branch() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
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
    corpus
}

/// Every `reconciliation.rename` row in `envelope` keyed at the ADR's path.
fn rename_rows(envelope: &serde_json::Value) -> Vec<serde_json::Value> {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `findings`: {envelope}"))
        .iter()
        .filter(|f| f["key"]["code"] == "reconciliation.rename" && f["key"]["target"] == ADR_PATH)
        .cloned()
        .collect()
}

/// The single advisory branch-switch row in `envelope`, asserted whole.
fn branch_switch_row(envelope: &serde_json::Value, what: &str) -> serde_json::Value {
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

/// **(a)** + **(b)** Flow E's L2 half: after the switch to `main` the store sweep exits 0 with
/// the advisory branch-switch row, and a task's gate reports the same key under the same
/// message and route.
#[test]
fn after_a_branch_switch_both_scopes_report_the_same_advisory_row() {
    let corpus = adr_on_milestone_branch();
    corpus.git(&["switch", "-q", "main"]);
    assert!(
        !corpus.repo().join(ADR_PATH).exists(),
        "precondition: `main` does not carry {ADR_PATH}",
    );

    // (a) The store sweep: report-only, exit 0.
    let out = corpus.jigc(&["validate", "--format", "json"]);
    let store: serde_json::Value = stdout_json(&out, &[0], "`jigc validate` after the switch");
    assert_eq!(
        store["report_only"], true,
        "the advisory row leaves the sweep report-only: {store}"
    );
    let store_row = branch_switch_row(&store, "store scope");

    // (b) The task gate over the same state.
    let task = corpus.start_workflow("single-task", "warm the read cache");
    corpus.set_field(&format!("commit:{task}#type"), &task, "docs");
    corpus.set_field(&format!("commit:{task}#scope"), &task, "cache");
    corpus.set_slot(
        &format!("commit:{task}#summary"),
        &task,
        "warm the read cache",
    );
    let out = corpus.jigc(&["task", "validate", &task, "--format", "json"]);
    let gate: serde_json::Value = stdout_json(
        &out,
        &[i32::from(cli::task::EXIT_SUCCESS)],
        "`jigc task validate` after the switch",
    );
    let task_row = branch_switch_row(&gate, "task scope");
    for field in ["key", "severity", "message", "route"] {
        assert_eq!(
            store_row[field], task_row[field],
            "one producer, one route: the {field} reads the same at both scopes\n\
             store: {store_row}\ntask: {task_row}",
        );
    }
}

/// **(c)** The route's promise: switching back to the branch that carries the doc clears
/// the row — the baseline was never stale, only out of view.
#[test]
fn switching_back_clears_the_row() {
    let corpus = adr_on_milestone_branch();
    corpus.git(&["switch", "-q", "main"]);
    let out = corpus.jigc(&["validate", "--format", "json"]);
    let away: serde_json::Value = stdout_json(&out, &[0], "`jigc validate` on `main`");
    branch_switch_row(&away, "precondition on `main`");

    corpus.git(&["switch", "-q", MILESTONE_BRANCH]);
    let out = corpus.jigc(&["validate", "--format", "json"]);
    let back: serde_json::Value =
        stdout_json(&out, &[0], "`jigc validate` back on the milestone branch");
    assert!(
        rename_rows(&back).is_empty(),
        "switching back clears the row; got {back}",
    );
}

/// **(d)** The control: the same ADR, `git rm`'d and committed on its own branch, has history
/// at `HEAD` — a genuine deletion — so the store sweep keeps the blocking weak finding and
/// exits 1 under the `oob-rename` closing line.
#[test]
fn a_committed_deletion_with_history_still_blocks() {
    let corpus = adr_on_milestone_branch();
    corpus.git(&["rm", "-q", ADR_PATH]);
    corpus.git(&["commit", "-qm", "docs: drop the cache decision"]);

    let out = corpus.jigc(&["validate", "--format", "json"]);
    let swept: serde_json::Value = stdout_json(&out, &[1], "`jigc validate` over a deletion");
    assert_eq!(
        swept["report_only"], false,
        "the deletion flips the exit: {swept}"
    );
    let rows = rename_rows(&swept);
    assert_eq!(rows.len(), 1, "one rename row at {ADR_PATH}; got {swept}");
    assert_eq!(rows[0]["severity"], "blocking", "{}", rows[0]);
    assert_eq!(
        rows[0]["message"],
        format!("tracked managed doc adr:single-node-cache ({ADR_PATH}) is missing"),
        "the weak deletion, no `git mv` suspect: {}",
        rows[0],
    );
    assert_ne!(rows[0]["route"], branch_switch_route(), "{}", rows[0]);

    let out = corpus.jigc(&["validate"]);
    assert_eq!(out.status.code(), Some(1), "the agent view exits 1 too");
    let oob = cli::render::STORE_EXIT_FLIPS
        .iter()
        .find(|flip| flip.id == "oob-rename")
        .expect("the oob-rename member is on the axis");
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(
        text.contains(&(oob.trailer)()),
        "the sweep closes on the `oob-rename` line; got:\n{text}",
    );
}
