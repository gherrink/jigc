//! **The branch-name rule, held to jigc's slug grammar and to the CI step that runs it**
//! (the branching switch, 2026-10-01 — [DECISIONS.md](../../../DECISIONS.md) → *the
//! branching switch*; the model itself is [CLAUDE.md](../../../CLAUDE.md) → Branches).
//!
//! The closed set is `main`, `milestone/<slug>/main`, `milestone/<slug>/<slug>`,
//! `fix/<slug>`, `work/<slug>` and release-plz's own `release-plz-*`, where `<slug>` is
//! jigc's slug grammar. The rule lives in one place, [`dev/branch-name`](../../../dev/branch-name),
//! as a regular expression — and a regex that spells a grammar by hand is exactly the
//! copy that drifts from the function it copies. So **(a)** drives the script over a
//! table of names and holds each verdict to an oracle built on
//! `engine::slug::is_slug` itself, the recognizer every jigc id is checked with; the
//! table states its own expected verdict too, so neither the script nor the oracle can
//! move alone. **(b)** a refused name prints the allowed patterns, and a usage error is
//! not a verdict. **(c)** the hygiene job's `branch name` step reads the head branch on
//! a pull request, the pushed branch on a push, and checks nothing on a tag push or on
//! a pull request from a fork (the human's decision of 2026-10-01: a fork's branch is
//! its contributor's to name; pushes are always checked) — its body driven verbatim for
//! each event. **(d)** the fork is exempt from the name rule and nothing else: no
//! hygiene step is conditional, so gitleaks runs, and the denylist step, without the
//! secret a fork's pull request never receives, skips with its notice and exit 0.

use crate::support::ci_workflow::{CI_WORKFLOW, repo_root};
use serde_yaml_ng::Value;
use std::fs;
use std::process::{Command, Output};

const SCRIPT: &str = "dev/branch-name";
const STEP: &str = "branch name";

/// The oracle: the closed prefix set, each slug position recognized by jigc's own
/// `is_slug`.
fn conforms(name: &str) -> bool {
    use engine::slug::is_slug;
    if name == "main" {
        return true;
    }
    if let Some(rest) = name.strip_prefix("release-plz-") {
        return !rest.is_empty();
    }
    let parts: Vec<&str> = name.split('/').collect();
    match parts.as_slice() {
        ["fix", slug] | ["work", slug] => is_slug(slug),
        ["milestone", milestone, increment] => is_slug(milestone) && is_slug(increment),
        _ => false,
    }
}

/// Every name the table drives, with the verdict it must get.
const TABLE: &[(&str, bool)] = &[
    // the trunk
    ("main", true),
    // a milestone and its increments — the names the build harness mints
    ("milestone/findings-channel/main", true),
    ("milestone/findings-channel/pre-public-audit-and", true),
    ("milestone/findings-channel/close", true),
    ("milestone/m55/increment-1", true),
    ("milestone/a/b", true),
    // outside a milestone
    ("fix/ci-ok-skipped-result", true),
    ("work/branch-per-milestone", true),
    ("work/0", true),
    // release-plz's own
    ("release-plz-2026-10-01T10-41-19Z", true),
    ("release-plz-x", true),
    // the prefixes the set leaves out
    ("feat/findings-channel", false),
    ("docs/readme", false),
    ("chore/x", false),
    ("master", false),
    ("develop", false),
    ("HEAD", false),
    // shape: a milestone needs exactly two slugs
    ("milestone/findings-channel", false),
    ("milestone/findings-channel/", false),
    ("milestone//main", false),
    ("milestone/a/b/c", false),
    ("milestone", false),
    ("fix/", false),
    ("work", false),
    ("fix/a/b", false),
    // slug grammar: case, separators, edges, doubles, foreign characters
    ("Main", false),
    ("main/x", false),
    ("work/Branch", false),
    ("work/a_b", false),
    ("work/a.b", false),
    ("work/a b", false),
    ("work/-a", false),
    ("work/a-", false),
    ("work/a--b", false),
    ("work/café", false),
    ("milestone/M55/main", false),
    ("milestone/findings_channel/main", false),
    // release-plz needs something after its prefix, and only it may use it
    ("release-plz-", false),
    ("release-plz", false),
    ("work/release-plz-x", true),
];

fn script(args: &[&str]) -> Output {
    Command::new(repo_root().join(SCRIPT))
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("run {SCRIPT}: {e}"))
}

/// **(a)** The script, the oracle and the table agree on every name.
#[test]
fn a_the_script_agrees_with_is_slug_on_every_name() {
    let mut wrong = Vec::new();
    for &(name, expected) in TABLE {
        let oracle = conforms(name);
        let out = script(&[name]);
        let code = out.status.code();
        let verdict = match code {
            Some(0) => Some(true),
            Some(1) => Some(false),
            _ => None,
        };
        if oracle != expected || verdict != Some(expected) {
            wrong.push(format!(
                "{name:?}: table {expected}, is_slug oracle {oracle}, {SCRIPT} exit {code:?}"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{SCRIPT} must accept exactly the closed branch set, each slug position by jigc's \
         slug grammar (`engine::slug::is_slug`):\n{}",
        wrong.join("\n")
    );
}

/// **(b)** A refusal names what is allowed, so the push that fails says how to fix it;
/// a missing or empty argument is a usage error (2), never a verdict on a name.
#[test]
fn b_a_refusal_lists_the_allowed_patterns_and_usage_is_not_a_verdict() {
    let out = script(&["feat/x"]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    for pattern in [
        "main",
        "milestone/<slug>/main",
        "milestone/<slug>/<increment-slug>",
        "fix/<slug>",
        "work/<slug>",
        "release-plz-*",
    ] {
        assert!(
            stderr.contains(pattern),
            "a refusal must list `{pattern}` among the allowed patterns:\n{stderr}"
        );
    }
    for args in [&[][..], &[""][..], &["main", "work/x"][..]] {
        assert_eq!(
            script(args).status.code(),
            Some(2),
            "{SCRIPT} {args:?} is a usage error, not a verdict"
        );
    }
}

/// The CI workflow's hygiene job.
fn hygiene_job() -> Value {
    let body = fs::read_to_string(repo_root().join(CI_WORKFLOW)).expect("read the CI workflow");
    let workflow: Value = serde_yaml_ng::from_str(&body).expect("the CI workflow is YAML");
    workflow["jobs"]["hygiene"].clone()
}

/// The hygiene job's step named `name`.
fn hygiene_step(name: &str) -> Value {
    hygiene_job()["steps"]
        .as_sequence()
        .expect("the hygiene job has steps")
        .iter()
        .find(|step| step["name"].as_str() == Some(name))
        .unwrap_or_else(|| panic!("the hygiene job has no step named `{name}`"))
        .clone()
}

/// The hygiene job's `branch name` step.
fn branch_step() -> Value {
    hygiene_step(STEP)
}

/// Bash as GitHub's default `shell: bash` runs a step body, from the repository root
/// (the bodies call `dev/…` relative to the checkout).
fn step_shell(body: &str) -> Command {
    let mut shell = Command::new("bash");
    shell
        .args(["--noprofile", "--norc", "-e", "-o", "pipefail", "-c", body])
        .current_dir(repo_root());
    shell
}

/// The event fields the `branch name` step reads, one case's worth.
struct Event<'a> {
    event: &'a str,
    ref_type: &'a str,
    ref_name: &'a str,
    head_ref: &'a str,
    repo: &'a str,
    head_repo: &'a str,
}

/// Run the step's body verbatim over one event.
fn run_step(body: &str, e: &Event) -> Output {
    step_shell(body)
        .env("BRANCH_EVENT", e.event)
        .env("BRANCH_REF_TYPE", e.ref_type)
        .env("BRANCH_REF_NAME", e.ref_name)
        .env("BRANCH_HEAD_REF", e.head_ref)
        .env("BRANCH_REPO", e.repo)
        .env("BRANCH_HEAD_REPO", e.head_repo)
        .output()
        .expect("run bash over the step body")
}

/// This repository, as `github.repository` names it.
const REPO: &str = "gherrink/jigc";
/// A fork of it, as `github.event.pull_request.head.repo.full_name` names one.
const FORK: &str = "contributor/jigc";

/// **(c)** The step reads the right ref per event and runs the script on it. Its env
/// carries the six event fields, never spliced into the body; a pull request is judged
/// by its head branch (its `ref_name` is the `<n>/merge` ref), a push and a manual run by
/// the branch, and a tag push is not judged at all. A pull request whose head repository
/// is not this one is a fork's and is not judged either — a deleted fork's head
/// repository is null, so an empty one is a fork too — while a push is judged whatever
/// repository it runs in, a fork's own CI included.
#[test]
fn c_the_ci_step_checks_the_head_branch_of_a_pr_and_the_pushed_branch_and_skips_tags() {
    let step = branch_step();
    for (key, expression) in [
        ("BRANCH_EVENT", "${{ github.event_name }}"),
        ("BRANCH_REF_TYPE", "${{ github.ref_type }}"),
        ("BRANCH_REF_NAME", "${{ github.ref_name }}"),
        ("BRANCH_HEAD_REF", "${{ github.head_ref }}"),
        ("BRANCH_REPO", "${{ github.repository }}"),
        (
            "BRANCH_HEAD_REPO",
            "${{ github.event.pull_request.head.repo.full_name }}",
        ),
    ] {
        assert_eq!(
            step["env"][key].as_str(),
            Some(expression),
            "the `{STEP}` step must read `{key}` from `{expression}`"
        );
    }
    let body = step["run"].as_str().expect("the step has a body");
    assert!(
        !body.contains("${{"),
        "the `{STEP}` body splices an expression into the script; pass it through `env:`"
    );
    // (event, ref_type, ref_name, head_ref, repo, head_repo, passes)
    let cases = [
        (
            "push",
            "branch",
            "milestone/findings-channel/main",
            "",
            REPO,
            "",
            true,
        ),
        ("push", "branch", "feat/x", "", REPO, "", false),
        ("push", "branch", "patch-1", "", REPO, "", false),
        // a fork's own CI, on a push to the fork: still judged
        ("push", "branch", "patch-1", "", FORK, "", false),
        ("push", "tag", "jigc-v1.0.0-rc.22", "", REPO, "", true),
        ("workflow_dispatch", "branch", "work/x", "", REPO, "", true),
        (
            "workflow_dispatch",
            "branch",
            "Feature",
            "",
            REPO,
            "",
            false,
        ),
        (
            "pull_request",
            "branch",
            "7/merge",
            "milestone/findings-channel/main",
            REPO,
            REPO,
            true,
        ),
        (
            "pull_request",
            "branch",
            "7/merge",
            "release-plz-2026-10-01T10-41-19Z",
            REPO,
            REPO,
            true,
        ),
        (
            "pull_request",
            "branch",
            "7/merge",
            "feat/x",
            REPO,
            REPO,
            false,
        ),
        // the same head branch, from this repository and from a fork
        (
            "pull_request",
            "branch",
            "9/merge",
            "patch-1",
            REPO,
            REPO,
            false,
        ),
        (
            "pull_request",
            "branch",
            "9/merge",
            "patch-1",
            REPO,
            FORK,
            true,
        ),
        (
            "pull_request",
            "branch",
            "9/merge",
            "main",
            REPO,
            FORK,
            true,
        ),
        // a deleted fork: GitHub reports its head repository as null
        (
            "pull_request",
            "branch",
            "9/merge",
            "patch-1",
            REPO,
            "",
            true,
        ),
    ];
    let mut wrong = Vec::new();
    for (event, ref_type, ref_name, head_ref, repo, head_repo, passes) in cases {
        let e = Event {
            event,
            ref_type,
            ref_name,
            head_ref,
            repo,
            head_repo,
        };
        let out = run_step(body, &e);
        if out.status.success() != passes {
            wrong.push(format!(
                "{event} {ref_type} {ref_name:?} (head {head_ref:?} of {head_repo:?}, in \
                 {repo:?}): expected {}, exit {:?}\n{}",
                if passes { "pass" } else { "refusal" },
                out.status.code(),
                String::from_utf8_lossy(&out.stderr)
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// **(d)** A fork's pull request is exempt from the name rule and from nothing else.
/// Neither the job nor any hygiene step carries an `if:`, so gitleaks runs once the name
/// step passes; gitleaks reads no secret; and the denylist step, with `JIGC_DENYLIST`
/// empty — what a fork's pull request receives — or unset, exits 0 with its notice
/// rather than failing or passing silently.
#[test]
fn d_a_fork_pull_request_still_gets_gitleaks_and_a_clean_denylist_skip() {
    let job = hygiene_job();
    assert!(
        job.get("if").is_none(),
        "the hygiene job must not be conditional"
    );
    let conditional: Vec<String> = job["steps"]
        .as_sequence()
        .expect("the hygiene job has steps")
        .iter()
        .filter(|step| step.get("if").is_some())
        .map(|step| format!("{:?}", step["name"]))
        .collect();
    assert!(
        conditional.is_empty(),
        "no hygiene step may be conditional, or a fork's pull request could skip it: \
         {conditional:?}"
    );
    let gitleaks = serde_yaml_ng::to_string(&hygiene_step("gitleaks")).expect("re-serialize");
    assert!(
        !gitleaks.contains("secrets."),
        "the gitleaks step must read no secret, so it runs the same on a fork's pull request"
    );
    let denylist = hygiene_step("denylist");
    assert_eq!(
        denylist["env"]["JIGC_DENYLIST"].as_str(),
        Some("${{ secrets.JIGC_DENYLIST }}")
    );
    let body = denylist["run"]
        .as_str()
        .expect("the denylist step has a body");
    for secret in [Some(""), None] {
        let mut shell = step_shell(body);
        shell.env("HYGIENE_RANGE", "HEAD");
        match secret {
            Some(value) => shell.env("JIGC_DENYLIST", value),
            None => shell.env_remove("JIGC_DENYLIST"),
        };
        let out = shell.output().expect("run bash over the denylist body");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success() && stdout.contains("::notice title=denylist skipped::"),
            "with JIGC_DENYLIST {secret:?} the denylist step must skip with its notice and \
             exit 0; exit {:?}\n{stdout}{}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        );
    }
}
