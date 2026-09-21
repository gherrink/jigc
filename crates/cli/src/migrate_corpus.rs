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
use engine::finding::{Finding, Findings, Location, Route, Severity};
use engine::packsource::PackSource;
use engine::schema::{SCHEMA_VERSION_FIELD, Schema, SectionBody};
use engine::schema_diff::{SchemaChange, SchemaChangeKind, schema_diff};
use engine::transform::{
    CorpusDoc, CorpusMigration, DocOutcome, HaltReason, TransformError, migrate_corpus,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One doctype's migration job: its current shape (`to`, stamp-injected) and its current
/// manifest schema-version (the value the stamp is filled/bumped to + the "already current"
/// match — a stamp *above* it blocks as a future/foreign stamp, never already-current). The **prior** shape (`from`) is resolved **per committed doc by stamp** in
/// [`migrate_committed_corpus`] — stamp-absent (v0) docs derive it from the doctype's *earliest*
/// shipped snapshot ([`v0_prior_shape`]); below-version (v1→v2) docs source it from the snapshot
/// at their own stamp via [`crate::pack::load_prior_schema`] — so it is not a per-doctype field.
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
    /// The resolved `placement-root` — `None` when the knob is unset, `Some("")` for the repo
    /// root ([`crate::start::placement_root`]'s vocabulary). `docs_root`'s twin, and for the
    /// same one reason: the walk consumes a **prior** home out of the snapshot store, and a
    /// snapshot stores its `placement.file` as the raw **declaration**, so the knob has to be
    /// re-applied to it exactly as schema resolution applies it to the current shape.
    ///
    /// Applied through [`crate::start::reroot_placement_file`] and **only to a declaration**:
    /// re-rooting an already-resolved home is that primitive's documented trap (under a root
    /// the resolved home has no leading component left, so the second re-root is a silent
    /// no-op). The current `to.placement.file` is already resolved (via `all_schemas`), so
    /// this is inert there.
    pub placement_root: Option<String>,
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
    /// Docs that could not migrate cleanly — **real [`Finding`]s**, sorted by target
    /// ([`blocked_finding`]; M42 completion audit, Finding 2). A refusal is the verb's
    /// machine-actionable output: each carries a `migrate-corpus.*` `code`, the stable
    /// `(code, target)` key a driver dedupes on, and a route. They ride the structural
    /// serialization seam ([`Findings`]), so they cannot be projected without one
    /// (`design/command-output-contract.md` → The membership test).
    ///
    /// They shipped as untyped `(path, route)` string tuples — **inaudible to a machine**:
    /// no code, no key, entirely outside the finding-key contract — while the run
    /// nonetheless exited **0**, so `validate` (exit 1, *"run migrate-corpus"*) →
    /// `migrate-corpus` (exit 0) → `validate` was an **infinite CI loop with nothing
    /// machine-readable naming why**. A non-empty `blocked` now **exits non-zero**
    /// ([`run`]).
    pub blocked: Findings,
    /// The committed files at a managed home that jigc was **never handed** — excluded from
    /// the fold before it runs, and **reported** as the store door's own adoption advisory
    /// ([`engine::validate::AdoptionInputs::unadopted`]), verbatim, from the one producer.
    /// Sorted by target, like [`Self::blocked`].
    ///
    /// **Its own field, deliberately not [`Self::blocked`]** — whose emptiness *is* the exit
    /// rule ([`run`]). A never-adopted foreign file is not this verb's subject
    /// (`design/corpus-migration.md` → The corpus walk; `design/validation.md` → The
    /// managed-vs-foreign discriminator): the corpus migration upgrades the **managed** corpus,
    /// and a brownfield repo's own Keep-a-Changelog `CHANGELOG.md` belongs to the adoption path
    /// (`jigc ingest` / `jigc migrate <path> --as <ty>`). Blocking on it exited 1 with a route
    /// — *"author the prose, then re-run"* — that changes nothing for a file jigc never wrote,
    /// **and** held the real corpus hostage: the fold halts at its first blocker, so a genuinely
    /// stale managed doc behind a foreign squatter was reported `deferred` and never migrated.
    ///
    /// Excluding it silently would be the sibling failure — this file's own *never a silent
    /// already-current* rule — so the set rides both surfaces: counted in the text headline and
    /// listed with its code and route, and serialized here for a driver.
    ///
    /// A key on a pinned envelope is **declared where its siblings are**, never merely shipped:
    /// `design/command-output-contract.md` → Evolution posture, *The M46 additive key* — which
    /// records why the set is not re-derivable from [`Self::migrated`] / [`Self::already_current`]
    /// / [`Self::blocked`] (an excluded file is in none of them).
    pub unadopted: Findings,
    /// The `set:`-derived leaves this run's migrations **left unfilled** — one advisory per
    /// added leaf whose declaration carries a `set:` deriver and no `default:`
    /// ([`unfilled_set_field_finding`]), derived once in the engine
    /// ([`engine::schema_diff::unfilled_set_leaves`]) from each migrated doc's own change list.
    /// Sorted by target, like [`Self::blocked`] and [`Self::unadopted`].
    ///
    /// **The loudness rider of M46 Increment 4** (`DECISIONS.md` → 2026-08-18 M46 planned). T1
    /// stopped the fold refusing an absence that already conforms — a `set:`-derived field's
    /// absence is conformance-clean, so no author can fill it and no re-run can change it. That
    /// makes the fold a byte no-op, and a silent no-op is its own hazard: a doctype author who
    /// adds a `set:`-bearing field expecting the corpus to carry a value would read `1 migrated`
    /// and nothing else. The run names what it left undone instead.
    ///
    /// **Its own field, deliberately not [`Self::blocked`]** — whose emptiness *is* the exit rule
    /// ([`run`]): nothing here is a refusal, the doc migrated, and the exit stays 0. The
    /// [`Self::unadopted`] precedent, for the same structural reason.
    ///
    /// A key on a pinned envelope is **declared where its siblings are**, never merely shipped:
    /// `design/command-output-contract.md` → the `migrate-corpus.*` sub-table (the code) and
    /// Evolution posture, *The M46 additive key: `unfilled`* (the key).
    pub unfilled: Findings,
    /// The short sha of the commit the verb landed its own migration in ([`commit_migration`]),
    /// or `None` when nothing was committed (nothing migrated, a re-run that staged no change,
    /// or a non-git worktree). Named in both surfaces — the operator/driver reads back *where*
    /// the migration landed (`design/corpus-migration.md` → The commit boundary).
    pub commit: Option<String>,
    /// The self-commit's captured non-blocking hook stream — **present-always**, the
    /// empty string when nothing was committed or no hook spoke (the hook_output
    /// producer axis; `design/command-output-contract.md` → Stream discipline). [`run`]
    /// relays the same string on the other channel.
    pub hook_output: String,
    /// The repo-relative paths the migration **touched** — every destination written *and*
    /// every relocation source removed. The self-commit's pathspec: the removed source is not
    /// recoverable from `migrated` (which carries only destinations), and staging the add half
    /// alone would land a **half-migration** — the silent-partial-commit class. Internal to the
    /// commit boundary, so it stays out of the report's serialized surface.
    #[serde(skip)]
    touched: Vec<String>,
    /// The docs whose migrated form is **written but not in `HEAD`** — an earlier run's
    /// migration whose commit was rejected, or a `--no-commit` run's writes ([`unlanded_paths`]),
    /// narrowed to the paths this run's walk found at the current version on disk. Sorted.
    ///
    /// They are current *on disk*, which is exactly why the pre-F11 report counted them
    /// `already current` (M48 Inc 9 T4) — a run that **landed** them announced `0 migrated,
    /// N already current` and listed each one `current`, while the only true sentence sat in the
    /// trailing commit line. That is the *headline describes the scan, the action sits below it*
    /// class through this door. They are held out of [`Self::already_current`] because `HEAD` is
    /// still behind, which is precisely what a fresh clone sees — the per-clone split N2 closed.
    ///
    /// **Text-only, like [`Self::no_commit`]**, and for the same reason: the envelope already
    /// discriminates the recovery (`dry_run: false` + `commit: <sha>` + `migrated: []`), and
    /// `commit` names the sha whose tree carries exactly these paths — re-derivable, not a
    /// withheld value (`design/command-output-contract.md` → the parity rule, whose fence is
    /// `crates/cli/tests/text_json_parity_axis.rs`).
    #[serde(skip)]
    pub(crate) unlanded: Vec<String>,
    /// `--dry-run` — the run **suppressed the write** (nothing on disk changed). Threaded into
    /// the report so both surfaces say what the run *is*: the render frames every migrated path
    /// as *"would migrate"* under a *"dry run — nothing written"* header (past-tense
    /// *"migrated N"* over writes that never happened is a Law 1 lie — M43 surface census, F2),
    /// and the JSON carries `dry_run` so a driver distinguishes the preview from an applying run
    /// (`commit: null` alone cannot — `--no-commit` also commits nothing).
    pub dry_run: bool,
    /// `--no-commit` — the run **wrote** the migrated bytes but staged and committed nothing.
    /// Internal to the text renderer's commit-status line (an applying run that landed nothing
    /// says so explicitly, distinct from both the dry-run preview and the committed run); the
    /// JSON already distinguishes it by `dry_run: false` + `commit: null`.
    #[serde(skip)]
    pub(crate) no_commit: bool,
}

/// Run `jigc migrate-corpus` against `cwd`: locate the repo + project layer, build the
/// frozen persisted doctypes' migration jobs from the pack, migrate the committed corpus,
/// **land it in a pathspec-limited self-commit** ([`commit_migration`]), render the report
/// through `format`, and print it. A locator error — or a **rejected commit** (a `pre-commit`
/// hook declining the managed-doc writes) — routes to stderr and exits non-zero.
///
/// # A refused migration exits non-zero (M42 completion audit, Finding 2)
///
/// The exit is **0 iff nothing is blocked**. It used to be 0 *unconditionally* on any `Ok`,
/// on the rationale that *blocked docs are an expected interim state, not a failure* — so a
/// run in which **every doc was refused** reported success. That is wrong twice over:
///
/// - the three Increment-5 refusal classes (the empty-diff backstop, the narrowing refusal,
///   the removed-field refusal) are the ones the roadmap calls **"refuse loudly"** — and a
///   refusal that exits 0 is not loud, it is *inaudible*;
/// - combined with the M42 `validate` exit-flip it is an **infinite CI loop**: `validate` →
///   exit 1, *"run `jigc migrate-corpus`"* → `migrate-corpus` → exit **0** → `validate` →
///   exit 1 → forever. Something has to say *"this did not work"*, and the verb that refused
///   is the one that knows.
///
/// The interim-state intuition was not wrong about the *docs* — a blocked doc is genuinely a
/// waypoint, and the migration's other writes still land and still commit. It was wrong about
/// the **run**: a corpus with a refused doc in it is **not migrated**, and the caller that
/// asked for it to be migrated must hear so. The findings carry the *why* machine-readably.
///
/// [`Options`] narrows what the run **applies**: `--no-commit` keeps the writes but lands
/// nothing; `--dry-run` suppresses the writes too, printing the identical report an applying
/// run would print.
pub fn run(cwd: &Path, format: Format, options: Options) -> Outcome {
    match migrate_in_repo(cwd, options) {
        Ok(report) => {
            println!("{}", render::corpus_migration(format, &report));
            // The self-commit's captured non-blocking hook stream — the same string the
            // report's `hook_output` key carries, relayed on the other channel (stderr
            // under `--format json`, the delimited stdout section on agent-text; the
            // hook_output producer axis).
            crate::task::relay_hook_output(format, &report.hook_output);
            // The log carries **what the run decided** — every set the report speaks with,
            // on both branches. A run that declined to act on three files is not the same
            // event as a run that found nothing, a run that left two `set:`-derived leaves
            // absent is not the same event as one that filled everything the schema
            // declares, and a run that refused a doc is not the same event as one that
            // landed it: the log is where those differences are readable after the fact.
            // Selecting one set per branch made the record answer only part of that — the
            // exit-0 branch was silent about `unfilled`, and the blocking branch dropped the
            // adoption declines the exit-0 branch logs (M46 completion audit, finding F3).
            let decided: Vec<Finding> = report
                .blocked
                .iter()
                .chain(report.unadopted.iter())
                .chain(report.unfilled.iter())
                .cloned()
                .collect();
            // The **exit rule is unmoved** by that merge: `blocked`'s emptiness is the whole
            // of it — 0 when nothing was refused, 1 when something was — and neither
            // `unadopted` nor `unfilled` is a refusal (each field's own doc-comment records
            // why it is deliberately not `blocked`). `Outcome::with_findings` carries the
            // status alongside the codes, so the two are stated in one place, once.
            Outcome::with_findings(u8::from(!report.blocked.is_empty()), &decided)
        }
        // The door's half of the survivable frame (M47 Inc 3 T7). `commit_migration` stages
        // the touched paths *before* it commits, so a rejection leaves the migrated bytes
        // written and staged — and since N2 the re-run judges currency off the **committed**
        // corpus, so the identical `jigc migrate-corpus` re-stages and lands them rather than
        // reporting "already current" at exit 0.
        Err(err) => crate::task::surface_commit_rejection(
            format,
            &err,
            &crate::task::RejectionFrame {
                code: crate::invocation_log::ERROR_MIGRATE_CORPUS_REJECTED,
                // The door's own verb: this run's subject is the **whole corpus**, which jigc
                // has no address for, and at most one refusal exists per invocation — so the
                // key is unique per instance with the verb as its target, on the
                // `create.*` / `pack-probe-integrity` precedent (a bare declared identifier,
                // never a doc URI). Declared at
                // `design/command-output-contract.md` → the `*.commit-rejected` row.
                target: "migrate-corpus".to_string(),
                survived: "nothing was committed — the migrated bytes are written and staged, \
                           and the corpus is still recorded as unmigrated"
                    .to_string(),
                // G-47's one asymmetric cell: the hook-cell clause above says the bytes are
                // *staged*, and in the non-hook cell the stage is precisely what failed. One
                // clause across both would tell an operator to look for an index entry that
                // does not exist, at the moment they are recovering.
                survived_non_hook: Some(
                    "nothing was committed — the migrated bytes are written to disk, the stage \
                     did not complete, and the corpus is still recorded as unmigrated"
                        .to_string(),
                ),
                rerun: "jigc migrate-corpus".to_string(),
            },
            // `commit_migration` runs no shared-executor worktree rollback, so this door
            // carries no conflicts to fold.
            &[],
        ),
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
    // Its twin for the placement half of the walk — re-applied to a prior snapshot's raw
    // `placement:` **declaration**, never to the already-resolved current home.
    let placement_root = crate::start::placement_root(&resolved).map(str::to_string);

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
            placement_root: placement_root.clone(),
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
    if options.commits()
        && let Some((sha, hook_output)) = commit_migration(&jigc_home, &report.touched)?
    {
        report.commit = Some(sha);
        report.hook_output = hook_output;
    }
    Ok(report)
}

/// The message the verb's self-commit carries (the [`crate::setup`] install-commit precedent:
/// one fixed, conventional subject — the verb's writes are one kind of change).
const MIGRATION_COMMIT_MESSAGE: &str =
    "chore(jigc): migrate the managed corpus to the current schema versions";

/// **Land the migration** — stage exactly `touched` (every destination written and every
/// relocation source removed) and commit those paths, returning the short sha + the
/// commit's captured non-blocking hook stream (`None` when nothing was committed).
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
fn commit_migration(repo_root: &Path, touched: &[String]) -> Result<Option<(String, String)>> {
    if touched.is_empty() {
        return Ok(None);
    }
    // No git work tree → nothing to land (the docs are written; the commit is the convenience).
    if !git_ok(repo_root, &["rev-parse", "--is-inside-work-tree"]) {
        return Ok(None);
    }
    let paths: Vec<&str> = touched
        .iter()
        .filter(|p| committable(repo_root, p))
        .map(String::as_str)
        .collect();
    if paths.is_empty() {
        return Ok(None);
    }

    // Stage exactly those paths — never a blanket `git add -A`. A tracked-and-removed
    // relocation source stages as its deletion here (the move's other half).
    //
    // **Committable is wider than addable** (N2): a relocation source an earlier, *rejected* run
    // already staged as deleted is in neither the index nor the worktree, so `git add` would
    // `fatal: pathspec … did not match any files` on it — while `git commit -- <path>` records
    // the deletion from `HEAD` exactly as intended. So the two pathspecs are computed with two
    // predicates: [`stageable`] for the `add`, [`committable`] for the commit.
    let addable: Vec<&str> = paths
        .iter()
        .copied()
        .filter(|p| stageable(repo_root, p))
        .collect();
    if !addable.is_empty() {
        let mut add: Vec<&str> = vec!["add", "--"];
        add.extend(&addable);
        // N20 — the stage is the one non-hook failure point inside this door's commit
        // transaction (a stale `.git/index.lock` is its ordinary cause). Marked so the door's
        // surface frames it with its code, clause and re-run instead of dropping the frame;
        // the post-commit `rev-parse` below is deliberately NOT marked, because by then the
        // commit has landed and *"nothing was committed"* would be false.
        git_run(repo_root, &add).map_err(crate::task::mark_commit_failure)?;
    }

    // Nothing staged among our paths (a re-run over an already-migrated corpus) → no commit,
    // no empty commit. `git diff --cached --quiet` exits 0 when there is no staged diff.
    let mut diff: Vec<&str> = vec!["diff", "--cached", "--quiet", "--"];
    diff.extend(&paths);
    if git_ok(repo_root, &diff) {
        return Ok(None);
    }

    // The hook-capable commit runs through the ONE seam that RETURNS the captured
    // non-blocking hook stream (`crate::task::git_commit_capture`, the hook_output
    // producer axis) — never the output-discarding `git_run`, which was this producer's
    // defect. The hook posture is unchanged: never `--no-verify`, a rejection surfaces
    // git's bytes verbatim and fails the run loudly.
    let mut commit: Vec<&std::ffi::OsStr> = vec![
        std::ffi::OsStr::new("-m"),
        std::ffi::OsStr::new(MIGRATION_COMMIT_MESSAGE),
        std::ffi::OsStr::new("--"),
    ];
    commit.extend(paths.iter().map(std::ffi::OsStr::new));
    let hook_output =
        crate::task::git_commit_capture(&crate::repo::SeamSubject::live(repo_root), &commit)?;

    let sha = git_stdout(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(Some((sha, hook_output)))
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

/// Whether `path` belongs in the **commit** pathspec: anything [`stageable`] admits, plus a path
/// that is only in `HEAD` — an earlier, rejected run's relocation source, already staged as a
/// deletion and therefore in neither the index nor the worktree. `git add` cannot name such a
/// path, but `git commit -- <path>` must, or the recovery lands the add half alone and leaves a
/// half-migration (`design/corpus-migration.md` → The commit boundary).
fn committable(repo_root: &Path, path: &str) -> bool {
    stageable(repo_root, path) || exists_in(repo_root, path, Corpus::Head)
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

/// `git <args>`' **untrimmed** stdout, lossily decoded — `None` when git could not be spawned
/// or exited non-zero. The predicate-form sibling of [`git_stdout`] for the reads whose failure
/// is a fact (no `HEAD` yet, a path absent from the tree) and whose bytes must survive verbatim:
/// [`git_stdout`]'s `trim()` would eat a doc's trailing newline and the `-z` walk's separators.
fn git_stdout_raw(repo_root: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Migrate the committed corpus under `repo_root` for each [`DoctypeMigration`], per-doc
/// gated and WIP-safe, re-baselining any rewritten doc that already carried a file-state
/// hash. `jigc_root` is the `.jigc/` dir holding the file-state record; `pack` is the
/// snapshot store the below-version prior shapes are sourced from.
///
/// The **prior shape (`from`) is resolved per committed doc by its schema-version stamp**
/// (`design/corpus-migration.md` → Prior-schema sourcing):
/// - **at** the doctype's current version → `already-current`, byte-untouched.
/// - **above** the current version (a future/foreign stamp, 2026-07-24) → **blocked** with a
///   route — this binary has no schema to migrate the doc *to*, and a silent
///   `already-current` would hide it from every future run ([`future_stamp_finding`]).
/// - **stamp absent** (the v0 corpus state) → `from` = the doctype's **earliest shipped
///   snapshot**, stamp-stripped ([`v0_prior_shape`]; `strip_stamp(to)` only for a doctype that
///   has never bumped, whose earliest shape *is* its current one). The diff is the whole
///   v0→current chain — the stamp `added-optional-field` (the live Inc-3 dogfood) *plus* every
///   structural link since v1 — and the stamp is *added* at the current value.
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
        blocked: Findings::default(),
        unadopted: Findings::default(),
        unfilled: Findings::default(),
        commit: None,
        hook_output: String::new(),
        touched: Vec::new(),
        unlanded: Vec::new(),
        dry_run: options.dry_run,
        no_commit: options.no_commit,
    };

    // THE MANAGED-VS-FOREIGN DISCRIMINATOR'S THREE PACK FACTS, resolved once for the run
    // (`design/validation.md` → The managed-vs-foreign discriminator). The engine produces none
    // of them — the manifest version map, the shipped prior-version shapes and the doctypes
    // whose `migrate-<ty>` workflow exists are pack facts the CLI resolves and threads in, the
    // determinism boundary — and they are the **same three helpers** the store door feeds the
    // same question (`crate::cli` → `run_validate_store`), which is what makes the advisory
    // below identical rather than merely similar.
    let versions = pack::frozen_doctype_versions(pack);
    let priors = pack::prior_doctype_schemas(pack, &versions);
    let migratable = pack::migratable_doctypes(pack);
    let adoption = engine::validate::AdoptionInputs::new(&versions, &priors, &migratable);

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
        // The stamp-absent (v0) arm's prior shape, resolved **once per doctype** (a pack read).
        let v0_from = v0_prior_shape(pack, dt, &to_diff);
        // THE UNLANDED PATHSPEC (N2): what `HEAD` is still missing because an earlier run wrote
        // the migration and its commit was rejected. Computed **before** the fold applies this
        // run's writes, so it names only the *earlier* run's residue. It is both the recovery's
        // commit pathspec and — narrowed to the docs the walk below reports current — the set
        // the report must NOT call `already current` (F11).
        //
        // BOTH READS OF THE WALK FAIL CLOSED (M52 Increment 7 / T2). A snapshot the store
        // cannot answer for makes the doctype's home history unknowable, so neither the
        // recovery audit nor the fold may proceed over a silently narrower union: the doctype
        // is refused whole, with one blocking finding keyed at the snapshot the pack owes, and
        // not one of its documents is reported migrated, current or anything else.
        let unlanded = match unlanded_paths(pack, repo_root, dt) {
            Ok(paths) => paths,
            Err(finding) => {
                report.blocked.push(finding);
                continue;
            }
        };
        report.unlanded.extend(unlanded.iter().cloned());
        report.touched.extend(unlanded);
        // Each candidate is `(source, destination)` — the FROM home the walk found the
        // committed instance at, and the path the gated bytes land at
        // (`design/corpus-migration.md` → Relocation: the walk keys on the from home).
        let candidates = match candidate_docs(pack, repo_root, dt, Corpus::Worktree) {
            Ok(candidates) => candidates,
            Err(finding) => {
                report.blocked.push(finding);
                continue;
            }
        };
        for (rel_key, target_key) in candidates {
            let Ok(bytes) = std::fs::read(repo_root.join(&rel_key)) else {
                continue; // read race: skip; the next run re-checks.
            };
            let source = String::from_utf8_lossy(&bytes).into_owned();
            match read_stamp_from_source(&source) {
                // At the current version: already current, byte-untouched.
                Some(s) if s == dt.version => report.already_current.push(rel_key),
                // Above the current version (2026-07-24, the sibling-hunt item 1): an
                // OOB-planted or foreign-future stamp. Blocked with a route — never a
                // silent `already-current` (the same never-a-silent-skip rule as the
                // missing-snapshot arm): this binary has no schema to migrate the doc
                // *to*, so reporting it current would hide it from every future run.
                Some(s) if s > dt.version => {
                    report
                        .blocked
                        .push(future_stamp_finding(&rel_key, &dt.ty, s, dt.version));
                }
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
                            report.blocked.push(unclassifiable_change_finding(
                                &rel_key, &dt.ty, k, dt.version,
                            ));
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
                        if let Some(SchemaChange::NarrowedCardinality { locus, field }) = diff
                            .iter()
                            .find(|c| matches!(c, SchemaChange::NarrowedCardinality { .. }))
                        {
                            report.blocked.push(narrowed_cardinality_finding(
                                &rel_key, &dt.ty, locus, field, k, dt.version,
                            ));
                            continue;
                        }
                        // THE REMOVAL REFUSAL (the recorded pick — `DECISIONS.md` → 2026-07-13
                        // M42 Inc-5 T5: refuse, not strip). A dropped leaf is refused here, beside
                        // the backstop and the narrowing and for the same reason: there is nothing
                        // an operator can do to *this doc*, so the fold's halt route would be a
                        // lie, and the repair is a schema-authoring one. The committed value stays
                        // on disk — **No-data-loss** is a declared property of this pair.
                        if let Some(SchemaChange::RemovedField { locus, field }) = diff
                            .iter()
                            .find(|c| matches!(c, SchemaChange::RemovedField { .. }))
                        {
                            report.blocked.push(removed_field_finding(
                                &rel_key, &dt.ty, locus, field, k, dt.version,
                            ));
                            continue;
                        }
                        // THE ITEM-SLOT REMOVAL REFUSAL — the same pick at the item locus,
                        // routed separately because what it names is a **prose slot**, not a
                        // field line: telling an author to restore a field they never declared
                        // is a route that cannot be followed.
                        if let Some(SchemaChange::RemovedItemSlot { locus, leaf }) = diff
                            .iter()
                            .find(|c| matches!(c, SchemaChange::RemovedItemSlot { .. }))
                        {
                            report.blocked.push(removed_item_slot_finding(
                                &rel_key, &dt.ty, locus, leaf, k, dt.version,
                            ));
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
                        report
                            .blocked
                            .push(missing_snapshot_finding(&rel_key, &dt.ty, k));
                    }
                },
                // Stamp absent (the v0 corpus state): the add-field path (the stamp is *added*
                // at the **current** value via its `default`, so no post-fold bump) — over the
                // doctype's genuine v0 shape ([`v0_prior_shape`]), so the diff is the whole
                // v0→current chain, not just the stamp.
                None => {
                    // NOT THIS VERB'S SUBJECT — asked **before the fold**, so a never-adopted
                    // foreign file never becomes a `PreparedDoc` at all. Stamp-absence is the
                    // only arm where the question arises: a stamp *is* jigc's own hand
                    // (`classify_provenance`' first arm), so a stamped doc is managed by
                    // definition and the discriminator would answer `false` for it anyway.
                    //
                    // The discriminator is the **shipped** one, asked through the **one**
                    // producer of the advisory, fed the same `CascadeDefs::all_schemas` shape
                    // the store door feeds it — no second classifier, no second check id, no
                    // second route. Reported, never silently skipped (below, and in both
                    // surfaces); it holds neither `blocked` nor `migrated` nor
                    // `already_current`, and therefore not the exit.
                    // The identity the file carries at its home, by the one rule the
                    // store enumerator uses (`engine::index::instance_slug`) — a placement
                    // singleton's slug is its type id, never `CHANGELOG`.
                    let slug = engine::index::instance_slug(
                        &dt.ty,
                        &dt.to,
                        std::path::Path::new(&rel_key),
                    )
                    .unwrap_or_default();
                    if let Some(advisory) =
                        adoption.unadopted(&dt.ty, &slug, &dt.to, &source, &rel_key)
                    {
                        report.unadopted.push(advisory);
                        continue;
                    }
                    let from = v0_from.clone();
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
                            .push(destination_collision_finding(id, target));
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
                // THE LOUDNESS RIDER (M46 Inc-4 T2): what this doc's fold left unfilled. Derived
                // **once, in the engine**, from this doc's own change list against the schema the
                // fold actually used — `prep.to`, the stamp-defaulted clone, so the value the CLI
                // threads in for the schema-version stamp is excluded by its `default:` rather
                // than by a second list of exceptions. Reported at the destination the bytes
                // landed at, so the address a reader follows is the file that now exists.
                for leaf in engine::schema_diff::unfilled_set_leaves(&prep.to, &prep.changes) {
                    report
                        .unfilled
                        .push(unfilled_set_field_finding(target, &leaf));
                }
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
                // THE HALTED DOC IS TOLD WHAT HALTED IT (M46 Inc-4 T3). The run's *first* refusal
                // carries its own cause out of the fold; every doc behind it is untouched for a
                // different reason — it was never reached — and keeps the deferral.
                report.blocked.push(match &result.halt_reason {
                    Some(reason) if result.halted_at == Some(i) => halt_finding(id, reason),
                    _ => deferred_finding(id),
                });
            }
        }
    }

    report.migrated.sort();
    report.already_current.sort();
    // F11 — a doc whose migrated form `HEAD` is still missing is **not** `already current`; it is
    // an earlier run's unlanded migration, which a committing run lands. Partitioned off a SET,
    // so the split never depends on the order the doctype walk contributed its paths in, and the
    // two lists stay path-sorted (`already_current` is sorted just above, and a filter preserves
    // that). The unlanded pathspec's other members — a relocation's removed source — name no doc
    // the walk reports, so intersecting with `already_current` is what narrows it to documents.
    let unlanded: std::collections::BTreeSet<String> =
        std::mem::take(&mut report.unlanded).into_iter().collect();
    report.unlanded = report
        .already_current
        .iter()
        .filter(|path| unlanded.contains(*path))
        .cloned()
        .collect();
    report
        .already_current
        .retain(|path| !unlanded.contains(path));
    // Sorted by the stable target (the doc's path) — the findings collection is the seam, so
    // it is re-wrapped rather than sorted in place. The adoption set sorts the same way, for
    // the same reason: the doctype walk contributes its paths in doctype order, and neither
    // report list may depend on that.
    let mut blocked = std::mem::take(&mut report.blocked).into_vec();
    blocked.sort_by(|a, b| a.key().target.cmp(&b.key().target));
    report.blocked = blocked.into();
    let mut unadopted = std::mem::take(&mut report.unadopted).into_vec();
    unadopted.sort_by(|a, b| a.key().target.cmp(&b.key().target));
    report.unadopted = unadopted.into();
    let mut unfilled = std::mem::take(&mut report.unfilled).into_vec();
    unfilled.sort_by(|a, b| a.key().target.cmp(&b.key().target));
    report.unfilled = unfilled.into();
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
            // A nested-locus `ProseNeeding` mints no `## Heading` either — its section is
            // already there by construction — so the probe keys on the locus's SECTION, which
            // is what a heading collision is about at every depth.
            SchemaChange::AddedOptionalSection { locus }
            | SchemaChange::AddedRepeatableSection { locus }
            | SchemaChange::ProseNeeding { locus, .. } => {
                !has_section_heading(source, locus.section())
            }
            // Every other kind splices inside an existing section (or no bytes at all), so it
            // has no heading to collide with and is always kept. `AddedItemField` mints no
            // heading either — it splices a field bullet **into each item of an existing
            // repeatable section** — and it carries its own re-run guard *per item* (the driver
            // skips an item that already has the bullet, which a whole-change filter here could
            // not express: one doc can hold both kinds of item). `AddedItemSlot` mints a
            // `#### <Leaf-Title>` **sub-label inside each item**, never a `## Heading`, and
            // carries the identical per-item guard for the identical reason: one doc can hold an
            // item that already renders the sub-labels beside one that does not.
            SchemaChange::AddedOptionalField { .. }
            | SchemaChange::AddedItemField { .. }
            | SchemaChange::AddedItemSlot { .. }
            | SchemaChange::OptionalRelaxed { .. }
            | SchemaChange::WidenedCardinality { .. }
            | SchemaChange::NarrowedCardinality { .. }
            | SchemaChange::RemovedField { .. }
            | SchemaChange::RemovedItemSlot { .. }
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
            SchemaChange::ValueRemapped { locus, field, map } if map.is_empty() => {
                // Keyed by the locus **path**, which is what the map-gap refusal's route
                // names (M50 Increment 7 / T5). Keyed by `locus.section()` the lookup asked
                // a question the route did not: a nested block's rename resolved to the
                // *outer* block's authored entry — `changelog.releases` declares `date`
                // outside and inside — so one map would silently answer for two different
                // fields, and the key the operator was told to declare was not a key at all.
                let map = authored_remap(ty, &locus.to_string(), &field).unwrap_or(map);
                SchemaChange::ValueRemapped { locus, field, map }
            }
            other => other,
        })
        .collect()
}

/// The CLI-authored old→new value maps for enum-member renames — keyed by
/// `(doctype, locus path, field)`. An enum rename is a semantic choice the schema pair cannot
/// recover, so the mapping is declared here (the migration input the classifier emits
/// blank), not derived. Returns `None` for any (doctype, locus, field) with no authored
/// rename, leaving the classifier's empty map — the driver then blocks that doc loudly.
///
/// **The key is the locus path** ([`engine::schema_diff::Locus`]'s `Display` — `entries` at a
/// section or its item block, `releases/changes` one level down), never the bare section id,
/// so it discriminates the block the rename was made in and matches, byte for byte, the key
/// the map-gap refusal's route tells the operator to declare (M50 Increment 7 / T5). At loci
/// 1 and 2 the two spellings coincide, which is why the entry below is unchanged.
///
/// The one authored entry: the M41 F4 `deferral-ledger` `entries.kind` rename `D`→`Decision`
/// / `I`→`Idea` (the first methodology v1→v2 migration).
fn authored_remap(ty: &str, locus: &str, field: &str) -> Option<BTreeMap<String, String>> {
    match (ty, locus, field) {
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

/// **The one refusal envelope** — a blocking [`Finding`] for a doc the migration could not
/// migrate, targeted at the doc's **filesystem path** (M42 completion audit, Finding 2).
///
/// # Why the path, and not the doc's `<type>:<slug>` URI
///
/// The URI is the target form for every *content* family, and a committed managed doc has one —
/// so this is a deliberate choice, not an oversight. The subject of a migration refusal is the
/// **file the verb could not move or rewrite**, and during a relocation the URI is precisely the
/// thing that is *contested*: two files — the prior-home strand and the placement home — can
/// claim one identity, which is the entire content of [`destination_collision_finding`]. Keyed on
/// the URI, those two refusals would **collide on one `(code, target)`**; keyed on the path they
/// discriminate, and the operator reads back exactly the file to repair. It joins the declared
/// **file-path** form beside `file-state.*`, `finalize.promote-clobber` and
/// `schema-conformance.unadopted-instance` — each there for the same reason: the finding is about
/// a file whose managed identity is absent or contested
/// (`design/command-output-contract.md` → the six declared target forms).
///
/// **One member of the family is keyed on something that is not a file, and it is declared**
/// (M52 Increment 7 / T2): [`enumeration_missing_snapshot_finding`] refuses *before* any
/// document is in hand, so its `path` argument carries the `<ty>@v<k>` snapshot the pack owes.
/// Every other caller passes a repo-relative file path.
///
/// `message` carries the diagnosis, `route` the repair — never the two fused, so a driver can act
/// on the route alone.
fn blocked_finding(code: &str, path: &str, message: String, route: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        message,
        Some(Location::addressed(path, 1, 1)),
        Some(route.into()),
    )
}

/// **The loudness rider's advisory** (M46 Inc-4 T2): a migrated doc carried an added leaf whose
/// declared field names a `set:` deriver and no `default:`, so the fold placed **no bytes** for
/// it and left it absent — conformantly (`validate::is_author_required` never asks for a `set:`
/// field, which is exactly why T1 stopped blocking the doc over it).
///
/// **Advisory, never blocking**, and it rides [`CorpusMigrationReport::unfilled`] rather than
/// `blocked`: the doc migrated, and the run's exit is the refusal set's emptiness ([`run`]).
///
/// **Targeted at `<path>#<locus-path>/<field>` at EVERY locus** — the file-path form every
/// `migrate-corpus.*` finding takes (the path is not a doc URI, so the target parses as no
/// address and resolves through no verb at any locus), carrying the **declared leaf's schema
/// locus** as its fragment so two unfilled leaves in one doc discriminate rather than collapsing
/// onto one `(code, target)`. It is a key naming *the file to open and the leaf to look at*, not
/// a write address — which is why the item loci keep it rather than growing id-hop placeholders
/// the key cannot fill (below).
///
/// **The locus is carried into what the finding SAYS, because the loci have different write
/// addresses** (M46 completion audit, finding F2; M50 Increment 6 audit for the third). A
/// simple/header leaf is written at `#<section>/<field>`; an item leaf only at
/// `#<section>/<item-id>/<field>`; and a **nested** item leaf only at
/// `#<section>/<item-id>/<nested-section>/<nested-block-item-id>/<field>` — the alternation
/// [`engine::address::MAX_FRAGMENT_HOPS`] states, **one id hop per level**, each hole spelled by
/// [`item_write_fragment`]. This report holds **no**
/// item id, at any depth. It must not mint one per item either: a repeatable section carrying
/// **zero** items still has an unfilled leaf, and a per-item finding would go silent on exactly
/// the corpus a doctype author most needs to hear about — so the finding stays **one per leaf**
/// and the route names the item-qualified *form* ([`item_write_fragment`]) plus the read that
/// enumerates the real ids (`jigc doc show`), the [`engine::store`] no-such-section precedent. It
/// fabricates no address.
///
/// **The locus's `Display` is prose, never an address** (M50 Increment 6 audit). At locus 3 it
/// renders the diagnostic path `releases/changes`, which *omits the outer item hop* — so
/// interpolated into a verb position it composes a command that answers `store.no-such-item` no
/// matter which id a reader substitutes. The emitted read therefore names
/// [`engine::schema_diff::Locus::section`] (whose one render hands back every anchor the write
/// needs) and the emitted write names the hop-complete form; only the *prose* keeps the path.
///
/// **The emitted commands are bytes a reader runs, so they are held to running.** Driving them
/// verbatim found two more breaks shipped since M46, neither in any finding: the write passed its
/// value as a bare positional at **every** locus (`jigc doc set-field` answers `error: unexpected
/// argument`), and at **both item** loci the read carried `--task <task-id>` against a doc this
/// very run had just **committed** and no task had staged (`store.not-staged`, whose own route
/// says to drop the flag). A [`Route::human`] escapes the argv fence a [`Route::mechanical`] is held to —
/// which is exactly why they survived — so the fence here is the acceptance arm, which fills only
/// the reader's own values into the route's bytes and **runs them**, at both item loci
/// (`crates/cli/tests/migrate_locus_axis.rs`).
///
/// **The route splits on the `set:` kind**, through the one authority
/// ([`engine::schema::is_machine_maintained_absolute`]): a **machine-maintained absolute** (the
/// freeze stamp, a milestone transition) is informational — no author write may set it, so
/// naming an action would be a route nobody can take, and the locus changes nothing about that;
/// an **author-overridable** `on-create` takes a **human** route naming the write path *at its
/// own locus*. Neither is mechanical: a `jigc doc set-field` argv needs a task id, and a
/// migration report holds none — a mechanical route is a promise the command runs from where the
/// reader stands.
fn unfilled_set_field_finding(
    path: &str,
    leaf: &engine::schema_diff::UnfilledSetLeaf<'_>,
) -> Finding {
    let locus = &leaf.locus;
    let field = leaf.field;
    let id = &field.id;
    let set = field.set.as_deref().unwrap_or_default();
    let route = if engine::schema::is_machine_maintained_absolute(field) {
        Route::informational(format!(
            "no action needed — `{id}` is machine-maintained (`set: {set}`): jigc derives its \
             value and no `jigc doc` write may set it"
        ))
    } else if locus.is_item() {
        // The two verb positions take the ADDRESS forms, never the locus path: the read at the
        // section the ids are read out of, the write at the hop-complete item chain.
        let section = locus.section();
        let write = item_write_fragment(locus, id);
        // A nested leaf's read and write differ by more than an id hop — the read is one level
        // out — so the route says how the two relate rather than leaving the reader to infer it.
        let through = if locus.is_nested() {
            format!(
                "; each item of `{locus}` sits **inside** a `{section}` item, so the write \
                 address carries one id hop per level and that one read renders every anchor it \
                 needs"
            )
        } else {
            String::new()
        };
        Route::human(format!(
            "`{id}` is author-overridable (`set: {set}`) and is declared per **item** of \
             `{locus}`, so a write names the item: read the items back with `jigc doc show \
             <doc-address>#{section}` — the task-less read, since the doc this run just migrated \
             is committed and no task has staged it — then set one with `jigc doc set-field \
             <doc-address>#{write} --value <value> --task <task-id>`{through}; this report holds \
             no task id, so the write is a form to fill, not a runnable command"
        ))
    } else {
        Route::human(format!(
            "`{id}` is author-overridable (`set: {set}`) — if this record warrants a value, set \
             it inside a task with `jigc doc set-field <doc-address>#{locus}/{id} --value \
             <value> --task <task-id>`; this report holds no task id, so it composes no runnable \
             command"
        ))
    };
    let message = if locus.is_item() {
        format!(
            "`{path}` migrated with `{locus}`'s **item** field `{id}` left unfilled — the \
             field declares `set: {set}` and no `default:`, so the migration placed no `{id}` in \
             any item of `{locus}` and invented none; their absence conforms"
        )
    } else {
        format!(
            "`{path}` migrated with `{locus}/{id}` left unfilled — the field declares \
             `set: {set}` and no `default:`, so the migration had no deterministic value to \
             place and invented none; its absence conforms"
        )
    };
    Finding::graded(
        Severity::Advisory,
        "migrate-corpus.set-field-unfilled",
        message,
        Some(Location::addressed(format!("{path}#{locus}/{id}"), 1, 1)),
        Some(route),
    )
}

/// The **write-address fragment** of a leaf declared inside a repeatable item block, as a *form*:
/// the section, then one `<…item-id>` hole per item hop, alternating with the nested-section id
/// that declares the next block, then the leaf.
///
/// `releases` + `date` → `releases/<item-id>/date`; `releases/changes` + `stamped` →
/// `releases/<item-id>/changes/<changes-item-id>/stamped`. The alternation is the address
/// grammar's own ([`engine::address::MAX_FRAGMENT_HOPS`]: *section / item / nested-section / item
/// / … / leaf*), and the chain is walked off [`engine::schema_diff::Locus::nested`] rather than
/// counted, so a deeper locus composes correctly the day the nesting cap rises — the locus path's
/// `Display` cannot, because it omits every item hop.
///
/// **No two holes are spelled the same.** A nested hole is named for the block it indexes
/// (`<changes-item-id>`), so a reader filling the form knows which id goes where; two bare
/// `<item-id>`s at different depths would be a form nobody can fill unambiguously.
fn item_write_fragment(locus: &engine::schema_diff::Locus, field: &str) -> String {
    let mut out = format!("{}/<item-id>", locus.section());
    for hop in locus.nested() {
        out.push_str(&format!("/{hop}/<{hop}-item-id>"));
    }
    out.push('/');
    out.push_str(field);
    out
}

/// An **above-current** stamped doc (2026-07-24, the confidence-audit sibling-hunt item 1):
/// an OOB-planted or foreign-future stamp this binary has no schema to migrate the doc *to*,
/// so it is blocked — never a silent `already-current`, which would hide it from every
/// future run (the permanent migrate-skip the hunt found). The route mirrors the detector's
/// `schema-conformance.schema-version-ahead` break and is Human-shaped: no verb fixes a
/// future stamp (`set-field` refuses the machine-maintained stamp) — upgrade jigc, or
/// restore the stamp from git history.
fn future_stamp_finding(rel_key: &str, ty: &str, stamp: u32, current: u32) -> Finding {
    blocked_finding(
        "migrate-corpus.schema-version-ahead",
        rel_key,
        format!(
            "`{rel_key}` is stamped schema-version {stamp}, above the current `{ty}` \
             schema-version {current} — this jigc build has no schema to migrate it to \
             (a newer jigc wrote it, or the stamp was edited out-of-band)"
        ),
        format!(
            "upgrade jigc to a build whose `{ty}` schema-version is at least {stamp}, or \
             restore the stamp from git history, then re-run `jigc migrate-corpus`"
        ),
    )
}

/// A below-version stamped doc whose prior-shape snapshot is **not shipped**: the migration
/// cannot source the `from` it would diff against, so the doc is blocked (never a silent
/// `already-current` — the detector routes it `migrate`). Ship the snapshot, re-run.
///
/// **The doc-keyed half of one code's two target forms** (M52 Increment 7 / T2). Since the
/// enumeration fails closed over the same store ([`enumeration_missing_snapshot_finding`]),
/// every stamp in `1..current` is proven loadable before the fold ever asks — so what still
/// reaches *this* arm is the stamp the enumeration does not cover: a doc carrying an
/// out-of-band `schema-version: 0` (a written zero, distinct from the stamp-*absent* v0 corpus
/// state the add-field branch owns), for which no `<ty>.v0.yaml` can exist. It is the per-doc
/// refusal because a doc **is** in hand there, and that is exactly why its target stays the
/// file path while the enumeration's is `<ty>@v<k>`.
fn missing_snapshot_finding(rel_key: &str, ty: &str, stamp: u32) -> Finding {
    blocked_finding(
        "migrate-corpus.missing-snapshot",
        rel_key,
        format!(
            "`{rel_key}` is stamped schema-version {stamp}, below current, but no prior-schema \
             snapshot `schema-snapshots/{ty}.v{stamp}.yaml` is shipped to source the migration \
             from"
        ),
        format!(
            "ship the prior-schema snapshot `schema-snapshots/{ty}.v{stamp}.yaml`, then re-run \
             `jigc migrate-corpus`"
        ),
    )
}

/// The **enumeration's** missing-snapshot refusal — the walk itself could not be built
/// (M52 Increment 7 / T2; `design/corpus-migration.md` → Prior-schema sourcing).
///
/// [`candidate_docs`] reads the doctype's homes out of the snapshot store, so a snapshot that
/// will not load is not one document's problem: the homes that version declared cannot be
/// enumerated at all, and **every** committed instance of that doctype leaves the run's subject.
/// Swallowed (`load_prior_schema(…).ok()`), that is silent in both directions — a doc that
/// happens to sit at the current home is reported `already current`, and one stranded at a home
/// only the missing snapshot names is reported by nothing at all — at **exit 0**, while the
/// version-aware detector goes on routing `migrate`. The rule `corpus-migration.md:90` states
/// per doc (*never a silent `already-current`*) is therefore made true here for the walk.
///
/// **The target is `<ty>@v<k>`, the one divergence in a file-path-keyed family**
/// (`design/command-output-contract.md` → the `migrate-corpus.*` sub-table, where both forms of
/// this code are declared). No document is in hand at the enumeration and none can be: the
/// subject is the **snapshot the pack owes**, which is also why the per-doc
/// [`missing_snapshot_finding`] is not reused — its target names a file, and naming an arbitrary
/// one of the docs this refusal covers would key the refusal on a bystander.
///
/// The route is the pack author's act, not an operator's: author the snapshot
/// (`implementation/doctype-authoring.md` → Freeze / versioning), then re-run.
fn enumeration_missing_snapshot_finding(ty: &str, version: u32, current: u32) -> Finding {
    blocked_finding(
        "migrate-corpus.missing-snapshot",
        &format!("{ty}@v{version}"),
        format!(
            "`{ty}` is at schema-version {current}, but no prior-schema snapshot \
             `schema-snapshots/{ty}.v{version}.yaml` is shipped — the corpus walk cannot \
             enumerate the homes schema-version {version} declared, so no `{ty}` document is \
             this run's subject at all"
        ),
        format!(
            "ship the prior-schema snapshot `schema-snapshots/{ty}.v{version}.yaml`, then re-run \
             `jigc migrate-corpus`"
        ),
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
fn unclassifiable_change_finding(rel_key: &str, ty: &str, from: u32, to: u32) -> Finding {
    blocked_finding(
        "migrate-corpus.unclassified-change",
        rel_key,
        format!(
            "`{ty}` changed its conformance-relevant structure between schema-version {from} and \
             {to}, but the schema-diff classifies no transform kind for that change, so \
             `{rel_key}` cannot be migrated (an empty diff is not a no-op: migrating would stamp \
             the doc {to} while leaving it non-conformant). This is a schema-authoring gap, not a \
             doc problem"
        ),
        "build the transform kind for the change in `crates/engine/src/schema_diff.rs` + \
         `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`"
            .to_string(),
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
fn narrowed_cardinality_finding(
    rel_key: &str,
    ty: &str,
    locus: &engine::schema_diff::Locus,
    field: &str,
    from: u32,
    to: u32,
) -> Finding {
    blocked_finding(
        "migrate-corpus.narrowed-cardinality",
        rel_key,
        format!(
            "`{ty}` narrows the cardinality of `{locus}.{field}` between schema-version {from} \
             and {to}, so `{rel_key}` cannot be migrated: a narrowing is content-affecting, not a \
             no-op (a committed instance may carry more values than the new bound admits, or lack \
             one it now demands), and no transform kind adjudicates it — migrating would stamp \
             the doc {to} while leaving it possibly non-conformant. This is a schema-authoring \
             gap, not a doc problem"
        ),
        format!(
            "restore the wider bound on `{locus}.{field}`, or build the narrowing arm (validate \
             every committed instance against the new bound) in \
             `crates/engine/src/schema_diff.rs` + `crates/engine/src/transform.rs`, then re-run \
             `jigc migrate-corpus`"
        ),
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
fn removed_field_finding(
    rel_key: &str,
    ty: &str,
    locus: &engine::schema_diff::Locus,
    field: &str,
    from: u32,
    to: u32,
) -> Finding {
    blocked_finding(
        "migrate-corpus.removed-field",
        rel_key,
        format!(
            "`{ty}` drops the declared field `{locus}.{field}` between schema-version {from} \
             and {to}, so `{rel_key}` cannot be migrated: committed instances still carry the \
             field, and the migration never strips a value (no data loss) — migrating would stamp \
             the doc {to} while it keeps a field the schema no longer declares. This is a \
             schema-authoring gap, not a doc problem"
        ),
        format!(
            "restore `{locus}.{field}` to the schema, or build the field-removal (strip) arm \
             with a deliberate data-loss opt-in in `crates/engine/src/schema_diff.rs` + \
             `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`"
        ),
    )
}

/// The **item-slot removal** refusal's route: the doctype dropped a repeatable item block's
/// **prose slot** between the two versions, so every committed item may still carry prose under
/// a sub-label the current schema no longer declares (`design/corpus-migration.md`:188 — *the
/// kind must exist rather than ride the backstop*; the pick — *refuse, not strip* — is
/// [`removed_field_finding`]'s, recorded in `DECISIONS.md` → 2026-07-13 M42 Inc-5 T5).
///
/// It is [`removed_field_finding`]'s sibling and not a reuse of it, because the two name
/// different things: a field line carries a *value*, an item slot carries the entry's **authored
/// prose**, and a route telling an author to restore a field the schema never declared is one
/// nobody can follow. The doc's bytes are left alone either way — **No-data-loss** is a declared
/// property of this pair — so the repair is a **schema-authoring** one: restore the slot, or
/// build the strip arm with a deliberate data-loss opt-in.
fn removed_item_slot_finding(
    rel_key: &str,
    ty: &str,
    locus: &engine::schema_diff::Locus,
    leaf: &str,
    from: u32,
    to: u32,
) -> Finding {
    blocked_finding(
        "migrate-corpus.removed-item-slot",
        rel_key,
        format!(
            "`{ty}` drops the declared prose slot `{locus}.{leaf}` from its repeatable item \
             block between schema-version {from} and {to}, so `{rel_key}` cannot be migrated: \
             committed items still carry their prose under that slot, and the migration never \
             strips authored prose (no data loss) — migrating would stamp the doc {to} while it \
             keeps prose the schema no longer declares. This is a schema-authoring gap, not a \
             doc problem"
        ),
        format!(
            "restore the `{locus}.{leaf}` slot to the schema, or build the item-slot removal \
             (strip) arm with a deliberate data-loss opt-in in \
             `crates/engine/src/schema_diff.rs` + `crates/engine/src/transform.rs`, then re-run \
             `jigc migrate-corpus`"
        ),
    )
}

/// The **v0 (pre-stamp) shape** of a doctype — the schema-diff's `from` for a **stamp-absent**
/// committed doc, and the fix for the M42 audit's Finding 1.
///
/// A v0 doc predates the M34 stamp injection, so its shape is *the doctype's earliest declared
/// shape, minus the stamp*. It sourced that as `strip_stamp(current)` — which silently asserts
/// **the doctype never bumped**: true while every doctype sat at v1, and false since M36, when
/// `adr` went to schema-version 2 (the `## Options` section). For a v0 ADR the diff then carried
/// **only** the stamp add-field: `AddedOptionalSection` was never classified, the empty-diff
/// backstop could not catch it (the diff is *non-empty*), the doc failed its parse-under-current
/// gate, and it was blocked with the prose-needing route — a **permanent dead end** whose route
/// lied on all three counts (no new *required* prose; the write verbs cannot author it, since the
/// doc does not parse under the current schema; re-running changes nothing), while `jigc validate`
/// went on routing the doc at the verb that refused it. `deferral-ledger` (v2, the enum rename)
/// carried the same trap.
///
/// So the prior shape comes from the doctype's **earliest shipped snapshot** — the snapshots exist
/// precisely so a prior shape is *knowable*, and this arm never consulted them. `strip_stamp` it
/// (the snapshot is a v≥1 shape, which [`crate::pack::load_prior_schema`] stamp-injects like every
/// CLI schema-load; a v0 doc carries no stamp), and the diff becomes the honest v0→current union —
/// stamp add-field **plus** every structural link of the chain — so the doc migrates in one pass
/// and lands conformant.
///
/// Where **no** snapshot is shipped — a doctype that has never bumped, whose earliest shape *is*
/// its current one — `strip_stamp(to_diff)` remains correct, and is the honest fallback. Both
/// sides stay docs-root-free by construction (`to_diff` has the prefix stripped; a snapshot stores
/// its `location:` raw), the invariant the below-version arm rests on too.
fn v0_prior_shape(pack: &dyn PackSource, dt: &DoctypeMigration, to_diff: &Schema) -> Schema {
    match earliest_snapshot(pack, dt) {
        Some(earliest) => strip_stamp(&earliest),
        None => strip_stamp(to_diff),
    }
}

/// The doctype's **earliest shipped prior-schema snapshot** (`schema-snapshots/<ty>.v<k>.yaml`,
/// smallest `k` below the current version that resolves), or `None` when it ships none — a
/// doctype that has never bumped, or one whose snapshot set starts later. The `from`-source of
/// [`v0_prior_shape`]: a v0 doc predates *every* shipped version, so the shape it must be diffed
/// against is the **earliest** one, never the immediately-prior one.
fn earliest_snapshot(pack: &dyn PackSource, dt: &DoctypeMigration) -> Option<Schema> {
    (1..dt.version).find_map(|k| crate::pack::load_prior_schema(pack, &dt.ty, k).ok())
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

/// Render one halted doc's refusal from **its own cause** — the exhaustive match over
/// [`HaltReason`] (M46 Inc-4 T3; `design/command-output-contract.md` → the `migrate-corpus.*`
/// sub-table).
///
/// Exhaustive **on purpose**, at both levels: a fourth halt cause — or a transform error the
/// driver learns to raise — cannot be added without deciding what the report says about it and
/// what repair it names. The whole defect this closes is a catch-all that answered three
/// questions with one sentence, two of them false.
///
/// The split is *who can repair it*:
///
/// - the **gate** — the fold produced conformant-shaped bytes and the doc broke its own
///   conformance check. An author clears that from the doc, so it keeps the contract-pinned
///   Framing-A waypoint and route ([`prose_needing_finding`]);
/// - the **unmodelled-content refusal** — the second cause an author clears from the doc, and
///   the reason this list is a *split by repairer* rather than "the gate versus everything
///   else": the `added-item-slot` fold found committed bytes inside an item that the parse does
///   not model, so re-rendering the item would destroy them (M49). Nothing is wrong with the
///   schema pair or with the transform arm, so `fold_refused_finding`'s schema-authoring route
///   would itself be the lie this function exists to stop; the repair is to move those bytes
///   into a declared leaf and re-run;
/// - the **`id-from` remap refusal** (M52 Increment 7 / T3) — a third non-gate cause with its
///   own words, because the two it would otherwise borrow are both false for it: the fold did
///   not fail for want of a map entry (declaring one changes nothing — the leaf carries no
///   bullet to rewrite), and the arm is not un-built (there is no arm to build: a remap of the
///   leaf an item's identity is slugged from is a re-slug, which no migration performs). It
///   keeps `fold-refused`'s schema-authoring route, pointed at the bump rather than at jigc's
///   source;
/// - **anything else** — the transform arm refused, or its output does not parse. Nothing written
///   into this doc changes that, so it takes [`fold_refused_finding`] and a **schema-authoring**
///   route: the same treatment the pre-fold refusals (`unclassified-change`,
///   `narrowed-cardinality`, `removed-field`) already get, on the same recorded reason — the
///   fold's *"author the prose, then re-run"* route "would be a lie".
///
/// **The nested-locus branch is gone, and the callers it would have caught are named (M50
/// Increment 7 / T6).** M50 Increment 6 added a branch here saying a nested-locus
/// `Unsupported` meant the kind's arm existed at the item locus and nowhere deeper, and routing
/// the reader to write the missing one — a **permanent** surface around what was then a
/// **transient** refusal (the Settle's declared bound). With T3–T5 built, no caller reaches it
/// for want of a nested arm, and for every caller that can still carry a nested locus the text
/// was false:
///
/// - the `locus_disposition == Unbuilt` pre-check in [`engine::transform::transform`] — **no
///   cell reads `Unbuilt` at HEAD**, fenced over `1..=LOCI` by
///   `crates/cli/tests/migrate_locus_axis.rs`;
/// - `ValueRemapped`'s **map gap** — taken by its own arm above, at every locus, since T5;
/// - the by-design refusals (`narrowed-cardinality`, `removed-field`, `removed-item-slot`) —
///   blocked **pre-fold** on the below-version path, but *reachable* here on the stamp-absent
///   (v0) one, whose diff is the whole v0→current chain and carries no such guard. Telling that
///   caller to build an arm would be the sharper lie: the arm is refused by design, not missing;
/// - `ProseNeeding { leaf: Some }` (the field sub-case) and the two added-field drivers'
///   belt-and-braces `unsupported()` — un-built or unresolvable at **every** locus, never at the
///   nested one specifically.
///
/// The generic arm below is true for all of them — *"an un-built arm, or one refused by
/// design"* — and `{locus}` already renders the path, so the *where* the branch was added for
/// survives its removal.
fn halt_finding(rel_key: &str, reason: &HaltReason) -> Finding {
    match reason {
        HaltReason::Gate(findings) => prose_needing_finding(rel_key, findings),
        // A parse failure always carries the diagnostics that say where the buffer broke
        // (`parse_sections`: a non-empty finding set is what makes it an `Err`), so they are
        // relayed here exactly as the gate's are.
        HaltReason::Parse(findings) => fold_refused_finding(
            rel_key,
            format!(
                "the bytes the fold produced do not parse under the new schema ({})",
                relayed(findings)
            ),
            "repair the transform arm that produced them in `crates/engine/src/transform.rs`, \
             then re-run `jigc migrate-corpus`"
                .to_string(),
        ),
        HaltReason::Transform(err) => match err {
            // The one refusal whose repair is a **migration input**, not an arm: the old→new map
            // for an enum rename is unrecoverable from the schema pair, so the CLI authors it —
            // and a committed value the map does not cover is a gap in *that table*.
            //
            // AT EVERY LOCUS (M50 Increment 7 / T5). The arm was guarded `&& !locus.is_nested()`
            // while the nested one was un-built, so a nested uncovered value fell through to the
            // un-built-arm text below. Once the arm exists that text is a **lie** (the arm is
            // built) and **unfollowable** (the operator has no jigc workspace), so the guard goes
            // with the cell it was describing. `{locus}` renders the path, which is exactly the
            // key `enrich_value_remaps` looks the map up by — the route names a real key.
            TransformError::Unsupported { kind, locus }
                if *kind == SchemaChangeKind::ValueRemapped.as_str() =>
            {
                fold_refused_finding(
                    rel_key,
                    format!(
                        "the migration renames the enum members of a field in `{locus}`, and \
                         this doc carries a committed value the authored old→new map does not \
                         cover, so no deterministic rewrite of it exists"
                    ),
                    format!(
                        "declare the missing old→new value mapping for `{locus}` in \
                         `crates/cli/src/migrate_corpus.rs` → `authored_remap`, then re-run \
                         `jigc migrate-corpus`"
                    ),
                )
            }
            // THE `id-from` ROLE (M52 Increment 7 / T3) — the one enum rename whose repair is
            // neither a map entry nor an arm, so it must not borrow either message. The leaf
            // the rename was made on is the one the item's identity is slugged from, so
            // "remap the value" means "re-mint the item's `{#id}`" — an identity change no
            // migration performs. Before this the fold wrote zero bytes and the doc fell
            // through to the conformance gate's `prose-needed`, whose route asked for prose
            // that does not exist and whose re-run reproduced the refusal byte for byte; and
            // the map-gap arm above would be a false diagnosis in the other direction, since
            // declaring the entry it names would change nothing.
            TransformError::IdFromRemap { locus, field } => fold_refused_finding(
                rel_key,
                format!(
                    "the migration renames the enum members of `{field}` in `{locus}`, and \
                     `{field}` is that block's `id-from` — each item's heading *is* its \
                     committed value and its `{{#id}}` anchor is slugged from it, so remapping \
                     it would re-mint the identity of every item this doc carries, which no \
                     migration performs"
                ),
                format!(
                    "re-author the bump so the `id-from` field `{field}` in `{locus}` keeps \
                     its declared members (rename a leaf the item's identity does not derive \
                     from, or add the new members alongside the old ones), then re-run \
                     `jigc migrate-corpus`"
                ),
            ),
            TransformError::Unsupported { kind, locus } => fold_refused_finding(
                rel_key,
                format!(
                    "the migration classifies a `{kind}` change in `{locus}` that the transform \
                     driver does not apply — an un-built arm, or one refused by design"
                ),
                format!(
                    "build (or restore the schema shape behind) the `{kind}` arm in \
                     `crates/engine/src/schema_diff.rs` + `crates/engine/src/transform.rs`, then \
                     re-run `jigc migrate-corpus`"
                ),
            ),
            // The one refusal whose repair is **in the doc** without being the gate's: the
            // committed item region carries bytes the parse does not model, so the reshape's
            // whole-item re-render would destroy them and the fold refuses instead
            // (No-data-loss). Nothing about the schema pair or the transform arm is wrong, so
            // `fold-refused`'s schema-authoring route would be a lie; the operator moves the
            // stray bytes into a declared leaf (or deletes them) and re-runs.
            TransformError::UnmodelledItemContent { section, item } => blocked_finding(
                "migrate-corpus.item-unmodelled-content",
                rel_key,
                format!(
                    "`{rel_key}` carries content in item `{section}/{item}` that its schema does not \
                     model — text outside the item's declared slots and field group, most \
                     often a paragraph hand-appended after the `<!-- fields -->` group. The \
                     migration reshapes that item by re-rendering it, which would drop those \
                     bytes, so it refuses and leaves the file untouched"
                ),
                format!(
                    "open `{rel_key}`, move the text in item `{section}/{item}` that sits outside \
                     its declared slots into one of them (or delete it), commit that, then \
                     re-run `jigc migrate-corpus`"
                ),
            ),
            TransformError::Unclassified => fold_refused_finding(
                rel_key,
                "the schema pair moved its conformance-relevant structure but classifies no \
                 transform kind, so there are no bytes to fold"
                    .to_string(),
                "build the transform kind for the change in `crates/engine/src/schema_diff.rs` + \
                 `crates/engine/src/transform.rs`, then re-run `jigc migrate-corpus`"
                    .to_string(),
            ),
            TransformError::Generate(_) | TransformError::Splice(_) => fold_refused_finding(
                rel_key,
                "a structural splice primitive failed while folding it, so no migrated bytes were \
                 produced"
                    .to_string(),
                "repair the transform arm in `crates/engine/src/transform.rs`, then re-run \
                 `jigc migrate-corpus`"
                    .to_string(),
            ),
        },
    }
}

/// The engine findings' **own words**, joined — a relay, never a re-wording. Two surfaces
/// describing one break in two vocabularies is the class this wave exists to close, and the
/// migration report has no standing to re-diagnose what the gate (or the parser) already
/// diagnosed.
///
/// **The words include the *where* (M49 Increment 8 / T4.)** The relayed diagnostics are the
/// ones that "say where the buffer broke" ([`halt_finding`]) — and the relay dropped exactly
/// that half, so an operator read *what* was wrong with a doc whose refusal is keyed at its
/// **path** and had to find the byte themselves. Each relayed message now carries its own
/// locus ([`crate::render::finding_locus`]) when it has one; a location-less diagnostic
/// relays exactly as before.
fn relayed(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|finding| match crate::render::finding_locus(finding) {
            Some(locus) => format!("{} (at {locus})", finding.message),
            None => finding.message.clone(),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// The Framing-A route for a doc the conformance gate refused: author the prose, then re-run —
/// the stamp flips only once the doc gates clean.
fn prose_needing_route(rel_key: &str) -> String {
    format!(
        "author the new required prose in `{rel_key}` through the write verbs, then re-run \
         `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)"
    )
}

/// A doc whose folded bytes **broke the conformance gate** — the Framing-A prose handoff: the
/// structural splice landed in the scratch buffer but the doc cannot gate clean until an agent
/// authors the prose, so it rolls back byte-identical and is routed to the author.
///
/// **The message relays the gate's findings** (M46 Inc-4 T3). It used to assert that the
/// migration *"mints a new **required** prose slot"* — true for one of the states that reach here
/// and false for the other, since the gate breaks just as readily on a slot the doc left empty
/// long before this run, which no migration minted. The gate already says which slot and why; the
/// report has that set in hand and repeats it rather than narrating over it.
///
/// **The route is unchanged**, and deliberately: it is the one halt cause an author can clear
/// **from the doc**, which is exactly what the route directs them to do.
fn prose_needing_finding(rel_key: &str, findings: &[Finding]) -> Finding {
    blocked_finding(
        "migrate-corpus.prose-needed",
        rel_key,
        format!(
            "`{rel_key}` does not gate clean under its new schema, so its bytes are rolled back \
             untouched — the conformance gate reports: {}",
            relayed(findings)
        ),
        prose_needing_route(rel_key),
    )
}

/// A doc the fold refused for a reason **the gate never saw** — the transform driver would not
/// apply the change, or the bytes it produced do not parse (M46 Inc-4 T3).
///
/// Its route is a **schema-authoring** repair, never a doc instruction, for the reason the
/// pre-fold refusals (`unclassified-change`, `narrowed-cardinality`, `removed-field`) are already
/// refused before the fold can reach them: there is nothing an operator or agent can do *to this
/// doc*. Routed at the fold's own route it was a **permanent dead end** — author prose that
/// changes nothing, re-run into the identical refusal — which is the class M46 Increment 4 closes
/// on the transform side and this closes on the report's.
fn fold_refused_finding(rel_key: &str, cause: String, route: String) -> Finding {
    blocked_finding(
        "migrate-corpus.fold-refused",
        rel_key,
        format!(
            "`{rel_key}` cannot be migrated: {cause}. This is a schema-authoring gap, not a doc \
             problem — no prose authored into `{rel_key}` and no re-run changes it"
        ),
        route,
    )
}

/// The route for the **both-homes destination collision** (M42 — the walk union): a
/// prior-home instance whose relocation destination already holds a *different* document. The
/// migration refuses to overwrite it — **No-data-loss** is a declared property of the corpus
/// migration, and no deterministic merge of two documents exists — so the doc is blocked and
/// the operator reconciles the two homes by hand (`design/corpus-migration.md` → the union).
fn destination_collision_finding(rel_key: &str, target: &str) -> Finding {
    blocked_finding(
        "migrate-corpus.destination-collision",
        rel_key,
        format!(
            "`{rel_key}` relocates to `{target}`, which already holds a *different* document; the \
             migration never overwrites it (no data loss), and no deterministic merge of two \
             documents exists"
        ),
        format!(
            "fold the content of `{rel_key}` into `{target}` through the write verbs, delete \
             `{rel_key}`, then re-run `jigc migrate-corpus`"
        ),
    )
}

/// A doc left untouched **behind the run's first blocker** (WIP-safety: the fold halts at the
/// first blocked doc, never half-transforming the rest). Not a defect of *this* doc — it simply
/// has not been reached — but it did not migrate, so it is reported and it holds the exit.
fn deferred_finding(rel_key: &str) -> Finding {
    blocked_finding(
        "migrate-corpus.deferred",
        rel_key,
        format!(
            "`{rel_key}` was not reached — the fold halts at the run's first blocked doc rather \
             than half-transforming the rest (WIP-safety), so this doc is untouched"
        ),
        format!(
            "resolve the blocker reported above, then re-run `jigc migrate-corpus` — `{rel_key}` \
             migrates once the fold can reach it"
        ),
    )
}

/// Which corpus a walk enumerates — the **worktree** (the bytes the migration reads, folds and
/// writes back) or **`HEAD`** (the bytes the repo actually carries for every other clone). The
/// two are the same on a clean tree and diverge exactly when a migration was written but never
/// landed, which is the state [`unlanded_paths`] exists to see.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Corpus {
    /// The files on disk under `repo_root`.
    Worktree,
    /// The blobs committed at `HEAD`.
    Head,
}

/// Every home doctype `ty` declared **below** `version`, read out of the versioned snapshot
/// store and **re-rooted through the cascade knobs** — the one derivation of *where this
/// doctype's documents used to live*, and the reason two doors can no longer hold two opinions
/// about one committed file (M52 completion audit, fix 2).
///
/// Both home kinds, every version below the current one, de-duplicated and ordered: a doctype
/// that relocated at v2 and bumped again at v3 declared **two** prior homes, and consulting
/// only `version - 1`'s re-opens the hole one version along (the M42 completion audit's
/// Finding 3, re-derived here rather than re-learned).
///
/// **Why the re-rooting happens here and nowhere else.** A snapshot stores its `location:` and
/// its `placement.file` as the raw **declaration**; a current shape arrives at its consumers
/// already resolved (`CascadeDefs::all_schemas`' closing `apply_docs_root` +
/// `apply_placement_root`). So each prior home is put through the same two knobs — the
/// `docs_root` prefix and [`crate::start::reroot_placement_file`] — and this is the sole
/// re-application site. It is applied to the declaration and never to an already-resolved
/// home, which is that primitive's documented trap: under a root a resolved home has no
/// leading component left, so re-rooting *it* is a silent no-op.
///
/// **The error posture is the caller's, deliberately**, which is why a snapshot that will not
/// load is *reported* rather than raised. [`candidate_docs`] — a **walk** whose subject is the
/// documents it will rewrite — fails closed on it, because homes it cannot enumerate make its
/// subject silently narrower than the doctype's history. The read-only store sweep
/// ([`crate::orphan::prior_home_instances`]) reads the same store **best-effort**, the posture
/// [`crate::pack::prior_doctype_schemas`] already takes and for the reason stated there: a
/// missing snapshot is a *pack's* omission, and reddening an adopter's stock repository over
/// it is not the sweep's to do.
pub(crate) struct PriorHomes {
    /// Prior `location:` homes, re-rooted under `docs-root`, each keeping its declared
    /// trailing slash (`format!("{location}{slug}.md")` is how every consumer joins them).
    pub(crate) locations: std::collections::BTreeSet<String>,
    /// Prior `placement:` homes, re-rooted under `placement-root` — exact repo-relative files.
    pub(crate) files: std::collections::BTreeSet<String>,
    /// The versions below `version` whose snapshot would not load, ascending. Empty is the
    /// ordinary case; a caller that cannot tolerate a narrower union refuses on `first()`.
    pub(crate) unloadable: Vec<u32>,
}

/// See [`PriorHomes`].
pub(crate) fn prior_homes(
    pack: &dyn PackSource,
    ty: &str,
    version: u32,
    docs_root: &str,
    placement_root: Option<&str>,
) -> PriorHomes {
    let mut out = PriorHomes {
        locations: Default::default(),
        files: Default::default(),
        unloadable: Vec::new(),
    };
    for k in 1..version {
        let Ok(prior) = crate::pack::load_prior_schema(pack, ty, k) else {
            out.unloadable.push(k);
            continue;
        };
        if let Some(raw_home) = prior.location {
            out.locations.insert(if docs_root.is_empty() {
                raw_home
            } else {
                format!("{docs_root}/{raw_home}")
            });
        }
        if let Some(placement) = prior.placement {
            out.files.insert(crate::start::reroot_placement_file(
                &placement.file,
                placement_root,
            ));
        }
    }
    out
}

/// Enumerate one doctype migration job's committed candidate docs as
/// `(source, destination)` pairs — the **from** home the corpus walk found each committed
/// instance at, and the path its gated bytes land at (`design/corpus-migration.md` →
/// Relocation: the walk keys on the from home). Empty when nothing is committed to walk.
///
/// `corpus` selects **which** corpus is walked: [`Corpus::Worktree`] for the migration itself,
/// [`Corpus::Head`] for the landed-state audit in [`unlanded_paths`]. The home resolution is
/// identical for both; only the enumeration primitive differs.
///
/// # The walk keys on `from`, over **every prior home of every kind**
///
/// A doctype's home moves in one of four ways — `location:`→`location:`,
/// `location:`→`placement:`, `placement:`→`placement:`, `placement:`→`location:` — and the
/// snapshot store records the whole prior [`Schema`] either way, so all four are derivable.
/// The union is therefore taken over **both** home kinds of **every** snapshot below the
/// current version, plus the current home itself (the in-place half):
///
/// - a prior **`location:`** home contributes every committed `.md` directly under it, each
///   destined for the current home keyed on its own slug — the file **stem**, which is the
///   identity that has to survive the move ([`engine::index::instance_slug`]);
/// - a prior **`placement:`** home contributes that one file, if committed. Its identity is
///   the doctype's **type id** (`instance_slug`'s placement rule — the one address a
///   fixed-identity doctype has), so landing it in a `location:` directory names it
///   `<location><ty>.md` rather than keeping the old file's stem;
/// - the **current** home is walked in place (`destination == source`), which is what makes a
///   partially-completed migration re-runnable: the instances are findable *wherever* it left
///   them.
///
/// A destination two candidates share (both homes populated) is adjudicated at the write
/// boundary — see [`destination_collision_finding`] — never by candidate order.
///
/// # A snapshot that will not load refuses the walk (M52 Increment 7 / T2)
///
/// Because the homes are *read out of the store*, a snapshot the store cannot answer for is not
/// one document's problem: the homes that version declared are unknowable, so the union this
/// function returns would be **silently narrower than the doctype's history**. Skipping it
/// (`load_prior_schema(…).ok()`) therefore removed every instance of the doctype from the run's
/// subject at exit 0. It is an `Err` instead — one blocking
/// [`enumeration_missing_snapshot_finding`] keyed at `<ty>@v<k>` — and **both** call sites fail
/// closed on it, the prepared walk and [`unlanded_paths`], so a rejected-commit recovery cannot
/// narrow either. The read-only store sweep's prior-shape load stays best-effort by contrast and
/// deliberately ([`crate::pack::prior_doctype_schemas`]).
///
/// **Why the prior homes need re-rooting and the current one does not.** A snapshot stores its
/// `location:` and its `placement.file` as the raw **declaration**; the current shape arrives
/// here already resolved through the cascade (`CascadeDefs::all_schemas`' closing
/// `apply_docs_root` + `apply_placement_root`). So each prior home is put through the same two
/// knobs — and that derivation now lives in [`prior_homes`], which is the sole re-application
/// site and the **one** answer to *where did this doctype's documents used to live*. It was
/// inlined here, and the store-scope sweep had no answer at all, which is exactly how the two
/// doors came to hold two opinions about one committed file (M52 completion audit, fix 2).
///
/// # What each half cost before it existed
///
/// **The both-homes union** (M42): keyed on whether the prior snapshot carried a `location:`,
/// the two placement branches were mutually exclusive — and for `changelog` the prior snapshot
/// always does, so the in-place half was *dead code for that doctype* and a stale root
/// `CHANGELOG.md` was **invisible** to the verb (`0 migrated, 0 already current, 0 blocked`;
/// the stamp stayed at 1 forever, and the detector's `migrate` route pointed at a verb that
/// did nothing).
///
/// **Every prior home, not just `version - 1`'s** (M42 completion audit, Finding 3): consulting
/// only the immediately-prior snapshot re-opens that hole one version along — a doctype that
/// relocated at v2 and has since bumped to v3 loads only the v2 snapshot, already the placement
/// shape, so its **v1** home goes unwalked.
///
/// **Every prior home of every *kind*** (M52 Increment 7 / T1): three of the four home-pair
/// cells were invisible for the same reason one version at a time was. The `location:` branch
/// returned early having loaded **no snapshot at all**, so a `location:`→`location:` move (and
/// a `placement:`→`location:` one) saw nothing but the new home; and the placement branch's
/// `.filter_map(|prior| prior.location)` dropped every prior `placement:` home by construction,
/// so `placement:`→`placement:` saw nothing but the new file. Driven at `1932c00f`
/// (`completions/artifacts/M52/baseline-freeze.md` §2.1), each stranded doc reported
/// `0 migrated, 0 already current, 0 blocked` at exit 0 — and where the stranded home was the
/// repo root, **no surface named the document at all**: `validate` said *"validates clean"*.
/// The bump kind is not the discriminator (W2 and W5 carry a content change and behave
/// identically to W1 and W4); the home pair is.
///
/// # What an adopter sees change, and where they are told (M52 Increment 7 / T4)
///
/// Widening the walk is a **tightening** at the store surface, and a deliberate one
/// (settle-record D4.3). A below-version instance at a prior home was previously invisible to
/// this function, so on a repo-root home no surface named it at all — `jigc validate` said
/// *"validates clean"* at exit 0. Seen, it is adjudicated like any other managed doc: the
/// existing `schema-conformance.schema-version-current` fires and, as a `STORE_EXIT_FLIPS`
/// member, flips `validate` **0 → 1** on a corpus that graded clean before the upgrade. Nothing
/// about the doc changed; what changed is that the sweep can now see it — and the verb the
/// finding routes to can now land it, which is the half that makes the tightening followable
/// rather than a new dead end (`crates/cli/tests/migrate_route_family.rs` drives that route
/// end to end).
///
/// The adopter-facing half of this — the flip-list line in
/// [MIGRATING.md](../../../MIGRATING.md) — is **owed to M52 Increment 10**, the wave's guide
/// batch, which is where the roadmap places every adopter-guide edit; it is named here so the
/// punt is tracked to a landing rather than left as prose.
fn candidate_docs(
    pack: &dyn PackSource,
    repo_root: &Path,
    dt: &DoctypeMigration,
    corpus: Corpus,
) -> Result<Vec<(String, String)>, Finding> {
    // EVERY prior snapshot below the current version, of BOTH home kinds. De-duplicated
    // (successive versions usually re-declare one home) and ordered, so the walk is
    // deterministic. A snapshot that fails to load refuses the whole walk (above): the homes
    // it declared cannot be enumerated, so a narrower union would be a silent lie about scope.
    let homes = prior_homes(
        pack,
        &dt.ty,
        dt.version,
        &dt.docs_root,
        dt.placement_root.as_deref(),
    );
    if let Some(&k) = homes.unloadable.first() {
        return Err(enumeration_missing_snapshot_finding(&dt.ty, k, dt.version));
    }
    let (prior_locations, prior_files) = (homes.locations, homes.files);

    let mut out: Vec<(String, String)> = Vec::new();
    if let Some(location) = &dt.to.location {
        // Every prior folder home **and** the current one, each instance keyed on its own
        // stem: `docs/decisions/x.md` → `docs/adrs/x.md`, and `docs/adrs/x.md` → itself.
        for from_home in prior_locations.iter().chain(std::iter::once(location)) {
            out.extend(
                committed_slugs(repo_root, from_home, corpus)
                    .into_iter()
                    .map(|slug| {
                        (
                            format!("{from_home}{slug}.md"),
                            format!("{location}{slug}.md"),
                        )
                    }),
            );
        }
        // A prior single-file home: its identity is the type id, so it lands under the stem
        // that keeps the identity rather than under the old file's name.
        for file in &prior_files {
            if exists_in(repo_root, file, corpus) {
                out.push((file.clone(), format!("{location}{}.md", dt.ty)));
            }
        }
    } else if let Some(placement) = &dt.to.placement {
        for from_home in &prior_locations {
            out.extend(
                committed_slugs(repo_root, from_home, corpus)
                    .into_iter()
                    .map(|slug| (format!("{from_home}{slug}.md"), placement.file.clone())),
            );
        }
        // Every prior single-file home **and** the current one (in place).
        for file in prior_files.iter().chain(std::iter::once(&placement.file)) {
            if exists_in(repo_root, file, corpus) {
                out.push((file.clone(), placement.file.clone()));
            }
        }
    } else {
        return Ok(Vec::new()); // a transient doctype has no committed home to walk.
    }
    // Dedupe: a home re-declared across versions, or a placement file that itself sits under a
    // prior folder home (a `docs/x.md` placement whose prior home was `docs/`), enumerates the
    // identical pair twice.
    out.sort();
    out.dedup();
    Ok(out)
}

/// The committed-doc slugs of a persisted type — the `.md` file stems directly under
/// `<location>` in `corpus`, slug-sorted. A missing / unreadable location yields none.
fn committed_slugs(repo_root: &Path, location: &str, corpus: Corpus) -> Vec<String> {
    let mut slugs: Vec<String> = match corpus {
        Corpus::Worktree => {
            let Ok(entries) = std::fs::read_dir(repo_root.join(location)) else {
                return Vec::new();
            };
            entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
                .collect()
        }
        // `-z` so git emits raw, unquoted names (its default C-style quoting of unusual bytes
        // would corrupt a slug); `-r` then a direct-child filter, mirroring the non-recursive
        // `read_dir` above exactly.
        Corpus::Head => {
            let Some(out) = git_stdout_raw(
                repo_root,
                &["ls-tree", "-r", "--name-only", "-z", "HEAD", "--", location],
            ) else {
                return Vec::new();
            };
            out.split('\0')
                .filter_map(|path| path.strip_prefix(location))
                .filter(|rest| !rest.contains('/'))
                .filter_map(|name| name.strip_suffix(".md"))
                .map(str::to_string)
                .collect()
        }
    };
    slugs.sort();
    slugs
}

/// Whether `path` exists as a file in `corpus`.
fn exists_in(repo_root: &Path, path: &str, corpus: Corpus) -> bool {
    match corpus {
        Corpus::Worktree => repo_root.join(path).is_file(),
        Corpus::Head => git_ok(repo_root, &["cat-file", "-e", &format!("HEAD:{path}")]),
    }
}

/// A doc's bytes in `corpus`, lossily decoded — `None` when it is not there.
fn doc_source(repo_root: &Path, path: &str, corpus: Corpus) -> Option<String> {
    match corpus {
        Corpus::Worktree => std::fs::read(repo_root.join(path))
            .ok()
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned()),
        Corpus::Head => git_stdout_raw(repo_root, &["show", &format!("HEAD:{path}")]),
    }
}

/// **The unlanded pathspec** — the paths whose migrated form is on disk but **not in `HEAD`**
/// (M47 Inc-3, N2). [`commit_migration`] stages before it commits, so a rejected commit (a
/// `pre-commit` hook, a hook the operator then repairs) leaves the migration *written and
/// staged* but unlanded — and the re-run, which judges currency from the **worktree**, read the
/// stamp it had itself just written, reported the doc `already-current` and, with `touched`
/// empty, committed nothing at **exit 0**. The migration was stranded in that one clone: every
/// other clone still carried the unmigrated corpus and still routed `migrate-corpus`.
///
/// The judgment is therefore keyed on the **committed** corpus. Walking `HEAD` (not disk), a
/// doc is *unlanded* when `HEAD` is behind — below the doctype's current version, or still
/// sitting at a prior home — **and** the worktree already holds the migrated result at the
/// current stamp (for a relocation, with the source removed, so the move's remove-half is
/// complete). Those paths rejoin `report.touched`, so the re-run re-stages the **identical**
/// pathspec the refused run built — both halves of a relocation move included, never the
/// half-migration the commit boundary exists to prevent — and lands it.
///
/// It cannot fire on a doc this run migrates (that doc's worktree bytes are still the *old*
/// ones when this is computed) nor on ambient dirt (only the doctypes' own homes are walked),
/// and a doc landed at the current version in `HEAD` is skipped outright.
fn unlanded_paths(
    pack: &dyn PackSource,
    repo_root: &Path,
    dt: &DoctypeMigration,
) -> Result<Vec<String>, Finding> {
    let mut out = Vec::new();
    for (source, target) in candidate_docs(pack, repo_root, dt, Corpus::Head)? {
        // Landed: `HEAD` carries this doc at its final home, already at the current version.
        if source == target
            && doc_source(repo_root, &source, Corpus::Head)
                .and_then(|committed| read_stamp_from_source(&committed))
                == Some(dt.version)
        {
            continue;
        }
        // `HEAD` is behind. Only a worktree that already holds the migrated result is an
        // unlanded migration; a worktree still holding the old bytes is this run's own work.
        if doc_source(repo_root, &target, Corpus::Worktree)
            .and_then(|worktree| read_stamp_from_source(&worktree))
            != Some(dt.version)
        {
            continue;
        }
        if source != target {
            // A relocation whose source is still on disk has not been moved yet — the migration
            // path owns it. Only the completed-but-unlanded move contributes its remove half.
            if repo_root.join(&source).exists() {
                continue;
            }
            out.push(source);
        }
        out.push(target);
    }
    Ok(out)
}

/// Locate the repo root and its `.jigc/config/` project layer — the store-walk locate
/// preamble shared with `jigc validate` / `jigc ingest` (and the freeze-exempt relocation
/// path in `crate::relocate`). Errors with routed messages when the repo or the project
/// layer is absent.
///
/// Since M52 Increment 8 / T1 it is also the **whole `jigc milestone` family's** door-top
/// ask ([`crate::milestone::MilestoneCommand::dispatch`]), where the caller wants only
/// the refusal: a milestone minted into a repository with no `jigc setup` lives in a
/// gitignored `.jigc/` no clone sees, inverting `design/team-ready-state.md`'s
/// committed-record-is-the-source-of-truth settlement.
pub(crate) fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        return Err(crate::locate::not_set_up(&ctx.project_config_path()));
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
    //!
    //! Plus **the envelope's own key-set fence** (M51 Increment 7 / T2), which lives here rather
    //! than in a `tests/` suite because closing the key set needs *every* field of
    //! [`CorpusMigrationReport`] — and three of the eleven are `#[serde(skip)]` and not `pub`, so
    //! a suite outside the crate can read what the envelope carries but not state what is
    //! deliberately missing from it.

    use super::*;
    use engine::field_block::{Field, Value};
    use engine::parse::parse_sections;
    use engine::schema::{inject_schema_version_stamp, load_schema};
    use engine::validate::schema_conformance;
    use engine::write::{Instance, ItemContent, SectionContent, render};
    use std::collections::BTreeSet;
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
                engine::tempname::unique_nanos(),
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

    // ── The envelope's own key set (M51 Increment 7 / T2) ────────────────────────────────

    /// **What one field of [`CorpusMigrationReport`] is on the wire** — the
    /// `crates/cli/tests/text_json_parity_axis.rs` `Disposition` mold, narrowed from a verb to a
    /// struct's fields. A **field name is not a wire key**: serde may rename one, and three of
    /// the eleven fields are `#[serde(skip)]`, so each field states which it is and — where it is
    /// neither — why (each field's own doc-comment carries the long form).
    #[derive(Debug)]
    enum FieldDisposition {
        /// The field reaches the pinned envelope under this key, carrying this value
        /// (`design/command-output-contract.md` → the `jigc migrate-corpus` row, which declares
        /// every one of them).
        Wire {
            key: &'static str,
            value: serde_json::Value,
        },
        /// `#[serde(skip)]` — the field serves a renderer or the commit boundary and never
        /// reaches a driver, with the recorded reason.
        NotWire(&'static str),
    }

    /// A wire field's disposition — **built from the field itself**, so the row cannot speak for
    /// a neighbour it was copy-pasted from and the emitted value is compared, not just the key.
    fn wire<T: serde::Serialize>(key: &'static str, field: T) -> FieldDisposition {
        FieldDisposition::Wire {
            key,
            value: serde_json::to_value(field).expect("a report field serializes"),
        }
    }

    /// A skipped field's disposition. The value is taken and dropped: what is proved here is
    /// that the row **owns** that field, which is what makes the destructure below exhaustive in
    /// both directions.
    fn not_wire<T>(reason: &'static str, _field: T) -> FieldDisposition {
        FieldDisposition::NotWire(reason)
    }

    /// **Every field of [`CorpusMigrationReport`], classified — one stated disposition per
    /// field.**
    ///
    /// The fence is the **exhaustive destructure**: a field added to the struct does not compile
    /// until it is bound here, and a row deleted from the table leaves its binding unused —
    /// *denied*, not warned — so neither half can drift from the other (the compiler's own
    /// `field: _` suggestion silences it, which is exactly the deliberate *"this field has no
    /// disposition"* someone would then have to write down). The key-set equality in
    /// [`the_report_key_set_is_closed_by_its_stated_field_dispositions`] then closes the loop
    /// against the bytes serde actually emits, which is the half a destructure alone cannot see.
    ///
    /// It lives in-crate rather than in a `tests/` suite because the closure needs **every**
    /// field, and three of them are `#[serde(skip)]` and not `pub` — a `tests/` suite can read
    /// the envelope but cannot state what is missing from it.
    #[deny(unused_variables)]
    fn report_field_dispositions(
        report: CorpusMigrationReport,
    ) -> Vec<(&'static str, FieldDisposition)> {
        let CorpusMigrationReport {
            migrated,
            already_current,
            blocked,
            unadopted,
            unfilled,
            commit,
            hook_output,
            touched,
            unlanded,
            dry_run,
            no_commit,
        } = report;
        vec![
            ("migrated", wire("migrated", migrated)),
            ("already_current", wire("already_current", already_current)),
            // `blocked`'s emptiness — and **only** `blocked`'s — is the verb's exit rule
            // ([`run`]); the other two `Findings` fields are reported, never gating.
            ("blocked", wire("blocked", blocked)),
            ("unadopted", wire("unadopted", unadopted)),
            ("unfilled", wire("unfilled", unfilled)),
            ("commit", wire("commit", commit)),
            ("hook_output", wire("hook_output", hook_output)),
            (
                "touched",
                not_wire(
                    "the self-commit's pathspec — internal to the commit boundary, and not a \
                     fact a driver reads back",
                    touched,
                ),
            ),
            (
                "unlanded",
                not_wire(
                    "text-only: the envelope already discriminates the recovery \
                     (`dry_run: false` + `commit: <sha>` + `migrated: []`), so the set is \
                     re-derivable rather than withheld",
                    unlanded,
                ),
            ),
            ("dry_run", wire("dry_run", dry_run)),
            (
                "no_commit",
                not_wire(
                    "text-only: the JSON already distinguishes it by `dry_run: false` + \
                     `commit: null`",
                    no_commit,
                ),
            ),
        ]
    }

    /// A report with **every** field populated and distinct — nothing here passes by accident of
    /// an empty collection or a `None`.
    fn populated_report() -> CorpusMigrationReport {
        let finding = |code: &str, path: &str| {
            blocked_finding(
                code,
                path,
                format!("{code} at {path}"),
                "a human route".to_string(),
            )
        };
        CorpusMigrationReport {
            migrated: vec!["docs/migrated.md".to_string()],
            already_current: vec!["docs/current.md".to_string()],
            blocked: Findings::from(vec![finding("migrate-corpus.blocked", "docs/blocked.md")]),
            unadopted: Findings::from(vec![finding(
                "schema-conformance.unadopted-instance",
                "docs/unadopted.md",
            )]),
            unfilled: Findings::from(vec![finding(
                "migrate-corpus.unfilled-set-field",
                "docs/unfilled.md",
            )]),
            commit: Some("0ff1ce5".to_string()),
            hook_output: "a hook spoke".to_string(),
            touched: vec!["docs/touched.md".to_string()],
            unlanded: vec!["docs/unlanded.md".to_string()],
            dry_run: true,
            no_commit: true,
        }
    }

    /// The wire keys, in the order [`report_field_dispositions`] states them.
    fn declared_wire_keys() -> Vec<(&'static str, serde_json::Value)> {
        report_field_dispositions(populated_report())
            .into_iter()
            .filter_map(|(_, disposition)| match disposition {
                FieldDisposition::Wire { key, value } => Some((key, value)),
                FieldDisposition::NotWire(_) => None,
            })
            .collect()
    }

    /// **The emitted key set equals the stated dispositions' wire keys — nothing more, nothing
    /// less** (M51 Increment 7 / T2).
    ///
    /// The prose homes for this envelope were hand-written lists, and a hand-written list of keys
    /// is exactly the mechanism D8 records failing: the roadmap that chartered this task said
    /// *three* triage keys where the binary emits five. The destructure closes the field side and
    /// this closes the wire side, so a `#[serde(rename)]`, a new key, or a newly-skipped field
    /// reddens here instead of being discovered in a driver.
    #[test]
    fn the_report_key_set_is_closed_by_its_stated_field_dispositions() {
        let dispositions = report_field_dispositions(populated_report());
        // The **emitted bytes**, through the renderer the verb prints from ([`run`]) — not a
        // re-serialization of the same value in test code, which is the shape that can pass
        // while what a driver actually receives is something else.
        let emitted: serde_json::Value =
            serde_json::from_str(&render::corpus_migration(Format::Json, &populated_report()))
                .expect("`--format json` emits JSON");
        let emitted = emitted
            .as_object()
            .expect("the report is emitted as a JSON object");

        for (field, disposition) in &dispositions {
            if let FieldDisposition::NotWire(reason) = disposition {
                assert!(
                    !emitted.contains_key(*field),
                    "`{field}` is declared off the wire ({reason}) but the envelope carries it",
                );
            }
        }

        let declared = declared_wire_keys();
        assert_eq!(
            emitted.keys().map(String::as_str).collect::<BTreeSet<_>>(),
            declared
                .iter()
                .map(|(key, _)| *key)
                .collect::<BTreeSet<_>>(),
            "the emitted key set and the report's stated field dispositions disagree — \
             whichever moved, both this table and `design/command-output-contract.md`'s \
             `jigc migrate-corpus` row must say so",
        );
        for (key, value) in &declared {
            assert_eq!(
                emitted.get(*key),
                Some(value),
                "`{key}` is declared for one field and carries another field's value",
            );
        }
    }

    /// **The contract's `jigc migrate-corpus` row declares exactly those keys** — the doc leg of
    /// the same fence, on the `doctype_map_versions` mold (read the code-side set, assert the
    /// doc's rows). A key that reaches the wire without a declaration is a key a driver has no
    /// path to, which is the whole content of that doc's closing rule.
    #[test]
    fn the_contract_row_declares_exactly_the_reports_wire_keys() {
        let doc =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../design/command-output-contract.md");
        let text = fs::read_to_string(&doc).expect("the command-output contract is readable");
        let row = text
            .lines()
            .find(|line| line.starts_with("**`jigc migrate-corpus`"))
            .expect("the contract carries a `jigc migrate-corpus` row");

        // The row spells each declared key — and only a declared key — as a `**`token`**`. The
        // row's own head is `**`jigc migrate-corpus` — the corpus-upgrade report.**`, whose code
        // span closes before the bold does, so it is not one of these tokens; every other verb
        // named in the row is prose-backticked, not bolded.
        let declared: BTreeSet<&str> = row
            .split("**`")
            .skip(1)
            .filter_map(|rest| rest.split_once("`**").map(|(token, _)| token))
            .collect();
        assert!(
            !declared.is_empty(),
            "the `jigc migrate-corpus` row declares no key at all — its shape moved",
        );
        assert_eq!(
            declared,
            declared_wire_keys()
                .iter()
                .map(|(key, _)| *key)
                .collect::<BTreeSet<_>>(),
            "the contract row and the report's wire keys disagree",
        );
    }
    /// **The authored remap table is keyed by the locus path, not the bare section id.**
    /// (M50 Increment 7 / T5.)
    ///
    /// A section's own item block and a nested one inside it are two different blocks that can
    /// declare the same leaf id — `changelog.releases` declares `date` outside its nested
    /// `changes` block and, in the reshapes M50 ships, inside it too. Keyed by
    /// `locus.section()` the lookup asked only *which section*, so a map authored for one
    /// block would silently be handed to the other's rename; and the refusal's route, which
    /// interpolates `{locus}`, named a key the table was not keyed by.
    ///
    /// The one shipped entry is the probe, and no entry is added to test it: it must fill the
    /// **locus-2** `entries` change and leave the **locus-3** `entries/notes` one empty — an
    /// empty map being exactly what makes the driver block that doc loudly.
    #[test]
    fn the_authored_remap_is_keyed_by_the_locus_path_not_the_section() {
        let at_item = engine::schema_diff::Locus::at_item_block("entries");
        let nested = at_item.nested_in("notes");
        let change = |locus: engine::schema_diff::Locus| SchemaChange::ValueRemapped {
            locus,
            field: "kind".to_string(),
            map: BTreeMap::new(),
        };

        assert_eq!(
            enrich_value_remaps(vec![change(at_item.clone())], "deferral-ledger"),
            vec![SchemaChange::ValueRemapped {
                locus: at_item,
                field: "kind".to_string(),
                map: BTreeMap::from([
                    ("D".to_string(), "Decision".to_string()),
                    ("I".to_string(), "Idea".to_string()),
                ]),
            }],
            "the shipped entry fills the block it was authored for",
        );
        assert_eq!(
            enrich_value_remaps(vec![change(nested.clone())], "deferral-ledger"),
            vec![change(nested)],
            "a nested block's rename is a different key: it keeps the empty map, so the \
             driver blocks the doc rather than being handed the outer block's semantics",
        );
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
            placement_root: None,
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

    /// A blocked doc's **stable target** — its filesystem path, read off the refusal finding's
    /// address (M42 completion audit, Finding 2: the report's blocked entries were untyped
    /// `(path, route)` tuples; they are real [`Finding`]s now, and this is the old `.0`).
    fn blocked_path(finding: &Finding) -> &str {
        finding
            .location
            .as_ref()
            .and_then(|l| l.address.as_deref())
            .expect("a refusal finding carries its path target")
    }

    /// A blocked doc's **route** — the repair half of its refusal finding (the old `.1`, minus
    /// the diagnosis, which now lives in `message`).
    fn blocked_route(finding: &Finding) -> &str {
        finding
            .route
            .as_deref()
            .expect("a refusal finding carries a route")
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

    /// A second `note`-shaped doctype homed at a location that **sorts after** `notes/`
    /// (`zzz/` > `notes/`), so in the path-sorted write pass its instance is migrated
    /// *after* a `notes/` instance — the ordering
    /// [`written_docs_are_baselined_per_doc_even_when_a_later_write_aborts`] relies on to
    /// abort the run at the *later* doc's write.
    fn note2_v0_yaml() -> &'static [u8] {
        b"\
type: note2
location: zzz/
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
    /// Two tracked v0 docs at distinct homes: `a` (`docs/notes/a.md`) sorts before `b`
    /// (`docs/zzz/b.md`), so `a` persists first; `b`'s atomic write is forced to fail (its home
    /// `docs/zzz/` is made read-only, so [`engine::state::persist`]'s temp write errors EACCES),
    /// aborting the loop. The read-only-home trigger is temp-name-independent — the atomic temp
    /// sibling is process-unique (M45 Inc 7), so pre-occupying a fixed `<name>.tmp` no longer
    /// works. After the abort `a` is on disk migrated, and its on-disk baseline must already
    /// equal the hash of those bytes.
    #[test]
    fn written_docs_are_baselined_per_doc_even_when_a_later_write_aborts() {
        let repo = TempDir::new("partial-abort");
        let jigc_root = repo.path().join(".jigc");

        // Two doctypes at distinct homes: `a` a `note` at `notes/`, `b` a `note2` at `zzz/`.
        // In the path-sorted write pass `docs/notes/a.md` < `docs/zzz/b.md`, so `a` is written
        // (and baselined) *first* and `b`'s write is the one that aborts.
        let note_to = v1_schema(note_v0_yaml());
        let note2_to = v1_schema(note2_v0_yaml());
        let note_schema = load_schema(note_v0_yaml()).expect("v0 note loads");
        let note2_schema = load_schema(note2_v0_yaml()).expect("v0 note2 loads");
        let render_note = |schema: &Schema, title: &str| {
            render(
                schema,
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
        let a0 = render_note(&note_schema, "A Note");
        let b0 = render_note(&note2_schema, "B Note");
        write_doc(repo.path(), "docs/notes/a.md", &a0);
        write_doc(repo.path(), "docs/zzz/b.md", &b0);

        // Both docs are already tracked in the file-state baseline (the migration re-baselines
        // only docs it already tracks) at their v0 hashes.
        let mut seed = FileStateRecord::new();
        seed.record("docs/notes/a.md".to_string(), hash_bytes(a0.as_bytes()));
        seed.record("docs/zzz/b.md".to_string(), hash_bytes(b0.as_bytes()));
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        // Force `b`'s atomic write to fail: make its home read-only, so `persist`'s temp write
        // into `docs/zzz/` errors (EACCES) — the loop aborts after `a` is written. This is
        // temp-name-independent (the atomic temp sibling is process-unique, M45 Inc 7), unlike
        // occupying a fixed `<name>.tmp` path.
        let zzz = repo.path().join("docs/zzz");
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&zzz, fs::Permissions::from_mode(0o555)).expect("chmod zzz ro");
        }

        let pack = crate::pack::EmbeddedPack::new();
        let result = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(note_to, 1), migration(note2_to, 1)],
            Options::default(),
        );
        // Restore write access so the assertions below and the `TempDir` teardown can proceed.
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&zzz, fs::Permissions::from_mode(0o755)).expect("chmod zzz rw");
        }
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
        let blocked = &blocked_report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/memos/cache-memo.md");
        assert_eq!(blocked.code, "migrate-corpus.prose-needed");
        assert!(
            blocked_route(blocked).contains("re-run `jigc migrate-corpus`"),
            "the route is Framing-A author-then-re-run: {}",
            blocked_route(blocked)
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
        let blocked = &report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/briefs/bare-brief.md");
        assert_eq!(blocked.code, "migrate-corpus.prose-needed");
        let route = blocked_route(blocked);
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
        let blocked = &report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/ledgers/open-ledger.md");
        assert_eq!(blocked.code, "migrate-corpus.unclassified-change");
        assert!(
            blocked.message.contains("no transform kind"),
            "the diagnosis names the unclassifiable change; got: {}",
            blocked.message
        );
        assert!(
            blocked_route(blocked).contains("build the transform kind"),
            "the route names the real repair (build the kind first), not a migration \
             instruction; got: {}",
            blocked_route(blocked)
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
        let blocked = &report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/linked/hub.md");
        assert_eq!(blocked.code, "migrate-corpus.narrowed-cardinality");
        assert!(
            blocked.message.contains("narrows") && blocked.message.contains("meta.rel"),
            "the diagnosis names the narrowed leaf; got: {}",
            blocked.message
        );
        let route = blocked_route(blocked);
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
        let blocked = &report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/linked/hub.md");
        assert_eq!(blocked.code, "migrate-corpus.removed-field");
        assert!(
            blocked.message.contains("drops the declared field")
                && blocked.message.contains("meta.rel"),
            "the diagnosis names the dropped leaf; got: {}",
            blocked.message
        );
        let route = blocked_route(blocked);
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

    /// A prior-shape snapshot that is **not shipped** blocks **at the enumeration** — never a
    /// silent `already-current`, and never a walk silently narrower than the doctype's home
    /// history (M52 Increment 7 / T2).
    ///
    /// The refusal is keyed at `<ty>@v<k>` because no document is in hand there: the walk reads
    /// the doctype's homes out of the store, so the missing snapshot removes **every** instance
    /// of the doctype from the run's subject, not the one that happens to carry that stamp.
    #[test]
    fn a_missing_snapshot_blocks_the_whole_doctype_at_the_enumeration() {
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
        assert_eq!(
            report.blocked.len(),
            1,
            "the doctype is refused: {report:?}"
        );
        let blocked = &report.blocked[0];
        assert_eq!(
            blocked_path(blocked),
            "card@v1",
            "the subject is the snapshot the pack owes, not a bystanding document: {blocked:?}"
        );
        assert_eq!(blocked.code, "migrate-corpus.missing-snapshot");
        assert!(
            blocked_route(blocked).contains("schema-snapshots/card.v1.yaml")
                && blocked_route(blocked).contains("re-run `jigc migrate-corpus`"),
            "the route names the missing snapshot + the re-run: {}",
            blocked_route(blocked)
        );
        // The doc is left byte-untouched (never silently rewritten).
        let after = fs::read_to_string(repo.path().join("docs/cards/first-card.md")).expect("read");
        assert_eq!(after, v1, "the blocked doc is byte-untouched");
    }

    /// The **doc-keyed** half of the same code, and what still reaches it (M52 Increment 7 / T2).
    ///
    /// The enumeration proves every stamp in `1..current` loadable, so the fold's own
    /// missing-snapshot arm answers only the stamp the enumeration does not cover: an
    /// out-of-band `schema-version: 0` — a *written* zero, which no `<ty>.v0.yaml` can ever
    /// source. A document is in hand there, so the target stays the file path, and the run's
    /// other documents are unaffected.
    #[test]
    fn a_doc_stamped_zero_is_blocked_at_its_own_path_while_the_walk_still_builds() {
        let repo = TempDir::new("stamp-zero");
        let jigc_root = repo.path().join(".jigc");

        // The store is COMPLETE for `1..current` (`card.v1.yaml` ships), so the walk builds.
        let pack_dir = multi_snapshot_pack("card", &[(1, card_v1_yaml())]);
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(card_v2_yaml());
        let from_shape = {
            let mut s = load_schema(card_v1_yaml().as_bytes()).expect("v1 card loads");
            inject_schema_version_stamp(&mut s);
            s
        };
        let render_card = |title: &str, stamp: &str| {
            render(
                &from_shape,
                &Instance {
                    title: title.to_string(),
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
                            id: "body".to_string(),
                            slot: Some("Wire the cache.".to_string()),
                            ..Default::default()
                        },
                    ],
                },
            )
        };
        let zero = render_card("Zero Card", "0");
        write_doc(repo.path(), "docs/cards/zero-card.md", &zero);
        write_doc(
            repo.path(),
            "docs/cards/first-card.md",
            &render_card("First Card", "1"),
        );

        let report = migrate_committed_corpus(
            &pack,
            repo.path(),
            &jigc_root,
            &[migration(to, 2)],
            Options::default(),
        )
        .expect("migration runs");

        assert_eq!(report.blocked.len(), 1, "one doc is blocked: {report:?}");
        let blocked = &report.blocked[0];
        assert_eq!(blocked.code, "migrate-corpus.missing-snapshot");
        assert_eq!(
            blocked_path(blocked),
            "docs/cards/zero-card.md",
            "a document IS in hand here, so the target is its path: {blocked:?}"
        );
        assert!(
            blocked_route(blocked).contains("schema-snapshots/card.v0.yaml"),
            "the route names the stamp's own unsourceable snapshot: {}",
            blocked_route(blocked)
        );
        assert_eq!(
            fs::read_to_string(repo.path().join("docs/cards/zero-card.md")).expect("read"),
            zero,
            "the blocked doc is byte-untouched",
        );
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

        // Force the write to the TO home to fail: occupy the destination `CHANGELOG.md`
        // *itself* with a directory, so `persist`'s temp→final `rename` fails (EISDIR) — the
        // move aborts AT the write, BEFORE the source removal (write-before-remove). This is
        // temp-name-independent (the atomic temp sibling is process-unique, M45 Inc 7), unlike
        // occupying a fixed `CHANGELOG.md.tmp` path. The collision adjudicator reads the
        // destination as `None` (a directory does not read as a doc), so it routes to the write.
        fs::create_dir_all(repo.path().join("CHANGELOG.md")).expect("occupy destination");

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
        // The rename failed, so no partial target *file* was written (only the empty
        // occupying directory remains — the injection artifact, not a partial write).
        assert!(
            !repo.path().join("CHANGELOG.md").is_file(),
            "the aborted write left no partial target file"
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
        let blocked = &report.blocked[0];
        assert_eq!(blocked_path(blocked), "docs/deferrals/deferral-ledger.md");
        // THE DISCRIMINATING ASSERTION — the block alone does not distinguish T7 (pre-T7 the
        // change diffed to `[]` and the *backstop* refused it with the `build the transform
        // kind` route, a schema-authoring instruction). Now the change is **named**, so the
        // adjudicator is the per-doc conformance gate and the route is the actionable
        // **prose-authoring** one: the agent fills the required per-item field, then re-runs.
        assert_eq!(
            blocked.code, "migrate-corpus.prose-needed",
            "a named prose need routes to the author, never to `build the transform kind`"
        );
        assert_eq!(
            blocked_route(blocked),
            prose_needing_route("docs/deferrals/deferral-ledger.md"),
        );

        let on_disk = fs::read_to_string(repo.path().join("docs/deferrals/deferral-ledger.md"))
            .expect("read");
        assert_eq!(
            on_disk, v1,
            "the blocked doc is byte-identical v0 — the stamp is NOT bumped"
        );
    }

    /// A throwaway pack carrying **several** prior-schema snapshots for one doctype
    /// (`schema-snapshots/<ty>.v<k>.yaml` per `(k, yaml)`) plus a freeze manifest naming `<ty>`,
    /// so [`crate::pack::load_prior_schema`] resolves each and injects the schema-version stamp
    /// exactly as a shipped pack does. The [`snapshot_pack`] sibling, for a doctype that has
    /// bumped **more than once** — the topology the relocation walk must survive.
    fn multi_snapshot_pack(ty: &str, snapshots: &[(u32, &str)]) -> TempDir {
        let dir = TempDir::new("multi-snap-pack");
        let snaps = dir.path().join("schema-snapshots");
        fs::create_dir_all(&snaps).expect("mk schema-snapshots");
        for (version, yaml) in snapshots {
            fs::write(snaps.join(format!("{ty}.v{version}.yaml")), yaml).expect("write snap");
        }
        let cfg = dir.path().join("config");
        fs::create_dir_all(&cfg).expect("mk config");
        // Only the doctype name gates stamp injection (the freeze gate does not run here), so
        // the manifest version/hash are placeholders — the [`snapshot_pack`] precedent.
        fs::write(
            cfg.join("schema-manifest.yaml"),
            format!(
                "doctypes:\n  - type: {ty}\n    schema-version: 1\n    schema-hash: {}\n",
                "0".repeat(64)
            ),
        )
        .expect("write manifest");
        dir
    }

    /// The v1 (folder-home) shape of a `thing`: `location: things/`, one slot section.
    fn thing_v1_yaml() -> &'static str {
        "\
type: thing
location: things/
singleton: true
id-from: title
sections:
  - id: body
    slot: { hint: \"the thing\" }
"
    }

    /// The v2+ (relocated, placement) shape of a `thing`: the literal root `THING.md`, the same
    /// section. The relocation happened at **v2**; the doctype has since bumped again, to **v3**.
    fn thing_placement_yaml() -> &'static str {
        "\
type: thing
placement: { file: THING.md }
singleton: true
id-from: title
sections:
  - id: body
    slot: { hint: \"the thing\" }
"
    }

    /// **The relocation walk unions EVERY prior home, not just the immediately-prior one**
    /// (M42 completion audit, Finding 3).
    ///
    /// [`candidate_docs`] sourced the prior home from `version - 1` alone. A placement doctype
    /// that **relocated at v2** and has since bumped to **v3** therefore loads only the v2
    /// snapshot — which is *already* the placement shape and carries no `location:` — so the
    /// **v1 folder home is never walked**, and an instance a partially-completed migration left
    /// stranded there goes **invisible to the verb** all over again: the exact defect M42's walk
    /// union fixed for the v1→v2 case, one version along. (This wave's signature failure — the
    /// fix applied to the instance, not the class.) Latent on the shipped packs today
    /// (`changelog` is the only relocated doctype and it sits at v2), so it is reproduced over a
    /// synthetic two-snapshot pack — but the union's own rationale is *findable **wherever** a
    /// partially-completed migration left them*, which argues over **every** prior home.
    ///
    /// RED pre-fix: the walk returns **nothing** for the stranded v1-home instance.
    #[test]
    fn the_relocation_walk_unions_every_prior_home_not_just_the_last() {
        let repo = TempDir::new("prior-homes");

        // `thing`: v1 at `things/`, RELOCATED to the literal `THING.md` at v2, now at **v3**.
        let pack_dir = multi_snapshot_pack(
            "thing",
            &[(1, thing_v1_yaml()), (2, thing_placement_yaml())],
        );
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());
        let dt = migration(v1_schema(thing_placement_yaml().as_bytes()), 3);

        // An instance a partially-completed migration left stranded at the **v1** folder home
        // (under the resolved docs-root — the prior snapshot stores its `location:` raw).
        write_doc(
            repo.path(),
            "docs/things/thing.md",
            "# Thing\n\n## Body\n\nX.\n",
        );

        let candidates = candidate_docs(&pack, repo.path(), &dt, Corpus::Worktree)
            .expect("the store ships every snapshot below current, so the walk builds");

        assert!(
            candidates.contains(&("docs/things/thing.md".to_string(), "THING.md".to_string())),
            "the v1 home is walked even though the doctype is at v3 (pre-fix: only v2's home was \
             consulted, which is already the placement shape — so the stranded instance was \
             invisible); got: {candidates:?}",
        );
    }
}
