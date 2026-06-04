//! Cascade resolution (`project > team > pack-default`) and the delta ladder.
//!
//! See `design/overrides.md` (the 9-phase resolution algorithm) and
//! `VISION.md` principle #5.
//!
//! This module implements the read-path resolution for the layers actually
//! present — **phase 2** (by-id file shadowing) and **phase 3** (scalar-delta
//! application) of the algorithm — plus the **cascade-provenance** header data
//! every long-lived surface shows (`design/overrides.md` → Cascade provenance is
//! visible on every long-lived surface).
//!
//! Only the **phase-3 scalar surface** is wired into the live compose path (the
//! `default-workflow` read goes through [`Resolved::scalar_required`]). The
//! **phase-2 file-owner surface** ([`Resolved::file_owner`]) is *resolved* here
//! but **not yet consumed by compose** — the live step source reads pack-default
//! bytes only; routing step ids through it lands in a later increment.
//!
//! The engine is *fed* the layers (feed-layers-in / assert-results-out): the
//! pack-default layer carries the **closed declared-key surface** and the base
//! scalar values; `team` / `project` are optional layers carrying `scalar-set`
//! deltas and shadowing files. Resolution is a pure function of those inputs.
//!
//! Scalar values are opaque strings here: typed adjudication (enum / bool / int)
//! is the document-type field model and lands with the write path. This slice
//! enforces only the closed surface — an undeclared `scalar-set` is an error.

use std::collections::BTreeMap;
use thiserror::Error;

use crate::finding::{Finding, Location};

/// The scheme every `structural-op` definition-target carries — the literal
/// `workflow` in `workflow:<id>`. This parser handles the **workflow include
/// list** namespace only (the MVP structural surface); `schema:<id>` sections
/// share the grammar but are out of this increment's scope.
const WORKFLOW_SCHEME: &str = "workflow";

/// The scheme every `slot-fill` target carries — the literal `step` in
/// `step:<id>#<fill-id>`. A slot-fill addresses a `{{fill:<id>}}` extension point
/// in a *step body*, a namespace distinct from the `workflow:` include-list
/// target and the content [`crate::address::Address`] (`design/overrides.md` →
/// Delta targets).
const STEP_SCHEME: &str = "step";

/// Where a `structural-op` delta attaches in a workflow's include list — the
/// **anchor**, distinct from the content [`crate::address::Address`] (which is
/// instance-scoped, `type:slug#unit/...`). A definition-target names *"a list
/// entry in `WorkflowDef.includes`"*, a different namespace from a document
/// slice (`design/overrides.md` → Delta targets — addressing a definition).
///
/// The anchor is **spelled explicitly** — `after:` / `before:` are their own
/// manifest keys, never overloaded onto `#` (`design/overrides.md` → Delta
/// targets). The `#<step-id>` form is the `replace` / `remove` target, which
/// needs no anchor and is modelled as [`Anchor::At`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// `workflow:<id>#<step-id>` — the entry *at* this step id (replace / remove).
    At(String),
    /// `workflow:<id>` + `after:<step-id>` — insert after this anchor step.
    After(String),
    /// `workflow:<id>` + `before:<step-id>` — insert before this anchor step.
    Before(String),
}

/// The explicit insert anchor a manifest supplies as an `after:` / `before:`
/// key, paired with its step id — the parser input distinct from the `#`
/// (replace / remove) target form (`design/overrides.md` → Delta targets).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnchorSpec {
    /// The manifest's `after: <step-id>` key.
    After(String),
    /// The manifest's `before: <step-id>` key.
    Before(String),
}

/// A parsed `structural-op` definition-target: which workflow's include list,
/// and where in it. The load-bearing distinction from the content
/// [`crate::address::Address`]: `workflow:single-task#validate` here means *"the
/// entry `validate` in `single-task`'s include list"* — `single-task` is a
/// **workflow id**, `validate` a **step-id list entry**, both living in a
/// namespace separate from document addressing (`design/overrides.md` → Delta
/// targets). Pure structure; no I/O, no cascade consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralTarget {
    /// The workflow id whose include list this target operates on.
    pub workflow_id: String,
    /// Where in the include list — the `#`, `after:`, or `before:` anchor.
    pub anchor: Anchor,
}

/// Why a [`StructuralTarget`] failed to parse. Every variant carries the text
/// for a located, blocking [`Finding`]; hostile input is never a panic
/// (`design/overrides.md` → Delta targets).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TargetParseError {
    /// The scheme before `:` was not `workflow`.
    WrongScheme,
    /// No `:` separating scheme from id.
    MissingColon,
    /// The workflow id was empty.
    EmptyWorkflowId,
    /// The `#<step-id>` form had an empty step id (a trailing `#`).
    EmptyStepId,
    /// An `after:` / `before:` anchor carried an empty step id.
    EmptyAnchorStepId,
    /// Neither a `#<step-id>` nor an `after:` / `before:` anchor was supplied —
    /// a bare `workflow:<id>` does not name a list position.
    MissingAnchor,
    /// Both a `#<step-id>` and an `after:` / `before:` anchor were supplied —
    /// the two anchor channels are mutually exclusive.
    ConflictingAnchors,
    /// A target or anchor id contained a non-ASCII byte (ids are ASCII).
    NonAscii,
}

impl TargetParseError {
    /// The stable machine code for this error's [`Finding`].
    fn code(self) -> &'static str {
        match self {
            TargetParseError::WrongScheme => "structural-target.wrong-scheme",
            TargetParseError::MissingColon => "structural-target.missing-colon",
            TargetParseError::EmptyWorkflowId => "structural-target.empty-workflow-id",
            TargetParseError::EmptyStepId => "structural-target.empty-step-id",
            TargetParseError::EmptyAnchorStepId => "structural-target.empty-anchor-step-id",
            TargetParseError::MissingAnchor => "structural-target.missing-anchor",
            TargetParseError::ConflictingAnchors => "structural-target.conflicting-anchors",
            TargetParseError::NonAscii => "structural-target.non-ascii",
        }
    }

    /// The human-readable message for this error's [`Finding`].
    fn message(self) -> &'static str {
        match self {
            TargetParseError::WrongScheme => "structural-op target scheme must be `workflow`",
            TargetParseError::MissingColon => "structural-op target needs a `workflow:<id>` scheme",
            TargetParseError::EmptyWorkflowId => "structural-op target has an empty workflow id",
            TargetParseError::EmptyStepId => "structural-op target `#<step-id>` is empty",
            TargetParseError::EmptyAnchorStepId => {
                "structural-op `after:` / `before:` anchor step id is empty"
            }
            TargetParseError::MissingAnchor => {
                "structural-op target needs a `#<step-id>` or an `after:` / `before:` anchor"
            }
            TargetParseError::ConflictingAnchors => {
                "structural-op target cannot carry both `#<step-id>` and an `after:` / `before:` anchor"
            }
            TargetParseError::NonAscii => "structural-op target ids must be ASCII",
        }
    }

    /// Project to a located, blocking [`Finding`] — these targets are short
    /// config strings parsed positionally, so the location is the string head.
    fn into_finding(self) -> Finding {
        Finding::blocking(self.code(), self.message(), Location::at(1, 1))
    }
}

impl StructuralTarget {
    /// Parse a definition-target from a `target` string plus the optional
    /// explicit insert [`AnchorSpec`] (the manifest's `after:` / `before:` key).
    ///
    /// - `workflow:<id>#<step-id>`, `anchor = None` → [`Anchor::At`] (replace / remove).
    /// - `workflow:<id>`, `anchor = Some(After/Before)` → [`Anchor::After`] / [`Anchor::Before`].
    ///
    /// Hostile input — empty id, wrong scheme, missing anchor, both anchor
    /// channels, non-ASCII — returns a located, blocking [`Finding`], never a
    /// panic. Pure: no I/O, no cascade consulted (`design/overrides.md` → Delta
    /// targets — addressing a definition).
    pub fn parse(target: &str, anchor: Option<AnchorSpec>) -> Result<Self, Finding> {
        Self::parse_inner(target, anchor).map_err(TargetParseError::into_finding)
    }

    fn parse_inner(target: &str, anchor: Option<AnchorSpec>) -> Result<Self, TargetParseError> {
        if !target.is_ascii() {
            return Err(TargetParseError::NonAscii);
        }

        let (reference, hash_step) = match target.split_once('#') {
            Some((reference, step)) => (reference, Some(step)),
            None => (target, None),
        };

        let (scheme, workflow_id) = reference
            .split_once(':')
            .ok_or(TargetParseError::MissingColon)?;
        if scheme != WORKFLOW_SCHEME {
            return Err(TargetParseError::WrongScheme);
        }
        if workflow_id.is_empty() {
            return Err(TargetParseError::EmptyWorkflowId);
        }

        let anchor = match (hash_step, anchor) {
            (Some(_), Some(_)) => return Err(TargetParseError::ConflictingAnchors),
            (Some(step), None) => {
                if step.is_empty() {
                    return Err(TargetParseError::EmptyStepId);
                }
                Anchor::At(step.to_owned())
            }
            (None, Some(spec)) => {
                let (id, make): (&str, fn(String) -> Anchor) = match &spec {
                    AnchorSpec::After(id) => (id, Anchor::After),
                    AnchorSpec::Before(id) => (id, Anchor::Before),
                };
                if !id.is_ascii() {
                    return Err(TargetParseError::NonAscii);
                }
                if id.is_empty() {
                    return Err(TargetParseError::EmptyAnchorStepId);
                }
                make(id.to_owned())
            }
            (None, None) => return Err(TargetParseError::MissingAnchor),
        };

        Ok(StructuralTarget {
            workflow_id: workflow_id.to_owned(),
            anchor,
        })
    }
}

/// One `structural-op` delta: which kind of mutation, over which
/// [`StructuralTarget`]. The three kinds are the override ladder's rung 2
/// (`design/overrides.md` → The ladder); they operate on a workflow's **include
/// id list** at phase 4, *before* include expansion.
///
/// - [`StructuralDelta::Insert`] — splice a step id at the target's `after:` /
///   `before:` anchor. The inserted id is `step` (the native step file's
///   basename — `design/overrides.md` → Native-file id = filename basename); the
///   anchor lives in `target.anchor` ([`Anchor::After`] / [`Anchor::Before`]).
/// - [`StructuralDelta::Replace`] — swap the id at the target's `#<step-id>`
///   position ([`Anchor::At`]) for `step`. The replacement is **another step
///   id**, never inline content (`design/overrides.md` → `replace` vs
///   `tracked-fork`).
/// - [`StructuralDelta::Remove`] — drop the id at the target's `#<step-id>`
///   position ([`Anchor::At`]).
///
/// A delta whose anchor / target id is absent from the current list is an
/// **orphaned** `workflow-refs` finding ([`crate::compose::apply_structural_deltas`]);
/// within a layer, deltas apply in manifest order, so an earlier delta's result
/// is the later delta's input (`design/overrides.md` → Within-layer manifest
/// order). Pure data; no I/O, no cascade consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructuralDelta {
    /// Insert `step` at the target's `after:` / `before:` anchor.
    Insert {
        /// The workflow + anchor this insert attaches to.
        target: StructuralTarget,
        /// The step id to splice in (the native step file's basename).
        step: String,
    },
    /// Replace the id at the target's `#<step-id>` position with `step`.
    Replace {
        /// The workflow + `#<step-id>` position to swap.
        target: StructuralTarget,
        /// The replacement step id (another step id, never inline content).
        step: String,
    },
    /// Remove the id at the target's `#<step-id>` position.
    Remove {
        /// The workflow + `#<step-id>` position to drop.
        target: StructuralTarget,
    },
}

impl StructuralDelta {
    /// The [`StructuralTarget`] this delta operates on — the workflow id and the
    /// include-list position / anchor.
    pub fn target(&self) -> &StructuralTarget {
        match self {
            StructuralDelta::Insert { target, .. }
            | StructuralDelta::Replace { target, .. }
            | StructuralDelta::Remove { target } => target,
        }
    }
}

/// A parsed `slot-fill` target: which step's body, and which `{{fill:<id>}}`
/// extension point in it. The load-bearing distinction from both the
/// [`StructuralTarget`] (which names a `workflow:` *include-list entry*) and the
/// content [`crate::address::Address`] (instance-scoped, `type:slug#unit/...`):
/// `step:implement#extra-guidance` names *"the `{{fill: extra-guidance}}` point in
/// the `implement` step body"* — `implement` is a **step id**, `extra-guidance` a
/// **fill-id**, a namespace separate from include lists and document slices
/// (`design/overrides.md` → Delta targets). Pure structure; no I/O, no cascade
/// consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotFillTarget {
    /// The step id whose body carries the `{{fill:<id>}}` point.
    pub step_id: String,
    /// The fill-id naming the `{{fill:<id>}}` extension point in that body.
    pub fill_id: String,
}

/// Why a [`SlotFillTarget`] failed to parse. Every variant carries the text for a
/// located, blocking [`Finding`]; hostile input is never a panic
/// (`design/overrides.md` → The `{{fill:}}` placeholder).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SlotFillTargetParseError {
    /// The scheme before `:` was not `step` (e.g. a `workflow:` include-list
    /// target or a `type:slug` content address is not a slot-fill target).
    WrongScheme,
    /// No `:` separating scheme from id.
    MissingColon,
    /// No `#` separating the step id from the fill-id — a bare `step:<id>` does
    /// not name an extension point.
    MissingHash,
    /// The step id (before `#`) was empty.
    EmptyStepId,
    /// The fill-id (after `#`) was empty (a trailing `#`).
    EmptyFillId,
    /// A target id contained a non-ASCII byte (ids are ASCII).
    NonAscii,
}

impl SlotFillTargetParseError {
    /// The stable machine code for this error's [`Finding`].
    fn code(self) -> &'static str {
        match self {
            SlotFillTargetParseError::WrongScheme => "slot-fill-target.wrong-scheme",
            SlotFillTargetParseError::MissingColon => "slot-fill-target.missing-colon",
            SlotFillTargetParseError::MissingHash => "slot-fill-target.missing-hash",
            SlotFillTargetParseError::EmptyStepId => "slot-fill-target.empty-step-id",
            SlotFillTargetParseError::EmptyFillId => "slot-fill-target.empty-fill-id",
            SlotFillTargetParseError::NonAscii => "slot-fill-target.non-ascii",
        }
    }

    /// The human-readable message for this error's [`Finding`].
    fn message(self) -> &'static str {
        match self {
            SlotFillTargetParseError::WrongScheme => "slot-fill target scheme must be `step`",
            SlotFillTargetParseError::MissingColon => "slot-fill target needs a `step:<id>` scheme",
            SlotFillTargetParseError::MissingHash => {
                "slot-fill target needs a `#<fill-id>` extension point"
            }
            SlotFillTargetParseError::EmptyStepId => "slot-fill target has an empty step id",
            SlotFillTargetParseError::EmptyFillId => "slot-fill target `#<fill-id>` is empty",
            SlotFillTargetParseError::NonAscii => "slot-fill target ids must be ASCII",
        }
    }

    /// Project to a located, blocking [`Finding`] — these targets are short config
    /// strings parsed positionally, so the location is the string head.
    fn into_finding(self) -> Finding {
        Finding::blocking(self.code(), self.message(), Location::at(1, 1))
    }
}

impl SlotFillTarget {
    /// Parse a slot-fill target from a `step:<id>#<fill-id>` string.
    ///
    /// Hostile input — empty step id, empty fill-id, a missing `#`, a
    /// `workflow:` / content-address scheme, non-ASCII — returns a located,
    /// blocking [`Finding`], never a panic. Pure: no I/O, no cascade consulted
    /// (`design/overrides.md` → The `{{fill:}}` placeholder, Delta targets).
    pub fn parse(target: &str) -> Result<Self, Finding> {
        Self::parse_inner(target).map_err(SlotFillTargetParseError::into_finding)
    }

    fn parse_inner(target: &str) -> Result<Self, SlotFillTargetParseError> {
        if !target.is_ascii() {
            return Err(SlotFillTargetParseError::NonAscii);
        }

        let (reference, fill_id) = target
            .split_once('#')
            .ok_or(SlotFillTargetParseError::MissingHash)?;

        let (scheme, step_id) = reference
            .split_once(':')
            .ok_or(SlotFillTargetParseError::MissingColon)?;
        if scheme != STEP_SCHEME {
            return Err(SlotFillTargetParseError::WrongScheme);
        }
        if step_id.is_empty() {
            return Err(SlotFillTargetParseError::EmptyStepId);
        }
        if fill_id.is_empty() {
            return Err(SlotFillTargetParseError::EmptyFillId);
        }

        Ok(SlotFillTarget {
            step_id: step_id.to_owned(),
            fill_id: fill_id.to_owned(),
        })
    }
}

/// One `slot-fill` delta: which `{{fill:<id>}}` extension point to fill, and the
/// native fill file whose body supplies the content. A **separate delta kind**
/// from [`StructuralDelta`] — slot-fill operates on a *step body* at phase 5, not
/// on a workflow's include list at phase 4 (`design/overrides.md` → The ladder,
/// Resolution algorithm).
///
/// `content_id` is the native fill file's id — its basename, the same
/// "id = filename" rule (`.jigc/config/fills/<id>.md` → `content_id = <id>`;
/// `design/overrides.md` → Authoring deltas). The bytes themselves are loaded by
/// the phase-5 application pass (a later task), not held here. Pure data; no I/O,
/// no cascade consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotFillDelta {
    /// The `step:<id>#<fill-id>` extension point this delta fills.
    pub target: SlotFillTarget,
    /// The native fill file's id (basename) whose body supplies the content.
    pub content_id: String,
}

/// One `tracked-fork` delta: the override ladder's rung 4 (`design/overrides.md`
/// → The ladder) — the last resort that copies a unit's body into a native file
/// at the overriding layer, recording the **pinned basis** that lets a (M5)
/// 3-way merge detect upstream conflicts.
///
/// The `target` reuses [`StructuralTarget`] with an [`Anchor::At`] step id —
/// `workflow:single-task#implement` names *which* unit the fork copied (the
/// native step file `implement`, shadowed whole at phase 2), not the workflow
/// itself (`design/overrides.md` → `tracked-fork` hash basis). At compose time a
/// fork is just a shadowed file, so this type is **not consumed by compose**.
///
/// `base_version` + `base_hash` are the recorded ancestor: the pack version the
/// fork was taken from and the **blake3** hash of the resolved native step/section
/// file bytes (the post-shadow, pre-expansion body — `decisions-pending.md` →
/// Hashing). M4 *records* this basis; only the (M5) reconciliation reads it, where
/// the stateless compare `v2 hash ≠ base_hash` flags upstream conflicts. Pure
/// data; no I/O, no cascade consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackedForkDelta {
    /// The `workflow:<id>#<step-id>` unit this fork copied ([`Anchor::At`]).
    pub target: StructuralTarget,
    /// The pack version the fork was taken from — the recorded ancestor version.
    pub base_version: String,
    /// The blake3 hash of the resolved unit bytes at fork time — the pinned
    /// basis the (M5) stateless compare reads.
    pub base_hash: String,
}

/// The M5 base-hash basis recorded for a content-bearing **`replace`** / **`remove`**
/// delta — the separate recording surface keyed by the delta's [`StructuralTarget`],
/// **not** a field on the compose-facing [`StructuralDelta`] (design-review B2). The
/// basis is the **blake3 of the displaced *pack-default* unit's resolved native
/// bytes** plus the pack version it was recorded against — the same basis
/// [`TrackedForkDelta`] already carries, recorded the same pack-direct way for all
/// three content-bearing kinds (`design/overrides.md` → Per-kind base-hash basis).
///
/// It rides a separate record so the phase-4 compose path and its byte-identical
/// goldens are untouched — the basis is read only by the (M5) `override-default`
/// reconciliation, never by compose. For backward-compat an M4-written
/// `replace`/`remove` delta has **no** recorded basis, so the loader parses the
/// keys **Optional** for these kinds; a delta with no recorded basis simply has no
/// [`StructuralBasis`] record (it classifies `needs-rebasing` at probe time). Pure
/// data; no I/O, no cascade consulted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralBasis {
    /// The `workflow:<id>#<step-id>` target of the `replace`/`remove` delta this
    /// basis was recorded for — its identity (a delta has no stable id; its target
    /// keys it uniquely, `StructuralTarget` being `Clone + Eq`).
    pub target: StructuralTarget,
    /// The pack version the basis was recorded against — narrative-only at probe
    /// time, mirroring [`TrackedForkDelta::base_version`].
    pub base_version: String,
    /// The blake3 hash of the displaced pack-default unit's resolved native bytes —
    /// the pinned basis the (M5) stateless compare reads.
    pub base_hash: String,
}

/// Identifies one cascade layer by precedence. `Project` is most-specific and
/// wins; `PackDefault` is the base (`design/overrides.md` → The cascade).
///
/// Its JSON projection (`pack-default` / `team` / `project`) is the stable
/// per-step provenance label the `--explain` resolution tree binds to
/// (`design/workflow-dialect.md` → `--explain` output contract, layers 1–2), so
/// the kebab-case rename is pinned like the other result-contract enums.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum LayerKind {
    PackDefault,
    Team,
    Project,
}

impl LayerKind {
    /// The provenance label shown in the cascade header / `--explain` tree
    /// (`pack-default` / `team` / `project`).
    pub fn label(self) -> &'static str {
        match self {
            LayerKind::PackDefault => "pack-default",
            LayerKind::Team => "team",
            LayerKind::Project => "project",
        }
    }
}

/// The pack-default layer: the base of the cascade. It declares the **closed**
/// scalar-knob surface (only these keys are settable) and ships the base scalar
/// values and base files. Its id + version feed the provenance header.
pub struct PackDefaultLayer {
    pack_id: String,
    pack_version: String,
    /// The closed set of settable scalar keys, with their base values.
    scalars: BTreeMap<String, String>,
    /// File ids this layer provides (workflow / step / schema ids).
    files: Vec<String>,
}

impl PackDefaultLayer {
    /// Build the base layer from its identity, base scalars, and base file ids.
    pub fn new(
        pack_id: impl Into<String>,
        pack_version: impl Into<String>,
        scalars: BTreeMap<String, String>,
        files: Vec<String>,
    ) -> Self {
        Self {
            pack_id: pack_id.into(),
            pack_version: pack_version.into(),
            scalars,
            files,
        }
    }
}

/// An override layer (`team` or `project`): `scalar-set` deltas in manifest
/// order plus the file ids it shadows. The base declares the closed surface, so
/// these layers only *set* declared keys — an undeclared key is an error.
#[derive(Default)]
pub struct OverrideLayer {
    /// `scalar-set` deltas, in manifest (application) order.
    scalar_sets: Vec<(String, String)>,
    /// File ids this layer shadows (highest-precedence present layer wins).
    files: Vec<String>,
    /// Where this layer's committed config lives, for the provenance header.
    config_path: Option<String>,
}

impl OverrideLayer {
    /// An empty layer — present in the cascade but carrying no deltas or files.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Record a `scalar-set` delta (applied in the order added).
    pub fn scalar_set(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.scalar_sets.push((key.into(), value.into()));
        self
    }

    /// The keys of every recorded `scalar-set` delta, in manifest (application)
    /// order. The upgrade-time `override-default` classifier reads these to ask the
    /// scalar-set existence question (`overrides.md` → Upgrade reconciliation: a
    /// `scalar-set` is `clean` while its key is still a declared knob, `orphaned`
    /// once the current pack drops it from the closed surface). The values are
    /// irrelevant to that existence check, so only the keys are exposed.
    pub fn scalar_set_keys(&self) -> impl Iterator<Item = &str> {
        self.scalar_sets.iter().map(|(key, _)| key.as_str())
    }

    /// Declare a file id this layer shadows.
    pub fn shadow_file(mut self, id: impl Into<String>) -> Self {
        self.files.push(id.into());
        self
    }

    /// Record this layer's committed-config path (shown in the provenance header).
    pub fn config_path(mut self, path: impl Into<String>) -> Self {
        self.config_path = Some(path.into());
        self
    }
}

/// Why cascade resolution failed.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum CascadeError {
    /// A `scalar-set` targeted a key the pack-default layer never declared. The
    /// knob surface is closed — an undeclared key is a config-conformance error
    /// (`design/overrides.md` → Scalar knobs are config-level fields).
    #[error("layer `{layer}` sets undeclared scalar key `{key}` (the knob surface is closed)")]
    UndeclaredScalar { layer: &'static str, key: String },

    /// A compose-path read targeted a key the resolved surface never carried.
    /// Routing a compose read through the cascade is byte-safe only if the key
    /// is declared (and so seeded into the base map); a missing value is a hard
    /// error, never a silent `None`/raw fallback — the read-side half of the
    /// closed-surface rule (`design/overrides.md` → Read-side determinism
    /// invariant).
    #[error("compose read of undeclared scalar key `{key}` (the knob surface is closed)")]
    UndeclaredComposeRead { key: String },
}

/// The cascade-provenance header data — what every long-lived surface shows so
/// the resolved cascade (a declared input to determinism) is never hidden
/// (`design/overrides.md` → Cascade provenance is visible on every long-lived
/// surface). The branch / HEAD segment is supplied by the frontend at render
/// time; the engine owns the pack + config-path segments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Provenance {
    pack_id: String,
    pack_version: String,
    project_config: Option<String>,
    team_config: Option<String>,
}

impl Provenance {
    /// `Pack: <id>/<version>` — always present.
    pub fn pack_segment(&self) -> String {
        format!("Pack: {}/{}", self.pack_id, self.pack_version)
    }

    /// The header segments the engine owns, joined with ` · `: the pack segment,
    /// then a `Project config:` / `Team config:` segment for each populated
    /// external layer. The frontend appends `Branch:` / HEAD at render time.
    pub fn header(&self) -> String {
        let mut header = self.pack_segment();
        if let Some(path) = &self.project_config {
            header.push_str(&format!(" · Project config: {path}"));
        }
        if let Some(path) = &self.team_config {
            header.push_str(&format!(" · Team config: {path}"));
        }
        header
    }
}

/// The resolved cascade for the layers present: the resolved scalar values, the
/// resolved file owners (by-id shadowing), and the provenance header data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolved {
    scalars: BTreeMap<String, String>,
    /// Per-overridden-key winning layer: the layer that last applied a
    /// `scalar-set` for the key. Keys left at the pack-default base are absent —
    /// this surface holds *only* the overrides, feeding `--explain` layer 1
    /// (`overrides applied: N`, the winning layer per key). Additive: the
    /// resolved values in [`Resolved::scalars`] are unchanged by its presence.
    scalar_provenance: BTreeMap<String, LayerKind>,
    file_owners: BTreeMap<String, LayerKind>,
    provenance: Provenance,
}

impl Resolved {
    /// The resolved value of a declared scalar key, or `None` if undeclared.
    pub fn scalar(&self, key: &str) -> Option<&str> {
        self.scalars.get(key).map(String::as_str)
    }

    /// The resolved value of a key a compose path reads — the read-side
    /// determinism accessor. Unlike [`Resolved::scalar`], a missing key is a
    /// hard [`CascadeError::UndeclaredComposeRead`], never a silent `None`: a
    /// compose read is byte-safe only over the declared, seeded surface
    /// (`design/overrides.md` → Read-side determinism invariant).
    pub fn scalar_required(&self, key: &str) -> Result<&str, CascadeError> {
        self.scalar(key)
            .ok_or_else(|| CascadeError::UndeclaredComposeRead {
                key: key.to_owned(),
            })
    }

    /// The resolved value of a key **only if an override layer applied a
    /// `scalar-set` for it** — `None` for a key left at the pack-default base (even
    /// when the base declares it). This is the *explicit-delta* surface the M6
    /// severity post-pass keys on: severity is re-graded **only on an explicit
    /// `scalar-set`** (`design/validation.md` → Override-only-on-explicit-delta), so
    /// the no-override path is byte-identical without the knob defaults having to
    /// mirror the code's emitted literals. Distinct from [`Resolved::scalar`], which
    /// returns the base value for any declared key.
    pub fn overridden_scalar(&self, key: &str) -> Option<&str> {
        self.scalar_provenance
            .get(key)
            .and_then(|_| self.scalars.get(key).map(String::as_str))
    }

    /// Each overridden scalar key paired with its resolved value and the layer
    /// that won it — the per-knob provenance the `--explain` tree's layer-1 lines
    /// render (`design/workflow-dialect.md` → `--explain` output contract: "any
    /// scalar-key overrides applied with their source layer"). Keys left at the
    /// pack-default base are absent (no override). Iterates in key order
    /// (`scalar_provenance` is a `BTreeMap`), so the rendered lines are stable.
    pub fn scalar_overrides(&self) -> impl Iterator<Item = (&str, &str, LayerKind)> {
        self.scalar_provenance.iter().map(|(key, layer)| {
            let value = self.scalars.get(key).map(String::as_str).unwrap_or("");
            (key.as_str(), value, *layer)
        })
    }

    /// Which layer owns the file with this id after shadowing, or `None` if no
    /// layer provides it.
    pub fn file_owner(&self, id: &str) -> Option<LayerKind> {
        self.file_owners.get(id).copied()
    }

    /// The cascade-provenance header data.
    pub fn provenance(&self) -> &Provenance {
        &self.provenance
    }
}

/// Resolve the cascade for the layers present.
///
/// Phase 2 (by-id file shadowing): for each file id, the highest-precedence
/// present layer wins (project > team > pack-default), atomic at file level.
///
/// Phase 3 (scalar deltas): start from the pack-default base values, then apply
/// `team` then `project` `scalar-set` deltas — within a layer in manifest order
/// — so project wins for a shared key. Every `scalar-set` must target a key the
/// base declared, else [`CascadeError::UndeclaredScalar`].
pub fn resolve(
    pack: &PackDefaultLayer,
    team: Option<&OverrideLayer>,
    project: Option<&OverrideLayer>,
) -> Result<Resolved, CascadeError> {
    // Phase 3 — scalar deltas: base, then team, then project (project last).
    // Record the winning layer per overridden key as we fold, so `--explain`
    // can show provenance; a later layer re-setting a key wins both the value
    // and its provenance. Keys left at the base never enter this map.
    let mut scalars = pack.scalars.clone();
    let mut scalar_provenance: BTreeMap<String, LayerKind> = BTreeMap::new();
    for (layer_kind, layer) in [(LayerKind::Team, team), (LayerKind::Project, project)] {
        let Some(layer) = layer else { continue };
        for (key, value) in &layer.scalar_sets {
            if !scalars.contains_key(key) {
                return Err(CascadeError::UndeclaredScalar {
                    layer: layer_kind.label(),
                    key: key.clone(),
                });
            }
            scalars.insert(key.clone(), value.clone());
            scalar_provenance.insert(key.clone(), layer_kind);
        }
    }

    // Phase 2 — by-id file shadowing: base first, then team, then project, each
    // higher layer overwriting the owner for any id it provides.
    let mut file_owners: BTreeMap<String, LayerKind> = BTreeMap::new();
    for id in &pack.files {
        file_owners.insert(id.clone(), LayerKind::PackDefault);
    }
    for (layer_kind, layer) in [(LayerKind::Team, team), (LayerKind::Project, project)] {
        let Some(layer) = layer else { continue };
        for id in &layer.files {
            file_owners.insert(id.clone(), layer_kind);
        }
    }

    let provenance = Provenance {
        pack_id: pack.pack_id.clone(),
        pack_version: pack.pack_version.clone(),
        project_config: project.and_then(|l| l.config_path.clone()),
        team_config: team.and_then(|l| l.config_path.clone()),
    };

    Ok(Resolved {
        scalars,
        scalar_provenance,
        file_owners,
        provenance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Severity;

    /// The `#<step-id>` form parses to a [`Anchor::At`] target (replace / remove),
    /// carrying the workflow id and the step id from the fragment.
    #[test]
    fn parses_replace_remove_target() {
        let target = StructuralTarget::parse("workflow:single-task#validate", None)
            .expect("well-formed #-target parses");

        assert_eq!(
            target,
            StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::At("validate".to_owned()),
            },
        );
    }

    /// `workflow:<id>` + an explicit `after:` anchor parses to [`Anchor::After`].
    #[test]
    fn parses_after_insert_anchor() {
        let target = StructuralTarget::parse(
            "workflow:single-task",
            Some(AnchorSpec::After("locate".to_owned())),
        )
        .expect("well-formed after-anchor parses");

        assert_eq!(
            target,
            StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::After("locate".to_owned()),
            },
        );
    }

    /// `workflow:<id>` + an explicit `before:` anchor parses to [`Anchor::Before`].
    #[test]
    fn parses_before_insert_anchor() {
        let target = StructuralTarget::parse(
            "workflow:single-task",
            Some(AnchorSpec::Before("implement".to_owned())),
        )
        .expect("well-formed before-anchor parses");

        assert_eq!(
            target,
            StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::Before("implement".to_owned()),
            },
        );
    }

    /// Hostile input is a located, blocking [`Finding`] (never a panic), carrying
    /// the stable per-cause `code` at the string head.
    #[test]
    fn hostile_input_is_a_located_blocking_finding() {
        for (target, anchor, code) in [
            // empty workflow id
            (
                "workflow:#validate",
                None,
                "structural-target.empty-workflow-id",
            ),
            (
                "workflow:",
                Some(AnchorSpec::After("x".to_owned())),
                "structural-target.empty-workflow-id",
            ),
            // wrong scheme — a content-Address `type:slug` is not a definition target
            ("adr:foo#decision", None, "structural-target.wrong-scheme"),
            // missing colon entirely
            ("single-task", None, "structural-target.missing-colon"),
            // missing anchor — a bare workflow ref names no list position
            (
                "workflow:single-task",
                None,
                "structural-target.missing-anchor",
            ),
            // empty `#` step id (trailing hash)
            (
                "workflow:single-task#",
                None,
                "structural-target.empty-step-id",
            ),
            // empty anchor step id
            (
                "workflow:single-task",
                Some(AnchorSpec::After(String::new())),
                "structural-target.empty-anchor-step-id",
            ),
            // both anchor channels — mutually exclusive
            (
                "workflow:single-task#validate",
                Some(AnchorSpec::After("locate".to_owned())),
                "structural-target.conflicting-anchors",
            ),
            // non-ASCII in the target
            (
                "workflow:naïve#validate",
                None,
                "structural-target.non-ascii",
            ),
            // non-ASCII in the anchor step id
            (
                "workflow:single-task",
                Some(AnchorSpec::Before("naïve".to_owned())),
                "structural-target.non-ascii",
            ),
        ] {
            let finding = StructuralTarget::parse(target, anchor.clone())
                .expect_err("hostile input is a Finding");
            assert_eq!(finding.severity, Severity::Blocking, "for {target:?}");
            assert_eq!(finding.code, code, "for {target:?}");
            assert_eq!(
                finding.location,
                Some(Location::at(1, 1)),
                "hostile input is located, for {target:?}",
            );
        }
    }

    /// `step:<id>#<fill-id>` parses to its `{step_id, fill_id}` shape, carrying
    /// the step id and fill-id from either side of the `#`.
    #[test]
    fn parses_slot_fill_target() {
        let target =
            SlotFillTarget::parse("step:implement#extra-guidance").expect("well-formed target");

        assert_eq!(
            target,
            SlotFillTarget {
                step_id: "implement".to_owned(),
                fill_id: "extra-guidance".to_owned(),
            },
        );
    }

    /// A [`SlotFillDelta`] pairs a parsed target with the native fill file's
    /// content id (its basename).
    #[test]
    fn slot_fill_delta_pairs_target_with_content_id() {
        let delta = SlotFillDelta {
            target: SlotFillTarget::parse("step:implement#extra-guidance").expect("parses"),
            content_id: "extra-guidance".to_owned(),
        };

        assert_eq!(delta.target.step_id, "implement");
        assert_eq!(delta.target.fill_id, "extra-guidance");
        assert_eq!(delta.content_id, "extra-guidance");
    }

    /// Hostile slot-fill-target input is a located, blocking [`Finding`] (never a
    /// panic), carrying the stable per-cause `code` at the string head.
    #[test]
    fn hostile_slot_fill_target_is_a_located_blocking_finding() {
        for (target, code) in [
            // empty step id
            ("step:#extra-guidance", "slot-fill-target.empty-step-id"),
            // empty fill-id (trailing hash)
            ("step:implement#", "slot-fill-target.empty-fill-id"),
            // missing `#` — a bare `step:<id>` names no extension point
            ("step:implement", "slot-fill-target.missing-hash"),
            // missing colon entirely (and no `#`)
            ("implement", "slot-fill-target.missing-hash"),
            // a `workflow:` include-list scheme is not a slot-fill target
            (
                "workflow:single-task#validate",
                "slot-fill-target.wrong-scheme",
            ),
            // a `type:slug` content address is not a slot-fill target
            ("adr:cache#decision", "slot-fill-target.wrong-scheme"),
            // missing colon but present `#` — no scheme separator
            ("implement#extra-guidance", "slot-fill-target.missing-colon"),
            // non-ASCII in the step id
            ("step:naïve#extra-guidance", "slot-fill-target.non-ascii"),
            // non-ASCII in the fill-id
            ("step:implement#naïve", "slot-fill-target.non-ascii"),
        ] {
            let finding = SlotFillTarget::parse(target).expect_err("hostile input is a Finding");
            assert_eq!(finding.severity, Severity::Blocking, "for {target:?}");
            assert_eq!(finding.code, code, "for {target:?}");
            assert_eq!(
                finding.location,
                Some(Location::at(1, 1)),
                "hostile input is located, for {target:?}",
            );
        }
    }

    /// A [`TrackedForkDelta`] pairs a parsed `workflow:<id>#<step-id>` target
    /// with the recorded basis (`base-version` + `base-hash`); all fields
    /// round-trip in memory (pure data, no I/O).
    #[test]
    fn tracked_fork_delta_round_trips_target_and_basis() {
        let delta = TrackedForkDelta {
            target: StructuralTarget::parse("workflow:single-task#implement", None)
                .expect("well-formed #-target parses"),
            base_version: "v1".to_owned(),
            base_hash: "a3f9deadbeef".to_owned(),
        };

        assert_eq!(
            delta,
            TrackedForkDelta {
                target: StructuralTarget {
                    workflow_id: "single-task".to_owned(),
                    anchor: Anchor::At("implement".to_owned()),
                },
                base_version: "v1".to_owned(),
                base_hash: "a3f9deadbeef".to_owned(),
            },
        );
    }

    /// The forked-unit step id reads back off the [`Anchor::At`] target — the
    /// `#<step-id>` form names which native step file the fork copied.
    #[test]
    fn tracked_fork_step_id_reads_off_anchor_at_target() {
        let delta = TrackedForkDelta {
            target: StructuralTarget::parse("workflow:single-task#implement", None)
                .expect("well-formed #-target parses"),
            base_version: "v1".to_owned(),
            base_hash: "a3f9deadbeef".to_owned(),
        };

        assert_eq!(delta.target.anchor, Anchor::At("implement".to_owned()));
    }

    /// A [`StructuralBasis`] pairs a `replace`/`remove` delta's `#<step-id>` target
    /// with the recorded basis (`base-version` + `base-hash`); all fields round-trip
    /// in memory (pure data, no I/O) — the same shape [`TrackedForkDelta`] carries,
    /// recorded the same pack-direct way (`design/overrides.md` → Per-kind base-hash
    /// basis).
    #[test]
    fn structural_basis_round_trips_target_and_basis() {
        let basis = StructuralBasis {
            target: StructuralTarget::parse("workflow:single-task#validate", None)
                .expect("well-formed #-target parses"),
            base_version: "v1".to_owned(),
            base_hash: "a3f9deadbeef".to_owned(),
        };

        assert_eq!(
            basis,
            StructuralBasis {
                target: StructuralTarget {
                    workflow_id: "single-task".to_owned(),
                    anchor: Anchor::At("validate".to_owned()),
                },
                base_version: "v1".to_owned(),
                base_hash: "a3f9deadbeef".to_owned(),
            },
        );
    }

    fn pack_default() -> PackDefaultLayer {
        let mut scalars = BTreeMap::new();
        scalars.insert("default-workflow".to_owned(), "single-task".to_owned());
        scalars.insert(
            "validation.doc-code.severity".to_owned(),
            "blocking".to_owned(),
        );
        PackDefaultLayer::new("dev-pack", "0.1.0", scalars, vec!["single-task".to_owned()])
    }

    /// Pack-default present, team + project absent: resolution returns the
    /// pack-default scalar values and a provenance that reports `Pack:
    /// <id>/<version>`.
    #[test]
    fn pack_default_only_returns_base_values_and_pack_provenance() {
        let pack = pack_default();

        let resolved = resolve(&pack, None, None).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("single-task"));
        assert_eq!(
            resolved.scalar("validation.doc-code.severity"),
            Some("blocking"),
        );
        assert_eq!(resolved.provenance().pack_segment(), "Pack: dev-pack/0.1.0");
        // No external layer populated → no config-path segments.
        assert_eq!(resolved.provenance().header(), "Pack: dev-pack/0.1.0");
    }

    /// An *empty* project layer (present but no deltas) leaves the base value in
    /// place — present-but-empty is not the same as absent and must not perturb
    /// resolution.
    #[test]
    fn empty_project_layer_leaves_base_value() {
        let pack = pack_default();
        let project = OverrideLayer::empty();

        let resolved = resolve(&pack, None, Some(&project)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("single-task"));
    }

    /// A project `scalar-set` for a declared key wins — project applies last
    /// (`design/overrides.md` phase 3).
    #[test]
    fn project_scalar_set_wins_over_base() {
        let pack = pack_default();
        let project = OverrideLayer::empty().scalar_set("default-workflow", "router");

        let resolved = resolve(&pack, None, Some(&project)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("router"));
    }

    /// Project applies after team for a shared key: project wins even when team
    /// also set it.
    #[test]
    fn project_wins_over_team_for_a_shared_key() {
        let pack = pack_default();
        let team = OverrideLayer::empty().scalar_set("default-workflow", "team-choice");
        let project = OverrideLayer::empty().scalar_set("default-workflow", "project-choice");

        let resolved = resolve(&pack, Some(&team), Some(&project)).expect("resolves");

        assert_eq!(resolved.scalar("default-workflow"), Some("project-choice"));
    }

    /// Read-side determinism invariant: the compose-read accessor returns the
    /// resolved value for a declared key (`design/overrides.md` → Read-side
    /// determinism invariant).
    #[test]
    fn scalar_required_returns_value_for_declared_key() {
        let pack = pack_default();

        let resolved = resolve(&pack, None, None).expect("resolves");

        assert_eq!(
            resolved.scalar_required("default-workflow"),
            Ok("single-task"),
        );
    }

    /// Read-side determinism invariant: a compose-read of a key the closed
    /// surface never declared is a hard error, never a `None`/raw fallback — the
    /// read-side half of the closed-surface rule.
    #[test]
    fn scalar_required_errors_for_undeclared_key() {
        let pack = pack_default();

        let resolved = resolve(&pack, None, None).expect("resolves");

        assert_eq!(
            resolved.scalar_required("not-a-knob"),
            Err(CascadeError::UndeclaredComposeRead {
                key: "not-a-knob".to_owned(),
            }),
        );
    }

    /// The knob surface is closed: a `scalar-set` for a key the pack never
    /// declared is a hard error, naming the offending layer and key.
    #[test]
    fn undeclared_scalar_key_errors() {
        let pack = pack_default();
        let project = OverrideLayer::empty().scalar_set("not-a-knob", "x");

        let err = resolve(&pack, None, Some(&project)).expect_err("undeclared key errors");

        assert_eq!(
            err,
            CascadeError::UndeclaredScalar {
                layer: "project",
                key: "not-a-knob".to_owned(),
            },
        );
    }

    /// Phase 2 — by-id file shadowing: project provides a file with the same id
    /// the pack ships, so project owns it; an id only the pack ships stays with
    /// pack-default.
    #[test]
    fn project_file_shadows_pack_default_by_id() {
        let pack = pack_default();
        let project = OverrideLayer::empty().shadow_file("single-task");

        let resolved = resolve(&pack, None, Some(&project)).expect("resolves");

        assert_eq!(resolved.file_owner("single-task"), Some(LayerKind::Project));
        assert_eq!(resolved.file_owner("absent"), None);
    }

    /// No override layer: every key resolves from the base, so no key carries a
    /// winning-layer provenance and the override-key count is zero — the
    /// `--explain` tree renders `overrides applied: none`
    /// (`design/workflow-dialect.md` → `--explain` output contract, layer 1).
    #[test]
    fn no_override_layer_has_empty_scalar_provenance() {
        let pack = pack_default();

        let resolved = resolve(&pack, None, None).expect("resolves");

        assert_eq!(resolved.scalar_overrides().count(), 0);
    }

    /// A project `scalar-set` records the winning layer for that key
    /// (`default-workflow` → `Project`) and bumps the override-key count to one;
    /// keys left at the base carry no provenance
    /// (`design/worked-examples.md` → 3b).
    #[test]
    fn project_scalar_set_records_project_as_winning_layer() {
        let pack = pack_default();
        let project = OverrideLayer::empty().scalar_set("default-workflow", "single-task");

        let resolved = resolve(&pack, None, Some(&project)).expect("resolves");

        let overrides: Vec<_> = resolved.scalar_overrides().collect();
        assert_eq!(
            overrides,
            vec![("default-workflow", "single-task", LayerKind::Project)],
        );
    }

    /// A team key the project does not re-set keeps `Team` as its winning layer —
    /// provenance records the *last* layer that set the key, project only when it
    /// actually applies a delta (`design/overrides.md` → Resolution algorithm
    /// phase 3).
    #[test]
    fn team_set_not_reset_by_project_records_team() {
        let pack = pack_default();
        let team = OverrideLayer::empty().scalar_set("default-workflow", "team-choice");
        let project = OverrideLayer::empty();

        let resolved = resolve(&pack, Some(&team), Some(&project)).expect("resolves");

        let overrides: Vec<_> = resolved.scalar_overrides().collect();
        assert_eq!(
            overrides,
            vec![("default-workflow", "team-choice", LayerKind::Team)],
        );
    }

    /// Byte-safe-read guard: adding the provenance surface does not perturb the
    /// resolved values — `scalar` / `scalar_required` return identical results
    /// before and after a delta is applied, so the no-override compose path stays
    /// byte-identical (`design/overrides.md` → Read-side determinism invariant).
    #[test]
    fn provenance_surface_leaves_scalar_reads_unchanged() {
        let pack = pack_default();
        let project = OverrideLayer::empty().scalar_set("default-workflow", "router");

        let base = resolve(&pack, None, None).expect("resolves");
        let overridden = resolve(&pack, None, Some(&project)).expect("resolves");

        // The read accessors return exactly the resolved value, provenance or not.
        assert_eq!(base.scalar("default-workflow"), Some("single-task"));
        assert_eq!(base.scalar_required("default-workflow"), Ok("single-task"));
        assert_eq!(overridden.scalar("default-workflow"), Some("router"));
        assert_eq!(overridden.scalar_required("default-workflow"), Ok("router"));
    }

    /// Populated external layers add `Project config:` / `Team config:` segments
    /// to the provenance header, in pack → project → team segment order.
    #[test]
    fn header_includes_populated_config_path_segments() {
        let pack = pack_default();
        let team = OverrideLayer::empty().config_path("/home/u/.config/jigc");
        let project = OverrideLayer::empty().config_path("/repo/.jigc/config");

        let resolved = resolve(&pack, Some(&team), Some(&project)).expect("resolves");

        assert_eq!(
            resolved.provenance().header(),
            "Pack: dev-pack/0.1.0 · Project config: /repo/.jigc/config · Team config: /home/u/.config/jigc",
        );
    }
}
