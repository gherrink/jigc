//! `EmbeddedPack` — the MVP `PackSource` impl that serves the built-in dev pack
//! from bytes embedded in the `jigc` binary.
//!
//! Embed mechanism is `include_dir` (decided 2026-05-31; always-embedded, so what
//! you test is what ships). The pack tree lives in `crates/cli/pack/`, one
//! sub-directory per [`PackResourceKind`] (`workflows/`, `schemas/`, `steps/`,
//! `config/`); a resource's [`ResourceId`] is its file stem. The pack versions
//! with the release, so `pack_version` is the binary's `CARGO_PKG_VERSION`
//! (override-reconciliation: built-in pack-default version = binary version).
//! See `implementation/module-layout.md` → The dev pack's home.

use engine::packsource::{PackError, PackResourceKind, PackSource, ResourceId};
use engine::schema::{PackTypeDecl, Schema, SchemaError, load_schema_with_types};
use include_dir::{Dir, include_dir};
use std::ffi::OsString;
use std::path::PathBuf;

/// The `config/` resource id of the pack's field-type declarations (the M10
/// extension axis). A pack listing `(name, adjudicator)` entries here makes those
/// type spellings nameable by its schemas; the dev pack declares `code-anchor` →
/// `doc-code`. The engine ships none (the engine-empty invariant) — the CLI reads
/// this file and threads the set into schema loading.
const FIELD_TYPES_ID: &str = "field-types";

/// The pack-declared field types, read from `config/field-types.yaml` (a YAML
/// sequence of `{ name, adjudicator }`). An absent file is **no** declared types
/// (an empty set), never an error — a pack need not declare any. This is the set
/// every CLI schema load threads in via [`load_pack_schema`], so a pack-declared
/// `code-anchor` field resolves (and an undeclared type is rejected loudly by the
/// engine). See `document-type-schema.md` → Pack-declared field types.
pub fn pack_field_types(pack: &dyn PackSource) -> Result<Vec<PackTypeDecl>, SchemaError> {
    let Ok(bytes) = pack.read(PackResourceKind::Config, &ResourceId::from(FIELD_TYPES_ID)) else {
        return Ok(Vec::new());
    };
    let text = std::str::from_utf8(&bytes).map_err(|_| SchemaError::NotUtf8)?;
    Ok(serde_yaml_ng::from_str(text)?)
}

/// Load a doc-type schema from `bytes`, resolving its fields against the pack's own
/// field-type declarations — the single CLI entry point that replaces the bare
/// engine `load_schema` everywhere a *shipped* pack schema is parsed, so a
/// pack-declared `code-anchor` field (`adr.cites-code`, `spec.criteria/maps-to-test`)
/// resolves with its bound adjudicator. The engine stays domain-empty; the CLI feeds
/// the pack's declared set in here.
pub fn load_pack_schema(pack: &dyn PackSource, bytes: &[u8]) -> Result<Schema, SchemaError> {
    load_schema_with_types(bytes, &pack_field_types(pack)?)
}

/// The built-in dev pack, embedded at compile time from `crates/cli/pack/`.
static PACK: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/pack");

/// `PackSource` over the binary-embedded built-in dev pack.
pub struct EmbeddedPack;

impl EmbeddedPack {
    pub fn new() -> Self {
        EmbeddedPack
    }
}

impl Default for EmbeddedPack {
    fn default() -> Self {
        Self::new()
    }
}

/// The pack sub-directory that holds resources of `kind`.
fn kind_dir(kind: PackResourceKind) -> &'static str {
    match kind {
        PackResourceKind::Schemas => "schemas",
        PackResourceKind::Workflows => "workflows",
        PackResourceKind::Steps => "steps",
        PackResourceKind::Config => "config",
    }
}

impl PackSource for EmbeddedPack {
    fn pack_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let Some(dir) = PACK.get_dir(kind_dir(kind)) else {
            return Vec::new();
        };
        let mut ids: Vec<ResourceId> = dir
            .files()
            .filter_map(|f| f.path().file_stem())
            .filter_map(|stem| stem.to_str())
            .map(ResourceId::from)
            .collect();
        ids.sort();
        ids
    }

    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        let dir = PACK.get_dir(kind_dir(kind));
        let bytes = dir.and_then(|dir| {
            dir.files()
                .find(|f| f.path().file_stem().and_then(|s| s.to_str()) == Some(id.as_str()))
                .map(|f| f.contents().to_vec())
        });
        bytes.ok_or_else(|| PackError::NotFound {
            kind,
            id: id.clone(),
        })
    }
}

/// `pack_version` sentinel when a `FilesystemPack` dir declares no `version:`
/// (or has no `config/defaults.yaml`). `base-version` is narrative-only, so the
/// sentinel is harmless. See overrides.md → the `FilesystemPack` seam.
const FS_LOCAL_VERSION: &str = "fs-local";

/// The env var that selects a directory pack over the binary-embedded default.
/// See overrides.md → the `FilesystemPack` seam.
const PACK_DIR_ENV: &str = "JIGC_PACK_DIR";

/// The pre-cascade pack-assembly input: the ordered list of project-local pack
/// directories read from `<project_config_dir>/packs.yaml`, **highest-precedence
/// first** (earlier in the list = higher precedence). The composite `PackSource`
/// assembles these over the base pack (the base sits lowest).
///
/// This is an **optional** pre-cascade input, **not** a cascade knob: it cannot be
/// resolved by the cascade (the cascade resolves *over* the pack-set this selects).
/// So **absent file** and an **absent/empty `packs:` list** both yield `Vec::new()`
/// (the single-pack `[base]` floor), never an error. A **malformed** `packs.yaml`
/// (not valid YAML, or a `packs:` that is not a list of strings) is a **located**
/// `Err` naming the file — never a panic. See `design/multi-pack.md` → The pack-set.
///
/// Exercised by the unit tests now; wired into `make_pack()`'s pack-set assembly
/// in T3 of this increment (the production caller).
#[allow(dead_code)]
pub fn read_pack_list(project_config_dir: &std::path::Path) -> anyhow::Result<Vec<PathBuf>> {
    use anyhow::Context;

    let path = project_config_dir.join("packs.yaml");
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", path.display()));
        }
    };

    #[derive(serde::Deserialize)]
    struct PacksFile {
        #[serde(default)]
        packs: Vec<PathBuf>,
    }

    let parsed: PacksFile = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("{} is not a valid pack-set list", path.display()))?;
    Ok(parsed.packs)
}

/// The pack-source factory — the **single** production construction point for a
/// [`PackSource`]. Every production path (the orientation/compose front door, the
/// `jigc config` recording verbs, the task/doc working areas) routes through this
/// so the *recording* and *upgrade* paths read the **same** env-selected pack
/// (overrides.md → the `FilesystemPack` seam: "every production pack-source
/// construction goes through the factory").
///
/// `JIGC_PACK_DIR` set to a directory selects a [`FilesystemPack`] over that tree;
/// unset, the binary-embedded [`EmbeddedPack`] is the default, so output is
/// byte-identical to a build without the seam.
pub fn make_pack() -> Box<dyn PackSource> {
    make_pack_from(std::env::var_os(PACK_DIR_ENV))
}

/// The testable core of [`make_pack`]: select on an already-read env value rather
/// than reading the process environment, so the selection logic is exercised
/// without mutating global state (parallel-test-safe).
fn make_pack_from(pack_dir: Option<OsString>) -> Box<dyn PackSource> {
    match pack_dir {
        Some(dir) if !dir.is_empty() => Box::new(FilesystemPack::new(PathBuf::from(dir))),
        _ => Box::new(EmbeddedPack::new()),
    }
}

/// `PackSource` over a pack tree read live from a directory — the testability
/// seam for driving a genuine alternate pack (`v1 → v2`) through the built
/// binary, and independently useful for project-local packs. Selected by
/// `JIGC_PACK_DIR` via the pack-source factory. See overrides.md → the
/// `FilesystemPack` seam; module-layout.md → The dev pack's home.
///
/// The tree mirrors `EmbeddedPack`: one sub-directory per [`PackResourceKind`]
/// (`workflows/`, `schemas/`, `steps/`, `config/`); a resource's [`ResourceId`]
/// is its file stem.
pub struct FilesystemPack {
    root: PathBuf,
}

impl FilesystemPack {
    pub fn new(root: PathBuf) -> Self {
        FilesystemPack { root }
    }
}

impl PackSource for FilesystemPack {
    fn pack_version(&self) -> String {
        let path = self
            .root
            .join(kind_dir(PackResourceKind::Config))
            .join("defaults.yaml");
        let Ok(text) = std::fs::read_to_string(&path) else {
            return FS_LOCAL_VERSION.to_owned();
        };
        serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&text)
            .ok()
            .and_then(|value| {
                value
                    .get("version")
                    .and_then(serde_yaml_ng::Value::as_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| FS_LOCAL_VERSION.to_owned())
    }

    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let Ok(entries) = std::fs::read_dir(self.root.join(kind_dir(kind))) else {
            return Vec::new();
        };
        let mut ids: Vec<ResourceId> = entries
            .filter_map(Result::ok)
            .filter(|e| e.path().is_file())
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(ResourceId::from)
            })
            .collect();
        ids.sort();
        ids
    }

    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        let entries = std::fs::read_dir(self.root.join(kind_dir(kind))).ok();
        let bytes = entries.and_then(|entries| {
            entries
                .filter_map(Result::ok)
                .find(|e| {
                    e.path().is_file()
                        && e.path().file_stem().and_then(|s| s.to_str()) == Some(id.as_str())
                })
                .and_then(|e| std::fs::read(e.path()).ok())
        });
        bytes.ok_or_else(|| PackError::NotFound {
            kind,
            id: id.clone(),
        })
    }
}

/// An **ordered composite** [`PackSource`] over a pack-set, **highest-precedence
/// first** (earlier in the `Vec` wins same-id collisions). It is the assembled
/// `pack-default` layer the cascade resolves over: the listed packs
/// (`.jigc/config/packs:`) over the base pack (`JIGC_PACK_DIR`/`EmbeddedPack`,
/// implicitly last/lowest). It hides behind the existing `&dyn PackSource`, so
/// the ~22 call sites do not ripple.
///
/// Top-level **precedence-override** falls out of `read` (no separate
/// adjudicator): a colliding `knobs.yaml`/`commit`/workflow id resolves to the
/// winner's whole file. `list` is the **union deduped-by-id then sorted** — the
/// same stable sorted-by-id contract a single pack's `list()` already honours, so
/// the `[base]` floor and any union both emit ids in deterministic order
/// (no `HashSet` iteration order reaches output). `pack_version` is the
/// highest-precedence pack's. This task does **not** pack-localize body-references
/// (a *loser*-pack workflow's `{{include: step:X}}` is still mis-resolved here —
/// fixed in increment 2). See `design/multi-pack.md` → Collision resolution;
/// Where it sits.
pub struct CompositePack(Vec<Box<dyn PackSource>>);

impl CompositePack {
    /// Assemble a composite over `packs`, **highest-precedence first**. A
    /// single-element `Vec` is the byte-identity floor: its `list`/`read`/
    /// `pack_version` equal that one pack's.
    ///
    /// Exercised by the unit tests now; wired into `make_pack()`'s pack-set
    /// assembly in T3 of this increment (the production caller).
    #[allow(dead_code)]
    pub fn new(packs: Vec<Box<dyn PackSource>>) -> Self {
        CompositePack(packs)
    }
}

impl PackSource for CompositePack {
    /// The highest-precedence (first) pack's version. An empty pack-set cannot
    /// arise in production (the base is always present), but the empty-`Vec`
    /// version is the empty string rather than a panic.
    fn pack_version(&self) -> String {
        self.0.first().map(|p| p.pack_version()).unwrap_or_default()
    }

    /// The union of every pack's ids for `kind`, **deduped-by-id then sorted**.
    /// A `BTreeSet` keyed by the stable [`ResourceId`] gives both at once — never
    /// a `HashSet`, whose iteration order would leak into the emitted list
    /// (increment-workflow hardening #7).
    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let ids: std::collections::BTreeSet<ResourceId> =
            self.0.iter().flat_map(|p| p.list(kind)).collect();
        ids.into_iter().collect()
    }

    /// The **precedence-winner's** bytes: the first pack (highest-precedence)
    /// whose `read` succeeds. If no pack owns the id, a clean
    /// [`PackError::NotFound`] naming the requested `kind`/`id` — never the last
    /// pack's own error instance.
    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        self.0
            .iter()
            .find_map(|p| p.read(kind, id).ok())
            .ok_or_else(|| PackError::NotFound {
                kind,
                id: id.clone(),
            })
    }
}

/// The YAML for the 16 intrinsic per-check severity knobs, each floored at
/// `blocking`, generated from [`engine::knobs::INTRINSIC_CHECK_KEYS`]. A minimal
/// test pack appends this to its `config/knobs.yaml` so it satisfies the engine's
/// load-time intrinsic-floored assertion ([`engine::knobs::load_knobs`]) without
/// hand-listing the surface — and stays in sync if the intrinsic set changes.
#[cfg(test)]
pub(crate) fn intrinsic_knobs_yaml() -> String {
    engine::knobs::INTRINSIC_CHECK_KEYS
        .iter()
        .map(|k| {
            format!(
                "{k}:\n  type: enum\n  of: [blocking, warning, advisory]\n  default: blocking\n  floor: blocking\n"
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_pack_lists_and_reads_the_shipped_workflows() {
        let pack = EmbeddedPack::new();

        let workflows = pack.list(PackResourceKind::Workflows);
        assert!(
            workflows.contains(&ResourceId::from("single-task")),
            "the embedded pack must ship the `single-task` workflow; got {workflows:?}",
        );

        let bytes = pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from("single-task"),
            )
            .expect("the shipped `single-task` workflow reads back");
        assert!(
            !bytes.is_empty(),
            "the `single-task` workflow definition must be non-empty YAML",
        );
    }

    #[test]
    fn embedded_pack_read_of_a_missing_id_is_not_found() {
        let pack = EmbeddedPack::new();
        let err = pack
            .read(PackResourceKind::Workflows, &ResourceId::from("absent"))
            .expect_err("an id with no embedded file errors");
        assert_eq!(
            err,
            PackError::NotFound {
                kind: PackResourceKind::Workflows,
                id: ResourceId::from("absent"),
            },
        );
    }

    #[test]
    fn embedded_pack_version_is_the_binary_version() {
        let pack = EmbeddedPack::new();
        assert_eq!(pack.pack_version(), env!("CARGO_PKG_VERSION"));
    }

    /// Read a resource as UTF-8 text (pack definitions are text).
    fn read_text(pack: &EmbeddedPack, kind: PackResourceKind, id: &str) -> String {
        let bytes = pack
            .read(kind, &ResourceId::from(id))
            .unwrap_or_else(|e| panic!("resource `{id}` must read back: {e}"));
        String::from_utf8(bytes).expect("pack resources are UTF-8 text")
    }

    /// The single-task workflow ships the full MVP definition: spec-less `when`,
    /// `creates-task: true`, the `{type: adr, as: decision}` create-gate, and a
    /// body that is exactly the four ordered step includes. Golden over the bytes
    /// pins the canonical pack content (no serializer here — the file *is* the
    /// contract). See workflow-dialect.md → On-disk definition format.
    #[test]
    fn single_task_workflow_body_is_the_canonical_definition() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Workflows, "single-task");
        insta::assert_snapshot!(body, @r###"
        ---
        when: implement one scoped change end-to-end
        description: An end-to-end scoped change — locate, implement, optionally record a decision, and commit, all as one task.
        usage: the work is one coherent change you can hold in your head and carry from intent to commit in a single pass.
        creates-task: true
        allows-create: [{type: adr, as: decision}]
        ---
        {{ include: step:locate }}
        {{ include: step:implement }}
        {{ include: step:superseded-context }}
        {{ include: step:finalize }}
        "###);
    }

    /// The shipped dev pack authors `description:`/`usage:` on **every** workflow
    /// and **every** doctype it ships — the M11 pack-prose deliverable
    /// (`introspection.md` → Deliverable scope: skip-on-absent is the runtime
    /// contract, not a license to ship an under-narrated pack). Each definition is
    /// loaded through the **production** loader the binary uses (`load_workflow_def`
    /// for front-matter, `load_pack_schema` for the field-type-resolving doctype
    /// path), so this doubles as the clean real-binary pack-load proof: a typo'd
    /// key, a mis-nested field, or a `deny_unknown_fields` violation on any of the
    /// 9 workflows or 5 doctypes fails here. Asserts presence (`Some`), not the
    /// prose bytes — wording is review-policed, the per-schema goldens pin the
    /// bytes that ship.
    #[test]
    fn shipped_pack_narrates_every_workflow_and_doctype() {
        let pack = EmbeddedPack::new();

        let workflows = pack.list(PackResourceKind::Workflows);
        assert_eq!(
            workflows.len(),
            10,
            "the shipped pack must carry all 10 workflows; got {workflows:?}",
        );
        for id in &workflows {
            let bytes = pack
                .read(PackResourceKind::Workflows, id)
                .unwrap_or_else(|e| panic!("workflow `{}` reads back: {e}", id.as_str()));
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|e| panic!("workflow `{}` loads: {e:?}", id.as_str()));
            assert!(
                def.description.is_some(),
                "workflow `{}` must author a `description:`",
                id.as_str(),
            );
            assert!(
                def.usage.is_some(),
                "workflow `{}` must author a `usage:`",
                id.as_str(),
            );
        }

        let schemas = pack.list(PackResourceKind::Schemas);
        assert_eq!(
            schemas.len(),
            5,
            "the shipped pack must carry all 5 doctypes; got {schemas:?}",
        );
        for id in &schemas {
            let bytes = pack
                .read(PackResourceKind::Schemas, id)
                .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
            let schema = load_pack_schema(&pack, &bytes)
                .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
            assert!(
                schema.description.is_some(),
                "doctype `{}` must author a `description:`",
                schema.ty,
            );
            assert!(
                schema.usage.is_some(),
                "doctype `{}` must author a `usage:`",
                schema.ty,
            );
        }
    }

    /// list(Steps) yields the MVP step ids, sorted (the pack lists in stem
    /// order). The composer's includes resolve against exactly these — the four
    /// `single-task` steps, `implement-quick` (the ADR-free variant `quick-fix`
    /// includes), the router's `present-catalog` / `route-to-workflow`,
    /// `author-spec` (the `plan` workflow's create-gated spec-authoring step),
    /// `locate-from-spec` (the `implement-from-spec` workflow's spec-driven locate,
    /// distinct from the shared `locate`), `author-commit` (the fanned
    /// `sub-task` workflow's finalize-free commit-authoring step), plus the
    /// `project-setup` trio `develop-idea` / `author-prd` / `project-finalize`
    /// (the M9 new-project on-ramp), plus the `ingest-existing` pair `run-scan` /
    /// `review-verdicts` (the M9 existing-project on-ramp), plus `author-arch-doc`
    /// (the M13 architecture-documentation workflow's create-gated, item-authoring
    /// arch-doc step).
    #[test]
    fn embedded_pack_lists_the_mvp_steps() {
        let pack = EmbeddedPack::new();
        let steps = pack.list(PackResourceKind::Steps);
        assert_eq!(
            steps,
            vec![
                ResourceId::from("author-arch-doc"),
                ResourceId::from("author-commit"),
                ResourceId::from("author-prd"),
                ResourceId::from("author-spec"),
                ResourceId::from("develop-idea"),
                ResourceId::from("finalize"),
                ResourceId::from("implement"),
                ResourceId::from("implement-quick"),
                ResourceId::from("implement-tasks"),
                ResourceId::from("join-tasks"),
                ResourceId::from("locate"),
                ResourceId::from("locate-from-spec"),
                ResourceId::from("milestone-finalize"),
                ResourceId::from("present-catalog"),
                ResourceId::from("project-finalize"),
                ResourceId::from("review-verdicts"),
                ResourceId::from("route-to-workflow"),
                ResourceId::from("run-scan"),
                ResourceId::from("superseded-context"),
            ],
        );
    }

    #[test]
    fn step_locate_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "locate");
        insta::assert_snapshot!(body, @r###"
        Reason about the change. The intent is:
        {{ task.intent }}

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.
        "###);
    }

    #[test]
    fn step_implement_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "implement");
        insta::assert_snapshot!(body, @r###"
        Implement the change directly in the working tree. When done, stage the
        commit prose:

        {{ cli.set-commit-summary }}
        <<author: {{ task.commit#summary }}>>

        If a decision is warranted, create an ADR and author its slots:

        {{ cli.create-adr }}

        {{fill: extra-guidance}}
        "###);
    }

    #[test]
    fn step_superseded_context_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "superseded-context");
        insta::assert_snapshot!(body, @r###"
        If your decision supersedes an earlier one, here is that decision for
        reference — make your consequences explain what changes:
        {{ @task.decision.supersedes#decision }}
        "###);
    }

    #[test]
    fn step_finalize_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "finalize");
        insta::assert_snapshot!(body, @r###"
        Validate and commit the task as one logical commit:

        {{ cli.finalize-task }}
        "###);
    }

    /// list(Config) carries the `defaults` resource whose `default-workflow`
    /// points at `router` (the cascade knob, flipped from `single-task` once the
    /// router shipped — `DECISIONS.md` 2026-06-01 → M2 flips `default-workflow` to
    /// `router`), plus the `commands` catalog. The composer reads both.
    #[test]
    fn embedded_pack_config_carries_defaults_and_commands() {
        let pack = EmbeddedPack::new();
        let config = pack.list(PackResourceKind::Config);
        assert!(
            config.contains(&ResourceId::from("defaults")),
            "the pack config layer must ship a `defaults` resource; got {config:?}",
        );
        assert!(
            config.contains(&ResourceId::from("commands")),
            "the pack config layer must ship the `commands` catalog; got {config:?}",
        );

        let defaults = read_text(&pack, PackResourceKind::Config, "defaults");
        insta::assert_snapshot!(defaults, @r###"
        pack-id: dev
        default-workflow: router
        "###);

        // The commands catalog is present and readable (its parser arrives with
        // the composer; here we only pin its presence + canonical bytes).
        let commands = read_text(&pack, PackResourceKind::Config, "commands");
        assert!(
            commands.contains("set-commit-summary"),
            "the commands catalog must define the workflow's command-refs; got:\n{commands}",
        );
    }

    /// The pack-default layer ships its closed, typed knob *declaration* as the
    /// `knobs` Config resource (`{key, type, of?, default}`, reusing the
    /// document-type `FieldType` vocabulary). The loader (later in this
    /// increment) seeds the `PackDefaultLayer` scalar surface from it: the
    /// closed key set + each knob's materialized default. Golden over the bytes
    /// pins the canonical declared surface — `default-workflow` (enum, default
    /// `router`) + the full per-check `validation.*.severity` surface (the 20
    /// inventory rows, validation.md → MVP check inventory) plus the two M4
    /// per-probe keys retained as additive defaults. `pack-id` is **not** a knob
    /// (it is pack identity, read for the provenance header) — it stays in
    /// `defaults.yaml`, asserted absent here. See overrides.md → Scalar knobs
    /// (On-disk declaration — config/knobs.yaml); storage.md → Config layout.
    #[test]
    fn embedded_pack_config_declares_the_knob_surface() {
        let pack = EmbeddedPack::new();
        let config = pack.list(PackResourceKind::Config);
        assert!(
            config.contains(&ResourceId::from("knobs")),
            "the pack-default layer must ship a `knobs` declaration resource; got {config:?}",
        );

        let knobs = read_text(&pack, PackResourceKind::Config, "knobs");
        insta::assert_snapshot!(knobs, @r###"
        # pack/config/knobs.yaml — the closed, typed knob surface the cascade
        # resolves. One entry per settable key, reusing the document-type
        # `FieldType` vocabulary (`type` + optional `of`) so a `scalar-set` is
        # adjudicated by the same `check_value` the doc write path uses. The
        # loader seeds the PackDefaultLayer scalar surface from this file: the
        # closed key set (what `scalar-set` may target) + each knob's
        # materialized default. `pack-id` is NOT a knob — it is pack identity,
        # not a project-overridable value, so it stays in defaults.yaml.
        # The `validation.*.severity` keys are the per-check severity surface; their
        # defaults + intrinsic-ness are governed by the single source of truth,
        # validation.md → MVP check inventory (20 checks across 6 categories). The two
        # `validation.<probe>.severity` per-probe keys are retained from M4 as additive
        # per-probe *defaults* (never a rename) so an M4-authored manifest still
        # resolves; they sit alongside the per-check keys.
        # See overrides.md → Scalar knobs / Soft-rejection; storage.md → Config layout.
        default-workflow:
          type: enum
          of: [router, single-task, quick-fix, plan, implement-from-spec]
          default: router

        # --- finalize.fan-out.* — the milestone commit-shaping knob (M8) ---
        # squash: how a fan-out (milestone) finalize shapes the commit. `true`
        # (default) = ONE aggregate commit with the CLI-synthesized structural
        # message (the M7 form, byte-identical to the no-knob path). `false` = one
        # commit per sub-task in id-sorted order rendering each sub-task's authored
        # commit doc, plus the parent's synthesized aggregate. A tunable knob (no
        # floor). See finalize.md → `fan-out` finalize (the two squash modes).
        finalize.fan-out.squash:
          type: bool
          default: "true"

        # --- per-probe severity defaults (M4, retained — additive) ---
        validation.workflow-refs.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.file-state.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking

        # --- workflow-refs.* (9, intrinsic) ---
        validation.workflow-refs.placeholder-resolves.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.include-resolves.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.command-ref-resolves.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.include-cycle-absent.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.at-marker-on-non-scalar.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.run-marker-not-shadowed.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.spawn-marker-not-shadowed.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.fan-out-join-paired.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.workflow-refs.body-include-only.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking

        # --- pack-probe-integrity.* (3, intrinsic — the enforced meta-findings) ---
        # A probe that timed out, crashed, or returned malformed output cannot be
        # trusted to have validated anything; demoting these would let a misbehaving
        # probe pass silently (validation.md → What 'intrinsic' means mechanically).
        # `sandbox-violation` is NOT declared — deferred with OS-level sandboxing.
        validation.pack-probe-integrity.timeout.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.pack-probe-integrity.crash.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.pack-probe-integrity.malformed-output.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking

        # --- schema-conformance.* (4, intrinsic) ---
        validation.schema-conformance.ref-resolves.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.schema-conformance.required-slot-present.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.schema-conformance.required-field-present.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking
        validation.schema-conformance.field-value-conformant.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking

        # --- file-state.hash-matches (1, tunable) ---
        validation.file-state.hash-matches.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking

        # --- schema-completeness.inverse-cardinality (1, tunable; advisory at task scope) ---
        validation.schema-completeness.inverse-cardinality.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory

        # --- override-default.* (3, tunable from M6; blocking-by-default) ---
        validation.override-default.target-exists.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.override-default.target-unchanged.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.override-default.basis-recorded.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking

        # --- commit-rendering.* (2, tunable; advisory-by-default convention checks) ---
        validation.commit-rendering.line-limit-subject.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory
        validation.commit-rendering.line-limit-body.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory

        # --- doc-code.* (2, tunable; M10 — the pack-provided doc↔code probe) ---
        # blocking-by-default (a dangling anchor is a real integrity failure) but
        # cascade-tunable, NOT floored — a project may rationally demote to warning.
        # Unlike the floor-locked pack-probe-integrity.* meta-findings above.
        validation.doc-code.symbol-exists.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.doc-code.criterion-maps-to-test.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        "###);

        // `pack-id` is a non-knob identity field — it lives in defaults.yaml,
        // never the knob surface (overrides.md → "pack-id is not a knob"). A
        // top-level YAML key is a non-indented `<key>:` line; assert no such
        // line declares `pack-id` (comment mentions don't count).
        assert!(
            !knobs
                .lines()
                .any(|l| l.starts_with("pack-id:") || l.starts_with("pack-id ")),
            "`pack-id` is pack identity, not a settable knob; it must not be declared in knobs.yaml",
        );
    }

    /// The pack-default layer declares the pack's own identity: its
    /// `pack-id` is `dev`. This is what makes the `Pack: dev/<version>`
    /// provenance segment cascade-sourced rather than a CLI constant — the pack
    /// names itself. See overrides.md → pack-default layer carries pack id.
    #[test]
    fn embedded_pack_config_declares_pack_id() {
        let pack = EmbeddedPack::new();
        let defaults = read_text(&pack, PackResourceKind::Config, "defaults");
        assert!(
            defaults.lines().any(|l| l.trim() == "pack-id: dev"),
            "the pack config must declare `pack-id: dev`; got:\n{defaults}",
        );
    }

    mod pack_list {
        use super::super::*;
        use std::path::{Path, PathBuf};

        /// A throwaway directory that removes itself on drop.
        struct TempDir(PathBuf);

        impl TempDir {
            fn new() -> Self {
                let mut path = std::env::temp_dir();
                path.push(format!(
                    "jigc-packlist-unit-{}-{:?}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos(),
                ));
                std::fs::create_dir_all(&path).expect("create temp dir");
                TempDir(path)
            }

            fn path(&self) -> &Path {
                &self.0
            }
        }

        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        /// A two-entry `packs:` list yields both dirs in declared order
        /// (highest-precedence first — the list order is preserved verbatim).
        #[test]
        fn two_entry_list_preserves_declared_order() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"packs:\n  - /opt/jigc-packs/methodology\n  - /opt/jigc-packs/extra\n",
            )
            .expect("seed packs.yaml");

            let list = read_pack_list(dir.path()).expect("a valid packs.yaml reads back");
            assert_eq!(
                list,
                vec![
                    PathBuf::from("/opt/jigc-packs/methodology"),
                    PathBuf::from("/opt/jigc-packs/extra"),
                ],
            );
        }

        /// An absent `packs.yaml` is the common cold-start case: the empty pack-set
        /// (the `[base]` floor), never an error.
        #[test]
        fn absent_file_is_empty() {
            let dir = TempDir::new();
            let list = read_pack_list(dir.path()).expect("an absent packs.yaml is not an error");
            assert!(
                list.is_empty(),
                "absent file => empty pack-set; got {list:?}"
            );
        }

        /// An explicit empty list (`packs: []`) is also the empty pack-set — a
        /// present-but-empty selection is inert, not an error.
        #[test]
        fn empty_list_is_empty() {
            let dir = TempDir::new();
            std::fs::write(dir.path().join("packs.yaml"), b"packs: []\n")
                .expect("seed empty packs.yaml");
            let list = read_pack_list(dir.path()).expect("`packs: []` is not an error");
            assert!(
                list.is_empty(),
                "`packs: []` => empty pack-set; got {list:?}"
            );
        }

        /// A present file with no `packs:` key at all is still the empty pack-set
        /// (the key defaults to empty) — absent key === absent file.
        #[test]
        fn absent_packs_key_is_empty() {
            let dir = TempDir::new();
            std::fs::write(dir.path().join("packs.yaml"), b"# nothing here\n")
                .expect("seed keyless packs.yaml");
            let list = read_pack_list(dir.path()).expect("a missing `packs:` key is not an error");
            assert!(
                list.is_empty(),
                "absent `packs:` key => empty; got {list:?}"
            );
        }

        /// Garbage YAML is a **located** `Err` naming the file — never a panic. The
        /// hostile-input pass: the reader is the selection input the composite
        /// assembles over, so it must fail cleanly on malformed bytes.
        #[test]
        fn garbage_is_a_located_err() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"packs: : : not valid : yaml ][\n",
            )
            .expect("seed garbage packs.yaml");

            let err = read_pack_list(dir.path()).expect_err("garbage packs.yaml is a clean Err");
            let msg = format!("{err:#}");
            assert!(
                msg.contains("packs.yaml"),
                "the error must locate the offending file; got: {msg}",
            );
        }

        /// A `packs:` that is the wrong shape (a scalar, not a list of paths) is
        /// likewise a located `Err`, not a panic — the wrong-type hostile case.
        #[test]
        fn wrong_shape_packs_is_a_located_err() {
            let dir = TempDir::new();
            std::fs::write(dir.path().join("packs.yaml"), b"packs: not-a-list\n")
                .expect("seed wrong-shape packs.yaml");

            let err = read_pack_list(dir.path()).expect_err("a non-list `packs:` is a clean Err");
            let msg = format!("{err:#}");
            assert!(
                msg.contains("packs.yaml"),
                "the error must locate the offending file; got: {msg}",
            );
        }
    }

    mod factory {
        use super::super::*;
        use std::ffi::OsString;
        use std::path::{Path, PathBuf};

        /// A throwaway directory that removes itself on drop.
        struct TempDir(PathBuf);

        impl TempDir {
            fn new() -> Self {
                let mut path = std::env::temp_dir();
                path.push(format!(
                    "jigc-factory-unit-{}-{:?}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos(),
                ));
                std::fs::create_dir_all(&path).expect("create temp dir");
                TempDir(path)
            }

            fn path(&self) -> &Path {
                &self.0
            }
        }

        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        /// With `JIGC_PACK_DIR` unset, the factory yields an `EmbeddedPack`-backed
        /// source: its `list`/`read`/`pack_version` equal `EmbeddedPack`'s, so a
        /// no-env build is byte-identical to one without the seam.
        #[test]
        fn unset_env_yields_an_embedded_backed_source() {
            let pack = make_pack_from(None);
            let embedded = EmbeddedPack::new();

            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                embedded.list(PackResourceKind::Workflows),
                "the unset-env factory must list exactly what EmbeddedPack lists",
            );
            assert_eq!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                ),
                embedded.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                ),
                "the unset-env factory must read exactly what EmbeddedPack reads",
            );
            assert_eq!(
                pack.pack_version(),
                embedded.pack_version(),
                "the unset-env factory's pack_version must equal EmbeddedPack's",
            );
        }

        /// An empty `JIGC_PACK_DIR=` falls through to the embedded default rather
        /// than reading an empty path (an unset-equivalent value is inert).
        #[test]
        fn empty_env_yields_an_embedded_backed_source() {
            let pack = make_pack_from(Some(OsString::new()));
            let embedded = EmbeddedPack::new();
            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                embedded.list(PackResourceKind::Workflows),
            );
            assert_eq!(pack.pack_version(), embedded.pack_version());
        }

        /// With `JIGC_PACK_DIR=<dir>` the factory yields a `FilesystemPack` reading
        /// that directory: a workflow seeded only on disk lists and reads back, and
        /// the directory's `config/defaults.yaml` `version:` is the `pack_version`
        /// (distinct from the embedded binary version) — proving an alternate pack
        /// drives the built binary.
        #[test]
        fn set_env_yields_a_filesystem_pack_reading_the_dir() {
            let dir = TempDir::new();
            let wf = dir.path().join("workflows");
            std::fs::create_dir_all(&wf).expect("mk workflows/");
            std::fs::write(wf.join("only-on-disk.yaml"), b"when: from disk\n").expect("seed wf");
            let cfg = dir.path().join("config");
            std::fs::create_dir_all(&cfg).expect("mk config/");
            std::fs::write(cfg.join("defaults.yaml"), b"pack-id: dev\nversion: 9.9.9\n")
                .expect("seed defaults");

            let pack = make_pack_from(Some(OsString::from(dir.path())));

            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                vec![ResourceId::from("only-on-disk")],
                "the set-env factory must list the on-disk directory pack, not the embedded one",
            );
            assert_eq!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("only-on-disk"),
                )
                .expect("the disk-only workflow reads back"),
                b"when: from disk\n",
            );
            assert_eq!(
                pack.pack_version(),
                "9.9.9",
                "the set-env factory's pack_version comes from the dir's defaults.yaml version key",
            );
            assert_ne!(
                pack.pack_version(),
                EmbeddedPack::new().pack_version(),
                "the directory pack reports a version distinct from the embedded binary version",
            );
        }
    }

    mod filesystem_pack {
        use super::super::*;
        use std::path::{Path, PathBuf};

        /// A throwaway directory that removes itself on drop (the project's
        /// no-tempfile pattern, mirrored from `setup.rs`).
        struct TempDir(PathBuf);

        impl TempDir {
            fn new() -> Self {
                let mut path = std::env::temp_dir();
                let unique = format!(
                    "jigc-fspack-unit-{}-{:?}",
                    std::process::id(),
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos(),
                );
                path.push(unique);
                std::fs::create_dir_all(&path).expect("create temp dir");
                TempDir(path)
            }

            fn path(&self) -> &Path {
                &self.0
            }
        }

        impl Drop for TempDir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }

        /// Write a pack resource file into `<root>/<kind_dir>/<stem>.<ext>`.
        fn seed(root: &Path, kind_dir: &str, file: &str, bytes: &[u8]) {
            let dir = root.join(kind_dir);
            std::fs::create_dir_all(&dir).expect("create kind dir");
            std::fs::write(dir.join(file), bytes).expect("seed pack resource");
        }

        /// `list` returns the seeded file stems sorted, matching the
        /// `EmbeddedPack` stem=ResourceId convention.
        #[test]
        fn list_returns_seeded_stems_sorted() {
            let dir = TempDir::new();
            seed(dir.path(), "workflows", "single-task.yaml", b"a");
            seed(dir.path(), "workflows", "router.yaml", b"b");

            let pack = FilesystemPack::new(dir.path().to_owned());
            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                vec![ResourceId::from("router"), ResourceId::from("single-task")],
            );
        }

        /// An absent kind directory lists nothing (parity with `EmbeddedPack`).
        #[test]
        fn list_of_absent_kind_dir_is_empty() {
            let dir = TempDir::new();
            let pack = FilesystemPack::new(dir.path().to_owned());
            assert!(pack.list(PackResourceKind::Steps).is_empty());
        }

        /// `read` round-trips the seeded bytes; an absent id is `NotFound`.
        #[test]
        fn read_round_trips_bytes_and_absent_is_not_found() {
            let dir = TempDir::new();
            seed(
                dir.path(),
                "workflows",
                "single-task.yaml",
                b"workflow: single-task",
            );

            let pack = FilesystemPack::new(dir.path().to_owned());
            let bytes = pack
                .read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .expect("seeded id reads back");
            assert_eq!(bytes, b"workflow: single-task");

            let err = pack
                .read(PackResourceKind::Workflows, &ResourceId::from("absent"))
                .expect_err("an id with no file errors");
            assert_eq!(
                err,
                PackError::NotFound {
                    kind: PackResourceKind::Workflows,
                    id: ResourceId::from("absent"),
                },
            );
        }

        /// `pack_version` is the `version:` value from `config/defaults.yaml`.
        #[test]
        fn pack_version_reads_the_defaults_version_key() {
            let dir = TempDir::new();
            seed(
                dir.path(),
                "config",
                "defaults.yaml",
                b"pack-id: dev\nversion: 0.4.0\n",
            );

            let pack = FilesystemPack::new(dir.path().to_owned());
            assert_eq!(pack.pack_version(), "0.4.0");
        }

        /// A `defaults.yaml` without a `version:` key falls back to the
        /// `fs-local` sentinel.
        #[test]
        fn pack_version_without_version_key_is_the_sentinel() {
            let dir = TempDir::new();
            seed(dir.path(), "config", "defaults.yaml", b"pack-id: dev\n");

            let pack = FilesystemPack::new(dir.path().to_owned());
            assert_eq!(pack.pack_version(), "fs-local");
        }

        /// An absent `defaults.yaml` altogether falls back to the sentinel.
        #[test]
        fn pack_version_without_defaults_file_is_the_sentinel() {
            let dir = TempDir::new();
            let pack = FilesystemPack::new(dir.path().to_owned());
            assert_eq!(pack.pack_version(), "fs-local");
        }

        /// The trait is usable behind a `&dyn PackSource`, like `EmbeddedPack`.
        #[test]
        fn trait_is_object_usable() {
            let dir = TempDir::new();
            seed(dir.path(), "workflows", "single-task.yaml", b"a");
            let pack = FilesystemPack::new(dir.path().to_owned());
            let as_dyn: &dyn PackSource = &pack;
            assert_eq!(as_dyn.list(PackResourceKind::Workflows).len(), 1);
        }
    }

    mod composite {
        use super::super::*;
        use std::collections::HashMap;

        /// A trivial in-memory `PackSource` — drives the composite without
        /// touching the filesystem. `resources` is a `HashMap` so its own
        /// iteration order is *unstable*: a composite that leaked container
        /// order into `list` would flake against this fixture, which is the
        /// point (hardening #7 — the emitted union must be sorted, not in
        /// hash-iteration order).
        struct MemPack {
            version: String,
            resources: HashMap<(PackResourceKind, ResourceId), Vec<u8>>,
        }

        impl MemPack {
            fn new(version: &str) -> Self {
                MemPack {
                    version: version.to_owned(),
                    resources: HashMap::new(),
                }
            }

            fn with(mut self, kind: PackResourceKind, id: &str, bytes: &[u8]) -> Self {
                self.resources
                    .insert((kind, ResourceId::from(id)), bytes.to_vec());
                self
            }
        }

        impl PackSource for MemPack {
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

        /// Pack A (highest-precedence): the colliding `commit` doctype (bytes
        /// `A-commit`) plus a non-colliding `adr`. Pack B (lower): the same
        /// `commit` id with **divergent** bytes (`B-commit`) plus a
        /// non-colliding `spec`. The composite is `[A, B]` — A wins.
        fn two_pack_composite() -> CompositePack {
            let a = MemPack::new("a-ver")
                .with(PackResourceKind::Schemas, "commit", b"A-commit")
                .with(PackResourceKind::Schemas, "adr", b"A-adr");
            let b = MemPack::new("b-ver")
                .with(PackResourceKind::Schemas, "commit", b"B-commit")
                .with(PackResourceKind::Schemas, "spec", b"B-spec");
            CompositePack::new(vec![Box::new(a), Box::new(b)])
        }

        /// The colliding id reads the **highest-precedence** pack's bytes — the
        /// precedence-override that falls out of composite `read`.
        #[test]
        fn colliding_id_reads_the_precedence_winner() {
            let composite = two_pack_composite();
            assert_eq!(
                composite
                    .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
                    .expect("the colliding id reads back from the winner"),
                b"A-commit",
            );
        }

        /// A non-colliding id owned only by the **lower-precedence** pack still
        /// reads back — the union read, not just the winner's resources.
        #[test]
        fn loser_only_id_reads_back_via_union() {
            let composite = two_pack_composite();
            assert_eq!(
                composite
                    .read(PackResourceKind::Schemas, &ResourceId::from("spec"))
                    .expect("the loser-only id reads back"),
                b"B-spec",
            );
        }

        /// `list` is the union deduped-by-id then sorted: the colliding `commit`
        /// appears **once**, alongside both packs' non-colliding ids, in sorted
        /// order. The golden pins the exact emitted sequence (adr, commit, spec)
        /// — the dedup *and* the sort.
        #[test]
        fn list_dedups_the_collision_and_sorts_the_union() {
            let composite = two_pack_composite();
            assert_eq!(
                composite.list(PackResourceKind::Schemas),
                vec![
                    ResourceId::from("adr"),
                    ResourceId::from("commit"),
                    ResourceId::from("spec"),
                ],
            );
        }

        /// An id no pack owns is a clean `NotFound` naming the requested
        /// kind/id, never a panic.
        #[test]
        fn unowned_id_is_not_found() {
            let composite = two_pack_composite();
            let err = composite
                .read(PackResourceKind::Schemas, &ResourceId::from("absent"))
                .expect_err("an id no pack owns errors");
            assert_eq!(
                err,
                PackError::NotFound {
                    kind: PackResourceKind::Schemas,
                    id: ResourceId::from("absent"),
                },
            );
        }

        /// `pack_version` is the highest-precedence (first) pack's.
        #[test]
        fn pack_version_is_the_highest_precedence_pack() {
            let composite = two_pack_composite();
            assert_eq!(composite.pack_version(), "a-ver");
        }

        /// The in-isolation **floor**: `Composite([single])` is byte-identical
        /// to the single pack — `list`/`read`/`pack_version` all equal it. This
        /// is the headline regression (the one-pack path must be unperturbed by
        /// the composite wrapper); proven here against an in-memory pack and
        /// again in-binary against the real embedded pack in later tasks.
        #[test]
        fn single_pack_composite_equals_the_pack() {
            let lone = MemPack::new("only-ver")
                .with(PackResourceKind::Workflows, "single-task", b"lone-wf")
                .with(PackResourceKind::Workflows, "router", b"lone-router");
            let reference = MemPack::new("only-ver")
                .with(PackResourceKind::Workflows, "single-task", b"lone-wf")
                .with(PackResourceKind::Workflows, "router", b"lone-router");

            let composite = CompositePack::new(vec![Box::new(lone)]);

            assert_eq!(
                composite.list(PackResourceKind::Workflows),
                reference.list(PackResourceKind::Workflows),
                "Composite([single]).list must equal the single pack's list",
            );
            assert_eq!(
                composite
                    .read(
                        PackResourceKind::Workflows,
                        &ResourceId::from("single-task")
                    )
                    .expect("the lone pack's id reads through the composite"),
                reference
                    .read(
                        PackResourceKind::Workflows,
                        &ResourceId::from("single-task")
                    )
                    .expect("the single pack reads its id"),
                "Composite([single]).read must equal the single pack's read",
            );
            assert_eq!(
                composite.pack_version(),
                reference.pack_version(),
                "Composite([single]).pack_version must equal the single pack's",
            );
        }
    }
}
