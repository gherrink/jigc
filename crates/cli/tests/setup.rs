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

/// The M20 probe-extract acceptance: a `cargo install`-style install (the `jigc`
/// binary alone in a **probe-less** bin dir, no build-tree `doc-code` sibling, no
/// `JIGC_DOC_CODE_PROBE` override) has `jigc setup` place a runnable `doc-code`
/// sibling beside it, applying the pinned heal/upgrade policy (write if absent or if
/// the existing sibling's bytes differ from the embedded copy)
/// (`module-layout.md` → Probe distribution, M20). Driven against the real built
/// binary copied into a fresh dir so `current_exe().parent()` is that dir.
///
/// Asserts: (i) setup writes an executable `doc-code` sibling whose bytes equal the
/// embedded copy and which runs; (ii) a second setup is a no-op when the sibling
/// already matches; (iii) a sibling with different bytes is **healed** — overwritten
/// with the embedded copy (heal/upgrade); (iv) an unwritable target dir yields exactly
/// one `setup.*` operational error, not a panic.
#[test]
fn setup_extracts_runnable_doc_code_probe_into_probe_less_bin_dir() {
    // A probe-less install dir: copy ONLY the `jigc` binary into it (no sibling
    // `doc-code`), mirroring a `cargo install` that relocates only the `[[bin]]`.
    let bin = TempDir::new("probe-extract-bin");
    let jigc = bin.path().join("jigc");
    fs::copy(env!("CARGO_BIN_EXE_jigc"), &jigc).expect("copy the built jigc into a probe-less dir");
    make_executable(&jigc);
    let probe = bin.path().join("doc-code");
    assert!(
        !probe.exists(),
        "the fresh install dir must start with no doc-code sibling",
    );

    let repo = TempDir::new("probe-extract-repo");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    // (i) First setup writes an executable doc-code sibling = the embedded bytes.
    let out = run_copied_setup(&jigc, repo.path(), home.path());
    assert!(
        out.status.success(),
        "`jigc setup` from a probe-less install must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(probe.exists(), "setup must extract a doc-code sibling");
    assert_eq!(
        mode(&probe) & 0o100,
        0o100,
        "the extracted doc-code probe must be owner-executable",
    );
    let extracted = fs::read(&probe).expect("read the extracted probe");
    // The extracted bytes are a native executable (ELF / Mach-O magic) — the embedded
    // copy that rode in the `jigc` binary, not an empty placeholder. (Byte-equality to
    // the in-process embedded slice is unit-asserted in `setup.rs`; the build-tree
    // sibling is not a stable comparand — it can be re-copied by any later `cli`
    // rebuild after the `jigc` under test was compiled.)
    assert!(
        extracted.starts_with(&[0x7f, b'E', b'L', b'F'])
            || matches!(
                extracted.get(..4),
                Some([0xFE, 0xED, 0xFA, 0xCE])
                    | Some([0xCE, 0xFA, 0xED, 0xFE])
                    | Some([0xFE, 0xED, 0xFA, 0xCF])
                    | Some([0xCF, 0xFA, 0xED, 0xFE])
                    | Some([0xCA, 0xFE, 0xBA, 0xBE])
                    | Some([0xBE, 0xBA, 0xFE, 0xCA])
            ),
        "the extracted probe must be a native executable; first bytes {:02x?}",
        &extracted[..extracted.len().min(4)],
    );
    // …and it RUNS: the probe reads a JSON request on stdin and emits JSON. An empty
    // request is malformed, but a runnable probe still *starts* and exits — a missing
    // ELF interpreter or a non-executable would fail to spawn at all. Spawning and
    // getting any exit status proves the extracted bytes are a runnable executable.
    let ran = Command::new(&probe)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    assert!(
        ran.is_ok(),
        "the extracted doc-code probe must be a runnable executable; spawn failed: {:?}",
        ran.err(),
    );

    // (ii) A second setup is a no-op: the sibling already matches → unchanged.
    let out2 = run_copied_setup(&jigc, repo.path(), home.path());
    assert!(out2.status.success(), "second setup must exit 0");
    assert_eq!(
        fs::read(&probe).expect("probe still present"),
        extracted,
        "a second setup over a matching sibling must leave it byte-identical",
    );

    // (iii) A pre-existing DIFFERENT sibling (stale upgrade leftover / corrupt stub)
    //       is HEALED — overwritten with the embedded copy and made executable.
    let sentinel = b"#!/bin/sh\n# a stale / corrupt leftover probe\nexit 0\n";
    fs::write(&probe, sentinel).expect("seed a different sibling");
    make_executable(&probe);
    let out3 = run_copied_setup(&jigc, repo.path(), home.path());
    assert!(
        out3.status.success(),
        "setup over a different sibling must exit 0"
    );
    assert_eq!(
        fs::read(&probe).expect("the healed sibling is present"),
        extracted,
        "a pre-existing byte-different doc-code sibling must be healed (overwritten with \
         the embedded copy)",
    );
    assert_eq!(
        mode(&probe) & 0o100,
        0o100,
        "the healed probe must be owner-executable",
    );

    // (iv) An unwritable target dir yields exactly one `setup.*` operational error,
    //      not a panic. Remove the sibling first so the write is actually attempted,
    //      then make the bin dir read-only so the write fails.
    fs::remove_file(&probe).expect("remove the sibling so a write is attempted");
    set_dir_readonly(bin.path(), true);
    let out4 = run_copied_setup(&jigc, repo.path(), home.path());
    set_dir_readonly(bin.path(), false); // restore so TempDir can clean up
    assert!(
        !out4.status.success(),
        "an unwritable probe target must fail the install (non-zero exit)",
    );
    let stderr4 = String::from_utf8_lossy(&out4.stderr);
    assert!(
        stderr4.contains("setup.extract-probe"),
        "an unwritable probe target must surface a `setup.extract-probe` finding, not a \
         panic; got stderr:\n{stderr4}",
    );
    assert!(
        !stderr4.contains("panicked"),
        "the unwritable target must NOT panic; got stderr:\n{stderr4}",
    );
}

/// Run the **copied** `jigc setup` (a specific binary path, not `CARGO_BIN_EXE_jigc`)
/// with `cwd = repo` and `$HOME = home`, so `current_exe()` resolves to the
/// probe-less install dir the extract step writes the probe into.
fn run_copied_setup(jigc: &Path, repo: &Path, home: &Path) -> std::process::Output {
    Command::new(jigc)
        .arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the copied jigc binary")
}

/// Make `path` owner-executable (`0o755`) — used both for the copied `jigc` and a
/// seeded sibling probe.
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path).expect("stat for chmod").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

/// Toggle a directory between read-only (`0o555`) and writable (`0o755`) so the
/// extract step's write fails on an unwritable target.
fn set_dir_readonly(dir: &Path, readonly: bool) {
    use std::os::unix::fs::PermissionsExt;
    let mode = if readonly { 0o555 } else { 0o755 };
    let mut perms = fs::metadata(dir).expect("stat dir for chmod").permissions();
    perms.set_mode(mode);
    fs::set_permissions(dir, perms).expect("chmod dir");
}

/// The permission bits of `path`, masked to the low 9 bits.
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .expect("stat for mode")
        .permissions()
        .mode()
        & 0o777
}

/// `jigc setup` writes the `compose-embedded-methodology: true` marker into the
/// project layer's `.jigc/config/packs.yaml` — the marker the pack factory
/// (`pack::read_compose_marker`) reads to compose the embedded methodology pack, so
/// a clean `setup` gives a real project the dev+methodology surface out of the box
/// (M21). The write is repo-local, idempotent, and non-destructive:
///   (i) after setup, `packs.yaml` carries `compose-embedded-methodology: true`;
///   (ii) a second setup leaves the file byte-identical (idempotent no-op);
///   (iii) a pre-seeded `packs:` list survives setup, with the marker added.
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

    // (iii) A pre-seeded `packs:` list survives setup (non-destructive), with the
    //       marker added alongside it.
    let seeded_repo = TempDir::new("compose-marker-seeded");
    mark_repo(seeded_repo.path());
    let seeded_config = seeded_repo.path().join(".jigc/config");
    fs::create_dir_all(&seeded_config).expect("create the seeded project config dir");
    fs::write(
        seeded_config.join("packs.yaml"),
        "packs:\n  - packs/local-pack\n",
    )
    .expect("seed a hand-written packs.yaml carrying a packs: list");

    let out3 = run_setup(seeded_repo.path(), home.path());
    assert!(
        out3.status.success(),
        "`jigc setup` over a seeded packs.yaml must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out3.stderr),
    );
    let seeded_after = fs::read_to_string(seeded_config.join("packs.yaml"))
        .expect("the seeded packs.yaml is present after setup");
    let seeded_parsed: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&seeded_after).expect("the post-setup seeded packs.yaml is valid");
    assert_eq!(
        seeded_parsed
            .get("compose-embedded-methodology")
            .and_then(serde_yaml_ng::Value::as_bool),
        Some(true),
        "setup must add the marker to a pre-seeded packs.yaml; got:\n{seeded_after}",
    );
    let listed = seeded_parsed
        .get("packs")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .expect("the pre-seeded `packs:` list must survive setup (non-destructive)");
    assert!(
        listed
            .iter()
            .any(|p| p.as_str() == Some("packs/local-pack")),
        "the pre-seeded pack entry must survive setup; got:\n{seeded_after}",
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
