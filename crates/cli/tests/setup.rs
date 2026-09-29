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
//!   4. the `Bash(jigc:*)` permit **and** the `SessionStart` hook running `jigc start`
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
            engine::tempname::unique_nanos(),
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

/// Make `root` a real git repo. `jigc setup` now installs a `pre-commit` hook that
/// resolves the repo's real hooks dir via git, so a bare `.git` marker no longer
/// suffices — the install runs `git rev-parse` against an actual repo.
fn mark_repo(root: &Path) {
    let out = Command::new("git")
        .args(["init", "-q"])
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git init");
    assert!(
        out.status.success(),
        "git init failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    // `jigc setup` commits its own install footprint (M30 audit finding 1) — even on an
    // unborn HEAD it mints the repo's first commit — so a test repo needs a usable
    // identity and signing off for that commit to land. The deliberate no-identity
    // rejection test (`setup_fails_loudly_when_install_commit_is_rejected`) strips this
    // back out.
    for kv in [
        ["user.email", "test@example.com"],
        ["user.name", "Test"],
        ["commit.gpgsign", "false"],
    ] {
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["config", kv[0], kv[1]])
            .output()
            .expect("run git config")
            .status
            .success();
        assert!(ok, "git config {} failed", kv[0]);
    }
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

    // (d) Both the `Bash(jigc:*)` allowlist AND the SessionStart hook running
    //     `jigc start` landed in .claude/settings.json (parsed as JSON).
    let settings_raw = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json was written");
    assert!(
        settings_raw.contains("\"Bash(jigc:*)\""),
        ".claude/settings.json must permit `Bash(jigc:*)`; got:\n{settings_raw}",
    );
    let settings: serde_json::Value =
        serde_json::from_str(&settings_raw).expect(".claude/settings.json must be valid JSON");
    assert_eq!(
        settings["permissions"]["allow"][0], "Bash(jigc:*)",
        "the allowlist must permit `Bash(jigc:*)`; got:\n{settings_raw}",
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
            .is_some_and(|a| a.iter().any(|v| v == "Bash(jigc:*)")),
        "setup must add the `Bash(jigc:*)` permit alongside the seeded one; got:\n{settings_raw}",
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

/// `jigc setup` writes nothing beside the binary (M54): the `doc-code` probe runs inside
/// `jigc`, so there is no sibling to extract. Driven against the real built binary copied
/// alone into a fresh dir, as a `cargo install` leaves it, and that dir made **read-only**
/// — the state in which the retired probe-extract step failed the whole install. Setup
/// exits 0, and the dir still holds only the `jigc` that ran.
#[test]
fn setup_writes_nothing_beside_the_binary_even_when_its_dir_is_read_only() {
    let bin = TempDir::new("setup-bin");
    let jigc = bin.path().join("jigc");
    fs::copy(env!("CARGO_BIN_EXE_jigc"), &jigc).expect("copy the built jigc into a fresh dir");
    make_executable(&jigc);

    let repo = TempDir::new("setup-bin-repo");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    set_dir_readonly(bin.path(), true);
    let out = run_copied_setup(&jigc, repo.path(), home.path());
    let listed = fs::read_dir(bin.path()).map(|dir| {
        let mut names: Vec<String> = dir
            .map(|entry| {
                entry
                    .expect("read a bin dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    });
    set_dir_readonly(bin.path(), false); // restore so TempDir can clean up
    assert!(
        out.status.success(),
        "`jigc setup` from a read-only bin dir must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        listed.expect("list the bin dir"),
        vec!["jigc".to_string()],
        "setup must leave the bin dir holding only the `jigc` it ran from",
    );
}

/// Run the **copied** `jigc setup` (a specific binary path, not `CARGO_BIN_EXE_jigc`)
/// with `cwd = repo` and `$HOME = home`, so `current_exe()` resolves to the copied
/// install dir.
fn run_copied_setup(jigc: &Path, repo: &Path, home: &Path) -> std::process::Output {
    Command::new(jigc)
        .arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the copied jigc binary")
}

/// Make `path` owner-executable (`0o755`) — used for the copied `jigc`.
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path).expect("stat for chmod").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

/// Toggle a directory between read-only (`0o555`) and writable (`0o755`), so any write
/// setup attempted beside the binary would fail.
fn set_dir_readonly(dir: &Path, readonly: bool) {
    use std::os::unix::fs::PermissionsExt;
    let mode = if readonly { 0o555 } else { 0o755 };
    let mut perms = fs::metadata(dir).expect("stat dir for chmod").permissions();
    perms.set_mode(mode);
    fs::set_permissions(dir, perms).expect("chmod dir");
}

/// `jigc setup` writes the `compose-embedded-methodology: true` marker into the
/// project layer's `.jigc/config/packs.yaml` — the marker the pack factory
/// (`pack::read_compose_marker`) reads to compose the embedded methodology pack, so
/// a clean `setup` gives a real project the dev+methodology surface out of the box
/// (M21). The write is repo-local, idempotent, and non-destructive:
///   (i) after setup, `packs.yaml` carries `compose-embedded-methodology: true`;
///   (ii) a second setup leaves the file byte-identical (idempotent no-op);
///   (iii) a pre-seeded `packs:` list keeps its **entries, in order and content**, and
///         gains the marker alongside them.
///
/// **(iii) has now swung back** (M49 Inc 6 T2). M42 Inc 6 made setup decline the marker
/// over a hand-written `packs:` list, because the combination was one the pack factory
/// could not honor: it used to discard the listed packs silently, and then refused the
/// pack-set outright. Since T1 the factory *composes* it — `[listed… ▸ dev ▸
/// methodology]`, with a listed pack demoted for any doctype the freeze governs — so
/// declining the marker no longer protects anything and instead costs the operator the
/// whole methodology surface for declaring one house pack. Setup writes the marker and
/// keeps the list. **Bound:** the write is a YAML parse-mutate-serialize, so the
/// entries survive verbatim while the file's layout is re-emitted canonically — which
/// is why (iii) asserts on the entries rather than on the whole file's bytes.
#[test]
fn setup_writes_compose_embedded_methodology_marker() {
    // (i) A clean setup writes the marker.
    let repo = TempDir::new("compose-marker");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_setup(repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    let packs_path = repo.path().join(".jigc/config/packs.yaml");
    let first = fs::read_to_string(&packs_path).expect("setup must write .jigc/config/packs.yaml");
    let parsed: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&first).expect("the written packs.yaml must be valid YAML");
    assert_eq!(
        parsed
            .get("compose-embedded-methodology")
            .and_then(serde_yaml_ng::Value::as_bool),
        Some(true),
        "setup must write `compose-embedded-methodology: true`; got:\n{first}",
    );

    // (ii) A second setup is a byte-identical no-op on the marker file.
    let out2 = run_setup(repo.path(), home.path());
    assert!(
        out2.status.success(),
        "second `jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out2.stderr),
    );
    let second = fs::read_to_string(&packs_path).expect("packs.yaml still present after re-setup");
    assert_eq!(
        first, second,
        "a second `jigc setup` must leave .jigc/config/packs.yaml byte-identical",
    );

    // (iii) A pre-seeded `packs:` list keeps its entries, in order and content, and
    //       gains the marker alongside them — the two compose since M49 Inc 6.
    let seeded_repo = TempDir::new("compose-marker-seeded");
    mark_repo(seeded_repo.path());
    let seeded_config = seeded_repo.path().join(".jigc/config");
    fs::create_dir_all(&seeded_config).expect("create the seeded project config dir");
    let seeded_bytes = "packs:\n  - packs/local-pack\n";
    fs::write(seeded_config.join("packs.yaml"), seeded_bytes)
        .expect("seed a hand-written packs.yaml carrying a packs: list");

    let out3 = run_setup(seeded_repo.path(), home.path());
    assert!(
        out3.status.success(),
        "`jigc setup` over a seeded packs.yaml must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out3.stderr),
    );
    let seeded_after = fs::read_to_string(seeded_config.join("packs.yaml"))
        .expect("the seeded packs.yaml is present after setup");
    let seeded_entries: Vec<&str> = seeded_after
        .lines()
        .skip_while(|line| line.trim_end() != "packs:")
        .skip(1)
        .map_while(|line| line.trim_start().strip_prefix("- "))
        .map(str::trim)
        .collect();
    assert_eq!(
        seeded_entries,
        vec!["packs/local-pack"],
        "the pre-seeded `packs:` entries must survive setup in order and content; \
         got:\n{seeded_after}",
    );
    assert!(
        seeded_after.contains("compose-embedded-methodology: true"),
        "setup must wire the marker alongside the operator's list; got:\n{seeded_after}",
    );
    let warning = String::from_utf8_lossy(&out3.stderr);
    assert!(
        !warning.contains("left unwired"),
        "setup must no longer report the embedded methodology pack as left unwired; \
         stderr:\n{warning}",
    );
}

/// Run the built `jigc setup` with `cwd = repo` and **no usable git identity**:
/// `$HOME = home` (an empty temp dir, no `~/.gitconfig`), global/system config
/// neutralized to `/dev/null`, and the `GIT_AUTHOR_*` / `GIT_COMMITTER_*` env vars
/// cleared — so the install commit is rejected by git ("tell me who you are").
fn run_setup_no_identity(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .env_remove("GIT_AUTHOR_NAME")
        .env_remove("GIT_AUTHOR_EMAIL")
        .env_remove("GIT_COMMITTER_NAME")
        .env_remove("GIT_COMMITTER_EMAIL")
        .output()
        .expect("run the jigc binary")
}

/// Run `git -C <root> <args>` with an **inline, non-persisted** throwaway identity
/// (`-c user.email=… -c user.name=…`) and global/system config neutralized — used to
/// seed an initial commit so HEAD exists without writing any identity into the repo
/// config. Asserts success.
fn git_inline_identity(root: &Path, args: &[&str]) {
    let mut full = vec!["-c", "user.email=seed@example.com", "-c", "user.name=Seed"];
    full.extend_from_slice(args);
    let out = Command::new("git")
        .args(&full)
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .expect("run git with an inline identity");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Seed an initial commit so `HEAD` exists (a *born* HEAD — the bug's precondition;
/// an unborn HEAD is the intended graceful-skip path). The seed identity is passed
/// inline and is NOT persisted to the repo config, so the subsequent `jigc setup`
/// still runs against a repo with no usable identity.
fn seed_initial_commit(root: &Path) {
    fs::write(root.join("README.md"), "seed\n").expect("seed README");
    git_inline_identity(root, &["add", "README.md"]);
    git_inline_identity(root, &["commit", "-q", "-m", "seed"]);
}

/// REGRESSION (dogfood): `jigc setup` must NOT print an unqualified success and exit 0
/// when its install commit is silently rejected (e.g. no git identity), leaving the
/// install files staged-but-uncommitted. Over a repo with a born HEAD but no usable
/// identity, the rejected install commit must fail loudly — exit non-zero with an
/// actionable message naming the rejected `git commit` — mirroring `finalize`'s
/// identical git-identity failure. The install files stay staged so a re-run (once the
/// identity is configured) commits them (recoverable, not lost).
#[test]
fn setup_fails_loudly_when_install_commit_is_rejected() {
    let repo = TempDir::new("commit-rejected");
    let home = TempDir::new("home");
    mark_repo(repo.path());
    seed_initial_commit(repo.path());
    // `mark_repo` now seeds a repo-local identity (setup commits its install); strip it
    // so this test still exercises the no-usable-identity rejection path.
    git_inline_identity(repo.path(), &["config", "--unset", "user.email"]);
    git_inline_identity(repo.path(), &["config", "--unset", "user.name"]);
    // …and forbid git's *implicit* identity (gecos name + hostname), which it
    // derives silently on some hosts — there the install commit would SUCCEED and
    // this test's precondition (a rejected commit) would never arise, turning a
    // regression fence into a machine-dependent red. `user.useConfigOnly` makes the
    // rejection deterministic on every host.
    git_inline_identity(repo.path(), &["config", "user.useConfigOnly", "true"]);

    let head_before = String::from_utf8_lossy(
        &Command::new("git")
            .args(["-C"])
            .arg(repo.path())
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("rev-parse HEAD")
            .stdout,
    )
    .trim()
    .to_string();

    let out = run_setup_no_identity(repo.path(), home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // (1) NOT an unqualified exit-0 success.
    assert!(
        !out.status.success(),
        "a rejected install commit must NOT report an unqualified exit-0 success; \
         got exit {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status,
    );

    // (2) The message names the cause — the rejected `git commit` — actionably.
    assert!(
        stderr.contains("git commit") && stderr.contains("rejected"),
        "the failure must name the rejected git commit; got stderr:\n{stderr}",
    );

    // (3) No install commit landed — HEAD is unchanged.
    let head_after = String::from_utf8_lossy(
        &Command::new("git")
            .args(["-C"])
            .arg(repo.path())
            .args(["rev-parse", "HEAD"])
            .output()
            .expect("rev-parse HEAD")
            .stdout,
    )
    .trim()
    .to_string();
    assert_eq!(
        head_before, head_after,
        "no install commit must have landed when the commit was rejected",
    );

    // (4) The install files are left STAGED — recoverable: configure an identity and
    //     re-run `jigc setup` to commit them. (The writes themselves succeeded.)
    let staged = String::from_utf8_lossy(
        &Command::new("git")
            .args(["-C"])
            .arg(repo.path())
            .args(["diff", "--cached", "--name-only"])
            .output()
            .expect("diff --cached")
            .stdout,
    )
    .into_owned();
    assert!(
        staged.lines().any(|l| l == "CLAUDE.md"),
        "the install files must be left staged for a re-run; staged:\n{staged}",
    );
}

/// The happy path the fix must NOT break: over a repo with a usable git identity and a
/// born HEAD, `jigc setup` self-commits its install files and the success banner names
/// the install commit (`- install commit → <sha>`), exiting 0.
#[test]
fn setup_self_commits_install_with_a_git_identity() {
    let repo = TempDir::new("commit-ok");
    let home = TempDir::new("home");
    mark_repo(repo.path());
    // A usable local identity (persisted to the repo config) + a born HEAD.
    git_inline_identity(repo.path(), &["config", "user.email", "dev@example.com"]);
    git_inline_identity(repo.path(), &["config", "user.name", "Dev"]);
    git_inline_identity(repo.path(), &["config", "commit.gpgsign", "false"]);
    seed_initial_commit(repo.path());

    let out = run_setup(repo.path(), home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "`jigc setup` with a git identity must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stdout.contains("install commit →"),
        "the success banner must name the install commit; got stdout:\n{stdout}",
    );
    // The HEAD subject is the dedicated install commit.
    let subject = String::from_utf8_lossy(
        &Command::new("git")
            .args(["-C"])
            .arg(repo.path())
            .args(["log", "-1", "--format=%s"])
            .output()
            .expect("git log")
            .stdout,
    )
    .trim()
    .to_string();
    assert_eq!(
        subject, "chore(jigc): install jigc workspace config",
        "HEAD must be the dedicated install commit",
    );
}

/// Run a git subcommand in `root` with the ambient global/system config neutralized,
/// asserting it succeeds (the hermetic sibling of [`mark_repo`]'s `git init`).
fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap_or_else(|err| panic!("run git {args:?}: {err}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The path the setup success summary names for the installed `pre-commit` hook —
/// extracted from the emitted line itself (the surface under test is the emitted
/// bytes, never a reconstruction).
fn printed_hook_path(stdout: &str) -> &str {
    const MARKER: &str = "pre-commit hook → ";
    let line = stdout
        .lines()
        .find(|line| line.contains(MARKER))
        .unwrap_or_else(|| {
            panic!("the setup summary must name the pre-commit hook; got:\n{stdout}")
        });
    let (_, rest) = line.split_once(MARKER).expect("the marker is present");
    rest.split_once("   (").map_or(rest, |(path, _)| path)
}

/// Resolve a printed path the way its reader would: absolute as-is, relative against the
/// **root the install was made at**.
///
/// That root is the parameter, and since M53 (the cwd fixes' review, LOW 5) it is not
/// always the directory `jigc setup` was typed in: the door binds `repo::jigc_home`, so
/// from a linked worktree it installs at the main checkout and every relative path it
/// prints is spelled against *that* root — law 1's own rule (`render::repo_relative`,
/// relative inside the root, absolute outside it), applied to the root the install
/// actually used. What makes that resolvable for a reader standing elsewhere is the ack's
/// own site line, which names the root (`render::InstallSite`).
fn resolve_printed(root: &Path, printed: &str) -> PathBuf {
    let path = Path::new(printed);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

/// Assert the summary's hook line names **the file the install actually wrote** — the
/// D4 / P1-7 contract: `jigc setup` names the hooks dir git resolved, not the assumed
/// `.git/hooks` literal. `expected` is where the hook really landed, and `root` is the
/// install's own root ([`resolve_printed`]).
fn assert_names_installed_hook(stdout: &str, root: &Path, expected: &Path) {
    assert_named_hook_is_installed(printed_hook_path(stdout), root, expected);
}

/// The shared half of the hook-path contract, over whichever surface emitted `named`:
/// the path a reader resolves from the emitted bytes must be the hook the install
/// actually wrote. Text and JSON make the same promise, so they assert through one body.
fn assert_named_hook_is_installed(printed: &str, cwd: &Path, expected: &Path) {
    let named = resolve_printed(cwd, printed);
    assert!(
        named.exists(),
        "the summary names `{printed}`, which resolves to `{}` — a file that does not \
         exist (the hook the install wrote is `{}`)",
        named.display(),
        expected.display(),
    );
    assert_eq!(
        fs::canonicalize(&named).expect("canonicalize the named hook"),
        fs::canonicalize(expected).expect("canonicalize the installed hook"),
        "the summary must name the installed hook; it named `{printed}`",
    );
}

/// With `core.hooksPath` set, the summary names **that** dir's `pre-commit`, not the
/// `.git/hooks` literal (which holds no hook at all here).
#[test]
fn setup_names_the_core_hookspath_hook() {
    let repo = TempDir::new("hookspath");
    mark_repo(repo.path());
    let home = TempDir::new("home");
    let hooks = repo.path().join("my-hooks");
    git(
        repo.path(),
        &["config", "core.hooksPath", hooks.to_str().unwrap()],
    );

    let out = run_setup(repo.path(), home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "`jigc setup` must exit 0; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let installed = hooks.join("pre-commit");
    assert!(
        installed.exists(),
        "the install must write the hook into the core.hooksPath dir",
    );
    assert!(
        !repo.path().join(".git/hooks/pre-commit").exists(),
        "with core.hooksPath set nothing lands in .git/hooks — the literal names nothing",
    );
    assert_names_installed_hook(&stdout, repo.path(), &installed);
}

/// From a linked worktree (the `.git`-is-a-file case), the summary names the **common**
/// hooks dir under the main checkout — the worktree's own `.git/hooks` does not exist.
#[test]
fn setup_names_the_worktree_common_hooks_dir() {
    let main = TempDir::new("wt-main");
    mark_repo(main.path());
    let home = TempDir::new("home");
    git(
        main.path(),
        &["commit", "-q", "--allow-empty", "-m", "init"],
    );
    let linked = main.path().join("linked");
    git(
        main.path(),
        &["worktree", "add", "-q", linked.to_str().unwrap()],
    );
    assert!(
        linked.join(".git").is_file(),
        "the linked worktree's .git must be a file",
    );

    let out = run_setup(&linked, home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "`jigc setup` in a linked worktree must exit 0; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let installed = main.path().join(".git/hooks/pre-commit");
    assert!(
        installed.exists(),
        "a worktree install writes into the common hooks dir",
    );
    assert!(
        !linked.join(".git/hooks/pre-commit").exists(),
        "the worktree has no `.git/hooks` — the literal names nothing",
    );
    // Resolved against the **main checkout**, which is where the install landed and which
    // the ack names on its own site line (M53, LOW 5). The printed spelling moved with the
    // root — absolute while the door was pointed at the worktree, now `.git/hooks/pre-commit`
    // against jigc_home — and both spellings name the same file, which is the contract.
    assert_names_installed_hook(&stdout, main.path(), &installed);
    assert!(
        stdout.contains("not the worktree you are standing in"),
        "…and the ack must name the root that spelling is relative to; got:\n{stdout}",
    );
}

/// Run the built `jigc setup` under the machine surface — `--format json`, `cwd = repo`,
/// `$HOME = home`.
fn run_setup_json(repo: &Path, home: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["--format", "json", "setup"])
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The path the `--format json` install envelope names for the `pre-commit` hook — read
/// out of the emitted envelope itself, so the assertion drives the bytes a driver
/// deserializes and never a reconstruction of them.
fn json_hook_path(stdout: &str) -> String {
    let value: serde_json::Value = serde_json::from_str(stdout).unwrap_or_else(|err| {
        panic!("`jigc setup --format json` must emit one JSON object ({err}); got:\n{stdout}")
    });
    let hook = value.get("hook_file").unwrap_or_else(|| {
        panic!(
            "the install envelope must carry `hook_file` — the value the agent text \
             already names is computed and withheld from the driver; got:\n{stdout}",
        )
    });
    hook.as_str()
        .unwrap_or_else(|| panic!("`hook_file` must be a string; got:\n{stdout}"))
        .to_string()
}

/// The JSON twin of [`setup_names_the_core_hookspath_hook`]'s sibling text arm, on the
/// **default** hooks dir: the machine envelope names the hook the install wrote, so a
/// driver reads the same fact the agent text prints.
#[test]
fn setup_json_names_the_installed_hook() {
    let repo = TempDir::new("json-hook-default");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_setup_json(repo.path(), home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "`jigc setup --format json` must exit 0; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let installed = repo.path().join(".git/hooks/pre-commit");
    assert!(
        installed.exists(),
        "the default install writes the hook into `.git/hooks`",
    );
    assert_named_hook_is_installed(&json_hook_path(&stdout), repo.path(), &installed);
}

/// The JSON twin on the **`core.hooksPath`** shape — the shape that makes the `.git/hooks`
/// literal a lie. The envelope names that dir's `pre-commit`, exactly as the text does.
#[test]
fn setup_json_names_the_core_hookspath_hook() {
    let repo = TempDir::new("json-hook-hookspath");
    mark_repo(repo.path());
    let home = TempDir::new("home");
    let hooks = repo.path().join("my-hooks");
    git(
        repo.path(),
        &["config", "core.hooksPath", hooks.to_str().unwrap()],
    );

    let out = run_setup_json(repo.path(), home.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "`jigc setup --format json` must exit 0; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let installed = hooks.join("pre-commit");
    assert!(
        installed.exists(),
        "the install must write the hook into the core.hooksPath dir",
    );
    assert!(
        !repo.path().join(".git/hooks/pre-commit").exists(),
        "with core.hooksPath set nothing lands in .git/hooks — the literal names nothing",
    );
    assert_named_hook_is_installed(&json_hook_path(&stdout), repo.path(), &installed);
}

/// The hooks-dir shapes the install commit's pathspec must be correct over — the axis,
/// enumerated, not the one shape the defect was reported on.
#[derive(Clone, Copy, Debug)]
enum HooksDirShape {
    /// (1) the default `.git/hooks` — inside the repo, but in git's own control dir.
    Default,
    /// (2) an in-worktree `core.hooksPath`, set in its **relative** form — the hook is a
    /// tracked-able working-tree file.
    InWorktreeRelative,
    /// (3) a `core.hooksPath` pointing **outside** the repo — not committable at all.
    OutsideRepo,
    /// (4) a linked worktree, whose hooks resolve to the main checkout's **common** dir —
    /// outside the worktree git commits from.
    LinkedWorktree,
    /// (5) a `core.hooksPath` inside a **submodule** (shared hooks vendored as one) —
    /// under the root, outside git's dirs, and owned by the submodule's index: `git add`
    /// refuses it fatally, taking the whole install commit down with it.
    Submodule,
    /// (6) a `core.hooksPath` inside an **embedded, unregistered** git repo — the same
    /// ownership question with no `.gitmodules` entry: `git add` stages nothing at exit 0
    /// and the pathspec-limited `git commit` then fails on a path git does not know.
    EmbeddedRepo,
    /// (7) the same submodule as (5), **registered in the index but not checked out** —
    /// what a plain `git clone` (no `--recursive`) leaves behind, and the *common* shape:
    /// an empty directory with no `.git` inside, so the filesystem's ownership answer is
    /// "this repo" while the index still holds the `160000` gitlink and `git add` refuses
    /// the hook exactly as fatally as in (5).
    SubmoduleNotCheckedOut,
    /// (8) an in-worktree `core.hooksPath` **excluded by a sparse-checkout** — a hook that
    /// every location and ownership test calls committable and that `git add` refuses
    /// anyway. No test can promise git will take a path, so this cell is what pins the
    /// hook's **soft** membership: the refusal costs the hook's place in the pathspec,
    /// never the install commit.
    SparseCheckoutExcluded,
}

/// The install commit's footprint apart from the `pre-commit` hook: the eight paths
/// `install_tracked_paths` names on a repo with a born HEAD (the root `.gitignore`
/// secrets floor is seeded, and committed, only on a zero-commit repo). Sorted-set
/// compared, so the assertion is *exactly* this set — a path that silently joins the
/// install commit reddens here.
///
/// The eighth is the adapter's owned **guide artifact** (M48 Increment 10), at the path
/// the shipped Claude Code profile declares: it is committed for the same reason the
/// bootstrap file is — a clone must get the guides that match the binary that wrote them.
const INSTALL_COMMIT_BASE_PATHS: [&str; 8] = [
    ".claude/settings.json",
    ".claude/skills/jigc/SKILL.md",
    ".jigc/.gitignore",
    ".jigc/AGENT.md",
    ".jigc/config/.gitkeep",
    ".jigc/config/packs.yaml",
    ".jigc/version",
    "CLAUDE.md",
];

/// The distinguishing words of the summary clause that says the installed hook did **not**
/// ride the install commit. Written out here rather than imported from the renderer: this
/// suite's subject is the emitted bytes a reader sees, and a constant shared with the
/// producer would assert only that the producer equals itself.
const HOOK_LOCAL_ONLY: &str = "not in the install commit";

/// Run `git -C <root> <args>` with the ambient global/system config neutralized and
/// return its stdout (the capturing sibling of [`git`]).
fn git_capture(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .output()
        .unwrap_or_else(|err| panic!("run git {args:?}: {err}"));
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// `path`'s repo-root-relative form when it lives under `root`, else `None` — the
/// committability question git itself answers: a path outside the working tree cannot
/// be in any commit. Both sides canonicalized (macOS `/tmp` → `/private/tmp`).
fn repo_relative(root: &Path, path: &Path) -> Option<String> {
    let root = fs::canonicalize(root).expect("canonicalize the repo root");
    let real = fs::canonicalize(path).expect("canonicalize the path");
    real.strip_prefix(&root)
        .ok()
        .map(|rel| rel.display().to_string())
}

/// The install commit must carry the `pre-commit` hook **iff** the hook is a committable
/// working-tree file — over the whole hooks-dir axis, not the default shape the pathspec
/// was written for (M48 Increment 5 / F4).
///
/// For each shape: the resolved hook's membership in the install commit equals its
/// committability, nothing from inside git's control dir is ever staged, the rest of the
/// commit is exactly [`INSTALL_COMMIT_BASE_PATHS`] — so the eight jigc-owned paths land
/// in **one** install commit whatever the hooks dir turns out to be — a committable hook
/// is left **tracked** in the working tree, and a second `jigc setup` mints no second
/// commit (idempotency survives the widened pathspec).
///
/// The axis has three halves, and only the first is about *location*. **Ownership:** a
/// hooks dir can sit under the root, outside git's own dirs, and still belong to
/// **another** repository — checked out (shapes (5)/(6)) or, the common case, *not*
/// (shape (7), what a plain `git clone` leaves), where only the index can answer.
/// **Acceptance:** even ownership settled, no test promises git will take the path
/// (shape (8), a sparse-checkout excluding the hooks dir) — so the hook is a **soft**
/// member of the pathspec and a refusal it causes costs the hook, never the install
/// commit. Every non-committable cell therefore asserts the same thing the committable
/// ones do: `jigc setup` exits 0 and the eight jigc-owned paths land in one commit.
///
/// Declared bound, stated rather than engineered around: such a hook is installed and
/// reported but not committable *here*, so the working tree keeps it as the containing
/// repo's business — which is why the untracked-hook clause below is asserted exactly
/// where the hook is committable.
#[test]
fn setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file() {
    for shape in [
        HooksDirShape::Default,
        HooksDirShape::InWorktreeRelative,
        HooksDirShape::OutsideRepo,
        HooksDirShape::LinkedWorktree,
        HooksDirShape::Submodule,
        HooksDirShape::EmbeddedRepo,
        HooksDirShape::SubmoduleNotCheckedOut,
        HooksDirShape::SparseCheckoutExcluded,
    ] {
        let home = TempDir::new("home");
        let repo = TempDir::new("hook-axis");
        let outside = TempDir::new("outside-hooks");
        let hooks_source = TempDir::new("hooks-source");
        mark_repo(repo.path());
        git(
            repo.path(),
            &["commit", "-q", "--allow-empty", "-m", "init"],
        );

        // Wire the shape, then name where the hook must land, whether that path is under
        // the tree `jigc setup` commits in at all, and whether it is committable there.
        //
        // **The cwd is the axis; the committing tree is always `repo.path()`** (M53 — the
        // cwd fixes' review, LOW 5). `jigc setup` binds `repo::jigc_home`, so the install
        // and its commit land at the main checkout from every cwd — including
        // `LinkedWorktree`, whose cwd is a worktree that was *not* the install's subject.
        // Reading `git show HEAD` there, as this arm did, asked the wrong checkout: it came
        // back empty, which under the old binding was correct and is now the defect.
        let commit_root = repo.path().to_path_buf();
        let (cwd, hook, under_root, committable) = match shape {
            HooksDirShape::Default => (
                repo.path().to_path_buf(),
                repo.path().join(".git/hooks/pre-commit"),
                // Under the repo root, yet in git's control dir — the shape that makes a
                // relative-path discriminator look green while staging nothing.
                true,
                false,
            ),
            HooksDirShape::InWorktreeRelative => {
                git(repo.path(), &["config", "core.hooksPath", "my-hooks"]);
                (
                    repo.path().to_path_buf(),
                    repo.path().join("my-hooks/pre-commit"),
                    true,
                    true,
                )
            }
            HooksDirShape::OutsideRepo => {
                git(
                    repo.path(),
                    &[
                        "config",
                        "core.hooksPath",
                        outside.path().to_str().expect("utf-8 temp path"),
                    ],
                );
                (
                    repo.path().to_path_buf(),
                    outside.path().join("pre-commit"),
                    false,
                    false,
                )
            }
            HooksDirShape::LinkedWorktree => {
                let linked = repo.path().join("linked");
                git(
                    repo.path(),
                    &[
                        "worktree",
                        "add",
                        "-q",
                        linked.to_str().expect("utf-8 temp path"),
                    ],
                );
                // The hook resolves to the COMMON hooks dir — the main checkout's
                // `.git/hooks`. That is outside the linked worktree the command is *typed*
                // in, and inside the tree the install commits in, which since M53 is
                // jigc_home: under the root, and not committable, because it is inside
                // git's own control dir.
                (
                    linked,
                    repo.path().join(".git/hooks/pre-commit"),
                    true,
                    false,
                )
            }
            HooksDirShape::Submodule => {
                // A tiny repo with a `hooks/` dir, vendored into the repo as a submodule.
                mark_repo(hooks_source.path());
                fs::create_dir_all(hooks_source.path().join("hooks"))
                    .expect("create the source hooks dir");
                fs::write(hooks_source.path().join("hooks/keep"), "x\n")
                    .expect("seed the source hooks dir");
                git(hooks_source.path(), &["add", "-A"]);
                git(hooks_source.path(), &["commit", "-q", "-m", "hooks"]);
                git(
                    repo.path(),
                    &[
                        "-c",
                        "protocol.file.allow=always",
                        "submodule",
                        "add",
                        "-q",
                        hooks_source.path().to_str().expect("utf-8 temp path"),
                        "shared-hooks",
                    ],
                );
                git(repo.path(), &["commit", "-q", "-m", "add submodule"]);
                git(
                    repo.path(),
                    &["config", "core.hooksPath", "shared-hooks/hooks"],
                );
                (
                    repo.path().to_path_buf(),
                    repo.path().join("shared-hooks/hooks/pre-commit"),
                    true,
                    false,
                )
            }
            HooksDirShape::EmbeddedRepo => {
                git(repo.path(), &["init", "-q", "nested"]);
                git(repo.path(), &["config", "core.hooksPath", "nested/hooks"]);
                (
                    repo.path().to_path_buf(),
                    repo.path().join("nested/hooks/pre-commit"),
                    true,
                    false,
                )
            }
            HooksDirShape::SubmoduleNotCheckedOut => {
                // Shape (5), then de-checked-out. `submodule deinit` leaves exactly what a
                // `git clone` without `--recursive` leaves — an empty directory over a
                // gitlink in the index — without needing a second clone to build it.
                mark_repo(hooks_source.path());
                fs::create_dir_all(hooks_source.path().join("hooks"))
                    .expect("create the source hooks dir");
                fs::write(hooks_source.path().join("hooks/keep"), "x\n")
                    .expect("seed the source hooks dir");
                git(hooks_source.path(), &["add", "-A"]);
                git(hooks_source.path(), &["commit", "-q", "-m", "hooks"]);
                git(
                    repo.path(),
                    &[
                        "-c",
                        "protocol.file.allow=always",
                        "submodule",
                        "add",
                        "-q",
                        hooks_source.path().to_str().expect("utf-8 temp path"),
                        "shared-hooks",
                    ],
                );
                git(repo.path(), &["commit", "-q", "-m", "add submodule"]);
                git(
                    repo.path(),
                    &["submodule", "deinit", "-f", "--", "shared-hooks"],
                );
                // The premise, asserted: no `.git` for the filesystem to see, a gitlink
                // the index still holds. Blind the first and this cell is (5) again.
                assert!(
                    !repo.path().join("shared-hooks/.git").exists(),
                    "the deinit'd submodule must have no `.git`",
                );
                assert!(
                    git_capture(repo.path(), &["ls-files", "--stage", "--", "shared-hooks"])
                        .starts_with("160000 "),
                    "the index must still hold the submodule's gitlink",
                );
                git(
                    repo.path(),
                    &["config", "core.hooksPath", "shared-hooks/hooks"],
                );
                (
                    repo.path().to_path_buf(),
                    repo.path().join("shared-hooks/hooks/pre-commit"),
                    true,
                    false,
                )
            }
            HooksDirShape::SparseCheckoutExcluded => {
                git(repo.path(), &["config", "core.hooksPath", "my-hooks"]);
                // Cone mode always keeps root files (`CLAUDE.md`), so naming `.claude`
                // keeps the rest of the install footprint in the cone and leaves exactly
                // `my-hooks/` outside it.
                git(repo.path(), &["sparse-checkout", "init", "--cone"]);
                git(repo.path(), &["sparse-checkout", "set", ".claude", ".jigc"]);
                (
                    repo.path().to_path_buf(),
                    repo.path().join("my-hooks/pre-commit"),
                    true,
                    false,
                )
            }
        };

        let out = run_setup(&cwd, home.path());
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "{shape:?}: `jigc setup` must exit 0; stdout:\n{stdout}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            hook.exists(),
            "{shape:?}: the install must write the hook to `{}`",
            hook.display(),
        );

        // The install commit's actual footprint.
        let committed: Vec<String> = {
            let mut lines: Vec<String> =
                git_capture(&commit_root, &["show", "--name-only", "--format=", "HEAD"])
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(str::to_string)
                    .collect();
            lines.sort();
            lines
        };

        // Nothing from inside git's own control dir is ever staged — a `.git`-internal
        // pathspec entry stages nothing and would look green (the trap the relative-path
        // discriminator falls into).
        assert!(
            !committed.iter().any(|p| p.starts_with(".git/")),
            "{shape:?}: nothing inside `.git/` may be committed; got:\n{committed:#?}",
        );

        // Membership == committability, over the resolved hook path — spelled against the
        // tree the install commits in, never against the cwd it was typed from.
        let relative_hook = repo_relative(&commit_root, &hook);
        assert_eq!(
            relative_hook.is_some(),
            under_root,
            "{shape:?}: the fixture's own premise about where the hook landed is wrong \
             (hook `{}`, committing tree `{}`)",
            hook.display(),
            commit_root.display(),
        );
        if let Some(relative) = relative_hook.as_deref() {
            assert_eq!(
                committed.iter().any(|p| p == relative),
                committable,
                "{shape:?}: the hook `{relative}` must be in the install commit iff it is \
                 committable ({committable}); the commit carries:\n{committed:#?}",
            );
        }

        // …and the door SAYS which of the two happened. The summary lists the hook under
        // "setup installed:" on every shape, which is true — it is installed, locally —
        // so the lie a reader can be told here is by omission: on a shape where the hook
        // did not ride the install commit, nothing said so, and the default `.git/hooks`
        // is the shape almost every adopter is on. The clause is keyed on the pathspec the
        // commit was actually made from (`committable_hook_path`'s own answer, plus the
        // soft-member drop of shape (8)) — never on a re-test of the path shape, so the
        // sentence and the commit cannot disagree.
        assert_eq!(
            stdout.contains(HOOK_LOCAL_ONLY),
            !committable,
            "{shape:?}: the summary must carry the local-only clause exactly when the \
             install commit does NOT carry the hook (committable={committable}); \
             summary:\n{stdout}",
        );

        // …and the rest of the commit is exactly today's set.
        let mut expected: Vec<String> = INSTALL_COMMIT_BASE_PATHS
            .iter()
            .map(|p| (*p).to_string())
            .collect();
        if committable {
            expected.push(
                relative_hook
                    .clone()
                    .expect("a committable hook is in-tree"),
            );
        }
        expected.sort();
        assert_eq!(
            committed, expected,
            "{shape:?}: the install commit must carry exactly the install footprint",
        );

        // No COMMITTABLE hook is left behind in the working tree (RED before the fix for
        // shape (2), where `git status --porcelain` reports the collapsed `?? my-hooks/`).
        // Compared as a path PREFIX, precisely because git collapses a wholly-untracked
        // directory to its name — matching the full `my-hooks/pre-commit` would miss the
        // defect. Asserted where the hook is committable: shapes (5)/(6) install a hook
        // this repo *cannot* track, and status honestly reports the containing submodule /
        // embedded repo — the declared bound, not a defect to assert away.
        let porcelain = git_capture(&cwd, &["status", "--porcelain"]);
        if let Some(relative) = relative_hook.as_deref().filter(|_| committable) {
            // A `.git`-internal hook is never reported by `status` at all; an in-worktree
            // one must be tracked and clean.
            let covering = porcelain
                .lines()
                .filter_map(|line| line.get(3..))
                .find(|reported| relative == *reported || relative.starts_with(*reported));
            assert!(
                covering.is_none(),
                "{shape:?}: the hook `{relative}` is left uncommitted — \
                 `git status --porcelain` reports `{}`; full status:\n{porcelain}",
                covering.unwrap_or_default(),
            );
        }

        // Idempotency survives the widened pathspec: a second `setup` regenerates a
        // byte-identical hook, so there is no net change and no second commit.
        let head_before = git_capture(&cwd, &["rev-parse", "HEAD"]).trim().to_string();
        let again = run_setup(&cwd, home.path());
        assert!(
            again.status.success(),
            "{shape:?}: a second `jigc setup` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&again.stderr),
        );
        assert_eq!(
            git_capture(&cwd, &["rev-parse", "HEAD"]).trim(),
            head_before,
            "{shape:?}: a second `jigc setup` must mint no second install commit",
        );
        // The clause survives the re-run, which mints nothing: it states where the hook
        // *is*, not what this particular run committed — a reader who runs `setup` twice
        // must not watch the warning disappear off a hook that is still uncommittable.
        assert_eq!(
            String::from_utf8_lossy(&again.stdout).contains(HOOK_LOCAL_ONLY),
            !committable,
            "{shape:?}: the re-run's summary must say the same thing about the hook; \
             summary:\n{}",
            String::from_utf8_lossy(&again.stdout),
        );
    }
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
