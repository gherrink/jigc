//! `jigc migrate <path> --as <doctype>` — the auto-migration trigger (M23 G1).
//!
//! A foreign `CHANGELOG.md` at repo root classifies `Unmanaged` (no finding, no
//! route, no hook), so neither `jigc ingest`'s `needs-reconcile` arm nor `jigc start`
//! (which has no source-path parameter) can reach it. An explicit `migrate` verb
//! taking `path + --as <doctype>` addresses that foreign file directly
//! ([auto-migration.md](../../../design/auto-migration.md) → The `jigc migrate` verb).
//!
//! **A foreign file *inside this repository*, and the word "arbitrary" is struck** (M51
//! Increment 1 / T1). The design doc and this header both said *an arbitrary foreign
//! file*, and the door honoured it: driven, `jigc migrate <absolute-path-outside-the-repo>`
//! exited 0 and recorded the host path as the value `finalize --approve` deletes. The
//! source is adjudicated before anything mints ([`adjudicate_source_path`]).
//!
//! **…and a file git holds a copy of** (M51 Increment 1 / T2). Location is not trackedness:
//! an *untracked* in-repo source passed every location leg, and `--approve` then deleted it
//! from the worktree while [`crate::task`]'s migration staging skipped a pathspec that
//! matched nothing — so the bytes ended in no git object and the removal was named on no
//! surface. [`adjudicate_source_tracked`] refuses a source present in neither the index nor
//! `HEAD`, and routes at the one act that resolves it, `git add`.
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
use std::path::Path;

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

/// Run `jigc migrate <path> --as <doctype>` (optional `--slug`) against `cwd`: locate
/// the repo + project layer, read the foreign file, mint the off-router migration
/// task, stage the foreign bytes, compose the `migrate-<doctype>` workflow over the
/// source seam, render the composed view through `format`, and print it. A clean run
/// exits 0; an unknown doctype, a malformed `--slug`, a source path the door refuses as
/// untrackable or as untracked, a missing foreign file, a serial collision, or a blocking
/// compose finding surfaces on stderr (with its route) and exits non-zero.
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

/// Adjudicate the verb's `<path>` argument **before anything mints**, answering with the
/// clean repo-relative spelling recorded as the retire target — or with the door's refusal
/// (M51 Increment 1 / T1; `design/auto-migration.md` → The `jigc migrate` verb).
///
/// Two jobs in one answer, and that is the point. The *normalization* half is review F2's: a
/// `./`-prefixed or absolute spelling collapses to the canonical `changelog/changelog.md`, so
/// the finalize retire's in-location-squatter guard compares like with like whatever the
/// caller typed — **and since M53 (the cwd census, C2-03) a relative token is resolved
/// against the caller's `cwd`, which is what every shell tool means by a path**. It was
/// joined to the repository root, so from `docs/deep` this door could not find the `note.md`
/// beside the caller, and answered `jigc migrate ../../rootnote.md` with
/// `migrate.source-untrackable` — *"resolves outside the repository"* about a file plainly
/// inside it, because the `..` was folded against the root instead of against where the
/// caller stood. The recorded value stays the clean repo-relative spelling; only the base of
/// the join moved. The *adjudication* half is this increment's, and it is the half that was
/// missing: the recorded value is the path `jigc task finalize --approve` **deletes**, and
/// until now the door normalized any token at all into it — an absolute path outside the
/// repository stayed absolute, and a `../` one above it was folded **lexically** into a
/// repo-relative spelling naming a file that is not the operator's. Both landed at exit 0.
///
/// The rule itself lives in [`crate::trackable::resolve_source_token`], one home for the door
/// and the sink; what is door-local is the code and the sentence. One code for every location
/// leg, with the reason in the message — `config.untrackable-root`'s precedent, where the
/// operator's fix is the same whichever leg answered — and a **`Human`** route, because no
/// argv resolves this state: only naming a different source does
/// (`settle-record.md` → §10, the destroying-door mold).
fn adjudicate_source_path(
    repo_root: &Path,
    cwd: &Path,
    path: &str,
    doctype: &str,
) -> Result<String> {
    crate::trackable::resolve_source_token(repo_root, cwd, path).map_err(|reason| {
        render::finding_error(&Finding::block(
            "migrate.source-untrackable",
            format!("`{path}` cannot be migrated as a `{doctype}` source: {reason}"),
            Route::human(
                "name a source this repository can record and recover: a file under the \
                 repository root, outside `.git/` and jigc's own `.jigc/` workbench, not \
                 reached through a symlink, and named in bytes git reads literally (no \
                 leading `:`, no `*`, `?`, `[` or `\\`)",
            ),
        ))
    })
}

/// Refuse a source **git has never recorded** — the fourth question, and the only one that is
/// not about location (M51 Increment 1 / T2; `settle-record.md` → §2, the trackedness leg).
///
/// [`adjudicate_source_path`]'s three predicates all ask *where* the source is. None asks
/// whether git knows about it, and the gap was not theoretical: driven end to end, an untracked
/// in-repo `HISTORY.md` migrated, authored and `--approve`d committed the canonical doc, removed
/// the source from the worktree, and left `git log --all -- HISTORY.md` **empty**. The deletion
/// was named on no surface, because [`crate::task`]'s migration staging deliberately skips a
/// retirement pathspec that matches nothing — a skip that is lossless for every *tracked* source
/// and total loss for this one. That falsified the recorded warrant under which the adapter deny
/// floor keeps its blanket permit for `migrate` + `finalize --approve` (*"a guarded migrate
/// destroys only a reviewed, in-repo, **git-recoverable** file"*); after this leg the warrant is
/// true rather than restated.
///
/// **The question is whether git holds a copy at all — the index *or* `HEAD`**
/// ([`crate::task::path_in_index`], the same call [`crate::task`]'s migration staging makes, and
/// [`crate::task::path_at_head`]). The index leg alone is what makes the refusal a narrowing
/// instead of a dead end: the single `git add` the route names is *sufficient*, so nobody has to
/// commit a foreign file in order to be allowed to retire it. The `HEAD` leg is the one the
/// Settle's shorthand (*"the index, not HEAD"*) did not name and the shipped product needs: a
/// source the user **committed and then pre-staged a `git rm` for** (M40 F7, `design/finalize.md`
/// → the retirement pathspec discriminated on the index) is absent from the index by design, and
/// `git ls-files` alone would refuse a file this repository has a perfectly good copy of. Asking
/// only the index would have turned a documented recovery path into a refusal; asking only `HEAD`
/// would have made the route a two-command story. The union is the predicate the warrant actually
/// names — *git-recoverable* — so it is the one asked.
///
/// The route is a [`Route::human`] naming `git add` verbatim — M45's `owner-artifact.present`
/// untracked cause is the precedent, and the reason is the route fence's: a `Route::mechanical`
/// argv must lead with `jigc`, and no `jigc` argv resolves this state.
///
/// **Its own code, not a fifth reason under `migrate.source-untrackable`.** That code's five
/// legs share one fix — *name a different source* — and this one's is the opposite: *keep this
/// source, and stage it*. A different act is a different identity (`settle-record.md` → §10's
/// table, which lists the two separately).
fn adjudicate_source_tracked(repo_root: &Path, recorded: &str, doctype: &str) -> Result<()> {
    if crate::task::path_in_index(repo_root, recorded)
        || crate::task::path_at_head(repo_root, recorded)
    {
        return Ok(());
    }
    Err(render::finding_error(&Finding::graded(
        Severity::Blocking,
        "migrate.source-untracked",
        format!(
            "`{recorded}` is in neither this repository's index nor its HEAD — git holds no \
             copy of it, and `jigc task finalize --approve` retires the source it migrates, \
             so the file would be deleted from the worktree with nothing to recover it from \
             and no deletion in the commit to say so"
        ),
        Some(Location::addressed(recorded, 1, 1)),
        Some(Route::human(format!(
            "stage it with `git add -- {token}`, then re-run \
             `jigc migrate {token} --as {doctype}` — the index is enough, the source \
             need not be committed first",
            token = crate::task::shell_token(recorded),
        ))),
    )))
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
        return Err(crate::locate::not_set_up(&ctx.project_config_path()));
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
    // …and the name-ceiling half (M51 Inc 9 / T3, EC-28). Driven at `d7ebbeb9` this door
    // took a 300-byte override at **exit 0** and minted a task, the value inert because the
    // `changelog` target is a singleton — so the ceiling is asked here for the same reason
    // the grammar is: the override is recorded and drives the created doc's id verbatim for
    // every non-singleton doctype, and a refusal before the mint strands no task dir.
    if let Some(slug) = slug_override {
        crate::task::reject_slug_over_name_ceiling(slug)?;
    }

    // Adjudicate the `<path>` argument BEFORE the read and BEFORE the mint — the same
    // mint-after-validate discipline as `ensure_migratable` and `--slug`, for a sharper
    // reason: the value this returns is the one `finalize --approve` deletes, so a token the
    // door has not adjudicated must never reach the working area at all, and a refusal must
    // strand no task dir to resume from.
    let recorded = adjudicate_source_path(&repo_root, cwd, path, doctype)?;

    // Read the foreign file's bytes (the source the seam carries). Resolve the path
    // against the repo root so a repo-relative `CHANGELOG.md` reaches the root file.
    //
    // A read fault is the one refusal whose subject is the string the operator typed and can
    // edit, so that spelling — and not the resolved `foreign_path` — is what the refusal
    // below quotes. Until the M51 completion audit (LOW 3) this comment stood over a message
    // rendering `foreign_path.display()`: an operator who typed `adir` was answered with
    // `/private/var/…/repo/adir`, a host path on a surface law 1 binds, and one they could
    // not paste back into the route it prints.
    // **The ADJUDICATED spelling is what is joined** (M53 — the cwd census, C2-03). It was
    // the caller's own token, on the warrant that the adjudication had proved the two name
    // the same file — true while a relative token was joined to the root, and false the
    // moment it is joined to the cwd: `note.md` typed from `docs/deep` names
    // `docs/deep/note.md`, and joining the raw token to the root would read the wrong file
    // or none at all. The refusal below still quotes `path`, the operator's own token, for
    // the reason the comment below it gives.
    let foreign_path = repo_root.join(&recorded);
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
            "could not read the foreign `{doctype}` source at `{path}`\n  route: check the path, then re-run {route} with a readable file"
        )
    })?;

    // …and the trackedness leg, asked of the adjudicated spelling. It is asked **after** the
    // read and not beside the three location legs: `git ls-files` answers "not in the index"
    // for a path that does not exist at all, so asking first would answer a typo'd `<path>`
    // with a `git add` that cannot match it, in place of the read fault that names the real
    // problem. It is still before the mint, which is the property that matters — a refusal
    // strands no task dir.
    adjudicate_source_tracked(&repo_root, &recorded, doctype)?;

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
    let source_path = minted.dir.join(state::SOURCE_FILE);
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
    use super::adjudicate_source_path;
    use std::path::Path;

    /// Review F2, carried forward onto the adjudicating door: the recorded `source-path` is
    /// normalized to a clean repo-relative form so the finalize retire's in-location-squatter
    /// guard compares canonically. A `./`-prefixed, an absolute (repo-root-prefixed), and a
    /// `..`-round-trip spelling of the canonical changelog path all collapse to
    /// `changelog/changelog.md`.
    ///
    /// The root is a path that does not exist, which is deliberate: it pins that the
    /// canonicalization the door added is a *best effort* over the real filesystem and never a
    /// precondition — an unresolvable root falls back to the raw comparison rather than
    /// refusing every source under it.
    ///
    /// **The cwd is the root here**, the one arrangement under which these spellings still
    /// mean what the case names say (M53 — the cwd census, C2-03). What the base *does* is
    /// driven from real subdirectories in `crates/cli/tests/cwd_verb_subject.rs`; this unit
    /// pins the normalization, which is a different claim and stays exactly as it was.
    #[test]
    fn records_a_clean_repo_relative_source_path() {
        let repo_root = Path::new("/abs/repo");
        for (typed, recorded) in [
            ("CHANGELOG.md", "CHANGELOG.md"),
            ("./changelog/changelog.md", "changelog/changelog.md"),
            ("/abs/repo/changelog/changelog.md", "changelog/changelog.md"),
            (
                "changelog/../changelog/changelog.md",
                "changelog/changelog.md",
            ),
        ] {
            assert_eq!(
                adjudicate_source_path(repo_root, repo_root, typed, "changelog")
                    .unwrap_or_else(|err| panic!("`{typed}` must be admitted: {err:#}")),
                recorded,
            );
        }
    }

    /// The refusing half, at the unit: a token that lands outside the repository is refused
    /// **as itself**, never folded back in. The `..` spelling is the one that mattered — it was
    /// recorded as the repo-relative `outside.md` and handed to the retire sink.
    ///
    /// Only the **placement** step is pinned here, because only it is answerable against a root
    /// that does not exist. The three predicates asked after it are all conservative toward the
    /// caller when git cannot be reached at all (`untrackable_reason` abstains outright on an
    /// unresolvable root), so a `.git/`-component or workbench cell asserted at this unit would
    /// be asserting the fixture rather than the rule; those cells are driven against a real
    /// repository in `crates/cli/tests/migrate_source_rules.rs`.
    #[test]
    fn a_source_outside_the_repository_is_refused_rather_than_folded_back_in() {
        let repo_root = Path::new("/abs/repo");
        for typed in ["../outside.md", "/elsewhere/keepme.md", "../../keepme.md"] {
            let err = adjudicate_source_path(repo_root, repo_root, typed, "changelog")
                .expect_err("a source that lands outside the repository is refused");
            assert!(
                format!("{err:#}").contains("migrate.source-untrackable"),
                "`{typed}` must be refused under the door's own code; got: {err:#}",
            );
        }
    }
}
