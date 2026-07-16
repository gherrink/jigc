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
    let mut schema = load_schema_with_types(bytes, &pack_field_types(pack)?)?;
    // The per-doc schema-version stamp (M34): inject the engine-declared stamp field
    // into every **persisted** doctype whose **governing manifest entry** declares it
    // frozen — the convergence point shared by the freeze gate, `ingest::load_schemas`,
    // and `all_schemas`, so the stamp rides every CLI schema-load uniformly while the
    // bare engine `load_schema` (and `arb_doc()` fuzz) stays stamp-free. Gated on the
    // per-origin rule ([`governing_version`]: the doctype's own origin pack's manifest,
    // the pack's own declaration, not an engine-baked list — the engine-empty
    // invariant) AND *persisted* — a `location:` folder home OR a literal `placement:`
    // home (the M38 changelog root-`CHANGELOG.md` form). So the transient `commit`
    // (in the manifest, neither location nor placement) is excluded and a manifest-less
    // pack injects nothing (`design/corpus-migration.md` → The schema-version stamp).
    if (schema.location.is_some() || schema.placement.is_some())
        && governing_version(pack, schema.ty.as_str()).is_some()
    {
        engine::schema::inject_schema_version_stamp(&mut schema);
    }
    Ok(schema)
}

/// Load a doctype's **prior-version schema shape** from the versioned snapshot store
/// (`schema-snapshots/<ty>.v<version>.yaml`, the M34 [`PackResourceKind::SchemaSnapshots`]
/// kind) — the *actual* declared schema at `version`, which a v1→v2 **structural**
/// change cannot derive from the current shape. The verb sources `from` through this
/// per committed doc by stamp, so the engine diffs **two real declared schemas**
/// (the determinism boundary in its strongest form — never a hand-written recipe;
/// `design/corpus-migration.md` → Prior-schema sourcing).
///
/// Parsed through [`load_pack_schema`] (not the bare engine loader) so the prior shape
/// resolves the pack's field-types **and** gets the schema-version stamp injected
/// **identically** to the current schema — the consequence that makes a v1→v2 diff
/// emit only the structural change (the stamp present in both `from` and `to`, so its
/// transition is a value-bump, not a spurious add-field; T2 handles the bump).
///
/// A **missing** snapshot is a **located** `Err` naming the resource (never a panic);
/// the consumer (the T2 verb) turns it into a per-doc `blocked`-with-route rather than
/// a silent `already-current`.
///
/// The producer half of a clean producer→consumer seam (like Inc-2's
/// `promote_slot_to_repeatable`): its only non-test caller is the T2 verb, which wires
/// it to production within this same increment, so the `allow(dead_code)` is shed at
/// increment end.
#[allow(dead_code)]
pub fn load_prior_schema(pack: &dyn PackSource, ty: &str, version: u32) -> anyhow::Result<Schema> {
    use anyhow::Context;

    let id = ResourceId::from(format!("{ty}.v{version}"));
    let bytes = pack
        .read(PackResourceKind::SchemaSnapshots, &id)
        .with_context(|| {
            format!(
                "no prior-schema snapshot `{ty}.v{version}` (schema-snapshots/{ty}.v{version}.yaml)"
            )
        })?;
    load_pack_schema(pack, &bytes)
        .with_context(|| format!("prior-schema snapshot `{ty}.v{version}` is malformed"))
}

/// A pack's **own** `config/schema-manifest.yaml`, best-effort: an **absent** or
/// **malformed** manifest is `None` — the field-types-absent precedent: only a pack
/// that ships a valid freeze manifest stamps/gates. The loud manifest authority is
/// the freeze gate ([`assert_schema_freeze`]); this read stays best-effort so a
/// non-freeze pack-load never errors here. Reads the pack directly (its own bytes),
/// never a composed surface — the per-origin rule's building block.
fn own_manifest(pack: &dyn PackSource) -> Option<engine::manifest::Manifest> {
    let bytes = pack
        .read(
            PackResourceKind::Config,
            &ResourceId::from(SCHEMA_MANIFEST_ID),
        )
        .ok()?;
    serde_yaml_ng::from_slice(&bytes).ok()
}

/// The doctype's **governing manifest entry's** schema-version — the **unified
/// per-origin-pack resolution rule** (M40 A1, `design/corpus-migration.md` → the
/// split-brain close): a doctype `ty` is governed by the manifest of
/// `origin_pack(Schemas, ty)`, the pack whose schema definition wins — never by the
/// single Config-resource precedence winner's manifest (under which one of two
/// manifest-shipping packs went silently unread, and a colliding doctype could be
/// judged by a manifest that never froze the shape actually composed). `None` when
/// the origin pack ships no valid manifest, or its manifest omits `ty` — the
/// freeze-exempt case.
fn governing_version(pack: &dyn PackSource, ty: &str) -> Option<u32> {
    let owner = pack.origin_pack(PackResourceKind::Schemas, &ResourceId::from(ty));
    own_manifest(owner)?
        .doctypes
        .into_iter()
        .find(|e| e.ty == ty)
        .map(|e| e.schema_version)
}

/// The composed pack-set's `doctype → schema-version` map — the "current manifest
/// version" the store-scope schema-conformance detector routes each non-conformant doc
/// against (`design/validation.md` → Version-aware routing; `design/corpus-migration.md` →
/// The schema-version stamp). The **per-doctype governed union**: each shipped doctype
/// resolves through [`governing_version`] to **its own origin pack's** manifest entry,
/// so two manifest-shipping packs each govern exactly the doctypes they win. Best-effort
/// throughout: a doctype whose origin pack ships no manifest (or omits it) is simply
/// absent from the map — a manifest-less pack-set routes nothing, never an error.
pub(crate) fn frozen_doctype_versions(
    pack: &dyn PackSource,
) -> std::collections::BTreeMap<String, u32> {
    let mut out = std::collections::BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        if let Some(version) = governing_version(pack, id.as_str()) {
            out.insert(id.as_str().to_owned(), version);
        }
    }
    out
}

/// The composed pack-set's `doctype → shipped prior-version schema shapes` map — the
/// **parse-against-a-prior** arm of the store sweep's managed-vs-foreign classifier
/// (`design/validation.md` → The managed-vs-foreign discriminator; the engine's
/// `classify_provenance`). For each versioned doctype it loads every shipped snapshot below
/// the current manifest version (`schema-snapshots/<ty>.v<k>.yaml`, `1..current`), so an
/// **unstamped** committed doc that parses against a shape jigc once shipped is read as a
/// **managed, v0-era** doc — not as a foreign file to adopt.
///
/// Best-effort like [`frozen_doctype_versions`]: a doctype whose snapshot is absent or
/// malformed simply contributes no prior shape (the classifier then narrows to *stamp or
/// parses-against-current* for it), never an error — the sweep is read-only and must not fail
/// on a pack that ships no snapshot store.
pub(crate) fn prior_doctype_schemas(
    pack: &dyn PackSource,
    versions: &std::collections::BTreeMap<String, u32>,
) -> std::collections::BTreeMap<String, Vec<Schema>> {
    let mut out = std::collections::BTreeMap::new();
    for (ty, current) in versions {
        let shapes: Vec<Schema> = (1..*current)
            .filter_map(|version| load_prior_schema(pack, ty, version).ok())
            .collect();
        if !shapes.is_empty() {
            out.insert(ty.clone(), shapes);
        }
    }
    out
}

/// The `config/` resource id of a pack's frozen doctype-set manifest (the M33
/// freeze artifact). A pack that ships this file gets its declared doctype shapes
/// checked against it at pack-load by [`assert_schema_freeze`]; a pack that omits
/// it is unchecked — the freeze records what a pack *declares* frozen, never a
/// blanket requirement. See `design/corpus-migration.md` → The freeze.
const SCHEMA_MANIFEST_ID: &str = "schema-manifest";

/// The **runtime pack-load freeze assertion** — the productive-path sibling of the
/// build-time freeze gate ([`tests::shipped_schema_manifest_matches_the_frozen_doctype_set`])
/// and of the intrinsic-floor knob assertion, fired inside the pack-source factory
/// ([`make_pack`]) so an un-migrated schema-shape change is **blocked** at *every*
/// door — read, report, compose and write alike — not merely reported
/// (`design/corpus-migration.md` → The enforcement gate fires at pack-load — review
/// Finding 3; *not* the report-only `validate` store sweep). It guarded only the
/// compose front door until M42 Inc 6, which left `validate` / `describe` /
/// `doc schema` / `migrate-corpus` clean over a drifted pack and let
/// `jigc milestone create` **commit** a new record at exit 0.
///
/// Recomputes each doctype's `schema-hash` over each **manifest-owning pack's own
/// shipped schema shapes** and compares against that pack's manifest
/// ([`engine::manifest::check`]) — failing loudly on a drift, an added, or a removed
/// doctype. It walks **every** constituent that ships a manifest, enumerated via
/// [`origin_packs`](PackSource::origin_packs) (the M40 per-origin unification —
/// under the old single-`origin_pack` rule, one of two manifest-shipping packs went
/// **silently unenforced**), and checks each **in isolation** — never the composed
/// cascade: the freeze records what each pack ships, so a higher-precedence pack
/// that shadows a frozen doctype with a divergent shape (the methodology pack's own
/// `commit`) does not perturb the dev pack's freeze, and vice versa. Schemas load
/// through the field-type-resolving [`load_pack_schema`] against their own owning
/// pack, and the read is shadow- / `docs-root`-independent (it reads the pack
/// directly, not the project-resolved [`crate::start::CascadeDefs::all_schemas`]).
///
/// An **absent** manifest is skipped (no owners → `Ok(())`) — the
/// field-types-absent precedent ([`pack_field_types`]): a seeded / composed pack
/// that ships no manifest stays unchecked, so only a pack that opts into the freeze
/// is held to it. That opt-out is **wholesale**, and it is the only one: a pack that
/// *does* ship a manifest is held to all of it, including the `slug-rule:` block —
/// an omitted block is [`engine::manifest::ManifestError::SlugRuleUndeclared`], not a
/// second, quieter opt-out (M42 audit; `design/storage.md` → Identity → *The slug rule
/// is itself a versioned rule*).
pub fn assert_schema_freeze(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    // Every constituent that SHIPS a manifest (for a non-composite: the pack itself
    // when it ships one, else nothing — the manifest-less inert path).
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        // Owners are enumerated by a successful read, so this read succeeds; a racy
        // filesystem pack that lost the file between the two reads simply skips.
        let Ok(manifest_bytes) = owner.read(PackResourceKind::Config, &manifest_id) else {
            continue;
        };
        let manifest: engine::manifest::Manifest = serde_yaml_ng::from_slice(&manifest_bytes)
            .context("config/schema-manifest.yaml is not a valid freeze manifest")?;

        // The manifest-owning pack's OWN shipped doctype shapes, keyed by type, loaded
        // through the field-type-resolving loader against that same pack.
        let mut schemas = std::collections::BTreeMap::new();
        for id in owner.list(PackResourceKind::Schemas) {
            let bytes = owner
                .read(PackResourceKind::Schemas, &id)
                .with_context(|| format!("the `{}` schema is unreadable", id.as_str()))?;
            let schema = load_pack_schema(owner, &bytes)
                .with_context(|| format!("the `{}` schema is malformed", id.as_str()))?;
            schemas.insert(schema.ty.clone(), schema);
        }

        engine::manifest::check(&manifest, &schemas)
            .map_err(|err| anyhow::anyhow!("pack-load freeze check failed: {err}"))?;
    }
    Ok(())
}

/// The **eager workflow-front-matter sweep** — the M43 pack-load fence home
/// (`design/surface-contract.md` → The fences: pack-load posture). Workflows
/// parse lazily on the compose path, so before this sweep a front-matter defect
/// surfaced only when its workflow was composed; the fences need every shipped
/// workflow's front-matter **loaded at pack-load**, at every door.
///
/// Scope mirrors [`assert_schema_freeze`]: **manifest-shipping constituents,
/// each checked in isolation** (enumerated via
/// [`origin_packs`](PackSource::origin_packs)) — a pack opts into the pack-load
/// fences by shipping a `config/schema-manifest.yaml`, so a manifest-less
/// seeded / project-local pack stays on skip-on-absent, never an error. Each
/// owner's workflows load through the production [`load_workflow_def`] (a
/// malformed front-matter now blocks at pack-load, naming the workflow).
///
/// **The suppression fence (law 2):** every `selectable: false` workflow must
/// declare `suppressed: {reason, expires}` — a hidden capability carries a
/// machine-visible reason that can expire (`never` is legal for a
/// permanent-by-design hide; a *malformed* block is already rejected by the
/// loader's required-shape check). Missing ⇒ fail, naming the workflow — the
/// `decided-task` lesson made mechanical.
///
/// **The catalog shape fence (the style guide's floor):** every **selectable
/// work-workflow** (`creates-task: true && selectable: true` — the router
/// catalog's population) must carry non-empty `when:`/`description:`/`usage:`,
/// and its `when:` must be mechanically shaped per [`assert_when_shape`]. The
/// scope is exactly the catalog: a non-selectable or `creates-task: false`
/// workflow keeps `introspection.md`'s skip-on-absent, so project-authored
/// workflows in a manifest-less pack are doubly outside the fence. The lazy
/// catalog assert in `start.rs` (`selectable_workflows`) stays; this factory
/// fence fires first, at every door.
fn assert_workflow_front_matter(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        for id in owner.list(PackResourceKind::Workflows) {
            let bytes = owner
                .read(PackResourceKind::Workflows, &id)
                .with_context(|| format!("the `{}` workflow is unreadable", id.as_str()))?;
            let def = engine::compose::load_workflow_def(&bytes).map_err(|finding| {
                anyhow::anyhow!(
                    "pack-load workflow-front-matter sweep failed on `{}`: {}",
                    id.as_str(),
                    finding.message,
                )
            })?;
            if !def.selectable && def.suppressed.is_none() {
                anyhow::bail!(
                    "pack-load suppression fence failed: workflow `{}` declares \
                     `selectable: false` with no `suppressed:` block — a hidden capability \
                     must carry a machine-visible reason \
                     (design/surface-contract.md → The suppression fence)\n\
                     route: add `suppressed: {{reason: <why it is hidden>, expires: never | \
                     <the condition that un-hides it>}}` to the workflow's front-matter",
                    id.as_str(),
                );
            }
            if def.creates_task && def.selectable {
                for (field, value) in [
                    ("when", &def.when),
                    ("description", &def.description),
                    ("usage", &def.usage),
                ] {
                    if value.as_deref().is_none_or(|v| v.trim().is_empty()) {
                        anyhow::bail!(
                            "pack-load catalog shape fence failed: selectable work-workflow \
                             `{}` carries no `{field}:` — the router catalog and `describe` \
                             narrate from these fields \
                             (design/surface-contract.md → The catalog shape fence)\n\
                             route: author a non-empty `{field}:` in the workflow's front-matter",
                            id.as_str(),
                        );
                    }
                }
                if let Some(when) = &def.when {
                    assert_when_shape(id.as_str(), when)?;
                }
            }
        }
    }
    Ok(())
}

/// The `when:` catalog line's char cap (the catalog shape fence's length half).
/// The line interpolates mid-sentence into the router catalog beside its
/// neighbours, so it must stay a short situation phrase
/// (`design/workflow-dialect.md` → The `when:`-line craft convention); the
/// longest shipped selectable `when:` today is 104 chars
/// (`architecture-documentation`), so the cap leaves authoring headroom without
/// licensing a paragraph.
const CATALOG_WHEN_MAX_CHARS: usize = 120;

/// The `when:` mechanical-shape half of the catalog shape fence: one line,
/// period-less (it interpolates mid-sentence), and at most
/// [`CATALOG_WHEN_MAX_CHARS`] chars — the `workflow-dialect.md` craft rules'
/// checkable half. The when-NOT-clause assert is deliberately absent (prose
/// semantics — declined at M43 Settle #7; the when-NOT discipline lives on
/// `usage:` via the review checklist).
fn assert_when_shape(workflow: &str, when: &str) -> anyhow::Result<()> {
    let bail = |defect: &str| {
        anyhow::bail!(
            "pack-load catalog shape fence failed: selectable work-workflow `{workflow}`'s \
             `when:` line {defect} — the line interpolates mid-sentence into the router \
             catalog (design/workflow-dialect.md → The `when:`-line craft convention; \
             design/surface-contract.md → The catalog shape fence)\n\
             route: rewrite the `when:` as one short, period-less situation phrase",
        )
    };
    let when = when.trim();
    if when.contains('\n') {
        return bail("spans multiple lines");
    }
    if when.ends_with('.') {
        return bail("ends with a period");
    }
    let chars = when.chars().count();
    if chars > CATALOG_WHEN_MAX_CHARS {
        return bail(&format!(
            "is {chars} chars, over the {CATALOG_WHEN_MAX_CHARS}-char cap"
        ));
    }
    Ok(())
}

/// The built-in dev pack, embedded at compile time from `crates/cli/pack/`.
static PACK: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/pack");

/// The methodology pack (the M12 second pack — `roadmap`/`planning`/`completion`/…),
/// embedded at compile time from `packs/methodology/` by a **second** `include_dir!`.
/// Pure-YAML data (no `target/` build-tree, so the M20 bloat lesson does not apply);
/// composed in-binary, never extracted. Selected by [`EmbeddedPack::methodology`].
/// See `design/multi-pack.md` → Embedded second pack; `module-layout.md` → Pack
/// distribution.
static METHODOLOGY: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../packs/methodology");

/// `PackSource` over a binary-embedded pack tree. **Field-carrying:** the selected
/// `&'static Dir` is the dev base ([`new`](EmbeddedPack::new)) or the methodology
/// tree ([`methodology`](EmbeddedPack::methodology)), so the two in-binary packs can
/// be composed. Both selectors report `pack_version = CARGO_PKG_VERSION` — the pack
/// versions with the binary release regardless of which tree is selected
/// (`multi-pack.md` → Version ties to the binary).
pub struct EmbeddedPack {
    dir: &'static Dir<'static>,
}

impl EmbeddedPack {
    /// The dev base pack (the lowest-precedence foundation).
    pub fn new() -> Self {
        EmbeddedPack { dir: &PACK }
    }

    /// The methodology pack — the second embedded tree, composed dev-highest at
    /// `jigc setup` behind the `compose-embedded-methodology` marker. Consumed by the
    /// pack-source factory ([`make_pack_from_marker`]) when the marker is set.
    pub fn methodology() -> Self {
        EmbeddedPack { dir: &METHODOLOGY }
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
        PackResourceKind::SchemaSnapshots => "schema-snapshots",
    }
}

impl PackSource for EmbeddedPack {
    fn pack_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let Some(dir) = self.dir.get_dir(kind_dir(kind)) else {
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
        let dir = self.dir.get_dir(kind_dir(kind));
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
/// Read by [`make_pack`]'s CWD-discovery (the production caller) and by the unit
/// tests.
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

/// The setup-written **compose marker**: whether `<project_config_dir>/packs.yaml`
/// carries `compose-embedded-methodology: true`. When set, [`make_pack`] composes
/// the two in-binary packs as `[dev ▸ methodology]` (dev-highest); when absent or
/// `false`, the pack-set is exactly `[dev]` — byte-identical to today
/// (`design/multi-pack.md` → Embedded second pack + setup auto-wiring).
///
/// This is a **NET-NEW** parse of the same `packs.yaml` [`read_pack_list`] reads —
/// no new discovery walk. It reads a *different* key (the marker, not `packs:`), so
/// **absent file** and an **absent/false marker key** both yield `Ok(false)` (the
/// single-pack floor), never an error; a **malformed** `packs.yaml` is a **located**
/// `Err` naming the file (parity with [`read_pack_list`]). The marker carries **no
/// path** (both packs are in-binary), so it sidesteps the CWD-relative-path landmine
/// of the listed-pack form entirely.
pub fn read_compose_marker(project_config_dir: &std::path::Path) -> anyhow::Result<bool> {
    use anyhow::Context;

    let path = project_config_dir.join("packs.yaml");
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", path.display()));
        }
    };

    #[derive(serde::Deserialize)]
    struct MarkerFile {
        #[serde(rename = "compose-embedded-methodology", default)]
        compose_embedded_methodology: bool,
    }

    let parsed: MarkerFile = serde_yaml_ng::from_str(&text)
        .with_context(|| format!("{} is not a valid pack-set list", path.display()))?;
    Ok(parsed.compose_embedded_methodology)
}

/// The pack-source factory — the **single** production construction point for a
/// [`PackSource`]. Every production path (the orientation/compose front door, the
/// `jigc config` recording verbs, the task/doc working areas) routes through this
/// so the *recording* and *upgrade* paths read the **same** env-selected pack
/// (overrides.md → the `FilesystemPack` seam: "every production pack-source
/// construction goes through the factory").
///
/// The returned source is an ordered [`CompositePack`] over the pack-set
/// (multi-pack.md → The pack-set): the project's listed packs
/// (`.jigc/config/packs:`, highest-precedence first) over the **base** pack — a
/// [`FilesystemPack`] if `JIGC_PACK_DIR` is set, else the binary-embedded
/// [`EmbeddedPack`] — which sits implicitly **last/lowest**. The factory stays
/// **zero-arg** and **CWD-discovers** the project config (walk up from the process
/// CWD to the repo root's `.jigc/config/`) so the ~22 call sites do not ripple;
/// each caller already operates on the process CWD, so the discovered pack-set is
/// the one its in-repo project dir would name (proven by the T3 two-pack
/// real-binary load, not assumed).
///
/// **Cold-start floor:** absent/empty `packs.yaml` (or no discoverable project
/// config — most `make_pack()` callers may run before any project layer exists)
/// yields a composite of **exactly `[base]`**, which `CompositePack` makes
/// byte-identical to the single-pack path. Discovery is therefore best-effort: a
/// missing repo/config dir is the empty pack-set, never an error. A *malformed*
/// `packs.yaml` is a located error surfaced by [`read_pack_list`] — propagated,
/// not swallowed.
///
/// **The freeze gate fires here** ([`assert_schema_freeze`]): the assembled pack-set
/// is checked against each constituent's own `config/schema-manifest.yaml` before it
/// is handed to any caller, so a drifted frozen schema blocks **every** door rather
/// than the compose front door alone (M42 Inc 6 — the M33 deferral whose trigger,
/// "packs are ever loaded from the filesystem in production", fired when the M14
/// multi-pack surface started reading pack *directories* from `packs.yaml`). Inert
/// for a manifest-less pack.
pub fn make_pack() -> anyhow::Result<Box<dyn PackSource>> {
    let listed = discover_pack_list().unwrap_or_else(|err| {
        // A malformed `packs.yaml` is a real authoring fault; surface it rather
        // than silently falling back to the base. (An *absent* file is `Ok(vec![])`
        // from `read_pack_list`, so this arm fires only on genuine corruption.)
        eprintln!("warning: {err:#}");
        Vec::new()
    });
    let compose_methodology = discover_compose_marker().unwrap_or_else(|err| {
        // Same fail-loud-but-don't-abort posture as the list discovery above: a
        // malformed `packs.yaml` is surfaced, then treated as no marker (the floor).
        eprintln!("warning: {err:#}");
        false
    });
    let pack_dir = std::env::var_os(PACK_DIR_ENV);
    // A purely in-binary pack-set — exactly `[dev]` or `[dev ▸ methodology]`,
    // no filesystem constituent — is immutable in-process, so the eager
    // front-matter sweep below memoizes per composition shape.
    let embedded_only = listed.is_empty() && pack_dir.as_ref().is_none_or(|dir| dir.is_empty());
    let pack = make_pack_from_marker(pack_dir, listed, compose_methodology)?;
    assert_schema_freeze(pack.as_ref())?;

    // The eager workflow-front-matter sweep (M43, `design/surface-contract.md`
    // → The fences): memoized for the two embedded compositions (their bytes
    // cannot change within a process; `make_pack` has ~38 call sites), recomputed
    // whenever a filesystem pack is in the set (its tree is live-mutable).
    // `anyhow::Error` is not `Clone`, so the cache carries the rendered message.
    if embedded_only {
        static EMBEDDED_SWEEPS: [std::sync::OnceLock<Result<(), String>>; 2] =
            [std::sync::OnceLock::new(), std::sync::OnceLock::new()];
        EMBEDDED_SWEEPS[usize::from(compose_methodology)]
            .get_or_init(|| {
                assert_workflow_front_matter(pack.as_ref()).map_err(|err| format!("{err:#}"))
            })
            .clone()
            .map_err(|msg| anyhow::anyhow!(msg))?;
    } else {
        assert_workflow_front_matter(pack.as_ref())?;
    }
    Ok(pack)
}

/// CWD-discover the project's pre-cascade pack-set: walk up from the process CWD to
/// the repo root (the dir holding `.git`), then read `<root>/.jigc/config/packs.yaml`
/// via [`read_pack_list`]. No repo / no `.jigc/config/` is the empty pack-set
/// (`Ok(vec![])`) — the cold-start floor — not an error; only a malformed
/// `packs.yaml` is an `Err`.
fn discover_pack_list() -> anyhow::Result<Vec<PathBuf>> {
    let Ok(cwd) = std::env::current_dir() else {
        return Ok(Vec::new());
    };
    let Some(repo_root) = cwd.ancestors().find(|dir| dir.join(".git").exists()) else {
        return Ok(Vec::new());
    };
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        return Ok(Vec::new());
    }
    read_pack_list(&project_config)
}

/// CWD-discover the project's **compose marker** — the same `packs.yaml` walk as
/// [`discover_pack_list`], reading the `compose-embedded-methodology` key via
/// [`read_compose_marker`]. No repo / no `.jigc/config/` is no marker (`Ok(false)`)
/// — the single-pack floor — not an error; only a malformed `packs.yaml` is an
/// `Err`. No second filesystem walk is introduced: the factory reads the file it
/// already discovers for the listed-pack set.
fn discover_compose_marker() -> anyhow::Result<bool> {
    let Ok(cwd) = std::env::current_dir() else {
        return Ok(false);
    };
    let Some(repo_root) = cwd.ancestors().find(|dir| dir.join(".git").exists()) else {
        return Ok(false);
    };
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        return Ok(false);
    }
    read_compose_marker(&project_config)
}

/// The marker-aware testable core of [`make_pack`]: assemble the composite from
/// already-read inputs (the `JIGC_PACK_DIR` env value, the listed pack dirs, and the
/// setup-written compose marker) rather than reading the process environment / CWD,
/// so the assembly is exercised without mutating global state (parallel-test-safe).
///
/// **Marker set** (`compose_methodology == true`) **and no explicit `JIGC_PACK_DIR`**
/// composes the two in-binary packs as `CompositePack([dev, methodology])` — **dev
/// FIRST = dev-highest**, the inverse of the listed>base convention, so the real
/// `commit`/`default-workflow` collisions resolve to dev's bytes (`design/multi-pack.md`
/// → Embedded second pack: dev-highest). This path composes **exactly**
/// `[dev ▸ methodology]`.
///
/// **Marker set *and* a non-empty `packs:` list is a loud refusal** (M42 Inc 6 fix).
/// Combining the marker with listed filesystem packs is **out of scope** by design —
/// a listed pack's precedence against the two in-binary packs is a deferred
/// composition (`design/multi-pack.md` → Embedded second pack: "combining the marker
/// with additional listed packs is out of M21 scope"). It used to be honored by
/// **silently discarding the whole listed set**, which is the lying-route class: in a
/// `jigc setup`-initialized project (setup writes the marker into *every* project's
/// `packs.yaml`) an operator's listed pack was never loaded, so a drifted frozen
/// schema in it had nothing to check it and every door — including the committing
/// `jigc milestone create` — ran at exit 0. A pack-set the loader cannot honor is now
/// an error carrying both exits (drop the marker, or drop the list); the *silent* drop
/// is gone. `jigc setup` no longer manufactures the combination
/// (`crate::setup::write_compose_marker`).
///
/// **`JIGC_PACK_DIR` supersedes the marker.** A non-empty `JIGC_PACK_DIR` is the
/// explicit/dogfood channel: when set it selects the base and the marker does **not**
/// fire, so the composition is exactly the pre-M21 `make_pack_from` result (e.g.
/// methodology-alone for the dogfood) — and, the marker being inert, listed packs
/// compose over it exactly as on the M14 path (no refusal: nothing is dropped).
/// `design/worked-examples.md` flow 15.
///
/// **Marker absent/false** is the M14 path delegated to [`make_pack_from`]: listed
/// packs first (highest-precedence), base last (`JIGC_PACK_DIR`/`EmbeddedPack`). With
/// no listed packs that is `Composite([base])` — the byte-identity floor, byte-for-byte
/// what shipped before this task.
fn make_pack_from_marker(
    pack_dir: Option<OsString>,
    listed_dirs: Vec<PathBuf>,
    compose_methodology: bool,
) -> anyhow::Result<Box<dyn PackSource>> {
    // `JIGC_PACK_DIR` is the explicit/dogfood channel and supersedes the marker:
    // when it selects a base the embedded `[dev ▸ methodology]` composition stays
    // inert (`design/multi-pack.md` → Embedded second pack; `worked-examples.md`
    // flow 15). An empty `JIGC_PACK_DIR=` is *not* an explicit selection — it falls
    // through to the embedded base, matching `make_base_pack`'s own empty handling.
    let explicit_base = pack_dir.as_ref().is_some_and(|dir| !dir.is_empty());
    if compose_methodology && !explicit_base {
        if !listed_dirs.is_empty() {
            let listed = listed_dirs
                .iter()
                .map(|dir| dir.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "unsupported pack-set in `.jigc/config/packs.yaml`: \
                 `compose-embedded-methodology: true` cannot be combined with a `packs:` list \
                 (listed: {listed}) — the marker composes the two in-binary packs \
                 `[dev ▸ methodology]`, and a listed pack's precedence against that pair is a \
                 deferred composition (`design/multi-pack.md` → Embedded second pack), so \
                 honoring the marker would drop every listed pack\n\
                 route: keep the listed packs by removing `compose-embedded-methodology: true` \
                 from `.jigc/config/packs.yaml`, or keep the embedded `[dev ▸ methodology]` pair \
                 by removing the `packs:` list"
            );
        }
        // Exactly `[dev ▸ methodology]`, both in-binary; dev first = dev-highest.
        return Ok(Box::new(CompositePack::new(vec![
            Box::new(EmbeddedPack::new()),
            Box::new(EmbeddedPack::methodology()),
        ])));
    }
    Ok(make_pack_from(pack_dir, listed_dirs))
}

/// The marker-free testable core: assemble the M14 composite from the `JIGC_PACK_DIR`
/// env value + the listed pack dirs (the [`make_pack_from_marker`] no-marker arm).
///
/// The pack-set is **listed packs first (highest-precedence), base last (lowest)**:
/// each listed dir becomes a [`FilesystemPack`]; the base is a [`FilesystemPack`]
/// over `JIGC_PACK_DIR` if set, else the [`EmbeddedPack`]. With no listed packs the
/// composite is `[base]` — the byte-identity floor.
fn make_pack_from(pack_dir: Option<OsString>, listed_dirs: Vec<PathBuf>) -> Box<dyn PackSource> {
    let mut packs: Vec<Box<dyn PackSource>> = listed_dirs
        .into_iter()
        .map(|dir| Box::new(FilesystemPack::new(dir)) as Box<dyn PackSource>)
        .collect();
    packs.push(make_base_pack(pack_dir));
    Box::new(CompositePack::new(packs))
}

/// Construct the **base** pack — the lowest-precedence foundation every listed pack
/// composes over. `JIGC_PACK_DIR` set to a non-empty directory selects a
/// [`FilesystemPack`] over that tree; unset (or empty), the binary-embedded
/// [`EmbeddedPack`].
fn make_base_pack(pack_dir: Option<OsString>) -> Box<dyn PackSource> {
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

    /// A directory pack's resolving path **is** its root — what `--explain` renders
    /// so the human sees the exact directory composed (`design/multi-pack.md` →
    /// Provenance under N packs). Lossy on a non-UTF-8 root (display-only, never a
    /// hard-fail path). The base `EmbeddedPack` keeps the trait default
    /// (`<embedded>`); only a directory-backed pack reports a real path.
    fn resolving_path(&self) -> String {
        self.root.display().to_string()
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
    /// Wired into [`make_pack`]'s pack-set assembly (the production caller) and
    /// exercised directly by the unit tests.
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

    /// How many constituents own `(kind, id)` — `--explain` reads this to tell a
    /// genuine cross-pack collision (≥2 owners, precedence-override adjudicated a
    /// winner) from a single owner (`design/multi-pack.md` → Provenance: where a
    /// collision was adjudicated, name the winner). Counts every constituent whose
    /// `read` succeeds, not just the precedence winner.
    fn owner_count(&self, kind: PackResourceKind, id: &ResourceId) -> usize {
        self.0.iter().filter(|p| p.read(kind, id).is_ok()).count()
    }

    /// The constituent pack that **owns** `(kind, id)` — the **first
    /// (highest-precedence)** pack whose [`read`](PackSource::read) succeeds, the
    /// same pack the precedence `read` selects. This is the body-reference
    /// resolution anchor a composed definition resolves its `{{include: step:X}}`
    /// / `{{cli.X}}` / field-type names against (`design/multi-pack.md` →
    /// Pack-local body-reference resolution), so a *loser*-pack workflow composes
    /// **its own** steps/catalog rather than the precedence-winner's divergent
    /// ones. When no constituent owns the id the composite returns `self`, so a
    /// dangling reference still flows to the existing not-found path (a clean
    /// [`PackError::NotFound`] from `read`), never a panic. A single-element
    /// composite returns that one pack — origin = the pack (the floor).
    fn origin_pack(&self, kind: PackResourceKind, id: &ResourceId) -> &dyn PackSource {
        self.0
            .iter()
            .find(|p| p.read(kind, id).is_ok())
            .map(|p| p.as_ref())
            .unwrap_or(self)
    }

    /// **Every** constituent that ships `(kind, id)`, highest-precedence first —
    /// the plural sibling of [`origin_pack`](PackSource::origin_pack): a resource
    /// shadowed by a higher-precedence pack is still enumerated. Delegates to each
    /// constituent's own `origin_packs` (a nested composite flattens), so the walk
    /// reaches every genuine owner. The freeze assertion
    /// ([`assert_schema_freeze`](crate::pack::assert_schema_freeze)) reads this to
    /// enforce every manifest-shipping constituent, never just the Config-resource
    /// precedence winner. No owner is the empty `Vec` — there is nothing to walk.
    fn origin_packs(&self, kind: PackResourceKind, id: &ResourceId) -> Vec<&dyn PackSource> {
        self.0
            .iter()
            .flat_map(|p| p.origin_packs(kind, id))
            .collect()
    }

    /// Each constituent pack's own `(pack-id, version)` segment, **in precedence
    /// order** (highest-precedence first) — the composed-set provenance the
    /// multi-pack `Pack:` header renders. A single-element composite yields exactly
    /// that one pack's segment, so the byte-identity floor holds on the provenance
    /// axis (`design/multi-pack.md` → Provenance). Each constituent's id is read
    /// from *its own* `config/defaults`, never the precedence-winner's, so a loser
    /// pack still names itself in the header.
    fn provenance_segments(&self) -> Vec<(String, String)> {
        self.0
            .iter()
            .flat_map(|p| p.provenance_segments())
            .collect()
    }

    /// Each constituent pack's own provenance entry (resolving path + blake3
    /// content-hash), **in precedence order** (highest-precedence first) — the
    /// per-pack `path + content-hash` the multi-pack `--explain` line renders so the
    /// human sees the exact pack inputs behind a deterministic outcome
    /// (`design/multi-pack.md` → Provenance under N packs). A single-element composite
    /// yields exactly that one pack's entry, so the floor degrades to one entry just
    /// as [`provenance_segments`](PackSource::provenance_segments) degrades to one
    /// segment. Each entry's path + hash come from *its own* constituent, never the
    /// precedence-winner's — a loser pack still names its own directory and bytes.
    fn provenance_entries(&self) -> Vec<engine::packsource::PackProvenance> {
        self.0.iter().flat_map(|p| p.provenance_entries()).collect()
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

    /// `load_prior_schema` reads a versioned snapshot
    /// (`schema-snapshots/<ty>.v<N>.yaml`) through the field-type-resolving
    /// [`load_pack_schema`], so the prior shape resolves pack field-types **and** gets
    /// the schema-version stamp injected **identically** to a current schema — the
    /// producer half of the T2 sourcing seam. The fixture's manifest declares `prd`
    /// frozen, so the persisted prior shape is stamped (asserted present), and the
    /// load is byte-for-byte equal to the same bytes run through `load_pack_schema`.
    #[test]
    fn load_prior_schema_reads_a_versioned_snapshot_through_load_pack_schema() {
        let fixture =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/prior-schema-prd");
        let pack = FilesystemPack::new(fixture);

        let prior = load_prior_schema(&pack, "prd", 1).expect("the prd.v1 snapshot loads");

        // Identical to the same bytes run through `load_pack_schema` directly — one
        // load path, so field-type resolution + stamp injection are byte-identical.
        let bytes = pack
            .read(
                PackResourceKind::SchemaSnapshots,
                &ResourceId::from("prd.v1"),
            )
            .expect("the prd.v1 snapshot reads back");
        let direct = load_pack_schema(&pack, &bytes).expect("the snapshot bytes load");
        assert_eq!(prior, direct);

        // The prior shape is the real (fixed-slot) `prd`, with the stamp injected.
        assert_eq!(prior.ty, "prd");
        assert!(prior.location.is_some());
        let has_stamp = prior.sections.iter().any(|s| match &s.body {
            engine::schema::SectionBody::Simple { fields, .. } => fields
                .iter()
                .any(|f| f.id == engine::schema::SCHEMA_VERSION_FIELD),
            engine::schema::SectionBody::Repeatable { .. } => false,
        });
        assert!(
            has_stamp,
            "the loaded prior schema must carry the injected schema-version stamp",
        );
    }

    /// A missing snapshot (`prd.v9`) is a **located** error naming the requested
    /// resource — never a panic (the hostile-input pass, increment-workflow #3).
    #[test]
    fn load_prior_schema_of_a_missing_snapshot_is_a_located_error() {
        let fixture =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/prior-schema-prd");
        let pack = FilesystemPack::new(fixture);

        let err = load_prior_schema(&pack, "prd", 9).expect_err("a missing snapshot errors");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("prd.v9"),
            "the error must locate the missing snapshot; got: {msg}",
        );
    }

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

    /// The embedded `PACK` carries **only** real pack content — never the
    /// `doc-code` probe's source tree (and its gitignored multi-hundred-MB
    /// `target/`). The probe source lives outside the `include_dir!` root
    /// (`crates/cli/probes/`, not `pack/probes/`), so the embed sweeps no
    /// `probes/` directory. See module-layout.md → Probe distribution (the
    /// de-bloat site).
    #[test]
    fn embedded_pack_carries_no_probes_directory() {
        assert!(
            PACK.get_dir("probes").is_none(),
            "the embedded PACK must not carry a `probes/` entry — the probe source \
             lives outside the include_dir! root",
        );
    }

    /// Read a resource as UTF-8 text (pack definitions are text).
    fn read_text(pack: &EmbeddedPack, kind: PackResourceKind, id: &str) -> String {
        let bytes = pack
            .read(kind, &ResourceId::from(id))
            .unwrap_or_else(|e| panic!("resource `{id}` must read back: {e}"));
        String::from_utf8(bytes).expect("pack resources are UTF-8 text")
    }

    /// The single-task workflow ships the full MVP definition: spec-less `when`,
    /// `creates-task: true`, the `{type: adr, as: decision}, {type: changelog, as:
    /// change}` create-gate (M22 fold-in), and a body that is exactly the four
    /// ordered step includes. Golden over the bytes pins the canonical pack content
    /// (no serializer here — the file *is* the contract). See workflow-dialect.md →
    /// On-disk definition format.
    #[test]
    fn single_task_workflow_body_is_the_canonical_definition() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Workflows, "single-task");
        insta::assert_snapshot!(body, @r###"
        ---
        when: implement one scoped change end-to-end, recording the decisions it makes
        description: An end-to-end scoped change — locate, implement, optionally record a decision, and commit, all as one task.
        usage: the work is one coherent change you can hold in your head and carry from intent to commit in a single pass.
        creates-task: true
        allows-create: [{type: adr, as: decision}, {type: changelog, as: change}]
        ---
        {{ include: step:locate }}
        {{ include: step:implement }}
        {{ include: step:record-changelog }}
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
    /// 11 workflows or 6 doctypes fails here. Asserts presence (`Some`), not the
    /// prose bytes — wording is review-policed, the per-schema goldens pin the
    /// bytes that ship.
    #[test]
    fn shipped_pack_narrates_every_workflow_and_doctype() {
        let pack = EmbeddedPack::new();

        let workflows = pack.list(PackResourceKind::Workflows);
        assert_eq!(
            workflows.len(),
            16,
            "the shipped pack must carry all 16 workflows; got {workflows:?}",
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
            6,
            "the shipped pack must carry all 6 doctypes; got {schemas:?}",
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

    /// The shipped `changelog` doctype is the M22 doctype-expansion deliverable: a
    /// `singleton: true` Keep-a-Changelog doc at the literal root `CHANGELOG.md`
    /// (`placement`, `display-title: Changelog`, relocated there at schema v2), edge-free,
    /// with a multi-word `unreleased-changes` section (single-level repeatable
    /// change-groups) and a two-level `releases` section (each release nests a
    /// repeatable `changes` of change-groups — the `Leaf::Repeatable` target), plus
    /// an item-level `date` set-on-create and an optional `link` field. Golden over
    /// the bytes pins the canonical pack content (the file *is* the contract — no
    /// serializer here). See design/changelog.md → The `changelog` doctype.
    #[test]
    fn changelog_schema_is_the_canonical_doctype() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Schemas, "changelog");
        insta::assert_snapshot!(body, @r#"
        # changelog — the doctype-expansion (M22) project doctype: a Keep-a-Changelog
        # singleton maintained over time. A running SINGLETON that lives at the literal
        # root `CHANGELOG.md` (a `placement` doctype — the canonical KaC home, bypassing
        # docs-root and the slug; design/storage.md → Placement), so a re-`create`
        # deterministically targets the same committed file — the premise idempotent
        # warm-append rests on. Its H1 is fixed to `# Changelog` via `display-title`
        # (not the lowercase slug). Relocated here from the folder home `changelog/` at
        # schema v2 (design/corpus-migration.md → the v1→v2 relocation). Edge-free
        # (no managed relation): entries cross-reference in prose, not a managed ref (the
        # `commit` target is transient, and a per-entry repeatable edge is deferred —
        # design/changelog.md → Relations).
        #
        # Two sections, the KaC convention:
        #   - `unreleased-changes` (a MULTI-WORD section id, `## Unreleased Changes`): the
        #     staging area, a single-level repeatable of change-groups (a `category` enum +
        #     a `notes` slot). This is the deliberate target for the multi-word-section-id
        #     parser fix (engine work #2) — it does NOT route around the defect.
        #   - `releases` (a TWO-LEVEL repeatable): each item is a cut version (free-text item
        #     title = the version string), carrying an item-level `date` (set on-create) +
        #     an OPTIONAL `link` field (the KaC diff URL, the optional-field target), and
        #     each release NESTS a repeatable `changes` of change-groups (same `category`
        #     enum + `notes` slot — the `Leaf::Repeatable` target).
        #
        # NO leading prose slot on a release item (review finding B1): a release holds only
        # scalar fields then the nested `changes` repeatable — a leading bare-prose slot
        # would swallow the nested `####` groups. The change-group block is the genuinely-
        # shared section, declared ONCE under `fragments:` and pulled in with `include` at
        # both the staging and per-release sites (M33 schema-fragment de-dup — byte-identical
        # instance render; design/document-type-schema.md → On-disk definition format,
        # design/corpus-migration.md → schema-fragment include).
        # Engine-native types only (no pack field-types declared).
        # See design/changelog.md → The `changelog` doctype.
        type: changelog
        placement: { file: CHANGELOG.md }
        display-title: Changelog
        singleton: true
        id-from: title
        description: A Keep-a-Changelog singleton — staged unreleased changes plus the cut releases, each grouped by category, maintained over the life of the project.
        usage: a user-facing change lands and the project keeps a human-readable record of what changed, staged now and cut into versioned releases over time.

        # The genuinely-shared change-group block (`category` enum + `notes` slot),
        # included at both the staging area and each cut release.
        fragments:
          change-group:
            - { id: category, type: enum, of: [added, changed, deprecated, removed, fixed, security] }
            - { id: notes, slot: { hint: "One bullet per change in this category." } }

        sections:
          - id: unreleased-changes
            repeatable:
              id-from: category
              block:
                - include: change-group
          - id: releases
            repeatable:
              id-from: title
              block:
                - { id: title, type: string }
                - { id: date, type: date, set: on-create }
                - { id: link, type: string, optional: true }
                - id: changes
                  repeatable:
                    id-from: category
                    block:
                      - include: change-group
        "#);
    }

    /// The shipped `record-change` workflow is the changelog's standalone driver:
    /// `creates-task: true`, `selectable: false` (off-router, so no router-golden
    /// leak by construction), and a create-gate that admits exactly the `changelog`
    /// (`as: changelog`). Its body is the two ordered step includes
    /// (`author-change` then the shared `finalize`). Golden over the bytes pins the
    /// canonical driver. See design/changelog.md → The driver.
    #[test]
    fn record_change_workflow_body_is_the_canonical_definition() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Workflows, "record-change");
        insta::assert_snapshot!(body, @r###"
        ---
        when: record a user-facing change on the project changelog
        description: Record a change on the changelog — create-or-update the singleton, author a release (or a staged change-group) with its nested category groups, and commit.
        usage: a user-facing change needs recording on the changelog — staged now, or cut into a versioned release.
        creates-task: true
        selectable: false
        suppressed:
          reason: reached by name for the deliberate record-a-change/cut-a-release pass (`jigc start --workflow record-change`); routine change recording already rides `single-task`'s record-changelog step, so a catalog line would duplicate it
          expires: never
        allows-create: [{type: changelog, as: changelog}]
        ---
        {{ include: step:author-change }}
        {{ include: step:finalize }}
        "###);
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
    /// arch-doc step), plus `author-change` (the M22 `record-change` workflow's
    /// create-gated, item-authoring changelog step), plus `author-migration` (the
    /// M23 `migrate-changelog` workflow's foreign-source-driven changelog step), plus
    /// `author-migration-adr` (the M25 `migrate-adr` workflow's foreign-source-driven,
    /// adr-shaped migration step — heading map + foreign-status→enum + supersedes edge
    /// guidance: in-set bracket-list, ordering contract, out-of-set drop-to-prose), plus
    /// `author-migration-spec` (the M25 `migrate-spec` workflow's foreign-source-driven,
    /// spec-shaped migration step — goal/context slots + repeatable criteria items with
    /// an optional `maps-to-test`), plus `author-migration-prd` (the M25 `migrate-prd`
    /// workflow's foreign-source-driven, prd-shaped migration step — vision/context slots
    /// and repeatable requirements items, the post-inc-5 schema), plus
    /// `author-migration-arch-doc` (the M26 `migrate-arch-doc` workflow's
    /// foreign-source-driven, arch-doc-shaped migration step — the `overview` slot +
    /// repeatable `components` items carrying a `description` slot and an optional
    /// `implemented-by` code anchor, plus the bracketed in-store `cites` edge).
    #[test]
    fn embedded_pack_lists_the_mvp_steps() {
        let pack = EmbeddedPack::new();
        let steps = pack.list(PackResourceKind::Steps);
        assert_eq!(
            steps,
            vec![
                ResourceId::from("author-arch-doc"),
                ResourceId::from("author-change"),
                ResourceId::from("author-commit"),
                ResourceId::from("author-migration"),
                ResourceId::from("author-migration-adr"),
                ResourceId::from("author-migration-arch-doc"),
                ResourceId::from("author-migration-prd"),
                ResourceId::from("author-migration-spec"),
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
                ResourceId::from("provision-worktrees"),
                ResourceId::from("record-changelog"),
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
        insta::assert_snapshot!(body, @r#"
        Implement the change directly in the working tree. `git add` your code edits
        before finalize — it commits only what you have staged. When done, set the
        required Conventional-Commits type — your editorial call on what this change
        does — then stage the summary prose:

        {{ cli.set-commit-type }}
        {{ cli.set-commit-summary }}
        <<author: {{ task.commit#summary }}>>

        The `scope` and `body` are optional: add a `scope` to name the area touched, or
        author a `body` to explain the motivation, only when they earn their place —

        jigc doc set-field commit:{{task.id}}#scope --value <area> --task {{task.id}}
        jigc doc set-slot commit:{{task.id}}#body --from-file - --task {{task.id}}

        When the work shares authorship — a co-author, or an agent that wrote it — record
        it in a commit trailer. Add one trailer item, then set its value on the address
        `add-item` prints:

        jigc doc add-item commit:{{task.id}}#trailers --title Co-Authored-By --task {{task.id}}
        jigc doc set-field commit:{{task.id}}#trailers/<id>/value --value "Name <email>" --task {{task.id}}

        If a decision is warranted, create an ADR and author its slots — a line per slot
        usually suffices; an ADR earns its keep by capturing the *why*, not by running
        long:

        {{ cli.create-adr }}

        Author its three required slots on the address `create` prints — `context` (the
        forces at play), `decision` (the call itself), `consequences` (tradeoffs and
        follow-on effects):

        jigc doc set-slot adr:<slug>#context --from-file - --task {{task.id}}
        jigc doc set-slot adr:<slug>#decision --from-file - --task {{task.id}}
        jigc doc set-slot adr:<slug>#consequences --from-file - --task {{task.id}}

        The `options` slot is optional — fill it only when alternatives were genuinely
        weighed; omit it when the call was obvious:

        jigc doc set-slot adr:<slug>#options --from-file - --task {{task.id}}

        Before you finalize, verify the change actually works: build it and run the
        tests, and confirm the behaviour you set out to produce. Finalize commits your
        staged work; it does not check that the work is correct.

        {{fill: extra-guidance}}
        "#);
    }

    #[test]
    fn step_superseded_context_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "superseded-context");
        insta::assert_snapshot!(body, @r###"
        If your decision supersedes an earlier one, set `supersedes` on the ADR; the
        superseded decision then appears below for reference, so your consequences can
        explain what changes (nothing appears if it supersedes none).
        {{ @task.decision.supersedes#decision }}
        "###);
    }

    #[test]
    fn step_finalize_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "finalize");
        insta::assert_snapshot!(body, @"
        Validate and commit the task as one logical commit. Make sure your code edits
        are staged (`git add`) first — finalize commits only the staged set plus the
        docs it manages:

        {{ cli.finalize-task }}
        ");
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
        insta::assert_snapshot!(knobs, @r#"
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
        # validation.md → MVP check inventory (34 checks across 10 categories — pinned by
        # engine::knobs per_check_severity_surface_reconciles_to_the_34_20_14_inventory).
        # (The engine's CHECK_INVENTORY post-pass membership set is a 31-row subset of
        # these 34 — it drops the 3 compose-time marker checks; see engine::result
        # check_inventory_membership_count_is_stable.) The two
        # `validation.<probe>.severity` per-probe keys are retained from M4 as additive
        # per-probe *defaults* (never a rename) so an M4-authored manifest still
        # resolves; they sit alongside the per-check keys.
        # See overrides.md → Scalar knobs / Soft-rejection; storage.md → Config layout.
        default-workflow:
          type: enum
          of: [router, single-task, quick-fix, plan, implement-from-spec]
          default: router

        # --- docs-root — the managed-doc parent dir (storage.md → Config layout) ---
        # Nests every persisted doctype's on-disk `location:` under one parent:
        # `docs/decisions/`, `docs/specs/`, … The doctype id, addressing (`adr:foo`),
        # and stable-id invariants are unchanged — only the on-disk path gains this
        # prefix. Per-project overridable like any knob; set `.` (or `""`, canonicalized
        # to `.`) for the old flat repo-root layout. A tunable knob (no floor).
        docs-root:
          type: string
          default: docs/

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

        # --- invocation-log — the opt-in in-repo invocation log (M36) ---
        # When `true`, one JSONL record `{timestamp, argv, exit_code, duration_ms,
        # finding_codes}` is appended per `jigc` invocation to the gitignored
        # `.jigc/logs/invocations.jsonl` — a jigc-side friction/failure log so a real
        # external-repo RC run yields analyzable in-repo data. Default OFF (safe for an
        # adopter who didn't ask to be logged; the RC-trial protocol turns it on). A
        # tunable knob (no floor). See measurement.md → The in-repo invocation log.
        invocation-log:
          type: bool
          default: "false"

        # --- per-probe severity defaults (M4, retained — additive) ---
        validation.workflow-refs.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.file-state.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking

        # --- workflow-refs.* (11, intrinsic) ---
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
        validation.workflow-refs.checkpoint-marker-not-shadowed.severity:
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
        # schema-ref-resolves (M43): a lone `{{schema:<doctype>}}` names a doctype in the
        # COMPOSED cascade's doctype set — never per-origin-pack (a methodology step
        # legitimately solicits a dev doctype). Fires at compose AND the store sweep.
        validation.workflow-refs.schema-ref-resolves.severity:
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

        # --- schema-conformance.* (5, intrinsic) ---
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
        # The version-currency break (M42): a MANAGED committed instance whose
        # `schema-version` stamp is below its doctype's manifest version. Its own check
        # id — a stale stamp and an invalid enum value are different facts with
        # different consequences, and the collapsed id made the break indistinguishable
        # to any machine consumer (validation.md → Version-currency is itself a surfaced
        # break, the retraction). Intrinsic and floored: an unmigrated corpus means every
        # other check in the sweep is adjudicating docs against a schema they were never
        # written to, so a demotion would let a project silently opt out of knowing its
        # own validation results are meaningless.
        validation.schema-conformance.schema-version-current.severity:
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

        # --- schema-conformance.mention-resolves (1, tunable; advisory, store-scope only — M33) ---
        # The lighter, prose-embedded sibling of `ref-resolves`: an in-prose managed mention
        # `#<type>:<slug>` that names no committed doc is reported (never a per-task finalize
        # gate). Tunable (unfloored) and advisory by default — that placement is exactly what
        # "lighter than field-refs" means (validation.md → In-prose mention integrity).
        validation.schema-conformance.mention-resolves.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory

        # --- schema-conformance.repeatable-populated (1, tunable; advisory — M40) ---
        # Hollow-adoption visibility: a required repeatable section that parses zero
        # items is structurally silent (the adoption trial adopted a zero-item roadmap
        # clean), so the store sweep + the adopt-time triage annotate it — advisory,
        # never a gate (validation.md → Hollow and surplus adoption). The sibling
        # `.exempt` string knob carries space-separated `doctype#section` tokens where
        # zero items IS the steady state (the staging area, a valid just-created state,
        # a clean audit) — a matching token suppresses the finding entirely.
        validation.schema-conformance.repeatable-populated.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory
        validation.schema-conformance.repeatable-populated.exempt:
          type: string
          default: "changelog#unreleased-changes milestone-record#tasks completion-record#findings"

        # --- schema-conformance.surplus-sections-absent (1, tunable; advisory — M40) ---
        # Surplus-adoption visibility: body sections map positionally onto H2s and the
        # parser never visits a TRAILING surplus heading (the adoption trial adopted a
        # surplus `## Legacy planning notes` clean), so the store sweep + the adopt-time
        # triage annotate it — advisory, never a gate (validation.md → Hollow and surplus
        # adoption). A surplus H2 BETWEEN required sections is
        # conformance.section-renamed's territory instead; the two never double-fire.
        validation.schema-conformance.surplus-sections-absent.severity:
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

        # --- doc-code.* (3, tunable; M10 ×2 + the M40 stale-heading guard) ---
        # symbol-exists / criterion-maps-to-test: blocking-by-default (a dangling
        # anchor is a real integrity failure) but cascade-tunable, NOT floored — a
        # project may rationally demote to warning. Unlike the floor-locked
        # pack-probe-integrity.* meta-findings above.
        validation.doc-code.symbol-exists.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        validation.doc-code.criterion-maps-to-test.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
        # title-names-symbol: the stale-heading guard (a component title carrying a
        # compound identifier that is not its anchored symbol). Advisory-by-default —
        # the brand-name false-positive class (`WordPress`, `PostgreSQL`) has no valid
        # remedy under blocking; a strict project re-promotes it with one scalar-set
        # (validation.md → doc-code.title-names-symbol, the M40 demotion block).
        validation.doc-code.title-names-symbol.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory

        # --- owner-artifact.present (1, intrinsic — the M16 #5 completion-half gate) ---
        # A finalize-time presence assertion: the `owner-artifact` owned-location path on a
        # `completion-record` must be durably staged under the owned artifact home. The
        # milestone is not shippable without the artifact, so demoting it would let a
        # completion finalize with no recorded audit artifact — floored blocking
        # (methodology-docs.md → The engine work, item 3).
        validation.owner-artifact.present.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: blocking
          floor: blocking

        # --- changelog-recording.gate-granted-unused (1, tunable; advisory — M42) ---
        # The granted-and-unused changelog gate: the task's workflow `allows-create`s
        # `changelog` and the task authored NO changelog entry. Surfaced at finalize,
        # never a refusal — and it keys on the GATE, never on the CLI adjudicating
        # whether a diff is user-facing (that is judgment; the determinism boundary
        # forbids it). Advisory by default, but keyed so a project that means it — the
        # adoption trial's "if it matters, gate it" — promotes it to `blocking` with one
        # cascade line, no code change (validation.md → The changelog-gate advisory).
        validation.changelog-recording.gate-granted-unused.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory
        "#);

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

    /// The per-doc **schema-version stamp** injection contract (M34): the shipped
    /// loader ([`load_pack_schema`]) injects the engine-declared stamp field into
    /// every **persisted frozen** doctype, and is **inert** for the contexts that
    /// omit the target — the transient `commit` (in the manifest, no `location:`) and
    /// any doctype the manifest does not freeze. Asserts the stamp's shape
    /// (`int` + `set: schema-version`) and its front-matter home (the header section,
    /// first, so `prd`/`changelog` gain a `---` block — `parse.rs` → header-first).
    #[test]
    fn load_pack_schema_stamps_persisted_frozen_doctypes_and_is_inert_elsewhere() {
        use engine::schema::{FieldType, SCHEMA_VERSION_FIELD, SCHEMA_VERSION_SET, SectionBody};

        let pack = EmbeddedPack::new();

        // Every persisted frozen doctype carries the stamp in its (first) header.
        for ty in ["adr", "spec", "prd", "arch-doc", "changelog"] {
            let bytes = pack
                .read(PackResourceKind::Schemas, &ResourceId::from(ty))
                .expect("schema reads");
            let schema = load_pack_schema(&pack, &bytes).expect("schema loads");

            let header = &schema.sections[0];
            assert!(
                header.header,
                "`{ty}` must carry a header section first (front-matter `---` block)",
            );
            let SectionBody::Simple { fields, .. } = &header.body else {
                panic!("`{ty}` header is a simple field section");
            };
            let stamp = fields
                .iter()
                .find(|f| f.id == SCHEMA_VERSION_FIELD)
                .unwrap_or_else(|| panic!("`{ty}` header carries the schema-version stamp"));
            assert_eq!(stamp.ty, FieldType::Int, "the stamp is an int");
            assert_eq!(
                stamp.set.as_deref(),
                Some(SCHEMA_VERSION_SET),
                "the stamp carries the schema-version deriver marker",
            );
            assert!(
                stamp.default.is_none(),
                "the stamp shape is version-independent (no baked default)",
            );
        }

        // Inert for the transient `commit` (in the manifest, but no `location:`): no
        // stamp anywhere, byte-identical shape to a bare load.
        let commit_bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
            .expect("commit reads");
        let commit = load_pack_schema(&pack, &commit_bytes).expect("commit loads");
        assert!(
            !has_schema_version_field(&commit),
            "the transient `commit` (no location) must not gain a stamp",
        );

        // Inert for a persisted doctype the manifest does NOT freeze: a fixture `note`
        // doctype with a `location:` loaded against the shipped pack (whose manifest
        // lists only the six) gains no stamp — the omitting-context guard.
        let note = b"type: note\nlocation: notes/\nid-from: title\nsections:\n  - id: body\n    slot: { hint: \"x\" }\n";
        let note_schema = load_pack_schema(&pack, note).expect("note loads");
        assert!(
            !has_schema_version_field(&note_schema),
            "a doctype absent from the freeze manifest must not gain a stamp",
        );
    }

    /// Whether any section of `schema` declares a `schema-version` field — the
    /// stamp-presence probe for the injection-contract test.
    fn has_schema_version_field(schema: &Schema) -> bool {
        use engine::schema::{SCHEMA_VERSION_FIELD, SectionBody};
        schema.sections.iter().any(|s| match &s.body {
            SectionBody::Simple { fields, .. } => {
                fields.iter().any(|f| f.id == SCHEMA_VERSION_FIELD)
            }
            SectionBody::Repeatable { .. } => false,
        })
    }

    /// The **build-time freeze gate** (the intrinsic-floor-assertion sibling —
    /// `design/corpus-migration.md` → The freeze, declared *and* enforced).
    ///
    /// Loads every shipped dev-pack doctype through the **production** loader
    /// ([`load_pack_schema`]), reads the shipped `config/schema-manifest.yaml`, and
    /// drives them through the real engine check ([`engine::manifest::check`]). It
    /// passes only when the declared set and the shipped set are **exactly equal**
    /// with every hash matching — so a schema-shape change that bumps no version (or
    /// a doctype added/removed without touching the manifest) fails **here, loudly**,
    /// at build time, instead of slipping past review. The artifact + this gate land
    /// in one commit: the manifest must carry correct hashes the instant it ships or
    /// this test fails.
    ///
    /// The v1 frozen set is the **six** persisted dev-pack doctypes (`commit`,
    /// `adr`, `spec`, `prd`, `arch-doc`, `changelog`) — exactly the set proven
    /// byte-stable. The methodology doctypes are governed by the methodology pack's
    /// **own** manifest (M40 A1 — [`methodology_schema_manifest_matches_the_frozen_doctype_set`]);
    /// the manifest mechanism is general (an unlisted doctype would be an
    /// [`engine::manifest::ManifestError::ExtraEntry`]).
    #[test]
    fn shipped_schema_manifest_matches_the_frozen_doctype_set() {
        use std::collections::BTreeMap;

        let pack = EmbeddedPack::new();

        // Every shipped doctype, loaded through the production field-type-resolving
        // loader and keyed by its own type id — the exact set the gate freezes.
        let schemas: BTreeMap<String, Schema> = pack
            .list(PackResourceKind::Schemas)
            .iter()
            .map(|id| {
                let bytes = pack
                    .read(PackResourceKind::Schemas, id)
                    .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
                let schema = load_pack_schema(&pack, &bytes)
                    .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
                (schema.ty.clone(), schema)
            })
            .collect();

        // The shipped manifest artifact, parsed into the engine model.
        let manifest_bytes = pack
            .read(
                PackResourceKind::Config,
                &ResourceId::from("schema-manifest"),
            )
            .expect("the pack must ship config/schema-manifest.yaml");
        let manifest: engine::manifest::Manifest = serde_yaml_ng::from_slice(&manifest_bytes)
            .expect("config/schema-manifest.yaml parses as a freeze manifest");

        // The frozen set is exactly the six persisted dev-pack doctypes — six present,
        // no extra, no missing. Each is at schema-version 1 except `adr`, bumped to 2 by
        // the M36 options-slot v1→v2 shape change, and `changelog`, bumped to 2 by the
        // M38 v1→v2 root-`CHANGELOG.md` relocation (`design/corpus-migration.md` → the adr
        // v1→v2 flow, the changelog v1→v2 relocation).
        let mut declared: Vec<&str> = manifest.doctypes.iter().map(|e| e.ty.as_str()).collect();
        declared.sort_unstable();
        assert_eq!(
            declared,
            ["adr", "arch-doc", "changelog", "commit", "prd", "spec"],
            "the freeze manifest must enumerate exactly the six frozen v1 doctypes",
        );
        for entry in &manifest.doctypes {
            let expected = if entry.ty == "adr" || entry.ty == "changelog" {
                2
            } else {
                1
            };
            assert_eq!(
                entry.schema_version, expected,
                "doctype `{}` is frozen at schema-version {expected}",
                entry.ty,
            );
        }

        // The real freeze check: recompute each shipped hash and require exact
        // equality with the manifest — the loud build-time failure on any drift.
        engine::manifest::check(&manifest, &schemas)
            .expect("shipped doctype set must match the frozen schema-manifest");
    }

    /// The **methodology sibling** of the build-time freeze gate (M40 A1 —
    /// `design/corpus-migration.md` → M40 revises the dichotomy): the methodology
    /// pack now ships its **own** `config/schema-manifest.yaml` listing all **ten**
    /// shipped schemas — the nine persisted work-doc/design-altitude doctypes plus
    /// the transient `commit` shadow (the freeze assert is strict set-equality; the
    /// dev precedent lists its transient `commit`) — every one frozen at
    /// schema-version 1 (the v1 baseline) except `deferral-ledger`, bumped to 2 by the
    /// M41 F4 `kind` rename (which populates `schema-snapshots/deferral-ledger.v1.yaml`),
    /// with every recomputed hash matching. A methodology schema-shape change that bumps
    /// no version fails **here, loudly**, at build time.
    #[test]
    fn methodology_schema_manifest_matches_the_frozen_doctype_set() {
        use std::collections::BTreeMap;

        let pack = EmbeddedPack::methodology();

        // Every shipped methodology doctype, loaded through the production
        // field-type-resolving loader against its own pack (so the persisted nine
        // carry the injected stamp, exactly what the pack-load gate recomputes).
        let schemas: BTreeMap<String, Schema> = pack
            .list(PackResourceKind::Schemas)
            .iter()
            .map(|id| {
                let bytes = pack
                    .read(PackResourceKind::Schemas, id)
                    .unwrap_or_else(|e| panic!("schema `{}` reads back: {e}", id.as_str()));
                let schema = load_pack_schema(&pack, &bytes)
                    .unwrap_or_else(|e| panic!("schema `{}` loads: {e:?}", id.as_str()));
                (schema.ty.clone(), schema)
            })
            .collect();

        let manifest_bytes = pack
            .read(
                PackResourceKind::Config,
                &ResourceId::from("schema-manifest"),
            )
            .expect("the methodology pack must ship config/schema-manifest.yaml");
        let manifest: engine::manifest::Manifest = serde_yaml_ng::from_slice(&manifest_bytes)
            .expect("config/schema-manifest.yaml parses as a freeze manifest");

        // The frozen set is exactly the ten shipped methodology schemas, each at
        // schema-version 1 (the crystallizing v1 baseline) except two: `deferral-ledger`,
        // bumped to 2 by the M41 F4 v1→v2 `kind` enum-member rename (D/I → Decision/Idea
        // — the first methodology v1→v2 migration; `design/corpus-migration.md` → the
        // structural-auto / value-semantic-authored distinction), and `milestone-record`,
        // bumped to 2 by the M42 Inc-7 lifecycle widening (`status` gains `discarded` at
        // both loci — an `EnumWidened` pair; `design/team-ready-state.md` → The lifecycle).
        let mut declared: Vec<&str> = manifest.doctypes.iter().map(|e| e.ty.as_str()).collect();
        declared.sort_unstable();
        assert_eq!(
            declared,
            [
                "commit",
                "completion-record",
                "decisions-log",
                "deferral-ledger",
                "dogfood-record",
                "idea",
                "milestone-record",
                "research",
                "roadmap",
                "vision",
            ],
            "the methodology freeze manifest must enumerate exactly the ten shipped schemas",
        );
        for entry in &manifest.doctypes {
            let expected = match entry.ty.as_str() {
                "deferral-ledger" | "milestone-record" => 2,
                _ => 1,
            };
            assert_eq!(
                entry.schema_version, expected,
                "methodology doctype `{}` is frozen at schema-version {expected}",
                entry.ty,
            );
        }

        engine::manifest::check(&manifest, &schemas)
            .expect("the shipped methodology doctype set must match its frozen schema-manifest");
    }

    /// The methodology sibling of the stamp-injection contract: with the methodology
    /// manifest shipped (M40 A1), [`load_pack_schema`] injects the schema-version
    /// stamp into every **persisted** methodology doctype — appended to an existing
    /// `meta` header, or carried by a fresh first header for the header-less
    /// singletons (`roadmap`/`decisions-log`/`deferral-ledger`, which thereby gain a
    /// `---` block on mint) — and stays **inert** for the transient `commit` shadow
    /// (in the manifest, neither `location:` nor `placement:` — stamp-excluded).
    #[test]
    fn load_pack_schema_stamps_persisted_methodology_doctypes_and_shadow_commit_is_inert() {
        use engine::schema::{FieldType, SCHEMA_VERSION_FIELD, SCHEMA_VERSION_SET, SectionBody};

        let pack = EmbeddedPack::methodology();

        for ty in [
            "completion-record",
            "decisions-log",
            "deferral-ledger",
            "dogfood-record",
            "idea",
            "milestone-record",
            "research",
            "roadmap",
            "vision",
        ] {
            let bytes = pack
                .read(PackResourceKind::Schemas, &ResourceId::from(ty))
                .expect("schema reads");
            let schema = load_pack_schema(&pack, &bytes).expect("schema loads");

            let header = &schema.sections[0];
            assert!(
                header.header,
                "`{ty}` must carry a header section first after stamp injection",
            );
            let SectionBody::Simple { fields, .. } = &header.body else {
                panic!("`{ty}` header is a simple field section");
            };
            let stamp = fields
                .iter()
                .find(|f| f.id == SCHEMA_VERSION_FIELD)
                .unwrap_or_else(|| panic!("`{ty}` header carries the schema-version stamp"));
            assert_eq!(stamp.ty, FieldType::Int, "the stamp is an int");
            assert_eq!(
                stamp.set.as_deref(),
                Some(SCHEMA_VERSION_SET),
                "the stamp carries the schema-version deriver marker",
            );
        }

        // Inert for the transient `commit` shadow (in the manifest, no home).
        let commit_bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
            .expect("commit reads");
        let commit = load_pack_schema(&pack, &commit_bytes).expect("commit loads");
        assert!(
            !has_schema_version_field(&commit),
            "the transient methodology `commit` shadow must not gain a stamp",
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

    mod marker {
        use super::super::*;
        use std::path::{Path, PathBuf};

        /// A throwaway directory that removes itself on drop.
        struct TempDir(PathBuf);

        impl TempDir {
            fn new() -> Self {
                let mut path = std::env::temp_dir();
                path.push(format!(
                    "jigc-marker-unit-{}-{:?}",
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

        /// The marker key set `true` reads back `true` — the setup-written
        /// `compose-embedded-methodology: true` the factory composes on. The key is
        /// a NET-NEW parse of the same `packs.yaml` `read_pack_list` reads (no new
        /// discovery walk).
        #[test]
        fn marker_true_reads_true() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"compose-embedded-methodology: true\n",
            )
            .expect("seed marker");
            assert!(
                read_compose_marker(dir.path()).expect("a valid marker reads back"),
                "`compose-embedded-methodology: true` must read back true",
            );
        }

        /// An absent file is `false` (the single-pack floor) — never an error. The
        /// common cold-start / dev-only case.
        #[test]
        fn absent_file_is_false() {
            let dir = TempDir::new();
            assert!(
                !read_compose_marker(dir.path()).expect("an absent file is not an error"),
                "absent packs.yaml => no marker => false",
            );
        }

        /// A present file with no marker key is `false` (the key defaults off) —
        /// e.g. a hand-written `packs:` list with no marker. The M14 listed-pack
        /// path keeps composing without tripping the embedded-pair path.
        #[test]
        fn absent_key_is_false() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"packs:\n  - /opt/jigc-packs/x\n",
            )
            .expect("seed keyless marker");
            assert!(
                !read_compose_marker(dir.path()).expect("a missing marker key is not an error"),
                "absent marker key => false",
            );
        }

        /// An explicit `false` is `false` — a present-but-off marker is inert,
        /// byte-identical to absent.
        #[test]
        fn marker_false_reads_false() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"compose-embedded-methodology: false\n",
            )
            .expect("seed false marker");
            assert!(
                !read_compose_marker(dir.path()).expect("`false` is not an error"),
                "`compose-embedded-methodology: false` must read back false",
            );
        }

        /// Garbage YAML is a **located** `Err` naming the file — never a panic. The
        /// marker is a selection input, so it must fail cleanly on malformed bytes
        /// (parity with `read_pack_list`).
        #[test]
        fn garbage_is_a_located_err() {
            let dir = TempDir::new();
            std::fs::write(
                dir.path().join("packs.yaml"),
                b"compose-embedded-methodology: : : ][\n",
            )
            .expect("seed garbage");
            let err =
                read_compose_marker(dir.path()).expect_err("garbage packs.yaml is a clean Err");
            assert!(
                format!("{err:#}").contains("packs.yaml"),
                "the error must locate the offending file; got: {err:#}",
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

        /// With `JIGC_PACK_DIR` unset, the base selector yields an
        /// `EmbeddedPack`-backed source: its `list`/`read`/`pack_version` equal
        /// `EmbeddedPack`'s, so a no-env build is byte-identical to one without the
        /// seam.
        #[test]
        fn unset_env_yields_an_embedded_backed_source() {
            let pack = make_base_pack(None);
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
            let pack = make_base_pack(Some(OsString::new()));
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

            let pack = make_base_pack(Some(OsString::from(dir.path())));

            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                vec![ResourceId::from("only-on-disk")],
                "the set-env base selector must list the on-disk directory pack, not the embedded one",
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

        /// The composite-assembly floor at the factory core: with **no** listed
        /// packs, `make_pack_from` is a `Composite([base])` whose `list`/`read`/
        /// `pack_version` equal the base (`EmbeddedPack`) — byte-identical to the
        /// single-pack path. This is the headline regression proven at the seam the
        /// production `make_pack()` flows through (the real-binary floor rides the
        /// existing `start_compose` goldens).
        #[test]
        fn no_listed_packs_is_the_base_only_floor() {
            let pack = make_pack_from(None, Vec::new());
            let embedded = EmbeddedPack::new();

            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                embedded.list(PackResourceKind::Workflows),
                "Composite([base]).list must equal the base pack's list",
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
                "Composite([base]).read must equal the base pack's read",
            );
            assert_eq!(
                pack.pack_version(),
                embedded.pack_version(),
                "Composite([base]).pack_version must equal the base pack's",
            );
        }

        /// **Marker set** → the core composes the embedded `[dev ▸ methodology]`
        /// pair (dev highest): the methodology union (`planning`/`roadmap`) AND the
        /// dev base (`single-task`) both resolve through the composite, and
        /// `pack_version` is the binary version (both embedded packs tie to the
        /// release). Proves the two-embedded-pack path the setup marker wires.
        #[test]
        fn marker_composes_the_embedded_dev_methodology_pair() {
            let pack = make_pack_from_marker(None, Vec::new(), true)
                .expect("the marker composite assembles");

            // The methodology union resolves through the composite ...
            for id in ["planning", "roadmap"] {
                assert!(
                    pack.read(
                        if id == "roadmap" {
                            PackResourceKind::Schemas
                        } else {
                            PackResourceKind::Workflows
                        },
                        &ResourceId::from(id),
                    )
                    .is_ok(),
                    "the methodology `{id}` must resolve through the marker composite",
                );
            }
            // ... and the dev base's `single-task` still resolves (the union).
            assert!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .is_ok(),
                "the dev base's `single-task` must resolve through the marker composite",
            );
            // Both embedded packs tie to the binary release.
            assert_eq!(
                pack.pack_version(),
                EmbeddedPack::new().pack_version(),
                "the marker composite's pack_version is the binary version",
            );
        }

        /// **Dev-highest collision proof.** On the two real cross-pack collisions
        /// (`Config/knobs`, `Schemas/commit`) the marker composite reads **DEV's**
        /// bytes — the inverse of the listed>base convention. The composed `commit`
        /// carries `implements` (dev's; methodology's drops it) and the composed
        /// `default-workflow` knob declares `router` (dev's; methodology's is
        /// `dev-task`). Byte-equality against the dev base's own `read` is the proof.
        #[test]
        fn marker_resolves_collisions_dev_highest() {
            let pack = make_pack_from_marker(None, Vec::new(), true)
                .expect("the marker composite assembles");
            let dev = EmbeddedPack::new();

            let commit = pack
                .read(PackResourceKind::Schemas, &ResourceId::from("commit"))
                .expect("the composed commit doctype reads back");
            assert_eq!(
                commit,
                dev.read(PackResourceKind::Schemas, &ResourceId::from("commit"))
                    .expect("dev ships commit"),
                "dev-highest: the composed `commit` must be DEV's bytes",
            );
            assert!(
                String::from_utf8(commit).unwrap().contains("implements"),
                "dev's commit carries `implements` — the methodology drop must be shadowed",
            );

            let knobs = pack
                .read(PackResourceKind::Config, &ResourceId::from("knobs"))
                .expect("the composed knobs declaration reads back");
            assert_eq!(
                knobs,
                dev.read(PackResourceKind::Config, &ResourceId::from("knobs"))
                    .expect("dev ships knobs"),
                "dev-highest: the composed `knobs` must be DEV's bytes",
            );
            assert!(
                String::from_utf8(knobs)
                    .unwrap()
                    .contains("default: router"),
                "dev's default-workflow knob declares `router` — methodology's `dev-task` shadowed",
            );
        }

        /// **Floor.** Marker absent/false with no listed packs → the core is the
        /// bare `Composite([base])`, byte-identical to a direct `EmbeddedPack::dev()`
        /// on `list`/`read`/`pack_version`. The two-embedded-pack path is inert
        /// without the marker (the single-pack floor that keeps every dev-only test
        /// green).
        #[test]
        fn marker_absent_is_the_base_only_floor() {
            let pack = make_pack_from_marker(None, Vec::new(), false)
                .expect("the no-marker composite assembles");
            let embedded = EmbeddedPack::new();

            assert_eq!(
                pack.list(PackResourceKind::Workflows),
                embedded.list(PackResourceKind::Workflows),
                "no-marker list must equal the bare dev pack's list",
            );
            // A methodology-only workflow must NOT resolve without the marker.
            assert!(
                pack.read(PackResourceKind::Workflows, &ResourceId::from("planning"))
                    .is_err(),
                "without the marker the methodology surface must be absent",
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
                "no-marker read must equal the bare dev pack's read",
            );
            assert_eq!(
                pack.pack_version(),
                embedded.pack_version(),
                "no-marker pack_version must equal the bare dev pack's",
            );
        }

        /// **`JIGC_PACK_DIR` supersedes the marker.** When the base is selected by
        /// `JIGC_PACK_DIR` (the explicit/dogfood channel), the
        /// `compose-embedded-methodology` marker must **not** fire: the
        /// `JIGC_PACK_DIR` base wins exactly as it did pre-M21 (the marker is
        /// settable repo-wide, but the dogfood's `JIGC_PACK_DIR` must keep selecting
        /// its base alone — `design/multi-pack.md` → Embedded second pack + setup
        /// auto-wiring; `worked-examples.md` flow 15). Proven by: a sentinel resource
        /// present only on the `JIGC_PACK_DIR` tree resolves through the composite,
        /// AND the dev base's `single-task` does **not** — so the embedded
        /// `[dev ▸ methodology]` composition is inert.
        #[test]
        fn pack_dir_supersedes_the_marker() {
            let base = TempDir::new();
            let wf = base.path().join("workflows");
            std::fs::create_dir_all(&wf).expect("mk workflows/");
            std::fs::write(wf.join("pack-dir-only.yaml"), b"when: from pack dir\n")
                .expect("seed pack-dir wf");

            // Marker is set (`true`), but the base comes from `JIGC_PACK_DIR`.
            let pack =
                make_pack_from_marker(Some(base.path().as_os_str().to_owned()), Vec::new(), true)
                    .expect("the JIGC_PACK_DIR composite assembles");

            // The `JIGC_PACK_DIR` base's own workflow resolves through the composite ...
            assert_eq!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("pack-dir-only"),
                )
                .expect("the pack-dir-only workflow reads through the composite"),
                b"when: from pack dir\n",
                "`JIGC_PACK_DIR` must select the base when the marker is also set",
            );
            // ... and the embedded dev/methodology composition is INERT: neither the
            // dev base's `single-task` nor the methodology's `planning` resolves.
            assert!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .is_err(),
                "the marker must not fire under `JIGC_PACK_DIR`: dev's `single-task` must be absent",
            );
            assert!(
                pack.read(PackResourceKind::Workflows, &ResourceId::from("planning"))
                    .is_err(),
                "the marker must not fire under `JIGC_PACK_DIR`: methodology's `planning` must be absent",
            );
        }

        /// **The marker never silently drops a listed pack** (M42 Inc 6). The marker +
        /// a non-empty `packs:` list is the combination `design/multi-pack.md` puts out
        /// of scope; it used to resolve by discarding the whole listed set — and since
        /// `jigc setup` writes the marker into **every** project's `packs.yaml`, that
        /// made an operator's listed pack inert in the shipped topology (its drifted
        /// frozen schema then had nothing to check it). It is now an **error** naming
        /// both the marker and the dropped packs, with the two exits as the route.
        #[test]
        fn marker_plus_a_listed_pack_is_refused_not_silently_dropped() {
            let listed = TempDir::new();
            let Err(err) = make_pack_from_marker(None, vec![listed.path().to_path_buf()], true)
            else {
                panic!(
                    "marker + a listed pack must not assemble — the listed pack would be dropped"
                );
            };
            let msg = format!("{err:#}");

            assert!(
                msg.contains("compose-embedded-methodology") && msg.contains("packs.yaml"),
                "the refusal must name the marker and the file that carries it; got:\n{msg}",
            );
            assert!(
                msg.contains(&listed.path().display().to_string()),
                "the refusal must name the listed pack it refuses to drop; got:\n{msg}",
            );
            assert!(
                msg.contains("route:"),
                "the refusal must carry a route (drop the marker, or drop the list); got:\n{msg}",
            );
        }

        /// The refusal is scoped to the arm that would *drop* something: under an
        /// explicit `JIGC_PACK_DIR` the marker is already inert
        /// ([`pack_dir_supersedes_the_marker`]), so listed packs compose over that base
        /// on the plain M14 path — nothing is discarded, nothing is refused.
        #[test]
        fn marker_plus_listed_under_pack_dir_still_composes_the_m14_path() {
            let base = TempDir::new();
            let listed = TempDir::new();
            let wf = listed.path().join("workflows");
            std::fs::create_dir_all(&wf).expect("mk workflows/");
            std::fs::write(wf.join("listed-only.yaml"), b"when: from listed\n")
                .expect("seed listed wf");

            let pack = make_pack_from_marker(
                Some(base.path().as_os_str().to_owned()),
                vec![listed.path().to_path_buf()],
                true,
            )
            .expect("`JIGC_PACK_DIR` makes the marker inert, so the listed pack composes");

            assert_eq!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("listed-only"),
                )
                .expect("the listed pack's workflow reads through the composite"),
                b"when: from listed\n",
                "the listed pack must compose over the `JIGC_PACK_DIR` base",
            );
        }

        /// Two-pack assembly at the core: a listed dir is composed **over** the base.
        /// A resource present **only** on the listed pack resolves through the
        /// composite (the union read), and the listed pack — being highest-precedence
        /// — supplies `pack_version`. This proves `make_pack_from` orders listed-first,
        /// base-last; the real-binary CWD-discovery equivalence is proven in
        /// `start_compose.rs`.
        #[test]
        fn a_listed_pack_composes_over_the_base() {
            let listed = TempDir::new();
            let wf = listed.path().join("workflows");
            std::fs::create_dir_all(&wf).expect("mk workflows/");
            std::fs::write(wf.join("listed-only.yaml"), b"when: from listed\n")
                .expect("seed listed wf");
            let cfg = listed.path().join("config");
            std::fs::create_dir_all(&cfg).expect("mk config/");
            std::fs::write(
                cfg.join("defaults.yaml"),
                b"pack-id: listed\nversion: 7.7.7\n",
            )
            .expect("seed listed defaults");

            // Base = EmbeddedPack (JIGC_PACK_DIR unset); the listed dir sits above it.
            let pack = make_pack_from(None, vec![listed.path().to_owned()]);

            // The listed pack's own workflow resolves through the composite ...
            assert_eq!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("listed-only")
                )
                .expect("the listed-only workflow reads through the composite"),
                b"when: from listed\n",
            );
            // ... and the base pack's `single-task` still resolves (the union, base last).
            assert!(
                pack.read(
                    PackResourceKind::Workflows,
                    &ResourceId::from("single-task"),
                )
                .is_ok(),
                "the base pack's single-task must still resolve through the composite union",
            );
            // The listed (highest-precedence) pack supplies the version.
            assert_eq!(
                pack.pack_version(),
                "7.7.7",
                "the highest-precedence (listed) pack must supply pack_version",
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

        /// Pack A (higher) and pack B (lower) each ship a **colliding**
        /// `step:implement` with **distinct bytes**, plus a non-colliding step
        /// each. The composite is `[A, B]` — A wins the `implement` top-level id.
        fn two_pack_step_composite() -> CompositePack {
            let a = MemPack::new("a-ver")
                .with(PackResourceKind::Steps, "implement", b"A-implement")
                .with(PackResourceKind::Steps, "locate", b"A-locate");
            let b = MemPack::new("b-ver")
                .with(PackResourceKind::Steps, "implement", b"B-implement")
                .with(PackResourceKind::Steps, "finalize", b"B-finalize");
            CompositePack::new(vec![Box::new(a), Box::new(b)])
        }

        /// A colliding id's **origin** is the precedence **winner** — `A`. The
        /// pack-of-origin lookup returns the constituent that owns the top-level
        /// id (the body-reference resolution anchor), so resolving its bytes
        /// through that origin yields **A's** `implement`, never B's. This is the
        /// M3-class hazard the seam exists to avert: a definition's body-refs must
        /// resolve against the pack that owns its top-level id.
        #[test]
        fn colliding_id_origin_is_the_precedence_winner() {
            let composite = two_pack_step_composite();
            let origin =
                composite.origin_pack(PackResourceKind::Steps, &ResourceId::from("implement"));
            assert_eq!(
                origin
                    .read(PackResourceKind::Steps, &ResourceId::from("implement"))
                    .expect("the winner owns the colliding id"),
                b"A-implement",
            );
        }

        /// A **loser-only** id's origin is the lower-precedence pack `B` (not the
        /// winner) — the constituent that actually defines it. Resolving through
        /// that origin yields **B's** bytes, proving the lookup is first-success,
        /// not always-the-winner.
        #[test]
        fn loser_only_id_origin_is_the_defining_pack() {
            let composite = two_pack_step_composite();
            let origin =
                composite.origin_pack(PackResourceKind::Steps, &ResourceId::from("finalize"));
            assert_eq!(
                origin
                    .read(PackResourceKind::Steps, &ResourceId::from("finalize"))
                    .expect("B defines the loser-only id"),
                b"B-finalize",
            );
        }

        /// The single-pack **floor**: `Composite([A]).origin_pack(...)` is `A`
        /// itself — a single pack is its own origin. Proven by resolving the id
        /// through the returned origin: it reads `A`'s bytes, byte-identical to a
        /// direct read.
        #[test]
        fn single_pack_composite_origin_is_the_pack() {
            let a =
                MemPack::new("a-ver").with(PackResourceKind::Steps, "implement", b"A-implement");
            let composite = CompositePack::new(vec![Box::new(a)]);
            let origin =
                composite.origin_pack(PackResourceKind::Steps, &ResourceId::from("implement"));
            assert_eq!(
                origin
                    .read(PackResourceKind::Steps, &ResourceId::from("implement"))
                    .expect("the lone pack defines the id"),
                b"A-implement",
            );
        }

        /// The seam must be callable on a `&dyn PackSource` — every consumer
        /// (T2/T3/T4's `compose_core`) holds the composite as `&dyn`, not the
        /// concrete type. Resolving the colliding id through the `dyn` receiver
        /// returns the winner's bytes, proving the method is in the vtable (the
        /// object-safe `AsPackSource` upcast, not a `Self: Sized` default that
        /// would be un-dispatchable on a trait object).
        #[test]
        fn origin_pack_is_callable_on_a_trait_object() {
            let composite = two_pack_step_composite();
            let as_dyn: &dyn PackSource = &composite;
            let origin =
                as_dyn.origin_pack(PackResourceKind::Steps, &ResourceId::from("implement"));
            assert_eq!(
                origin
                    .read(PackResourceKind::Steps, &ResourceId::from("implement"))
                    .expect("the winner owns the colliding id"),
                b"A-implement",
            );
        }

        /// `origin_packs` enumerates **every** owner of a shadowed id in
        /// precedence order — the winner A first, then the shadowed B — where
        /// the singular `origin_pack` stops at A. An id no constituent owns is
        /// the empty `Vec` (never a `self` fallback — there is nothing to walk).
        #[test]
        fn origin_packs_enumerates_every_owner_in_precedence_order() {
            let composite = two_pack_step_composite();
            let owners =
                composite.origin_packs(PackResourceKind::Steps, &ResourceId::from("implement"));
            let bytes: Vec<Vec<u8>> = owners
                .iter()
                .map(|p| {
                    p.read(PackResourceKind::Steps, &ResourceId::from("implement"))
                        .expect("an enumerated owner ships the id")
                })
                .collect();
            assert_eq!(
                bytes,
                vec![b"A-implement".to_vec(), b"B-implement".to_vec()],
                "both owners, winner first — the shadowed constituent is not dropped",
            );
            assert!(
                composite
                    .origin_packs(PackResourceKind::Steps, &ResourceId::from("absent"))
                    .is_empty(),
                "an unowned id has no owners to walk",
            );
        }

        /// An id **no** constituent owns falls back to `self` (the composite) —
        /// so a dangling body-reference still flows to the existing not-found
        /// path (a clean `NotFound`), never a panic.
        #[test]
        fn unowned_id_origin_falls_back_to_self() {
            let composite = two_pack_step_composite();
            let origin =
                composite.origin_pack(PackResourceKind::Steps, &ResourceId::from("absent"));
            let err = origin
                .read(PackResourceKind::Steps, &ResourceId::from("absent"))
                .expect_err("an unowned id reads back NotFound through the fallback origin");
            assert_eq!(
                err,
                PackError::NotFound {
                    kind: PackResourceKind::Steps,
                    id: ResourceId::from("absent"),
                },
            );
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

        /// The **per-origin manifest-resolution rule** (M40 A1 T3,
        /// `design/corpus-migration.md` → the split-brain close): a doctype's
        /// governing `schema-manifest.yaml` entry is the one shipped by
        /// `origin_pack(Schemas, ty)` — the pack whose schema definition wins —
        /// never the single Config-resource precedence winner's manifest; and the
        /// pack-load freeze assertion enforces **every** manifest-shipping
        /// constituent, not just that winner.
        mod per_origin_manifests {
            use super::*;

            /// Pack A's transient `commit` shape — structurally distinct from pack
            /// B's, so the two manifests carry **different hashes** for the same
            /// doctype (the genuine-overlap fixture the rule is proven over).
            const COMMIT_A: &str = "type: commit
sections:
  - id: summary
    slot: { hint: \"Pack A's subject line.\" }
";

            /// Pack B's divergent `commit` shadow — an extra optional slot, so its
            /// schema-hash differs from A's.
            const COMMIT_B: &str = "type: commit
sections:
  - id: summary
    slot: { hint: \"Pack B's subject line.\" }
  - id: body
    slot: { hint: \"Pack B's body.\", optional: true }
";

            /// Pack B's non-colliding `note` doctype — the loser-pack-only doctype
            /// whose manifest entry the one-winner rule would drop.
            const NOTE_B: &str = "type: note
sections:
  - id: text
    slot: { hint: \"The note.\" }
";

            /// The [`schema_hash`](engine::manifest::schema_hash) of fixture
            /// `bytes`, loaded through the production [`load_pack_schema`] — the
            /// same load the freeze assertion recomputes over.
            fn hash_of(bytes: &[u8]) -> String {
                let schema = load_pack_schema(&MemPack::new("hash-tmp"), bytes)
                    .expect("the fixture schema loads");
                engine::manifest::schema_hash(&schema)
            }

            /// A `schema-manifest.yaml` body over `(type, schema-version,
            /// schema-hash)` entries, serialized through the engine model so the
            /// on-disk key spelling can never drift from the deserializer. It declares
            /// the slug rule the engine ships, because **every** manifest must (M42
            /// audit — an absent block is `SlugRuleUndeclared`, no longer a silent
            /// opt-out); these fixtures exercise the doctype-*shape* arms, so they
            /// declare the identity rule truthfully and let those arms be what fires.
            fn manifest_yaml(entries: &[(&str, u32, String)]) -> Vec<u8> {
                let manifest = engine::manifest::Manifest {
                    slug_rule: Some(engine::manifest::SlugRule {
                        version: engine::slug::SLUG_RULE_VERSION,
                        hash: engine::slug::rule_fingerprint().to_string(),
                    }),
                    doctypes: entries
                        .iter()
                        .map(|(ty, version, hash)| engine::manifest::ManifestEntry {
                            ty: (*ty).to_owned(),
                            schema_version: *version,
                            schema_hash: hash.clone(),
                        })
                        .collect(),
                };
                serde_yaml_ng::to_string(&manifest)
                    .expect("the manifest model serializes")
                    .into_bytes()
            }

            /// The two-manifest composite `[A, B]`: pack A (highest-precedence)
            /// ships its `commit` + a manifest freezing it at version 3; pack B
            /// ships a **divergent** `commit` shadow + a non-colliding `note`,
            /// with its **own** manifest freezing both (commit at 7, note at 5).
            /// `note_hash` is B's *declared* hash for `note`, so the enforcement
            /// test can corrupt exactly the loser pack's manifest.
            fn two_manifest_composite(note_hash: String) -> CompositePack {
                let a = MemPack::new("a-ver")
                    .with(PackResourceKind::Schemas, "commit", COMMIT_A.as_bytes())
                    .with(
                        PackResourceKind::Config,
                        "schema-manifest",
                        &manifest_yaml(&[("commit", 3, hash_of(COMMIT_A.as_bytes()))]),
                    );
                let b = MemPack::new("b-ver")
                    .with(PackResourceKind::Schemas, "commit", COMMIT_B.as_bytes())
                    .with(PackResourceKind::Schemas, "note", NOTE_B.as_bytes())
                    .with(
                        PackResourceKind::Config,
                        "schema-manifest",
                        &manifest_yaml(&[
                            ("commit", 7, hash_of(COMMIT_B.as_bytes())),
                            ("note", 5, note_hash),
                        ]),
                    );
                CompositePack::new(vec![Box::new(a), Box::new(b)])
            }

            /// Each doctype's version resolves to **its own origin pack's**
            /// manifest entry: the colliding `commit` is governed by A's entry
            /// (3 — A's schema wins, so A's manifest governs; never B's 7), and
            /// the loser-only `note` is governed by B's entry (5 — invisible
            /// under the one-winner Config-resource rule, which read only A's
            /// manifest).
            #[test]
            fn frozen_doctype_versions_is_the_per_doctype_governed_union() {
                let composite = two_manifest_composite(hash_of(NOTE_B.as_bytes()));
                let versions = frozen_doctype_versions(&composite);
                let expected: std::collections::BTreeMap<String, u32> =
                    [("commit".to_owned(), 3), ("note".to_owned(), 5)].into();
                assert_eq!(
                    versions, expected,
                    "each doctype must resolve to its origin pack's manifest entry",
                );
            }

            /// `assert_schema_freeze` enforces **both** manifests: the clean
            /// composite passes, and corrupting only the **loser** pack B's
            /// declared `note` hash blocks — even though pack A (the Config
            /// precedence winner, the only manifest the one-winner rule checked)
            /// is still clean, so the old rule would have silently passed.
            #[test]
            fn assert_schema_freeze_enforces_every_manifest_shipping_constituent() {
                let clean = two_manifest_composite(hash_of(NOTE_B.as_bytes()));
                assert_schema_freeze(&clean)
                    .expect("a composite whose every manifest holds composes clean");

                let drifted = two_manifest_composite("0".repeat(64));
                let err = assert_schema_freeze(&drifted)
                    .expect_err("the loser pack's manifest must be enforced too");
                let msg = format!("{err:#}");
                assert!(
                    msg.contains("note") && msg.contains("schema-hash mismatch"),
                    "the failure must name the drifted `note` hash mismatch; got: {msg}",
                );
            }
        }
    }

    mod provenance {
        use super::super::*;
        use engine::packsource::{EMBEDDED_PATH, PackProvenance};
        use std::path::{Path, PathBuf};

        /// A throwaway directory that removes itself on drop.
        struct TempDir(PathBuf);

        impl TempDir {
            fn new(tag: &str) -> Self {
                let mut path = std::env::temp_dir();
                path.push(format!(
                    "jigc-prov-unit-{tag}-{}-{:?}",
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

        /// Seed `<root>/<kind_dir>/<file>` with `bytes`.
        fn seed(root: &Path, kind_dir: &str, file: &str, bytes: &[u8]) {
            let dir = root.join(kind_dir);
            std::fs::create_dir_all(&dir).expect("create kind dir");
            std::fs::write(dir.join(file), bytes).expect("seed pack resource");
        }

        /// (a) A two-pack composite (a directory `FilesystemPack` over the embedded
        /// base) yields **one provenance entry per constituent, in precedence
        /// order**: the listed `FilesystemPack` first carrying *its own* root path,
        /// the embedded base second carrying the `<embedded>` sentinel — and each
        /// entry has a **non-empty** blake3 content-hash. This is the net-new data
        /// surface (`design/multi-pack.md` → Provenance under N packs:
        /// path + content-hash, never id/version alone).
        #[test]
        fn two_pack_entries_carry_own_path_and_hash_in_order() {
            let listed = TempDir::new("listed");
            seed(
                listed.path(),
                "workflows",
                "only-here.yaml",
                b"when: listed\n",
            );

            let composite = CompositePack::new(vec![
                Box::new(FilesystemPack::new(listed.path().to_owned())),
                Box::new(EmbeddedPack::new()),
            ]);

            let entries = composite.provenance_entries();
            assert_eq!(
                entries.len(),
                2,
                "one entry per constituent, in precedence order; got {entries:?}",
            );

            // Highest-precedence first: the listed FilesystemPack names its own root.
            assert_eq!(
                entries[0].path,
                listed.path().display().to_string(),
                "the listed pack's entry must carry its own resolving path",
            );
            assert!(
                !entries[0].content_hash.is_empty(),
                "the listed pack's content-hash must be non-empty; got {:?}",
                entries[0],
            );

            // The embedded base reports the `<embedded>` sentinel, but a real hash.
            assert_eq!(
                entries[1].path, EMBEDDED_PATH,
                "the embedded base must report the `<embedded>` path sentinel",
            );
            assert!(
                !entries[1].content_hash.is_empty(),
                "the embedded base still computes a real content-hash; got {:?}",
                entries[1],
            );

            // The two constituents' hashes differ (distinct bytes) — the hash is
            // genuinely content-derived, not a constant.
            assert_ne!(
                entries[0].content_hash, entries[1].content_hash,
                "distinct packs must hash distinctly",
            );
        }

        /// (b) **Determinism / hardening #7** — a `FilesystemPack`'s content-hash is
        /// `read_dir`-order invariant: two directory packs holding the **same**
        /// resources whose files were *created in divergent orders* (id-order vs
        /// reverse) hash **byte-identically**. The content-hash iterates each kind's
        /// already-sorted `list()` and frames each `(kind, id, bytes)` unit, so the
        /// underlying `read_dir` enumeration order never reaches the digest.
        #[test]
        fn content_hash_is_read_dir_order_invariant() {
            // Forward: files created in ascending id order.
            let fwd = TempDir::new("fwd");
            seed(fwd.path(), "steps", "aaa.txt", b"alpha\n");
            seed(fwd.path(), "steps", "mmm.txt", b"middle\n");
            seed(fwd.path(), "steps", "zzz.txt", b"omega\n");

            // Reverse: the identical resource set, files created in descending order.
            let rev = TempDir::new("rev");
            seed(rev.path(), "steps", "zzz.txt", b"omega\n");
            seed(rev.path(), "steps", "mmm.txt", b"middle\n");
            seed(rev.path(), "steps", "aaa.txt", b"alpha\n");

            let fwd_hash = FilesystemPack::new(fwd.path().to_owned()).content_hash();
            let rev_hash = FilesystemPack::new(rev.path().to_owned()).content_hash();

            assert_eq!(
                fwd_hash, rev_hash,
                "the content-hash must be identical regardless of file-creation / read_dir order",
            );
            assert!(!fwd_hash.is_empty(), "the content-hash must be non-empty");
        }

        /// (c) The single-pack **floor**: `Composite([base])` yields **exactly one**
        /// provenance entry — byte-identical to the base pack's own
        /// `provenance_entries()` degrade (one `PackProvenance`), mirroring the
        /// one-segment `provenance_segments()` floor. The `--explain` line degrades
        /// to one pack when only one composes.
        #[test]
        fn single_pack_composite_yields_one_entry() {
            let base = EmbeddedPack::new();
            let composite = CompositePack::new(vec![Box::new(EmbeddedPack::new())]);

            let entries = composite.provenance_entries();
            assert_eq!(
                entries.len(),
                1,
                "a one-pack composite yields exactly one provenance entry; got {entries:?}",
            );
            assert_eq!(
                entries,
                vec![PackProvenance {
                    path: base.resolving_path(),
                    content_hash: base.content_hash(),
                }],
                "the lone entry must equal the base pack's own provenance-entry degrade",
            );
            // The provenance-entry count tracks the provenance-segment count (both
            // degrade to one for a single-pack composite).
            assert_eq!(
                composite.provenance_entries().len(),
                composite.provenance_segments().len(),
                "the entry count must mirror the one-segment provenance_segments floor",
            );
        }
    }

    /// The second embedded pack — the methodology tree carried in-binary by a
    /// second `include_dir!`, selected by [`EmbeddedPack::methodology`]. Proves
    /// the field-carrying `EmbeddedPack` can serve a *different* `&'static Dir`
    /// than the dev base, that its content loads through the **production**
    /// loaders, that both selectors honour the binary-version invariant, and that
    /// the embedded methodology `Dir` carries no `target/`/build subtree (only the
    /// four resource dirs). See `design/multi-pack.md` → Embedded second pack;
    /// `module-layout.md` → Pack distribution.
    mod methodology_pack {
        use super::super::*;

        /// The methodology selector lists + reads its own workflows and schemas
        /// back **non-empty** through the production loaders the binary uses
        /// (`load_workflow_def` for workflow front-matter, `load_pack_schema` for
        /// the field-type-resolving doctype path) — the bullet-2 "confirm the
        /// embedded methodology content loads" check. A typo'd key, a mis-nested
        /// field, or a `deny_unknown_fields` violation on the embedded methodology
        /// content fails here, so this is the clean real-binary load proof for the
        /// second pack.
        #[test]
        fn methodology_selector_lists_and_loads_through_production_loaders() {
            let pack = EmbeddedPack::methodology();

            // The methodology workflows compose its surface: dev-task, planning,
            // completion all ride in-binary.
            let workflows = pack.list(PackResourceKind::Workflows);
            for id in ["dev-task", "planning", "completion"] {
                assert!(
                    workflows.contains(&ResourceId::from(id)),
                    "the methodology selector must ship the `{id}` workflow; got {workflows:?}",
                );
                let bytes = pack
                    .read(PackResourceKind::Workflows, &ResourceId::from(id))
                    .unwrap_or_else(|e| panic!("methodology workflow `{id}` reads back: {e}"));
                assert!(
                    !bytes.is_empty(),
                    "methodology workflow `{id}` must be non-empty YAML",
                );
                // Through the production front-matter loader, not a hand-parse.
                engine::compose::load_workflow_def(&bytes)
                    .unwrap_or_else(|e| panic!("methodology workflow `{id}` loads: {e:?}"));
            }

            // The persisted methodology doctype `roadmap` loads through the
            // field-type-resolving production schema loader.
            let schemas = pack.list(PackResourceKind::Schemas);
            assert!(
                schemas.contains(&ResourceId::from("roadmap")),
                "the methodology selector must ship the `roadmap` doctype; got {schemas:?}",
            );
            let bytes = pack
                .read(PackResourceKind::Schemas, &ResourceId::from("roadmap"))
                .expect("the methodology `roadmap` schema reads back");
            assert!(!bytes.is_empty(), "the `roadmap` schema must be non-empty");
            load_pack_schema(&pack, &bytes)
                .expect("the methodology `roadmap` schema loads through load_pack_schema");
        }

        /// **Binary-version invariant for both selectors.** Both the dev base and
        /// the methodology selector report `pack_version = CARGO_PKG_VERSION` (the
        /// binary release) — the methodology pack's own `defaults.yaml: 0.1.0` is
        /// *not* its embedded version, so override-reconciliation keys both
        /// embedded packs to one release (`multi-pack.md` → Version ties to the
        /// binary).
        #[test]
        fn both_selectors_report_the_binary_version() {
            assert_eq!(
                EmbeddedPack::new().pack_version(),
                env!("CARGO_PKG_VERSION"),
                "the dev selector reports the binary version",
            );
            assert_eq!(
                EmbeddedPack::methodology().pack_version(),
                env!("CARGO_PKG_VERSION"),
                "the methodology selector reports the binary version, not its 0.1.0 defaults",
            );
        }

        /// The two selectors carry **distinct** trees: the methodology selector
        /// lists `dev-task`/`planning`/… that the dev base does not, and the dev
        /// base lists `single-task`/`router`/… the methodology pack does not — so
        /// the field genuinely selects a different `&'static Dir`, not the same one
        /// twice.
        #[test]
        fn the_two_selectors_carry_distinct_trees() {
            let dev = EmbeddedPack::new().list(PackResourceKind::Workflows);
            let methodology = EmbeddedPack::methodology().list(PackResourceKind::Workflows);

            assert!(
                methodology.contains(&ResourceId::from("dev-task")),
                "methodology lists its own `dev-task`; got {methodology:?}",
            );
            assert!(
                !dev.contains(&ResourceId::from("dev-task")),
                "the dev base must NOT carry methodology's `dev-task`; got {dev:?}",
            );
            assert!(
                dev.contains(&ResourceId::from("single-task")),
                "the dev base lists its own `single-task`; got {dev:?}",
            );
            assert!(
                !methodology.contains(&ResourceId::from("single-task")),
                "the methodology pack must NOT carry dev's `single-task`; got {methodology:?}",
            );
        }

        /// The embedded methodology `Dir` carries **only** the resource dirs
        /// (`workflows/`, `schemas/`, `steps/`, `config/`, and — since the M41 F4
        /// v1→v2 `deferral-ledger` rename — `schema-snapshots/`) — never a `target/`
        /// / build subtree. The methodology tree is pure YAML data, so an embed that
        /// swept a build tree would re-introduce the M20 bloat. Guards that the
        /// `include_dir!` root holds no `target/`, mirroring
        /// `embedded_pack_carries_no_probes_directory` for the dev pack.
        #[test]
        fn methodology_dir_carries_no_build_subtree() {
            assert!(
                METHODOLOGY.get_dir("target").is_none(),
                "the embedded methodology Dir must not carry a `target/` build subtree",
            );
            let top_level: Vec<&str> = METHODOLOGY
                .dirs()
                .filter_map(|d| d.path().file_name().and_then(|n| n.to_str()))
                .collect();
            for name in &top_level {
                assert!(
                    matches!(
                        *name,
                        "workflows" | "schemas" | "steps" | "config" | "schema-snapshots"
                    ),
                    "the methodology Dir must hold only the resource dirs; saw `{name}` \
                     among {top_level:?}",
                );
            }
        }

        /// **Knob-surface parity — the whole-`knobs.yaml`-shadow drift guard (M40).**
        /// In a methodology-primary project (`.jigc/config/packs.yaml` listing the
        /// methodology pack) the methodology `knobs.yaml` whole-file-shadows dev's
        /// (listed > base), so a dev-only knob key becomes **undeclared** there —
        /// its pinned pack-default unreachable and the key unsettable (a
        /// `scalar-set` on it hard-aborts `UndeclaredScalar`). That is exactly how
        /// the M36–M40 dev-knob mints (`invocation-log`, `mention-resolves`,
        /// `title-names-symbol`, `repeatable-populated.severity`/`.exempt`,
        /// `surplus-sections-absent`) silently vanished from methodology-primary
        /// projects, leaving `repeatable-populated.exempt`'s methodology tokens
        /// (`milestone-record#tasks`, `completion-record#findings`) dead in the
        /// only context they can apply. This test pins the recorded safety
        /// condition (`multi-pack.md` → Per-kind collision behavior): the two knob
        /// surfaces differ on **only**
        /// - `default-workflow` — divergent by design (disjoint enums), and
        /// - `docs-root` — dev-only by record: methodology doctypes land flat at
        ///   their `location:` (`DECISIONS.md` 2026-07-04, M37 inc-4 T3).
        ///
        /// Every other dev knob must be declared **identically** (type/of/default/
        /// floor) in the methodology pack, so the next dev-knob mint fails here
        /// instead of drifting silently.
        #[test]
        fn methodology_knob_surface_mirrors_dev_except_recorded_divergences() {
            use std::collections::BTreeSet;

            let load = |pack: &EmbeddedPack, which: &str| {
                engine::knobs::load_knobs(
                    &pack
                        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
                        .unwrap_or_else(|e| panic!("{which} knobs.yaml reads back: {e}")),
                )
                .unwrap_or_else(|e| panic!("{which} knobs.yaml loads: {e}"))
            };
            let dev = load(&EmbeddedPack::new(), "dev");
            let methodology = load(&EmbeddedPack::methodology(), "methodology");

            // The two recorded divergences — everything else must mirror.
            const DEV_ONLY: &[&str] = &["docs-root"];
            const DIVERGENT_DECL: &[&str] = &["default-workflow"];

            let dev_keys: BTreeSet<&str> = dev.keys().collect();
            let methodology_keys: BTreeSet<&str> = methodology.keys().collect();

            let missing: Vec<&str> = dev_keys
                .iter()
                .filter(|k| !DEV_ONLY.contains(k) && !methodology_keys.contains(*k))
                .copied()
                .collect();
            assert!(
                missing.is_empty(),
                "dev knob keys missing from packs/methodology/config/knobs.yaml — a \
                 methodology-primary project loses each to the whole-file shadow \
                 (pinned default unreachable, key unsettable); mirror them (or record \
                 a divergence here AND in multi-pack.md): {missing:?}",
            );
            let surplus: Vec<&str> = methodology_keys
                .iter()
                .filter(|k| !dev_keys.contains(*k))
                .copied()
                .collect();
            assert!(
                surplus.is_empty(),
                "methodology declares knob keys the dev base lacks: {surplus:?}",
            );

            let dev_defaults = dev.base_scalars();
            let methodology_defaults = methodology.base_scalars();
            for key in dev_keys
                .iter()
                .filter(|k| !DEV_ONLY.contains(k) && !DIVERGENT_DECL.contains(k))
            {
                assert_eq!(
                    dev.field(key),
                    methodology.field(key),
                    "knob `{key}`: type/of must be identical across the two packs",
                );
                assert_eq!(
                    dev_defaults.get(*key),
                    methodology_defaults.get(*key),
                    "knob `{key}`: the pinned default must be identical across the two packs",
                );
                assert_eq!(
                    dev.floors().get(*key),
                    methodology.floors().get(*key),
                    "knob `{key}`: the demotion-lock floor must be identical across the two packs",
                );
            }
        }

        /// (M42 Increment 4 / T1) The minted version-currency key is declared in **both**
        /// shipped packs, floored `blocking` in each — the mint that the parity guard
        /// above would otherwise catch only *after* a methodology-primary project had
        /// silently lost the key to the whole-file shadow. `validation.md` → MVP check
        /// inventory (the `schema-version-current` row) + → Version-currency is itself a
        /// surfaced break (the retraction: *intrinsic + keyed*).
        ///
        /// This is the pack-side half of the mint. The engine-side half (membership in
        /// `INTRINSIC_CHECK_KEYS` and in `CHECK_INVENTORY`) is pinned by
        /// `engine::knobs::schema_version_current_key_is_declared_intrinsic_blocking`
        /// together with `engine::result::schema_version_current_is_a_check_inventory_row`.
        #[test]
        fn both_packs_declare_the_version_currency_severity_knob() {
            const KEY: &str = "validation.schema-conformance.schema-version-current.severity";

            for (which, pack) in [
                ("dev", EmbeddedPack::new()),
                ("methodology", EmbeddedPack::methodology()),
            ] {
                let knobs = engine::knobs::load_knobs(
                    &pack
                        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
                        .unwrap_or_else(|e| panic!("{which} knobs.yaml reads back: {e}")),
                )
                .unwrap_or_else(|e| panic!("{which} knobs.yaml loads: {e}"));

                assert!(
                    knobs.field(KEY).is_some(),
                    "{which}: `{KEY}` must be declared — an undeclared key hard-aborts a \
                     `scalar-set` (UndeclaredScalar) and exempts the check from the severity \
                     post-pass",
                );
                assert_eq!(
                    knobs.base_scalars().get(KEY).map(String::as_str),
                    Some("blocking"),
                    "{which}: the version-currency break is blocking by default",
                );
                assert_eq!(
                    knobs.floors().get(KEY).map(String::as_str),
                    Some("blocking"),
                    "{which}: intrinsic — floored at blocking, never demotable",
                );
            }
        }
    }
}
