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
use anyhow::Result;
use engine::override_default::RecordedDeltas;
use engine::packsource::PackSource;
use engine::probe::{OverrideCtx, OverrideDefaultProbe, Probe};
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
    let pack = make_pack()?;
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
    let (layer, structural, slot_fills, forks, bases) = load_project_layer(project_config)?;
    // The `scalar-set` existence question reads the recorded keys (the values are
    // irrelevant to "is the key still a declared knob?"). The `OverrideLayer` owns
    // them; the classifier checks each against the current pack's closed surface.
    let scalars: Vec<String> = layer.scalar_set_keys().map(str::to_owned).collect();
    // `override-default` is invoked through the non-task `Probe` seam (its ctx is
    // `(recorded deltas, pack)`, not a task working area — `validation.md` → The
    // non-task `Probe` seam). The seam delegates to the M5 `classify` logic; severity
    // stays engine-owned, assigned by the post-pass at `ValidationReport::new` below.
    let findings = OverrideDefaultProbe.check(
        &(),
        OverrideCtx {
            deltas: RecordedDeltas {
                structural: &structural,
                forks: &forks,
                bases: &bases,
                slot_fills: &slot_fills,
                scalars: &scalars,
            },
            pack,
        },
    );
    // The engine severity post-pass reads the resolved cascade: a recorded
    // `validation.override-default.*.severity` scalar-set tunes the classifier's
    // findings (`design/validation.md` → Every finding-emitting entry point must
    // resolve the cascade). Resolved *resiliently* — an orphaned scalar-set (a knob
    // the current pack dropped) is the very thing `classify` reports above, so it is
    // filtered out of the severity resolution rather than aborting it. A no-override
    // project layer resolves to the base scalars, so the post-pass is inert and
    // `override-default`'s emitted severities stay byte-identical.
    let resolved = crate::start::resolve_severity_cascade_resilient(pack, project_config)?;
    Ok(ValidationReport::new(findings, &resolved))
}

/// The number of recorded config deltas the upgrade sweep checks — structural ops +
/// slot-fills + tracked forks + `scalar-set` keys, the same surfaces
/// [`upgrade_with_pack`] feeds the classifier. Read for the clean line's honesty
/// (round-2 D4: "no findings" must name **what was checked** — a config-delta
/// re-apply, never "the task", and never the same line for zero deltas and N).
pub(crate) fn recorded_delta_count(cwd: &Path) -> Result<usize> {
    let project_config = require_project_layer(cwd)?;
    let (layer, structural, slot_fills, forks, _bases) = load_project_layer(&project_config)?;
    Ok(structural.len() + slot_fills.len() + forks.len() + layer.scalar_set_keys().count())
}

/// Locate the repo root and its `.jigc/config/` project layer — the same preamble
/// the `jigc config` verbs use (`config.rs`'s `require_project_layer`), replicated
/// here so the upgrade seam has no cross-module private dependency. Errors with the
/// same routed messages when the repo or the project layer is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    // The `.jigc/` project layer binds to jigc_home (the main checkout), so a worktree
    // resolves the one shared layer (M31 Inc 2 / WF3). `upgrade` rewrites the project
    // cascade only, so jigc_home is the single base it needs (no git, no worktree code).
    crate::start::require_project_config(cwd)
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
    /// step files plus a minimal `config/knobs.yaml` declaring `default-workflow` —
    /// the "env-selected pack" the recording/upgrade paths share, here constructed
    /// directly (no `JIGC_PACK_DIR` env mutation, parallel-test-safe; the
    /// env-through-binary flow is the T4 subprocess e2e). The knob surface is seeded
    /// so a recorded `scalar-set default-workflow` classifies clean (the key is still
    /// declared) — the scalar-set existence question reads this surface.
    fn fs_pack(dir: &TempDir, steps: &[(&str, &str)]) -> FilesystemPack {
        let pack_root = dir.path().join("pack");
        let steps_dir = pack_root.join("steps");
        fs::create_dir_all(&steps_dir).expect("mk pack/steps");
        for (stem, body) in steps {
            fs::write(steps_dir.join(format!("{stem}.yaml")), body).expect("seed step");
        }
        let config_dir = pack_root.join("config");
        fs::create_dir_all(&config_dir).expect("mk pack/config");
        fs::write(
            config_dir.join("knobs.yaml"),
            format!(
                "default-workflow:\n  type: string\n  default: single-task\n{}",
                crate::pack::intrinsic_knobs_yaml(),
            ),
        )
        .expect("seed knobs.yaml");
        // The severity post-pass now resolves the project cascade against this pack
        // (T3: `override-default`'s findings tune through the resolved cascade), which
        // reads `config/defaults`'s `pack-id` (the pack-default layer provenance). A
        // real pack always ships it; seed it so the directly-built pack resolves.
        fs::write(config_dir.join("defaults.yaml"), "pack-id: test-pack\n")
            .expect("seed defaults.yaml");
        FilesystemPack::new(pack_root)
    }

    /// A `FilesystemPack` like [`fs_pack`] whose `config/knobs.yaml` **also** declares
    /// the `validation.override-default.target-unchanged.severity` knob (an `enum` of
    /// the three severity tokens, defaulting to `blocking`) — the closed surface a
    /// recorded `scalar-set` on that key resolves against, so the M6 post-pass can tune
    /// an `override-default` conflict. A real pack ships this knob; the test seeds it so
    /// the directly-built pack carries it (the env-through-binary flow is the flow-8 T3
    /// e2e).
    fn fs_pack_with_severity_knob(dir: &TempDir, steps: &[(&str, &str)]) -> FilesystemPack {
        let pack = fs_pack(dir, steps);
        let knobs = dir.path().join("pack").join("config").join("knobs.yaml");
        let mut text = fs::read_to_string(&knobs).expect("read knobs");
        text.push_str(
            "validation.override-default.target-unchanged.severity:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n",
        );
        fs::write(&knobs, text).expect("append severity knob");
        pack
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

    /// **T1 — the seam preserves the post-pass tuning.** `override-default`'s `classify`
    /// is now invoked **through the non-task `Probe` seam** ([`engine::probe`]); this
    /// proves routing through the seam does not regress the inc-1 severity post-pass. A
    /// `tracked-fork` whose recorded `base-hash` differs from the env-selected pack's
    /// `validate` step (a **conflict**, emitted blocking) is demoted to
    /// [`engine::finding::Severity::Warning`] by a recorded
    /// `scalar:` on `validation.override-default.target-unchanged.severity` — through the
    /// seam path. The manifest is byte-unchanged on disk (report-and-route only).
    #[test]
    fn upgrade_demotes_a_conflict_to_warning_through_the_seam() {
        use engine::finding::Severity;

        let dir = TempDir::new("conflict-demote");
        let project_config = seed_repo(&dir);

        // The recorded basis = the blake3 of the *v1* `validate` body the fork was
        // taken from; the pack ships a *changed* v2 body → the pack-direct compare
        // conflicts (emitted blocking under `target-unchanged`).
        let validate_v1 = "validate body v1\n";
        let recorded = engine::file_state::hash_bytes(validate_v1.as_bytes());
        let manifest_text = format!(
            "\
scalar:
  validation.override-default.target-unchanged.severity: warning
deltas:
  - kind: tracked-fork
    target: workflow:single-task#validate
    base-version: v1
    base-hash: {recorded}
"
        );
        let manifest = project_config.join("manifest.yaml");
        fs::write(&manifest, &manifest_text).expect("seed manifest");

        // The pack's `validate` step changed upstream (≠ the recorded basis) → conflict.
        let pack =
            fs_pack_with_severity_knob(&dir, &[("validate", "validate body v2 — changed\n")]);

        let report = upgrade_with_pack(&project_config, &pack).expect("classify");

        assert_eq!(report.findings.len(), 1, "exactly the conflict: {report:?}");
        let finding = &report.findings[0];
        assert_eq!(finding.code, "override-default.content-changed");
        assert_eq!(finding.check, "target-unchanged");
        assert_eq!(
            finding.severity,
            Severity::Warning,
            "the recorded `scalar-set` demotes the conflict to warning through the seam path",
        );
        assert!(
            !report.has_blocking(),
            "a demoted conflict no longer blocks the upgrade: {report:?}",
        );

        // The seam mutates nothing — the manifest is byte-identical on disk.
        let after = fs::read_to_string(&manifest).expect("re-read manifest");
        assert_eq!(after, manifest_text, "upgrade is report-and-route only");
    }

    /// **Validation hardening #5 — the omitting-target context.** The *same* conflict,
    /// classified through the seam against a project layer that **omits** the severity
    /// `scalar-set`, keeps its emitted **blocking** severity (the post-pass is inert when
    /// the cascade carries no override). A green pass over the demote test alone would
    /// hide a seam bug that always tunes; this pins the inert-on-omission behaviour.
    #[test]
    fn upgrade_conflict_stays_blocking_when_severity_override_is_omitted() {
        use engine::finding::Severity;

        let dir = TempDir::new("conflict-no-override");
        let project_config = seed_repo(&dir);

        let validate_v1 = "validate body v1\n";
        let recorded = engine::file_state::hash_bytes(validate_v1.as_bytes());
        // No `scalar:` severity override — the omitting context.
        let manifest_text = format!(
            "\
deltas:
  - kind: tracked-fork
    target: workflow:single-task#validate
    base-version: v1
    base-hash: {recorded}
"
        );
        fs::write(project_config.join("manifest.yaml"), &manifest_text).expect("seed manifest");

        let pack =
            fs_pack_with_severity_knob(&dir, &[("validate", "validate body v2 — changed\n")]);

        let report = upgrade_with_pack(&project_config, &pack).expect("classify");

        assert_eq!(report.findings.len(), 1, "exactly the conflict: {report:?}");
        assert_eq!(
            report.findings[0].severity,
            Severity::Blocking,
            "with no severity override the post-pass is inert — the conflict stays blocking",
        );
        assert!(
            report.has_blocking(),
            "an un-demoted conflict blocks: {report:?}"
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
