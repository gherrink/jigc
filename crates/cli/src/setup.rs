//! `jigc setup` — the adapter install handler.
//!
//! Orchestrates the MVP adapter install (`design/assistant-adapter.md` →
//! Generated, minimal, regenerated; `DECISIONS.md` 2026-05-31 → adapter install
//! reworked) against the located repo root, using the embedded Claude Code
//! profile:
//!   1. write the canonical bootstrap sentence to the managed `.jigc/AGENT.md`
//!      and inject a bare `@.jigc/AGENT.md` import into `CLAUDE.md` (the
//!      reference floor — no marker-fenced block);
//!   2. initialize the project cascade layer (`.jigc/config/.gitkeep` +
//!      `.jigc/.gitignore`), so the project resolves as *set up*;
//!   3. merge the `jigc *` **allowlist** into `.claude/settings.json` (the
//!      path-of-least-resistance the bootstrap depends on);
//!   4. install the `SessionStart` **hook** running `jigc start` into the same
//!      settings file (the primary bootstrap injection — advertise+demonstrate at
//!      session start).
//!
//! `jigc setup` is the install the unset-project orientation routes the agent to
//! (`crate::orient` → `OrientationView::unset_project`; `design/bootstrap.md` →
//! Orientation output examples). The `Resume` hook + the fan-out spawn binding are
//! post-MVP and out of scope here.
//!
//! Outcome is reported through the settled **finding** envelope (`DECISIONS.md`
//! 2026-05-31 → block-payload = a blocking-severity finding carrying a route):
//! success yields a plain summary; a write failure yields a single blocking
//! `setup.*` finding whose `route` directs the human's next action, and the
//! dispatcher maps it to a non-zero exit.

use crate::adapter::{self, AdapterProfile};
use crate::locate;
use engine::finding::Finding;
use std::path::Path;

/// The assistant whose embedded profile MVP `setup` installs. Single-assistant in
/// the MVP (Claude Code); a `--assistant` selector is post-MVP
/// (`design/assistant-adapter.md` → Generated, minimal, regenerated).
const SETUP_ASSISTANT: &str = "claude-code";

/// The result of a `jigc setup` install: the located repo root and the profile's
/// two host targets, so the dispatcher can render a precise success summary.
#[derive(Debug)]
pub struct SetupSummary {
    /// The repo-root-relative always-loaded file the bootstrap reference was
    /// injected into.
    pub line_file: String,
    /// The repo-root-relative settings file the allowlist was merged into.
    pub allowlist_file: String,
}

/// Run `jigc setup` from `start`: locate the repo root, load the Claude Code
/// profile, and run both injections idempotently.
///
/// `Ok(summary)` on a clean install (including a re-run, which is a byte-identical
/// no-op by the injectors' idempotency). `Err(finding)` is a single blocking
/// `setup.*` finding carrying a route — the dispatcher renders it and exits
/// non-zero. CLI **locates** the repo root; the injectors do the writes (the
/// engine stays presentation-free and filesystem-free).
pub fn run(start: &Path) -> Result<SetupSummary, Finding> {
    let ctx = locate::locate(start).map_err(|err| {
        Finding::block(
            "setup.repo-root",
            format!("cannot locate the repository root: {err:#}"),
            "run `jigc setup` from inside the target git repository",
        )
    })?;

    let profile = adapter::load_profile(SETUP_ASSISTANT).map_err(|err| {
        Finding::block(
            "setup.profile-load",
            format!("cannot load the `{SETUP_ASSISTANT}` adapter profile: {err}"),
            "reinstall jigc — the embedded adapter profile is missing or malformed",
        )
    })?;

    install(&ctx.repo_root, &profile)
}

/// Run both injections against `repo_root` with `profile`, mapping an IO failure
/// to a blocking `setup.*` finding with a route. The testable core of [`run`]
/// (no location step).
fn install(repo_root: &Path, profile: &AdapterProfile) -> Result<SetupSummary, Finding> {
    // 0. Gate the spawn launch template against the decidable install-time rule
    //    *before* any write, so a broken template fails install touching nothing
    //    (`design/assistant-adapter.md` → Bind the spawn mechanism: "A broken
    //    template is an install-time error … the install does not complete"). The
    //    violated clause's pointer rides as the route.
    if let Err(reason) = adapter::validate_spawn_template(&profile.spawn.template) {
        return Err(Finding::block(
            "setup.spawn-template",
            format!(
                "the `{}` adapter profile's spawn launch template is invalid: {reason}",
                profile.assistant
            ),
            reason.to_string(),
        ));
    }

    let reference = profile.reference().ok_or_else(|| {
        Finding::block(
            "setup.profile-incomplete",
            format!(
                "the `{}` adapter profile declares no inject reference floor",
                profile.assistant
            ),
            "reinstall jigc — the embedded adapter profile is missing its bootstrap reference",
        )
    })?;
    let line_file = reference.file.clone();
    let bootstrap_file = reference.to.clone();

    // 1. Reference floor: write the managed bootstrap file, then point the
    //    always-loaded file at it with a bare import line.
    adapter::write_bootstrap_file(repo_root).map_err(|err| {
        Finding::block(
            "setup.write-bootstrap",
            format!("cannot write the managed bootstrap file `{bootstrap_file}`: {err}"),
            format!("ensure `{bootstrap_file}` is writable, then re-run `jigc setup`"),
        )
    })?;
    adapter::inject_reference(repo_root).map_err(|err| {
        Finding::block(
            "setup.inject-reference",
            format!("cannot inject the bootstrap reference into `{line_file}`: {err}"),
            format!("ensure `{line_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 2. Initialize the project cascade layer, so the project resolves as set up.
    adapter::init_project_layer(repo_root).map_err(|err| {
        Finding::block(
            "setup.init-project-layer",
            format!("cannot initialize the project layer under `.jigc/`: {err}"),
            "ensure `.jigc/` is writable, then re-run `jigc setup`",
        )
    })?;

    // 3. Allowlist `jigc` so the agent runs it without friction.
    let allowlist_file = profile.allowlist.file.clone();
    adapter::inject_allowlist(repo_root, profile).map_err(|err| {
        Finding::block(
            "setup.inject-allowlist",
            format!("cannot merge the allowlist into `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    // 4. Install the SessionStart hook (the primary bootstrap injection) into the
    //    same settings file. A no-op for a profile that declares no hook.
    adapter::inject_hook(repo_root, profile).map_err(|err| {
        Finding::block(
            "setup.inject-hook",
            format!("cannot install the session hook into `{allowlist_file}`: {err}"),
            format!("ensure `{allowlist_file}` is writable, then re-run `jigc setup`"),
        )
    })?;

    Ok(SetupSummary {
        line_file,
        allowlist_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop (the project's
    /// no-tempfile pattern).
    struct TempDir(PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-setup-unit-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// A clean install writes both host files and reports their paths from the
    /// profile.
    #[test]
    fn install_writes_both_targets_and_reports_paths() {
        let dir = TempDir::new();
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        let summary = install(dir.path(), &profile).expect("install succeeds");

        assert_eq!(summary.line_file, "CLAUDE.md");
        assert_eq!(summary.allowlist_file, ".claude/settings.json");
        assert!(
            dir.path().join("CLAUDE.md").exists(),
            "install must write CLAUDE.md",
        );
        assert!(
            dir.path().join(".claude/settings.json").exists(),
            "install must write .claude/settings.json",
        );
    }

    /// Install over the SHIPPED profile (a valid spawn template) does not trip the
    /// spawn-template gate — the success path is unbroken (this asserts the gate
    /// lets the shipped template through; the full write success is covered above).
    #[test]
    fn install_accepts_shipped_spawn_template() {
        let dir = TempDir::new();
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        install(dir.path(), &profile).expect("the shipped valid spawn template installs clean");
    }

    /// Install over a profile whose spawn template violates the decidable rule
    /// fails *before* any write with a single blocking `setup.spawn-template`
    /// finding whose route names the violated clause (the `SpawnTemplateReason`
    /// pointer). The gate runs ahead of the host-file writes, so a broken template
    /// touches nothing on disk.
    #[test]
    fn install_rejects_broken_spawn_template_with_clause_route() {
        use engine::finding::Severity;

        let dir = TempDir::new();
        let mut profile = adapter::load_profile("claude-code").expect("the shipped profile loads");
        // Strip the `{{task_id}}` placeholder — the first decidable clause.
        profile.spawn.template =
            "Use your Task tool to run: `jigc workflow {{workflow}} --task X`".to_string();

        let finding =
            install(dir.path(), &profile).expect_err("a broken spawn template must fail install");

        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(
            finding.code, "setup.spawn-template",
            "the block must be the spawn-template family code; got `{}`",
            finding.code,
        );
        let route = finding
            .route
            .as_deref()
            .expect("a hard block must carry a route");
        assert!(
            route.contains(&adapter::SpawnTemplateReason::MissingTaskIdPlaceholder.to_string()),
            "the route must name the violated clause; got `{route}`",
        );
        // Nothing was written: the gate runs before the host-file writes.
        assert!(
            !dir.path().join("CLAUDE.md").exists(),
            "a rejected template must touch no host files",
        );
    }

    /// A write failure surfaces a blocking `setup.*` finding carrying a route, not
    /// a panic and not a bare error. Here `CLAUDE.md` is a *directory*, so the
    /// line write fails.
    #[test]
    fn install_failure_yields_blocking_finding_with_route() {
        use engine::finding::Severity;

        let dir = TempDir::new();
        // Make the line target unwritable: a directory where a file must go.
        std::fs::create_dir_all(dir.path().join("CLAUDE.md")).expect("seed a directory");
        let profile = adapter::load_profile("claude-code").expect("the shipped profile loads");

        let finding = install(dir.path(), &profile).expect_err("the line write must fail");

        assert_eq!(finding.severity, Severity::Blocking);
        assert!(
            finding.code.starts_with("setup."),
            "the block code must be in the `setup.*` family; got `{}`",
            finding.code,
        );
        assert!(
            finding.route.is_some(),
            "a hard block must carry a route directing the next action",
        );
    }
}
