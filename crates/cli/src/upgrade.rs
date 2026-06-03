//! `jigc upgrade` — the cli assembly seam (M5 increment 4, T2).
//!
//! **Report-and-route only.** This seam loads the project layer's recorded deltas
//! and runs the engine `override-default` classifier over them against the
//! **env-selected** pack (the same `make_pack` factory the *recording* verbs use,
//! so a basis recorded against one pack compares against the same one —
//! `overrides.md` → the `FilesystemPack` seam). It **reads nothing it writes and
//! writes nothing**: the manifest is byte-unchanged on disk after the call. The
//! returned [`ValidationReport`] is what the `Command::Upgrade` dispatch (T3)
//! renders and gates on (`overrides.md` → The `jigc upgrade` command, steps 1–2:
//! re-apply, classify).
//!
//! `Command::Upgrade` (clap + dispatch, T3) is the production consumer of
//! [`upgrade_in_repo`]; `upgrade_with_pack` is the directly-built-pack core it
//! delegates to (and the tests drive without mutating `JIGC_PACK_DIR`).

use crate::pack::make_pack;
use crate::start::load_project_layer;
use anyhow::{Context, Result, bail};
use engine::override_default::{RecordedDeltas, classify};
use engine::packsource::PackSource;
use engine::result::ValidationReport;
use std::path::{Path, PathBuf};

/// Run the `override-default` classifier over every recorded delta against the
/// **current (env-selected) pack**, returning the [`ValidationReport`] of findings
/// the upgrade dispatch renders and gates on — **report-and-route only, mutates
/// nothing** (`overrides.md` → The `jigc upgrade` command).
///
/// Locate the repo root + its `.jigc/config/` project layer, build the pack
/// through the **same `make_pack` factory the recording verbs use** (so the
/// recorded basis and the compared pack are read from the same env-selected
/// source — `overrides.md` → the `FilesystemPack` seam: "the recording path and
/// the upgrade path read the same env-selected pack"), and classify. Nothing is
/// written; nothing read here is written elsewhere.
pub(crate) fn upgrade_in_repo(cwd: &Path) -> Result<ValidationReport> {
    let project_config = require_project_layer(cwd)?;
    let pack = make_pack();
    upgrade_with_pack(&project_config, pack.as_ref())
}

/// The testable core of [`upgrade_in_repo`]: classify the deltas recorded in
/// `project_config` against an already-constructed `pack`, so the classification
/// is exercised against a directly-built [`crate::pack::FilesystemPack`] without
/// mutating the process environment (`JIGC_PACK_DIR` selection — and its
/// parallel-test hazard — stays in [`make_pack`]; the genuine env-through-binary
/// flow is the T4 subprocess e2e).
///
/// Load the project layer's recorded surfaces (`load_project_layer`'s 5-tuple),
/// borrow the M5-relevant members into [`RecordedDeltas`], classify, and wrap the
/// resulting findings in a [`ValidationReport`]. A no-delta / clean manifest yields
/// an empty report (`has_blocking() == false`). This call **reads** the manifest
/// and **writes nothing** — the manifest is byte-unchanged on disk afterward.
pub(crate) fn upgrade_with_pack(
    project_config: &Path,
    pack: &dyn PackSource,
) -> Result<ValidationReport> {
    let (_layer, structural, slot_fills, forks, bases) = load_project_layer(project_config)?;
    let findings = classify(
        RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
            slot_fills: &slot_fills,
        },
        pack,
    );
    Ok(ValidationReport::new(findings))
}

/// Locate the repo root and its `.jigc/config/` project layer — the same preamble
/// the `jigc config` verbs use (`config.rs`'s `require_project_layer`), replicated
/// here so the upgrade seam has no cross-module private dependency. Errors with the
/// same routed messages when the repo or the project layer is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(project_config)
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::config` / `crate::start` / `crate::task` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pack::FilesystemPack;
    use std::fs;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-upgrade-seam-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
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

    /// Seed a throwaway repo (`.git` marker + `.jigc/config/`) and return its
    /// `.jigc/config` path. The repo-root discovery walks to the `.git` holder.
    fn seed_repo(dir: &TempDir) -> PathBuf {
        fs::create_dir_all(dir.path().join(".git")).expect("mk .git");
        let project_config = dir.path().join(".jigc").join("config");
        fs::create_dir_all(&project_config).expect("mk .jigc/config");
        project_config
    }

    /// A `FilesystemPack` over a fresh directory carrying the given `(stem, body)`
    /// step files — the "env-selected pack" the recording/upgrade paths share, here
    /// constructed directly (no `JIGC_PACK_DIR` env mutation, parallel-test-safe;
    /// the env-through-binary flow is the T4 subprocess e2e).
    fn fs_pack(dir: &TempDir, steps: &[(&str, &str)]) -> FilesystemPack {
        let pack_root = dir.path().join("pack");
        let steps_dir = pack_root.join("steps");
        fs::create_dir_all(&steps_dir).expect("mk pack/steps");
        for (stem, body) in steps {
            fs::write(steps_dir.join(format!("{stem}.yaml")), body).expect("seed step");
        }
        FilesystemPack::new(pack_root)
    }

    /// T2 done-criterion (the blocking half). A hand-authored manifest carries a
    /// `remove-step` delta whose target step the env-selected pack **omits**;
    /// `upgrade_with_pack` returns a [`ValidationReport`] carrying the expected
    /// **blocking orphaned** finding, and the manifest is **byte-unchanged on disk**
    /// after the call (the seam mutates nothing — `overrides.md` → The `jigc
    /// upgrade` command: report-and-route only).
    #[test]
    fn upgrade_classifies_an_orphaned_delta_and_mutates_nothing() {
        let dir = TempDir::new("orphaned");
        let project_config = seed_repo(&dir);

        // A `remove-step` over `workflow:single-task#gone`, no recorded basis. The
        // pack omits `gone` entirely → the existence question (asked first) orphans
        // it, blocking, before the no-basis branch.
        let manifest_text = "\
deltas:
  - kind: remove-step
    target: workflow:single-task#gone
";
        let manifest = project_config.join("manifest.yaml");
        fs::write(&manifest, manifest_text).expect("seed manifest");

        // The pack carries some *other* step but not `gone`.
        let pack = fs_pack(&dir, &[("implement", "implement body\n")]);

        let report = upgrade_with_pack(&project_config, &pack).expect("classify");

        assert!(
            report.has_blocking(),
            "an orphaned delta blocks the upgrade: {report:?}"
        );
        assert_eq!(report.findings.len(), 1, "exactly one finding: {report:?}");
        let finding = &report.findings[0];
        assert_eq!(finding.code, "override-default.target-exists");
        assert!(
            finding.message.contains("workflow:single-task#gone"),
            "the orphaned finding names the delta's target string: {}",
            finding.message
        );

        // The seam mutates nothing — the manifest is byte-identical on disk.
        let after = fs::read_to_string(&manifest).expect("re-read manifest");
        assert_eq!(
            after, manifest_text,
            "upgrade is report-and-route only: the manifest must be byte-unchanged",
        );
    }

    /// T2 done-criterion (the clean half). A no-delta manifest yields an **empty**
    /// report (`has_blocking() == false`) — a clean cascade emits nothing
    /// (`overrides.md` → Report: `clean` deltas emit nothing).
    #[test]
    fn upgrade_over_a_no_delta_manifest_returns_an_empty_report() {
        let dir = TempDir::new("clean");
        let project_config = seed_repo(&dir);
        // A manifest carrying only a `scalar:` block — no `deltas:` at all.
        fs::write(
            project_config.join("manifest.yaml"),
            "scalar:\n  default-workflow: single-task\n",
        )
        .expect("seed manifest");

        let pack = fs_pack(&dir, &[("implement", "implement body\n")]);

        let report = upgrade_with_pack(&project_config, &pack).expect("classify");

        assert!(
            report.findings.is_empty(),
            "a no-delta manifest classifies clean — no findings: {report:?}"
        );
        assert!(
            !report.has_blocking(),
            "a clean report does not block: {report:?}"
        );
    }

    /// `upgrade_in_repo` errors (routed) when the cwd is not inside a project layer
    /// — the locator preamble the `jigc config` verbs share. (`upgrade_in_repo`
    /// builds the pack via `make_pack`, so this exercises the production entry's
    /// locate step without mutating `JIGC_PACK_DIR`.)
    #[test]
    fn upgrade_in_repo_errors_outside_a_project_layer() {
        let dir = TempDir::new("no-layer");
        // A bare `.git` repo with no `.jigc/config/` cascade layer.
        fs::create_dir_all(dir.path().join(".git")).expect("mk .git");

        let err = upgrade_in_repo(dir.path()).expect_err("no project layer must error");
        assert!(
            err.to_string().contains("jigc setup"),
            "the missing-layer error routes to `jigc setup`: {err:#}"
        );
    }
}
