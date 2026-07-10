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
/// home: `schemas | workflows | steps | config`), plus [`SchemaSnapshots`] — the
/// M34 versioned prior-schema store, a kind apart from `schemas` (it lives outside
/// `schemas/` so the freeze gate's by-doctype enumeration never collides on `type`;
/// `design/corpus-migration.md` → Prior-schema sourcing).
///
/// [`SchemaSnapshots`]: PackResourceKind::SchemaSnapshots
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PackResourceKind {
    Schemas,
    Workflows,
    Steps,
    Config,
    /// Versioned prior-schema snapshots — `schema-snapshots/<type>.v<N>.yaml`, each
    /// a full reviewable schema file the M34 corpus-migration verb diffs against the
    /// current shape. Keyed by the file stem `<type>.v<N>` as its [`ResourceId`].
    SchemaSnapshots,
}

/// The `resolving_path` sentinel a pack with no on-disk root reports — the
/// binary-embedded base, whose bytes ship inside `jigc` rather than at a directory.
/// `--explain` renders it verbatim as the pack's "path" (`design/worked-examples.md`
/// → flow 17: `dev/0.0.0 = <embedded>`).
pub const EMBEDDED_PATH: &str = "<embedded>";

/// One composed pack's **provenance entry** — its resolving directory path and a
/// content-hash over its bytes. `--explain` renders one of these per composed pack,
/// highest-precedence first, so the human can *see* the exact pack input behind a
/// deterministic outcome (`design/multi-pack.md` → Provenance under N packs:
/// "id/version alone is not the identity — dir contents can change"). The carrier
/// for [`PackSource::provenance_entries`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackProvenance {
    /// The pack's resolving directory path — a [`FilesystemPack`] root, or
    /// [`EMBEDDED_PATH`] for the binary-embedded base.
    pub path: String,
    /// A blake3 content-hash over the pack's bytes (`engine::file_state::hash_bytes`
    /// — the same primitive the drift hash uses, no new dependency). Order-stable:
    /// the same bytes hash identically regardless of `read_dir` order.
    pub content_hash: String,
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

/// Upcast `&self` to `&dyn PackSource` — the object-safe seam that lets
/// [`PackSource::origin_pack`]'s default body return `self` without a
/// `Self: Sized` bound (which would drop the method from the vtable and make it
/// un-callable on the `&dyn PackSource` every consumer holds). The blanket impl
/// covers every sized pack (the coercion is valid there) and supplies the vtable
/// entry, so a `dyn PackSource` receiver dispatches `as_pack_source` through its
/// concrete type. As a supertrait of [`PackSource`], it makes the upcast
/// reachable from the default `origin_pack` body. Not a public surface — an
/// internal upcast helper.
pub trait AsPackSource {
    fn as_pack_source(&self) -> &dyn PackSource;
}

impl<T: PackSource> AsPackSource for T {
    fn as_pack_source(&self) -> &dyn PackSource {
        self
    }
}

/// How a frontend provides the pack-default cascade layer to the engine.
///
/// The engine is *fed* this provider and resolves over it (feed-layers-in /
/// assert-results-out); it compiles in no pack content of its own. `list` and
/// `read` are keyed by [`PackResourceKind`]; bytes are raw and unparsed.
pub trait PackSource: AsPackSource {
    /// The pack-default layer version (built-in pack: = binary version).
    fn pack_version(&self) -> String;

    /// The ids of every resource of `kind` this source provides.
    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId>;

    /// The raw bytes of one resource, or [`PackError::NotFound`] if absent.
    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError>;

    /// How many constituent packs **own** `(kind, id)` — the count `--explain`'s
    /// collision-winner detection reads to tell a genuine cross-pack collision
    /// (≥2 owners, precedence-override picked a winner) from a non-collision
    /// (`design/multi-pack.md` → Collision resolution; Provenance). A single pack
    /// owns at most one (`read().is_ok()`), so the default never reports a
    /// collision; a composite ([`CompositePack`]) overrides this to count its
    /// constituents.
    ///
    /// [`CompositePack`]: ../../cli/pack/struct.CompositePack.html
    fn owner_count(&self, kind: PackResourceKind, id: &ResourceId) -> usize {
        usize::from(self.read(kind, id).is_ok())
    }

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

    /// The constituent pack that **defines** `(kind, id)` — the body-reference
    /// resolution anchor, distinct from the precedence [`read`]. A definition's
    /// body-references (`{{include: step:X}}`, `{{cli.X}}`, a schema's field-type
    /// names) must resolve against the pack that owns the definition's *top-level*
    /// id, never a merged surface — otherwise a loser-pack workflow silently
    /// composes the precedence-winner's divergent step/catalog (the M3-class
    /// corruption; `design/multi-pack.md` → Pack-local body-reference resolution).
    ///
    /// A single pack **is** its own origin, so the default returns `self`. A
    /// composite ([`CompositePack`]) overrides this to return the first
    /// (highest-precedence) constituent whose [`read`] succeeds — the same pack
    /// the composite `read` already selects — falling back to `self` when no
    /// constituent owns the id, so a dangling reference still flows to the
    /// existing not-found path.
    ///
    /// [`read`]: PackSource::read
    /// [`CompositePack`]: ../../cli/pack/struct.CompositePack.html
    fn origin_pack(&self, _kind: PackResourceKind, _id: &ResourceId) -> &dyn PackSource {
        self.as_pack_source()
    }

    /// **Every** constituent pack that ships `(kind, id)`, highest-precedence
    /// first — the plural sibling of [`origin_pack`]. Where `origin_pack` answers
    /// "who wins this id" (the body-reference anchor), this answers "who all ships
    /// it": a resource shadowed by a higher-precedence pack is still enumerated,
    /// which is what the pack-load freeze assertion needs to enforce **every**
    /// manifest-shipping constituent rather than only the precedence winner
    /// (`design/corpus-migration.md` → unified per-origin-pack manifest
    /// resolution). A single pack reports itself when it owns the id
    /// ([`read`] succeeds) and nothing otherwise; a composite overrides this to
    /// concatenate its constituents' owners in precedence order.
    ///
    /// [`origin_pack`]: PackSource::origin_pack
    /// [`read`]: PackSource::read
    fn origin_packs(&self, kind: PackResourceKind, id: &ResourceId) -> Vec<&dyn PackSource> {
        if self.read(kind, id).is_ok() {
            vec![self.as_pack_source()]
        } else {
            Vec::new()
        }
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

    /// The pack's **resolving directory path** — what `--explain` renders as the
    /// pack's "path" so the human sees the exact directory behind a composed pack.
    /// A directory-backed pack (`FilesystemPack`) overrides this to its root; the
    /// default is the [`EMBEDDED_PATH`] sentinel, since a pack with no on-disk root
    /// (the binary-embedded base) ships its bytes inside `jigc`. Display-only — never
    /// a hard-fail path (`design/multi-pack.md` → Provenance under N packs).
    fn resolving_path(&self) -> String {
        EMBEDDED_PATH.to_owned()
    }

    /// A **blake3 content-hash over this pack's bytes** — the per-pack content
    /// identity `--explain` renders alongside [`resolving_path`], so two packs that
    /// share an `id/version` but differ in content are still distinguishable
    /// (`design/multi-pack.md` → Provenance under N packs). Computed by hashing each
    /// `(kind, id, bytes)` in a **stable order** — kinds in a fixed sequence, ids in
    /// each kind's already-sorted [`list`] order — so the hash is `read_dir`-order
    /// invariant (increment-workflow hardening #7): a pack whose files were created
    /// out of id-order hashes identically to one created in order. Reuses
    /// [`crate::file_state::hash_bytes`] (the drift-hash primitive — no new
    /// dependency). Not overridden by `CompositePack`, which surfaces each
    /// constituent's hash via [`provenance_entries`] instead.
    ///
    /// [`list`]: PackSource::list
    /// [`provenance_entries`]: PackSource::provenance_entries
    fn content_hash(&self) -> String {
        // The fixed kind order — iteration must not depend on a `HashMap`'s order.
        const KINDS: [PackResourceKind; 5] = [
            PackResourceKind::Schemas,
            PackResourceKind::Workflows,
            PackResourceKind::Steps,
            PackResourceKind::Config,
            PackResourceKind::SchemaSnapshots,
        ];
        let mut hashed = Vec::new();
        for kind in KINDS {
            // `list` is sorted-by-id, so the per-kind id order is stable.
            for id in self.list(kind) {
                if let Ok(bytes) = self.read(kind, &id) {
                    // Frame each unit by its (kind, id) so distinct layouts that
                    // happen to share concatenated bytes still hash distinctly.
                    hashed.extend_from_slice(format!("{kind:?}\u{0}{id}\u{0}").as_bytes());
                    hashed.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
                    hashed.extend_from_slice(&bytes);
                }
            }
        }
        crate::file_state::hash_bytes(&hashed)
    }

    /// This pack's **provenance entry** — its [`resolving_path`] + [`content_hash`]
    /// — as a one-element `Vec`. A single pack reports exactly one entry, so the
    /// `--explain` provenance line degrades to one pack just as [`provenance_segments`]
    /// degrades to one segment. A composite (`CompositePack`) overrides this to
    /// concatenate its constituents' entries in precedence order (highest-precedence
    /// first) — the per-pack `path + content-hash` the multi-pack provenance line
    /// renders (`design/multi-pack.md` → Provenance under N packs).
    ///
    /// [`resolving_path`]: PackSource::resolving_path
    /// [`content_hash`]: PackSource::content_hash
    /// [`provenance_segments`]: PackSource::provenance_segments
    fn provenance_entries(&self) -> Vec<PackProvenance> {
        vec![PackProvenance {
            path: self.resolving_path(),
            content_hash: self.content_hash(),
        }]
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

    /// The default `origin_packs` is the singular ownership test: a pack that
    /// ships the id reports exactly itself; a pack that does not reports nothing
    /// (never a fallback `self`, unlike `origin_pack` — an owner-less id has no
    /// manifest-shipping constituent to walk).
    #[test]
    fn origin_packs_default_is_self_when_owning_else_empty() {
        let pack = seeded();
        let owners = pack.origin_packs(
            PackResourceKind::Workflows,
            &ResourceId::from("single-task"),
        );
        assert_eq!(owners.len(), 1, "an owning pack reports exactly itself");
        assert!(
            owners[0]
                .read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .is_ok(),
            "the reported owner ships the id",
        );
        assert!(
            pack.origin_packs(PackResourceKind::Steps, &ResourceId::from("absent"))
                .is_empty(),
            "a non-owning pack reports no owners",
        );
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
