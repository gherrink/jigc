//! The `PackSource` provider trait — how a frontend supplies the pack-default
//! cascade layer to the engine, keeping the engine empty of domain content.
//!
//! The engine knows only this provider boundary; it never embeds pack bytes.
//! MVP impl is `EmbeddedPack` (in `cli`); `FilesystemPack` is the post-MVP seam.
//! See `implementation/module-layout.md` → The dev pack's home.

use thiserror::Error;

/// Opaque, stable identifier for a single pack resource within its kind.
///
/// A thin `String` newtype (the id-newtype style of `address`), not a position.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceId(String);

impl ResourceId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ResourceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ResourceId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

/// The kinds of resource a pack-default layer supplies.
///
/// One variant per cascade resource family (module-layout.md → The dev pack's
/// home: `schemas | workflows | steps | config`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PackResourceKind {
    Schemas,
    Workflows,
    Steps,
    Config,
}

/// Why a `PackSource::read` failed.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PackError {
    #[error("no pack resource of kind {kind:?} with id `{id}`")]
    NotFound {
        kind: PackResourceKind,
        id: ResourceId,
    },
}

/// How a frontend provides the pack-default cascade layer to the engine.
///
/// The engine is *fed* this provider and resolves over it (feed-layers-in /
/// assert-results-out); it compiles in no pack content of its own. `list` and
/// `read` are keyed by [`PackResourceKind`]; bytes are raw and unparsed.
pub trait PackSource {
    /// The pack-default layer version (built-in pack: = binary version).
    fn pack_version(&self) -> String;

    /// The ids of every resource of `kind` this source provides.
    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId>;

    /// The raw bytes of one resource, or [`PackError::NotFound`] if absent.
    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError>;

    /// The pack's own provenance segments — `(pack-id, version)` pairs — that the
    /// `Pack:` header renders, **highest-precedence first**. A single pack reports
    /// exactly one segment (its own `config/defaults` `pack-id` + [`pack_version`]),
    /// so the compact header degrades byte-identically to `Pack: <id>/<version>`
    /// when only one pack composes. A composite (`CompositePack`) overrides this to
    /// concatenate its constituents' segments in precedence order — the
    /// multi-pack provenance the determinism contract requires (`design/multi-pack.md`
    /// → Provenance: the single segment becomes the composed set). The pack-id is
    /// read best-effort from `config/defaults`; a pack that declares none reports an
    /// empty id (provenance is a display surface, never a hard-fail path).
    ///
    /// [`pack_version`]: PackSource::pack_version
    fn provenance_segments(&self) -> Vec<(String, String)> {
        vec![(self.own_pack_id(), self.pack_version())]
    }

    /// This pack's own `config/defaults` `pack-id`, best-effort (empty if the
    /// resource is absent, non-UTF-8, non-YAML, or declares no `pack-id`). The
    /// default [`provenance_segments`] reads it; not overridden by `CompositePack`,
    /// which sources each id from its constituents instead.
    ///
    /// [`provenance_segments`]: PackSource::provenance_segments
    fn own_pack_id(&self) -> String {
        self.read(PackResourceKind::Config, &ResourceId::from("defaults"))
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|text| serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&text).ok())
            .and_then(|value| {
                value
                    .get("pack-id")
                    .and_then(|v| v.as_str())
                    .map(str::to_owned)
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// A trivial in-memory `PackSource` — proves the trait is impl-usable with
    /// no domain content compiled into the engine.
    struct FakePack {
        version: String,
        resources: HashMap<(PackResourceKind, ResourceId), Vec<u8>>,
    }

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            self.version.clone()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            let mut ids: Vec<ResourceId> = self
                .resources
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect();
            ids.sort();
            ids
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            self.resources
                .get(&(kind, id.clone()))
                .cloned()
                .ok_or_else(|| PackError::NotFound {
                    kind,
                    id: id.clone(),
                })
        }
    }

    fn seeded() -> FakePack {
        let mut resources = HashMap::new();
        resources.insert(
            (PackResourceKind::Workflows, ResourceId::from("single-task")),
            b"workflow: single-task".to_vec(),
        );
        resources.insert(
            (PackResourceKind::Workflows, ResourceId::from("router")),
            b"workflow: router".to_vec(),
        );
        resources.insert(
            (PackResourceKind::Config, ResourceId::from("defaults")),
            b"default-workflow: single-task".to_vec(),
        );
        FakePack {
            version: "0.0.0".to_owned(),
            resources,
        }
    }

    #[test]
    fn list_returns_the_seeded_ids_for_the_kind() {
        let pack = seeded();
        assert_eq!(
            pack.list(PackResourceKind::Workflows),
            vec![ResourceId::from("router"), ResourceId::from("single-task")],
        );
        assert_eq!(
            pack.list(PackResourceKind::Config),
            vec![ResourceId::from("defaults")],
        );
        assert!(pack.list(PackResourceKind::Steps).is_empty());
    }

    #[test]
    fn read_returns_the_seeded_bytes() {
        let pack = seeded();
        let bytes = pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from("single-task"),
            )
            .expect("seeded id reads back");
        assert_eq!(bytes, b"workflow: single-task");
    }

    #[test]
    fn read_of_a_missing_id_is_not_found() {
        let pack = seeded();
        let err = pack
            .read(PackResourceKind::Workflows, &ResourceId::from("absent"))
            .expect_err("absent id errors");
        assert_eq!(
            err,
            PackError::NotFound {
                kind: PackResourceKind::Workflows,
                id: ResourceId::from("absent"),
            },
        );
    }

    #[test]
    fn pack_version_is_callable() {
        let pack = seeded();
        assert_eq!(pack.pack_version(), "0.0.0");
    }

    /// The trait must be usable behind a `dyn` reference (object-safe) — a
    /// frontend hands the engine some `&dyn PackSource`.
    #[test]
    fn trait_is_object_usable() {
        let pack = seeded();
        let as_dyn: &dyn PackSource = &pack;
        assert_eq!(as_dyn.list(PackResourceKind::Workflows).len(), 2);
    }
}
