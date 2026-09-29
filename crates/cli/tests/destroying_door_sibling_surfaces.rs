//! M52 Increment 4 / T7 — **the three siblings the review's rows named, each driven at the
//! door that prints it** (`completions/artifacts/M52/settle-record.md` → D3.7;
//! `completions/artifacts/M52/baseline-destroying.md` → §2.7 D-1, §2.8 D-2, §2.10 D-4).
//!
//! Three surfaces, three defects, one shape: a door said something the binary does not do.
//!
//! - **D-1 — a route that names a verb the state refuses.** Driven at `f8d5957a`: over a
//!   *milestone sub-task* both staged-prose refusals routed at ``jigc task finalize <sub>``,
//!   and that command answers `finalize.milestone-sub-task` at exit **3** — *"the parent
//!   milestone's finalize is the only commit boundary"*. The baseline measured the class
//!   exactly: of the six route producers that name `jigc task finalize`, exactly **two** are
//!   servable over a sub-task (`cli::task`'s discard refusal and `cli::setup`'s uninstall
//!   refusal), and the third family member — `milestone.staged-prose` — already names the
//!   right door, which is what makes this an un-swept axis rather than a design choice.
//! - **D-2 — an ack that claims a removal that did not happen.** Driven at the same sha with
//!   `.jigc/worktrees` made unwritable: `jigc milestone discard <id> --force` warned twice
//!   that it could not remove the fan-out worktree and then printed *"workbench removed"* at
//!   exit 0, with both worktrees still on disk.
//! - **D-4 — a fail-closed route that blames the wrong thing.** With `.jigc/worktrees`
//!   unreadable, `jigc uninstall` refused (correctly) and then told the operator to *"make
//!   sure `git` is on PATH"* — for an `EACCES` on a `read_dir` of a directory, with the
//!   `--force` consent unnamed while the same door's two sibling fail-closed refusals both
//!   name it.
//!
//! **D-3 is not here, and the reason is a measured fact rather than an omission.** Its
//! producer (`cli::task::staged_task_prose`'s `docs/` fault) is fixed in this same task and
//! fenced in `crates/cli/tests/repo_relative_paths.rs` — by disposition and by the standing
//! source fence, not by a drive — because at `f8d5957a` **no door can reach it any more**:
//! T5's foreign-byte guard is asked *first* at all three doors and probes a **superset** of
//! the same bytes. `staged_doc_ids` errors only where `std::fs::metadata` fails on an entry
//! whose name parses as a staged doc id, and every such entry (a dangling symlink, a symlink
//! to anything, a directory, a fifo) is `!file_type().is_file()`, which
//! `engine::state::foreign_area_paths` classifies **foreign** — so the foreign refusal
//! answers first, every time. Driven: `chmod 000 .jigc/tasks/<id>/docs` and a dangling
//! symlink named `commit:<id>.md` both answer `task-discard.foreign-bytes`, never
//! `task-discard.staged-prose`. The fix still lands (law 1 binds the surface whether or not a
//! door currently reaches it); what changes is how it is proven, and the bound is stated here
//! rather than dressed up as a drive.
//!
//! **The omitting context is an arm, not an afterthought** (the M4 front-door rule): every
//! cell below has a sibling over a state *without* the discriminator — an ordinary task that
//! belongs to no milestone, a discard whose removals all succeed — asserting the surface is
//! unchanged there. A green pass over the single composing context would hide the scope bug
//! in every omitting one.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-sibling-surfaces-{tag}-{}-{:?}",
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
        // Best-effort: a cell that made a directory unreadable restores it first, but a
        // panic mid-cell may not have.
        for probe in [".jigc/worktrees", ".jigc/tasks"] {
            let _ = std::process::Command::new("chmod")
                .args(["-R", "u+rwx", probe])
                .current_dir(&self.0)
                .output();
        }
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Run `jigc <args>` with `cwd = cwd` and `$HOME = home`, against the embedded packs.
fn jigc(cwd: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

fn ok(cwd: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(cwd, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn both_streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

const MILESTONE_TITLE: &str = "Cache rework";
const MILESTONE_ID: &str = "cache-rework";
const SUB_INTENT: &str = "Warm the read cache";
const SUB_TASK: &str = "warm-the-read-cache";
const TOP_INTENT: &str = "Fix the retry cap";
const TOP_TASK: &str = "fix-the-retry-cap";

/// A real install carrying a milestone with one sub-task and one ordinary `jigc start` task
/// — the **two populations** every cell below needs: the composing context (a sub-task) and
/// the omitting one (a task that belongs to no milestone).
fn workbench(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    ok(
        repo.path(),
        home.path(),
        &["milestone", "create", MILESTONE_TITLE],
        "milestone create",
    );
    ok(
        repo.path(),
        home.path(),
        &["milestone", "add-task", MILESTONE_ID, SUB_INTENT],
        "milestone add-task",
    );
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", TOP_INTENT],
        "start --workflow quick-fix",
    );
    (repo, home)
}

/// Give the sub-task a **staged doc**, which is what both staged-prose refusals key on. A
/// sub-task stages nothing at mint (`staged_prose_consent_axis`' arm (c)), so the cell has to
/// put one there — through the binary, from inside the sub-task's own worktree, which is the
/// only place `jigc` will serve a sub-task write.
fn stage_a_doc_in_the_sub_task(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let worktree = repo.join(".jigc").join("worktrees").join(SUB_TASK);
    ok(
        &worktree,
        home,
        &[
            "doc", "create", "adr", "--title", SUB_INTENT, "--task", SUB_TASK,
        ],
        "doc create adr --task <sub>",
    );
}

/// Every **concrete** `jigc …` command a rendered surface emits — the backticked spans with
/// no `<placeholder>` in them, which is exactly the set an operator can paste and run.
///
/// The emitted bytes are the contract, so the cells below run *these*, verbatim, rather than
/// a command reconstructed in test code: a route that reads right and runs wrong is the whole
/// defect D-1 is.
fn concrete_jigc_commands(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        let span = &after[..close];
        if span.starts_with("jigc ") && !span.contains('<') && !out.contains(&span.to_string()) {
            out.push(span.to_string());
        }
        rest = &after[close + 1..];
    }
    out
}

/// `finalize.milestone-sub-task` is the refusal that makes a route a dead end — the exact
/// identity the baseline drove both routes into.
const SUB_TASK_REFUSAL: &str = "finalize.milestone-sub-task";

// ---------------------------------------------------------------------------------
// D-1 — the two routes servable over a sub-task
// ---------------------------------------------------------------------------------

/// **Cell D-1a — `jigc task discard <sub>`.** Its refusal's every concrete command runs
/// verbatim and none of them is answered by `finalize.milestone-sub-task`; the consent it
/// names lands at exit 0 and settles the sub-task in the committed milestone record, which
/// the route now states rather than leaving for the operator to discover in the ack.
#[test]
fn d1a_the_task_discard_refusal_over_a_sub_task_routes_where_the_door_accepts() {
    let (repo, home) = workbench("d1a");
    stage_a_doc_in_the_sub_task(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["task", "discard", SUB_TASK]);
    let text = both_streams(&out);
    assert!(
        text.contains("task-discard.staged-prose"),
        "the cell must reach the staged-prose refusal; got:\n{text}",
    );
    assert!(
        text.contains(&format!("sub-task of milestone `{MILESTONE_ID}`")),
        "the refusal must say the discriminator it now keys on:\n{text}",
    );
    assert!(
        text.contains("settles"),
        "the route must state the consent's record side-effect:\n{text}",
    );

    let commands = concrete_jigc_commands(&text);
    assert!(
        commands.contains(&format!("jigc task discard {SUB_TASK} --force")),
        "the route must name the consent with the real id; emitted: {commands:?}",
    );
    assert!(
        !commands.contains(&format!("jigc task finalize {SUB_TASK}")),
        "the route must not name the verb this state refuses; emitted: {commands:?}",
    );

    // Each emitted command run VERBATIM, each from its own fixture so one cell's act does
    // not decide the next one's state.
    for (i, command) in commands.iter().enumerate() {
        let (repo, home) = workbench(&format!("d1a-run{i}"));
        stage_a_doc_in_the_sub_task(repo.path(), home.path());
        let argv: Vec<&str> = command.split_whitespace().skip(1).collect();
        let ran = jigc(repo.path(), home.path(), &argv);
        let answer = both_streams(&ran);
        assert!(
            !answer.contains(SUB_TASK_REFUSAL),
            "`{command}` — emitted by the refusal, run verbatim — is refused by \
             `{SUB_TASK_REFUSAL}`:\n{answer}",
        );
    }

    // The consent the route names, run verbatim, lands and names the record settle.
    let (repo, home) = workbench("d1a-consent");
    stage_a_doc_in_the_sub_task(repo.path(), home.path());
    let landed = jigc(
        repo.path(),
        home.path(),
        &["task", "discard", SUB_TASK, "--force"],
    );
    assert!(
        landed.status.success(),
        "the consent must be accepted:\n{}",
        both_streams(&landed),
    );
    assert!(
        both_streams(&landed).contains("record commit:"),
        "the consent's ack names the record settle the route now promises:\n{}",
        both_streams(&landed),
    );
}

/// **Cell D-1a', the omitting context.** An ordinary task belongs to no milestone, so nothing
/// above applies to it: its refusal keeps `jigc task finalize <id>` — the exit that *does*
/// work there — and says nothing about a milestone.
#[test]
fn d1a_an_ordinary_task_keeps_the_finalize_exit_that_works() {
    let (repo, home) = workbench("d1a-omit");
    let out = jigc(repo.path(), home.path(), &["task", "discard", TOP_TASK]);
    let text = both_streams(&out);
    assert!(
        text.contains("task-discard.staged-prose"),
        "the cell must reach the staged-prose refusal; got:\n{text}",
    );
    assert!(
        !text.contains("sub-task"),
        "a task that belongs to no milestone must not be described as one:\n{text}",
    );
    assert!(
        concrete_jigc_commands(&text).contains(&format!("jigc task finalize {TOP_TASK}")),
        "the ordinary route keeps the landing exit that works:\n{text}",
    );

    // …and it works: the landing exit is refused for a *content* reason, never for being a
    // sub-task.
    let ran = jigc(repo.path(), home.path(), &["task", "finalize", TOP_TASK]);
    assert!(
        !both_streams(&ran).contains(SUB_TASK_REFUSAL),
        "an ordinary task's finalize is not a sub-task refusal:\n{}",
        both_streams(&ran),
    );
}

/// **Cell D-1b — `jigc uninstall`'s staged-prose refusal.** Its listing enumerates several
/// tasks, so the discriminator belongs on the **row**, where the id already is: the sub-task
/// row names its own two concrete exits, and every concrete command the whole surface emits
/// runs verbatim without reaching `finalize.milestone-sub-task`.
#[test]
fn d1b_the_uninstall_refusal_names_a_sub_tasks_own_exits() {
    let (repo, home) = workbench("d1b");
    stage_a_doc_in_the_sub_task(repo.path(), home.path());

    let out = jigc(repo.path(), home.path(), &["uninstall"]);
    let text = both_streams(&out);
    assert!(
        text.contains("uninstall.staged-prose"),
        "the cell must reach the staged-prose refusal; got:\n{text}",
    );
    assert!(
        text.contains(&format!("sub-task of milestone `{MILESTONE_ID}`")),
        "the sub-task row must carry the discriminator:\n{text}",
    );

    let commands = concrete_jigc_commands(&text);
    assert!(
        commands.contains(&format!("jigc task discard {SUB_TASK} --force")),
        "the sub-task row names its own concrete consent; emitted: {commands:?}",
    );
    assert!(
        !commands.contains(&format!("jigc task finalize {SUB_TASK}")),
        "the surface must not name the verb this state refuses; emitted: {commands:?}",
    );

    for (i, command) in commands.iter().enumerate() {
        let (repo, home) = workbench(&format!("d1b-run{i}"));
        stage_a_doc_in_the_sub_task(repo.path(), home.path());
        let argv: Vec<&str> = command.split_whitespace().skip(1).collect();
        let ran = jigc(repo.path(), home.path(), &argv);
        let answer = both_streams(&ran);
        assert!(
            !answer.contains(SUB_TASK_REFUSAL),
            "`{command}` — emitted by the refusal, run verbatim — is refused by \
             `{SUB_TASK_REFUSAL}`:\n{answer}",
        );
    }
}

// ---------------------------------------------------------------------------------
// D-2 — the ack keys on the outcome
// ---------------------------------------------------------------------------------

/// **Cell D-2 — a `--force` discard whose worktree removal failed.** The two warnings name
/// the leaked worktree; the ack must not then claim the workbench came out. Class size 1,
/// driven: the sibling `milestone finalize` ack was already honest on the identical failure.
#[test]
fn d2_a_forced_discard_that_left_a_worktree_does_not_claim_the_workbench_removed() {
    let (repo, home) = workbench("d2");
    ok(
        repo.path(),
        home.path(),
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let worktrees = repo.path().join(".jigc").join("worktrees");
    // Make the removal fail without making the *probe* fail: the root stays readable, so
    // every guard answers as it would have; only `remove` cannot unlink.
    let out = Command::new("chmod")
        .args(["a-w", worktrees.to_str().expect("utf-8 path")])
        .output()
        .expect("chmod");
    assert!(out.status.success(), "chmod a-w must succeed");

    let ran = jigc(
        repo.path(),
        home.path(),
        &["milestone", "discard", MILESTONE_ID, "--force"],
    );
    let text = both_streams(&ran);

    let _ = Command::new("chmod")
        .args(["u+w", worktrees.to_str().expect("utf-8 path")])
        .output();

    assert!(
        text.contains("could not remove the fan-out worktree"),
        "the cell must reach the failing removal; got:\n{text}",
    );
    assert!(
        worktrees.join(SUB_TASK).exists(),
        "the premise: the worktree is still on disk",
    );
    assert!(
        !text.contains("workbench removed"),
        "the ack claims a removal that did not happen:\n{text}",
    );
    assert!(
        text.contains("workbench"),
        "the ack still has to say what happened to the workbench:\n{text}",
    );
}

/// **Cell D-2', the omitting context.** When every removal succeeds the ack is unchanged —
/// the fix keys on the outcome, so it must be invisible on the ordinary path.
#[test]
fn d2_a_forced_discard_that_removed_everything_still_says_workbench_removed() {
    let (repo, home) = workbench("d2-omit");
    ok(
        repo.path(),
        home.path(),
        &["milestone", "provision", MILESTONE_ID],
        "milestone provision",
    );
    let ack = ok(
        repo.path(),
        home.path(),
        &["milestone", "discard", MILESTONE_ID, "--force"],
        "milestone discard --force",
    );
    assert!(
        ack.contains("workbench removed"),
        "a clean teardown keeps the ack it earned:\n{ack}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("worktrees")
            .join(SUB_TASK)
            .exists(),
        "the premise: the worktree really is gone",
    );
}

// ---------------------------------------------------------------------------------
// D-4 — the worktrees-root fail-closed route
// ---------------------------------------------------------------------------------

/// **Cell D-4 — `.jigc/worktrees/` unreadable.** The refusal is right; its route was not. It
/// must name the unreadable path, name `read_dir` as what failed, and name `--force`, the
/// consent the same door's two sibling fail-closed refusals already name — instead of
/// blaming `git` on PATH for an `EACCES` on a directory.
#[test]
fn d4_the_unreadable_worktrees_root_names_read_dir_and_the_consent() {
    let (repo, home) = workbench("d4");
    let worktrees = repo.path().join(".jigc").join("worktrees");
    fs::create_dir_all(&worktrees).expect("create worktrees root");
    let out = Command::new("chmod")
        .args(["000", worktrees.to_str().expect("utf-8 path")])
        .output()
        .expect("chmod");
    assert!(out.status.success(), "chmod 000 must succeed");

    let ran = jigc(repo.path(), home.path(), &["uninstall"]);
    let text = both_streams(&ran);

    let _ = Command::new("chmod")
        .args(["u+rwx", worktrees.to_str().expect("utf-8 path")])
        .output();

    assert!(
        text.contains("uninstall.dirty-worktree"),
        "the cell must reach the fail-closed refusal; got:\n{text}",
    );
    assert!(
        text.contains(".jigc/worktrees/"),
        "the refusal names the unreadable path, repo-relative:\n{text}",
    );
    assert!(
        text.contains("read_dir"),
        "the route names what actually failed:\n{text}",
    );
    assert!(
        !text.contains("`git` is on PATH"),
        "an EACCES on a directory is not a missing `git`:\n{text}",
    );
    assert!(
        concrete_jigc_commands(&text).contains(&"jigc uninstall --force".to_string()),
        "the route names the consent its two sibling fail-closed refusals name:\n{text}",
    );
}

/// **Cell D-4', the omitting context.** With the root readable the door never reaches this
/// refusal at all — the fixture's own uninstall is answered by a different guard entirely, so
/// the new route text appears nowhere on a state that does not have the fault.
#[test]
fn d4_a_readable_worktrees_root_never_reaches_the_fail_closed_route() {
    let (repo, home) = workbench("d4-omit");
    let ran = jigc(repo.path(), home.path(), &["uninstall"]);
    let text = both_streams(&ran);
    assert!(
        !text.contains("read_dir"),
        "the fail-closed route must not render on a readable root:\n{text}",
    );
}
