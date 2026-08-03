//! M43 Inc 1 T6 — unknown-subcommand semantic guesses get the **honest sibling tip**,
//! never a silent alias (`DECISIONS.md` 2026-07-16 M43 Settle, cross-cutting; law 2 —
//! nothing hides; trial provenance: A1 papercut, log rec 254).
//!
//! Two guessed verbs agents actually reached for are curated (`crates/cli/src/cli.rs`
//! → `unknown_subcommand_tip`):
//!
//! - `jigc task discard-write` — the ghost verb the reconciliation route used to name
//!   (repaired at T4). clap's own did-you-mean steers to `discard`, which destroys the
//!   WHOLE task, so the tip must state that effect out loud.
//! - `jigc task status` — the tip names what the real siblings actually *do*
//!   (`task list` enumerates; `task validate <id>` previews part of the gate), not a bare
//!   did-you-mean.
//!
//! Each test asserts the **emitted stderr bytes**: clap's own error + usage output is
//! preserved (the guess is still a usage error — never a silent alias, so exit stays 2
//! and nothing dispatches), the honest tip follows it, and a placeholder-free command
//! span extracted from the emitted bytes **runs verbatim** (the emitted bytes are the
//! contract; placeholder-carrying spans are parse-fenced at construction by the T2
//! route fence, live in this debug-build binary). An unknown guess **outside** the
//! curated map stays inert: clap's error unchanged, no tip, exit 2 — never an error in
//! the tip machinery.
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
            "jigc-siblingtip-{tag}-{}-{:?}",
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

/// Assert the guess stayed a genuine clap usage error — exit 2, clap's own
/// `unrecognized subcommand` + usage output preserved on stderr (never a silent
/// alias: nothing dispatched) — and return the stderr text.
fn assert_usage_error_preserved(out: &std::process::Output, guess: &str) -> String {
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown subcommand stays a clap usage error (exit 2); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert!(
        stderr.contains(&format!("error: unrecognized subcommand '{guess}'")),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        stderr.contains("Usage: jigc task"),
        "clap's own usage output is preserved; got:\n{stderr}",
    );
    stderr
}

/// Extract the first backticked `jigc …` span from emitted text and run it verbatim,
/// asserting exit 0 — the emitted bytes are the contract, so the command an agent
/// would copy must actually run.
fn run_first_emitted_span(text: &str, repo: &Path, home: &Path) {
    let (_, tail) = text
        .split_once('`')
        .unwrap_or_else(|| panic!("no backticked span in:\n{text}"));
    let (span, _) = tail
        .split_once('`')
        .unwrap_or_else(|| panic!("unterminated backticked span in:\n{text}"));
    let argv: Vec<&str> = span.split_whitespace().collect();
    assert_eq!(
        argv.first(),
        Some(&"jigc"),
        "the emitted span must lead with the binary name; got `{span}`",
    );
    let run = jigc(repo, home, &argv[1..]);
    assert_eq!(
        run.status.code(),
        Some(0),
        "the emitted span `{span}` must run verbatim; stderr:\n{}",
        String::from_utf8_lossy(&run.stderr),
    );
}

/// `jigc task discard-write <path>` — the ghost-verb guess. clap's did-you-mean points
/// at `discard`; the honest tip must say what `task discard` actually DOES (abandons
/// the whole task), so the guesser is not steered into destroying it.
#[test]
fn task_discard_write_guess_gets_the_whole_task_effect_tip() {
    let repo = TempDir::new("discard-write");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "discard-write", "docs/adr.md"],
    );
    let stderr = assert_usage_error_preserved(&out, "discard-write");

    let tip_at = stderr
        .find("tip: no per-write discard exists")
        .unwrap_or_else(|| panic!("the honest tip is missing from:\n{stderr}"));
    let tip = &stderr[tip_at..];
    assert!(
        tip.contains("`jigc task discard <task-id>`"),
        "the tip names the real sibling; got:\n{tip}",
    );
    assert!(
        tip.contains("abandons the WHOLE task"),
        "the tip states the sibling's effect, not a bare did-you-mean; got:\n{tip}",
    );
    assert!(
        tip.contains("revert that file on disk"),
        "the tip names the per-write alternative (the on-disk revert); got:\n{tip}",
    );
}

/// `jigc task status` — the tip names what each real sibling does: `task list`
/// enumerates the active tasks, `task validate <id>` previews **part** of the gate.
/// The scoping is M47 Inc 4 T4's (law 1): the preview covers this task's content
/// findings, the carryover gate and the staging-independent `owner-artifact` causes,
/// so the tip says *part of* rather than re-asserting the retired flat promise. The
/// placeholder-free span (`jigc task list`) is extracted from the emitted bytes and
/// run verbatim.
#[test]
fn task_status_guess_gets_the_real_sibling_effects_and_the_span_runs() {
    let repo = TempDir::new("status");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["task", "status"]);
    let stderr = assert_usage_error_preserved(&out, "status");

    let tip_at = stderr
        .find("tip: ")
        .unwrap_or_else(|| panic!("the honest tip is missing from:\n{stderr}"));
    let tip = &stderr[tip_at..];
    assert!(
        tip.contains("`jigc task list` enumerates the active tasks"),
        "the tip states what `task list` does; got:\n{tip}",
    );
    assert!(
        tip.contains("`jigc task validate <task-id>` previews part of the finalize gate"),
        "the tip states what `task validate` does; got:\n{tip}",
    );
    for covered in ["content findings", "carryover", "owner-artifact"] {
        assert!(
            tip.contains(covered),
            "the tip names `{covered}` as covered by the preview; got:\n{tip}",
        );
    }

    // The emitted `jigc task list` span runs verbatim (exit 0) in a real repo.
    run_first_emitted_span(tip, repo.path(), home.path());
}

/// The omitting context: an unknown guess **outside** the curated map stays inert —
/// clap's error and exit 2 unchanged, no curated tip text — never an error in the
/// tip machinery itself.
#[test]
fn an_uncurated_guess_stays_a_plain_clap_error_with_no_tip() {
    let repo = TempDir::new("uncurated");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["task", "frobnicate"]);
    let stderr = assert_usage_error_preserved(&out, "frobnicate");
    assert!(
        !stderr.contains("no per-write discard exists")
            && !stderr.contains("enumerates the active tasks"),
        "an uncurated guess gets no curated tip; got:\n{stderr}",
    );
}

/// The parent-scoped context: the curated guess under the WRONG parent (`jigc doc
/// status`) stays inert — the map keys on (parent, guess), so a `doc` guess must not
/// receive the `task`-sibling tip.
#[test]
fn a_curated_guess_under_the_wrong_parent_gets_no_tip() {
    let repo = TempDir::new("wrong-parent");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["doc", "status"]);
    assert_eq!(
        out.status.code(),
        Some(2),
        "an unknown doc subcommand stays a clap usage error (exit 2)",
    );
    let stderr = String::from_utf8(out.stderr.clone()).expect("utf-8 stderr");
    assert!(
        stderr.contains("error: unrecognized subcommand 'status'"),
        "clap's own error line is preserved; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("enumerates the active tasks"),
        "the task-sibling tip must not fire under `doc`; got:\n{stderr}",
    );
}
