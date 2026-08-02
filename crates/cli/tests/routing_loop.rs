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
const SINGLE_TASK_WHEN: &str =
    "implement one scoped change end-to-end, recording its decisions as ADRs";
const QUICK_FIX_WHEN: &str =
    "apply a small commit-only fix that touches no documented code and records no decision";
const PLAN_WHEN: &str = "draft the specification for upcoming work before writing any code";
const IMPLEMENT_FROM_SPEC_WHEN: &str =
    "a committed spec already covers the intent, with acceptance criteria to build against";
const PROJECT_SETUP_WHEN: &str =
    "bootstrap a brand-new project by developing the idea into its first product requirements";

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
    // `plan` is `creates-task: true`, so it joins the selectable catalog
    // automatically (Increment 3 T2 — `workflow-dialect.md` → Workflow selection).
    assert!(
        stdout.contains(&format!("- plan — {PLAN_WHEN}")),
        "the router must list plan with its `when` hint; got:\n{stdout}",
    );
    // `implement-from-spec` is `creates-task: true`, so it too joins the selectable
    // catalog automatically (Increment 4 T4 — `workflow-dialect.md` → Workflow
    // selection). With it the router lists all FOUR work-workflows.
    assert!(
        stdout.contains(&format!(
            "- implement-from-spec — {IMPLEMENT_FROM_SPEC_WHEN}"
        )),
        "the router must list implement-from-spec with its `when` hint; got:\n{stdout}",
    );
    // `project-setup` is `creates-task: true`, so it joins the selectable catalog
    // automatically (M9 Increment 1 T2 — the new-project on-ramp). With it the router
    // lists all FIVE work-workflows.
    assert!(
        stdout.contains(&format!("- project-setup — {PROJECT_SETUP_WHEN}")),
        "the router must list project-setup with its `when` hint; got:\n{stdout}",
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
fn step_3_the_work_workflow_when_hints_are_pairwise_non_overlapping() {
    // The router routes model-free only if its option lines distinguish the
    // workflows: the `when` hints must share no word longer than a stop-word, so
    // the agent has a real signal to pick on. With `plan` (Increment 3 T2) and
    // `implement-from-spec` (Increment 4 T4) joining the catalog the property must
    // hold pairwise across all FOUR selectables. (`sub-task` is `selectable: false`
    // — never a router pick — so it is not part of this property.)
    let lower = |s: &str| s.to_lowercase();
    let words = |s: &str| -> std::collections::HashSet<String> {
        lower(s)
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 4) // drop stop-words like "with", "no", "one", "end"
            .map(|w| w.to_string())
            .collect()
    };
    let selectables = [
        ("single-task", words(SINGLE_TASK_WHEN)),
        ("quick-fix", words(QUICK_FIX_WHEN)),
        ("plan", words(PLAN_WHEN)),
        ("implement-from-spec", words(IMPLEMENT_FROM_SPEC_WHEN)),
        ("project-setup", words(PROJECT_SETUP_WHEN)),
    ];
    for (i, (a_id, a)) in selectables.iter().enumerate() {
        for (b_id, b) in &selectables[i + 1..] {
            let overlap: Vec<_> = a.intersection(b).collect();
            assert!(
                overlap.is_empty(),
                "`{a_id}` and `{b_id}` `when` hints must be non-overlapping for model-free \
                 routing; shared content words: {overlap:?}",
            );
        }
    }
}

/// Parse the router's `- <id> — <when>` option lines straight from the LIVE
/// binary's composed router view, returning `(id, when)` pairs. Each option line
/// is `- <id> — <when>` (em-dash separated).
fn live_catalog(repo: &Path, home: &Path) -> Vec<(String, String)> {
    let out = run_start(repo, home, &["add a thing"]);
    assert!(
        out.status.success(),
        "bare `jigc start` must compose the router; streams:\n{}",
        streams(&out),
    );
    stdout_of(&out)
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .filter_map(|rest| rest.split_once(" — "))
        .map(|(id, when)| (id.trim().to_string(), when.trim().to_string()))
        .collect()
}

#[test]
fn step_5_the_live_router_catalog_lists_exactly_the_real_work_workflows() {
    // The deliverable: the router lists the real work-workflows. A TEST-FIXTURE
    // workflow shipped in the embedded pack (`creates-task: true`) would leak into
    // this production orientation surface — invisible to the hand-listed sweeps
    // above, which never iterate the live catalog. Drive the REAL binary and assert
    // the catalog the agent actually sees is exactly the real workflows, with
    // pairwise non-overlapping `when` hints over the LIVE list. Increment 5 (T2) adds
    // the fanned `sub-task` — but it is `selectable: false` (it ships no finalize
    // step; its only commit boundary is the parent milestone's `finalize`), so it
    // must NOT join the router catalog. The selectable list stays FOUR.
    let repo = TempDir::new("live-catalog");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let catalog = live_catalog(repo.path(), home.path());

    let ids: Vec<&str> = catalog.iter().map(|(id, _)| id.as_str()).collect();
    assert!(
        !ids.contains(&"sub-task"),
        "the finalize-less `sub-task` is `selectable: false` and must NOT appear in \
         the live router catalog; got:\n{ids:?}",
    );
    let mut sorted_ids = ids.clone();
    sorted_ids.sort_unstable();
    assert_eq!(
        sorted_ids,
        [
            "architecture-documentation",
            "implement-from-spec",
            "plan",
            "project-setup",
            "quick-fix",
            "record-decision",
            "single-task",
        ],
        "the live router must list EXACTLY the real selectable work-workflows — no \
         TEST-FIXTURE and no non-selectable `sub-task` leak; got:\n{ids:?}",
    );

    // The non-overlap property must hold over the LIVE catalog, not a hand-picked
    // list — a fixture sharing content words with a real workflow would break it.
    let words = |s: &str| -> std::collections::HashSet<String> {
        s.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 4)
            .map(|w| w.to_string())
            .collect()
    };
    let bagged: Vec<(&str, std::collections::HashSet<String>)> = catalog
        .iter()
        .map(|(id, when)| (id.as_str(), words(when)))
        .collect();
    for (i, (a_id, a)) in bagged.iter().enumerate() {
        for (b_id, b) in &bagged[i + 1..] {
            let overlap: Vec<_> = a.intersection(b).collect();
            assert!(
                overlap.is_empty(),
                "live catalog `{a_id}` and `{b_id}` `when` hints must be non-overlapping; \
                 shared content words: {overlap:?}",
            );
        }
    }
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
