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
use crate::invocation_log::Outcome;
use crate::render;
use crate::start;
use anyhow::{Context, Result, anyhow, bail};
use engine::finding::{Finding, Location, Route, Severity};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::state;
use std::path::{Component, Path, PathBuf};

/// The byte floor below which a foreign source is **too trivial to migrate** — a
/// near-empty or placeholder file has little prose to preserve, and an agent reaching
/// for `jigc migrate` on such a source is almost always exploiting migration as an
/// authoring back door (the rc.7 placeholder-source loophole; `design/auto-migration.md`
/// → The byte-floor advisory). A source of at least this many bytes migrates silently.
///
/// Calibrated **below every shipped foreign fixture** (the smallest is 58 bytes) so a
/// real foreign document — even a minimal one — never trips the advisory; the guard is a
/// mechanical byte count (Framing-A), never a content judgment.
pub(crate) const TRIVIAL_SOURCE_FLOOR: usize = 48;

/// The working-area filename the staged foreign source bytes live at — the read-only
/// source artifact the source seam surfaces. Plain bytes, read back verbatim (the same
/// diff-friendly style as the `intent`/`workflow` task files).
pub(crate) const SOURCE_FILE: &str = "source";

/// Run `jigc migrate <path> --as <doctype>` (optional `--slug`) against `cwd`: locate
/// the repo + project layer, read the foreign file, mint the off-router migration
/// task, stage the foreign bytes, compose the `migrate-<doctype>` workflow over the
/// source seam, render the composed view through `format`, and print it. A clean run
/// exits 0; an unknown doctype, a malformed `--slug`, a missing foreign file, a serial
/// collision, or a blocking compose finding surfaces on stderr (with its route) and
/// exits non-zero.
pub fn run(
    cwd: &Path,
    path: &str,
    doctype: &str,
    slug_override: Option<&str>,
    format: Format,
) -> Outcome {
    match migrate_in_repo(cwd, path, doctype, slug_override) {
        Ok((view, advisory)) => {
            println!("{}", render::composed(format, &view));
            // The byte-floor triviality advisory (M44 Inc 5, S2) rides the **agent/human
            // presentation surface only** — the composed `--format json` stays the pinned
            // `{task, text}` contract, byte-identical (`render::composed`'s json arm is
            // untouched). Stream discipline mirrors the finalize advisories
            // (`task.rs::emit_left_out_advisory`): agent/human text to **stdout** (where
            // the agent reads the migrate surface), but under `--format json` the
            // structured envelope owns stdout, so the advisory goes to **stderr** and the
            // JSON stdout bytes never move. Non-blocking either way — exit 0.
            if let Some(finding) = advisory {
                let line = render::advisory_line(&finding);
                if matches!(format, Format::Json) {
                    eprint!("{line}");
                } else {
                    print!("{line}");
                }
            }
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// The byte-floor triviality advisory (M44 Inc 5, S2) — surfaced when an **adr** foreign
/// source is below [`TRIVIAL_SOURCE_FLOOR`] bytes, `None` otherwise. A non-blocking
/// [`Severity::Advisory`] finding (its own `migrate.trivial-source` code, colliding with
/// neither error identity) that names the concrete byte count + floor and routes to the
/// from-knowledge `jigc start --workflow record-decision <intent>` path — the honest
/// alternative to migrating a placeholder source (`design/auto-migration.md` → The
/// byte-floor advisory; `surface-contract.md` law 1). The from-knowledge path never
/// reaches `migrate`, so it is exempt by construction (no source to measure).
///
/// **Scoped to `adr` (audit fix).** The rider (fork 4) was built to close the rc.7
/// *adr* placeholder-source loophole, and both its advice ("author the decision from
/// knowledge") and route (`record-decision`, whose `allows-create` is `{type: adr}`)
/// produce an **ADR** — not the doctype under migration. Firing it on a non-adr migration
/// misdirected the agent to author an adr instead of the intended doctype (law 1: nothing
/// lies), so the guard is restored to the exact scope of the loophole it addresses;
/// non-adr trivial migrations get no advisory.
fn trivial_source_advisory(recorded: &str, doctype: &str, byte_len: usize) -> Option<Finding> {
    if doctype != "adr" || byte_len >= TRIVIAL_SOURCE_FLOOR {
        return None;
    }
    Some(Finding::graded(
        Severity::Advisory,
        "migrate.trivial-source",
        format!(
            "the foreign `{doctype}` source at `{recorded}` is {byte_len} bytes — below the \
             {TRIVIAL_SOURCE_FLOOR}-byte floor for a document worth migrating; a near-empty or \
             placeholder source has little prose to preserve, so authoring the decision from \
             knowledge is usually the honest path (migration still composed below)"
        ),
        Some(Location::at(1, 1)),
        Some(Route::mechanical(
            ["jigc", "start", "--workflow", "record-decision", "<intent>"],
            " — record the decision from knowledge; no source to migrate",
        )),
    ))
}

/// The off-router `migrate-<doctype>` workflow id — the migration workflow the
/// minted task composes. Any doctype whose composed pack ships a `migrate-<doctype>`
/// workflow migrates (the dev five plus the methodology seven, as of M40); the verb
/// is doctype-parameterized so the workflow id is derived, not hard-wired.
fn migration_workflow(doctype: &str) -> String {
    format!("migrate-{doctype}")
}

/// Validate `--as <doctype>` against the available `migrate-<doctype>` workflows
/// **before** any task is minted — so a typo'd or non-migratable doctype strands no
/// task dir (the `read_workflow` mint-after-validate discipline: a rejected migrate
/// leaves `jigc task list` unchanged, never an orphan that can neither compose nor
/// finalize). The message distinguishes an *unknown* doctype from a *known-but-not-
/// migratable* one (a schema ships but no `migrate-<doctype>` workflow does — e.g.
/// `commit`, or methodology's machine-maintained `milestone-record`) and names the
/// migratable set.
fn ensure_migratable(pack: &dyn PackSource, doctype: &str) -> Result<()> {
    let workflow_id = migration_workflow(doctype);
    if pack
        .list(PackResourceKind::Workflows)
        .iter()
        .any(|id| *id == ResourceId::from(workflow_id.as_str()))
    {
        return Ok(());
    }

    // The migratable set: every doctype with a shipped `migrate-<doctype>` workflow,
    // address-sorted (`list` returns sorted ids).
    let migratable: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .iter()
        .filter_map(|id| id.as_str().strip_prefix("migrate-").map(str::to_owned))
        .collect();
    let set = migratable.join(", ");

    // An **unknown** doctype is the axis's fault, not this door's: it names nothing in the
    // resolved cascade, which is `store.unknown-type` wherever it is typed, so it carries
    // that code and the runnable doctype-surface route every other door carries (M49
    // Increment 11 / T1). What stays door-local is the *message*: `migrate`'s usable set is
    // narrower than the cascade's, so the refusal still names the migratable set.
    let known = pack
        .list(PackResourceKind::Schemas)
        .iter()
        .any(|id| *id == ResourceId::from(doctype));
    if !known {
        let mut finding = engine::store::unknown_doctype(doctype);
        finding.message = format!("{}; migratable doctypes: {set}", finding.message);
        return Err(crate::render::finding_error(&finding));
    }

    // A **known** doctype with no `migrate-<doctype>` workflow is a different fault — the
    // doctype exists — so it keeps its own sentence and its re-run route, whose `<path>`
    // is a declared non-derivable placeholder (`design/surface-contract.md` → P6).
    let route =
        engine::finding::Route::mechanical(["jigc", "migrate", "<path>", "--as", "<doctype>"], "");
    bail!(
        "doctype `{doctype}` exists but is not migratable (no `migrate-{doctype}` workflow); \
         migratable doctypes: {set}\n  route: re-run {route} with one of: {set}"
    );
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
fn migrate_in_repo(
    cwd: &Path,
    path: &str,
    doctype: &str,
    slug_override: Option<&str>,
) -> Result<(crate::start::Composition, Option<Finding>)> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        return Err(crate::locate::not_set_up());
    }
    let repo_root = ctx.repo_root;
    let project_config = repo_root.join(".jigc").join("config");

    // Validate `--as <doctype>` BEFORE minting: a missing `migrate-<doctype>` workflow
    // is rejected here so a typo'd or non-migratable doctype strands no task dir (the
    // bug this guards: minting first then discovering the missing workflow left a
    // permanent orphan in `jigc task list`).
    let pack = crate::pack::make_pack()?;
    ensure_migratable(pack.as_ref(), doctype)?;

    // Validate a `--slug` override BEFORE minting (the same mint-after-validate
    // discipline as `ensure_migratable`: a rejected value strands no task dir). The
    // value drives the migrated doc's id **verbatim** — a malformed one is rejected,
    // never silently re-slugified (the `doc create --slug` reject precedent;
    // `DECISIONS.md` 2026-07-06 M39 planning → Slug (G6)).
    if let Some(slug) = slug_override
        && !engine::slug::is_slug(slug)
    {
        bail!(
            "`--slug {slug:?}` is not a valid slug — use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"
        );
    }

    // Read the foreign file's bytes (the source the seam carries). Resolve the path
    // against the repo root so a repo-relative `CHANGELOG.md` reaches the root file.
    let foreign_path = repo_root.join(path);
    // A route-carrying error, not a `with_context` over the raw I/O error: the latter
    // chains the `os error 2` tail into `{err:#}` — a dead end for the agent. Name the
    // path and route back to the verb with a readable source (M36 Inc-4, errors-with-
    // remediation).
    let foreign = std::fs::read_to_string(&foreign_path).map_err(|_| {
        // The runtime doctype rides the span's argv (a real value parses like any value);
        // `<path>` is a declared dummy-table placeholder.
        let route =
            engine::finding::Route::mechanical(["jigc", "migrate", "<path>", "--as", doctype], "");
        anyhow!(
            "could not read the foreign `{doctype}` source at {}\n  route: check the path, then re-run {route} with a readable file",
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

    // The byte-floor triviality advisory (S2), computed off the just-read foreign bytes —
    // presentation-only, surfaced by `run` beside (never inside) the pinned composed
    // contract. `None` for a normal-sized source; the from-knowledge path never reaches
    // here, so it is exempt by construction.
    let advisory = trivial_source_advisory(&recorded, doctype, foreign.len());

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

    // Record the validated `--slug` override as a SOURCE_PATH_FILE-sibling task-state
    // file, read back by `doc author` to drive the target doc's id verbatim
    // ([`engine::state::read_slug_override`]). Absent on a slug-less migrate — the
    // author then derives the slug from the payload title, byte-identical to before.
    if let Some(slug) = slug_override {
        state::persist(&minted.dir.join(state::SLUG_OVERRIDE_FILE), slug.as_bytes())
            .with_context(|| format!("could not record the slug override for `{}`", minted.id))?;
    }

    // Compose the migration workflow over the minted task with the foreign bytes fed
    // into the source seam — the composed view's `{{source}}` surfaces them verbatim.
    let composition = start::compose_migrate_in_repo(
        &repo_root,
        &project_config,
        &minted.dir,
        &minted.id,
        &workflow_id,
        &foreign,
    )?;
    Ok((composition, advisory))
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
