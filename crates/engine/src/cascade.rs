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
