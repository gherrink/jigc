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

use crate::manifest_freeze_fence::repo_root;
use crate::support::run_then_parse::stdout_json;
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
