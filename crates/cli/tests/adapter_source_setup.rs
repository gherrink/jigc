//! End-to-end proof of the `JIGC_ADAPTERS_DIR` adapter-profile-source seam — the
//! adapter analogue of the `JIGC_PACK_DIR` pack-source seam
//! (`DECISIONS.md` 2026-06-05). It makes the install-time spawn-template
//! **rejection** path binary-observable: the shipped binary embeds only the valid
//! `claude-code.yaml`, so without this seam only the ACCEPT path is reachable
//! through `jigc setup`. With `JIGC_ADAPTERS_DIR=<dir>` set, `load_profile` reads
//! `<dir>/claude-code.yaml` from disk, so a deliberately-broken spawn template can
//! be driven through the real install and observed to reject — the gate
//! (`crate::setup::install` step 0) runs **before any host write**.
//!
//! Three behaviors, all over the built `jigc` binary (`CARGO_BIN_EXE_jigc`):
//!   - (reject) a profile whose `spawn.template` violates the decidable rule fails
//!     install with the blocking `setup.spawn-template` finding routed to the
//!     violated clause, exits non-zero, and writes **no** host files. Two distinct
//!     violation kinds are covered so the rule is binary-exercised, not one branch.
//!   - (accept) a profile with a VALID spawn template installs clean (exit 0).
//!   - (determinism) with the env **unset**, the shipped embedded profile installs
//!     exactly as today — byte-identical CLAUDE.md / .jigc/AGENT.md /
//!     .claude/settings.json against the override-accept install.
//!
//! No external test crates: the temp repo is built with `std::fs` and a
//! self-cleaning `TempDir`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-adapters-{tag}-{}-{:?}",
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
/// suffices — the install runs `git rev-parse` against an actual repo (as the real
/// `jigc setup` always does).
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
}

/// Write a `claude-code.yaml` adapter profile into a fresh directory whose
/// `spawn.template` is `template`, leaving every other field exactly as the
/// shipped profile. Returns the directory `JIGC_ADAPTERS_DIR` points at.
fn adapters_dir_with_template(tag: &str, template: &str) -> TempDir {
    let dir = TempDir::new(tag);
    let yaml = format!(
        "assistant: claude-code\n\
         inject:\n\
        \x20 - reference: {{ file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }}\n\
        \x20 - hook: {{ event: SessionStart, run: \"jigc start\" }}\n\
         allowlist:\n\
        \x20 file: .claude/settings.json\n\
        \x20 permit: [\"jigc *\"]\n\
         spawn:\n\
        \x20 template: {template:?}\n",
    );
    fs::write(dir.path().join("claude-code.yaml"), yaml).expect("write override profile");
    dir
}

/// Run the built `jigc setup` binary with `cwd = repo`, `$HOME = home`, and an
/// optional `JIGC_ADAPTERS_DIR` override (passed through unchanged when `None`,
/// i.e. inherited/absent — the determinism path).
fn run_setup(repo: &Path, home: &Path, adapters_dir: Option<&Path>) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_jigc"));
    cmd.arg("setup")
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_ADAPTERS_DIR");
    if let Some(dir) = adapters_dir {
        cmd.env("JIGC_ADAPTERS_DIR", dir);
    }
    cmd.output().expect("run the jigc binary")
}

/// The host files the install writes — none of which may exist after a rejected
/// install (the gate runs before any write).
fn host_files(repo: &Path) -> [PathBuf; 4] {
    [
        repo.join("CLAUDE.md"),
        repo.join(".jigc/AGENT.md"),
        repo.join(".claude/settings.json"),
        repo.join(".jigc/config/.gitkeep"),
    ]
}

/// Drive a broken `template` through the real `jigc setup` over the
/// `JIGC_ADAPTERS_DIR` seam and assert: non-zero exit, the blocking
/// `setup.spawn-template` finding on stderr, the `clause_pointer` (the violated
/// clause's route) present, and **no** host file written (gate-before-write).
fn assert_rejected(tag: &str, template: &str, clause_pointer: &str) {
    let repo = TempDir::new(tag);
    mark_repo(repo.path());
    let home = TempDir::new("home");
    let adapters = adapters_dir_with_template("profile", template);

    let out = run_setup(repo.path(), home.path(), Some(adapters.path()));
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        !out.status.success(),
        "[{tag}] a broken spawn template must fail install (non-zero exit); got {:?}\nstderr:\n{stderr}",
        out.status,
    );
    assert!(
        stderr.contains("setup.spawn-template"),
        "[{tag}] the blocking finding must carry the `setup.spawn-template` code; got stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(clause_pointer),
        "[{tag}] the finding must route to the violated clause ({clause_pointer:?}); got stderr:\n{stderr}",
    );
    for file in host_files(repo.path()) {
        assert!(
            !file.exists(),
            "[{tag}] a rejected install must write NO host file — the gate runs before any write; \
             but {} exists",
            file.display(),
        );
    }
}

/// (reject) Two distinct violation kinds, each driven through the binary, each
/// failing install with the clause-specific route and touching no host file — so
/// the decidable rule is binary-exercised on more than one branch.
#[test]
fn broken_spawn_template_rejects_install_through_the_binary() {
    // Kind 1 — missing the `{{task_id}}` placeholder (clause 1).
    assert_rejected(
        "missing-task-id",
        "Use your Task tool to run: `jigc workflow {{workflow}} --task X`",
        "missing the required `{{task_id}}` placeholder",
    );

    // Kind 2 — two newlines (clause 3a), a structurally different violation.
    assert_rejected(
        "too-many-newlines",
        "Use your Task tool to run:\n`jigc workflow {{workflow}} --task {{task_id}}`\nagain",
        "contains more than one newline",
    );
}

/// (accept) A VALID spawn template driven through the seam installs clean (exit 0,
/// no `setup.spawn-template` finding) — the ACCEPT path over the override, proving
/// the seam is inert for a conformant profile.
#[test]
fn valid_spawn_template_over_the_seam_installs_clean() {
    let repo = TempDir::new("accept");
    mark_repo(repo.path());
    let home = TempDir::new("home");
    let adapters = adapters_dir_with_template(
        "profile",
        "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`",
    );

    let out = run_setup(repo.path(), home.path(), Some(adapters.path()));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "a valid spawn template over the seam must exit 0; got {:?}\nstderr:\n{stderr}",
        out.status,
    );
    assert!(
        !stderr.contains("setup.spawn-template"),
        "a valid template must not trip the install-time gate; got stderr:\n{stderr}",
    );
    assert!(
        repo.path().join(".claude/settings.json").exists(),
        "a clean install must write the allowlist settings file",
    );
}

/// (determinism) With `JIGC_ADAPTERS_DIR` **unset**, the shipped embedded profile
/// installs exactly as a seam-driven install over a byte-faithful copy of that
/// same profile — the three host artifacts are byte-identical. This is the
/// determinism guard: the seam is inert when unset, so it cannot perturb today's
/// output.
#[test]
fn unset_env_installs_byte_identically_to_the_embedded_profile() {
    // The embedded profile's exact spawn template (`adapters/claude-code.yaml`).
    let shipped_template =
        "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`";

    // (a) Override path: a faithful copy of the shipped profile via the seam.
    let repo_env = TempDir::new("det-env");
    mark_repo(repo_env.path());
    let home_env = TempDir::new("home");
    let adapters = adapters_dir_with_template("profile", shipped_template);
    let out_env = run_setup(repo_env.path(), home_env.path(), Some(adapters.path()));
    assert!(
        out_env.status.success(),
        "the seam over a faithful profile copy must exit 0; got {:?}\nstderr:\n{}",
        out_env.status,
        String::from_utf8_lossy(&out_env.stderr),
    );

    // (b) No-env path: env unset, the binary-embedded profile.
    let repo_unset = TempDir::new("det-unset");
    mark_repo(repo_unset.path());
    let home_unset = TempDir::new("home");
    let out_unset = run_setup(repo_unset.path(), home_unset.path(), None);
    assert!(
        out_unset.status.success(),
        "the no-env embedded install must exit 0; got {:?}\nstderr:\n{}",
        out_unset.status,
        String::from_utf8_lossy(&out_unset.stderr),
    );

    // The three managed host artifacts are byte-identical across the two paths.
    for rel in ["CLAUDE.md", ".jigc/AGENT.md", ".claude/settings.json"] {
        let via_env = fs::read(repo_env.path().join(rel)).expect("override install wrote the file");
        let via_unset =
            fs::read(repo_unset.path().join(rel)).expect("no-env install wrote the file");
        assert_eq!(
            via_env, via_unset,
            "with JIGC_ADAPTERS_DIR unset, `{rel}` must be byte-identical to the seam-over-copy install",
        );
    }
}
