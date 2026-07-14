//! `jigc migrate-corpus` — the managed-corpus schema-migration verb (M34 Inc-3 T4).
//!
//! The **corpus-side** of the productive-readiness pair: a committed managed corpus at an
//! old schema version is detected (the Inc-1 fifth `validate` family + Inc-3 T3
//! version-aware route), and this verb **migrates** it onto the current schema —
//! byte-stable, per-doc gated, the stamp flipped last — consuming the engine
//! [`engine::transform::migrate_corpus`] fold (`design/corpus-migration.md` → The
//! deterministic transform / Acceptance flows).
//!
//! Disjoint from `jigc migrate` (foreign-doc *adoption*, LLM re-author) and `jigc upgrade`
//! (config-delta reconciliation): this is a managed *structural* schema-version upgrade (v0→v1
//! and v1→v2) over N committed instances, **CLI-owned and deterministic** (the determinism
//! boundary — no LLM in the structural path).
//!
//! # The live `add-field` dogfood
//!
//! The one genuinely pending shape change post-freeze is the **schema-version stamp**
//! itself: the engine injects an `added-optional-field` declaration into every frozen
//! persisted doctype, so a committed v0 corpus (authored before the stamp existed) is a
//! real `added-field` migration target. This verb runs the real schema-diff classifier
//! over the genuine corpus and applies the engine transform's `added-optional-field`
//! branch — the live e2e, not synthetic coverage.
//!
//! # Prior-schema sourcing + the v1→v2 value-bump
//!
//! A **below-version** stamped doc (a real v1→v2 transition) cannot derive its prior shape
//! from the current schema, so the verb sources `from` per committed doc by stamp from the
//! versioned snapshot store (`schema-snapshots/<ty>.v<k>.yaml`, via
//! [`crate::pack::load_prior_schema`]) — the engine diffs **two real declared schemas**, the
//! determinism boundary in its strongest form. The stamp, present in both shapes, is
//! **value-bumped** `k → current` ([`bump_and_regate`]) rather than added; a missing snapshot
//! **blocks** the doc with a route (never a silent `already-current`, closing the Inc-3
//! detector/verb divergence — `design/corpus-migration.md` → Prior-schema sourcing).
//!
//! # Per-doc transaction + stamp-flips-last
//!
//! Each doc is migrated in a scratch buffer and written back **only on a clean conformance
//! gate** (the engine fold's per-doc granularity). A doc whose migration mints an empty
//! required slot (a Framing-A prose-needing change) **fails the gate and rolls back** —
//! it stays byte-identical v0, unstamped, and re-detects `migrate`. So the schema-version
//! stamp **flips last**: it only lands once every structural splice *and* every
//! prose-pending slot is authored and the doc gates clean (`design/corpus-migration.md` →
//! the stamp-flips-last rule). A blocked doc is **routed to the agent** to author the prose
//! (Framing A); the agent authors it through the write verbs and re-runs the migration.

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::pack;
use crate::render;
use anyhow::{Context, Result};
use engine::file_state::{FileStateRecord, hash_bytes};
use engine::packsource::PackSource;
use engine::schema::{SCHEMA_VERSION_FIELD, Schema, SectionBody};
use engine::schema_diff::{SchemaChange, schema_diff};
use engine::transform::{CorpusDoc, CorpusMigration, DocOutcome, migrate_corpus};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One doctype's migration job: its current shape (`to`, stamp-injected) and its current
/// manifest schema-version (the value the stamp is filled/bumped to + the "already current"
/// threshold). The **prior** shape (`from`) is resolved **per committed doc by stamp** in
/// [`migrate_committed_corpus`] — stamp-absent (v0) docs derive it as `strip_stamp(to)`;
/// below-version (v1→v2) docs source it from the versioned snapshot store via
/// [`crate::pack::load_prior_schema`] — so it is not a per-doctype field.
pub(crate) struct DoctypeMigration {
    /// The doctype's id (the snapshot-store key + diagnostics).
    pub ty: String,
    /// The current schema shape, with the engine-injected schema-version stamp.
    pub to: Schema,
    /// The doctype's current manifest schema-version (the stamp value + currency floor).
    pub version: u32,
    /// The resolved `docs-root` prefix (`""` for a flat layout). Applied to the **relocation
    /// walk-home** only: a relocated doctype's committed instances sit under this prefix at
    /// the prior `location:` home, but the prior *snapshot* stores that location raw, so
    /// [`candidate_docs`] re-applies the prefix. The in-place `to.location` is
    /// already docs-root-resolved (via `all_schemas`), so this is inert there.
    pub docs_root: String,
}

/// The two operator flags of the commit boundary (`design/corpus-migration.md` → The commit
/// boundary). Both `false` — the default — is the standing behaviour: apply the migration's
/// writes, then land them in the pathspec-limited self-commit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Options {
    /// `--no-commit` — migrate and write, but stage and commit **nothing**, leaving the tree
    /// exactly as the git-free verb left it (an unstaged delete of a relocation source, an
    /// untracked destination), for an operator who wants to review the migration or fold it
    /// into a larger commit.
    pub no_commit: bool,
    /// `--dry-run` — **suppress the write**: no migrated bytes, no relocation move, no
    /// file-state re-baseline, and (necessarily) no commit. Not *"stop before a stage"*: there
    /// is no operator-facing `persist` stage to skip — [`engine::state::persist`] writes the
    /// bytes and flips the baseline in one motion — so the run simply does not apply the fold's
    /// outcomes. The fold is pure **and the destination-collision adjudication is derived from
    /// the run's own outcomes** (the claim ledger in [`migrate_committed_corpus`], not from a
    /// disk side effect this mode suppresses), so the triage report is the identical one an
    /// applying run prints.
    pub dry_run: bool,
}

impl Options {
    /// Whether this run **applies** the fold's outcomes to disk (bytes + relocation move +
    /// file-state baseline). `--dry-run` is exactly the suppression of this.
    pub(crate) fn writes(self) -> bool {
        !self.dry_run
    }

    /// Whether this run **lands** its writes in the self-commit ([`commit_migration`]).
    /// `--dry-run` **implies** no commit — nothing was written, so there is nothing to stage.
    pub(crate) fn commits(self) -> bool {
        !self.no_commit && !self.dry_run
    }
}

/// The outcome of a corpus migration run, rendered by [`render::corpus_migration`].
#[derive(Debug, serde::Serialize)]
pub struct CorpusMigrationReport {
    /// Docs migrated (stamped + structurally upgraded), written back byte-stable —
    /// repo-relative paths, sorted.
    pub migrated: Vec<String>,
    /// Docs already at the current schema-version (skipped, byte-untouched), sorted.
    pub already_current: Vec<String>,
    /// Docs that could not migrate cleanly (a prose-needing mint that blocks until
    /// authored, or a doc halted behind one), each with its route — sorted by path.
    pub blocked: Vec<(String, String)>,
    /// The short sha of the commit the verb landed its own migration in ([`commit_migration`]),
    /// or `None` when nothing was committed (nothing migrated, a re-run that staged no change,
    /// or a non-git worktree). Named in both surfaces — the operator/driver reads back *where*
    /// the migration landed (`design/corpus-migration.md` → The commit boundary).
    pub commit: Option<String>,
    /// The repo-relative paths the migration **touched** — every destination written *and*
    /// every relocation source removed. The self-commit's pathspec: the removed source is not
    /// recoverable from `migrated` (which carries only destinations), and staging the add half
    /// alone would land a **half-migration** — the silent-partial-commit class. Internal to the
    /// commit boundary, so it stays out of the report's serialized surface.
    #[serde(skip)]
    touched: Vec<String>,
}

/// Run `jigc migrate-corpus` against `cwd`: locate the repo + project layer, build the
/// frozen persisted doctypes' v0→v1 migration jobs from the pack, migrate the committed
/// corpus, **land it in a pathspec-limited self-commit** ([`commit_migration`]), render the
/// report through `format`, and print it. A clean run (even with blocked docs routed to the
/// agent) exits 0 — the migration writes; blocked docs are an expected interim state, not a
/// failure. A locator error — or a **rejected commit** (a `pre-commit` hook declining the
/// managed-doc writes) — routes to stderr and exits non-zero.
///
/// [`Options`] narrows what the run **applies**: `--no-commit` keeps the writes but lands
/// nothing; `--dry-run` suppresses the writes too, printing the identical report an applying
/// run would print.
pub fn run(cwd: &Path, format: Format, options: Options) -> Outcome {
    match migrate_in_repo(cwd, options) {
        Ok(report) => {
            println!("{}", render::corpus_migration(format, &report));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
    }
}

/// Locate the repo + project layer, assemble the frozen persisted doctypes' migration jobs
/// (each carrying its current stamp-injected shape `to` + its manifest schema-version; the
/// prior `from` is resolved per committed doc by stamp inside [`migrate_committed_corpus`]),
/// and migrate the committed corpus — applying and landing it as far as `options` allows.
fn migrate_in_repo(cwd: &Path, options: Options) -> Result<CorpusMigrationReport> {
    let jigc_home = require_project_layer(cwd)?;
    let pack = pack::make_pack()?;
    let pack = pack.as_ref();
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);
    let schemas = defs.all_schemas(pack)?;
    // The doctype → manifest schema-version map gates which doctypes migrate (the frozen
    // persisted set) and supplies each stamp's value — the same authority the version-aware
    // detector routes against (`design/validation.md` → Version-aware routing).
    let versions = pack::frozen_doctype_versions(pack);
    // The resolved docs-root prefix — re-applied to a relocated doctype's prior (snapshot,
    // raw) `location:` walk-home so the committed instances under `docs-root` are found.
    let docs_root = crate::start::docs_root_prefix(&resolved).to_string();

    let mut doctypes = Vec::new();
    for (ty, to) in schemas {
        // Only the frozen persisted doctypes carry the stamp + a migration target; a
        // transient doctype (neither `location:` nor `placement:` — e.g. `commit`) or a
        // non-frozen one migrates nothing. **Persisted** is `location OR placement` — a
        // relocated doctype (M38 changelog: `placement: CHANGELOG.md`, `location: None`)
        // must reach the job list so its instances relocate (the sibling `pack.rs`
        // stamp-inject gate uses the same `location.is_some() || placement.is_some()`
        // idiom; `design/storage.md` → Placement). `candidate_docs` then walks the
        // prior `location:` home and moves each instance to the placement `file` — or,
        // for a placement-born doctype (no prior location), migrates the literal
        // placement file in place.
        //
        // **The frozen gate — reconciled with the freeze-exempt sibling (M39 inc-5 T4).**
        // This relocation arm gates to `frozen_doctype_versions` (the manifest set): its prior
        // home is *derived* from the versioned snapshot (`candidate_docs`), so it is
        // safe to auto-move. A **freeze-exempt** doctype (no manifest entry, no snapshot) has
        // no derivable prior home, so it is skipped here and relocates through the parallel
        // `crate::relocate::relocate_freeze_exempt` path with a **human-supplied** prior home
        // (`design/corpus-migration.md` → Relocation: freeze-exempt sibling).
        let Some(&version) = versions.get(&ty) else {
            continue;
        };
        if to.location.is_none() && to.placement.is_none() {
            continue;
        }
        doctypes.push(DoctypeMigration {
            ty,
            to,
            version,
            docs_root: docs_root.clone(),
        });
    }
    // Deterministic doctype order (the committed walk + the fold both consume it in order).
    doctypes.sort_by(|a, b| a.ty.cmp(&b.ty));

    let jigc_root = jigc_home.join(".jigc");
    let mut report = migrate_committed_corpus(pack, &jigc_home, &jigc_root, &doctypes, options)?;
    // THE COMMIT BOUNDARY (`design/corpus-migration.md` → The commit boundary). The per-doc
    // fold above stays **git-free** (the relocation is an `fs::rename`, not a `git mv`);
    // staging happens **once**, here, over exactly the paths the migration touched — so git's
    // rename-detection collapses the move's delete+add into the clean `R` the relocation note
    // predicts, and the operator never has to reach for the raw `git add -A` the adapter
    // contract forbids. `--no-commit` opts out of it (the writes stand, unlanded); `--dry-run`
    // implies it (nothing was written, so there is nothing to stage).
    if options.commits() {
        report.commit = commit_migration(&jigc_home, &report.touched)?;
    }
    Ok(report)
}

/// The message the verb's self-commit carries (the [`crate::setup`] install-commit precedent:
/// one fixed, conventional subject — the verb's writes are one kind of change).
const MIGRATION_COMMIT_MESSAGE: &str =
    "chore(jigc): migrate the managed corpus to the current schema versions";

/// **Land the migration** — stage exactly `touched` (every destination written and every
/// relocation source removed) and commit those paths, returning the short sha (`None` when
/// nothing was committed).
///
/// The **mold is [`crate::setup`]'s install commit** (`setup.rs` → `commit_install`), the one
/// other verb that commits its own writes: a **pathspec-limited** `git add -- <paths>` —
/// **never** a blanket `git add -A`, so an ambient dirty tree is never swept in — a
/// `git diff --cached --quiet -- <paths>` idempotence check, so a clean re-run commits nothing
/// (no empty commit), and a **pathspec-limited** `git commit -- <paths>`, which leaves any
/// changes the operator had already staged staged and uncommitted.
///
/// **The hook posture is `finalize`/`rename`'s, not setup's** — this commit is **never**
/// `--no-verify`. Setup's `--no-verify` rests on a rationale that does *not* transfer: its
/// commit *"carries install artifacts, not managed docs"* (and its hook must not self-trigger
/// on the commit that installs it). This one carries **managed docs**, so the user's hooks are
/// **policy** ([`crate::task::git_commit`] / [`crate::rename`]): a hook rejection surfaces
/// git's stderr verbatim and **fails the run loudly**, never a silent skip behind a success
/// banner.
///
/// A **non-git worktree** is the one benign skip (the migration's writes still stand; there is
/// simply nothing to land). Every other git failure — a rejected `add`, a rejected `commit` —
/// is an `Err`.
fn commit_migration(repo_root: &Path, touched: &[String]) -> Result<Option<String>> {
    if touched.is_empty() {
        return Ok(None);
    }
    // No git work tree → nothing to land (the docs are written; the commit is the convenience).
    if !git_ok(repo_root, &["rev-parse", "--is-inside-work-tree"]) {
        return Ok(None);
    }
    let paths: Vec<&str> = touched
        .iter()
        .filter(|p| stageable(repo_root, p))
        .map(String::as_str)
        .collect();
    if paths.is_empty() {
        return Ok(None);
    }

    // Stage exactly those paths — never a blanket `git add -A`. A tracked-and-removed
    // relocation source stages as its deletion here (the move's other half).
    let mut add: Vec<&str> = vec!["add", "--"];
    add.extend(&paths);
    git_run(repo_root, &add)?;

    // Nothing staged among our paths (a re-run over an already-migrated corpus) → no commit,
    // no empty commit. `git diff --cached --quiet` exits 0 when there is no staged diff.
    let mut diff: Vec<&str> = vec!["diff", "--cached", "--quiet", "--"];
    diff.extend(&paths);
    if git_ok(repo_root, &diff) {
        return Ok(None);
    }

    let mut commit: Vec<&str> = vec!["commit", "-m", MIGRATION_COMMIT_MESSAGE, "--"];
    commit.extend(&paths);
    git_run(repo_root, &commit)?;

    let sha = git_stdout(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(Some(sha))
}

/// Whether git can stage `path`: it is **tracked** (so a *removed* relocation source stages as
/// its deletion), or it exists on disk and is not gitignored (so a freshly-written destination
/// stages as an add/modify). Anything else — an untracked source the migration removed, a doc
/// under a gitignored tree — would make `git add` fail on a pathspec it cannot match; it is
/// dropped from the pathspec instead, since there is nothing there for git to record.
fn stageable(repo_root: &Path, path: &str) -> bool {
    if git_ok(repo_root, &["ls-files", "--error-unmatch", "--", path]) {
        return true;
    }
    repo_root.join(path).exists() && !git_ok(repo_root, &["check-ignore", "-q", "--", path])
}

/// Whether `git <args>` ran **and** exited 0 (a git that could not be spawned reads as `false`)
/// — the predicate form, for the probes whose failure is a fact, not an error.
fn git_ok(repo_root: &Path, args: &[&str]) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Run `git <args>`, bailing **loudly** with git's own stdout+stderr when it rejects — the
/// hook-rejection channel (the `finalize`/`rename` posture: the user's hooks are policy, and
/// their verbatim output is the correction signal).
fn git_run(repo_root: &Path, args: &[&str]) -> Result<()> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .with_context(|| format!("could not run `git {}` (is git on PATH?)", args[0]))?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` was rejected — the migration is written to disk but NOT committed:\n{}{}",
            args[0],
            String::from_utf8_lossy(&out.stdout).trim(),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    Ok(())
}

/// Run `git <args>` and return its trimmed stdout, bailing on a non-zero exit.
fn git_stdout(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .with_context(|| format!("could not run `git {}` (is git on PATH?)", args[0]))?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args[0],
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Migrate the committed corpus under `repo_root` for each [`DoctypeMigration`], per-doc
/// gated and WIP-safe, re-baselining any rewritten doc that already carried a file-state
/// hash. `jigc_root` is the `.jigc/` dir holding the file-state record; `pack` is the
/// snapshot store the below-version prior shapes are sourced from.
///
/// The **prior shape (`from`) is resolved per committed doc by its schema-version stamp**
/// (`design/corpus-migration.md` → Prior-schema sourcing):
/// - **at-or-above** the doctype's current version → `already-current`, byte-untouched.
/// - **stamp absent** (the v0 corpus state) → `from = strip_stamp(to)`, the unchanged
///   `added-optional-field` stamp path (the live Inc-3 dogfood); the stamp is *added*.
/// - **below-version** (`1 ≤ k < current`, a v1→v2 transition) → `from` = the versioned
///   snapshot `schema-snapshots/<ty>.v<k>.yaml` via [`crate::pack::load_prior_schema`]; the
///   structural diff is applied **and** the stamp is **value-bumped** `k → current` (it
///   already exists, so this is a `set_field` splice, not an add-field). A **missing**
///   snapshot **blocks** that doc with a route — never a silent `already-current` (closing
///   the Inc-3 detector/verb divergence; [`DECISIONS.md`] → 2026-06-25 audit Finding 2).
///
/// Each doc's change list is the schema-diff filtered to what it still needs (a prose slot
/// already authored is dropped, so it flips cleanly on a re-run). The engine fold applies
/// them, gates each doc on conformance against the current schema, and commits only the
/// clean ones; this writes the migrated bytes back to disk and flips the file-state baseline.
///
/// Under [`Options::writes`] `== false` (`--dry-run`) the fold still runs — it is **pure** —
/// and the report is built exactly as an applying run would build it; only the on-disk
/// application (the byte write, the relocation move, the file-state re-baseline) is suppressed.
/// The one part of the report that is *not* a function of the fold alone — the
/// destination-collision adjudication, which asks what the destination holds — reads the run's
/// **claim ledger** before disk, so a destination an earlier doc in this run migrated onto is
/// seen by the next candidate whether or not the write was persisted (without it, `--dry-run`
/// reported a second candidate for a shared destination `migrated` where the applying run
/// reports it `blocked`).
pub(crate) fn migrate_committed_corpus(
    pack: &dyn PackSource,
    repo_root: &Path,
    jigc_root: &Path,
    doctypes: &[DoctypeMigration],
    options: Options,
) -> Result<CorpusMigrationReport> {
    let mut report = CorpusMigrationReport {
        migrated: Vec::new(),
        already_current: Vec::new(),
        blocked: Vec::new(),
        commit: None,
        touched: Vec::new(),
    };

    // Prepare every candidate doc across the corpus (heterogeneous: each carries its own
    // schema pair + per-doc change list), collected and path-sorted so the fold — which
    // halts at the first blocker — is deterministic.
    let mut prepared: Vec<PreparedDoc> = Vec::new();
    for dt in doctypes {
        // The stamp's value comes from the doctype's manifest version: thread it in as the
        // field `default` so the v0 add-field branch (which has no value source for a bare
        // `set`-derived field) places it deterministically. Inert for the below-version
        // path (the stamp is already present there — it is value-bumped, not added).
        let to = with_stamp_default(&dt.to, dt.version);
        // THE DIFF-SIDE SHAPE — docs-root-free, so the two sides of the schema-diff are
        // comparable. `to` comes from `CascadeDefs::all_schemas`, whose last act is
        // `apply_docs_root` (`location: docs/decisions/`), while the prior snapshot
        // `from` stores its `location:` **raw** (`decisions/`) — and the engine's
        // `resolved_home` is docs-root-*blind* by contract (the raw schema-declared home;
        // resolution is the CLI's concern). Diffing the resolved `to` against the raw
        // `from` therefore fired a **spurious** `SchemaChange::Relocated` on *every*
        // below-version migration of a `location:`-bearing doctype under the shipped
        // default `docs-root: docs/` — which moves nothing (it is a byte no-op, and the
        // relocation destination is `candidate_docs`' concern, not the diff's) but is
        // enough to make the diff non-empty, so the residual — **the empty-diff backstop**
        // and its `PresentationOnly` sibling — was unreachable for 9 of the 14 persisted
        // doctypes: an unclassifiable structural change silently restamped the corpus.
        // Strip the prefix back off here and `Relocated` fires only on a genuine
        // `location:` / `placement:` change (the docs-root knob is project config, never a
        // schema relocation — a `config set docs-root` move routes through
        // `crate::relocate`).
        let to_diff = docs_root_free(&to, &dt.docs_root);
        // Each candidate is `(source, destination)` — the FROM home the walk found the
        // committed instance at, and the path the gated bytes land at
        // (`design/corpus-migration.md` → Relocation: the walk keys on the from home).
        for (rel_key, target_key) in candidate_docs(pack, repo_root, dt) {
            let Ok(bytes) = std::fs::read(repo_root.join(&rel_key)) else {
                continue; // read race: skip; the next run re-checks.
            };
            let source = String::from_utf8_lossy(&bytes).into_owned();
            match read_stamp_from_source(&source) {
                // At or above the current version: already current, byte-untouched.
                Some(s) if s >= dt.version => report.already_current.push(rel_key),
                // A below-version stamp (a v1→v2 transition): source the prior shape from
                // the versioned snapshot store and value-bump the stamp `k → current`. A
                // missing snapshot blocks the doc — never a silent already-current (the
                // Finding-2 fix: a doc the version-aware detector routes `migrate` is
                // migrated by the verb).
                Some(k) => match crate::pack::load_prior_schema(pack, &dt.ty, k) {
                    Ok(from) => {
                        // Both sides docs-root-free: `from` is the raw snapshot, `to_diff`
                        // the raw current shape (see above).
                        let diff = schema_diff(&from, &to_diff);
                        // THE EMPTY-DIFF BACKSTOP (`design/corpus-migration.md` → The empty-diff
                        // backstop — no silent bump). The classifier signals a schema pair whose
                        // conformance-relevant projection **moved** with **no transform kind** to
                        // apply. An empty diff is not a no-op: migrating anyway folds zero bytes,
                        // value-bumps the stamp, and lands the corpus at *v2 failing its own
                        // gate* — silently. So the migration is **refused** here, at
                        // classification, on the **raw** diff (before `per_doc_changes` filters
                        // it) and never through the fold's halt path — whose `prose_needing_route`
                        // ("author the prose, then re-run") would be a lie: there is no prose to
                        // author, and re-running changes nothing. The route names the real repair:
                        // build the transform kind first. This verb is the only surface that loads
                        // the prior snapshot the diff needs, which is why the refusal lives here —
                        // a *migration refusal*, not a build error.
                        if diff.contains(&SchemaChange::Unclassified) {
                            let route =
                                unclassifiable_change_route(&rel_key, &dt.ty, k, dt.version);
                            report.blocked.push((rel_key, route));
                            continue;
                        }
                        // THE NARROWING REFUSAL (the recorded pick — `DECISIONS.md` →
                        // 2026-07-13 M42 Inc-5 T3). A `card` narrowing is content-affecting: a
                        // committed instance may carry more values than the new bound admits.
                        // Refused at classification, beside the backstop and for the same
                        // reason — the fold's halt route ("author the prose, then re-run") would
                        // be a lie, and folding it (the pre-M42 direction-blind behaviour)
                        // restamps the doc past the gate. The route names the schema-authoring
                        // repair, not a doc instruction.
                        if let Some(SchemaChange::NarrowedCardinality { section, field }) = diff
                            .iter()
                            .find(|c| matches!(c, SchemaChange::NarrowedCardinality { .. }))
                        {
                            let route = narrowed_cardinality_route(
                                &rel_key, &dt.ty, section, field, k, dt.version,
                            );
                            report.blocked.push((rel_key, route));
                            continue;
                        }
                        // THE REMOVAL REFUSAL (the recorded pick — `DECISIONS.md` → 2026-07-13
                        // M42 Inc-5 T5: refuse, not strip). A dropped leaf is refused here, beside
                        // the backstop and the narrowing and for the same reason: there is nothing
                        // an operator can do to *this doc*, so the fold's halt route would be a
                        // lie, and the repair is a schema-authoring one. The committed value stays
                        // on disk — **No-data-loss** is a declared property of this pair.
                        if let Some(SchemaChange::RemovedField { section, field }) = diff
                            .iter()
                            .find(|c| matches!(c, SchemaChange::RemovedField { .. }))
                        {
                            let route = removed_field_route(
                                &rel_key, &dt.ty, section, field, k, dt.version,
                            );
                            report.blocked.push((rel_key, route));
                            continue;
                        }
                        // The v1→v2 path is the only one that can surface a `ValueRemapped`
                        // (an enum member rename needs two *different* declared enum sets;
                        // the stamp-absent path diffs `strip_stamp(to)` against `to`, whose
                        // members are identical). The classifier emits the variant with an
                        // **empty** map (it detects only *that* the members moved); the CLI
                        // supplies the authored old→new map before the fold.
                        let changes =
                            enrich_value_remaps(per_doc_changes(&diff, &source, false), &dt.ty);
                        // The stamp is value-bumped `k → current` **post-fold** (it already
                        // exists, so it is a `set_field` value splice, not an add-field) — on
                        // the gated v2 bytes, which are guaranteed to conform to `to` (the
                        // committed source's own shape may be the prior `from`, so it cannot be
                        // bumped directly; the gated v2 always can). `bump_to` carries the
                        // target version into the post-fold pass.
                        prepared.push(PreparedDoc {
                            rel_key,
                            target_key,
                            source,
                            from,
                            to: to.clone(),
                            changes,
                            bump_to: Some(dt.version),
                        });
                    }
                    Err(_) => {
                        let route = missing_snapshot_route(&rel_key, &dt.ty, k);
                        report.blocked.push((rel_key, route));
                    }
                },
                // Stamp absent (the v0 corpus state): the unchanged add-field path (the stamp
                // is *added* at the current value via its `default`, so no post-fold bump).
                None => {
                    // Derived from the diff-side (docs-root-free) shape, so this pair is
                    // docs-root-consistent by construction — the same invariant the
                    // below-version arm restores by stripping the prefix off `to`.
                    let from = strip_stamp(&to_diff);
                    let changes = per_doc_changes(&schema_diff(&from, &to_diff), &source, true);
                    if changes.is_empty() {
                        // Nothing this doc needs (its shape already matches): leave it.
                        report.already_current.push(rel_key);
                        continue;
                    }
                    prepared.push(PreparedDoc {
                        rel_key,
                        target_key,
                        source,
                        from,
                        to: to.clone(),
                        changes,
                        bump_to: None,
                    });
                }
            }
        }
    }
    prepared.sort_by(|a, b| a.rel_key.cmp(&b.rel_key));

    let corpus: Vec<CorpusDoc> = prepared
        .iter()
        .map(|p| CorpusDoc {
            id: &p.rel_key,
            old_schema: &p.from,
            new_schema: &p.to,
            source: &p.source,
            changes: &p.changes,
        })
        .collect();
    let result: CorpusMigration = migrate_corpus(&corpus);

    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("loading the file-state record at {jigc_root:?}"))?;
    // THE CLAIM LEDGER: destination → the migrated bytes an *earlier doc in this run* landed
    // there. The collision rule below adjudicates against the destination's bytes **as this run
    // leaves them**, which on disk is only half the story: under `--dry-run` the write is
    // suppressed, so a second candidate for the same destination would read the destination as
    // it was *before* the run (absent, typically) and be reported `migrated` — the same target
    // twice — where the applying run reports it `blocked`. That falsifies dry-run's whole
    // contract (*the identical triage report an applying run prints*). The ledger is recorded in
    // **both** modes, so the adjudication is a function of the run's own outcomes, not of a disk
    // side effect the mode suppresses.
    let mut claimed: BTreeMap<String, String> = BTreeMap::new();
    for (i, outcome) in result.docs.iter().enumerate() {
        match outcome {
            DocOutcome::Migrated { id, v2 } => {
                // A below-version doc value-bumps its schema-version stamp `k → current` on
                // the gated v2 bytes, then re-gates (the bump is a byte-stable value splice
                // that keeps the doc conformant — asserted, not assumed). A stamp-absent
                // (v0) doc already carries the current stamp from the add-field branch.
                let prep = &prepared[i];
                let v2 = match prep.bump_to {
                    Some(version) => bump_and_regate(&prep.to, v2, version)?,
                    None => v2.clone(),
                };
                // A **relocated** doctype's destination differs from its source home; an
                // in-place migration writes back to the same key (`target_key == rel_key`).
                let target = &prep.target_key;
                let moved = target != id;

                // THE DESTINATION-COLLISION RULE (M42 — the walk union makes the both-homes
                // state reachable; `corpus-migration.md` → the union). A relocating doc must
                // never clobber a document already sitting at its destination:
                //   - the destination already holds **exactly these migrated bytes** → an
                //     *interrupted move* (write-before-remove aborted between its halves):
                //     complete it — the write below is a byte-identical no-op and the
                //     old-home strand is removed;
                //   - the destination holds **anything else** → a genuine collision: the doc
                //     is BLOCKED with a route, nothing written and nothing removed
                //     (No-data-loss, the declared property — the operator reconciles).
                // The destination's bytes are read **as this run leaves them**: the claim ledger
                // first (a destination an earlier doc in this run already migrated onto — the
                // bytes the applying run wrote there, and the bytes `--dry-run` *would* have),
                // then disk (a destination that was already committed). Never candidate order.
                if moved {
                    let existing = match claimed.get(target) {
                        Some(bytes) => Some(bytes.as_bytes().to_vec()),
                        None => std::fs::read(repo_root.join(target)).ok(),
                    };
                    if existing.is_some_and(|existing| existing != v2.as_bytes()) {
                        report
                            .blocked
                            .push((id.clone(), destination_collision_route(id, target)));
                        continue;
                    }
                }
                // This doc claims the destination — for every later candidate that shares it,
                // and in both modes (see THE CLAIM LEDGER above).
                claimed.insert(target.clone(), v2.clone());

                // THE DRY-RUN GATE (`corpus-migration.md` → The commit boundary: `--dry-run`).
                // Everything above is pure (the fold, the conformance gate, the collision
                // adjudication read from disk); everything below **applies** the outcome —
                // the byte write, the relocation move, the file-state re-baseline. `--dry-run`
                // suppresses exactly that, so the doc is reported migrated and the disk is
                // left byte-identical.
                if options.writes() {
                    // WRITE-BEFORE-REMOVE (`corpus-migration.md` → Relocation: write-to-`to`
                    // precedes remove-`from`, so an abort between strands neither copy). The
                    // gated v2 bytes land at the destination **first** — a fault here leaves
                    // the source at `id` untouched on disk (never zero copies). Only once `to`
                    // exists is the old-home source removed and the file-state re-keyed, one
                    // atomic per-doc unit.
                    engine::state::persist(&repo_root.join(target), v2.as_bytes())
                        .with_context(|| format!("writing the migrated doc {target}"))?;
                    if moved {
                        std::fs::remove_file(repo_root.join(id))
                            .with_context(|| format!("removing the relocated source {id}"))?;
                    }

                    // Re-baseline / re-key the file-state, paired **per doc** with this doc's
                    // write so the on-disk baseline always matches the on-disk files (the
                    // per-doc-gated transaction granularity — `corpus-migration.md` → Migration
                    // atomicity / WIP-safety; `reconciliation.md` → the file-state re-key). A
                    // **moved**, tracked doc drops its old-home key and re-registers at the
                    // destination `from → to`, so the instance re-registers and none is
                    // orphaned; an in-place tracked doc re-hashes at its key; an untracked doc
                    // adopts nothing (the migration tracks nothing it didn't already track).
                    if moved {
                        if record.forget(id) {
                            record.record(target.clone(), hash_bytes(v2.as_bytes()));
                            record.save(jigc_root).with_context(|| {
                                format!("saving the file-state record at {jigc_root:?}")
                            })?;
                        }
                    } else if record.get(id).is_some() {
                        record.record(id.clone(), hash_bytes(v2.as_bytes()));
                        record.save(jigc_root).with_context(|| {
                            format!("saving the file-state record at {jigc_root:?}")
                        })?;
                    }
                }
                // A completed interrupted move: the destination was *itself* enumerated by the
                // union and reported `already-current` (it is stamped current). The completion
                // supersedes that line — the target is reported once, as migrated.
                if moved {
                    report.already_current.retain(|k| k != target);
                }
                report.migrated.push(target.clone());
                // The self-commit's pathspec: **both** halves of the write — the destination
                // just persisted and, for a relocation, the source just removed. Staging only
                // the add half would land a half-migration (`design/corpus-migration.md` → The
                // commit boundary).
                report.touched.push(target.clone());
                if moved {
                    report.touched.push(id.clone());
                }
            }
            DocOutcome::Untouched { id, .. } => {
                let route = if result.halted_at == Some(i) {
                    prose_needing_route(id)
                } else {
                    deferred_route(id)
                };
                report.blocked.push((id.clone(), route));
            }
        }
    }

    report.migrated.sort();
    report.already_current.sort();
    report.blocked.sort();
    // A stable, deduped pathspec (a destination shared by a completed interrupted move is
    // enumerated once) — the staging order never varies between runs.
    report.touched.sort();
    report.touched.dedup();
    Ok(report)
}

/// One prepared migration job, owning its source + schema pair + per-doc change list (the
/// fold borrows them).
struct PreparedDoc {
    /// The doc's **source** home — where the committed instance sits (the fold's id, the
    /// bytes read, and the file removed after a relocation move).
    rel_key: String,
    /// The doc's **destination** home. Equal to `rel_key` for an in-place migration; the
    /// literal placement `to` for a relocated doctype (the file the gated v2 bytes are
    /// written to; a move iff `target_key != rel_key`).
    target_key: String,
    source: String,
    from: Schema,
    to: Schema,
    changes: Vec<SchemaChange>,
    /// `Some(version)` for a below-version doc whose schema-version stamp is value-bumped to
    /// `version` post-fold (over the gated v2 bytes); `None` for a stamp-absent (v0) doc,
    /// whose stamp is *added* at the current value by the add-field branch.
    bump_to: Option<u32>,
}

/// The per-doc change list: the doctype's `fixed` schema-diff filtered to what this doc
/// still needs — **the re-run-safety filter** (`design/corpus-migration.md` → The stranding
/// defect: a migration is not re-runnable; the property census → Re-run safety / convergence).
/// The schema-version stamp add-field is kept iff the doc carries no stamp.
///
/// **Every heading-minting kind is dropped iff the doc already carries that `## Heading`.**
/// [`engine::write::generate_section`] — the block-insert every such kind splices through —
/// **refuses** an already-present section (`GenerateError::AlreadyPresent`), so an unfiltered
/// kind does not duplicate the heading: it *halts the doc*, which then collects a route
/// ("author the new required prose … then re-run") that is a **dead end** when the section is
/// optional and already authored — there is nothing to write and re-running changes nothing.
/// Pre-M42 only `ProseNeeding` was filtered and `_ => true` let the rest through, which
/// **permanently stranded** any doc that had hand-authored the added optional section (live in
/// the shipped `adr` v1→v2 `options` migration).
///
/// The match is **exhaustive on purpose**: a new transform kind must declare whether it mints a
/// heading, rather than inheriting a catch-all that silently strands the docs that already carry
/// it.
///
/// `ProseNeeding { leaf: Some(..) }` — a new *required field*, which mints no heading of its own
/// — is filtered on its section's heading too, unchanged from pre-M42: dropping it hands the doc
/// to the per-doc conformance gate, which blocks it with the right route (the missing field),
/// rather than to `generate_section`'s refusal.
fn per_doc_changes(fixed: &[SchemaChange], source: &str, stamp_absent: bool) -> Vec<SchemaChange> {
    fixed
        .iter()
        .filter(|change| match change {
            // The schema-version stamp: added only to a doc that carries none (a below-version
            // doc already has it — it is value-bumped post-fold instead).
            SchemaChange::AddedOptionalField { field, .. } if field == SCHEMA_VERSION_FIELD => {
                stamp_absent
            }
            // THE HEADING-MINTING KINDS — dropped iff the doc already carries the heading.
            SchemaChange::AddedOptionalSection { section }
            | SchemaChange::AddedRepeatableSection { section }
            | SchemaChange::ProseNeeding { section, .. } => !has_section_heading(source, section),
            // Every other kind splices inside an existing section (or no bytes at all), so it
            // has no heading to collide with and is always kept. `AddedItemField` mints no
            // heading either — it splices a field bullet **into each item of an existing
            // repeatable section** — and it carries its own re-run guard *per item* (the driver
            // skips an item that already has the bullet, which a whole-change filter here could
            // not express: one doc can hold both kinds of item).
            SchemaChange::AddedOptionalField { .. }
            | SchemaChange::AddedItemField { .. }
            | SchemaChange::OptionalRelaxed { .. }
            | SchemaChange::WidenedCardinality { .. }
            | SchemaChange::NarrowedCardinality { .. }
            | SchemaChange::RemovedField { .. }
            | SchemaChange::EnumWidened { .. }
            | SchemaChange::ValueRemapped { .. }
            | SchemaChange::FixedSlotToRepeatable { .. }
            | SchemaChange::Relocated { .. }
            | SchemaChange::DisplayTitleChanged { .. }
            | SchemaChange::PresentationOnly
            | SchemaChange::Unclassified => true,
        })
        .cloned()
        .collect()
}

/// Fill each emitted [`SchemaChange::ValueRemapped`]'s authored old→new map from the
/// CLI-supplied remap table ([`authored_remap`], keyed by doctype/section/field). The
/// classifier emits the variant with an **empty** map — it detects only *that* the enum
/// members moved (`[D, I]` → `[Decision, Idea]`), never *which* old value maps onto *which*
/// new one, an assignment unrecoverable from the schema pair alone. The CLI supplies that
/// semantic choice as a deterministic migration input — the `with_stamp_default` precedent
/// for a value-source the classifier emits blank; the determinism *boundary* holds (a fixed
/// table, not an LLM call) (`design/corpus-migration.md` → the structural-auto /
/// value-semantic-authored distinction).
///
/// A rename with **no authored entry** keeps the empty map, so the engine driver blocks the
/// doc loudly on its first committed value (`TransformError::Unsupported`) — never a silent
/// strand: a schema bump that omits its map is surfaced, not swallowed
/// ([`DECISIONS.md`] → 2026-07-11 M41 Settle Fork 4).
fn enrich_value_remaps(changes: Vec<SchemaChange>, ty: &str) -> Vec<SchemaChange> {
    changes
        .into_iter()
        .map(|change| match change {
            SchemaChange::ValueRemapped {
                section,
                field,
                map,
            } if map.is_empty() => {
                let map = authored_remap(ty, &section, &field).unwrap_or(map);
                SchemaChange::ValueRemapped {
                    section,
                    field,
                    map,
                }
            }
            other => other,
        })
        .collect()
}

/// The CLI-authored old→new value maps for enum-member renames — keyed by
/// `(doctype, section, field)`. An enum rename is a semantic choice the schema pair cannot
/// recover, so the mapping is declared here (the migration input the classifier emits
/// blank), not derived. Returns `None` for any (doctype, section, field) with no authored
/// rename, leaving the classifier's empty map — the driver then blocks that doc loudly.
///
/// The one authored entry: the M41 F4 `deferral-ledger` `entries.kind` rename `D`→`Decision`
/// / `I`→`Idea` (the first methodology v1→v2 migration).
fn authored_remap(ty: &str, section: &str, field: &str) -> Option<BTreeMap<String, String>> {
    match (ty, section, field) {
        ("deferral-ledger", "entries", "kind") => Some(BTreeMap::from([
            ("D".to_string(), "Decision".to_string()),
            ("I".to_string(), "Idea".to_string()),
        ])),
        _ => None,
    }
}

/// Whether `source` carries a body section heading (`## …`) matching `section_id` under the
/// engine's heading↔id **recognition** rule (`slug::renormalize(heading) == id`) — the same
/// inverse the parser uses, so a multi-word id round-trips. Never the *mint* rule
/// (`slugify`, which caps and drops edge stopwords): this is the migration engine's own
/// idempotency guard, and reading an authored `## In Scope` as absent would route a doc back
/// for prose it already carries. Used to decide whether a prose-needing slot is already
/// authored (independent of a full parse, which a still-incomplete doc fails).
fn has_section_heading(source: &str, section_id: &str) -> bool {
    source.lines().any(|line| {
        line.strip_prefix("## ")
            .is_some_and(|text| engine::slug::renormalize(text.trim()) == section_id)
    })
}

/// Read a committed doc's schema-version stamp from its leading `---` front-matter block, if
/// present and integer-valued. Independent of a full parse so an incomplete (mid-migration)
/// doc's stamp is still legible. A doc with no front-matter fence, or no `schema-version`
/// line, yields `None` (the v0 corpus state).
fn read_stamp_from_source(source: &str) -> Option<u32> {
    let body = source.strip_prefix("---\n")?;
    let end = body.find("\n---")?;
    body[..end].lines().find_map(|line| {
        line.strip_prefix(&format!("{SCHEMA_VERSION_FIELD}:"))
            .and_then(|v| v.trim().parse::<u32>().ok())
    })
}

/// A clone of `schema` with the engine schema-version stamp field's `default` set to
/// `version` — so the transform's `added-optional-field` branch (which reads `default` for
/// the value) places the stamp deterministically. The stamp's shape is otherwise
/// unchanged; this is only the migration-time value source the CLI threads in.
fn with_stamp_default(schema: &Schema, version: u32) -> Schema {
    let mut out = schema.clone();
    for section in &mut out.sections {
        if let SectionBody::Simple { fields, .. } = &mut section.body
            && let Some(field) = fields.iter_mut().find(|f| f.id == SCHEMA_VERSION_FIELD)
        {
            field.default = Some(version.to_string());
        }
    }
    out
}

/// **Value-bump** a migrated doc's schema-version stamp to `version` and **re-gate** the
/// result — the v1→v2 stamp transition (`design/corpus-migration.md` → the stamp value-bump),
/// distinct from the v0→v1 add-field path (the field already exists, so `added-field` does not
/// cover it). Called **post-fold** over the gated v2 bytes, which conform to `schema` (the
/// committed source's own shape may still be the prior `from`, so it cannot be bumped
/// directly; the gated v2 always can).
///
/// The bump is [`engine::write::set_field`] — a present-field value splice (byte-stable: it
/// replaces only the value digits after `schema-version:`, every other byte intact). The
/// re-gate asserts the bumped bytes still conform (they do — the stamp's *value* is not a
/// conformance constraint), proving byte-stability + conformance rather than trusting reuse.
/// A failure (a snapshot drift the deferred snapshot-hash gate would otherwise catch) fails
/// the run **loudly** ([`DECISIONS.md`] → snapshot hashing deferred / fails loudly).
fn bump_and_regate(schema: &Schema, v2: &str, version: u32) -> Result<String> {
    let bumped = bump_stamp(schema, v2, version)?;
    let doc = engine::parse::parse_sections(schema, &bumped)
        .map_err(|e| anyhow::anyhow!("re-parsing the value-bumped doc: {e:?}"))?;
    let findings = engine::validate::schema_conformance(schema, &bumped, &doc);
    if !findings.is_empty() {
        anyhow::bail!("the value-bumped doc no longer conforms: {findings:?}");
    }
    Ok(bumped)
}

/// Splice a present schema-version stamp's value to `version` via [`engine::write::set_field`]
/// (a byte-stable value splice). `schema` is the shape the bumped `source` conforms to (its
/// header carries the stamp field). An absent stamp section / a non-conforming source is an
/// `Err` (loud, never a silent skip).
fn bump_stamp(schema: &Schema, source: &str, version: u32) -> Result<String> {
    let section = stamp_section_id(schema).ok_or_else(|| {
        anyhow::anyhow!("the schema declares no schema-version stamp section to value-bump")
    })?;
    engine::write::set_field(
        schema,
        source,
        &section,
        SCHEMA_VERSION_FIELD,
        &version.to_string(),
    )
    .map_err(|e| anyhow::anyhow!("value-bumping the schema-version stamp `{section}`: {e:?}"))
}

/// The id of the section carrying the schema-version stamp field — the existing header for a
/// header-bearing doctype, the injected `meta` header for a header-less one. The [`set_field`]
/// address for the value-bump.
///
/// [`set_field`]: engine::write::set_field
fn stamp_section_id(schema: &Schema) -> Option<String> {
    schema
        .sections
        .iter()
        .find_map(|section| match &section.body {
            SectionBody::Simple { fields, .. }
                if fields.iter().any(|f| f.id == SCHEMA_VERSION_FIELD) =>
            {
                Some(section.id.clone())
            }
            _ => None,
        })
}

/// The route for a below-version stamped doc whose prior-shape snapshot is **not shipped**:
/// the migration cannot source the `from` it would diff against, so the doc is blocked (never
/// a silent `already-current` — the detector routes it `migrate`). Ship the snapshot, re-run.
fn missing_snapshot_route(rel_key: &str, ty: &str, stamp: u32) -> String {
    format!(
        "blocked — `{rel_key}` is stamped schema-version {stamp}, below current, but no prior-schema \
         snapshot `schema-snapshots/{ty}.v{stamp}.yaml` is shipped to source the migration from; \
         ship the snapshot, then re-run `jigc migrate-corpus`"
    )
}

/// The **empty-diff backstop**'s route: the doctype's conformance-relevant structural shape
/// moved between `from` and `to`, but the schema-diff classifies **no transform kind** for that
/// change, so the doc cannot be migrated (`design/corpus-migration.md` → The empty-diff backstop
/// — no silent bump).
///
/// The repair is **not** a migration instruction — there is nothing an operator or agent can do
/// to this doc. It is a **build** instruction, aimed at the pack author who bumped the schema:
/// build the missing transform kind, then re-run. Refusing beats the alternative the backstop
/// exists to kill — folding zero bytes, bumping the stamp, and stranding the whole corpus at a
/// version it does not conform to.
fn unclassifiable_change_route(rel_key: &str, ty: &str, from: u32, to: u32) -> String {
    format!(
        "blocked — `{ty}` changed its conformance-relevant structure between schema-version \
         {from} and {to}, but the schema-diff classifies no transform kind for that change, so \
         `{rel_key}` cannot be migrated (an empty diff is not a no-op: migrating would stamp the \
         doc {to} while leaving it non-conformant). This is a schema-authoring gap, not a doc \
         problem: build the transform kind for the change in `crates/engine/src/schema_diff.rs` + \
         `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`"
    )
}

/// The **cardinality-narrowing** refusal's route: the doctype tightened a field's `card` bound
/// between the two versions, so a committed instance may carry more values than the new bound
/// admits (or lack one it now demands) — a **content-affecting** change, never a no-op
/// (`design/corpus-migration.md` → The two silent-classification holes; the pick — *refuse* — is
/// recorded in `DECISIONS.md` → 2026-07-13 M42 Inc-5 T3).
///
/// Like the backstop's route, the repair is a **schema-authoring** one, not a migration
/// instruction: nothing an operator or agent does to *this doc* unblocks it. Either the bump
/// gives the narrowing back, or the pack author builds the arm that adjudicates it (validate
/// every committed instance against the new bound — net-new validation surface, deliberately not
/// built here). Refusing beats the pre-M42 behaviour it replaces: a direction-blind fold to zero
/// bytes that restamped the corpus past its own gate.
fn narrowed_cardinality_route(
    rel_key: &str,
    ty: &str,
    section: &str,
    field: &str,
    from: u32,
    to: u32,
) -> String {
    format!(
        "blocked — `{ty}` narrows the cardinality of `{section}.{field}` between schema-version \
         {from} and {to}, so `{rel_key}` cannot be migrated: a narrowing is content-affecting, \
         not a no-op (a committed instance may carry more values than the new bound admits, or \
         lack one it now demands), and no transform kind adjudicates it — migrating would stamp \
         the doc {to} while leaving it possibly non-conformant. This is a schema-authoring gap, \
         not a doc problem: restore the wider bound, or build the narrowing arm (validate every \
         committed instance against the new bound) in `crates/engine/src/schema_diff.rs` + \
         `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`"
    )
}

/// The **field-removal** refusal's route: the doctype **dropped a declared leaf** between the two
/// versions, so every committed instance may still carry a field line the current schema no longer
/// declares (`design/corpus-migration.md` → The two silent-classification holes; the pick —
/// *refuse, not strip* — is recorded in `DECISIONS.md` → 2026-07-13 M42 Inc-5 T5).
///
/// The doc's bytes are **left alone**: stripping the field line is deterministic but destroys the
/// committed values, a knowing exception to **No-data-loss** — a *declared* property of this pair
/// (the property census). So, like the backstop's and the narrowing's routes, the repair is a
/// **schema-authoring** one, not a migration instruction: restore the leaf, or build the strip arm
/// with a deliberate data-loss opt-in (purely additive — no frozen doctype has needed a removal).
fn removed_field_route(
    rel_key: &str,
    ty: &str,
    section: &str,
    field: &str,
    from: u32,
    to: u32,
) -> String {
    format!(
        "blocked — `{ty}` drops the declared field `{section}.{field}` between schema-version \
         {from} and {to}, so `{rel_key}` cannot be migrated: committed instances still carry the \
         field, and the migration never strips a value (no data loss) — migrating would stamp the \
         doc {to} while it keeps a field the schema no longer declares. This is a schema-authoring \
         gap, not a doc problem: restore `{section}.{field}` to the schema, or build the \
         field-removal (strip) arm with a deliberate data-loss opt-in in \
         `crates/engine/src/schema_diff.rs` + `crates/engine/src/transform.rs`, then re-run `jigc \
         migrate-corpus`"
    )
}

/// A clone of `schema` with the engine schema-version stamp field removed — the doctype's
/// **prior (v0) shape**, the schema-diff's `from`. For a header-bearing doctype this drops
/// just the appended stamp leaf; for a header-less one (whose stamp lives in the synthetic
/// `meta` header the injection minted) it leaves that header empty, so the diff classifies
/// the stamp as an `added-optional-field` that introduces the fence.
fn strip_stamp(schema: &Schema) -> Schema {
    let mut out = schema.clone();
    for section in &mut out.sections {
        if let SectionBody::Simple { fields, .. } = &mut section.body {
            fields.retain(|f| f.id != SCHEMA_VERSION_FIELD);
        }
    }
    out
}

/// A clone of `schema` with the resolved `docs-root` prefix taken **back off** its
/// `location:` — the **schema-diff's** view of the current shape, and the inverse of
/// [`crate::start::apply_docs_root`] (which every CLI schema-load applies).
///
/// The diff must run over two **docs-root-free** shapes, because the prior-schema snapshot the
/// below-version arm loads stores its `location:` raw (a pack ships schemas, not project
/// layouts) and the engine's home comparison is docs-root-blind by contract
/// (`engine::schema_diff` → `resolved_home`: *"the raw schema-declared home … resolution is the
/// CLI's concern"*). Feeding it a resolved `to` against a raw `from` made `docs-root` itself
/// look like a relocation. `docs-root` is a **project config knob**, never a schema change: a
/// doctype's declared home is what the pack ships, and moving the corpus because the knob moved
/// is `crate::relocate`'s job (`config set docs-root`), not the migration's.
///
/// A `placement:` doctype bypasses `docs-root` entirely (`location: None` — `design/storage.md`
/// → Placement), so this is inert for it; a flat layout (`docs_root == ""`) is inert too.
fn docs_root_free(schema: &Schema, docs_root: &str) -> Schema {
    let mut out = schema.clone();
    if docs_root.is_empty() {
        return out;
    }
    if let Some(raw) = out
        .location
        .as_deref()
        .and_then(|home| home.strip_prefix(&format!("{docs_root}/")))
    {
        out.location = Some(raw.to_string());
    }
    out
}

/// The Framing-A route for a doc whose migration minted an empty required slot: author the
/// prose, then re-run — the stamp flips only once the doc gates clean.
fn prose_needing_route(rel_key: &str) -> String {
    format!(
        "author the new required prose in `{rel_key}` through the write verbs, then re-run \
         `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)"
    )
}

/// The route for the **both-homes destination collision** (M42 — the walk union): a
/// prior-home instance whose relocation destination already holds a *different* document. The
/// migration refuses to overwrite it — **No-data-loss** is a declared property of the corpus
/// migration, and no deterministic merge of two documents exists — so the doc is blocked and
/// the operator reconciles the two homes by hand (`design/corpus-migration.md` → the union).
fn destination_collision_route(rel_key: &str, target: &str) -> String {
    format!(
        "blocked — `{rel_key}` relocates to `{target}`, which already holds a *different* \
         document; the migration never overwrites it (no data loss). Fold the content of \
         `{rel_key}` into `{target}` through the write verbs, delete `{rel_key}`, then re-run \
         `jigc migrate-corpus`"
    )
}

/// The route for a doc left untouched behind the run's first blocker (WIP-safety: the fold
/// halts at the first blocked doc, never half-transforming the rest).
fn deferred_route(rel_key: &str) -> String {
    format!(
        "deferred — `{rel_key}` will migrate once the blocker above is resolved; re-run `jigc migrate-corpus`"
    )
}

/// Enumerate one doctype migration job's committed candidate docs as
/// `(source, destination)` pairs — the **from** home the corpus walk found each committed
/// instance at, and the path its gated bytes land at (`design/corpus-migration.md` →
/// Relocation: the walk keys on the from home). Empty when nothing is committed to walk.
///
/// - A doctype whose **current** shape declares a `location:` directory migrates
///   **in place** — every `.md` under that (already docs-root-resolved) directory,
///   `destination == source`.
/// - A doctype whose current shape is a single-file `placement:` (no `location:`) walks the
///   **union of both homes** (`design/corpus-migration.md` → The corpus walk — the placement
///   branches become a union):
///   - the **prior home** — for a *relocated* doctype (the M38 changelog) the versioned
///     snapshot at `version - 1` (via [`crate::pack::load_prior_schema`]) declares the
///     `location:` home its committed instances still sit at; walk that old directory, each
///     instance destined for the placement `file`. A *placement-born* doctype (the M40
///     methodology singletons — placement at v1) has no location-bearing prior snapshot, so
///     this half is empty;
///   - the **placement home** — the literal `placement.file`, if committed: that instance is
///     migrated **in place** (`destination == source`).
///
///   Pre-M42 these two were **mutually exclusive**, keyed on whether the prior snapshot
///   carried a `location:` — and for `changelog` it always does, so the in-place half was
///   *dead code for that doctype* and a stale root `CHANGELOG.md` was **invisible** to the
///   verb (`0 migrated, 0 already current, 0 blocked`; the stamp stayed at 1 forever, and the
///   detector's `migrate` route pointed at a verb that did nothing). Walking both halves and
///   deduping the `(source, destination)` pairs makes a placement doctype's instances findable
///   *wherever* a partially-completed migration left them. A destination shared by two
///   candidates (both homes populated) is resolved at the write boundary — see
///   [`destination_collision_route`].
fn candidate_docs(
    pack: &dyn PackSource,
    repo_root: &Path,
    dt: &DoctypeMigration,
) -> Vec<(String, String)> {
    if let Some(location) = &dt.to.location {
        return committed_slugs(repo_root, location)
            .into_iter()
            .map(|slug| {
                let key = format!("{location}{slug}.md");
                (key.clone(), key)
            })
            .collect();
    }
    let Some(placement) = &dt.to.placement else {
        return Vec::new();
    };
    let prior_location = dt
        .version
        .checked_sub(1)
        .and_then(|k| crate::pack::load_prior_schema(pack, &dt.ty, k).ok())
        .and_then(|prior| prior.location);
    let mut out: Vec<(String, String)> = Vec::new();
    if let Some(raw_home) = prior_location {
        // The prior snapshot stores its `location:` **raw** (docs-root-free), but the
        // committed instances sit under the resolved `docs-root` prefix — re-apply it
        // (the in-place branch above walks an already-resolved `to.location`, so this is
        // the sole re-application site; `crate::start::docs_root_prefix`).
        let from_home = if dt.docs_root.is_empty() {
            raw_home
        } else {
            format!("{}/{raw_home}", dt.docs_root)
        };
        out.extend(
            committed_slugs(repo_root, &from_home)
                .into_iter()
                .map(|slug| (format!("{from_home}{slug}.md"), placement.file.clone())),
        );
    }
    if repo_root.join(&placement.file).is_file() {
        out.push((placement.file.clone(), placement.file.clone()));
    }
    // Dedupe: the placement file can *itself* sit under the prior home (a `docs/x.md`
    // placement whose prior home was `docs/`), which enumerates the identical pair twice.
    out.sort();
    out.dedup();
    out
}

/// The committed-doc slugs of a persisted type — the `.md` file stems under
/// `<repo_root>/<location>`, slug-sorted. A missing / unreadable location yields none.
fn committed_slugs(repo_root: &Path, location: &str) -> Vec<String> {
    let dir = repo_root.join(location);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut slugs: Vec<String> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect();
    slugs.sort();
    slugs
}

/// Locate the repo root and its `.jigc/config/` project layer — the store-walk locate
/// preamble shared with `jigc validate` / `jigc ingest` (and the freeze-exempt relocation
/// path in `crate::relocate`). Errors with routed messages when the repo or the project
/// layer is absent.
pub(crate) fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        anyhow::bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(ctx.jigc_home)
}

#[cfg(test)]
mod tests {
    //! Verb-core coverage the real-binary dogfood (adr, header-bearing add-field) cannot
    //! reach over the frozen dev pack: the **header-less fence-introducing** add-field, and
    //! the **combined stamp-flips-last** case (a v0→v1 that adds the stamp *and* a new
    //! required slot — no such change exists in the frozen-v1 dev pack, so it is exercised
    //! over synthetic doctypes through [`migrate_committed_corpus`], the same core the verb
    //! runs).

    use super::*;
    use engine::field_block::{Field, Value};
    use engine::parse::parse_sections;
    use engine::schema::{inject_schema_version_stamp, load_schema};
    use engine::validate::schema_conformance;
    use engine::write::{Instance, ItemContent, SectionContent, render};
    use std::fs;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-migrate-corpus-{tag}-{}-{:?}",
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

    /// M42 inc 10, the migration engine's own arm: [`has_section_heading`] — the guard that
    /// decides whether a prose-needing slot is **already authored** (the idempotency guard of
    /// the tool that would have to repair a slug-rule change) — recognizes a heading by
    /// **renormalizing** it, never by re-running the *mint* rule. Under a `slugify`-based
    /// compare a section id carrying a leading edge stopword (`in-scope` → `## In Scope` →
    /// `slugify` yields `scope`) reads as **absent** even when authored, so a re-run would
    /// route the doc back for authoring it already has.
    #[test]
    fn has_section_heading_recognizes_an_edge_stopword_section_id() {
        let source = "\
# Brief

## In Scope
The read path only.
";
        assert!(
            has_section_heading(source, "in-scope"),
            "an authored `## In Scope` must be seen as the `in-scope` section",
        );
        // Multi-word (no stopword) and single-word ids keep working; a genuinely absent
        // section still reads absent.
        assert!(has_section_heading(
            "## Unreleased Changes\n",
            "unreleased-changes"
        ));
        assert!(has_section_heading("## Context\n", "context"));
        assert!(!has_section_heading("## Context\n", "decision"));
    }

    /// Load a schema, then inject the engine schema-version stamp — the CLI pack-loader's
    /// v1 shape, reconstructed for a synthetic doctype.
    fn v1_schema(yaml: &[u8]) -> Schema {
        let mut schema = load_schema(yaml).expect("schema loads");
        inject_schema_version_stamp(&mut schema);
        schema
    }

    /// The `docs-root` every core fixture seeds under: the **shipped default**, not the flat
    /// repo-root layout. See [`migration`].
    const DOCS_ROOT: &str = "docs";

    /// Build a [`DoctypeMigration`] from the stamp-injected current shape `to`, at `version`.
    /// The prior shape is resolved per committed doc by stamp inside
    /// [`migrate_committed_corpus`] (stamp-absent → `strip_stamp(to)`; below-version →
    /// the snapshot store), so it is not a field here.
    ///
    /// **The topology is production's** (M42 Inc-5 validation finding): the job carries the
    /// shipped default `docs-root: docs/` **and** a `location:`-bearing `to` already resolved
    /// under it — exactly what the verb receives, since every CLI schema-load ends in
    /// [`crate::start::apply_docs_root`] while the prior snapshot's `location:` stays raw.
    /// Seeding the *flat* layout instead (`docs_root: ""`, a raw `to.location`) is the M10
    /// fixture-topology masking face: it was the one topology in which the two sides of the
    /// schema-diff happened to be docs-root-consistent, so the whole residual arm — the
    /// **empty-diff backstop** and `PresentationOnly` — was reachable *only* in the fixtures
    /// and dead in the shipped default.
    fn migration(mut to: Schema, version: u32) -> DoctypeMigration {
        if let Some(location) = to.location.as_deref() {
            to.location = Some(format!("{DOCS_ROOT}/{location}"));
        }
        DoctypeMigration {
            ty: to.ty.clone(),
            to,
            version,
            docs_root: DOCS_ROOT.to_string(),
        }
    }

    /// Build a throwaway pack dir carrying one prior-schema snapshot
    /// (`schema-snapshots/<ty>.v<version>.yaml`) + a freeze manifest declaring `<ty>` frozen,
    /// so [`crate::pack::load_prior_schema`] resolves field-types **and** injects the
    /// schema-version stamp into the snapshot identically to the current schema. Returns the
    /// owning [`TempDir`] (the caller keeps it alive so the files outlive the pack).
    fn snapshot_pack(ty: &str, version: u32, snapshot_yaml: &str) -> TempDir {
        let dir = TempDir::new("snap-pack");
        let snaps = dir.path().join("schema-snapshots");
        fs::create_dir_all(&snaps).expect("mk schema-snapshots");
        fs::write(snaps.join(format!("{ty}.v{version}.yaml")), snapshot_yaml).expect("write snap");
        let cfg = dir.path().join("config");
        fs::create_dir_all(&cfg).expect("mk config");
        // Only the doctype name gates stamp injection; the manifest version/hash are not read
        // on this path (the freeze gate does not run here), so they are placeholders.
        fs::write(
            cfg.join("schema-manifest.yaml"),
            format!(
                "doctypes:\n  - type: {ty}\n    schema-version: {version}\n    schema-hash: {}\n",
                "0".repeat(64)
            ),
        )
        .expect("write manifest");
        dir
    }

    /// Write `content` to `repo_root/<rel_key>`, creating the location dir.
    fn write_doc(repo_root: &Path, rel_key: &str, content: &str) {
        let path = repo_root.join(rel_key);
        fs::create_dir_all(path.parent().unwrap()).expect("mk location dir");
        fs::write(&path, content).expect("write doc");
    }

    /// Assert a doc's bytes conform to `schema` (zero conformance findings) and round-trip
    /// byte-identical.
    fn assert_conformant_and_stable(schema: &Schema, source: &str) {
        let doc = parse_sections(schema, source).expect("doc parses under the current schema");
        let findings = schema_conformance(schema, source, &doc);
        assert!(
            findings.is_empty(),
            "migrated doc must conform; got {findings:?}\nsource:\n{source}"
        );
        let inst = engine::write::instance_from_source(schema, source).expect("re-parse");
        assert_eq!(
            render(schema, &inst),
            source,
            "migrated doc must round-trip byte-identical"
        );
    }

    /// A header-less doctype (`note`): no `---` block until the stamp introduces one.
    fn note_v0_yaml() -> &'static [u8] {
        b"\
type: note
location: notes/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: success
    slot: { hint: \"s\" }
"
    }

    /// The header-less add-field case **through the verb core**: migrating a committed v0
    /// `note` introduces the `---` fence carrying `schema-version: 1` byte-stable, preserves
    /// the body slots, and reports it migrated.
    #[test]
    fn header_less_doc_migration_introduces_the_fence_byte_stable() {
        let repo = TempDir::new("fence");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());

        // A conformant v0 note authored through `render` over the **genuine** header-less
        // shape (the byte-stable form a real header-less committed doc has — `strip_stamp`
        // leaves an empty `meta` header, used only by the schema-diff, never to render).
        let v0 = render(
            &load_schema(note_v0_yaml()).expect("v0 note loads"),
            &Instance {
                title: "A Note".to_string(),
                sections: vec![
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("A clean flow.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "success".to_string(),
                        slot: Some("It works.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            !v0.starts_with("---"),
            "the v0 note carries no front-matter fence"
        );
        write_doc(repo.path(), "docs/notes/a-note.md", &v0);

        // The stamp-absent (v0) path derives `from = strip_stamp(to)` internally and never
        // touches the snapshot store, so the pack is unused here.
        let pack = crate::pack::EmbeddedPack::new();
        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 1)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(report.migrated, vec!["docs/notes/a-note.md".to_string()]);
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        let migrated = fs::read_to_string(repo.path().join("docs/notes/a-note.md")).expect("read");
        assert!(
            migrated.starts_with("---\nschema-version: 1\n---\n\n"),
            "the fence is introduced carrying the stamp; got:\n{migrated}"
        );
        assert!(
            migrated.contains("A clean flow.") && migrated.contains("It works."),
            "the body slots survive; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);
    }

    /// An already-stamped doc is the false-positive guard: it is skipped, byte-untouched.
    #[test]
    fn already_current_doc_is_skipped_byte_untouched() {
        let repo = TempDir::new("current");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());

        // A note already stamped at the current version (rendered through the v1 schema).
        let stamped = render(
            &to,
            &Instance {
                title: "Done Note".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Already done.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "success".to_string(),
                        slot: Some("Stamped.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/notes/done-note.md", &stamped);

        let pack = crate::pack::EmbeddedPack::new();
        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 1)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(
            report.already_current,
            vec!["docs/notes/done-note.md".to_string()]
        );
        assert!(report.migrated.is_empty(), "nothing to migrate");
        let after = fs::read_to_string(repo.path().join("docs/notes/done-note.md")).expect("read");
        assert_eq!(after, stamped, "an already-current doc is byte-untouched");
    }

    /// Per-doc baseline atomicity (M34 audit, `corpus-migration.md` → Migration atomicity /
    /// WIP-safety): the file write and its file-state baseline flip are **one per-doc unit**, so
    /// the on-disk baseline always matches the on-disk files. When an I/O fault aborts the write
    /// loop mid-corpus, the docs already written must already carry their refreshed baseline on
    /// disk — otherwise a subsequent `file-state` detect over the committed store mis-reports
    /// those correctly-migrated docs as out-of-band drift.
    ///
    /// Two tracked v0 notes: `a` sorts before `b`, so `a` persists first; `b`'s atomic write is
    /// forced to fail (its temp-sibling `notes/b.md.tmp` is pre-occupied by a directory, so
    /// [`engine::state::persist`]'s temp write errors), aborting the loop. After the abort `a` is
    /// on disk migrated, and its on-disk baseline must already equal the hash of those bytes.
    #[test]
    fn written_docs_are_baselined_per_doc_even_when_a_later_write_aborts() {
        let repo = TempDir::new("partial-abort");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());
        let v0_schema = load_schema(note_v0_yaml()).expect("v0 note loads");
        let note = |title: &str| {
            render(
                &v0_schema,
                &Instance {
                    title: title.to_string(),
                    sections: vec![
                        SectionContent {
                            id: "vision".to_string(),
                            slot: Some("V.".to_string()),
                            ..Default::default()
                        },
                        SectionContent {
                            id: "success".to_string(),
                            slot: Some("S.".to_string()),
                            ..Default::default()
                        },
                    ],
                },
            )
        };
        let a0 = note("A Note");
        let b0 = note("B Note");
        write_doc(repo.path(), "docs/notes/a.md", &a0);
        write_doc(repo.path(), "docs/notes/b.md", &b0);

        // Both docs are already tracked in the file-state baseline (the migration re-baselines
        // only docs it already tracks) at their v0 hashes.
        let mut seed = FileStateRecord::new();
        seed.record("docs/notes/a.md".to_string(), hash_bytes(a0.as_bytes()));
        seed.record("docs/notes/b.md".to_string(), hash_bytes(b0.as_bytes()));
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        // Force `b`'s atomic write to fail: occupy its temp-sibling path with a directory, so
        // `persist` errors when it writes the temp file — the loop aborts after `a` is written.
        fs::create_dir_all(repo.path().join("docs/notes/b.md.tmp")).expect("occupy temp sibling");

        let pack = crate::pack::EmbeddedPack::new();
        let result = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 1)],
            Options::default(),
        );
        assert!(
            result.is_err(),
            "the aborted write surfaces as an error: {result:?}"
        );

        // `a` was written to disk migrated; its on-disk baseline must already reflect those
        // bytes (the per-doc commit), so a later `file-state` detect would NOT mis-report it.
        let on_disk_a = fs::read(repo.path().join("docs/notes/a.md")).expect("a was written");
        assert_ne!(
            on_disk_a,
            a0.as_bytes(),
            "a was actually migrated (not still v0)"
        );
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("docs/notes/a.md"),
            Some(hash_bytes(&on_disk_a).as_str()),
            "the already-written doc `a` must be baselined to its on-disk bytes before the abort"
        );
    }

    /// The v1 prior shape of the `memo` doctype (a single `vision` slot) — the snapshot the
    /// below-version path sources `from` from.
    fn memo_v1_yaml() -> &'static str {
        "\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// The v2 current shape of the `memo` doctype (a new required trailing `rationale` slot).
    fn memo_v2_yaml() -> &'static [u8] {
        b"\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: rationale
    slot: { hint: \"why\" }
"
    }

    /// The **stamp-flips-last** discipline on the below-version (v1→v2) snapshot path: a memo
    /// whose v1→v2 adds a new required `rationale` slot, sourced `from` the versioned snapshot
    /// store. The committed **v1-stamped** doc lacks the slot, so the migration mints it empty,
    /// **fails the gate, and rolls back** — the doc stays byte-identical v1 (stamp `1`, **not**
    /// value-bumped to `2`), routed to the agent. Once the prose is authored, the same doc
    /// migrates clean and the stamp **flips** `1→2` — proof the value-bump lands only on a
    /// clean gate (post-fold), never while prose is pending.
    #[test]
    fn below_version_prose_needing_stamp_flips_only_after_prose_authored() {
        let repo = TempDir::new("flips-last");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("memo", 1, memo_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(memo_v2_yaml());
        // The prior shape the verb sources from the snapshot store (vision + injected stamp)
        // — the byte form a committed v1 memo is rendered against.
        let from = crate::pack::load_prior_schema(&pack, "memo", 1).expect("the memo.v1 snapshot");

        // --- phase 1: the v1 doc lacks rationale → migration blocks, doc stays v1 ---
        let v1 = render(
            &from,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Move the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/memos/cache-memo.md", &v1);

        let blocked_report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert!(
            blocked_report.migrated.is_empty(),
            "the prose-needing doc does not migrate: {:?}",
            blocked_report.migrated
        );
        assert_eq!(
            blocked_report.blocked.len(),
            1,
            "the doc is blocked, routed to the agent: {:?}",
            blocked_report.blocked
        );
        assert_eq!(blocked_report.blocked[0].0, "docs/memos/cache-memo.md");
        assert!(
            blocked_report.blocked[0]
                .1
                .contains("re-run `jigc migrate-corpus`"),
            "the route is Framing-A author-then-re-run: {}",
            blocked_report.blocked[0].1
        );
        // STAMP-FLIPS-LAST: the on-disk doc is byte-identical v1 — stamp NOT bumped to 2 —
        // because the value-bump is post-fold and the doc never reached a clean gate, so
        // nothing was written.
        let still_v1 =
            fs::read_to_string(repo.path().join("docs/memos/cache-memo.md")).expect("read");
        assert_eq!(still_v1, v1, "the blocked doc is byte-identical v1");
        assert!(
            still_v1.contains("schema-version: 1") && !still_v1.contains("schema-version: 2"),
            "the stamp did NOT flip while the prose is pending; got:\n{still_v1}"
        );

        // --- phase 2: the agent authors the rationale slot → migration flips the stamp 1→2 ---
        let authored = render(
            &to,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Move the cache.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "rationale".to_string(),
                        slot: Some("Latency wins.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/memos/cache-memo.md", &authored);

        let flipped_report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("re-migration runs");

        assert_eq!(
            flipped_report.migrated,
            vec!["docs/memos/cache-memo.md".to_string()],
            "with the prose authored the doc migrates: {flipped_report:?}",
        );
        assert!(flipped_report.blocked.is_empty(), "no longer blocked");
        let flipped =
            fs::read_to_string(repo.path().join("docs/memos/cache-memo.md")).expect("read");
        assert!(
            flipped.contains("schema-version: 2"),
            "the stamp flips 1→2 once the prose is authored and the doc gates clean; got:\n{flipped}"
        );
        assert!(
            flipped.contains("Move the cache.") && flipped.contains("Latency wins."),
            "the authored prose survives; got:\n{flipped}"
        );
        assert_conformant_and_stable(&to, &flipped);
    }

    /// The v1 prior shape of the `brief` doctype: a required `summary` slot + an **optional**
    /// `risks` slot — the shape a doc may legitimately leave empty.
    fn brief_v1_yaml() -> &'static str {
        "\
type: brief
location: briefs/
id-from: title
sections:
  - id: summary
    slot: { hint: \"s\" }
  - id: risks
    slot: { hint: \"r\", optional: true }
"
    }

    /// The v2 current shape: `risks` **tightens** to required (`optional: true → false`) — the
    /// sixth hole, which renders a currently-conformant doc non-conformant.
    fn brief_v2_yaml() -> &'static [u8] {
        b"\
type: brief
location: briefs/
id-from: title
sections:
  - id: summary
    slot: { hint: \"s\" }
  - id: risks
    slot: { hint: \"r\" }
"
    }

    /// **The `optional: true → false` tighten, end to end** (the sixth hole; T4). Two committed
    /// v1-stamped briefs share the bump, and the two halves of the done-picture must both hold:
    ///
    /// - the **authored** brief (it carries `## Risks` prose) is already conformant under the
    ///   tightened schema, so it migrates with **zero byte work** and restamps `1 → 2`;
    /// - the **bare** brief (it carries the `## Risks` heading and legitimately left it empty) is
    ///   **blocked** with the Framing-A prose route, and its stamp is **NOT** bumped.
    ///
    /// The tighten mints nothing: T2's guard drops a `ProseNeeding` whose heading the doc already
    /// carries (minting it would hit `generate_section`'s `AlreadyPresent` refusal and dead-end
    /// the doc), so the shipped **per-doc conformance gate** is the adjudicator — the empty
    /// required slot breaks `required-slot-present` and rolls the doc back.
    ///
    /// Red before T4: the flag delta classified **nothing**, so the pair fell through to the
    /// backstop's residual (`Unclassified`) and **both** docs were refused with the *build the
    /// transform kind first* route — the authored one included; before the backstop it was worse
    /// still (the stamp bumped and the corpus stranded at v2 failing its own gate).
    #[test]
    fn below_version_optional_tighten_blocks_the_bare_doc_and_migrates_the_authored_one() {
        let repo = TempDir::new("optional-tighten");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("brief", 1, brief_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(brief_v2_yaml());
        let from =
            crate::pack::load_prior_schema(&pack, "brief", 1).expect("the brief.v1 snapshot");

        let stamp_v1 = || SectionContent {
            id: "meta".to_string(),
            fields: vec![Field {
                key: SCHEMA_VERSION_FIELD.to_string(),
                value: Value::Scalar("1".to_string()),
            }],
            ..Default::default()
        };

        // The AUTHORED brief — it filled the (then-optional) `risks` slot, so it is already
        // conformant under the tightened v2 shape.
        let authored = render(
            &from,
            &Instance {
                title: "Authored Brief".to_string(),
                sections: vec![
                    stamp_v1(),
                    SectionContent {
                        id: "summary".to_string(),
                        slot: Some("Ship the cache.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "risks".to_string(),
                        slot: Some("Cold-start latency.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        // The BARE brief — the optional slot legitimately left empty; the writer still emits its
        // `## Risks` heading (which is what would dead-end a minting transform).
        let bare = render(
            &from,
            &Instance {
                title: "Bare Brief".to_string(),
                sections: vec![
                    stamp_v1(),
                    SectionContent {
                        id: "summary".to_string(),
                        slot: Some("Ship the queue.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            bare.contains("## Risks"),
            "the bare brief carries the heading but no prose; got:\n{bare}"
        );
        // `authored` sorts before `bare`, so the run reaches the blocker only after committing
        // the migratable doc (the fold halts at the first blocker).
        write_doc(repo.path(), "docs/briefs/authored-brief.md", &authored);
        write_doc(repo.path(), "docs/briefs/bare-brief.md", &bare);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        // The authored doc migrates and restamps 1 → 2, its prose byte-identical.
        assert_eq!(
            report.migrated,
            vec!["docs/briefs/authored-brief.md".to_string()],
            "the conformant doc migrates: {report:?}"
        );
        let migrated =
            fs::read_to_string(repo.path().join("docs/briefs/authored-brief.md")).expect("read");
        assert!(
            migrated.contains("schema-version: 2") && migrated.contains("Cold-start latency."),
            "the stamp flips and the prose survives; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);

        // The bare doc is BLOCKED with the Framing-A prose route — not the backstop's
        // *build the transform kind* route (the kind exists; the doc needs prose).
        assert_eq!(
            report.blocked.len(),
            1,
            "only the bare doc blocks: {:?}",
            report.blocked
        );
        let (blocked_key, route) = &report.blocked[0];
        assert_eq!(blocked_key, "docs/briefs/bare-brief.md");
        assert!(
            route.contains("author the new required prose"),
            "the route routes the prose, not a transform-kind build: {route}"
        );
        // STAMP-FLIPS-LAST: the blocked doc is byte-identical v1 — never restamped at a version
        // it fails the gate of (the silent strand this kind exists to close).
        let still_v1 =
            fs::read_to_string(repo.path().join("docs/briefs/bare-brief.md")).expect("read");
        assert_eq!(still_v1, bare, "the blocked doc is byte-identical v1");
        assert!(
            still_v1.contains("schema-version: 1") && !still_v1.contains("schema-version: 2"),
            "the stamp did NOT flip on a doc that fails its own v2 gate; got:\n{still_v1}"
        );
    }

    /// The v1 prior shape of the `ledger` doctype: a `vision` slot **and** a `retired` slot the
    /// v2 shape drops — a projection move (a removed section) with **no transform kind**.
    fn ledger_v1_yaml() -> &'static str {
        "\
type: ledger
location: ledgers/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: retired
    slot: { hint: \"a section v2 drops\" }
"
    }

    /// The v2 current shape of the `ledger` doctype: `retired` is **gone**. The classifier can
    /// express no kind for a section removal, so the pair trips the empty-diff backstop.
    fn ledger_v2_yaml() -> &'static [u8] {
        b"\
type: ledger
location: ledgers/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// **THE EMPTY-DIFF BACKSTOP, through the verb core.** A below-version doc (stamp `1`,
    /// current `2`) whose v1→v2 pair moved the conformance-relevant projection but classifies
    /// **no transform kind** is **refused**: reported `blocked` with a route that names the
    /// real repair — *build the transform kind* — with its bytes **untouched** and its stamp
    /// **still `1`**.
    ///
    /// Red before the backstop: the pair diffed to `[]`, the fold "migrated" the doc as a
    /// no-op, the post-fold value-bump stamped it `2`, and the corpus landed at **v2 failing
    /// its own gate, silently** — the strand class both M41 and M42 paid to learn
    /// (`design/corpus-migration.md` → The empty-diff backstop — no silent bump). This is the
    /// only surface that loads the prior snapshot the diff needs, so it is where the refusal
    /// fires — a **migration refusal, not a build error**.
    #[test]
    fn below_version_unclassifiable_change_is_refused_with_the_build_the_kind_route() {
        let repo = TempDir::new("backstop");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("ledger", 1, ledger_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(ledger_v2_yaml());
        let from =
            crate::pack::load_prior_schema(&pack, "ledger", 1).expect("the ledger.v1 snapshot");

        // A conformant, v1-stamped committed doc carrying the section v2 drops.
        let v1 = render(
            &from,
            &Instance {
                title: "Open Ledger".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("The ledger holds.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "retired".to_string(),
                        slot: Some("Prose the v2 shape has no home for.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/ledgers/open-ledger.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("the run completes — the refusal is a routed block, not a run failure");

        assert!(
            report.migrated.is_empty() && report.already_current.is_empty(),
            "an unclassifiable change never migrates and is never called current: {report:?}"
        );
        assert_eq!(
            report.blocked.len(),
            1,
            "the doc is blocked, routed: {:?}",
            report.blocked
        );
        assert_eq!(report.blocked[0].0, "docs/ledgers/open-ledger.md");
        let route = &report.blocked[0].1;
        assert!(
            route.contains("no transform kind") && route.contains("build the transform kind"),
            "the route names the real repair (build the kind first), not a migration \
             instruction; got: {route}"
        );

        // The bytes are untouched and the stamp is STILL 1 — no silent bump.
        let after =
            fs::read_to_string(repo.path().join("docs/ledgers/open-ledger.md")).expect("read");
        assert_eq!(after, v1, "the refused doc is byte-identical");
        assert!(
            after.contains("schema-version: 1") && !after.contains("schema-version: 2"),
            "the stamp did NOT bump over an unclassified change; got:\n{after}"
        );
    }

    /// The v1 prior shape of the `linked` doctype: a header `rel` ref at `card: "0..*"` (a
    /// committed instance may legitimately carry several edges).
    fn linked_v1_yaml() -> &'static str {
        "\
type: linked
location: linked/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: linked, card: \"0..*\" }
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// The v2 current shape: the same `rel` **narrowed** to `card: "0..1"` — the refused
    /// direction (the corpus may hold instances the new bound no longer admits).
    fn linked_v2_yaml() -> &'static [u8] {
        b"\
type: linked
location: linked/
id-from: title
sections:
  - id: meta
    header: true
    fields:
      - { id: rel, type: ref, to: linked, card: \"0..1\" }
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// **A cardinality narrowing blocks with its own route** (the recorded pick: refuse —
    /// `DECISIONS.md` → 2026-07-13 M42 Inc-5 T3). A below-version doc (stamp `1`, current `2`)
    /// whose v1→v2 pair narrows a `card` is reported `blocked`, its bytes **untouched** and its
    /// stamp **still `1`** — with a route that names the schema-authoring repair, never the
    /// prose-needing route (there is no prose to author).
    ///
    /// Red before T3: the pair classified `[WidenedCardinality]` (direction-blind), the fold
    /// folded **zero bytes**, the post-fold bump stamped it `2`, and the doc — which carries
    /// **two** committed `rel` edges the new `0..1` bound no longer admits — landed *migrated*,
    /// past the gate, silently.
    #[test]
    fn below_version_card_narrowing_is_refused_with_its_own_route() {
        let repo = TempDir::new("narrowing");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("linked", 1, linked_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(linked_v2_yaml());
        let from =
            crate::pack::load_prior_schema(&pack, "linked", 1).expect("the linked.v1 snapshot");

        // A conformant, v1-stamped committed doc carrying TWO `rel` edges — exactly what the
        // narrowed `0..1` bound no longer admits (the reason a narrowing is not a no-op).
        let v1 = render(
            &from,
            &Instance {
                title: "Hub".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![
                            Field {
                                key: SCHEMA_VERSION_FIELD.to_string(),
                                value: Value::Scalar("1".to_string()),
                            },
                            Field {
                                key: "rel".to_string(),
                                value: Value::List(vec![
                                    "linked:spoke-a".to_string(),
                                    "linked:spoke-b".to_string(),
                                ]),
                            },
                        ],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("The hub links both spokes.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/linked/hub.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("the run completes — the refusal is a routed block, not a run failure");

        assert!(
            report.migrated.is_empty() && report.already_current.is_empty(),
            "a narrowing never migrates and is never called current: {report:?}"
        );
        assert_eq!(
            report.blocked.len(),
            1,
            "the doc is blocked, routed: {:?}",
            report.blocked
        );
        assert_eq!(report.blocked[0].0, "docs/linked/hub.md");
        let route = &report.blocked[0].1;
        assert!(
            route.contains("narrows") && route.contains("meta.rel"),
            "the route names the narrowed leaf; got: {route}"
        );
        assert!(
            !route.contains("author the new required prose"),
            "the prose-needing route would be a lie here; got: {route}"
        );

        // The bytes are untouched and the stamp is STILL 1 — no silent bump past the gate.
        let after = fs::read_to_string(repo.path().join("docs/linked/hub.md")).expect("read");
        assert_eq!(after, v1, "the refused doc is byte-identical");
        assert!(
            after.contains("schema-version: 1") && !after.contains("schema-version: 2"),
            "the stamp did NOT bump over a refused narrowing; got:\n{after}"
        );
    }

    /// The v2 current shape of `linked` with the `rel` leaf **dropped entirely** — a field
    /// removal, the refused shape (`DECISIONS.md` → 2026-07-13 M42 Inc-5 T5).
    fn linked_v2_removed_yaml() -> &'static [u8] {
        b"\
type: linked
location: linked/
id-from: title
sections:
  - id: meta
    header: true
    fields: []
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// **A removed field blocks with its own route** (the recorded pick: refuse, not strip —
    /// `DECISIONS.md` → 2026-07-13 M42 Inc-5 T5). A below-version doc (stamp `1`, current `2`)
    /// whose v1→v2 pair **drops a declared leaf** is reported `blocked`, its bytes **untouched**
    /// (the committed value it still carries is never destroyed) and its stamp **still `1`** —
    /// with a route that names the schema-authoring repair, never the prose-needing route.
    ///
    /// Red before T5: the classifier never saw the removal (both loops iterate v2's leaves), so
    /// the doc rode the backstop's *unclassifiable* route — and a removal riding **alongside**
    /// any classified change was dropped outright: migrated, restamped `2`, still carrying a
    /// field line the current schema no longer declares.
    #[test]
    fn below_version_field_removal_is_refused_with_its_own_route() {
        let repo = TempDir::new("removal");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("linked", 1, linked_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(linked_v2_removed_yaml());
        let from =
            crate::pack::load_prior_schema(&pack, "linked", 1).expect("the linked.v1 snapshot");

        // A conformant, v1-stamped committed doc carrying a `rel` value the v2 shape no longer
        // declares — the value a strip would destroy, and the reason the pick is to refuse.
        let v1 = render(
            &from,
            &Instance {
                title: "Hub".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![
                            Field {
                                key: SCHEMA_VERSION_FIELD.to_string(),
                                value: Value::Scalar("1".to_string()),
                            },
                            Field {
                                key: "rel".to_string(),
                                value: Value::Scalar("linked:spoke-a".to_string()),
                            },
                        ],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("The hub links a spoke.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/linked/hub.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("the run completes — the refusal is a routed block, not a run failure");

        assert!(
            report.migrated.is_empty() && report.already_current.is_empty(),
            "a removal never migrates and is never called current: {report:?}"
        );
        assert_eq!(
            report.blocked.len(),
            1,
            "the doc is blocked, routed: {:?}",
            report.blocked
        );
        assert_eq!(report.blocked[0].0, "docs/linked/hub.md");
        let route = &report.blocked[0].1;
        assert!(
            route.contains("drops the declared field") && route.contains("meta.rel"),
            "the route names the dropped leaf; got: {route}"
        );
        assert!(
            !route.contains("author the new required prose"),
            "the prose-needing route would be a lie here; got: {route}"
        );

        // The bytes are untouched — the committed value survives — and the stamp is STILL 1.
        let after = fs::read_to_string(repo.path().join("docs/linked/hub.md")).expect("read");
        assert_eq!(after, v1, "the refused doc is byte-identical");
        assert!(
            after.contains("rel: linked:spoke-a"),
            "the committed value of the dropped leaf is never destroyed; got:\n{after}"
        );
        assert!(
            after.contains("schema-version: 1") && !after.contains("schema-version: 2"),
            "the stamp did NOT bump over a refused removal; got:\n{after}"
        );
    }

    /// The v1 prior shape of the `card` doctype: a single **fixed-slot** `body` section — the
    /// reconstructed-shape analog of the M25 `prd.requirements` fixed-slot→repeatable reshape.
    fn card_v1_yaml() -> &'static str {
        "\
type: card
location: cards/
id-from: title
sections:
  - id: body
    slot: { hint: \"the card body\" }
"
    }

    /// The v2 current shape of the `card` doctype: `body` promoted to a **repeatable**
    /// item-block (the old slot prose becomes the default first item).
    fn card_v2_yaml() -> &'static [u8] {
        b"\
type: card
location: cards/
id-from: title
sections:
  - id: body
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one card\" } }
"
    }

    /// The headline T2 case: a **below-version stamped** doc (stamp `1`, current `2`) migrates
    /// through `migrate_committed_corpus` (the same core the verb runs) — `from` sourced from
    /// the `schema-snapshots/card.v1.yaml` snapshot, the `fixed-slot→repeatable` structural
    /// change applied, and the stamp **value-bumped** `1→2`. The result (a) round-trips
    /// byte-stable + (b) conforms under the current schema ([`assert_conformant_and_stable`]),
    /// (c) is **idempotent** on re-run (now stamp-2 ⇒ already-current, byte-untouched), and
    /// (d) **detector/verb agree**: a below-version doc the version-aware detector routes
    /// `migrate` (proven over the real binary in `tests/schema_conformance_routing.rs`) is
    /// **migrated** by the verb, never reported `already-current` (DECISIONS → audit Finding 2).
    #[test]
    fn below_version_doc_migrates_via_snapshot_with_stamp_value_bump() {
        let repo = TempDir::new("below-version");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("card", 1, card_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(card_v2_yaml());
        // The fixed-slot prior shape (with the injected stamp) — the form a committed v1 card
        // is rendered against, sourced from the snapshot store exactly as the verb sources it.
        let from = crate::pack::load_prior_schema(&pack, "card", 1).expect("the card.v1 snapshot");

        let v1 = render(
            &from,
            &Instance {
                title: "First Card".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Wire the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            v1.contains("schema-version: 1"),
            "the committed doc is stamped below current; got:\n{v1}"
        );
        write_doc(repo.path(), "docs/cards/first-card.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        // (d) detector/verb agree: a below-version doc is MIGRATED, never already-current.
        assert_eq!(
            report.migrated,
            vec!["docs/cards/first-card.md".to_string()],
            "the below-version doc migrates: {report:?}"
        );
        assert!(
            report.already_current.is_empty() && report.blocked.is_empty(),
            "never already-current or blocked: {report:?}"
        );

        let migrated =
            fs::read_to_string(repo.path().join("docs/cards/first-card.md")).expect("read");
        // The stamp is value-bumped 1→2 (the v1→v2 transition).
        assert!(
            migrated.contains("schema-version: 2") && !migrated.contains("schema-version: 1"),
            "the stamp is value-bumped 1→2; got:\n{migrated}"
        );
        // The structural change landed: the slot prose is preserved as the default item.
        assert!(
            migrated.contains("Wire the cache."),
            "the slot prose survives as the default item; got:\n{migrated}"
        );
        // (a) round-trip byte-stable + (b) conformant under the current schema.
        assert_conformant_and_stable(&to, &migrated);

        // (c) idempotent: a re-run finds the now-stamp-2 doc already current, byte-untouched.
        let rerun = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("re-migration runs");
        assert_eq!(
            rerun.already_current,
            vec!["docs/cards/first-card.md".to_string()],
            "the migrated doc is already-current on a re-run: {rerun:?}"
        );
        assert!(
            rerun.migrated.is_empty(),
            "nothing migrates twice: {rerun:?}"
        );
        let after = fs::read_to_string(repo.path().join("docs/cards/first-card.md")).expect("read");
        assert_eq!(after, migrated, "the re-run leaves the doc byte-untouched");
    }

    /// A below-version stamped doc whose prior-shape snapshot is **not shipped** is **blocked**
    /// with a route naming the missing snapshot — never a silent `already-current` (the
    /// detector routes it `migrate`, so the verb must surface it, not drop it).
    #[test]
    fn below_version_doc_with_a_missing_snapshot_is_blocked_with_a_route() {
        let repo = TempDir::new("missing-snap");
        let jigc_root = repo.path().join(".jigc");

        // A pack with NO `schema-snapshots/` entry, so `load_prior_schema(card, 1)` errors.
        let pack_dir = TempDir::new("empty-pack");
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(card_v2_yaml());
        // The committed v1 doc is rendered against the in-memory fixed-slot shape (the pack
        // ships no snapshot to source it from — that is the point of this case).
        let from_shape = {
            let mut s = load_schema(card_v1_yaml().as_bytes()).expect("v1 card loads");
            inject_schema_version_stamp(&mut s);
            s
        };
        let v1 = render(
            &from_shape,
            &Instance {
                title: "First Card".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Wire the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "docs/cards/first-card.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert!(
            report.migrated.is_empty() && report.already_current.is_empty(),
            "a missing snapshot is neither migrated nor already-current: {report:?}"
        );
        assert_eq!(report.blocked.len(), 1, "the doc is blocked: {report:?}");
        assert_eq!(report.blocked[0].0, "docs/cards/first-card.md");
        assert!(
            report.blocked[0]
                .1
                .contains("schema-snapshots/card.v1.yaml")
                && report.blocked[0].1.contains("re-run `jigc migrate-corpus`"),
            "the route names the missing snapshot + the re-run: {}",
            report.blocked[0].1
        );
        // The doc is left byte-untouched (never silently rewritten).
        let after = fs::read_to_string(repo.path().join("docs/cards/first-card.md")).expect("read");
        assert_eq!(after, v1, "the blocked doc is byte-untouched");
    }

    /// The v1 prior shape of the `log` doctype: a folder-location home (`changelog/`) with a
    /// single `body` slot — the reconstructed pre-relocation changelog. The snapshot the
    /// relocation walk sources both the old home *and* the schema-diff `from` from.
    fn log_v1_yaml() -> &'static str {
        "\
type: log
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
"
    }

    /// The v2 current shape of the `log` doctype: **relocated** to a single literal
    /// `placement:` home (`CHANGELOG.md`, no `location:`) and given a `display-title:
    /// Changelog` (so `# changelog` → `# Changelog`) — the reconstructed changelog v1→v2
    /// relocation.
    fn log_v2_yaml() -> &'static [u8] {
        b"\
type: log
placement: { file: CHANGELOG.md }
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
"
    }

    /// Render a committed **v1-stamped** `log` at its old folder home against `from` (the
    /// snapshot shape, stamp injected): a lowercase `# changelog` H1 + the body slot. The
    /// byte form a real pre-relocation committed changelog has.
    fn log_v1_doc(from: &Schema) -> String {
        render(
            from,
            &Instance {
                title: "changelog".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Released 1.0 with the new limiter.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        )
    }

    /// The headline T2 relocation case through `migrate_committed_corpus` (the same core the
    /// verb runs): a committed **v1-stamped** `log` sits at its old folder home
    /// (`changelog/changelog.md`) with a lowercase `# changelog` H1. Its v2 shape is
    /// **relocated** to the literal `CHANGELOG.md` placement home + display-title `Changelog`.
    /// The move-arm (a) walks the **`from`** home (the walk keys on the old location, not the
    /// empty placement `to`), finds the doc, and moves it; (b) writes the gated v2 bytes to
    /// `CHANGELOG.md` byte-faithful — the H1 fixed to `# Changelog`, the stamp value-bumped
    /// `1→2`, the body preserved; (c) **removes** the old `changelog/changelog.md`; and (d)
    /// **re-keys** the file-state baseline `from → to` (the old-home key dropped, the target
    /// re-registered at the migrated hash — no orphan).
    #[test]
    fn relocated_doc_moves_to_placement_home_byte_faithful_with_rekey() {
        let repo = TempDir::new("relocate");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("log", 1, log_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(log_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "log", 1).expect("the log.v1 snapshot");

        let v1 = log_v1_doc(&from);
        assert!(
            v1.starts_with("---\nschema-version: 1\n---\n\n# changelog\n"),
            "the committed doc is stamped v1 with a lowercase H1; got:\n{v1}"
        );
        write_doc(repo.path(), "docs/changelog/changelog.md", &v1);

        // The doc is tracked at its old home (the migration re-keys only what it tracks).
        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        // (a) the walk found+moved the doc at the from home; the report names the new home.
        assert_eq!(
            report.migrated,
            vec!["CHANGELOG.md".to_string()],
            "the relocated doc migrates to its placement home: {report:?}"
        );
        assert!(
            report.blocked.is_empty() && report.already_current.is_empty(),
            "never blocked or already-current: {report:?}"
        );

        // (c) the old-home source file is removed.
        assert!(
            !repo.path().join("docs/changelog/changelog.md").exists(),
            "the old-home source is removed after the move"
        );
        // (b) the doc lives at the placement home, byte-faithful: H1 fixed, stamp bumped 1→2,
        //     body preserved.
        let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("relocated home");
        assert!(
            moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
            "the stamp value-bumps 1→2 and the H1 is fixed to `# Changelog`; got:\n{moved}"
        );
        assert!(
            moved.contains("Released 1.0 with the new limiter."),
            "the body prose survives the move; got:\n{moved}"
        );
        assert_conformant_and_stable(&to, &moved);

        // (d) the file-state is re-keyed from → to: the target carries the migrated hash and
        //     the old-home key is dropped (no orphaned baseline entry).
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("CHANGELOG.md"),
            Some(hash_bytes(moved.as_bytes()).as_str()),
            "the target home is baselined at the migrated bytes"
        );
        assert!(
            record.get("docs/changelog/changelog.md").is_none(),
            "the old-home key is dropped (re-keyed, not orphaned)"
        );
    }

    /// Relocation-safety census row — **write-before-remove / abort-strands-neither**
    /// (`design/corpus-migration.md` → Relocation: write-to-`to` precedes remove-`from`, so an
    /// abort between strands neither copy). The move writes the gated v2 bytes to the target
    /// home **first**, then removes the old-home source. When the write to `to` is forced to
    /// fail (its atomic temp-sibling `CHANGELOG.md.tmp` is pre-occupied by a directory, so
    /// [`engine::state::persist`] errors before the rename), the old-home source is **never
    /// removed** — the doc still exists at `from` (never zero copies). Under the *wrong*
    /// (remove-first) ordering the source would already be gone **and** the write failed → the
    /// doc lost entirely; this test fails there, so it pins the ordering. The move never
    /// completed, so the file-state baseline is **not** prematurely re-keyed.
    #[test]
    fn relocation_abort_at_the_write_strands_neither_copy() {
        let repo = TempDir::new("relocate-abort");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("log", 1, log_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(log_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "log", 1).expect("the log.v1 snapshot");
        let v1 = log_v1_doc(&from);
        write_doc(repo.path(), "docs/changelog/changelog.md", &v1);

        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        // Force the write to the TO home to fail: occupy its atomic temp-sibling
        // `CHANGELOG.md.tmp` with a directory, so `persist` errors before the rename — the
        // move aborts AT the write, BEFORE the source removal (write-before-remove).
        fs::create_dir_all(repo.path().join("CHANGELOG.md.tmp")).expect("occupy temp sibling");

        let result = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        );
        assert!(
            result.is_err(),
            "the aborted write surfaces as an error: {result:?}"
        );

        // WRITE-BEFORE-REMOVE: the failed write to `to` means the source at `from` was NEVER
        // removed — the doc survives at its old home (never zero copies; nothing stranded).
        assert!(
            repo.path().join("docs/changelog/changelog.md").exists(),
            "the from copy survives the aborted write (never zero copies)"
        );
        // The write failed before the rename, so the target home was not created.
        assert!(
            !repo.path().join("CHANGELOG.md").exists(),
            "the aborted write left no partial target"
        );
        // The move never completed, so the baseline is intact — no premature re-key.
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("docs/changelog/changelog.md"),
            Some(hash_bytes(v1.as_bytes()).as_str()),
            "the from-home baseline is intact (re-key not applied on abort)"
        );
        assert!(
            record.get("CHANGELOG.md").is_none(),
            "no premature re-key to the target home"
        );
    }

    // ---- (h) the REAL shipped changelog v1→v2 relocation over the EMBEDDED pack ----

    /// (M38 inc-5 T1, done-criterion) The **shipped** `changelog` doctype's v1→v2 shape
    /// change, classified over the embedded dev pack: the prior-version snapshot
    /// (`schema-snapshots/changelog.v1.yaml`, folder home `changelog/`, slug H1) diffed
    /// against the current shipped schema (`placement: CHANGELOG.md` + `display-title:
    /// Changelog`) classifies to **exactly** `[Relocated {changelog/ → CHANGELOG.md},
    /// DisplayTitleChanged {Changelog}]`. Both shapes load through [`crate::pack::load_pack_schema`]
    /// so the stamp rides both identically (a value-bump, not a spurious field-add) and the
    /// two KaC sections are byte-identical — the sole diffs are the doctype-level relocation
    /// and H1 re-title. This is the classifier driving the real relocation, not a synthetic
    /// `log` stand-in.
    #[test]
    fn embedded_changelog_v1_to_v2_classifies_exactly_relocated_and_display_title() {
        use engine::packsource::{PackResourceKind, PackSource, ResourceId};
        use engine::schema_diff::{SchemaChange, schema_diff};

        let pack = crate::pack::EmbeddedPack::new();
        let bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("changelog"))
            .expect("the embedded pack ships the changelog schema");
        let current =
            crate::pack::load_pack_schema(&pack, &bytes).expect("current changelog loads");
        let prior = crate::pack::load_prior_schema(&pack, "changelog", 1)
            .expect("the shipped changelog.v1 snapshot loads");

        let diff = schema_diff(&prior, &current);
        assert_eq!(
            diff,
            vec![
                SchemaChange::Relocated {
                    // The RAW pack-declared home — both shapes here load straight from the
                    // pack (`load_pack_schema` / the snapshot store), docs-root-free, which
                    // is exactly the pair the engine classifier is contracted to see.
                    from: "changelog/".to_string(),
                    to: "CHANGELOG.md".to_string(),
                },
                SchemaChange::DisplayTitleChanged {
                    to: "Changelog".to_string(),
                },
            ],
            "the shipped changelog v1→v2 diff is exactly the relocation + the H1 re-title",
        );
    }

    /// (M38 inc-5 T1, done-criterion) A committed **v1-stamped** shipped `changelog`
    /// instance at the prior folder home (`changelog/changelog.md`, the docs-root-resolved
    /// `docs/changelog/changelog.md` old canonical) with a lowercase `# changelog` H1
    /// **relocates** to the literal root `CHANGELOG.md` through `migrate_committed_corpus`
    /// over the **embedded** pack (the same core the verb runs): byte-faithful — the H1
    /// fixed to `# Changelog`, the stamp value-bumped `1→2`, the two KaC section headers
    /// preserved; the old-home source removed; the file-state re-keyed. jigc's own repo has
    /// no managed changelog instance, so the proof rides this fixture.
    #[test]
    fn embedded_changelog_relocates_committed_doc_to_root_byte_faithful() {
        use engine::packsource::{PackResourceKind, PackSource, ResourceId};
        use engine::schema::SCHEMA_VERSION_FIELD;

        let repo = TempDir::new("changelog-relocate");
        let jigc_root = repo.path().join(".jigc");
        let pack = crate::pack::EmbeddedPack::new();

        let bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("changelog"))
            .expect("the embedded pack ships the changelog schema");
        let to = crate::pack::load_pack_schema(&pack, &bytes).expect("current changelog loads");
        let from = crate::pack::load_prior_schema(&pack, "changelog", 1)
            .expect("the shipped changelog.v1 snapshot loads");

        // A committed v1-stamped changelog at the OLD folder home, lowercase-slug H1 + the
        // two empty KaC section headers — the byte form a real pre-relocation instance has.
        let v1 = render(
            &from,
            &Instance {
                title: "changelog".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "unreleased-changes".to_string(),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "releases".to_string(),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            v1.starts_with("---\nschema-version: 1\n---\n\n# changelog\n"),
            "the committed doc is v1-stamped with a lowercase H1; got:\n{v1}"
        );
        write_doc(repo.path(), "docs/changelog/changelog.md", &v1);

        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(
            report.migrated,
            vec!["CHANGELOG.md".to_string()],
            "the shipped changelog relocates to its root placement home: {report:?}"
        );
        assert!(
            !repo.path().join("docs/changelog/changelog.md").exists(),
            "the old-home source is removed after the move"
        );
        let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("relocated home");
        assert!(
            moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
            "the stamp value-bumps 1→2 and the H1 is fixed to `# Changelog`; got:\n{moved}"
        );
        assert_conformant_and_stable(&to, &moved);

        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("CHANGELOG.md"),
            Some(hash_bytes(moved.as_bytes()).as_str()),
            "the target home is baselined at the migrated bytes"
        );
        assert!(
            record.get("docs/changelog/changelog.md").is_none(),
            "the old-home key is dropped (re-keyed, not orphaned)"
        );
    }

    // ---- the added-repeatable-section kind, through the verb core (M42 Inc-5 T6) ----

    /// The v1 snapshot of a `plan`: a `context` slot + an `outcome` slot, no repeatable.
    fn plan_v1_yaml() -> &'static str {
        "\
type: plan
location: plans/
id-from: title
sections:
  - id: context
    slot: { hint: \"c\" }
  - id: outcome
    slot: { hint: \"o\" }
"
    }

    /// The v2 current shape: a **wholly-new repeatable** `tasks` section between the two slots
    /// — the [`SchemaChange::AddedRepeatableSection`] shape.
    fn plan_v2_yaml() -> &'static [u8] {
        b"\
type: plan
location: plans/
id-from: title
sections:
  - id: context
    slot: { hint: \"c\" }
  - id: tasks
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one task\" } }
  - id: outcome
    slot: { hint: \"o\" }
"
    }

    /// A committed `plan` rendered against `schema` with the given `stamp`, `context`/`outcome`
    /// prose, and `tasks` items — the byte-stable form a real committed doc has.
    fn plan_doc(schema: &Schema, stamp: &str, tasks: Vec<ItemContent>) -> String {
        render(
            schema,
            &Instance {
                title: "Ship The Limiter".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar(stamp.to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "context".to_string(),
                        slot: Some("Limits were enforced ad hoc.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "tasks".to_string(),
                        items: tasks,
                        ..Default::default()
                    },
                    SectionContent {
                        id: "outcome".to_string(),
                        slot: Some("One limiter at the gateway.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        )
    }

    /// **A wholly-new repeatable section migrates through the verb** (M42 Inc-5 T6). A
    /// below-version doc (stamp `1`, current `2`) whose v1→v2 pair adds a repeatable section
    /// gets its empty `## Tasks` heading minted at the schema-ordered offset, conforms with
    /// **zero items**, and is restamped `1→2` — every prior byte preserved.
    ///
    /// Red before T6: the classifier emitted nothing for a wholly-new repeatable, so the doc
    /// rode the empty-diff backstop's *unclassifiable* route and the whole corpus was
    /// **refused** — a doctype could not grow a repeatable section at all.
    #[test]
    fn below_version_added_repeatable_section_mints_the_heading_and_restamps() {
        let repo = TempDir::new("added-repeatable");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("plan", 1, plan_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(plan_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "plan", 1).expect("the plan.v1 snapshot");

        // A conformant, v1-stamped committed doc — authored before the repeatable existed, so
        // it carries no `## Tasks` heading at all.
        let v1 = render(
            &from,
            &Instance {
                title: "Ship The Limiter".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "context".to_string(),
                        slot: Some("Limits were enforced ad hoc.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "outcome".to_string(),
                        slot: Some("One limiter at the gateway.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            !v1.contains("## Tasks"),
            "the pre-bump doc carries no repeatable section; got:\n{v1}"
        );
        write_doc(repo.path(), "docs/plans/ship-the-limiter.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(
            report.migrated,
            vec!["docs/plans/ship-the-limiter.md".to_string()],
            "the doc migrates, never blocks: {report:?}"
        );
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        let migrated =
            fs::read_to_string(repo.path().join("docs/plans/ship-the-limiter.md")).expect("read");
        assert!(
            migrated.contains("## Tasks"),
            "the repeatable heading is minted; got:\n{migrated}"
        );
        assert!(
            migrated.starts_with("---\nschema-version: 2\n---\n"),
            "the stamp value-bumps 1→2; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);

        // Zero items — a zero-item repeatable conforms; the CLI mints no prose.
        let inst = engine::write::instance_from_source(&to, &migrated).expect("re-parse under v2");
        let tasks = inst
            .sections
            .iter()
            .find(|s| s.id == "tasks")
            .expect("the tasks section is present");
        assert!(
            tasks.items.is_empty(),
            "the minted repeatable carries zero items; got {:?}",
            tasks.items
        );
        // Every prior byte is preserved: the migrated doc is the v1 doc plus the empty heading
        // block, with only the stamp digit flipped.
        assert_eq!(
            migrated
                .replace("## Tasks\n\n\n", "")
                .replace("schema-version: 2", "schema-version: 1"),
            v1,
            "the migration adds the empty heading and bumps the stamp — nothing else"
        );
    }

    /// **The re-run/idempotency guard, over the new heading-minting kind** (T2's guard, which
    /// the exhaustive `per_doc_changes` match forces this kind to declare itself to). A
    /// below-version doc an adopter **hand-authored** the new repeatable section into — heading
    /// *and* items already present — migrates **clean**: the mint is dropped (the block-insert
    /// would refuse an already-present section and strand the doc on a dead-end route), the
    /// authored items survive verbatim, and the doc restamps `1→2`.
    #[test]
    fn a_doc_already_carrying_the_added_repeatable_section_migrates_clean() {
        let repo = TempDir::new("added-repeatable-present");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("plan", 1, plan_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(plan_v2_yaml());

        // The hand-authored shape: still stamped `1`, but already carrying `## Tasks` with a
        // real item (the byte form an adopter who anticipated the bump has on disk).
        let v1 = plan_doc(
            &to,
            "1",
            vec![ItemContent {
                id: "rate-limit-the-gateway".to_string(),
                title: "Rate limit the gateway".to_string(),
                slot: Some("Enforce one limiter at the edge.".to_string()),
                ..Default::default()
            }],
        );
        write_doc(repo.path(), "docs/plans/ship-the-limiter.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(
            report.migrated,
            vec!["docs/plans/ship-the-limiter.md".to_string()],
            "a doc that already carries the section migrates — it is never stranded: {report:?}"
        );
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        let migrated =
            fs::read_to_string(repo.path().join("docs/plans/ship-the-limiter.md")).expect("read");
        assert_eq!(
            migrated.matches("## Tasks").count(),
            1,
            "exactly one heading — the mint is dropped, never re-spliced; got:\n{migrated}"
        );
        assert_eq!(
            migrated,
            v1.replace("schema-version: 1", "schema-version: 2"),
            "the doc differs from its authored form only in the stamp digit"
        );
        assert!(
            migrated.contains("Enforce one limiter at the edge."),
            "the authored item survives verbatim; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);
    }

    // ---- the added-item-field kind, through the verb core (M42 Inc-5 T7) ----

    /// The v1 snapshot of a `deferrals` doctype: one repeatable `entries` section (id-from title, a
    /// `trigger` field, a `body` prose slot) — the shipped `deferral-ledger` shape.
    fn deferrals_v1_yaml() -> &'static str {
        "\
type: deferrals
location: deferrals/
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: body, slot: { hint: \"the deferral\" } }
"
    }

    /// The v2 current shape: the item block grows a **defaulted** `kind` enum — the
    /// [`SchemaChange::AddedItemField`] deterministic arm.
    fn deferrals_v2_yaml() -> &'static [u8] {
        b"\
type: deferrals
location: deferrals/
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: kind, type: enum, of: [Decision, Idea], default: Decision }
        - { id: body, slot: { hint: \"the deferral\" } }
"
    }

    /// A committed `ledger` rendered against `schema` with the given stamp and two entries.
    fn deferrals_doc(schema: &Schema, stamp: &str) -> String {
        let entry = |id: &str, title: &str, trigger: &str, body: &str| ItemContent {
            id: id.to_string(),
            title: title.to_string(),
            slot: Some(body.to_string()),
            fields: vec![Field {
                key: "trigger".to_string(),
                value: Value::Scalar(trigger.to_string()),
            }],
            ..Default::default()
        };
        render(
            schema,
            &Instance {
                title: "Deferral Ledger".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar(stamp.to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "entries".to_string(),
                        items: vec![
                            entry(
                                "the-freeze-exempt-floor",
                                "The freeze-exempt floor",
                                "M39",
                                "A detect+route floor for freeze-exempt doctypes.",
                            ),
                            entry(
                                "the-abandon-path",
                                "The abandon path",
                                "M42",
                                "What a milestone's abandon path commits.",
                            ),
                        ],
                        ..Default::default()
                    },
                ],
            },
        )
    }

    /// **A defaulted new item field migrates every item, through the verb** (M42 Inc-5 T7). A
    /// below-version doc (stamp `1`, current `2`) whose v1→v2 pair adds a `default:`-carrying
    /// field to a repeatable item block gets the value spliced into **each** entry and is
    /// restamped `1→2` — every prior byte preserved.
    ///
    /// Red before T7: the classifier emitted nothing for an added item field, so the diff was
    /// `[]`, the empty-diff backstop refused the whole corpus (post-T1) — and pre-T1 the doc
    /// "migrated by accident": the stamp flipped and the declared default never landed.
    #[test]
    fn below_version_added_item_field_splices_every_item_and_restamps() {
        let repo = TempDir::new("added-item-field");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("deferrals", 1, deferrals_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(deferrals_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "deferrals", 1)
            .expect("the deferrals.v1 snapshot");

        let v1 = deferrals_doc(&from, "1");
        assert!(
            !v1.contains("kind:"),
            "the pre-bump doc carries no kind bullet; got:\n{v1}"
        );
        write_doc(repo.path(), "docs/deferrals/deferral-ledger.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to.clone(), 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(
            report.migrated,
            vec!["docs/deferrals/deferral-ledger.md".to_string()],
            "the doc migrates, never blocks: {report:?}"
        );
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        let migrated = fs::read_to_string(repo.path().join("docs/deferrals/deferral-ledger.md"))
            .expect("read");
        assert_eq!(
            migrated.matches("- kind: Decision").count(),
            2,
            "EVERY item carries the defaulted bullet; got:\n{migrated}"
        );
        assert!(
            migrated.starts_with("---\nschema-version: 2\n---\n"),
            "the stamp value-bumps 1→2; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);
        assert_eq!(
            migrated
                .replace("- kind: Decision\n", "")
                .replace("schema-version: 2", "schema-version: 1"),
            v1,
            "the migration adds the defaulted bullet per item and bumps the stamp — nothing else"
        );
    }

    /// **A required-no-default new item field blocks the doc** (the third arm, through the verb).
    /// The classifier names it `ProseNeeding { leaf: Some }` where pre-T7 it diffed to `[]` — a
    /// permanent mutual dead end (`validate` said *run the migration*, `migrate-corpus` said
    /// nothing at all and restamped). T2's guard drops the mint (the section heading is already
    /// there), so the **per-doc conformance gate** is the adjudicator: the doc is blocked, its
    /// bytes untouched and its stamp **not** bumped.
    #[test]
    fn below_version_required_item_field_blocks_the_doc_and_leaves_it_v0() {
        let repo = TempDir::new("required-item-field");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("deferrals", 1, deferrals_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        // v2 adds a **required, default-less** `owner` field to the item block.
        let to = v1_schema(
            b"\
type: deferrals
location: deferrals/
id-from: title
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: trigger, type: string }
        - { id: owner, type: string }
        - { id: body, slot: { hint: \"the deferral\" } }
",
        );
        let from = crate::pack::load_prior_schema(&pack, "deferrals", 1)
            .expect("the deferrals.v1 snapshot");

        let v1 = deferrals_doc(&from, "1");
        write_doc(repo.path(), "docs/deferrals/deferral-ledger.md", &v1);

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert!(
            report.migrated.is_empty(),
            "a doc needing per-item prose is never migrated: {report:?}"
        );
        assert_eq!(
            report.blocked.len(),
            1,
            "the doc is blocked with a route: {report:?}"
        );
        assert_eq!(report.blocked[0].0, "docs/deferrals/deferral-ledger.md");
        // THE DISCRIMINATING ASSERTION — the block alone does not distinguish T7 (pre-T7 the
        // change diffed to `[]` and the *backstop* refused it with the `build the transform
        // kind` route, a schema-authoring instruction). Now the change is **named**, so the
        // adjudicator is the per-doc conformance gate and the route is the actionable
        // **prose-authoring** one: the agent fills the required per-item field, then re-runs.
        assert_eq!(
            report.blocked[0].1,
            prose_needing_route("docs/deferrals/deferral-ledger.md"),
            "a named prose need routes to the author, never to `build the transform kind`"
        );

        let on_disk = fs::read_to_string(repo.path().join("docs/deferrals/deferral-ledger.md"))
            .expect("read");
        assert_eq!(
            on_disk, v1,
            "the blocked doc is byte-identical v0 — the stamp is NOT bumped"
        );
    }
}
