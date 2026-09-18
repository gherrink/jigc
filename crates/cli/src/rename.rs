//! `jigc rename <old-slug> --to "<New Title>"` — the CLI-owned identity refactor
//! (M35 Increment 1, T2: the verb + the committed-in-place atomic transaction).
//!
//! A top-level, **task-less self-committing** direct-store op (the shape of `unmanage` /
//! `migrate-corpus`, [write-commands.md](../../../design/write-commands.md) → `jigc rename`):
//! a doc-level identity change runs through **one** explicit, atomic op so cross-refs never
//! dangle under the move. The CLI derives the new slug = [`engine::slug::slugify`] (`--slug`
//! overrides) — so a renamed doc is byte-identical to a freshly-created one of that title;
//! walks the target's **inverse edges** ([`engine::index::referrers_of`]) and repoints every
//! persisted referrer's ref-field old→new ([`engine::write::repoint_ref`], scalar replaced
//! whole / list-valued re-emitting the canonical whole list); rewrites the moved doc's `# H1`
//! to the new title; and `git mv`s old→new.
//!
//! It commits as **one atomic transaction** with a real [`rollback_rename`]: pre-image
//! capture of the old path + every referrer (restored via `git restore --staged --worktree`)
//! plus the file-state record (inside the rollback inventory, [DECISIONS.md](../../../DECISIONS.md)
//! 2026-06-28, pin I3). The verb runs its **own** pre-commit integrity assertion
//! ([`rebuild_committed`](engine::index::rebuild_committed) →
//! [`ref_resolves_store`](engine::index::ref_resolves_store)), refusing the commit on any
//! dangling ref; the user's git hooks run, never `--no-verify`.
//!
//! The up-front validation gate runs **before** the transaction (T3): a **dirty working tree**
//! blocks (a rename commits in place with no pathspec, so any pre-existing tracked change would
//! be swept into the one atomic rename commit — commit or stash first); a **collision** with a
//! *different* committed doc blocks (an identity refactor never silently suffixes); a **no-op
//! reslug** (the new title slugs to the doc's own current slug) degrades to a **retitle-only**
//! (rewrite the H1 + commit, no `git mv`, no referrer repoint); a **mid-fan-out** guard
//! blocks whenever any task working area *or* milestone is in-flight ([DECISIONS.md] 2026-06-28,
//! pins I2/I4); a **placement-singleton** reslug rejects with the real rule (identity fixed to
//! the type; retitle-only) and a **milestone-record** reslug refuses always — between milestones
//! too (M40 A4); and a **`--slug` override that is not a slug** blocks before the destination
//! path is built, the sixth and last of the mint doors to ask that question (M50 Inc 2 / T2);
//! and a **destination git cannot record** — the doctype's home inside a submodule, an
//! embedded repo or git's own directory — is refused *here* rather than from inside the
//! transaction, so the arm carries an identity, a route and a log entry instead of the
//! shared move primitive's bare bail printing the host's filesystem (M50 Inc 2 / T3).
//! The advisory prose/unmanaged-mention report is T5.
//!
//! Every one of those refusals is declared and disposed in one place — [`RefusalKind`], which
//! carries the finding code each raises and whether its route is a command or a judgment
//! (M49 Increment 11 / T4, PT-A).

use anyhow::{Context, Result, anyhow, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use engine::file_state::{FileStateRecord, hash_bytes};
use engine::finding::{Finding, Location, Route, Severity};
use engine::index;
use engine::schema::Schema;
use engine::slug::slugify;

use crate::ingest::{load_schemas, require_project_layer};
use crate::pack::make_pack;
use crate::task::{
    WORK_UNIT_ID_GRAMMAR, git_capture, git_commit, git_head, git_run, git_status_entries,
};

/// **The refusal axis of `jigc rename`** — every state this door declines *before it has
/// mutated anything*, declared once so the sites cannot drift from the sweep that drives
/// them (`flow37_rename::every_rename_refusal_carries_an_identity_and_an_exit`).
///
/// **The defect (M49 Increment 11 / T4 — PT-A).** The occupancy arm answered
/// `` cannot rename to `adr:keeper` — a different doc already exists at … `` with **no
/// finding code and no route**, while *the same fault one argument away* — the
/// destination-occupancy guard of `jigc doc rename … --task <id>` — blocks
/// `write.already-present` and routes. Eight of this door's nine pre-transaction refusals
/// shared that shape, and nothing fenced them: an `anyhow::bail!` is outside M43's route
/// floor **by construction**, since [`engine::finding::is_route_exempt`] takes a *finding*
/// code and a bail has none.
///
/// **Why the `Finding` promotion, and not the anyhow-embedded span.** Two precedents
/// ship: `tests/anyhow_route_spans.rs`' checked [`engine::finding::Route::mechanical`]
/// span written *inside* an `anyhow` message (M43 Increment 1 / T7), and M49 Increment
/// 10 / T5's `finding_to_err` promotion at `jigc milestone provision`. The embedded span
/// buys a **route** and nothing else — and half of what a refusal owes here is an
/// **identity**: the code that reaches `finding_codes` in the invocation log, without
/// which a refused rename is indistinguishable there from the eight other ways this door
/// says no (exit 1, no findings, `error_code: null`). Only the promotion carries both, so
/// the promotion is what this axis uses.
///
/// **What is deliberately not a member.** The malformed-address reject ([`parse_addr`])
/// is a row of the *address-parse* fault axis, adjudicated over
/// [`engine::address::ParseError`]'s own six variants at M49 Increment 5 / T3 — which gave
/// every **head** fault the `<type>:<slug>` sentence and the `jigc describe` route and
/// left the family code-less. This door's local parser gives exactly that answer, so
/// minting a code for it **here alone** would fork one class across two doors. The
/// in-transaction failures are the other non-members, for the opposite reason: they are
/// the *transaction's*, not the gate's — each rolls the store back ([`rollback_rename`]),
/// and a commit-phase rejection already carries its own log identity
/// ([`crate::invocation_log::ERROR_RENAME_REJECTED`]) through the survivable frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalKind {
    /// The address names a doctype no resolved cascade carries.
    UnknownDoctype,
    /// The address names a **transient** doctype — neither `location:` nor `placement:`,
    /// so there is no committed file to move.
    TransientDoctype,
    /// The doctype resolves, but no managed doc answers to that identity.
    NoSuchDoc,
    /// `--to` carries a title with no slug-able content, and no `--slug` override.
    UnslugableTitle,
    /// The working tree carries a tracked change, staged or unstaged.
    DirtyTree,
    /// A task working area or a milestone is in flight.
    InFlight,
    /// A **singleton** doctype's reslug: its slug IS the type id.
    FixedIdentity,
    /// A `milestone-record`'s reslug: its slug IS the milestone work-unit id.
    WorkUnitIdentity,
    /// The destination identity is already answered by a different committed doc.
    OccupiedDestination,
    /// A `--slug` override that is not a well-formed slug — the value drives the new
    /// identity **verbatim**, so it is a path component, not a hint.
    MalformedSlug,
    /// The destination the new identity resolves to is a path **git cannot record** in
    /// this repository — inside another repository (a submodule or an embedded repo),
    /// inside git's own directory, or outside the repository root altogether.
    UntrackableDestination,
}

/// How a refusal's route repairs the state — the disposition every member owes, and the
/// axis suite's branch: a [`Repair::Command`] row's route names a `jigc` command the
/// sweep **runs verbatim**, a [`Repair::Judgment`] row states at the site why no single
/// command is the answer. There is no third disposition: the route floor admits no
/// route-less blocking finding outside `engine::finding::is_route_exempt`'s conformance
/// parse diagnostics, and none of these is one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repair {
    /// The route names a `jigc` command that moves the state on. The axis runs it.
    Command,
    /// The repair is a judgment, and the field says whose and why. A route still ships —
    /// it names the exits — but no argv is *the* answer, so the sweep does not run one.
    Judgment(&'static str),
}

impl RefusalKind {
    /// Every member — the axis the sweep iterates. A new variant joins here as well as
    /// [`RefusalKind::code`] and [`RefusalKind::repair`] (both exhaustive matches, so the
    /// compiler asks for two of the three and `every_refusal_kind_is_declared` for the
    /// third).
    pub const ALL: &'static [RefusalKind] = &[
        RefusalKind::UnknownDoctype,
        RefusalKind::TransientDoctype,
        RefusalKind::NoSuchDoc,
        RefusalKind::UnslugableTitle,
        RefusalKind::DirtyTree,
        RefusalKind::InFlight,
        RefusalKind::FixedIdentity,
        RefusalKind::WorkUnitIdentity,
        RefusalKind::OccupiedDestination,
        RefusalKind::MalformedSlug,
        RefusalKind::UntrackableDestination,
    ];

    /// The finding code this refusal carries — its identity on the printed surface **and**
    /// in the invocation log's `finding_codes`.
    ///
    /// Seven of the eleven reuse a **shipped** code rather than minting a door-private one:
    /// the fault is the same fault the read and in-task write paths already name, and one
    /// fault owes one code (the M49 Increment 11 / T1 rule, applied to refusals). In
    /// particular [`RefusalKind::FixedIdentity`] and [`RefusalKind::WorkUnitIdentity`]
    /// share `write.identity-change` with `jigc doc rename`'s own two identity
    /// refusals — the two verbs differ in *what identity they may move*, never in what
    /// kind of fault a fixed one is.
    ///
    /// Two of the four mints are the states no other door can be in: only this verb
    /// refuses over the **working tree** it is about to commit in place, and only this
    /// verb refuses over an in-flight **fan-out** whose join key it would change. The
    /// other two are the **family's** code rather than this door's, minted here because
    /// the siblings that share the fault refuse it with a bare `anyhow` carrying no code
    /// to reuse: [`RefusalKind::MalformedSlug`]'s `write.malformed-slug` (the five other
    /// `--slug` doors, `crate::cli::SLUG_DOORS`) and
    /// [`RefusalKind::UntrackableDestination`]'s `write.untrackable-destination` (the
    /// other three callers of [`crate::relocate::move_doc`] — `jigc relocate`,
    /// `migrate-corpus`'s relocation arm, `finalize`'s promote/retire). Both are the code
    /// those doors join when they are converged (M50 Increment 2, declared bounds vi and
    /// vii: *that* they refuse is in scope, *how they render* is not).
    pub fn code(self) -> &'static str {
        match self {
            RefusalKind::UnknownDoctype => "store.unknown-type",
            RefusalKind::TransientDoctype => "store.transient-type",
            RefusalKind::NoSuchDoc => "store.not-found",
            RefusalKind::UnslugableTitle => "write.unslugable-title",
            RefusalKind::DirtyTree => "rename.dirty-tree",
            RefusalKind::InFlight => "rename.in-flight",
            RefusalKind::FixedIdentity | RefusalKind::WorkUnitIdentity => "write.identity-change",
            RefusalKind::OccupiedDestination => "write.already-present",
            RefusalKind::MalformedSlug => "write.malformed-slug",
            RefusalKind::UntrackableDestination => "write.untrackable-destination",
        }
    }

    /// This refusal's disposition — see [`Repair`]. Each `Judgment` carries its reason
    /// here, at the declaration, so a reader asking *"why does this one not just tell me
    /// the command?"* is answered where the verdict is taken.
    pub fn repair(self) -> Repair {
        match self {
            // The doctype surface answers all three: it lists what exists, what persists,
            // and under which identity — which is exactly what each of these got wrong.
            RefusalKind::UnknownDoctype
            | RefusalKind::TransientDoctype
            | RefusalKind::NoSuchDoc => Repair::Command,
            // The retitle the fixed identity *does* support, argv-complete: the doc's own
            // slug is derivable (it is the type id / the work-unit id), so the command is
            // concrete rather than a placeholder the reader has to fill.
            RefusalKind::FixedIdentity | RefusalKind::WorkUnitIdentity => Repair::Command,
            RefusalKind::UnslugableTitle => Repair::Judgment(
                "the id is the author's to choose: a title with no word character cannot yield one, and a mechanical argv may carry only a declared placeholder (`engine::finding::ROUTE_PLACEHOLDERS`), which a slug is not",
            ),
            RefusalKind::DirtyTree => Repair::Judgment(
                "committing or stashing is git's move, not jigc's, and which of the two the tracked change deserves is the operator's call",
            ),
            RefusalKind::InFlight => Repair::Judgment(
                "finalize keeps the in-flight work and discard drops it — the route names both exits because choosing between them is not jigc's to do",
            ),
            RefusalKind::OccupiedDestination => Repair::Judgment(
                "a free slug is the agent's to pick, the same reason `jigc doc rename`'s own destination-occupancy guard routes `Human`",
            ),
            RefusalKind::MalformedSlug => Repair::Judgment(
                "the id is the caller's to name: jigc cannot compose the slug the caller meant, and a mechanical argv may carry only a declared placeholder (`engine::finding::ROUTE_PLACEHOLDERS`), which a slug is not",
            ),
            RefusalKind::UntrackableDestination => Repair::Judgment(
                "the destination is the doctype's home plus the new slug, and jigc cannot tell which of the two the operator meant to change — nor whether the home itself (a submodule, an embedded repo, git's own directory) is the thing to move",
            ),
        }
    }
}

/// **Refuse with an identity.** Flatten `kind`'s [`Finding`] — its declared
/// [`code`](RefusalKind::code), the located subject, the message and the route — into the
/// `anyhow` channel this verb returns on, through the shared
/// [`crate::render::finding_error`] carrier. The carrier is what makes the identity
/// survive the trip: the dispatch reads the finding back off the error and logs its code,
/// so the code the surface prints is the code `finding_codes` records.
///
/// `at` is the subject the refusal is *about* — the `<type>:<slug>` identity in every arm
/// that has resolved one, the bare doctype id in the two that refuse before an instance
/// exists ([`RefusalKind::UnknownDoctype`] / [`RefusalKind::TransientDoctype`], the
/// doctype-scoped locus rule `engine::store` follows).
fn refuse(kind: RefusalKind, at: &str, message: String, route: Route) -> anyhow::Error {
    crate::render::finding_error(&Finding::graded(
        Severity::Blocking,
        kind.code(),
        message,
        Some(Location::addressed(at, 1, 1)),
        Some(route),
    ))
}

/// The **retitle** route the two fixed-identity refusals carry: this exact doc, the title
/// the caller asked for, and an explicit `--slug` pinning the identity that may not move.
///
/// It is [`Repair::Command`] and not a judgment because both halves are *derivable* — the
/// slug is the type id (a placement singleton) or the milestone work-unit id, and the
/// title is the caller's own `--to` — so the command is concrete rather than a shape the
/// reader has to fill in. Run verbatim it lands the retitle-only arm this door does
/// support, which is the whole of what the refusal is withholding.
fn retitle_route(old_id: &str, title: &str, fixed_slug: &str) -> Route {
    Route::mechanical(
        [
            "jigc",
            "rename",
            old_id,
            "--to",
            &crate::task::shell_token(title),
            "--slug",
            fixed_slug,
        ],
        " keeps the identity that cannot move and rewrites only the title",
    )
}

/// The outcome of a rename: the old/new `<type>:<slug>` identities, the repo-relative
/// paths the move spanned, the new title, and the repointed referrers (each
/// `<from>#<relation>`, sorted).
#[derive(Clone, Debug, serde::Serialize)]
pub struct RenameReport {
    /// The old `<type>:<slug>` identity.
    pub from: String,
    /// The new `<type>:<slug>` identity.
    pub to: String,
    /// The repo-relative old path (before the move).
    pub old_path: String,
    /// The repo-relative new path (after the move).
    pub new_path: String,
    /// The new H1 title the moved doc carries.
    pub title: String,
    /// The persisted referrers repointed, each `<from>#<relation>`, sorted.
    pub referrers: Vec<String>,
    /// **Advisory** old-slug occurrences in managed-doc prose + unmanaged tracked files
    /// (each `<repo-rel-path>:<line>`, sorted) that the CLI **reports but never rewrites** —
    /// the determinism boundary's honest line (it rewrites only structured ref-fields and
    /// authors no prose). A word-boundary/token match, scoped to `git ls-files`. Never
    /// changes the verb's exit status; empty for a retitle-only (the slug is unchanged).
    pub prose_mentions: Vec<String>,
    /// The short sha of the atomic rename commit, or `None` when **nothing was committed** —
    /// the machine discriminator of the **idempotent** no-op: a `--to` title that slugs to the
    /// doc's own current slug *and* matches the H1 it already carries changes nothing, so the
    /// staged set is empty and no commit is made (never an empty one, and never git's refusal
    /// of one dressed as a hook rejection — M48 Increment 8;
    /// `design/write-commands.md` → `jigc rename` step 2, the degenerate arm). The
    /// `migrate-corpus` precedent (`CorpusMigrationReport::commit`).
    pub commit: Option<String>,
    /// The atomic rename commit's captured non-blocking hook stream — **present-always**,
    /// the empty string when no hook spoke **or nothing was committed** (the hook_output
    /// producer axis; `design/command-output-contract.md` → Stream discipline). The caller
    /// relays the same string on the other channel ([`crate::task::relay_hook_output`]).
    pub hook_output: String,
}

/// Run `jigc rename <old_addr> --to <title>` (optional `--slug`) against `cwd`: load the
/// rename substrate (schemas, committed index, file-state), re-derive the new identity,
/// repoint every referrer + rewrite the moved doc's H1, `git mv`, and commit as one
/// atomic transaction with a real rollback on any pre-commit failure.
///
/// `conflicts` is the out-param the rollback's compare-and-swap fills on a failed
/// transaction: one blocking `rename.rollback-conflict` per path whose bytes are no longer
/// jigc's (M52 Increment 5 / T6). It stays empty on every success and on every pre-transaction
/// refusal, so the caller can carry it unconditionally on its `Err` arm — the same shape the
/// milestone record-only doors take one file over.
pub(crate) fn run(
    cwd: &Path,
    old_addr: &str,
    title: &str,
    slug_override: Option<&str>,
    conflicts: &mut Vec<Finding>,
) -> Result<RenameReport> {
    let repo_root = require_project_layer(cwd)?;
    let pack = make_pack()?;
    let schemas = load_schemas(pack.as_ref(), &repo_root.join(".jigc/config"))?;
    let schema_map: BTreeMap<String, Schema> =
        schemas.iter().map(|s| (s.ty.clone(), s.clone())).collect();

    let jigc_root = repo_root.join(".jigc");
    let head = git_head(&repo_root)?;
    let index = index::load_committed(&repo_root, &jigc_root, &schema_map, &head);

    // Resolve the target's identity + on-disk path.
    let (ty, old_slug) = parse_addr(old_addr)?;
    let old_rel = doc_path(&schema_map, &ty, &old_slug)?;
    let old_abs = repo_root.join(&old_rel);
    let old_id = format!("{ty}:{old_slug}");
    if !old_abs.is_file() {
        return Err(refuse(
            RefusalKind::NoSuchDoc,
            &old_id,
            format!("no managed doc `{old_id}` to rename (expected at {old_rel})"),
            Route::mechanical(
                ["jigc", "describe"],
                " lists the doctype surface — check the id you typed against it",
            ),
        ));
    }
    let old_source = std::fs::read_to_string(&old_abs)
        .with_context(|| format!("could not read the doc to rename at {old_rel}"))?;

    // A `--slug` override drives the new identity **verbatim**, and a doc's slug IS its
    // path component (`<docs-root>/<location>/<slug>.md`) — so a value that is not a slug
    // names a file the store cannot address. Asked **before** the destination path is
    // built and before the empty-slug arm below, because both of those answer about the
    // *title*: driven at `b32def1`, `--slug '../../src/pwned'` exited 0 and committed
    // `docs/decisions/keeper.md => src/pwned.md` (after which no `doc list` row, no
    // `doc show` and no `validate` finding could name the doc), and `--slug ''` answered
    // `write.unslugable-title` — "`--to "New Title"` slugs to nothing" — about a title
    // that slugs fine, routing the caller to repeat what had just failed.
    //
    // This is the **sixth and last** `--slug` door to ask the question its five siblings
    // have asked since M39 (`crate::start`'s mint, `crate::migrate`'s adoption, and
    // `crate::doc::reject_malformed_slug`'s three); the door set is derived from the clap
    // tree at [`crate::cli::SLUG_DOORS`] and driven whole by
    // `crates/cli/tests/slug_override_axis.rs`. The grammar sentence is the one shipped
    // literal, reused rather than respelled (`design/surface-contract.md` → law 1).
    if let Some(slug) = slug_override
        && !engine::slug::is_slug(slug)
    {
        return Err(refuse(
            RefusalKind::MalformedSlug,
            &old_id,
            format!("`--slug {slug:?}` is not a valid slug — {WORK_UNIT_ID_GRAMMAR}"),
            Route::human(
                "the id a rename mints is taken verbatim, so name one that fits the \
                 grammar, or drop the override and let the title mint it",
            ),
        ));
    }
    // …and the name-ceiling half, from the shared home the other five doors read (M51 Inc 9
    // / T3, EC-28). Driven at `d7ebbeb9` the `doc rename` sibling answered a 300-byte
    // override with the bare OS error — no code, no route, no `at:`; this door's own
    // destination build would reach `git mv` with a name the filesystem cannot take. It is
    // not a [`RefusalKind`] member for [`crate::task::reject_malformed_slug_head`]'s reason
    // one task over: the code belongs to the `--slug` **family**, not to this door, and
    // minting a member here would fork the family across one of its six doors.
    if let Some(slug) = slug_override {
        crate::task::reject_slug_over_name_ceiling(slug)?;
    }
    let new_slug = match slug_override {
        Some(s) => s.to_string(),
        None => slugify(title),
    };
    if new_slug.is_empty() {
        return Err(refuse(
            RefusalKind::UnslugableTitle,
            &old_id,
            format!(
                "`--to {title:?}` slugs to nothing — a rename derives the new id from the \
                 title, and this one carries no slug-able content"
            ),
            Route::human(format!(
                "re-run with a title carrying at least one word character, or name the id \
                 yourself: `jigc rename {old_id} --to <a title> --slug <new-slug>`"
            )),
        ));
    }
    let new_rel = doc_path(&schema_map, &ty, &new_slug)?;
    let new_abs = repo_root.join(&new_rel);
    let new_id = format!("{ty}:{new_slug}");

    // The up-front validation gate (runs before any mutation):
    // (a) **clean-tree precondition** — a rename is a deliberate standalone op that commits
    //     in place with **no pathspec** (`git_commit` runs `git commit -F`, capturing the
    //     whole index), so any pre-existing tracked change would be swept into the "one
    //     atomic rename commit" (a staged change) or clobbered by the referrer rewrites (an
    //     unstaged edit to a managed doc). Refuse up front whenever the working tree carries
    //     any staged or unstaged **tracked** change, routing the user to commit or stash
    //     first. Untracked files are not part of the commit (the verb stages only the moved
    //     doc + its referrers, never `git add --all`), so they do not count — they are
    //     filtered out (`??`). (write-commands.md → `jigc rename` step 2, "tree clean".)
    let dirty: Vec<String> = git_status_entries(&repo_root)?
        .into_iter()
        .filter(|(code, _)| code != "??")
        .map(|(_, path)| path)
        .collect();
    if !dirty.is_empty() {
        return Err(refuse(
            RefusalKind::DirtyTree,
            &old_id,
            format!(
                "cannot rename with a dirty working tree — commit or stash your changes \
                 first (a rename is a deliberate standalone op that commits in place): {}",
                dirty.join(", ")
            ),
            Route::human(
                "commit those tracked changes, or `git stash` them, then re-run the rename \
                 — a rename commits in place with no pathspec, so anything already in the \
                 index would ride its commit",
            ),
        ));
    }
    // (b) **mid-fan-out guard** — a rename changes the by-task-id join's same-doc-clash key
    //     and a task working area may hold an old-slug copy that would promote stale; block
    //     whenever any task working area *or* milestone is in-flight (the coarse guard, I4).
    if let Some(marker) = mid_fan_out_marker(&jigc_root) {
        let (unit, id) = (marker.unit, marker.id.as_str());
        return Err(refuse(
            RefusalKind::InFlight,
            &old_id,
            format!(
                "cannot rename while {unit} `{id}` is in flight — finalize or discard it \
                 first (a rename changes the by-task-id join key)"
            ),
            Route::human(format!(
                "settle it first — `jigc {unit} finalize {id}` if that work is done, \
                 `jigc {unit} discard {id}` if it is not; a rename is task-less and \
                 self-committing, so it runs once neither is open"
            )),
        ));
    }
    // (c) **no-op reslug** — when the new slug equals the doc's own current slug the identity
    //     is unchanged, so the rename degrades to a retitle-only (rewrite H1 + commit, no
    //     `git mv`, no referrer repoint — nothing dangles).
    let is_retitle = new_slug == old_slug;
    // (d) **placement-singleton reslug** — undefined, not merely blocked (M40 A4;
    //     write-commands.md → Placement singletons): a placement doctype's identity is fixed
    //     to its type — the singleton's slug IS the type id and the doc lives at its literal
    //     `placement.file` — so there is no reslug to perform. Reject with the real rule
    //     rather than falling through to (f)'s misleading collision text (`doc_path` resolves
    //     the placement literal ignoring the slug, so `new_abs == old_abs`: it is the SAME
    //     file, not a collision). Retitle-only (the degenerate arm (c)) stays supported.
    if !is_retitle && schema_map[ty.as_str()].placement.is_some() {
        return Err(refuse(
            RefusalKind::FixedIdentity,
            &old_id,
            format!(
                "cannot reslug `{old_id}` — a placement singleton's identity is fixed to \
                 its type (the slug IS the type id `{ty}` and the doc lives at the literal \
                 {old_rel}); only a retitle is supported"
            ),
            retitle_route(&old_id, title, &old_slug),
        ));
    }
    // (e) **milestone-record reslug** — refused always, between milestones too (M40 A4.4;
    //     write-commands.md → Milestone-record reslug): the record's slug IS the milestone
    //     work-unit id — it keys `.jigc/milestones/<id>` and every milestone op — so a reslug
    //     would sever the committed record from its work unit. Keyed on the (necessarily
    //     committed) rename target's doctype, fresh-clone survivable — no workbench read; the
    //     coarse mid-fan-out guard (b) covers only the in-flight window.
    if !is_retitle && ty == crate::milestone::MILESTONE_RECORD_TYPE {
        return Err(refuse(
            RefusalKind::WorkUnitIdentity,
            &old_id,
            format!(
                "cannot reslug `{old_id}` — a milestone-record's slug IS its milestone \
                 work-unit id (it keys `.jigc/milestones/{old_slug}` and every milestone \
                 op), so a reslug would sever the committed record from its work unit; \
                 only a retitle is supported"
            ),
            retitle_route(&old_id, title, &old_slug),
        ));
    }
    // (f) **collision** — a non-degenerate new slug must be free; a collision with a
    //     *different* committed doc blocks (an identity refactor, never the join's suffix).
    if !is_retitle && new_abs.is_file() {
        return Err(refuse(
            RefusalKind::OccupiedDestination,
            &new_id,
            format!("cannot rename to `{new_id}` — a different doc already exists at {new_rel}"),
            Route::human(format!(
                "give this doc an id nothing else answers to — re-run `jigc rename \
                 {old_id} --to {} --slug <other-slug>`; or, if `{new_id}` is the doc you \
                 meant to work on, read it with `jigc doc show {new_id}` and rename that \
                 one instead",
                crate::task::shell_token(title),
            )),
        ));
    }

    // (g) **unmovable destination** — the destination must be a path git can *record*.
    //     `git mv <src> <dst>` into a path it cannot (inside another repository, inside
    //     git's own directory, outside the root) prints `error: invalid path`, moves the
    //     file on disk, drops the source from the index, adds nothing — and **exits 0**
    //     (M49's completion-triage HIGH; `crate::trackable::untrackable_reason` is the
    //     predicate that replaced reading that exit code as a verdict).
    //
    //     The shared move primitive already refuses it, and keeps doing so for its other
    //     callers — but it refuses from *inside* the transaction and as a bare `anyhow`:
    //     no code for the invocation log, no route, and (the predicate composes three of
    //     its five reasons from absolute paths) the host's filesystem on the surface,
    //     which is neither repo-real nor a typed identity
    //     (`design/surface-contract.md` → law 1). Asked **here**, at this door's own
    //     pre-check, the arm carries an identity, a route and a log entry like its ten
    //     siblings; the message is composed from repo-relative parts only, so no host
    //     path can reach either channel by construction rather than by wording.
    //
    //     Scoped exactly like the primitive's own check — to a run that actually moves. A
    //     retitle-only rewrites the doc at the path it already occupies, and a path the
    //     store is already reading from is not this gate's to relitigate.
    if !is_retitle && crate::trackable::untrackable_reason(&repo_root, &new_rel).is_some() {
        return Err(refuse(
            RefusalKind::UntrackableDestination,
            &new_id,
            format!(
                "cannot rename `{old_id}` to `{new_id}` — git cannot record `{new_rel}` in \
                 this repository, so the move would take the doc off disk and leave it \
                 surviving only in history"
            ),
            Route::human(format!(
                "`{new_rel}` is `{ty}`'s home plus the new slug: check whether that home is \
                 inside another repository (a submodule or an embedded repo), inside git's \
                 own directory, or outside the repository root — then rename to a slug this \
                 repository can record, or move the home; `jigc config list` shows the roots \
                 in force and the layer each wins from"
            )),
        ));
    }

    // Compute every referrer's old→new rewrite (grouped per source doc, so a doc that
    // references the target through more than one relation rewrites once over a running
    // buffer). Sorted by `referrers_of`'s deterministic `(from, relation, to)` order. A
    // retitle-only has no identity change, so it repoints nothing.
    let mut referrer_writes: Vec<ReferrerWrite> = Vec::new();
    let mut referrer_labels: Vec<String> = Vec::new();
    if !is_retitle {
        let mut by_from: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for edge in index::referrers_of(&index, &old_id) {
            by_from
                .entry(edge.from.clone())
                .or_default()
                .push(edge.relation.clone());
        }
        for (from_id, relations) in &by_from {
            let (fty, fslug) = parse_addr(from_id)?;
            let fschema = schema_map
                .get(&fty)
                .ok_or_else(|| anyhow!("referrer `{from_id}` has an unknown doctype `{fty}`"))?;
            let frel = doc_path(&schema_map, &fty, &fslug)?;
            let fabs = repo_root.join(&frel);
            let mut source = std::fs::read_to_string(&fabs)
                .with_context(|| format!("could not read the referrer at {frel}"))?;
            for relation in relations {
                source = engine::write::repoint_ref(fschema, &source, relation, &old_id, &new_id)
                    .map_err(|e| anyhow!("could not repoint {from_id}#{relation}: {e:?}"))?;
                referrer_labels.push(format!("{from_id}#{relation}"));
            }
            referrer_writes.push(ReferrerWrite {
                rel: frel,
                abs: fabs,
                source,
            });
        }
    }

    // The moved doc's new bytes — only the `# H1` line changes (the unified retitle).
    let new_source =
        rewrite_h1(&old_source, title).ok_or_else(|| anyhow!("the doc at {old_rel} has no H1"))?;

    // The tracked paths the transaction restores from HEAD on rollback (the old path +
    // every referrer — all committed at HEAD). `ROLLBACK_POPULATIONS`' `rename-head-restore`
    // row: the door refuses over a dirty tree, so HEAD is what the worktree held and the
    // restore has no third-party bytes to take.
    let mut tracked_restore: Vec<String> = vec![old_rel.clone()];
    tracked_restore.extend(referrer_writes.iter().map(|w| w.rel.clone()));

    // …and the two paths HEAD does **not** answer for — `ROLLBACK_POPULATIONS`'
    // `rename-worktree` row, the `FileCas` half of this door (M52 Increment 5 / T6). Both were
    // restored unconditionally through rc.15, and the user's `pre-commit` hook runs *inside*
    // the interval between jigc's write and that restore, so both took a third party's bytes
    // there. They carry the same entry every `FileCas` population carries — the pre-image, and
    // what jigc left at the path, read back one statement after each write — so the rollback
    // asks the family's one question instead of assuming the answer.
    let fs_path = FileStateRecord::path_in(&jigc_root);
    let fs_spec = crate::render::repo_relative(&repo_root, &fs_path);
    let mut worktree = crate::rollback::PreImageFamily::empty(crate::rollback::RENAME_DOOR);
    // The gitignored file-state cache: git tracks it nowhere, so `git restore` cannot recover
    // it and it rides the rollback by bytes (pin I3).
    worktree.push(
        crate::rollback::PreImage::capture(fs_spec.clone(), fs_path.clone())
            .context("could not read the file-state record")?,
    );
    // The landing path — but only while it is a path the HEAD-sourced arm does not already
    // own. On a **retitle-only** the landing path *is* the doc's own committed path, which is
    // `tracked_restore`'s first member; registering it here too would set two rows restoring
    // one file against each other, and the second would read the first's work as a racer's.
    // The two rows partition the paths, which is why they are two rows.
    if !tracked_restore.contains(&new_rel) {
        worktree.push(
            crate::rollback::PreImage::capture(new_rel.clone(), new_abs.clone())
                .with_context(|| format!("could not read the rename's destination at {new_rel}"))?,
        );
    }

    // The transaction proper: any failure rolls the store back byte-and-record identical.
    // A landed commit yields its short sha + captured non-blocking hook stream for the
    // report; an idempotent run (nothing staged) yields `None` and commits nothing.
    let (commit, hook_output) = match apply_and_commit(
        &repo_root,
        &jigc_root,
        &schema_map,
        &head,
        &old_rel,
        &new_rel,
        &new_source,
        &referrer_writes,
        &mut worktree,
        &fs_spec,
    ) {
        Ok(landed) => landed,
        Err(err) => {
            // The restore may refuse a path whose bytes are no longer jigc's; those findings
            // travel out to the dispatch arm, which prints them beside this door's own frame
            // and folds them into the log — never in place of the door's own error.
            conflicts.extend(rollback_rename(
                &repo_root,
                &jigc_root,
                &tracked_restore,
                &new_rel,
                &worktree,
            ));
            // The rollback has run, so the door's clause ("the rename was rolled back") is
            // true of what is now on disk — mark the failure so the surface frames it rather
            // than dropping the frame (N20; a stale `.git/index.lock` meeting the move's
            // `git mv` is this cell's ordinary cause). A hook rejection passes through
            // unmarked and keeps its own verbatim frame.
            return Err(crate::task::mark_commit_failure(err));
        }
    };

    // Post-commit: invalidate the persisted index so the next read rebuilds against the
    // new HEAD (the finalize post-commit step; best-effort — the stale stamp self-heals).
    let _ = index::invalidate(&jigc_root);

    // The advisory prose/unmanaged-mention report (the determinism boundary's honest line):
    // scan the *post-rename* tracked worktree for surviving old-slug occurrences — the CLI
    // has already rewritten every structured ref-field, so what remains is exactly the prose
    // + unmanaged mentions it cannot author. A retitle-only leaves the slug unchanged, so it
    // has nothing to report. Best-effort + advisory: it never blocks and never changes the
    // exit status.
    let prose_mentions = if is_retitle {
        Vec::new()
    } else {
        scan_prose_mentions(&repo_root, &old_slug)
    };

    referrer_labels.sort();
    Ok(RenameReport {
        from: old_id,
        to: new_id,
        old_path: old_rel,
        new_path: new_rel,
        title: title.to_string(),
        referrers: referrer_labels,
        prose_mentions,
        commit,
        hook_output,
    })
}

/// A referrer's computed rewrite: its repo-relative + absolute path and the new bytes.
struct ReferrerWrite {
    rel: String,
    abs: PathBuf,
    source: String,
}

/// Apply the rename mutations on disk, run the pre-commit integrity assertion, re-baseline
/// file-state, and commit — the inside of the transaction. Returns the landed commit's short
/// sha (`None` when the run staged nothing — the idempotent no-op, step 6) plus its captured
/// non-blocking hook stream (the hook_output producer axis — dropping it here was this
/// producer's defect), and `Err` on any failure (a `git mv` error, a dangling-ref integrity
/// violation, or a hook/commit rejection); the caller rolls back on `Err`.
///
/// `worktree` is the caller's captured `FileCas` family and `fs_spec` its file-state entry's
/// identity. Each is told *what jigc left* immediately after every write that reaches its
/// path — `PreImageFamily::wrote` re-reads the file, so a later call simply refreshes the
/// image and an arm that never reached its write leaves the entry `PostWrite::Untouched`,
/// which the rollback treats as *never written* and leaves alone. That is why the calls sit
/// at each write rather than once at the end: a failure between the move and the H1 rewrite
/// would otherwise leave the rollback unable to prove what it put there.
#[allow(clippy::too_many_arguments)]
fn apply_and_commit(
    repo_root: &Path,
    jigc_root: &Path,
    schema_map: &BTreeMap<String, Schema>,
    head: &str,
    old_rel: &str,
    new_rel: &str,
    new_source: &str,
    referrer_writes: &[ReferrerWrite],
    worktree: &mut crate::rollback::PreImageFamily,
    fs_spec: &str,
) -> Result<(Option<String>, String)> {
    // 0. Capture the **pre-rename** dangling-edge set off the committed store *before* any
    // mutation (disk == HEAD here). The integrity gate (step 4) refuses only on a dangle the
    // rename itself *introduces* — a pre-existing dangle unrelated to the move must pass
    // through untouched (the M18/M19 masking-trap guard: a transaction is never blocked by
    // drift it did not cause; pre-existing rot stays a report-only `jigc validate` concern).
    let before = index::rebuild_committed(repo_root, schema_map, head);
    let before_dangling: BTreeSet<(String, String, String)> =
        index::dangling_edges(&before, repo_root, schema_map)
            .into_iter()
            .map(edge_key)
            .collect();

    // 1. Write each referrer's repointed bytes.
    for write in referrer_writes {
        std::fs::write(&write.abs, &write.source)
            .with_context(|| format!("could not write the repointed referrer at {}", write.rel))?;
    }
    // 2. Move the doc via the shared move primitive — `git mv` old→new (skipped for a
    // retitle-only, where old_rel == new_rel and there is no identity change) plus the
    // moved-doc file-state re-key (forget old, record new at the retitled bytes' hash) —
    // then rewrite its H1 at the (possibly unchanged) path. The primitive re-keys only the
    // moved doc; the referrers' re-keys are layered on in step 5 (the two record saves
    // compose byte-identically).
    crate::relocate::move_doc(
        repo_root,
        jigc_root,
        old_rel,
        new_rel,
        &hash_bytes(new_source.as_bytes()),
    )?;
    // The move wrote both of this family's paths — the landing path and the file-state
    // record's re-key — so both learn what jigc left there before anything can fail below.
    worktree.wrote(new_rel);
    worktree.wrote(fs_spec);
    std::fs::write(repo_root.join(new_rel), new_source)
        .with_context(|| format!("could not write the retitled doc at {new_rel}"))?;
    worktree.wrote(new_rel);
    // 3. Stage the content changes (the move is staged; the H1 + referrer edits are not).
    git_run(repo_root, &["add", "--", new_rel])?;
    for write in referrer_writes {
        git_run(repo_root, &["add", "--", &write.rel])?;
    }

    // 4. The verb's own pre-commit integrity assertion: rebuild the committed index off the
    // mutated tree and refuse the commit only on a dangle the rename *introduced* — an edge
    // dangling *after* the move that was *not* dangling *before* it. Scoped by the before/after
    // diff (step 0), so a pre-existing dangle unrelated to the rename never blocks the move
    // and is never misattributed to it (the M18/M19 masking-trap guard). A resolvable rename
    // repoints every referrer in lockstep, so it introduces zero new dangles.
    let rebuilt = index::rebuild_committed(repo_root, schema_map, head);
    let after_dangling = index::dangling_edges(&rebuilt, repo_root, schema_map);
    let introduced = introduced_dangles(&before_dangling, after_dangling);
    if !introduced.is_empty() {
        bail!(
            "rename would introduce {} dangling cross-reference(s) — refusing to commit: {}",
            introduced.len(),
            introduced.join(", "),
        );
    }

    // 5. Re-baseline the referrers' file-state inside the boundary (the moved doc's own
    // re-key already happened in the move primitive, step 2), so the renamed store is
    // in-sync on the next sweep.
    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("could not load the file-state record under {jigc_root:?}"))?;
    for write in referrer_writes {
        record.record(write.rel.clone(), hash_bytes(write.source.as_bytes()));
    }
    record
        .save(jigc_root)
        .with_context(|| format!("could not save the file-state record under {jigc_root:?}"))?;
    worktree.wrote(fs_spec);

    // 6a. The **idempotent** run stops here (M48 Increment 8): a `--to` title that slugs to
    // the doc's own current slug AND matches the H1 the doc already carries rewrites nothing,
    // moves nothing and repoints nothing, so this transaction's own paths stage no change.
    // git refuses a commit that would record nothing, and that refusal reaches the commit seam
    // as a non-zero exit indistinguishable from a hook's — which is how this door came to
    // assert `git commit` was rejected over a run nobody rejected, route to a re-run that can
    // only fail identically, and relay git's unrelated *untracked-file* listing as the cause
    // (`completions/artifacts/RC-pre-1.0/v1-walk.md` → Arm 6). So the emptiness is
    // discriminated **before** the commit, at the shared axis predicate, and the run acks its
    // no-op instead (`commit: None`). The pathspec is exactly this transaction's paths — the
    // move's two ends plus every repointed referrer — the `migrate-corpus` / `setup` skip
    // shape; the up-front clean-tree gate means nothing else could be staged anyway.
    let mut pathspec: Vec<&str> = vec![old_rel, new_rel];
    pathspec.extend(referrer_writes.iter().map(|write| write.rel.as_str()));
    if crate::task::nothing_staged(repo_root, &pathspec) {
        return Ok((None, String::new()));
    }

    // 6b. Commit (the user's hooks run, never `--no-verify`). A rejected hook bails here and
    // the caller rolls back.
    let msg_path = std::env::temp_dir().join(format!("jigc-rename-msg-{}", std::process::id()));
    std::fs::write(
        &msg_path,
        format!("rename {old_rel} -> {new_rel}\n\nRepoint every persisted referrer in lockstep.\n"),
    )
    .context("could not write the rename commit message")?;
    let commit = git_commit(&crate::repo::SeamSubject::live(repo_root), &msg_path);
    let _ = std::fs::remove_file(&msg_path);
    // The landed commit's captured non-blocking hook stream, threaded to the report
    // (the hook_output producer axis — this was the site that discarded it).
    let hook_output = commit?;
    // The landed commit's short sha — the report's `Some`/`None` discriminator. Read only
    // after the commit succeeded (so git is present and HEAD exists) and deliberately **not**
    // propagated as an error: the caller rolls back on `Err`, and rolling back a commit git
    // has already recorded would half-revert the store. An unreadable sha names the landed
    // commit as `HEAD` — never `None`, which is reserved for "nothing was committed".
    let sha = git_capture(repo_root, &["rev-parse", "--short", "HEAD"])
        .unwrap_or_else(|_| "HEAD".to_owned());
    Ok((Some(sha), hook_output))
}

/// Roll the store back to its pre-rename state on a pre-commit failure: restore every
/// tracked path (the old doc + referrers) from HEAD, unstage the move's landing, and put the
/// two worktree paths HEAD cannot answer for back **compare-and-swap**. Best-effort on the
/// restore itself — a failure is swallowed (the commit did not land, and the door's own error
/// is what the operator has to act on), mirroring `rollback_promotions`.
///
/// Returns one blocking `rename.rollback-conflict` per path whose bytes are no longer jigc's;
/// the caller carries them beside its own frame ([`crate::task::carry_rollback_conflicts`]).
///
/// **The two populations, and why they are two** (`cli::rollback::ROLLBACK_POPULATIONS`):
///
///   * `rename-head-restore` — the old doc + every referrer, restored **from HEAD**. Its
///     discipline is `DoorGuard("rename.dirty-tree")`: the door refuses to run at all over a
///     dirty tree, so HEAD is what the worktree held and there are no third-party bytes for
///     this arm to take.
///   * `rename-worktree` — the landing path and the gitignored
///     `.jigc/state/file-state.json`, neither of which HEAD answers for. Its discipline is
///     `FileCas`, because the user's `pre-commit` hook runs *inside* the interval between
///     jigc's write and this restore. Through rc.15 both arms here were unconditional, and
///     both took a third party's bytes: the landing path's `remove_file` **deleted** a file a
///     racing hook had written, at exit 1, named by nothing
///     (`completions/artifacts/M52/baseline-rollback.md` §2.3b), and the file-state arm
///     rewrote or removed the record however it had changed — including removing one this
///     transaction had never reached.
fn rollback_rename(
    repo_root: &Path,
    jigc_root: &Path,
    tracked_restore: &[String],
    new_rel: &str,
    worktree: &crate::rollback::PreImageFamily,
) -> Vec<Finding> {
    // The old path + every referrer are committed at HEAD — `git restore --staged
    // --worktree` brings their index + worktree bytes back to HEAD (= pre-rename).
    for rel in tracked_restore {
        let _ = git_run(repo_root, &["restore", "--staged", "--worktree", rel]);
    }
    // The move's landing: unstage it. An **index** arm, deliberately without `--worktree` —
    // the bytes there are the swap's subject below, and dropping them here would re-enact the
    // unconditional removal one statement earlier.
    let _ = git_run(repo_root, &["restore", "--staged", new_rel]);
    // The two worktree paths HEAD cannot answer for, each restored only while it still holds
    // the bytes jigc wrote. A landing path jigc created is removed under that same condition;
    // when it does not hold them, nothing is overwritten and nothing is discarded.
    worktree.restore(repo_root, jigc_root)
}

/// An in-flight fan-out marker: the **work-unit kind** (`task` / `milestone`) and its id.
///
/// `unit` is spelled as the `jigc` verb family that settles that kind, because the
/// refusal's route names both of its exits (`jigc <unit> finalize <id>` /
/// `jigc <unit> discard <id>`) — so the two spellings cannot disagree.
struct FanOutMarker {
    /// `"task"` or `"milestone"` — the verb family, and the noun the message uses.
    unit: &'static str,
    /// The work unit's id.
    id: String,
}

/// The first in-flight fan-out marker, if any — a task working area
/// (`<jigc>/tasks/<id>/`) or a milestone (`<jigc>/milestones/<id>/`). A rename mid-fan-out
/// would change the by-task-id join's same-doc-clash key and a working area may hold an
/// old-slug copy that would promote stale, so the verb refuses the identity op while either
/// is live (the coarse guard, [DECISIONS.md] 2026-06-28 pin I4). Both enumerations are
/// sorted (the first id is deterministic), so the message and its route are stable.
fn mid_fan_out_marker(jigc_root: &Path) -> Option<FanOutMarker> {
    if let Some(id) = engine::state::list_active_task_ids(jigc_root).first() {
        return Some(FanOutMarker {
            unit: "task",
            id: id.clone(),
        });
    }
    first_dir_name(&jigc_root.join("milestones")).map(|id| FanOutMarker {
        unit: "milestone",
        id,
    })
}

/// The lexicographically-first sub-directory name under `dir`, or `None` when `dir` is
/// absent or holds no sub-directory (a missing dir is not an error — no fan-out has run).
fn first_dir_name(dir: &Path) -> Option<String> {
    let mut names: Vec<String> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_dir())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names.into_iter().next()
}

/// Scan every **tracked** worktree file for word-boundary occurrences of `needle` (the old
/// slug), returning a sorted `<repo-rel-path>:<line>` label per hit. This is the determinism
/// boundary's **advisory** report: the CLI has already rewritten every structured ref-field,
/// so the surviving occurrences are exactly the managed-doc *prose* and *unmanaged* mentions
/// it cannot author — reported, never rewritten. Scoped to `git ls-files` (tracked,
/// `.gitignore`-respecting). Best-effort: an unreadable/binary file (or a `git` failure) is
/// skipped, never an error — the report must never change the verb's exit status.
fn scan_prose_mentions(repo_root: &Path, needle: &str) -> Vec<String> {
    let Ok(listing) = git_capture(repo_root, &["ls-files"]) else {
        return Vec::new();
    };
    let mut hits: Vec<String> = Vec::new();
    for rel in listing.lines().filter(|l| !l.is_empty()) {
        let Ok(content) = std::fs::read_to_string(repo_root.join(rel)) else {
            continue;
        };
        for (i, line) in content.lines().enumerate() {
            if line_has_token(line, needle) {
                hits.push(format!("{rel}:{}", i + 1));
            }
        }
    }
    hits.sort();
    hits
}

/// True iff `needle` occurs in `line` as a standalone slug **token** — bounded on both sides
/// by a non-token byte (or the line edge). A token byte is ASCII-alphanumeric, `-`, or `_`
/// (the slug alphabet plus the separators that would extend it), so the old slug `cache`
/// matches `the cache layer` but never `caches` — the word-boundary discrimination the
/// advisory report rests on.
fn line_has_token(line: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let bytes = line.as_bytes();
    let n = needle.len();
    let mut start = 0;
    while let Some(off) = line[start..].find(needle) {
        let i = start + off;
        let before_ok = i == 0 || !is_token_byte(bytes[i - 1]);
        let end = i + n;
        let after_ok = end >= bytes.len() || !is_token_byte(bytes[end]);
        if before_ok && after_ok {
            return true;
        }
        start = i + 1;
    }
    false
}

/// A byte that extends a slug token: ASCII-alphanumeric or a slug separator (`-`/`_`).
fn is_token_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

/// The order-independent identity of a forward edge — `(from, relation, to)` — for diffing
/// the rename's before/after dangling-edge sets (step 0 vs step 4 of [`apply_and_commit`]).
fn edge_key(edge: &index::Edge) -> (String, String, String) {
    (edge.from.clone(), edge.relation.clone(), edge.to.clone())
}

/// The dangling cross-refs the rename **introduced** — every `after`-rename dangling edge
/// whose `(from, relation, to)` identity was **not** already dangling `before` the rename.
/// This is the M18/M19 masking-trap guard made precise: a pre-existing dangle (present in
/// both sets) is excluded, so the verb's integrity gate refuses only on rot the move itself
/// caused and never on drift it merely inherited (which stays a report-only `jigc validate`
/// concern). Each returned label is `<from>#<relation> -> <to>` for an accurate block
/// message that names only the newly-introduced edges.
fn introduced_dangles<'a>(
    before: &BTreeSet<(String, String, String)>,
    after: impl IntoIterator<Item = &'a index::Edge>,
) -> Vec<String> {
    after
        .into_iter()
        .filter(|edge| !before.contains(&edge_key(edge)))
        .map(|edge| format!("{}#{} -> {}", edge.from, edge.relation, edge.to))
        .collect()
}

/// Split a `<type>:<slug>` address into its parts. Rejects a missing `:` or an empty half —
/// both bail sites carry an actionable route (a concrete example + the `jigc describe`
/// pointer at the doctype surface), since a bare slug (`foo`) is the natural first guess.
fn parse_addr(addr: &str) -> Result<(String, String)> {
    let malformed = || {
        anyhow!(
            "`{addr}` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`\n  \
             route: run {} for the doctype surface",
            engine::finding::Route::mechanical(["jigc", "describe"], ""),
        )
    };
    let (ty, slug) = addr.split_once(':').ok_or_else(malformed)?;
    if ty.is_empty() || slug.is_empty() {
        return Err(malformed());
    }
    // The slug head is what names the file this verb `git mv`s, so it has to be a slug —
    // the third of the four user-address parse boundaries the guard covers (M50 Inc 2 /
    // T1). Driven at `23487ab`, `jigc rename 'research:../../src/planted' --to "Captured
    // Doc"` exited 0 and committed an arbitrary source file into the docs root.
    crate::task::reject_malformed_slug_head(addr, slug)?;
    Ok((ty.to_string(), slug.to_string()))
}

/// The on-disk repo-relative path for the `<ty>:<slug>` doc. A **placement** doctype
/// lives at its literal `placement.file` (the slug is fixed = type id, so it is ignored),
/// making a placement doc reachable as a rename target *and* repointable as a referrer of
/// a renamed doc (`design/storage.md` → Placement — census site `rename`). A
/// `location:`-bearing doctype resolves to `<location>/<slug>.md` (docs-root prefixed,
/// trailing slash trimmed) — unchanged. Errors on an unknown or transient (neither
/// `location` nor `placement`) doctype.
fn doc_path(schema_map: &BTreeMap<String, Schema>, ty: &str, slug: &str) -> Result<String> {
    let schema = schema_map
        .get(ty)
        .ok_or_else(|| crate::render::finding_error(&engine::store::unknown_doctype(ty)))?;
    if let Some(placement) = &schema.placement {
        return Ok(placement.file.clone());
    }
    let location = schema.location.as_deref().ok_or_else(|| {
        refuse(
            RefusalKind::TransientDoctype,
            ty,
            format!(
                "`{ty}` is a transient doctype — it never lands as a repo file, so it has \
                 no persisted path to rename"
            ),
            Route::mechanical(
                ["jigc", "describe"],
                " lists the doctypes that do persist, and the identity each one carries",
            ),
        )
    })?;
    Ok(format!("{}/{slug}.md", location.trim_end_matches('/')))
}

/// Rewrite the document's `# H1` title to `new_title`, preserving every other byte. The
/// H1 is the first ATX level-1 heading (`# `) outside a fenced code block and outside the
/// leading front-matter block (the document title the writer renders from `id-from`).
/// Returns `None` if the doc has no H1.
///
/// **Shared with the in-task sibling** (`crate::doc`'s `jigc doc rename`): the two
/// verbs differ in *what identity they may move*, never in how a title reaches the
/// bytes, so one H1 primitive serves both and neither can drift from the other
/// (`design/write-commands.md` → `jigc doc rename`).
pub(crate) fn rewrite_h1(source: &str, new_title: &str) -> Option<String> {
    let title = h1_span(source)?;
    let mut out = String::with_capacity(source.len() + new_title.len());
    out.push_str(&source[..title.start]);
    out.push_str(new_title);
    out.push_str(&source[title.end..]);
    Some(out)
}

/// The document's `# H1` **title text** — the reader half of the H1 primitive.
///
/// The write-path title pre-check (`crate::doc` — `design/write-commands.md` → The
/// four-way write over a committed doc) must compare a supplied title against exactly
/// the bytes [`rewrite_h1`] would change, so both are spans of the same scan: the reader
/// cannot drift from the writer because the writer *is* the reader plus a splice.
pub(crate) fn read_h1(source: &str) -> Option<&str> {
    h1_span(source).map(|span| &source[span])
}

/// The byte range of the H1's title text — everything after the `# ` marker up to (not
/// including) the line's newline. `None` when the doc carries no H1.
fn h1_span(source: &str) -> Option<std::ops::Range<usize>> {
    // A leading BOM is not document content — `engine::write::first_touch_canonicalize`
    // strips it — but it *is* bytes, so scan past it and keep every returned span an offset
    // into the caller's own source. Left in place, it defeats the front-matter detection
    // below (the block opens on the document's *first* line) and the metadata is scanned as
    // body.
    let bom = if source.starts_with('\u{feff}') {
        '\u{feff}'.len_utf8()
    } else {
        0
    };
    let scanned = &source[bom..];
    let mut in_fence = false;
    let mut offset = bom;
    // The leading `---`-fenced YAML front-matter block (if present) is metadata, not the
    // document body — a human-authored `# ` YAML comment there must never be taken for the
    // H1. Copy it through verbatim and only scan for the H1 past its closing fence. Mirrors
    // the engine's metadata-block detection (`engine::parse::scan_blocks`, pulldown
    // YAML-style metadata): the block opens only when `---` is the document's first line
    // and closes on a `---`/`...` line.
    let mut in_front_matter =
        line_body(scanned.split_inclusive('\n').next().unwrap_or("")) == "---";
    let mut opening_fence = in_front_matter;
    for line in scanned.split_inclusive('\n') {
        let body = line_body(line);
        if in_front_matter {
            if opening_fence {
                opening_fence = false;
            } else if body == "---" || body == "..." {
                in_front_matter = false;
            }
            offset += line.len();
            continue;
        }
        if body.starts_with("```") || body.starts_with("~~~") {
            in_fence = !in_fence;
        } else if !in_fence && let Some(title) = body.strip_prefix("# ") {
            let start = offset + (body.len() - title.len());
            return Some(start..start + title.len());
        }
        offset += line.len();
    }
    None
}

/// One line's content — its EOL stripped, **CRLF and LF alike**. A managed doc's EOL is
/// matched, never globally normalized (`implementation/parsing.md` → "preserve the file's
/// existing EOL"), so a CRLF checkout's `\r` is a line terminator here and never part of
/// the H1's title text.
fn line_body(line: &str) -> &str {
    let body = line.strip_suffix('\n').unwrap_or(line);
    body.strip_suffix('\r').unwrap_or(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrite_h1_swaps_only_the_title_line() {
        let src = "---\nstatus: accepted\n---\n\n# Old title\n\n## Context\n\nProse.\n";
        let out = rewrite_h1(src, "New title").expect("an H1 is present");
        assert_eq!(
            out,
            "---\nstatus: accepted\n---\n\n# New title\n\n## Context\n\nProse.\n"
        );
    }

    #[test]
    fn rewrite_h1_ignores_h2_and_fenced_hash_lines() {
        // A `# ` inside a code fence must not be mistaken for the H1.
        let src = "```\n# not a heading\n```\n\n# Real title\n\n## Section\n";
        let out = rewrite_h1(src, "New").expect("the real H1 is found past the fence");
        assert_eq!(out, "```\n# not a heading\n```\n\n# New\n\n## Section\n");
    }

    #[test]
    fn rewrite_h1_skips_a_front_matter_yaml_comment() {
        // Humans edit managed docs through git (storage.md), so front matter can carry a
        // `# `-prefixed YAML comment. That line is metadata — it must not be taken for the
        // H1: the front matter stays byte-for-byte, and the real body H1 is the one rewritten.
        let src = "---\nstatus: accepted\n# a human-added yaml comment\ndate: 2026-01-01\n---\n\n# Single node cache\n\n## Context\n\nProse.\n";
        let out = rewrite_h1(src, "Distributed cache")
            .expect("the body H1 is found past the front matter");
        assert_eq!(
            out,
            "---\nstatus: accepted\n# a human-added yaml comment\ndate: 2026-01-01\n---\n\n# Distributed cache\n\n## Context\n\nProse.\n"
        );
    }

    /// The **checkout-shape axis** of the H1 primitive. Both halves read bytes a *human*
    /// owns — managed docs are reviewed and edited through git (`design/storage.md`) — so
    /// every shape the engine's own writer already supports (`engine::write::
    /// first_touch_canonicalize`: a leading BOM is stripped, an EOL is *matched*, never
    /// globally normalized) must resolve to the same H1 text and splice back byte-faithful.
    /// Iterated over the shapes rather than pinned at one, so a shape is a row here, not a
    /// rediscovered defect.
    #[test]
    fn the_h1_resolves_across_every_supported_checkout_shape() {
        let lf = "---\nstatus: accepted\n# a human-added yaml comment\ndate: 2026-01-01\n---\n\n# Single node cache\n\n## Context\n\nProse.\n";
        let shapes = [
            ("LF", lf.to_string()),
            ("CRLF", lf.replace('\n', "\r\n")),
            ("BOM + LF", format!("\u{feff}{lf}")),
            (
                "BOM + CRLF",
                format!("\u{feff}{}", lf.replace('\n', "\r\n")),
            ),
        ];
        for (shape, source) in &shapes {
            assert_eq!(
                read_h1(source),
                Some("Single node cache"),
                "{shape}: the H1 text is the body heading — never the front matter's YAML \
                 comment, and never carrying the line's EOL",
            );
            assert_eq!(
                rewrite_h1(source, "Distributed cache").expect("{shape}: an H1 is present"),
                source.replace("Single node cache", "Distributed cache"),
                "{shape}: the splice replaces the title text and nothing else — the BOM, \
                 the front matter and every EOL survive verbatim",
            );
        }
    }

    #[test]
    fn rewrite_h1_none_without_an_h1() {
        assert!(rewrite_h1("## only an h2\n\nbody\n", "X").is_none());
    }

    #[test]
    fn line_has_token_matches_a_standalone_slug_token() {
        let slug = "single-node-cache";
        // Bounded by spaces / line edges / punctuation → a real mention.
        assert!(line_has_token("the single-node-cache decision", slug));
        assert!(line_has_token("single-node-cache", slug));
        assert!(line_has_token("// single-node-cache: legacy", slug));
        assert!(line_has_token("[adr:single-node-cache]", slug));
    }

    #[test]
    fn line_has_token_rejects_a_caches_style_near_match() {
        let slug = "single-node-cache";
        // A trailing alnum (`s`) or a separator (`-`/`_`) extends the token → not a mention.
        assert!(!line_has_token(
            "we considered single-node-caches instead",
            slug
        ));
        assert!(!line_has_token("single-node-cache-v2 supersedes it", slug));
        assert!(!line_has_token("pre-single-node-cache prefix", slug));
        assert!(!line_has_token("single-node-cache_legacy", slug));
    }

    fn edge(from: &str, relation: &str, to: &str) -> index::Edge {
        index::Edge {
            from: from.to_string(),
            relation: relation.to_string(),
            to: to.to_string(),
        }
    }

    #[test]
    fn introduced_dangles_excludes_a_preexisting_dangle() {
        // The masking-trap guard: a dangle present *before* the rename (in `before`) is
        // inherited rot, not caused by the move — it must NOT be flagged as introduced, so
        // the rename is never blocked by drift it did not cause.
        let preexisting = edge("adr:unrelated-dangler", "supersedes", "adr:ghost");
        let before: BTreeSet<(String, String, String)> =
            [edge_key(&preexisting)].into_iter().collect();
        // After the rename the same pre-existing dangle is still present (untouched).
        let after = vec![preexisting.clone()];
        assert!(
            introduced_dangles(&before, &after).is_empty(),
            "a pre-existing dangle must not count as introduced by the rename",
        );
    }

    #[test]
    fn introduced_dangles_flags_a_newly_introduced_dangle() {
        // The gate must still fire: a dangle that appears *only after* the rename (not in
        // `before`) is one the move itself caused — it is flagged with an accurate label.
        let preexisting = edge("adr:unrelated-dangler", "supersedes", "adr:ghost");
        let before: BTreeSet<(String, String, String)> =
            [edge_key(&preexisting)].into_iter().collect();
        // The pre-existing dangle survives AND a new one appears (a referrer the move failed
        // to repoint, say) — only the new one is reported.
        let newly = edge("adr:missed-referrer", "supersedes", "adr:single-node-cache");
        let after = vec![preexisting, newly];
        assert_eq!(
            introduced_dangles(&before, &after),
            vec!["adr:missed-referrer#supersedes -> adr:single-node-cache".to_string()],
            "a newly-introduced dangle must be flagged, naming only the new edge",
        );
    }

    #[test]
    fn parse_addr_rejects_malformed() {
        assert!(parse_addr("adr:slug").is_ok());
        // Both bail sites (missing `:` and empty half) carry the actionable route: a
        // concrete `<type>:<slug>` example and the `jigc describe` pointer.
        for bad in ["noslug", ":slug", "adr:"] {
            let msg = parse_addr(bad).unwrap_err().to_string();
            assert!(msg.contains("<type>:<slug>"), "form: {msg}");
            assert!(msg.contains("adr:"), "example: {msg}");
            assert!(msg.contains("jigc describe"), "describe pointer: {msg}");
        }
    }

    fn schema_map(yamls: &[&str]) -> BTreeMap<String, Schema> {
        yamls
            .iter()
            .map(|y| {
                let s = engine::schema::load_schema(y.as_bytes()).expect("schema loads");
                (s.ty.clone(), s)
            })
            .collect()
    }

    /// (M38 inc-2 T3) `doc_path` resolves a **placement** doctype's on-disk path to its
    /// literal `placement.file` (the slug is fixed = type id), so a placement doc that
    /// references a renamed doc repoints at its real home — never the `{dir}/{slug}.md`
    /// composition that a location doctype uses (`design/storage.md` → Placement — census
    /// site `rename`). A `location:`-bearing doctype is unchanged: `<location>/<slug>.md`.
    #[test]
    fn doc_path_resolves_a_placement_literal_and_leaves_location_docs_unchanged() {
        let map = schema_map(&[
            "type: foo\nplacement: { file: FOO.md }\nsections: []\n",
            "type: adr\nlocation: decisions/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: x }\n",
        ]);

        // A placement doctype resolves to its literal file, ignoring the passed slug.
        assert_eq!(
            doc_path(&map, "foo", "foo").expect("placement path resolves"),
            "FOO.md",
            "a placement doctype's path is its literal placement.file",
        );
        // A location doctype is byte-unchanged: `<location>/<slug>.md`.
        assert_eq!(
            doc_path(&map, "adr", "single-node-cache").expect("location path resolves"),
            "decisions/single-node-cache.md",
            "a location doctype still resolves to <location>/<slug>.md",
        );
    }
}
