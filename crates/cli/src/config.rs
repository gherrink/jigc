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

use crate::invocation_log::Outcome;
use crate::orphan::{self, Home};
use crate::pack::make_pack;
use crate::render::{ConfigAck, KnobReading, RejectedSet};
use anyhow::{Context, Result, bail};
use engine::cascade::{
    Anchor, LayerKind, SlotFillTarget, StructuralDelta, StructuralTarget, TrackedForkDelta,
};
use engine::field_block::Value;
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The `jigc config <verb>` subcommand tree — `set`, the structural-op verbs
/// `insert-step` / `replace-step` / `remove-step`, the `fill` verb, and `fork`.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum ConfigCommand {
    /// Record a `scalar-set` delta — set a closed-surface knob in the project
    /// layer's `manifest.yaml` `scalar:` block. Adjudicated at write time against
    /// the pack's declared knob (`check_value`): an undeclared key or a wrong-type
    /// value is rejected non-zero with a routed finding. Setting a location knob
    /// (`docs-root`) also relocates the committed docs the re-point would strand
    /// at the prior resolved root — every committed doc under it, managed or not —
    /// as staged `git mv` moves, printed per file (they land with your next commit).
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
            ConfigCommand::Set { key, value } => {
                run_set(cwd, &key, &value).map(|ack| crate::render::config_ack(format, &ack))
            }
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
            Err(err) => {
                eprintln!("{}", crate::render::operational_error(format, &err));
                Outcome::failure()
            }
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
    let knobs_bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .context("the embedded pack is missing `config/knobs`")?;
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
fn run_set(cwd: &Path, key: &str, value: &str) -> Result<ConfigAck> {
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
    let value = if matches!(key, "docs-root" | "placement-root") && value.is_empty() {
        "."
    } else {
        value
    };

    // Step 1 — the key must be a declared knob (the closed surface).
    let pack = make_pack()?;
    let knobs_bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .context("the embedded pack is missing `config/knobs`")?;
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

    // docs-root re-point: detect + route + MOVE the committed docs the change would strand at
    // the *prior* resolved root (M39 inc-5 T3, replacing the M36 warn-then-strand). This is
    // `run_set`'s first read of the committed doc surface — a store-access seam that resolves
    // the *current* cascade (its `docs-root` is the old root) and enumerates `git ls-files`,
    // comparing the old resolved root against the new value (`design/storage.md` → docs-root;
    // `design/reconciliation.md` → the route home). It runs BEFORE the knob lands so the
    // resolution reads the old cascade. Best-effort: it never fails the write (the set lands).
    if key == "docs-root" {
        route_docs_root_repoint_orphans(pack.as_ref(), &project_config, value);
    }

    // `placement-root` re-point: the placement sibling of the loop above, same place and
    // same posture — it runs BEFORE the knob lands (so the prior home resolves off the old
    // cascade) and is best-effort (a hiccup never fails the write). T1 shipped the knob and
    // said so plainly: a re-point re-resolves every nested placement home while the
    // committed instance stays put, and the store goes QUIET about it (the record still
    // baselines the old path and the file still matches). This is the floor that closes it.
    if key == "placement-root" {
        route_placement_root_repoint_strands(pack.as_ref(), &project_config, value);
    }

    // Step 3 — record the `scalar-set` into the project manifest (last-write-wins).
    write_scalar(&project_config, key, value)?;
    Ok(ConfigAck::Set {
        key: key.to_owned(),
        value: value.to_owned(),
    })
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
fn route_docs_root_repoint_orphans(pack: &dyn PackSource, project_config: &Path, new_value: &str) {
    let Ok(resolved) = crate::start::resolve_severity_cascade(pack, project_config) else {
        return;
    };
    let old_docs_root = crate::start::docs_root_prefix(&resolved).to_string();
    let defs = crate::start::CascadeDefs::new(&resolved, project_config);
    let Ok(old_schemas) = defs.all_schemas(pack) else {
        return;
    };
    // `<repo>/.jigc/config` → the repo root the committed docs (and `git ls-files`) live at.
    let Some(repo_root) = project_config.parent().and_then(Path::parent) else {
        return;
    };
    let stranded = crate::orphan::docs_root_would_orphan(
        repo_root,
        old_schemas.values(),
        &old_docs_root,
        new_value,
    );
    if stranded.is_empty() {
        return;
    }
    let new_root = normalize_docs_root(new_value);
    let jigc_root = repo_root.join(".jigc");
    // The solicit/act honesty pair (round-2 D2+D3): name the sweep's basis — every
    // committed doc under the prior resolved root, managed or not (committed truth,
    // deliberately index-blind so a fresh clone still relocates) — and the git state
    // the moves land in (each is a staged `git mv`, committed by the operator's next
    // commit, never here).
    eprintln!(
        "relocating {} committed doc(s) stranded by the `docs-root` re-point to `{new_value}` \
         (every committed doc under the prior resolved root, managed or not; each move is a \
         staged `git mv` — commit it with your next commit):",
        stranded.len()
    );
    for old_rel in &stranded {
        let Some(new_rel) = reroot(old_rel, &old_docs_root, &new_root) else {
            continue;
        };
        // A pure relocation preserves the bytes, so the re-keyed hash is the current file's.
        let new_hash = match std::fs::read(repo_root.join(old_rel)) {
            Ok(bytes) => engine::file_state::hash_bytes(&bytes),
            Err(_) => continue,
        };
        // `git mv` needs the destination directory to exist (a re-point to a fresh root creates
        // dirs that never existed).
        if let Some(parent) = Path::new(&new_rel).parent() {
            let _ = std::fs::create_dir_all(repo_root.join(parent));
        }
        match crate::relocate::move_doc(repo_root, &jigc_root, old_rel, &new_rel, &new_hash) {
            Ok(()) => eprintln!("  - {old_rel} → {new_rel}"),
            Err(err) => {
                eprintln!("  - {old_rel}: could not relocate ({err:#}) — move it by hand")
            }
        }
    }
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
/// surfaced (routing the operator to move it by hand) but never fails the write.
fn route_placement_root_repoint_strands(
    pack: &dyn PackSource,
    project_config: &Path,
    new_value: &str,
) {
    let Ok(resolved) = crate::start::resolve_severity_cascade(pack, project_config) else {
        return;
    };
    let old_root = crate::start::placement_root(&resolved);
    let new_root = crate::start::normalize_placement_root(new_value);
    if old_root == new_root {
        return; // the same home for every doctype — nothing can be stranded.
    }
    let defs = crate::start::CascadeDefs::new(&resolved, project_config);
    let Ok(declared) = defs.declared_schemas(pack) else {
        return;
    };
    // `<repo>/.jigc/config` → the repo root the committed docs (and `git ls-files`) live at.
    let Some(repo_root) = project_config.parent().and_then(Path::parent) else {
        return;
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
        return;
    }
    // The solicit/act honesty pair the `docs-root` sibling prints: name the sweep's basis
    // (every committed doc at a placement doctype's prior home, managed or not — committed
    // truth, deliberately index-blind so a fresh clone still relocates) and the git state the
    // moves land in (each is a staged `git mv`, committed by the operator's next commit).
    eprintln!(
        "relocating the committed doc(s) stranded by the `placement-root` re-point to \
         `{new_value}` (every committed doc at a placement doctype's prior home, managed or \
         not; each move is a staged `git mv` — commit it with your next commit):"
    );
    let jigc_root = repo_root.join(".jigc");
    for (prior, current) in &pairs {
        let report = crate::relocate::relocate_stranded(repo_root, &jigc_root, prior, current);
        for (from, to) in &report.moved {
            eprintln!("  - {from} → {to}");
        }
        for (from, to) in &report.displaced {
            eprintln!("  - {from} → {to} (a foreign file at the new home, parked out of the way)");
        }
        for (path, reason) in &report.blocked {
            eprintln!("  - {path}: could not relocate ({reason}) — move it by hand");
        }
    }
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
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let severity = match finding.severity {
        engine::finding::Severity::Blocking => "blocking",
        engine::finding::Severity::Warning => "warning",
        engine::finding::Severity::Advisory => "advisory",
    };
    let head = format!("{severity} · {} — {}", finding.code, finding.message);
    match finding.route {
        Some(route) => anyhow::anyhow!("{head}\n  route: {route}"),
        None => anyhow::anyhow!("{head}"),
    }
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
            let ack = run_set(repo.path(), "default-workflow", "quick-fix").expect("set records");
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
        let err = run_set(repo.path(), "not-a-knob", "x").expect_err("an undeclared key rejects");
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
        };
        let out = crate::render::config_ack(crate::cli::Format::Json, &ack);
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("json ack parses");
        assert_eq!(parsed["op"], "config-set");
        assert_eq!(parsed["key"], "docs-root");
        assert_eq!(parsed["value"], "docs2");
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
}
