//! **The release pipeline, held to S9** (M54 Increment 9 · gate-record row 18).
//!
//! S9 settles how a version is proposed and published: one workspace release PR, a
//! publish only after the human merges it, docs-only commits never bumping anything,
//! `<package>-v<version>` tags, prerelease GitHub releases for an `-rc` version. Every one
//! of those is a knob in `release-plz.toml`, and a knob that drifts changes what the
//! pipeline does without anything reddening — release-plz reads the file only on `main`,
//! after the merge. This suite is that fence. The pinned table and one line of why per
//! knob live in [release.md](../../../implementation/release.md) → *release-plz knobs*;
//! each arm below names the property, never restates the why.
//!
//! The config arms: **(a)** `pr_name` uses `{{ package }}`/`{{ version }}` only inside the
//! `{% if package and version %}` guard, because release-plz fails the run on an
//! unconditional use when more than one package releases; **(b)** every publishable
//! workspace package — as `cargo metadata` reports it, never a list written here — has
//! exactly one `[[package]]` entry whose `changelog_path` is its own manifest directory's
//! `CHANGELOG.md`, and none names the root one, which jigc's `changelog` doctype owns;
//! **(c)** `release_always = false`; **(d)** the pinned `release_commits` filter; **(e)**
//! the `git_tag_name` template; **(f)** `git_release_type = "auto"`; **(g)**
//! `publish_timeout` fits inside the crates.io trusted-publishing token's life with a
//! margin for the verify builds; **(h)** `semver_check` and `dependencies_update` both
//! `false`.
//!
//! The workflow arms, over `.github/workflows/release.yml` (release.md → *Publishing*):
//! **(i)** no `CARGO_REGISTRY_TOKEN` anywhere under `.github/workflows/`, read as text, so
//! Trusted Publishing is the only way a crate reaches crates.io; **(j)** the one trigger
//! is a push to `main`; **(k)** exactly one job is bound to environment `release`, it
//! `needs` the check job and runs only on its flag, and it is the only holder of
//! `id-token: write`; **(l)** the check job has no environment, runs
//! `dev/unpublished-versions` and outputs its flag; **(m)** the release-PR job's checkout
//! and `release-plz` both run on the GitHub App's token; **(n)** nothing keys on a commit
//! subject; **(o)** every `release-plz/action` step pins release-plz `0.3.169`; **(p)** the
//! dry run and the engine overlay apply off `gherrink/jigc` only.
//!
//! The agents' arm, over `.claude/` (release.md → *What agents may not do*): **(q)** this
//! repository's `.claude/settings.json` denies every pinned form of merging a PR, pushing
//! a tag, approving a deployment and yanking a crate, and every agent definition under
//! `.claude/agents/` carries the one *Never* paragraph. The settings are the enforced
//! layer; a definition can only instruct, because a scoped Bash deny in its frontmatter
//! removes Bash from the agent wholesale.

use crate::manifest_freeze_fence::repo_root;
use crate::support::root_walk;
use crate::support::run_then_parse::stdout_json;
use serde_yaml_ng::Value as Yaml;
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use std::time::Duration;
use toml::{Table, Value};

/// The release-plz config, repository-relative.
const RELEASE_PLZ_TOML: &str = "release-plz.toml";

/// The guard that makes `pr_name`'s package variables conditional (release-plz
/// `config.md` → `pr_name`: `{{ package }}` and `{{ version }}` are populated only when a
/// single package is released).
const PR_NAME_GUARD: &str = "{% if package and version %}";
const PR_NAME_GUARD_END: &str = "{% endif %}";

/// The commits that may produce a release. The `(scope)` and `!` group, followed by the
/// `:`, is what stops `fixup!`, `feature…` and `refactoring` from matching.
const RELEASE_COMMITS: &str = r"^(feat|fix|perf|refactor|revert)(\([^)]*\))?!?:";

/// `jigc-v1.0.0-rc.22`, `jigc-engine-v0.1.0-rc.1` (release.md → Publishing).
const GIT_TAG_NAME: &str = "{{ package }}-v{{ version }}";

/// The life of the token crates.io mints for a trusted-publishing exchange:
/// `expires_at: chrono::Utc::now() + chrono::Duration::minutes(30)` in
/// `src/controllers/trustpub/tokens/exchange/mod.rs:233` of `rust-lang/crates.io` at
/// `18bce5be023b1cc6b7a2eb75fbebfd36de6b25f7`.
const TRUSTED_PUBLISHING_TOKEN_LIFE: Duration = Duration::from_secs(30 * 60);

/// What the one token must also cover besides one index wait. release-plz 0.3.169 mints
/// it before the first `cargo publish` and reuses it for every package, and
/// `publish_timeout` bounds only the wait for the index after each upload. So between the
/// mint and `jigc`'s upload lie the engine's verify build and upload, the engine's index
/// wait (≤ `publish_timeout`), and `jigc`'s verify build. The two verify builds took
/// 14.66 s and 35.90 s on the CI runner (`ci-runtime.md` → the `publish-dry-run` job);
/// five minutes is that with a margin of six.
const VERIFY_BUILD_MARGIN: Duration = Duration::from_secs(5 * 60);

fn config() -> Table {
    let path = repo_root().join(RELEASE_PLZ_TOML);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("`{RELEASE_PLZ_TOML}` must be readable: {e}"));
    body.parse::<Table>()
        .unwrap_or_else(|e| panic!("`{RELEASE_PLZ_TOML}` must be valid TOML: {e}"))
}

fn workspace(config: &Table) -> &Table {
    config
        .get("workspace")
        .and_then(Value::as_table)
        .unwrap_or_else(|| panic!("`{RELEASE_PLZ_TOML}` has no `[workspace]` table"))
}

fn workspace_value<'a>(config: &'a Table, key: &str) -> &'a Value {
    workspace(config)
        .get(key)
        .unwrap_or_else(|| panic!("`{RELEASE_PLZ_TOML}` → `[workspace]` sets no `{key}`"))
}

fn workspace_str<'a>(config: &'a Table, key: &str) -> &'a str {
    workspace_value(config, key)
        .as_str()
        .unwrap_or_else(|| panic!("`[workspace]` → `{key}` is not a string"))
}

// ---------------------------------------------------------------------------
// (a) the conditional `pr_name`
// ---------------------------------------------------------------------------

/// The template expressions (`{{ … }}` and `{% … %}`) in `text` that mention `package`
/// or `version`.
fn package_expressions(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find('{') {
        let after = &rest[start..];
        let close = if after.starts_with("{{") {
            "}}"
        } else if after.starts_with("{%") {
            "%}"
        } else {
            rest = &after[1..];
            continue;
        };
        let end = after
            .find(close)
            .map_or(after.len(), |end| end + close.len());
        let expression = &after[..end];
        if expression.contains("package") || expression.contains("version") {
            found.push(expression.to_string());
        }
        rest = &after[end..];
    }
    found
}

/// `Ok` when `template` uses the package variables only inside the guard; otherwise the
/// reason it does not.
fn pr_name_is_conditional(template: &str) -> Result<(), String> {
    let Some(open) = template.find(PR_NAME_GUARD) else {
        return Err(format!("it carries no `{PR_NAME_GUARD}` guard"));
    };
    let inside = &template[open + PR_NAME_GUARD.len()..];
    let Some(close) = inside.find(PR_NAME_GUARD_END) else {
        return Err(format!(
            "its guard is never closed by `{PR_NAME_GUARD_END}`"
        ));
    };
    let outside = format!(
        "{}{}",
        &template[..open],
        &inside[close + PR_NAME_GUARD_END.len()..]
    );
    let stray = package_expressions(&outside);
    if stray.is_empty() {
        Ok(())
    } else {
        Err(format!("it uses {stray:?} outside the guard"))
    }
}

#[test]
fn a_pr_name_uses_the_package_variables_only_inside_its_guard() {
    let config = config();
    let pr_name = workspace_str(&config, "pr_name");
    if let Err(why) = pr_name_is_conditional(pr_name) {
        panic!(
            "`pr_name = {pr_name:?}` is not conditional: {why}. release-plz populates \
             `{{{{ package }}}}` and `{{{{ version }}}}` only when a single package \
             releases, and fails the run otherwise (release.md → The release PR)."
        );
    }
}

/// The checker itself, over fixtures: the Proves' mutant and its near misses fail, the
/// guarded form passes. A template with no guard at all fails too — the arm pins the
/// guard's presence, not only the variables' absence.
#[test]
fn a_the_conditional_check_rejects_an_unguarded_package_variable() {
    for mutant in [
        "chore: release {{ package }}",
        "chore: release {{ version }}",
        "chore: release {% if package and version %}{{ package }}",
        "chore: {{ package }}{% if package and version %} v{{ version }}{% endif %}",
    ] {
        assert!(
            pr_name_is_conditional(mutant).is_err(),
            "the check admitted {mutant:?}"
        );
    }
    let guarded =
        "chore: release{% if package and version %} {{ package }} v{{ version }}{% endif %}";
    assert_eq!(pr_name_is_conditional(guarded), Ok(()), "for {guarded:?}");
}

// ---------------------------------------------------------------------------
// (b) one changelog per publishable package, never the root one
// ---------------------------------------------------------------------------

/// Every publishable workspace package (`publish` is not `[]`), by name, with its
/// manifest directory relative to the workspace root.
fn publishable_packages() -> BTreeMap<String, String> {
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
    let root = metadata["workspace_root"]
        .as_str()
        .expect("metadata workspace_root");
    let mut packages = BTreeMap::new();
    for package in metadata["packages"].as_array().expect("metadata packages") {
        let unpublishable = package["publish"]
            .as_array()
            .is_some_and(|registries| registries.is_empty());
        if unpublishable {
            continue;
        }
        let name = package["name"].as_str().expect("package name");
        let manifest = package["manifest_path"].as_str().expect("manifest path");
        let dir = manifest
            .strip_prefix(root)
            .and_then(|rel| rel.strip_suffix("Cargo.toml"))
            .expect("a member's manifest lies under the workspace root")
            .trim_matches('/');
        packages.insert(name.to_string(), dir.to_string());
    }
    packages
}

#[test]
fn b_every_publishable_package_has_its_own_changelog_and_none_is_the_root_one() {
    let publishable = publishable_packages();
    assert!(
        !publishable.is_empty(),
        "cargo metadata reports no publishable package"
    );
    let config = config();
    let entries = config
        .get("package")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("`{RELEASE_PLZ_TOML}` has no `[[package]]` entries"));

    let mut changelogs: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for entry in entries {
        let name = entry
            .get("name")
            .and_then(Value::as_str)
            .expect("every `[[package]]` entry names its package");
        let path = entry
            .get("changelog_path")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("`[[package]]` `{name}` sets no `changelog_path`"));
        assert_ne!(
            path.trim_start_matches("./"),
            "CHANGELOG.md",
            "`[[package]]` `{name}` writes the root `CHANGELOG.md`, which jigc's \
             `changelog` doctype owns (release.md → The release PR → Changelogs)"
        );
        changelogs
            .entry(name.to_string())
            .or_default()
            .push(path.to_string());
    }

    let expected: BTreeMap<String, Vec<String>> = publishable
        .iter()
        .map(|(name, dir)| (name.clone(), vec![format!("{dir}/CHANGELOG.md")]))
        .collect();
    assert_eq!(
        changelogs, expected,
        "each publishable package (cargo metadata) needs exactly one `[[package]]` entry \
         whose `changelog_path` is its own manifest directory's `CHANGELOG.md`, and no \
         entry may name anything else"
    );
}

// ---------------------------------------------------------------------------
// (c)–(h) the workspace knobs
// ---------------------------------------------------------------------------

#[test]
fn c_release_always_is_false() {
    assert_eq!(
        workspace_value(&config(), "release_always").as_bool(),
        Some(false),
        "`release_always` must be `false`: merging the release PR is the go, and `true` \
         would publish from any version-bumping push (S9, gate-record O4)"
    );
}

#[test]
fn d_release_commits_is_the_pinned_filter() {
    assert_eq!(
        workspace_str(&config(), "release_commits"),
        RELEASE_COMMITS,
        "docs-only commits must never bump anything (release.md → Versioning)"
    );
}

#[test]
fn e_git_tag_name_is_package_dash_v_version() {
    assert_eq!(workspace_str(&config(), "git_tag_name"), GIT_TAG_NAME);
}

#[test]
fn f_git_release_type_marks_an_rc_a_prerelease() {
    assert_eq!(
        workspace_str(&config(), "git_release_type"),
        "auto",
        "`auto` marks a GitHub release prerelease when its version is one"
    );
}

/// The subset of the duration grammar this config uses: one `<n><unit>`, unit `s`, `m` or
/// `h`. Anything else is refused, so a form this reader does not know cannot pass by
/// being misread.
fn parse_duration(text: &str) -> Duration {
    let (number, unit) = text.split_at(text.len().saturating_sub(1));
    let n: u64 = number
        .parse()
        .unwrap_or_else(|_| panic!("`publish_timeout = {text:?}` is not `<n>s|m|h`"));
    match unit {
        "s" => Duration::from_secs(n),
        "m" => Duration::from_secs(n * 60),
        "h" => Duration::from_secs(n * 3600),
        _ => panic!("`publish_timeout = {text:?}` is not `<n>s|m|h`"),
    }
}

#[test]
fn g_publish_timeout_fits_inside_the_trusted_publishing_token() {
    let timeout = parse_duration(workspace_str(&config(), "publish_timeout"));
    assert!(
        timeout + VERIFY_BUILD_MARGIN < TRUSTED_PUBLISHING_TOKEN_LIFE,
        "`publish_timeout` {timeout:?} plus the verify-build margin \
         {VERIFY_BUILD_MARGIN:?} must stay under the {TRUSTED_PUBLISHING_TOKEN_LIFE:?} life \
         of the one trusted-publishing token both uploads share"
    );
}

#[test]
fn h_no_semver_check_and_no_dependency_update() {
    let config = config();
    for knob in ["semver_check", "dependencies_update"] {
        assert_eq!(
            workspace_value(&config, knob).as_bool(),
            Some(false),
            "`{knob}` must be `false`"
        );
    }
}

// ---------------------------------------------------------------------------
// The workflow: `.github/workflows/release.yml`
// ---------------------------------------------------------------------------

/// The workflow directory arm (i) reads as text, and the release workflow in it.
const WORKFLOWS_DIR: &str = ".github/workflows";
const RELEASE_YML: &str = ".github/workflows/release.yml";

/// The three jobs, by id.
const RELEASE_PR_JOB: &str = "release-pr";
const CHECK_JOB: &str = "check";
const RELEASE_JOB: &str = "release";

/// The GitHub environment the publish runs in, its reviewer the human.
const RELEASE_ENVIRONMENT: &str = "release";

/// The check job's output, as `dev/unpublished-versions` writes it to `$GITHUB_OUTPUT`.
const MISSING_FLAG: &str = "missing";
const UNPUBLISHED_VERSIONS: &str = "dev/unpublished-versions";

/// The release job's condition: the check job found a workspace version crates.io lacks.
const RELEASE_JOB_IF: &str = "needs.check.outputs.missing == 'true'";

/// The GitHub App's credentials, as repository secrets (release.md → The release PR).
const APP_ID_SECRET: &str = "${{ secrets.JIGC_RELEASE_APP_ID }}";
const APP_KEY_SECRET: &str = "${{ secrets.JIGC_RELEASE_APP_PRIVATE_KEY }}";

const APP_TOKEN_ACTION: &str = "actions/create-github-app-token@";
const CHECKOUT_ACTION: &str = "actions/checkout@";
const RELEASE_PLZ_ACTION: &str = "release-plz/action@v0.5";

/// The release-plz version every proof on the record ran (release.md → release-plz knobs).
const RELEASE_PLZ_VERSION: &str = "0.3.169";

/// Anything off the real repository is a rehearsal.
const OFF_THE_REAL_REPO: &str = "github.repository != 'gherrink/jigc'";

/// The `dry_run` input. `release-plz/action` adds `--dry-run` when the input is
/// **non-empty** (`if [[ -n "${{ inputs.dry_run }}" ]]` in its `action.yml`), so the bare
/// boolean `${{ github.repository != 'gherrink/jigc' }}` would pass the string `false` on
/// `gherrink/jigc` and dry-run the real publish too. This form is `'true'` off the real
/// repository and empty on it.
const DRY_RUN_INPUT: &str = "${{ github.repository != 'gherrink/jigc' && 'true' || '' }}";

/// The contexts that carry a pushed commit's message.
const COMMIT_MESSAGE_CONTEXTS: [&str; 2] = ["head_commit", "event.commits"];

fn release_workflow() -> Yaml {
    let path = repo_root().join(RELEASE_YML);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("`{RELEASE_YML}` must be readable: {e}"));
    serde_yaml_ng::from_str(&body)
        .unwrap_or_else(|e| panic!("`{RELEASE_YML}` must be valid YAML: {e}"))
}

fn workflow_jobs(workflow: &Yaml) -> Vec<(String, &Yaml)> {
    workflow["jobs"]
        .as_mapping()
        .unwrap_or_else(|| panic!("`{RELEASE_YML}` has no `jobs` mapping"))
        .iter()
        .map(|(id, job)| (id.as_str().expect("a job id").to_string(), job))
        .collect()
}

fn workflow_job<'a>(workflow: &'a Yaml, id: &str) -> &'a Yaml {
    let job = &workflow["jobs"][id];
    assert!(
        job.is_mapping(),
        "`{RELEASE_YML}` has no `{id}` job. Jobs present: {:?}",
        workflow_jobs(workflow)
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>(),
    );
    job
}

fn job_steps(job: &Yaml) -> &[Yaml] {
    job["steps"].as_sequence().map_or(&[], Vec::as_slice)
}

/// The steps of `job` whose `uses` starts with `prefix`.
fn steps_using<'a>(job: &'a Yaml, prefix: &str) -> Vec<&'a Yaml> {
    job_steps(job)
        .iter()
        .filter(|step| step["uses"].as_str().is_some_and(|u| u.starts_with(prefix)))
        .collect()
}

/// The one step of `job` whose `uses` starts with `prefix`.
fn step_using<'a>(job: &'a Yaml, job_id: &str, prefix: &str) -> &'a Yaml {
    let found = steps_using(job, prefix);
    assert_eq!(
        found.len(),
        1,
        "job `{job_id}` must have exactly one step using `{prefix}…`"
    );
    found[0]
}

/// An `if:` condition with its optional `${{ … }}` wrapper removed.
fn condition(value: &Yaml) -> Option<&str> {
    let text = value.as_str()?.trim();
    Some(
        text.strip_prefix("${{")
            .and_then(|t| t.strip_suffix("}}"))
            .map_or(text, str::trim),
    )
}

/// The environment a job is bound to: `environment: <name>` or `environment: {name: …}`.
fn environment(job: &Yaml) -> Option<&str> {
    let env = &job["environment"];
    env.as_str().or_else(|| env["name"].as_str())
}

/// `needs:` as a list, whether written as one id or a sequence.
fn needs(job: &Yaml) -> Vec<&str> {
    let needs = &job["needs"];
    needs.as_str().map_or_else(
        || {
            needs
                .as_sequence()
                .map_or(Vec::new(), |s| s.iter().filter_map(Yaml::as_str).collect())
        },
        |one| vec![one],
    )
}

// ---------------------------------------------------------------------------
// (i) no registry token, anywhere under .github/workflows/
// ---------------------------------------------------------------------------

#[test]
fn i_no_workflow_names_a_registry_token() {
    let dir = repo_root().join(WORKFLOWS_DIR);
    let mut read = Vec::new();
    for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("`{WORKFLOWS_DIR}`: {e}")) {
        let path = entry.expect("a workflow directory entry").path();
        if !path.is_file() {
            continue;
        }
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("`{}` must be readable: {e}", path.display()));
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .expect("a UTF-8 file name")
            .to_string();
        assert!(
            !body.contains("CARGO_REGISTRY_TOKEN"),
            "`{WORKFLOWS_DIR}/{name}` names `CARGO_REGISTRY_TOKEN`. The publish is Trusted \
             Publishing alone, and no registry token exists (release.md → Publishing)"
        );
        read.push(name);
    }
    assert!(
        read.iter().any(|name| RELEASE_YML.ends_with(name.as_str())),
        "the scan must cover `{RELEASE_YML}`; it read {read:?}"
    );
}

// ---------------------------------------------------------------------------
// (j) the trigger
// ---------------------------------------------------------------------------

#[test]
fn j_the_only_trigger_is_a_push_to_main() {
    let workflow = release_workflow();
    let trigger = workflow
        .get("on")
        .unwrap_or_else(|| panic!("`{RELEASE_YML}` has no `on:`"));
    let expected: Yaml =
        serde_yaml_ng::from_str("push:\n  branches: [main]\n").expect("the pinned trigger");
    assert_eq!(
        trigger, &expected,
        "`{RELEASE_YML}` must run on a push to `main` and on nothing else: the release job \
         runs after the release PR's merge, and the environment admits `main` only"
    );
}

// ---------------------------------------------------------------------------
// (k) the environment-bound release job
// ---------------------------------------------------------------------------

#[test]
fn k_one_environment_job_needs_the_check_and_alone_holds_id_token() {
    let workflow = release_workflow();
    let bound: Vec<String> = workflow_jobs(&workflow)
        .into_iter()
        .filter(|(_, job)| environment(job).is_some())
        .map(|(id, _)| id)
        .collect();
    assert_eq!(
        bound,
        [RELEASE_JOB],
        "exactly the `{RELEASE_JOB}` job is bound to an environment"
    );

    let release = workflow_job(&workflow, RELEASE_JOB);
    assert_eq!(environment(release), Some(RELEASE_ENVIRONMENT));
    assert_eq!(
        needs(release),
        [CHECK_JOB],
        "the `{RELEASE_JOB}` job must `needs: {CHECK_JOB}`, or it raises the approval prompt \
         on every push, whether or not anything is unpublished"
    );
    assert_eq!(
        condition(&release["if"]),
        Some(RELEASE_JOB_IF),
        "the `{RELEASE_JOB}` job runs only on the check job's flag"
    );
    let expected: Yaml =
        serde_yaml_ng::from_str("contents: write\npull-requests: read\nid-token: write\n")
            .expect("the pinned permissions");
    assert_eq!(
        release["permissions"], expected,
        "the `{RELEASE_JOB}` job's permissions: tags and releases, the release-PR lookup, \
         and the OIDC token Trusted Publishing exchanges"
    );

    assert!(
        workflow["permissions"]["id-token"].is_null(),
        "the workflow-level `permissions` must grant no `id-token`"
    );
    for (id, job) in workflow_jobs(&workflow) {
        if id != RELEASE_JOB {
            assert!(
                job["permissions"]["id-token"].is_null(),
                "job `{id}` grants `id-token`; only the environment-bound job may"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// (l) the check job
// ---------------------------------------------------------------------------

#[test]
fn l_the_check_job_runs_outside_the_environment_and_outputs_the_flag() {
    let workflow = release_workflow();
    let check = workflow_job(&workflow, CHECK_JOB);
    assert!(
        check["environment"].is_null(),
        "the `{CHECK_JOB}` job must run outside any environment, or deciding whether to \
         prompt would itself prompt"
    );
    let flagging: Vec<&Yaml> = job_steps(check)
        .iter()
        .filter(|step| {
            step["run"]
                .as_str()
                .is_some_and(|run| run.contains(UNPUBLISHED_VERSIONS))
        })
        .collect();
    assert_eq!(
        flagging.len(),
        1,
        "the `{CHECK_JOB}` job must run `{UNPUBLISHED_VERSIONS}` in exactly one step"
    );
    let step_id = flagging[0]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the step running `{UNPUBLISHED_VERSIONS}` needs an `id`"));
    assert_eq!(
        check["outputs"][MISSING_FLAG].as_str(),
        Some(format!("${{{{ steps.{step_id}.outputs.{MISSING_FLAG} }}}}").as_str()),
        "the `{CHECK_JOB}` job must output `{MISSING_FLAG}` from that step"
    );
}

// ---------------------------------------------------------------------------
// (m) the release-PR job on the App's token
// ---------------------------------------------------------------------------

#[test]
fn m_the_release_pr_job_checks_out_and_runs_on_the_app_token() {
    let workflow = release_workflow();
    let job = workflow_job(&workflow, RELEASE_PR_JOB);
    let mint = step_using(job, RELEASE_PR_JOB, APP_TOKEN_ACTION);
    // `client-id`, never the `app-id` that `v3.1.0` deprecated: the action reads
    // `client-id || app-id` into one variable, so the App ID secret serves either.
    assert_eq!(mint["with"]["client-id"].as_str(), Some(APP_ID_SECRET));
    assert!(
        mint["with"]["app-id"].is_null(),
        "the `{APP_TOKEN_ACTION}…` step passes the deprecated `app-id` input"
    );
    assert_eq!(mint["with"]["private-key"].as_str(), Some(APP_KEY_SECRET));
    let mint_id = mint["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the `{APP_TOKEN_ACTION}…` step needs an `id`"));
    let token = format!("${{{{ steps.{mint_id}.outputs.token }}}}");

    let checkout = step_using(job, RELEASE_PR_JOB, CHECKOUT_ACTION);
    assert_eq!(
        checkout["with"]["token"].as_str(),
        Some(token.as_str()),
        "the `{RELEASE_PR_JOB}` checkout must use the App's token"
    );
    let plz = step_using(job, RELEASE_PR_JOB, RELEASE_PLZ_ACTION);
    assert_eq!(plz["with"]["command"].as_str(), Some("release-pr"));
    assert_eq!(
        plz["env"]["GITHUB_TOKEN"].as_str(),
        Some(token.as_str()),
        "`release-plz release-pr` must open the PR with the App's token: a PR the \
         workflow's own token opens triggers no CI run"
    );
}

// ---------------------------------------------------------------------------
// (n) nothing keys on a commit subject
// ---------------------------------------------------------------------------

/// Every `if:` value, every `run:` body and every `${{ … }}` expression in `value`, with
/// the path it sits at.
fn expressions(value: &Yaml, at: &str, out: &mut Vec<(String, String)>) {
    match value {
        Yaml::Mapping(map) => {
            for (key, child) in map {
                let key = key.as_str().unwrap_or("?");
                let here = format!("{at}.{key}");
                if let (Some(text), "if" | "run") = (child.as_str(), key) {
                    out.push((here.clone(), text.to_string()));
                }
                expressions(child, &here, out);
            }
        }
        Yaml::Sequence(items) => {
            for (i, child) in items.iter().enumerate() {
                expressions(child, &format!("{at}[{i}]"), out);
            }
        }
        Yaml::String(text) => {
            let mut rest = text.as_str();
            while let Some(start) = rest.find("${{") {
                let after = &rest[start..];
                let end = after.find("}}").map_or(after.len(), |end| end + 2);
                out.push((at.to_string(), after[..end].to_string()));
                rest = &after[end..];
            }
        }
        _ => {}
    }
}

#[test]
fn n_no_condition_or_command_reads_a_commit_message() {
    let workflow = release_workflow();
    let mut found = Vec::new();
    expressions(&workflow, "", &mut found);
    let keyed: Vec<&(String, String)> = found
        .iter()
        .filter(|(_, text)| COMMIT_MESSAGE_CONTEXTS.iter().any(|c| text.contains(c)))
        .collect();
    assert!(
        keyed.is_empty(),
        "`{RELEASE_YML}` reads a commit message at {keyed:?}. The merge strategy is \
         unchosen and a default merge commit reads `Merge pull request #N`, so no job may \
         key on a subject (release.md → Publishing)"
    );
}

// ---------------------------------------------------------------------------
// (o) the release-plz version
// ---------------------------------------------------------------------------

#[test]
fn o_every_release_plz_step_pins_the_proven_version() {
    let workflow = release_workflow();
    let mut pinned = Vec::new();
    for (id, job) in workflow_jobs(&workflow) {
        for step in steps_using(job, "release-plz/") {
            assert_eq!(
                step["uses"].as_str(),
                Some(RELEASE_PLZ_ACTION),
                "job `{id}`"
            );
            assert_eq!(
                step["with"]["version"].as_str(),
                Some(RELEASE_PLZ_VERSION),
                "job `{id}`'s `{RELEASE_PLZ_ACTION}` step must pin `version: \
                 \"{RELEASE_PLZ_VERSION}\"`, the release-plz every proof on the record ran"
            );
            pinned.push(id.clone());
        }
    }
    assert_eq!(
        pinned,
        [RELEASE_PR_JOB, RELEASE_JOB],
        "one `{RELEASE_PLZ_ACTION}` step in each of the two release-plz jobs"
    );
}

// ---------------------------------------------------------------------------
// (p) the rehearsal: dry run and engine overlay off the real repository only
// ---------------------------------------------------------------------------

#[test]
fn p_the_dry_run_and_the_overlay_apply_off_the_real_repository_only() {
    let workflow = release_workflow();
    let release = workflow_job(&workflow, RELEASE_JOB);
    let steps = job_steps(release);
    let plz_at = steps
        .iter()
        .position(|s| s["uses"].as_str() == Some(RELEASE_PLZ_ACTION))
        .unwrap_or_else(|| panic!("the `{RELEASE_JOB}` job runs no `{RELEASE_PLZ_ACTION}`"));
    let plz = &steps[plz_at];
    assert_eq!(plz["with"]["command"].as_str(), Some("release"));
    assert_eq!(
        plz["with"]["dry_run"].as_str(),
        Some(DRY_RUN_INPUT),
        "the release must be dry off `gherrink/jigc` and real on it; the action tests the \
         input for being non-empty, never for `true`"
    );

    let overlays: Vec<usize> = steps
        .iter()
        .enumerate()
        .filter(|(_, s)| {
            s["run"]
                .as_str()
                .is_some_and(|run| run.contains("[patch.crates-io]") && run.contains("jigc-engine"))
        })
        .map(|(at, _)| at)
        .collect();
    assert_eq!(
        overlays.len(),
        1,
        "the `{RELEASE_JOB}` job needs exactly one step writing the `jigc-engine` overlay"
    );
    let overlay_at = overlays[0];
    assert!(
        overlay_at < plz_at,
        "the overlay must be written before release-plz runs"
    );
    assert_eq!(
        condition(&steps[overlay_at]["if"]),
        Some(OFF_THE_REAL_REPO),
        "the overlay verifies `jigc` against the local engine, so it may apply only to the \
         rehearsal's dry run"
    );
}

// ---------------------------------------------------------------------------
// (q) the agents are denied the human's acts
// ---------------------------------------------------------------------------

/// The agents' permissions and their definitions, repository-relative.
const AGENT_SETTINGS: &str = ".claude/settings.json";
const AGENT_DEFINITIONS: &str = ".claude/agents";

/// Every deny entry release.md → *What agents may not do* pins, with the act it stops.
/// Each was driven with a headless `claude -p --permission-mode bypassPermissions`
/// (Claude Code 2.1.284), directly and through an Agent-tool subagent, and each reported
/// a `permission_denials` record; the controls that table names ran under the same list.
const AGENT_DENY: [(&str, &str); 11] = [
    ("Bash(gh *pr merge*)", "merge a PR"),
    ("Bash(gh api *pulls/*/merge*)", "merge a PR (REST)"),
    ("Bash(gh api *mergePullRequest*)", "merge a PR (GraphQL)"),
    (
        "Bash(gh api *enablePullRequestAutoMerge*)",
        "merge a PR (GraphQL auto-merge)",
    ),
    ("Bash(git *push *--tags*)", "push a tag"),
    ("Bash(git *push *--follow-tags*)", "push a tag"),
    ("Bash(git *push *refs/tags/*)", "push a tag"),
    (
        "Bash(gh *release create*)",
        "push a tag (a release creates one)",
    ),
    ("Bash(gh api *git/refs*)", "push a tag (REST ref creation)"),
    ("Bash(gh api *pending_deployments*)", "approve a deployment"),
    ("Bash(cargo *yank *)", "yank a crate"),
];

/// The paragraph every agent definition carries, verbatim.
const NEVER_PARAGRAPH: &str = "**Never merge a pull request, push a tag, approve a \
     deployment or yank a crate.** Those acts are the human's \
     ([release.md](../../implementation/release.md) → *What agents may not do*). \
     `.claude/settings.json` denies the commands that perform them; a denial is the answer, \
     never something to route around.";

#[test]
fn q_the_agent_settings_deny_every_pinned_act() {
    let path = repo_root().join(AGENT_SETTINGS);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("`{AGENT_SETTINGS}` must be readable: {e}"));
    let settings: serde_json::Value = serde_json::from_str(&body)
        .unwrap_or_else(|e| panic!("`{AGENT_SETTINGS}` must be valid JSON: {e}"));
    let deny: Vec<&str> = settings["permissions"]["deny"]
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect()
        })
        .unwrap_or_default();
    let missing: Vec<&(&str, &str)> = AGENT_DENY
        .iter()
        .filter(|(entry, _)| !deny.contains(entry))
        .collect();
    assert!(
        missing.is_empty(),
        "`{AGENT_SETTINGS}` → `permissions.deny` lacks {missing:?}. Agents run `gh`, `git` \
         and `cargo` with the human's own credentials, so this list is the only thing \
         between an agent and those acts (release.md → What agents may not do)"
    );
}

#[test]
fn q_every_agent_definition_carries_the_never_paragraph() {
    let root = repo_root();
    let definitions = root_walk::files_in(&root.join(AGENT_DEFINITIONS), root_walk::ext("md"));
    let lacking: Vec<String> = definitions
        .iter()
        .filter(|path| {
            !fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("`{}` must be readable: {e}", path.display()))
                .contains(NEVER_PARAGRAPH)
        })
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap_or(path)
                .display()
                .to_string()
        })
        .collect();
    assert!(
        lacking.is_empty(),
        "every agent definition must carry the Never paragraph verbatim; these do not: \
         {lacking:?}. A definition cannot enforce a scoped Bash deny, so the paragraph is \
         how it names the acts that are the human's"
    );
}
