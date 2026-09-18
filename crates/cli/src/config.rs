//! `jigc config <verb>` — authoring cascade deltas into the project layer
//! (`design/overrides.md` → Authoring deltas — the `jigc config` verbs).
//!
//! `config set <key> <value>` (the `scalar-set` rung), the three `structural-op`
//! verbs `insert-step` / `replace-step` / `remove-step`, the `slot-fill` verb `fill`,
//! and the `tracked-fork` verb `fork` all land here. `set` writes **only** the project
//! layer's `.jigc/config/manifest.yaml` `scalar:` block; `insert-step` / `replace-step`
//! append to its `deltas:` list + write a native `steps/<basename>.yaml`, `remove-step`
//! appends a delta with no native file (`replace-step` / `remove-step` also record the
//! **displaced** pack unit's `base-version` + `base-hash` — the M5 basis), `fill`
//! appends a `slot-fill` delta + writes a
//! native `fills/<id>.md`, and `fork` appends a `tracked-fork` delta (with its pinned
//! `base-version` + `base-hash`) + copies the resolved unit to `steps/<id>.yaml` — each
//! a delta against the project layer, never a base definition.
//!
//! **Write-time adjudication** (`overrides.md` → Write-time vs resolve-time
//! split): the cheap, local checks run here so the human gets an immediate error
//! — the key must be a **declared knob** (the pack's `config/knobs.yaml` closed
//! surface) and its value must pass the **same** [`engine::write::check_value`]
//! the doc write path uses (no second type system). An undeclared key or a
//! wrong-type value is rejected with a routed blocking finding and **no** write;
//! whole-cascade consequences (cycles, orphaning) surface later at resolution
//! through `workflow-refs`.

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::orphan::{self, Home};
use crate::pack::make_pack;
use crate::render::{ConfigAck, KnobReading, RejectedSet, Relocated};
use anyhow::{Context, Result, bail};
use engine::cascade::{
    Anchor, LayerKind, SlotFillTarget, StructuralDelta, StructuralTarget, TrackedForkDelta,
};
use engine::field_block::Value;
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// **The two root knobs** — the keys that re-point where every managed doc lives, and whose
/// re-point then *moves* the committed docs it strands (`docs-root` over every `location:`
/// doctype, `placement-root` over every nested `placement:` home).
///
/// One home, because every rule that binds one binds the other: the empty-value
/// canonicalization below and M49's untrackable-home refusal both read it rather than
/// re-spelling the pair per call site, and every further root rule joins them here.
/// Two hand-written `matches!` over these keys is exactly the shape M45's complete-fix lens is
/// named for — a rule applied at one arm and not the other is a rule that is not applied at
/// all — so `crates/cli/tests/root_knob_rules.rs` fences the pair literal out of this file and
/// asserts each member is a **declared** knob in the pack's `config/knobs.yaml`.
///
/// It is deliberately *not* the subject of the two per-knob re-point movers further down
/// (`route_docs_root_repoint_orphans`, `route_placement_root_repoint_strands`): those dispatch
/// each knob to its **own** floor, so they are a per-knob switch by design, not a duplicated
/// pair rule.
pub const ROOT_KNOBS: [&str; 2] = ["docs-root", "placement-root"];

/// The `jigc config <verb>` subcommand tree — `set`, the structural-op verbs
/// `insert-step` / `replace-step` / `remove-step`, the `fill` verb, and `fork`.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum ConfigCommand {
    /// Record a `scalar-set` delta — set a closed-surface knob in the project
    /// layer's `manifest.yaml` `scalar:` block. Adjudicated at write time against
    /// the pack's declared knob (`check_value`): an undeclared key or a wrong-type
    /// value is rejected non-zero with a routed finding. Setting a location knob
    /// (`docs-root`) also relocates the committed docs the re-point would strand —
    /// every committed doc under a doctype's prior resolved `location:` directory,
    /// managed or not. A **placement** doctype's file is not among them: it homes at
    /// its declared `placement.file`, which resolves through `placement-root`, so a
    /// `docs-root` re-point never moves it even when it sits under the old root
    /// (`design/storage.md` → Placement). The moves are staged `git mv`s, printed per
    /// file (they land with your next commit).
    Set {
        /// The knob key to set — one of the pack's declared `config/knobs.yaml` keys.
        key: String,
        /// The value to set (adjudicated against the knob's declared type).
        value: String,
    },

    /// Splice a native step into a workflow's include list, anchored `--after` or
    /// `--before` an existing step. The native step takes its id from the source
    /// `<file>`'s basename and is written to `.jigc/config/steps/<basename>.yaml`.
    /// Write-time adjudicated: a basename colliding with an existing step id, or an
    /// anchor absent from the current resolution, is rejected non-zero with no write.
    /// (Unlike `replace-step`/`remove-step`, which take a `workflow:<id>#<step-id>`
    /// address, insert needs an anchor step *and* a side — which the `#` address can't
    /// express — hence the `--workflow` + `--after`/`--before` form.)
    InsertStep {
        /// The workflow whose include list the step splices into.
        #[arg(long)]
        workflow: String,
        /// Insert the native step *after* this anchor step id (mutually exclusive
        /// with `--before`; exactly one anchor is required).
        #[arg(long, conflicts_with = "before", required_unless_present = "before")]
        after: Option<String>,
        /// Insert the native step *before* this anchor step id (mutually exclusive
        /// with `--after`; exactly one anchor is required).
        #[arg(long)]
        before: Option<String>,
        /// The source step file; its basename becomes the native step id.
        file: PathBuf,
    },

    /// Swap which step appears at a position in a workflow's include list, addressed
    /// `workflow:<id>#<step-id>`. The replacement is the native step the `<file>`
    /// registers (id = file basename, written to `.jigc/config/steps/<basename>.yaml`).
    /// Write-time adjudicated: a basename colliding with an existing step id, or a
    /// target step-id absent from the current resolution, is rejected non-zero with
    /// no write.
    ReplaceStep {
        /// The `workflow:<id>#<step-id>` entry to replace.
        target: String,
        /// The source step file; its basename becomes the native step id.
        file: PathBuf,
    },

    /// Drop the step at a position in a workflow's include list, addressed
    /// `workflow:<id>#<step-id>`. No native file (nothing to add). Write-time
    /// adjudicated: a target step-id absent from the current resolution is rejected
    /// non-zero with no write.
    RemoveStep {
        /// The `workflow:<id>#<step-id>` entry to remove.
        target: String,
    },

    /// Inject content into a `{{fill:<fill-id>}}` extension point a step body
    /// anticipates, addressed `step:<id>#<fill-id>`. The content arrives via stdin /
    /// `--from-file` (prose, never inline) and is written to
    /// `.jigc/config/fills/<fill-id>.md`. Two write-time checks (both before any
    /// write): the `{{fill:<fill-id>}}` point must exist in the resolved step body,
    /// and the content must contain no nested `{{fill:}}`. A rejection exits non-zero.
    /// (Distinct from `doc set-slot`: `fill` injects into a *workflow step's*
    /// extension point — cascade-authoring; `set-slot` fills a managed *document's*
    /// slot — the write path.)
    Fill {
        /// The `step:<id>#<fill-id>` extension point to fill.
        target: String,
        /// The content source: a path, or `-` for stdin (prose never inline).
        #[arg(long)]
        from_file: String,
    },

    /// Read one declared knob's resolved value, naming the cascade layer that won it
    /// (`pack-default` / `team` / `project`) and — when a `scalar-set` for the key was
    /// dropped by the knob's demotion-lock `floor` — the value that was attempted and
    /// the floor it ranked below. An undeclared key is rejected non-zero and routed to
    /// `jigc config list`, which enumerates the closed surface.
    Get {
        /// The knob key to read — one of the pack's declared `config/knobs.yaml` keys
        /// (`jigc config list` prints them all).
        key: String,
    },

    /// List every declared knob with its resolved value and winning layer — the closed
    /// surface a `scalar-set` may target, read from the pack's `config/knobs.yaml`
    /// rather than a curated excerpt. A knob whose `scalar-set` was dropped by its
    /// floor carries that drop on its row.
    List,

    /// Copy a resolved step's body into a native file that shadows it, recording the
    /// pinned ancestor (`base-version` + the blake3 `base-hash` of the copied bytes),
    /// addressed `workflow:<id>#<step-id>`. The body bytes are copied verbatim to
    /// `.jigc/config/steps/<step-id>.yaml`, so at compose time the fork is just a file
    /// shadow. Write-time adjudicated: the target step-id must resolve, and a unit
    /// already forked is a collision — each is rejected non-zero. The recorded basis is
    /// what a later upgrade reconciliation reads.
    Fork {
        /// The `workflow:<id>#<step-id>` unit to fork.
        target: String,
    },
}

impl ConfigCommand {
    /// Dispatch the parsed `config` verb against `cwd`, mapping the result to a
    /// process exit code. A blocking adjudication finding surfaces on stderr (with
    /// its key + route) through the shared operational-error funnel — `{"error": …}`
    /// under `--format json`, the plain `{err:#}` bytes otherwise — and exits
    /// non-zero; a clean run exits 0 after printing its surface on stdout.
    ///
    /// Each arm renders its own surface before the shared funnel: the six authoring
    /// verbs share the [`ConfigAck`] write-confirmation, the two read verbs their
    /// knob-reading views — so both rungs of the family exit through one place.
    pub fn dispatch(self, cwd: &Path, format: crate::cli::Format) -> Outcome {
        let result = match self {
            ConfigCommand::Get { key } => {
                run_get(cwd, &key).map(|reading| crate::render::config_get(format, &reading))
            }
            ConfigCommand::List => {
                run_list(cwd).map(|readings| crate::render::config_list(format, &readings))
            }
            // Every config write states its effect (Law 1 "acks state the effect"; the M43
            // surface census) — the positive ack the six verbs mapped to silence before.
            ConfigCommand::Set { key, value } => run_set(cwd, format, &key, &value)
                .map(|ack| crate::render::config_ack(format, &ack)),
            ConfigCommand::InsertStep {
                workflow,
                after,
                before,
                file,
            } => run_insert_step(cwd, &workflow, after.as_deref(), before.as_deref(), &file)
                .map(|ack| crate::render::config_ack(format, &ack)),
            ConfigCommand::ReplaceStep { target, file } => run_replace_step(cwd, &target, &file)
                .map(|ack| crate::render::config_ack(format, &ack)),
            ConfigCommand::RemoveStep { target } => {
                run_remove_step(cwd, &target).map(|ack| crate::render::config_ack(format, &ack))
            }
            ConfigCommand::Fill { target, from_file } => run_fill(cwd, &target, &from_file)
                .map(|ack| crate::render::config_ack(format, &ack)),
            ConfigCommand::Fork { target } => {
                run_fork(cwd, &target).map(|ack| crate::render::config_ack(format, &ack))
            }
        };
        match result {
            Ok(surface) => {
                println!("{surface}");
                Outcome::success()
            }
            Err(err) => crate::invocation_log::operational_failure(format, &err),
        }
    }
}

/// Read the whole declared knob surface as resolved readings — the one read the two
/// read verbs share (`design/overrides.md` → Reading the resolved cascade).
///
/// The enumeration is the pack's **declared** knob set (`config/knobs.yaml` via
/// [`engine::knobs::load_knobs`]) — the same closed surface `run_set` adjudicates
/// against, never a curated excerpt — and each key's value/provenance comes from the
/// already-shipped resolution path ([`crate::start::resolve_severity_cascade`]): the
/// resolved value from [`engine::cascade::Resolved::scalar`], the winning layer from
/// `scalar_overrides` (absent = the pack-default base), and any dropped `scalar-set`
/// from `rejected_scalar_sets`. No engine change: this rung only *projects* what
/// resolution already computes.
///
/// Rows come out in declared key order (`load_knobs` reads a `BTreeMap`), so both
/// verbs' output is stable.
fn read_knobs(cwd: &Path) -> Result<Vec<KnobReading>> {
    let project_config = require_project_layer(cwd)?;
    let pack = make_pack()?;
    let knobs_bytes = crate::start::read_pack(pack.as_ref(), PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let declared: Vec<String> = knobs.keys().map(str::to_owned).collect();

    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config)?;
    let layers: BTreeMap<&str, LayerKind> = resolved
        .scalar_overrides()
        .map(|(key, _value, layer)| (key, layer))
        .collect();
    let mut rejected: BTreeMap<&str, RejectedSet> = resolved
        .rejected_scalar_sets()
        .map(|(key, attempted, floor, layer)| {
            (
                key,
                RejectedSet {
                    attempted: attempted.to_owned(),
                    floor: floor.to_owned(),
                    layer: layer.label(),
                },
            )
        })
        .collect();

    Ok(declared
        .iter()
        .map(|key| KnobReading {
            value: resolved.scalar(key).unwrap_or_default().to_owned(),
            layer: layers
                .get(key.as_str())
                .copied()
                .unwrap_or(LayerKind::PackDefault)
                .label(),
            rejected: rejected.remove(key.as_str()),
            key: key.clone(),
        })
        .collect())
}

/// `jigc config get <key>` — one declared knob's resolved reading (value + winning
/// layer + any soft-rejected set). An undeclared key is the same closed-surface
/// rejection `config set` raises ([`undeclared_key_finding`]) — the surface is closed
/// for reads exactly as it is for writes, so a typo answers rather than resolving to
/// nothing.
fn run_get(cwd: &Path, key: &str) -> Result<KnobReading> {
    read_knobs(cwd)?
        .into_iter()
        .find(|reading| reading.key == key)
        .ok_or_else(|| finding_to_err(undeclared_key_finding(key)))
}

/// `jigc config list` — every declared knob with its resolved reading, in declared key
/// order. The enumeration of the closed surface, and the verb every
/// `config.undeclared-key` rejection routes to.
fn run_list(cwd: &Path) -> Result<Vec<KnobReading>> {
    read_knobs(cwd)
}

/// The closed-surface rejection both read (`config get`) and write (`config set`)
/// raise for a key the pack does not declare — one finding, one code, one route.
///
/// The route names `jigc config list`, the verb that enumerates the surface: a read
/// intent lands on a read verb, never on the write verb that happens to share the
/// rejection (`design/surface-contract.md` → law 2, the recovery must lead to the fix).
fn undeclared_key_finding(key: &str) -> Finding {
    Finding::block(
        "config.undeclared-key",
        format!("`{key}` is not a declared knob — the cascade surface is closed"),
        "run `jigc config list` to see every declared knob with its resolved value and \
         winning layer, then re-run with one of those keys",
    )
}

/// `jigc config set <key> <value>` — record a `scalar-set` in the project layer.
///
/// Adjudicates the write locally (`overrides.md` → Write-time vs resolve-time
/// split), then records the value into the project manifest's `scalar:` block:
/// 1. `<key>` is a declared knob in the pack's `config/knobs.yaml` else reject;
/// 2. `<value>` passes `check_value` against the knob's declared type else reject;
/// 3. write `scalar.<key> = <value>` into `.jigc/config/manifest.yaml`
///    (last-write-wins; any existing `scalar:` entries and `deltas:` block are
///    preserved).
fn run_set(cwd: &Path, format: Format, key: &str, value: &str) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;

    // `docs-root` treats an empty value as "no prefix" (the flat repo-root layout),
    // but the opaque-scalar floor (`check_value`) rejects an empty string — that floor
    // exists to keep a *document* from rendering an empty `- key:` bullet, which a knob
    // never does. Canonicalize an empty `docs-root` to its flat sentinel `.` so
    // `jigc config set docs-root ""` does the intuitive thing instead of erroring (the
    // read-side helper already maps both `""` and `.` to flat).
    //
    // `placement-root` (M49) rides the same branch, and for it the canonicalization is
    // load-bearing rather than a convenience: its resolved `""` is the *unset* value —
    // every declared placement home stands — while `.` is the repo root, so an empty set
    // that landed verbatim would read back as "never set" and silently do nothing
    // (`crate::start::placement_root`).
    //
    // **Both canonicalizations happen before anything else looks at the value** — the fold
    // first, so `docs/..` reaches the empty branch and lands as `.` rather than as a root no
    // reader folds. See [`normalize_root_value`] for why the fold is here and not in each
    // reader.
    //
    // `typed` survives the fold because a **refusal names what the operator typed**: the three
    // root rules below adjudicate the folded value (that is what makes them un-evadable) but
    // quote `typed`, so `docs/../.jigc` is refused as `docs/../.jigc` and not as a `.jigc` the
    // operator never wrote. The *ack* prints the folded value, which is the one that landed.
    let typed = value;
    let folded = if ROOT_KNOBS.contains(&key) {
        normalize_root_value(value)
    } else {
        value.to_owned()
    };
    let value: &str = if ROOT_KNOBS.contains(&key) && folded.is_empty() {
        "."
    } else {
        &folded
    };

    // Step 1 — the key must be a declared knob (the closed surface).
    let pack = make_pack()?;
    let knobs_bytes = crate::start::read_pack(pack.as_ref(), PackResourceKind::Config, "knobs")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let Some(field) = knobs.field(key) else {
        // The closed surface now has a verb of its own to name (M48 Inc 6): `jigc config
        // list` prints every declared knob with its resolved value and winning layer, so
        // the route leads to the fix (Law 2) without this message having to *be* the
        // enumeration. The inline listing it replaces was the only enumeration in the
        // binary — which is why it had to characterize-and-count the large
        // `validation.<probe>.<check>.severity` family rather than print it.
        return Err(finding_to_err(undeclared_key_finding(key)));
    };

    // Step 2 — the value must pass the same `check_value` the doc write path uses.
    if let Err(reason) = engine::write::check_value(field, &Value::Scalar(value.to_owned())) {
        return Err(finding_to_err(Finding::block(
            "config.value-rejected",
            format!("`{value}` is not a valid value for `{key}`: {reason}"),
            "set a value matching the knob's declared type",
        )));
    }

    // Step 2b — **a home git cannot record is not a home** (M49 completion triage, the HIGH
    // data-loss finding). Both root knobs re-point where every managed doc lives, and both
    // then MOVE the committed docs the re-point strands. `jigc config set placement-root .git`
    // printed `docs/roadmap.md → .git/roadmap.md`, exited 0, and left a staged deletion with
    // no matching add — `git mv` into git's own directory prints `error: invalid path` and
    // **exits 0**, so the mover read success and the doc survived only in history, gone from
    // the next clone. `validate` then graded that store clean.
    //
    // The refusal is at the *door*, not per-doc, and it lands **before** anything moves or the
    // knob is written, because the alternatives are both worse: a warn-and-proceed loses the
    // bytes at exit 0, and a per-doc skip that still lands the knob leaves the store pointing
    // at a home no doc is at. The value is the operator's, so a refusal is actionable — and it
    // holds even with no committed doc to strand yet, since every *future* create and
    // finalize-promote would write to the same unrecordable home.
    //
    // Scoped to what git cannot record, never to roots that merely look unusual: a
    // **gitignored** root is a path git tracks perfectly well and has only been told to skip —
    // it stages a real `R` rename — and it keeps working (driven: with `ignored-notes/` in
    // `.gitignore`, `placement-root ignored-notes` still moves and stages `R`). The workbench
    // `.jigc` is refused one step below, for a different reason and under its own code: jigc
    // owns that tree, which is not git's inability to record it — and it was never the
    // gitignored exemplar this comment used to name, since `.jigc/.gitignore` lists only the
    // transient subdirs (`.jigc/roadmap.md` is not ignored at all; `.jigc/displaced/x.md` is).
    if ROOT_KNOBS.contains(&key)
        && let Some(repo_root) = project_config.parent().and_then(Path::parent)
        && let Some(reason) = crate::trackable::untrackable_reason(repo_root, value)
    {
        return Err(finding_to_err(Finding::block(
            "config.untrackable-root",
            format!("`{typed}` cannot be the `{key}`: {reason}"),
            "re-run with a root git can record — a path under the repository root and outside \
             `.git/`; `jigc config list` shows the value in force and the layer it wins from",
        )));
    }

    // Step 2c — **jigc's own workbench is not a home for managed docs** (M50 Increment 4 / T2;
    // `settle-record.md` → D8; `design/storage.md` → Placement). Both root knobs re-point where
    // every managed doc lives and then move the committed docs the re-point strands, so
    // `jigc config set docs-root .jigc` staged `docs/research/x.md → .jigc/research/x.md` and
    // exited 0 — driven, on both knobs. That home is the tree `jigc uninstall` removes **whole**
    // ([`crate::setup`]'s teardown step 1 is `remove_dir_all(.jigc)`), so a managed doc homed
    // there is one teardown away from bytes no index has a copy of.
    //
    // It is a **third** predicate, not a widening of step 2b: the workbench is a path git tracks
    // perfectly well, and 2b must keep saying so — a gitignored destination stays permitted, on
    // which `relocate`'s squatter displacement into `.jigc/displaced/` depends
    // ([`crate::trackable`]'s own stated bound). The two answer different questions about the
    // same path, so they carry different codes and sit one after the other.
    //
    // Same position and same posture as 2b: before anything moves and before the knob lands,
    // because the alternatives are the ones that lose bytes — a warn-and-proceed moves the docs
    // into the workbench anyway, and a landed knob with no move leaves the store pointing at a
    // home no doc is at.
    if ROOT_KNOBS.contains(&key) && is_workbench_root(value) {
        return Err(finding_to_err(Finding::block(
            "config.workbench-root",
            format!(
                "`{typed}` is inside jigc's own workbench (`.jigc/`) — `{key}` cannot home \
                 managed docs in the tree `jigc uninstall` removes whole"
            ),
            "re-run with a root outside `.jigc/` — the workbench holds jigc's own state, not \
             your documents; `jigc config list` shows the value in force and the layer \
             it wins from",
        )));
    }

    // Step 2d — **a root the store cannot describe is refused before it is written** (M50
    // Increment 4 / T3; `settle-record.md` → D8, the value rule; `design/storage.md` → Placement).
    // 2b and 2c ask *where* the value points; this asks whether the value is a usable root at
    // all. Three shapes are not, all three driven at HEAD:
    //
    //   - **file-shaped** (`README.md`): the knob landed at exit 0 with EVERY move failed
    //     (`creating the destination dir …: File exists`), and `jigc doc list` then dropped both
    //     re-rooted docs from the store surface entirely — the exact state step 2b's own
    //     rationale says must not exist, produced by the door that states it.
    //   - **absolute** (`/tmp/elsewhere`): silently reinterpreted as repo-relative — the mover
    //     staged `R docs/roadmap.md -> tmp/elsewhere/roadmap.md` (`crate::trackable` trims the
    //     leading `/`) while `jigc config get` echoed back `/tmp/elsewhere`, so the knob and the
    //     store named different homes for the same doc.
    //   - **symlinked** (`linked`, or `linked/sub`): the move staged `RD` — git records the link,
    //     not a path through it, so the index holds a path the worktree does not have and
    //     `jigc doc list`'s path and git's recorded path disagree permanently.
    //
    // One code with the reason in the message, on `config.untrackable-root`'s five-reasons-one-code
    // precedent: the operator's fix is the same in all three cases — supply a different root.
    //
    // Same position and posture as 2b/2c, and for the same reason: before anything moves and
    // before the knob lands.
    if ROOT_KNOBS.contains(&key)
        && let Some(repo_root) = project_config.parent().and_then(Path::parent)
        && let Some(reason) = unusable_root_reason(repo_root, value)
    {
        return Err(finding_to_err(Finding::block(
            "config.unusable-root",
            format!("`{typed}` cannot be the `{key}`: {reason}"),
            format!(
                "re-run with a repo-relative directory — an existing one, or one jigc should \
                 create; never a file, an absolute path, a path through a symlink, a value \
                 padded with whitespace, a component over {} bytes, or one git reads as a \
                 pathspec rather than a name (a leading `:`, or a `*`, `?`, `[` or `\\` \
                 anywhere); `jigc config list` shows the value in force and the layer it \
                 wins from",
                crate::cli::NAME_MAX_BYTES,
            ),
        )));
    }

    // docs-root re-point: detect + route + MOVE the committed docs the change would strand at
    // the *prior* resolved root (M39 inc-5 T3, replacing the M36 warn-then-strand). This is
    // `run_set`'s first read of the committed doc surface — a store-access seam that resolves
    // the *current* cascade (its `docs-root` is the old root) and enumerates `git ls-files`,
    // comparing the old resolved root against the new value (`design/storage.md` → docs-root;
    // `design/reconciliation.md` → the route home). It runs BEFORE the knob lands so the
    // resolution reads the old cascade.
    //
    // **The re-point is ONE transaction** (M52 Increment 5 / T8; `settle-record.md` → D1.5,
    // population 10). The moves and the knob write only make sense together — the moves
    // re-home the docs the new root strands, the knob is what makes the new root the store's
    // — and through rc.15 there was nothing between them: a failing `write_scalar` left every
    // `git mv` staged and every file-state key re-pointed against a knob that never landed,
    // at exit 1, with the store pointing at a home no doc was at. So both floors now capture
    // what they overwrite into `undo`, and both report a doc they could **not** move into
    // `failures` instead of exiting 0 over it — the two triggers that roll the batch back.
    //
    // The floors **return** what they moved (M51 Increment 5 / T4, EC-4): the moves
    // narrate on stderr as they land, and until the ack carried them a `--format json`
    // driver — which reads stdout alone — got bytes identical to a re-point that moved
    // nothing.
    let mut relocated: Vec<Relocated> = Vec::new();
    let mut undo = crate::relocate::MoveRollback::for_door(crate::rollback::CONFIG_DOOR);
    let mut failures: Vec<RepointBlocker> = Vec::new();
    if key == "docs-root" {
        relocated = route_docs_root_repoint_orphans(
            pack.as_ref(),
            format,
            &project_config,
            value,
            &mut undo,
            &mut failures,
        );
    }

    // `placement-root` re-point: the placement sibling of the loop above, same place and
    // same posture — it runs BEFORE the knob lands (so the prior home resolves off the old
    // cascade). T1 shipped the knob and
    // said so plainly: a re-point re-resolves every nested placement home while the
    // committed instance stays put, and the store goes QUIET about it (the record still
    // baselines the old path and the file still matches). This is the floor that closes it.
    if key == "placement-root" {
        relocated = route_placement_root_repoint_strands(
            pack.as_ref(),
            format,
            &project_config,
            value,
            &mut undo,
            &mut failures,
        );
    }

    // Step 3 — record the `scalar-set` into the project manifest (last-write-wins), and close
    // the transaction. A knob write that fails is the second trigger; it is asked only when
    // the moves left none of their own, because a batch that already has to be undone must
    // not also land the knob it was undone for.
    if failures.is_empty()
        && let Err(err) = write_scalar(&project_config, key, value)
    {
        failures.push(RepointBlocker::Cause {
            at: crate::render::repo_relative(
                repo_root_of(&project_config).unwrap_or(&project_config),
                &project_config.join("manifest.yaml"),
            ),
            cause: format!("{err:#}"),
        });
    }
    if !failures.is_empty() {
        return Err(reject_repoint(
            &project_config,
            key,
            value,
            &failures,
            &undo,
        ));
    }
    Ok(ConfigAck::Set {
        key: key.to_owned(),
        value: value.to_owned(),
        relocated,
        // **The fold is named in the ack, never performed silently** (M52 Increment 6 / T6;
        // `settle-record.md` → D8's surface tier). `typed` is what the operator wrote and
        // `value` is what landed; they differ only when the root-knob fold above changed the
        // spelling, which is exactly when the operator cannot read the one from the other.
        // Driven at HEAD, both coercions landed at exit 0 saying nothing: `docs-root ""`
        // acked `= `.`` — *the repository root* for a token that says *unset* — and
        // `placement-root x/../y` acked `= `y``.
        folded_from: (typed != value).then(|| typed.to_owned()),
    })
}

/// `<repo>/.jigc/config` → the repository root the committed docs live at — the derivation
/// every root rule and both move floors already made privately, named once so the
/// transaction's rollback and its findings resolve the same root they do.
fn repo_root_of(project_config: &Path) -> Option<&Path> {
    project_config.parent().and_then(Path::parent)
}

/// **One reason a root-knob re-point did not complete** (M52 Increment 5 / T8).
///
/// Three produce one, and the second is the discharge of M52 Increment 1 / T1's declared
/// residual: the knob write failing (`at` = the project manifest), a committed doc the floor
/// could not relocate (`at` = the doc's prior home), and the sweep-ending repository refusal.
/// The per-doc failure used to narrate `could not relocate … — move it by hand` on stderr,
/// be withheld entirely under `--format json`, and exit **0** with the knob landed — so a
/// driver could not read it and a human was left with a store pointing at a home the doc was
/// not at.
enum RepointBlocker {
    /// A fault this door has to word itself, as `config.repoint-failed` keyed at `at` — the
    /// repo-relative path the fault is about, which is also what discriminates two faults in
    /// one run.
    Cause {
        /// The path the reason is about, repo-relative.
        at: String,
        /// What went wrong, in the producer's own words (a rendered `anyhow` chain).
        cause: String,
    },
    /// A refusal another seam **already worded, with its own code, target and route** — the
    /// posture family's operation-in-progress, which ends the sweep because it is a fact
    /// about the repository. It is carried verbatim rather than flattened into a
    /// `config.repoint-failed` message, because a code inside a message is not a key
    /// (`design/command-output-contract.md` → the membership test).
    Raised(Finding),
}

/// Roll the re-point back and compose its refusal — **one document**, carrying the failure(s)
/// and every path the rollback could not put back (M52 Increment 5 / T8).
///
/// The contract's selection rule is *a reject that carries a finding takes the findings arm,
/// with the operational error itself as a finding*, and *on the arm whose document is
/// stderr's, nothing else is written there*
/// (`design/command-output-contract.md` → The two reject arms / Stream discipline) — so the
/// conflicts ride [`crate::render::envelope_finding_error_beside`] rather than printing
/// beside the envelope, which is the shape M52 Increment 1 closed at the committing doors.
fn reject_repoint(
    project_config: &Path,
    key: &str,
    value: &str,
    failures: &[RepointBlocker],
    undo: &crate::relocate::MoveRollback,
) -> anyhow::Error {
    let conflicts = match repo_root_of(project_config) {
        Some(repo_root) => {
            crate::relocate::rollback_relocations(repo_root, &repo_root.join(".jigc"), undo)
        }
        // No repository root to resolve against is a shape the door cannot reach (the project
        // layer was located under one), and a rollback that cannot name its paths must not
        // guess at them: the failures below still refuse the run.
        None => Vec::new(),
    };
    let mut findings: Vec<Finding> = failures
        .iter()
        .map(|failure| match failure {
            RepointBlocker::Cause { at, cause } => {
                repoint_failed(key, value, at, cause, !conflicts.is_empty())
            }
            RepointBlocker::Raised(finding) => finding.clone(),
        })
        .collect();
    let head = findings.remove(0);
    findings.extend(conflicts);
    crate::render::envelope_finding_error_beside(&head, findings)
}

/// The refusal one [`RepointBlocker::Cause`] raises: blocking, keyed at the path the cause
/// names, routed at the human who has to fix the cause and re-run.
///
/// **The route states what was undone, and says so differently when it could not be.** A
/// conflicted rollback leaves one path holding somebody else's bytes, so the unqualified
/// *"every doc is back at its prior home"* would be a law-1 lie on exactly the run where the
/// reader most needs the truth (`design/surface-contract.md` → law 1).
fn repoint_failed(key: &str, value: &str, at: &str, cause: &str, conflicted: bool) -> Finding {
    let restored = if conflicted {
        "every doc this re-point moved is back at its prior home except the path(s) \
         the `config.rollback-conflict` finding(s) beside this one name"
    } else {
        "every doc this re-point moved is back at its prior home"
    };
    Finding::graded(
        engine::finding::Severity::Blocking,
        "config.repoint-failed",
        format!("`{key}` was not set to `{value}`: {cause} — the re-point was undone"),
        Some(engine::finding::Location::addressed(at.to_owned(), 1, 1)),
        Some(engine::finding::Route::human(format!(
            "`{key}` is unchanged and {restored}. Fix what this message names, then \
             re-run `jigc config set {} {}`",
            engine::finding::shell_token(key),
            engine::finding::shell_token(value),
        ))),
    )
}

/// Fold a root-knob value to the **one spelling every reader resolves** — `./` segments
/// dropped, `..` hops applied, redundant separators collapsed — so the value that lands is the
/// value the store renders (M50 Increment 4 validation, N8).
///
/// **Why the fold is here and not in each reader.** Every root rule already folds privately to
/// find the home a value *reaches* ([`is_workbench_root`], [`unusable_root_reason`],
/// [`crate::trackable::untrackable_reason`]), and the two move floors and every read surface
/// then resolved the value **as typed** — a spelling no fold had touched. Driven at `71c8f7a`,
/// on a corpus whose docs are homed under `docs/`:
///
///   - `jigc config set placement-root docs/../docs` — a value naming byte-for-byte the home
///     the docs are already at — was admitted at exit 0 **as a re-point**. The destination was
///     the source, so [`crate::relocate::relocate_stranded`] read both committed docs as
///     foreign squatters, displaced them into the gitignored `.jigc/displaced/` and then could
///     not move them back: two staged deletions with no matching adds, the knob landed, and
///     `jigc doc list` stopped naming the docs at all — the exact state step 2b's own rationale
///     says must never exist. Five spellings of that one home reach it on **both** knobs
///     (`./docs`, `docs/./`, `docs/../docs`, `.jigc/../docs`, and every multi-hop fold of them);
///     only the trailing-slash form escaped, because `trim_matches('/')` was the single fold the
///     storage path already did.
///   - `jigc config set docs-root docs/../notes` — a genuine move — staged
///     `R docs/research/x.md -> notes/research/x.md` while `jigc doc list` printed
///     `docs/../notes/research/x.md`: the knob and git naming different homes for one doc,
///     which is the state the absolute-value reason in [`unusable_root_reason`] is refused for.
///
/// One fold at the **authoring** door answers both, and answers them for every reader at once —
/// the move floors, `apply_docs_root`/`apply_placement_root`, `doc list`'s path column,
/// `validate`'s orphan probe and `config get`'s echo all read one canonical string. It is not a
/// silent rewrite: the ack prints the value that landed, so `docs/../docs` acks as `docs`.
///
/// **Two shapes are deliberately returned unfolded, because a sibling rule owns them and its
/// message quotes the typed form:** an *absolute* value (refused by [`unusable_root_reason`] —
/// folding it would only produce a second absolute path), and a `..` with nothing left to pop,
/// which climbs above the repository root and is kept so
/// [`crate::trackable::untrackable_reason`] can refuse the value the operator actually named.
///
/// **Bound: this is the write door, not the cascade.** A value hand-written into
/// `.jigc/config/manifest.yaml`, or supplied by a team layer or a pack default, reaches the
/// readers unfolded exactly as before — the readers' private folds still find the right *home*,
/// but the rendered path is the layer's spelling. Every value jigc itself writes comes through
/// here.
fn normalize_root_value(value: &str) -> String {
    use std::ffi::OsStr;
    use std::path::Component;

    if Path::new(value).is_absolute() {
        return value.to_owned();
    }
    let parent = OsStr::new("..");
    let mut parts: Vec<&OsStr> = Vec::new();
    for component in Path::new(value).components() {
        match component {
            // A leading or interior `./` is the same path; it names nothing to keep.
            Component::CurDir => continue,
            Component::ParentDir => match parts.last() {
                // Pop the component the hop cancels.
                Some(last) if *last != parent => {
                    parts.pop();
                }
                // Nothing left to cancel: the value climbs above the repository root. Keep the
                // hop so step 2b refuses the path the operator named.
                _ => parts.push(parent),
            },
            Component::Normal(part) => parts.push(part),
            // Unreachable for a relative value; returning it untouched keeps the sibling
            // rules' messages quoting what was typed.
            Component::RootDir | Component::Prefix(_) => return value.to_owned(),
        }
    }
    parts
        .iter()
        .map(|part| part.to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Whether `value` names **jigc's own workbench** — the repo-root `.jigc/` directory itself,
/// or a path inside it — and is therefore no home for either root knob to point managed docs
/// at (M50 Increment 4 / T2).
///
/// The value is **normalized lexically first** — a `./` folded, a `..` applied against the
/// components accumulated so far — and only then is its first component asked. Asking the value
/// *as typed* is what the first cut did, and it left the rule evadable by one hop: at `6551d49`
/// the three literal spellings were refused (`config.workbench-root`, rc=1, both knobs) while
/// `jigc config set placement-root docs/../.jigc` exited **0** and staged
/// `R docs/decisions-log.md -> .jigc/decisions-log.md`, because git normalizes the path it
/// records even when this predicate does not. The subject is the destination the value
/// *reaches*, never the prefix it is typed with. Both sibling root rules already fold `..`
/// ([`unusable_root_reason`] pops on `Component::ParentDir`; [`crate::trackable`] resolves the
/// value before it asks), which is why this predicate was the only evadable one.
///
/// Only the **first normalized** component is asked, because the workbench is exactly one
/// directory: every door computes it as `repo_root.join(".jigc")`. A nested `docs/.jigc` is
/// somebody else's directory that happens to share the name, and refusing it would refuse a
/// home jigc has no claim on. By the same token a value that climbs back **out** of the
/// workbench (`.jigc/../docs`) reaches an ordinary home and is not refused here — this rule
/// is about where the value lands. A value that climbs above the repository root (`../.jigc`)
/// does not reach *this* repo's workbench either, and step 2b has already refused it as a path
/// git cannot record.
///
/// **What answers for the rest of it is [`normalize_root_value`], not "the two sibling root
/// rules"** — the claim this comment carried until M50's validation drove it. `.jigc/../docs`
/// does reach an ordinary home, and neither sibling rule had anything to say about a value that
/// reaches the home the docs are *already at*: it was admitted as a re-point and the move floor
/// destroyed the store surface (the driven repro is on [`normalize_root_value`]). The value now
/// arrives here already folded, so this predicate sees `docs` and the question does not come up;
/// the private fold below stays because the predicate must be correct about its own subject
/// whoever calls it.
///
/// The comparison is case-**insensitive**, and that is driven rather than defensive: on a
/// case-insensitive filesystem `jigc config set placement-root .JIGC` stages
/// `docs/roadmap.md -> .JIGC/roadmap.md` while the bytes land in the real `.jigc/`, so the
/// index and the worktree name different directories for one file, permanently. It is the rule
/// [`crate::trackable`] already applies to `.git` for git's own reason
/// (`core.protectHFS`/`protectNTFS`), asked here of the one directory jigc owns.
pub(crate) fn is_workbench_root(value: &str) -> bool {
    use std::path::Component;

    let mut normalized: Vec<&std::ffi::OsStr> = Vec::new();
    for component in Path::new(value).components() {
        match component {
            // A leading (or interior) `./` is the same path; it names nothing to keep.
            Component::CurDir => continue,
            Component::ParentDir => {
                if normalized.pop().is_none() {
                    // Climbs above the repository root — whatever is up there, it is not this
                    // repo's workbench (and step 2b refuses the value before this is asked).
                    return false;
                }
            }
            Component::Normal(part) => normalized.push(part),
            // An absolute value: refused one step below as a root the store cannot describe,
            // and never this repo's workbench.
            Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    // The first component of the normalized value decides, in both directions.
    normalized
        .first()
        .is_some_and(|part| part.eq_ignore_ascii_case(".jigc"))
}

/// Why `value` is a root the store **cannot describe** — `None` when it can (M50 Increment 4 /
/// T3). Asked of both root knobs, of the value as typed, before anything moves.
///
/// **Six shapes now, and the list has grown three times by the same route** — one driven cell
/// at a time, each joining this predicate rather than minting a code of its own, because the
/// operator's fix is the same in every case: supply a different root
/// (`config.untrackable-root`'s five-reasons-one-code precedent). M50 shipped three; M51
/// Increment 1 / T5 added **edge whitespace** ([`whitespace_padded_component`]) and T6 added
/// **git pathspec magic** ([`pathspec_magic_root`]); M52 Increment 6 / T6 added
/// **unnameable** ([`over_name_ceiling_component`]) — each with its own driven repro in its own
/// doc-comment. Each shape was driven at HEAD before its guard existed and each left the store
/// lying in its own way (the repros are in `crates/cli/tests/root_knob_rules.rs` and
/// `crates/cli/tests/path_arg_occurrence_axis.rs`). The three original ones:
///
///   - **absolute** — the value is resolved against the repository root by everything
///     downstream ([`crate::trackable`] trims the leading `/`), so `/tmp/elsewhere` moves docs
///     to `tmp/elsewhere/` while `jigc config get` echoes the absolute form back. Purely
///     lexical, and asked first: an absolute value's components are not this repo's to walk.
///   - **through a symlink** — git records the link itself, never a path through it, so the
///     move stages `RD`: an index entry the worktree has no path for. The subject is **every
///     existing component of the value**, not its leaf — a symlinked *ancestor* (`linked/sub`)
///     reaches the identical state.
///   - **file-shaped** — an existing non-directory. Every move into it fails
///     (`File exists` / `Not a directory`) while the knob lands anyway, and the docs then vanish
///     from `jigc doc list` because the store resolves them to homes nothing is at. Its subject
///     is **every existing component of the value** too, for the same reason and not merely by
///     symmetry: a file-shaped *ancestor* (`README.md/sub`) reaches that identical state, since
///     the destination directory cannot be created *under* a file any more than the leaf itself
///     can be one. The two on-disk shapes are therefore asked in one walk, of one subject.
///
/// **The walk starts at `repo_root` and never canonicalizes it.** Only the value's own
/// components are probed: a repository legitimately sits under a symlinked ancestor (every
/// macOS temp corpus lives under `/var` → `private/var`), and canonicalizing the root — or
/// comparing the canonical form of the whole join against it — would refuse every root in every
/// such repo. A value with no named components (`""`, `.`) is the repo root itself and is
/// always usable; that is the flat layout both knobs document.
///
/// The walk itself is [`offending_component`], because its symlink leg is asked on its own by a
/// door whose subject is a **source file** (M51 Increment 1 / T1) — the two legs still travel one
/// pass over one subject; only the sentences they earn are the caller's.
///
/// **The three lexical legs are asked before the walk**, so a value refused twice over keeps
/// the stronger sentence: `/tmp/x  ` still answers *an absolute path*, `:(top)docs` answers
/// about the magic rather than about a directory that does not exist, and an unnameable
/// component answers about its length rather than about a directory the filesystem could not
/// have been asked for in the first place.
fn unusable_root_reason(repo_root: &Path, value: &str) -> Option<String> {
    if Path::new(value).is_absolute() {
        return Some(format!(
            "an absolute path — every door resolves a root against the repository root, so the \
             docs would move to `{}` while `jigc config get` reads back `{value}`",
            value.trim_start_matches('/'),
        ));
    }

    if let Some(shown) = whitespace_padded_component(value) {
        let as_read = if shown.trim().is_empty() {
            "unset".to_owned()
        } else {
            format!("`{}`", shown.trim())
        };
        return Some(format!(
            "`{shown}` carries whitespace at its edge — a root has to read back as itself, and \
             `jigc config get` renders this one indistinguishably from {as_read}, so the value \
             in force and the directory every managed doc is moved under cannot be told apart"
        ));
    }

    if let Some(clause) = pathspec_magic_root(value) {
        return Some(format!(
            "{clause} — every managed doc's path is built from this value, so the \
             `git add`/`git mv` that stages one would match a set of files nobody named \
             instead of the doc"
        ));
    }

    if let Some((shown, bytes)) = over_name_ceiling_component(value) {
        let ceiling = crate::cli::NAME_MAX_BYTES;
        return Some(format!(
            "`{shown}` is {bytes} bytes — a path component may carry at most {ceiling}, so \
             every directory the re-point has to create fails to be *named* while the knob \
             lands anyway, and the store then resolves its docs to homes nothing is at"
        ));
    }

    match offending_component(repo_root, value) {
        // Every named component walked and none of them refused — including the case of no
        // named component at all (`""`, `.`), which is the repo root and the flat layout both
        // knobs document.
        None => None,
        Some(OffendingComponent::Symlink(shown)) => Some(format!(
            "`{shown}` is a symlink — git records the link, not a path through it, so the \
             moved docs would stage as index entries the worktree has no path for (`RD`) \
             and `jigc doc list` would name a home git does not"
        )),
        Some(OffendingComponent::FileShaped(shown)) => Some(format!(
            "`{shown}` is a file, not a directory — every move into it fails while the knob \
             lands anyway, and the store then resolves its docs to homes nothing is at"
        )),
    }
}

/// The first component of `value` carrying **leading or trailing whitespace**, as its own
/// spelling — `None` when every component reads back as itself (M51 Increment 1 / T5, EC-27).
///
/// The whitespace leg of [`unusable_root_reason`], and the only one of its four that asks
/// nothing of the filesystem: the defect is on the **read** surface, not on disk. Driven at
/// `b9e13cf`, `jigc config set docs-root '   '` exited 0 and `jigc config get docs-root` then
/// printed `docs-root =      (project)` — a reading no operator can tell from unset — while
/// every managed doc would finalize under a directory literally named three spaces.
/// `'  x  '` is the same defect one step less total. [`normalize_root_value`] folds `.` and
/// `..` and never trims, so a whitespace run survives the fold as an ordinary
/// `Component::Normal` and reaches the store as a home.
///
/// **Edge whitespace, not any whitespace.** An interior space (`my notes`) reads back as
/// itself and names a directory the operator can see and type, so refusing it would refuse a
/// legitimate home; and a tab never arrives here at all — `engine::write::check_value`'s
/// control-character floor refuses it two steps earlier under `config.value-rejected`. What is
/// left is exactly the class that is invisible at an edge, which `str::trim` names (Unicode
/// `White_Space`, so a non-breaking space is caught with the ASCII one).
///
/// **The subject is every component, not the leaf** — the correction [`offending_component`]
/// already carries for its own two shapes, applied here from the start: `docs/ notes` names an
/// invisible directory exactly as `  x  ` does, and the component is what the message quotes
/// so the refusal points at the half of the value that is wrong.
fn whitespace_padded_component(value: &str) -> Option<String> {
    use std::path::Component;

    Path::new(value)
        .components()
        .find_map(|component| match component {
            Component::Normal(part) => {
                let shown = part.to_string_lossy();
                (shown.trim() != shown.as_ref()).then(|| shown.into_owned())
            }
            // `.`/`..` name no component to read back, and a root or prefix component belongs to
            // the absolute value its caller refuses one leg above.
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => None,
        })
}

/// The first component of `value` the filesystem **cannot name** — its own spelling and its
/// byte length — `None` when every component fits (M52 Increment 6 / T6;
/// `completions/artifacts/M52/settle-record.md` → D5.6, *A1-D5 both knobs*).
///
/// The nameability leg of [`unusable_root_reason`], and the third of its legs that asks
/// nothing of the filesystem. The three predicates a root knob had through rc.15 test
/// **escape** ([`crate::trackable::untrackable_reason`]), **ownership**
/// ([`is_workbench_root`]) and **shape** (the rest of this function) — never whether the
/// value can be named at all. Driven at the wave's baseline
/// (`completions/artifacts/M52/baseline-tokens.md` §2.5 rows 10–11), a 300-byte component:
///
/// ```text
/// $ jigc config set docs-root "$(python3 -c "print('a'*300)")"
/// config: set `docs-root` = `aaaa…` — written to `.jigc/config/`, uncommitted …     rc=0
/// $ jigc validate
/// validating the committed store at "…/repo": File name too long (os error 63)      rc=1
/// ```
///
/// — the knob landed and the **next read of the store** failed with a code-less, route-less
/// OS error. The placement knob reaches the same fault from the other side: since M52
/// Increment 5 / T8 closed the byte-loss half it now attempts the move, fails per doc and
/// rolls the batch back, so the operator gets two `config.repoint-failed` findings whose
/// route — *fix what this message names* — cannot be followed, because the value is the
/// problem and the corpus is not.
///
/// **So this leg is a door-position fix, not a data-loss fix**, and the position is the one
/// the sibling file-shaped reason already argues for in its own sentence: *every move into it
/// fails while the knob lands anyway*. It refuses before anything moves and before the knob
/// lands, and it joins `config.unusable-root` rather than minting a code, on that code's
/// five-reasons-one-code precedent.
///
/// **The subject is every component, not the leaf** — the correction [`offending_component`]
/// and [`whitespace_padded_component`] both already carry: `docs/<300 bytes>` is as unnameable
/// as the bare component is, and the message quotes the component so the refusal points at the
/// half of the value that is wrong.
///
/// **The ceiling is [`crate::cli::NAME_MAX_BYTES`] bare**, not the slug family's derived
/// [`crate::cli::SLUG_NAME_CEILING`]: that one reserves the `<type>:` prefix, the `.md`
/// extension and the atomic write's temp suffix, which are what jigc wraps around an identity
/// on its way to becoming a **filename**. A root's components become **directories**, and
/// nothing is appended to a directory name — driven, a component of exactly
/// [`crate::cli::NAME_MAX_BYTES`] bytes relocates both committed docs and stages `R`.
///
/// Bytes, not characters, because `NAME_MAX` is a byte bound: a 200-character root of
/// two-byte scalars is 400 bytes and the filesystem refuses it.
fn over_name_ceiling_component(value: &str) -> Option<(String, usize)> {
    use std::path::Component;

    Path::new(value)
        .components()
        .find_map(|component| match component {
            Component::Normal(part) => {
                let bytes = part.as_encoded_bytes().len();
                (bytes > crate::cli::NAME_MAX_BYTES)
                    .then(|| (part.to_string_lossy().into_owned(), bytes))
            }
            // `.`/`..` name no component the filesystem has to create, and a root or prefix
            // component belongs to the absolute value the first leg refuses.
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => None,
        })
}

/// The clause naming the **git pathspec magic** a component of `value` carries — `None` when
/// every component is the literal directory name it looks like (M51 Increment 1 / T6, widened
/// from the leading-`:` spelling to the class by the increment's own validation).
///
/// The pathspec-magic leg of [`unusable_root_reason`], and the second of its legs that asks
/// nothing of the filesystem. A root knob prefixes **every** managed doc's path, so its first
/// component is the first component of every pathspec the movers and `finalize` hand git — and
/// `git add -- <path>` prevents **option** parsing, never magic: a path beginning `:(top)` or
/// `:!` is a pathspec, not a file name.
///
/// Driven at `be8ca6d`, before this leg: `jigc config set docs-root ':!docs'` exited **0**, a
/// task authored an `adr`, and `jigc task finalize` reported `1 file committed` while the
/// promoted doc — written to `:!docs/decisions/probe.md` on disk — matched the exclude
/// pathspec and reached **no commit**; the store went on calling it managed. The `:(top)`
/// spelling is the sharper half: the stage would match the *same-named file at the repository
/// top*, committing a file nobody named.
///
/// **Two positions, because git has two magics.** The `:` prefix is read only at the **start**
/// of a pathspec, so it is asked of the **first named component** alone — a directory named
/// `:x` deeper inside the value is an ordinary literal component and refusing it would refuse a
/// legitimate home. Wildmatch (`*`, `?`, `[`, `\`) carries no position rule at all: a pattern
/// byte in *any* component patterns the whole pathspec, which is why every component is asked
/// [`crate::trackable::wildmatch_magic_reason`]. Shipping only the first half is what left
/// `jigc config set docs-root 'd*cs'` admissible — every promoted doc's `git add` pathspec a
/// pattern — while its sibling spelling was refused.
///
/// The rule itself is [`crate::trackable`]'s, shared with the migrate source door and the
/// retire sink: one home, so the three places a caller token can enter a git argv cannot get
/// different answers about the same bytes. The *sentence* stays the caller's, because this
/// value **prefixes** the pathspec rather than being one.
fn pathspec_magic_root(value: &str) -> Option<String> {
    use std::path::Component;

    let mut first = true;
    Path::new(value)
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            // `./` names no component; a `..` hop, a root or a prefix belongs to the legs that
            // refuse those shapes, and none of them can begin a pathspec with `:`.
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => None,
        })
        .find_map(|part| {
            let reason = if first {
                crate::trackable::pathspec_magic_reason(&part)
            } else {
                crate::trackable::wildmatch_magic_reason(&part)
            };
            first = false;
            reason
        })
}

/// The two on-disk shapes an **existing component** of a relative value can have that a door
/// refuses it for — the one walk [`unusable_root_reason`]'s symlink and file-shaped legs are
/// both asked in, carrying the offending component's repo-relative spelling.
///
/// It is an enum rather than two predicates because the two shapes are asked of one subject in
/// one pass (see [`unusable_root_reason`]'s own account of why), while the *sentences* they earn
/// are the caller's: a root knob refuses both, and a door whose subject is a **source file**
/// refuses only the symlink — a source's leaf is legitimately a non-directory, which is the
/// whole point of it ([`symlinked_component`], M51 Increment 1 / T1).
enum OffendingComponent {
    Symlink(String),
    FileShaped(String),
}

/// The first existing component of `value` (resolved against `repo_root`) that is a **symlink**,
/// as its repo-relative spelling — `None` when no component of the value is one.
///
/// The symlink leg of [`unusable_root_reason`], asked on its own by a door whose subject is a
/// **file** rather than a home, where the sibling file-shaped leg would refuse every legitimate
/// value ([`crate::trackable::resolve_source_token`]).
pub(crate) fn symlinked_component(repo_root: &Path, value: &str) -> Option<String> {
    match offending_component(repo_root, value) {
        Some(OffendingComponent::Symlink(shown)) => Some(shown),
        Some(OffendingComponent::FileShaped(_)) | None => None,
    }
}

/// Walk every named component of `value` against `repo_root`, answering with the first one that
/// exists on disk in one of the two refused shapes.
///
/// **The walk starts at `repo_root` and never canonicalizes it** — see
/// [`unusable_root_reason`]'s doc for why (a repository legitimately sits under a symlinked
/// ancestor). Both on-disk shapes are asked of **this** component before moving to the next, so
/// the subject of each is every existing component of the value rather than its leaf; symlink
/// first, because a link *to* a file is git's problem before it is the mover's.
fn offending_component(repo_root: &Path, value: &str) -> Option<OffendingComponent> {
    use std::path::Component;

    let mut probe = repo_root.to_path_buf();
    let mut shown = String::new();
    for component in Path::new(value).components() {
        match component {
            // A leading `./` is the same path, and `..` is `crate::trackable`'s question
            // (it either stays inside the repo or is refused there) — neither names a
            // component to probe.
            Component::CurDir => continue,
            Component::ParentDir => {
                probe.pop();
                shown.clear();
                continue;
            }
            Component::Normal(part) => {
                probe.push(part);
                if !shown.is_empty() {
                    shown.push('/');
                }
                shown.push_str(&part.to_string_lossy());
            }
            // A root or prefix component cannot appear in a relative value, and an absolute one
            // is its caller's question, asked before this walk.
            Component::RootDir | Component::Prefix(_) => return None,
        }
        if std::fs::symlink_metadata(&probe).is_ok_and(|meta| meta.is_symlink()) {
            return Some(OffendingComponent::Symlink(shown.clone()));
        }
        if std::fs::metadata(&probe).is_ok_and(|meta| !meta.is_dir()) {
            return Some(OffendingComponent::FileShaped(shown.clone()));
        }
    }
    None
}

/// Narrate a relocation line on the **side channel** — and withhold it under `--format json`,
/// where the one document is the whole of what the door says
/// (`design/command-output-contract.md` → Stream discipline; M52 Increment 1 / T1).
///
/// **Why withholding is the fold and not a loss.** Prose beside a machine document is bytes a
/// driver cannot read: it either breaks the parse or is skipped, and until M51 minted
/// [`ConfigAck::Set`]'s `relocated` key a `--format json` re-point that moved the committed
/// corpus serialized byte-identically to one that moved nothing. That key is this narration's
/// carrier, so on the JSON arm the moves are *in* the document rather than beside it.
///
/// **Declared residual, stated rather than discovered.** Two of this narration's lines have no
/// such carrier — the move that *failed* (`could not relocate …`) and the foreign file parked
/// out of a destination — so under `--format json` they are withheld and named nowhere. They
/// are not findings today (minting a `config.*` code is M52 Increment 5's rollback work, and
/// this task registers no code), and they were unreadable to a driver before this change too:
/// what moves is that stderr now parses, not what a driver could act on.
fn narrate(format: Format, line: String) {
    if format != Format::Json {
        eprintln!("{line}");
    }
}

/// On a `docs-root` re-point to `new_value`, **detect + route + move** the committed docs the
/// change would strand under the prior resolved root — the surface-and-move resolution that
/// replaces the M36 warn-then-strand (`design/storage.md` → docs-root; `design/reconciliation.md`
/// → the route home; M39 inc-5 T3).
///
/// A `docs-root` re-point has a **recorded prior home** — the old resolved root, deterministic
/// from the current cascade — so auto-move is *safe* here (unlike a freeze-exempt doctype, which
/// carries no prior-home snapshot and whose relocation resolution is human-supplied — T4). Each
/// stranded doc is relocated to its new resolved home via the shared move primitive
/// ([`crate::relocate::move_doc`] — `git mv` + file-state re-key) and the move is surfaced. The
/// destination is a whole-path prefix swap old-root → new-root: `docs-root` is a uniform prefix
/// over every location doctype, so the doc's sub-path (`<location>/<slug>.md`) is invariant.
///
/// Best-effort: a resolution/store-access hiccup returns silently, and a per-doc move failure is
/// surfaced (routing the operator to move it by hand) but never fails the write — the set lands.
///
/// **Returns the moves that landed**, for [`ConfigAck::Set`]'s `relocated` key (M51 Increment 5 /
/// T4). Every silent-return path above yields the empty list, and a per-doc failure is excluded
/// by construction — it is pushed only on the `Ok` arm, so the ack can never name a `to` no file
/// is at (the key's own declared bound).
fn route_docs_root_repoint_orphans(
    pack: &dyn PackSource,
    format: Format,
    project_config: &Path,
    new_value: &str,
    undo: &mut crate::relocate::MoveRollback,
    failures: &mut Vec<RepointBlocker>,
) -> Vec<Relocated> {
    let mut relocated: Vec<Relocated> = Vec::new();
    let Ok(resolved) = crate::start::resolve_severity_cascade(pack, project_config) else {
        return relocated;
    };
    let old_docs_root = crate::start::docs_root_prefix(&resolved).to_string();
    let defs = crate::start::CascadeDefs::new(&resolved, project_config);
    let Ok(old_schemas) = defs.all_schemas(pack) else {
        return relocated;
    };
    // `<repo>/.jigc/config` → the repo root the committed docs (and `git ls-files`) live at.
    let Some(repo_root) = project_config.parent().and_then(Path::parent) else {
        return relocated;
    };
    let stranded = crate::orphan::docs_root_would_orphan(
        repo_root,
        old_schemas.values(),
        &old_docs_root,
        new_value,
    );
    if stranded.is_empty() {
        return relocated;
    }
    let new_root = normalize_docs_root(new_value);
    let jigc_root = repo_root.join(".jigc");
    // The solicit/act honesty pair (round-2 D2+D3): name the sweep's basis and the git
    // state the moves land in (each is a staged `git mv`, committed by the operator's
    // next commit, never here).
    //
    // The basis is `orphan::docs_root_would_orphan`'s subject, stated as the code walks
    // it (M51 Increment 9 / T5, EC-16): every committed doc under a doctype's prior
    // resolved `location:` directory, managed or not — committed truth, deliberately
    // index-blind so a fresh clone still relocates. It is NOT "every committed doc under
    // the prior resolved root": a `placement` doctype has `location: None` and homes at
    // its declared `placement.file`, resolved through `placement-root`, so `docs/roadmap.md`
    // sits under the old root and is never carried. The exclusion is stated positively,
    // so the reader gets the rule rather than an unexplained gap.
    narrate(
        format,
        format!(
            "relocating {} committed doc(s) stranded by the `docs-root` re-point to \
             `{new_value}` (every committed doc under a doctype's prior resolved `location:` \
             directory, managed or not — a placement doctype's file is not carried: it homes \
             at its declared `placement.file`, which resolves through `placement-root`; each \
             move is a staged `git mv` — commit it with your next commit):",
            stranded.len(),
        ),
    );
    for old_rel in &stranded {
        let Some(new_rel) = reroot(old_rel, &old_docs_root, &new_root) else {
            continue;
        };
        // A pure relocation preserves the bytes, so the re-keyed hash is the current file's.
        let Ok(bytes) = std::fs::read(repo_root.join(old_rel)) else {
            continue;
        };
        let new_hash = engine::file_state::hash_bytes(&bytes);
        // `git mv` needs the destination directory to exist (a re-point to a fresh root creates
        // dirs that never existed).
        if let Some(parent) = Path::new(&new_rel).parent() {
            let _ = std::fs::create_dir_all(repo_root.join(parent));
        }
        // The undo set's capture, **before** the move: the destination as it stands now, and
        // (once) the file-state record this batch re-keys (M52 Increment 5 / T8). A capture
        // that cannot read what it is about to overwrite ends this doc's move rather than
        // moving what it could not put back.
        if let Err(err) = undo.before_move(repo_root, &jigc_root, &new_rel) {
            failures.push(RepointBlocker::Cause {
                at: old_rel.clone(),
                cause: format!("{err:#}"),
            });
            continue;
        }
        match crate::relocate::move_doc(repo_root, &jigc_root, old_rel, &new_rel, &new_hash) {
            Ok(()) => {
                undo.landed(repo_root, old_rel, &new_rel, bytes);
                narrate(format, format!("  - {old_rel} → {new_rel}"));
                relocated.push(Relocated {
                    from: old_rel.clone(),
                    to: new_rel,
                });
            }
            // A move that failed is a **transaction** failure now, not a routed note beside a
            // knob that lands anyway: the store would otherwise point at a home this doc is
            // not at. The narration it replaces said `move it by hand`, which the rollback
            // makes false — the whole re-point is undone, and the finding's route says so.
            Err(err) => failures.push(RepointBlocker::Cause {
                at: old_rel.clone(),
                cause: format!("{err:#}"),
            }),
        }
    }
    relocated
}

/// On a `placement-root` re-point to `new_value`, **detect + route + move** the committed
/// instances the change would strand at each placement doctype's prior home — the placement
/// sibling of [`route_docs_root_repoint_orphans`] (`design/storage.md` → Placement /
/// docs-root; `design/reconciliation.md` → the route home / Relocation collisions).
///
/// A `placement-root` re-point has a **recorded prior home** — the declared `placement.file`
/// re-rooted under the *old* resolved value, deterministic from the current cascade — so
/// auto-move is safe here for exactly the reason it is on the `docs-root` sibling, and unsafe
/// on the freeze-exempt path (which carries no prior-home snapshot and takes the human's
/// `--from`).
///
/// **Both homes are computed from the DECLARED `placement.file`** — through the one shared
/// [`crate::start::reroot_placement_file`] rule — never by re-rooting the prior *resolved*
/// home, which the seam already produced. The two look
/// interchangeable and are not: once the root is `.` the resolved home has no leading
/// directory component left, so re-rooting it under the new value yields the same path — a
/// destination equal to the source, a silent no-op, and the doc left stranded at the repo
/// root. That is the exact silent-loss shape this floor exists to prevent.
///
/// The move itself delegates to [`crate::relocate::relocate_stranded`], which carries the
/// [`crate::relocate::move_doc`] primitive (`git mv` + file-state re-key) **and** the
/// foreign-squatter displacement into the gitignored `.jigc/displaced/` workbench that the
/// `docs-root` loop, calling `move_doc` directly, does not have — so a working file at the
/// new home is parked out of the way, never clobbered.
///
/// Best-effort: a resolution/store-access hiccup returns silently, and a per-doc failure is
/// surfaced (routing the operator to move it by hand) but never fails the write. A
/// **repository-posture** refusal is surfaced the same way and additionally **ends the
/// sweep** — it is a fact about the repository, so every remaining pair would meet it
/// ([`crate::relocate::relocate_stranded`], M52 Increment 3 / T4).
///
/// **Returns the moves that landed**, on the same rule as its `docs-root` sibling (M51
/// Increment 5 / T4): every silent return yields the empty list, a `blocked` per-doc failure is
/// excluded, and so is a **displaced** foreign squatter — that is a file parked out of the way,
/// not a managed doc rehomed, and [`ConfigAck::Set`]'s key names relocations.
fn route_placement_root_repoint_strands(
    pack: &dyn PackSource,
    format: Format,
    project_config: &Path,
    new_value: &str,
    undo: &mut crate::relocate::MoveRollback,
    failures: &mut Vec<RepointBlocker>,
) -> Vec<Relocated> {
    let mut relocated: Vec<Relocated> = Vec::new();
    let Ok(resolved) = crate::start::resolve_severity_cascade(pack, project_config) else {
        return relocated;
    };
    let old_root = crate::start::placement_root(&resolved);
    let new_root = crate::start::normalize_placement_root(new_value);
    if old_root == new_root {
        return relocated; // the same home for every doctype — nothing can be stranded.
    }
    let defs = crate::start::CascadeDefs::new(&resolved, project_config);
    let Ok(declared) = defs.declared_schemas(pack) else {
        return relocated;
    };
    // `<repo>/.jigc/config` → the repo root the committed docs (and `git ls-files`) live at.
    let Some(repo_root) = project_config.parent().and_then(Path::parent) else {
        return relocated;
    };
    // Each placement doctype whose home actually moves, as a (prior, current) home pair —
    // both sides re-rooted from the one declaration.
    let pairs: Vec<(Home, Home)> = declared
        .values()
        .filter_map(|schema| {
            let file = &schema.placement.as_ref()?.file;
            let prior = crate::start::reroot_placement_file(file, old_root);
            let current = crate::start::reroot_placement_file(file, new_root);
            (prior != current).then(|| (Home::placement(&prior), Home::placement(&current)))
        })
        .collect();
    let committed = orphan::committed_markdown(repo_root);
    let strands = pairs.iter().any(|(prior, current)| {
        committed
            .iter()
            .any(|rel| orphan::is_stranded(rel, prior, current))
    });
    if !strands {
        return relocated;
    }
    // The solicit/act honesty pair the `docs-root` sibling prints: name the sweep's basis
    // (every committed doc at a placement doctype's prior home, managed or not — committed
    // truth, deliberately index-blind so a fresh clone still relocates) and the git state the
    // moves land in (each is a staged `git mv`, committed by the operator's next commit).
    //
    // The two sweeps partition the store by home kind and neither is the universal: this one
    // carries the placement homes, the `docs-root` sibling the `location:` directories. A doc
    // under the old `docs-root` that is a placement instance belongs to this sweep only.
    narrate(
        format,
        format!(
            "relocating the committed doc(s) stranded by the `placement-root` re-point to \
             `{new_value}` (every committed doc at a placement doctype's prior home, managed \
             or not; each move is a staged `git mv` — commit it with your next commit):"
        ),
    );
    let jigc_root = repo_root.join(".jigc");
    for (prior, current) in &pairs {
        // A refusal about the **repository** — an operation the user has not concluded,
        // opened after this door's own posture verdict — ends the sweep rather than joining
        // the per-doc `blocked` rows: every remaining pair meets the identical state
        // (`crate::relocate::relocate_stranded`).
        let report = match crate::relocate::relocate_stranded(
            repo_root,
            &jigc_root,
            prior,
            current,
            Some(undo),
        ) {
            Ok(report) => report,
            // It ends the sweep **and** the transaction (M52 Increment 5 / T8): a refusal
            // about the repository leaves the moves that landed standing against a knob that
            // must not now land either. It is carried with its own identity rather than
            // re-worded, because a code inside a message is not a key.
            Err(err) => {
                failures.push(match crate::render::blocked_finding(&err) {
                    Some(finding) => RepointBlocker::Raised(finding.clone()),
                    None => RepointBlocker::Cause {
                        at: crate::render::repo_relative(
                            repo_root,
                            &project_config.join("manifest.yaml"),
                        ),
                        cause: format!("{err:#}"),
                    },
                });
                break;
            }
        };
        for (from, to) in &report.moved {
            narrate(format, format!("  - {from} → {to}"));
            relocated.push(Relocated {
                from: from.clone(),
                to: to.clone(),
            });
        }
        for (from, to) in &report.displaced {
            narrate(
                format,
                format!(
                    "  - {from} → {to} (a foreign file at the new home, parked out of the way)"
                ),
            );
        }
        // The `docs-root` sibling's rule, one floor over: a doc this sweep could not relocate
        // ends the transaction instead of narrating `move it by hand` beside a knob that
        // lands anyway.
        for (path, reason) in &report.blocked {
            failures.push(RepointBlocker::Cause {
                at: path.clone(),
                cause: reason.clone(),
            });
        }
    }
    relocated
}

/// Normalize a raw `docs-root` value to [`crate::start::apply_docs_root`]'s rule: strip
/// surrounding slashes; the flat forms (`""` / `.`) canonicalize to `""` (the repo-root layout).
/// The raw-string sibling of [`crate::start::docs_root_prefix`] (which normalizes a *resolved*
/// value) — needed because `new_value` is not yet in the cascade.
fn normalize_docs_root(value: &str) -> String {
    let trimmed = value.trim_matches('/');
    if trimmed == "." { "" } else { trimmed }.to_string()
}

/// Reroot a stranded committed path from `old_root` to `new_root` (both normalized): strip the
/// leading `<old_root>/` and prepend `<new_root>/`. `docs-root` is a uniform prefix over every
/// location doctype, so this whole-path prefix swap yields the doc's new resolved home. `None`
/// when the path lacks the expected old-root prefix (defensive — every stranded doc carries it).
fn reroot(rel: &str, old_root: &str, new_root: &str) -> Option<String> {
    let sub = if old_root.is_empty() {
        rel
    } else {
        rel.strip_prefix(&format!("{old_root}/"))?
    };
    Some(if new_root.is_empty() {
        sub.to_string()
    } else {
        format!("{new_root}/{sub}")
    })
}

/// Adjudicate the `<file>` argument of `insert-step` / `replace-step` **before the read and
/// before any write** (M51 Increment 1 / T4; `completions/artifacts/M51/settle-record.md` →
/// §2 · the §10 mold; `design/overrides.md` → Authoring deltas).
///
/// Both verbs took the token as an opaque path: `fs::read(file)`, `file_stem()`, and the bytes
/// landed at `.jigc/config/steps/<stem>.yaml` — an in-repo, committable file that **composes
/// into the step text `jigc start` hands the agent**. Driven at `dddc11a5`:
/// `jigc config insert-step … .git/config` exits **0** and copies this repository's git config
/// in. That the copy then renders `repositoryformatversion = 0` into that step text is the
/// baseline's own drive (`completions/artifacts/M51/baseline-tokens.md` §2d).
///
/// The rule itself is [`crate::trackable::source_read_reason`] — one home, both occurrences of
/// the `file` argument, which is why the M51 path-argument registry is keyed by *occurrence*
/// rather than by deduplicated argument id. What is door-local is the code and the sentence:
/// **one** code for both legs, with the reason in the message
/// (`config.untrackable-root`'s five-reasons-one-code precedent — the operator's fix is the
/// same either way: name a different file), and a **`Human`** route, because no `jigc` argv
/// resolves this state; only naming a different source does.
///
/// The consequence clause belongs to the door, not to the predicate: the predicate states
/// where the bytes are, and this door is the one that knows what it was about to do with them
/// (`crate::trackable`'s own leg-shared-but-sentence-local rule, from T1).
fn adjudicate_step_source(cwd: &Path, project_config: &Path, file: &Path) -> Result<()> {
    // The repo root is the project layer's grandparent (`<repo>/.jigc/config`), the same
    // derivation the two root-knob rules use. A layer with no grandparent is not a shape any
    // door reaches — `require_project_layer` built this path — and abstaining is the
    // conservative answer if one ever does.
    let Some(repo_root) = project_config.parent().and_then(Path::parent) else {
        return Ok(());
    };
    match crate::trackable::source_read_reason(cwd, repo_root, file) {
        None => Ok(()),
        Some(reason) => Err(finding_to_err(Finding::block(
            "config.step-source-untrackable",
            format!(
                "{reason}, and a step source is copied verbatim into \
                 `.jigc/config/steps/` and composed into the step text every `jigc start` \
                 renders"
            ),
            engine::finding::Route::human(
                "name a step source outside git's own directory and outside jigc's transient \
                 workbench (`.jigc/tasks/`, `.jigc/state/` and their siblings) — a file \
                 anywhere else, in this repository or not, is copied in verbatim and lands in \
                 your next diff",
            ),
        ))),
    }
}

/// `jigc config insert-step --workflow <id> (--after|--before) <step-id> <file>` —
/// record an `insert-step` `structural-op` + write the native step file
/// (`design/overrides.md` → Authoring deltas; `design/worked-examples.md` → 3a).
///
/// The native step takes its **id from the source file's basename** and is written
/// to `.jigc/config/steps/<basename>.yaml`; the delta references `step:<basename>`.
/// Both write-time checks run *before any write* so a rejection leaves the tree
/// untouched (`overrides.md` → Write-time vs resolve-time split):
/// 1. read the source file + derive its basename;
/// 2. `check_basename_collision` — the basename must be a fresh step id else reject;
/// 3. `check_anchor_present` — the anchor must be in the workflow's resolved include
///    list *as of this edit* else reject;
/// 4. write `steps/<basename>.yaml` (the source bytes) + append the `insert-step`
///    delta to the project manifest.
///
/// `after`/`before` are clap-guaranteed mutually exclusive and exactly-one-present,
/// so the anchor is unambiguous here.
fn run_insert_step(
    cwd: &Path,
    workflow: &str,
    after: Option<&str>,
    before: Option<&str>,
    file: &Path,
) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;

    // Adjudicate `<file>` BEFORE the read and before the basename derivation: the bytes this
    // reads are copied into the cascade and composed into step text, so a source the door has
    // not adjudicated must never be opened at all.
    adjudicate_step_source(cwd, &project_config, file)?;

    // The native step's id is the source file's basename (`overrides.md` →
    // Native-file id = filename basename).
    let basename = file
        .file_stem()
        .and_then(|s| s.to_str())
        .with_context(|| format!("source file {} has no basename", file.display()))?
        .to_owned();
    let source = std::fs::read(file)
        .with_context(|| format!("could not read source step file {}", file.display()))?;

    // The anchor step id + side — clap guarantees exactly one of `--after`/`--before`.
    let (anchor_id, anchor, side) = match (after, before) {
        (Some(id), None) => (id, Anchor::After(id.to_owned()), "after"),
        (None, Some(id)) => (id, Anchor::Before(id.to_owned()), "before"),
        // clap's `conflicts_with` + `required_unless_present` make both/neither
        // unreachable in practice.
        _ => bail!("insert-step needs exactly one of `--after` / `--before`"),
    };

    // Both write-time checks run before any write — a rejection touches nothing.
    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (_layer, existing_deltas, _slot_fills, _forks, _bases) =
        crate::start::load_project_layer(&project_config)?;
    check_basename_collision(pack, &project_config, &basename).map_err(finding_to_err)?;
    check_anchor_present(pack, workflow, &existing_deltas, anchor_id).map_err(finding_to_err)?;

    // Write the native step file, then append the `insert-step` delta.
    let steps_dir = project_config.join("steps");
    std::fs::create_dir_all(&steps_dir)
        .with_context(|| format!("could not create {}", steps_dir.display()))?;
    let native = steps_dir.join(format!("{basename}.yaml"));
    std::fs::write(&native, &source)
        .with_context(|| format!("could not write {}", native.display()))?;
    let delta = StructuralDelta::Insert {
        target: StructuralTarget {
            workflow_id: workflow.to_owned(),
            anchor,
        },
        step: basename.clone(),
    };
    append_delta(&project_config, &delta)?;
    Ok(ConfigAck::InsertStep {
        workflow: workflow.to_owned(),
        step: basename,
        side,
        anchor: anchor_id.to_owned(),
    })
}

/// `jigc config replace-step <workflow:id#step-id> <file>` — record a `replace`
/// `structural-op` + write the native step file (`design/overrides.md` → Authoring
/// deltas; `design/worked-examples.md` → 3a).
///
/// The replacement step takes its **id from the source file's basename** and is
/// written to `.jigc/config/steps/<basename>.yaml`; the delta references
/// `step:<basename>`. Both write-time checks run *before any write* so a rejection
/// leaves the tree untouched (`overrides.md` → Write-time vs resolve-time split):
/// 1. parse the `workflow:<id>#<step-id>` target ([`Anchor::At`]);
/// 2. read the source file + derive its basename;
/// 3. `check_basename_collision` — the basename must be a fresh step id else reject;
/// 4. `check_anchor_present` — the target step-id must be in the workflow's resolved
///    include list *as of this edit* else reject;
/// 5. write `steps/<basename>.yaml` (the source bytes) + append the `replace-step`
///    delta to the project manifest.
fn run_replace_step(cwd: &Path, target: &str, file: &Path) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;
    let parsed = StructuralTarget::parse(target, None).map_err(finding_to_err)?;
    let step_id = at_step(&parsed);

    // The same adjudication as `insert-step`, at the sibling occurrence of the `file`
    // argument — asked before the read, for the same reason.
    adjudicate_step_source(cwd, &project_config, file)?;

    let basename = file
        .file_stem()
        .and_then(|s| s.to_str())
        .with_context(|| format!("source file {} has no basename", file.display()))?
        .to_owned();
    let source = std::fs::read(file)
        .with_context(|| format!("could not read source step file {}", file.display()))?;

    // Both write-time checks run before any write — a rejection touches nothing.
    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (_layer, existing_deltas, _slot_fills, _forks, _bases) =
        crate::start::load_project_layer(&project_config)?;
    check_basename_collision(pack, &project_config, &basename).map_err(finding_to_err)?;
    check_anchor_present(pack, &parsed.workflow_id, &existing_deltas, &step_id)
        .map_err(finding_to_err)?;

    // The base-hash basis is the **displaced** pack unit — the target step `step_id`
    // being swapped out — read pack-direct (the same `pack.read(Steps, …)` basis
    // `config fork` records), never the replacement file (`overrides.md` → Per-kind
    // base-hash basis; the replace row). Resolved before any write.
    let displaced = resolve_fork_bytes(pack, &step_id)?;

    // Write the native step file, then append the `replace-step` delta carrying the
    // displaced unit's recorded basis (`base-version` + `base-hash`).
    let steps_dir = project_config.join("steps");
    std::fs::create_dir_all(&steps_dir)
        .with_context(|| format!("could not create {}", steps_dir.display()))?;
    let native = steps_dir.join(format!("{basename}.yaml"));
    std::fs::write(&native, &source)
        .with_context(|| format!("could not write {}", native.display()))?;
    let delta = StructuralDelta::Replace {
        target: parsed,
        step: basename.clone(),
    };
    append_delta_entry(
        &project_config,
        with_basis(delta_to_yaml(&delta), &pack.pack_version(), &displaced),
    )?;
    Ok(ConfigAck::ReplaceStep {
        target: target.to_owned(),
        step: basename,
    })
}

/// `jigc config remove-step <workflow:id#step-id>` — record a `remove`
/// `structural-op` (`design/overrides.md` → Authoring deltas). No native file —
/// a remove adds nothing.
///
/// The single write-time check (`overrides.md` → Write-time vs resolve-time split):
/// the target step-id must be in the workflow's resolved include list *as of this
/// edit* else reject. On pass, append the `remove-step` delta to the project
/// manifest.
fn run_remove_step(cwd: &Path, target: &str) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;
    let parsed = StructuralTarget::parse(target, None).map_err(finding_to_err)?;
    let step_id = at_step(&parsed);

    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (_layer, existing_deltas, _slot_fills, _forks, _bases) =
        crate::start::load_project_layer(&project_config)?;
    check_anchor_present(pack, &parsed.workflow_id, &existing_deltas, &step_id)
        .map_err(finding_to_err)?;

    // The base-hash basis is the **removed** pack unit — the target step `step_id`
    // being dropped — read pack-direct (the same `pack.read(Steps, …)` basis
    // `config fork`/`replace-step` record) (`overrides.md` → Per-kind base-hash
    // basis; the remove row). Resolved before the append.
    let removed = resolve_fork_bytes(pack, &step_id)?;

    let delta = StructuralDelta::Remove { target: parsed };
    append_delta_entry(
        &project_config,
        with_basis(delta_to_yaml(&delta), &pack.pack_version(), &removed),
    )?;
    Ok(ConfigAck::RemoveStep {
        target: target.to_owned(),
    })
}

/// `jigc config fill <step:id#fill-id> --from-file <path|->` — record a `slot-fill`
/// delta + write the native fill file (`design/overrides.md` → Authoring deltas, the
/// `config fill` row; `design/worked-examples.md` → 3c).
///
/// The content arrives via stdin / `--from-file` (prose, never inline) and is written
/// to `.jigc/config/fills/<fill-id>.md` (id = fill-id); the delta references
/// `step:<id>#<fill-id>` + `content: fills/<fill-id>.md`. Both write-time checks run
/// *before any write* so a rejection leaves the tree untouched (`overrides.md` →
/// Write-time vs resolve-time split):
/// 1. parse the `step:<id>#<fill-id>` target;
/// 2. read the content handoff;
/// 3. `check_content_no_nested` — the content must declare no `{{fill:}}` point (the
///    no-nested rule: phase 5 does not re-run) else reject;
/// 4. `check_fill_point_present` — the `{{fill:<fill-id>}}` point must exist in the
///    **resolved** step body (project shadow included) else reject;
/// 5. write `fills/<fill-id>.md` (the content bytes) + append the `slot-fill` delta.
fn run_fill(cwd: &Path, target: &str, from_file: &str) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;
    let parsed = SlotFillTarget::parse(target).map_err(finding_to_err)?;
    // `from_file` occurrence 1 of 3. Its path-rule disposition — **no rule**, with the reason
    // — is stated once at [`crate::doc::read_handoff`]. Note the asymmetry with this module's
    // own `<file>` argument two verbs up: that one takes the source rule, because `-` is not
    // stdin there.
    let content = crate::doc::read_handoff(from_file)?;

    // Both write-time checks run before any write — a rejection touches nothing.
    let pack = make_pack()?;
    let pack = pack.as_ref();
    check_content_no_nested(&content).map_err(finding_to_err)?;
    check_fill_point_present(pack, &project_config, &parsed).map_err(finding_to_err)?;

    // Write the native fill file (id = fill-id), then append the `slot-fill` delta.
    let fills_dir = project_config.join("fills");
    std::fs::create_dir_all(&fills_dir)
        .with_context(|| format!("could not create {}", fills_dir.display()))?;
    let native = fills_dir.join(format!("{}.md", parsed.fill_id));
    std::fs::write(&native, content.as_bytes())
        .with_context(|| format!("could not write {}", native.display()))?;
    append_slot_fill(&project_config, &parsed)?;
    Ok(ConfigAck::Fill {
        target: format!("step:{}#{}", parsed.step_id, parsed.fill_id),
    })
}

/// `jigc config fork <workflow:id#step-id>` — record a `tracked-fork` delta + copy
/// the resolved unit's body into a native step file that shadows it
/// (`design/overrides.md` → Authoring deltas, the `config fork` row; `tracked-fork`
/// hash basis).
///
/// The forked unit is the **step** the `#<step-id>` names ([`Anchor::At`]); its
/// resolved body bytes are the **post-shadow, pre-expansion** bytes phase 2 would
/// load. All write-time checks run *before any write* so a rejection leaves the tree
/// untouched (`overrides.md` → Write-time vs resolve-time split):
/// 1. parse the `workflow:<id>#<step-id>` target ([`Anchor::At`]) → the step id;
/// 2. `check_basename_collision` — the step id must not already be project-shadowed
///    (a unit already forked is a collision) else reject;
/// 3. `check_anchor_present` — the target step-id must resolve in the workflow's
///    include list *as of this edit* else reject (an unknown step id);
/// 4. resolve the step's body bytes (the pack-default owner after the collision guard);
/// 5. write `steps/<step-id>.yaml` (the copied bytes) + append the `tracked-fork`
///    delta recording `base-version` = the pack version and `base-hash` = the blake3
///    of the **copied** bytes (the pinned basis M5 reads).
fn run_fork(cwd: &Path, target: &str) -> Result<ConfigAck> {
    let project_config = require_project_layer(cwd)?;
    let parsed = StructuralTarget::parse(target, None).map_err(finding_to_err)?;
    let step_id = at_step(&parsed);

    // Both write-time checks run before any write — a rejection touches nothing. The
    // already-forked guard (the step id is already project-shadowed) runs before the
    // anchor check, and both before any byte resolution. Unlike insert/replace, the
    // forked id *is* a pack step id by design (a fork copies a pack unit), so the
    // collision is only with an existing **project** shadow — not the pack id.
    let pack = make_pack()?;
    let pack = pack.as_ref();
    let (_layer, existing_deltas, _slot_fills, _forks, _bases) =
        crate::start::load_project_layer(&project_config)?;
    check_not_already_forked(&project_config, &step_id).map_err(finding_to_err)?;
    check_anchor_present(pack, &parsed.workflow_id, &existing_deltas, &step_id)
        .map_err(finding_to_err)?;

    // The resolved unit is the step the `#<step-id>` names. After the collision guard,
    // no project file shadows this id, so the resolved owner is the pack — its body
    // bytes are the post-shadow, pre-expansion bytes phase 2 would load.
    let bytes = resolve_fork_bytes(pack, &step_id)?;

    // Write the native step file (id = step-id), then append the `tracked-fork` delta.
    // The recorded `base-hash` is the blake3 of the **copied** bytes (not a re-read).
    let steps_dir = project_config.join("steps");
    std::fs::create_dir_all(&steps_dir)
        .with_context(|| format!("could not create {}", steps_dir.display()))?;
    let native = steps_dir.join(format!("{step_id}.yaml"));
    std::fs::write(&native, &bytes)
        .with_context(|| format!("could not write {}", native.display()))?;
    let delta = TrackedForkDelta {
        target: parsed,
        base_version: pack.pack_version(),
        base_hash: engine::file_state::hash_bytes(&bytes),
    };
    append_fork(&project_config, &delta)?;
    // The blake3 prefix is the pinned basis a later upgrade reconciliation reads back.
    let base = delta.base_hash.chars().take(12).collect::<String>();
    Ok(ConfigAck::Fork {
        target: target.to_owned(),
        path: format!(".jigc/config/steps/{step_id}.yaml"),
        base,
    })
}

/// Reject a `config fork` of a unit the project layer **already shadows** — the
/// already-forked write-time guard (`design/overrides.md` → the `jigc config` verbs:
/// "target resolves as of this edit"; a re-fork would re-shadow what is already a
/// native file).
///
/// Unlike [`check_basename_collision`] (insert/replace, where the new basename must be
/// a *fresh* id), a fork's id is a **pack** step id by design — a fork copies a pack
/// unit — so only an existing **project** shadow (`<project_config>/steps/<id>.yaml`,
/// [`crate::start::project_step_ids`]) is a collision: the unit was already forked. A
/// clash is a routed blocking `config.step-id-collision` [`Finding`]; an un-forked id
/// passes.
fn check_not_already_forked(project_config: &Path, step_id: &str) -> Result<(), Finding> {
    if crate::start::project_step_ids(project_config)
        .iter()
        .any(|id| id == step_id)
    {
        return Err(Finding::block(
            "config.step-id-collision",
            format!(
                "`{step_id}` is already forked — the project layer shadows it; re-forking would re-shadow an existing native file"
            ),
            "edit the existing native file (or remove it first) instead of re-forking, then re-run",
        ));
    }
    Ok(())
}

/// Read the resolved body bytes of the step `step_id` to fork — the pack-default
/// owner's bytes after [`run_fork`]'s collision guard has ruled out a project shadow
/// of this id (`design/overrides.md` → `tracked-fork` hash basis: the resolved native
/// step file bytes). A step id the pack does not own is a routed `config.anchor-absent`
/// [`Finding`] (symmetric with the anchor check that precedes this read).
fn resolve_fork_bytes(pack: &dyn PackSource, step_id: &str) -> Result<Vec<u8>> {
    pack.read(PackResourceKind::Steps, &ResourceId::from(step_id))
        .map_err(|_| {
            finding_to_err(Finding::block(
                "config.anchor-absent",
                format!("no step `{step_id}` body to fork"),
                "name a step id present in the workflow's resolved include list, then re-run",
            ))
        })
}

/// Append one `tracked-fork` delta to `<project_config>/manifest.yaml`'s `deltas:`
/// list, preserving any existing top-level keys (the `scalar:` map) and prior entries
/// — the fork sibling of [`append_delta`] (`design/overrides.md` → Delta
/// representation; `design/storage.md` → Config layout: one `deltas:` list, every
/// kind).
///
/// The on-disk spelling mirrors what [`crate::start::load_project_layer`]'s
/// `parse_one_delta` reads back for a `tracked-fork` kind:
/// - `kind: tracked-fork`;
/// - `target: workflow:<id>#<step-id>` (the [`Anchor::At`] [`StructuralTarget`]);
/// - `base-version:` — the pack version the fork was taken from;
/// - `base-hash:` — the blake3 of the copied unit bytes (the pinned basis M5 reads).
fn append_fork(project_config: &Path, delta: &TrackedForkDelta) -> Result<()> {
    let entry = {
        use serde_yaml_ng::Value as Yaml;
        let mut map = serde_yaml_ng::Mapping::new();
        let mut put = |k: &str, v: String| {
            map.insert(Yaml::String(k.to_owned()), Yaml::String(v));
        };
        put("kind", "tracked-fork".to_owned());
        put(
            "target",
            format!(
                "workflow:{}#{}",
                delta.target.workflow_id,
                at_step(&delta.target)
            ),
        );
        put("base-version", delta.base_version.clone());
        put("base-hash", delta.base_hash.clone());
        Yaml::Mapping(map)
    };
    append_delta_entry(project_config, entry)
}

/// Reject fill content that itself declares a `{{fill:}}` point — the no-nested
/// write-time check (`design/overrides.md` → The `{{fill:}}` placeholder: "The
/// `config fill` verb rejects fill content containing `{{fill:}}` at write time").
///
/// Phase 5 ([`engine::compose::apply_slot_fills`]) does not re-run, so a `{{fill:}}`
/// inside applied content would survive to phase 8 unresolved (the `fill-survivor`
/// block). Catching it at write time gives the human an immediate error. The
/// recognizer is the engine's own [`engine::compose::fill_ids_in`] — the same
/// lone-`{{fill:}}` rule the phase-5 pass and the resolution-time checks use. A clean
/// content body passes.
fn check_content_no_nested(content: &str) -> Result<(), Finding> {
    if let Some(fill_id) = engine::compose::fill_ids_in(content).first() {
        return Err(Finding::block(
            "config.nested-fill",
            format!(
                "fill content declares a nested `{{{{fill: {fill_id}}}}}` point — phase 5 does not re-run, so a nested fill would survive composition unresolved"
            ),
            "remove the nested `{{fill:}}` from the content (fill content may not contain another fill point), then re-run",
        ));
    }
    Ok(())
}

/// Reject a `slot-fill` whose `{{fill:<fill-id>}}` point **no resolved step body
/// declares** — the write-time point-presence check (`design/overrides.md` → the
/// `jigc config` verbs: "the `{{fill:<fill-id>}}` point exists in the resolved step
/// body").
///
/// The snapshot is the target step's **resolved** body
/// ([`crate::start::resolve_step_body`] — project shadow included), scanned for the
/// declared `{{fill:}}` points by the engine's [`engine::compose::fill_ids_in`]. A
/// fill-id absent from that step's points — or a step no layer owns — is a routed
/// blocking `config.fill-point-absent` [`Finding`] (symmetric with the resolution-time
/// `slot-fill-orphan`, closing the closed-surface hole at write time); a declared
/// point passes.
///
/// A pure write-time check (`overrides.md` → Write-time vs resolve-time split): it
/// validates *this* edit's point against the current resolved body; whole-cascade
/// consequences (a later structural delta that drops the step) stay at resolution time
/// through `workflow-refs`.
pub(crate) fn check_fill_point_present(
    pack: &dyn PackSource,
    project_config: &Path,
    target: &SlotFillTarget,
) -> Result<(), Finding> {
    let absent = || {
        Finding::block(
            "config.fill-point-absent",
            format!(
                "`step:{}#{}` is not a `{{{{fill:}}}}` point in the resolved `{}` step body",
                target.step_id, target.fill_id, target.step_id
            ),
            "name a `{{fill:<fill-id>}}` point the step body declares (run `jigc start` to see the composed step bodies), then re-run",
        )
    };
    let def = crate::start::resolve_step_body(pack, project_config, &target.step_id)
        .map_err(|err| {
            Finding::block(
                "config.fill-point-absent",
                format!(
                    "could not resolve the `{}` step body to check its `{{{{fill:}}}}` points: {err:#}",
                    target.step_id
                ),
                "name an existing step whose body declares the `{{fill:<fill-id>}}` point, then re-run",
            )
        })?
        .ok_or_else(absent)?;
    if engine::compose::fill_ids_in(&def.body)
        .iter()
        .any(|id| *id == target.fill_id)
    {
        Ok(())
    } else {
        Err(absent())
    }
}

/// Locate the repo root and its `.jigc/config/` project layer, erroring with the
/// same routed messages [`run_set`] / [`run_insert_step`] use when the repo or the
/// project layer is absent. The shared preamble of every `config` verb.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    // The project cascade layer lives at `<jigc_home>/.jigc/config` (the main checkout),
    // so a worktree resolves the one shared layer (M31 Inc 2 / WF3). No git read here.
    crate::start::require_project_config(cwd)
}

/// Record `scalar.<key> = <value>` into `<project_config>/manifest.yaml`,
/// preserving any existing top-level keys (the `deltas:` list) and `scalar:`
/// entries. A missing manifest is created with a single-entry `scalar:` block.
///
/// The manifest is round-tripped through `serde_yaml_ng::Value` so the loader
/// (`crate::start::load_project_layer`) reads back exactly the `scalar:` map this
/// writes — the one manifest both the write and read paths agree on.
fn write_scalar(project_config: &Path, key: &str, value: &str) -> Result<()> {
    use serde_yaml_ng::Value as Yaml;

    let manifest = project_config.join("manifest.yaml");
    let mut doc: Yaml = match std::fs::read_to_string(&manifest) {
        Ok(text) => serde_yaml_ng::from_str(&text)
            .with_context(|| format!("{} is not valid YAML", manifest.display()))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Yaml::Mapping(Default::default()),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };

    // A present-but-non-mapping (or null/empty) manifest becomes a fresh map so the
    // `scalar:` block has a home.
    let map = match &mut doc {
        Yaml::Mapping(map) => map,
        _ => {
            doc = Yaml::Mapping(Default::default());
            let Yaml::Mapping(map) = &mut doc else {
                unreachable!("just set to a mapping")
            };
            map
        }
    };

    // Locate or create the `scalar:` sub-map, then set `<key> = <value>`.
    let scalar_key = Yaml::String("scalar".to_owned());
    let scalar = map
        .entry(scalar_key)
        .or_insert_with(|| Yaml::Mapping(Default::default()));
    if !scalar.is_mapping() {
        *scalar = Yaml::Mapping(Default::default());
    }
    let Yaml::Mapping(scalar_map) = scalar else {
        unreachable!("just ensured a mapping")
    };
    scalar_map.insert(Yaml::String(key.to_owned()), Yaml::String(value.to_owned()));

    let rendered = serde_yaml_ng::to_string(&doc)
        .with_context(|| format!("could not serialize {}", manifest.display()))?;
    std::fs::write(&manifest, rendered)
        .with_context(|| format!("could not write {}", manifest.display()))?;
    Ok(())
}

/// Append one `structural-op` delta to `<project_config>/manifest.yaml`'s
/// `deltas:` list, preserving any existing top-level keys (the `scalar:` map) and
/// prior `deltas:` entries. A missing manifest is created; a missing/blank
/// `deltas:` block becomes a fresh list (`design/overrides.md` → Delta
/// representation; `design/storage.md` → Config layout).
///
/// The on-disk spelling mirrors what [`crate::start::load_project_layer`] reads
/// back (`design/overrides.md` → Delta targets / the `jigc config` verbs):
/// - `kind:` — `insert-step` / `replace-step` / `remove-step`;
/// - `target:` — `workflow:<id>` for an insert (the anchor rides `after:`/`before:`),
///   `workflow:<id>#<step-id>` for a replace/remove ([`Anchor::At`]);
/// - `after:`/`before:` — the insert anchor step id;
/// - `with:` — `step:<basename>`, the native step file's id (insert/replace only).
///
/// The manifest is round-tripped through `serde_yaml_ng::Value` so the loader
/// reads back exactly the `deltas:` list this writes (the preserve discipline
/// [`write_scalar`] uses for the `scalar:` sub-map, mirrored for `deltas:`).
fn append_delta(project_config: &Path, delta: &StructuralDelta) -> Result<()> {
    append_delta_entry(project_config, delta_to_yaml(delta))
}

/// Push one already-rendered `deltas:` entry mapping onto
/// `<project_config>/manifest.yaml`'s `deltas:` list, preserving any existing
/// top-level keys (the `scalar:` map) and prior entries — the manifest-preserving
/// round-trip both [`append_delta`] (structural-op) and [`append_slot_fill`] share
/// (the one `deltas:` list carries every delta kind, `storage.md` → Config layout).
fn append_delta_entry(project_config: &Path, entry: serde_yaml_ng::Value) -> Result<()> {
    use serde_yaml_ng::Value as Yaml;

    let manifest = project_config.join("manifest.yaml");
    let mut doc: Yaml = match std::fs::read_to_string(&manifest) {
        Ok(text) => serde_yaml_ng::from_str(&text)
            .with_context(|| format!("{} is not valid YAML", manifest.display()))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Yaml::Mapping(Default::default()),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };

    // A present-but-non-mapping (or null/empty) manifest becomes a fresh map so the
    // `deltas:` list has a home.
    let map = match &mut doc {
        Yaml::Mapping(map) => map,
        _ => {
            doc = Yaml::Mapping(Default::default());
            let Yaml::Mapping(map) = &mut doc else {
                unreachable!("just set to a mapping")
            };
            map
        }
    };

    // Locate or create the `deltas:` sequence, then push the new entry.
    let deltas_key = Yaml::String("deltas".to_owned());
    let deltas = map
        .entry(deltas_key)
        .or_insert_with(|| Yaml::Sequence(Vec::new()));
    if !deltas.is_sequence() {
        *deltas = Yaml::Sequence(Vec::new());
    }
    let Yaml::Sequence(list) = deltas else {
        unreachable!("just ensured a sequence")
    };
    list.push(entry);

    let rendered = serde_yaml_ng::to_string(&doc)
        .with_context(|| format!("could not serialize {}", manifest.display()))?;
    std::fs::write(&manifest, rendered)
        .with_context(|| format!("could not write {}", manifest.display()))?;
    Ok(())
}

/// Append one `slot-fill` delta to `<project_config>/manifest.yaml`'s `deltas:` list,
/// preserving any existing top-level keys (the `scalar:` map) and prior `deltas:`
/// entries — the slot-fill sibling of [`append_delta`] (`design/overrides.md` → Delta
/// representation; `design/storage.md` → Config layout: one `deltas:` list, every
/// kind).
///
/// The on-disk spelling mirrors what [`crate::start::load_project_layer`]'s
/// `parse_one_delta` reads back for a `slot-fill` kind:
/// - `kind: slot-fill`;
/// - `target: step:<id>#<fill-id>` (the [`SlotFillTarget`]);
/// - `content: fills/<fill-id>.md` (the native fill file, id = fill-id).
fn append_slot_fill(project_config: &Path, target: &SlotFillTarget) -> Result<()> {
    let entry = {
        use serde_yaml_ng::Value as Yaml;
        let mut map = serde_yaml_ng::Mapping::new();
        let mut put = |k: &str, v: String| {
            map.insert(Yaml::String(k.to_owned()), Yaml::String(v));
        };
        put("kind", "slot-fill".to_owned());
        put(
            "target",
            format!("step:{}#{}", target.step_id, target.fill_id),
        );
        put("content", format!("fills/{}.md", target.fill_id));
        Yaml::Mapping(map)
    };
    append_delta_entry(project_config, entry)
}

/// Reject a new native-step **basename** that collides with an existing step id —
/// the `insert-step` / `replace-step` write-time collision check (`design/overrides.md`
/// → Native-file id = filename basename: "A name collision with an existing step id
/// is a write-time error").
///
/// The native step a verb registers takes its id from the source file's basename, so
/// a basename equal to a step id already in the resolution would silently shadow it
/// (phase-2 by-id shadowing) instead of adding a new unit. The collision set is the
/// **pack** step ids ([`PackResourceKind::Steps`]) ∪ the **project-shadowed** step
/// ids (`<project_config>/steps/<id>.yaml` basenames — [`crate::start::project_step_ids`]),
/// the two layers a new basename could clash with. A clash is a routed blocking
/// `config.step-id-collision` [`Finding`]; a fresh basename passes.
///
/// A pure write-time check: it reads the resolution *as of this edit* (the snapshot),
/// not the whole-cascade consequences, which stay at resolution time
/// (`overrides.md` → Write-time vs resolve-time split).
pub(crate) fn check_basename_collision(
    pack: &dyn PackSource,
    project_config: &Path,
    basename: &str,
) -> Result<(), Finding> {
    let collides = pack
        .list(PackResourceKind::Steps)
        .iter()
        .any(|id| id.as_str() == basename)
        || crate::start::project_step_ids(project_config)
            .iter()
            .any(|id| id == basename);
    if collides {
        return Err(Finding::block(
            "config.step-id-collision",
            format!(
                "`{basename}` is already a step id — the native file's id is its basename, so this would shadow the existing step, not add a new one"
            ),
            "rename the source file so its basename is a fresh step id, then re-run",
        ));
    }
    Ok(())
}

/// Reject a `replace-step` / `remove-step` `#<target>` (or an `insert-step`
/// `--after` / `--before` `<anchor>`) that is **absent** from the workflow's include
/// list *as of this edit* — the write-time anchor/target presence check
/// (`design/overrides.md` → the `jigc config` verbs: "anchor step-id present in the
/// resolution as of this edit").
///
/// The snapshot is the workflow's include list after the **existing** scoped deltas
/// apply ([`engine::compose::apply_structural_deltas`] over
/// [`load_workflow_def`](engine::compose::load_workflow_def)'s `includes`), so it
/// reflects a prior same-manifest `remove-step` that already dropped the id — an
/// anchor a prior remove retired is absent here and rejected. An absent target is a
/// routed blocking `config.anchor-absent` [`Finding`]; a present one passes.
///
/// A pure write-time check (`overrides.md` → Write-time vs resolve-time split): it
/// validates *this* edit's anchor against the current snapshot; whole-cascade
/// consequences (cycles, a later delta orphaning an earlier one) stay at resolution
/// time through `workflow-refs`.
pub(crate) fn check_anchor_present(
    pack: &dyn PackSource,
    workflow_id: &str,
    existing_deltas: &[StructuralDelta],
    anchor: &str,
) -> Result<(), Finding> {
    let bytes = crate::start::read_workflow(pack, workflow_id).map_err(|err| {
        Finding::block(
            "config.anchor-absent",
            format!("no workflow `{workflow_id}` to anchor against: {err:#}"),
            "name an existing workflow id, then re-run",
        )
    })?;
    let def = engine::compose::load_workflow_def(&bytes)?;
    let scoped = crate::start::scoped_deltas(workflow_id, existing_deltas);
    let snapshot = engine::compose::apply_structural_deltas(&def.includes, &scoped)?;
    if !snapshot.iter().any(|id| id == anchor) {
        return Err(Finding::block(
            "config.anchor-absent",
            format!(
                "`{anchor}` is not a step in `{workflow_id}` as of this edit — its include list is: {}",
                snapshot.join(", ")
            ),
            "name a step id present in the workflow's resolved include list, then re-run",
        ));
    }
    Ok(())
}

/// Render one [`StructuralDelta`] to its manifest-entry mapping — the inverse of
/// [`crate::start::load_project_layer`]'s `parse_one_delta`. The `with:` step id
/// is written with the `step:` prefix (the manifest spelling the loader strips).
fn delta_to_yaml(delta: &StructuralDelta) -> serde_yaml_ng::Value {
    use serde_yaml_ng::Value as Yaml;

    let mut entry = serde_yaml_ng::Mapping::new();
    let mut put = |k: &str, v: String| {
        entry.insert(Yaml::String(k.to_owned()), Yaml::String(v));
    };
    let wf = |id: &str| format!("workflow:{id}");
    match delta {
        StructuralDelta::Insert { target, step } => {
            put("kind", "insert-step".to_owned());
            put("target", wf(&target.workflow_id));
            match &target.anchor {
                Anchor::After(id) => put("after", id.clone()),
                Anchor::Before(id) => put("before", id.clone()),
                Anchor::At(id) => put("after", id.clone()),
            }
            put("with", format!("step:{step}"));
        }
        StructuralDelta::Replace { target, step } => {
            put("kind", "replace-step".to_owned());
            put(
                "target",
                format!("{}#{}", wf(&target.workflow_id), at_step(target)),
            );
            put("with", format!("step:{step}"));
        }
        StructuralDelta::Remove { target } => {
            put("kind", "remove-step".to_owned());
            put(
                "target",
                format!("{}#{}", wf(&target.workflow_id), at_step(target)),
            );
        }
    }
    Yaml::Mapping(entry)
}

/// Add the M5 base-hash basis to a `replace-step` / `remove-step` manifest entry —
/// the `base-version:` (the pack version) + `base-hash:` (the blake3 of the
/// **displaced pack unit's** bytes) keys, written onto the *same* entry
/// [`delta_to_yaml`] rendered, the same pack-direct basis `config fork` records
/// (`design/overrides.md` → Per-kind base-hash basis; On-disk representation).
///
/// The basis rides as net-new keys on the entry (not on the compose-facing
/// [`StructuralDelta`], whose `delta_to_yaml` spelling is untouched), so phase-4
/// compose and its byte-identical goldens never see it; T1's loader reads the pair
/// back **Optional** into a separate target-keyed [`engine::cascade::StructuralBasis`].
fn with_basis(
    entry: serde_yaml_ng::Value,
    base_version: &str,
    displaced: &[u8],
) -> serde_yaml_ng::Value {
    use serde_yaml_ng::Value as Yaml;
    let mut entry = entry;
    if let Yaml::Mapping(map) = &mut entry {
        map.insert(
            Yaml::String("base-version".to_owned()),
            Yaml::String(base_version.to_owned()),
        );
        map.insert(
            Yaml::String("base-hash".to_owned()),
            Yaml::String(engine::file_state::hash_bytes(displaced)),
        );
    }
    entry
}

/// The `#<step-id>` step a replace/remove target names (`Anchor::At`). Insert
/// targets never reach here; an `After`/`Before` on a replace/remove is a writer
/// bug, so the id is still emitted rather than silently dropped.
fn at_step(target: &StructuralTarget) -> String {
    match &target.anchor {
        Anchor::At(id) | Anchor::After(id) | Anchor::Before(id) => id.clone(),
    }
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its **key** + message +
/// route — the `severity · code — message` line shape the findings envelope prints,
/// the discipline `crate::milestone`'s funnel already applies (round-2 D6h: a finding
/// surfaced through the operational-error funnel used to drop its stable `(code,
/// target)` key, leaving it undiscriminable to a driver).
///
/// The `config` family needed it at M48 Inc 6: `config.undeclared-key` is now raised
/// from **two** verbs (`get` and `set`), so the code is what tells a driver which
/// rejection it met — and it costs nothing to carry it for the family's other codes.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-config-delta-{tag}-{}-{:?}",
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

    fn insert_after(workflow: &str, anchor: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Insert {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::After(anchor.to_owned()),
            },
            step: step.to_owned(),
        }
    }

    fn replace_at(workflow: &str, at: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(at.to_owned()),
            },
            step: step.to_owned(),
        }
    }

    /// A repo (`.git`) with a seeded `.jigc/config/` project layer — the shape the
    /// `run_*` verbs discover + require. Returns the repo `TempDir` and its layer path.
    fn repo_with_layer(tag: &str) -> (TempDir, PathBuf) {
        let repo = TempDir::new(tag);
        fs::create_dir_all(repo.path().join(".git")).expect("mk .git");
        let project_config = repo.path().join(".jigc").join("config");
        fs::create_dir_all(&project_config).expect("mk project layer");
        fs::write(
            project_config.join("manifest.yaml"),
            "scalar:\n  default-workflow: single-task\n",
        )
        .expect("seed manifest");
        (repo, project_config)
    }

    fn agent_ack(ack: &ConfigAck) -> String {
        crate::render::config_ack(crate::cli::Format::Agent, ack)
    }

    /// Assert `line` opens with the verb's `config: <effect>` statement. This test owns
    /// the **effect** half of the ack; the closing uncommitted-write clause every
    /// variant now carries is owned — over the whole `ConfigAck::ALL` axis, in both
    /// surfaces — by `crates/cli/tests/config_ack_uncommitted.rs`, so it is deliberately
    /// not re-spelled here.
    fn assert_effect(line: &str, effect: &str) {
        assert!(
            line.starts_with(effect),
            "the ack opens with its effect `{effect}`; got:\n{line}"
        );
    }

    /// F1 (M43 surface census, Law 1) — every `config` write returns an effect-stating
    /// ack instead of silence. Each verb, run against a real embedded-pack cascade,
    /// renders its `config: <effect>` line.
    #[test]
    fn config_verbs_ack_their_effect() {
        // set — the scalar-set effect.
        {
            let (repo, _cfg) = repo_with_layer("ack-set");
            let ack = run_set(repo.path(), Format::Agent, "default-workflow", "quick-fix")
                .expect("set records");
            assert_effect(
                &agent_ack(&ack),
                "config: set `default-workflow` = `quick-fix`",
            );
        }
        // insert-step — the splice effect (step id + workflow + side + anchor).
        {
            let (repo, _cfg) = repo_with_layer("ack-insert");
            let source = repo.path().join("team-extra.yaml");
            fs::write(&source, "team extra body\n").expect("write source step");
            let ack = run_insert_step(repo.path(), "single-task", Some("implement"), None, &source)
                .expect("insert-step records");
            assert_effect(
                &agent_ack(&ack),
                "config: inserted step `team-extra` into `single-task` after `implement`",
            );
        }
        // replace-step — the swap effect (target + replacement basename).
        {
            let (repo, _cfg) = repo_with_layer("ack-replace");
            let source = repo.path().join("project-implement.yaml");
            fs::write(&source, "{{ include: step:implement }}\n").expect("write source");
            let ack = run_replace_step(repo.path(), "workflow:single-task#implement", &source)
                .expect("replace-step records");
            assert_effect(
                &agent_ack(&ack),
                "config: replaced `workflow:single-task#implement` with `project-implement`",
            );
        }
        // remove-step — the drop effect (target).
        {
            let (repo, _cfg) = repo_with_layer("ack-remove");
            let ack = run_remove_step(repo.path(), "workflow:single-task#implement")
                .expect("remove-step records");
            assert_effect(
                &agent_ack(&ack),
                "config: removed `workflow:single-task#implement`",
            );
        }
        // fill — the inject effect (target). `implement` declares `{{fill: extra-guidance}}`.
        {
            let (repo, _cfg) = repo_with_layer("ack-fill");
            let content = repo.path().join("guidance.md");
            fs::write(&content, "extra project guidance\n").expect("write fill content");
            let ack = run_fill(
                repo.path(),
                "step:implement#extra-guidance",
                content.to_str().unwrap(),
            )
            .expect("fill records");
            assert_effect(
                &agent_ack(&ack),
                "config: filled `step:implement#extra-guidance`",
            );
        }
        // fork — the copy effect (target + native path + pinned base prefix).
        {
            let (repo, _cfg) = repo_with_layer("ack-fork");
            let ack =
                run_fork(repo.path(), "workflow:single-task#implement").expect("fork records");
            let line = agent_ack(&ack);
            assert_effect(
                &line,
                "config: forked `workflow:single-task#implement` -> \
                 .jigc/config/steps/implement.yaml (pinned base ",
            );
        }
    }

    /// F3 (M43 surface census, Law 2) — an undeclared `config set <key>` routes to the
    /// **verb that reveals the closed surface**. Until M48 Inc 6 the route had to *be*
    /// the enumeration, because nothing else in the binary enumerated the knobs; now
    /// `jigc config list` does, so the recovery is a command the reader can run rather
    /// than a knob list wedged into an error message.
    ///
    /// The rejection also carries its stable key — one finding, raised from two verbs
    /// (`get` and `set`), so the `code` is what tells a driver which one it met.
    #[test]
    fn undeclared_key_route_names_the_read_verb_that_enumerates_the_surface() {
        let (repo, _cfg) = repo_with_layer("undeclared");
        let err = run_set(repo.path(), Format::Agent, "not-a-knob", "x")
            .expect_err("an undeclared key rejects");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("config.undeclared-key"),
            "the rejection carries its stable key; got:\n{msg}"
        );
        assert!(
            msg.contains("route: run `jigc config list`"),
            "the route names the verb that enumerates the closed surface; got:\n{msg}"
        );
    }

    /// The `--format json` config ack is a parseable object, not empty success — each
    /// verb's effect is machine-readable (the mold every other write ack follows).
    #[test]
    fn config_ack_json_is_parseable() {
        let ack = ConfigAck::Set {
            key: "docs-root".to_owned(),
            value: "docs2".to_owned(),
            relocated: vec![crate::render::Relocated {
                from: "docs/decisions/x.md".to_owned(),
                to: "docs2/decisions/x.md".to_owned(),
            }],
            // A value the door did not fold — this unit asks what the JSON arm carries, and
            // the fold clause is text-only by declaration (`ConfigAck::Set`'s own field doc).
            folded_from: None,
        };
        let out = crate::render::config_ack(crate::cli::Format::Json, &ack);
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("json ack parses");
        assert_eq!(parsed["op"], "config-set");
        assert_eq!(parsed["key"], "docs-root");
        assert_eq!(parsed["value"], "docs2");
        // The relocation the re-point landed, as `{from, to}` (M51 Inc 5 / T4, EC-4).
        assert_eq!(parsed["relocated"][0]["from"], "docs/decisions/x.md");
        assert_eq!(parsed["relocated"][0]["to"], "docs2/decisions/x.md");
    }

    /// T1 done-criterion: append an insert-step then a replace-step into a temp
    /// manifest that already carries a `scalar:` entry; the round-tripped YAML
    /// carries both delta entries **and** the original `scalar:` entry, and
    /// `start::load_project_layer` parses the result back to the two expected
    /// [`StructuralDelta`]s — preserve discipline over the `deltas:` block, the
    /// inverse of the loader's `parse_one_delta` (`design/overrides.md` → Delta
    /// representation; `design/storage.md` → Config layout).
    #[test]
    fn append_delta_preserves_scalar_and_prior_deltas_and_round_trips() {
        let dir = TempDir::new("append");
        let project_config = dir.path();
        // Seed a manifest already carrying a `scalar:` entry (the writer must not
        // clobber it when it grows the `deltas:` list).
        fs::write(
            project_config.join("manifest.yaml"),
            "scalar:\n  default-workflow: single-task\n",
        )
        .expect("seed manifest");

        let insert = insert_after("single-task", "implement", "project-validate");
        let replace = replace_at("single-task", "validate", "project-validate");
        append_delta(project_config, &insert).expect("append insert-step");
        append_delta(project_config, &replace).expect("append replace-step");

        // The original `scalar:` entry survives the two appends.
        let text =
            fs::read_to_string(project_config.join("manifest.yaml")).expect("read manifest back");
        let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text).expect("manifest is YAML");
        assert_eq!(
            doc.get("scalar")
                .and_then(|s| s.get("default-workflow"))
                .and_then(serde_yaml_ng::Value::as_str),
            Some("single-task"),
            "the original scalar: entry must survive the delta appends\n{text}"
        );
        let count = doc
            .get("deltas")
            .and_then(serde_yaml_ng::Value::as_sequence)
            .map(Vec::len)
            .unwrap_or(0);
        assert_eq!(count, 2, "both delta entries must be present\n{text}");

        // The loader parses the written manifest back to the two deltas, in order.
        let (_layer, deltas, _slot_fills, _forks, _bases) =
            crate::start::load_project_layer(project_config).expect("loader parses the manifest");
        assert_eq!(
            deltas,
            vec![insert, replace],
            "load_project_layer must round-trip the two appended deltas in order\n{text}"
        );
    }

    /// An in-memory [`PackSource`] seeded from `(kind, id, bytes)` triples — drives
    /// the write-time checks over a fixture cascade with no embedded pack.
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
            self.0.get(&(kind, id.clone())).cloned().ok_or(
                engine::packsource::PackError::NotFound {
                    kind,
                    id: id.clone(),
                },
            )
        }
    }

    /// A `single-task` pack whose include list is `locate → implement → validate`,
    /// plus those three step files — the snapshot the anchor check resolves against.
    fn single_task_pack() -> FixturePack {
        FixturePack::with(vec![
            (
                PackResourceKind::Workflows,
                "single-task",
                "---\nwhen: implement one scoped change\ncreates-task: true\n---\n{{ include: step:locate }}\n{{ include: step:implement }}\n{{ include: step:validate }}\n",
            ),
            (PackResourceKind::Steps, "locate", "locate body\n"),
            (PackResourceKind::Steps, "implement", "implement body\n"),
            (PackResourceKind::Steps, "validate", "validate body\n"),
        ])
    }

    fn remove_at(workflow: &str, at: &str) -> StructuralDelta {
        StructuralDelta::Remove {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(at.to_owned()),
            },
        }
    }

    /// T2(a) — a new native-step basename equal to a **pack** step id (`implement`)
    /// is rejected (routed `config.step-id-collision`), and so is one equal to a
    /// **project-shadowed** id (a `steps/team-lint.yaml` file present in the project
    /// layer); a fresh basename (`team-extra`) passes. The native file's id is its
    /// basename, so a clash would shadow rather than add (`overrides.md` →
    /// Native-file id = filename basename).
    #[test]
    fn basename_collision_rejects_pack_and_project_step_ids_passes_fresh() {
        let pack = single_task_pack();
        let dir = TempDir::new("collision");
        let project_config = dir.path();
        // The project already shadows a `team-lint` step (its `steps/` dir is the
        // project step layer — `project_step_ids`).
        let steps = project_config.join("steps");
        fs::create_dir_all(&steps).expect("mk steps dir");
        fs::write(steps.join("team-lint.yaml"), "team lint body\n").expect("seed project step");

        // A basename equal to a pack step id is rejected.
        let pack_clash = check_basename_collision(&pack, project_config, "implement")
            .expect_err("a pack-step basename must collide");
        assert_eq!(pack_clash.code, "config.step-id-collision");
        assert!(
            pack_clash.route.is_some(),
            "the collision must carry a route"
        );

        // A basename equal to a project-shadowed id is rejected.
        let project_clash = check_basename_collision(&pack, project_config, "team-lint")
            .expect_err("a project-shadowed basename must collide");
        assert_eq!(project_clash.code, "config.step-id-collision");

        // A fresh basename passes.
        check_basename_collision(&pack, project_config, "team-extra")
            .expect("a fresh basename must pass");
    }

    /// T2(b) — an anchor/target **present** in the snapshot passes; an **absent** one
    /// is rejected (routed `config.anchor-absent`); and an anchor a prior same-manifest
    /// `remove-step` already dropped is rejected because the snapshot applies the
    /// existing deltas (`overrides.md` → the snapshot reflects prior deltas).
    #[test]
    fn anchor_present_in_snapshot_passes_absent_or_prior_removed_rejected() {
        let pack = single_task_pack();

        // Present in the pack include list, no prior deltas → passes.
        check_anchor_present(&pack, "single-task", &[], "implement")
            .expect("a present anchor must pass");

        // Never in the include list → rejected with its route.
        let absent = check_anchor_present(&pack, "single-task", &[], "nope")
            .expect_err("an absent anchor must be rejected");
        assert_eq!(absent.code, "config.anchor-absent");
        assert!(absent.route.is_some(), "the rejection must carry a route");

        // A prior `remove-step` dropped `implement`; the snapshot applies it, so
        // re-anchoring on `implement` is now rejected (snapshot reflects prior deltas).
        let prior = vec![remove_at("single-task", "implement")];
        let dropped = check_anchor_present(&pack, "single-task", &prior, "implement")
            .expect_err("an anchor a prior remove dropped must be rejected");
        assert_eq!(dropped.code, "config.anchor-absent");

        // A delta scoped to a *different* workflow must not affect this snapshot — a
        // remove of `implement` on `other` leaves `single-task`'s `implement` present.
        let other = vec![remove_at("other", "implement")];
        check_anchor_present(&pack, "single-task", &other, "implement")
            .expect("a delta scoped to another workflow must not drop this anchor");
    }

    /// T2 — `run_replace_step` / `run_remove_step` record the **displaced pack
    /// unit's** pack-direct basis on the appended manifest entry: `base-version` =
    /// the pack version, `base-hash` = an independently computed
    /// `hash_bytes(pack.read(Steps, "implement"))` — the same pack-direct way
    /// `config fork` records (`overrides.md` → Per-kind base-hash basis; the
    /// replace/remove rows). The entry round-trips back through T1's loader to a
    /// [`engine::cascade::StructuralBasis`] keyed by that `#<step-id>` target, leaving
    /// the compose-facing [`StructuralDelta`] untouched.
    #[test]
    fn replace_remove_record_displaced_pack_unit_basis_and_round_trip() {
        // The displaced pack unit is the target step `implement`; its basis is the
        // blake3 of its pack-default bytes, read pack-direct (independent of any
        // replacement file basename).
        let pack = crate::pack::EmbeddedPack::new();
        let displaced = pack
            .read(PackResourceKind::Steps, &ResourceId::from("implement"))
            .expect("pack ships an `implement` step");
        let expected_hash = engine::file_state::hash_bytes(&displaced);
        let expected_version = pack.pack_version();

        for verb in ["replace", "remove"] {
            // A repo root (`.git`) + the `.jigc/config/` project layer, since the
            // run_* helpers discover the repo and require the layer.
            let repo = TempDir::new(&format!("rr-basis-{verb}"));
            fs::create_dir_all(repo.path().join(".git")).expect("mk .git");
            let project_config = repo.path().join(".jigc").join("config");
            fs::create_dir_all(&project_config).expect("mk project layer");
            fs::write(
                project_config.join("manifest.yaml"),
                "scalar:\n  default-workflow: single-task\n",
            )
            .expect("seed manifest");

            let target = "workflow:single-task#implement";
            if verb == "replace" {
                // The replacement file's basename (`project-implement`) is a fresh
                // step id and re-includes the displaced step — the basis must still be
                // the *displaced* unit's bytes, not the replacement's.
                let source = repo.path().join("project-implement.yaml");
                fs::write(&source, "{{ include: step:implement }}\n").expect("write source");
                run_replace_step(repo.path(), target, &source).expect("replace-step records");
            } else {
                run_remove_step(repo.path(), target).expect("remove-step records");
            }

            // The appended entry carries the displaced pack unit's pack-direct basis.
            let text =
                fs::read_to_string(project_config.join("manifest.yaml")).expect("read manifest");
            let doc: serde_yaml_ng::Value =
                serde_yaml_ng::from_str(&text).expect("manifest is YAML");
            let entry = doc
                .get("deltas")
                .and_then(serde_yaml_ng::Value::as_sequence)
                .and_then(|s| s.first())
                .unwrap_or_else(|| panic!("{verb}: one delta entry\n{text}"));
            assert_eq!(
                entry
                    .get("base-version")
                    .and_then(serde_yaml_ng::Value::as_str),
                Some(expected_version.as_str()),
                "{verb}: base-version must equal the pack version\n{text}",
            );
            assert_eq!(
                entry
                    .get("base-hash")
                    .and_then(serde_yaml_ng::Value::as_str),
                Some(expected_hash.as_str()),
                "{verb}: base-hash must equal an independent pack-direct hash of the displaced unit\n{text}",
            );

            // The entry round-trips through T1's loader to a `StructuralBasis` for
            // that `#<step-id>` target — the structural delta itself unchanged.
            let (_layer, _deltas, _slot_fills, _forks, bases) =
                crate::start::load_project_layer(&project_config)
                    .expect("loader parses the recorded basis");
            assert_eq!(
                bases,
                vec![engine::cascade::StructuralBasis {
                    target: StructuralTarget {
                        workflow_id: "single-task".to_owned(),
                        anchor: Anchor::At("implement".to_owned()),
                    },
                    base_version: expected_version.clone(),
                    base_hash: expected_hash.clone(),
                }],
                "{verb}: the recorded basis must round-trip keyed by the target\n{text}",
            );
        }
    }

    /// **The home rule reads the destination, not the typed prefix** (M50 Increment 4 / T2,
    /// completed over its `..` axis).
    ///
    /// `crates/cli/tests/root_knob_rules.rs` drives the refusing half through the real binary
    /// at both knobs; this pins the predicate's *other* direction as well, which no corpus arm
    /// can reach cheaply — a value that merely contains `.jigc` somewhere, and a value that
    /// starts in the workbench and climbs back out, are both ordinary homes and must stay
    /// admitted, or the fold that closed the hop would have bought its refusals by refusing
    /// paths jigc has no claim on.
    #[test]
    fn workbench_home_rule_asks_where_the_value_lands() {
        // Reaches the workbench — as typed, and behind every hop shape.
        for value in [
            ".jigc",
            ".jigc/displaced",
            ".JIGC",
            "./.jigc",
            "docs/../.jigc",
            "./docs/../.jigc",
            "docs/../.jigc/displaced",
            "docs/../.JIGC",
            "notes/deep/../../.jigc",
        ] {
            assert!(
                is_workbench_root(value),
                "`{value}` reaches jigc's own workbench and is no home for a root knob",
            );
        }

        // Lands somewhere else — someone else's nested directory of the same name, a value
        // that climbs back out of the workbench, one that climbs above the repository
        // altogether (step 2b's question, not this one), an absolute path (step 2d's), and the
        // roots that carry no named component at all.
        for value in [
            "docs/.jigc",
            "docs/../docs/.jigc",
            ".jigc/../docs",
            ".jigc/displaced/../../notes",
            "../.jigc",
            "/tmp/.jigc",
            "",
            ".",
            "docs",
            "docs/../notes",
        ] {
            assert!(
                !is_workbench_root(value),
                "`{value}` does not land in jigc's own workbench — refusing it here would \
                 refuse a home jigc has no claim on, under a message that would not be true",
            );
        }
    }
}
