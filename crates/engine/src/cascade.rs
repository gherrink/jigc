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

/// Identifies one cascade layer by precedence. `Project` is most-specific and
/// wins; `PackDefault` is the base (`design/overrides.md` → The cascade).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LayerKind {
    PackDefault,
    Team,
    Project,
}

impl LayerKind {
    fn label(self) -> &'static str {
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
    let mut scalars = pack.scalars.clone();
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
