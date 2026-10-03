//! M55 Increment 11 / T5 — **flow 59 (E): branch and pull — L1 and L2 in one walk**
//! (`design/worked-examples.md` → 59; `design/findings-channel.md` §6 → L1, L2, §11 → E).
//!
//! One `[dev ▸ methodology]` corpus ([`State::CommittedSingletons`](crate::support::trial_corpus::State),
//! `VISION.md` baselined by a landed finalize), wired to a local bare `origin` and a
//! teammate's clone, walks both halves of flow E on the shipped `single-task`:
//!
//! 1. **L1, the pull.** The teammate's committed `VISION.md` edit is pulled. Store-scope
//!    `jigc validate` exits 0 with the `file-state.hash-matches` row advisory, routed *the
//!    baseline lags `HEAD`*; a `single-task` writing `vision#open-questions` reads the advisory
//!    `reconciliation.absorb` from its gate and finalizes, landing both sides; the next
//!    `jigc validate` carries no finding.
//! 2. **L2, the branch switch,** over the baseline that absorb wrote. An ADR is finalized on
//!    `milestone/x/main`, then `git switch main`: `jigc validate` exits 0 with one advisory
//!    `reconciliation.rename` row routed at the branch switch, a task's gate reports the same
//!    key under the same message and route, and neither surface names `jigc unmanage`.
//!
//! **The controls are the discriminating reds.** The walk's two predicates — [`absorbs`] at
//! the task gate and [`reports_branch_switch`] at the store sweep — are asserted *false* over
//! the state each must not excuse: an out-of-band edit made during the task after the same
//! pull still conflict-blocks (exit 3), and the ADR deleted with history on its own branch
//! still blocks the store sweep (exit 1).
//!
//! **What this adds over `l1_pull_absorption` and `l2_branch_switch`** (M55 Increments 4 and 5,
//! `g_finalize`), which prove each half from its own fresh corpus: both halves run in one
//! corpus's history, so the L2 switch is graded over a baseline the L1 absorb wrote. The pull,
//! the branch and the readers are theirs, moved to `support::branch_and_pull` and shared,
//! never copied.

use std::fs;

use crate::support::branch_and_pull::{
    ADR_PATH, LAG_ROUTE, TASK_PROSE, TEAMMATE_LINE, Team, VISION, adr_on_milestone_branch,
    blocking_probes, branch_switch_route, branch_switch_row, commit_count, rename_rows, rows_at,
    start_task, store_sweep,
};
use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{TrialCorpus, read};

/// The line an out-of-band edit adds to `VISION.md` while a task is open.
const DURING_TASK_LINE: &str = "A line written while the task was open.";

/// **The L1 predicate, at the task gate:** the drift at `VISION.md` is absorbed advisory, and
/// nothing conflict-blocks there.
fn absorbs(envelope: &serde_json::Value) -> bool {
    rows_at(envelope, "reconciliation.absorb", VISION)
        .iter()
        .any(|row| row["severity"] == "advisory")
        && rows_at(envelope, "reconciliation.conflict-block", VISION).is_empty()
}

/// **The L2 predicate, at either scope:** exactly one `reconciliation.rename` row at the ADR,
/// advisory and routed at the branch switch.
fn reports_branch_switch(envelope: &serde_json::Value) -> bool {
    let rows = rename_rows(envelope);
    rows.len() == 1
        && rows[0]["severity"] == "advisory"
        && rows[0]["route"] == branch_switch_route()
}

fn rendered(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

/// `jigc task validate <task> --format json`, at exit 0.
fn task_gate(corpus: &TrialCorpus, task: &str, context: &str) -> serde_json::Value {
    let out = corpus.jigc(&["task", "validate", task, "--format", "json"]);
    stdout_json(&out, &[i32::from(cli::task::EXIT_SUCCESS)], context)
}

/// **L1, walked:** the pulled edit reads as a lagging baseline at store scope, the task that
/// edits the doc absorbs it and lands both sides, and the next sweep is clean — back to the
/// findings it carried before the pull, which it returns (the fixture's own residue, none of
/// them a `file-state` row).
fn absorb_the_pull(team: &Team) -> serde_json::Value {
    let corpus = &team.corpus;
    let at_rest = store_sweep(corpus, "the store sweep before the pull")["findings"].clone();
    assert!(
        at_rest
            .as_array()
            .expect("the sweep carries `findings`")
            .iter()
            .all(|f| f["probe"] != "file-state" && f["probe"] != "reconciliation"),
        "precondition: the baselined corpus carries no drift; got {at_rest}",
    );
    team.pull_vision_edit();

    let swept = store_sweep(corpus, "the store sweep after the pull");
    let rows = rows_at(&swept, "file-state.hash-matches", VISION);
    assert_eq!(rows.len(), 1, "one drift row for `{VISION}`; got {swept}");
    assert_eq!(
        rows[0]["severity"], "advisory",
        "the lag is advisory: {}",
        rows[0]
    );
    assert_eq!(
        rows[0]["route"], LAG_ROUTE,
        "routed at the lag: {}",
        rows[0]
    );
    assert!(
        !blocking_probes(&swept).contains(&"file-state"),
        "an advisory lag blocks nothing; got {swept}",
    );

    let task = start_task(corpus, "sharpen the open questions");
    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);
    let gate = task_gate(corpus, &task, "task validate after the pull");
    assert!(
        absorbs(&gate),
        "the task's gate absorbs the pulled edit at `{VISION}`; got {gate}",
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

    let after = store_sweep(corpus, "the store sweep after the landed finalize");
    assert_eq!(
        after["findings"], at_rest,
        "the landed finalize re-baselined `{VISION}` and the sweep is clean; got {after}",
    );
    at_rest
}

/// **Flow E, walked.** L1 then L2 in one corpus: the switch is graded over the baseline the
/// absorb wrote, so the only row after it is the branch switch's, at both scopes, and neither
/// surface names `jigc unmanage`.
#[test]
fn the_pull_is_absorbed_then_the_branch_switch_reads_advisory_at_both_scopes() {
    let team = Team::new();
    let corpus = &team.corpus;
    let at_rest = absorb_the_pull(&team);

    adr_on_milestone_branch(corpus);
    corpus.git(&["switch", "-q", "main"]);
    assert!(
        !corpus.repo().join(ADR_PATH).exists(),
        "precondition: `main` does not carry {ADR_PATH}",
    );

    let store = store_sweep(corpus, "`jigc validate` after the switch");
    assert_eq!(
        store["report_only"], true,
        "the advisory row leaves the sweep report-only: {store}",
    );
    assert!(
        reports_branch_switch(&store),
        "the store sweep reads the branch switch advisory; got {store}",
    );
    let store_row = branch_switch_row(&store, "store scope");
    let others: Vec<serde_json::Value> = store["findings"]
        .as_array()
        .expect("the sweep carries `findings`")
        .iter()
        .filter(|f| **f != store_row)
        .cloned()
        .collect();
    assert_eq!(
        serde_json::Value::from(others),
        at_rest,
        "the absorb's baseline holds across the switch: the branch switch is the only new row; \
         got {store}",
    );
    let out = corpus.jigc(&["validate"]);
    assert_eq!(out.status.code(), Some(0), "the agent view exits 0 too");
    assert!(
        !rendered(&out).contains("jigc unmanage"),
        "the agent view never routes a branch switch at an index drop; got:\n{}",
        rendered(&out),
    );

    let task = start_task(corpus, "warm the read cache");
    let gate = task_gate(corpus, &task, "`jigc task validate` after the switch");
    assert!(
        reports_branch_switch(&gate),
        "the task gate reads the branch switch advisory; got {gate}",
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

/// **Control (L1):** after the same pull, an out-of-band edit made while the task is open is
/// not at the task's base pin, so the gate does not absorb it — finalize conflict-blocks at
/// exit 3 and commits nothing.
#[test]
fn an_edit_made_during_the_task_still_conflict_blocks_after_the_pull() {
    let team = Team::new();
    let corpus = &team.corpus;
    team.pull_vision_edit();

    let task = start_task(corpus, "sharpen the open questions");
    corpus.set_slot("vision:vision#open-questions", &task, TASK_PROSE);
    let pulled = read(&team.repo(), VISION);
    let edited = pulled.replacen(
        &format!("{TEAMMATE_LINE}\n"),
        &format!("{TEAMMATE_LINE}\n{DURING_TASK_LINE}\n"),
        1,
    );
    assert_ne!(edited, pulled, "the during-task edit must change {VISION}");
    fs::write(team.repo().join(VISION), edited).expect("an uncommitted edit after the mint");

    let before = commit_count(corpus);
    let out = corpus.jigc(&["task", "finalize", &task, "--format", "json"]);
    let refused: serde_json::Value = stdout_json(
        &out,
        &[i32::from(cli::task::EXIT_VALIDATION_BLOCKED)],
        "finalize over an edit made during the task",
    );
    assert!(
        !absorbs(&refused),
        "an edit made during the task is not absorbed; got {refused}",
    );
    let blocks = rows_at(&refused, "reconciliation.conflict-block", VISION);
    assert_eq!(
        blocks.len(),
        1,
        "one conflict-block at `{VISION}`; got {refused}"
    );
    assert_eq!(blocks[0]["severity"], "blocking", "{}", blocks[0]);
    assert_eq!(
        commit_count(corpus),
        before,
        "a conflict-block commits nothing"
    );
}

/// **Control (L2):** after the same walk's L1 half, the ADR deleted with `git rm` and committed
/// on its own branch has history at `HEAD` — a genuine deletion, not a branch switch — so the
/// store sweep keeps the blocking row and exits 1.
#[test]
fn a_deleted_doc_with_history_still_blocks_after_the_walk() {
    let team = Team::new();
    let corpus = &team.corpus;
    absorb_the_pull(&team);
    adr_on_milestone_branch(corpus);
    corpus.git(&["rm", "-q", ADR_PATH]);
    corpus.git(&["commit", "-qm", "docs: drop the cache decision"]);

    let out = corpus.jigc(&["validate", "--format", "json"]);
    let swept: serde_json::Value = stdout_json(&out, &[1], "`jigc validate` over a deletion");
    assert!(
        !reports_branch_switch(&swept),
        "a deletion with history is not a branch switch; got {swept}",
    );
    assert_eq!(
        swept["report_only"], false,
        "the deletion flips the exit: {swept}"
    );
    let rows = rename_rows(&swept);
    assert_eq!(rows.len(), 1, "one rename row at {ADR_PATH}; got {swept}");
    assert_eq!(rows[0]["severity"], "blocking", "{}", rows[0]);
    let out = corpus.jigc(&["validate"]);
    assert_eq!(out.status.code(), Some(1), "the agent view exits 1 too");
}
