//! **The append-only logs' merge, held to "both sides kept, in date order, anything else
//! refused"** (the human's decision of 2026-10-02 — [DECISIONS.md](../../../DECISIONS.md) →
//! *Merge `main` into the branch before every pull request*; the step's home is
//! [dev-workflow.md](../../../implementation/dev-workflow.md) → Before a pull request to
//! `main`).
//!
//! Every pull request to `main` first merges `origin/main` into its branch, and two
//! branches that each add a `DECISIONS.md` entry or a fold-back span conflict in those
//! logs and nowhere else — PR #6 against PR #5 did. [`dev/merge-logs`](../../../dev/merge-logs)
//! resolves exactly that conflict and refuses every other, and the build harness's close
//! runs it (`.claude/workflows/milestone-build.js`, `syncMainPrompt`). Each arm here stops a
//! real merge on a real conflict in a throwaway repository and drives the script there:
//!
//! - **(a)** two `DECISIONS.md` entries of one date, inserted at the top by both sides:
//!   both kept whole, the branch's above `main`'s, one blank line between, every other
//!   byte unchanged — and the merge then concludes with `git commit --no-edit`.
//! - **(b)** `main`'s entry dated *newer* than the branch's: date order wins, so `main`'s
//!   sits on top — the order is the dates', not the sides'.
//! - **(c)** two fold-back spans appended to `project-history.md`'s one record line (the
//!   shape the real record has): the base text once, `main`'s span, then the branch's.
//! - **(d)** both sides rewrote the same existing line: refused, exit 1, the file and the
//!   index untouched — a conflict over content is the human's.
//! - **(e)** a conflicted path outside the logs beside a resolvable log conflict: refused,
//!   and the log is not written either — all or nothing.
//! - **(f)** outside a merge: a usage error (exit 2), not a verdict.
//! - **(g)** the harness's `SYNC_LOGS` is the script's `LOGS`, and the sync step runs the
//!   script and aborts the merge when it refuses — so the step and the tool cannot drift.
//! - **(h)** its twin, the branch-start rule ([dev-workflow.md](../../../implementation/dev-workflow.md)
//!   → Starting a branch from `main`): the harness's milestone-branch step halts on a local
//!   `main` that is not an ancestor of `origin/main`, and checks that a branch it creates
//!   starts exactly at `origin/main`.
//! - **(i)** the per-increment land step (one act since 2026-10-03 — [DECISIONS.md](../../../DECISIONS.md)
//!   → *one land step per increment*) merges onto the milestone branch's current tip, lets the
//!   same logs conflict and resolves them with the script, aborts any other conflict, deletes
//!   the increment branch and pushes the milestone branch — and no separate push step exists.
//! - **(j)** every `build-git` step the harness spawns or returns carries `model: GIT_MODEL`,
//!   which is Sonnet.
//! - **(k)** the stabilization harness (`.claude/workflows/stabilize.js`) merges too — a
//!   round into its loop branch, and at the close `origin/main` into the loop branch — and
//!   lets the same logs conflict: its `SYNC_LOGS` is this harness's and the script's, its
//!   sync step is the same step, and its land step resolves with the script and aborts
//!   every other conflict.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SCRIPT: &str = "dev/merge-logs";
const HARNESS: &str = ".claude/workflows/milestone-build.js";
const STABILIZE: &str = ".claude/workflows/stabilize.js";
const DECISIONS: &str = "DECISIONS.md";
const HISTORY: &str = "implementation/project-history.md";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/cli has a repo root two levels up")
        .to_path_buf()
}

/// A throwaway repository root that removes itself on drop.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "jigc-merge-logs-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create the scratch repository");
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A command with this machine's git configuration masked, so the fixture's merges
/// behave as they would on a runner.
fn hermetic(program: &Path, cwd: &Path) -> Command {
    let mut c = Command::new(program);
    c.current_dir(cwd)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("HOME", cwd)
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid");
    c
}

fn git_out(repo: &Path, args: &[&str]) -> Output {
    hermetic(Path::new("git"), repo)
        .args(args)
        .output()
        .expect("run git")
}

fn git(repo: &Path, args: &[&str]) -> String {
    let out = git_out(repo, args);
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

fn write(repo: &Path, path: &str, text: &str) {
    let p = repo.join(path);
    fs::create_dir_all(p.parent().expect("a parent")).expect("mkdir");
    fs::write(p, text).expect("write a fixture file");
}

fn read(repo: &Path, path: &str) -> String {
    fs::read_to_string(repo.join(path)).expect("read a fixture file")
}

/// One side's edits: `(path, full new text)`.
type Edits<'a> = &'a [(&'a str, &'a str)];

/// Seed `base` on `main`, commit `theirs` on `main` and `ours` on the branch cut before
/// it, then merge `main` into the branch and stop on the conflict — the state the sync
/// step hands `dev/merge-logs`.
fn stopped_merge(tag: &str, base: Edits, ours: Edits, theirs: Edits) -> Scratch {
    let s = Scratch::new(tag);
    let r = s.0.as_path();
    git(r, &["init", "-q", "-b", "main"]);
    for (p, t) in base {
        write(r, p, t);
    }
    git(r, &["add", "-A"]);
    git(r, &["commit", "-qm", "seed"]);
    git(r, &["switch", "-q", "-c", "milestone/demo/main"]);
    for (p, t) in ours {
        write(r, p, t);
    }
    git(r, &["commit", "-qam", "branch"]);
    git(r, &["switch", "-q", "main"]);
    for (p, t) in theirs {
        write(r, p, t);
    }
    git(r, &["commit", "-qam", "main moved"]);
    git(r, &["switch", "-q", "milestone/demo/main"]);
    let merge = git_out(r, &["merge", "--no-ff", "--no-edit", "main"]);
    assert!(
        !merge.status.success(),
        "the fixture must stop on a conflict, and the merge went through: {}",
        String::from_utf8_lossy(&merge.stdout)
    );
    s
}

fn merge_logs(repo: &Path) -> Output {
    hermetic(&repo_root().join(SCRIPT), repo)
        .output()
        .expect("run dev/merge-logs")
}

fn unmerged(repo: &Path) -> String {
    git(repo, &["diff", "--name-only", "--diff-filter=U"])
}

fn assert_resolved(repo: &Path, out: &Output) {
    assert_eq!(
        out.status.code(),
        Some(0),
        "dev/merge-logs must resolve a pure-append log conflict; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(unmerged(repo), "", "every resolved log is staged");
    git(repo, &["commit", "--no-edit", "-q"]);
    let parents = git(repo, &["rev-list", "--parents", "-n", "1", "HEAD"]);
    assert_eq!(
        parents.split_whitespace().count(),
        3,
        "the merge concludes as a merge commit: {parents}"
    );
}

const D_BASE: &str = "# Decisions\n\nIntro.\n\n## 2026-10-01 — base entry\n\nBase body.\n";

fn d_with(top: &str) -> String {
    D_BASE.replace("## 2026-10-01", &format!("{top}\n\n## 2026-10-01"))
}

#[test]
fn a_one_date_entries_are_both_kept_the_branchs_on_top() {
    let ours = d_with("## 2026-10-02 — the branch's entry\n\nBranch body.");
    let theirs = d_with("## 2026-10-02 — main's entry\n\nMain body.");
    let s = stopped_merge(
        "a",
        &[(DECISIONS, D_BASE)],
        &[(DECISIONS, &ours)],
        &[(DECISIONS, &theirs)],
    );
    let out = merge_logs(&s.0);
    assert_resolved(&s.0, &out);
    assert_eq!(
        read(&s.0, DECISIONS),
        "# Decisions\n\nIntro.\n\n\
         ## 2026-10-02 — the branch's entry\n\nBranch body.\n\n\
         ## 2026-10-02 — main's entry\n\nMain body.\n\n\
         ## 2026-10-01 — base entry\n\nBase body.\n",
        "both entries whole, newest first, the branch's above main's on one date"
    );
}

#[test]
fn b_date_order_wins_over_the_side() {
    let ours = d_with("## 2026-10-02 — the branch's entry\n\nBranch body.");
    let theirs = d_with("## 2026-10-03 — main's newer entry\n\nMain body.");
    let s = stopped_merge(
        "b",
        &[(DECISIONS, D_BASE)],
        &[(DECISIONS, &ours)],
        &[(DECISIONS, &theirs)],
    );
    let out = merge_logs(&s.0);
    assert_resolved(&s.0, &out);
    assert_eq!(
        read(&s.0, DECISIONS),
        "# Decisions\n\nIntro.\n\n\
         ## 2026-10-03 — main's newer entry\n\nMain body.\n\n\
         ## 2026-10-02 — the branch's entry\n\nBranch body.\n\n\
         ## 2026-10-01 — base entry\n\nBase body.\n",
    );
}

#[test]
fn c_two_fold_backs_on_the_records_one_line_are_both_kept_main_first() {
    let base = "# History\n\n## The record\n\n**M53 — a span.** **M54 — a span.**\n";
    let ours = base.replace(".**\n", ".** **M56 — the branch's span.**\n");
    let theirs = base.replace(".**\n", ".** **M55 — main's span.**\n");
    let s = stopped_merge(
        "c",
        &[(HISTORY, base)],
        &[(HISTORY, &ours)],
        &[(HISTORY, &theirs)],
    );
    let out = merge_logs(&s.0);
    assert_resolved(&s.0, &out);
    assert_eq!(
        read(&s.0, HISTORY),
        "# History\n\n## The record\n\n\
         **M53 — a span.** **M54 — a span.** **M55 — main's span.** **M56 — the branch's span.**\n",
        "the record's text once, then main's span, then the branch's — newest last"
    );
}

#[test]
fn d_a_rewrite_of_existing_text_is_refused_and_nothing_is_written() {
    let ours = D_BASE.replace("Base body.", "The branch rewrote the body.");
    let theirs = D_BASE.replace("Base body.", "Main rewrote the body.");
    let s = stopped_merge(
        "d",
        &[(DECISIONS, D_BASE)],
        &[(DECISIONS, &ours)],
        &[(DECISIONS, &theirs)],
    );
    let before = read(&s.0, DECISIONS);
    let out = merge_logs(&s.0);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a conflict over content is refused"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("not two appends") && stderr.contains("DECISIONS.md"),
        "the refusal names the log and why: {stderr}"
    );
    assert_eq!(read(&s.0, DECISIONS), before, "the file is untouched");
    assert_eq!(unmerged(&s.0), "DECISIONS.md\n", "and still unmerged");
}

#[test]
fn e_a_conflict_outside_the_logs_refuses_the_whole_merge() {
    let ours = d_with("## 2026-10-02 — the branch's entry\n\nBranch body.");
    let theirs = d_with("## 2026-10-02 — main's entry\n\nMain body.");
    let s = stopped_merge(
        "e",
        &[(DECISIONS, D_BASE), ("code.txt", "one\ntwo\n")],
        &[(DECISIONS, &ours), ("code.txt", "one\nbranch\n")],
        &[(DECISIONS, &theirs), ("code.txt", "one\nmain\n")],
    );
    let before = read(&s.0, DECISIONS);
    let out = merge_logs(&s.0);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("code.txt"),
        "the refusal names the path outside the logs"
    );
    assert_eq!(
        read(&s.0, DECISIONS),
        before,
        "all or nothing: the resolvable log is not written either"
    );
    assert_eq!(unmerged(&s.0), "DECISIONS.md\ncode.txt\n");
}

#[test]
fn f_outside_a_merge_is_a_usage_error() {
    let s = Scratch::new("f");
    git(&s.0, &["init", "-q", "-b", "main"]);
    let out = merge_logs(&s.0);
    assert_eq!(
        out.status.code(),
        Some(2),
        "no MERGE_HEAD is not a verdict: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// The quoted entries of a `[ … ]` list that follows `anchor` in `text`.
fn quoted_list(text: &str, anchor: &str, close: char) -> Vec<String> {
    let at = text
        .find(anchor)
        .unwrap_or_else(|| panic!("`{anchor}` is present"));
    let rest = &text[at + anchor.len()..];
    let body = &rest[..rest.find(close).expect("the list closes")];
    body.split(',')
        .filter_map(|item| {
            let item = item.trim();
            let q = item.chars().next()?;
            let inner = item.strip_prefix(q)?;
            Some(inner[..inner.find(q)?].to_string())
        })
        .collect()
}

#[test]
fn g_the_harness_step_and_the_tool_name_the_same_logs() {
    let root = repo_root();
    let harness = fs::read_to_string(root.join(HARNESS)).expect("read the harness");
    let tool = fs::read_to_string(root.join(SCRIPT)).expect("read dev/merge-logs");
    let step_logs = quoted_list(&harness, "const SYNC_LOGS = [", ']');
    let tool_logs: Vec<String> = quoted_list(&tool, "LOGS = {", '}')
        .into_iter()
        .filter(|p| !p.starts_with("newest-"))
        .collect();
    assert_eq!(
        step_logs,
        vec![DECISIONS.to_string(), HISTORY.to_string()],
        "the harness's sync step names the two logs"
    );
    assert_eq!(
        tool_logs, step_logs,
        "dev/merge-logs resolves exactly the logs the sync step lets conflict"
    );
    let step = &harness[harness
        .find("function syncMainPrompt()")
        .expect("the sync step")..];
    let step = &step[..step.find("\n}\n").expect("its end")];
    for needle in [
        "`git fetch origin main`",
        "`git merge --no-ff --no-edit origin/main`",
        "`dev/merge-logs`",
        "`git merge --abort`, then HALT",
        "`git commit --no-edit`",
        "Do NOT push",
    ] {
        assert!(step.contains(needle), "the sync step lists {needle}");
    }
}

#[test]
fn h_the_branch_step_starts_from_the_current_remote_main() {
    let harness = fs::read_to_string(repo_root().join(HARNESS)).expect("read the harness");
    let step = &harness[harness
        .find("function milestoneBranchPrompt()")
        .expect("the milestone-branch step")..];
    let step = &step[..step.find("\n}\n").expect("its end")];
    for needle in [
        "`git fetch origin main`",
        "`git merge-base --is-ancestor main origin/main` must succeed",
        "never reset",
        "`git switch --no-track -c ' + b + ' origin/main`",
        "`git merge-base origin/main HEAD` must both equal `git rev-parse origin/main`",
    ] {
        assert!(
            step.contains(needle),
            "the milestone-branch step lists {needle}"
        );
    }
}

#[test]
fn i_the_land_step_merges_onto_the_current_tip_and_pushes_in_one_act() {
    let harness = fs::read_to_string(repo_root().join(HARNESS)).expect("read the harness");
    let step = &harness[harness
        .find("function landPrompt(inc)")
        .expect("the land step")..];
    let step = &step[..step.find("\n}\n").expect("its end")];
    for needle in [
        "`git merge --no-ff --no-edit ' + b + '`",
        "SYNC_LOGS",
        "`dev/merge-logs`",
        "`git merge --abort`, then HALT",
        "`git commit --no-edit`",
        "`git branch -d ' + b + '`",
        "`git push origin ' + m + '`",
        "tip_moved",
    ] {
        assert!(step.contains(needle), "the land step lists {needle}");
    }
    // A milestone tip that moved after the increment branched is merged onto, never refused:
    // the old land step's ancestry precondition must not come back.
    assert!(
        !step.contains("`git merge-base --is-ancestor ' + m + ' ' + b + '` must succeed"),
        "the land step does not require the increment to descend from the milestone tip"
    );
    assert!(
        !harness.contains("function pushPrompt("),
        "the push is part of the land step, not a step of its own"
    );
}

#[test]
fn j_every_git_step_runs_on_sonnet() {
    let harness = fs::read_to_string(repo_root().join(HARNESS)).expect("read the harness");
    assert!(
        harness.contains("const GIT_MODEL = 'sonnet'"),
        "the harness pins its git steps to Sonnet"
    );
    let git_calls: Vec<&str> = harness
        .lines()
        .filter(|l| l.contains("agentType: 'build-git'"))
        .collect();
    assert!(
        git_calls.len() >= 4,
        "milestone branch, open, land and the close's sync are build-git steps: {git_calls:#?}"
    );
    for call in git_calls {
        assert!(
            call.contains("model: GIT_MODEL"),
            "a build-git step without `model: GIT_MODEL`: {call}"
        );
    }
}

#[test]
fn k_the_stabilization_harness_lets_the_same_logs_conflict_and_no_other() {
    let root = repo_root();
    let build = fs::read_to_string(root.join(HARNESS)).expect("read the build harness");
    let stabilize = fs::read_to_string(root.join(STABILIZE)).expect("read the stabilize harness");
    let logs = quoted_list(&stabilize, "const SYNC_LOGS = [", ']');
    assert_eq!(
        logs,
        quoted_list(&build, "const SYNC_LOGS = [", ']'),
        "the two harnesses let the same logs conflict"
    );
    assert_eq!(logs, vec![DECISIONS.to_string(), HISTORY.to_string()]);

    let step_of = |harness: &str, name: &str| -> String {
        let step = &harness[harness
            .find(&format!("function {name}("))
            .unwrap_or_else(|| panic!("the step `{name}`"))..];
        step[..step.find("\n}\n").expect("its end")].to_owned()
    };
    // The close's sync step is one step, whichever harness returns it: the same numbered
    // commands, to the byte — each harness's own branch and its own list standing where
    // the text says `m` and `logs`.
    let commands = |step: &str| -> Vec<String> {
        step.lines()
            .map(str::trim)
            .filter(|line| {
                let mut chars = line.chars();
                chars.next() == Some('\'')
                    && chars.next().is_some_and(|c| c.is_ascii_digit())
                    && chars.next() == Some('.')
            })
            .map(str::to_owned)
            .collect()
    };
    let ours = commands(&step_of(&stabilize, "syncMainPrompt"));
    assert_eq!(ours.len(), 8, "the sync step's eight commands: {ours:#?}");
    assert_eq!(
        ours,
        commands(&step_of(&build, "syncMainPrompt")),
        "the stabilization harness's sync step is the build harness's"
    );

    let land = step_of(&stabilize, "landPrompt");
    for needle in [
        "SYNC_LOGS",
        "`git merge --no-ff --no-edit ' + branch + '`",
        "`dev/merge-logs`",
        "`git merge --abort`, then HALT",
        "`git commit --no-edit`",
        "`git push origin ' + loop + '`",
        "tip_moved",
    ] {
        assert!(
            land.contains(needle),
            "the round's land step lists {needle}"
        );
    }
    for forbidden in ["--force", "rebase", "git pull", "git reset"] {
        let listed = land
            .lines()
            .filter(|line| line.trim_start().starts_with('\''))
            .any(|line| {
                line.contains(&format!("`git {forbidden}"))
                    || line.contains(&format!(" {forbidden} "))
            });
        assert!(!listed, "the round's land step runs `{forbidden}`");
    }
}
