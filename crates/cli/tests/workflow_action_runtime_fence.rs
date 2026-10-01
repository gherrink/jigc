//! **Every action a workflow uses is pinned to a vetted commit, and that commit runs on a
//! Node.js runtime GitHub still supports** (2026-10-01: the human's finding on run
//! `36821202910`, then the M54 completion audit's floating-tag finding).
//!
//! Two facts about a pin, both upstream, so nothing local reddens when either changes:
//!
//! - **Its code.** A tag is a movable name: a moved or compromised `v5` runs new code under
//!   the same line, and `release.yml`'s jobs hold the crates.io OIDC token (`release`) and
//!   the GitHub App's private key (`release-pr`). So every `uses:` under
//!   `.github/workflows/` names its action at a full 40-hex commit SHA, with the release
//!   tag that SHA was read from as a trailing comment — `uses: actions/checkout@<sha> #
//!   v5.1.0`. `ci.yml` holds no publish rights; it is pinned the same way so there is one
//!   convention to fence.
//! - **Its runtime.** GitHub deprecated the `node20` action runtime and forces such actions
//!   onto `node24` with a warning on every job that uses one — `actions/checkout@v4`
//!   printed it on the `check`, `release-pr` and `release` jobs.
//!
//! So both are read once, by hand, and pinned here: every `uses:` must name an action in
//! [`VETTED`] at exactly its vetted SHA, commented with exactly its vetted tag. A new
//! action, or a pin moved to another commit, reddens until someone vets that commit.
//!
//! How a row is vetted (implementation/release.md → *Action pins*):
//! 1. `gh api repos/<owner>/<repo>/git/ref/tags/<tag>` — the latest release tag within
//!    the major in use. An `object.type` of `tag` is an annotated tag: dereference it with
//!    `gh api repos/<owner>/<repo>/git/tags/<sha>` until the object is a `commit`.
//! 2. `gh api 'repos/<owner>/<repo>/contents/action.yml?ref=<commit>' --jq .content`,
//!    base64-decoded, `runs.using` — never `node20` or older.
//! 3. A `composite` action is vetted through the actions its steps use, read at the same
//!    commit.
//!
//! **What a pin does not reach** (release.md → Publishing → The release-plz binary): a
//! pinned SHA fixes an action's *code*, never what that code downloads at run time. So no
//! vetted action downloads a binary into a credentialed job: release-plz itself is
//! installed by `dev/install-release-plz` against a recorded SHA-256, and
//! `release_pipeline_fence.rs` arm (o) holds that script, its digest and the absence of
//! `release-plz/action`. *Corrected 2026-10-01:* this paragraph said the composite
//! `release-plz/action` fetched `cargo-binstall` at 1.23.0. It did not: its
//! `BINSTALL_VERSION: 1.23.0` is overridden by the inner `cargo-bins/cargo-binstall`
//! action's empty `version` input, so runs `36821202910` and `36843531826` fetched
//! `releases/latest`.

use crate::manifest_freeze_fence::repo_root;
use serde_yaml_ng::Value;
use std::collections::BTreeSet;
use std::fs;

const WORKFLOWS_DIR: &str = ".github/workflows";

/// One vetted pin: the action, the commit every workflow must use, the release tag that
/// commit was resolved from (the trailing comment on every `uses:` line), and that
/// commit's `runs.using` as read on the date in the module docs.
pub struct Vetted {
    pub action: &'static str,
    pub sha: &'static str,
    pub tag: &'static str,
    pub runs_using: &'static str,
}

/// Runtimes GitHub no longer runs as declared.
const RETIRED_RUNTIMES: [&str; 3] = ["node12", "node16", "node20"];

/// Every action the workflows may use, read 2026-10-01. Each is the latest release within
/// the major the workflows used before the SHA pin, and each major's floating tag pointed
/// at the same commit that day. `checkout` and `create-github-app-token` are at the lowest
/// major whose `action.yml` declares `node24` (`v4` and `v2` declared `node20`).
/// `create-github-app-token` from `v3.1.0` deprecates its `app-id` input for `client-id`;
/// both feed one variable, so the release workflow passes the same secret as `client-id`.
/// `release-plz/git-config` is `composite`: one `bash` step that asks `gh api graphql`
/// for the token's viewer and writes `git config --global user.name`/`user.email`. It
/// uses no other action and downloads nothing; `gh` and `jq` are the runner's own. It is
/// the step `release-plz/action` `v0.5.139` ran at this same commit, and the latest
/// release, `v0.1.2` (the floating `v0.1` points at it too).
pub const VETTED: [Vetted; 4] = [
    Vetted {
        action: "actions/checkout",
        sha: "fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09",
        tag: "v5.1.0",
        runs_using: "node24",
    },
    Vetted {
        action: "actions/create-github-app-token",
        sha: "bcd2ba49218906704ab6c1aa796996da409d3eb1",
        tag: "v3.2.0",
        runs_using: "node24",
    },
    Vetted {
        action: "Swatinem/rust-cache",
        sha: "6323deb102c322ba6fcbdcafc7e3dddab59af2b6",
        tag: "v2.9.2",
        runs_using: "node24",
    },
    Vetted {
        action: "release-plz/git-config",
        sha: "59144859caf016f8b817a2ac9b051578729173c4",
        tag: "v0.1.2",
        runs_using: "composite",
    },
];

/// The exact `uses:` value every workflow must carry for `action`: `<action>@<sha>`. The
/// one home other workflow fences read a pin from.
pub fn vetted_uses(action: &str) -> String {
    let row = VETTED
        .iter()
        .find(|row| row.action == action)
        .unwrap_or_else(|| panic!("`{action}` has no vetted row"));
    format!("{}@{}", row.action, row.sha)
}

fn is_commit_sha(reference: &str) -> bool {
    reference.len() == 40
        && reference
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// A full release tag, `v<major>.<minor>.<patch>` — never a floating `v5` or `v0.5`.
fn is_full_release_tag(tag: &str) -> bool {
    let Some(version) = tag.strip_prefix('v') else {
        return false;
    };
    let parts: Vec<_> = version.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

/// The workflow files, sorted, as `(file name, body)`.
fn workflow_files() -> Vec<(String, String)> {
    let dir = repo_root().join(WORKFLOWS_DIR);
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{WORKFLOWS_DIR} is readable: {e}"))
        .map(|entry| entry.expect("a workflow directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
        })
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "{WORKFLOWS_DIR} holds no workflow file");
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .expect("a workflow file name")
                .to_string_lossy()
                .into_owned();
            let body = fs::read_to_string(&path).expect("the workflow file is readable");
            (name, body)
        })
        .collect()
}

/// Every `uses:` in every workflow file, as `(file, job, uses)`, read through the YAML
/// parser. A job-level `uses:` (a reusable workflow) counts as well as a step's.
fn every_uses() -> Vec<(String, String, String)> {
    let mut found = Vec::new();
    for (name, body) in workflow_files() {
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

/// Every `uses:` line in every workflow file, read as text because the parser drops the
/// trailing tag comment, as `(file, line number, value, comment)`.
fn every_uses_line() -> Vec<(String, usize, String, Option<String>)> {
    let mut found = Vec::new();
    for (name, body) in workflow_files() {
        for (index, line) in body.lines().enumerate() {
            let trimmed = line.trim_start();
            let trimmed = trimmed.strip_prefix("- ").unwrap_or(trimmed);
            let Some(rest) = trimmed.strip_prefix("uses:") else {
                continue;
            };
            let (value, comment) = match rest.split_once(" #") {
                Some((value, comment)) => (value, Some(comment.trim().to_owned())),
                None => (rest, None),
            };
            found.push((name.clone(), index + 1, value.trim().to_owned(), comment));
        }
    }
    found
}

#[test]
fn every_vetted_runtime_is_a_supported_one() {
    for row in &VETTED {
        assert!(
            !RETIRED_RUNTIMES.contains(&row.runs_using),
            "`{}@{}` ({}) is vetted at `runs.using: {}`, a retired runtime",
            row.action,
            row.sha,
            row.tag,
            row.runs_using,
        );
    }
}

#[test]
fn every_vetted_row_is_a_commit_sha_read_from_a_full_release_tag() {
    let mut wrong = Vec::new();
    for row in &VETTED {
        if !is_commit_sha(row.sha) {
            wrong.push(format!(
                "`{}`: `{}` is not a full 40-hex lowercase commit SHA",
                row.action, row.sha
            ));
        }
        if !is_full_release_tag(row.tag) {
            wrong.push(format!(
                "`{}`: `{}` is not a full `v<major>.<minor>.<patch>` release tag",
                row.action, row.tag
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_workflow_action_is_pinned_at_its_vetted_sha() {
    let mut wrong = Vec::new();
    for (file, job, uses) in every_uses() {
        let Some((action, reference)) = uses.split_once('@') else {
            wrong.push(format!("{file} `{job}`: `{uses}` carries no `@<ref>`"));
            continue;
        };
        match VETTED.iter().find(|row| row.action == action) {
            None => wrong.push(format!(
                "{file} `{job}`: `{uses}` is not vetted — resolve the latest release tag's \
                 commit, read its `action.yml` there, and add a row whose `runs.using` is \
                 not node20 or older"
            )),
            Some(row) if row.sha != reference => wrong.push(format!(
                "{file} `{job}`: `{uses}` is not the vetted `{action}@{}` ({}, {}) — a pin \
                 is a full commit SHA, never a tag; re-vet the commit at `{reference}` and \
                 move the row with the pin",
                row.sha, row.tag, row.runs_using,
            )),
            Some(_) => {}
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn every_uses_line_carries_its_vetted_tag_as_a_trailing_comment() {
    let lines = every_uses_line();
    // The text scan must see every `uses:` the parser sees, or a form it does not
    // recognise (quoted, flow-style) would slip past the comment check.
    assert_eq!(
        lines.len(),
        every_uses().len(),
        "the line scan and the YAML parse disagree on how many `uses:` there are"
    );
    let mut wrong = Vec::new();
    for (file, line, value, comment) in lines {
        let Some(row) = VETTED
            .iter()
            .find(|row| value.split_once('@').is_some_and(|(a, _)| a == row.action))
        else {
            // Unvetted actions are the pin arm's finding; nothing to compare a comment to.
            continue;
        };
        if comment.as_deref() != Some(row.tag) {
            wrong.push(format!(
                "{file}:{line}: `uses: {value}` must end `# {}` (the tag its SHA was read \
                 from), found {comment:?}",
                row.tag
            ));
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
