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
//! Nine properties, one arm each: **(a)** the `test` matrix's rows name exactly the
//! metadata's `test`-kind targets; **(b)** every other tested target is covered by the
//! `unit` job; **(c)** every job S8 names exists and runs its pinned command; **(d)**
//! every job carries `timeout-minutes`, and every leg a full-history checkout and the
//! `CPU count` step; **(e)** every `cargo test` runs `--no-fail-fast` except the
//! manifest-freeze fence's pinned argv; **(f)** nothing stops the other legs when one
//! fails — `fail-fast: false` and no `needs` on any leg; **(g)** the one aggregate,
//! `ci-ok`, `needs` exactly every other job and runs `if: always()`; **(h)** its body is
//! green only when every needed job's result is `success`, driven over each result GitHub
//! reports; **(i)** a group that runs in parts runs whole — every part but one names the
//! suites it runs, and the one that is left skips exactly those. The design is
//! [dev-workflow.md](../implementation/dev-workflow.md) → Gate, the required check
//! [release.md](../implementation/release.md) → The main branch; the measurement behind
//! each `timeout-minutes` is `completions/artifacts/M54/ci-runtime.md`, and behind a
//! part's the `DECISIONS.md` entry the workflow's header names.
//!
//! **Why a group runs in parts (2026-10-12).** A row is one test target, and one target's
//! tests can outgrow one leg: `g_tooling` grew past its leg's ten minutes with the
//! stabilization build and was cancelled there on thirty pushes in a row, so nothing said
//! its tests pass on Linux. A target is not split to shorten its leg — a target per part
//! is a test binary more for every local gate to link — so its row is: the matrix's
//! `include` lists the group once per part, each part with the libtest arguments that
//! select it (`tests`) and its own bound (`minutes`). What that risks is a test no part
//! runs, which arm (i) is for.

use crate::support::ci_workflow::{CI_STEP_NAME, CI_STEP_RUN, CI_WORKFLOW, repo_root};
use crate::support::run_then_parse::stdout_json;
use serde_yaml_ng::Value;
use std::collections::BTreeSet;
use std::fs;
use std::process::Command;

/// The package the matrix row's command addresses with `-p`.
const MATRIX_PACKAGE: &str = "jigc";

/// The matrix row's command: one `[[test]]` group per job — or one part of one, by the
/// libtest arguments its row carries, which a whole group's row has none of — and every
/// failure reported.
const MATRIX_RUN: &str =
    "cargo test -p jigc --test ${{ matrix.group }} --no-fail-fast ${{ matrix.tests }}";

/// The `test` job's bound: a part's own minutes, and ten for a row that names none.
const MATRIX_TIMEOUT: &str = "${{ matrix.minutes || 10 }}";

/// The floor of every leg's bound, in minutes (the workflow's header: `max(10, …)`).
const TIMEOUT_FLOOR: u64 = 10;

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

/// The public-hygiene guard's steps, by name: the branch-name rule
/// (`branch_name_fence.rs` holds its body), then the two scans over the pushed range.
const HYGIENE_STEPS: [&str; 4] = ["branch name", "hygiene range", "gitleaks", "denylist"];

/// The one aggregate job — the only check `main`'s ruleset requires — and the step that
/// decides it.
const AGGREGATE_JOB: &str = "ci-ok";
const AGGREGATE_STEP: &str = "every job succeeded";
/// The env var the aggregate's step reads `toJSON(needs)` from, and its expression.
const AGGREGATE_ENV: &str = "CI_NEEDS";
const AGGREGATE_ENV_VALUE: &str = "${{ toJSON(needs) }}";

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
    /// The target's root file, as cargo resolved it.
    src_path: String,
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
                src_path: target["src_path"]
                    .as_str()
                    .expect("target src_path")
                    .to_string(),
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

/// The groups the `test` matrix runs whole: its `group` list, one row each.
fn whole_groups(workflow: &Value) -> Vec<String> {
    job(workflow, "test")["strategy"]["matrix"]["group"]
        .as_sequence()
        .expect("the `test` job's `strategy.matrix.group` is a list")
        .iter()
        .map(|group| group.as_str().expect("a matrix group").to_string())
        .collect()
}

/// One row of the matrix's `include`: a part of a group.
struct Part {
    group: String,
    name: String,
    /// The suites it names — libtest's positional filters, each run where a test's name
    /// holds it.
    filters: Vec<String>,
    /// The suites it leaves to the other parts — libtest's `--skip`.
    skips: Vec<String>,
    /// Its own bound, where it names one.
    minutes: Option<Value>,
}

/// Every part the matrix names. A part's `tests` is read whole: `--`, then filters and
/// `--skip <filter>` pairs and nothing else — any other libtest flag (`--exact`,
/// `--ignored`) changes which tests a filter selects, so none is taken.
fn parts(workflow: &Value) -> Vec<Part> {
    let Some(include) = job(workflow, "test")["strategy"]["matrix"]["include"].as_sequence() else {
        return Vec::new();
    };
    include
        .iter()
        .map(|row| {
            let text = |key: &str| -> String {
                row[key]
                    .as_str()
                    .unwrap_or_else(|| panic!("an `include` row names its `{key}`: {row:?}"))
                    .to_string()
            };
            let (group, name, tests) = (text("group"), text("part"), text("tests"));
            let mut words = tests.split_whitespace();
            assert_eq!(
                words.next(),
                Some("--"),
                "`{group}` part `{name}`: `tests` opens with `--`, so that what follows is \
                 libtest's and never cargo's: {tests:?}"
            );
            let (mut filters, mut skips) = (Vec::new(), Vec::new());
            while let Some(word) = words.next() {
                if word == "--skip" {
                    let skipped = words.next().filter(|next| !next.starts_with('-'));
                    let skipped = skipped.unwrap_or_else(|| {
                        panic!("`{group}` part `{name}`: a `--skip` names what it skips: {tests:?}")
                    });
                    skips.push(skipped.to_string());
                } else {
                    assert!(
                        !word.starts_with('-'),
                        "`{group}` part `{name}`: `{word}` is neither a filter nor a `--skip`: \
                         {tests:?}"
                    );
                    filters.push(word.to_string());
                }
            }
            Part {
                group,
                name,
                filters,
                skips,
                minutes: row.get("minutes").cloned(),
            }
        })
        .collect()
}

/// **(a)** The `test` matrix's rows name exactly the `test`-kind targets — a dropped row,
/// an extra row, and a `[[test]]` group root the matrix was never told about each redden,
/// naming the difference. A group is named by the `group` list, which runs it whole, or
/// by `include`, which runs it in parts (arm (i)) — never by both: GitHub adds an
/// `include` row to a row the list already made, so the group would run once, as its
/// last part alone. Every one of them must also belong to the package the row's `-p`
/// names, or its row would address a target that package does not have.
#[test]
fn the_matrix_equals_the_test_targets_cargo_metadata_reports() {
    let workflow = workflow();
    let whole = whole_groups(&workflow);
    let in_parts: BTreeSet<String> = parts(&workflow)
        .into_iter()
        .map(|part| part.group)
        .collect();
    let both: Vec<&String> = whole
        .iter()
        .filter(|group| in_parts.contains(*group))
        .collect();
    assert!(
        both.is_empty(),
        "a group in the `group` list and in `include` runs as ONE row, its last part's: \
         {both:?}",
    );
    let matrix: BTreeSet<String> = whole.into_iter().chain(in_parts).collect();
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

/// **(d)** Every job is bounded, and every leg sees the whole history and prints its CPU
/// count. A job's bound is a positive integer; the `test` matrix's may instead be its
/// rows' — exactly `${{ matrix.minutes || 10 }}`, with every `minutes` a part names an
/// integer no lower than the floor. The history is not only the freeze fence's need: every leg checks out the same
/// way, so no job's result depends on a clone depth another job does not share. The
/// aggregate is bounded too, but it checks nothing out and runs no cargo — it reads the
/// `needs` context alone — so the checkout and the CPU count are not asked of it.
#[test]
fn every_job_is_bounded_checks_out_full_history_and_prints_its_cpu_count() {
    let workflow = workflow();
    let mut wrong = Vec::new();
    for (id, job) in jobs(&workflow) {
        // The matrix's bound is its rows': a part's own minutes, ten for every other row.
        let by_row = id == "test" && job["timeout-minutes"].as_str() == Some(MATRIX_TIMEOUT);
        if !by_row && job["timeout-minutes"].as_u64().is_none_or(|m| m == 0) {
            wrong.push(format!(
                "`{id}`: `timeout-minutes` is not a positive integer ({:?})",
                job["timeout-minutes"]
            ));
        }
        for part in parts(&workflow).iter().filter(|_| id == "test") {
            let Some(minutes) = &part.minutes else {
                continue;
            };
            if !by_row {
                wrong.push(format!(
                    "`{}` part `{}` names `minutes`, and the job's `timeout-minutes` is not \
                     `{MATRIX_TIMEOUT}`, so nothing reads them",
                    part.group, part.name
                ));
            } else if minutes.as_u64().is_none_or(|m| m < TIMEOUT_FLOOR) {
                wrong.push(format!(
                    "`{}` part `{}`: `minutes` is not an integer of at least \
                     {TIMEOUT_FLOOR} ({minutes:?})",
                    part.group, part.name
                ));
            }
        }
        if id == AGGREGATE_JOB {
            continue;
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
/// cancels every sibling row on the first red one), and no leg `needs` another (a red
/// `fmt` would skip every test). The aggregate is the one job that waits, and it waits
/// on all of them — arm (g).
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
        .filter(|(id, job)| id != AGGREGATE_JOB && !job["needs"].is_null())
        .map(|(id, job)| format!("`{id}` needs {:?}", job["needs"]))
        .collect();
    assert!(
        chained.is_empty(),
        "no job waits on another, so a red leg never hides the rest: {chained:?}",
    );
}

/// The suites a group root registers: the `<name>` of each `#[path = "../<name>.rs"]`.
fn registered_suites(root: &str) -> BTreeSet<String> {
    let body = fs::read_to_string(root).unwrap_or_else(|e| panic!("read `{root}`: {e}"));
    body.lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("#[path = \"../")?
                .strip_suffix(".rs\"]")
        })
        .filter(|name| !name.contains('/'))
        .map(str::to_string)
        .collect()
}

/// **(i)** A group that runs in parts runs whole. Its parts are read off the matrix's
/// `include`: every part but one names the suites it runs (`<suite>::`, each a suite the
/// group's root registers, none named twice), and the one part left skips exactly the
/// suites the others name — so a test is in a named part where its name holds a named
/// suite's, and in the last one where it holds none, and a suite added to the root runs
/// in the last part without a line here. What reddens: a part dropped or a suite renamed
/// while the last part still skips it (those tests would run nowhere), a suite named and
/// not skipped (they would run twice), two parts under one name, and a group with one
/// part, which is a group to run whole.
///
/// **Its bound.** It holds which suites a part names, never how long the part runs: a
/// part that outgrows its minutes is cancelled, and `ci-ok` is red for it.
#[test]
fn i_a_group_that_runs_in_parts_runs_whole() {
    let workflow = workflow();
    let targets = targets();
    let parts = parts(&workflow);
    let groups: BTreeSet<&str> = parts.iter().map(|part| part.group.as_str()).collect();
    let mut wrong = Vec::new();
    for group in groups {
        let of_group: Vec<&Part> = parts.iter().filter(|part| part.group == group).collect();
        let names: BTreeSet<&str> = of_group.iter().map(|part| part.name.as_str()).collect();
        if of_group.len() < 2 || names.len() != of_group.len() || names.contains("") {
            wrong.push(format!(
                "`{group}`: a group in parts has two or more, each under a name of its own \
                 (found {:?})",
                of_group.iter().map(|part| &part.name).collect::<Vec<_>>()
            ));
        }
        let (named, left): (Vec<&Part>, Vec<&Part>) =
            of_group.iter().partition(|part| part.skips.is_empty());
        let [left] = left[..] else {
            wrong.push(format!(
                "`{group}`: exactly one part runs what the others leave, by `--skip` (found \
                 {})",
                left.len()
            ));
            continue;
        };
        if !left.filters.is_empty() {
            wrong.push(format!(
                "`{group}` part `{}`: the part that skips names no suite of its own, or a \
                 test outside both lists runs nowhere ({:?})",
                left.name, left.filters
            ));
        }
        let ran: Vec<&String> = named.iter().flat_map(|part| &part.filters).collect();
        let ran_once: BTreeSet<&String> = ran.iter().copied().collect();
        let skipped: BTreeSet<&String> = left.skips.iter().collect();
        if ran.len() != ran_once.len()
            || left.skips.len() != skipped.len()
            || named.iter().any(|part| part.filters.is_empty())
            || ran_once != skipped
        {
            wrong.push(format!(
                "`{group}`: part `{}` must skip exactly what the other parts name, each \
                 once.\nNamed by a part: {ran:?}\nSkipped by `{}`: {:?}",
                left.name, left.name, left.skips
            ));
        }
        let Some(target) = targets.iter().find(|t| t.is("test") && t.name == group) else {
            // Arm (a) names a row for a target that does not exist.
            continue;
        };
        let suites = registered_suites(&target.src_path);
        for filter in ran_once.iter().chain(&skipped) {
            if filter
                .strip_suffix("::")
                .is_none_or(|suite| !suites.contains(suite))
            {
                wrong.push(format!(
                    "`{group}`: `{filter}` is not `<suite>::` for a suite `{}` registers",
                    target.src_path
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "a group the `test` matrix runs in parts must run whole:\n{}",
        wrong.join("\n"),
    );
}

/// The aggregate's `needs`, as a list of job ids.
fn aggregate_needs(workflow: &Value) -> Vec<String> {
    let needs = &job(workflow, AGGREGATE_JOB)["needs"];
    match needs {
        Value::String(one) => vec![one.clone()],
        Value::Sequence(many) => many
            .iter()
            .map(|id| {
                id.as_str()
                    .expect("a `needs` entry is a job id")
                    .to_string()
            })
            .collect(),
        other => panic!("`{AGGREGATE_JOB}`'s `needs` must be a job id or a list, found {other:?}"),
    }
}

/// **(g)** `ci-ok` waits on every other job and reports whatever they did. Its `needs` is
/// exactly the set of every other job — a job added to the workflow and not here would
/// run without being required, and its red would merge — and it runs `if: always()`,
/// without which GitHub *skips* a job whose needs failed, and a skipped job reports
/// `Success` to a required status check (GitHub's docs, *Using jobs* → *Using
/// conditions to control job execution*), so a red leg would merge.
#[test]
fn g_the_aggregate_needs_every_other_job_and_always_runs() {
    let workflow = workflow();
    let others: BTreeSet<String> = jobs(&workflow)
        .into_iter()
        .map(|(id, _)| id)
        .filter(|id| id != AGGREGATE_JOB)
        .collect();
    let needs: Vec<String> = aggregate_needs(&workflow);
    let needed: BTreeSet<String> = needs.iter().cloned().collect();
    assert_eq!(
        needs.len(),
        needed.len(),
        "`{AGGREGATE_JOB}` names a job twice in `needs`: {needs:?}"
    );
    let missing: Vec<&String> = others.difference(&needed).collect();
    let extra: Vec<&String> = needed.difference(&others).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "`{AGGREGATE_JOB}` must need exactly every other job in {CI_WORKFLOW}, because it is \
         the only check `main` requires.\nJobs it does not need (their red would merge): \
         {missing:?}\nNeeds naming no job: {extra:?}",
    );
    let aggregate = job(&workflow, AGGREGATE_JOB);
    assert_eq!(
        aggregate["if"].as_str().map(str::trim),
        Some("always()"),
        "`{AGGREGATE_JOB}` must run `if: always()`; without it GitHub skips the job when a \
         needed one fails, and a skipped required check reports success (found {:?})",
        aggregate["if"]
    );
}

/// The aggregate's deciding step: its `run:` body, with its env asserted to carry the
/// whole `needs` context.
fn aggregate_body(workflow: &Value) -> String {
    let step = steps(job(workflow, AGGREGATE_JOB))
        .iter()
        .find(|step| step["name"].as_str() == Some(AGGREGATE_STEP))
        .unwrap_or_else(|| panic!("`{AGGREGATE_JOB}` has no step named `{AGGREGATE_STEP}`"));
    assert_eq!(
        step["env"][AGGREGATE_ENV].as_str(),
        Some(AGGREGATE_ENV_VALUE),
        "`{AGGREGATE_STEP}` must read the whole `needs` context from `{AGGREGATE_ENV}`"
    );
    let body = step["run"].as_str().expect("the aggregate step has a body");
    assert!(
        !body.contains("${{"),
        "the aggregate body splices an expression into the script; pass it through `env:`"
    );
    body.to_string()
}

/// Run the aggregate's body as GitHub's default `shell: bash` does, with `CI_NEEDS` set.
fn run_aggregate(body: &str, needs_json: &str) -> std::process::Output {
    let dir = std::env::temp_dir().join(format!(
        "jigc-ci-ok-{}-{:?}",
        std::process::id(),
        engine::tempname::unique_nanos(),
    ));
    fs::create_dir_all(&dir).expect("create the body's temp dir");
    let script = dir.join("step.sh");
    fs::write(&script, body).expect("write the aggregate body");
    let out = Command::new("bash")
        .args(["--noprofile", "--norc", "-e", "-o", "pipefail"])
        .arg(&script)
        .env(AGGREGATE_ENV, needs_json)
        .output()
        .expect("run bash over the aggregate body");
    let _ = fs::remove_dir_all(&dir);
    out
}

/// The `needs` context GitHub would pass, every job given `result`, one job overridden.
fn needs_context(jobs: &[String], odd_one: Option<(&str, &str)>) -> String {
    let entries: Vec<String> = jobs
        .iter()
        .map(|id| {
            let result = match odd_one {
                Some((odd, result)) if odd == id => result,
                _ => "success",
            };
            format!("{id:?}: {{\"result\": {result:?}, \"outputs\": {{}}}}")
        })
        .collect();
    format!("{{{}}}", entries.join(", "))
}

/// **(h)** The aggregate is green only when every needed job succeeded. Driven verbatim
/// over each result a job can report: all `success` passes; one `failure`, `cancelled` or
/// `skipped` — for every needed job in turn — fails and names that job; an empty context
/// and one that does not parse fail too, so the step fails closed rather than passing
/// over nothing.
#[test]
fn h_the_aggregate_is_green_only_when_every_needed_job_succeeded() {
    let workflow = workflow();
    let body = aggregate_body(&workflow);
    let needs = aggregate_needs(&workflow);

    let out = run_aggregate(&body, &needs_context(&needs, None));
    assert!(
        out.status.success(),
        "every needed job succeeded, so `{AGGREGATE_JOB}` must pass:\n{}",
        String::from_utf8_lossy(&out.stdout)
    );

    let mut wrong = Vec::new();
    for odd in &needs {
        for result in ["failure", "cancelled", "skipped"] {
            let out = run_aggregate(&body, &needs_context(&needs, Some((odd, result))));
            let stdout = String::from_utf8_lossy(&out.stdout);
            if out.status.success() || !stdout.contains(&format!("{odd}: {result}")) {
                wrong.push(format!(
                    "`{odd}` = {result}: exit {:?}, stdout {stdout:?}",
                    out.status.code()
                ));
            }
        }
    }
    for (what, context) in [
        ("an empty context", "{}"),
        ("a context that does not parse", "{"),
    ] {
        if run_aggregate(&body, context).status.success() {
            wrong.push(format!("{what} passed"));
        }
    }
    assert!(
        wrong.is_empty(),
        "`{AGGREGATE_JOB}` must fail, naming the job, whenever a needed job did not succeed:\n{}",
        wrong.join("\n")
    );
}
