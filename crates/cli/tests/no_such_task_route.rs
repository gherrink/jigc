//! M43 Inc 1 T5 — the three divergent wrong-task-id messages converge on `jigc task
//! list` via one shared helper (`DECISIONS.md` 2026-07-16 M43 Settle, cross-cutting:
//! wrong-id routes converge — the old `start --task` route "list live tasks with
//! `jigc start`" was factually false, gap-pass verified on the real binary).
//!
//! Three surfaces reject a task id that names no live working area:
//!
//! - `jigc task <verb> <id>` (`task.rs` → `TaskArea::resolve`),
//! - `jigc start --task <id>` (`start.rs` → `resume_in_repo`),
//! - `jigc doc <verb> … --task <id>` (`doc.rs` → `ActiveTask::resolve`).
//!
//! Each test asserts the **emitted stderr bytes** carry the one converged message —
//! byte-identical across all three, the observable of the shared helper — and then
//! extracts the backticked route from those emitted bytes and **runs it verbatim**
//! (the route is the contract; a reconstructed command could mask a broken emission).
//!
//! **Since M51 Increment 6 / T1 the refusal is a typed `Finding`, not an `anyhow!`
//! string** (`engine::finalize::no_such_task_finding`), so the emitted bytes are the
//! house findings render — `blocking · finalize.no-task — …`, the `at:` locus, the
//! `route:` line, and the universal routing footer — rather than a bare sentence. The
//! convergence claim is unchanged and is asserted the same way: one set of bytes, at all
//! three doors. The route now rides the finding's own `route:` line, which is where this
//! suite reads it from; the footer's own `jigc` spans are not routes and are not read.
//!
//! **Since M53 Increment 3 / T3 the claim covers two sentences, not one**
//! (`completions/artifacts/M53/settle-record.md` → D3). A task id names an *absent* work
//! unit in two shapes — no directory at all, or a directory carrying no base pin — and the
//! second is a **leftover**, whose recovery is a path to clear rather than a roster to
//! read. Both are `finalize.no-task` at `task:<id>`, and the convergence claim is the same
//! claim in both: one set of bytes, at all three doors. So each shape has its own
//! byte-identity const and its own trio of arms, and the difference between them is
//! asserted rather than assumed — the absent sentence carries a **runnable** route this
//! suite executes verbatim, the residual sentence carries a `Human` route that names no
//! command at all, because jigc mints no verb that clears a leftover.
//!
//! Out of scope (a different state, its start-route correct): the **no-active-task**
//! reject (`no active task — start one with `jigc start``) when no `--task` is given
//! and no task exists.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! dev's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-nosuchtask-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        );
        path.push(unique);
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The one converged no-such-task refusal every wrong-id surface emits, byte for byte.
const CONVERGED: &str = "blocking · finalize.no-task — no task `nonexistent`\n  \
                         at: task:nonexistent\n  route: `jigc task list` lists the live \
                         tasks\n— jigc · run `jigc start` for orientation; all writes \
                         through `jigc`.\n";

/// The line the finding's route rides — the span this suite extracts and runs. The
/// routing footer below it also carries backticked `jigc` spans, and those are **not**
/// routes: reading the route off its own labelled line is what keeps the two apart.
const ROUTE_LABEL: &str = "\n  route: `";

/// Assert `out` is the converged wrong-id reject, then extract the backticked route
/// from the **emitted** stderr and run it verbatim in `repo` — the emitted bytes are
/// the contract, so the route the agent would copy must actually run (exit 0).
fn assert_converged_and_route_runs(out: &std::process::Output, repo: &Path, home: &Path) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "a wrong task id is an operational error (exit 1); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert_eq!(
        stderr, CONVERGED,
        "every wrong-task-id surface emits the one converged message (byte-identical \
         across the three — the shared-helper observable)",
    );
    assert!(
        !stderr.contains("list live tasks with `jigc start`"),
        "the factually-false `jigc start` listing claim must be gone; got:\n{stderr}",
    );

    // Extract the route from the emitted bytes: the first backticked span of the
    // finding's own `route:` line.
    let (_, tail) = stderr
        .split_once(ROUTE_LABEL)
        .unwrap_or_else(|| panic!("no backticked route on a `route:` line in:\n{stderr}"));
    let (route, _) = tail
        .split_once('`')
        .unwrap_or_else(|| panic!("unterminated backticked route in:\n{stderr}"));
    let argv: Vec<&str> = route.split_whitespace().collect();
    assert_eq!(
        argv.first(),
        Some(&"jigc"),
        "the emitted route must lead with the binary name; got `{route}`",
    );
    let run = jigc(repo, home, &argv[1..]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "the emitted route `{route}` must run verbatim; stderr:\n{}",
        String::from_utf8_lossy(&run.stderr),
    );
}

/// `jigc task <verb> <id>` with an unknown id (`task.rs` → `TaskArea::resolve`) — the
/// old "start one with `jigc start \"<intent>\"`" reject converges on `jigc task list`.
#[test]
fn task_verb_with_unknown_id_routes_to_task_list() {
    let repo = TempDir::new("task-verb");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}

/// `jigc start --task <id>` with an unknown id (`start.rs` → `resume_in_repo`) — the
/// old route claimed `jigc start` lists live tasks, which it never did; the reject
/// converges on `jigc task list`.
#[test]
fn start_resume_with_unknown_id_routes_to_task_list() {
    let repo = TempDir::new("start-resume");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--task", "nonexistent"],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}

/// `jigc doc <verb> … --task <id>` with an unknown explicit id (`doc.rs` →
/// `ActiveTask::resolve`) — already the right route; pinned onto the shared message.
#[test]
fn doc_verb_with_unknown_explicit_task_routes_to_task_list() {
    let repo = TempDir::new("doc-task");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:nope#type",
            "--value",
            "feat",
            "--task",
            "nonexistent",
        ],
    );
    assert_converged_and_route_runs(&out, repo.path(), home.path());
}

// ─────────────────── the second sentence: the residual (M53 Inc 3 / T3) ──────────────────

/// The one converged **residual** refusal every wrong-id surface emits over a directory
/// that carries no base pin, byte for byte.
///
/// Driven at `2e491967` the same three doors answered this state three different ways —
/// `task validate` *"the task validates clean"* at exit **0**, `doc set-field` a
/// `store.not-staged` block about a task's staged set, `start --task` a code-less
/// `could not read the base pin for task …` — because each had already accepted the
/// directory as a task and moved on to a later question about it.
const CONVERGED_RESIDUAL: &str = "blocking · finalize.no-task — no task `nonexistent`: `.jigc/tasks/nonexistent` is a \
     directory carrying no base pin, so it is a leftover and not a work unit — either jigc \
     never minted a task there, or a teardown stopped partway and left the directory \
     behind\n  at: task:nonexistent\n  route: nothing was changed. Keep anything you need \
     from `.jigc/tasks/nonexistent` and delete the rest by hand — jigc mints no verb that \
     clears a leftover working area, because what is in there is not jigc's to judge\n— \
     jigc · run `jigc start` for orientation; all writes through `jigc`.\n";

/// Plant the leftover the residual arms ask about: a bare `mkdir` at the id the absent
/// arms use, so the two sentences are driven over the *same* id and the only difference is
/// whether the directory is there.
fn plant_residual(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("tasks").join("nonexistent"))
        .expect("plant the residual working area");
}

/// Assert `out` is the converged residual reject — and that its route is **not** a command.
///
/// The absent sentence's route is runnable and this suite runs it; the residual's is a
/// `Human` route by decision, because there is no verb to run. Asserting the *absence* of a
/// `jigc` span is what keeps a later convergence from quietly handing this cell a
/// command — which would have to be a command that clears someone else's bytes.
fn assert_converged_residual(out: &std::process::Output, repo: &Path) {
    assert_eq!(
        out.status.code(),
        Some(1),
        "a residual is an absent work unit (exit 1), never a task that validates clean; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert_eq!(
        stderr, CONVERGED_RESIDUAL,
        "every wrong-task-id surface emits the one converged residual message \
         (byte-identical across the three — the shared-predicate observable)",
    );

    // The residual route does **not** open with a backticked span, which is why it is read
    // off the label alone rather than through `ROUTE_LABEL` (the absent sentence's
    // extractor, which requires the backtick immediately after the label).
    let (_, tail) = stderr
        .split_once("\n  route: ")
        .unwrap_or_else(|| panic!("no `route:` line in:\n{stderr}"));
    let route = tail
        .split_once('\n')
        .map(|(line, _)| line)
        .unwrap_or_else(|| panic!("unterminated `route:` line in:\n{stderr}"));
    let spans: Vec<&str> = route.split('`').skip(1).step_by(2).collect();
    assert_eq!(
        spans,
        vec![".jigc/tasks/nonexistent"],
        "the residual route's one backticked span is the path to clear, and nothing else",
    );
    assert!(
        !spans
            .iter()
            .any(|span| span.split_whitespace().next() == Some("jigc")),
        "jigc mints no verb that clears a leftover working area, so its route may not \
         name one; got `{route}`",
    );
    assert!(
        repo.join(".jigc")
            .join("tasks")
            .join("nonexistent")
            .is_dir(),
        "a refusal destroys nothing — the leftover is still on disk",
    );
}

/// `jigc task <verb> <id>` over a residual (`task.rs` → `TaskArea::resolve`).
#[test]
fn task_verb_over_a_residual_converges_on_the_leftover_sentence() {
    let repo = TempDir::new("task-verb-residual");
    let home = TempDir::new("home");
    init_repo(repo.path());
    plant_residual(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", "nonexistent"],
    );
    assert_converged_residual(&out, repo.path());
}

/// `jigc start --task <id>` over a residual (`start.rs` → `resume_in_repo`).
#[test]
fn start_resume_over_a_residual_converges_on_the_leftover_sentence() {
    let repo = TempDir::new("start-resume-residual");
    let home = TempDir::new("home");
    init_repo(repo.path());
    plant_residual(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--task", "nonexistent"],
    );
    assert_converged_residual(&out, repo.path());
}

/// `jigc doc <verb> … --task <id>` over a residual (`doc.rs` → `ActiveTask::resolve`).
#[test]
fn doc_verb_over_a_residual_converges_on_the_leftover_sentence() {
    let repo = TempDir::new("doc-task-residual");
    let home = TempDir::new("home");
    init_repo(repo.path());
    plant_residual(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:nope#type",
            "--value",
            "feat",
            "--task",
            "nonexistent",
        ],
    );
    assert_converged_residual(&out, repo.path());
}
