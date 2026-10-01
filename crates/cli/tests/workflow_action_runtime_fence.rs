//! **Every action a workflow uses runs on a Node.js runtime GitHub still supports**
//! (2026-10-01, the human's finding on run `36821202910`).
//!
//! GitHub deprecated the `node20` action runtime and forces such actions onto `node24`
//! with a warning on every job that uses one — `actions/checkout@v4` printed it on the
//! `check`, `release-pr` and `release` jobs. Nothing local reddens on that: the gate never
//! runs a workflow, and a pin's runtime is a fact about the action's `action.yml` at that
//! ref, upstream. So the fact is read once, by hand, and pinned here: every `uses:` under
//! `.github/workflows/` must name an action in [`VETTED`] at exactly its vetted ref. A new
//! action, or a pin moved to another ref, reddens until someone reads that ref's
//! `action.yml` and records its `runs.using` — never `node20` or older.
//!
//! How a row is vetted: `gh api 'repos/<owner>/<repo>/contents/action.yml?ref=<ref>'
//! --jq .content`, base64-decoded, `runs.using`. A floating major (`v5`) is re-read at
//! its current tag. A `composite` action is vetted through the actions its steps use.

use crate::manifest_freeze_fence::repo_root;
use serde_yaml_ng::Value;
use std::collections::BTreeSet;
use std::fs;

const WORKFLOWS_DIR: &str = ".github/workflows";

/// One vetted pin: the action, the ref every workflow must use, and that ref's
/// `runs.using` as read on the date in the module docs.
struct Vetted {
    action: &'static str,
    pinned: &'static str,
    runs_using: &'static str,
}

/// Runtimes GitHub no longer runs as declared.
const RETIRED_RUNTIMES: [&str; 3] = ["node12", "node16", "node20"];

/// Every action the workflows may use. `checkout` and `create-github-app-token` are at
/// the lowest major whose `action.yml` declares `node24` (`v4` and `v2` declared
/// `node20`). `create-github-app-token` from `v3.1.0` deprecates its `app-id` input for
/// `client-id`; both feed one variable, so the release workflow passes the same secret
/// as `client-id`. `rust-cache`'s `v2` tag already declares `node24`. `release-plz/action`
/// is `composite`, and the three actions its `v0.5` steps use
/// (`taiki-e/install-action`, `cargo-bins/cargo-binstall`, `release-plz/git-config`) are
/// `composite` too, so no Node runtime is involved.
const VETTED: [Vetted; 4] = [
    Vetted {
        action: "actions/checkout",
        pinned: "v5",
        runs_using: "node24",
    },
    Vetted {
        action: "actions/create-github-app-token",
        pinned: "v3",
        runs_using: "node24",
    },
    Vetted {
        action: "Swatinem/rust-cache",
        pinned: "v2",
        runs_using: "node24",
    },
    Vetted {
        action: "release-plz/action",
        pinned: "v0.5",
        runs_using: "composite",
    },
];

/// Every `uses:` in every workflow file, as `(file, job, uses)`. A job-level `uses:`
/// (a reusable workflow) counts as well as a step's.
fn every_uses() -> Vec<(String, String, String)> {
    let dir = repo_root().join(WORKFLOWS_DIR);
    let mut files: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{WORKFLOWS_DIR} is readable: {e}"))
        .map(|entry| entry.expect("a workflow directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
        })
        .collect();
    files.sort();
    assert!(!files.is_empty(), "{WORKFLOWS_DIR} holds no workflow file");

    let mut found = Vec::new();
    for path in files {
        let name = path
            .file_name()
            .expect("a workflow file name")
            .to_string_lossy()
            .into_owned();
        let body = fs::read_to_string(&path).expect("the workflow file is readable");
        let workflow: Value =
            serde_yaml_ng::from_str(&body).unwrap_or_else(|e| panic!("{name} is YAML: {e}"));
        let jobs = workflow["jobs"]
            .as_mapping()
            .unwrap_or_else(|| panic!("{name} has a `jobs` mapping"));
        for (id, job) in jobs {
            let id = id.as_str().expect("a job id is a string").to_owned();
            if let Some(uses) = job["uses"].as_str() {
                found.push((name.clone(), id.clone(), uses.to_owned()));
            }
            for step in job["steps"].as_sequence().into_iter().flatten() {
                if let Some(uses) = step["uses"].as_str() {
                    found.push((name.clone(), id.clone(), uses.to_owned()));
                }
            }
        }
    }
    found
}

#[test]
fn every_vetted_runtime_is_a_supported_one() {
    for row in &VETTED {
        assert!(
            !RETIRED_RUNTIMES.contains(&row.runs_using),
            "`{}@{}` is vetted at `runs.using: {}`, a retired runtime",
            row.action,
            row.pinned,
            row.runs_using,
        );
    }
}

#[test]
fn every_workflow_action_is_pinned_at_its_vetted_ref() {
    let mut wrong = Vec::new();
    for (file, job, uses) in every_uses() {
        let Some((action, reference)) = uses.split_once('@') else {
            wrong.push(format!("{file} `{job}`: `{uses}` carries no `@<ref>`"));
            continue;
        };
        match VETTED.iter().find(|row| row.action == action) {
            None => wrong.push(format!(
                "{file} `{job}`: `{uses}` is not vetted — read its `action.yml` at that ref \
                 and add a row whose `runs.using` is not node20 or older"
            )),
            Some(row) if row.pinned != reference => wrong.push(format!(
                "{file} `{job}`: `{uses}` is not the vetted `{action}@{}` ({}) — re-read \
                 `action.yml` at `{reference}` and move the row with the pin",
                row.pinned, row.runs_using,
            )),
            Some(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_vetted_row_is_used_by_a_workflow() {
    let used: BTreeSet<String> = every_uses()
        .into_iter()
        .filter_map(|(_, _, uses)| uses.split_once('@').map(|(a, _)| a.to_owned()))
        .collect();
    let stale: Vec<_> = VETTED
        .iter()
        .filter(|row| !used.contains(row.action))
        .map(|row| row.action)
        .collect();
    assert!(
        stale.is_empty(),
        "vetted rows no workflow uses (drop them): {stale:?}"
    );
}
