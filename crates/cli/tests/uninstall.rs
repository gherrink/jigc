//! End-to-end integration test for `jigc uninstall` — the repo-local teardown
//! (M21 Increment 4, T2; `design/project-setup.md` → Flow 2 hardening → Teardown /
//! cleanup (G5), bullet (b)).
//!
//! Drives the **built** `jigc` binary against a throwaway temp repo. `jigc setup`
//! installs the repo-local footprint (`.jigc/`, the `CLAUDE.md` `@.jigc/AGENT.md`
//! import line, the `.claude/settings.json` `Bash(jigc:*)` permit, the
//! `compose-embedded-methodology` marker) **and** the one machine-global write — the
//! `doc-code` probe sibling beside the `jigc` binary. `jigc uninstall` then reverses
//! **exactly the repo-local set**, leaving the probe sibling intact (design-review
//! B2: the probe is shared across every repo on the machine — deleting it would break
//! `jigc validate` for sibling repos).
//!
//! Asserts the Deliverable's done-picture: (i) `.jigc/` is gone; (ii) `CLAUDE.md` no
//! longer carries `@.jigc/AGENT.md` with the pre-existing house-rules prose preserved
//! **byte-for-byte**; (iii) `permissions.allow` no longer carries `Bash(jigc:*)` with
//! unrelated keys preserved and the file still valid JSON; (iv) the machine-global
//! `<bin-dir>/doc-code` probe **STILL exists** (the B2 guarantee — the
//! omits-the-target face of the hardening rule); (v) a second `jigc uninstall` is a
//! clean no-op (exit 0).
//!
//! The probe-survives assertion is driven against a **copied** `jigc` in a fresh bin
//! dir so `current_exe().parent()` is that dir — the same apparatus `tests/setup.rs`'s
//! probe-extract acceptance uses.
//!
//! No external test crates: the binary path comes from Cargo's `CARGO_BIN_EXE_jigc`,
//! the temp repo is built with `std::fs`, and a self-cleaning `TempDir` keeps the test
//! off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The adapter's owned guide artifact — a repo-local file `setup` created (M48 Increment
/// 10), so it is an ordinary member of the enumerated set this teardown reverses. The
/// ownership question it is removed *under* (jigc's own copy goes, a user-modified one
/// stays) is driven over its whole axis by `tests/adapter_artifact.rs`; here it is one more
/// artifact the summary must report honestly.
const GUIDE_PATH: &str = ".claude/skills/jigc/SKILL.md";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-uninstall-{tag}-{}-{:?}",
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

/// Make `root` a real git repo (`jigc setup` resolves the repo's real hooks dir via
/// git, so a bare `.git` marker no longer suffices).
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
    configure_identity(root);
}

/// `jigc setup` commits its own install footprint (M30 audit finding 1) — even on an
/// unborn HEAD it mints the repo's first commit — so a test repo needs a usable identity
/// and signing off for that commit to land deterministically.
fn configure_identity(root: &Path) {
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

/// Run a specific `jigc` binary with `cwd = repo` and `$HOME = home`.
fn run(jigc: &Path, repo: &Path, home: &Path, verb: &str) -> std::process::Output {
    Command::new(jigc)
        .arg(verb)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Make `path` owner-executable (`0o755`).
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path).expect("stat for chmod").permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).expect("chmod");
}

/// The full Deliverable, driven against the real built binary. `setup` writes the
/// repo-local footprint + the machine-global probe sibling; `uninstall` reverses
/// exactly the repo-local set and leaves the probe intact; a second `uninstall` is a
/// clean no-op.
#[test]
fn uninstall_removes_repo_local_footprint_keeps_probe_and_is_idempotent() {
    // A probe-less install dir: copy ONLY the `jigc` binary into it, so `setup`
    // extracts the `doc-code` sibling beside it and `current_exe().parent()` is this
    // dir (mirroring a `cargo install`). This lets us assert the probe SURVIVES
    // uninstall (the B2 guarantee).
    let bin = TempDir::new("bin");
    let jigc = bin.path().join("jigc");
    fs::copy(env!("CARGO_BIN_EXE_jigc"), &jigc).expect("copy the built jigc into a probe-less dir");
    make_executable(&jigc);
    let probe = bin.path().join("doc-code");

    let repo = TempDir::new("repo");
    mark_repo(repo.path());
    let home = TempDir::new("home");

    // Seed a non-trivial CLAUDE.md the human owns — uninstall must restore it
    // byte-for-byte after stripping jigc's appended `## Project interface` section.
    let seeded_claude = "# House rules\n\n\
        - Always run the linter before committing.\n\
        - Prefer small, focused PRs.\n\n\
        ## Architecture\n\n\
        The gateway owns rate limiting; do not duplicate it downstream.\n";
    fs::write(repo.path().join("CLAUDE.md"), seeded_claude).expect("seed CLAUDE.md");

    // Seed a .claude/settings.json with an unrelated top-level key, an unrelated
    // permit, and a FOREIGN SessionStart hook — uninstall must drop only the `Bash(jigc:*)`
    // permit and jigc's own SessionStart command, keeping everything else (surgical,
    // not a clobber).
    fs::create_dir_all(repo.path().join(".claude")).expect("seed .claude dir");
    let seeded_settings = serde_json::json!({
        "model": "claude-sonnet-4",
        "permissions": { "allow": ["git status"] },
        "hooks": {
            "SessionStart": [
                { "hooks": [ { "type": "command", "command": "my-own-tool --greet" } ] }
            ]
        },
    });
    fs::write(
        repo.path().join(".claude/settings.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&seeded_settings).unwrap()
        ),
    )
    .expect("seed .claude/settings.json");

    // Install.
    let setup = run(&jigc, repo.path(), home.path(), "setup");
    assert!(
        setup.status.success(),
        "`jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    // Sanity: setup wrote the footprint we're about to tear down.
    assert!(repo.path().join(".jigc").is_dir(), "setup writes .jigc/");
    assert!(
        probe.exists(),
        "setup extracts the machine-global doc-code probe"
    );
    let claude_after_setup =
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md present");
    assert!(
        claude_after_setup.contains("@.jigc/AGENT.md"),
        "setup wires the @.jigc/AGENT.md import; got:\n{claude_after_setup}",
    );
    let probe_bytes = fs::read(&probe).expect("read the extracted probe");
    assert!(
        repo.path().join(GUIDE_PATH).is_file(),
        "setup installs the adapter's owned guide artifact",
    );

    // Uninstall.
    let out = run(&jigc, repo.path(), home.path(), "uninstall");
    assert!(
        out.status.success(),
        "`jigc uninstall` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // (0) Honesty of the teardown summary: it must enumerate EVERY artifact it
    //     actually removed — not just the original three. This repo had a full setup,
    //     so the SessionStart hook, the deny safety floor, and the pre-commit hook were
    //     all present and removed; the summary must say so (M36 completion: the runtime
    //     summary must match the real removal set, mirroring the design honesty fix).
    let summary = String::from_utf8_lossy(&out.stdout);
    for needle in [
        "removed .jigc/",
        "SessionStart hook",
        "deny safety floor",
        "pre-commit hook",
        // The seventh artifact (M48 Increment 10): setup wrote it, so the teardown takes
        // it out and — by the same honesty rule — says so on the text surface too.
        "removed jigc guide artifact",
    ] {
        assert!(
            summary.contains(needle),
            "the uninstall summary must report `{needle}`; got:\n{summary}",
        );
    }

    // (i) .jigc/ is gone.
    assert!(
        !repo.path().join(".jigc").exists(),
        "uninstall must remove the repo-local .jigc/ tree",
    );

    // (ii) CLAUDE.md no longer carries the import line, and the pre-existing
    //      house-rules content is restored byte-for-byte.
    let claude = fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md present");
    assert!(
        !claude.contains("@.jigc/AGENT.md"),
        "uninstall must unwire the @.jigc/AGENT.md import line; got:\n{claude}",
    );
    assert!(
        !claude.contains("## Project interface"),
        "uninstall must remove the jigc-injected `## Project interface` section; got:\n{claude}",
    );
    assert_eq!(
        claude, seeded_claude,
        "uninstall must restore the pre-existing CLAUDE.md content byte-for-byte",
    );

    // (iii) permissions.allow no longer carries `Bash(jigc:*)`; the unrelated permit + the
    //       unrelated top-level key survive, and the file is still valid JSON.
    let settings_raw = fs::read_to_string(repo.path().join(".claude/settings.json"))
        .expect(".claude/settings.json present");
    let settings: serde_json::Value =
        serde_json::from_str(&settings_raw).expect(".claude/settings.json must stay valid JSON");
    let allow = settings["permissions"]["allow"]
        .as_array()
        .expect("permissions.allow is an array");
    assert!(
        !allow.iter().any(|v| v == "Bash(jigc:*)"),
        "uninstall must drop the `Bash(jigc:*)` permit; got:\n{settings_raw}",
    );
    assert!(
        allow.iter().any(|v| v == "git status"),
        "the unrelated permit must survive uninstall; got:\n{settings_raw}",
    );
    assert_eq!(
        settings["model"], "claude-sonnet-4",
        "an unrelated top-level key must survive uninstall; got:\n{settings_raw}",
    );

    // (iii-b) The SessionStart hook is torn down surgically: jigc's `jigc start`
    //         command is gone, but the FOREIGN SessionStart hook survives.
    let session = settings["hooks"]["SessionStart"]
        .as_array()
        .expect("SessionStart stays an array");
    let has_command = |cmd: &str| {
        session.iter().any(|matcher| {
            matcher["hooks"]
                .as_array()
                .is_some_and(|inner| inner.iter().any(|c| c["command"].as_str() == Some(cmd)))
        })
    };
    assert!(
        !has_command("jigc start"),
        "uninstall must drop jigc's SessionStart command; got:\n{settings_raw}",
    );
    assert!(
        has_command("my-own-tool --greet"),
        "the foreign SessionStart hook must survive uninstall; got:\n{settings_raw}",
    );

    // (iii-d) The adapter's owned guide artifact is torn down, and the directory jigc
    //         created to hold it goes with it once it empties — but `.claude/` itself is
    //         the user's own directory (it holds the settings file uninstall just edited
    //         surgically), so it stays standing.
    assert!(
        !repo.path().join(GUIDE_PATH).exists(),
        "uninstall must remove the adapter's owned guide artifact",
    );
    assert!(
        !repo.path().join(".claude/skills").exists(),
        "the skill directory jigc created goes with the artifact once it empties",
    );
    assert!(
        repo.path().join(".claude").is_dir() && repo.path().join(".claude/settings.json").is_file(),
        "uninstall must never take `.claude/` itself — it is the user's own directory",
    );

    // (iii-c) The jigc-managed pre-commit hook is torn down: this repo had no
    //         pre-existing hook, so setup wrote a standalone jigc hook and uninstall
    //         removes it entirely — no residue firing against a removed install.
    assert!(
        !repo.path().join(".git/hooks/pre-commit").exists(),
        "uninstall must remove the standalone jigc pre-commit hook",
    );

    // (iv) The machine-global doc-code probe STILL exists, byte-identical (B2): a
    //      sibling repo's `jigc validate` must still resolve a runnable probe.
    assert!(
        probe.exists(),
        "uninstall must NOT remove the machine-global doc-code probe (B2)",
    );
    assert_eq!(
        fs::read(&probe).expect("probe still present"),
        probe_bytes,
        "the machine-global doc-code probe must be left byte-identical",
    );

    // (v) A second uninstall is a clean no-op (exit 0).
    let out2 = run(&jigc, repo.path(), home.path(), "uninstall");
    assert!(
        out2.status.success(),
        "a second `jigc uninstall` must exit 0 as a clean no-op; stderr:\n{}",
        String::from_utf8_lossy(&out2.stderr),
    );
    // The repo-local footprint stays gone; the human content is untouched; the probe
    // survives the no-op too.
    assert!(
        !repo.path().join(".jigc").exists(),
        "second uninstall leaves .jigc/ gone"
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("CLAUDE.md")).expect("CLAUDE.md present"),
        seeded_claude,
        "a second uninstall leaves the restored CLAUDE.md untouched",
    );
    assert!(probe.exists(), "a second uninstall leaves the probe intact");

    // (v-b) The second uninstall removed nothing — its summary must NOT claim to have
    //       torn down artifacts that were already absent ("don't claim to remove what
    //       wasn't there"): a no-op reports a clean "nothing to remove" state.
    let summary2 = String::from_utf8_lossy(&out2.stdout);
    assert!(
        !summary2.contains("removed .jigc/")
            && !summary2.contains("pre-commit hook")
            && !summary2.contains("guide artifact"),
        "a no-op uninstall must not claim removals it did not make; got:\n{summary2}",
    );
}
