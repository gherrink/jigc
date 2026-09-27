//! `jigc start "<intent>"` — task minting + composition (the front door).
//!
//! The CLI reads HEAD (`git rev-parse`) and hands the engine an explicit base
//! SHA; the engine slugs the intent, opens `.jigc/tasks/<id>/`, and writes the
//! base pin (`design/write-commands.md` → Task origination; `design/storage.md`
//! → The per-task working area / base pinning). "CLI orchestrates, git executes"
//! ([storage.md](../../../design/storage.md) → CLI and git): the git invocation
//! lives here, never in the engine, so the engine stays a pure function of its
//! inputs.
//!
//! After minting, [`compose_in_repo`] composes the cascade's default workflow
//! (post-flip, the `creates-task: false` `router`; `DECISIONS.md` 2026-06-01)
//! with `{{task.intent}}` = the intent — locating the cascade
//! (CLI locates, engine resolves), running the `workflow-refs` gate at
//! compose-time, then emitting the four-class composed view
//! ([write-commands.md](../../../design/write-commands.md) → Task origination;
//! [workflow-dialect.md](../../../design/workflow-dialect.md) → The composed
//! output is a view). Composition is deterministic and makes no LLM call (same
//! resolved cascade in → same workflow out).

use crate::pack::make_pack;
use crate::repo::discover_repo_root;
use anyhow::{Context, Result, anyhow, bail};
use engine::address::Address;
use engine::cascade::{
    self, AnchorSpec, OverrideLayer, PackDefaultLayer, SlotFillDelta, SlotFillTarget,
    StructuralBasis, StructuralDelta, StructuralTarget, TrackedForkDelta,
};
use engine::compose::{
    self, CommandCatalog, ComposedWorkflow, ResolvedFills, StepDef, StepSource, StoreContext,
    WorkflowDef, apply_slot_fills, apply_structural_deltas, load_command_catalog, load_step_def,
    load_workflow_def, resolve_inline_data_values,
};
use engine::data_value::{ComposeContext, TaskRoot};
use engine::finding::{Finding, Severity};
use engine::index;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::result::CatalogEntry;
use engine::schema::Schema;
use engine::state::{self, BasePin, MintedTask, RolesRecord};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The default doc-type name minting falls back to when the intent slugs to
/// nothing. `single-task` is a `commit`-producing work-workflow, so its task-id
/// fallback is the `commit` type name (`write-commands.md` → Instance
/// provisioning: the task's commit doc).
const FALLBACK_TYPE: &str = "commit";

/// The fixed synthetic task slug the `--preview` compose feeds `build_context` in
/// place of a minted id (`compose_core`'s preview arm). It is an *identity*, not a
/// fictional real id: the preview mints nothing, so `view.task` stays `None` and the
/// render substitutes `--task your-task-id` verbatim, signalling the agent must mint
/// first (`surface-contract.md` → law 1). Chosen a valid slug so `Address::parse`
/// (`commit:your-task-id`) and every `{{task.id}}` substitution stays panic-safe.
const PREVIEW_SLUG: &str = "your-task-id";

/// The well-known commit-doc field/section ids the migration auto-provisioner fills.
/// The CLI names them by string here for the same reason
/// [`engine::write::render_commit_message`] does — the commit doctype is the one type
/// whose sink is the VCS message, a deliberate documented coupling for the single
/// VCS-sink type (`DECISIONS.md` 2026-05-31 → commit-message projection coupling).
const COMMIT_FIELD_TYPE: &str = "type";
const COMMIT_FIELD_SCOPE: &str = "scope";
const COMMIT_SLOT_SUMMARY: &str = "summary";
const COMMIT_SLOT_BODY: &str = "body";

/// The conventional-commit `type` an auto-provisioned migration commit always carries:
/// a migration is a documentation change, so it is formulaically `docs`
/// (`auto-migration.md` → Hardening #4).
const MIGRATION_COMMIT_TYPE: &str = "docs";

/// Mint a task in the repo containing `start`: read HEAD, then open the working
/// area + base pin under `<repo_root>/.jigc/`.
///
/// Repo-root discovery reuses the same `.git`-ancestor walk as cascade location
/// ([`crate::locate`]). A serial collision (an active task of the slugged id
/// already exists) surfaces as the engine's routed blocking finding, mapped here
/// to an `anyhow` error carrying that route.
pub fn mint_in_repo(
    start: &Path,
    intent: &str,
    workflow_id: &str,
    slug_override: Option<&str>,
) -> Result<MintedTask> {
    // Reject a user intent with no slug-able content up front, before minting: `slugify`
    // folds a title in another script, one of stopwords alone, and every empty /
    // whitespace / punctuation-only intent to nothing, and `mint_task`'s empty-slug
    // fallback would then mint the task under the `commit` type name — an identity nobody
    // typed, that a second such call serial-collides.
    //
    // The refusal is the **mint class's own**, not this door's private wording
    // (raised by `state::reject_unslugable_title` from the one producer private to
    // `engine::state`, M53 Increment 5 / D5): until then this bailed
    // with a code-less string saying the intent *"must contain at least one letter or
    // digit"*, which is false of `"日本語"` and gave a driver nothing to key on. The
    // work-unit type token it takes is `task` — this door's subject and, being the target,
    // this refusal's key.
    //
    // **And the question is asked, not re-spelled** (T3): the three committing mint doors
    // ask the same thing at their own seams, so the predicate moved beside the producer
    // ([`state::reject_unslugable_title`]) rather than being copied a fourth time — two
    // spellings of one condition is how the class acquired two wordings in the first place.
    state::reject_unslugable_title("task", intent, None).map_err(finding_to_err)?;
    // The `--slug` override drives the minted id **verbatim** — validate its shape at
    // this CLI boundary (never silently re-slugify a malformed value; `DECISIONS.md`
    // 2026-07-06 M39 planning → Slug (G6)). A value that is not a well-formed slug is
    // rejected with a route, mirroring `jigc rename`'s `--slug` discipline.
    if let Some(slug) = slug_override
        && !engine::slug::is_slug(slug)
    {
        bail!(
            "`--slug {slug:?}` is not a valid slug — use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"
        );
    }
    // …and the question the grammar cannot answer: a well-formed slug still has to fit one
    // filesystem path component, which the task directory below IS (M51 Inc 9 / T3, EC-28).
    // Asked here rather than discovered at `create_dir_all`, where it surfaced as
    // `task.working-area-io` routed at "a disk or permissions problem" — a law-1 lie.
    if let Some(slug) = slug_override {
        crate::task::reject_slug_over_name_ceiling(slug)?;
    }
    // The base pin is the *worktree* HEAD (code/HEAD resolve against the worktree); the
    // `.jigc/` working area binds to **jigc_home**, the main checkout, so every worktree
    // of one project shares a single `.jigc/` (M31 Inc 2 / WF3). Outside a worktree the
    // two coincide, so the mint is byte-identical.
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let jigc_root = jigc_home_or_repo(start)?.join(".jigc");
    let base = read_head(&repo_root)?;
    // The carryover gate's door half (M43 T1, `design/surface-contract.md` → The
    // carryover gate): probe the pre-task staged state before anything else touches
    // the tree, and persist it into the working area so finalize can tell "staged
    // before this task existed" from the task's own staging.
    // One `Written` member of [`state::MINT_DOORS`], the registry that says which
    // mints snapshot and which are exempt (M49 Increment 12 / T1).
    let staged = crate::task::git_staged_snapshot(&repo_root)?;

    let minted = state::mint_task(
        &jigc_root,
        intent,
        FALLBACK_TYPE,
        workflow_id,
        base,
        slug_override,
    )
    .map_err(finding_to_err)?;
    state::write_staged_snapshot(&minted.dir, &staged)
        .with_context(|| format!("could not write the staged snapshot for `{}`", minted.id))?;
    Ok(minted)
}

/// Resolve **jigc_home** — the main checkout the committed doc-store + `.jigc/` bind to
/// — from `start`, mapping a not-in-repo result to the standard routed error. Outside a
/// worktree this is the byte-identical walk-up root [`discover_repo_root`] returns (M31
/// Inc 2 / WF3); inside a linked worktree it redirects to the main checkout.
pub(crate) fn jigc_home_or_repo(start: &Path) -> Result<PathBuf> {
    crate::repo::jigc_home(start).ok_or_else(|| crate::locate::not_in_repo(start))
}

/// Resolve the project cascade layer dir (`<jigc_home>/.jigc/config`) from `start`,
/// bailing with the routed setup prompt when the project layer is absent. The `.jigc/`
/// layer binds to **jigc_home** (the main checkout), so a worktree resolves the one
/// shared project layer rather than its own (absent) `.jigc/` (M31 Inc 2 / WF3).
pub(crate) fn require_project_config(start: &Path) -> Result<PathBuf> {
    let project_config = jigc_home_or_repo(start)?.join(".jigc").join("config");
    if !project_config.is_dir() {
        return Err(crate::locate::not_set_up(&project_config));
    }
    Ok(project_config)
}

/// Mint an **off-router migration task** under `repo_root`: read HEAD, open the
/// working area, and pin the base — mirroring [`mint_in_repo`] but with an **empty
/// intent** (so the id falls back to the mint id-source) and a caller-supplied
/// `workflow_id` (the off-router `migrate-<doctype>` workflow). It
/// then **auto-provisions the task's commit doc *filled*** (`auto-migration.md` →
/// Hardening #4): a migration commit is mechanical, so the CLI fills it rather than
/// blocking finalize on empty author-required commit fields. `type`/`scope` are filled
/// as **field** values (`docs` / `doctype`) — the same deterministic CLI materialization
/// the doc-level `default:`/`set: on-create` seed performs, no boundary crossing — and
/// `summary`/`body` as **slot** prose from a CLI template that is a *pure deterministic
/// function of (`source_path` + `doctype`)*, never judgment (the bounded slot-fill rule,
/// `DECISIONS.md` S2). The agent then authors only the canonical doc and never touches
/// the commit form.
///
/// The task id is **per-file** — `migrate-<doctype>-<slug(source_path)>-<hash>`, **not**
/// the singleton `migrate-<doctype>`: a corpus of N foreign docs migrates sequentially
/// (each file mints its own task), where the fixed `migrate-<doctype>` would serial-
/// collide on the second file. It is also **not** the bare `doctype` name: the bare
/// name collides with both `--task <doctype>` (resuming a `doctype`-slugged task) and
/// the `doctype` id-space itself, so a migration task named after its doctype would
/// block an independent task at that name (`auto-migration.md` → Hardening #9). The
/// [`migration_task_id`] result is fed **verbatim** as `mint_task`'s `slug_override`
/// (bypassing the mint's re-slugify cap), so its `blake3(source-path)` disambiguator
/// survives to keep the id stable and collision-free for every path.
///
/// The `jigc migrate` verb owns this mint so composition never double-mints; it stages
/// the foreign source separately and then composes via [`compose_minted_in_repo`].
pub(crate) fn mint_migration_in_repo(
    repo_root: &Path,
    doctype: &str,
    workflow_id: &str,
    source_path: &str,
) -> Result<MintedTask> {
    // The migrate path (mint → stage → compose) stays uniformly on the caller's
    // worktree `repo_root`; threading it to jigc_home is the deferred worktree-migrate
    // concern (M31 Inc 2 binds the start/resume/reenter + task/finalize read paths).
    let jigc_root = repo_root.join(".jigc");
    let base = read_head(repo_root)?;
    // The carryover gate's door half (M43 T1): snapshot the pre-task staged state —
    // `jigc migrate` is a task-minting door like `start` (the foreign bytes are staged
    // into the gitignored working area, never the git index, so the probe sees only
    // genuinely foreign pre-staged changes).
    // One `Written` member of [`state::MINT_DOORS`].
    let staged = crate::task::git_staged_snapshot(repo_root)?;
    // The per-file migration id carries a `blake3(source-path)` disambiguator (fork 1),
    // so it is passed **verbatim** via the `slug_override` bypass — never re-slugified by
    // the mint (whose `MAX_WORDS`/`MAX_CHARS` cap would truncate the tail and re-collide
    // two long paths sharing their first-`MAX_WORDS` slug window). Empty intent still,
    // so resume re-composes with no stray `{{task.intent}}`; the bare `<doctype>` task
    // namespace stays free and the migration task stays per-file.
    let id = migration_task_id(doctype, source_path);
    let minted = state::mint_task(&jigc_root, "", doctype, workflow_id, base, Some(&id))
        .map_err(finding_to_err)?;
    state::write_staged_snapshot(&minted.dir, &staged)
        .with_context(|| format!("could not write the staged snapshot for `{}`", minted.id))?;
    let pack = make_pack()?;
    // The commit form is provisioned from the **resolved** commit schema — the same
    // resolution every read surface projects, so a project shadow cannot leave the
    // migration's form contradicting `jigc doc schema commit`.
    let project_config = jigc_root.join("config");
    let resolved = resolve_severity_cascade(pack.as_ref(), &project_config)?;
    let defs = CascadeDefs::new(&resolved, &project_config);
    provision_migration_commit_doc(
        pack.as_ref(),
        &defs,
        &minted.dir,
        &minted.id,
        source_path,
        doctype,
    )?;
    Ok(minted)
}

/// Open the working area of a `jigc task amend` task and mark it as one — [`amend_in_repo`]'s
/// mint half, and the [`state::MINT_DOORS`] site the source fence keys on.
///
/// It is its own function for the reason the registry's `site` field exists: the fence
/// matches the **enclosing** function of every production `mint_task` call, so a mint buried
/// inside a door that also composes would declare a site naming the whole door. Here the
/// three writes that make an area an amend area — the pin, the marker, the snapshot — sit in
/// one function that does nothing else, the shape [`mint_in_repo`] and
/// [`mint_migration_in_repo`] already take.
///
/// The area binds to **jigc_home** (the main checkout) while `base` was read from the
/// caller's worktree, exactly as at `jigc start`: one `.jigc/` per project, and the commit
/// the amend rewrites is the one the caller is standing on.
fn mint_amend_in_repo(
    start: &Path,
    repo_root: &Path,
    intent: &str,
    slug_override: Option<&str>,
    base: &BasePin,
) -> Result<MintedTask> {
    let jigc_root = jigc_home_or_repo(start)?.join(".jigc");
    // One `Written` member of [`state::MINT_DOORS`] — the pre-task staged state, snapshotted
    // before anything is written, on `mint_in_repo`'s mold. The amend arm does not *gate* on
    // it (its own dirty-index refusal is stricter and unconditional), but the snapshot is
    // what every other consumer of a task area reads, so an area without one would be the
    // odd member rather than the exempt one.
    let staged = crate::task::git_staged_snapshot(repo_root)?;
    let minted = state::mint_task(
        &jigc_root,
        intent,
        FALLBACK_TYPE,
        AMEND_WORKFLOW,
        base.clone(),
        slug_override,
    )
    .map_err(finding_to_err)?;
    // The marker is written **immediately**, before anything else about the area is true, so
    // no window exists in which it reads as an ordinary task `jigc task finalize` would
    // commit normally.
    state::write_amend_pin(&minted.dir, &base.sha)
        .with_context(|| format!("could not mark `{}` as an amend task", minted.id))?;
    state::write_staged_snapshot(&minted.dir, &staged)
        .with_context(|| format!("could not write the staged snapshot for `{}`", minted.id))?;
    Ok(minted)
}

/// The workflow a `jigc task amend` task is minted from — the dev pack's `amend`
/// (F-10). It is **verb-routed** (`suppressed.door: jigc task amend`), so `jigc start
/// --workflow amend` refuses it and this is the only door that composes it.
pub(crate) const AMEND_WORKFLOW: &str = "amend";

/// The commit at HEAD is not one an amend can rewrite — `amend.head-shape`, refused at
/// `jigc task amend` **before anything mints** (F-10; `design/finalize.md` → The amend arm).
///
/// The axis is *what shape is HEAD*, and it has four answers, not two: **unborn** (there is
/// no commit), **root** (no parent), **merge** (two or more), and the ordinary single-parent
/// commit that passes. The first three are refused here rather than at finalize, because a
/// mint that cannot land is a task the agent must then discard — and the shape of HEAD is
/// knowable at the mint, unlike the index state finalize gates on.
///
/// **Root is refused although `git commit --amend` would accept it.** A root commit is the
/// repository's first, and the settle's call is that jigc does not rewrite it on an agent's
/// behalf — the route hands the act back with the command that performs it, which is what a
/// `Human` route is for. A merge commit gets no such hand-back: its message is git's own
/// merge narration, and re-authoring it as a Conventional-Commits subject would replace a
/// structural message with a prose one, the same class the composed step names.
///
/// `id_hint` is **the id the mint would have taken** — the caller's slugged intent where there
/// is one, the commit-derived `amend-<sha7>` fallback where there is not. It was the fallback
/// unconditionally, so a refusal over a supplied intent addressed a work unit no invocation
/// would ever have created (the F-10 review's LOW-5); the caller now resolves it after the
/// intent is adjudicated, which is the only position where it can be right.
///
/// **Where this locus lands on the wire, so the repair is not mistaken for one** (M53, the last
/// pre-1.0.0 batch's review, LOW 2). The refusal leaves this door through
/// [`finding_to_err`], the **flattened** `{"error": …}` carrier, so under `--format json` the
/// code, the `at:` locus and the route ride inside one string rather than as keys:
/// `amend.head-shape` is no [`crate::render::ENVELOPE_OWED_CODES`] member, and the M53 batch's
/// change was to the *spelling inside that message* — it moved nothing about which arm answers.
/// That is a **recorded residual, not a covered case**, because the arm's own stated rule
/// ([`crate::render::ENVELOPE_ARMS`]' `Reject::Error` row) keeps the refusals whose code the
/// contract lists under **no** target form, and the work-unit ref `task:<id>` is a form the
/// contract does list. It stays flattened on the same measured ground M50's completion audit
/// declined the analogous envelope fix on: **nothing was minted**, so the `(code, target)` pair
/// would name a work unit that exists nowhere and resolves at no read surface — strictly less
/// for a driver than the message that says so in as many words. The condition that reopens it is
/// a target this door can key that *does* resolve — the repository's own HEAD, were a caller
/// shown to need the pair rather than the sentence.
fn head_shape_refusal(repo_root: &Path, id_hint: &str) -> Option<Finding> {
    let (shape, route) = if crate::task::head_is_unborn(repo_root).unwrap_or(false) {
        (
            String::from("unborn — this repository has no commit yet"),
            engine::finding::Route::human(
                "there is no commit to repair; make one first — `jigc start \"<intent>\"` \
                 composes the ordinary task that lands it",
            ),
        )
    } else {
        // `--parents` prints `<sha> <parent>…`, so the parent count is the token count less
        // the commit's own sha. Asked of HEAD alone (`-n 1`), never of a walk.
        let line = git_rev_parse(repo_root, &["rev-list", "--parents", "-n", "1", "HEAD"]).ok()?;
        match line.split_whitespace().count().saturating_sub(1) {
            0 => (
                String::from("a root commit — it has no parent"),
                engine::finding::Route::human(
                    "a repository's first commit is not rewritten on your behalf; amend it \
                     yourself with `git commit --amend` if that is what you want",
                ),
            ),
            1 => return None,
            n => (
                format!("a merge commit — it has {n} parents"),
                engine::finding::Route::human(
                    "a merge commit's message is git's own narration of the merge, not a \
                     Conventional-Commits subject; leave it, or redo the merge with the \
                     message you want",
                ),
            ),
        }
    };
    Some(Finding::graded(
        Severity::Blocking,
        "amend.head-shape",
        format!(
            "`jigc task amend` rewrites a single-parent commit, and HEAD is {shape} — nothing \
             was minted"
        ),
        // **`task:<id>`, the declared spelling** (M53, the rc.20 per-axis review
        // `(5, DEFECT 2 · rc.20)`). It shipped as `work-unit:<id>` — a third work-unit
        // address form that no grammar in the product declares: `structural-grammar.md`
        // enumerates `type:name` (`task:…`, `milestone:…`) and
        // `command-output-contract.md` the work-unit ref `task:<id>` / `milestone:<id>`,
        // and nothing resolves the third. The rationale it shipped with — that `task:<id>`
        // would name a task that does not exist — is answered by the message standing one
        // line above it, which says *nothing was minted* in as many words; and neither
        // spelling resolves at a read surface, so the invented one bought no honesty and
        // cost the only two homes that enumerate the form.
        Some(engine::finding::Location::addressed(
            format!("task:{id_hint}"),
            1,
            1,
        )),
        Some(route),
    ))
}

/// Mint a `jigc task amend` task in `start`'s repository and compose the `amend` workflow
/// over it (F-10) — the whole door, `Neither` on [`crate::cli::BEHALF_DOORS`]: it writes the
/// working area and composes text, and every byte it moves in git is
/// `jigc task finalize <id>`'s.
///
/// **The order, and why it is this one.** The pack loads first, then the caller's own intent
/// is adjudicated ([`state::reject_unslugable_title`]), then HEAD's shape
/// ([`head_shape_refusal`]), then the mint. Every gate precedes the mint so a
/// refused amend strands no task directory — the discipline `compose_named_in_repo` states
/// for its own verb-routed rejection.
///
/// **The intent precedes the shape probe** (the F-10 review's LOW-5), for two reasons that
/// point the same way: a token the caller typed is this door's to reject before it reports a
/// fact about the repository, and the shape refusal's locus *is* the id the mint would have
/// taken — which is knowable only once the intent has been accepted. Both cells refuse at the
/// same exit and mint nothing, so the swap changes which sentence a doubly-bad call gets and
/// nothing else.
///
/// And the pack precedes both so this door
/// answers the way every other door answers over a corpus jigc cannot adjudicate: the freeze
/// and ref-target fences block *everywhere*, and a door that reported a local fact about
/// HEAD instead would be the one place an operator learned something other than what is
/// actually wrong. Driven by `ref_target_fence` and `freeze_enforcement`, whose fixtures
/// disagree on HEAD's shape and would otherwise disagree on this door's answer.
///
/// The task id comes from the caller's `intent` when there is one (slugged like every other
/// task's) and otherwise from the commit itself — `amend-<sha7>`, fed as `mint_task`'s
/// `slug_override` so it is used verbatim. That fallback is what makes the intent optional:
/// an amend's *subject* is a commit, so the door can always name the task after it, where
/// `jigc start` has nothing to fall back to and must reject an unslugable intent.
///
/// The area is an ordinary task area plus one file — the [`state::AMEND_PIN_FILE`] marker,
/// written **immediately** after the mint, so no window exists in which the area reads as an
/// ordinary task that `jigc task finalize` would commit normally.
pub(crate) fn amend_in_repo(start: &Path, intent: Option<&str>) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    // **The pack loads before anything else this door decides.** Driven: with provisioning as
    // the first pack read, a corpus whose freeze had drifted got its task area minted and
    // *then* the refusal — a door that blocks and leaves a working area behind, which
    // `freeze_enforcement`'s own claim about a blocked door ("it refuses and acts on
    // nothing") is false of.
    let pack = make_pack()?;
    let resolved = resolve_severity_cascade(pack.as_ref(), &project_config)?;
    let defs = CascadeDefs::new(&resolved, &project_config);

    let base = read_head(&repo_root)?;
    let fallback_id = format!("amend-{}", base.short);

    // **The caller's own token is adjudicated before any fact about the repository** — and
    // that order is what makes the shape refusal's locus honest (the F-10 review's LOW-5).
    //
    // **The fallback covers the ABSENT intent, never a present one that says nothing.** The
    // branch is on `Option`, not on emptiness, and the difference is the whole degenerate-id
    // class: keyed on `trim().is_empty()`, `jigc task amend "   "` minted `amend-<sha7>` at
    // exit 0 — a caller who typed something and got a task named after something else. A
    // present intent goes to the mint class's own producer, exactly as at `jigc start`, so
    // `"   "`, `"日本語"` and `"the"` are each answered *unusable* rather than silently
    // renamed (`flow54_acceptance`'s arm 5, which is where this was driven).
    let (intent, slug_override) = match intent {
        None => ("", Some(fallback_id.clone())),
        Some(text) => {
            // **The one door with a second exit, and it says so** (M53, the rc.20 per-axis
            // review `(3, F-C)`). `state::MINT_DOORS` spells this row
            // `jigc task amend ["<intent>"]` — the brackets are the fact: omit the intent
            // and the mint names the task after the commit it rewrites. The shared advice
            // (*re-run with a title carrying ASCII letters or digits*) is complete at every
            // other row and incomplete here, so the exit rides the producer's own route
            // instead of the class acquiring a second wording at this door.
            state::reject_unslugable_title(
                "task",
                text,
                Some(
                    "omit the title — `jigc task amend` names the task after the commit \
                      it rewrites (`amend-<sha7>`)",
                ),
            )
            .map_err(finding_to_err)?;
            (text, None)
        }
    };

    // The id the mint **would** take, which is what `amend.head-shape`'s locus must be: the
    // override verbatim where there is one, and otherwise `slugify(intent)` — reached only
    // once the refusal above has cleared, so the slug is non-empty by construction and this
    // never has to guess. Shipped, the locus was `amend-<sha7>` unconditionally, so a refused
    // `jigc task amend "root probe"` pointed at `work-unit:amend-b219d05`, a work unit no
    // invocation would ever have created — while the function's own doc-comment claimed
    // otherwise.
    let id_hint = slug_override
        .clone()
        .unwrap_or_else(|| engine::slug::slugify(intent));
    if let Some(finding) = head_shape_refusal(&repo_root, &id_hint) {
        return Err(finding_to_err(finding));
    }

    let minted = mint_amend_in_repo(start, &repo_root, intent, slug_override.as_deref(), &base)?;
    provision_commit_doc(pack.as_ref(), &defs, &minted.dir, &minted.id)?;
    drop(pack);

    let composed = compose_minted_in_repo(
        &repo_root,
        &project_config,
        &minted.dir,
        &minted.id,
        AMEND_WORKFLOW,
        intent,
        None,
    )?;
    let composed = with_also_open(composed, start)?;
    Ok(Composition {
        // Read through the **same** producer the two re-compose doors read it through
        // ([`amend_target_of`], off the marker this mint just wrote), so the mint ack and
        // every later re-compose cannot describe one commit two ways (the F-10 review's
        // MEDIUM-2). The marker holds the sha `base` pinned, so this is the same commit
        // `head_shape_refusal` adjudicated one statement above.
        //
        // The checkout rides here and nowhere else (the F-10 review's LOW-7): this is the
        // invocation that read that `HEAD`, and `repo_root` is where it stood.
        amend: amend_target_of(
            &repo_root,
            &minted.dir,
            crate::render::CommitSite::differing(&jigc_home_or_repo(start)?, &repo_root),
        )?,
        ..composed
    })
}

/// Derive the **per-file** migration task **id** from `(doctype, source_path)` — a pure,
/// path-aware function whose result is the minted task id **verbatim** (fed to
/// [`state::mint_task`] as `slug_override`, never re-slugified).
///
/// The id is `migrate-<doctype>-<slug>-<hash>` (or `migrate-<doctype>-<hash>` when the
/// path slugs away entirely), where:
///
/// - `<slug>` folds the **repo-relative source path** (extension stripped, path
///   separators folded to `-`, then slugified — its `MAX_WORDS`/`MAX_CHARS` cap bounds
///   the id to a filesystem-safe length) in for **legibility**; and
/// - `<hash>` is a 12-hex-char prefix of `blake3(repo-relative source path)`, folded in
///   **unconditionally** (fork 1) for **identity**: it makes the id **deterministic**
///   (same path → same id, so re-migrating a file collides into the serial-collision
///   resume/discard route rather than double-minting — `mint_task` hard-rejects a serial
///   collision, never suffixes) and **collision-free** (distinct paths → distinct ids).
///
/// The hash is what disambiguates *every* path, not just the empty-slug case: two long
/// paths sharing their first-`MAX_WORDS` slug window collapse to one id under the mint's
/// re-slugify cap, so the legible `<slug>` alone cannot separate them. Passing the built
/// id verbatim via `slug_override` (bypassing that cap) is what lets the hash survive —
/// so `mint_migration_in_repo` must feed this as the override, never as the slugged
/// id-source (`auto-migration.md` → Hardening #9; `design/storage.md` → Identity).
///
/// This is an id-**construction** fold upstream of `slugify`, not a change to `slugify`
/// itself, so it is **not** a `SLUG_RULE_VERSION` change and ships no corpus migration
/// (`design/storage.md` → Identity — the slug-rule one-way door does not reach this
/// `start.rs` fold).
fn migration_task_id(doctype: &str, source_path: &str) -> String {
    // The disambiguator: a 12-hex-char prefix of blake3(repo-relative source path).
    // Distinct paths → distinct ids; the same path → the same id.
    let hash = engine::file_state::hash_bytes(source_path.as_bytes());
    let disambiguator = &hash[..12];
    // The legible slug portion: strip the extension and fold path separators to '-' so
    // the directory survives (slugify would otherwise drop a bare '/'), then slugify —
    // whose own MAX_WORDS/MAX_CHARS cap bounds the slug portion to a NAME_MAX-safe length.
    let stem = Path::new(source_path).with_extension("");
    let folded: String = stem
        .to_string_lossy()
        .chars()
        .map(|c| if matches!(c, '/' | '\\') { '-' } else { c })
        .collect();
    let slug = engine::slug::slugify(&folded);
    if slug.is_empty() {
        // The whole path slugged away (all non-Latin, e.g. `日本語.md`): the id is just
        // the prefix + disambiguator — distinct empty-slug paths still get distinct ids,
        // never collapsing to the bare `migrate-<doctype>` singleton.
        format!("migrate-{doctype}-{disambiguator}")
    } else {
        format!("migrate-{doctype}-{slug}-{disambiguator}")
    }
}

/// Provision the task's workflow-provisioned **commit** doc into the working
/// area as a **fillable form**: the empty skeleton with every schema header field
/// pre-stamped as an empty `key:` line, materialized at
/// `<task_dir>/docs/commit:<id>.md`. This is the deterministic structural act the
/// workflow owns — the agent then fills the `summary`/`body` slots (`set-slot`)
/// and splices the `type`/`scope` fields (`set-field`) into the present lines.
///
/// The header-field lines are pre-stamped because the engine's `set-field` is a
/// **surgical splice** of a present value line ([`engine::write::set_field`]):
/// front-matter has no generate-a-new-line path (the header is not a generatable
/// body section), so the form must carry the line for the agent to fill. (Body
/// slots provision empty and `set-slot` generates the section's home on demand —
/// `write-commands.md` → Instance provisioning; `DECISIONS.md` 2026-05-31 → inc-4
/// fillable-form provisioning.)
fn provision_commit_doc(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
    dir: &Path,
    id: &str,
) -> Result<()> {
    let schema = load_commit_schema(pack, defs)?;
    let instance = fillable_form(&schema, id);
    persist_provisioned_commit(&schema, dir, id, &instance)
}

/// Auto-provision the **migration** task's commit doc into the working area already
/// **filled** — Hardening #4's no-author finalize. Unlike [`provision_commit_doc`]'s
/// empty fillable form, the migration form carries the formulaic `docs`/`doctype` header
/// fields plus a templated `summary`/`body` (a pure deterministic function of
/// `source_path` and `doctype`), so the migration finalizes without the agent authoring
/// the commit doc (`auto-migration.md` → Hardening #4; `DECISIONS.md` S2 → bounded fill).
fn provision_migration_commit_doc(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
    dir: &Path,
    id: &str,
    source_path: &str,
    doctype: &str,
) -> Result<()> {
    let schema = load_commit_schema(pack, defs)?;
    let instance = migration_commit_form(&schema, id, source_path, doctype);
    persist_provisioned_commit(&schema, dir, id, &instance)
}

/// Load the **cascade-resolved** `commit` schema — the one read both commit-doc
/// provisioners share.
///
/// Resolved, not pack-read: `commit` is transient (no home to relocate), so a project
/// shadow of it changes *shape*, and the form provisioned here is the only surface that
/// can ever carry a header field — front-matter has no generate-a-line path, so a field
/// missing from the form is a field the agent cannot fill. Read pack-only, this door
/// handed out a form `jigc doc schema commit` contradicted.
fn load_commit_schema(pack: &dyn PackSource, defs: &CascadeDefs<'_>) -> Result<Schema> {
    defs.schema(pack, FALLBACK_TYPE)
        .with_context(|| format!("the `{FALLBACK_TYPE}` schema is malformed"))
}

/// Render `instance` to the task's `docs/commit:<id>.md` and record the commit doc as
/// `created` in the manifest the by-task-id join consumes — the persist tail both
/// provisioners share. A commit doc must appear in the manifest, and it is collision-safe
/// by construction (`commit:<id>` is task-id-derived, unique per area, so it never reaches
/// the join's same-slug suffix/clash rules) (`write-commands.md` → copy-on-first-touch).
/// Write-once: a re-entry never flips it.
fn persist_provisioned_commit(
    schema: &Schema,
    dir: &Path,
    id: &str,
    instance: &engine::write::Instance,
) -> Result<()> {
    let path = state::instance_path(dir, &schema.ty, id);
    let rendered = engine::write::render(schema, instance);
    state::persist(&path, rendered.as_bytes())
        .with_context(|| format!("could not provision the commit doc for `{id}`"))?;
    let address = format!("{}:{id}", schema.ty);
    state::record_doc_provenance(dir, &address, engine::state::Provenance::Created)
        .with_context(|| format!("could not record provenance for the commit doc `{address}`"))?;
    Ok(())
}

/// The filled migration commit instance: the [`fillable_form`] skeleton with the
/// `type`/`scope` header fields materialized (`docs` / `doctype`) and the
/// `summary`/`body` slots filled from the deterministic migration template. Every filled
/// value is a pure function of (`source_path`, `doctype`) — structural facts, never
/// judgment (the bounded slot-fill rule).
fn migration_commit_form(
    schema: &Schema,
    id: &str,
    source_path: &str,
    doctype: &str,
) -> engine::write::Instance {
    use engine::field_block::Value;
    let mut instance = fillable_form(schema, id);
    for section in &mut instance.sections {
        for field in &mut section.fields {
            match field.key.as_str() {
                COMMIT_FIELD_TYPE => {
                    field.value = Value::Scalar(MIGRATION_COMMIT_TYPE.to_string());
                }
                COMMIT_FIELD_SCOPE => field.value = Value::Scalar(doctype.to_string()),
                _ => {}
            }
        }
        match section.id.as_str() {
            COMMIT_SLOT_SUMMARY => {
                section.slot = Some(migration_commit_summary(source_path, doctype));
            }
            COMMIT_SLOT_BODY => section.slot = Some(migration_commit_body(source_path, doctype)),
            _ => {}
        }
    }
    instance
}

/// The templated commit **subject** prose — a pure deterministic function of the source
/// path + doctype (the bounded slot-fill rule). Rendered into `docs(<doctype>): <this>`.
fn migration_commit_summary(source_path: &str, doctype: &str) -> String {
    format!("adopt {source_path} as a managed {doctype}")
}

/// The templated commit **body** prose — a pure deterministic function of the source path
/// + doctype (the bounded slot-fill rule).
fn migration_commit_body(source_path: &str, doctype: &str) -> String {
    format!(
        "Migrate the foreign {source_path} into the managed {doctype} document and retire the original."
    )
}

/// Whether a workflow provisions a commit doc — the **single source of truth** for the
/// provisioning property, consumed at both call sites (the mint arm in [`compose_core`]
/// and [`provision_on_first_entry`]). A workflow provisions a commit doc iff it
/// `creates-task: true` — every task that can reach a commit boundary owns one, and a
/// `creates-task: false` `<W>` (the router and its kind) provisions nothing.
///
/// **`selectable` is deliberately NOT a provisioning gate.** `selectable` governs only
/// router *membership* (the `selectable_workflows` catalog filter) — a
/// `creates-task: true, selectable: false` workflow (the fan-out `sub-task`) still owns
/// and authors its own commit doc, it just never appears in a router's selection list
/// (`workflow-dialect.md` → the `selectable` contract: a non-selectable work-workflow
/// reaches its commit boundary only through the parent milestone's `finalize`, but it is
/// a commit-bearing task all the same).
fn should_provision_commit_doc(def: &WorkflowDef) -> bool {
    def.creates_task
}

/// Whether `type_name` is the doctype a task composed on `def` **provisions for itself**
/// — the task's commit doc, which *bypasses* the create-gate rather than being granted by
/// it (`write-commands.md` → The create-gate: "Workflow-provisioned instances … bypass the
/// gate"). Read by the absent-instance refusal, which must not name `jigc doc create` as
/// the provisioning act for a doctype no gate grants (M46 Inc 8 T3, B2-1).
///
/// Both conjuncts are the facts the provisioning call sites already key on — the commit
/// doctype [`FALLBACK_TYPE`] both provisioners load their schema by, and
/// [`should_provision_commit_doc`], the single source of truth for the property — so this
/// is a read of the provisioning rule, never a second copy of it.
pub(crate) fn provisions_at_compose(def: &WorkflowDef, type_name: &str) -> bool {
    type_name == FALLBACK_TYPE && should_provision_commit_doc(def)
}

/// Provision the sub-workflow's deterministic commit doc into the sub-task's
/// working area **on first re-entry only** — `jigc workflow <W> --task <id>`'s
/// deferred mirror of `jigc start`'s mint-time provisioning (`write-commands.md` →
/// Sub-agent re-entry: the first re-entry provisions the sub-workflow's deterministic
/// instances, deferred to first entry so unspawned areas aren't provisioned).
///
/// **Gated on [`should_provision_commit_doc`]** (`creates_task`) — a `creates-task: false`
/// `<W>` (the router and its kind) provisions nothing, mirroring [`compose_core`]'s
/// no-task arm; a `creates-task: true, selectable: false` `<W>` (the fan-out `sub-task`)
/// **does** provision (`selectable` is router-membership, not a provisioning gate — see
/// [`should_provision_commit_doc`]). **Idempotent / first-entry-only** — keyed on the
/// **absence** of the `docs/commit:<id>.md` skeleton (the mint-time write-once), so a
/// sub-agent's in-progress edits survive a later re-entry; the doc is provisioned exactly
/// once, on the first entry that finds it missing.
fn provision_on_first_entry(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
    def: &WorkflowDef,
    dir: &Path,
    id: &str,
) -> Result<()> {
    if !should_provision_commit_doc(def) {
        return Ok(());
    }
    // The skeleton's presence marks the area already-provisioned: a re-entry never
    // overwrites it, so an agent's in-progress edits are preserved.
    let path = state::instance_path(dir, FALLBACK_TYPE, id);
    if path.exists() {
        return Ok(());
    }
    provision_commit_doc(pack, defs, dir, id)
}

/// Build the **fillable** empty instance for `schema`: the H1 title is the task id,
/// every body section is content-free, and the header section's declared fields are
/// pre-stamped as empty `key:` lines (the surface `set-field` splices). The empty
/// value is `Value::Scalar("")`, which renders the canonical `key:` line.
///
/// **Exception — an optional `ref` is not pre-stamped.** An optional ref (forward
/// `card:` minimum 0, e.g. `commit.implements`'s `0..1`) carries no author
/// obligation: a spec-less task leaves it unset, so a pre-stamped empty `key:` line
/// would survive to finalize as a malformed-empty value + a dangling edge (both
/// blocking) — yet the design mandates a spec-less `commit` finalizes cleanly
/// without it (`implementation/doctype-map.md` → `commit —implements→ spec`, `0..1`).
/// So an optional ref is **absent** in the fillable form, exactly as `validate.rs`'s
/// `required-field-present` exempts it — through the engine's own
/// [`engine::validate::is_optional_ref`] predicate (M40 F1 retired the local
/// mirror), so provisioning and validation cannot drift. The spec-driven workflow
/// that *does* set it materializes the line when it lands (a later M3 increment),
/// not here.
///
/// Note the rule here is deliberately **all header fields minus optional refs** —
/// broader than the engine skeleton's author-required-only stamp
/// (`engine::state`'s M40 create skeleton): the commit form also stamps
/// `optional:` fields like `scope:` as fill-me levers, and that byte shape is
/// locked by the migration-commit golden, so the predicate's *home* moved without
/// changing the stamping rule.
fn fillable_form(schema: &engine::schema::Schema, slug: &str) -> engine::write::Instance {
    use engine::field_block::{Field, Value};
    use engine::schema::SectionBody;
    use engine::write::SectionContent;

    let sections = schema
        .sections
        .iter()
        .map(|section| {
            let fields = if section.header {
                match &section.body {
                    SectionBody::Simple { fields, .. } => fields
                        .iter()
                        .filter(|f| !engine::validate::is_optional_ref(f))
                        .map(|f| Field {
                            key: f.id.clone(),
                            value: Value::Scalar(String::new()),
                        })
                        .collect(),
                    SectionBody::Repeatable { .. } => Vec::new(),
                }
            } else {
                Vec::new()
            };
            SectionContent {
                id: section.id.clone(),
                fields,
                ..Default::default()
            }
        })
        .collect();
    engine::write::Instance {
        title: slug.to_string(),
        sections,
    }
}

/// The cascade knob the default-workflow id is read from
/// (`design/overrides.md`; `design/write-commands.md` → Task origination: the
/// `default-workflow: <id>` cascade knob the front door composes). Post-flip the
/// pack ships `default-workflow: router` in its `config/defaults` resource
/// (`DECISIONS.md` 2026-06-01 → M2 flips `default-workflow` to `router`).
const DEFAULT_WORKFLOW_KEY: &str = "default-workflow";

/// The doctype a **vision-forming** workflow admits through its create-gate — the
/// trigger for the empty-research advisory ([`resolve_research_advisory`]).
const VISION_DOCTYPE: &str = "vision";

/// The [`committed_store`] key (the `research/` location stem) under which committed
/// `research` instances are enumerated — the collection the empty-research advisory
/// queries. Absent ⇔ no research is committed (`committed_store` omits empty
/// collections), so a vision grounds in `0..*` research and zero is a legal state.
const RESEARCH_STORE_KEY: &str = "research";

/// The HTML-comment marker lines delimiting the empty-research advisory block the
/// `author-vision` step carries at its opening (`packs/methodology/steps/author-vision.yaml`).
/// The CLI keeps the inner advisory line when the compose warrants it, else drops the
/// whole block; the markers are **always** stripped, so they never reach the agent.
const ADVISORY_OPEN: &str = "<!-- research-advisory -->";
const ADVISORY_CLOSE: &str = "<!-- /research-advisory -->";

/// Whether the composing workflow warrants the empty-research advisory: it creates a
/// `vision` (its `allows-create` gate admits the [`VISION_DOCTYPE`]) **and** no
/// `research` is committed. A `vision` grounds in the research it cites, so an empty
/// research store earns a nudge toward `do-research` first — **advisory, never
/// blocking** (some visions ground in experience, and grounding may take several
/// research rounds; `ideas/form-vision-research-routing.md`, `DECISIONS.md`
/// 2026-07-06 → form-vision advisory G7).
fn warrants_research_advisory(def: &WorkflowDef, store: &BTreeMap<String, Vec<Address>>) -> bool {
    def.allows_create
        .iter()
        .any(|entry| entry.doc_type == VISION_DOCTYPE)
        && !store.contains_key(RESEARCH_STORE_KEY)
}

/// Resolve the empty-research advisory block a vision-forming workflow's
/// `author-vision` step carries: when `advise`, keep its inner advisory line (drop the
/// two markers); otherwise drop the whole marker block — plus a single blank line that
/// immediately follows it, so no leading gap remains. A composed text carrying no
/// advisory block (every non-vision workflow) is returned unchanged. The markers are
/// **always** removed, so they never leak into the emitted bytes on any compose path.
fn resolve_research_advisory(text: String, advise: bool) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let (Some(open), Some(close)) = (
        lines.iter().position(|l| l.trim() == ADVISORY_OPEN),
        lines.iter().position(|l| l.trim() == ADVISORY_CLOSE),
    ) else {
        return text; // no advisory block — nothing to resolve.
    };
    if close <= open {
        return text; // malformed block ordering — leave the bytes untouched.
    }
    let trailing_newline = text.ends_with('\n');
    let mut out: Vec<&str> = lines[..open].to_vec();
    if advise {
        out.extend_from_slice(&lines[open + 1..close]);
    }
    // Resume past the close marker; when dropping the block, also drop a single
    // trailing blank line so the following prose does not start with a gap.
    let mut resume = close + 1;
    if !advise && lines.get(resume).is_some_and(|l| l.trim().is_empty()) {
        resume += 1;
    }
    out.extend_from_slice(&lines[resume..]);
    let mut result = out.join("\n");
    if trailing_newline {
        result.push('\n');
    }
    result
}

/// A composed workflow **plus** the CLI-side presentation facts the pinned composed-output
/// JSON contract does not carry (M42) — the composing workflow's **create-gates**, and
/// whether *this* invocation **minted** the task it composed over.
///
/// The contract is exactly `{task, text}` (`design/command-output-contract.md` §1), so
/// neither fact can ride the engine's [`ComposedWorkflow`] (every field of which projects
/// into that JSON). They ride here instead, next to it: the CLI renderer appends the
/// `task minted:` header, the task-state lines (`resume:` / `what's-left:` /
/// `task scope:`, keyed on the id in [`view.task`](ComposedWorkflow::task) — M43 Inc 7),
/// and the `create-gates:` line to agent/human text only — the same frontend-appended
/// mold as the routing footer — and projects the [`ComposedWorkflow`] alone on
/// `--format json`.
#[derive(Debug)]
pub struct Composition {
    /// The composed view — the pinned `{task, text}` JSON projection, verbatim.
    pub view: ComposedWorkflow,
    /// The doctypes the composing workflow's `allows-create` grants, in declaration order
    /// (the `create-gates:` presentation line). Empty for a gate-less workflow
    /// (`quick-fix`, a `creates-task: false` router) — which renders no line at all.
    pub gates: Vec<String>,
    /// Whether **this invocation** minted [`view.task`](ComposedWorkflow::task) — the fact
    /// the `task minted: <id>` header states. It is *not* `view.task.is_some()`: a resume
    /// / sub-agent re-entry carries the **given** id (minted by an earlier invocation), and
    /// a header there would announce a mint that never happened. `false` on every
    /// re-compose and on the `creates-task: false` (router) arm, which mints nothing at all.
    pub minted: bool,
    /// The **milestone sub-task** facts, when [`view.task`](ComposedWorkflow::task) is one
    /// — the unit-kind discriminator the `task scope:` footer scopes its parallel-work
    /// claim on (M47 Inc 8 / N7) and the `resume:` line keys its door on (M51 Inc 9 / T1).
    /// `None` for a top-level task and for an id-less compose, which is what every
    /// *minting* door produces: a task joins a milestone only through `jigc milestone
    /// add-task`, never at the mint that composes.
    pub sub_task_of: Option<SubTaskOf>,
    /// The tasks that were **already open** when a work-starting `jigc start` form ran —
    /// every live task under the project's `.jigc/` home except the one *this* invocation
    /// minted (M50 Increment 5 / T3, the trial's F-5). The `also open:` presentation block
    /// the renderer appends to agent/human text only; the pinned `{task, text}` JSON is
    /// untouched.
    ///
    /// Filled at exactly the two forms that **start work from an intent**
    /// ([`compose_in_repo`] and [`compose_named_in_repo`]) — the doors an agent reaches
    /// for when it does not know what is in flight, and the pair the Settle's B8
    /// disposed. Every other construction site leaves it empty, so a resume, a sub-agent
    /// re-entry, a `--preview`, a `jigc migrate` compose and the milestone feed all render
    /// **no bytes**: the omitting context stays inert.
    ///
    /// It **suppresses nothing**. The mint proceeds exactly as before — `render.rs`'s own
    /// `task scope:` line composes *"several open tasks are legal … you can run them in
    /// parallel"* into every task, so refusing the second mint here would contradict the
    /// same binary's own surface (Settle B8).
    pub also_open: Vec<OpenTask>,
    /// The commit an **amend** task was minted against — the `amending:` presentation block
    /// (F-10). `None` for every other compose, which renders no bytes at all: the omitting
    /// context stays inert, the [`also_open`](Composition::also_open) mold.
    ///
    /// It rides here rather than on the engine's composed view for that field's reason — the
    /// composed-output JSON is pinned at exactly `{task, text}`
    /// (`design/command-output-contract.md` §1), so the mint ack's facts are presentation and
    /// add no key to the contract.
    pub amend: Option<AmendTarget>,
}

/// The commit a `jigc task amend` mint pinned — what the `amending:` block names.
///
/// **The subject is read once per compose and carried, never re-read at the render**: it is
/// the one fact the ack exists to show (*this is the message you are about to replace*), and
/// a second `git log` a moment later would answer about whatever HEAD is by then.
///
/// **Every compose of an amend task carries it, not only the mint** (the F-10 review's
/// MEDIUM-2). The mint reads it from `HEAD`; a **re-compose** ([`amend_target_of`], reached
/// by `jigc start --task <id>` and by `jigc workflow amend --task <id>`) reads it from the
/// area's own `amend` marker, which holds the pinned sha — the authority on *which commit
/// this task is repairing*, where `HEAD` is only where the repository happens to stand.
#[derive(Debug)]
pub struct AmendTarget {
    /// The abbreviated sha of the commit the amend rewrites — the human handle.
    pub short: String,
    /// That commit's current subject line, verbatim. Empty when the commit's message has
    /// no subject at all, which the block then says rather than printing empty quotes.
    pub subject: String,
    /// The checkout whose `HEAD` this amend pinned, when that is **not** the one the `.jigc/`
    /// workbench binds to — `None` on the ordinary path, where they are the same directory
    /// (the F-10 review's LOW-7).
    ///
    /// The settle's row *"the ack names the checkout when it is not the workbench's"* was
    /// driven **not real**: from a linked worktree the mint ack named no checkout, and the
    /// `committed / would commit in the linked worktree at …` line appeared only on the
    /// forecast and the landed ack. It is made real here rather than struck, because the fact
    /// is this door's own: a reader standing in a linked worktree is about to have *that*
    /// worktree's `HEAD` rewritten, and every task-roster surface they reached the door
    /// through belongs to the main checkout.
    ///
    /// Same producer as the finalize surfaces ([`crate::render::CommitSite::differing`]),
    /// different sentence: those speak in a commit tense, and this door has committed nothing
    /// — it has read a `HEAD`.
    pub checkout: Option<crate::render::CommitSite>,
}

/// The [`AmendTarget`] a **re-compose** of `task_dir` carries — `None` for every ordinary
/// task, which renders no `amending:` block at all (the F-10 review's MEDIUM-2).
///
/// Driven at `48d1d529`: `jigc start --task <amend-id>` and `jigc workflow amend --task
/// <amend-id>` both composed the `amend` step body — whose own prose pointed at *"HEAD's
/// subject line in the ack above"* — with no ack above it, on the one path five consecutive
/// trials show an agent taking when context is lost. The block is the settle's **only**
/// mitigation for the cell it deliberately refuses to refuse (*jigc cannot tell which door
/// made `HEAD`*), so a surface that drops it drops the whole mitigation.
///
/// **The subject is read from the pinned sha, never from `HEAD`.** They are the same commit
/// on every composable state — a moved `HEAD` is `finalize.base-mismatch` at the commit arm
/// — and where they differ the pin is the honest answer: it is the commit this task will
/// rewrite or die refusing to.
///
/// **A pin git cannot describe renders no block**, rather than a fabricated short sha: the
/// commit was `HEAD` at the mint and stays reachable through the reflog, so this is the
/// gc'd-and-expired cell, and in it the finalize arm refuses anyway. Naming a commit that is
/// not there would be exactly the law-1 lie the block exists to prevent.
///
/// `checkout` is [`AmendTarget::checkout`], and only the **mint** passes one: which checkout's
/// `HEAD` was pinned is a fact about the invocation that pinned it, and the marker records the
/// sha alone. A re-compose passes `None` rather than comparing *its own* cwd, which would put a
/// different question's answer under the same sentence — and the surfaces that then act (the
/// forecast, the landed ack) name their own checkout from their own position.
fn amend_target_of(
    repo_root: &Path,
    task_dir: &Path,
    checkout: Option<crate::render::CommitSite>,
) -> Result<Option<AmendTarget>> {
    let Some(sha) = state::read_amend_pin(task_dir)
        .with_context(|| format!("could not read the amend marker at {task_dir:?}"))?
    else {
        return Ok(None);
    };
    let Ok(short) = git_rev_parse(repo_root, &["rev-parse", "--short", &sha]) else {
        return Ok(None);
    };
    // Reached only once `rev-parse` has answered for this object, so an empty string here is
    // the genuine no-subject-line commit the block renders as such.
    let subject = git_rev_parse(repo_root, &["log", "-1", "--format=%s", &sha]).unwrap_or_default();
    Ok(Some(AmendTarget {
        short,
        subject,
        checkout,
    }))
}

/// One row of [`Composition::also_open`] — a task that was live before this call.
///
/// Two facts only: the **id** (the handle every subsequent door takes) and the
/// **workflow** that minted it (the answer to *what is that task*). The intent is not
/// carried: the id is slugged from it, so the row would restate it, and a composed
/// surface pays for every line it prints.
#[derive(Debug)]
pub struct OpenTask {
    /// The task id — the `--task <id>` handle, and what `jigc start --task <id>` resumes.
    pub id: String,
    /// The workflow the task was minted from, read from its working area. [`None`] for a
    /// task minted before that file existed, or whose area will not answer — the row is
    /// still named, because a task you cannot see is worse than a workflow you cannot read.
    pub workflow: Option<String>,
}

/// The facts a **milestone sub-task**'s re-compose carries that a top-level task's does
/// not ([`Composition::sub_task_of`]) — both read at the one shared re-compose spine
/// ([`compose_task_workflow`]), so the two doors a sub-task can reach (resume and
/// sub-agent re-entry) can never disagree about them.
///
/// They are carried **together** because the two surfaces keyed on this discriminator
/// need one each: the `task scope:` line names the milestone whose finalize is the
/// sub-task's only commit boundary, and the `resume:` line names the re-entry door
/// `jigc workflow <W> --task <id>`, which is the door that provisions the sub-task's
/// write-ready area ([`provision_on_first_entry`]). A single `Option<String>` milestone
/// id would have left the second line reaching for a workflow id the renderer does not
/// have, and the shipped line named `jigc start --task <id>` instead — a door that
/// re-composes and provisions nothing, which is what M51 Increment 9 / T1 repairs
/// (`design/surface-contract.md` → law 1).
#[derive(Debug)]
pub struct SubTaskOf {
    /// The id of the milestone that owns the task — the unit whose `jigc milestone
    /// finalize <m>` is the sub-task's only commit boundary.
    pub milestone: String,
    /// The workflow the re-compose composed, which for both doors **is** the sub-task's
    /// recorded minting workflow: resume reads it out of the working area, and re-entry
    /// is equality-guarded against it ([`reenter_in_repo`]'s W-equality guard). So it is
    /// the `<W>` the re-entry door accepts, never a second guess at it.
    pub workflow: String,
    /// **Where this compose was run from** — the posture the `resume:` line's *where*
    /// clause is scoped on (M52 Inc 10 / T4, the per-axis review's A6-1). Decided at the
    /// one shared re-compose spine ([`compose_task_workflow`]), so the two doors a
    /// sub-task can reach can never disagree about it either.
    pub posture: SubTaskPosture,
}

/// Which checkout a **milestone sub-task**'s re-compose was run from
/// ([`SubTaskOf::posture`]) — the discriminator the `resume:` line's *where* clause
/// needs, and the only thing about a sub-task compose that the invocation's own
/// location decides.
///
/// Driven at the M52 baseline (`completions/artifacts/M52/baseline-surfaces.md` §2.1):
/// both provisioned postures composed **byte-identical** bytes, so the shipped line
/// told a reader already standing in the sub-task's worktree to go where it already
/// was (`design/surface-contract.md` → law 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubTaskPosture {
    /// The invocation ran **inside the sub-task's own worktree** — the checkout
    /// `jigc milestone provision` cut for it at `.jigc/worktrees/<id>`, where its work
    /// happens.
    OwnWorktree,
    /// The invocation ran **somewhere else in the project** — the shared checkout
    /// standing at the milestone's pin (the only other posture that composes at exit 0;
    /// off the pin the door refuses before composing), or another sub-task's worktree.
    /// Either way, not the worktree this sub-task's work belongs in. The worktree the
    /// line then names is written **project-root-relative**, as it has been since M51
    /// Inc 9 / T1 — followable from the shared checkout, and from a sibling worktree
    /// readable only against the project root, which this row narrows but does not fix.
    ElsewhereInProject,
}

/// The composing workflow's create-gate doctypes, in declaration order — the source of the
/// `create-gates:` line and the same list the `create.gate-blocked` refusal enumerates
/// (`design/write-commands.md` → The create-gate), so the announcement and the refusal can
/// never disagree.
fn create_gates(def: &WorkflowDef) -> Vec<String> {
    def.allows_create
        .iter()
        .map(|entry| entry.doc_type.clone())
        .collect()
}

/// Mint a task from `intent`, then compose the cascade's default workflow over
/// it — the `jigc start "<intent>"` front door.
///
/// Locates the cascade (a missing project layer routes to `jigc setup`), resolves
/// `default-workflow` **through the cascade** (so a project `scalar:` delta
/// flips it — [`resolve_cascade`] + [`cascade::Resolved::scalar_required`]), mints
/// the task (seq 12),
/// builds the [`ComposeContext`] binding `{{task.intent}}`/`{{task.id}}` and
/// declaring the workflow's `allows-create` roles unbound, runs the
/// `workflow-refs` gate at compose-time, and on a clean gate emits the
/// deterministic four-class composed view. A blocking gate finding short-circuits
/// with that finding's message + route — nothing is emitted past the gate.
///
/// Returns the composed view; the caller renders it through the selected format.
pub fn compose_in_repo(
    start: &Path,
    intent: &str,
    slug_override: Option<&str>,
) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    // Resolve the cascade once: phase-3 scalars (the `default-workflow` read) +
    // phase-2 file owners + the manifest's phase-4 structural deltas (`overrides.md`
    // → Resolution algorithm). The same `Resolved` threads into the layer-aware
    // step source so a project step shadows the pack's.
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    let workflow_id = resolved
        .scalar_required(DEFAULT_WORKFLOW_KEY)
        .map(str::to_owned)
        .map_err(anyhow::Error::from)?;
    let source = CascadeStepSource::new(pack, &resolved, &project_config);
    let composed = compose_drained(
        &repo_root,
        intent,
        pack,
        &workflow_id,
        &source,
        &overrides,
        MilestoneFeed::none(),
        None,
        slug_override,
        false,
    )?;
    with_also_open(composed, start)
}

/// Attach the **already-open** set to a composition produced by one of the two
/// work-starting `jigc start` forms (M50 Increment 5 / T3).
///
/// Enumerated **after** the compose, from
/// [`state::list_active_task_ids`](engine::state::list_active_task_ids) — the single
/// enumeration source of truth `jigc task list`, the active-task resolution and bare-`start`
/// orientation all read, so no two doors disagree about which tasks are live — minus
/// [`ComposedWorkflow::task`], the id *this* invocation just minted. Filtering by that id
/// rather than by a pre-compose snapshot is what makes the freshly minted task unnameable
/// to itself by construction, on the minting arm and on the `creates-task: false` router
/// arm (where nothing was minted, so nothing is filtered) alike.
///
/// The jigc home is resolved through the same seam the caller's
/// [`require_project_config`] already went through, so the `?` here cannot fire after that
/// call succeeded — no degraded arm renders an unknown set as an empty one. A working area
/// whose `workflow` file will not answer degrades to [`None`] on that row alone; the task
/// is still named.
fn with_also_open(mut composed: Composition, start: &Path) -> Result<Composition> {
    let jigc_root = jigc_home_or_repo(start)?.join(".jigc");
    let minted = composed.view.task.clone();
    composed.also_open = engine::state::list_active_task_ids(&jigc_root)
        .into_iter()
        .filter(|id| Some(id) != minted.as_ref())
        .map(|id| {
            let workflow = engine::state::read_workflow_id(&jigc_root.join("tasks").join(&id))
                .ok()
                .flatten();
            OpenTask { id, workflow }
        })
        .collect();
    Ok(composed)
}

/// The finding code a composing door raises for a workflow the pack declares
/// **verb-routed** — reached through its own `jigc` verb, never composed by name
/// (`design/validation.md` → The M52 registrations — Increment 9).
pub const VERB_ROUTED_CODE: &str = "workflow.verb-routed";

/// The **named-workflow definition read** both `--workflow <X>` doors and
/// `jigc workflow <X> --preview` perform — with the verb-routed refusal in front of it.
///
/// A workflow whose `suppressed:` block declares a `door:` composes correctly **only**
/// through that door, because the door is what binds the input the body reads: `jigc
/// migrate <path> --as adr` stages the foreign source the twelve `migrate-*` bodies
/// rewrite, `jigc milestone execute <id>` binds the milestone whose sub-tasks the
/// fan-out spawns, and `jigc workflow sub-task --task <id>` names the sub-area the
/// body writes into. Composed by name from here, all three render a body whose
/// instructions point at nothing — driven at rc.15, `jigc start --workflow migrate-adr
/// "x"` minted a task and printed *"Below is the foreign source the CLI staged for
/// you"* over three blank lines, and the walk it opened landed a commit its own text
/// said was impossible (`completions/artifacts/M52/baseline-surfaces.md` §2.2).
///
/// So the declaration becomes a refusal: **blocking, exit 1**, keyed at the pack
/// resource (`workflow:<id>` — the pack-resource target form) and routed at the
/// declared door itself, argv-for-argv, so the reader's next command is the one that
/// works. The `reason` prose rides the message verbatim — it is the pack's own
/// statement of *why* the composition would be degenerate, and `describe --workflows`
/// already shows it one surface over.
///
/// **Where it does not live, and why.** Not in [`compose_drained`] / [`compose_core`]:
/// `jigc milestone execute` and every resume compose through those, and the resume of
/// a `sub-task`-minted task is exactly a composition of a verb-routed workflow that
/// *is* reachable — its door already ran. The check binds at the three doors that take
/// a workflow **by name from the caller**, which is the act that has no door behind it.
/// The cascade-default front door ([`compose_in_repo`]) is out of scope by decision
/// (settle-record D9's declared bound): its workflow comes from the `default-workflow`
/// knob, not from an argv the caller typed.
fn read_named_workflow(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
    workflow_id: &str,
) -> Result<WorkflowDef> {
    let def = load_workflow_def(&defs.read_workflow(pack, workflow_id)?).map_err(finding_to_err)?;
    if let Some(suppressed) = def.suppressed.as_ref()
        && let Some(door) = suppressed.door.as_deref()
    {
        let argv = engine::compose::Suppressed::door_argv(door);
        // **Belt and braces, in both postures.** `Route::mechanical`'s own parse check is a
        // debug-only *panic*, so a door that reaches this line without having passed the
        // pack-load fence ([`crate::pack`] → `assert_workflow_doors` /
        // `assert_project_workflow_doors`) would crash a debug build at exit 101 and emit an
        // unrunnable route in release. Driven at `c6b45da0`, that is exactly what a
        // project-layer workflow shadow did, because the parse fence's subject was the
        // manifest-shipping origin packs alone. The fence's subject is now every layer a
        // definition can be served from — and this door asks the same question again anyway,
        // in **every** posture, because a producer that cannot prove its argv parses must not
        // construct the route: the refusal is a blocking finding naming the workflow and the
        // layer that declared the door, never a panic and never a route that does not run.
        if !crate::route_fence::accepts(&argv) {
            let layer = defs
                .origin_pack_id(pack, PackResourceKind::Workflows, workflow_id)
                .map_or_else(
                    || format!("the project layer's `.jigc/config/workflows/{workflow_id}.yaml`"),
                    |pack_id| format!("pack `{pack_id}`"),
                );
            return Err(finding_to_err(Finding::block(
                engine::compose::SUPPRESSED_MALFORMED_CODE,
                format!(
                    "workflow `{workflow_id}` ({layer}) declares `suppressed.door: {door}`, \
                     which does not parse against the real CLI — so the door it names is a \
                     command that cannot run"
                ),
                format!(
                    "correct the `door:` in `{workflow_id}`'s front-matter to the command line \
                     the workflow is actually reached through"
                ),
            )));
        }
        return Err(crate::render::envelope_finding_error(&Finding::graded(
            Severity::Blocking,
            VERB_ROUTED_CODE,
            format!(
                "workflow `{workflow_id}` is not composed by name: {}",
                suppressed.reason,
            ),
            Some(engine::finding::Location::addressed(
                format!("workflow:{workflow_id}"),
                1,
                1,
            )),
            Some(engine::finding::Route::mechanical(argv, "")),
        )));
    }
    Ok(def)
}

/// Compose the workflow named by `workflow_id` from `intent` — the explicit
/// `jigc start --workflow <X> "<intent>"` front door (Form D, `write-commands.md`
/// → Task origination). Unlike [`compose_in_repo`] it bypasses the cascade
/// `default-workflow` knob and composes the *named* workflow, minting iff `X`
/// declares `creates-task: true`; an unknown `<X>` — or a **verb-routed** one, which
/// this door cannot bind the input for ([`read_named_workflow`]) — is rejected with a
/// routed finding before any mint. The cascade-presence guard (a missing project layer
/// routes to `jigc setup`) is identical to the default front door.
pub fn compose_named_in_repo(
    start: &Path,
    intent: &str,
    workflow_id: &str,
    slug_override: Option<&str>,
) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    // Form D bypasses the cascade `default-workflow` knob but still resolves the
    // cascade for phase-2 file owners + the phase-4/5 deltas, so a project step
    // override or slot-fill applies to a `--workflow <X>`-composed workflow too.
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    // The verb-routed refusal, before anything mints: a `<X>` whose `suppressed:`
    // block declares a `door:` is reached through that door, never composed by name
    // ([`read_named_workflow`]). It precedes the compose for the same reason the
    // unknown-`<X>` rejection does — a refused name must strand no task dir.
    read_named_workflow(
        pack,
        &CascadeDefs::new(&resolved, &project_config),
        workflow_id,
    )?;
    let source = CascadeStepSource::new(pack, &resolved, &project_config);
    let composed = compose_drained(
        &repo_root,
        intent,
        pack,
        workflow_id,
        &source,
        &overrides,
        MilestoneFeed::none(),
        None,
        slug_override,
        false,
    )?;
    with_also_open(composed, start)
}

/// Compose the explicitly-named `workflow_id` with **no `<intent>` positional** —
/// the `jigc start --workflow <X>` form with no intent. Branches on the workflow's
/// `creates-task` declaration (read through the cascade, the same read the compose
/// path uses to decide minting — never a hardcode):
///
/// - A **`creates-task: true`** `<X>` slugs its task id *from the intent*
///   (`design/write-commands.md` → Task origination), so it cannot mint without one.
///   It is **rejected** here with an actionable message naming the workflow and the
///   corrected intent-bearing form — mirroring `start`'s other reject discipline
///   (unknown `<X>`, slug collision) rather than silently falling through to the
///   read-only orientation listing.
/// - A **`creates-task: false`** `<X>` (the router, `ingest-existing`,
///   `milestone-execution`) composes with **no mint** — the intent threads nowhere
///   on that arm — so it composes here over an empty intent, exactly as the
///   intent-bearing Form D would.
///
/// An unknown `<X>` surfaces its routed not-found finding from the cascade read,
/// before any branch — the same rejection the intent-bearing Form D gives.
pub fn compose_named_no_intent_in_repo(start: &Path, workflow_id: &str) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    // Read the named workflow's `creates-task` declaration through the cascade — the
    // same definition read `compose_core` performs to decide minting — so the gate
    // below keys off the real declaration. An unknown `<X>` is rejected here (the
    // `read_workflow` membership check maps `NotFound` to a routed finding), and a
    // verb-routed `<X>` ahead of the `creates-task` branch, so a `creates-task: false`
    // member is routed at its verb rather than at this door.
    let def = read_named_workflow(
        pack,
        &CascadeDefs::new(&resolved, &project_config),
        workflow_id,
    )?;
    if def.creates_task {
        bail!(
            "workflow '{workflow_id}' requires an intent: jigc start \"<intent>\" --workflow {workflow_id}"
        );
    }

    let source = CascadeStepSource::new(pack, &resolved, &project_config);
    compose_drained(
        &repo_root,
        "",
        pack,
        workflow_id,
        &source,
        &overrides,
        MilestoneFeed::none(),
        None,
        // No mint on the `creates-task: false` arm (a `creates-task: true` `<X>` was
        // rejected above), so a `--slug` override could never apply here.
        None,
        false,
    )
}

/// Preview the explicitly-named `workflow_id` — the `jigc workflow <id> --preview`
/// read surface (`design/surface-contract.md` → law 2: nothing hides; the
/// read-surface spine of `introspection.md` / `doc-read-surface.md`). It composes a
/// **`creates-task: true`** workflow's step text **without minting a task**, so a
/// mutation-cautious agent can read what a work-minting workflow will ask *before*
/// consenting to mint. The composed bytes are byte-identical to a real mint's with
/// the minted id substituted by [`PREVIEW_SLUG`] — the [`compose_core`] preview arm
/// skips `mint_in_repo` + `provision_commit_doc` and feeds the fixed synthetic slug,
/// leaving `view.task = None` and `minted = false`.
///
/// Branches on the workflow's `creates-task` declaration (read through the cascade,
/// the same read the compose path uses), mirroring
/// [`compose_named_no_intent_in_repo`]'s reject discipline in the **opposite**
/// direction: a **`creates-task: false`** `<X>` (the router and its kind) mints
/// nothing to begin with, so `jigc start --workflow <X>` already composes it without
/// a mint — there is nothing to preview, and it is **rejected** here with that route.
/// An unknown `<X>` surfaces its routed not-found finding from the cascade read, and a
/// **verb-routed** `<X>` its own refusal ([`read_named_workflow`]) — both before the
/// branch, which is why a verb-routed `creates-task: false` member is now routed at
/// its verb instead of at the compose door that could only compose it degenerately.
// The `jigc workflow <id> --preview` dispatch (`cli.rs` → `run_preview`) calls this
// (Inc 3 T2, the `--preview` surface + render).
pub fn preview_in_repo(start: &Path, workflow_id: &str) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    // Read the named workflow's `creates-task` declaration through the cascade — the
    // same definition read `compose_core` performs to decide minting. An unknown
    // `<X>` is rejected here (the `read_workflow` membership check maps `NotFound` to
    // a routed finding), and a verb-routed `<X>` ahead of the `creates-task` branch —
    // which is what stops this door routing INTO the compose that cannot bind it.
    let def = read_named_workflow(
        pack,
        &CascadeDefs::new(&resolved, &project_config),
        workflow_id,
    )?;
    if !def.creates_task {
        bail!(
            "workflow '{workflow_id}' mints no task, so there is nothing to preview — run it directly: jigc start --workflow {workflow_id}"
        );
    }

    let source = CascadeStepSource::new(pack, &resolved, &project_config);
    // A preview never mints, so it needs no intent to slug and no `--slug` override;
    // `{{task.intent}}` resolves to empty and `--task {{task.id}}` renders the
    // synthetic identity.
    compose_drained(
        &repo_root,
        "",
        pack,
        workflow_id,
        &source,
        &overrides,
        MilestoneFeed::none(),
        None,
        None,
        true,
    )
}

/// The milestone state one compose is fed — the **bound** work-unit's id and its
/// id-sorted sub-task list, the two engine-native `milestone` leaves
/// ([`engine::data_value::ComposeContext::milestone_id`] / `::milestone`).
///
/// They travel as one value because they are one fact: the single door that binds a
/// milestone (`jigc milestone execute <id>`) binds both, and every other compose
/// passes [`MilestoneFeed::none`] — where `{{milestone.tasks}}` resolves to the empty
/// collection and `milestone.id` resolves absent, so a catalog arg sourcing it keeps
/// its `<MILESTONE_ID>` agent marker (`command-catalog.md` → The three arg kinds).
/// The CLI does the work-unit I/O; the resolver does none (the determinism boundary).
#[derive(Clone, Copy, Default)]
pub struct MilestoneFeed<'a> {
    /// The bound milestone's id — `None` on every compose off the execute door.
    pub id: Option<&'a str>,
    /// The milestone's live sub-tasks, each id paired with its recorded minting
    /// workflow; empty when no milestone is bound.
    pub tasks: &'a [engine::data_value::SubTask],
}

impl<'a> MilestoneFeed<'a> {
    /// The unbound feed — no milestone id, no sub-tasks.
    pub fn none() -> Self {
        Self::default()
    }

    /// The bound feed — the executing milestone's id and its live sub-task list.
    pub fn bound(id: &'a str, tasks: &'a [engine::data_value::SubTask]) -> Self {
        Self {
            id: Some(id),
            tasks,
        }
    }
}

/// Execute a milestone work-unit — compose the **explicitly-named**
/// `creates-task: false` `workflow_id` over the bound milestone's id + its id-sorted
/// sub-task list, feeding `milestone.tasks` so the workflow's `fan-out`
/// step resolves it (one `` Spawn: `jigc workflow <W> --task <id>` `` per sub-task, in
/// id-sorted order, `<W>` being **that sub-task's recorded minting workflow** — the CLI
/// reads it and feeds the pair; the resolver does no work-unit I/O), and `milestone.id`
/// so the walk's three `` Run: `jigc milestone …` `` lines carry the real id rather than
/// an agent-fill marker (M49). **Mints nothing**
/// — the milestone and its sub-tasks already
/// exist (`write-commands.md` → Executing the milestone — `jigc milestone execute
/// <id>`). The lone production site feeding a **bound** [`MilestoneFeed`];
/// `milestone.rs` resolves the work-unit + reads `TaskList::enumerate()` and calls
/// in. Bypasses the cascade `default-workflow` knob (the workflow is named, like
/// Form D) but still resolves the cascade for phase-2/4/5 owners.
pub fn execute_milestone_in_repo(
    start: &Path,
    workflow_id: &str,
    milestone: MilestoneFeed<'_>,
) -> Result<Composition> {
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    let source = CascadeStepSource::new(pack, &resolved, &project_config);
    // No `intent` — a milestone execution mints no task, so `{{task.intent}}` is
    // never resolved (the workflow is `creates-task: false`); pass empty.
    compose_drained(
        &repo_root,
        "",
        pack,
        workflow_id,
        &source,
        &overrides,
        milestone,
        None,
        // A milestone execution mints no top-level task, so no `--slug` applies.
        None,
        false,
    )
}

/// Compose an **already-minted** off-router task's workflow — the read/compose half every
/// verb-routed minting door drives after it has minted the area itself. Mints nothing, so
/// the off-router task is not re-minted and never double-provisioned.
///
/// Two doors reach it, and the parameters are exactly what they differ on:
///
///  - **`jigc migrate <path> --as <doctype>`** (`auto-migration.md` → The `jigc migrate`
///    verb / The source seam) passes an empty `intent` and `Some(foreign)` — the staged
///    foreign bytes **fed into [`ComposeContext::source`]**, so the composed workflow's
///    `{{source}}` placeholder surfaces them verbatim.
///  - **`jigc task amend`** (F-10) passes the caller's intent and `None`: its workflow
///    reads no seam, and its `{{task.intent}}` is a real authoring intent.
///
/// Composition feeds the seam off the in-memory bytes the verb just staged; the engine
/// resolver does **no** file I/O for the seam (the determinism boundary — the CLI owns
/// the read). The committed store + edge overlay are wired exactly as a resume, so a
/// `{{@…}}` deref in either workflow would resolve.
pub(crate) fn compose_minted_in_repo(
    repo_root: &Path,
    project_config: &Path,
    task_dir: &Path,
    task_id: &str,
    workflow_id: &str,
    intent: &str,
    seam: Option<&str>,
) -> Result<Composition> {
    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (resolved, overrides) = resolve_cascade(pack, project_config)?;
    let defs = CascadeDefs::new(&resolved, project_config);
    let workflow_bytes = defs.read_workflow(pack, workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;

    let origin = pack.origin_pack(PackResourceKind::Workflows, &ResourceId::from(workflow_id));
    let commands = load_catalog(origin)?;
    let selectable = selectable_workflows(pack, &defs)?;
    let schemas = defs.all_schemas(pack)?;
    let store_feed = committed_store(repo_root, &schemas);

    // The bound roles are read from the just-minted task (empty until the agent creates a
    // doc through a create-gate).
    let bound = RolesRecord::load(task_dir)
        .with_context(|| format!("could not read roles for `{task_id}`"))?;
    let ctx = build_context(
        task_id, intent, &def, &bound, selectable, store_feed, seam, schemas,
    );

    let stepsource = CascadeStepSource::new(pack, &resolved, project_config);
    stepsource.scope_to_workflow(workflow_id);
    let findings = compose::workflow_refs_with_fills(
        &workflow_bytes,
        &[],
        &overrides.slot_fills,
        &overrides.fills,
        &stepsource,
        &commands,
        &ctx,
    );
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    let filled = FillStepSource {
        inner: &stepsource,
        fills: &overrides.fills,
        ctx: &ctx,
    };
    let result = compose::compose(&def, &filled, &commands, &ctx).map_err(finding_to_err);
    if result.is_err()
        && let Some(located) = stepsource.take_error()
    {
        return Err(finding_to_err(located));
    }
    // The verb already minted the off-router task, so its id is the in-hand handle
    // surfaced in `--format json` (`command-output-contract.md` §1). The workflow's own
    // create-gates ride alongside as the `create-gates:` presentation line — the migration
    // workflow's is the doctype it migrates *into*; `amend` declares none, so that line
    // renders no bytes.
    result.map(|composed| Composition {
        view: ComposedWorkflow {
            task: Some(task_id.to_string()),
            ..composed
        },
        gates: create_gates(&def),
        // The calling verb minted this task in *this* invocation (just above the compose),
        // so the announcement is true here exactly as on the front door.
        minted: true,
        // A just-minted task belongs to no milestone (membership is `add-task`'s alone).
        sub_task_of: None,
        // Left empty here and filled by the door that wants it: `jigc migrate` is not a
        // work-starting form and renders no `also open:` bytes, while `jigc task amend` is
        // one and wraps this in [`with_also_open`].
        also_open: Vec::new(),
        // Likewise the amend target: the door that knows the commit attaches it.
        amend: None,
    })
}

/// The milestone-feeding compose seam — compose the no-task `workflow_id` over an
/// **injected** `pack` + `source` with the [`MilestoneFeed`] fed into the `milestone` root,
/// no cascade overrides (the structural-only shape `compose_core`'s no-override path
/// reads the pack unchanged). The `pub(crate)` boundary lets the `milestone.rs`
/// dispatch test drive the feeding contract over a `FixturePack` fan-out workflow
/// (the idiom the `start.rs` compose tests use) without the embedded pack or a real
/// milestone-execution workflow (T3). The production [`execute_milestone_in_repo`]
/// reaches the same `compose_core` no-task arm through the live cascade source — so
/// this seam is a `#[cfg(test)]` test affordance, not a production code path.
#[cfg(test)]
pub(crate) fn execute_milestone_core(
    repo_root: &Path,
    pack: &dyn PackSource,
    workflow_id: &str,
    source: &dyn StepSource,
    milestone: MilestoneFeed<'_>,
) -> Result<Composition> {
    let overrides = ComposeOverrides {
        deltas: Vec::new(),
        slot_fills: Vec::new(),
        fills: ResolvedFills::new(),
    };
    // A no-shadow cascade — the injected `source` is the test affordance, and the
    // milestone-dispatch test feeds a fan-out workflow over a `FixturePack` with no
    // project definition shadows, so the definition reads fall through to the pack.
    let base = cascade::PackDefaultLayer::new("dev", "0.0.0", BTreeMap::new(), Vec::new());
    let resolved = cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");
    let defs = CascadeDefs::new(&resolved, repo_root);
    compose_core(
        repo_root,
        "",
        pack,
        workflow_id,
        source,
        &defs,
        &overrides,
        milestone,
        None,
        None,
        false,
    )
}

/// Run [`compose_core`] over the layer-aware [`CascadeStepSource`], then prefer the
/// source's drained **located** fault over a generic compose error: a project step
/// the cascade owns but whose file is missing/malformed records a located finding
/// in the source's sink (the `Option<StepDef>` contract can't carry it), so a
/// failed compose surfaces that precise fault rather than the engine's generic
/// dangling-include message (`CascadeStepSource` doc → Located-error sink).
#[allow(clippy::too_many_arguments)]
fn compose_drained(
    repo_root: &Path,
    intent: &str,
    pack: &dyn PackSource,
    workflow_id: &str,
    source: &CascadeStepSource<'_>,
    overrides: &ComposeOverrides,
    milestone: MilestoneFeed<'_>,
    seam: Option<&str>,
    slug_override: Option<&str>,
    preview: bool,
) -> Result<Composition> {
    // Scope the step source's pack-default arm to the composing workflow's origin
    // pack, so its `{{include: step:X}}` resolves against the pack that *defines*
    // the workflow — the pack-local body-reference rule (`multi-pack.md` →
    // Pack-local body-reference resolution → Steps). Inert for a single pack
    // (origin = pack), so the cold-start floor is byte-identical.
    source.scope_to_workflow(workflow_id);
    let result = compose_core(
        repo_root,
        intent,
        pack,
        workflow_id,
        source,
        &source.defs(),
        overrides,
        milestone,
        seam,
        slug_override,
        preview,
    );
    if result.is_err()
        && let Some(located) = source.take_error()
    {
        return Err(finding_to_err(located));
    }
    result
}

/// Compose the workflow named by `workflow_id` from `intent`, branching on the
/// workflow's `creates-task` flag — the engine spine both front-door forms drive
/// (the cascade-default `jigc start "<intent>"` passes the cascade-resolved
/// `default-workflow`; the explicit `--workflow <X>` form passes `X`),
/// with the pack injected so the no-task arm is reachable under test.
///
/// An unknown `workflow_id` — one the pack does not provide — is **rejected with a
/// routed finding before any mint** (`write-commands.md` → Form D): the membership
/// check is the pack read, a `PackError::NotFound` mapped to a routed
/// `workflow-refs.unknown-workflow` block, so a typo'd id never strands a task dir.
///
/// A `creates-task: true` work-workflow (e.g. `single-task`) **mints**: it opens
/// the task working area (reads HEAD), provisions the task's commit doc, and
/// binds the `task` context root, then composes with `{{task.intent}}` bound. A
/// `creates-task: false` workflow (the router and its kind) composes with **no
/// task context** — no mint, no working area, no commit doc, no commit-role bind:
/// it carries `task = None` and feeds only the selectable-workflow `catalog` to
/// composition (`write-commands.md` → Task origination, the `creates-task: false`
/// compose contract; `workflow-dialect.md` → Workflow selection). Either arm feeds
/// the cascade catalog **filtered to the selectable (`creates-task: true`) work-
/// workflows** into the context, so `{{catalog}}` resolves the same list the
/// router lists and never names itself.
///
/// The `source` is the layer-aware [`StepSource`] (phase-2 by-id shadowing live);
/// `deltas` are the manifest's phase-4 `structural-op` deltas, applied to the
/// workflow's include id list **before** include expansion (`overrides.md` →
/// Resolution algorithm phase 4 / Why structural deltas precede expansion). The
/// no-delta / no-shadow path resolves the pack include list unchanged and reads
/// every step's pack body, so the composed bytes stay byte-identical to before
/// the wiring landed (the read-side determinism guard).
///
/// `defs` is the layer-aware definition read surface ([`CascadeDefs`]): the
/// workflow + schema reads route through the cascade's phase-2 file owners so a
/// project whole-file shadow wins — the definition-read counterpart of the
/// layer-aware step `source` (`overrides.md` → whole-file definition shadow).
// The compose seam threads its inputs positionally: the read surfaces (`source`,
// `defs`), the cascade override bundle, and the milestone list each pin to one
// argument; bundling them would obscure the seam more than it would simplify it.
#[allow(clippy::too_many_arguments)]
fn compose_core(
    repo_root: &Path,
    intent: &str,
    pack: &dyn PackSource,
    workflow_id: &str,
    source: &dyn StepSource,
    defs: &CascadeDefs<'_>,
    overrides: &ComposeOverrides,
    milestone: MilestoneFeed<'_>,
    seam: Option<&str>,
    slug_override: Option<&str>,
    preview: bool,
) -> Result<Composition> {
    let workflow_bytes = defs.read_workflow(pack, workflow_id)?;
    let mut def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    // The manifest's `structural-op` deltas scoped to *this* workflow id — a
    // manifest may carry deltas for several workflows; only these apply here.
    let scoped = scoped_deltas(workflow_id, &overrides.deltas);
    // Phase 4 — apply the scoped deltas to the include id list, before include
    // expansion. A `replace-step` swaps a pack step id for a project-shadowed one,
    // so the layer-aware source resolves the new id to the project body. An
    // orphaned anchor (a delta whose anchor a same-manifest delta removed) surfaces
    // here as a blocking, routed finding (`overrides.md` → Within-layer manifest
    // order); the gate below re-runs the same pass to validate the post-phase-4
    // list for delta-introduced cycles/dangles.
    def.includes = apply_structural_deltas(&def.includes, &scoped).map_err(finding_to_err)?;
    // The command catalog is read against the composing workflow's **origin pack**
    // — the constituent that defines `workflow_id`'s top-level id — so a loser-pack
    // workflow's `{{cli.X}}` resolves against ITS OWN pack's `commands.yaml`, never
    // the precedence-winner's catalog (which need not be a superset). This is the
    // same origin lookup the step source's `scope_to_workflow` performs (one origin
    // serves both body-reference reads); for a single pack the origin *is* `pack`,
    // so the read is byte-identical (`multi-pack.md` → Pack-local body-reference
    // resolution → Command-refs).
    let origin = pack.origin_pack(PackResourceKind::Workflows, &ResourceId::from(workflow_id));
    let commands = load_catalog(origin)?;
    // The selectable-workflow list both arms feed to composition — the router's
    // `{{catalog}}` input, filtered to `creates-task: true` so it never lists
    // itself or any other `creates-task: false` workflow.
    let selectable = selectable_workflows(pack, defs)?;
    // The committed managed store both arms feed — the `{{store.<doctype>}}` input,
    // enumerated from the committed `<location>/<slug>.md` instances (CLI locates,
    // engine resolves). Schemas resolve through the cascade so a project
    // `schemas/<id>.yaml` shadow's `location:` drives the enumeration.
    let schemas = defs.all_schemas(pack)?;
    // The committed doc-store binds to **jigc_home** (the main checkout): `repo_root` here
    // is the worktree, so enumerate instances from jigc_home. Outside a worktree the two
    // coincide, so the feed is byte-identical (M31 Inc 2 / WF3); `mint_in_repo` below
    // resolves the same split internally for its `.jigc/` write + worktree HEAD read.
    let store_root = crate::repo::jigc_home(repo_root).unwrap_or_else(|| repo_root.to_path_buf());
    let store = committed_store(&store_root, &schemas);
    // The empty-research advisory decision — computed before `store` is moved into the
    // context. A vision-forming workflow composed against an empty research store keeps
    // the `author-vision` advisory (route to `do-research` first); all else drops it.
    let advise_research = warrants_research_advisory(&def, &store);

    // The producer's in-hand task id, threaded onto the composed view so
    // `--format json` surfaces it structurally (`command-output-contract.md` §1):
    // the freshly minted id on the work-minting arm, `None` on the
    // `creates-task: false` compose arm.
    let task_id: Option<String>;
    let ctx = if should_provision_commit_doc(&def) && preview {
        // The preview arm (`jigc workflow <id> --preview`): compose exactly what
        // this work-minting workflow will ask, **without minting** — skip
        // `mint_in_repo` (no HEAD read, no `.jigc/tasks/<id>/` working area) and
        // `provision_commit_doc` (no staged commit skeleton). `build_context` gets
        // a fixed synthetic valid slug so the composed bytes are byte-identical to
        // a real mint's with the minted id textually substituted by
        // [`PREVIEW_SLUG`] (the mint arm feeds the same empty `RolesRecord`, so the
        // only difference is the id string). `task_id` stays `None` so `view.task`
        // carries no fictional id and `minted` is `false` (`surface-contract.md` →
        // law 1: nothing lies; a preview names no task that exists).
        task_id = None;
        build_context(
            PREVIEW_SLUG,
            intent,
            &def,
            &RolesRecord::new(),
            selectable,
            store,
            seam,
            schemas,
        )
    } else if should_provision_commit_doc(&def) {
        // Mint the task (reads HEAD). Minting after the definition loads so a
        // malformed pack never leaves a task dir behind. The `--slug` override, when
        // present, drives the minted id verbatim (validated inside `mint_in_repo`).
        let minted = mint_in_repo(repo_root, intent, workflow_id, slug_override)?;
        task_id = Some(minted.id.clone());
        // Workflow-provisioned instance — a `creates-task` work-workflow
        // provisions the task's commit doc (the sink of its
        // `<<author: {{task.commit#summary}}>>` slot), so the agent only fills
        // slots (`write-commands.md` → Instance provisioning → Workflow-
        // provisioned). The empty skeleton stages here, ready for the
        // `jigc doc set-field`/`set-slot` write loop.
        provision_commit_doc(pack, defs, &minted.dir, &minted.id)?;
        // At mint there are no bound context roles yet (the agent binds them
        // in-task, e.g. an ADR via the create-gate); resume re-reads them.
        build_context(
            &minted.id,
            intent,
            &def,
            &RolesRecord::new(),
            selectable,
            store,
            seam,
            schemas,
        )
    } else {
        // The `creates-task: false` compose contract: no mint, no working area,
        // no commit doc, no role binding — `task = None`. Intent threads forward
        // only via the re-run command's agent marker, never a resolved data-value;
        // any `task.*` reference here is the blocking
        // `workflow-refs.task-ref-in-no-task-workflow` conformance error.
        task_id = None;
        ComposeContext {
            task: None,
            catalog: selectable,
            store,
            // The composed cascade's resolved doctype set — the `{{schema:<doctype>}}`
            // projection's feed, identical on the no-task arm (a `creates-task: false`
            // workflow's steps may solicit an authoring too; an unfed map would turn
            // every `{{schema:…}}` into the blocking dangling-ref finding).
            schemas,
            // The `fan-out` list-source (`{{milestone.tasks}}`). Empty for the
            // single-`start` no-task path (the router); fed the milestone's
            // id-sorted sub-task list by the `jigc milestone execute` dispatch —
            // the lone production site feeding this non-empty (`milestone.rs`
            // `run_execute`; `write-commands.md` → Executing the milestone).
            milestone: milestone.tasks.to_vec(),
            // The bound work-unit's id — the `milestone.id` leaf the three
            // `{ agent: milestone_id }` catalog args source. `None` on every compose
            // but `jigc milestone execute`, where the `<MILESTONE_ID>` marker stands (M49).
            milestone_id: milestone.id.map(str::to_owned),
            // The CLI-owned source seam — `None` on every `start`/milestone compose
            // path; fed the staged foreign bytes only by the `jigc migrate` verb
            // (auto-migration.md → The source seam). A `creates-task: false`
            // migration workflow is not a shipped shape, so this stays `None` here.
            source: None,
        }
    };
    // Compose-time `workflow-refs` gate over the **post-phase-4** include list +
    // the **post-phase-5** step bodies: validate every placeholder / include /
    // command-ref / marker, plus the two M4 fill checks (slot-fill-orphan +
    // fill-survivor), before any output reaches the agent — the scoped structural
    // deltas applied, the slot-fills + resolved fills fed in — so a delta-introduced
    // cycle / dangling include, an orphaned slot-fill, or a surviving nested
    // `{{fill:}}` all surface here at resolution time through the same machinery
    // (`overrides.md` → Resolution algorithm phases 4–7; The `{{fill:}}` placeholder).
    // The slot-fills thread unscoped (they carry no workflow id; the orphan check is
    // keyed on whether *this* workflow's composed bodies declare each point). A
    // blocking finding short-circuits.
    let findings = compose::workflow_refs_with_fills(
        &workflow_bytes,
        &scoped,
        &overrides.slot_fills,
        &overrides.fills,
        source,
        &commands,
        &ctx,
    );
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    // Phase 5 — apply the cascade's slot-fill content to each step body *before*
    // include expansion (phase 7): the [`FillStepSource`] runs `apply_slot_fills` as
    // each `StepDef` is fetched by id, so the filled content's own `{{include:}}` /
    // `{{cli.…}}` / `{{@…}}` resolve in the later phases as if the pack had written
    // them inline. A no-fill cascade (empty `fills`) is the identity — every point
    // resolves to its pack default, so the no-override path stays byte-identical.
    let filled = FillStepSource {
        inner: source,
        fills: &overrides.fills,
        ctx: &ctx,
    };
    let composed = compose::compose(&def, &filled, &commands, &ctx).map_err(finding_to_err)?;
    Ok(Composition {
        // The fresh front door mints iff the workflow is `creates-task: true` (the arm
        // above that sets `task_id`), so on this path the mint *is* the presence of an id.
        minted: task_id.is_some(),
        view: ComposedWorkflow {
            task: task_id,
            text: resolve_research_advisory(composed.text, advise_research),
        },
        // The composing workflow's create-gates — the CLI-side presentation fact the
        // renderer names on the `create-gates:` line (never the pinned JSON).
        gates: create_gates(&def),
        // The fresh front door either mints a brand-new task or composes none at all;
        // either way this compose is never a milestone sub-task's (membership is
        // `jigc milestone add-task`'s alone, and it does not compose).
        sub_task_of: None,
        // The shared compose spine fills nothing here: the two work-starting `start`
        // forms attach the already-open set on the way out ([`with_also_open`]), so a
        // `--preview`, a no-intent compose and the milestone feed all stay inert.
        also_open: Vec::new(),
        amend: None,
    })
}

/// The manifest's `structural-op` deltas scoped to `workflow_id` — a
/// `structural-op` names its workflow in the target (`workflow:<id>#…`), so a
/// manifest may carry deltas for several workflows; only those targeting the
/// composed workflow apply at its phase 4 (`overrides.md` → Resolution algorithm
/// phase 4 / Delta targets). The scoped set threads into **both** the phase-4
/// `apply_structural_deltas` (the include list `compose` consumes) and the
/// post-phase-4 `workflow_refs_with_deltas` gate, so the gate validates the exact
/// list `compose` will expand. A no-delta cascade returns an empty set — the
/// byte-identity guard (`apply_structural_deltas` over `[]` is the identity).
pub(crate) fn scoped_deltas(workflow_id: &str, deltas: &[StructuralDelta]) -> Vec<StructuralDelta> {
    deltas
        .iter()
        .filter(|d| d.target().workflow_id == workflow_id)
        .cloned()
        .collect()
}

/// The top-level cross-pack id-spaces a collision can be *adjudicated* in, **derived
/// from the composed pack-set** rather than hand-listed: every `Schemas` id, every
/// `Workflows` id, and the whole-file `Config:knobs` resource. Each is
/// `(label, kind, resource-id)`.
///
/// These are exactly the **user-facing top-level definitions** that resolve by
/// pack-precedence (`design/multi-pack.md` → Collision resolution), so for each one a
/// single pack's whole definition wins and "won by X" is a true statement about the
/// composed surface.
///
/// **Excluded by rule:** `Steps`, `Config:commands` and the pack field-types resolve
/// **pack-locally** — against the pack that defines the referring workflow or schema
/// (`multi-pack.md` → Pack-local body-reference resolution). Two packs shipping
/// `step:implement` is not a collision anybody wins: each pack's workflows keep their
/// own body, so a winner line for them would be false.
///
/// Deriving rather than listing is what lets a *project* pack's demoted doctype be
/// named: the hand-written pair this replaced could only ever speak about `commit` and
/// `default-workflow`, so any other collision the pack-set carried was silently absent
/// from the determinism header. The order is fixed (schemas, then workflows, then the
/// knob file) over each kind's already-sorted [`PackSource::list`], so the emitted line
/// order is stable and pack-order-invariant.
fn adjudicated_collisions(pack: &dyn PackSource) -> Vec<(String, PackResourceKind, ResourceId)> {
    let mut spaces: Vec<(String, PackResourceKind, ResourceId)> = pack
        .list(PackResourceKind::Schemas)
        .into_iter()
        .map(|id| (format!("doctype:{id}"), PackResourceKind::Schemas, id))
        .collect();
    spaces.extend(
        pack.list(PackResourceKind::Workflows)
            .into_iter()
            .map(|id| (format!("workflow:{id}"), PackResourceKind::Workflows, id)),
    );
    spaces.push((
        "config:knobs".to_owned(),
        PackResourceKind::Config,
        ResourceId::from("knobs"),
    ));
    spaces
}

/// The adjudicated top-level cross-pack collision winners for `pack` — one
/// [`CollisionWinner`](engine::result::CollisionWinner) per [`adjudicated_collisions`]
/// id-space that ≥2 composed packs own, naming the pack that **owns** the id: its
/// [`PackSource::origin_pack`] — the same pack `read` selects and the same anchor the
/// definition's body-references resolve against — reported through that pack's **own**
/// provenance segment.
///
/// Never the composed set's highest-precedence segment: with three packs the top of the
/// precedence order need not own the id at all (it may ship nothing that collides, or
/// may have been demoted by the freeze rule), and naming it makes the determinism header
/// assert the opposite of what the loader resolved (`design/overrides.md` → the cascade
/// provenance determinism contract; `design/multi-pack.md` → Provenance under N packs).
/// At two packs the two readings coincide, which is why the defect survived M14.
///
/// A single-pack composition owns each id at most once, so it yields **no** winners and
/// the `--explain` output stays byte-identical (`design/multi-pack.md` → Provenance:
/// where a collision was adjudicated, name which pack won).
fn collision_winners(pack: &dyn PackSource) -> Vec<engine::result::CollisionWinner> {
    adjudicated_collisions(pack)
        .into_iter()
        .filter(|(_, kind, id)| pack.owner_count(*kind, id) > 1)
        .filter_map(|(collision, kind, id)| {
            let (pack_id, pack_version) = owning_pack_segment(pack.origin_pack(kind, &id))?;
            Some(engine::result::CollisionWinner {
                collision,
                pack_id,
                pack_version,
            })
        })
        .collect()
}

/// The `(pack-id, version)` an owning pack names **itself** by — its own first
/// [`PackSource::provenance_segments`] segment, the identical accessor the `Pack input:`
/// lines render, so one pack cannot be named two ways on one screen. `None` only for a
/// pack that reports no segment at all (an empty composite — unreachable in production,
/// where the base is always present).
fn owning_pack_segment(owner: &dyn PackSource) -> Option<(String, String)> {
    owner.provenance_segments().into_iter().next()
}

/// The composed pack-set's per-pack provenance inputs for `pack` — one
/// [`PackInput`](engine::result::PackInput) per composed pack, **in precedence
/// order** (highest-precedence first), zipping each pack's `provenance_segments()`
/// id/version with its `provenance_entries()` resolving path + blake3 content-hash.
/// Both accessors enumerate the constituents in the same precedence order, so the
/// zip is parallel by construction. A single-pack composition yields exactly one
/// entry (mirroring the one-segment `provenance_segments()` degrade), so the
/// `--explain` provenance line degrades to one pack — additive, never perturbing the
/// single-pack surface (`design/multi-pack.md` → Provenance under N packs;
/// `design/worked-examples.md` → flow 17 assertion 5).
fn pack_inputs(pack: &dyn PackSource) -> Vec<engine::result::PackInput> {
    pack.provenance_segments()
        .into_iter()
        .zip(pack.provenance_entries())
        .map(
            |((pack_id, pack_version), entry)| engine::result::PackInput {
                pack_id,
                pack_version,
                path: entry.path,
                content_hash: entry.content_hash,
            },
        )
        .collect()
}

/// Build the `--explain` resolution tree (layers 1–2 — `workflow-dialect.md` →
/// `--explain` output contract) for `workflow_id` over the resolved cascade —
/// the seam the `--explain` dispatch (T3) renders.
///
/// Assembles the **exact** inputs [`compose_core`] feeds phase 4: the pack
/// workflow's include id list ([`read_workflow`] + [`load_workflow_def`]), the
/// manifest's `structural-op` `deltas` [`scoped_deltas`]-filtered to this
/// workflow, the layer the workflow definition file itself resolved to
/// ([`cascade::Resolved::file_owner`], defaulting to pack-default since the pack
/// always ships the workflow), `file_owner` as the per-step layer lookup, and the
/// applied scalar-key overrides ([`cascade::Resolved::scalar_overrides`]) for the
/// tree's layer-1 per-knob provenance lines.
/// Delegates the position/annotation semantics to
/// [`compose::build_resolution_tree`], so the built tree's step order equals the
/// composed include order by construction — no re-resolution, no second algorithm.
///
/// Takes the manifest `deltas` directly (the same `ComposeOverrides.deltas` the
/// `--explain` dispatch already holds), keeping the seam free of the compose-only
/// override bundle.
///
/// Its live caller is [`compose_explain_in_repo`] (the `--explain` flag dispatch,
/// `render::explain`).
pub(crate) fn build_resolution_tree(
    pack: &dyn PackSource,
    resolved: &cascade::Resolved,
    project_config: &Path,
    deltas: &[StructuralDelta],
    workflow_id: &str,
) -> Result<engine::result::ResolutionTree> {
    // Read the workflow definition through the cascade — a project shadow's include
    // list drives the explain tree, so `--explain` never drifts from compose
    // (`overrides.md` → whole-file definition shadow).
    let workflow_bytes =
        CascadeDefs::new(resolved, project_config).read_workflow(pack, workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let scoped = scoped_deltas(workflow_id, deltas);
    let workflow_layer = resolved
        .file_owner(workflow_id)
        .unwrap_or(cascade::LayerKind::PackDefault);
    let scalar_overrides = resolved
        .scalar_overrides()
        .map(|(key, value, layer)| engine::result::ScalarOverride {
            key: key.to_owned(),
            value: value.to_owned(),
            layer,
        })
        .collect();
    // The soft-rejected below-floor `scalar-set`s ride the tree's layer 1 distinctly
    // from the applied overrides, mirroring how `scalar_overrides` flows from
    // `Resolved` (`design/overrides.md` → Soft-rejection: the rejected deltas ride
    // `Resolved`, consumed by `--explain`).
    let rejected_demotions = resolved
        .rejected_scalar_sets()
        .map(
            |(key, attempted, floor, layer)| engine::result::RejectedDemotion {
                key: key.to_owned(),
                attempted: attempted.to_owned(),
                floor: floor.to_owned(),
                layer,
            },
        )
        .collect();
    compose::build_resolution_tree(
        workflow_id,
        workflow_layer,
        &def.includes,
        &scoped,
        scalar_overrides,
        rejected_demotions,
        |id| resolved.file_owner(id),
    )
    .map_err(finding_to_err)
}

/// Build the `--explain` resolution tree for the workflow a `jigc start` would
/// compose, **without minting** — the `jigc start --explain "<intent>"` front door
/// (`design/workflow-dialect.md` → `--explain` output contract; `design/worked-
/// examples.md` → 3a). Returns the [`engine::result::ResolutionTree`] paired with
/// the pack provenance label (`<pack-id>/v<version>`) the renderer stamps on the
/// workflow line.
///
/// Task-independent by construction (the resolution tree is `definition + cascade`,
/// never live task state — `workflow-dialect.md`: "structure is fixed by
/// `definition + cascade` and is task-independent"). So `intent` threads nowhere
/// into the tree; it is accepted only to mirror the compose front door's call shape
/// and is deliberately unused here — no mint, no working area, no commit doc.
///
/// Resolves the cascade exactly as [`compose_in_repo`] does (the cascade-presence
/// guard routes a missing project layer to `jigc setup`), reads the workflow id
/// from the cascade `default-workflow` knob unless `workflow` names one explicitly
/// (`--workflow <X>`, mirroring [`compose_named_in_repo`]'s bypass), and delegates
/// the tree assembly to [`build_resolution_tree`] over the manifest's
/// `structural-op` deltas — so the tree's step order equals the order a compose
/// would expand.
pub fn compose_explain_in_repo(
    start: &Path,
    intent: Option<&str>,
    workflow: Option<&str>,
) -> Result<(engine::result::ResolutionTree, String)> {
    let _ = intent; // task-independent: the tree never embeds the intent.
    let project_config = require_project_config(start)?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (resolved, overrides) = resolve_cascade(pack, &project_config)?;
    let workflow_id = match workflow {
        Some(id) => id.to_owned(),
        None => resolved
            .scalar_required(DEFAULT_WORKFLOW_KEY)
            .map(str::to_owned)
            .map_err(anyhow::Error::from)?,
    };
    let mut tree = build_resolution_tree(
        pack,
        &resolved,
        &project_config,
        &overrides.deltas,
        &workflow_id,
    )?;
    // The adjudicated top-level cross-pack collision winners — empty for a
    // single-pack composition, so the tree (and its rendered `--explain`) stays
    // byte-identical (`design/multi-pack.md` → Provenance).
    tree.collision_winners = collision_winners(pack);
    // The composed pack-set's per-pack provenance inputs (path + content-hash) — one
    // entry per composed pack, highest-precedence first. A single-pack composition
    // degrades to one entry (additive provenance), so the `--explain` surface stays
    // byte-identical but for the one input line (`design/multi-pack.md` → Provenance).
    tree.pack_inputs = pack_inputs(pack);
    // The workflow line names the pack that actually **provided** the workflow — its
    // [`PackSource::origin_pack`], the same anchor its body-references resolve against
    // — never a precedence read of `config/defaults` + the composite's version, which
    // under three packs names whichever pack sits on top even when it ships neither the
    // workflow nor an identity of its own (`design/multi-pack.md` → Provenance under N
    // packs; `design/overrides.md` → the cascade provenance determinism contract). A
    // single pack is its own origin, so the label degrades byte-identically to the
    // single-pack floor.
    let origin = pack.origin_pack(
        PackResourceKind::Workflows,
        &ResourceId::from(workflow_id.as_str()),
    );
    // `<id>/<version>`, the one spelling every pack-naming surface uses — the
    // orientation `Pack:` header, the `collision:` lines and the `Pack input:` lines
    // all render the pair unadorned (`design/multi-pack.md` → Provenance states the
    // segment as `Pack: <id>/<version>`). The label glued a literal `v` on until M50,
    // so one `--explain` screen named one pack two ways — `dev/vfs-local` on this line
    // against `dev/fs-local` on its own `Pack input:` line, dressing the `fs-local`
    // sentinel a version-less pack reports as a version no pack declares
    // (`design/surface-contract.md` → law 1).
    let pack_label = owning_pack_segment(origin)
        .map(|(pack_id, version)| format!("{pack_id}/{version}"))
        .unwrap_or_default();
    Ok((tree, pack_label))
}

/// The compose-relevant override surface a project layer carries beyond its
/// resolved scalar/file owners: the phase-4 `structural-op` deltas, the `slot-fill`
/// deltas (for the phase-5 fill pass + the orphan check), and the cascade-resolved
/// [`ResolvedFills`] map (`(step_id, fill_id)` → content body) those slot-fills load.
/// Bundled so both front doors thread one value through [`compose_drained`] rather
/// than a widening tuple (`overrides.md` → Resolution algorithm phases 4–5).
struct ComposeOverrides {
    /// The manifest's `structural-op` deltas (phase-4 include-list mutation).
    deltas: Vec<StructuralDelta>,
    /// The manifest's `slot-fill` deltas — the gate's orphan check walks them; they
    /// carry no workflow id, so they thread unscoped (the orphan check is keyed on
    /// whether the *composed* workflow's bodies declare each target's point).
    slot_fills: Vec<SlotFillDelta>,
    /// The cascade-resolved fill content the phase-5 pass splices, keyed
    /// `(step_id, fill_id)`. A no-fill cascade is the empty map (every point → its
    /// pack default), so the no-override path stays byte-identical.
    fills: ResolvedFills,
}

impl ComposeOverrides {
    /// A structural-only override surface (no slot-fills, empty resolved fills) —
    /// the phase-4-only shape the `compose_core` unit tests drive (the slot-fill
    /// apply path is exercised binary-driven in `tests/start_compose.rs`).
    #[cfg(test)]
    fn structural(deltas: Vec<StructuralDelta>) -> Self {
        ComposeOverrides {
            deltas,
            slot_fills: Vec::new(),
            fills: ResolvedFills::new(),
        }
    }
}

/// Resolve the cascade for a compose: seed the [`PackDefaultLayer`] scalar surface
/// from `config/knobs`, load the project [`OverrideLayer`] + its `structural-op` /
/// `slot-fill` deltas from `<project_config>/manifest.yaml`, and resolve — returning
/// the [`cascade::Resolved`] (phase-2 file owners + phase-3 scalars) paired with the
/// [`ComposeOverrides`] the compose applies at phases 4–5. The one cascade build both
/// front doors share (`overrides.md` → Resolution algorithm).
fn resolve_cascade(
    pack: &dyn PackSource,
    project_config: &Path,
) -> Result<(cascade::Resolved, ComposeOverrides)> {
    // The `tracked-fork` deltas are recording surface only — read by the (M5)
    // reconciliation, never by compose — so they are dropped here (a fork applies
    // as its phase-2 shadowed step file, no new resolution logic).
    let (project, deltas, slot_fills, _forks, _bases) = load_project_layer(project_config)?;
    let fills = load_fills(project_config, &slot_fills)?;
    let resolved = resolve_layers(pack, &project)?;
    Ok((
        resolved,
        ComposeOverrides {
            deltas,
            slot_fills,
            fills,
        },
    ))
}

/// Resolve the cascade for severity assignment only — the [`cascade::Resolved`] the
/// engine's M6 post-pass reads (`design/validation.md` → Every finding-emitting entry
/// point must resolve the cascade). The finding-emitting paths that do *not* compose a
/// workflow (`task validate` / `finalize`, `jigc upgrade`) build their `Resolved` here,
/// so a recorded `validation.<probe>.<check>.severity` scalar-set tunes their findings
/// — while a no-override project layer resolves to the base scalars, leaving the
/// post-pass inert (the no-override path stays byte-identical).
///
/// A **missing** `project_config` dir is the no-override case: [`load_project_layer`]
/// yields an empty layer (no manifest, no `steps/` shadows), so the resolution is the
/// pack-default base — exactly `cascade::resolve(pack, None, None)` would give.
pub(crate) fn resolve_severity_cascade(
    pack: &dyn PackSource,
    project_config: &Path,
) -> Result<cascade::Resolved> {
    let (project, _deltas, _slot_fills, _forks, _bases) = load_project_layer(project_config)?;
    resolve_layers(pack, &project)
}

/// Resolve the severity cascade for `jigc upgrade` — like [`resolve_severity_cascade`]
/// but **resilient to an orphaned scalar-set**. Upgrade compares a layer recorded
/// against an *older* pack: a `scalar-set` whose key the current pack dropped is
/// exactly the orphan `override-default` reports, but feeding it to [`cascade::resolve`]
/// would trip the closed-surface check and abort the upgrade that exists to report it.
/// So the project layer's scalar-sets are filtered to keys the current pack still
/// declares before resolving — the live `validation.<probe>.<check>.severity` overrides
/// still tune the upgrade's findings, while a dropped knob is left to the classifier
/// (`design/validation.md` → Every finding-emitting entry point must resolve the
/// cascade; `overrides.md` → Upgrade reconciliation: a `scalar-set` is `orphaned` once
/// the pack drops it).
pub(crate) fn resolve_severity_cascade_resilient(
    pack: &dyn PackSource,
    project_config: &Path,
) -> Result<cascade::Resolved> {
    let knobs_bytes = read_pack(pack, PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let base = knobs.base_scalars();
    // Carry the demotion-lock floors here too (same as `resolve_layers`): a below-floor
    // demotion on a still-declared intrinsic key is soft-rejected, not applied, when
    // upgrade re-resolves the layer's findings (`design/overrides.md` → Soft-rejection).
    let pack_default = PackDefaultLayer::new(
        pack_id_from_config(pack)?,
        pack.pack_version(),
        base.clone(),
        Vec::new(),
    )
    .with_floors(knobs.floors().clone());
    let (project, _deltas, _slot_fills, _forks, _bases) = load_project_layer(project_config)?;
    // Keep only the scalar-sets whose key the current pack still declares; an orphaned
    // one is reported by the classifier, not resolved here.
    let mut filtered = OverrideLayer::empty();
    for (key, value) in project.scalar_set_pairs() {
        if base.contains_key(key) {
            filtered = filtered.scalar_set(key, value);
        }
    }
    Ok(cascade::resolve(&pack_default, None, Some(&filtered))?)
}

/// Seed the [`PackDefaultLayer`] scalar surface from `config/knobs` and resolve the
/// cascade with `project` as the sole override layer (`team` lands later) — the one
/// resolution step `resolve_cascade` / [`resolve_severity_cascade`] / [`resolve_step_body`]
/// share (`overrides.md` → Resolution algorithm).
fn resolve_layers(pack: &dyn PackSource, project: &OverrideLayer) -> Result<cascade::Resolved> {
    let knobs_bytes = read_pack(pack, PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    // Attach the demotion-lock floors so a below-floor `scalar-set` on a floored
    // intrinsic check is soft-rejected at resolution (`design/overrides.md` →
    // Soft-rejection; the floor is a `KnobDecl` field). Without this the floored
    // intrinsic keys would resolve floor-less and a demotion would silently apply.
    let pack_default = PackDefaultLayer::new(
        pack_id_from_config(pack)?,
        pack.pack_version(),
        knobs.base_scalars(),
        Vec::new(),
    )
    .with_floors(knobs.floors().clone());
    Ok(cascade::resolve(&pack_default, None, Some(project))?)
}

/// Load each `slot-fill` delta's native content file into the cascade-resolved
/// [`ResolvedFills`] map: for every delta, read `<project_config>/<content>` (its
/// `content_id` is the `fills/<id>.md` basename) and key the body by the target's
/// `(step_id, fill_id)` — the phase-5 input the pass splices (`overrides.md` →
/// Resolution algorithm phase 5; `storage.md` → Config layout, `fills/<id>.md`).
///
/// The project is the only override layer in M4, so a later layer's body never wins
/// here; when the team layer lands (M5), the same key would be overwritten in
/// precedence order (project last). A missing or unreadable fill file is a clear,
/// path-bearing error — the manifest references a native file that must exist.
fn load_fills(project_config: &Path, slot_fills: &[SlotFillDelta]) -> Result<ResolvedFills> {
    let mut fills = ResolvedFills::new();
    for delta in slot_fills {
        let path = project_config
            .join("fills")
            .join(format!("{}.md", delta.content_id));
        let body = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "slot-fill on `step:{}#{}` references {}, which is unreadable",
                delta.target.step_id,
                delta.target.fill_id,
                path.display(),
            )
        })?;
        fills.insert(
            (delta.target.step_id.clone(), delta.target.fill_id.clone()),
            body,
        );
    }
    Ok(fills)
}

/// Build the selectable-workflow catalog from `pack`: every workflow the pack
/// provides, parsed for its front-matter, **filtered to `creates-task: true`**
/// (the work-workflows a router selects among), each paired with its `when`
/// selection hint, in the pack's `list` order — the deterministic `catalog`
/// data-value root (`workflow-dialect.md` → data-value roots / Workflow
/// selection). A `creates-task: false` workflow (the router itself) is never a
/// selectable entry, so the router never lists itself. A `creates-task: true`
/// workflow that opts out with `selectable: false` (the fan-out `sub-task`, which
/// can never reach a commit boundary on its own — its only `finalize` is the
/// parent milestone's) is likewise excluded, so the router never offers a pick
/// that cannot commit. A *selectable* workflow that declares no `when` is a
/// definition bug, surfaced as a clear, id-bearing error; a non-selectable one
/// need carry no `when`.
///
/// **Each definition is read through the cascade** ([`CascadeDefs::read_workflow`]), so a
/// project-layer whole-file shadow decides both halves of the entry — whether the id is
/// offered at all, and the `when` it is offered under. Driven at HEAD before this
/// (2026-09-20): over a project shadow of `quick-fix` declaring `selectable: false` +
/// `suppressed.door`, `describe --workflows` said *hidden from the router catalog* and
/// `jigc start --workflow quick-fix` refused `workflow.verb-routed`, while bare `jigc
/// start` listed it anyway under the **pack's** `when` — the catalog offering a route two
/// other doors refuse (`surface-contract.md` → law 1). This was the last production
/// workflow read that did not go through the resolver, carried as a stated exemption in
/// `tests/workflow_resolution_unified.rs`'s `ALLOWED`; that row is retired with this datum.
///
/// The **id set** stays `pack.list` — the same enumeration [`crate::describe`] and
/// [`enumerate_store_workflows`] perform — so a project-layer-only id is listed by none of
/// the three surfaces rather than by one of them.
pub(crate) fn selectable_workflows(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
) -> Result<Vec<CatalogEntry>> {
    let mut entries = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = defs.read_workflow(pack, id.as_str())?;
        let def = load_workflow_def(&bytes).map_err(finding_to_err)?;
        if !def.creates_task || !def.selectable {
            continue;
        }
        let when = def.when.filter(|w| !w.trim().is_empty()).with_context(|| {
            format!("selectable workflow `{id}` is missing the required `when` selection hint")
        })?;
        entries.push(CatalogEntry::new(id.as_str(), when));
    }
    Ok(entries)
}

/// The **blanket base-pin refusal** — the one message every door of a *milestone
/// sub-task* raises when the checkout has moved off the milestone's shared pin.
///
/// It lives here, constructed once, because two doors raise it and their prose must
/// not drift: [`reenter_in_repo`] (`jigc workflow <W> --task <sub>`) and the
/// sub-task arm of [`resume_in_repo`] (`jigc start --task <sub>`). A sub-task's only
/// commit boundary is `jigc milestone finalize <m>` — which blocks on *any* moved
/// history, since the sub-agent worktrees were cut from the pin — so both of its read
/// doors stay blanket-strict rather than making the overlap-aware `decide_base_repin`
/// decision a top-level task's own `finalize` makes (M47 Inc 8 / N7; `design/storage.md`
/// → The per-task working area).
///
/// **The route is per unit kind** (M46 Inc 8 / B2-2 — the pre-1.0.0 trial's own repro,
/// `RC-1.0-gate/findings-verification.md` §3), because `milestone` says which recovery is
/// the real one:
///
/// - A **sub-task** (`Some(milestone_id)`) is routed at the isolation mechanism its work
///   actually happens in — `jigc milestone provision <m>` and the worktree at
///   `.jigc/worktrees/<id>`, cut from the very base this checkout has moved off. The two
///   routes this arm used to offer were both wrong for the only case it serves: `git
///   checkout <pin>` detaches the *shared* checkout onto a base the milestone's own record
///   commits already sit ahead of, and `jigc task discard <sub>` exits 0 while the committed
///   record still reads `status: active` and `milestone list-tasks` still lists the sub-task
///   — a route contradicting the record it leaves standing (`surface-contract.md` → law 1).
///
///   **The route is split on whether the worktree is actually there** (M53 — the cwd
///   census, C2-08). One line had covered both states on the ground that the provision span
///   *"adds a worktree that is missing and leaves one that exists untouched"* — idempotent,
///   and therefore honest. Driven, the provisioned state is the ordinary one: an agent
///   handed a `Spawn:` line, or resuming a sub-task from the shared checkout, meets this
///   refusal with the worktree already cut, and is told to run a command that does nothing
///   before the step that actually moves them. *Idempotent* is not *useful*: a route whose
///   first clause is a no-op in the state the reader is in teaches that jigc's routes are
///   approximate. So the provisioned arm names only the act that resolves it, and both arms
///   end on an absolute, pasteable `cd` (`design/surface-contract.md` → the printed-path
///   fence, the *pasteable shell bytes* disposition) — the same absolute the `Spawn:` line
///   emits, because this refusal and that line are two ways of reaching one cwd.
///
///   **And the split is three-way, because the reader can already be standing there** (M53,
///   the last pre-1.0.0 batch's review, MEDIUM 1). The `cd` arm answers a reader in the
///   *shared checkout*; the batch that landed just before this one made the first
///   jigc-sanctioned path that puts a reader **inside** the worktree with HEAD off the base
///   — `git switch -c <own-branch>` there, then an ordinary `jigc task finalize`, which used
///   to be refused at the seam and now lands. Driven from that cwd, the refusal printed `cd
///   <the very directory the reader was in>`: byte-identical to the caller's own canonical
///   path, and therefore the same no-op the arm above exists to avoid, one state over. So
///   the *already-there* cell names the act that actually resolves it — re-attaching HEAD to
///   the pin with `git switch --detach <pin>`, driven lossless: a branch created in the
///   worktree survives and every commit on it stays reachable, and the sub-task door composes
///   again immediately afterwards. It is `cd`'s sibling and not its replacement: from anywhere
///   else the `cd` is the act, and from a *subdirectory* of the worktree the `cd` resolves
///   nothing either, which is why the predicate is *the caller's repository root is this
///   worktree* rather than a raw cwd string equality. The **unprovisioned** arm needs nothing:
///   its worktree does not exist, so no caller's root can equal it.
/// - A **top-level task** (`None`) can only reach this refusal at the re-entry door
///   ([`reenter_in_repo`], which takes no membership decision), and there both offered
///   routes are right — no record names it and its own discard is the real teardown — so
///   that arm keeps its bytes.
fn blanket_base_pin_refusal(
    id: &str,
    milestone: Option<&str>,
    pinned: &BasePin,
    head: &BasePin,
    jigc_home: &Path,
    caller_root: &Path,
) -> anyhow::Error {
    let pinned_to = format!(
        "task `{id}` is pinned to base {} but you're on {}",
        pinned.short, head.short,
    );
    let Some(milestone_id) = milestone else {
        return anyhow!(
            "{pinned_to} — switch back with `git checkout {}` or {}",
            pinned.short,
            engine::finding::Route::mechanical(["jigc", "task", "discard", id, "--force"], ""),
        );
    };
    // The absolute the reader can paste from wherever they are standing — the same path
    // the `Spawn:` line emits, canonicalized the same way **and rendered by the same
    // predicate** ([`engine::finding::shell_operand`], below), so the two never name one
    // directory two ways. Until the M53 post-review fix they could: this site quoted through
    // `shell_token` and the `Spawn:` line interpolated raw, so on a repository path with a
    // space the refusal was runnable and the line it says it matches was not.
    let worktree = jigc_home.join(engine::milestone::worktree_path(id));
    // **Provisioned is a verdict git gives, never a directory shape.** `posture_subject` is
    // the shipped discriminator — `.git` is a file, the path is
    // `<jigc_home>/.jigc/worktrees/<id>`, and `<id>` is a sub-task of a milestone in this
    // workbench — and asking it is what keeps this refusal and the fan-out doors from
    // classifying one directory two ways. A bare `is_dir()` here would re-enact the M50
    // completion audit's `fanout_worktree_paths` finding: a claim about *shape* where the
    // question is about what git can vouch for.
    let provisioned = crate::repo::posture_subject(&worktree).is_dedicated();
    let worktree = worktree.canonicalize().unwrap_or(worktree);
    // **Is the reader already standing in it?** The comparison is over *canonicalized* roots,
    // because the two spellings differ on the one platform this was driven on: the caller's
    // cwd reads `/var/folders/…` while the emitted `cd` operand reads `/private/var/folders/…`
    // — the same directory, and a raw string compare would call them different and print the
    // no-op anyway. The subject is the caller's **repository root**, not its literal cwd, so a
    // reader in a subdirectory of the worktree takes this arm too: `cd <worktree>` would move
    // them, but it would not re-attach HEAD, so it resolves nothing there either.
    let already_there = provisioned
        && caller_root
            .canonicalize()
            .is_ok_and(|root| root == worktree);
    if already_there {
        return anyhow!(
            "{pinned_to} — this is a sub-task of milestone `{milestone_id}`, and this checkout \
             IS its worktree: its HEAD has moved off the base the milestone shares, so \
             re-attach it with `git switch --detach {}` and re-run this command — a branch you \
             created here and every commit on it stay reachable",
            pinned.short,
        );
    }
    let cd = engine::finding::shell_operand(&worktree.display().to_string());
    let here = "a sub-task's work happens in its own worktree, cut from that base rather \
                than in this checkout";
    if provisioned {
        // The worktree is already there: naming the provisioning door first would put a
        // no-op ahead of the only act that changes anything.
        return anyhow!(
            "{pinned_to} — this is a sub-task of milestone `{milestone_id}`, and {here}: run \
             this from that worktree — `cd {cd}`, then re-run this command",
        );
    }
    anyhow!(
        "{pinned_to} — this is a sub-task of milestone `{milestone_id}`, and {here}: run {}",
        engine::finding::Route::mechanical(
            ["jigc", "milestone", "provision", milestone_id],
            format!(
                " — it cuts the worktree this sub-task is missing — then `cd {cd}` and re-run \
                 this command there"
            )
            .as_str(),
        ),
    )
}

/// Resume an existing task by `id` and re-compose its **own** minting workflow —
/// the `jigc start --task <id>` form (`design/write-commands.md` → Task-id
/// collision & resume: `--task` "resumes an existing task … where it is in *its*
/// workflow"; `DECISIONS.md` 2026-05-31 → Four `jigc start` forms; `DECISIONS.md`
/// 2026-06-01 → M2 Increment 3 re-cut: resume composes the persisted minting
/// workflow id, not the cascade default — so a `single-task` task still composes
/// `single-task` after the default flips to `router`).
///
/// Unlike [`compose_in_repo`] this **mints nothing**: it resolves the existing
/// `.jigc/tasks/<id>/` working area, reads its persisted state (base pin, the
/// original intent, the recorded minting workflow id, the bound context roles in
/// `roles.json`), tests the task's base pin with the **same overlap-aware decision
/// the commit door makes** ([`engine::finalize::decide_base_repin`] — `storage.md` →
/// The per-task working area; `finalize.md` → Parallel hand-editing), then
/// re-composes with the bound roles in scope. A role bound since the task was minted
/// (e.g. an ADR created in-task through the create-gate) now resolves in the composed
/// view — the surface the superseding-decision context-slice reads
/// (`worked-examples.md` → Superseding decision).
///
/// A **malformed** id — not a well-formed slug — is refused before the join as not an id
/// at all ([`crate::task::reject_malformed_work_unit_id`];
/// `design/structural-grammar.md` → Work-units and runtime identity, resolution); a
/// well-formed but nonexistent one rejects with `no task \`<id>\``. Moved history that **overlaps**
/// the task's footprint rejects with the engine's routed `finalize.base-mismatch`
/// block naming the overlapping paths, while **disjoint** moved history re-pins to
/// HEAD in memory and composes (`write-commands.md` → Base mismatch on an existing
/// task) — **unless the id is a milestone sub-task**, whose commit door is the
/// milestone's, not this task's ([`blanket_base_pin_refusal`]).
pub fn resume_in_repo(start: &Path, id: &str) -> Result<Composition> {
    // Before anything joins `id` onto a path (M50 Inc 1 / T1).
    crate::task::reject_malformed_work_unit_id(id)?;
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    // The committed doc-store + `.jigc/` working area bind to **jigc_home** (the main
    // checkout); only the base-pin HEAD read below stays on the worktree `repo_root`
    // (M31 Inc 2 / WF3).
    let jigc_home = jigc_home_or_repo(start)?;
    let jigc_root = jigc_home.join(".jigc");
    // The one predicate all four by-id seams share (M53 Increment 3 / T3): absent is the
    // converged roster answer, and a directory carrying no base pin is a leftover, not a
    // task whose pin failed to read.
    let task_dir = crate::task::require_task_area(&jigc_home, id, crate::task::task_list_route())?;

    // The task is pinned to its base, and the pin is tested **exactly** as the commit
    // door of *this task's unit kind* tests it (M47 Inc 8 T1 / N7; `design/storage.md` →
    // The per-task working area). For a top-level task that door is `jigc task finalize`,
    // so resume makes the identical `decide_base_repin` decision over the same facts —
    // base→HEAD changed paths, dirty working-tree paths, and the task's promote
    // destinations: disjoint moved history re-pins to HEAD, only an overlap with the
    // task's footprint blocks. A read-only door may never be stricter than the commit
    // door it precedes, which the blanket `base != HEAD` refusal made it.
    //
    // For a **milestone sub-task** that door is `jigc milestone finalize <m>` — `jigc
    // task finalize <sub>` refuses outright (`finalize.milestone-sub-task`) — and the
    // milestone door blocks on *any* moved history. So the sub-task keeps the blanket
    // refusal, exactly as its sibling read door [`reenter_in_repo`] does: relaxing it
    // would make a read door **looser** than its own commit door, the inverse of the
    // defect above. The unit kind is read with the same `owning_milestone` membership
    // test `jigc task finalize` uses to mint that refusal, never a second rule.
    let pinned = state::read_base_pin(&task_dir)
        .with_context(|| format!("could not read the base pin for task `{id}`"))?;
    let head = read_head(&repo_root)?;
    if pinned.sha != head.sha {
        // Read **only on divergence**, like the schema set below: a task at its pin takes
        // the same door it always did, so it pays for no membership scan.
        if let Some(milestone_id) = engine::milestone::owning_milestone(&jigc_root, id) {
            return Err(blanket_base_pin_refusal(
                id,
                Some(&milestone_id),
                &pinned,
                &head,
                &jigc_home,
                &repo_root,
            ));
        }
        // The schema set the promote-destination half of the footprint needs — loaded
        // only on divergence, so a task at its pin pays nothing for the decision.
        let pack = make_pack()?;
        let pack = pack.as_ref();
        let (resolved, _) = resolve_cascade(pack, &project_config)?;
        let schemas = CascadeDefs::new(&resolved, &project_config).all_schemas(pack)?;
        if let Err(findings) = engine::finalize::decide_base_repin(
            id,
            &task_dir,
            &pinned,
            &head.sha,
            &crate::task::git_changed_paths(&repo_root, &pinned.sha, &head.sha)?,
            &crate::task::git_dirty_paths(&repo_root)?,
            &schemas,
        ) {
            // The decision blocks with one routed finding — `finalize.base-mismatch`,
            // which names the overlapping paths and claims no commit (honest as written
            // at a read-only door), or the promote sweep's own I/O block. `jigc start`
            // is not a gate verb, so it surfaces as the door's clean exit-1 error, never
            // a validation exit.
            let finding = findings
                .into_iter()
                .next()
                .expect("a blocked re-pin decision carries its finding");
            return Err(finding_to_err(finding));
        }
        // A re-pin is **in-memory only**: `base.json` is never rewritten (finalize's
        // landed-only discipline — the next invocation re-derives the decision). The
        // compose below already reads the committed store and the edge index from the
        // actual HEAD, so the intervening commit is in view with no further work.
    }

    // Resume composes the task's **own** minting workflow, never the cascade
    // default (`DECISIONS.md` 2026-06-01 → M2 Increment 3 re-cut): a `single-task`
    // task resumed after the default flips to `router` must still compose
    // `single-task`. A working area with no recorded workflow id is a clear fault,
    // not a silent fall-through to the default.
    let workflow_id = state::read_workflow_id(&task_dir)
        .with_context(|| format!("could not read the recorded workflow for `{id}`"))?
        .with_context(|| {
            format!(
                "task `{id}` has no recorded workflow — discard it with {} and re-start with {}",
                engine::finding::Route::mechanical(["jigc", "task", "discard", id, "--force"], ""),
                engine::finding::Route::mechanical(["jigc", "start"], ""),
            )
        })?;
    compose_task_workflow(
        ComposeCheckouts {
            jigc_home: &jigc_home,
            from: &repo_root,
        },
        &project_config,
        &task_dir,
        id,
        &workflow_id,
        &head,
        false,
    )
}

/// Re-enter a milestone sub-task as a fanned sub-agent — the `jigc workflow <W>
/// --task <id>` form (`design/write-commands.md` → Sub-agent re-entry). It composes
/// the **explicitly-named** sub-workflow `workflow_id`, **asserting it equals the
/// sub-task's recorded mint workflow** (`add-task`/`add-from-spec`'s `--workflow`)
/// before composing — so a stale launch template that names the wrong workflow fails
/// loudly rather than silently composing the wrong thing.
///
/// Distinct from [`resume_in_repo`] (`jigc start --task <id>`): resume takes no `<W>`
/// arg and recomposes the task's *own* recorded workflow with no equality assertion;
/// re-entry takes `<W>` and rejects a mismatch. Both share the same read/compose
/// machinery ([`compose_task_workflow`]): the base-pin guard, the bound-roles context,
/// the fill-aware `workflow-refs` gate, and `compose_with_store`. This is the
/// read/compose half only — provisioning the write-ready area is a later task.
///
/// A **malformed** id is refused before the join as not an id at all
/// ([`crate::task::reject_malformed_work_unit_id`]; `design/structural-grammar.md` →
/// Work-units and runtime identity, resolution); a well-formed but nonexistent one
/// rejects with `no task \`<id>\``. A `<W>` ≠ the recorded workflow
/// rejects with a routed `workflow-refs.workflow-mismatch` block naming both ids + the
/// sub-task; a base mismatch rejects with the divergence-routing prompt.
pub fn reenter_in_repo(start: &Path, workflow_id: &str, id: &str) -> Result<Composition> {
    // Before anything joins `id` onto a path (M50 Inc 1 / T1).
    crate::task::reject_malformed_work_unit_id(id)?;
    let repo_root = discover_repo_root(start).ok_or_else(|| crate::locate::not_in_repo(start))?;
    let project_config = require_project_config(start)?;

    // The committed doc-store + `.jigc/` working area bind to **jigc_home** (the main
    // checkout) so a fanned sub-agent re-enters from its worktree against the project's
    // shared `.jigc/`; only the base-pin HEAD read below stays on the worktree
    // `repo_root` — the worktree HEAD == the milestone pin under WF4 (M31 Inc 2 / WF3).
    let jigc_home = jigc_home_or_repo(start)?;
    let jigc_root = jigc_home.join(".jigc");
    // The same cause and the same key as every other unknown-work-unit door
    // ([`crate::task::require_task_area`]; M51 Increment 6 / T1) — `finalize.no-task` at
    // `task:<id>`, answered as the findings envelope — with **this** door's roster as the
    // **absent** cell's route: a `jigc workflow <W> --task <id>` re-entry is a milestone
    // sub-agent's, and its sibling ids are listed by the milestone, not by `jigc task
    // list`. The **residual** cell's route is the producer's own and is the same at all
    // four seams: no roster answers a leftover directory (M53 Increment 3 / T3).
    let task_dir = crate::task::require_task_area(
        &jigc_home,
        id,
        engine::finding::Route::mechanical(
            ["jigc", "milestone", "list-tasks", "<milestone-id>"],
            " lists a milestone's sub-tasks",
        ),
    )?;

    // The task is pinned to its (milestone-shared) base; never operate it off it.
    let pinned = state::read_base_pin(&task_dir)
        .with_context(|| format!("could not read the base pin for task `{id}`"))?;
    let head = read_head(&repo_root)?;
    if pinned.sha != head.sha {
        // Which unit kind is refusing decides which recovery is the real one — the same
        // `owning_milestone` membership test the sibling resume door asks, read only on
        // divergence ([`blanket_base_pin_refusal`]). This door takes no membership decision
        // of its own, so a top-level task can reach it too, and answers `None`.
        let milestone_id = engine::milestone::owning_milestone(&jigc_root, id);
        return Err(blanket_base_pin_refusal(
            id,
            milestone_id.as_deref(),
            &pinned,
            &head,
            &jigc_home,
            &repo_root,
        ));
    }

    // The W-equality guard: the requested `<W>` must equal the sub-task's recorded
    // mint workflow. A mismatch (the fan-out / launch template names a workflow the
    // sub-task wasn't seeded for) is a blocking, routed finding — fail loud, never
    // compose the wrong thing (`design/write-commands.md` → Sub-agent re-entry: the
    // re-entry equality guard). A missing recorded workflow is the same clear fault
    // resume reports.
    let recorded = state::read_workflow_id(&task_dir)
        .with_context(|| format!("could not read the recorded workflow for `{id}`"))?
        .with_context(|| {
            // The full, parseable re-seed form — the old bare `jigc milestone add-task`
            // span never parsed (required args short); the T7 fence forces it honest.
            format!(
                "task `{id}` has no recorded workflow — discard it with {} and re-seed it with {}",
                engine::finding::Route::mechanical(["jigc", "task", "discard", id, "--force"], ""),
                engine::finding::Route::mechanical(
                    [
                        "jigc",
                        "milestone",
                        "add-task",
                        "<milestone-id>",
                        "\"<intent>\""
                    ],
                    "",
                ),
            )
        })?;
    if recorded != workflow_id {
        // Both `jigc workflow … --task …` spans go through the checked constructor (F4 —
        // the route-fence seam-sweep), so the parse fence proves them, exactly like the
        // sibling `discard` / `add-task` spans a few lines up. The rendered text is
        // unchanged (`Route`'s Display is the backticked span).
        let ran =
            engine::finding::Route::mechanical(["jigc", "workflow", workflow_id, "--task", id], "");
        let reentry = engine::finding::Route::mechanical(
            ["jigc", "workflow", recorded.as_str(), "--task", id],
            "",
        );
        return Err(finding_to_err(Finding::block(
            "workflow-refs.workflow-mismatch",
            format!(
                "{ran} names workflow `{workflow_id}`, but sub-task `{id}` was minted with `{recorded}` — re-entry must compose the recorded workflow"
            ),
            format!("re-run as {reentry}, or re-seed the sub-task with the intended `--workflow`"),
        )));
    }

    compose_task_workflow(
        ComposeCheckouts {
            jigc_home: &jigc_home,
            from: &repo_root,
        },
        &project_config,
        &task_dir,
        id,
        workflow_id,
        &head,
        true,
    )
}

/// The two checkouts a re-compose stands between ([`compose_task_workflow`]).
///
/// They are the **same directory** outside a linked worktree and differ inside one, and
/// the pair is carried together because keeping them apart is exactly what the compose
/// has to get right: every `.jigc/` read binds to [`jigc_home`](Self::jigc_home) (the
/// main checkout, so all worktrees of one project share a single workbench), while
/// [`from`](Self::from) — where the invocation actually ran — decides nothing but the
/// sub-task [`SubTaskPosture`] (M52 Inc 10 / T4).
struct ComposeCheckouts<'a> {
    /// **jigc_home** — the main checkout the `.jigc/` workbench and the committed
    /// doc-store bind to (`crate::repo::jigc_home`).
    jigc_home: &'a Path,
    /// The checkout the invocation was run **from** — the worktree root walked up from
    /// the caller's cwd. Inside a linked worktree this is not `jigc_home`.
    from: &'a Path,
}

/// Compose `workflow_id` over an existing task's working area — the read/compose
/// spine both [`resume_in_repo`] (the recorded workflow) and [`reenter_in_repo`]
/// (the explicitly-named, equality-guarded `<W>`) share. The caller has already
/// resolved the task dir, verified the base pin against `head`, and resolved
/// `workflow_id`; this reads the intent + bound roles, runs the fill-aware
/// `workflow-refs` gate, and composes over the committed store + edge overlay so a
/// context-slice over a persisted doc dereferences (`worked-examples.md` → Task 2).
/// Mints nothing. On the **re-entry** path (`provision == true`) it provisions the
/// write-ready area on first entry ([`provision_on_first_entry`], first-entry-only,
/// `creates-task`-gated); resume (`provision == false`) provisions nothing — its
/// top-level task was already provisioned at mint.
///
/// The [`ComposeCheckouts`] pair keeps *which `.jigc/`* and *where the caller stands*
/// apart: every read below binds to `at.jigc_home`, while `at.from` decides only the
/// sub-task [`SubTaskPosture`] — decided here, at the shared spine, so the two
/// re-compose doors cannot answer it differently.
fn compose_task_workflow(
    at: ComposeCheckouts<'_>,
    project_config: &Path,
    task_dir: &Path,
    id: &str,
    workflow_id: &str,
    head: &BasePin,
    provision: bool,
) -> Result<Composition> {
    let ComposeCheckouts {
        jigc_home: repo_root,
        from: composed_in,
    } = at;
    let intent = state::read_intent(task_dir)
        .with_context(|| format!("could not read intent for `{id}`"))?;
    let bound =
        RolesRecord::load(task_dir).with_context(|| format!("could not read roles for `{id}`"))?;

    let pack = make_pack()?;
    let pack = pack.as_ref();
    // Resolve the live cascade up front so the definition reads below route through
    // the phase-2 file owners (a project `workflows/<id>.yaml` / `schemas/<id>.yaml`
    // whole-file shadow wins) — resume composes over the *live* cascade exactly as
    // the fresh front door (`overrides.md` → whole-file definition shadow).
    let (resolved, overrides) = resolve_cascade(pack, project_config)?;
    let defs = CascadeDefs::new(&resolved, project_config);
    // A workflow the pack does not provide is the routed `workflow-refs.unknown-
    // workflow` block (not a generic "pack is missing" — the recorded/named id may
    // be a not-yet-shipped pack workflow, e.g. `sub-task` before increment 5).
    let workflow_bytes = defs.read_workflow(pack, workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    // Provision-on-first-entry — the sub-agent re-entry's deferred mirror of mint-time
    // provisioning. Gated on `provision` (re-entry only) and idempotent inside (first
    // entry only, `creates-task`-gated), so a resume / a `creates-task: false` `<W>` /
    // a second re-entry over an edited doc all leave the area untouched.
    if provision {
        provision_on_first_entry(pack, &defs, &def, task_dir, id)?;
    }
    // The command catalog is read against the resumed workflow's **origin pack** — the
    // constituent that defines `workflow_id`'s top-level id — exactly as the fresh
    // front door ([`compose_core`]): a loser-pack workflow's `{{cli.X}}` resolves
    // against ITS OWN pack's catalog on resume, never the precedence winner's (which
    // need not be a superset). Without this, mint ≠ resume — a resumed loser-pack
    // workflow would silently re-resolve to the winner's body (`multi-pack.md` →
    // Pack-local body-reference resolution → Command-refs). Inert for a single pack
    // (origin = pack), so the resume floor stays byte-identical.
    let origin = pack.origin_pack(PackResourceKind::Workflows, &ResourceId::from(workflow_id));
    let commands = load_catalog(origin)?;
    let selectable = selectable_workflows(pack, &defs)?;
    // The committed store feed (`{{store.<doctype>}}`), enumerated from the committed
    // `<location>/<slug>.md` instances; the same `schemas` set the edge overlay below
    // resolves `<type>` prefixes against — resolved through the cascade so a project
    // `schemas/<id>.yaml` shadow wins.
    let schemas = defs.all_schemas(pack)?;
    let store_feed = committed_store(repo_root, &schemas);
    // The empty-research advisory decision (same as the fresh front door), computed
    // before `store_feed` is moved into the context — so a resumed vision-forming
    // workflow strips (or keeps) the `author-vision` advisory block identically.
    let advise_research = warrants_research_advisory(&def, &store_feed);

    // The source seam re-feeds from the persisted artifact: `jigc migrate` stages the
    // foreign bytes at `<task_dir>/source`, and the resume / re-entry compose must
    // surface them through `{{source}}` exactly as the minting compose did — the
    // serial-collision route ("resume with `jigc start --task <id>`") lands here, so
    // an unfed seam rendered the foreign content EMPTY (M43 Inc 6 T4; `DECISIONS.md`
    // 2026-07-16 M43 Settle → the migrate resume re-feeds the persisted source).
    // Absent on every non-migration task (nothing stages the file), so those resumes
    // feed `None` and stay byte-identical.
    // The schema feed clones: `schemas` is still needed below for the committed-store
    // index + the `ContentStore` wiring.
    let staged_source = read_staged_source(task_dir, id)?;
    let ctx = build_context(
        id,
        &intent,
        &def,
        &bound,
        selectable,
        store_feed,
        staged_source.as_deref(),
        schemas.clone(),
    );
    // Resume reads steps through the layer-aware [`CascadeStepSource`] over the *live*
    // cascade (a project `steps/<id>.yaml` whole-file shadow wins) and scopes its
    // pack-default arm to the resumed workflow's origin pack — so every
    // `{{include: step:X}}` resolves against the pack that DEFINES the workflow, not
    // the precedence winner. This mirrors the fresh front door's
    // `scope_to_workflow` ([`compose_drained`]); without it a resumed loser-pack
    // workflow would expand the winner's divergent step (the M3-class silent
    // corruption `multi-pack.md` → Pack-local body-reference resolution → Steps kills).
    // Inert for a single pack (origin = pack), so the resume floor stays byte-identical.
    let source = CascadeStepSource::new(pack, &resolved, project_config);
    source.scope_to_workflow(workflow_id);

    // Resume composes the task's pinned workflow, but still over the *live* cascade
    // (`resolved` / `overrides`, resolved up front so the definition reads route
    // through it) — a project `slot-fill` (or the pack's empty default for an unfilled
    // `{{fill:<id>}}` point) applies at phase 5 on re-compose exactly as on the fresh
    // front door, else a `{{fill:}}` point would survive resume to phase 8 unresolved.
    // The fill-aware gate runs the same M4 orphan + survivor checks (`overrides.md` →
    // The `{{fill:}}` placeholder); a no-fill cascade is the identity.
    let findings = compose::workflow_refs_with_fills(
        &workflow_bytes,
        &[],
        &overrides.slot_fills,
        &overrides.fills,
        &source,
        &commands,
        &ctx,
    );
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    // Wire the committed store + edge overlay so a context-slice over a persisted ADR
    // (`{{@task.decision.supersedes#decision}}`) dereferences to the prior decision's
    // prose — the superseding-decision read path (`worked-examples.md` → Task 2). The
    // CLI locates the layers (committed-store root + the `.jigc/` index home), the
    // engine resolves through the `ContentStore` trait (`VISION.md` principle #4).
    let jigc_root = repo_root.join(".jigc");
    let committed = index::load_committed(repo_root, &jigc_root, &schemas, &head.sha);
    let overlay = index::overlay_working(&committed, task_dir, &schemas);
    let store = StoreContext {
        repo_root,
        schemas: &schemas,
        overlay: &overlay,
    };

    // Phase 5 — apply the cascade's slot-fills to each fetched step body before
    // include expansion, identical to the fresh-compose path ([`compose_core`]): an
    // unfilled `{{fill:<id>}}` collapses to its empty pack default, a filled one
    // splices the project content. An empty `fills` is the identity, so the no-fill
    // resume stays byte-identical to the pre-`{{fill:}}`-point baseline.
    let filled = FillStepSource {
        inner: &source,
        fills: &overrides.fills,
        ctx: &ctx,
    };
    let composed = compose::compose_with_store(&def, &filled, &commands, &ctx, Some(&store))
        .map_err(finding_to_err)?;
    Ok(Composition {
        view: ComposedWorkflow {
            // Resume / reenter re-composes the given task, so its id is the in-hand handle
            // surfaced in `--format json` (`command-output-contract.md` §1).
            task: Some(id.to_string()),
            text: resolve_research_advisory(composed.text, advise_research),
        },
        // A resumed task carries the same gates its workflow granted at mint — the
        // announcement re-shows on every re-compose, exactly like the routing footer.
        gates: create_gates(&def),
        // ...but the *mint* announcement does not: an earlier invocation minted this id,
        // and the caller supplied it. A `task minted:` header here would state a mint that
        // did not happen (`design/workflow-dialect.md` → The `task minted:` header).
        minted: false,
        // The unit kind of the id being re-composed, read at the **shared** spine so both
        // re-compose doors (resume and sub-agent re-entry) carry it — the same
        // `owning_milestone` membership test the sub-task finalize refusal is minted from,
        // never a second rule. The `task scope:` footer scopes its parallel-work claim on
        // it: a sub-task's commit boundary is the milestone's, so the top-level
        // "resuming or finalizing here blocks and names the overlapping paths" would be a
        // law-1 lie there (M47 Inc 8 / N7; `design/surface-contract.md` → law 1).
        sub_task_of: engine::milestone::owning_milestone(&jigc_root, id).map(|milestone| {
            SubTaskOf {
                milestone,
                // The composed workflow id — the recorded one on both doors (see
                // [`SubTaskOf::workflow`]), so the `resume:` line's re-entry span is the
                // one the re-entry guard admits.
                workflow: workflow_id.to_string(),
                // Where this invocation stands, decided once for both doors
                // ([`SubTaskPosture`]).
                posture: sub_task_posture(composed_in, repo_root, id),
            }
        }),
        // A re-compose door (resume / sub-agent re-entry) already names its task: the
        // reader is *in* the work, not looking for it.
        also_open: Vec::new(),
        // **But the amend target it does carry** (the F-10 review's MEDIUM-2), read from the
        // area's own marker at the one shared re-compose spine — so both doors that re-compose
        // a task (`jigc start --task`, `jigc workflow <W> --task`) name the commit being
        // repaired, exactly as the mint does, and neither can drift from the other.
        amend: amend_target_of(repo_root, task_dir, None)?,
    })
}

/// Decide a sub-task compose's [`SubTaskPosture`]: is `composed_in` — the checkout the
/// invocation ran from — the sub-task's **own** worktree, the one `jigc milestone
/// provision` cut at `<jigc_home>/.jigc/worktrees/<id>`?
///
/// Both sides are canonicalized before the comparison, because the two paths arrive by
/// different routes — `composed_in` is walked up from the caller's cwd while the
/// worktree path is built from jigc_home — and on macOS the temp roots those walks start
/// in are symlinks (`/var` → `/private/var`). A path that will not canonicalize (it was
/// removed under the running process) falls back to its own bytes, so the comparison
/// still answers rather than failing the compose: the posture decides one line's *where*
/// clause, never whether the door opens.
fn sub_task_posture(composed_in: &Path, jigc_home: &Path, id: &str) -> SubTaskPosture {
    let real = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    if real(composed_in) == real(&jigc_home.join(engine::milestone::worktree_path(id))) {
        SubTaskPosture::OwnWorktree
    } else {
        SubTaskPosture::ElsewhereInProject
    }
}

/// Read a task's persisted source-seam artifact (`<task_dir>/source`, staged by
/// `jigc migrate` at mint — [`engine::state::SOURCE_FILE`]) if present, so a resumed
/// or re-entered migration task re-feeds [`ComposeContext::source`] with the same
/// foreign bytes the minting compose fed (`auto-migration.md` → The source seam).
/// `None` when the file is absent — every non-migration task — keeping those
/// composes byte-identical. A present-but-unreadable artifact is a hard error, not a
/// silent `None`: composing the migration workflow with an empty seam is exactly the
/// defect this read exists to close.
fn read_staged_source(task_dir: &Path, id: &str) -> Result<Option<String>> {
    let path = task_dir.join(engine::state::SOURCE_FILE);
    if !path.exists() {
        return Ok(None);
    }
    std::fs::read_to_string(&path)
        .map(Some)
        .with_context(|| format!("could not read the staged foreign source for `{id}`"))
}

/// Enumerate the committed managed store into the `store` data-value feed: for every
/// persisted schema, list its committed instances (through the shared placement-aware
/// [`index::committed_instances`] enumerator — `storage.md` → Placement census) and key
/// them by the doctype's path-facing **collection name**. Each value is the committed
/// instances as `<type>:<slug>` addresses, in sorted (stable) order.
///
/// The collection key, one per doctype ([workflow-dialect.md] → data-value roots,
/// `store.<doctype-id>`):
///
/// - a **located** doctype keys by its location's trimmed **final** path component
///   (`specs/` → `specs`, `docs/specs/` → `specs`) — exactly the `store.specs` the
///   `locate-from-spec` step interpolates. Keying by the final segment keeps
///   `{{store.<name>}}` stable under a `docs-root` prefix (`DECISIONS.md` 2026-06-18 —
///   the pre-existing deviation from the doctype-id spelling, kept).
/// - a **placement** doctype (`location: None`, homed at a literal `placement.file` —
///   `storage.md` → Placement) keys by its **type id**: it has no location stem to key
///   by, and its collection is the one-element `[<type>:<type>]` singleton list at that
///   literal home (M42 — the census's 14th site: before it, the placement class fell
///   through the transient arm below, so `{{store.changelog}}` rendered *empty text and
///   never a finding* over a committed `CHANGELOG.md`). As with a located doctype's
///   empty directory, an **absent** singleton contributes no key at all.
/// - a **transient** (location-less, non-placement) doctype — e.g. `commit` — contributes
///   nothing; it is never persisted as a repo file.
///
/// **One doctype per key is assumed**, not enforced — the inherited stance of the located
/// case (`target_surface.rs` records the same assumption for `location:`); two placement
/// doctypes cannot collide here, since a type id is unique by construction.
///
/// This is the **CLI-locates** half of the determinism split (`VISION.md` principle #4):
/// the CLI walks the committed working tree (the same surface `index::rebuild_committed`
/// walks); the engine resolver consumes the fed map and does no committed-store I/O.
fn committed_store(
    repo_root: &Path,
    schemas: &BTreeMap<String, Schema>,
) -> BTreeMap<String, Vec<Address>> {
    let mut store: BTreeMap<String, Vec<Address>> = BTreeMap::new();
    for schema in schemas.values() {
        let key = if schema.placement.is_some() {
            schema.ty.clone()
        } else {
            let Some(location) = schema.location.as_deref() else {
                continue; // a transient (location-less) type has no committed instances.
            };
            let stem = location.trim_matches('/').rsplit('/').next().unwrap_or("");
            if stem.is_empty() {
                continue;
            }
            stem.to_owned()
        };
        let addresses: Vec<Address> = index::committed_instances(repo_root, &schema.ty, schema)
            .iter()
            .filter_map(|(identity, _path)| Address::parse(identity).ok())
            .collect();
        if !addresses.is_empty() {
            store.insert(key, addresses);
        }
    }
    store
}

/// Build the [`ComposeContext`] for the composed workflow.
///
/// Binds the engine-native `task.id`/`task.intent` scalars, the task's **commit**
/// role to `commit:<id>` (the always-present sink of a `creates-task` work-
/// workflow — its `<<author: {{task.commit#summary}}>>` slot must resolve to that
/// address), and declares each `allows-create` **and** `reads` role from the
/// workflow front-matter. A role **bound** in `roles` (persisted in the task's
/// `roles.json` once the agent created the doc through the create-gate or ran
/// `jigc task bind` for a `reads` role) binds to its recorded address; an
/// **unbound** declared role declares as `None`, so `{{@task.<role>.…}}` resolves
/// to absent/empty (`design/workflow-dialect.md` → Empty vs unresolvable). At mint
/// `roles` is empty (nothing is bound yet); on `--task <id>` resume it carries
/// the binds recorded since (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding
/// at create).
// Each engine-native context feed pins to one argument (the [`ComposeContext`]
// fields it binds); bundling them would obscure the seam — the same call
// [`compose_core`] makes on its argument list.
#[allow(clippy::too_many_arguments)]
fn build_context(
    id: &str,
    intent: &str,
    def: &WorkflowDef,
    bound: &RolesRecord,
    catalog: Vec<CatalogEntry>,
    store: BTreeMap<String, Vec<Address>>,
    seam: Option<&str>,
    schemas: BTreeMap<String, Schema>,
) -> ComposeContext {
    let mut roles: BTreeMap<String, Option<Address>> = BTreeMap::new();
    // The task's commit doc — the engine-native sink, bound to `commit:<id>`.
    let commit =
        Address::parse(&format!("commit:{id}")).expect("`commit:<slug>` is a valid address");
    roles.insert("commit".to_owned(), Some(commit));
    // Each `allows-create` role is declared; bound iff `roles.json` recorded it.
    for entry in &def.allows_create {
        let binding = bound
            .get(&entry.as_role)
            .and_then(|addr| Address::parse(addr).ok());
        roles.entry(entry.as_role.clone()).or_insert(binding);
    }
    // Each `reads` role — the dual of `allows-create` — is declared the same way:
    // bound iff `roles.json` recorded a `jigc task bind` for it, else `None` so
    // `{{@task.<role>.…}}` resolves to absent/empty until bound
    // (`workflow-dialect.md` → `reads`: declared-but-unbound resolves to empty).
    for entry in &def.reads {
        let binding = bound
            .get(&entry.role)
            .and_then(|addr| Address::parse(addr).ok());
        roles.entry(entry.role.clone()).or_insert(binding);
    }
    ComposeContext {
        task: Some(TaskRoot {
            id: id.to_owned(),
            intent: intent.to_owned(),
            roles,
        }),
        // The selectable-workflow catalog, fed identically to the no-task arm so a
        // `creates-task: true` workflow that interpolates `{{catalog}}` resolves
        // the same list (it never lists itself — the filter excludes it only when
        // it is `creates-task: false`).
        catalog,
        // The committed managed store, fed by the CLI so `{{store.<doctype>}}`
        // resolves to the committed instances (the determinism boundary; the
        // engine resolver does no committed-store I/O).
        store,
        // No milestone in this single-`start` compose path (see the no-task arm) —
        // neither the sub-task collection nor the bound work-unit id.
        milestone: Vec::new(),
        milestone_id: None,
        // The CLI-owned source seam — the staged foreign bytes fed by `jigc migrate`
        // at mint and re-fed from the persisted `<task_dir>/source` on resume /
        // re-entry; `None` whenever no source is staged — every non-migration compose
        // (`auto-migration.md` → The source seam).
        source: seam.map(str::to_owned),
        // The composed cascade's resolved doctype set (`CascadeDefs::all_schemas` —
        // per-origin-pack, post-`docs-root`), keyed by type id — the
        // `{{schema:<doctype>}}` projection's feed, so the rendered homes are
        // resolved by construction (`surface-contract.md` → The schema projection).
        schemas,
    }
}

/// A single-layer [`StepSource`] that resolves step ids directly against one pack,
/// with no cascade/origin scoping. The live compose paths (fresh + resume) now read
/// steps through the layer-aware, origin-scoped [`CascadeStepSource`], so this plain
/// source survives only as a test fixture for the engine compose seam.
#[cfg(test)]
pub(crate) struct PackStepSource<'a> {
    pub(crate) pack: &'a dyn PackSource,
}

#[cfg(test)]
impl StepSource for PackStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let bytes = self
            .pack
            .read(PackResourceKind::Steps, &ResourceId::from(id))
            .ok()?;
        load_step_def(id, &bytes).ok()
    }
}

/// A phase-5 [`StepSource`] decorator: it fetches each step from `inner` (the
/// layer-aware cascade source) and applies the cascade's slot-fill content to the
/// body **before** returning it ([`compose::apply_slot_fills`]) — so include
/// expansion (phase 7) sees the filled body, and the filled content's own
/// `{{include:}}` / `{{cli.…}}` / `{{@…}}` resolve in the later phases as if the
/// pack had written them inline (`overrides.md` → Resolution algorithm phase 5; The
/// `{{fill:}}` placeholder).
///
/// `apply_slot_fills` is keyed `(step_id, fill_id)`, so applying it per fetched step
/// is exactly the right granularity. A no-fill cascade (`fills` empty) is the
/// identity: every `{{fill:}}` point resolves to its pack default (empty in M4), so
/// the body passes through byte-for-byte and the no-override path stays byte-
/// identical. Phase 5 does **not** re-run; a `{{fill:}}` nested in applied content
/// survives, already blocked by the `fill-survivor` gate check above.
///
/// Before splicing fills, the fetched **step-file body** gets the inline
/// data-value pass ([`resolve_inline_data_values`]) — pack/shadow-authored
/// composed prose substitutes its mid-line `{{task.…}}` tokens (the M17 inc-4
/// `--task` stamping), while the fill content spliced *after* is agent/project
/// prose the CLI never rewrites (`DECISIONS.md` 2026-06-12 — the determinism
/// boundary applied).
struct FillStepSource<'a> {
    inner: &'a dyn StepSource,
    fills: &'a ResolvedFills,
    ctx: &'a ComposeContext,
}

impl StepSource for FillStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let def = self.inner.step(id)?;
        // Inline data-values resolve in the step-file body only — before phase 5,
        // so fill-applied prose is never scanned.
        let stamped = resolve_inline_data_values(&def.body, self.ctx);
        // The pass has no failure of its own in M4 (orphan/survivor is the gate's
        // job, already run); on the unreachable `Err` the unfilled body is kept.
        let body = apply_slot_fills(id, &stamped, self.fills).unwrap_or(stamped);
        // The fill pass rewrites only the body; the step kind (and the
        // `states-constraints:` declaration) passes through.
        Some(StepDef {
            id: def.id,
            body,
            kind: def.kind,
            states_constraints: def.states_constraints,
        })
    }
}

/// A layer-aware [`StepSource`]: it consults the resolved cascade's phase-2 by-id
/// shadowing surface ([`cascade::Resolved::file_owner`]) to read each step from the
/// **highest-precedence layer that owns the id** — a project
/// `.jigc/config/steps/<id>.yaml` shadows the pack-default step, else the pack body
/// is read (`overrides.md` → Resolution algorithm phase 2). This replaces the
/// single-layer [`PackStepSource`] on the live compose read path: phase-2 by-id
/// shadowing wired in at last.
///
/// **Located-error sink.** The [`StepSource`] contract is `Option<StepDef>`, so a
/// fault cannot ride the return value. A `file_owner` of `Project` whose on-disk
/// file is **missing or malformed** is not a silent dangling-include nor a
/// fall-through to the pack body: `step()` records a clear located [`Finding`] into
/// an interior sink and returns `None`. The compose caller drains it via
/// [`CascadeStepSource::take_error`] after a `None` to surface the located fault.
/// This is the live compose step source (T4): the two front doors build it over
/// the resolved cascade + the project `steps/` dir and drain its sink after a
/// failed compose, so a missing/malformed project step surfaces its located fault
/// rather than the generic dangling-include message.
pub(crate) struct CascadeStepSource<'a> {
    pack: &'a dyn PackSource,
    resolved: &'a cascade::Resolved,
    /// The project layer's committed config dir — where `steps/<id>.yaml` lives.
    project_config: &'a Path,
    /// The **pack of origin** the pack-default step arm reads from — the
    /// constituent pack that defines the *composing workflow*'s top-level id, so a
    /// loser-pack workflow's `{{include: step:X}}` resolves against **its own**
    /// pack rather than the precedence-winner's divergent step (`multi-pack.md` →
    /// Pack-local body-reference resolution → Steps). `None` until `compose_core`
    /// scopes it; the read then falls back to `pack` (the composite itself), which
    /// for a single pack *is* the origin — so the cold-start floor is inert.
    origin: std::cell::Cell<Option<&'a dyn PackSource>>,
    /// The located fault recorded by the most recent `step()` that returned `None`
    /// for a reason other than "no layer owns the id".
    error: std::cell::RefCell<Option<Finding>>,
}

impl<'a> CascadeStepSource<'a> {
    /// Build the layer-aware source over the resolved cascade + the project config
    /// dir the `steps/<id>.yaml` files live under.
    pub(crate) fn new(
        pack: &'a dyn PackSource,
        resolved: &'a cascade::Resolved,
        project_config: &'a Path,
    ) -> Self {
        Self {
            pack,
            resolved,
            project_config,
            origin: std::cell::Cell::new(None),
            error: std::cell::RefCell::new(None),
        }
    }

    /// Take the located fault recorded by the last failing `step()`, clearing the
    /// sink. `None` when the last `step()` returned `Some` or returned `None`
    /// because no layer owns the id (a plain dangling include the engine reports).
    fn take_error(&self) -> Option<Finding> {
        self.error.borrow_mut().take()
    }

    /// The layer-aware **definition** read surface over the same resolved cascade +
    /// project config dir this step source carries — so `compose_core` reads the
    /// workflow / schema definitions through the cascade (whole-file shadow) exactly
    /// as it reads steps (`CascadeDefs`; `overrides.md` → whole-file definition shadow).
    fn defs(&self) -> CascadeDefs<'a> {
        CascadeDefs::new(self.resolved, self.project_config)
    }

    /// Read + parse a project-owned step file from `<project_config>/steps/<id>.yaml`,
    /// recording a located fault on a missing or malformed file (so the owning layer
    /// is never silently abandoned for the pack body). The missing-file fault is
    /// routed at its recovery (the route floor, M43 completion audit) — a two-branch
    /// human judgment: either the owned file was deleted out-of-band (restore it) or
    /// the project layer's claim on the id is stale (drop the `deltas:` entry /
    /// shadow that references it); which side is broken is the human's call.
    fn project_step(&self, id: &str) -> Option<StepDef> {
        let path = self.project_config.join("steps").join(format!("{id}.yaml"));
        let bytes = match std::fs::read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.record(engine::finding::Finding::graded(
                    engine::finding::Severity::Blocking,
                    "overrides.project-step-missing",
                    format!(
                        "project layer owns step `{id}` but its file {} is unreadable: {e}",
                        path.display()
                    ),
                    Some(engine::finding::Location::at(1, 1)),
                    Some(engine::finding::Route::human(format!(
                        "restore the project step file {}, or drop the project layer's \
                         claim on `{id}` (remove the `deltas:` entry or shadow that \
                         references it)",
                        path.display()
                    ))),
                ));
                return None;
            }
        };
        match load_step_def(id, &bytes) {
            Ok(def) => Some(def),
            Err(finding) => {
                self.record(finding);
                None
            }
        }
    }

    fn record(&self, finding: Finding) {
        *self.error.borrow_mut() = Some(finding);
    }
}

impl StepSource for CascadeStepSource<'_> {
    /// Scope the pack-default step read to the **origin pack** of the composing
    /// workflow — the constituent that defines `workflow_id`'s top-level id, so
    /// every `{{include: step:X}}` this workflow expands resolves against its own
    /// pack ([`PackSource::origin_pack`]; `multi-pack.md` → Pack-local
    /// body-reference resolution → Steps). `compose_core` calls this once per
    /// compose (and the store sweep once per enumerated workflow), before any
    /// `step()` runs. For a single pack the origin *is* the pack, so the scoped
    /// read is byte-identical to the unscoped one (the floor).
    fn scope_to_workflow(&self, workflow_id: &str) {
        self.origin.set(Some(self.pack.origin_pack(
            PackResourceKind::Workflows,
            &ResourceId::from(workflow_id),
        )));
    }

    fn step(&self, id: &str) -> Option<StepDef> {
        match self.resolved.file_owner(id) {
            // The project owns the id → read the project file (a missing/malformed
            // file is a located fault, never a fall-through to the pack body).
            Some(cascade::LayerKind::Project) => self.project_step(id),
            // Pack-default owns it, or no layer does — read the pack body from the
            // composing workflow's **origin pack** (the constituent that defines the
            // workflow's id), so a loser-pack workflow expands ITS OWN pack's step,
            // never the precedence-winner's divergent one (`multi-pack.md` →
            // Pack-local body-reference resolution). Unscoped (a single pack, or
            // before `scope_to_workflow`) the origin *is* `self.pack`, so the read
            // is byte-identical. A pack-unknown id is a plain dangling include the
            // engine reports.
            _ => {
                let origin = self.origin.get().unwrap_or(self.pack);
                let bytes = origin
                    .read(PackResourceKind::Steps, &ResourceId::from(id))
                    .ok()?;
                load_step_def(id, &bytes).ok()
            }
        }
    }
}

/// Read the pack's own cascade id from its `config/defaults` `pack-id` field —
/// the identity the [`PackDefaultLayer`] carries (`overrides.md`: the pack-default
/// layer names itself; `pack-id` is identity, never a knob).
fn pack_id_from_config(pack: &dyn PackSource) -> Result<String> {
    let bytes = read_pack(pack, PackResourceKind::Config, "defaults")?;
    let text = String::from_utf8(bytes).context("`config/defaults` is not UTF-8")?;
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).context("`config/defaults` is not valid YAML")?;
    value
        .get("pack-id")
        .and_then(serde_yaml_ng::Value::as_str)
        .map(str::to_owned)
        .context("`config/defaults` declares no `pack-id`")
}

/// The project cascade layer plus the delta surfaces a manifest's `deltas:` list
/// carries: the [`OverrideLayer`], the phase-4 `structural-op` [`StructuralDelta`]s,
/// the phase-5 `slot-fill` [`SlotFillDelta`]s, the `tracked-fork`
/// [`TrackedForkDelta`]s, and the M5 [`StructuralBasis`] records for any
/// `replace-step` / `remove-step` that carries a recorded `base-version`/`base-hash`.
///
/// The last two are **recording surface only**, read by the (M5) reconciliation —
/// never fed to compose. The `StructuralBasis` rides a separate vector keyed by the
/// delta's target (design-review B2), so the phase-4 `StructuralDelta`s — hence the
/// composed output — are untouched whether or not a basis was recorded.
type ProjectLayer = (
    OverrideLayer,
    Vec<StructuralDelta>,
    Vec<SlotFillDelta>,
    Vec<TrackedForkDelta>,
    Vec<StructuralBasis>,
);

/// Load the project cascade layer from `<project_config>/manifest.yaml` plus the
/// project's native `steps/` dir, returning the [`OverrideLayer`] paired with the
/// manifest's phase-4 `structural-op` deltas, phase-5 `slot-fill` deltas, and
/// `tracked-fork` recording deltas (see [`ProjectLayer`]).
///
/// The layer carries three surfaces:
/// - **`scalar:`** — each entry becomes one [`OverrideLayer::scalar_set`], recorded
///   **in manifest order** (`overrides.md` → phase 3 / Within-layer manifest order),
///   flipping a knob like `default-workflow` the cascade reads on the compose path.
/// - **shadowed files** — every `steps/<id>.yaml` basename is declared via
///   [`OverrideLayer::shadow_file`], so [`cascade::Resolved::file_owner`] returns
///   `Project` for an id the project ships (phase-2 by-id shadowing). A native step
///   file shadows the pack step regardless of any delta (the `steps/` dir *is* the
///   project's step layer).
/// - **`deltas:`** — one entry per delta, split by `kind:`: the `structural-op`
///   kinds (`insert-step` / `replace-step` / `remove-step`) parse into
///   [`StructuralDelta`]s for the phase-4 pass; a `slot-fill` kind parses into a
///   [`SlotFillDelta`] for the phase-5 fill pass + the orphan check; a
///   `tracked-fork` kind parses into a [`TrackedForkDelta`] — **recording surface
///   only**, not fed to compose (at compose time a fork is just the shadowed step
///   file phase 2 already applies; the recorded `base-version` + `base-hash` are
///   read solely by the (M5) reconciliation — `overrides.md` → `tracked-fork` hash
///   basis). All three ride the one `deltas:` list (`storage.md` → Config layout)
///   and are returned separately because they mutate different surfaces at
///   different phases.
///
/// A **missing** manifest file (the dir may still hold `steps/` shadows), or a
/// present manifest with a **missing or blank** `scalar:` / `deltas:` block, yields
/// the no-override path — present-but-empty, byte-identical to today. A non-string
/// scalar value (`true`, `3`) is recorded as its YAML scalar string (opaque here).
///
/// The closed-surface check (an undeclared scalar key is an error) is **not** done
/// here: it is the resolver's job, so a hand-edited manifest and a `config set`
/// write are adjudicated by the one path.
pub(crate) fn load_project_layer(project_config: &Path) -> Result<ProjectLayer> {
    let manifest = project_config.join("manifest.yaml");
    // Every native `steps/<id>.yaml`, `workflows/<id>.yaml`, and `schemas/<id>.yaml`
    // basename shadows the pack definition of that id at phase 2, independent of the
    // manifest — the three dirs *are* the project's definition layer (`overrides.md`
    // → Authored metadata on a definition resolves by whole-file shadow). All three
    // feed the **one** `file_owners` map `cascade::resolve` keys by bare id, so a
    // bare id appearing in more than one dir would silently collapse to a single
    // owner — that namespace collision is rejected up front (one id, one file).
    let shadowed = shadowed_definition_ids(project_config)?;
    // Stamp the committed-config path (provenance header) + the shadowed ids.
    let with_files = |mut layer: OverrideLayer| {
        layer = layer.config_path(project_config.display().to_string());
        for id in &shadowed {
            layer = layer.shadow_file(id);
        }
        layer
    };

    let text = match std::fs::read_to_string(&manifest) {
        Ok(text) => text,
        // No manifest file → no scalar/delta override (the `steps/` shadows still apply).
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok((
                with_files(OverrideLayer::empty()),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ));
        }
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };
    let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("{} is not valid YAML", manifest.display()))?;

    let mut layer = OverrideLayer::empty();
    // The `scalar:` block. A **blank** block (parsed as null) is the no-override
    // path — not an error — so present-but-empty leaves the layer empty.
    match doc.get("scalar") {
        None | Some(serde_yaml_ng::Value::Null) => {}
        Some(scalar) => {
            let map = scalar.as_mapping().with_context(|| {
                format!(
                    "{}: `scalar:` must be a map of knob keys",
                    manifest.display()
                )
            })?;
            for (key, value) in map {
                let key = key.as_str().with_context(|| {
                    format!("{}: `scalar:` keys must be strings", manifest.display())
                })?;
                layer = layer.scalar_set(key, scalar_to_string(value));
            }
        }
    }

    // The `deltas:` block — one list carrying the phase-4 `structural-op`, the
    // phase-5 `slot-fill`, the (M5-read) `tracked-fork` recording kind, plus the M5
    // `replace`/`remove` base-hash basis records (blank/absent → none of any).
    let (deltas, slot_fills, forks, bases) = match doc.get("deltas") {
        None | Some(serde_yaml_ng::Value::Null) => (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        Some(seq) => parse_deltas(seq, &manifest)?,
    };

    Ok((with_files(layer), deltas, slot_fills, forks, bases))
}

/// Resolve one step's body **through the cascade** — the phase-2 file-owner read a
/// `CascadeStepSource` does, as a free function for the write-time `config fill`
/// point-exists check (`overrides.md` → the `jigc config` verbs: "the
/// `{{fill:<fill-id>}}` point exists in the resolved step body"). A project
/// `steps/<id>.yaml` shadows the pack step (phase-2 by-id shadowing); else the pack
/// body is read. A step id no layer owns yields `Ok(None)` (the caller routes it as
/// an absent point); a project-owned file that is missing/malformed is a clear,
/// path-bearing error (never a silent fall-through to the pack body).
///
/// This builds only the phase-2 [`cascade::Resolved`] (knobs + the project layer's
/// `steps/` shadows), not the full [`ComposeOverrides`] — the check needs the file
/// owner, not the resolved fills.
pub(crate) fn resolve_step_body(
    pack: &dyn PackSource,
    project_config: &Path,
    step_id: &str,
) -> Result<Option<engine::compose::StepDef>> {
    let (project, _deltas, _slot_fills, _forks, _bases) = load_project_layer(project_config)?;
    let resolved = resolve_layers(pack, &project)?;

    match resolved.file_owner(step_id) {
        // The project owns the id → read the project file (missing/malformed is a
        // path-bearing error, never a fall-through to the pack body).
        Some(cascade::LayerKind::Project) => {
            let path = project_config.join("steps").join(format!("{step_id}.yaml"));
            let bytes = std::fs::read(&path).with_context(|| {
                format!(
                    "project layer owns step `{step_id}` but its file {} is unreadable",
                    path.display()
                )
            })?;
            Ok(Some(
                load_step_def(step_id, &bytes).map_err(finding_to_err)?,
            ))
        }
        // Pack-default owns it, or no layer does — read the pack body. A pack-unknown
        // id has no body to resolve (the caller routes the absent point).
        _ => match pack.read(PackResourceKind::Steps, &ResourceId::from(step_id)) {
            Ok(bytes) => Ok(Some(
                load_step_def(step_id, &bytes).map_err(finding_to_err)?,
            )),
            Err(_) => Ok(None),
        },
    }
}

/// List the native step ids the project layer ships — every `<project_config>/
/// steps/<id>.yaml` basename. These shadow the pack step of the same id at phase 2
/// (`overrides.md` → Native-file id = filename basename). A missing `steps/` dir is
/// the empty list (no project shadows), never an error.
pub(crate) fn project_step_ids(project_config: &Path) -> Vec<String> {
    project_native_ids(project_config, "steps")
}

/// List the native workflow ids the project layer ships — every `<project_config>/
/// workflows/<id>.yaml` basename. These shadow the pack workflow of the same id at
/// phase 2 (whole-file definition shadow — `overrides.md` → Authored metadata on a
/// definition resolves by whole-file shadow). A missing `workflows/` dir is the
/// empty list (no project shadows), never an error.
pub(crate) fn project_workflow_ids(project_config: &Path) -> Vec<String> {
    project_native_ids(project_config, "workflows")
}

/// List the native schema (doctype) ids the project layer ships — every
/// `<project_config>/schemas/<id>.yaml` basename. These shadow the pack doctype of
/// the same id at phase 2 (whole-file definition shadow — `overrides.md` → Authored
/// metadata on a definition resolves by whole-file shadow). A missing `schemas/`
/// dir is the empty list (no project shadows), never an error.
pub(crate) fn project_schema_ids(project_config: &Path) -> Vec<String> {
    project_native_ids(project_config, "schemas")
}

/// The merged, deduplicated set of every id the project layer shadows across its
/// three definition dirs (`steps/`, `workflows/`, `schemas/`) — the `files` surface
/// fed to [`OverrideLayer::shadow_file`] (`overrides.md` → Authored metadata on a
/// definition resolves by whole-file shadow).
///
/// **Namespace guard.** `cascade::resolve` builds **one** `file_owners` map keyed by
/// the bare id, shared across step / workflow / schema ids. So the same bare id in
/// two of these dirs (e.g. `steps/foo.yaml` + `workflows/foo.yaml`) would both
/// `shadow_file("foo")` and silently collapse to a single owner — an ambiguous
/// "which definition owns `foo`?". That is rejected here with a routed blocking
/// `config.shadow-id-collision` [`Finding`] (one id, one file). A disjoint set
/// merges cleanly, sorted (stable).
fn shadowed_definition_ids(project_config: &Path) -> Result<Vec<String>> {
    let mut owner: BTreeMap<String, &'static str> = BTreeMap::new();
    for (dir, ids) in [
        ("steps", project_step_ids(project_config)),
        ("workflows", project_workflow_ids(project_config)),
        ("schemas", project_schema_ids(project_config)),
    ] {
        for id in ids {
            if let Some(prior) = owner.insert(id.clone(), dir) {
                return Err(finding_to_err(Finding::block(
                    "config.shadow-id-collision",
                    format!(
                        "project layer shadows id `{id}` in both `{prior}/` and `{dir}/` — definition ids share one namespace, so one id maps to one file"
                    ),
                    format!(
                        "rename one of the two `{id}.yaml` shadow files so each id is owned by a single definition dir, then re-run"
                    ),
                )));
            }
        }
    }
    Ok(owner.into_keys().collect())
}

/// List the `<id>.yaml` basenames under `<project_config>/<subdir>/` — the shared
/// lister behind [`project_step_ids`] / [`project_workflow_ids`] /
/// [`project_schema_ids`] (`overrides.md` → Native-file id = filename basename). A
/// missing dir is the empty list (no project shadows), never an error; the result
/// is sorted (stable).
fn project_native_ids(project_config: &Path, subdir: &str) -> Vec<String> {
    let dir = project_config.join(subdir);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut ids: Vec<String> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("yaml"))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_owned))
        .collect();
    ids.sort();
    ids
}

/// The four delta surfaces [`parse_deltas`] partitions a `deltas:` list into — the
/// phase-4 `structural-op` deltas, the phase-5 `slot-fill` deltas, the
/// `tracked-fork` recordings, and the M5 `replace`/`remove` [`StructuralBasis`]
/// records — i.e. [`ProjectLayer`] minus its [`OverrideLayer`].
type DeltaSurfaces = (
    Vec<StructuralDelta>,
    Vec<SlotFillDelta>,
    Vec<TrackedForkDelta>,
    Vec<StructuralBasis>,
);

/// Parse the manifest's `deltas:` sequence, **partitioned by `kind:`** into the
/// phase-4 `structural-op` [`StructuralDelta`]s and the phase-5 `slot-fill`
/// [`SlotFillDelta`]s (`overrides.md` → Delta representation / Delta targets;
/// `storage.md` → Config layout: one `deltas:` list, every kind). A `structural-op`
/// entry carries a `kind` (`insert-step` / `replace-step` / `remove-step`), a
/// `target`, and — for insert/replace — a `with:` step reference; a `slot-fill`
/// entry carries `kind: slot-fill`, a `target: step:<id>#<fill-id>`, and a
/// `content: fills/<id>.md`; a `tracked-fork` entry carries `kind: tracked-fork`, a
/// `target: workflow:<id>#<step-id>`, a `base-version:`, and a `base-hash:`. A
/// `replace-step` / `remove-step` may **optionally** carry a `base-version:` +
/// `base-hash:` pair (the M5 basis) — when both are present they parse into a
/// separate [`StructuralBasis`] record keyed by the delta's target, leaving the
/// `StructuralDelta` itself unchanged. A malformed entry surfaces as its located
/// finding.
fn parse_deltas(seq: &serde_yaml_ng::Value, manifest: &Path) -> Result<DeltaSurfaces> {
    let items = seq
        .as_sequence()
        .with_context(|| format!("{}: `deltas:` must be a list of deltas", manifest.display()))?;
    let mut structural = Vec::new();
    let mut slot_fills = Vec::new();
    let mut forks = Vec::new();
    let mut bases = Vec::new();
    for item in items {
        match parse_one_delta(item, manifest)? {
            ParsedDelta::Structural(delta, basis) => {
                structural.push(delta);
                bases.extend(basis);
            }
            ParsedDelta::SlotFill(delta) => slot_fills.push(delta),
            ParsedDelta::TrackedFork(delta) => forks.push(delta),
        }
    }
    Ok((structural, slot_fills, forks, bases))
}

/// One parsed `deltas:` entry, partitioned by kind — a phase-4 structural-op, a
/// phase-5 slot-fill, or a `tracked-fork` recording (`overrides.md` → The ladder:
/// distinct kinds, distinct phases). The fork is recording-only — not fed to
/// compose; its basis is read solely by the (M5) reconciliation.
enum ParsedDelta {
    /// A phase-4 `structural-op`, plus — for a `replace-step`/`remove-step` that
    /// recorded the M5 basis — the [`StructuralBasis`] keyed by its target (the
    /// separate recording surface, design-review B2; an insert / a basis-less
    /// replace/remove carries `None`).
    Structural(StructuralDelta, Option<StructuralBasis>),
    SlotFill(SlotFillDelta),
    TrackedFork(TrackedForkDelta),
}

/// Parse one `deltas:` entry into a [`ParsedDelta`], branching on `kind:`. A
/// `with:`/`target:` value may carry a `step:` prefix (the manifest spelling —
/// flow 3a's `with: step:project-implement`); the bare step id is what the include
/// list holds. A `target`'s `after:`/`before:` anchor is read from sibling keys. A
/// `slot-fill` entry's `content: fills/<id>.md` yields the native file's basename id.
fn parse_one_delta(item: &serde_yaml_ng::Value, manifest: &Path) -> Result<ParsedDelta> {
    let at = |key: &str| item.get(key).and_then(serde_yaml_ng::Value::as_str);
    let kind = at("kind")
        .with_context(|| format!("{}: a `deltas:` entry needs a `kind`", manifest.display()))?;
    let target_str = at("target")
        .with_context(|| format!("{}: a `deltas:` entry needs a `target`", manifest.display()))?;

    match kind {
        "replace-step" => {
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            let step = with_step(at("with"), kind, manifest)?;
            let basis =
                optional_basis(at("base-version"), at("base-hash"), &target, kind, manifest)?;
            Ok(ParsedDelta::Structural(
                StructuralDelta::Replace { target, step },
                basis,
            ))
        }
        "remove-step" => {
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            let basis =
                optional_basis(at("base-version"), at("base-hash"), &target, kind, manifest)?;
            Ok(ParsedDelta::Structural(
                StructuralDelta::Remove { target },
                basis,
            ))
        }
        "insert-step" => {
            let anchor = match (at("after"), at("before")) {
                (Some(a), None) => AnchorSpec::After(a.to_owned()),
                (None, Some(b)) => AnchorSpec::Before(b.to_owned()),
                (Some(_), Some(_)) => bail!(
                    "{}: an `insert-step` needs exactly one of `after:`/`before:`",
                    manifest.display()
                ),
                (None, None) => bail!(
                    "{}: an `insert-step` needs an `after:` or `before:` anchor",
                    manifest.display()
                ),
            };
            let target =
                StructuralTarget::parse(target_str, Some(anchor)).map_err(finding_to_err)?;
            let step = with_step(at("with"), kind, manifest)?;
            Ok(ParsedDelta::Structural(
                StructuralDelta::Insert { target, step },
                None,
            ))
        }
        "slot-fill" => {
            let target = SlotFillTarget::parse(target_str).map_err(finding_to_err)?;
            let content_id = content_id(at("content"), manifest)?;
            Ok(ParsedDelta::SlotFill(SlotFillDelta { target, content_id }))
        }
        "tracked-fork" => {
            // `workflow:<id>#<step-id>` → an `Anchor::At` target naming the forked
            // unit (the native step file shadowed whole at phase 2). The recorded
            // basis (`base-version` + the blake3 `base-hash`) is M4's pinned
            // ancestor — read only by the (M5) compare; both are required, so a
            // missing field is a clear, located error here, never a panic.
            let target = StructuralTarget::parse(target_str, None).map_err(finding_to_err)?;
            let base_version = required_field(at("base-version"), "base-version", kind, manifest)?;
            let base_hash = required_field(at("base-hash"), "base-hash", kind, manifest)?;
            Ok(ParsedDelta::TrackedFork(TrackedForkDelta {
                target,
                base_version,
                base_hash,
            }))
        }
        other => bail!(
            "{}: unknown delta kind `{other}` (expected insert-step / replace-step / remove-step / slot-fill / tracked-fork)",
            manifest.display()
        ),
    }
}

/// The native fill file's basename id from a `slot-fill`'s `content:` value
/// (`content: fills/extra-guidance.md` → `extra-guidance`), the same
/// "id = filename basename" rule the `steps/` dir uses (`storage.md` → Config
/// layout, `fills/<id>.md`). A missing `content:` is a clear error.
fn content_id(raw: Option<&str>, manifest: &Path) -> Result<String> {
    let raw = raw.with_context(|| {
        format!(
            "{}: a `slot-fill` needs a `content:` file reference",
            manifest.display()
        )
    })?;
    let basename = Path::new(raw)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .with_context(|| {
            format!(
                "{}: a `slot-fill` `content:` must name a `fills/<id>.md` file; got `{raw}`",
                manifest.display()
            )
        })?;
    Ok(basename.to_owned())
}

/// A required string `field` off a `deltas:` entry (`tracked-fork`'s `base-version`
/// / `base-hash`) — a missing field is a clear, located error naming the field and
/// the entry's `kind`, never a panic (`overrides.md` → `tracked-fork` hash basis:
/// the recorded basis must be present to round-trip).
fn required_field(raw: Option<&str>, field: &str, kind: &str, manifest: &Path) -> Result<String> {
    let raw =
        raw.with_context(|| format!("{}: a `{kind}` needs a `{field}`", manifest.display()))?;
    Ok(raw.to_owned())
}

/// The **optional** M5 base-hash basis on a `replace-step` / `remove-step` entry —
/// the displaced pack unit's recorded ancestor (`overrides.md` → Per-kind base-hash
/// basis). The pair is **all-or-nothing**: both keys present → a [`StructuralBasis`]
/// keyed by the delta's `target`; both absent → `None` (an M4-written delta carried
/// no basis — backward-compat, never an error); exactly one present → a clear,
/// located error naming the missing field (a half-recorded basis cannot round-trip
/// into the M5 compare), never a panic.
fn optional_basis(
    base_version: Option<&str>,
    base_hash: Option<&str>,
    target: &StructuralTarget,
    kind: &str,
    manifest: &Path,
) -> Result<Option<StructuralBasis>> {
    match (base_version, base_hash) {
        (None, None) => Ok(None),
        (Some(base_version), Some(base_hash)) => Ok(Some(StructuralBasis {
            target: target.clone(),
            base_version: base_version.to_owned(),
            base_hash: base_hash.to_owned(),
        })),
        (None, Some(_)) => bail!(
            "{}: a `{kind}` with a `base-hash` also needs a `base-version` (the basis pair is all-or-nothing)",
            manifest.display()
        ),
        (Some(_), None) => bail!(
            "{}: a `{kind}` with a `base-version` also needs a `base-hash` (the basis pair is all-or-nothing)",
            manifest.display()
        ),
    }
}

/// The bare step id from a `with:` reference, stripping the manifest's optional
/// `step:` prefix (`with: step:project-implement` → `project-implement`). A
/// missing `with:` on an insert/replace is a clear error.
fn with_step(raw: Option<&str>, kind: &str, manifest: &Path) -> Result<String> {
    let raw = raw.with_context(|| {
        format!(
            "{}: a `{kind}` needs a `with:` step reference",
            manifest.display()
        )
    })?;
    Ok(raw.strip_prefix("step:").unwrap_or(raw).to_owned())
}

/// Render a YAML scalar `value` to the opaque string the cascade stores. A string
/// scalar passes through verbatim; a bool / int / other scalar renders to its YAML
/// text (`true`, `3`). A non-scalar (map / sequence) is not a valid knob value and
/// renders to its YAML form, which the resolver / `check_value` then rejects.
fn scalar_to_string(value: &serde_yaml_ng::Value) -> String {
    match value {
        serde_yaml_ng::Value::String(s) => s.clone(),
        serde_yaml_ng::Value::Bool(b) => b.to_string(),
        serde_yaml_ng::Value::Number(n) => n.to_string(),
        other => serde_yaml_ng::to_string(other)
            .unwrap_or_default()
            .trim_end()
            .to_owned(),
    }
}

/// Load and parse the pack's command catalog (`config/commands`).
pub(crate) fn load_catalog(pack: &dyn PackSource) -> Result<CommandCatalog> {
    let bytes = read_pack(pack, PackResourceKind::Config, "commands")?;
    load_command_catalog(&bytes).map_err(finding_to_err)
}

/// The layer-aware **definition** read surface: the resolved cascade's phase-2
/// by-id shadowing ([`cascade::Resolved::file_owner`]) plus the project layer's
/// committed config dir, so a project `workflows/<id>.yaml` / `schemas/<id>.yaml`
/// **whole-file shadow** wins over the pack definition (`overrides.md` → Authored
/// metadata on a definition resolves by whole-file shadow; Resolution algorithm
/// phase 2). The dual of [`CascadeStepSource`] for the *definition* reads
/// reads ([`CascadeDefs::read_workflow`] / [`CascadeDefs::all_schemas`]) the way
/// that source is the dual for the *step* reads — compose must reflect the cascade
/// or it would drift from `describe`.
///
/// **Whole-file REPLACE, never field-merge.** When the project owns the id, the
/// project file's bytes are read and parsed in full; the pack definition is not
/// consulted (`overrides.md` → No partial field-level merge across layers).
///
/// **Project-owned-but-missing/malformed = located fault**, never a silent
/// fall-through to the pack: unlike [`CascadeStepSource`] (whose `Option` contract
/// can't carry a fault) these reads return `Result`, so the fault rides the error
/// directly — the `CascadeStepSource` located-error precedent without the interior
/// sink.
/// Prefix every persisted (location-bearing) schema's `location:` with the resolved
/// `docs-root` (default `docs/`) — the **single** hygiene point for the managed-doc
/// parent dir (`design/storage.md` → Config layout; `DECISIONS.md` 2026-06-18). Applied
/// at every schema-load surface (`CascadeDefs::all_schemas`, `ingest::load_schemas`,
/// `TaskCtx::schemas`) so the read and write paths resolve the *same* parent.
///
/// An empty value (`""`) or `.` restores the flat repo-root layout (locations
/// untouched). The location keeps its **trailing slash** (`decisions/` →
/// `docs/decisions/`) — `file_state` concatenates `{location}{slug}.md` directly, so the
/// slash must survive. Transient (location-less) schemas are untouched. The empty-value +
/// slash hygiene lives here only.
///
/// `docs-root` is an **opt-in cascade knob**: it nests only where the resolved cascade
/// actually carries the key. The dev pack declares it (default `docs/`), so any dev-pack
/// project nests. A pack whose closed knob surface omits `docs-root` (e.g. the
/// methodology pack, whose owner-artifact home is the flat engine const
/// `completions/artifacts/`) resolves `None` here and stays flat — `unwrap_or("")` keeps
/// docs-root from silently nesting a pack that never opted in.
pub(crate) fn apply_docs_root<'a>(
    resolved: &cascade::Resolved,
    schemas: impl IntoIterator<Item = &'a mut Schema>,
) {
    let root = docs_root_prefix(resolved);
    if root.is_empty() {
        return; // flat repo-root layout — no prefix.
    }
    for schema in schemas {
        if let Some(location) = schema.location.as_deref() {
            schema.location = Some(format!("{root}/{location}"));
        }
    }
}

/// The resolved `docs-root` prefix, normalized: trailing/leading slashes stripped and the
/// flat forms (`""` / `.` / absent) canonicalized to `""`. The **single** normalization
/// point shared by [`apply_docs_root`] (the schema-load surfaces) and the `migrate-corpus`
/// relocation walk-home (a relocated doctype's committed instances still sit under this
/// prefix at the prior `location:` home, but the prior *snapshot* stores that location raw,
/// so the walk must re-apply the prefix — `crate::migrate_corpus::candidate_docs`).
pub(crate) fn docs_root_prefix(resolved: &cascade::Resolved) -> &str {
    let root = resolved.scalar("docs-root").unwrap_or("").trim_matches('/');
    if root == "." { "" } else { root }
}

/// Re-root every **placement** schema's literal home under the resolved `placement-root`
/// — the placement twin of [`apply_docs_root`], and the only path an adopter has to a
/// `placement:` doctype's home at all (`design/storage.md` → Placement; `DECISIONS.md` →
/// the M49 Settle, D6).
///
/// The asymmetry it closes: a `location:` doctype's home resolves through `docs-root`, a
/// `placement:` doctype's resolves through nothing — the literal `placement.file` is what
/// ships — and since M38 that literal is inside the doctype's frozen `schema-hash`, so
/// `jigc relocate` refuses to move it. An adopter whose managed docs do not live under
/// `docs/` had no path to `docs/roadmap.md`: not a knob, not a verb.
///
/// **The rule is also the scope answer.** A declared home carrying a leading directory
/// component (`docs/roadmap.md`) resolves to `<placement-root>/<remainder>`; a home
/// declared **at the repo root** (`VISION.md`, `CHANGELOG.md`) has no leading component
/// and is never re-rooted. The ecosystem-idiomatic root files are unburiable by
/// *derivation*, so there is no doctype allow-list here that could go stale.
///
/// Composition-invariance — the property `storage.md` → Placement exists to protect —
/// survives: the override is **one fixed project value**, so a placement home still does
/// not vary by which packs composed. That only holds while the knob is declared in every
/// shipped pack (a knob declared in one is invisible under the whole-file `knobs.yaml`
/// shadow), which `cli::pack::placement_root_is_declared_in_both_packs_identically` pins.
///
/// Unlike [`apply_docs_root`] this re-root **replaces** rather than prefixes, so it is
/// idempotent; it is still applied at the one schema-resolution seam only, beside its
/// twin, so both packs' doctypes get one home from one place.
pub(crate) fn apply_placement_root<'a>(
    resolved: &cascade::Resolved,
    schemas: impl IntoIterator<Item = &'a mut Schema>,
) {
    let root = placement_root(resolved);
    if root.is_none() {
        return; // unset — every declared home stands.
    }
    for schema in schemas {
        let Some(placement) = schema.placement.as_mut() else {
            continue; // a located or transient doctype — `docs-root`'s business, not ours.
        };
        placement.file = reroot_placement_file(&placement.file, root);
    }
}

/// The home a **declared** `placement.file` resolves to under `root` — the whole re-root
/// rule, in one place, so the two callers that need it cannot drift apart:
/// [`apply_placement_root`] (the schema-resolution seam) and the `config set
/// placement-root` move floor (`crate::config`), which computes a doctype's *prior* and
/// *current* home by applying this to the same declaration under the old and the new value.
///
/// `None` is **unset** — the declaration stands. Otherwise a declared home carrying a
/// leading directory component re-roots to `<root>/<remainder>` (`""` being the repo root,
/// so the component is simply dropped); a home declared **at** the repo root has no leading
/// component and is returned unchanged, which is the ecosystem-idiomatic rule.
///
/// **Always apply this to the DECLARATION, never to an already-resolved home.** The two
/// look interchangeable and are not: once the root is `.` a resolved home has no leading
/// component left, so re-rooting *it* under a new value is a silent no-op — which, on the
/// move floor, computes a destination equal to the source and strands the doc it was
/// supposed to move.
pub(crate) fn reroot_placement_file(declared: &str, root: Option<&str>) -> String {
    let Some(root) = root else {
        return declared.to_owned();
    };
    let Some((_, remainder)) = declared.split_once('/') else {
        return declared.to_owned();
    };
    if root.is_empty() {
        remainder.to_owned()
    } else {
        format!("{root}/{remainder}")
    }
}

/// The resolved `placement-root`, normalized — `None` when the knob is **unset**, else the
/// parent dir a nested placement home re-roots under (`""` being the repo root).
///
/// The two empties are deliberately *not* the same value here, which is where the
/// vocabulary parts company with [`docs_root_prefix`]: the pack default is `""`, meaning
/// **unset** (every declared home stands, byte-identical to a build with no knob), while
/// the repo root is spelled `.`. `jigc config set placement-root ""` canonicalizes to `.`
/// on the write path (`crate::config`), so the intuitive spelling reaches the repo root
/// and the empty string stays reserved for "never set". A pack whose closed knob surface
/// omits `placement-root` resolves `None` here and stays declared, the same opt-in shape
/// `apply_docs_root` has.
pub(crate) fn placement_root(resolved: &cascade::Resolved) -> Option<&str> {
    normalize_placement_root(resolved.scalar("placement-root").unwrap_or(""))
}

/// Normalize a **raw** `placement-root` value to [`placement_root`]'s vocabulary: strip
/// surrounding slashes, `""` is **unset** (`None`), `.` is the repo root (`Some("")`).
///
/// The raw-string sibling of [`placement_root`] (which normalizes a *resolved* value) — the
/// `config set` move floor needs it because the new value is not yet in the cascade, exactly
/// as `config::normalize_docs_root` is the raw sibling of [`docs_root_prefix`].
pub(crate) fn normalize_placement_root(raw: &str) -> Option<&str> {
    match raw.trim_matches('/') {
        "" => None,
        "." => Some(""),
        root => Some(root),
    }
}

pub(crate) struct CascadeDefs<'a> {
    resolved: &'a cascade::Resolved,
    /// The project layer's committed config dir — where `workflows/<id>.yaml` and
    /// `schemas/<id>.yaml` shadow files live.
    project_config: &'a Path,
}

impl<'a> CascadeDefs<'a> {
    pub(crate) fn new(resolved: &'a cascade::Resolved, project_config: &'a Path) -> Self {
        Self {
            resolved,
            project_config,
        }
    }

    /// `true` iff the project layer owns the file with this id after phase-2
    /// shadowing — the read should come from `<project_config>/<dir>/<id>.yaml`.
    fn project_owns(&self, id: &str) -> bool {
        matches!(
            self.resolved.file_owner(id),
            Some(cascade::LayerKind::Project)
        )
    }

    /// Read a project-owned definition file `<project_config>/<dir>/<id>.yaml`,
    /// mapping a missing/malformed file to a located fault (the owning layer is
    /// never silently abandoned for the pack body).
    fn project_def(&self, dir: &str, id: &str) -> Result<Vec<u8>> {
        let path = self.project_config.join(dir).join(format!("{id}.yaml"));
        std::fs::read(&path).with_context(|| {
            format!(
                "project layer owns `{id}` but its {dir} shadow {} is unreadable",
                path.display()
            )
        })
    }

    /// The `pack-id` of the pack that **actually provided** the definition `id` of
    /// `kind`, or `None` when no pack did — the attribution `jigc describe` carries on
    /// every projected definition (`engine::introspect::DefinitionProse::origin_pack`).
    ///
    /// Two ways to answer `None`, and both are real answers rather than missing ones:
    /// the **project layer** owns the id (a whole-file shadow — the bytes the projection
    /// narrates came from `<project_config>/`, so no pack provided them), or **no
    /// constituent ships it** at all. The pack lookup goes through `origin_packs`, not
    /// `origin_pack`, precisely for the second case: `origin_pack` falls back to `self`
    /// for an id nobody owns, which would report an unowned id as pack-provided, while
    /// `origin_packs` is empty there and non-empty **winner-first** everywhere else.
    pub(crate) fn origin_pack_id(
        &self,
        pack: &dyn PackSource,
        kind: PackResourceKind,
        id: &str,
    ) -> Option<String> {
        if self.project_owns(id) {
            return None;
        }
        pack.origin_packs(kind, &ResourceId::from(id))
            .first()
            .map(|owner| owner.own_pack_id())
    }

    /// Read a named workflow's bytes through the cascade — the project
    /// `workflows/<id>.yaml` shadow when the project owns the id, else the pack
    /// definition (the Form-D unknown-`<X>` rejection rides the pack miss).
    pub(crate) fn read_workflow(&self, pack: &dyn PackSource, id: &str) -> Result<Vec<u8>> {
        if self.project_owns(id) {
            return self.project_def("workflows", id);
        }
        read_workflow(pack, id)
    }

    /// Read **one** doctype's schema through the cascade — the single production
    /// schema-resolution seam, and the only place in `crates/cli/src` that reads schema
    /// bytes for a *document* (fenced by
    /// `tests/schema_resolution_unified.rs`; the pack layer's own reads — the loader, the
    /// manifest, the freeze gate — are a different question and stay in `pack.rs`).
    ///
    /// The project `schemas/<id>.yaml` whole-file shadow wins when the project owns the
    /// id, else the pack definition (no field-merge). **Docs-root is deliberately NOT
    /// applied here**: both callers apply it exactly once over the schemas they return,
    /// and [`apply_docs_root`] prefixes rather than sets, so applying it twice would nest
    /// the home under the root twice over.
    fn read_one(&self, pack: &dyn PackSource, id: &str) -> Result<Schema> {
        let bytes = if self.project_owns(id) {
            self.project_def("schemas", id)?
        } else {
            read_pack(pack, PackResourceKind::Schemas, id)?
        };
        // A schema's field-type names are body-references resolved against the
        // **origin pack** — the constituent that defines this schema's top-level
        // id — so a loser-pack doctype's `code-anchor` resolves from ITS OWN
        // pack's `field-types.yaml`, never the precedence-winner's (which need
        // not declare it). Same origin lookup the step/command-ref reads perform;
        // for a single pack (or a project-owned id, whose `field_owner` made the
        // composite return its only constituent) the origin *is* `pack`, so the
        // read is byte-identical (`multi-pack.md` → Pack-local body-reference
        // resolution → Field-types are the same rule, one definition kind over).
        let origin = pack.origin_pack(PackResourceKind::Schemas, &ResourceId::from(id));
        crate::pack::load_pack_schema(origin, &bytes)
            .with_context(|| format!("the `{id}` schema is malformed"))
    }

    /// One doctype's cascade-resolved schema — its `location:` nested under the resolved
    /// `docs-root`, its `placement:` home re-rooted under the resolved `placement-root` —
    /// the single-doctype twin of [`all_schemas`](Self::all_schemas), for the surfaces that
    /// address one document rather than sweep the store.
    pub(crate) fn schema(&self, pack: &dyn PackSource, ty: &str) -> Result<Schema> {
        let mut schema = self.read_one(pack, ty)?;
        apply_docs_root(self.resolved, std::iter::once(&mut schema));
        apply_placement_root(self.resolved, std::iter::once(&mut schema));
        Ok(schema)
    }

    /// Every cascade-resolved schema keyed by doctype, **as declared** — the whole-file
    /// project shadow resolved, but neither `docs-root` nor `placement-root` applied.
    ///
    /// The raw read [`all_schemas`](Self::all_schemas) is built on, exposed because the
    /// `config set placement-root` move floor must compute a doctype's prior **and** current
    /// home from the same declaration ([`reroot_placement_file`] — applying the rule to an
    /// already-resolved home is the silent-strand trap that helper documents). Every other
    /// caller wants `all_schemas`: a resolved home is what the store reads and writes.
    pub(crate) fn declared_schemas(
        &self,
        pack: &dyn PackSource,
    ) -> Result<BTreeMap<String, Schema>> {
        let mut out = BTreeMap::new();
        for id in pack.list(PackResourceKind::Schemas) {
            let schema = self.read_one(pack, id.as_str())?;
            out.insert(schema.ty.clone(), schema);
        }
        Ok(out)
    }

    /// Load every cascade-resolved schema keyed by doctype: for each doctype the
    /// pack ships, read the project `schemas/<id>.yaml` shadow when the project
    /// owns the id, else the pack definition (whole-file shadow, no field-merge).
    pub(crate) fn all_schemas(&self, pack: &dyn PackSource) -> Result<BTreeMap<String, Schema>> {
        let mut out = self.declared_schemas(pack)?;
        // Surface A: nest every persisted doctype's `location:` under the resolved
        // `docs-root` before returning — covers `describe`, `start`/compose, and the
        // `committed_store` sweep (all read schemas through here).
        apply_docs_root(self.resolved, out.values_mut());
        // Surface B: the placement twin — a `placement:` doctype has no `location:` for
        // `docs-root` to touch, and its literal home is the only one no knob could reach.
        apply_placement_root(self.resolved, out.values_mut());
        Ok(out)
    }
}

/// **The one schema resolution**, for a caller that holds only the project layer's path.
///
/// Every production surface that needs a doctype schema for a *document* resolves it
/// here or through [`CascadeDefs`] directly — one answer per doctype, so no two doors of
/// the binary can disagree about the same file. Seven surfaces used to read the pack
/// directly (`design/overrides.md` → Resolution algorithm phase 2), which made a project
/// `schemas/<ty>.yaml` shadow invisible to them: `jigc ingest` reported a document
/// `jigc validate` was adjudicating as an `adr` as matching no schema at all, finalize
/// promoted to a home `jigc doc show` could not read, and `jigc doc create` minted a
/// blank skeleton over committed prose (M49 Increment 3 T2).
pub(crate) fn resolved_schemas(
    pack: &dyn PackSource,
    project_config: &Path,
) -> Result<BTreeMap<String, Schema>> {
    let resolved = resolve_severity_cascade(pack, project_config)?;
    CascadeDefs::new(&resolved, project_config).all_schemas(pack)
}

/// One doctype's cascade-resolved schema — the single-doctype twin of
/// [`resolved_schemas`].
pub(crate) fn resolved_schema(
    pack: &dyn PackSource,
    project_config: &Path,
    ty: &str,
) -> Result<Schema> {
    let resolved = resolve_severity_cascade(pack, project_config)?;
    CascadeDefs::new(&resolved, project_config).schema(pack, ty)
}

/// One workflow's **cascade-resolved** definition bytes — the workflow twin of
/// [`resolved_schema`], and the read every consumer of a task's **recorded** workflow
/// owes.
///
/// The recorded id names a *definition*, and which layer provides that definition is the
/// cascade's answer, not the pack's: a project `workflows/<id>.yaml` whole-file shadow
/// wins, and a project-layer id no pack ships resolves at all. Every composing door has
/// read it that way since M14 ([`CascadeDefs::read_workflow`]); the three doors that read
/// the *recorded* id — the create-gate, the `reads` declaration and the finalize seam —
/// read the **pack** until M52, so the workflow a task was minted on and the workflow its
/// gates enforced were two different documents (`overrides.md` → whole-file definition
/// shadow).
///
/// This is `TaskArea::schemas`' sibling one resource kind over: M49 Increment 3 T2 closed
/// exactly this divergence for **schemas** (a pack-only read named the staged copy at the
/// pack's home while the read surfaces looked at the resolved one), and left the workflow
/// kind un-swept.
pub(crate) fn resolved_workflow(
    pack: &dyn PackSource,
    project_config: &Path,
    id: &str,
) -> Result<Vec<u8>> {
    let resolved = resolve_severity_cascade(pack, project_config)?;
    let defs = CascadeDefs::new(&resolved, project_config);
    if defs.project_owns(id) {
        return defs.project_def("workflows", id);
    }
    // The pack branch is [`read_pack`], **not** [`read_workflow`]: the Form-D mapping to
    // `workflow-refs.unknown-workflow` answers a **caller-typed** `<X>`, where "list the
    // selectable work-workflows" is the repair. A *recorded* id the packs no longer ship
    // is a pack-resource miss, and its shipped answer names the packs actually searched
    // with a route that restores the file (`pack_resource_miss_axis` site 4). Routing it
    // through the Form-D miss would hand the reader `jigc start`, which restores nothing.
    read_pack(pack, PackResourceKind::Workflows, id)
}

/// Read a named workflow's bytes, mapping a **missing** workflow to a routed
/// blocking finding — the Form-D unknown-`<X>` rejection (`write-commands.md`: an
/// unknown `<X>` is rejected with a routed finding). Membership is the pack read:
/// a `PackError::NotFound` is the "not in the catalog" rejection. Fires before any
/// mint, so a typo'd id strands no task dir.
///
/// This is the **pack-only** read — the layer-aware definition read that routes a
/// project whole-file shadow through `file_owner` is [`CascadeDefs::read_workflow`].
pub(crate) fn read_workflow(pack: &dyn PackSource, id: &str) -> Result<Vec<u8>> {
    pack.read(PackResourceKind::Workflows, &ResourceId::from(id))
        .map_err(|_| {
            {
                // The bare `jigc start` span goes through the checked constructor (F4 — the
                // route-fence seam-sweep); the `--workflow <id> "<intent>"` span stays inline
                // (its quoted `"<intent>"` placeholder is not a flat-argv token). Text is
                // unchanged.
                let start = engine::finding::Route::mechanical(["jigc", "start"], "");
                finding_to_err(Finding::block(
                    "workflow-refs.unknown-workflow",
                    format!("no workflow `{id}` — list the selectable work-workflows with {start}"),
                    format!("run {start} to see the selectable work-workflows, then re-run `jigc start --workflow <id> \"<intent>\"`"),
                ))
            }
        })
}

/// Enumerate every cascade-resolved workflow definition as a [`StoreWorkflow`] bundle —
/// its id, its raw bytes, and its **origin-pack** command catalog — **address-sorted** by
/// workflow id, the store-scope `workflow↔refs` target's deterministic input
/// (`validation.md` → Completing the envelope: a deterministic, address-sorted enumeration
/// of every workflow definition the resolved cascade provides). Reads each id layer-aware
/// via [`CascadeDefs::read_workflow`] (a project whole-file shadow wins), the proven
/// `describe` enumeration idiom.
///
/// Each catalog resolves against the workflow's **origin pack** —
/// `pack.origin_pack(Workflows, id)` then [`load_catalog`] — so a loser-pack workflow's
/// `{{cli.X}}` refs are checked against ITS OWN pack's `commands.yaml`, never the
/// precedence-winner's (which need not be a superset). This is the same origin lookup the
/// emit-path catalog read and the step source's `scope_to_workflow` perform (`multi-pack.md`
/// → Pack-local body-reference resolution → Command-refs). For a single pack the origin *is*
/// `pack`, so each catalog is byte-identical to a flat read (the no-composition floor). The
/// bundles are validated per-definition by [`engine::validate::validate_store_families`];
/// this only collects them in order.
pub(crate) fn enumerate_store_workflows(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
) -> Result<Vec<engine::validate::StoreWorkflow>> {
    let mut ids: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .into_iter()
        .map(|id| id.as_str().to_owned())
        .collect();
    ids.sort();
    let mut out = Vec::with_capacity(ids.len());
    for id in &ids {
        let bytes = defs.read_workflow(pack, id)?;
        let origin = pack.origin_pack(PackResourceKind::Workflows, &ResourceId::from(id.as_str()));
        let catalog = load_catalog(origin)?;
        out.push(engine::validate::StoreWorkflow {
            id: id.clone(),
            bytes,
            catalog,
        });
    }
    Ok(out)
}

/// Read a pack resource by kind + id, mapping a missing resource to the **one** routed
/// refusal the composed pack-set can honestly give ([`pack_resource_missing`]).
///
/// **The single producer for the whole family** (M50 Increment 10 / T3): `config.rs`'s two
/// knob reads and `doc.rs`'s task-bound workflow read hand-copied this read and its message,
/// so one literal answered at four sites and every one of them blamed *"the embedded pack"*
/// — a pack that need not be in the composition at all. They call this now.
///
/// **The class was six read sites, not four.** T3's first pass swept the four that shared the
/// *"the embedded pack is missing"* literal and missed `orient.rs`'s two — private copies of
/// `pack_id_from_config` and the `config/knobs` scalar seed carrying a *different* bare
/// `with_context` (*"the pack must ship a `config/knobs` declaration"*) — so grepping the
/// literal found four and enumerating the **mechanism** finds six. The two missed ones sit
/// behind bare `jigc start`, the door the adjudication below names first, so the fix's own
/// rationale was false at its first clause until they were converted (the M45/M49
/// incomplete-sweep shape: a fence applied where its wave pointed is not applied at all).
/// Out of the mechanism with a stated reason: `config.rs::resolve_fork_bytes` already mints
/// its own routed `config.anchor-absent`; the pack-load fences in `pack.rs` read ids they got
/// from `list()`, so a failing read there is an I/O fault rather than a *no pack ships it*
/// miss; and the tolerant probes (`pack.rs`'s `doc_write_command_ids`, `resolve_step_body`'s
/// pack-body arm) map a miss to `None`, never to a refusal.
pub(crate) fn read_pack(
    pack: &dyn PackSource,
    kind: PackResourceKind,
    id: &str,
) -> Result<Vec<u8>> {
    pack.read(kind, &ResourceId::from(id))
        .map_err(|_| pack_resource_missing(pack, kind, id))
}

/// The refusal for a resource **no composed pack ships** — the code, the searched pack-set,
/// and the route the four `read_pack` doors had none of until M50 Increment 10 / T3.
///
/// **It names what was searched, never the precedence winner.** The decomposition proposed
/// splitting the sites — the *origin pack* here, the pack *set* elsewhere — on
/// [`PackSource::origin_pack`]. Driven, that split does not hold: `origin_pack` falls back to
/// `self` when no constituent owns the id, so at a **miss** — the only state this finding
/// exists for — it names the winner rather than what was searched, which is the same lie one
/// word over. So this names **its receiver's own** provenance instead
/// ([`PackSource::provenance_segments`] zipped with [`PackSource::provenance_entries`] — the
/// ids/versions the `Pack:` header renders, each with its resolving path, highest-precedence
/// first), and no site is special-cased. That is honest *per door* rather than uniformly
/// composite, which is the sharper form of the split: [`load_catalog`] is handed a workflow's
/// **origin pack** by the pack-local body-reference rule, so a `config/commands` miss there
/// genuinely searched one pack and says so, while the two `config` doors and `doc`'s
/// create-gate hold the composite and name both (`DECISIONS.md` → 2026-09-07).
///
/// **The route is `Human` by adjudication, not by omission.** No jigc verb restores a pack
/// resource, and under a `config/knobs` miss `jigc start`, `describe`, `validate`, `upgrade`,
/// `ingest` and `doc list` all block on this very finding — so a mechanical route would
/// either repair nothing or hard-reject, which `design/surface-contract.md` names as law 2
/// failing one level down (and which M46's PT-1 caught once already). The printed pack roots
/// are what make it followable: they say where to put the file back.
///
/// The family flattens through [`finding_to_err`] at every site, so it projects no envelope
/// key and is declared **outside the envelope**
/// (`design/command-output-contract.md` → The membership test).
fn pack_resource_missing(pack: &dyn PackSource, kind: PackResourceKind, id: &str) -> anyhow::Error {
    let searched = pack
        .provenance_segments()
        .into_iter()
        .zip(pack.provenance_entries())
        .map(|((pack_id, version), entry)| format!("{pack_id}/{version} ({})", entry.path))
        .collect::<Vec<_>>()
        .join(", ");
    finding_to_err(Finding::block(
        "pack.resource-missing",
        format!(
            "no composed pack ships `{kind}/{id}` — searched, highest-precedence first: {searched}"
        ),
        engine::finding::Route::human(format!(
            "restore `{kind}/{id}.yaml` in a pack the composition already searches, or add the \
             pack that ships it to `.jigc/config/packs.yaml`"
        )),
    ))
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its **key** + message +
/// route — the same envelope minting already uses (a hard block is a blocking-severity
/// finding carrying a route, `DECISIONS.md` 2026-05-31).
///
/// **One shape, one site** (M50 Increment 10 / T1): this delegates to
/// [`crate::render::finding_error`], whose [`BlockedFinding`](crate::render::BlockedFinding)
/// `Display` **is** the house findings line — `severity · code — message`, the `at:` locus,
/// the `route:`. Until M50 the six modules that own a funnel each re-derived that shape, and
/// four of them (`describe` / `doc` / `start` / `task`) rendered `finding.message` **alone**: the code a driver keys on
/// reached `--format json` and never the text (`design/command-output-contract.md` → The
/// stable finding key; `design/surface-contract.md` → law 1). Carrying the finding rather
/// than only its rendering also lets the dispatch log the identity it prints
/// ([`crate::render::blocked_finding`]).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    crate::render::finding_error(&finding)
}

/// Read HEAD as a [`BasePin`] (full + short SHA) by shelling out to the user's
/// `git` (`DECISIONS.md` 2026-05-31 → Git invocation: shell out to the `git`
/// binary, not git2/gitoxide).
///
/// A repo with **no commits** (unborn HEAD) pins to the canonical empty-tree
/// sentinel instead of bailing, so every `creates-task` workflow runs pre-first-commit
/// and the first finalize diffs against the empty tree (`design/project-setup.md` →
/// Flow 2 hardening — zero-commit). The unborn case is detected distinctly via
/// [`crate::task::head_is_unborn`] (`git rev-parse --verify -q HEAD` exit 1, vs 128
/// for a genuinely broken/missing git), so a real git failure still bails.
fn read_head(repo_root: &Path) -> Result<BasePin> {
    if crate::task::head_is_unborn(repo_root)? {
        return Ok(BasePin::new(
            crate::task::EMPTY_TREE_SHA,
            crate::task::EMPTY_TREE_SHORT,
        ));
    }
    let sha = git_rev_parse(repo_root, &["rev-parse", "HEAD"])?;
    let short = git_rev_parse(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(BasePin::new(sha, short))
}

/// Run `git <args>` in `repo_root` and return the single trimmed line of stdout.
fn git_rev_parse(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    let line = String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")?
        .trim()
        .to_string();
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::cascade::Anchor;
    use std::fs;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-start-mint-{tag}-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
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

    /// Initialize a real git repo with one commit, returning its HEAD SHAs.
    fn init_repo_with_commit(root: &Path) -> (String, String) {
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout)
                .expect("utf-8")
                .trim()
                .to_string()
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test"]);
        fs::write(root.join("README.md"), "hello\n").expect("write file");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);
        let sha = git(&["rev-parse", "HEAD"]);
        let short = git(&["rev-parse", "--short", "HEAD"]);
        (sha, short)
    }

    /// The CLI integration done-criterion: minting under a real temp git repo
    /// creates the working dir and a base pin recording the repo's actual HEAD.
    #[test]
    fn mint_opens_working_dir_and_base_pin_under_a_temp_git_repo() {
        let repo = TempDir::new("repo");
        let (sha, short) = init_repo_with_commit(repo.path());

        let minted = mint_in_repo(repo.path(), "Add rate limiter", "single-task", None)
            .expect("mint succeeds");

        assert_eq!(minted.id, "add-rate-limiter");
        let dir = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter");
        assert!(dir.is_dir(), "working dir must appear under .jigc/tasks/");
        assert_eq!(minted.dir, dir);

        let pin = fs::read_to_string(dir.join("base.json")).expect("base pin written");
        let base: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(base.sha, sha, "base pin records HEAD's full SHA");
        assert_eq!(base.short, short, "base pin records HEAD's short SHA");

        // A serial re-mint of the same intent rejects, surfacing the route.
        let err = mint_in_repo(repo.path(), "Add rate limiter", "single-task", None)
            .expect_err("re-mint rejects");
        let msg = err.to_string();
        assert!(
            msg.contains("add-rate-limiter") && msg.contains("route:"),
            "serial collision must name the task and carry a route; got: {msg}"
        );
    }

    /// M43 T1 (`design/surface-contract.md` → The carryover gate): the `jigc start`
    /// task-minting door snapshots the **pre-task staged state** — a pre-staged add,
    /// modify, *and* delete all read back from the working area's
    /// `staged-snapshot.json`: the add + modify as `(path, staged blob)` entries
    /// (blobs asserted against the index's own `git rev-parse :<path>`), the
    /// staged `git rm` in the deletion set (the entry-blind A7 case). A mint over
    /// a **clean** index writes an *empty* snapshot — `Some(empty)`, distinct from
    /// the fail-open pre-M43 `None`.
    #[test]
    fn mint_snapshots_prestaged_add_modify_and_delete() {
        let repo = TempDir::new("snapshot");
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(repo.path())
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout)
                .expect("utf-8")
                .trim()
                .to_string()
        };
        init_repo_with_commit(repo.path());
        // Commit the files the foreign pre-staged modify + delete will touch.
        fs::write(repo.path().join("mod.txt"), "original\n").expect("write mod.txt");
        fs::write(repo.path().join("del.txt"), "doomed\n").expect("write del.txt");
        git(&["add", "mod.txt", "del.txt"]);
        git(&["commit", "-q", "-m", "seed"]);

        // The foreign pre-staged change-set: an add, a modify, and a delete.
        fs::write(repo.path().join("added.txt"), "new\n").expect("write added.txt");
        fs::write(repo.path().join("mod.txt"), "changed\n").expect("modify mod.txt");
        git(&["add", "added.txt", "mod.txt"]);
        git(&["rm", "-q", "del.txt"]);
        let added_blob = git(&["rev-parse", ":added.txt"]);
        let mod_blob = git(&["rev-parse", ":mod.txt"]);

        let minted = mint_in_repo(repo.path(), "Add rate limiter", "single-task", None)
            .expect("mint succeeds");
        let snapshot = engine::state::read_staged_snapshot(&minted.dir)
            .expect("snapshot reads")
            .expect("the mint door must write a staged snapshot");
        assert_eq!(
            snapshot.entries.get("added.txt"),
            Some(&added_blob),
            "the pre-staged add reads back as a (path, staged blob) entry"
        );
        assert_eq!(
            snapshot.entries.get("mod.txt"),
            Some(&mod_blob),
            "the pre-staged modify reads back as a (path, staged blob) entry"
        );
        assert_eq!(snapshot.entries.len(), 2, "nothing else is staged");
        assert!(
            snapshot.deletions.contains("del.txt"),
            "the pre-staged `git rm` reads back in the deletion set"
        );
        assert_eq!(snapshot.deletions.len(), 1);

        // A clean-index mint writes an *empty* snapshot (Some, not the pre-M43 None).
        git(&["reset", "-q", "--hard", "HEAD"]);
        let clean = mint_in_repo(repo.path(), "Second task", "single-task", None)
            .expect("clean-index mint succeeds");
        assert_eq!(
            engine::state::read_staged_snapshot(&clean.dir).expect("snapshot reads"),
            Some(engine::state::StagedSnapshot::default()),
            "a clean index snapshots as Some(empty)"
        );
    }

    /// M43 T1: the `jigc migrate` task-minting door snapshots the pre-task staged
    /// state too — a foreign pre-staged add is in the migration task's snapshot.
    #[test]
    fn migration_mint_snapshots_prestaged_state() {
        let repo = TempDir::new("migrate-snapshot");
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(repo.path())
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout)
                .expect("utf-8")
                .trim()
                .to_string()
        };
        init_repo_with_commit(repo.path());
        fs::write(repo.path().join("stray.txt"), "staged before the task\n")
            .expect("write stray.txt");
        git(&["add", "stray.txt"]);
        let stray_blob = git(&["rev-parse", ":stray.txt"]);

        let minted = mint_migration_in_repo(
            repo.path(),
            "changelog",
            "migrate-changelog",
            "CHANGELOG.md",
        )
        .expect("migration mint succeeds");
        let snapshot = engine::state::read_staged_snapshot(&minted.dir)
            .expect("snapshot reads")
            .expect("the migrate door must write a staged snapshot");
        assert_eq!(
            snapshot.entries.get("stray.txt"),
            Some(&stray_blob),
            "the pre-staged add reads back from the migration task's snapshot"
        );
        assert!(snapshot.deletions.is_empty());
    }

    /// M44 Inc 1 (fork 1): every migration task id carries a `blake3(repo-relative
    /// source path)` disambiguator, so two **distinct** source paths whose slug windows
    /// are identical after the mint's `MAX_WORDS` cap still mint **distinct** tasks —
    /// measured on the id `mint_task` actually creates (the working dir on disk), not a
    /// reconstructed equivalent. Today the second such path serial-collides on the shared
    /// capped id (the committed `milestone-record` `id-from: task-id` item collision at
    /// the root); after the fix both migrate and the same path re-derives to the same id
    /// (→ the resume/discard route, never a double-mint).
    #[test]
    fn migration_mint_ids_are_collision_free_for_colliding_paths() {
        let repo = TempDir::new("migrate-collision");
        init_repo_with_commit(repo.path());
        // Two distinct paths under one dir whose long stems share the first-MAX_WORDS
        // slug window — they collapse to one capped id today.
        let alpha = "docs/adr/one-two-three-four-five-alpha.md";
        let beta = "docs/adr/one-two-three-four-five-beta.md";

        let a = mint_migration_in_repo(repo.path(), "adr", "migrate-adr", alpha)
            .expect("first colliding-window path mints");
        let b = mint_migration_in_repo(repo.path(), "adr", "migrate-adr", beta)
            .expect("second colliding-window path must mint its OWN task, not serial-collide");

        assert_ne!(
            a.id, b.id,
            "distinct source paths must mint distinct task ids"
        );
        assert!(
            a.dir.is_dir() && b.dir.is_dir(),
            "both migration task dirs exist on disk (the id mint_task actually created)"
        );

        // Re-migrating the SAME path re-derives the SAME id → serial-collision resume
        // route, never a double-mint.
        let err = mint_migration_in_repo(repo.path(), "adr", "migrate-adr", alpha)
            .expect_err("re-migrating the same path serial-collides into the resume route");
        assert!(
            err.to_string().contains(&a.id),
            "re-migration collides on the same id `{}`; got: {err}",
            a.id
        );
    }

    /// A user intent with no slug-able content is rejected up front **under the mint
    /// class's own identity** — `write.unslugable-title`, the third producer of that code
    /// (M53 Increment 5 / D5) — and mints **nothing**: never silently minted under
    /// `state::mint_id`'s empty→type-name fallback, which would put the work at a
    /// fabricated `commit` id that a second such call serial-collides. The fallback stays
    /// for legitimate internal callers and as a defensive floor; only the user's task
    /// intent is guarded at this mint boundary.
    ///
    /// **The cells are ordinary, not adversarial**, which is why the printed sentence has
    /// to be true of all of them: the pre-D5 bail said *"intent must contain at least one
    /// letter or digit"*, which is plainly false of `"日本語"` — a title carrying three
    /// letters and no ASCII — and carried no code at all, so a driver keying on
    /// `(code, target)` read nothing back and the invocation log recorded an anonymous
    /// exit 1. The emitted bytes are driven at
    /// `crates/cli/tests/start_compose.rs::a_title_that_slugs_to_nothing_refuses_under_the_mint_class_code`;
    /// this arm pins the refusal at the mint function itself, over the whole degenerate
    /// input set.
    #[test]
    fn empty_slug_intent_is_rejected_and_mints_nothing() {
        let repo = TempDir::new("repo");
        init_repo_with_commit(repo.path());
        let tasks = repo.path().join(".jigc").join("tasks");

        for bad in ["", "   ", "!!!", "日本語", "the"] {
            let err = mint_in_repo(repo.path(), bad, "single-task", None)
                .expect_err("no-sluggable-content intent must reject");
            let msg = err.to_string();
            assert!(
                msg.starts_with("blocking · write.unslugable-title — "),
                "the refusal must carry the mint class's identity for intent {bad:?}; got: {msg}"
            );
            assert!(
                msg.contains(
                    "ids are built from ASCII letters and digits, so a title in another \
                     script, or of stopwords only, yields none"
                ),
                "the sentence must be true for a title in any script (intent {bad:?}); \
                 got: {msg}"
            );
            assert!(
                msg.contains("\n  at: task\n"),
                "the block keys at the bare work-unit type token; got: {msg}"
            );
            assert!(
                msg.contains("\n  route: "),
                "a blocking refusal carries a route; got: {msg}"
            );
            // Mint nothing: no task dir appears (not even the `commit` fallback).
            let minted_any = std::fs::read_dir(&tasks)
                .map(|mut d| d.next().is_some())
                .unwrap_or(false);
            assert!(
                !minted_any,
                "rejected intent {bad:?} must leave .jigc/tasks/ empty"
            );
        }

        // A normal intent still mints.
        let minted = mint_in_repo(repo.path(), "Add rate limiter", "single-task", None)
            .expect("real intent mints");
        assert_eq!(minted.id, "add-rate-limiter");
    }

    /// The per-file migration task id is a pure, path-aware function of
    /// `(doctype, source_path)`: the same path yields the same id (so re-migrating a
    /// file collides into the existing serial-collision route, not a double-mint), two
    /// distinct paths yield distinct ids (so a corpus migrates sequentially without the
    /// singleton blocker), and two same-stem files in different directories yield
    /// distinct ids (the path is folded into the disambiguator, not just the stem). The
    /// changelog root file reads legibly as `migrate-changelog-changelog-<hash>`.
    #[test]
    fn migration_task_id_is_per_file_and_path_aware() {
        // Same path -> same id (deterministic; re-migration collides, never double-mints).
        assert_eq!(
            migration_task_id("adr", "decisions/0001-cache.md"),
            migration_task_id("adr", "decisions/0001-cache.md"),
        );
        // Two distinct paths -> distinct ids.
        assert_ne!(
            migration_task_id("adr", "decisions/0001-cache.md"),
            migration_task_id("adr", "decisions/0002-retry.md"),
        );
        // Same stem in different directories -> distinct ids (the path, not just the
        // stem, drives the disambiguator).
        assert_ne!(
            migration_task_id("adr", "a/CHANGELOG.md"),
            migration_task_id("adr", "b/CHANGELOG.md"),
        );
        // Every id is a well-formed slug (fed verbatim as `slug_override`), keeps the
        // legible `migrate-<doctype>-<slug>-` head, and carries the 12-hex disambiguator.
        let id = migration_task_id("changelog", "CHANGELOG.md");
        assert!(
            engine::slug::is_slug(&id),
            "id must be a well-formed slug: {id}"
        );
        assert!(
            id.starts_with("migrate-changelog-changelog-"),
            "id must read legibly as migrate-changelog-changelog-<hash>: {id}"
        );
    }

    /// A source path whose every char slugs away (all non-Latin, e.g. `日本語.md`)
    /// must still yield a per-file, non-singleton id: two DISTINCT empty-slug paths
    /// must get DISTINCT ids (else they serial-collide on the bare `migrate-<doctype>`
    /// the per-file mint exists to avoid — `auto-migration.md` → Hardening #9), while
    /// the SAME path stays stable (so re-migration collides into the resume/discard
    /// route, never double-mints).
    #[test]
    fn migration_task_id_disambiguates_paths_that_slug_to_empty() {
        // Two distinct all-non-Latin paths -> distinct ids (no collapse to singleton).
        assert_ne!(
            migration_task_id("adr", "日本語.md"),
            migration_task_id("adr", "中文.md"),
        );
        // Same empty-slug path -> same id (deterministic; re-migration collides).
        assert_eq!(
            migration_task_id("adr", "日本語.md"),
            migration_task_id("adr", "日本語.md"),
        );
        // The empty-slug id never collapses to the bare `migrate-<doctype>` singleton and
        // carries no empty slug segment (a well-formed `migrate-<doctype>-<hash>` slug).
        let id = migration_task_id("adr", "日本語.md");
        assert_ne!(id, "migrate-adr");
        assert!(
            engine::slug::is_slug(&id),
            "empty-slug id must be a slug: {id}"
        );
        assert!(
            id.starts_with("migrate-adr-") && !id.starts_with("migrate-adr--"),
            "empty-slug id must be migrate-adr-<hash>, no empty slug segment: {id}"
        );
    }

    /// An in-memory [`PackSource`] seeded from `(kind, id, bytes)` triples — lets a
    /// unit test drive [`compose_core`] over a fixture cascade with no embedded
    /// pack and no filesystem.
    struct FixturePack(std::collections::HashMap<(PackResourceKind, ResourceId), Vec<u8>>);

    impl FixturePack {
        fn with(triples: Vec<(PackResourceKind, &str, &str)>) -> Self {
            let map = triples
                .into_iter()
                .map(|(kind, id, body)| ((kind, ResourceId::from(id)), body.as_bytes().to_vec()))
                .collect();
            FixturePack(map)
        }
    }

    impl PackSource for FixturePack {
        fn pack_version(&self) -> String {
            "0.0.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            let mut ids: Vec<ResourceId> = self
                .0
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect();
            ids.sort();
            ids
        }

        fn read(
            &self,
            kind: PackResourceKind,
            id: &ResourceId,
        ) -> Result<Vec<u8>, engine::packsource::PackError> {
            self.0.get(&(kind, id.clone())).cloned().ok_or_else(|| {
                engine::packsource::PackError::NotFound {
                    kind,
                    id: id.clone(),
                }
            })
        }
    }

    /// A no-shadow resolved cascade — an empty pack-default layer with no project
    /// overrides, so `file_owner(id)` is `None` for every id and the layer-aware
    /// definition reads ([`CascadeDefs`]) all fall through to the pack. The default
    /// `CascadeDefs` for a `compose_core` test that exercises no whole-file shadow
    /// (the omit-the-target context — hardening #5: the read path stays byte-
    /// identical to the pack-only baseline when no shadow is present).
    fn no_shadow_resolved() -> cascade::Resolved {
        let base =
            engine::cascade::PackDefaultLayer::new("dev", "0.0.0", BTreeMap::new(), Vec::new());
        engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves")
    }

    /// The no-shadow [`CascadeDefs`] the catalog unit tests read definitions through —
    /// [`no_shadow_resolved`] over a project config dir that does not exist, so
    /// `project_owns` is false for every id and each read falls through to the pack.
    /// The shadowed half is driven binary-side in
    /// `tests/workflow_resolution_unified.rs`; this is the omitting context.
    fn no_shadow_defs(resolved: &cascade::Resolved) -> CascadeDefs<'_> {
        CascadeDefs::new(resolved, Path::new("no-project-layer"))
    }

    /// The fan-out `sub-task` ships `creates-task: true` (so a `jigc workflow
    /// <W> --task` re-entry can compose it) but `selectable: false` (it ships no
    /// finalize step — its only commit boundary is the parent milestone's). It
    /// must therefore NOT appear in the router's selectable catalog over the REAL
    /// embedded pack, while every legitimate selectable work-workflow still does.
    #[test]
    fn selectable_catalog_excludes_the_non_selectable_sub_task_over_the_embedded_pack() {
        let pack = crate::pack::EmbeddedPack::new();
        let resolved = no_shadow_resolved();
        let catalog =
            selectable_workflows(&pack, &no_shadow_defs(&resolved)).expect("catalog builds");
        let ids: Vec<&str> = catalog.iter().map(|e| e.id.as_str()).collect();

        assert!(
            !ids.contains(&"sub-task"),
            "the finalize-less `sub-task` is `selectable: false` and must NOT be a \
             selectable catalog entry; got {ids:?}",
        );
        for expected in ["single-task", "quick-fix", "plan", "implement-from-spec"] {
            assert!(
                ids.contains(&expected),
                "selectable work-workflow `{expected}` must still appear; got {ids:?}",
            );
        }
    }

    /// T3 done-criterion (catalog-absence, the M8-leak pin): `ingest-existing`
    /// ships `creates-task: false` (orient/route shape like `router`), so it must
    /// NOT appear in the selectable router catalog over the REAL embedded pack —
    /// admitting it would re-introduce the M8 sub-task catalog leak
    /// (`increment-workflow.md` → #4). The legitimate selectable work-workflows
    /// still appear; `ingest-existing` and `router` (both `creates-task: false`)
    /// do not.
    #[test]
    fn selectable_catalog_excludes_ingest_existing_over_the_embedded_pack() {
        let pack = crate::pack::EmbeddedPack::new();
        let resolved = no_shadow_resolved();
        let catalog =
            selectable_workflows(&pack, &no_shadow_defs(&resolved)).expect("catalog builds");
        let ids: Vec<&str> = catalog.iter().map(|e| e.id.as_str()).collect();

        assert!(
            !ids.contains(&"ingest-existing"),
            "the `creates-task: false` `ingest-existing` must NOT be a selectable \
             catalog entry (the M8-leak pin); got {ids:?}",
        );
        assert!(
            !ids.contains(&"router"),
            "the router itself (creates-task: false) must never be selectable; got {ids:?}",
        );
        for expected in ["single-task", "quick-fix"] {
            assert!(
                ids.contains(&expected),
                "selectable work-workflow `{expected}` must still appear; got {ids:?}",
            );
        }
    }

    /// T3 done-criterion (Form-D composition): `jigc start --workflow
    /// ingest-existing "<intent>"` composes the `ingest-existing` workflow over the
    /// REAL embedded pack via Form D (`compose_core` with the named id). Being
    /// `creates-task: false`, it mints **nothing** (no `.jigc/tasks/` area), and its
    /// composed body carries the `Run: jigc ingest` line its `run-scan` step emits —
    /// the orient/route walk that drives the agent into the engine scan. The repo is
    /// a bare temp dir: the no-task arm never reads HEAD, so no git is needed.
    #[test]
    fn ingest_existing_composes_via_form_d_and_emits_run_jigc_ingest() {
        let repo = TempDir::new("ingest-existing-form-d");

        let pack = crate::pack::EmbeddedPack::new();
        let source = PackStepSource { pack: &pack };
        let composed = compose_core(
            repo.path(),
            "bring this repo under jigc management",
            &pack,
            "ingest-existing",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("Form-D compose of ingest-existing")
        .view;

        // (a) creates-task: false ⇒ no working area is minted.
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "a `creates-task: false` ingest-existing compose must not open any \
             `.jigc/tasks/` dir",
        );

        // (b) the run-scan step emits the `jigc ingest` Run line. The composer
        // reserves the `Run: ` marker for resolved command-refs and emits the
        // command backtick-wrapped (the machine-extractable `^Run: `(.+)`$`
        // contract), so the emitted artifact is `` Run: `jigc ingest` ``. Extract
        // the composed line verbatim and assert on the emitted bytes (never a
        // hand-built equivalent).
        let run_line = composed
            .text
            .lines()
            .find(|l| l.starts_with("Run: "))
            .unwrap_or_else(|| panic!("no `Run: ` line in composed body:\n{}", composed.text));
        assert_eq!(
            run_line, "Run: `jigc ingest`",
            "the composed ingest-existing body must emit the `jigc ingest` Run line; got:\n{}",
            composed.text,
        );
    }

    /// A `selectable: false` workflow that carries no `when` hint must be skipped
    /// silently — the "selectable workflow missing `when`" error applies only to a
    /// workflow that is actually selectable. This guards the relaxation that lets
    /// `sub-task` drop its now-unneeded `when:`.
    #[test]
    fn non_selectable_workflow_without_when_does_not_trip_the_missing_when_error() {
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "sub-task",
                "---\ncreates-task: true\nselectable: false\n---\n{{ include: step:noop }}\n",
            ),
        ]);
        let resolved = no_shadow_resolved();
        let catalog = selectable_workflows(&pack, &no_shadow_defs(&resolved)).expect(
            "a `selectable: false` workflow with no `when` must not trip the missing-`when` error",
        );
        let ids: Vec<&str> = catalog.iter().map(|e| e.id.as_str()).collect();
        assert_eq!(
            ids,
            ["single-task"],
            "only the selectable workflow is listed; the no-`when` non-selectable is skipped",
        );
    }

    /// The T4 done-criterion: composing a `creates-task: false` default workflow
    /// (a router whose body interpolates `{{catalog}}`) mints **nothing** — no
    /// `.jigc/tasks/<id>/` working area appears — and its output lists exactly the
    /// **selectable** (`creates-task: true`) work-workflows, never the router
    /// itself. The `repo_root` is an empty temp dir: the no-task arm never reads
    /// HEAD, so no git is needed (proof the arm carries no task context).
    #[test]
    fn no_task_workflow_composes_the_catalog_without_minting() {
        let repo = TempDir::new("no-task");

        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: router\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "router",
                "---\nwhen: help me pick a workflow\ncreates-task: false\n---\n{{ include: step:route }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "quick-fix",
                "---\nwhen: a small focused fix\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Steps,
                "route",
                "Pick one of the work-workflows below:\n\n{{ catalog }}\n",
            ),
            (PackResourceKind::Steps, "noop", "no-op body\n"),
        ]);

        // `compose_core` is driven with the workflow id directly here — the
        // cascade read (`resolve_cascade` + `scalar_required`) is exercised by the binary-
        // driven flip + byte-identical goldens (`tests/start_compose.rs`). The
        // no-override step source + empty phase-4 deltas read the fixture pack
        // unchanged (the live `CascadeStepSource` path is the binary-driven tests').
        let source = PackStepSource { pack: &pack };
        let composed = compose_core(
            repo.path(),
            "anything",
            &pack,
            "router",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("no-task compose")
        .view;

        // (a) The no-task arm mints nothing: no working area is opened.
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "a `creates-task: false` compose must not open any `.jigc/tasks/` dir",
        );

        // (b) `{{catalog}}` resolves to the selectable work-workflows, each as a
        // `- <id> — <when>` option line — and the router never lists itself.
        assert!(
            composed
                .text
                .contains("- single-task — implement one scoped change"),
            "the catalog must list single-task; got:\n{}",
            composed.text,
        );
        assert!(
            composed.text.contains("- quick-fix — a small focused fix"),
            "the catalog must list quick-fix; got:\n{}",
            composed.text,
        );
        assert!(
            !composed.text.contains("- router —"),
            "the router must not list itself (creates-task: false is filtered); got:\n{}",
            composed.text,
        );
    }

    /// Build an `insert` structural delta from a `workflow:<id>` + `after:`/`before:`
    /// anchor + the inserted step id (the CLI-side mirror of the engine test helper).
    fn insert_delta(workflow: &str, anchor: AnchorSpec, step: &str) -> StructuralDelta {
        StructuralDelta::Insert {
            target: StructuralTarget::parse(workflow, Some(anchor)).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `replace` structural delta from a `workflow:<id>#<step-id>` ref + the
    /// replacement step id.
    fn replace_delta(target: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget::parse(target, None).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `remove` structural delta from a `workflow:<id>#<step-id>` ref.
    fn remove_delta(target: &str) -> StructuralDelta {
        StructuralDelta::Remove {
            target: StructuralTarget::parse(target, None).expect("valid target"),
        }
    }

    /// A `creates-task: false` `router`-style fixture pack over a two-step
    /// `flow` workflow (`[locate, implement]`) — enough surface to apply a
    /// structural delta to its include list and compose the result without a mint.
    fn structural_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (PackResourceKind::Steps, "implement", "implement body\n"),
        ])
    }

    /// T5 wire-through (a): `compose_core` runs its `workflow-refs` gate over the
    /// **post-phase-4** include list, so an orphaned-anchor delta — `remove #locate`
    /// then `insert --after locate` in the same manifest — fails the compose with the
    /// routed `structural-anchor-resolves` block (`overrides.md` → Within-layer
    /// manifest order). The orphan surfaces on the real CLI path, never a panic.
    #[test]
    fn compose_core_orphaned_anchor_delta_fails_with_route() {
        let repo = TempDir::new("t5-orphan");
        let pack = structural_pack();
        let source = PackStepSource { pack: &pack };
        let deltas = vec![
            remove_delta("workflow:flow#locate"),
            insert_delta(
                "workflow:flow",
                AnchorSpec::After("locate".to_owned()),
                "lint",
            ),
        ];

        let err = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(deltas),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect_err("the insert's anchor was removed by the earlier delta");

        let msg = err.to_string();
        assert!(
            msg.contains("orphaned") && msg.contains("locate"),
            "the orphaned-anchor failure must name the cause + the anchor; got: {msg}",
        );
        assert!(
            msg.contains("route:") && msg.contains("jigc config"),
            "the orphan surfaces its repair route on the CLI path; got: {msg}",
        );
    }

    /// T5 wire-through (b): a `replace-step` whose replacement step re-includes a
    /// step that loops back introduces an include cycle the gate catches at phase 6
    /// over the post-phase-4 list — the compose fails with `include-cycle-absent`,
    /// not a panic (`overrides.md` → No silent cycle handling).
    #[test]
    fn compose_core_delta_introduced_cycle_fails_at_phase_6() {
        let repo = TempDir::new("t5-cycle");
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (PackResourceKind::Steps, "implement", "implement body\n"),
            // The replacement loops: looping -> back -> looping.
            (
                PackResourceKind::Steps,
                "looping",
                "house rule\n{{ include: step:back }}\n",
            ),
            (
                PackResourceKind::Steps,
                "back",
                "{{ include: step:looping }}\n",
            ),
        ]);
        let source = PackStepSource { pack: &pack };
        let deltas = vec![replace_delta("workflow:flow#implement", "looping")];

        let err = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(deltas),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect_err("the delta introduces an include cycle");

        assert!(
            err.to_string().contains("include cycle"),
            "a delta-introduced cycle must fail the gate at phase 6; got: {err}",
        );
    }

    /// T5 wire-through (c): flow 3a's clean re-include — `replace #implement →
    /// project-implement` where `project-implement` re-includes the pack `implement`
    /// (a **different** id) — composes cleanly with no false cycle, and the composed
    /// view carries both the pack `implement` body and the house rule
    /// (`worked-examples.md` → 3a). The deltas flow through the real CLI compose path.
    #[test]
    fn compose_core_flow_3a_re_include_composes_clean() {
        let repo = TempDir::new("t5-3a");
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
            // The project step re-includes the pack `implement` (a different id).
            (
                PackResourceKind::Steps,
                "project-implement",
                "{{ include: step:implement }}\nhouse rule: run the lint probe\n",
            ),
        ]);
        let source = PackStepSource { pack: &pack };
        let deltas = vec![replace_delta(
            "workflow:flow#implement",
            "project-implement",
        )];

        let composed = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(deltas),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("flow 3a's different-id re-include composes clean")
        .view;

        assert!(
            composed.text.contains("pack implement body")
                && composed.text.contains("house rule: run the lint probe"),
            "the re-include must pull the pack body in then the house rule; got:\n{}",
            composed.text,
        );
    }

    /// A `creates-task: false` `flow` pack whose `replace-step` target —
    /// `#implement` → `project-implement` — composes to a non-trivial body (the
    /// project step re-includes the pack `implement` then adds a house rule), so a
    /// basis that leaked into the `StructuralDelta` would perturb both the loaded
    /// deltas vector and the composed bytes. No mint needed (`creates-task: false`),
    /// so the compose path runs without a task working area.
    fn replace_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: flow\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
            (
                PackResourceKind::Steps,
                "project-implement",
                "{{ include: step:implement }}\nhouse rule: run the lint probe\n",
            ),
        ])
    }

    /// T3 — the compose byte-identical golden for the M5 base-hash basis: a
    /// `replace-step` delta authored **with** `base-version`/`base-hash` and one
    /// authored **without** them must drive compose identically. Both manifests are
    /// parsed through the **real loader** (`load_project_layer`, where the basis is
    /// parsed off the same entry), then fed to `compose_core` over the same pack.
    ///
    /// The contract (design-review B2; `overrides.md` → On-disk vs in-memory
    /// representation): the basis rides only the separate `StructuralBasis` record,
    /// never the compose-facing `StructuralDelta` — so (a) the `Vec<StructuralDelta>`
    /// compose receives is byte-for-byte identical, and (b) the composed **output
    /// bytes** are identical. Asserting the emitted composed text (not a
    /// reconstruction) makes this the byte-identical golden: a basis that leaked into
    /// `StructuralDelta` would diverge one or both.
    #[test]
    fn replace_basis_never_perturbs_compose() {
        let with_basis = "deltas:\n  \
             - kind: replace-step\n    \
             target: workflow:flow#implement\n    \
             with: step:project-implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n";
        let without_basis = "deltas:\n  \
             - kind: replace-step\n    \
             target: workflow:flow#implement\n    \
             with: step:project-implement\n";

        let cfg_with = TempDir::new("t3-with-basis");
        fs::write(cfg_with.path().join("manifest.yaml"), with_basis).expect("write manifest");
        let cfg_without = TempDir::new("t3-without-basis");
        fs::write(cfg_without.path().join("manifest.yaml"), without_basis).expect("write manifest");

        let (_l_with, deltas_with, _sf_with, _f_with, bases_with) =
            load_project_layer(cfg_with.path()).expect("with-basis manifest loads");
        let (_l_without, deltas_without, _sf_without, _f_without, bases_without) =
            load_project_layer(cfg_without.path()).expect("without-basis manifest loads");

        // Precondition: the two manifests differ *only* in the recorded basis — the
        // with-basis parse records one `StructuralBasis`, the without-basis parse
        // records none. (If this didn't hold the byte-identity below would be vacuous.)
        assert_eq!(
            bases_with.len(),
            1,
            "the with-basis manifest records its basis"
        );
        assert!(
            bases_without.is_empty(),
            "the without-basis manifest records no basis",
        );

        // (a) The `Vec<StructuralDelta>` compose receives is identical — the basis
        // never reaches the compose-facing delta.
        assert_eq!(
            deltas_with, deltas_without,
            "the basis must not perturb the StructuralDelta vector compose receives",
        );

        // (b) The composed **output bytes** are identical. Drive the real compose
        // path over both loaded delta vectors and compare the emitted text verbatim.
        let pack = replace_pack();
        let source = PackStepSource { pack: &pack };
        let repo = TempDir::new("t3-compose");
        let composed = |deltas: Vec<StructuralDelta>| {
            compose_core(
                repo.path(),
                "anything",
                &pack,
                "flow",
                &source,
                &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
                &ComposeOverrides::structural(deltas),
                MilestoneFeed::none(),
                None,
                None,
                false,
            )
            .expect("flow composes under the replace delta")
            .view
            .text
        };

        assert_eq!(
            composed(deltas_with),
            composed(deltas_without),
            "the recorded basis must never perturb the composed output bytes",
        );
    }

    /// A minimal but valid `commit` schema (`type: commit`, a header section + a
    /// `summary` slot) — enough for `provision_commit_doc` to render a fillable
    /// form when a `creates-task: true` workflow mints under Form D.
    const COMMIT_SCHEMA: &str = "type: commit\nsections:\n  - id: header\n    header: true\n    fields:\n      - { id: type, type: string }\n  - id: summary\n    slot: {}\n";

    /// The pack a Form-D test composes over: a `creates-task: true` `single-task`,
    /// a `creates-task: false` `router`, the `commit` schema minting provisions,
    /// and the no-op step both workflow bodies include. `default-workflow` points
    /// at the router so a stray cascade-default read would be observably wrong —
    /// Form D must compose the *named* workflow, not the default.
    fn form_d_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: router\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (PackResourceKind::Schemas, "commit", COMMIT_SCHEMA),
            (
                PackResourceKind::Workflows,
                "router",
                "---\nwhen: help me pick a workflow\ncreates-task: false\n---\n{{ include: step:route }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
            ),
            (
                PackResourceKind::Steps,
                "route",
                "Pick one of the work-workflows below:\n\n{{ catalog }}\n",
            ),
            (PackResourceKind::Steps, "noop", "no-op body\n"),
        ])
    }

    /// The T2 done-criterion (a): Form D over a **`creates-task: true`** named
    /// workflow opens `.jigc/tasks/<slug>/` and composes its body. The named
    /// workflow is `single-task` while `default-workflow` is the router, so a
    /// successful mint proves the *named* workflow composed, not the default.
    #[test]
    fn form_d_over_a_creates_task_workflow_mints_and_composes() {
        let repo = TempDir::new("form-d-mint");
        init_repo_with_commit(repo.path());

        let pack = form_d_pack();
        let source = PackStepSource { pack: &pack };
        let composed = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "single-task",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("Form-D compose of a creates-task workflow")
        .view;

        let dir = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter");
        assert!(
            dir.is_dir(),
            "a creates-task: true Form-D workflow must open .jigc/tasks/<slug>/",
        );
        assert!(
            composed.text.contains("no-op body"),
            "the named workflow's body must compose; got:\n{}",
            composed.text,
        );
    }

    /// The T2 done-criterion (b): a `<X>` the pack does not provide rejects with a
    /// routed finding, and **no** `.jigc/tasks/` dir is created — the rejection
    /// precedes minting. The repo is a bare temp dir: rejection fires before HEAD
    /// is ever read.
    #[test]
    fn form_d_unknown_workflow_rejects_before_minting() {
        let repo = TempDir::new("form-d-unknown");

        let pack = form_d_pack();
        let source = PackStepSource { pack: &pack };
        let err = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "does-not-exist",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect_err("an unknown --workflow id must reject");

        let msg = err.to_string();
        assert!(
            msg.contains("does-not-exist") && msg.contains("route:"),
            "the rejection must name the unknown id and carry a route; got: {msg}",
        );
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "rejection must precede minting — no .jigc/tasks/ dir may be created",
        );
    }

    /// The pack a preview test composes over: a `creates-task: true` `single-task`
    /// whose step body interpolates `{{task.id}}` twice — so the byte-identity
    /// oracle's id substitution is **non-vacuous** (the id genuinely appears in the
    /// composed body) — plus the `commit` schema minting provisions and a
    /// `creates-task: false` `router`. `default-workflow` points at the router so a
    /// stray cascade-default read would be observably wrong (the preview names the
    /// workflow explicitly, like Form D).
    fn preview_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: dev\ndefault-workflow: router\n",
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (PackResourceKind::Schemas, "commit", COMMIT_SCHEMA),
            (
                PackResourceKind::Workflows,
                "router",
                "---\nwhen: help me pick a workflow\ncreates-task: false\n---\n{{ include: step:route }}\n",
            ),
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:work }}\n",
            ),
            (
                PackResourceKind::Steps,
                "route",
                "Pick one of the work-workflows below:\n\n{{ catalog }}\n",
            ),
            (
                PackResourceKind::Steps,
                "work",
                "Working on task {{task.id}} now.\nFinalize with jigc task finalize {{task.id}}.\n",
            ),
        ])
    }

    /// The T1 done-criterion (a)–(c): the `--preview` compose path is
    /// `compose_core`-minus-mint. Over a `creates-task: true` workflow it (a) writes
    /// **no** `.jigc/tasks/*` working area and provisions **no** commit doc, (b)
    /// returns a [`Composition`] with `view.task == None` and `minted == false`
    /// (`surface-contract.md` → law 1: a preview names no task that exists), and (c)
    /// is **byte-identical** to a real mint's composed body with the minted id
    /// textually substituted by [`PREVIEW_SLUG`]. The oracle is an *independent*
    /// real-mint compose of the same workflow over the same intent — the emitted
    /// bytes are the contract, so the assertion drives the composed body verbatim.
    #[test]
    fn preview_composes_minus_mint_and_is_byte_identical_to_a_real_mint() {
        let repo = TempDir::new("preview");
        init_repo_with_commit(repo.path());
        let pack = preview_pack();

        // The preview compose — the minus-mint arm (preview = true).
        let source = PackStepSource { pack: &pack };
        let preview = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "single-task",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            true,
        )
        .expect("preview compose");

        // (a) mints nothing: no working area opens, no commit skeleton stages.
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "a preview must not open any .jigc/tasks/ dir",
        );
        // (b) the view carries no task id and the mint announcement is false.
        assert_eq!(
            preview.view.task, None,
            "a preview names no minted task (law 1)",
        );
        assert!(!preview.minted, "a preview mints nothing");

        // (c) the independent byte-identity oracle: a REAL mint compose of the same
        // workflow over the same intent. Its composed body, with the minted id
        // substituted by the synthetic slug, must equal the preview body verbatim.
        let source2 = PackStepSource { pack: &pack };
        let real = compose_core(
            repo.path(),
            "Add rate limiter",
            &pack,
            "single-task",
            &source2,
            &CascadeDefs::new(&no_shadow_resolved(), repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("real mint compose");
        let minted_id = real.view.task.clone().expect("a real mint carries its id");
        assert_eq!(minted_id, "add-rate-limiter");
        // The fixture body interpolates `{{task.id}}`, so the id genuinely appears —
        // the oracle substitution is not vacuous.
        assert!(
            real.view.text.contains(&minted_id),
            "the fixture must reference the id so the oracle is non-vacuous; got:\n{}",
            real.view.text,
        );
        let substituted = real.view.text.replace(&minted_id, PREVIEW_SLUG);
        assert_eq!(
            preview.view.text, substituted,
            "the preview body must equal a real mint's with the id → {PREVIEW_SLUG}",
        );
        assert!(
            preview.view.text.contains(PREVIEW_SLUG),
            "the preview renders the synthetic identity in place of the id; got:\n{}",
            preview.view.text,
        );
    }

    /// The T1 done-criterion (d): [`preview_in_repo`] on a `creates-task: false`
    /// workflow returns the **routed rejection** — that workflow mints nothing, so
    /// `jigc start --workflow <id>` already composes it and there is nothing to
    /// preview. Driven over the embedded pack (the surface `preview_in_repo`
    /// composes), whose `router` is `creates-task: false`; an empty project config
    /// dir resolves to a no-shadow cascade over that pack.
    #[test]
    fn preview_in_repo_rejects_a_no_task_workflow() {
        let repo = TempDir::new("preview-reject");
        init_repo_with_commit(repo.path());
        // The project layer must be present (require_project_config).
        fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config dir");

        let err = preview_in_repo(repo.path(), "router")
            .expect_err("a creates-task: false workflow has nothing to preview");
        let msg = err.to_string();
        assert!(
            msg.contains("router") && msg.contains("jigc start --workflow router"),
            "the rejection must name the workflow and route to running it directly; got: {msg}",
        );
        assert!(
            !repo.path().join(".jigc").join("tasks").exists(),
            "the rejection precedes any mint — no .jigc/tasks/ dir",
        );
    }

    /// The T2 done-criterion: a `reads`-declared role is **seeded into the compose
    /// role map** so `task.<role>` is a valid `workflow-refs` root that resolves to
    /// [`Resolution::Absent`] (empty text) when **unbound**, while an **undeclared**
    /// `task.<other>` still raises `workflow-refs.undeclared-role`
    /// (`workflow-dialect.md` → `reads`: declared-but-unbound resolves to empty,
    /// undeclared is a conformance error).
    #[test]
    fn reads_role_seeds_the_compose_map_and_resolves_empty_when_unbound() {
        use engine::compose::Reads;
        use engine::data_value::{Path, Resolution};

        let def = WorkflowDef {
            when: Some("implement from a spec".to_owned()),
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            suppressed: None,
            allows_create: vec![],
            reads: vec![Reads {
                role: "spec".to_owned(),
                doc_type: "spec".to_owned(),
            }],
            includes: vec![],
        };
        // No role is bound yet — the task carries the declaration only.
        let ctx = build_context(
            "add-rate-limiter",
            "Add rate limiter",
            &def,
            &RolesRecord::new(),
            Vec::new(),
            BTreeMap::new(),
            None,
            BTreeMap::new(),
        );

        // The declared-but-unbound `reads` role resolves to absent/empty text — no
        // `undeclared-role` finding.
        let resolved = Path::parse("task.spec")
            .expect("valid path")
            .resolve(&ctx)
            .expect("a declared role must not raise a finding");
        assert_eq!(
            resolved,
            Resolution::Absent,
            "an unbound declared `reads` role must resolve to Absent (empty text)",
        );

        // An undeclared role under the same task still raises the conformance error.
        let undeclared = Path::parse("task.other")
            .expect("valid path")
            .resolve(&ctx)
            .expect_err("an undeclared role must raise a finding");
        assert_eq!(
            undeclared.code, "workflow-refs.undeclared-role",
            "an undeclared `task.<role>` must stay a conformance error; got {}",
            undeclared.code,
        );
    }

    /// `committed_store` enumerates the committed `<location>/<slug>.md` instances
    /// into the `store` feed, keyed by the location stem (`specs/` → `specs`) — the
    /// `store.specs` collection the `locate-from-spec` step interpolates. A transient
    /// (location-less) doctype (`commit`) contributes nothing; a doctype with a
    /// declared location but no committed files is omitted (the empty-store stance).
    #[test]
    fn committed_store_enumerates_by_location_stem() {
        let repo = TempDir::new("committed-store");
        // The no-shadow cascade carries no `docs-root` scalar, so locations stay flat
        // (`apply_docs_root` is opt-in); the store key is the final path segment (`specs`).
        let specs = repo.path().join("specs");
        fs::create_dir_all(&specs).expect("mk specs/");
        fs::write(specs.join("gateway-rate-limiting.md"), "# Gateway\n").expect("w");
        fs::write(specs.join("auth-token-rotation.md"), "# Auth\n").expect("w");
        // A non-`.md` file is ignored.
        fs::write(specs.join("notes.txt"), "ignore me").expect("w");

        // The real pack schemas: `spec` (location `specs/`), `adr` (`decisions/`,
        // no committed files here), `commit` (transient — no location). Loaded
        // through a no-shadow cascade — every schema reads its pack definition.
        let pack = crate::pack::EmbeddedPack::new();
        let resolved = no_shadow_resolved();
        let schemas = CascadeDefs::new(&resolved, repo.path())
            .all_schemas(&pack)
            .expect("schemas load");

        let store = committed_store(repo.path(), &schemas);

        assert_eq!(
            store.get("specs").map(Vec::as_slice),
            Some(
                [
                    Address::parse("spec:auth-token-rotation").expect("valid"),
                    Address::parse("spec:gateway-rate-limiting").expect("valid"),
                ]
                .as_slice()
            ),
            "`store.specs` lists the committed spec instances (sorted, `<type>:<slug>`)",
        );
        // A declared-location doctype with no committed files is absent (empty store).
        assert!(
            !store.contains_key("decisions"),
            "a doctype with no committed instances must be omitted; got {store:?}",
        );
        // The transient `commit` type (no `location:`) never appears.
        assert!(
            !store.keys().any(|k| k == "commit"),
            "a transient (location-less) doctype contributes nothing; got {store:?}",
        );
    }

    /// **M42 Inc-2 T4 — the census's 14th site (`storage.md` → The census).** A
    /// **placement** doctype has `location: None`, so the location-stem walk dropped it
    /// through the transient arm: its committed singleton was invisible to the
    /// compose-time `store` feed, and — an absent `store` key being the *empty* case,
    /// never *unresolvable* — a workflow writing `{{store.changelog}}` composed as if no
    /// changelog were committed, silently. The collection key of a placement doctype is
    /// its **type id** (it has no location stem to key by), and its collection the
    /// one-element `[<type>:<type>]` singleton list at the literal `placement.file`.
    #[test]
    fn committed_store_keys_a_placement_doctype_by_its_type_id() {
        let repo = TempDir::new("committed-store-placement");
        let pack = crate::pack::EmbeddedPack::new();
        let resolved = no_shadow_resolved();
        let schemas = CascadeDefs::new(&resolved, repo.path())
            .all_schemas(&pack)
            .expect("schemas load");

        // No committed `CHANGELOG.md` yet — the placement singleton is absent, exactly as
        // an empty location dir is: NO key (the omitting context stays inert, never a
        // fabricated empty-slug entry).
        let empty = committed_store(repo.path(), &schemas);
        assert!(
            !empty.contains_key("changelog"),
            "an absent placement singleton must contribute no `store` key; got {empty:?}",
        );

        // The committed singleton at its literal repo-root home (`changelog`'s
        // `placement.file` — `CHANGELOG.md`; storage.md → Placement).
        fs::write(repo.path().join("CHANGELOG.md"), "# Changelog\n").expect("write CHANGELOG.md");

        let store = committed_store(repo.path(), &schemas);
        assert_eq!(
            store.get("changelog").map(Vec::as_slice),
            Some([Address::parse("changelog:changelog").expect("valid")].as_slice()),
            "`store.changelog` lists the committed placement singleton keyed by its type id",
        );
        // The located enumeration is untouched by the placement branch.
        assert!(
            !store.contains_key("specs"),
            "no committed specs here — the located walk is unchanged; got {store:?}",
        );
    }

    /// The fillable form pre-stamps required header fields but **omits** an optional
    /// `ref` (`card: 0..1`): a spec-less `commit` is provisioned without an empty
    /// `implements:` line, so it finalizes cleanly (no malformed-empty value, no
    /// dangling edge). Pins the provisioning side of the `commit.implements` optional
    /// ref against the shipped `commit.yaml`.
    #[test]
    fn fillable_form_omits_an_optional_ref_but_keeps_required_fields() {
        use engine::field_block::Value;
        use engine::schema::SectionBody;

        let schema = engine::schema::load_schema(
            crate::pack::EmbeddedPack::new()
                .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
                .expect("commit schema")
                .as_slice(),
        )
        .expect("commit.yaml loads");

        let instance = fillable_form(&schema, "add-rate-limiter");
        let header = instance
            .sections
            .iter()
            .find(|s| s.id == "header")
            .expect("header section");
        let keys: Vec<&str> = header.fields.iter().map(|f| f.key.as_str()).collect();

        assert!(
            keys.contains(&"type") && keys.contains(&"scope"),
            "required header fields stay pre-stamped; got {keys:?}",
        );
        assert!(
            !keys.contains(&"implements"),
            "the optional `implements` ref must not be pre-stamped; got {keys:?}",
        );

        // The omission survives the render → no `implements:` front-matter line.
        let rendered = engine::write::render(&schema, &instance);
        assert!(
            !rendered.contains("implements:"),
            "the provisioned form must carry no empty `implements:` line; got:\n{rendered}",
        );

        // Sanity: the schema *does* declare `implements` as an optional ref, so the
        // test would catch a regression that pre-stamps it again.
        let SectionBody::Simple { fields, .. } = &schema
            .sections
            .iter()
            .find(|s| s.id == "header")
            .unwrap()
            .body
        else {
            panic!("header is a simple section");
        };
        let implements = fields.iter().find(|f| f.id == "implements").unwrap();
        assert!(engine::validate::is_optional_ref(implements));
        // The pre-stamped required fields carry an empty scalar (the fillable line).
        assert_eq!(header.fields[0].value, Value::Scalar(String::new()));
    }

    /// The done-criterion (ii) (`write-commands.md` → copy-on-first-touch:
    /// "`provision_commit_doc` likewise records `created` provenance under M8 — a
    /// per-sub-task commit doc must appear in the provenance manifest the join
    /// consumes"; `DECISIONS.md` 2026-06-04 → M8 Increment 3 T3). After
    /// `provision_on_first_entry` provisions the sub-area's commit doc, the
    /// provenance manifest the join reads reports `commit:<id>` → `Created` —
    /// collision-safe by construction (`commit:<sub-id>` is sub-task-id-derived,
    /// unique per sub-area). A second entry over the already-provisioned area is a
    /// no-op (first-entry-only) and the provenance stays `created` (write-once).
    #[test]
    fn provision_commit_doc_records_created_provenance() {
        use engine::state::{Provenance, ProvenanceRecord};

        let area = TempDir::new("provenance-created");
        let sub_dir = area.path();
        let pack = form_d_pack();

        // The recorded sub-workflow is `creates-task: true`, so the re-entry mirror
        // provisions the commit doc on first entry.
        let def = load_workflow_def(
            &pack
                .read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .expect("single-task workflow"),
        )
        .expect("single-task def loads");

        let resolved = no_shadow_resolved();
        let defs = CascadeDefs::new(&resolved, sub_dir);
        provision_on_first_entry(&pack, &defs, &def, sub_dir, "add-rate-limiter")
            .expect("first entry provisions the commit doc");

        // The commit skeleton landed AND its `created` provenance is in the manifest
        // the join consumes.
        assert!(
            state::instance_path(sub_dir, FALLBACK_TYPE, "add-rate-limiter").is_file(),
            "the commit skeleton must be provisioned on first entry",
        );
        let record = ProvenanceRecord::load(sub_dir).expect("provenance manifest loads");
        assert_eq!(
            record.get("commit:add-rate-limiter"),
            Some(Provenance::Created),
            "provision_commit_doc must record the commit doc as `created` in the manifest",
        );

        // A second entry is first-entry-only (no re-provision) and write-once keeps
        // the provenance `created`.
        provision_on_first_entry(&pack, &defs, &def, sub_dir, "add-rate-limiter")
            .expect("a second entry is a no-op");
        let again = ProvenanceRecord::load(sub_dir).expect("provenance manifest reloads");
        assert_eq!(
            again.get("commit:add-rate-limiter"),
            Some(Provenance::Created),
            "a re-entry never flips the recorded `created` provenance",
        );
    }

    /// A pack-default base layer declaring `default-workflow: single-task` — the
    /// base a project `scalar-set` resolves over to prove the delta *applies*.
    fn base_layer() -> engine::cascade::PackDefaultLayer {
        let mut scalars = BTreeMap::new();
        scalars.insert("default-workflow".to_owned(), "single-task".to_owned());
        engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new())
    }

    /// The T3 done-criterion: a hand-authored manifest with a top-level `scalar:`
    /// map loads into an `OverrideLayer` whose `scalar-set` delta, resolved over the
    /// base, **flips** the resolved value (`single-task` → `router`).
    #[test]
    fn manifest_scalar_block_flips_the_resolved_value() {
        let cfg = TempDir::new("manifest-flip");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  default-workflow: router\n",
        )
        .expect("write manifest");

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("router"),
            "the project `scalar:` delta must flip the resolved value over the base",
        );
    }

    /// A manifest with **no** `scalar:` block yields the no-override path: resolved
    /// over the base, the base value is unchanged (`OverrideLayer::empty()`).
    #[test]
    fn manifest_without_scalar_block_yields_empty_layer() {
        let cfg = TempDir::new("manifest-no-scalar");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "deltas: []\n", // a `deltas:`-only manifest carries no scalar block.
        )
        .expect("write manifest");

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a manifest with no `scalar:` block must leave the base value untouched",
        );
    }

    /// A **blank** `scalar:` block (the key present but empty) is the no-override
    /// path too — `serde_yaml_ng` parses `scalar:` with no children as null, which
    /// is not a mapping, so the layer stays empty and the base value survives.
    #[test]
    fn manifest_with_blank_scalar_block_yields_empty_layer() {
        let cfg = TempDir::new("manifest-blank-scalar");
        fs::write(cfg.path().join("manifest.yaml"), "scalar:\n").expect("write manifest");

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a blank `scalar:` block must leave the base value untouched",
        );
    }

    /// A **missing** manifest file (the project layer dir exists but holds no
    /// `manifest.yaml`) yields the no-override path — present-but-empty, not an
    /// error: the layer is the cascade's no-delta base reader.
    #[test]
    fn missing_manifest_file_yields_empty_layer() {
        let cfg = TempDir::new("manifest-missing");
        // No manifest.yaml written.

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("missing manifest is not an error");
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("single-task"),
            "a missing manifest must resolve to the base value (no-override path)",
        );
    }

    /// Every `scalar:` entry is recorded as a `scalar-set` delta: a manifest setting
    /// two declared knobs resolves both over the base. Pins that the loader walks the
    /// whole `scalar:` map (in manifest order, the iteration order `serde_yaml_ng`'s
    /// mapping preserves), not just the first entry.
    #[test]
    fn manifest_records_every_scalar_entry() {
        let mut scalars = BTreeMap::new();
        scalars.insert("default-workflow".to_owned(), "single-task".to_owned());
        scalars.insert(
            "validation.file-state.severity".to_owned(),
            "blocking".to_owned(),
        );
        let base = engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new());

        let cfg = TempDir::new("manifest-multi");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  default-workflow: router\n  validation.file-state.severity: warning\n",
        )
        .expect("write manifest");

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("router"));
        assert_eq!(
            resolved.scalar("validation.file-state.severity"),
            Some("warning"),
            "every `scalar:` entry must be recorded, not just the first",
        );
    }

    /// The read-side determinism invariant, end-to-end through the compose read:
    /// the bare front door resolves the cascade then reads `default-workflow` via
    /// `scalar_required`, so a pack whose `config/knobs` **does not declare**
    /// `default-workflow` fails **loudly** (the `UndeclaredComposeRead` hard error)
    /// rather than silently composing nothing. Pins that the wired compose read
    /// never falls back to a raw `None` for an undeclared key.
    #[test]
    fn undeclared_compose_read_key_fails_loudly() {
        let cfg = TempDir::new("undeclared-read");
        fs::create_dir_all(cfg.path()).expect("mk config dir");

        // A pack whose knob surface omits `default-workflow` entirely — so the
        // cascade-resolved surface never carries it. The intrinsic knobs are
        // present (the engine's load-time assertion requires them); the point is
        // that `default-workflow` specifically is undeclared.
        let knobs = format!(
            "some-other-knob:\n  type: string\n  default: x\n{}",
            crate::pack::intrinsic_knobs_yaml(),
        );
        let pack = FixturePack::with(vec![
            (PackResourceKind::Config, "defaults", "pack-id: dev\n"),
            (PackResourceKind::Config, "knobs", &knobs),
        ]);

        // The live front-door read: resolve the cascade, then read the key through
        // `scalar_required` (the exact two steps `compose_in_repo` runs).
        let (resolved, _overrides) = resolve_cascade(&pack, cfg.path()).expect("cascade resolves");
        let err = resolved
            .scalar_required(DEFAULT_WORKFLOW_KEY)
            .map(str::to_owned)
            .map_err(anyhow::Error::from)
            .expect_err("an undeclared compose-read key must fail loudly");
        let msg = err.to_string();
        assert!(
            msg.contains("default-workflow") && msg.contains("undeclared"),
            "the read of an undeclared key must name it and the closed-surface rule; got: {msg}",
        );
    }

    /// The T3 done-criterion: a [`CascadeStepSource`] consults
    /// [`cascade::Resolved::file_owner`] so a project-owned step id reads the
    /// **project** body from `.jigc/config/steps/<id>.yaml`, while an id the
    /// project does not own reads the **pack** body — the highest-precedence
    /// layer that owns the id wins (`overrides.md` → Resolution algorithm phase 2).
    #[test]
    fn cascade_step_source_reads_project_body_when_project_owns_the_id() {
        let cfg = TempDir::new("cascade-step");
        let steps = cfg.path().join("steps");
        fs::create_dir_all(&steps).expect("mk steps/");
        // The project layer ships its own `implement` step file; `noop` it does not.
        fs::write(steps.join("implement.yaml"), "project implement body\n").expect("w");

        // The pack ships both `implement` and `noop`.
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
            (PackResourceKind::Steps, "noop", "pack noop body\n"),
        ]);

        // The resolved cascade: pack ships both ids, project shadows `implement`.
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned(), "noop".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let source = CascadeStepSource::new(&pack, &resolved, cfg.path());

        // Project owns `implement` → the project file's body is read.
        let implement = source.step("implement").expect("project step resolves");
        assert_eq!(
            implement.body, "project implement body\n",
            "a project-owned id must read the project layer's body",
        );
        assert!(
            source.take_error().is_none(),
            "a clean read records no located error",
        );

        // `noop` is pack-owned → the pack body is read.
        let noop = source.step("noop").expect("pack step resolves");
        assert_eq!(
            noop.body, "pack noop body\n",
            "an id the project does not own must read the pack body",
        );
    }

    /// The T3 located-error half: a `file_owner` of `Project` whose on-disk file
    /// is **missing** is a clear located error — `step()` returns `None` (no
    /// silent pack fall-through past the owning layer) and the source records a
    /// located [`Finding`] naming the absent file, drained via `take_error`.
    #[test]
    fn cascade_step_source_missing_project_file_is_a_located_error() {
        let cfg = TempDir::new("cascade-step-missing");
        fs::create_dir_all(cfg.path().join("steps")).expect("mk steps/");
        // No `implement.yaml` written — the project owns the id but ships no file.

        let pack = FixturePack::with(vec![(
            PackResourceKind::Steps,
            "implement",
            "pack implement body\n",
        )]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let source = CascadeStepSource::new(&pack, &resolved, cfg.path());

        // The project owns the id but its file is missing → `None`, never a silent
        // fall-through to the pack body.
        assert!(
            source.step("implement").is_none(),
            "a project-owned id whose file is missing must not fall through to the pack",
        );
        let finding = source
            .take_error()
            .expect("a missing project-owned file records a located finding");
        assert!(
            finding.location.is_some(),
            "the missing-file fault must be located; got {finding:?}",
        );
        assert!(
            finding.message.contains("implement"),
            "the located error must name the absent step id; got: {}",
            finding.message,
        );
    }

    /// (M43 completion audit) The route floor over `overrides.project-step-missing`:
    /// the finding must survive the serialization seam — `Finding`'s `Serialize`
    /// asserts blocking ⇒ route present, and `overrides.` is **not** route-exempt —
    /// and the route must name the two-branch recovery (restore the project step
    /// file, or drop the project layer's claim on the id).
    #[test]
    fn missing_project_step_finding_routes_through_the_serialization_seam() {
        let cfg = TempDir::new("cascade-step-missing-route");
        fs::create_dir_all(cfg.path().join("steps")).expect("mk steps/");
        // No `implement.yaml` written — the project owns the id but ships no file.

        let pack = FixturePack::with(vec![(
            PackResourceKind::Steps,
            "implement",
            "pack implement body\n",
        )]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let source = CascadeStepSource::new(&pack, &resolved, cfg.path());
        assert!(source.step("implement").is_none(), "the read fails");
        let finding = source.take_error().expect("the fault is recorded");
        assert_eq!(finding.code, "overrides.project-step-missing");

        // The seam: a route-less, non-exempt blocking finding cannot serialize.
        let wire = serde_json::to_value(&finding).expect("the finding serializes");
        let route = wire["route"]
            .as_str()
            .expect("blocking implies a route, never null");
        assert!(
            route.contains("restore") && route.contains("steps/implement.yaml"),
            "the route names the restore-or-drop recovery at the owned file: {route}"
        );
    }

    /// A pack-default-only `creates-task: false` pack: one workflow `wf`
    /// including `step:implement`, the named step, plus the `defaults` /
    /// `commands` config every compose reads. `pack_id` and the step body are
    /// caller-supplied so the two constituents of a composite carry **distinct**
    /// bytes (forced overlap, hardening #7).
    fn step_overlap_pack(pack_id: &str, implement_body: &str) -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                // `default-workflow` is unused on the Form-D-shaped compose path
                // (the workflow id is named), but `defaults` must parse.
                &format!("pack-id: {pack_id}\ndefault-workflow: wf\n"),
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "implement", implement_body),
        ])
    }

    /// T2 done-criterion — **pack-local step includes**: a loser-pack workflow's
    /// `{{include: step:implement}}` composes its **own** pack's `implement`
    /// body, even though the higher-precedence pack ships a **divergent**
    /// `step:implement` that wins the top-level id (`multi-pack.md` → Pack-local
    /// body-reference resolution → Steps; the M3-class corruption averted).
    ///
    /// The composite is `[higher, lower]`. Only `lower` defines `wf-lower`, so
    /// the precedence `read` of `workflow:wf-lower` returns the lower pack's
    /// bytes — but a *globally-merged* step read of `step:implement` would return
    /// `higher`'s divergent body. The pack-of-origin scoping makes `wf-lower`'s
    /// include resolve against `lower` (the workflow's own pack), so the composed
    /// body carries **lower's** implement, never higher's.
    #[test]
    fn loser_pack_workflow_composes_its_own_step_body() {
        let repo = TempDir::new("t2-loser-step");
        let higher = step_overlap_pack("higher", "HIGHER implement body\n");
        // The lower pack renames its workflow so the loser owns a non-colliding
        // top-level workflow id, but keeps the colliding `step:implement` id.
        let mut lower = step_overlap_pack("lower", "LOWER implement body\n");
        let wf_bytes = lower
            .0
            .remove(&(PackResourceKind::Workflows, ResourceId::from("wf")))
            .expect("the base fixture ships `wf`");
        lower.0.insert(
            (PackResourceKind::Workflows, ResourceId::from("wf-lower")),
            wf_bytes,
        );
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        // A no-shadow cascade over a pack-default layer that owns both top-level
        // ids — no project shadows, so the pack-default step arm runs.
        let base = engine::cascade::PackDefaultLayer::new(
            "higher",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");
        let source = CascadeStepSource::new(&composite, &resolved, repo.path());

        let composed = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf-lower",
            &source,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("the loser-pack workflow composes")
        .view;

        assert!(
            composed.text.contains("LOWER implement body"),
            "the loser-pack workflow must compose ITS OWN pack's implement body; got:\n{}",
            composed.text,
        );
        assert!(
            !composed.text.contains("HIGHER implement body"),
            "the precedence winner's divergent step:implement must NOT leak into the \
             loser-pack workflow (the M3-class corruption); got:\n{}",
            composed.text,
        );
    }

    /// T2 control — the precedence **winner**'s own workflow composes its own
    /// body unchanged. The higher pack owns both `wf` (the workflow) and
    /// `step:implement`, so its origin is itself; no regression from the
    /// pack-local scoping.
    #[test]
    fn winner_pack_workflow_composes_its_own_step_body() {
        let repo = TempDir::new("t2-winner-step");
        let higher = step_overlap_pack("higher", "HIGHER implement body\n");
        let lower = step_overlap_pack("lower", "LOWER implement body\n");
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        let base = engine::cascade::PackDefaultLayer::new(
            "higher",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");
        let source = CascadeStepSource::new(&composite, &resolved, repo.path());

        let composed = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf",
            &source,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("the winner-pack workflow composes")
        .view;

        assert!(
            composed.text.contains("HIGHER implement body")
                && !composed.text.contains("LOWER implement body"),
            "the winner-pack workflow composes its own (winning) implement body; got:\n{}",
            composed.text,
        );
    }

    /// T2 floor — a single-pack `Composite([base])` is byte-identical: origin =
    /// base = the pack, so the pack-local scoping is inert and the composed body
    /// matches a compose over the bare pack (the cold-start regression).
    #[test]
    fn single_pack_composite_compose_is_byte_identical() {
        let repo = TempDir::new("t2-floor");
        let base_pack = step_overlap_pack("base", "BASE implement body\n");
        let base_pack_clone = step_overlap_pack("base", "BASE implement body\n");
        let composite = crate::pack::CompositePack::new(vec![Box::new(base_pack)]);

        let base = engine::cascade::PackDefaultLayer::new(
            "base",
            "0.0.0",
            BTreeMap::new(),
            vec!["implement".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");

        let over_composite = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf",
            &CascadeStepSource::new(&composite, &resolved, repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("compose over the single-pack composite")
        .view;
        let over_bare = compose_drained(
            repo.path(),
            "anything",
            &base_pack_clone,
            "wf",
            &CascadeStepSource::new(&base_pack_clone, &resolved, repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("compose over the bare pack")
        .view;

        assert_eq!(
            over_composite.text, over_bare.text,
            "a single-pack composite must compose byte-identically to the bare pack",
        );
    }

    /// A pack whose workflow `wf` includes `step:emit`, whose body renders a
    /// pack-specific `{{cli.<cmd_id>}}` command-ref, and whose `commands.yaml`
    /// declares **only** `cmd_id` — so two constituents of a composite carry
    /// **non-mutual-superset** catalogs (the dev × methodology hazard, hardening
    /// #7: forced overlap on the workflow/step ids, divergent catalog contents).
    fn cmd_overlap_pack(pack_id: &str, cmd_id: &str) -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                &format!("pack-id: {pack_id}\ndefault-workflow: wf\n"),
            ),
            (
                PackResourceKind::Config,
                "commands",
                // The lone catalog entry: this pack ships only `cmd_id`, so the
                // other constituent's `{{cli.X}}` is absent here — the
                // not-mutual-superset condition.
                &format!(
                    "commands:\n  - id: {cmd_id}\n    command: jigc\n    args: [\"noop\"]\n    hint: h\n"
                ),
            ),
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:emit }}\n",
            ),
            (
                PackResourceKind::Steps,
                "emit",
                &format!("{{{{ cli.{cmd_id} }}}}\n"),
            ),
        ])
    }

    /// T3 done-criterion — **pack-local command-refs**: a loser-pack workflow's
    /// `{{cli.X}}` resolves against its **own** pack's `commands.yaml`, even
    /// though the precedence-winner's catalog **omits** `X` (the catalogs are
    /// not mutual supersets). A non-pack-local catalog read would feed the
    /// winner's catalog, and `wf-lower`'s `{{cli.low-cmd}}` would dangle →
    /// `command-ref-resolves` blocks (`multi-pack.md` → Pack-local body-reference
    /// resolution → Command-refs; the same hazard, one resource over).
    #[test]
    fn loser_pack_workflow_resolves_its_own_command_ref() {
        let repo = TempDir::new("t3-loser-cmd");
        let higher = cmd_overlap_pack("higher", "high-cmd");
        // The lower pack renames its workflow to a non-colliding top-level id,
        // but keeps the colliding `step:emit` id (whose body now carries the
        // lower-only `{{cli.low-cmd}}`).
        let mut lower = cmd_overlap_pack("lower", "low-cmd");
        let wf_bytes = lower
            .0
            .remove(&(PackResourceKind::Workflows, ResourceId::from("wf")))
            .expect("the base fixture ships `wf`");
        lower.0.insert(
            (PackResourceKind::Workflows, ResourceId::from("wf-lower")),
            wf_bytes,
        );
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        let base = engine::cascade::PackDefaultLayer::new(
            "higher",
            "0.0.0",
            BTreeMap::new(),
            vec!["emit".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");
        let source = CascadeStepSource::new(&composite, &resolved, repo.path());

        let composed = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf-lower",
            &source,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect(
            "the loser-pack workflow must resolve its own `{{cli.low-cmd}}` against ITS pack's catalog",
        )
        .view;

        assert!(
            composed.text.contains("Run: `jigc noop`"),
            "the loser-pack `{{cli.low-cmd}}` must render from its own pack's catalog; got:\n{}",
            composed.text,
        );
    }

    /// T3 control — the precedence **winner**'s own workflow resolves its own
    /// `{{cli.high-cmd}}` unchanged; no regression from the pack-local scoping.
    /// (The winner pack owns both `wf` and `high-cmd`, so its origin is itself.)
    #[test]
    fn winner_pack_workflow_resolves_its_own_command_ref() {
        let repo = TempDir::new("t3-winner-cmd");
        let higher = cmd_overlap_pack("higher", "high-cmd");
        let lower = cmd_overlap_pack("lower", "low-cmd");
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        let base = engine::cascade::PackDefaultLayer::new(
            "higher",
            "0.0.0",
            BTreeMap::new(),
            vec!["emit".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");
        let source = CascadeStepSource::new(&composite, &resolved, repo.path());

        let composed = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf",
            &source,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("the winner-pack workflow composes its own command-ref")
        .view;

        assert!(
            composed.text.contains("Run: `jigc noop`"),
            "the winner-pack `{{cli.high-cmd}}` must resolve from its own catalog; got:\n{}",
            composed.text,
        );
    }

    /// T3 floor — a single-pack `Composite([base])` is byte-identical: origin =
    /// base = the pack, so the pack-local catalog scoping is inert and the
    /// composed body matches a compose over the bare pack (the cold-start
    /// regression guard, for the command-ref read this task moves).
    #[test]
    fn single_pack_composite_command_ref_is_byte_identical() {
        let repo = TempDir::new("t3-floor");
        let base_pack = cmd_overlap_pack("base", "base-cmd");
        let base_pack_clone = cmd_overlap_pack("base", "base-cmd");
        let composite = crate::pack::CompositePack::new(vec![Box::new(base_pack)]);

        let base = engine::cascade::PackDefaultLayer::new(
            "base",
            "0.0.0",
            BTreeMap::new(),
            vec!["emit".to_owned()],
        );
        let resolved =
            engine::cascade::resolve(&base, None, None).expect("no-shadow cascade resolves");

        let over_composite = compose_drained(
            repo.path(),
            "anything",
            &composite,
            "wf",
            &CascadeStepSource::new(&composite, &resolved, repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("compose over the single-pack composite")
        .view;
        let over_bare = compose_drained(
            repo.path(),
            "anything",
            &base_pack_clone,
            "wf",
            &CascadeStepSource::new(&base_pack_clone, &resolved, repo.path()),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("compose over the bare pack")
        .view;

        assert_eq!(
            over_composite.text, over_bare.text,
            "a single-pack composite must compose its command-refs byte-identically to the bare pack",
        );
    }

    /// A pack whose schema `<schema_id>` declares a `code-anchor` field — a
    /// pack-provided field type — and whose own `field-types.yaml` defines
    /// `code-anchor`. The schema id is caller-supplied so two constituents of a
    /// composite ship **distinct** schema ids (no top-level collision — the
    /// fixture forces a resolution *miss* via the winner's omitted field-type, not
    /// a same-id schema collision, which stays deferred). `defaults`/`commands` are
    /// present so the pack parses on every read path.
    fn field_type_pack(pack_id: &str, schema_id: &str) -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                &format!("pack-id: {pack_id}\ndefault-workflow: wf\n"),
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            (
                PackResourceKind::Config,
                "field-types",
                "- { name: code-anchor, adjudicator: doc-code, check: symbol-exists }\n",
            ),
            (
                PackResourceKind::Schemas,
                schema_id,
                &format!(
                    "type: {schema_id}\nlocation: {schema_id}/\nsections:\n  - id: meta\n    header: true\n    fields:\n      - {{ id: anchor, type: code-anchor, check: symbol-exists }}\n"
                ),
            ),
        ])
    }

    /// The same pack but shipping a `field-types.yaml` that **omits** `code-anchor`
    /// — and a schema that uses only a **native** type, so the pack loads its own
    /// schema. This is the precedence **winner** in the loser test: its catalog of
    /// field types is what a *non-pack-local* (composite) read would feed every
    /// schema, so the loser's `code-anchor` would resolve against this omitting
    /// surface and fail `UnknownFieldType`.
    fn no_field_type_pack(pack_id: &str, schema_id: &str) -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                &format!("pack-id: {pack_id}\ndefault-workflow: wf\n"),
            ),
            (PackResourceKind::Config, "commands", "commands: []\n"),
            // The omitting field-types.yaml — an empty declared set (`code-anchor`
            // absent). A composite `read(Config, field-types)` would return THIS
            // (the precedence winner's), so a loser schema's `code-anchor` would
            // dangle unless its field-types resolve pack-locally.
            (PackResourceKind::Config, "field-types", "[]\n"),
            (
                PackResourceKind::Schemas,
                schema_id,
                &format!(
                    "type: {schema_id}\nlocation: {schema_id}/\nsections:\n  - id: meta\n    header: true\n    fields:\n      - {{ id: title, type: string }}\n"
                ),
            ),
        ])
    }

    /// T4 done-criterion — **pack-local field-types**: a loser-pack schema's
    /// declared field-type names resolve against its **own** pack's
    /// `field-types.yaml`, even though the precedence-winning pack ships a
    /// `field-types.yaml` that **omits** the type (`code-anchor`). A non-pack-local
    /// (composite) field-type read would feed the winner's omitting surface and the
    /// loser schema would fail `UnknownFieldType`; the pack-of-origin scoping makes
    /// `lower-doc`'s `code-anchor` resolve against the lower pack (the schema's own
    /// pack), so it loads (`multi-pack.md` → Pack-local body-reference resolution →
    /// Field-types are the same rule, one definition kind over).
    #[test]
    fn loser_pack_schema_resolves_its_own_field_type() {
        // Higher (precedence winner) omits `code-anchor`; lower ships it + the
        // schema that uses it. The schema ids are disjoint (no top-level collision).
        let higher = no_field_type_pack("higher", "higher-doc");
        let lower = field_type_pack("lower", "lower-doc");
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        let resolved = no_shadow_resolved();
        let cfg = TempDir::new("t4-loser-fieldtype");
        let defs = CascadeDefs::new(&resolved, cfg.path());

        let schemas = defs
            .all_schemas(&composite)
            .expect("the loser-pack schema must resolve its own pack's `code-anchor` field type");
        assert!(
            schemas.contains_key("lower-doc"),
            "the loser-pack `lower-doc` schema must load (its `code-anchor` resolves from \
             ITS OWN pack's field-types.yaml, not the winner's omitting surface); got {:?}",
            schemas.keys().collect::<Vec<_>>(),
        );
    }

    /// T4 control — the precedence **winner**'s own schema loads unchanged: its
    /// origin is itself, so its native-typed `higher-doc` resolves against its own
    /// field-types regardless of the pack-local scoping (no regression).
    #[test]
    fn winner_pack_schema_loads() {
        let higher = no_field_type_pack("higher", "higher-doc");
        let lower = field_type_pack("lower", "lower-doc");
        let composite = crate::pack::CompositePack::new(vec![Box::new(higher), Box::new(lower)]);

        let resolved = no_shadow_resolved();
        let cfg = TempDir::new("t4-winner-fieldtype");
        let defs = CascadeDefs::new(&resolved, cfg.path());

        let schemas = defs
            .all_schemas(&composite)
            .expect("both packs' schemas load");
        assert!(
            schemas.contains_key("higher-doc"),
            "the winner-pack `higher-doc` schema must load; got {:?}",
            schemas.keys().collect::<Vec<_>>(),
        );
    }

    /// T4 floor — a single-pack `Composite([base])` is byte-identical: origin =
    /// base = the pack, so the pack-local field-type scoping is inert and the
    /// loaded schema set matches a load over the bare pack (the cold-start
    /// regression guard for the field-type read this task moves).
    #[test]
    fn single_pack_composite_field_types_are_byte_identical() {
        let base_pack = field_type_pack("base", "base-doc");
        let base_pack_clone = field_type_pack("base", "base-doc");
        let composite = crate::pack::CompositePack::new(vec![Box::new(base_pack)]);

        let resolved = no_shadow_resolved();
        let cfg = TempDir::new("t4-floor-fieldtype");
        let defs = CascadeDefs::new(&resolved, cfg.path());

        let over_composite = defs
            .all_schemas(&composite)
            .expect("load over the single-pack composite");
        let over_bare = defs
            .all_schemas(&base_pack_clone)
            .expect("load over the bare pack");

        assert_eq!(
            over_composite.keys().collect::<Vec<_>>(),
            over_bare.keys().collect::<Vec<_>>(),
            "a single-pack composite must load the same schema set as the bare pack",
        );
        assert!(
            over_composite.contains_key("base-doc"),
            "the single-pack schema with a pack-provided field type must load",
        );
    }

    /// The CLI `--explain` tree seam over a real cascade: it reads the pack
    /// workflow's include list, tags each step against the resolved
    /// `file_owner` (phase-2 by-id shadowing), and threads any scoped
    /// `structural-op` delta into the engine builder. The project shadows the
    /// `implement` step file, so the no-delta tree tags `implement` **project**
    /// and `locate` **pack-default** with `overrides_applied: 0` — the
    /// file-owner provenance the CLI half of T1 wires (`overrides.md` →
    /// Resolution algorithm phase 2; `workflow-dialect.md` → `--explain`).
    #[test]
    fn cli_resolution_tree_tags_each_step_by_resolved_file_owner() {
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: true\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "pack locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
        ]);
        // The cascade: pack owns both steps + the workflow; the project shadows the
        // `implement` step file (phase-2 by-id shadowing), so `file_owner` reports
        // it project-owned.
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["wf".to_owned(), "locate".to_owned(), "implement".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        // `wf` is not project-owned (only the `implement` step is shadowed), so the
        // workflow definition reads from the pack — the project_config path is never
        // consulted for it.
        let tree = build_resolution_tree(&pack, &resolved, Path::new("/nonexistent"), &[], "wf")
            .expect("tree builds");

        assert_eq!(tree.workflow, "wf");
        assert_eq!(tree.overrides_applied, 0);
        let by_owner: Vec<(&str, engine::cascade::LayerKind)> = tree
            .steps
            .iter()
            .map(|s| (s.id.as_str(), s.layer))
            .collect();
        assert_eq!(
            by_owner,
            vec![
                ("locate", engine::cascade::LayerKind::PackDefault),
                ("implement", engine::cascade::LayerKind::Project),
            ],
            "each step is tagged by its resolved file owner, in include order",
        );
        assert!(
            tree.steps.iter().all(|s| s.replaces.is_none()),
            "a no-delta tree carries no replace annotation",
        );
    }

    /// The CLI seam threads a **scoped** `replace-step` delta through to the
    /// engine builder: a project `replace-step workflow:wf#implement →
    /// step:project-implement` yields `overrides_applied: 1`, the replacing step
    /// tagged project at the same position with the `replaces implement at
    /// position 2` annotation, while a same-manifest delta scoped to *another*
    /// workflow is filtered out (`scoped_deltas`).
    #[test]
    fn cli_resolution_tree_threads_scoped_replace_delta() {
        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "wf",
                "---\nwhen: x\ncreates-task: true\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n",
            ),
            (PackResourceKind::Steps, "locate", "pack locate body\n"),
            (
                PackResourceKind::Steps,
                "implement",
                "pack implement body\n",
            ),
        ]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec![
                "wf".to_owned(),
                "locate".to_owned(),
                "implement".to_owned(),
                "project-implement".to_owned(),
            ],
        );
        let project = OverrideLayer::empty().shadow_file("project-implement");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let deltas = vec![
            StructuralDelta::Replace {
                target: StructuralTarget::parse("workflow:wf#implement", None).expect("target"),
                step: "project-implement".to_owned(),
            },
            // A delta for a different workflow — `scoped_deltas` must drop it, so it
            // neither applies nor inflates `overrides_applied`.
            StructuralDelta::Remove {
                target: StructuralTarget::parse("workflow:other#locate", None).expect("target"),
            },
        ];

        let tree =
            build_resolution_tree(&pack, &resolved, Path::new("/nonexistent"), &deltas, "wf")
                .expect("tree builds");

        assert_eq!(
            tree.overrides_applied, 1,
            "only the `wf`-scoped delta counts; the `other`-scoped delta is filtered",
        );
        let replacing = &tree.steps[1];
        assert_eq!(replacing.id, "project-implement");
        assert_eq!(replacing.layer, engine::cascade::LayerKind::Project);
        assert_eq!(
            replacing.replaces,
            Some(engine::result::Replacement {
                replaced: "implement".to_owned(),
                position: 2,
            }),
        );
    }

    /// A fixture pack used to drive the `--explain` collision-winner detection: it
    /// ships one resource in **every** id-space the derivation walks — `config/knobs`,
    /// `schemas/commit`, `workflows/wf` — and one in each id-space the derivation
    /// **excludes by rule** (`steps/implement`, `config/commands`), each under its own
    /// `pack-id`. Composing two of these collides all five, so the test can assert both
    /// what is adjudicated and what is not.
    fn collision_pack(pack_id: &str) -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                &format!("pack-id: {pack_id}\ndefault-workflow: wf\n"),
            ),
            (
                PackResourceKind::Config,
                "knobs",
                "default-workflow:\n  type: enum\n",
            ),
            (PackResourceKind::Config, "commands", "commands: {}\n"),
            (PackResourceKind::Schemas, "commit", "type: commit\n"),
            (PackResourceKind::Workflows, "wf", "id: wf\nsteps: []\n"),
            (PackResourceKind::Steps, "implement", "id: implement\n"),
        ])
    }

    /// The **highest-precedence** pack of the three-pack fixture: it ships its own
    /// identity and one doctype nobody else owns, and **nothing** that collides. Every
    /// attribution `--explain` prints over `[house ▸ b ▸ c]` must therefore name a pack
    /// *below* it — which is exactly the misattribution T3 fixes.
    fn extension_only_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Config,
                "defaults",
                "pack-id: house\ndefault-workflow: wf\n",
            ),
            (PackResourceKind::Schemas, "note", "type: note\n"),
        ])
    }

    /// T3 done-criterion (the unit half) — the `--explain` collision winner names the
    /// pack that **owns** the id, and the adjudicated set is **derived** from the
    /// composed pack-set rather than hand-listed.
    ///
    /// Three assertions over one axis — *a pack attribution names the pack that
    /// actually provided the thing*:
    ///
    /// 1. Over a **three-pack** composite whose highest-precedence pack (`house`) ships
    ///    neither the colliding doctype nor the colliding workflow, every winner names
    ///    the real owner (`b`) — not `house`, which the pre-T3 read of
    ///    `provenance_segments()[0]` named.
    /// 2. The set is derived, so **every** precedence-resolved id-space is covered: the
    ///    `commit` doctype, the `wf` workflow **and** the `knobs` file. An id only one
    ///    pack owns (`note`) adjudicates nothing.
    /// 3. The id-spaces that resolve **pack-locally** (`Steps`, `Config:commands`) are
    ///    excluded by rule — both packs ship them, and neither may produce a winner
    ///    line, because no pack *wins* a resource each pack keeps for its own
    ///    definitions (`design/multi-pack.md` → Pack-local body-reference resolution).
    ///
    /// A **single-pack** composite yields none (the byte-identity floor; hardening #5 —
    /// the omitting context).
    #[test]
    fn collision_winners_name_the_owning_pack_over_every_derived_id_space() {
        let labels = |winners: &[engine::result::CollisionWinner]| -> Vec<(String, String)> {
            winners
                .iter()
                .map(|w| {
                    (
                        w.collision.clone(),
                        format!("{}/{}", w.pack_id, w.pack_version),
                    )
                })
                .collect()
        };

        // Three-pack composite: `house` is highest-precedence yet owns none of the
        // colliding ids; `b` outranks `c` on every one of them.
        let three = crate::pack::CompositePack::new(vec![
            Box::new(extension_only_pack()),
            Box::new(collision_pack("b")),
            Box::new(collision_pack("c")),
        ]);
        let winners = labels(&collision_winners(&three));
        assert_eq!(
            winners,
            vec![
                ("doctype:commit".to_owned(), "b/0.0.0".to_owned()),
                ("workflow:wf".to_owned(), "b/0.0.0".to_owned()),
                ("config:knobs".to_owned(), "b/0.0.0".to_owned()),
            ],
            "each winner must name the pack that OWNS the id (`b`), never the \
             highest-precedence pack (`house`); the derived set must cover the doctype, \
             the workflow and the knob file, and must exclude the pack-local \
             `steps`/`commands` id-spaces and the single-owner `note`",
        );

        // Two-pack composite: `methodology` is highest-precedence AND the owner, so the
        // pre-T3 surface's answer is preserved where it was already right.
        let composite = crate::pack::CompositePack::new(vec![
            Box::new(collision_pack("methodology")),
            Box::new(collision_pack("dev")),
        ]);
        assert_eq!(
            labels(&collision_winners(&composite)),
            vec![
                ("doctype:commit".to_owned(), "methodology/0.0.0".to_owned()),
                ("workflow:wf".to_owned(), "methodology/0.0.0".to_owned()),
                ("config:knobs".to_owned(), "methodology/0.0.0".to_owned()),
            ],
            "where the precedence winner IS the owner, the named winner is unchanged",
        );

        // Single-pack composite: no id is owned by >1 pack → NO winners (the
        // byte-identity floor; the omitting context).
        let single = crate::pack::CompositePack::new(vec![Box::new(collision_pack("dev"))]);
        assert!(
            collision_winners(&single).is_empty(),
            "a single-pack composition adjudicates no cross-pack collision",
        );
        // A bare `FixturePack` (not even a composite) likewise yields none.
        assert!(
            collision_winners(&collision_pack("dev")).is_empty(),
            "a non-composite pack adjudicates no cross-pack collision",
        );
    }

    /// T2 done-criterion — the `--explain` per-pack provenance inputs: over a
    /// **two-pack** composite, [`pack_inputs`] yields **one entry per composed pack**,
    /// **highest-precedence first**, each zipping the pack's `provenance_segments()`
    /// id/version with its `provenance_entries()` resolving path + blake3 content-hash
    /// — naming each pack's **own** path/hash, never the precedence-winner's. A
    /// **single-pack** composite degrades to exactly **one** entry (the byte-identity
    /// floor; the one-segment `provenance_segments()` degrade) (`design/multi-pack.md`
    /// → Provenance under N packs; `design/worked-examples.md` → flow 17 assertion 5).
    #[test]
    fn pack_inputs_names_each_composed_pack_by_path_and_hash_in_order() {
        // Two-pack composite: `methodology` highest-precedence (first), `dev` second.
        let composite = crate::pack::CompositePack::new(vec![
            Box::new(collision_pack("methodology")),
            Box::new(collision_pack("dev")),
        ]);
        let inputs = pack_inputs(&composite);

        assert_eq!(
            inputs.len(),
            2,
            "one provenance entry per composed pack, in precedence order; got:\n{inputs:?}",
        );
        // Highest-precedence first, each naming its OWN id/version.
        assert_eq!(inputs[0].pack_id, "methodology");
        assert_eq!(inputs[0].pack_version, "0.0.0");
        assert_eq!(inputs[1].pack_id, "dev");
        assert_eq!(inputs[1].pack_version, "0.0.0");

        // The path/hash come from each pack's OWN `provenance_entries()`, zipped
        // parallel to its segment — so the zip is by-pack, never cross-wired.
        let entries = composite.provenance_entries();
        assert_eq!(inputs[0].path, entries[0].path);
        assert_eq!(inputs[0].content_hash, entries[0].content_hash);
        assert_eq!(inputs[1].path, entries[1].path);
        assert_eq!(inputs[1].content_hash, entries[1].content_hash);
        // A genuine content-hash, not empty (the determinism-visible identity).
        assert!(
            !inputs[0].content_hash.is_empty() && !inputs[1].content_hash.is_empty(),
            "each pack's content-hash must be non-empty; got:\n{inputs:?}",
        );

        // Single-pack composite: exactly ONE entry (the byte-identity floor).
        let single = crate::pack::CompositePack::new(vec![Box::new(collision_pack("dev"))]);
        let single_inputs = pack_inputs(&single);
        assert_eq!(
            single_inputs.len(),
            1,
            "a single-pack composition yields exactly one provenance input; got:\n{single_inputs:?}",
        );
        assert_eq!(single_inputs[0].pack_id, "dev");
    }

    /// A non-string scalar value (`bool`) is recorded as its YAML scalar string
    /// (`true`) — the cascade stores opaque strings; typed adjudication is the
    /// `config set` write path (T6), not this loader.
    #[test]
    fn manifest_non_string_scalar_renders_to_its_yaml_string() {
        let mut scalars = BTreeMap::new();
        scalars.insert("some-flag".to_owned(), "false".to_owned());
        let base = engine::cascade::PackDefaultLayer::new("dev", "0.0.0", scalars, Vec::new());

        let cfg = TempDir::new("manifest-bool");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  some-flag: true\n",
        )
        .expect("write manifest");

        let (layer, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(cfg.path()).expect("manifest loads");
        let resolved = engine::cascade::resolve(&base, None, Some(&layer)).expect("resolves");

        assert_eq!(
            resolved.scalar("some-flag"),
            Some("true"),
            "a bool scalar must record as its YAML string form",
        );
    }

    /// A manifest carrying a hand-authored `tracked-fork` entry **alongside** a
    /// `scalar:` block and a `slot-fill` delta round-trips: the loader parses the
    /// fork back to the T1 [`TrackedForkDelta`] (target + `base-version` +
    /// `base-hash`) without erroring, and the other blocks survive its presence —
    /// the `scalar:` flips the resolved value and the `slot-fill` delta is still
    /// returned (`overrides.md` → Delta representation; `storage.md` → Config
    /// layout: one `deltas:` list, every kind).
    #[test]
    fn manifest_tracked_fork_entry_round_trips_alongside_scalar_and_slot_fill() {
        let cfg = TempDir::new("manifest-tracked-fork");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "scalar:\n  \
             default-workflow: router\n\
             deltas:\n  \
             - kind: slot-fill\n    \
             target: step:implement#extra-guidance\n    \
             content: fills/extra-guidance.md\n  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n",
        )
        .expect("write manifest");

        let (layer, deltas, slot_fills, forks, _bases) =
            load_project_layer(cfg.path()).expect("a tracked-fork entry must not error the load");

        // The fork round-trips to the T1 delta.
        assert_eq!(
            forks,
            vec![TrackedForkDelta {
                target: StructuralTarget {
                    workflow_id: "single-task".to_owned(),
                    anchor: Anchor::At("implement".to_owned()),
                },
                base_version: "v1".to_owned(),
                base_hash: "a3f9deadbeef".to_owned(),
            }],
            "the `tracked-fork` entry must parse back to the recorded T1 delta",
        );

        // The other blocks survive the fork entry's presence.
        let resolved =
            engine::cascade::resolve(&base_layer(), None, Some(&layer)).expect("resolves");
        assert_eq!(
            resolved.scalar("default-workflow"),
            Some("router"),
            "the `scalar:` block must still flip the resolved value",
        );
        assert_eq!(
            slot_fills,
            vec![SlotFillDelta {
                target: SlotFillTarget::parse("step:implement#extra-guidance")
                    .expect("valid slot-fill target"),
                content_id: "extra-guidance".to_owned(),
            }],
            "the `slot-fill` delta must survive the fork entry's presence",
        );
        assert!(
            deltas.is_empty(),
            "the fork is not a structural-op, so no structural delta is recorded",
        );
    }

    /// The determinism guard: the structural-op and slot-fill vectors are
    /// **byte-identical** whether or not a `tracked-fork` entry sits in the same
    /// `deltas:` list — the fork is recording surface, never fed to compose, so it
    /// must not perturb the phase-4 / phase-5 inputs.
    #[test]
    fn tracked_fork_entry_does_not_perturb_structural_or_slot_fill_vectors() {
        let without = "deltas:\n  \
             - kind: insert-step\n    \
             target: workflow:single-task\n    \
             after: implement\n    \
             with: step:project-implement\n  \
             - kind: slot-fill\n    \
             target: step:implement#extra-guidance\n    \
             content: fills/extra-guidance.md\n";
        let with = format!(
            "{without}  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n"
        );

        let cfg_without = TempDir::new("fork-guard-without");
        fs::write(cfg_without.path().join("manifest.yaml"), without).expect("write manifest");
        let cfg_with = TempDir::new("fork-guard-with");
        fs::write(cfg_with.path().join("manifest.yaml"), with).expect("write manifest");

        let (_l0, deltas0, slot_fills0, forks0, _bases0) =
            load_project_layer(cfg_without.path()).expect("loads");
        let (_l1, deltas1, slot_fills1, forks1, _bases1) =
            load_project_layer(cfg_with.path()).expect("loads");

        assert_eq!(
            deltas0, deltas1,
            "the structural-op vector must be identical with or without the fork entry",
        );
        assert_eq!(
            slot_fills0, slot_fills1,
            "the slot-fill vector must be identical with or without the fork entry",
        );
        assert!(
            forks0.is_empty(),
            "the fork-free manifest records no fork delta",
        );
        assert_eq!(
            forks1.len(),
            1,
            "the fork-bearing manifest records exactly its one fork delta",
        );
    }

    /// The M5 replace/remove base-hash basis: a manifest seeds a `replace-step`
    /// entry **carrying** `base-version`/`base-hash` and a `remove-step` entry
    /// **without** them. The loader returns the structural deltas unchanged (the
    /// replace + remove both compose exactly as before) **plus** a
    /// [`StructuralBasis`] record for the replace target **only** — the remove,
    /// lacking the optional keys, loads clean (no basis, never an error)
    /// (`overrides.md` → Per-kind base-hash basis; backward-compat).
    #[test]
    fn replace_remove_basis_records_present_keys_and_loads_absent_clean() {
        let cfg = TempDir::new("rr-basis");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "deltas:\n  \
             - kind: replace-step\n    \
             target: workflow:single-task#implement\n    \
             with: step:project-implement\n    \
             base-version: v1\n    \
             base-hash: a3f9deadbeef\n  \
             - kind: remove-step\n    \
             target: workflow:single-task#validate\n",
        )
        .expect("write manifest");

        let (_layer, deltas, _slot_fills, _forks, bases) =
            load_project_layer(cfg.path()).expect("manifest loads");

        // The structural-op vector is unchanged — the basis never perturbs compose.
        assert_eq!(
            deltas,
            vec![
                StructuralDelta::Replace {
                    target: StructuralTarget {
                        workflow_id: "single-task".to_owned(),
                        anchor: Anchor::At("implement".to_owned()),
                    },
                    step: "project-implement".to_owned(),
                },
                StructuralDelta::Remove {
                    target: StructuralTarget {
                        workflow_id: "single-task".to_owned(),
                        anchor: Anchor::At("validate".to_owned()),
                    },
                },
            ],
            "the structural deltas must be byte-identical to the no-basis parse",
        );

        // Exactly one basis: the replace target. The remove carried no keys, so it
        // records no basis (and is not an error).
        assert_eq!(
            bases,
            vec![StructuralBasis {
                target: StructuralTarget {
                    workflow_id: "single-task".to_owned(),
                    anchor: Anchor::At("implement".to_owned()),
                },
                base_version: "v1".to_owned(),
                base_hash: "a3f9deadbeef".to_owned(),
            }],
            "only the replace target — carrying both keys — records a basis",
        );
    }

    /// A malformed `replace-step` / `remove-step` basis — exactly **one** of
    /// `base-version` / `base-hash` present (the pair is all-or-nothing for these
    /// optional-basis kinds) — is a **clear, located error**, not a panic: a
    /// half-recorded basis cannot round-trip into the (M5) compare (`overrides.md`
    /// → Per-kind base-hash basis).
    #[test]
    fn replace_remove_partial_basis_is_a_clear_error_not_a_panic() {
        for (kind_block, missing) in [
            (
                "- kind: replace-step\n    \
                 target: workflow:single-task#implement\n    \
                 with: step:project-implement\n    \
                 base-version: v1\n",
                "base-hash",
            ),
            (
                "- kind: remove-step\n    \
                 target: workflow:single-task#validate\n    \
                 base-hash: a3f9deadbeef\n",
                "base-version",
            ),
        ] {
            let cfg = TempDir::new("rr-partial-basis");
            fs::write(
                cfg.path().join("manifest.yaml"),
                format!("deltas:\n  {kind_block}"),
            )
            .expect("write manifest");

            // The 4-tuple+ `OverrideLayer` is not `Debug`, so match the error out
            // rather than `expect_err` the whole `Ok` value.
            let msg = match load_project_layer(cfg.path()) {
                Ok(_) => panic!("a partial `{missing}`-missing basis must be an error"),
                Err(e) => e.to_string(),
            };
            assert!(
                msg.contains(missing),
                "the error must name the missing `{missing}` field; got: {msg}",
            );
        }
    }

    /// A malformed `tracked-fork` entry — `base-hash` missing — is a **clear,
    /// located error**, not a panic: the recorded basis is the pinned ancestor the
    /// (M5) compare reads, so an entry that omits it cannot round-trip (`overrides.md`
    /// → `tracked-fork` hash basis).
    #[test]
    fn tracked_fork_missing_base_hash_is_a_clear_error_not_a_panic() {
        let cfg = TempDir::new("fork-no-hash");
        fs::write(
            cfg.path().join("manifest.yaml"),
            "deltas:\n  \
             - kind: tracked-fork\n    \
             target: workflow:single-task#implement\n    \
             base-version: v1\n",
        )
        .expect("write manifest");

        // The 4-tuple's `OverrideLayer` is not `Debug`, so match the error out
        // rather than `expect_err` the whole `Ok` value.
        let msg = match load_project_layer(cfg.path()) {
            Ok(_) => panic!("a `tracked-fork` missing `base-hash` must be an error"),
            Err(e) => e.to_string(),
        };
        assert!(
            msg.contains("base-hash"),
            "the error must name the missing `base-hash` field; got: {msg}",
        );
    }

    /// T1 done-criterion — a project `workflows/<id>.yaml` shadow lands in the
    /// cascade `files` surface, so `file_owner(<id>)` resolves to `Project`; a
    /// `schemas/<id>.yaml` shadow does the same; an unshadowed id stays `None`
    /// (no layer owns it). The whole-file definition shadow that lets a project
    /// override a workflow / doctype definition (`overrides.md` → Authored metadata
    /// on a definition resolves by whole-file shadow). Without this wiring the two
    /// new shadow dirs are inert (only `steps/` feeds the surface today), so the
    /// `Project` owner assertions fail.
    #[test]
    fn project_workflow_and_schema_shadows_resolve_to_project_owner() {
        let cfg = TempDir::new("def-shadow");
        let project_config = cfg.path();
        // Shadow `single-task` (a workflow) and `adr` (a doctype) by writing native
        // files into the project layer's two new definition dirs.
        let workflows = project_config.join("workflows");
        let schemas = project_config.join("schemas");
        fs::create_dir_all(&workflows).expect("mk workflows dir");
        fs::create_dir_all(&schemas).expect("mk schemas dir");
        fs::write(workflows.join("single-task.yaml"), "project single-task\n")
            .expect("write workflow shadow");
        fs::write(schemas.join("adr.yaml"), "project adr\n").expect("write schema shadow");

        let pack = make_pack().expect("the shipped pack passes its own freeze gate");
        let (project, _deltas, _slot_fills, _forks, _bases) =
            load_project_layer(project_config).expect("project layer loads");
        let resolved = resolve_layers(pack.as_ref(), &project).expect("cascade resolves");

        // The two shadowed definition ids resolve to the project layer.
        assert_eq!(
            resolved.file_owner("single-task"),
            Some(cascade::LayerKind::Project),
            "a project `workflows/single-task.yaml` must own the `single-task` id",
        );
        assert_eq!(
            resolved.file_owner("adr"),
            Some(cascade::LayerKind::Project),
            "a project `schemas/adr.yaml` must own the `adr` id",
        );
        // An id no project file shadows stays unowned (the pack ships definitions but
        // does not feed the cascade `files` surface, so `file_owner` is `None`).
        assert_eq!(
            resolved.file_owner("router"),
            None,
            "an unshadowed id must not resolve to any owner",
        );
    }

    /// T1 namespace-hazard guard — `cascade::resolve` builds **one** `file_owners`
    /// map keyed by the bare id, shared across step / workflow / schema ids. Two
    /// project files of the **same bare id** in different dirs (here a `steps/foo.yaml`
    /// alongside a `workflows/foo.yaml`) would both `shadow_file("foo")` and silently
    /// collapse to one owner entry — the shared-keyspace hazard. The load must reject
    /// it with a routed blocking `config.shadow-id-collision` finding, never silently
    /// absorb it (`overrides.md` → Native-file id = filename basename; one id, one file).
    #[test]
    fn same_id_project_step_and_workflow_shadow_is_a_routed_blocking_finding() {
        let cfg = TempDir::new("shadow-collision");
        let project_config = cfg.path();
        let steps = project_config.join("steps");
        let workflows = project_config.join("workflows");
        fs::create_dir_all(&steps).expect("mk steps dir");
        fs::create_dir_all(&workflows).expect("mk workflows dir");
        // The same bare id `foo` in two definition dirs — the collapse hazard.
        fs::write(steps.join("foo.yaml"), "step foo\n").expect("write step shadow");
        fs::write(workflows.join("foo.yaml"), "workflow foo\n").expect("write workflow shadow");

        // The 5-tuple's `OverrideLayer` is not `Debug`, so match the error out.
        let msg = match load_project_layer(project_config) {
            Ok(_) => panic!("a same-id step+workflow project shadow must be rejected"),
            Err(e) => e.to_string(),
        };
        assert!(
            msg.contains("route:"),
            "the rejection must be routed (carry a route line); got: {msg}",
        );
        assert!(
            msg.contains("foo") && msg.contains("steps/") && msg.contains("workflows/"),
            "the rejection must name the colliding id `foo` + both dirs; got: {msg}",
        );

        // A disjoint set (the same id in only one dir) loads cleanly.
        let ok = TempDir::new("shadow-disjoint");
        let ok_workflows = ok.path().join("workflows");
        fs::create_dir_all(&ok_workflows).expect("mk workflows dir");
        fs::write(ok_workflows.join("foo.yaml"), "workflow foo\n").expect("write workflow shadow");
        load_project_layer(ok.path()).expect("a disjoint shadow set must load cleanly");
    }

    /// T2 done-criterion — a project `workflows/<id>.yaml` **whole-file shadow**
    /// changes the composed output: composing the shadowed `flow` reads the
    /// project definition (a different include list → a different body) rather than
    /// the pack's, while composing an **unshadowed** `other` workflow stays
    /// **byte-identical** to the pack-default (hardening #5 — a context that omits
    /// the shadow target must be inert, never error). Without routing `read_workflow`
    /// through `file_owner` the project shadow is inert and both compose the pack
    /// body (`overrides.md` → Authored metadata on a definition resolves by whole-
    /// file shadow; Resolution algorithm phase 2).
    #[test]
    fn compose_routes_workflow_read_through_the_project_file_shadow() {
        let repo = TempDir::new("wf-shadow-compose");
        let cfg = TempDir::new("wf-shadow-cfg");
        let project_config = cfg.path();
        // The project shadows `flow` with a definition that includes a project-only
        // step id (`project-body`) the pack workflow never references.
        let workflows = project_config.join("workflows");
        fs::create_dir_all(&workflows).expect("mk workflows dir");
        fs::write(
            workflows.join("flow.yaml"),
            "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:project-body }}\n",
        )
        .expect("write workflow shadow");

        let pack = FixturePack::with(vec![
            (PackResourceKind::Config, "commands", "commands: []\n"),
            // The pack `flow` includes the pack `pack-body` step.
            (
                PackResourceKind::Workflows,
                "flow",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:pack-body }}\n",
            ),
            // A second workflow the project does NOT shadow — the omit-the-target arm.
            (
                PackResourceKind::Workflows,
                "other",
                "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:pack-body }}\n",
            ),
            (PackResourceKind::Steps, "pack-body", "PACK BODY MARKER\n"),
            (
                PackResourceKind::Steps,
                "project-body",
                "PROJECT BODY MARKER\n",
            ),
        ]);
        let source = PackStepSource { pack: &pack };

        // The cascade: pack declares both workflow ids + both steps; the project
        // shadows the `flow` workflow file (phase-2 by-id shadowing).
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec![
                "flow".to_owned(),
                "other".to_owned(),
                "pack-body".to_owned(),
                "project-body".to_owned(),
            ],
        );
        let project = OverrideLayer::empty().shadow_file("flow");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");
        let defs = CascadeDefs::new(&resolved, project_config);

        // (a) The shadowed `flow` composes the PROJECT definition's body.
        let shadowed = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &defs,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("shadowed flow composes")
        .view;
        assert!(
            shadowed.text.contains("PROJECT BODY MARKER")
                && !shadowed.text.contains("PACK BODY MARKER"),
            "the project shadow's whole-file definition must drive compose; got:\n{}",
            shadowed.text,
        );

        // (b) The same `flow` over a NO-SHADOW cascade composes the pack body — the
        // shadow is what changed the output, proven by the divergence.
        let pack_baseline = compose_core(
            repo.path(),
            "anything",
            &pack,
            "flow",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), project_config),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("pack-baseline flow composes")
        .view;
        assert!(
            pack_baseline.text.contains("PACK BODY MARKER"),
            "the no-shadow baseline must read the pack body; got:\n{}",
            pack_baseline.text,
        );
        assert_ne!(
            shadowed.text, pack_baseline.text,
            "a whole-file workflow shadow must CHANGE the composed output vs the pack baseline",
        );

        // (c) The omit-the-target arm: the UNSHADOWED `other` composes byte-identical
        // under the shadow cascade and the no-shadow cascade (the shadow is inert for
        // an id it does not own — hardening #5).
        let other_under_shadow = compose_core(
            repo.path(),
            "anything",
            &pack,
            "other",
            &source,
            &defs,
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("unshadowed other composes under the shadow cascade")
        .view
        .text;
        let other_pack = compose_core(
            repo.path(),
            "anything",
            &pack,
            "other",
            &source,
            &CascadeDefs::new(&no_shadow_resolved(), project_config),
            &ComposeOverrides::structural(Vec::new()),
            MilestoneFeed::none(),
            None,
            None,
            false,
        )
        .expect("unshadowed other composes under the no-shadow cascade")
        .view
        .text;
        assert_eq!(
            other_under_shadow, other_pack,
            "an unshadowed workflow must compose byte-identical to pack-default \
             whether or not another definition is shadowed (the shadow is inert here)",
        );
    }

    /// T2 done-criterion (the `all_schemas` half) — a project `schemas/<id>.yaml`
    /// **whole-file shadow** wins on the `all_schemas` read the compose / finalize /
    /// validate paths consume: the shadowed doctype carries the project file's
    /// fields (here a changed `location:`, which `committed_store` reads), while an
    /// unshadowed doctype keeps its pack definition. Whole-file REPLACE, never a
    /// field-merge (`overrides.md` → Authored metadata … resolves by whole-file
    /// shadow). Without routing `all_schemas` through `file_owner` the shadow is
    /// inert and every doctype reads its pack definition.
    #[test]
    fn all_schemas_routes_through_the_project_schema_shadow() {
        let cfg = TempDir::new("schema-shadow-cfg");
        let project_config = cfg.path();
        let schemas_dir = project_config.join("schemas");
        fs::create_dir_all(&schemas_dir).expect("mk schemas dir");
        // The project shadows the `note` doctype with a definition whose `location:`
        // differs from the pack's — the change `committed_store` would observe.
        fs::write(
            schemas_dir.join("note.yaml"),
            "type: note\nlocation: project-notes/\nsections: []\n",
        )
        .expect("write schema shadow");

        let pack = FixturePack::with(vec![
            (
                PackResourceKind::Schemas,
                "note",
                "type: note\nlocation: pack-notes/\nsections: []\n",
            ),
            // An unshadowed doctype — its pack definition must survive untouched.
            (
                PackResourceKind::Schemas,
                "memo",
                "type: memo\nlocation: pack-memos/\nsections: []\n",
            ),
        ]);

        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["note".to_owned(), "memo".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("note");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let schemas = CascadeDefs::new(&resolved, project_config)
            .all_schemas(&pack)
            .expect("schemas load");

        // The cascade carries no seeded `docs-root` scalar here (a bare PackDefaultLayer),
        // so `apply_docs_root` is a no-op (opt-in: it nests only where the cascade carries
        // the key) and each `location:` stays flat.
        assert_eq!(
            schemas.get("note").and_then(|s| s.location.as_deref()),
            Some("project-notes/"),
            "the project `schemas/note.yaml` shadow must win on the all_schemas read",
        );
        assert_eq!(
            schemas.get("memo").and_then(|s| s.location.as_deref()),
            Some("pack-memos/"),
            "an unshadowed doctype must keep its pack definition (the shadow is inert)",
        );

        // The pack-baseline (no shadow): `note` reads its pack `location:`, proving
        // the shadow is what changed the output.
        let pack_baseline = CascadeDefs::new(&no_shadow_resolved(), project_config)
            .all_schemas(&pack)
            .expect("pack-baseline schemas load");
        assert_eq!(
            pack_baseline
                .get("note")
                .and_then(|s| s.location.as_deref()),
            Some("pack-notes/"),
            "the no-shadow baseline must read the pack `note` definition",
        );
    }

    /// The located-fault precedent (the `CascadeStepSource` rule, extended to
    /// definition reads): a `file_owner` of `Project` whose on-disk
    /// `workflows/<id>.yaml` is **missing** is a located fault naming the absent
    /// file — never a silent fall-through to the pack definition (`overrides.md` →
    /// whole-file definition shadow; the owning layer is never silently abandoned).
    #[test]
    fn project_owned_but_missing_workflow_shadow_is_a_located_fault_not_a_pack_fallthrough() {
        let cfg = TempDir::new("wf-shadow-missing");
        let project_config = cfg.path();
        // The project owns `flow` (the cascade says so) but ships no file for it.
        let pack = FixturePack::with(vec![(
            PackResourceKind::Workflows,
            "flow",
            "---\nwhen: x\ncreates-task: false\n---\n{{ include: step:pack-body }}\n",
        )]);
        let base = engine::cascade::PackDefaultLayer::new(
            "dev",
            "0.0.0",
            BTreeMap::new(),
            vec!["flow".to_owned()],
        );
        let project = OverrideLayer::empty().shadow_file("flow");
        let resolved = engine::cascade::resolve(&base, None, Some(&project)).expect("resolves");

        let err = CascadeDefs::new(&resolved, project_config)
            .read_workflow(&pack, "flow")
            .expect_err("a project-owned but missing workflow shadow must fault, not fall through");
        let msg = err.to_string();
        assert!(
            msg.contains("flow") && msg.contains("workflows"),
            "the located fault must name the absent `flow` workflow shadow; got: {msg}",
        );
    }
}
