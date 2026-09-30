//! **The CI job matrix, held to the targets cargo compiles** (M54 Increment 8, S8 ·
//! gate-record row 17).
//!
//! S8 splits the one serial CI job into a job per leg so a green push reports in ≤ 15
//! min and a red one reports **every** failure. A matrix is a hand-written list of test
//! targets, though, and `autotests = false` already makes the manifest's `[[test]]` list
//! hand-written too — so a group root the matrix lacks compiles, passes the local gate
//! and never runs in CI, and nothing reddens. This suite is that fence. It reads the
//! targets from **`cargo metadata`**, never from `tests/groups/` on disk or from a parse
//! of `Cargo.toml`, because what cargo reports is what `cargo test` compiles.
//!
//! Six properties, one arm each: **(a)** the `test` matrix equals the metadata's
//! `test`-kind targets; **(b)** every other tested target is covered by the `unit` job;
//! **(c)** every job S8 names exists and runs its pinned command; **(d)** every job
//! carries `timeout-minutes`, a full-history checkout and the `CPU count` step; **(e)**
//! every `cargo test` runs `--no-fail-fast` except the manifest-freeze fence's pinned
//! argv; **(f)** nothing stops the other legs when one fails — `fail-fast: false` and no
//! `needs`. The design is [dev-workflow.md](../../../implementation/dev-workflow.md) →
//! Gate; the measurement behind each `timeout-minutes` is
//! `completions/artifacts/M54/ci-runtime.md`.

use crate::manifest_freeze_fence::{CI_STEP_NAME, CI_STEP_RUN, CI_WORKFLOW, repo_root};
use crate::support::run_then_parse::stdout_json;
use serde_yaml_ng::Value;
use std::collections::BTreeSet;
use std::fs;
use std::process::Command;

/// The package the matrix row's command addresses with `-p`.
const MATRIX_PACKAGE: &str = "jigc";

/// The matrix row's command: one `[[test]]` group per job, every failure reported.
const MATRIX_RUN: &str = "cargo test -p jigc --test ${{ matrix.group }} --no-fail-fast";

/// The `unit` job's two commands: the lib and bin unit tests of every workspace member,
/// and every lib's doctests. `--workspace` is what reaches the engine.
const UNIT_LIB_BINS_RUN: &str = "cargo test --workspace --lib --bins --no-fail-fast";
const UNIT_DOC_RUN: &str = "cargo test --workspace --doc --no-fail-fast";

/// The step every job prints the runner's CPU count in — the name Increment 8's entry
/// gate read the before run's `nproc` by, and the one the *after* run is read by.
const CPU_STEP_NAME: &str = "CPU count";

/// Every job S8 names, each with the command it is pinned to run. `hygiene` is not an
/// S8 leg; it is the public-hygiene guard (`implementation/public-hygiene.md`), and its
/// pin is its three named steps rather than one command.
fn pinned_jobs() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("fmt", vec!["cargo fmt --check"]),
        ("clippy", vec!["cargo clippy --all-targets -- -D warnings"]),
        ("build", vec!["cargo build"]),
        ("test", vec![MATRIX_RUN]),
        ("unit", vec![UNIT_LIB_BINS_RUN, UNIT_DOC_RUN]),
        ("manifest-freeze", vec![CI_STEP_RUN]),
        (
            "publish-dry-run",
            vec!["cargo publish --workspace --dry-run"],
        ),
        // Never `--no-deps`: without resolution there is no lock to be stale against,
        // so a mutant pin that `--locked` refuses exits 0 under `--no-deps`.
        (
            "lock-current",
            vec!["cargo metadata --locked --format-version 1 > /dev/null"],
        ),
    ]
}

/// The public-hygiene guard's three steps, by name.
const HYGIENE_STEPS: [&str; 3] = ["hygiene range", "gitleaks", "denylist"];

fn workflow() -> Value {
    let path = repo_root().join(CI_WORKFLOW);
    let body = fs::read_to_string(&path).expect("the CI workflow is readable");
    serde_yaml_ng::from_str(&body).expect("the CI workflow is valid YAML")
}

fn jobs(workflow: &Value) -> Vec<(String, &Value)> {
    workflow["jobs"]
        .as_mapping()
        .expect("the CI workflow has a `jobs` mapping")
        .iter()
        .map(|(id, job)| (id.as_str().expect("a job id").to_string(), job))
        .collect()
}

fn job<'a>(workflow: &'a Value, id: &str) -> &'a Value {
    let job = &workflow["jobs"][id];
    assert!(
        job.is_mapping(),
        "the CI workflow has no `{id}` job. Jobs present: {:?}",
        jobs(workflow).iter().map(|(id, _)| id).collect::<Vec<_>>(),
    );
    job
}

fn steps(job: &Value) -> &[Value] {
    job["steps"].as_sequence().map_or(&[], Vec::as_slice)
}

fn step_runs(job: &Value) -> Vec<&str> {
    steps(job)
        .iter()
        .filter_map(|step| step["run"].as_str())
        .map(str::trim)
        .collect()
}

/// One workspace target, as `cargo metadata` reports it.
struct Target {
    package: String,
    name: String,
    kinds: Vec<String>,
    test: bool,
    doctest: bool,
}

/// Every target of every workspace member — `--no-deps`, so only members are listed,
/// and `--offline`, since no resolution is asked for.
fn targets() -> Vec<Target> {
    let out = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--format-version",
            "1",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run cargo metadata");
    let metadata: serde_json::Value = stdout_json(&out, &[0], "`cargo metadata --no-deps`");
    let mut targets = Vec::new();
    for package in metadata["packages"].as_array().expect("metadata packages") {
        let package_name = package["name"].as_str().expect("package name");
        for target in package["targets"].as_array().expect("package targets") {
            targets.push(Target {
                package: package_name.to_string(),
                name: target["name"].as_str().expect("target name").to_string(),
                kinds: target["kind"]
                    .as_array()
                    .expect("target kind")
                    .iter()
                    .map(|kind| kind.as_str().expect("a kind").to_string())
                    .collect(),
                test: target["test"].as_bool().expect("target test flag"),
                doctest: target["doctest"].as_bool().expect("target doctest flag"),
            });
        }
    }
    targets
}

impl Target {
    fn is(&self, kind: &str) -> bool {
        self.kinds.iter().any(|k| k == kind)
    }
}

/// **(a)** The `test` matrix is exactly the `test`-kind targets — a dropped row, an extra
/// row, and a `[[test]]` group root the matrix was never told about each redden, naming
/// the difference. Every one of them must also belong to the package the row's `-p`
/// names, or its row would address a target that package does not have.
#[test]
fn the_matrix_equals_the_test_targets_cargo_metadata_reports() {
    let workflow = workflow();
    let matrix: BTreeSet<String> = job(&workflow, "test")["strategy"]["matrix"]["group"]
        .as_sequence()
        .expect("the `test` job's `strategy.matrix.group` is a list")
        .iter()
        .map(|group| group.as_str().expect("a matrix group").to_string())
        .collect();
    let targets = targets();
    let test_targets: Vec<&Target> = targets.iter().filter(|t| t.is("test")).collect();
    let foreign: Vec<String> = test_targets
        .iter()
        .filter(|t| t.package != MATRIX_PACKAGE)
        .map(|t| format!("{} (package {})", t.name, t.package))
        .collect();
    assert!(
        foreign.is_empty(),
        "every matrix row runs `-p {MATRIX_PACKAGE}`, so a test target of another package \
         runs nowhere in CI: {foreign:?}",
    );
    let declared: BTreeSet<String> = test_targets.iter().map(|t| t.name.clone()).collect();
    let missing: Vec<&String> = declared.difference(&matrix).collect();
    let extra: Vec<&String> = matrix.difference(&declared).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "the `test` matrix in {CI_WORKFLOW} must equal the test targets cargo compiles.\n\
         In cargo metadata, not in the matrix (they would never run in CI): {missing:?}\n\
         In the matrix, not in cargo metadata (a row for a target that does not exist): \
         {extra:?}",
    );
}

/// **(b)** Every tested target that is not a `[[test]]` group is covered by the `unit`
/// job: a lib or bin by `--workspace --lib --bins`, a lib's doctests by `--doc`. A tested
/// target of any other kind (an example, a bench) is covered by nothing and reddens.
#[test]
fn every_other_tested_target_is_covered_by_the_unit_job() {
    let workflow = workflow();
    let unit_runs = step_runs(job(&workflow, "unit"));
    let targets = targets();
    let mut uncovered = Vec::new();
    for target in targets.iter().filter(|t| t.test && !t.is("test")) {
        if !target.is("lib") && !target.is("bin") {
            uncovered.push(format!(
                "{} {:?} (package {}): no CI job tests this kind",
                target.name, target.kinds, target.package
            ));
        } else if !unit_runs.contains(&UNIT_LIB_BINS_RUN) {
            uncovered.push(format!(
                "{} {:?}: the `unit` job does not run `{UNIT_LIB_BINS_RUN}`",
                target.name, target.kinds
            ));
        }
    }
    for target in targets.iter().filter(|t| t.doctest) {
        if !unit_runs.contains(&UNIT_DOC_RUN) {
            uncovered.push(format!(
                "{} (doctest): the `unit` job does not run `{UNIT_DOC_RUN}`",
                target.name
            ));
        }
    }
    assert!(
        uncovered.is_empty(),
        "every tested target reaches a CI job — the `test` matrix takes the `[[test]]` \
         groups and the `unit` job everything else:\n{}",
        uncovered.join("\n"),
    );
}

/// **(c)** Every job S8 names exists and runs its pinned command, and the hygiene job
/// carries the guard's three steps.
#[test]
fn every_job_s8_names_exists_and_runs_its_pinned_command() {
    let workflow = workflow();
    let mut wrong = Vec::new();
    for (id, commands) in pinned_jobs() {
        let Some(job) = workflow["jobs"].get(id).filter(|job| job.is_mapping()) else {
            wrong.push(format!("`{id}`: no such job"));
            continue;
        };
        let runs = step_runs(job);
        for command in commands {
            if !runs.contains(&command) {
                wrong.push(format!(
                    "`{id}`: no step runs `{command}` (it runs {runs:?})"
                ));
            }
        }
    }
    if let Some(job) = workflow["jobs"].get("manifest-freeze")
        && !steps(job).iter().any(|step| {
            step["name"].as_str() == Some(CI_STEP_NAME)
                && step["run"].as_str().map(str::trim) == Some(CI_STEP_RUN)
        })
    {
        wrong.push(format!(
            "`manifest-freeze`: `{CI_STEP_RUN}` does not run in the step named \
             `{CI_STEP_NAME}`"
        ));
    }
    match workflow["jobs"].get("hygiene") {
        Some(job) if job.is_mapping() => {
            let names: Vec<&str> = steps(job)
                .iter()
                .filter_map(|step| step["name"].as_str())
                .collect();
            for step in HYGIENE_STEPS {
                if !names.contains(&step) {
                    wrong.push(format!("`hygiene`: no step named `{step}`"));
                }
            }
        }
        _ => wrong.push("`hygiene`: no such job".to_string()),
    }
    assert!(
        wrong.is_empty(),
        "{CI_WORKFLOW} must run every leg S8 names as its own job:\n{}",
        wrong.join("\n"),
    );
}

/// **(d)** Every job is bounded, sees the whole history, and prints its CPU count. The
/// history is not only the freeze fence's need: every leg checks out the same way, so no
/// job's result depends on a clone depth another job does not share.
#[test]
fn every_job_is_bounded_checks_out_full_history_and_prints_its_cpu_count() {
    let workflow = workflow();
    let mut wrong = Vec::new();
    for (id, job) in jobs(&workflow) {
        if job["timeout-minutes"].as_u64().is_none_or(|m| m == 0) {
            wrong.push(format!(
                "`{id}`: `timeout-minutes` is not a positive integer ({:?})",
                job["timeout-minutes"]
            ));
        }
        let full_history = steps(job).iter().any(|step| {
            step["uses"]
                .as_str()
                .is_some_and(|uses| uses.starts_with("actions/checkout@"))
                && step["with"]["fetch-depth"].as_u64() == Some(0)
        });
        if !full_history {
            wrong.push(format!(
                "`{id}`: no `actions/checkout` step with `fetch-depth: 0`"
            ));
        }
        let cpu_count = steps(job).iter().any(|step| {
            step["name"].as_str() == Some(CPU_STEP_NAME)
                && step["run"].as_str().map(str::trim) == Some("nproc")
        });
        if !cpu_count {
            wrong.push(format!(
                "`{id}`: no step named `{CPU_STEP_NAME}` running `nproc`"
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **(e)** A red push reports every failure, so every `cargo test` runs `--no-fail-fast`
/// — except the manifest-freeze fence's pinned argv, which addresses one `--exact` test in
/// one binary and has nothing further to run.
#[test]
fn every_cargo_test_runs_no_fail_fast_except_the_pinned_freeze_argv() {
    let workflow = workflow();
    let mut wrong = Vec::new();
    for (id, job) in jobs(&workflow) {
        for run in step_runs(job) {
            if run == CI_STEP_RUN {
                continue;
            }
            for line in run.lines().filter(|line| line.contains("cargo test")) {
                if !line.contains("--no-fail-fast") {
                    wrong.push(format!("`{id}`: `{}`", line.trim()));
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a `cargo test` without `--no-fail-fast` stops at its first failing binary, so \
         a red push hides the rest:\n{}",
        wrong.join("\n"),
    );
}

/// **(f)** No leg stops another: the matrix is `fail-fast: false` (GitHub's default
/// cancels every sibling row on the first red one), and no job `needs` another (a red
/// `fmt` would skip every test).
#[test]
fn no_leg_stops_another() {
    let workflow = workflow();
    let fail_fast = &job(&workflow, "test")["strategy"]["fail-fast"];
    assert_eq!(
        fail_fast.as_bool(),
        Some(false),
        "the `test` matrix must set `fail-fast: false`; unset, GitHub cancels every \
         sibling row when one goes red (found {fail_fast:?})",
    );
    let chained: Vec<String> = jobs(&workflow)
        .into_iter()
        .filter(|(_, job)| !job["needs"].is_null())
        .map(|(id, job)| format!("`{id}` needs {:?}", job["needs"]))
        .collect();
    assert!(
        chained.is_empty(),
        "no job waits on another, so a red leg never hides the rest: {chained:?}",
    );
}
