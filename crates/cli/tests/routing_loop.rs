//! Independent END-TO-END ACCEPTANCE of the M2 routing loop, driving the REAL
//! `jigc` binary (`CARGO_BIN_EXE_jigc`) against throwaway temp git repos. Written
//! as the acceptance gate for M2 Increment 3 (`implementation/roadmap.md` →
//! Increment 3 → "the acceptance path") — NOT the builders' per-task integration
//! test (`start_compose.rs`), but the independent proof of the headline:
//! model-free selection among ≥2 work-workflows with a *real* (not hollow) router.
//!
//! The harness (TempDir + git + jigc helpers) deliberately re-implements the
//! established pattern from `e2e_audit.rs` / `superseding_decision.rs` so this file
//! is self-contained and does not depend on the builders' helpers.
//!
//! The acceptance path (post-flip — `DECISIONS.md` 2026-06-01 → M2 flips
//! `default-workflow` to `router`):
//!   1. bare `jigc start "<intent>"` composes the `router` (`creates-task: false`)
//!      — NO mint — listing `single-task` + `quick-fix` with their `when` hints and
//!      the literal agent-substitution re-run `jigc start --workflow <chosen>
//!      "<intent>"`;
//!   2. `jigc start --workflow quick-fix "<intent>"` MINTS + composes `quick-fix`,
//!      whose commit-only (`allows-create: []`) view differs MATERIALLY from
//!      `single-task`: no ADR/create affordance, no superseded-context line;
//!   3. the two work-workflows carry non-overlapping `when` hints;
//!   4. `jigc start --workflow <nonesuch> "<intent>"` rejects cleanly — non-zero,
//!      no panic, no mint.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ───────────────────────────── harness ─────────────────────────────

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-routing-loop-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// A real git repo with one commit (composition mints, which reads HEAD) + the
/// `.jigc/config/` project layer so the cascade resolves.
fn init_repo(repo: &Path) {
    let git = |args: &[&str]| {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc start <args>` with `cwd = repo`, `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("utf-8 stdout")
}

fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

// The two selectable work-workflows' `when` hints, verbatim from the pack
// (`pack/workflows/{single-task,quick-fix}.yaml`). The router renders one
// `- <id> — <when>` option line per selectable workflow.
const SINGLE_TASK_WHEN: &str = "implement one scoped change end-to-end";
const QUICK_FIX_WHEN: &str = "apply a small commit-only fix with no decision to record";

// ─────────────────── the routing-loop acceptance path ───────────────────

#[test]
fn step_1_bare_intent_composes_the_router_listing_both_workflows_without_minting() {
    let repo = TempDir::new("router");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = run_start(repo.path(), home.path(), &["add rate limiter"]);
    assert!(
        out.status.success(),
        "bare `jigc start \"<intent>\"` must exit 0; streams:\n{}",
        streams(&out),
    );
    let stdout = stdout_of(&out);

    // The router is `creates-task: false`: it mints NOTHING.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "bare `jigc start` composes the router — no .jigc/tasks/ dir may be minted; got:\n{stdout}",
    );

    // It lists BOTH selectable work-workflows as `- <id> — <when>` option lines.
    assert!(
        stdout.contains(&format!("- single-task — {SINGLE_TASK_WHEN}")),
        "the router must list single-task with its `when` hint; got:\n{stdout}",
    );
    assert!(
        stdout.contains(&format!("- quick-fix — {QUICK_FIX_WHEN}")),
        "the router must list quick-fix with its `when` hint; got:\n{stdout}",
    );

    // It carries the literal agent-substitution re-run prose.
    assert!(
        stdout.contains("jigc start --workflow <chosen> \"<intent>\""),
        "the router must carry the literal agent-substitution re-run; got:\n{stdout}",
    );

    // And ends with the routing footer (agent-text composition).
    assert!(
        stdout.trim_end().ends_with(ROUTING_FOOTER),
        "the composed router view must end with the routing footer; got:\n{stdout}",
    );
}

#[test]
fn step_2_chosen_quick_fix_mints_and_composes_materially_differently_from_single_task() {
    let home = TempDir::new("home");

    // The agent picks `quick-fix` and re-runs Form D in a fresh repo.
    let quick_repo = TempDir::new("quick");
    init_repo(quick_repo.path());
    let quick = run_start(
        quick_repo.path(),
        home.path(),
        &["--workflow", "quick-fix", "fix typo in readme"],
    );
    assert!(
        quick.status.success(),
        "`jigc start --workflow quick-fix \"<intent>\"` must exit 0; streams:\n{}",
        streams(&quick),
    );
    let quick_out = stdout_of(&quick);

    // quick-fix is `creates-task: true`: Form D minted a working area + base pin.
    assert!(
        quick_repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("fix-typo-in-readme")
            .join("base.json")
            .is_file(),
        "quick-fix must open .jigc/tasks/fix-typo-in-readme/ with a base pin; got:\n{quick_out}",
    );

    // It composed, embedding the resolved intent.
    assert!(
        quick_out.contains("fix typo in readme"),
        "the composed quick-fix view must embed the resolved intent; got:\n{quick_out}",
    );

    // The same intent composed through `single-task` in its own fresh repo, to
    // contrast the two composed views directly.
    let single_repo = TempDir::new("single");
    init_repo(single_repo.path());
    let single = run_start(
        single_repo.path(),
        home.path(),
        &["--workflow", "single-task", "fix typo in readme"],
    );
    assert!(
        single.status.success(),
        "`jigc start --workflow single-task \"<intent>\"` must exit 0; streams:\n{}",
        streams(&single),
    );
    let single_out = stdout_of(&single);

    // single-task DOES carry the ADR/create affordance + the superseded-context
    // line — anchoring that the difference below is material, not vacuous.
    assert!(
        single_out.to_lowercase().contains("adr"),
        "single-task's composed view must carry the ADR/create affordance; got:\n{single_out}",
    );
    assert!(
        single_out.to_lowercase().contains("supersede"),
        "single-task's composed view must carry the superseded-context line; got:\n{single_out}",
    );

    // quick-fix is commit-only (`allows-create: []`): no ADR/create affordance ...
    assert!(
        !quick_out.to_lowercase().contains("adr"),
        "quick-fix is commit-only — its composed view must carry no ADR/create affordance; got:\n{quick_out}",
    );
    // ... and no superseded-context line.
    assert!(
        !quick_out.to_lowercase().contains("supersede"),
        "quick-fix must carry no superseded-context line; got:\n{quick_out}",
    );
}

#[test]
fn step_3_the_two_work_workflow_when_hints_are_non_overlapping() {
    // The router routes model-free only if its option lines distinguish the
    // workflows: the `when` hints must share no word longer than a stop-word, so
    // the agent has a real signal to pick on.
    let lower = |s: &str| s.to_lowercase();
    let words = |s: &str| -> std::collections::HashSet<String> {
        lower(s)
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 4) // drop stop-words like "with", "no", "one", "end"
            .map(|w| w.to_string())
            .collect()
    };
    let single = words(SINGLE_TASK_WHEN);
    let quick = words(QUICK_FIX_WHEN);
    let overlap: Vec<_> = single.intersection(&quick).collect();
    assert!(
        overlap.is_empty(),
        "the two work-workflows' `when` hints must be non-overlapping for model-free \
         routing; shared content words: {overlap:?}",
    );
}

#[test]
fn step_4_unknown_chosen_workflow_rejects_cleanly_without_minting() {
    let repo = TempDir::new("unknown");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = run_start(
        repo.path(),
        home.path(),
        &["--workflow", "nonesuch", "add rate limiter"],
    );

    // Reject cleanly: non-zero, no panic.
    assert!(
        !out.status.success(),
        "an unknown `--workflow` id must exit non-zero; streams:\n{}",
        streams(&out),
    );
    let surfaced = streams(&out);
    assert!(
        !surfaced.contains("panicked"),
        "the rejection must not panic; got:\n{surfaced}",
    );
    assert!(
        surfaced.contains("nonesuch"),
        "the rejection must name the unknown id; got:\n{surfaced}",
    );

    // No mint precedes the rejection.
    assert!(
        !repo.path().join(".jigc").join("tasks").exists(),
        "an unknown workflow must reject before minting — no .jigc/tasks/ dir; got:\n{surfaced}",
    );
}
