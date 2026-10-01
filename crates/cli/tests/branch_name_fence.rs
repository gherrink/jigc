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
//! a pull request, the pushed branch on a push, and checks nothing on a tag push —
//! its body driven verbatim for each event.

use crate::manifest_freeze_fence::{CI_WORKFLOW, repo_root};
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

/// The hygiene job's `branch name` step.
fn branch_step() -> Value {
    let body = fs::read_to_string(repo_root().join(CI_WORKFLOW)).expect("read the CI workflow");
    let workflow: Value = serde_yaml_ng::from_str(&body).expect("the CI workflow is YAML");
    let steps = workflow["jobs"]["hygiene"]["steps"]
        .as_sequence()
        .expect("the hygiene job has steps");
    steps
        .iter()
        .find(|step| step["name"].as_str() == Some(STEP))
        .unwrap_or_else(|| panic!("the hygiene job has no step named `{STEP}`"))
        .clone()
}

/// Run the step's body verbatim, as GitHub's default `shell: bash` does, from the
/// repository root (the body calls `dev/branch-name` relative to the checkout).
fn run_step(body: &str, event: &str, ref_type: &str, ref_name: &str, head_ref: &str) -> Output {
    Command::new("bash")
        .args(["--noprofile", "--norc", "-e", "-o", "pipefail", "-c", body])
        .current_dir(repo_root())
        .env("BRANCH_EVENT", event)
        .env("BRANCH_REF_TYPE", ref_type)
        .env("BRANCH_REF_NAME", ref_name)
        .env("BRANCH_HEAD_REF", head_ref)
        .output()
        .expect("run bash over the step body")
}

/// **(c)** The step reads the right ref per event and runs the script on it. Its env
/// carries the four event fields, never spliced into the body; a pull request is judged
/// by its head branch (its `ref_name` is the `<n>/merge` ref), a push and a manual run by
/// the branch, and a tag push is not judged at all.
#[test]
fn c_the_ci_step_checks_the_head_branch_of_a_pr_and_the_pushed_branch_and_skips_tags() {
    let step = branch_step();
    for (key, expression) in [
        ("BRANCH_EVENT", "${{ github.event_name }}"),
        ("BRANCH_REF_TYPE", "${{ github.ref_type }}"),
        ("BRANCH_REF_NAME", "${{ github.ref_name }}"),
        ("BRANCH_HEAD_REF", "${{ github.head_ref }}"),
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
    // (event, ref_type, ref_name, head_ref, passes)
    let cases = [
        (
            "push",
            "branch",
            "milestone/findings-channel/main",
            "",
            true,
        ),
        ("push", "branch", "feat/x", "", false),
        ("push", "tag", "jigc-v1.0.0-rc.22", "", true),
        ("workflow_dispatch", "branch", "work/x", "", true),
        ("workflow_dispatch", "branch", "Feature", "", false),
        (
            "pull_request",
            "branch",
            "7/merge",
            "milestone/findings-channel/main",
            true,
        ),
        (
            "pull_request",
            "branch",
            "7/merge",
            "release-plz-2026-10-01T10-41-19Z",
            true,
        ),
        ("pull_request", "branch", "7/merge", "feat/x", false),
    ];
    let mut wrong = Vec::new();
    for (event, ref_type, ref_name, head_ref, passes) in cases {
        let out = run_step(body, event, ref_type, ref_name, head_ref);
        if out.status.success() != passes {
            wrong.push(format!(
                "{event} {ref_type} {ref_name:?} (head {head_ref:?}): expected {}, exit {:?}\n{}",
                if passes { "pass" } else { "refusal" },
                out.status.code(),
                String::from_utf8_lossy(&out.stderr)
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
