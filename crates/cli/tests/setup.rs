//! End-to-end integration test for `jigc setup` — the adapter install.
//!
//! Drives the built `jigc` binary against a throwaway temp repo and asserts the
//! reworked install (`design/assistant-adapter.md` → Generated, minimal,
//! regenerated; `DECISIONS.md` 2026-05-31 → adapter install reworked):
//!   1. the bootstrap sentence lands in a managed `.jigc/AGENT.md`;
//!   2. a bare `@.jigc/AGENT.md` import line lands in `CLAUDE.md` (no marker
//!      comments);
//!   3. the project layer is initialized (`.jigc/config/.gitkeep`) and
//!      `.jigc/.gitignore` exists;
//!   4. the `jigc *` permit **and** the `SessionStart` hook running `jigc start`
//!      land in `.claude/settings.json`.
//!
//! It also asserts the command exits clean, is idempotent (a second run leaves
//! `CLAUDE.md` + `.jigc/AGENT.md` + `.claude/settings.json` byte-identical), and
//! that `jigc start` renders the *clean* orientation after setup (the project
//! reads as set up).
//!
//! The **no-clobber + idempotency** acceptance (`design/worked-examples.md` →
//! flow 11 / flow 12 assertion 3; `design/project-setup.md` → Idempotency &
//! irreversibility; roadmap M9 Increment 1 scope bullet 4) lifts today's
//! unit-only structure-aware-merge coverage to the binary: a repo seeded with a
//! non-trivial `CLAUDE.md` (house-rules prose) and a `.claude/settings.json`
//! carrying a pre-existing non-jigc hook + unrelated permit + unrelated key has
//! `jigc setup` run **twice**, asserting the seeded human content survives
//! **verbatim** (structure-aware merge, never clobber), the jigc reference /
//! allowlist / SessionStart hook were added, and the second run is a
//! byte-identical no-op on `CLAUDE.md` + `.claude/settings.json` + `.jigc/AGENT.md`.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is built with `std::fs`, and a
//! self-cleaning `TempDir` keeps the test off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-setup-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Mark `root` as a git repo for repo-root discovery, without invoking git — a
/// bare `.git` directory is enough for `locate`.
fn mark_repo(root: &Path) {
    fs::create_dir_all(root.join(".git")).expect("create .git marker");
}

/// Run the built `jigc setup` binary with `cwd = repo` and `$HOME = home`.
fn run_setup(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run the built `jigc start` binary with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("start")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn setup_installs_reference_layer_and_allowlist() {
    let repo = TempDir::new("install");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_setup(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // (a) The bootstrap sentence lands in a managed `.jigc/AGENT.md`.
    let agent_md =
        fs::read_to_string(repo.path().join(".jigc/AGENT.md")).expect(".jigc/AGENT.md was written");
    assert!(
        agent_md.contains("`jigc` is your interface to this project"),
        ".jigc/AGENT.md must carry the bootstrap sentence; got:\n{agent_md}",
    );

    // (b) CLAUDE.md carries the bare `@.jigc/AGENT.md` reference and NO markers.
    let claude_md =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md was written");
    assert!(
        claude_md.contains("@.jigc/AGENT.md"),
        "CLAUDE.md must carry the bare `@.jigc/AGENT.md` reference; got:\n{claude_md}",
    );
    assert!(
        !claude_md.contains("<!-- jigc:bootstrap"),
        "CLAUDE.md must NOT carry the old marker comments; got:\n{claude_md}",
    );

    // (c) The project layer is initialized and the `.jigc/.gitignore` exists.
    assert!(
        repo.path().join(".jigc/config/.gitkeep").exists(),
        "setup must create `.jigc/config/.gitkeep`",
    );
    assert!(
        repo.path().join(".jigc/.gitignore").exists(),
        "setup must create `.jigc/.gitignore`",
    );

    // (d) Both the `jigc *` allowlist AND the SessionStart hook running
    //     `jigc start` landed in .claude/settings.json (parsed as JSON).
    let settings_raw = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json was written");
    assert!(
        settings_raw.contains("\"jigc *\""),
        ".claude/settings.json must permit `jigc *`; got:\n{settings_raw}",
    );
    let settings: serde_json::Value =
        serde_json::from_str(&settings_raw).expect(".claude/settings.json must be valid JSON");
    assert_eq!(
        settings["permissions"]["allow"][0], "jigc *",
        "the allowlist must permit `jigc *`; got:\n{settings_raw}",
    );
    assert!(
        session_start_runs_jigc_start(&settings),
        "settings.json must carry a SessionStart hook running `jigc start`; got:\n{settings_raw}",
    );

    // Idempotent: a second run leaves CLAUDE.md + .jigc/AGENT.md +
    // .claude/settings.json byte-identical (no duplicated allowlist or hook).
    let out2 = run_setup(repo.path(), home.path());
    assert!(out2.status.success(), "second `jigc setup` must exit 0");
    let claude_md2 =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md still present");
    let agent_md2 = fs::read_to_string(repo.path().join(".jigc/AGENT.md"))
        .expect(".jigc/AGENT.md still present");
    let settings2 = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json still present");
    assert_eq!(
        claude_md, claude_md2,
        "a second `jigc setup` must leave CLAUDE.md byte-identical",
    );
    assert_eq!(
        agent_md, agent_md2,
        "a second `jigc setup` must leave .jigc/AGENT.md byte-identical",
    );
    assert_eq!(
        settings_raw, settings2,
        "a second `jigc setup` must leave .claude/settings.json byte-identical",
    );
}

/// Whether `settings` carries a `hooks.SessionStart[*].hooks[*]` entry that runs
/// the `jigc start` command — the structural presence the hook install ensures.
fn session_start_runs_jigc_start(settings: &serde_json::Value) -> bool {
    settings["hooks"]["SessionStart"]
        .as_array()
        .is_some_and(|matchers| {
            matchers.iter().any(|matcher| {
                matcher["hooks"].as_array().is_some_and(|hooks| {
                    hooks
                        .iter()
                        .any(|hook| hook["command"] == "jigc start" && hook["type"] == "command")
                })
            })
        })
}

/// `jigc setup` over the SHIPPED profile (a valid spawn launch template) installs
/// clean and exits 0 — the install-time spawn-template gate
/// (`crate::setup::install` step 0; `design/assistant-adapter.md` → Bind the spawn
/// mechanism) lets the good template through. This is the binary-level proof of the
/// Deliverable's "setup with a good template" half: the gate runs in the real
/// install path on every `jigc setup`, and a valid template does not block it.
///
/// The complementary "several rejected templates each fail install" half is proven
/// through the binary over the `JIGC_ADAPTERS_DIR` adapter-profile-source seam
/// (`tests/adapter_source_setup.rs`; `DECISIONS.md` 2026-06-05) — the adapter
/// analogue of `JIGC_PACK_DIR` — which drives a deliberately-broken spawn template
/// through this same install path and observes the blocking `setup.spawn-template`
/// finding with no host write. It is also unit-proven at the `install()` boundary
/// (`setup.rs`'s `install_rejects_broken_spawn_template_with_clause_route`) plus
/// T2's full clause-coverage table.
#[test]
fn setup_with_valid_spawn_template_installs_clean() {
    let repo = TempDir::new("spawn-gate");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_setup(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` over the shipped (valid) spawn template must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("setup.spawn-template"),
        "a valid spawn template must not trip the install-time gate; got stderr:\n{stderr}",
    );
}

#[test]
fn start_renders_clean_orientation_after_setup() {
    let repo = TempDir::new("orient");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let setup = run_setup(repo.path(), home.path());
    assert!(setup.status.success(), "`jigc setup` must exit 0");

    let out = run_start(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc start` must exit 0 after setup; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.contains("isn't set up"),
        "after setup, `jigc start` must NOT report the unset-project view; got:\n{stdout}",
    );
    assert!(
        stdout.contains("Pack:"),
        "after setup, `jigc start` must render the clean provenance header; got:\n{stdout}",
    );
}

/// The binary-level no-clobber + idempotency acceptance (`design/worked-examples.md`
/// → flow 11 / flow 12 assertion 3; `design/project-setup.md` → Idempotency &
/// irreversibility). Today's `setup.rs` idempotency assertions run only on a FRESH
/// repo (no pre-existing host content); the no-clobber-of-seeded-content +
/// verbatim-preservation case is uncovered at the binary. This seeds a non-trivial
/// `CLAUDE.md` (house-rules prose) + a `.claude/settings.json` carrying a
/// pre-existing non-jigc hook, an unrelated permit, and an unrelated top-level key,
/// runs `jigc setup` **twice**, and asserts the seeded human content survives
/// verbatim, the jigc additions land, and the second run is a byte-identical no-op.
#[test]
fn setup_preserves_seeded_host_content_and_is_idempotent() {
    let repo = TempDir::new("noclobber");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    // Seed a non-trivial CLAUDE.md with house-rules prose the human owns.
    let seeded_claude = "# House rules\n\n\
        - Always run the linter before committing.\n\
        - Prefer small, focused PRs.\n\n\
        ## Architecture\n\n\
        The gateway owns rate limiting; do not duplicate it downstream.\n";
    fs::write(repo.path().join("CLAUDE.md"), seeded_claude).expect("seed CLAUDE.md");

    // Seed a .claude/settings.json with a pre-existing NON-jigc hook (a
    // PreToolUse matcher running a house command), an unrelated permit, and an
    // unrelated top-level key — all of which the structure-aware merge must
    // preserve while it adds the jigc allowlist + SessionStart hook.
    fs::create_dir_all(repo.path().join(".claude")).expect("seed .claude dir");
    let seeded_settings = serde_json::json!({
        "model": "claude-sonnet-4",
        "permissions": { "allow": ["git status"] },
        "hooks": {
            "PreToolUse": [
                { "hooks": [ { "type": "command", "command": "house-precheck.sh" } ] }
            ]
        }
    });
    fs::write(
        repo.path().join(".claude/settings.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&seeded_settings).unwrap()
        ),
    )
    .expect("seed .claude/settings.json");

    // First run.
    let out = run_setup(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` over seeded host content must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // (a) The seeded human content survives VERBATIM.
    let claude_md =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md present after setup");
    assert!(
        claude_md.contains(seeded_claude),
        "the seeded house-rules CLAUDE.md prose must survive verbatim; got:\n{claude_md}",
    );

    let settings_raw = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json present after setup");
    let settings: serde_json::Value =
        serde_json::from_str(&settings_raw).expect(".claude/settings.json must be valid JSON");
    assert_eq!(
        settings["model"], "claude-sonnet-4",
        "an unrelated top-level key must survive the merge; got:\n{settings_raw}",
    );
    assert!(
        settings["permissions"]["allow"]
            .as_array()
            .is_some_and(|a| a.iter().any(|v| v == "git status")),
        "the pre-existing unrelated permit must survive the merge; got:\n{settings_raw}",
    );
    assert!(
        pretool_runs_house_precheck(&settings),
        "the pre-existing non-jigc PreToolUse hook must survive the merge; got:\n{settings_raw}",
    );

    // (b) The jigc reference / allowlist / SessionStart hook were ADDED.
    assert!(
        claude_md.contains("@.jigc/AGENT.md"),
        "setup must add the `@.jigc/AGENT.md` reference to the seeded CLAUDE.md; got:\n{claude_md}",
    );
    assert!(
        settings["permissions"]["allow"]
            .as_array()
            .is_some_and(|a| a.iter().any(|v| v == "jigc *")),
        "setup must add the `jigc *` permit alongside the seeded one; got:\n{settings_raw}",
    );
    assert!(
        session_start_runs_jigc_start(&settings),
        "setup must add the SessionStart hook running `jigc start`; got:\n{settings_raw}",
    );
    let agent_md = fs::read_to_string(repo.path().join(".jigc/AGENT.md"))
        .expect(".jigc/AGENT.md written by setup");

    // (c) The second run is a byte-identical no-op on all three managed files.
    let out2 = run_setup(repo.path(), home.path());
    assert!(
        out2.status.success(),
        "the second `jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out2.status,
        String::from_utf8_lossy(&out2.stderr),
    );
    let claude_md2 =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md still present");
    let settings_raw2 = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json still present");
    let agent_md2 = fs::read_to_string(repo.path().join(".jigc/AGENT.md"))
        .expect(".jigc/AGENT.md still present");
    assert_eq!(
        claude_md, claude_md2,
        "a second `jigc setup` must leave CLAUDE.md byte-identical",
    );
    assert_eq!(
        settings_raw, settings_raw2,
        "a second `jigc setup` must leave .claude/settings.json byte-identical",
    );
    assert_eq!(
        agent_md, agent_md2,
        "a second `jigc setup` must leave .jigc/AGENT.md byte-identical",
    );
}

/// Whether `settings` carries a `hooks.PreToolUse[*].hooks[*]` entry running the
/// seeded `house-precheck.sh` command — the pre-existing non-jigc hook the merge
/// must preserve.
fn pretool_runs_house_precheck(settings: &serde_json::Value) -> bool {
    settings["hooks"]["PreToolUse"]
        .as_array()
        .is_some_and(|matchers| {
            matchers.iter().any(|matcher| {
                matcher["hooks"].as_array().is_some_and(|hooks| {
                    hooks
                        .iter()
                        .any(|hook| hook["command"] == "house-precheck.sh")
                })
            })
        })
}
