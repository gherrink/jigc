//! The standalone **move primitive** — `git mv` a managed doc old→new and re-key its
//! file-state entry — extracted from [`rename::apply_and_commit`](crate::rename) (M39
//! Increment 5, T1).
//!
//! This is the single-concern foundation the M39 relocation floor recomposes: a doc-level
//! *move* is exactly `git mv old→new` (staging the rename) plus, in the gitignored
//! file-state record, forgetting the old path key and recording the new path at its hash.
//! It owns **nothing** else — no referrer repoint, no `# H1` rewrite, no introduced-dangling
//! integrity gate. `rename::apply_and_commit` recomposes it with those concerns layered
//! around it (identical behavior), and the M39 `config set docs-root` / freeze-exempt
//! relocation paths consume it as their whole move (they carry no referrers to repoint).
//!
//! The primitive does **not** author the moved doc's new bytes: a pure relocation preserves
//! them (`git mv` keeps the file byte-identical) and a rename rewrites the `# H1` *after* the
//! move at the caller's layer. The caller therefore supplies the moved doc's final hash
//! (`hash_bytes` of whatever bytes will land at `new_rel`) so the record stays in-sync.

use anyhow::{Context, Result, anyhow};
use std::path::Path;

use engine::file_state::{FileStateRecord, hash_bytes};
use engine::finding::Finding;
use engine::packsource::PackSource;
use engine::schema::Schema;

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::orphan::{self, Home};
use crate::pack;
use crate::render;
use crate::task::git_run;

/// **`jigc relocate`'s refusal axis** — every state this door declines, each paired with the
/// finding code it carries on the printed surface *and* in the invocation log's
/// `finding_codes` (M52 completion audit, fix 5).
///
/// **Why it is an axis and not six repairs.** At that audit the door declined **ten** states
/// carrying **nine** distinct codes (the two malformed `--from` cells share one — the
/// consequence is one); the rc.24 fix pass added the eleventh state and the tenth code,
/// [`RelocateRefusal::ForeignSource`]. Driven at `26d021de`, **four** of the ten already
/// carried a code and a route — the unknown doctype and the two installed-root `--from` values (M52 Increment 8 / T7), and
/// the destination-identity gate (Increment 8 / T6) — while the other **six** reached the
/// wire as bare `anyhow` strings: no `blocking · <code>` prefix, no `route:` line, and on
/// `--format json` an `{"error": "<sentence>"}` a driver cannot key on. A bail carries no
/// code, so those six sat outside the
/// [route floor](../../../design/surface-contract.md) **by construction**:
/// `engine::finding::is_route_exempt` takes a *finding* code and a bail has none. That is the
/// same defect, in the same shape, that `jigc rename`'s eleven refusals were promoted out of
/// at M49 ([`crate::rename::RefusalKind`]) — so this door takes the same mold, and takes it
/// over the **whole** door rather than over the six the audit reported: the four that were
/// already right are members too, so the registry is a statement about the door's refusal
/// surface and not a list of what one fix touched.
///
/// **Reuse over mint, one fault one code.** Of the ten codes, **five** name a fault some
/// other door already raises — a transient doctype, an occupied destination, a destination
/// git cannot record, and the two installed-root prior homes whose sibling is `jigc config
/// set <root-knob>`. **Two** mint (`relocate.` — the states no other door can be in: only
/// this verb takes a `--from` prior home, and only this verb is the freeze-exempt path a
/// frozen doctype must be turned away from). **Three** are raised by the crates that own the
/// condition (`engine::store`, `crate::ingest`, and — for a source that is not a regular
/// file, the same fault the committing doors name — `engine::finalize`) and are named here
/// so the axis is total.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelocateRefusal {
    /// `<ty>` resolves to no doctype in the composed cascade — minted by `engine::store`.
    UnknownDoctype,
    /// `--from` was empty or whitespace: no prior home was named at all.
    PriorHomeMissing,
    /// `--from` named something that is no home — a value that normalizes away (`/`, `//`).
    PriorHomeUnusable,
    /// `--from` named jigc's own workbench (`.jigc/`).
    WorkbenchPriorHome,
    /// `--from` named a root `jigc setup` writes (the adapter install).
    InstalledPriorHome,
    /// `<ty>` is manifest-governed: it relocates through the version-gated `migrate-corpus`.
    FrozenDoctype,
    /// `<ty>` has no home at all (a transient doctype) — there is nothing to relocate.
    TransientDoctype,
    /// A **managed** instance already occupies the destination; a move-INTO it is deferred.
    OccupiedDestination,
    /// The destination is a path git cannot record — inside `.git/`, inside another
    /// repository, or outside this one. Raised by the shared move primitive
    /// ([`move_doc`]), which is why closing it here closes it for that primitive's other
    /// callers too.
    UntrackableDestination,
    /// The destination would sit at a managed home under no `<type>:<slug>` identity —
    /// minted by [`crate::ingest`] (M52 Increment 8 / T6).
    UnaddressableDestination,
    /// The **source** — the doc's own entry at the home it would be moved from — is not a
    /// regular file: a symbolic link, which `git mv` carries to the new home as the entry it
    /// is. Raised by the shared move primitive ([`move_doc`], [`refuse_foreign_source`]), so
    /// it is one guard for every door that moves a doc (the rc.24 fix pass).
    ForeignSource,
}

impl RelocateRefusal {
    /// Every member, in the order the door can reach them: the argument checks, then the
    /// doctype checks, then the per-document move.
    pub const ALL: &'static [RelocateRefusal] = &[
        RelocateRefusal::UnknownDoctype,
        RelocateRefusal::PriorHomeMissing,
        RelocateRefusal::PriorHomeUnusable,
        RelocateRefusal::WorkbenchPriorHome,
        RelocateRefusal::InstalledPriorHome,
        RelocateRefusal::FrozenDoctype,
        RelocateRefusal::TransientDoctype,
        RelocateRefusal::OccupiedDestination,
        RelocateRefusal::UntrackableDestination,
        RelocateRefusal::UnaddressableDestination,
        RelocateRefusal::ForeignSource,
    ];

    /// The finding code this refusal carries.
    pub fn code(self) -> &'static str {
        match self {
            RelocateRefusal::UnknownDoctype => engine::store::UNKNOWN_TYPE,
            RelocateRefusal::PriorHomeMissing | RelocateRefusal::PriorHomeUnusable => {
                MALFORMED_PRIOR_HOME
            }
            RelocateRefusal::WorkbenchPriorHome => "config.workbench-root",
            RelocateRefusal::InstalledPriorHome => "config.unusable-root",
            RelocateRefusal::FrozenDoctype => FROZEN_DOCTYPE,
            RelocateRefusal::TransientDoctype => "store.transient-type",
            RelocateRefusal::OccupiedDestination => "write.already-present",
            RelocateRefusal::UntrackableDestination => "write.untrackable-destination",
            RelocateRefusal::UnaddressableDestination => crate::ingest::UNADDRESSABLE_IDENTITY,
            // The code every door raises over a managed doc's home that is not a regular
            // file — minted by `engine::finalize` ([`engine::finalize::store_home_refusal`]),
            // named here so the axis is total; `the_foreign_source_row_reads_the_shipped_code`
            // holds the two together.
            RelocateRefusal::ForeignSource => "finalize.promote-clobber",
        }
    }
}

/// The `--from` value is no usable prior home — **this door's mint**, because `--from` is a
/// prior home and no other verb takes one (`jigc config set docs-root` adjudicates the same
/// *kind* of value but only as a **destination**, which is what its `config.unusable-root`
/// family is about).
///
/// One code for both malformed cells — absent and un-normalizable — because the consequence
/// is one: the sweep has no prefix to walk. The token the caller typed named nothing, so the
/// finding carries **no locus**, on the precedent `design/command-output-contract.md` states
/// for `work-unit.malformed-id` (*the token the caller typed was never a work-unit id, so
/// there is no `task:<id>` to address it at*) and which this door's two installed-root
/// siblings already follow.
pub const MALFORMED_PRIOR_HOME: &str = "relocate.malformed-prior-home";

/// A manifest-governed doctype was handed to the **freeze-exempt** path — this door's second
/// mint, the reconciliation of the two relocation paths, and a state no other verb is in.
pub const FROZEN_DOCTYPE: &str = "relocate.frozen-doctype";

/// The one repair every `--from` refusal carries: the door cannot compose the prefix the
/// caller meant, and `jigc doc list` is the surface that names every committed doc jigc knows
/// together with the path each one is at. `Human` rather than `Mechanical` because the
/// directory is the operator's to supply — a mechanical argv may carry only a declared
/// placeholder (`engine::finding::ROUTE_PLACEHOLDERS`), and a prior home is not one.
const PRIOR_HOME_ROUTE: &str = "re-run `--from` with the directory this doctype's instances \
                                actually sat at — `jigc doc list` names every committed doc \
                                jigc knows and the path each one is at";

/// Refuse with an identity: flatten `refusal`'s [`Finding`] into the `anyhow` channel this
/// verb returns on, through the shared [`render::finding_error`] carrier — the same shape
/// [`crate::rename::refuse`] uses, and for the same reason, so the code the surface prints is
/// the code `finding_codes` records.
///
/// **Declared bound, inherited and not introduced:** these refusals reach `--format json`
/// inside the `{"error": …}` envelope rather than as the structured finding projection. That
/// is the flattening funnel's shipped posture — the one
/// `design/command-output-contract.md` → *the complement* declares and accepts for a code it
/// lists under **no** target form, and the one this door's two installed-root siblings and
/// `jigc rename`'s whole eleven-member axis already ship. The one member that does project a
/// key is [`RelocateRefusal::UnknownDoctype`], and it does so because `store.unknown-type` is
/// an [`render::ENVELOPE_OWED_CODES`](crate::render::ENVELOPE_OWED_CODES) member — the arm is
/// the **code's** property since the audit's fix 4, so this door does not choose it.
fn refuse(
    refusal: RelocateRefusal,
    at: Option<&str>,
    message: String,
    route: impl Into<engine::finding::Route>,
) -> anyhow::Error {
    render::finding_error(&Finding::graded(
        engine::finding::Severity::Blocking,
        refusal.code(),
        message,
        at.map(|at| engine::finding::Location::addressed(at, 1, 1)),
        Some(route.into()),
    ))
}

/// **Refuse to move a doc whose own entry at its home is not a regular file** — the source
/// half of the move primitive's contract (the rc.24 fix pass; `design/finalize.md` → 4.
/// Promote, *the doors that write a committed home in place*).
///
/// `git mv` moves a directory entry, and a symbolic link is one: it is carried to the new
/// home **as a link**, relative target and all. Driven before this guard, `jigc config set
/// docs-root handbook/sub` over a doc whose home was `→ ../../elsewhere/<doc>.md` exited 0
/// saying `relocating 2 committed doc(s)`; the link dangled one directory deeper, `jigc doc
/// list` then answered a code-less exit 1 for the whole store, and `jigc validate` routed at
/// `jigc unmanage`. And at `jigc rename`, which moves through this primitive too, the moved
/// link was then written *through*.
///
/// jigc keeps a managed doc as a regular file at exactly its home, so a relocation lands one
/// or refuses: the entry is asked without following a link ([`engine::store::home_entry`])
/// and one that is not a regular file is refused under the code every other door raises for
/// that state of that doc. An **absent** source is not this guard's — `git mv` names it.
///
/// Called by [`move_doc`], which every mover funnels through, and by [`relocate_one`] ahead
/// of its squatter displacement, so a move that will be refused parks nobody's file first.
pub(crate) fn refuse_foreign_source(repo_root: &Path, old_rel: &str, new_rel: &str) -> Result<()> {
    if let engine::store::HomeEntry::Foreign(shape) =
        engine::store::home_entry(&repo_root.join(old_rel))
    {
        return Err(render::finding_error(
            &engine::finalize::store_home_refusal(
                repo_root,
                old_rel,
                shape,
                &format!("it is not moved to `{new_rel}`"),
                "re-run this command",
            ),
        ));
    }
    Ok(())
}

/// Move a committed managed doc `old_rel` → `new_rel` and re-key its file-state entry.
///
/// Runs `git mv old_rel new_rel` (skipped when `old_rel == new_rel` — a retitle-in-place
/// has no move) and then, in the file-state record under `jigc_root`, forgets `old_rel`
/// and records `new_rel` at `new_hash`. Loads and saves the record itself, so a caller with
/// no further file-state work (the relocation paths) needs nothing more; `apply_and_commit`
/// re-loads it to layer its referrer re-keys on top (the two saves compose byte-identically).
///
/// **The destination is checked against git's own trackability first**
/// ([`crate::trackable::untrackable_reason`]) and an untrackable one is refused before any
/// byte moves — the M49 completion-triage data-loss fix. `git mv <src> .git/<dst>` prints
/// `error: invalid path` and **exits 0**: it moves the file on disk, drops the source from
/// the index, adds nothing, and every caller here read that 0 as a successful move. The
/// exit code is therefore not the test; this check is, and it lives on the **primitive** so
/// the four doors that funnel through it (`config set docs-root`, `config set
/// placement-root`, `jigc relocate`, `jigc rename`) are covered by one guard rather than
/// four — including when the destination comes from a hand-edited manifest no door
/// adjudicated.
///
/// **The `git mv` batch re-probes the repository posture immediately before it runs**
/// (M51 Increment 2; `settle-record.md` → Review amendments §3), as
/// [`crate::repo::SeamAct::Move`]: *operation in progress* only, because which commit the
/// move joins stays the user's to decide while moving a tracked file out from under a
/// half-finished merge is not. The door adjudicated the same member before anything was
/// resolved; this asks again at the act, where a hook, a concurrent process or an earlier
/// phase of this same run could have started one in between.
///
/// **The subject is not a parameter, and that is the decision, not an omission.** There is
/// exactly one `git mv` in the product and no mover ever runs in a fan-out worktree, so a
/// `&SeamSubject` here would carry no choice — while a subject threaded in from a caller
/// would be a *stale* expectation by the time it reached this line, which is the opposite
/// of what §3 asks for. [`crate::task::git_commit_capture`] is told its subject because it
/// genuinely cannot tell live from dedicated; this one can.
pub fn move_doc(
    repo_root: &Path,
    jigc_root: &Path,
    old_rel: &str,
    new_rel: &str,
    new_hash: &str,
) -> Result<()> {
    if old_rel != new_rel {
        if let Some(reason) = crate::trackable::untrackable_reason(repo_root, new_rel) {
            // **Coded at the primitive, which is where the check is** (M52 completion audit,
            // fix 5). `jigc rename` has refused this fault as `write.untrackable-destination`
            // since M50 and minted that code *for* the primitive's other callers
            // ([`crate::rename::RefusalKind::code`]: *"the code those doors join when they
            // are converged"*); the convergence is here, so `jigc relocate`,
            // `migrate-corpus`'s relocation arm and `finalize`'s promote/retire stop
            // refusing the identical fault code-less and route-less.
            return Err(refuse(
                RelocateRefusal::UntrackableDestination,
                Some(new_rel),
                format!("refusing to move `{old_rel}` to a destination git cannot track: {reason}"),
                engine::finding::Route::human(
                    "the destination is the doctype's declared home plus the doc's own \
                     file name — re-point that home at a path git can record (`jigc \
                     config get docs-root` and `jigc config get placement-root` name \
                     the two in force) and re-run",
                ),
            ));
        }
        // The source's own entry, asked on the primitive for the reason the destination's
        // trackability is: one guard for every door that moves a doc, not one per door.
        refuse_foreign_source(repo_root, old_rel, new_rel)?;
        crate::repo::SeamSubject::live(repo_root).verify(crate::repo::SeamAct::Move)?;
        git_run(repo_root, &["mv", old_rel, new_rel])?;
    }
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    record.forget(old_rel);
    record.record(new_rel.to_string(), new_hash.to_string());
    record
        .save(jigc_root)
        .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;
    Ok(())
}

/// **The undo of a staged `git mv` batch** — the capture half of `ROLLBACK_POPULATIONS`'
/// `config-root-relocation` row (M52 Increment 5 / T8; `settle-record.md` → **D1.5**,
/// population 10).
///
/// It lives beside [`move_doc`] because it is that primitive's inverse, and because both
/// doors that batch moves reach the primitive from different modules — `config set
/// docs-root`'s own loop calls it directly, `config set placement-root` reaches it through
/// [`relocate_stranded`]. One home, so the two arms of one knob pair cannot capture
/// differently.
///
/// **What a move writes, and therefore what the undo puts back**, per doc: the destination
/// (created), the prior home (emptied), and — once for the whole batch — the gitignored
/// `.jigc/state/file-state.json` the primitive re-keys. All three ride the shared
/// compare-and-swap ([`crate::rollback::PreImageFamily`]), so a path a third party has
/// written to since jigc left it is never overwritten: its bytes stand, jigc's pre-image
/// parks in the gitignored workbench, and one blocking `<door>.rollback-conflict` names both.
///
/// The **index** is the other axis and is not the family's: each landed pair's entries are
/// un-staged with `git restore --staged`, deliberately without `--worktree`, because the
/// bytes at both paths are the swap's subject ([`crate::rename`]'s landing arm, same rule).
pub(crate) struct MoveRollback {
    /// The worktree paths this batch rewrote, compare-and-swap.
    family: crate::rollback::PreImageFamily,
    /// The `(prior, destination)` pairs whose **index** entries the rollback un-stages —
    /// only the moves that actually landed, so a failed one is never un-staged twice.
    staged: Vec<(String, String)>,
    /// The file-state record's own identity, once captured — the one entry of the batch
    /// that is not per-doc, since [`move_doc`] rewrites that file on every call.
    record: Option<String>,
}

impl MoveRollback {
    /// An empty undo set for `door` — inert until a move captures into it, because the
    /// family restores **per entry** (`crate::rollback::PreImageFamily`): a capture that
    /// never ran and a capture that found nothing are the same safe value.
    pub(crate) fn for_door(door: crate::rollback::ConflictDoor) -> Self {
        Self {
            family: crate::rollback::PreImageFamily::empty(door),
            staged: Vec::new(),
            record: None,
        }
    }

    /// Capture what a move is **about** to overwrite: the destination as it stands now, and
    /// (once per batch) the file-state record.
    ///
    /// Called immediately before [`move_doc`]. The prior home is deliberately **not**
    /// captured here — see [`Self::landed`].
    pub(crate) fn before_move(
        &mut self,
        repo_root: &Path,
        jigc_root: &Path,
        new_rel: &str,
    ) -> Result<()> {
        if self.record.is_none() {
            let path = FileStateRecord::path_in(jigc_root);
            let identity = render::repo_relative(repo_root, &path);
            self.family.push(
                crate::rollback::PreImage::capture(identity.clone(), path).with_context(|| {
                    format!(
                        "could not read the file-state record under {jigc_root:?} — the \
                             re-point will not re-key a record it cannot put back"
                    )
                })?,
            );
            self.record = Some(identity);
        }
        self.family.push(
            crate::rollback::PreImage::capture(new_rel, repo_root.join(new_rel))
                .with_context(|| format!("could not read the relocation destination {new_rel}"))?,
        );
        Ok(())
    }

    /// Record a move that **landed**: what jigc left at the destination and in the record,
    /// the prior home it emptied, and the index pair to un-stage.
    ///
    /// The prior home's entry is pushed **here** rather than at the capture, and that is the
    /// ordering M52 Increment 5 / T3 paid for once already: an entry claiming *jigc removed
    /// this* over a move that failed would find the doc still there, read it as a racer's,
    /// and park a pre-image for a file jigc never touched. `bytes` are the doc's own — read
    /// by the caller one statement before the move, since by now they are at `new_rel`.
    pub(crate) fn landed(
        &mut self,
        repo_root: &Path,
        old_rel: &str,
        new_rel: &str,
        bytes: Vec<u8>,
    ) {
        self.family.wrote(new_rel);
        if let Some(record) = self.record.clone() {
            self.family.wrote(&record);
        }
        self.family.push(crate::rollback::PreImage::removed(
            old_rel,
            repo_root.join(old_rel),
            bytes,
        ));
        self.staged.push((old_rel.to_owned(), new_rel.to_owned()));
    }
}

/// Undo a landed batch of moves — the restore unit `ROLLBACK_POPULATIONS`'
/// `config-root-relocation` names (M52 Increment 5 / T8).
///
/// Two axes, in the order that keeps them from reading each other's work: the **index**
/// first (`git restore --staged`, which moves no byte on disk), then the **worktree**'s
/// compare-and-swap, which returns one blocking `<door>.rollback-conflict` per path whose
/// bytes are no longer jigc's.
///
/// Best-effort on the index arm, exactly as every sibling rollback is: the door's own
/// failure is what the operator has to act on, and a `git restore` that cannot run must not
/// replace it.
///
/// **Declared bound: a displaced foreign squatter is not put back.**
/// [`displace_foreign_squatter`] parks a working file that occupied a destination into the
/// gitignored `.jigc/displaced/` workbench *before* the move, and this undo leaves it there —
/// the bytes survive, at a path the success narration names, but the destination is not
/// re-occupied. It is the residual M52 Increment 5's plan carries by name (bound (vi)):
/// restoring it is a second, opposite act — a move back *out* of the workbench — and it has
/// no entry in this family because the displacement is not one of the moves this batch made.
pub(crate) fn rollback_relocations(
    repo_root: &Path,
    jigc_root: &Path,
    undo: &MoveRollback,
) -> Vec<Finding> {
    for (old_rel, new_rel) in &undo.staged {
        let _ = git_run(repo_root, &["restore", "--staged", "--", old_rel, new_rel]);
    }
    undo.family.restore(repo_root, jigc_root)
}

/// The outcome of a freeze-exempt relocation run, rendered by
/// [`render::freeze_exempt_relocation`].
#[derive(Debug, serde::Serialize)]
pub struct RelocationReport {
    /// Instances moved prior-home → current-home (`from`, `to` pairs), address-sorted.
    pub moved: Vec<(String, String)>,
    /// Instances detected stranded whose move failed (a `git mv` clobber, an unreadable
    /// file), each with its reason — address-sorted. Surfaced, never fatal.
    pub blocked: Vec<(String, String)>,
    /// Foreign (untracked/unmanaged) files that **squatted a relocation destination** and
    /// were **moved into the gitignored `.jigc/` workbench** (`from` destination path → `to`
    /// workbench path), out of the way so the managed instance could land — uncommittable,
    /// never clobbered (`design/reconciliation.md` → Relocation collisions). Address-sorted.
    pub displaced: Vec<(String, String)>,
}

/// Run `jigc relocate <ty> --from <from>` against `cwd`: locate the repo + project layer,
/// resolve `ty`'s current home from the cascade, and relocate every committed instance
/// stranded at the supplied prior home to it. Renders the report through `format` and prints
/// it; a locator error or a frozen-doctype refusal routes to stderr and exits non-zero.
pub fn run(cwd: &Path, ty: &str, from: &str, format: Format) -> Outcome {
    match relocate_in_repo(cwd, ty, from) {
        Ok(report) => {
            println!("{}", render::freeze_exempt_relocation(format, &report));
            Outcome::success()
        }
        Err(err) => crate::invocation_log::operational_failure(format, &err),
    }
}

/// Locate the repo + project layer, resolve the cascade + `ty`'s current schema, parse the
/// human-supplied prior home, and delegate to [`relocate_freeze_exempt`].
fn relocate_in_repo(cwd: &Path, ty: &str, from: &str) -> Result<RelocationReport> {
    let jigc_home = crate::migrate_corpus::require_project_layer(cwd)?;
    let pack = pack::make_pack()?;
    let pack = pack.as_ref();
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);
    let schemas = defs.all_schemas(pack)?;
    // The unknown doctype is the axis's block, not a bare sentence: this door used to
    // answer `no doctype `x` in the resolved cascade` — no finding code, no route, the
    // only door on the axis that offered the reader nothing to run (M49 Increment 11 / T1).
    let schema = schemas
        .get(ty)
        .ok_or_else(|| crate::render::finding_error(&engine::store::unknown_doctype(ty)))?;
    let prior = parse_prior_home(from)?;
    let jigc_root = jigc_home.join(".jigc");
    relocate_freeze_exempt(pack, &jigc_home, &jigc_root, schema, prior)
}

/// Parse a `--from` value into a prior [`Home`]: a value ending in `.md` is a literal
/// **placement** file (`docs/vision.md`), any other non-empty value is a **location**
/// directory (`docs/vision/`, trailing slash stripped). Empty is an error, and so is a prior
/// home naming one of jigc's own installed roots ([`installed_root_refusal`]).
fn parse_prior_home(from: &str) -> Result<Home> {
    if from.trim().is_empty() {
        return Err(refuse(
            RelocateRefusal::PriorHomeMissing,
            None,
            "`--from` (the prior home) is required — a relocation sweeps the committed docs \
             under one prior home, and no prior home was named"
                .to_string(),
            engine::finding::Route::human(PRIOR_HOME_ROUTE),
        ));
    }
    if let Some(finding) = installed_root_refusal(from) {
        return Err(render::finding_error(&finding));
    }
    if from.ends_with(".md") {
        Ok(Home::placement(from))
    } else {
        Home::location(from).ok_or_else(|| {
            refuse(
                RelocateRefusal::PriorHomeUnusable,
                None,
                format!(
                    "`--from` `{from}` is not a valid prior home — it normalizes to nothing, \
                     so it names no directory the sweep could walk"
                ),
                engine::finding::Route::human(PRIOR_HOME_ROUTE),
            )
        })
    }
}

/// Why a `--from` value naming **one of jigc's own installed roots** is refused before
/// anything is swept — `None` for every prior home that is the operator's own (M52 Increment
/// 8 / T7; `completions/artifacts/M52/settle-record.md` → D10.5, A1-D2).
///
/// `--from` is a **prefix**: the door moves every committed `.md` under it into the doctype's
/// home. Every other spelling reaches at worst nothing — an absolute value, a `../` hop and a
/// symlink all match no committed spelling, which is the no-rule this row used to carry — but
/// two prefixes are guaranteed to reach files, and both of them are **jigc's own install**.
/// Driven at `4c0c513e` on a freeze-exempt pack with one ordinarily-named doc planted under
/// each: `--from .claude` reported `1 moved` at exit **0** and staged
/// `R .claude/notes.md -> docs/decisions/notes.md`, and `--from .jigc` did the same to
/// `.jigc/notes.md`; the artifacts whose basenames are not doc ids (`.jigc/AGENT.md`,
/// `.claude/skills/jigc/SKILL.md`, `CLAUDE.md`) escaped only by T6's destination-identity
/// gate, which then printed — as the repair — a `git mv` of jigc's own bootstrap file into
/// the docs root followed by `jigc ingest`.
///
/// **No code is minted: both refusals take a sibling door's shipped code**, under the rule
/// this wave's ledger declares. `jigc config set <root-knob>` adjudicates the same *kind* of
/// value (a directory managed docs are resolved against) and already owns both answers —
/// [`crate::config::is_workbench_root`] → `config.workbench-root` for jigc's workbench, and
/// [`crate::config::installed_artifact_root`], the seventh shape of
/// [`crate::config::unusable_root_reason`]'s family → `config.unusable-root` for the adapter
/// install roots the profile declares. **The sentence is this door's**, because the
/// consequence is: a root knob's is *docs homed among files the install rewrites*, and this
/// one's is *the install carried out of the paths it is loaded from*.
fn installed_root_refusal(from: &str) -> Option<Finding> {
    let route = "re-run `--from` with the directory this doctype's instances actually sat at \
                 — `jigc doc list` names every committed doc jigc knows and the path each one \
                 is at";
    if crate::config::is_workbench_root(from) {
        return Some(Finding::block(
            "config.workbench-root",
            format!(
                "`{from}` is inside jigc's own workbench (`.jigc/`) — `--from` sweeps every \
                 committed doc under the prior home into this doctype's home, and jigc's own \
                 state was never a home any doctype's instances sat at"
            ),
            route,
        ));
    }
    crate::config::installed_artifact_root(from).map(|root| {
        Finding::block(
            "config.unusable-root",
            format!(
                "`{from}` is inside jigc's own adapter install — `{root}` is one of the paths \
                 `jigc setup` writes, and `--from` sweeps every committed doc under the prior \
                 home into this doctype's home, which would carry jigc's own install out of \
                 the paths it is loaded from"
            ),
            route,
        )
    })
}

/// The **parallel freeze-exempt relocation path** — the sibling of the frozen, snapshot-gated
/// [`crate::migrate_corpus::migrate_committed_corpus`] relocation arm (M39 inc-5 T4;
/// `design/corpus-migration.md` → Relocation: freeze-exempt sibling; `design/storage.md` →
/// Freeze + relocation).
///
/// A **freeze-exempt** doctype (no `schema-manifest.yaml` entry) carries **no prior-home
/// snapshot**, so — unlike the frozen path, which *derives* the prior home from the versioned
/// snapshot and can auto-move — its prior home is **human-supplied** (`prior`); auto-derivation
/// stays unsafe. This guards that `schema` is genuinely freeze-exempt (a frozen doctype
/// relocates through the version-gated `jigc migrate-corpus`, so it is **refused** here — the
/// reconciliation of the two paths), reads its **current** home from the schema, and delegates
/// the detect+move to [`relocate_stranded`].
///
/// # The refusal's route became true rather than being repaired (M52 Increment 7 / T4)
///
/// That hand-off named the right owner of the act and, for three of the four
/// `{location, placement}²` home moves, pointed at a verb that did **nothing**: the corpus
/// walk keyed on the *current* home alone, so an operator sent here ran `jigc migrate-corpus`
/// against a genuinely stranded instance and read `0 migrated, 0 already current, 0 blocked`
/// at exit 0. T1 widened the walk to every prior home of every kind
/// (`crate::migrate_corpus::candidate_docs`), which is why this sentence stands unchanged:
/// the fix belonged in the verb the route names, not in the route (settle-record D4.5).
///
/// It is **driven, not asserted** —
/// `crates/cli/tests/migrate_route_family.rs::the_relocate_refusal_routes_to_a_verb_that_acts`
/// lifts the emitted command out of this refusal, runs it verbatim through a real shell, and
/// reads the stranded instance back landed at the current home. That suite also fences the rest
/// of the family: every production site naming the verb carries a stated firing state and
/// disposition (D4.6).
pub(crate) fn relocate_freeze_exempt(
    pack: &dyn PackSource,
    repo_root: &Path,
    jigc_root: &Path,
    schema: &Schema,
    prior: Home,
) -> Result<RelocationReport> {
    if pack::frozen_doctype_versions(pack).contains_key(&schema.ty) {
        return Err(refuse(
            RelocateRefusal::FrozenDoctype,
            Some(&schema.ty),
            format!(
                "`{}` is a frozen doctype — relocate it through the version-gated \
                 `jigc migrate-corpus`, not the freeze-exempt path",
                schema.ty,
            ),
            engine::finding::Route::mechanical(
                ["jigc", "migrate-corpus"],
                " walks every prior home the doctype's versioned snapshots declare and lands \
                 the move under the freeze",
            ),
        ));
    }
    let current = orphan::home_of(schema).ok_or_else(|| {
        refuse(
            RelocateRefusal::TransientDoctype,
            Some(&schema.ty),
            format!(
                "`{}` is a transient (home-less) doctype — nothing to relocate",
                schema.ty
            ),
            engine::finding::Route::mechanical(
                ["jigc", "describe"],
                " names which doctypes persist to a home and which are transient",
            ),
        )
    })?;
    // `jigc relocate` is not a transaction: it has no second act to fail after the moves, so
    // there is nothing for a rollback to be triggered by (`ROLLBACK_POPULATIONS` carries the
    // population under `config set <root-knob>`, the door that does).
    relocate_stranded(repo_root, jigc_root, &prior, &current, Some(schema), None)
}

/// Detect every committed doc **stranded** at `prior` (sitting there, no longer at `current`)
/// and move it into `current`, byte-preserving, via the T1 [`move_doc`] primitive. Walks the
/// same `git ls-files` committed truth the orphan detector does, keying on
/// [`orphan::is_stranded`]. A per-doc failure (a `git mv` clobber, an unreadable file) is
/// captured in `blocked` and surfaced, never fatal — the rest still relocate.
///
/// **`schema` is the destination-identity gate's subject** (M52 Increment 8 / T6): when a
/// caller supplies the *resolved* schema whose home is `current`, each destination is asked
/// [`unaddressable_destination`] before anything moves, and a destination no
/// `<type>:<slug>` address reaches becomes a `blocked` row carrying
/// `ingest.unaddressable-identity` instead of a move. `None` is the `placement-root` sweep,
/// whose destinations are fixed-identity homes the gate is vacuous over — see that helper.
/// The gate binds the **move**, never the run: `jigc relocate` is a triage door, so the
/// addressable instances of the same sweep still relocate and the exit is unchanged
/// (`design/validation.md` → the row's exit clause, `jigc ingest`'s own divergence).
///
/// **One error is not a per-doc failure and ends the sweep: the posture family's operation
/// in progress** (M52 Increment 3 / T4). Both acts below re-probe it immediately before
/// they touch the index ([`displace_foreign_squatter`]'s `git rm --cached`, [`move_doc`]'s
/// `git mv`), and what they answer is a fact about the *repository* — so collecting it as a
/// row would exit 0 over a refusal, and would go on asking the same question of every
/// remaining instance.
pub(crate) fn relocate_stranded(
    repo_root: &Path,
    jigc_root: &Path,
    prior: &Home,
    current: &Home,
    schema: Option<&Schema>,
    mut undo: Option<&mut MoveRollback>,
) -> Result<RelocationReport> {
    let mut report = RelocationReport {
        moved: Vec::new(),
        blocked: Vec::new(),
        displaced: Vec::new(),
    };
    for rel in orphan::committed_markdown(repo_root) {
        if !orphan::is_stranded(&rel, prior, current) {
            continue;
        }
        let Some(dest) = dest_in_home(&rel, current) else {
            continue;
        };
        if dest == rel {
            continue; // already at the destination (is_stranded guards this — defensive).
        }
        if let Some(schema) = schema
            && let Some(finding) = unaddressable_destination(repo_root, &rel, &dest, schema)
        {
            report
                .blocked
                .push((rel, render::finding_line(&finding, false)));
            continue;
        }
        match relocate_one(repo_root, jigc_root, &rel, &dest, undo.as_deref_mut()) {
            Ok(displaced) => {
                report.moved.push((rel, dest));
                if let Some(pair) = displaced {
                    report.displaced.push(pair);
                }
            }
            // A posture refusal is about the **repository**, not about this doc: every
            // remaining stranded instance would meet the identical state, and a sweep that
            // collected it as one `blocked` row would exit 0 over a refusal — the run
            // saying *success* about a repository it was not allowed to act in. So it ends
            // the sweep. The moves that already landed stay as the staged `git mv`s they
            // were (`git status` names them, exactly as for a run that completed), and the
            // re-run the route asks for relocates what is left: a doc already at its home
            // is no longer stranded.
            Err(err) if is_operation_in_progress(&err) => return Err(err),
            Err(err) => report.blocked.push((rel, format!("{err:#}"))),
        }
    }
    report.moved.sort();
    report.displaced.sort();
    report.blocked.sort();
    Ok(report)
}

/// Whether `err` is the posture family's **operation in progress** refusal — the one error
/// [`relocate_stranded`]'s per-doc loop can meet that is a fact about the repository rather
/// than about the doc in hand.
///
/// It reads the **carried finding's code** rather than the rendered message: the two seam
/// sites above ([`displace_foreign_squatter`], [`move_doc`]) both raise it through
/// [`crate::render::finding_error`], so the identity is on the error, and a message match
/// would be a second spelling of a code that already has one home
/// ([`crate::repo::PostureMember::code`]).
fn is_operation_in_progress(err: &anyhow::Error) -> bool {
    err.downcast_ref::<crate::render::BlockedFinding>()
        .is_some_and(|blocked| {
            blocked.finding.code == crate::repo::PostureMember::OperationInProgress.code()
        })
}

/// Move one stranded doc `old_rel` → `new_rel` byte-preserving via [`move_doc`]. A pure
/// relocation keeps the bytes (`git mv` is byte-identical), so the re-keyed file-state hash is
/// the current file's. Any **foreign** squatter occupying the destination is first displaced
/// into the workbench ([`displace_foreign_squatter`]) so the move lands and no working file is
/// clobbered; a *managed* instance already there blocks (a managed move-INTO is deferred). The
/// destination directory is created after (a `git mv` needs it to exist — a relocation to a
/// fresh home creates dirs that never existed). Returns the displaced-squatter pair, if any.
fn relocate_one(
    repo_root: &Path,
    jigc_root: &Path,
    old_rel: &str,
    new_rel: &str,
    undo: Option<&mut MoveRollback>,
) -> Result<Option<(String, String)>> {
    // Before anything is displaced: a source this sweep will not move must not cost the
    // destination's squatter its place first.
    refuse_foreign_source(repo_root, old_rel, new_rel)?;
    let displaced = displace_foreign_squatter(repo_root, jigc_root, new_rel)?;
    let bytes = std::fs::read(repo_root.join(old_rel))
        .with_context(|| format!("reading the stranded doc {old_rel}"))?;
    let new_hash = hash_bytes(&bytes);
    if let Some(parent) = Path::new(new_rel).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(repo_root.join(parent))
            .with_context(|| format!("creating the destination dir for {new_rel}"))?;
    }
    match undo {
        // A caller that can undo this batch captures the destination **before** the move and
        // registers the vacated home **after** it (M52 Increment 5 / T8).
        Some(undo) => {
            undo.before_move(repo_root, jigc_root, new_rel)?;
            move_doc(repo_root, jigc_root, old_rel, new_rel, &new_hash)?;
            undo.landed(repo_root, old_rel, new_rel, bytes);
        }
        None => move_doc(repo_root, jigc_root, old_rel, new_rel, &new_hash)?,
    }
    Ok(displaced)
}

/// The gitignored `.jigc/` subdir a **foreign** file squatting a relocation destination is
/// parked in — out of the destination, uncommittable (the move-INTO-workbench arm;
/// `design/reconciliation.md` → Relocation collisions). A dedicated home (not `tasks/`,
/// `milestones/`, or `worktrees/`, each of which a reader enumerates) so parked detritus never
/// masquerades as a task/worktree; listed in [`crate::gitignore::ENTRIES`].
///
/// **Two producers park here, and they park differently** (M52 Increment 4 / T3): this arm
/// parks one squatter by **basename** at the root, while
/// [`crate::task::displace_foreign_area`] parks a whole working area's complement under
/// `<unit-id>/`, relative paths preserved. The home is shared because the property both need
/// is the same one — a gitignored tree inside `.jigc/` that no commit can sweep up — and it is
/// named from here rather than re-spelled, so the entry in
/// [`crate::gitignore::ENTRIES`] governs both.
pub(crate) const WORKBENCH_SUBDIR: &str = "displaced";

/// Resolve a **collision at the relocation destination** `dest_rel` by *kind*
/// (`design/reconciliation.md` → Relocation collisions — the two-resolutions reconcile):
///
/// - **destination free** → `Ok(None)`, nothing to do;
/// - a **managed** instance already there (baselined in the file-state record) → a managed
///   move-INTO the destination is **deferred** (it needs an unmodeled managed-but-uncommitted
///   state), so the collision is **surfaced as a block**, never a clobber;
/// - a **foreign** (untracked/unmanaged) file → **moved into the gitignored `.jigc/`
///   workbench** ([`WORKBENCH_SUBDIR`]) so the managed instance can land and a working file is
///   never accidentally committed — returned as `Some((dest, workbench))` for the report.
///
/// Classification is by baseline: a managed instance is recorded in the file-state map; a
/// foreign file is not. The index slot of a *committed-but-unmanaged* squatter is freed
/// (`git rm --cached --ignore-unmatch`, a no-op for an untracked squatter) so the managed
/// `git mv` into the now-vacated destination succeeds.
fn displace_foreign_squatter(
    repo_root: &Path,
    jigc_root: &Path,
    dest_rel: &str,
) -> Result<Option<(String, String)>> {
    let dest_abs = repo_root.join(dest_rel);
    // The destination's own entry, read without following a link (the rc.24 fix pass). This
    // asked `exists()`, which follows one — so a **dangling** link squatting the destination
    // read as free, was not displaced, and the move then met git's own `fatal: destination
    // exists` as a code-less `blocked` row. A link is a squatter like any other entry: it is
    // parked below as the entry it is (`fs::rename` moves a link, it does not follow it).
    if engine::store::home_entry(&dest_abs) == engine::store::HomeEntry::Free {
        return Ok(None);
    }
    let record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    if record.get(dest_rel).is_some() {
        return Err(refuse(
            RelocateRefusal::OccupiedDestination,
            Some(dest_rel),
            format!(
                "the destination `{dest_rel}` is occupied by a managed instance — a managed \
                 move-INTO the destination is deferred"
            ),
            engine::finding::Route::human(format!(
                // Both claimants are in the file-state record, i.e. **committed**, and
                // `jigc relocate` is a task-less store op — so the verb that moves one of
                // them is `jigc rename`, not its in-task sibling. Driven at `609da011`,
                // the shipped route dead-ended: `jigc doc rename vision --to X` over a
                // committed doc with no task open exits 1 with *no active task — start one
                // with `jigc start`*, and with a task open it refuses the re-slug of a
                // committed identity and routes back at `jigc rename` (M53 — the pre-v1
                // usability batch, row 3; the sibling-confusion class the rc.14 trial's
                // F-4 reports at the two `--help`s, shipped here as a route).
                "two managed docs claim `{dest_rel}` — decide which one keeps the home, then \
                 `jigc rename` the other onto a free identity or `jigc unmanage` it; \
                 `jigc doc list` names what is managed there",
            )),
        ));
    }
    let name = Path::new(dest_rel)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow!("the squatter `{dest_rel}` has no file name"))?;
    let workbench_dir = jigc_root.join(WORKBENCH_SUBDIR);
    std::fs::create_dir_all(&workbench_dir)
        .with_context(|| format!("creating the .jigc workbench {workbench_dir:?}"))?;
    let workbench_abs = workbench_dir.join(name);
    // **Re-probe immediately before the index mutation** (M52 Increment 3 / T4;
    // `settle-record.md` → D2.5), as [`crate::repo::SeamAct::Move`] — the same class
    // [`move_doc`] asks, for the same reason, at the last of the eleven index-mutating
    // sites the M52 baseline enumerated and the only one no probe stood in front of
    // ([baseline-posture.md](../../../completions/artifacts/M52/baseline-posture.md)
    // §1.3, §2.10). The door adjudicated the posture many calls earlier; a hook, a
    // concurrent process or an earlier phase of this same run can open an operation in
    // between, and `move_doc`'s own probe is **too late** for this file: by the time it
    // runs, the `git rm --cached` below has already dropped the user's
    // committed-but-unmanaged bytes from the index and the `fs::rename` has parked them
    // in a gitignored tree — driven, that ran under an un-concluded cherry-pick at exit
    // 0 (`tests/relocate.rs`).
    crate::repo::SeamSubject::live(repo_root).verify(crate::repo::SeamAct::Move)?;
    // Free any index slot the squatter holds (a committed-but-unmanaged file) so the managed
    // `git mv` into the destination succeeds; `--ignore-unmatch` makes an untracked squatter a
    // no-op.
    git_run(
        repo_root,
        &[
            "rm",
            "--cached",
            "--ignore-unmatch",
            "--quiet",
            "--",
            dest_rel,
        ],
    )?;
    std::fs::rename(&dest_abs, &workbench_abs)
        .with_context(|| format!("moving the foreign squatter {dest_rel} into the workbench"))?;
    let workbench_rel = workbench_abs
        .strip_prefix(repo_root)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| workbench_abs.to_string_lossy().into_owned());
    Ok(Some((dest_rel.to_string(), workbench_rel)))
}

/// **The destination-identity gate** — `Some(finding)` when moving `rel` to `dest` would
/// put a file at `schema`'s home that no `<type>:<slug>` address reaches (M52 Increment 8 /
/// T6; `design/validation.md` → the `ingest.unaddressable-identity` row).
///
/// **The subject is the destination, not the `--from` token.** The move is byte-faithful —
/// `git mv` carries the source's own filename into the home — so the identity the store
/// will mint is decided by a name the caller never typed on the command line. Driven at
/// `25cb978a`, `jigc relocate note --from notes` over `notes/My Note.md` moved it at exit
/// 0; `jigc doc list` then reported `note:My Note` **managed** and `jigc doc show` refused
/// that very identity `store.malformed-slug`, routing the reader back at `jigc doc list` —
/// the loop `ingest.unaddressable-identity` was minted to close, at the *other* door that
/// puts a file at a managed home.
///
/// **One predicate, not two agreeing ones.** It asks [`crate::ingest::home_identity`] — the
/// same function `jigc ingest`'s adoption gate asks and `engine::index::committed_instances`
/// mints from — rather than re-deriving the slug rule here, so the two doors cannot answer
/// differently about one filename.
///
/// **A `Placement` destination is never gated, by construction**, which is why the caller
/// may pass no schema at all: a fixed-identity doctype's slug is the type id, so its home's
/// file name supplies nothing to the identity and [`crate::ingest::home_identity`] answers
/// `<ty>:<ty>` whatever that file is called.
///
/// **Declared bound: the route is the identity repair, not a file-state repair.** The
/// finding's `git mv` names the home *and* an addressable name, so running it verbatim
/// achieves what the relocation was for and leaves the doc adoptable. If the stranded doc
/// was already baselined, that hand move leaves the record keyed at the old path — the
/// pre-existing, already-routed `file-state.orphaned-doc` state, not a dead end this gate
/// introduces.
fn unaddressable_destination(
    repo_root: &Path,
    rel: &str,
    dest: &str,
    schema: &Schema,
) -> Option<Finding> {
    crate::ingest::home_identity(dest, schema)
        .is_none()
        .then(|| {
            // Keyed at `rel`, the path that **exists**: law 1's printed-path rule — `dest` is
            // where the doc would have gone, and naming a file that is not there is the lie
            // the rule forbids. The identity fault is the same either way: a `Location` move
            // keeps the filename, so the stem the gate measured at `dest` is `rel`'s own.
            crate::ingest::unaddressable_identity_finding(
                repo_root,
                rel,
                schema,
                crate::ingest::UnaddressableDoor::Relocate,
            )
        })
}

/// The destination path a stranded doc `rel` moves to inside `current`: a `Placement` home is
/// the literal singleton file; a `Location` home keeps the doc's own filename under the dir.
fn dest_in_home(rel: &str, current: &Home) -> Option<String> {
    match current {
        Home::Placement(file) => Some(file.clone()),
        Home::Location(dir) => {
            let name = Path::new(rel).file_name()?.to_str()?;
            Some(format!("{dir}/{name}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// A throwaway git repo that removes itself on drop (the project's no-tempfile pattern).
    struct TempRepo(PathBuf);

    impl TempRepo {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-relocate-unit-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            ));
            std::fs::create_dir_all(&path).expect("create temp repo");
            let repo = TempRepo(path);
            repo.git(&["init", "-q"]);
            repo.git(&["config", "user.email", "t@t"]);
            repo.git(&["config", "user.name", "t"]);
            repo
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn git(&self, args: &[&str]) {
            let out = std::process::Command::new("git")
                .arg("-C")
                .arg(&self.0)
                .args(args)
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_CONFIG_SYSTEM", "/dev/null")
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr),
            );
        }

        fn commit_file(&self, rel: &str, body: &str) {
            let abs = self.0.join(rel);
            if let Some(parent) = abs.parent() {
                std::fs::create_dir_all(parent).expect("create parent dir");
            }
            std::fs::write(&abs, body).expect("write file");
            self.git(&["add", "--", rel]);
            self.git(&["commit", "-q", "-m", "add"]);
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The primitive drives a real `git mv` and re-keys file-state: after moving a committed
    /// doc, the file lives at the new path (gone from the old), the record has **forgotten**
    /// the old key, and the new key carries the supplied hash — the whole contract the M39
    /// relocation floor recomposes.
    #[test]
    fn move_doc_moves_the_file_and_rekeys_file_state() {
        let repo = TempRepo::new();
        let body = "# A decision\n\nProse.\n";
        repo.commit_file("decisions/old.md", body);

        let jigc_root = repo.path().join(".jigc");
        let old_hash = hash_bytes(body.as_bytes());
        let mut seed = FileStateRecord::new();
        seed.record("decisions/old.md".to_string(), old_hash.clone());
        seed.save(&jigc_root).expect("seed the file-state record");

        move_doc(
            repo.path(),
            &jigc_root,
            "decisions/old.md",
            "decisions/new.md",
            &old_hash,
        )
        .expect("the move primitive succeeds");

        assert!(
            !repo.path().join("decisions/old.md").exists(),
            "the old file is gone after the move",
        );
        assert!(
            repo.path().join("decisions/new.md").exists(),
            "the moved file lands at the new path",
        );

        let record = FileStateRecord::load(&jigc_root).expect("reload the record");
        assert_eq!(
            record.get("decisions/old.md"),
            None,
            "the old file-state key is forgotten",
        );
        assert_eq!(
            record.get("decisions/new.md"),
            Some(old_hash.as_str()),
            "the new file-state key is recorded at the supplied hash",
        );
    }

    /// **The primitive never carries a link to a new home** (the rc.24 fix pass): a source
    /// whose own entry is not a regular file is refused before `git mv`, under the code
    /// every door raises for a managed doc's home in that state, and nothing moves — not the
    /// link, not the index, not the record. Driven before the guard, `git mv` moved the link
    /// as the entry it is and a relative one dangled at its new depth.
    #[test]
    fn move_doc_refuses_a_source_that_is_a_link_and_moves_nothing() {
        let repo = TempRepo::new();
        let body = "# A decision\n\nProse.\n";
        repo.commit_file("elsewhere/real.md", body);
        std::fs::create_dir_all(repo.path().join("decisions")).expect("mk decisions/");
        for (name, target) in [
            ("live.md", "../elsewhere/real.md"),
            ("dangling.md", "../elsewhere/gone.md"),
        ] {
            let rel = format!("decisions/{name}");
            std::os::unix::fs::symlink(target, repo.path().join(&rel)).expect("link");
            repo.git(&["add", "--", &rel]);
            repo.git(&["commit", "-q", "-m", "a doc's home is a link"]);

            let jigc_root = repo.path().join(".jigc");
            let dest = format!("moved/{name}");
            std::fs::create_dir_all(repo.path().join("moved")).expect("mk the destination dir");
            let err = move_doc(
                repo.path(),
                &jigc_root,
                &rel,
                &dest,
                &hash_bytes(body.as_bytes()),
            )
            .expect_err("a link is not moved");
            let finding = crate::render::blocked_finding(&err)
                .unwrap_or_else(|| panic!("{name}: the refusal carries a finding: {err:#}"));
            assert_eq!(
                finding.code,
                RelocateRefusal::ForeignSource.code(),
                "{name}: the registry row reads the shipped constructor's code",
            );
            assert!(
                finding.message.contains("a symbolic link")
                    && finding.message.contains(&rel)
                    && finding.message.contains(&dest),
                "{name}: names the entry, its home and the move it withheld: {}",
                finding.message,
            );
            assert!(
                finding.route.is_some(),
                "{name}: a blocking finding carries a route",
            );
            assert!(
                std::fs::symlink_metadata(repo.path().join(&rel))
                    .is_ok_and(|entry| entry.file_type().is_symlink()),
                "{name}: the link stands where it stood",
            );
            assert!(
                std::fs::symlink_metadata(repo.path().join(&dest)).is_err(),
                "{name}: nothing landed at the destination",
            );
            let status = std::process::Command::new("git")
                .args(["status", "--porcelain"])
                .current_dir(repo.path())
                .output()
                .expect("run git status");
            assert!(
                String::from_utf8_lossy(&status.stdout).trim().is_empty(),
                "{name}: the index is untouched: {}",
                String::from_utf8_lossy(&status.stdout),
            );
            assert!(
                FileStateRecord::load(&jigc_root)
                    .expect("load the record")
                    .get(&dest)
                    .is_none(),
                "{name}: and no file-state key was minted for a move that did not happen",
            );
        }
    }

    /// The foreign-source row's code is the shipped constructor's — the registry names it so
    /// the axis is total, and may not become a second spelling of it.
    #[test]
    fn the_foreign_source_row_reads_the_shipped_code() {
        assert_eq!(
            RelocateRefusal::ForeignSource.code(),
            engine::finalize::store_home_refusal(
                Path::new("/repo"),
                "legacy/a.md",
                engine::store::ForeignEntry::Symlink,
                "it is not moved",
                "re-run this command",
            )
            .code,
        );
    }

    /// **A dangling link squatting the destination is a squatter** (the rc.24 fix pass). The
    /// occupancy probe asked `exists()`, which follows links, so a dangling one read as a
    /// free destination, was not displaced, and the move met git's own `fatal: destination
    /// exists`. Read without following it, the link takes the arm every other foreign entry
    /// takes: parked in the gitignored workbench as the entry it is, and the doc lands.
    #[test]
    fn a_dangling_link_squatting_the_destination_is_displaced_like_any_squatter() {
        let repo = TempRepo::new();
        let managed = "---\nx: y\n---\n\n# Vision\n\nThesis.\n";
        repo.commit_file("docs/vision/vision.md", managed);
        std::os::unix::fs::symlink("nowhere.md", repo.path().join("VISION.md"))
            .expect("plant the dangling squatter");

        let jigc_root = repo.path().join(".jigc");
        crate::gitignore::ensure(&jigc_root).expect("write the canonical .jigc/.gitignore");
        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/vision/vision.md".to_string(),
            hash_bytes(managed.as_bytes()),
        );
        seed.save(&jigc_root).expect("seed the file-state record");

        let schema = placement_schema("vision", "VISION.md");
        let pack = crate::pack::EmbeddedPack::new();
        let prior = Home::location("docs/vision/").expect("prior home");
        let report = relocate_freeze_exempt(&pack, repo.path(), &jigc_root, &schema, prior)
            .expect("the relocation runs");

        assert!(
            report.blocked.is_empty(),
            "no blockers — and no `fatal: destination exists`: {:?}",
            report.blocked
        );
        assert_eq!(
            report.displaced,
            vec![(
                "VISION.md".to_string(),
                ".jigc/displaced/VISION.md".to_string()
            )],
            "the link is parked, and reported: {report:?}",
        );
        assert_eq!(
            std::fs::read_to_string(repo.path().join("VISION.md")).expect("destination"),
            managed,
            "the managed doc lands at its home as the regular file it is",
        );
        assert_eq!(
            std::fs::read_link(repo.path().join(".jigc/displaced/VISION.md"))
                .expect("the parked entry is still a link"),
            Path::new("nowhere.md"),
            "the squatter is preserved as the entry it was — moved, never followed",
        );
        assert!(
            !repo.path().join("nowhere.md").exists(),
            "and nothing was written through it",
        );
    }

    /// **The primitive refuses a destination git cannot record — and refuses it BEFORE it
    /// moves anything** (M49 completion triage, the HIGH data-loss finding).
    ///
    /// `git mv <src> .git/<dst>` prints `error: invalid path` and **exits 0**: the file
    /// moves on disk, the source leaves the index, nothing is added, and the caller —
    /// which had only the exit code to go on — reported a successful move. The doc then
    /// existed only in git history and was gone from the next clone.
    ///
    /// The guard sits on the **primitive**, not on the door that reported the loss, so the
    /// four doors that funnel through it are covered by one check even when the
    /// destination comes from a hand-edited manifest that no door adjudicated. The
    /// assertion is therefore the **index**, not the filesystem: the defect's signature was
    /// a staged deletion with no matching add.
    #[test]
    fn move_doc_refuses_a_destination_git_cannot_track_and_moves_nothing() {
        let repo = TempRepo::new();
        let body = "# A decision\n\nProse.\n";
        repo.commit_file("decisions/old.md", body);

        let jigc_root = repo.path().join(".jigc");
        let hash = hash_bytes(body.as_bytes());
        let mut seed = FileStateRecord::new();
        seed.record("decisions/old.md".to_string(), hash.clone());
        seed.save(&jigc_root).expect("seed the file-state record");

        // Both shapes git cannot record: inside its own directory, and outside the repo.
        for dest in [".git/old.md", ".git/hooks/old.md", "../escaped.md"] {
            let err = move_doc(repo.path(), &jigc_root, "decisions/old.md", dest, &hash)
                .expect_err("an untrackable destination is refused, never reported as moved");
            let rendered = format!("{err:#}");
            assert!(
                rendered.contains("git cannot track"),
                "the refusal says what it refused and why; got: {rendered}",
            );

            assert!(
                repo.path().join("decisions/old.md").exists(),
                "{dest}: the source is untouched — the refusal precedes the move",
            );
            assert!(
                !repo.path().join(dest).exists(),
                "{dest}: nothing was written at the destination either",
            );

            let record = FileStateRecord::load(&jigc_root).expect("reload the record");
            assert_eq!(
                record.get("decisions/old.md"),
                Some(hash.as_str()),
                "{dest}: the file-state baseline is not re-keyed onto a home nothing is at",
            );
        }

        let tracked = std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["ls-files"])
            .output()
            .expect("run git ls-files");
        assert!(
            String::from_utf8_lossy(&tracked.stdout)
                .lines()
                .any(|p| p == "decisions/old.md"),
            "the doc is still TRACKED — the loss was a staged deletion with no matching add",
        );
    }

    /// A location-or-placement schema fixture for the freeze-exempt relocation tests.
    fn placement_schema(ty: &str, file: &str) -> Schema {
        let yaml = format!(
            "type: {ty}\nplacement: {{ file: {file} }}\nsections:\n  - id: body\n    slot: {{ hint: x }}\n"
        );
        engine::schema::load_schema(yaml.as_bytes()).expect("placement schema loads")
    }

    /// (M39 inc-5 T4, done-criterion / Prove #1) The **parallel freeze-exempt relocation
    /// path**: a freeze-exempt doctype (`vision`) whose schema home moved to root `VISION.md`,
    /// with a pre-existing committed instance at the **human-supplied** prior home
    /// (`docs/vision/`), is **detected and routed+moved** via the T1 [`move_doc`] primitive to
    /// the current home — byte-faithful, the old-home source removed, the file-state re-keyed
    /// `from → to`. It never silently strands.
    #[test]
    fn freeze_exempt_relocation_moves_a_stranded_instance_to_the_placement_home() {
        let repo = TempRepo::new();
        let body = "---\nx: y\n---\n\n# Vision\n\nThesis.\n";
        repo.commit_file("docs/vision/vision.md", body);

        let jigc_root = repo.path().join(".jigc");
        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/vision/vision.md".to_string(),
            hash_bytes(body.as_bytes()),
        );
        seed.save(&jigc_root).expect("seed the file-state record");

        // A freeze-exempt (methodology) doctype whose home is now root `VISION.md`.
        let schema = placement_schema("vision", "VISION.md");
        let pack = crate::pack::EmbeddedPack::new();

        // The prior home is supplied by hand (no snapshot records it): the old `docs/vision/`.
        let prior = Home::location("docs/vision/").expect("prior home");
        let report = relocate_freeze_exempt(&pack, repo.path(), &jigc_root, &schema, prior)
            .expect("the freeze-exempt relocation runs");

        assert_eq!(
            report.moved,
            vec![("docs/vision/vision.md".to_string(), "VISION.md".to_string())],
            "the stranded instance is detected + moved to the current home: {report:?}",
        );
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        // Never silently stranded: the old-home source is gone, the current home is present
        // and byte-faithful (a pure relocation preserves the bytes).
        assert!(
            !repo.path().join("docs/vision/vision.md").exists(),
            "the old-home source is removed after the move",
        );
        let moved = std::fs::read_to_string(repo.path().join("VISION.md")).expect("relocated home");
        assert_eq!(
            moved, body,
            "a pure relocation preserves the bytes byte-identical"
        );

        // The file-state is re-keyed from → to: the target carries the (unchanged) hash, the
        // old-home key is dropped — no orphaned baseline entry.
        let record = FileStateRecord::load(&jigc_root).expect("reload the record");
        assert_eq!(
            record.get("VISION.md"),
            Some(hash_bytes(body.as_bytes()).as_str()),
            "the target home is baselined",
        );
        assert_eq!(
            record.get("docs/vision/vision.md"),
            None,
            "the old-home key is dropped (re-keyed, not orphaned)",
        );
    }

    /// (M39 inc-5 T4) The two paths are **reconciled**: a **frozen** doctype (`changelog`, in
    /// the dev pack's manifest set) is **refused** by the freeze-exempt path — it relocates
    /// through the version-gated `jigc migrate-corpus` (which derives its prior home from the
    /// versioned snapshot). Auto-move here would be unsafe (no snapshot to gate against).
    #[test]
    fn freeze_exempt_relocation_refuses_a_frozen_doctype() {
        let repo = TempRepo::new();
        let jigc_root = repo.path().join(".jigc");
        let schema = placement_schema("changelog", "CHANGELOG.md");
        let pack = crate::pack::EmbeddedPack::new();

        let err = relocate_freeze_exempt(
            &pack,
            repo.path(),
            &jigc_root,
            &schema,
            Home::location("changelog/").expect("prior home"),
        )
        .expect_err("a frozen doctype is refused");
        assert!(
            format!("{err:#}").contains("migrate-corpus"),
            "the refusal routes to the version-gated path: {err:#}",
        );
    }

    /// (M39 inc-5 T4) No false move: an instance **already at the current home** is not
    /// stranded, so even given a prior home the path leaves it byte-untouched (an idempotent
    /// no-op) — the discriminator moves only what is actually stranded.
    #[test]
    fn freeze_exempt_relocation_leaves_an_instance_already_at_the_current_home() {
        let repo = TempRepo::new();
        repo.commit_file("VISION.md", "# Vision\n");
        let jigc_root = repo.path().join(".jigc");
        let schema = placement_schema("vision", "VISION.md");
        let pack = crate::pack::EmbeddedPack::new();

        let report = relocate_freeze_exempt(
            &pack,
            repo.path(),
            &jigc_root,
            &schema,
            Home::location("docs/vision/").expect("prior home"),
        )
        .expect("the relocation runs");
        assert!(
            report.moved.is_empty() && report.blocked.is_empty(),
            "nothing is stranded, so nothing relocates: {report:?}",
        );
        assert!(
            repo.path().join("VISION.md").exists(),
            "the instance at the current home stays put",
        );
    }

    /// Whether git treats `rel` (repo-relative) as ignored — `git check-ignore` exits 0
    /// when the path is ignored, 1 when it is not. Used to prove the displaced squatter's
    /// workbench home is genuinely uncommittable.
    fn git_ignores(repo: &TempRepo, rel: &str) -> bool {
        std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["check-ignore", "-q", rel])
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .status()
            .expect("run git check-ignore")
            .success()
    }

    /// (M39 inc-5 T5, done-criterion / Prove #3) The **foreign-squatter move-into-workbench
    /// arm**: a foreign (untracked/unmanaged) file squatting the relocation *destination* is
    /// relocated **into the gitignored `.jigc/` workbench** — out of the destination,
    /// uncommittable — and the managed instance lands. The two-resolutions-for-one-collision
    /// reconcile (`design/reconciliation.md` → Relocation collisions): a foreign squatter →
    /// move-into-workbench (this arm); a *managed* instance at the destination → adopt-in-place
    /// (deferred move-INTO), never a clobber.
    #[test]
    fn a_foreign_squatter_at_the_destination_moves_into_the_workbench_and_the_managed_lands() {
        let repo = TempRepo::new();
        // The managed instance at the human-supplied prior home.
        let managed = "---\nx: y\n---\n\n# Vision\n\nThesis.\n";
        repo.commit_file("docs/vision/vision.md", managed);
        // A FOREIGN (untracked, unmanaged) file squatting the relocation destination.
        let foreign = "# Someone else's VISION\n\nNot a managed doc.\n";
        std::fs::write(repo.path().join("VISION.md"), foreign).expect("write the foreign squatter");

        let jigc_root = repo.path().join(".jigc");
        // The gitignore is the workbench's uncommittable guarantee — write the canonical set
        // (which now carries the `displaced/` workbench subdir) so `git check-ignore` can
        // witness it.
        crate::gitignore::ensure(&jigc_root).expect("write the canonical .jigc/.gitignore");
        // Only the managed instance is baselined; the foreign squatter is not (→ foreign).
        let mut seed = FileStateRecord::new();
        seed.record(
            "docs/vision/vision.md".to_string(),
            hash_bytes(managed.as_bytes()),
        );
        seed.save(&jigc_root).expect("seed the file-state record");

        let schema = placement_schema("vision", "VISION.md");
        let pack = crate::pack::EmbeddedPack::new();
        let prior = Home::location("docs/vision/").expect("prior home");
        let report = relocate_freeze_exempt(&pack, repo.path(), &jigc_root, &schema, prior)
            .expect("the relocation runs");

        // The managed instance is moved to the current home; the foreign squatter is displaced
        // into the workbench — no blockers.
        assert_eq!(
            report.moved,
            vec![("docs/vision/vision.md".to_string(), "VISION.md".to_string())],
            "the managed instance lands at the destination: {report:?}",
        );
        assert_eq!(
            report.displaced,
            vec![(
                "VISION.md".to_string(),
                ".jigc/displaced/VISION.md".to_string()
            )],
            "the foreign squatter is displaced into the gitignored workbench: {report:?}",
        );
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        // The managed instance now owns the destination, byte-faithful (git mv preserves it).
        let landed = std::fs::read_to_string(repo.path().join("VISION.md")).expect("destination");
        assert_eq!(landed, managed, "the managed bytes land at the destination");

        // The foreign content is preserved in the workbench, out of the destination and
        // genuinely uncommittable (gitignored under `.jigc/displaced/`).
        let parked = std::fs::read_to_string(repo.path().join(".jigc/displaced/VISION.md"))
            .expect("the displaced foreign file");
        assert_eq!(
            parked, foreign,
            "the foreign bytes are preserved in the workbench"
        );
        assert!(
            git_ignores(&repo, ".jigc/displaced/VISION.md"),
            "the displaced squatter's workbench home is gitignored — uncommittable",
        );

        // The file-state is re-keyed to the managed instance at its new home; the foreign
        // squatter is never baselined.
        let record = FileStateRecord::load(&jigc_root).expect("reload the record");
        assert_eq!(
            record.get("VISION.md"),
            Some(hash_bytes(managed.as_bytes()).as_str()),
            "the destination is baselined at the managed hash",
        );
        assert_eq!(
            record.get(".jigc/displaced/VISION.md"),
            None,
            "the displaced foreign file is never baselined",
        );
    }
}
