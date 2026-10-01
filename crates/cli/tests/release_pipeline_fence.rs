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
//! `dev/unpublished-versions` and outputs its flag; **(m)** the release-PR job's checkout,
//! its git identity and `release-plz release-pr` all run on the GitHub App's token, and
//! the `run:` body, executed against a stub, passes exactly the argv `release-plz/action`
//! built; **(n)** nothing keys on a commit subject; **(o)** each release-plz job installs
//! release-plz through `dev/install-release-plz`, which pins `0.3.169` and its asset's
//! SHA-256 as `release.md` records them, refuses a mismatching archive, and no workflow
//! names `release-plz/action` any more; **(p)** the dry run and the engine overlay apply
//! off `gherrink/jigc` only, the `release` body read from an environment variable and
//! executed against the stub for each value.
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
use crate::workflow_action_runtime_fence::vetted_uses;
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
/// The git-identity action, by name; its pinned commit has one home,
/// `workflow_action_runtime_fence::VETTED`.
const GIT_CONFIG_ACTION: &str = "release-plz/git-config";

/// The composite the two jobs no longer use: it fetched `cargo-binstall` at `latest` and
/// release-plz through it, neither against a checksum (release.md → Publishing → The
/// release-plz binary). Any pinned reference to it, even a commented-out one, reddens.
const RETIRED_RELEASE_PLZ_ACTION: &str = "release-plz/action@";

/// The script that installs release-plz against its recorded digest, and the two lines
/// of it that pin the version and the digest.
const INSTALL_RELEASE_PLZ: &str = "dev/install-release-plz";
const INSTALL_VERSION_LINE: &str = "RELEASE_PLZ_VERSION=";
const INSTALL_SHA256_LINE: &str = "RELEASE_PLZ_SHA256=";

/// The one asset the script fetches, and the doc that records its version and digest.
const RELEASE_PLZ_ASSET: &str = "release-plz-x86_64-unknown-linux-gnu.tar.gz";
const RELEASE_MD: &str = "implementation/release.md";

/// The release-plz version every proof on the record ran (release.md → release-plz knobs).
const RELEASE_PLZ_VERSION: &str = "0.3.169";

/// Anything off the real repository is a rehearsal.
const OFF_THE_REAL_REPO: &str = "github.repository != 'gherrink/jigc'";

/// The `DRY_RUN` environment value of the `release` run step: `'true'` off the real
/// repository and empty on it. The body maps exactly those two values and refuses any
/// other, so the bare boolean's string `false` could not reach release-plz as a dry run
/// of the real publish, as it would have through `release-plz/action`'s non-empty test.
const DRY_RUN_ENV: &str = "${{ github.repository != 'gherrink/jigc' && 'true' || '' }}";

/// The argv `release-plz/action` `v0.5.139` built for each command with only `command`
/// (and, for `release`, `dry_run`) set, with `$GITHUB_TOKEN` = [`STUB_TOKEN`] and the
/// repository URL from [`STUB_SERVER`]/[`STUB_REPOSITORY`] (its `action.yml`, *Run
/// release-plz*). The run bodies must reproduce them exactly.
const STUB_TOKEN: &str = "stub-token";
const STUB_SERVER: &str = "https://github.example";
const STUB_REPOSITORY: &str = "owner/repo";
const RELEASE_PR_ARGV: [&str; 9] = [
    "release-pr",
    "--git-token",
    STUB_TOKEN,
    "--repo-url",
    "https://github.example/owner/repo",
    "--forge",
    "github",
    "-o",
    "json",
];
const RELEASE_ARGV: [&str; 7] = [
    "release",
    "--git-token",
    STUB_TOKEN,
    "--forge",
    "github",
    "-o",
    "json",
];

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
// The release-plz steps, shared by (m), (o) and (p)
// ---------------------------------------------------------------------------

/// The `run:` body of a step, if it has one.
fn run_body(step: &Yaml) -> Option<&str> {
    step["run"].as_str()
}

/// Whether `body` invokes `release-plz <command>`: a line whose first two words are
/// exactly those, so `release` never matches `release-pr`.
fn invokes_release_plz(body: &str, command: &str) -> bool {
    body.lines().any(|line| {
        let mut words = line.split_whitespace();
        words.next() == Some("release-plz") && words.next() == Some(command)
    })
}

/// The three steps a release-plz job runs, by index into its steps: the checksummed
/// install, the git identity, and the `release-plz <command>` run.
struct ReleasePlzSteps<'a> {
    identity: &'a Yaml,
    run: &'a Yaml,
    run_at: usize,
}

/// The release-plz steps of `job`, each asserted to occur exactly once and in the order
/// checkout → install → identity → run: the install script lives in the checkout, and
/// release-plz commits under the identity the step before it writes.
fn release_plz_steps<'a>(job: &'a Yaml, job_id: &str, command: &str) -> ReleasePlzSteps<'a> {
    let steps = job_steps(job);
    let one = |what: &str, matches: &dyn Fn(&Yaml) -> bool| -> usize {
        let found: Vec<usize> = steps
            .iter()
            .enumerate()
            .filter(|(_, step)| matches(step))
            .map(|(at, _)| at)
            .collect();
        assert_eq!(
            found.len(),
            1,
            "job `{job_id}` must have exactly one {what} step, found {}",
            found.len()
        );
        found[0]
    };
    let checkout_at = one("checkout", &|s| {
        s["uses"]
            .as_str()
            .is_some_and(|u| u.starts_with(CHECKOUT_ACTION))
    });
    let install_at = one(&format!("`run: {INSTALL_RELEASE_PLZ}`"), &|s| {
        run_body(s).is_some_and(|run| run.trim() == INSTALL_RELEASE_PLZ)
    });
    let identity_at = one(&format!("`{GIT_CONFIG_ACTION}`"), &|s| {
        s["uses"]
            .as_str()
            .is_some_and(|u| u.starts_with(&format!("{GIT_CONFIG_ACTION}@")))
    });
    let run_at = one(&format!("`release-plz {command}`"), &|s| {
        run_body(s).is_some_and(|run| invokes_release_plz(run, command))
    });
    assert!(
        checkout_at < install_at && install_at < identity_at && identity_at < run_at,
        "job `{job_id}` must run checkout ({checkout_at}) → `{INSTALL_RELEASE_PLZ}` \
         ({install_at}) → `{GIT_CONFIG_ACTION}` ({identity_at}) → `release-plz {command}` \
         ({run_at}), in that order"
    );
    let run = &steps[run_at];
    let body = run_body(run).expect("the run step has a body");
    assert!(
        !body.contains("${{"),
        "job `{job_id}`'s `release-plz {command}` body splices a `${{{{ … }}}}` \
         expression into the script; pass it through `env:` instead:\n{body}"
    );
    ReleasePlzSteps {
        identity: &steps[identity_at],
        run,
        run_at,
    }
}

/// A throwaway directory that removes itself on drop.
struct TempDir(std::path::PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "jigc-release-pipeline-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Execute a step's `run:` body verbatim, as GitHub's default `shell: bash` does
/// (`bash --noprofile --norc -e -o pipefail <file>`), with a stub `release-plz` first on
/// `PATH` that records its argv. Returns the process output and the argv the stub saw,
/// `None` when the body never reached it.
fn run_against_stub(
    body: &str,
    env: &[(&str, &str)],
) -> (std::process::Output, Option<Vec<String>>) {
    let dir = TempDir::new("stub");
    let bin = dir.path().join("bin");
    fs::create_dir_all(&bin).expect("create the stub's bin");
    let argv_file = dir.path().join("argv");
    let stub = bin.join("release-plz");
    fs::write(&stub, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$STUB_ARGV\"\n").expect("write stub");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&stub, fs::Permissions::from_mode(0o755)).expect("chmod stub");
    }
    let script = dir.path().join("step.sh");
    fs::write(&script, body).expect("write the step body");
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let mut command = Command::new("bash");
    command
        .args(["--noprofile", "--norc", "-e", "-o", "pipefail"])
        .arg(&script)
        .env("PATH", path)
        .env("STUB_ARGV", &argv_file)
        .env("GITHUB_TOKEN", STUB_TOKEN)
        .env("GITHUB_SERVER_URL", STUB_SERVER)
        .env("GITHUB_REPOSITORY", STUB_REPOSITORY)
        .env_remove("DRY_RUN");
    for (key, value) in env {
        command.env(key, value);
    }
    let out = command.output().expect("run bash");
    let argv = fs::read_to_string(&argv_file)
        .ok()
        .map(|text| text.lines().map(str::to_owned).collect());
    (out, argv)
}

fn describe(out: &std::process::Output) -> String {
    format!(
        "{:?}\n--- stdout ---\n{}\n--- stderr ---\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
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
    let plz = release_plz_steps(job, RELEASE_PR_JOB, "release-pr");
    assert_eq!(
        plz.identity["env"]["GITHUB_TOKEN"].as_str(),
        Some(token.as_str()),
        "the release PR's commits must carry the App's identity, read from its token"
    );
    assert_eq!(
        plz.run["env"]["GITHUB_TOKEN"].as_str(),
        Some(token.as_str()),
        "`release-plz release-pr` must open the PR with the App's token: a PR the \
         workflow's own token opens triggers no CI run"
    );
    let body = run_body(plz.run).expect("a run body");
    let (out, argv) = run_against_stub(body, &[]);
    assert!(
        out.status.success(),
        "the release-pr body failed: {}",
        describe(&out)
    );
    assert_eq!(
        argv.as_deref(),
        Some(RELEASE_PR_ARGV.map(str::to_owned).as_slice()),
        "the release-pr body must pass exactly the argv `release-plz/action` built"
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
// (o) the release-plz binary: one version, one digest, no composite
// ---------------------------------------------------------------------------

#[test]
fn o_each_release_plz_job_installs_the_checksummed_binary_and_none_uses_the_composite() {
    let workflow = release_workflow();
    let mut installing = Vec::new();
    for (id, job) in workflow_jobs(&workflow) {
        for step in steps_using(job, "release-plz/") {
            assert_eq!(
                step["uses"].as_str(),
                Some(vetted_uses(GIT_CONFIG_ACTION).as_str()),
                "job `{id}`: the only release-plz action a workflow may use is the git \
                 identity, at its vetted commit"
            );
        }
        if job_steps(job)
            .iter()
            .any(|s| run_body(s).is_some_and(|run| run.trim() == INSTALL_RELEASE_PLZ))
        {
            installing.push(id.clone());
        }
    }
    assert_eq!(
        installing,
        [RELEASE_PR_JOB, RELEASE_JOB],
        "each of the two release-plz jobs installs release-plz through `{INSTALL_RELEASE_PLZ}`"
    );
    release_plz_steps(
        workflow_job(&workflow, RELEASE_PR_JOB),
        RELEASE_PR_JOB,
        "release-pr",
    );
    release_plz_steps(workflow_job(&workflow, RELEASE_JOB), RELEASE_JOB, "release");

    let dir = repo_root().join(WORKFLOWS_DIR);
    for entry in fs::read_dir(&dir).expect("the workflow directory is readable") {
        let path = entry.expect("a workflow directory entry").path();
        let body = fs::read_to_string(&path).expect("a workflow file is readable");
        assert!(
            !body.contains(RETIRED_RELEASE_PLZ_ACTION),
            "{} names `{RETIRED_RELEASE_PLZ_ACTION}…`, which downloads cargo-binstall at \
             `latest` and release-plz through it, neither checked (release.md → Publishing → The release-plz binary)",
            path.display()
        );
    }
}

/// The value after `prefix` on the one line of `text` that starts with it.
fn pinned_line<'a>(text: &'a str, prefix: &str) -> &'a str {
    let found: Vec<&str> = text
        .lines()
        .filter_map(|line| line.strip_prefix(prefix))
        .collect();
    assert_eq!(
        found.len(),
        1,
        "`{INSTALL_RELEASE_PLZ}` must carry exactly one `{prefix}` line"
    );
    found[0].trim()
}

#[test]
fn o_the_install_script_pins_the_version_and_digest_release_md_records() {
    let script = fs::read_to_string(repo_root().join(INSTALL_RELEASE_PLZ))
        .unwrap_or_else(|e| panic!("`{INSTALL_RELEASE_PLZ}` must be readable: {e}"));
    let version = pinned_line(&script, INSTALL_VERSION_LINE);
    let digest = pinned_line(&script, INSTALL_SHA256_LINE);
    assert_eq!(
        version, RELEASE_PLZ_VERSION,
        "`{INSTALL_RELEASE_PLZ}` must install release-plz `{RELEASE_PLZ_VERSION}`, the \
         one every proof on the record ran"
    );
    assert!(
        digest.len() == 64
            && digest
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "`{INSTALL_SHA256_LINE}{digest}` is not a 64-character lowercase hex SHA-256"
    );
    assert!(
        script.contains(RELEASE_PLZ_ASSET),
        "`{INSTALL_RELEASE_PLZ}` must fetch `{RELEASE_PLZ_ASSET}`"
    );

    let doc = fs::read_to_string(repo_root().join(RELEASE_MD))
        .unwrap_or_else(|e| panic!("`{RELEASE_MD}` must be readable: {e}"));
    let rows: Vec<&str> = doc
        .lines()
        .filter(|line| line.starts_with('|') && line.contains(RELEASE_PLZ_ASSET))
        .collect();
    assert!(
        !rows.is_empty(),
        "`{RELEASE_MD}` must record `{RELEASE_PLZ_ASSET}` in a table row (Publishing → The release-plz binary)"
    );
    for row in rows {
        assert!(
            row.contains(&format!("`{version}`")) && row.contains(&format!("`{digest}`")),
            "`{RELEASE_MD}` records `{RELEASE_PLZ_ASSET}` as\n{row}\nbut \
             `{INSTALL_RELEASE_PLZ}` pins `{version}` at `{digest}`; move the two together"
        );
    }
}

#[test]
fn o_the_install_script_refuses_an_archive_whose_digest_differs() {
    let dir = TempDir::new("install");
    let scratch = dir.path().join("scratch");
    let staging = dir.path().join("staging");
    let dest = dir.path().join("dest");
    let github_path = dir.path().join("github_path");
    fs::create_dir_all(&scratch).expect("create scratch");
    fs::create_dir_all(&staging).expect("create staging");
    fs::write(&github_path, "").expect("create GITHUB_PATH");

    // A well-formed archive carrying a `release-plz` that would pass the script's own
    // version check, so only the digest stands between it and the install.
    fs::write(
        staging.join("release-plz"),
        format!("#!/bin/sh\necho release-plz {RELEASE_PLZ_VERSION}\n"),
    )
    .expect("write the impostor");
    let impostor = dir.path().join("impostor.tar.gz");
    let tar = Command::new("tar")
        .arg("-czf")
        .arg(&impostor)
        .arg("-C")
        .arg(&staging)
        .arg("release-plz")
        .output()
        .expect("run tar");
    assert!(tar.status.success(), "tar failed: {}", describe(&tar));
    let garbage = dir.path().join("garbage.tar.gz");
    fs::write(&garbage, "not an archive").expect("write garbage");

    for archive in [&impostor, &garbage] {
        let out = Command::new(repo_root().join(INSTALL_RELEASE_PLZ))
            .arg("--archive")
            .arg(archive)
            .arg("--dest")
            .arg(&dest)
            .env("TMPDIR", &scratch)
            .env("GITHUB_PATH", &github_path)
            .output()
            .expect("run the install script");
        assert_eq!(
            out.status.code(),
            Some(1),
            "`{INSTALL_RELEASE_PLZ} --archive {}` must fail closed: {}",
            archive.display(),
            describe(&out)
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("SHA-256 mismatch") && stderr.contains("not installed"),
            "the refusal must name the digest mismatch: {}",
            describe(&out)
        );
        assert!(
            !dest.join("release-plz").exists(),
            "a mismatching archive must place no binary"
        );
        assert_eq!(
            fs::read_to_string(&github_path).expect("read GITHUB_PATH"),
            "",
            "a mismatching archive must put nothing on the job's PATH"
        );
    }
}

// ---------------------------------------------------------------------------
// (p) the rehearsal: dry run and engine overlay off the real repository only
// ---------------------------------------------------------------------------

#[test]
fn p_the_dry_run_and_the_overlay_apply_off_the_real_repository_only() {
    let workflow = release_workflow();
    let release = workflow_job(&workflow, RELEASE_JOB);
    let steps = job_steps(release);
    let plz = release_plz_steps(release, RELEASE_JOB, "release");
    assert_eq!(
        plz.run["env"]["DRY_RUN"].as_str(),
        Some(DRY_RUN_ENV),
        "the release must be dry off `gherrink/jigc` and real on it"
    );

    // The body itself, run for each value the expression can produce and one it cannot.
    let body = run_body(plz.run).expect("a run body");
    let mut dry = RELEASE_ARGV.map(str::to_owned).to_vec();
    dry.insert(dry.len() - 2, "--dry-run".to_owned());
    for (value, expected) in [
        ("true", Some(dry)),
        ("", Some(RELEASE_ARGV.map(str::to_owned).to_vec())),
    ] {
        let (out, argv) = run_against_stub(body, &[("DRY_RUN", value)]);
        assert!(
            out.status.success(),
            "the release body failed for DRY_RUN={value:?}: {}",
            describe(&out)
        );
        assert_eq!(argv, expected, "the release argv for DRY_RUN={value:?}");
    }
    let (out, argv) = run_against_stub(body, &[("DRY_RUN", "false")]);
    assert!(
        !out.status.success() && argv.is_none(),
        "DRY_RUN=\"false\" must stop the job before release-plz runs, never pass as a real \
         or a dry release: {}",
        describe(&out)
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
        overlay_at < plz.run_at,
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
