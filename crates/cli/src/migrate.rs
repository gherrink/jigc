//! `jigc migrate <path> --as <doctype>` — the auto-migration trigger (M23 G1).
//!
//! A foreign `CHANGELOG.md` at repo root classifies `Unmanaged` (no finding, no
//! route, no hook), so neither `jigc ingest`'s `needs-reconcile` arm nor `jigc start`
//! (which has no source-path parameter) can reach it. An explicit `migrate` verb
//! taking `path + --as <doctype>` addresses an arbitrary foreign file directly
//! ([auto-migration.md](../../../design/auto-migration.md) → The `jigc migrate` verb).
//!
//! The verb, for the staged-only increment (Inc 1):
//!   1. reads the foreign file's bytes;
//!   2. mints an **off-router migration task** ([`start::mint_migration_in_repo`] —
//!      the `record-change` shape: `creates-task: true`, off-router so it is never a
//!      selectable router pick), with a stable doctype-derived id;
//!   3. **stages** the foreign bytes into the task working area as a read-only source
//!      artifact (`<task_dir>/source`);
//!   4. composes the `migrate-<doctype>` workflow with the foreign bytes fed into the
//!      **CLI-owned source seam** ([`engine::data_value::ComposeContext::source`]), so
//!      the composed view's `{{source}}` placeholder surfaces them verbatim
//!      ([auto-migration.md](../../../design/auto-migration.md) → The source seam).
//!
//! It ends at a composed migration workflow over the staged foreign content; the LLM
//! then authors the canonical doc through the existing write verbs, and a later
//! increment adds the review gate + retire + commit + adopt. The CLI owns the read
//! (the determinism boundary — the agent never fetches the file out-of-band).

use crate::cli::Format;
use crate::render;
use crate::start;
use anyhow::{Context, Result, bail};
use engine::compose::ComposedWorkflow;
use engine::state;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

/// The working-area filename the staged foreign source bytes live at — the read-only
/// source artifact the source seam surfaces. Plain bytes, read back verbatim (the same
/// diff-friendly style as the `intent`/`workflow` task files).
pub(crate) const SOURCE_FILE: &str = "source";

/// Run `jigc migrate <path> --as <doctype>` against `cwd`: locate the repo + project
/// layer, read the foreign file, mint the off-router migration task, stage the foreign
/// bytes, compose the `migrate-<doctype>` workflow over the source seam, render the
/// composed view through `format`, and print it. A clean run exits 0; an unknown
/// doctype, a missing foreign file, a serial collision, or a blocking compose finding
/// surfaces on stderr (with its route) and exits non-zero.
pub fn run(cwd: &Path, path: &str, doctype: &str, format: Format) -> ExitCode {
    match migrate_in_repo(cwd, path, doctype) {
        Ok(view) => {
            println!("{}", render::composed(format, &view));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// The off-router `migrate-<doctype>` workflow id — the migration workflow the
/// minted task composes. Only `changelog` is migrated this milestone, but the verb
/// is doctype-parameterized so the workflow id is derived, not hard-wired.
fn migration_workflow(doctype: &str) -> String {
    format!("migrate-{doctype}")
}

/// Normalize the verb's `path` arg to a clean repo-relative string for recording as the
/// retire target (review F2). Strip a `repo_root` prefix from an absolute spelling
/// (`/abs/repo/changelog/changelog.md` → `changelog/changelog.md`), then drop `.`
/// components and resolve `..` lexically. Mirrors the engine retire guard's
/// [`engine::finalize`] normalization so `source-path` is canonical on disk — the
/// in-location-squatter guard then compares it against the clean canonical promote
/// destination regardless of how the caller spelled the path.
fn repo_relative_source_path(repo_root: &Path, path: &str) -> String {
    let supplied = Path::new(path);
    let relative = supplied.strip_prefix(repo_root).unwrap_or(supplied);
    let mut out = PathBuf::new();
    for component in relative.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out.to_string_lossy().into_owned()
}

/// Mint the off-router migration task, stage the foreign bytes, and compose the
/// migration workflow over the source seam — the verb's repo-rooted spine.
///
/// `path` is the foreign file's path, resolved relative to `cwd` (the repo). The mint
/// happens before composition so the seam can be fed off the just-staged bytes; a
/// failed compose leaves the minted task in place for inspection / re-entry (the
/// determinism boundary keeps the seam read CLI-owned).
fn migrate_in_repo(cwd: &Path, path: &str, doctype: &str) -> Result<ComposedWorkflow> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    let repo_root = ctx.repo_root;
    let project_config = repo_root.join(".jigc").join("config");

    // Read the foreign file's bytes (the source the seam carries). Resolve the path
    // against the repo root so a repo-relative `CHANGELOG.md` reaches the root file.
    let foreign_path = repo_root.join(path);
    let foreign = std::fs::read_to_string(&foreign_path).with_context(|| {
        format!(
            "could not read the foreign `{doctype}` source at {}",
            foreign_path.display()
        )
    })?;

    // The repo-relative foreign path — recorded so finalize can retire the foreign
    // original inside the commit transaction (`design/auto-migration.md` →
    // Retire-the-foreign-original), and fed to the mint so the auto-provisioned commit
    // doc's templated summary/body name it (Hardening #4). Normalized to a clean
    // repo-relative form — strip a `repo_root` prefix from an absolute spelling and drop
    // redundant `./` components — so the finalize retire's path-collision guard (the
    // in-location squatter) compares canonically regardless of the caller's spelling
    // (review F2).
    let recorded = repo_relative_source_path(&repo_root, path);

    // Mint the off-router migration task (the `record-change` shape) + auto-provision its
    // commit doc *filled* off the recorded source path + doctype, then stage the foreign
    // bytes into the working area.
    let workflow_id = migration_workflow(doctype);
    let minted = start::mint_migration_in_repo(&repo_root, doctype, &workflow_id, &recorded)?;
    let source_path = minted.dir.join(SOURCE_FILE);
    state::persist(&source_path, foreign.as_bytes())
        .with_context(|| format!("could not stage the foreign source for `{}`", minted.id))?;

    state::persist(
        &minted.dir.join(state::SOURCE_PATH_FILE),
        recorded.as_bytes(),
    )
    .with_context(|| {
        format!(
            "could not record the foreign source path for `{}`",
            minted.id
        )
    })?;

    // Compose the migration workflow over the minted task with the foreign bytes fed
    // into the source seam — the composed view's `{{source}}` surfaces them verbatim.
    start::compose_migrate_in_repo(
        &repo_root,
        &project_config,
        &minted.dir,
        &minted.id,
        &workflow_id,
        &foreign,
    )
}

#[cfg(test)]
mod tests {
    use super::repo_relative_source_path;
    use std::path::Path;

    /// Review F2: the recorded `source-path` is normalized to a clean repo-relative form
    /// so the finalize retire's in-location-squatter guard compares canonically. A
    /// `./`-prefixed, an absolute (repo-root-prefixed), and a `..`-round-trip spelling of
    /// the canonical changelog path all collapse to `changelog/changelog.md`.
    #[test]
    fn records_a_clean_repo_relative_source_path() {
        let repo_root = Path::new("/abs/repo");
        assert_eq!(
            repo_relative_source_path(repo_root, "CHANGELOG.md"),
            "CHANGELOG.md",
        );
        assert_eq!(
            repo_relative_source_path(repo_root, "./changelog/changelog.md"),
            "changelog/changelog.md",
        );
        assert_eq!(
            repo_relative_source_path(repo_root, "/abs/repo/changelog/changelog.md"),
            "changelog/changelog.md",
        );
        assert_eq!(
            repo_relative_source_path(repo_root, "changelog/../changelog/changelog.md"),
            "changelog/changelog.md",
        );
    }
}
