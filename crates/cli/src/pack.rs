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
use std::path::{Path, PathBuf};

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
    governing_entry(pack, ty).map(|entry| entry.schema_version)
}

/// The doctype's **governing manifest entry** — the whole declaration
/// ([`engine::manifest::ManifestEntry`]: version *and* frozen hash), resolved by the
/// same per-origin-pack rule [`governing_version`] projects the version out of. Split
/// out because the project-layer freeze arm ([`assert_project_schema_shadows`]) needs
/// the declared **hash**, and asking the same question twice through two resolutions
/// is how the M40 split-brain happened. `None` when the origin pack ships no valid
/// manifest, or its manifest omits `ty` — the freeze-exempt case.
fn governing_entry(pack: &dyn PackSource, ty: &str) -> Option<engine::manifest::ManifestEntry> {
    let owner = pack.origin_pack(PackResourceKind::Schemas, &ResourceId::from(ty));
    own_manifest(owner)?
        .doctypes
        .into_iter()
        .find(|e| e.ty == ty)
}

/// The **freeze-governed doctype id-space** of the given packs: every `type:` declared
/// in any of their own `config/schema-manifest.yaml` files, unioned. Read from the
/// manifests themselves — the set is never hand-listed, so it cannot drift from the
/// freeze it represents.
///
/// This is the boundary [`FreezeDemotion`] applies: a project-listed pack composes
/// **above** the embedded pair for every id *outside* this set (a house doctype is a
/// genuine extension) and **below** it for every id inside (a project pack may not
/// shadow a doctype the freeze governs — `design/multi-pack.md` → Embedded second pack).
/// A pack shipping no valid manifest contributes nothing, the same skip-on-absent
/// posture [`own_manifest`] and [`assert_schema_freeze`] already take.
fn governed_doctype_ids(packs: &[&dyn PackSource]) -> std::collections::BTreeSet<String> {
    packs
        .iter()
        .filter_map(|pack| own_manifest(*pack))
        .flat_map(|manifest| manifest.doctypes.into_iter().map(|entry| entry.ty))
        .collect()
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

/// The composed pack-set's **migratable doctypes** — those shipping the `migrate-<ty>`
/// workflow that `jigc migrate <path> --as <ty>` composes. The **M40 two-tier route's**
/// condition (`design/validation.md` → the M40 two-tier route): the adoption route names that
/// doctype-directed verb only where it exists, because `jigc migrate` hard-errors *"not
/// migratable"* when the composed pack ships no such workflow — never command a verb that
/// hard-errors.
///
/// Derived from the workflow ids exactly as `jigc doc show`'s read-side reroute derives it
/// (`crate::doc` → `reroute_unadopted`), lifted here because the task-scope reconciler is now
/// a third consumer (M48 Inc 4 / T1). The set is a plain `migrate-` prefix strip, so a
/// non-doctype workflow like `migrate-corpus` contributes a member no doctype lookup ever
/// matches — membership is only ever asked with a real doctype in hand.
pub(crate) fn migratable_doctypes(pack: &dyn PackSource) -> std::collections::BTreeSet<String> {
    pack.list(PackResourceKind::Workflows)
        .iter()
        .filter_map(|id| id.as_str().strip_prefix("migrate-").map(str::to_owned))
        .collect()
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
/// pack, and the read is `docs-root`-independent (the manifest stores the raw
/// declared `location:`, so the hash is taken before
/// [`crate::start::apply_docs_root`] ever nests it).
///
/// **The project layer is checked too** ([`assert_project_schema_shadows`], M49).
/// This comment used to state the *opposite* — that the read is
/// "shadow-independent" — and offered it as a feature. It was a hole: a whole-file
/// definition shadow at `.jigc/config/schemas/<ty>.yaml` (`design/overrides.md` →
/// Authored metadata on a definition resolves by whole-file shadow) is the resolved
/// schema at **every** surface, so it could drop four sections from a frozen doctype
/// and leave `jigc validate` at exit 0, `jigc doc schema` reporting the frozen
/// `schema-version` for an unfrozen shape, and `doc create` writing a third. The
/// freeze binds at every layer that can change a schema, so a project-owned id is
/// hashed **resolved**. The documented capability is untouched: M47's presentation
/// projection erases `description:` / `usage:` / slot `hint:`, so a prose reword
/// still shadows cleanly — the shape may not move.
///
/// An **absent** manifest is skipped (no owners → `Ok(())`) — the
/// field-types-absent precedent ([`pack_field_types`]): a seeded / composed pack
/// that ships no manifest stays unchecked, so only a pack that opts into the freeze
/// is held to it. That opt-out is **wholesale**, and it is the only one: a pack that
/// *does* ship a manifest is held to all of it, including the `slug-rule:` block —
/// an omitted block is [`engine::manifest::ManifestError::SlugRuleUndeclared`], not a
/// second, quieter opt-out (M42 audit; `design/storage.md` → Identity → *The slug rule
/// is itself a versioned rule*).
pub fn assert_schema_freeze(
    pack: &dyn PackSource,
    project_config: Option<&Path>,
) -> anyhow::Result<()> {
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

    assert_project_schema_shadows(pack, project_config)
}

/// The **project-layer arm** of the freeze assertion: a whole-file schema shadow at
/// `<project_config>/schemas/<ty>.yaml` is hashed **as resolved** and compared to the
/// governing manifest entry, so a project cannot change a frozen doctype's shape from
/// the layer that outranks every pack (M49 Increment 3; `design/corpus-migration.md` →
/// The freeze — declared *and* enforced; `design/overrides.md` → Authored metadata on
/// a definition resolves by whole-file shadow).
///
/// **Why hashed rather than refused by name.** `overrides.md` makes the whole-file
/// schema shadow the *stated* mechanism for overriding a doctype's authored
/// `description:` / `usage:` / slot `hint:`, and M47's presentation projection already
/// erases exactly those three keys from the hash. Hashing the resolved schema
/// therefore refuses precisely what the pack layer already forbids and permits
/// precisely what the pack layer already permits — one rule, both layers — where a
/// refusal by name would delete a documented capability for all sixteen shipped
/// doctypes.
///
/// **Scope, on the same opt-in as the pack arm.** A doctype is checked iff (a) some
/// pack in the set actually ships the id — a project schema no pack ships is not a
/// shadow and is invisible to `CascadeDefs::all_schemas`, which enumerates pack ids —
/// and (b) its **origin pack** (the constituent whose definition the shadow displaces)
/// declares it in a manifest ([`governing_entry`], the M40 per-origin rule). So a
/// shadow of a manifest-less pack's doctype stays unchecked, exactly as that pack's
/// own schemas do: the freeze records what a pack *declares* frozen, at either layer.
///
/// `None` (no discoverable project config) is the cold-start floor — nothing to check.
fn assert_project_schema_shadows(
    pack: &dyn PackSource,
    project_config: Option<&Path>,
) -> anyhow::Result<()> {
    let Some(project_config) = project_config else {
        return Ok(());
    };
    for ty in crate::start::project_schema_ids(project_config) {
        let id = ResourceId::from(ty.as_str());
        // (a) Only a genuine *shadow* — an id the pack set ships — resolves through
        // the cascade at all.
        if pack.read(PackResourceKind::Schemas, &id).is_err() {
            continue;
        }
        // (b) Only a doctype its origin pack declares frozen is governed.
        let Some(entry) = governing_entry(pack, &ty) else {
            continue;
        };
        let path = project_config.join("schemas").join(format!("{ty}.yaml"));
        let bytes = std::fs::read(&path).map_err(|err| {
            anyhow::anyhow!(
                "the project schema shadow {} is unreadable: {err}",
                path.display()
            )
        })?;
        // Field types + the schema-version stamp resolve against the **origin** pack,
        // exactly as `CascadeDefs::all_schemas` loads a shadow, so the hash compared
        // here is the hash of the schema every other surface will use.
        let origin = pack.origin_pack(PackResourceKind::Schemas, &id);
        let schema = load_pack_schema(origin, &bytes).map_err(|err| {
            anyhow::anyhow!(
                "the project schema shadow {} is malformed: {err}",
                path.display()
            )
        })?;
        let actual = engine::manifest::schema_hash(&schema);
        if actual != entry.schema_hash {
            // The `route:` span is the **only** stated exit from a state in which every
            // door exits non-zero, and it is bytes an operator pastes into a shell — so
            // the one interpolated token in it is rendered through the same
            // [`crate::task::shell_token`] the mechanical route fence demands of every
            // emitted argv token (`crates/cli/src/route_fence.rs` → "every token must be
            // shell-safe as emitted"). This bail is a plain `anyhow::bail!` naming `rm`,
            // not `jigc`, so it sits outside that fence's domain by construction and
            // nothing else catches it: interpolated raw, a repo under `~/my repo` emitted
            // an exit that silently `rm`'d nothing and left every door blocked. The
            // *prose* occurrence below stays bare, as its two sibling bails in this
            // function do — it names a location, not a command line.
            let route_path = crate::task::shell_token(&path.display().to_string());
            anyhow::bail!(
                "pack-load freeze check failed: doctype `{ty}`: schema-hash mismatch                  (manifest declares `{expected}`, recomputed `{actual}`) — the project                  schema shadow {path} changes the shape of a frozen doctype, which the                  freeze forbids at every layer (`design/corpus-migration.md` → The freeze)
                 route: `rm {route_path}` restores the frozen shape — a project schema shadow may                  only reword the authored presentation keys (`description:`, `usage:`, a slot                  `hint:`); changing the shape or the home of a manifest-governed doctype means                  bumping its `schema-version` in the owning pack's                  `config/schema-manifest.yaml` and shipping a corpus migration",
                expected = entry.schema_hash,
                path = path.display(),
            );
        }
    }
    Ok(())
}

/// The `anyhow` error a pack-load fence raises when a definition it must read does not parse
/// — **one funnel for the family**, so all five sweeps name the resource, relay the loader's
/// diagnosis, and carry its locus in the same words (M49 Increment 8 / T4).
///
/// The locus is usually absent here and that is correct: the front-matter loaders raise at
/// `Location::at(1, 1)` — the placeholder coordinate the load boundary
/// later replaces with the resource address — so [`crate::render::finding_locus`] returns
/// `None` and the sentence reads exactly as it did. A loader that ever raises at a real line
/// in the definition says so here by construction, rather than by a fence author remembering.
fn def_load_failure(sweep: &str, id: &str, finding: engine::finding::Finding) -> anyhow::Error {
    let at = crate::render::finding_locus(&finding)
        .map(|locus| format!(" (at {locus})"))
        .unwrap_or_default();
    anyhow::anyhow!(
        "pack-load {sweep} sweep failed on `{}`: {}{at}",
        id,
        finding.message,
    )
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
/// **The suppression fence (law 2):** every workflow the router catalog leaves out
/// must declare `suppressed: {reason, expires}` — an absent capability carries a
/// machine-visible reason that can expire (`never` is legal for a
/// permanent-by-design absence; a *malformed* block is already rejected by the
/// loader's required-shape check). Missing ⇒ fail, naming the workflow — the
/// `decided-task` lesson made mechanical.
///
/// **The subject is the catalog's complement, not one of its two causes** (M49
/// Increment 11 / T6). The catalog's population is `creates-task: true &&
/// selectable: true`, so a workflow falls off it *either* by being hidden
/// (`selectable: false`) *or* by minting no task (`creates-task: false`) — and
/// `step:route-to-workflow` promises the reader a reason for **every** absence. Keyed
/// on `selectable` alone, the fence bought half that promise: `router`,
/// `ingest-existing` and `increment` sat off the catalog narrating nothing, while
/// `milestone-execution` — `creates-task: false` and voluntarily carrying a
/// `suppressed:` block — showed the shape was already the right one. `expires: never`
/// is the honest value for a structural absence.
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
                def_load_failure("workflow-front-matter", id.as_str(), finding)
            })?;
            if !(def.creates_task && def.selectable) && def.suppressed.is_none() {
                let cause = if def.selectable {
                    "`creates-task: false`"
                } else {
                    "`selectable: false`"
                };
                anyhow::bail!(
                    "pack-load suppression fence failed: workflow `{}` declares {cause} — so the \
                     router catalog leaves it out — with no `suppressed:` block; a capability \
                     off the catalog must carry a machine-visible reason \
                     (design/surface-contract.md → The suppression fence)\n\
                     route: add `suppressed: {{reason: <why the catalog leaves it out>, \
                     expires: never | <the condition that puts it on>}}` to the workflow's \
                     front-matter",
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

/// The **ambush-class constraint-identifier set** (M43 law 3 —
/// `design/surface-contract.md` → The stated-at fence, structural tier): the
/// identifiers whose binding contracts are irreducibly prose, so the statement
/// cannot be seam-generated from a code-owned constant — a soliciting step must
/// carry it and declare so (`states-constraints:` front-matter). Code-side
/// beside its assert, **not** pack config: the obligation is jigc's, not the
/// pack author's. Membership (verified against the real producers): three are
/// **minted finding codes** — `finalize.promote-clobber` (the
/// `--approve`/clobber/retire contract, engine `finalize.rs`) ·
/// `finalize.nothing-staged` (the CLI index-empty block, `task.rs`) · the M43
/// `finalize.carried-staged` carryover gate (engine `finalize.rs`) — and one is
/// **not**: `finalize.left-out` names the staging contract whose only
/// production surface is the M42 pre/post-commit left-out **print**
/// (`task.rs::emit_left_out_advisory` / `render::left_out_advisory`) —
/// print-over-refuse by the M42 settle, so no producer mints it as a `Finding`;
/// the string serves here purely as the contract's declared identifier.
const AMBUSH_CLASS_CODES: [&str; 4] = [
    "finalize.promote-clobber",
    "finalize.left-out",
    "finalize.nothing-staged",
    "finalize.carried-staged",
];

/// **The stated-at fence (law 3, structural tier)** — `design/surface-contract.md`
/// → The stated-at fence: every member of [`AMBUSH_CLASS_CODES`] must have at
/// least one declarer among the pack's steps' `states-constraints:` front-matter,
/// so each contract is stated where it binds instead of first appearing in its
/// block message (an ambush even when the block is correct).
///
/// Scope mirrors [`assert_workflow_front_matter`]: **manifest-shipping
/// constituents, each checked in isolation** — every shipped pack loaded *alone*
/// (the methodology-alone dogfood path) must carry every declarer itself; a
/// manifest-less seeded / project-local pack stays on skip-on-absent. Both sides
/// are structural (the members are the fixed identifiers of
/// [`AMBUSH_CLASS_CODES`] — mostly M42-keyed finding codes, one a declared
/// print-surface contract identifier; the declaration is YAML) — no
/// prose-matching. Honest bound: this proves the
/// *obligation* is carried, never that the prose is good (the review checklist's
/// job).
fn assert_stated_at(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        let mut declared = std::collections::BTreeSet::new();
        for id in owner.list(PackResourceKind::Steps) {
            let bytes = owner
                .read(PackResourceKind::Steps, &id)
                .with_context(|| format!("the `{}` step is unreadable", id.as_str()))?;
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .map_err(|finding| def_load_failure("step-front-matter", id.as_str(), finding))?;
            declared.extend(def.states_constraints);
        }
        let undeclared: Vec<&str> = AMBUSH_CLASS_CODES
            .into_iter()
            .filter(|code| !declared.contains(*code))
            .collect();
        if !undeclared.is_empty() {
            anyhow::bail!(
                "pack-load stated-at fence failed: no step of this pack declares \
                 `states-constraints:` for the ambush-class code(s) {} — the contract \
                 would first appear in its block message, an ambush \
                 (design/surface-contract.md → The stated-at fence)\n\
                 route: state each contract in the step that solicits the write it gates \
                 and declare its code in that step's `states-constraints:` front-matter",
                undeclared
                    .iter()
                    .map(|code| format!("`{code}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }
    }
    Ok(())
}

/// The copy-in/append contract's declared identifier (M44 Inc 6, D5 —
/// `design/surface-contract.md` → The stated-at fence, per-soliciting-step tier):
/// a step that solicits a **singleton** doctype's authoring must state that an
/// already-committed singleton is copied in as the edit base (authored items
/// append / slots overwrite), so the constraint is stated where it binds instead
/// of first surfacing when an authored item doubles (an ambush). Code-side beside
/// its assert — the obligation is jigc's, not the pack author's — and **not** a
/// minted `Finding` code (its production surface is the migrate step's own guide
/// prose, per the A-3 presence-only tier); the string serves as the declared
/// identifier the fence checks for.
const SINGLETON_COPY_IN_CODE: &str = "create.singleton-copy-in";

/// **The append/collision clause of the copy-in contract** (M47 Inc 9 T2, corrected
/// at M49 Inc 10 T2 — the conditional tier of the named-fact fence): the facts a
/// copy-in declarer owes *only when the singleton it solicits can hold repeating
/// items*. The flat map ([`CONSTRAINT_REQUIRED_TOKENS`]) carries what binds at
/// **every** declarer — copy-in as the edit base, slots overwrite; these carry what
/// binds where **items exist**: that authored items *append* beside the committed
/// ones, and what happens when one collides.
///
/// **The collision is a refusal, not a doubling.** The tier shipped demanding
/// *"would double"*, which the binary never does: a payload item whose title mints an
/// id the doc already holds is refused with `write.already-present`, the **whole
/// payload** rejected and nothing staged, leaving the author to edit that item **in
/// place** (`crates/cli/tests/author_write_contract.rs` drives all three facts end to
/// end over a committed singleton). A fence demanding a false consequence is a law-1
/// lie the fence itself mandates — the sharpest form of the class — so the tokens are
/// the behaviour, four facts rather than two.
///
/// The condition is the referenced singleton's own structure (≥1 `repeatable:`
/// section), so the tier is derived, never listed: flat-mapping these tokens onto
/// `create.singleton-copy-in` would force a slot-only declarer (methodology's
/// `author-migration-vision`, whose `vision` schema OVERWRITES and holds no items to
/// collide) to state facts that are false there. Authored in [`normalized_body`]'s
/// form, fenced by `constraint_tokens_are_authored_in_normalized_form`.
///
/// Declared bound, unchanged: this fence reaches the **declarer family** only — a step
/// that solicits the same batch author through a literal `jigc doc author <T>` line
/// and no `{{schema:<T>}}` ref (dev's `author-change`) carries no `states-constraints:`
/// code and is outside every pack-load fence. The clause is swept over *both* shapes
/// by `author_write_contract.rs`, which bijects its list against this const so the two
/// halves cannot drift.
pub const COPY_IN_APPEND_TOKENS: [&str; 4] = [
    "append",
    "write.already-present",
    "whole payload",
    "in place",
];

/// Extract every `<T>` from a step body's `{{schema:<T>}}` references — the same
/// `schema:`-prefix the compose seam strips (`engine::compose` → the schema
/// projection). Whitespace-tolerant inside the braces and around the type id.
fn schema_refs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(open) = rest.find("{{") {
        rest = &rest[open + 2..];
        let Some(close) = rest.find("}}") else { break };
        let inner = rest[..close].trim();
        if let Some(ty) = inner.strip_prefix("schema:") {
            out.push(ty.trim().to_owned());
        }
        rest = &rest[close + 2..];
    }
    out
}

/// **The stated-at fence, per-soliciting-step tier (law 3, D5)** —
/// `design/surface-contract.md` → The stated-at fence: every step whose body
/// references `{{schema:<T>}}` where `T` is a **singleton** doctype (a
/// create-or-update singleton author solicit) must declare
/// [`SINGLETON_COPY_IN_CODE`] in its `states-constraints:` front-matter, so the
/// copy-in/append constraint is stated where the authoring is solicited rather
/// than first surfacing when an authored item doubles.
///
/// The owe-set is **derived from the enumerable structural signal the step
/// already renders** (the `{{schema:<T>}}` ref × `T`'s `singleton` flag — B2-baked:
/// no separate `authors-into:` marker to drift), so a soliciting template that
/// omits the statement reddens at pack-load, not by author diligence. Scope
/// mirrors [`assert_stated_at`]: manifest-shipping constituents, each checked in
/// isolation; **per-origin schema resolution suffices** — each soliciting step
/// references a singleton of its own origin pack (the composition model's
/// cross-pack solicit does not arise for these five steps). A manifest-less pack
/// stays on skip-on-absent.
///
/// **The conditional append/collision tier (M47 Inc 9 T2; corrected M49 Inc 10 T2).**
/// The declaration bought presence; the named-fact map
/// ([`CONSTRAINT_REQUIRED_TOKENS`]) buys what binds at *every* declarer. What binds
/// only where **items can collide** rides here, on the same schema load: when the
/// solicited singleton declares ≥1 `repeatable:` section, the step must also state
/// [`COPY_IN_APPEND_TOKENS`] — that authored items append beside the committed ones,
/// and that one whose title mints an id the doc already holds is refused with
/// `write.already-present`, the whole payload rejected, the exit being to edit that
/// item in place. The condition is the referenced schema's own structure, so the tier
/// is derived exactly like the owe-set above and needs no exclusion list: a slot-only
/// singleton's declarer (methodology's `author-migration-vision`) is inert here, never
/// in error — flat-mapping the tokens would force it to state facts that are false
/// there. Same A-3 bound: the *named facts* of jigc's own contract, never prose
/// quality.
///
/// **The obligation direction (M47 Inc 9 T3).** Every tier above runs *ref ⇒
/// declaration ⇒ named facts*; this runs the reverse, closing the pair into a
/// **biconditional**: a step declaring [`SINGLETON_COPY_IN_CODE`] must still
/// reference a `{{schema:<T>}}` whose `T` resolves to a singleton. Without it,
/// deleting the ref deletes the *obligation itself* — every tier above goes inert
/// (there is no solicit left to fence) while the step's prose keeps promising a
/// payload that never follows. Demonstrated live: with `{{schema:changelog}}`
/// deleted, the dev pack loads clean and `record-changelog` composes *"The target
/// schema and its batch payload … follow"* above nothing.
/// `workflow-refs.schema-ref-resolves` guards only the opposite case (ref present,
/// schema absent) and structurally cannot see this one. The check is **structural,
/// not token-based** — no prose-matching of the promise sentence, which would leave
/// the A-3 bound. Declared boundary: a step deleting **both** its ref and its
/// declaration leaves the fence family entirely (the family is *every step carrying
/// a `states-constraints:` code*), so that case is out of scope by the family's own
/// definition rather than silently narrowed.
fn assert_singleton_copy_in_stated(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        for id in owner.list(PackResourceKind::Steps) {
            let bytes = owner
                .read(PackResourceKind::Steps, &id)
                .with_context(|| format!("the `{}` step is unreadable", id.as_str()))?;
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .map_err(|finding| def_load_failure("step-front-matter", id.as_str(), finding))?;
            let mut solicits_singleton = false;
            let mut items_can_collide = false;
            for ty in schema_refs(&def.body) {
                let Some(schema) = owner
                    .read(PackResourceKind::Schemas, &ResourceId::from(ty.as_str()))
                    .ok()
                    .and_then(|b| load_pack_schema(owner, &b).ok())
                    .filter(|schema| schema.singleton)
                else {
                    continue;
                };
                solicits_singleton = true;
                // The conditional tier's signal: a `repeatable:` section means the
                // authored items land beside the committed ones, so a payload item can
                // collide with a committed one and the clause binds at this solicit.
                items_can_collide |= schema.sections.iter().any(|section| {
                    matches!(section.body, engine::schema::SectionBody::Repeatable { .. })
                });
            }
            let declares_copy_in = def
                .states_constraints
                .iter()
                .any(|c| c == SINGLETON_COPY_IN_CODE);
            if solicits_singleton && !declares_copy_in {
                anyhow::bail!(
                    "pack-load stated-at fence failed: step `{}` solicits a singleton \
                     doctype's authoring (a `{{{{schema:<T>}}}}` ref with `T` singleton) but does \
                     not declare `{SINGLETON_COPY_IN_CODE}` in `states-constraints:` — the \
                     copy-in/append contract would first surface when an authored item doubles, \
                     an ambush (design/surface-contract.md → The stated-at fence)\n\
                     route: state the copy-in/append constraint above the authoring solicit and \
                     declare `{SINGLETON_COPY_IN_CODE}` in the step's `states-constraints:` \
                     front-matter",
                    id.as_str(),
                );
            }
            // The obligation direction (T3): the declaration is only worth what it
            // guards. Deleting the `{{schema:<T>}}` ref deletes the write the
            // contract binds to, while the step's prose keeps promising a payload
            // that never follows.
            if declares_copy_in && !solicits_singleton {
                anyhow::bail!(
                    "pack-load stated-at fence failed: step `{}` declares \
                     `{SINGLETON_COPY_IN_CODE}` but its body references no \
                     `{{{{schema:<T>}}}}` with `T` a resolvable singleton — the declared \
                     copy-in/append contract no longer guards any authoring solicit, while the \
                     composed step still promises a schema payload that never follows, an ambush \
                     (design/surface-contract.md → The stated-at fence)\n\
                     route: restore the `{{{{schema:<T>}}}}` reference this step's prose \
                     promises — or, if the step no longer solicits a singleton's authoring, drop \
                     `{SINGLETON_COPY_IN_CODE}` from its `states-constraints:` front-matter \
                     together with the contract sentences it stands for",
                    id.as_str(),
                );
            }
            if items_can_collide {
                let body = normalized_body(&def.body);
                let missing: Vec<&str> = COPY_IN_APPEND_TOKENS
                    .into_iter()
                    .filter(|token| !body.contains(token))
                    .collect();
                if !missing.is_empty() {
                    anyhow::bail!(
                        "pack-load named-fact fence failed: step `{}` declares \
                         `{SINGLETON_COPY_IN_CODE}` and solicits a singleton whose sections \
                         repeat (authored items land beside the committed ones) but its prose \
                         never says {} — the append/collision half of the copy-in contract \
                         would first surface when an authored item is refused, an ambush \
                         (design/surface-contract.md → The stated-at fence)\n\
                         route: state, beside the copy-in sentence, that what you author \
                         APPENDS to the items already committed and that an item whose title \
                         mints an id the doc already holds is refused \
                         (`write.already-present`), the WHOLE payload rejected and nothing \
                         staged — so edit that item in place instead of re-authoring it",
                        id.as_str(),
                        missing
                            .iter()
                            .map(|token| format!("\"{token}\""))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
            }
        }
    }
    Ok(())
}

/// Every lone-line `{{ cli.<id> }}` reference of a step body. A `{{cli.…}}`
/// placeholder renders only as a whole line (the compose seam's class rule —
/// `engine::compose` → the command-ref emitter), so the scan is per line rather
/// than the free-text sweep [`schema_refs`] performs for its own class.
fn cli_refs(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| {
            let inner = line.trim().strip_prefix("{{")?.strip_suffix("}}")?.trim();
            let id = inner.strip_prefix("cli.")?.trim();
            (!id.is_empty() && !id.contains(char::is_whitespace)).then(|| id.to_owned())
        })
        .collect()
}

/// The pack's own catalog ids whose command-ref is a `jigc doc <write-verb>` call —
/// the first signal of the read-back fence's owe-set. The write-verb partition is
/// the production one ([`crate::doc::doc_write_verbs`]: the clap `doc` leaf set
/// minus the declared read verbs), so a `doc` verb added later widens this set
/// instead of needing a second hand list beside it.
///
/// A pack that ships no readable/parseable catalog contributes nothing here (its
/// `{{cli.<id>}}` refs cannot resolve at all, and composition surfaces that on its
/// own front door) — the same skip-on-absent posture the fences take toward a
/// manifest-less pack, and the `{{schema:<T>}}` arm still applies.
fn doc_write_command_ids(owner: &dyn PackSource) -> std::collections::BTreeSet<String> {
    let Some(catalog) = owner
        .read(PackResourceKind::Config, &ResourceId::from("commands"))
        .ok()
        .and_then(|bytes| engine::compose::load_command_catalog(&bytes).ok())
    else {
        return std::collections::BTreeSet::new();
    };
    let write_verbs = crate::doc::doc_write_verbs();
    catalog
        .commands
        .iter()
        .filter(|(_, command)| {
            if command.command != "jigc" {
                return false;
            }
            // The leading literal args are the verb path; a `from:`/`agent:` arg in
            // between is a value, never part of it.
            let mut literals = command.args.iter().filter_map(|arg| match arg {
                engine::compose::CommandArg::Literal { literal } => Some(literal.as_str()),
                _ => None,
            });
            literals.next() == Some("doc")
                && literals
                    .next()
                    .is_some_and(|verb| write_verbs.iter().any(|write| write == verb))
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// **The stated-at fence, write-solicit tier (law 2, M48 Inc 3)** —
/// `design/surface-contract.md` → The stated-at fence: a step that solicits a
/// managed-doc write must declare [`STAGED_READ_BACK_CODE`], and (via
/// [`CONSTRAINT_REQUIRED_TOKENS`]) state the read-back it stands for —
/// `jigc doc show <addr> --task <id>`, the staged read that shows the agent what it
/// just wrote.
///
/// The owe-set is **derived from the two enumerable structural signals a soliciting
/// step already renders**, never a hand-copied list:
///
///  - a lone-line `{{cli.<id>}}` ref whose catalog entry is a `jigc doc <write-verb>`
///    call ([`doc_write_command_ids`], keyed on the production write-verb partition);
///  - **union** a `{{schema:<T>}}` ref — the authoring-payload projection, which is
///    how the migrate author templates solicit their whole write (they carry no
///    `{{cli.<id>}}` ref at all, so either signal alone would miss half the surface).
///
/// Scope mirrors [`assert_stated_at`]: manifest-shipping constituents, each checked
/// in isolation, with the catalog read **per origin pack** (a step's `{{cli.<id>}}`
/// resolves against its own pack's catalog). A manifest-less pack stays on
/// skip-on-absent.
///
/// Declared bounds, recorded rather than silently narrowed. **The direction is
/// solicit ⇒ declaration only**: a declarer that solicits no write is out of scope
/// here (unlike `create.singleton-copy-in`'s M47 biconditional, whose reverse arm
/// exists because deleting *its* ref deletes a promise the composed step still
/// prints; a withdrawn write solicit leaves no such dangling promise). And a step
/// that solicits its writes as **literal** command lines only — the dev pack's
/// `locate-from-spec` sets fields with hand-written `jigc doc set-field` lines and
/// no catalog ref — carries no structural signal for either arm to see, so it states
/// the read-back without joining the fenced set.
fn assert_staged_read_back_stated(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        let write_refs = doc_write_command_ids(owner);
        for id in owner.list(PackResourceKind::Steps) {
            let bytes = owner
                .read(PackResourceKind::Steps, &id)
                .with_context(|| format!("the `{}` step is unreadable", id.as_str()))?;
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .map_err(|finding| def_load_failure("step-front-matter", id.as_str(), finding))?;
            let solicits_write = cli_refs(&def.body)
                .iter()
                .any(|reference| write_refs.contains(reference))
                || !schema_refs(&def.body).is_empty();
            if !solicits_write {
                continue;
            }
            if !def
                .states_constraints
                .iter()
                .any(|code| code == STAGED_READ_BACK_CODE)
            {
                anyhow::bail!(
                    "pack-load stated-at fence failed: step `{}` solicits a managed-doc write \
                     (a `{{{{cli.<id>}}}}` ref resolving to `jigc doc <write-verb>`, or a \
                     `{{{{schema:<T>}}}}` authoring payload) but does not declare \
                     `{STAGED_READ_BACK_CODE}` in `states-constraints:` — the agent is told how \
                     to write and never how to read what it wrote, so it goes to the filesystem \
                     for its own staged work (design/surface-contract.md → The stated-at fence; \
                     design/doc-read-surface.md → the staged read)\n\
                     route: name the staged read-back above or beside the solicit — \
                     `jigc doc show <addr> --task {{{{task.id}}}}` — and declare \
                     `{STAGED_READ_BACK_CODE}` in that step's `states-constraints:` front-matter",
                    id.as_str(),
                );
            }
        }
    }
    Ok(())
}

/// The **staged read-back**'s declared identifier (M48 Inc 3 —
/// `design/surface-contract.md` → The stated-at fence, the write-solicit tier): a
/// step that solicits a managed-doc write must name the read that shows the agent
/// what it just wrote — `jigc doc show <addr> --task <id>`, which serves the task's
/// **staged** copy the committed store does not carry yet
/// (`design/doc-read-surface.md` → R7). Six consecutive trials went to the
/// filesystem to read in-flight work while that capability shipped; the mechanism
/// behind them is that no soliciting surface named it, so the obligation is fenced
/// where the write is solicited rather than patched instance by instance.
///
/// Code-side beside its assert — the obligation is jigc's, not the pack author's —
/// and **not** a minted `Finding` code: its production surface is the soliciting
/// step's own prose (the A-3 presence-only tier), so the string serves as the
/// declared identifier the fence checks for.
pub const STAGED_READ_BACK_CODE: &str = "read.staged-read-back";

/// **The named-fact map** (M47 Inc 9 — `design/surface-contract.md` → The
/// stated-at fence, named-fact tier): for each constraint code the two tiers
/// above fence, the phrase(s) the declaring step's own prose must contain for the
/// declaration to buy anything. Without it a `states-constraints:` code is a
/// receipt for a statement that need not exist — the M47 review deleted 590
/// characters of copy-in/append contract prose, kept the code, and every fence
/// stayed green.
///
/// The tokens are compared against [`normalized_body`]'s view (whitespace runs
/// collapsed, ASCII case folded), so a phrase that wraps across a newline or opens
/// a sentence capitalized still matches — **both** halves are load-bearing over the
/// shipped prose (methodology's `finalize` opens with "Unstaged"; the copy-in
/// sentence wraps in every declarer). Each token is therefore authored in
/// normalized form (lowercase, single-spaced), fenced by
/// `constraint_tokens_are_authored_in_normalized_form`.
///
/// Honest bound — this stays inside the A-3 presence-never-content rule
/// ([design/methodology-docs.md](../../../design/methodology-docs.md)): it buys the
/// *named facts* of a contract jigc itself owns, never prose quality, register, or
/// order. The set of fenced codes is jigc's, not the pack author's — a
/// pack-authored code outside it carries no token requirement, and the map is
/// bijected against the two code-side consts by
/// `constraint_token_map_bijects_with_the_fenced_codes`.
pub const CONSTRAINT_REQUIRED_TOKENS: [(&str, &[&str]); 6] = [
    (
        "finalize.promote-clobber",
        &["--approve", "retire", "fidelity diff"],
    ),
    ("finalize.left-out", &["unstaged", "untracked", "left out"]),
    ("finalize.nothing-staged", &["nothing staged", "refuse"]),
    (
        "finalize.carried-staged",
        &["--carry-staged", "this task was minted"],
    ),
    (
        SINGLETON_COPY_IN_CODE,
        &["copies the committed body in as", "edit base", "overwrites"],
    ),
    (STAGED_READ_BACK_CODE, &["jigc doc show", "--task"]),
];

/// The named-fact comparison view of a step body: every whitespace run collapsed
/// to a single space, every ASCII letter case-folded. Step prose is hard-wrapped
/// and sentence-cased, so a fact's phrase legitimately spans a line break or opens
/// capitalized; matching the raw bytes would fail on presentation, not on content.
///
/// Shared with [`crate::gate_coverage`], whose token fence runs the same comparison
/// over the same class of prose — one normalization, so a token authored for one
/// table cannot mean something else in the other.
pub(crate) fn normalized_body(body: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;
    for ch in body.chars() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }
        if pending_space && !out.is_empty() {
            out.push(' ');
        }
        pending_space = false;
        out.push(ch.to_ascii_lowercase());
    }
    out
}

/// **The stated-at fence, named-fact tier (law 3, M47 Inc 9)** —
/// `design/surface-contract.md` → The stated-at fence: a step that declares a
/// fenced constraint code must actually state that contract's named facts in its
/// own body ([`CONSTRAINT_REQUIRED_TOKENS`]), so the declaration cannot outlive the
/// statement it stands for.
///
/// Scope is the **fence family**, not one file: every declaring step of every
/// manifest-shipping constituent, in both shipped packs — the break was
/// demonstrated on a migrate step's `create.singleton-copy-in` prose, so a
/// finalize-only guard would not sweep its own axis. Every miss in a pack is
/// reported at once (step + code + token), so the author sees the whole owed set
/// rather than one bail per re-run.
///
/// Declared bounds: a **project-layer forked step** (`.jigc/config/steps/*.yaml`)
/// is outside this fence as it is outside every pack-load fence — the fences run
/// over manifest-shipping pack constituents only. A **manifest-less** pack stays on
/// skip-on-absent (the `assert_schema_freeze` opt-in precedent). And the fence
/// proves the facts are *named*, never that the surrounding prose is good.
fn assert_named_facts_stated(pack: &dyn PackSource) -> anyhow::Result<()> {
    use anyhow::Context;

    let manifest_id = ResourceId::from(SCHEMA_MANIFEST_ID);
    for owner in pack.origin_packs(PackResourceKind::Config, &manifest_id) {
        let mut missing: Vec<String> = Vec::new();
        for id in owner.list(PackResourceKind::Steps) {
            let bytes = owner
                .read(PackResourceKind::Steps, &id)
                .with_context(|| format!("the `{}` step is unreadable", id.as_str()))?;
            let def = engine::compose::load_step_def(id.as_str(), &bytes)
                .map_err(|finding| def_load_failure("step-front-matter", id.as_str(), finding))?;
            let body = normalized_body(&def.body);
            for code in &def.states_constraints {
                let Some((_, tokens)) = CONSTRAINT_REQUIRED_TOKENS
                    .iter()
                    .find(|(fenced, _)| fenced == code)
                else {
                    continue;
                };
                for token in *tokens {
                    if !body.contains(token) {
                        missing.push(format!(
                            "step `{}` declares `{code}` but its prose never says \"{token}\"",
                            id.as_str(),
                        ));
                    }
                }
            }
        }
        if !missing.is_empty() {
            anyhow::bail!(
                "pack-load named-fact fence failed: {} — the `states-constraints:` \
                 declaration would buy presence alone while the contract itself went \
                 unstated, so the constraint still first surfaces when it binds, an ambush \
                 (design/surface-contract.md → The stated-at fence)\n\
                 route: restate each named fact in that step's body, above the solicit the \
                 constraint gates — or, if the step no longer states the contract, drop its \
                 code from the step's `states-constraints:` front-matter",
                missing.join("; "),
            );
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
    // Discovered **once** and threaded to all three consumers (the `packs:` list, the
    // compose marker, and the project-layer freeze arm) — one walk, one answer.
    let project_config = discover_project_config();
    let listed = discover_pack_list(project_config.as_deref()).unwrap_or_else(|err| {
        // A malformed `packs.yaml` is a real authoring fault; surface it rather
        // than silently falling back to the base. (An *absent* file is `Ok(vec![])`
        // from `read_pack_list`, so this arm fires only on genuine corruption.)
        eprintln!("warning: {err:#}");
        Vec::new()
    });
    let compose_methodology =
        discover_compose_marker(project_config.as_deref()).unwrap_or_else(|err| {
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
    assert_schema_freeze(pack.as_ref(), project_config.as_deref())?;

    // The eager front-matter sweeps (M43, `design/surface-contract.md` → The
    // fences): the workflow sweep (suppression + catalog shape) and the step
    // sweeps — the stated-at fence's ambush-class tier ([`assert_stated_at`]),
    // its M44 per-soliciting-step tier ([`assert_singleton_copy_in_stated`]), and
    // its M48 write-solicit tier ([`assert_staged_read_back_stated`], which owes the
    // staged read-back wherever a write is solicited), and its M47 named-fact tier
    // ([`assert_named_facts_stated`], which buys the declared contract's own facts
    // rather than the declaration alone).
    // Memoized for the two embedded compositions
    // (their bytes cannot change within a process; `make_pack` has ~38 call
    // sites), recomputed whenever a filesystem pack is in the set (its tree is
    // live-mutable). `anyhow::Error` is not `Clone`, so the cache carries the
    // rendered message.
    if embedded_only {
        static EMBEDDED_SWEEPS: [std::sync::OnceLock<Result<(), String>>; 2] =
            [std::sync::OnceLock::new(), std::sync::OnceLock::new()];
        EMBEDDED_SWEEPS[usize::from(compose_methodology)]
            .get_or_init(|| {
                assert_workflow_front_matter(pack.as_ref())
                    .and_then(|()| assert_stated_at(pack.as_ref()))
                    .and_then(|()| assert_singleton_copy_in_stated(pack.as_ref()))
                    .and_then(|()| assert_staged_read_back_stated(pack.as_ref()))
                    .and_then(|()| assert_named_facts_stated(pack.as_ref()))
                    .map_err(|err| format!("{err:#}"))
            })
            .clone()
            .map_err(|msg| anyhow::anyhow!(msg))?;
    } else {
        assert_workflow_front_matter(pack.as_ref())?;
        assert_stated_at(pack.as_ref())?;
        assert_singleton_copy_in_stated(pack.as_ref())?;
        assert_staged_read_back_stated(pack.as_ref())?;
        assert_named_facts_stated(pack.as_ref())?;
    }
    Ok(pack)
}

/// CWD-discover the project's `.jigc/config/` layer: walk up from the process CWD to
/// the repo root (the dir holding `.git`) and return `<root>/.jigc/config` when it is
/// a directory. No repo / no `.jigc/config/` is `None` — the cold-start floor, never an
/// error. The **single** walk behind [`make_pack`]'s three project-layer reads (the
/// `packs:` list, the compose marker, and the freeze gate's project arm).
fn discover_project_config() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let repo_root = cwd.ancestors().find(|dir| dir.join(".git").exists())?;
    let project_config = repo_root.join(".jigc").join("config");
    project_config.is_dir().then_some(project_config)
}

/// The project's pre-cascade pack-set, read from the discovered project layer's
/// `packs.yaml` via [`read_pack_list`]. No project layer is the empty pack-set
/// (`Ok(vec![])`) — the cold-start floor — not an error; only a malformed
/// `packs.yaml` is an `Err`.
fn discover_pack_list(project_config: Option<&Path>) -> anyhow::Result<Vec<PathBuf>> {
    let Some(project_config) = project_config else {
        return Ok(Vec::new());
    };
    read_pack_list(project_config)
}

/// The project's **compose marker** — the same discovered `packs.yaml` as
/// [`discover_pack_list`], reading the `compose-embedded-methodology` key via
/// [`read_compose_marker`]. No project layer is no marker (`Ok(false)`) — the
/// single-pack floor — not an error; only a malformed `packs.yaml` is an `Err`.
fn discover_compose_marker(project_config: Option<&Path>) -> anyhow::Result<bool> {
    let Some(project_config) = project_config else {
        return Ok(false);
    };
    read_compose_marker(project_config)
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
/// **Marker set *and* a non-empty `packs:` list composes, at a declared precedence**
/// (M49 Increment 6; `design/multi-pack.md` → Embedded second pack). The assembly is
/// `[listed… ▸ dev ▸ methodology]` under one demotion ([`FreezeDemotion`]): for a
/// `Schemas` id either embedded manifest declares frozen, the listed packs sort
/// **below** the pair — *a project pack may not shadow a doctype the freeze governs*.
/// Everything outside that id-space keeps the ordinary listed-highest convention, so a
/// house doctype is a genuine extension.
///
/// **The two rejected alternatives, and why.** Plain `[listed ▸ dev ▸ methodology]`
/// would let a **manifest-less** listed pack shadow a frozen doctype while
/// [`assert_schema_freeze`] is skip-on-absent for it — the freeze invariant falsified
/// from the project layer, at exit 0. The prior **loud refusal** (M42 Inc 6, which
/// itself replaced a *silent* drop of the whole listed set) was safe but left an
/// adopter no way to add a house doctype at all: `jigc setup` writes the marker into
/// every project, so declaring one project pack bricked every door.
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
        let dev = EmbeddedPack::new();
        let methodology = EmbeddedPack::methodology();
        // The governed id-space is READ from the two manifests at assembly, never
        // hand-listed: a doctype added to or removed from either manifest moves the
        // demotion boundary with it, in one place.
        let governed = governed_doctype_ids(&[&dev, &methodology]);
        let mut packs: Vec<Box<dyn PackSource>> = listed_dirs
            .into_iter()
            .map(|dir| Box::new(FilesystemPack::new(dir)) as Box<dyn PackSource>)
            .collect();
        // `[listed… ▸ dev ▸ methodology]`; dev before methodology = dev-highest.
        let embedded_from = packs.len();
        packs.push(Box::new(dev));
        packs.push(Box::new(methodology));
        return Ok(Box::new(CompositePack::with_freeze_demotion(
            packs,
            embedded_from,
            governed,
        )));
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
/// winner's whole file. Precedence for a given `(kind, id)` comes from the single
/// [`ordered`](CompositePack::ordered) function, which is assembly order except under
/// the [`FreezeDemotion`] the marker-plus-a-listed-pack composition carries. `list` is the **union deduped-by-id then sorted** — the
/// same stable sorted-by-id contract a single pack's `list()` already honours, so
/// the `[base]` floor and any union both emit ids in deterministic order
/// (no `HashSet` iteration order reaches output). `pack_version` is the
/// highest-precedence pack's. This task does **not** pack-localize body-references
/// (a *loser*-pack workflow's `{{include: step:X}}` is still mis-resolved here —
/// fixed in increment 2). See `design/multi-pack.md` → Collision resolution;
/// Where it sits.
pub struct CompositePack {
    /// The pack-set in **assembly order**, highest-precedence first.
    packs: Vec<Box<dyn PackSource>>,
    /// The per-id **demotion rule**, or `None` for a plain precedence composite.
    demotion: Option<FreezeDemotion>,
}

/// The **freeze demotion**: for a `Schemas` id the embedded pair declares frozen, the
/// packs *before* `embedded_from` (the project's listed packs) sort **below** the pair
/// — *a project pack may not shadow a doctype the freeze governs*
/// (`design/multi-pack.md` → Embedded second pack; the M49 settle record → D5).
///
/// **Why the rule exists.** The natural assembly `[listed ▸ dev ▸ methodology]` lets a
/// **manifest-less** listed pack shadow a frozen doctype while
/// [`assert_schema_freeze`] is skip-on-absent for exactly that pack — so the freeze
/// invariant would be falsifiable from the layer this composition opens, silently and
/// at exit 0. Demoting the listed packs for the governed id-space closes it without
/// closing the *extension* case: an id no manifest declares (a house `note`) still
/// resolves listed-first.
///
/// **Why it lives inside the composite.** One ordering function
/// ([`CompositePack::ordered`]) feeds `read`, `origin_pack` and `origin_packs`, so the
/// freeze gate, `doc schema`, the store sweep and `--explain` cannot disagree about who
/// won (M46 Increment 3's one-producer rule). A demotion applied at a call site would
/// give each caller its own answer.
struct FreezeDemotion {
    /// Index of the first embedded pack in [`CompositePack::packs`] — everything before
    /// it is a project-listed pack subject to the demotion. `0` makes the rule a no-op
    /// (no listed packs), which is the marker-alone byte-identity floor.
    embedded_from: usize,
    /// Every doctype id declared in either embedded pack's own
    /// `config/schema-manifest.yaml`, read at assembly — **never hand-listed**, so a
    /// doctype added to (or removed from) a manifest moves the boundary with it.
    governed: std::collections::BTreeSet<String>,
}

impl CompositePack {
    /// Assemble a composite over `packs`, **highest-precedence first**, with no
    /// demotion. A single-element `Vec` is the byte-identity floor: its `list`/`read`/
    /// `pack_version` equal that one pack's.
    ///
    /// Wired into [`make_pack`]'s pack-set assembly (the production caller) and
    /// exercised directly by the unit tests.
    pub fn new(packs: Vec<Box<dyn PackSource>>) -> Self {
        CompositePack {
            packs,
            demotion: None,
        }
    }

    /// Assemble `[listed… ▸ dev ▸ methodology]` under the [`FreezeDemotion`] — the
    /// marker-plus-a-listed-pack composition (`make_pack_from_marker`). `embedded_from`
    /// is where the embedded pair starts; `governed` is the union of the two embedded
    /// manifests' declared doctypes ([`governed_doctype_ids`]).
    fn with_freeze_demotion(
        packs: Vec<Box<dyn PackSource>>,
        embedded_from: usize,
        governed: std::collections::BTreeSet<String>,
    ) -> Self {
        CompositePack {
            packs,
            demotion: Some(FreezeDemotion {
                embedded_from,
                governed,
            }),
        }
    }

    /// **The one ordering function.** The pack-set in the precedence order that applies
    /// to *this* `(kind, id)`: assembly order, except that a `Schemas` id the embedded
    /// pair's manifests govern puts the embedded pair first and the listed packs after
    /// it. `read`, `origin_pack` and `origin_packs` all read precedence from here, so no
    /// two surfaces can disagree about the winner.
    fn ordered(&self, kind: PackResourceKind, id: &ResourceId) -> Vec<&dyn PackSource> {
        let demoted = self
            .demotion
            .as_ref()
            .filter(|d| kind == PackResourceKind::Schemas && d.governed.contains(id.as_str()));
        match demoted {
            Some(d) => self.packs[d.embedded_from..]
                .iter()
                .chain(self.packs[..d.embedded_from].iter())
                .map(Box::as_ref)
                .collect(),
            None => self.packs.iter().map(Box::as_ref).collect(),
        }
    }
}

impl PackSource for CompositePack {
    /// The highest-precedence (first) pack's version. An empty pack-set cannot
    /// arise in production (the base is always present), but the empty-`Vec`
    /// version is the empty string rather than a panic.
    fn pack_version(&self) -> String {
        self.packs
            .first()
            .map(|p| p.pack_version())
            .unwrap_or_default()
    }

    /// The union of every pack's ids for `kind`, **deduped-by-id then sorted**.
    /// A `BTreeSet` keyed by the stable [`ResourceId`] gives both at once — never
    /// a `HashSet`, whose iteration order would leak into the emitted list
    /// (increment-workflow hardening #7).
    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let ids: std::collections::BTreeSet<ResourceId> =
            self.packs.iter().flat_map(|p| p.list(kind)).collect();
        ids.into_iter().collect()
    }

    /// The **precedence-winner's** bytes: the first pack in this `(kind, id)`'s
    /// [`ordered`](CompositePack::ordered) precedence whose `read` succeeds. If no pack owns the id, a clean
    /// [`PackError::NotFound`] naming the requested `kind`/`id` — never the last
    /// pack's own error instance.
    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        self.ordered(kind, id)
            .into_iter()
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
        self.packs
            .iter()
            .filter(|p| p.read(kind, id).is_ok())
            .count()
    }

    /// The constituent pack that **owns** `(kind, id)` — the **first** pack in this
    /// `(kind, id)`'s [`ordered`](CompositePack::ordered) precedence whose
    /// [`read`](PackSource::read) succeeds, the same pack the precedence `read`
    /// selects (one ordering function, so they cannot disagree). This is the body-reference
    /// resolution anchor a composed definition resolves its `{{include: step:X}}`
    /// / `{{cli.X}}` / field-type names against (`design/multi-pack.md` →
    /// Pack-local body-reference resolution), so a *loser*-pack workflow composes
    /// **its own** steps/catalog rather than the precedence-winner's divergent
    /// ones. When no constituent owns the id the composite returns `self`, so a
    /// dangling reference still flows to the existing not-found path (a clean
    /// [`PackError::NotFound`] from `read`), never a panic. A single-element
    /// composite returns that one pack — origin = the pack (the floor).
    fn origin_pack(&self, kind: PackResourceKind, id: &ResourceId) -> &dyn PackSource {
        self.ordered(kind, id)
            .into_iter()
            .find(|p| p.read(kind, id).is_ok())
            .unwrap_or(self)
    }

    /// **Every** constituent that ships `(kind, id)`, in this id's
    /// [`ordered`](CompositePack::ordered) precedence (winner first) —
    /// the plural sibling of [`origin_pack`](PackSource::origin_pack): a resource
    /// shadowed by a higher-precedence pack is still enumerated. Delegates to each
    /// constituent's own `origin_packs` (a nested composite flattens), so the walk
    /// reaches every genuine owner. The freeze assertion
    /// ([`assert_schema_freeze`](crate::pack::assert_schema_freeze)) reads this to
    /// enforce every manifest-shipping constituent, never just the Config-resource
    /// precedence winner. No owner is the empty `Vec` — there is nothing to walk.
    fn origin_packs(&self, kind: PackResourceKind, id: &ResourceId) -> Vec<&dyn PackSource> {
        self.ordered(kind, id)
            .into_iter()
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
        self.packs
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
        self.packs
            .iter()
            .flat_map(|p| p.provenance_entries())
            .collect()
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
        insta::assert_snapshot!(body, @"
        ---
        when: implement one scoped change end-to-end, recording its decisions as ADRs and user-facing effects on the changelog
        description: An end-to-end scoped change — locate, implement, optionally record a decision and a changelog entry, and commit, all as one task.
        usage: the work is one coherent change you can hold in your head and carry from intent to commit in a single pass. Also the pick when the change touches documented code (a symbol a managed doc names, renamed or reshaped) — its doc gates cover the update, where quick-fix's commit-only path does not. It grants the changelog gate alongside the ADR one, so a user-facing change is recorded as it lands; when the change is internal, author no entry — finalize notes the unused gate as an advisory and commits anyway.
        creates-task: true
        allows-create: [{type: adr, as: decision}, {type: changelog, as: change}]
        ---
        {{ include: step:locate }}
        {{ include: step:implement }}
        {{ include: step:record-changelog }}
        {{ include: step:superseded-context }}
        {{ include: step:author-commit }}
        {{ include: step:finalize }}
        ");
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
            17,
            "the shipped pack must carry all 17 workflows; got {workflows:?}",
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
        {{ include: step:author-commit }}
        {{ include: step:finalize }}
        "###);
    }

    /// list(Steps) yields the MVP step ids, sorted (the pack lists in stem
    /// order). The composer's includes resolve against exactly these — `author-adr`
    /// (the M44 `record-decision` workflow's from-knowledge, create-gated
    /// adr-authoring step — the `context`/`decision`/`consequences` slots, no foreign
    /// source and the on-create date stamp kept, sorting first), the four
    /// `single-task` steps, `implement-quick` (the ADR-free variant `quick-fix`
    /// includes), the router's `present-catalog` / `route-to-workflow`,
    /// `author-spec` (the `plan` workflow's create-gated spec-authoring step),
    /// `locate-from-spec` (the `implement-from-spec` workflow's spec-driven locate,
    /// distinct from the shared `locate`), `author-commit` (the M47 pack-wide
    /// commit-doc solicit — the **one** body every task-minting non-migrate
    /// workflow composes before its finalize include) and `sub-task-commit` (the
    /// fanned `sub-task` workflow's boundary framing stated *above* an include of
    /// that shared solicit, so the double-instruct is structurally impossible),
    /// plus the
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
    /// `implemented-by` code anchor, plus the bracketed in-store `cites` edge), plus
    /// `migration-finalize` (the M43 migration-finalize solicit — the
    /// `--approve`/clobber/retire contract stated above its `step:finalize` include,
    /// `states-constraints`-declared for the stated-at fence).
    #[test]
    fn embedded_pack_lists_the_mvp_steps() {
        let pack = EmbeddedPack::new();
        let steps = pack.list(PackResourceKind::Steps);
        assert_eq!(
            steps,
            vec![
                ResourceId::from("author-adr"),
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
                ResourceId::from("migration-finalize"),
                ResourceId::from("milestone-finalize"),
                ResourceId::from("present-catalog"),
                ResourceId::from("project-finalize"),
                ResourceId::from("provision-worktrees"),
                ResourceId::from("record-changelog"),
                ResourceId::from("review-verdicts"),
                ResourceId::from("route-to-workflow"),
                ResourceId::from("run-scan"),
                ResourceId::from("sub-task-commit"),
                ResourceId::from("superseded-context"),
            ],
        );
    }

    /// (M44 Inc 6 T1) Every migrate author step that solicits a **singleton**
    /// doctype's authoring — its body carries a `{{schema:<T>}}` ref with `T`
    /// declared `singleton: true` — states the copy-in/append constraint and
    /// declares its code (`create.singleton-copy-in`) in `states-constraints:`
    /// front-matter (the D5 path-local-guidance owe-set; `design/surface-contract.md`
    /// → The stated-at fence). Checked over both shipped packs: the dev-only
    /// embedded surface (owes `changelog`) and the composed `[dev ▸ methodology]`
    /// surface (owes `changelog` + `roadmap`/`decisions-log`/`deferral-ledger`/
    /// `vision`). The enforcing pack-load fence lands in T2, so this change adds
    /// no fence and both shipped surfaces still load unchanged.
    #[test]
    fn singleton_migrate_author_steps_declare_the_copy_in_constraint() {
        /// Step ids whose body solicits a singleton doctype yet omit the copy-in code.
        fn offenders(pack: &dyn PackSource) -> Vec<String> {
            let mut missing = Vec::new();
            for id in pack.list(PackResourceKind::Steps) {
                let bytes = pack
                    .read(PackResourceKind::Steps, &id)
                    .expect("step is readable");
                let def = engine::compose::load_step_def(id.as_str(), &bytes)
                    .expect("step front-matter parses");
                let solicits_singleton = schema_refs(&def.body).into_iter().any(|ty| {
                    pack.read(PackResourceKind::Schemas, &ResourceId::from(ty.as_str()))
                        .ok()
                        .and_then(|b| load_pack_schema(pack, &b).ok())
                        .is_some_and(|s| s.singleton)
                });
                if solicits_singleton
                    && !def
                        .states_constraints
                        .iter()
                        .any(|c| c == SINGLETON_COPY_IN_CODE)
                {
                    missing.push(id.as_str().to_string());
                }
            }
            missing
        }

        let dev = EmbeddedPack::new();
        assert_eq!(offenders(&dev), Vec::<String>::new(), "dev pack");

        let composed = CompositePack::new(vec![
            Box::new(EmbeddedPack::new()),
            Box::new(EmbeddedPack::methodology()),
        ]);
        assert_eq!(
            offenders(&composed),
            Vec::<String>::new(),
            "composed [dev ▸ methodology]",
        );

        // T1 adds no fence (that is T2), so the shipped embedded surface still loads.
        make_pack().expect("the embedded pack loads");
    }

    /// (M47 Inc 9 T1) The named-fact map covers **exactly** the codes jigc fences
    /// — [`AMBUSH_CLASS_CODES`] plus [`SINGLETON_COPY_IN_CODE`] and (M48 Inc 3)
    /// [`STAGED_READ_BACK_CODE`] — iterated from the
    /// consts, both directions. A fenced code with no token requirement would be
    /// back to buying presence alone; a token requirement on a code jigc does not
    /// fence would put jigc's prose demands on a pack author's own vocabulary
    /// (the declared bound in `design/surface-contract.md` → The stated-at fence).
    #[test]
    fn constraint_token_map_bijects_with_the_fenced_codes() {
        let mapped: std::collections::BTreeSet<&str> = CONSTRAINT_REQUIRED_TOKENS
            .iter()
            .map(|(code, _)| *code)
            .collect();
        let fenced: std::collections::BTreeSet<&str> = AMBUSH_CLASS_CODES
            .into_iter()
            .chain([SINGLETON_COPY_IN_CODE, STAGED_READ_BACK_CODE])
            .collect();
        assert_eq!(mapped, fenced);
        assert_eq!(
            mapped.len(),
            CONSTRAINT_REQUIRED_TOKENS.len(),
            "no code may appear twice in the map",
        );
    }

    /// (M47 Inc 9 T1) Every required token is authored in [`normalized_body`]'s own
    /// form — lowercase, single-spaced, untrimmed-free. A token carrying a capital
    /// or a double space could never match a normalized body, so the fence would
    /// redden the shipped packs (or, worse, be silently unsatisfiable for a new one).
    /// (M47 Inc 9 T2) The conditional [`COPY_IN_APPEND_TOKENS`] tier matches over the
    /// same normalized view, so it is held to the same rule.
    #[test]
    fn constraint_tokens_are_authored_in_normalized_form() {
        let flat = CONSTRAINT_REQUIRED_TOKENS
            .iter()
            .flat_map(|(code, tokens)| tokens.iter().map(move |token| (*code, *token)));
        let conditional = COPY_IN_APPEND_TOKENS
            .iter()
            .map(|token| (SINGLETON_COPY_IN_CODE, *token));
        for (code, token) in flat.chain(conditional) {
            assert_eq!(
                normalized_body(token),
                token,
                "`{code}`'s token {token:?} is not in normalized form",
            );
        }
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
        ---
        states-constraints: [read.staged-read-back]
        ---
        Implement the change directly in the working tree. `git add` your code edits
        before finalize — it commits only what you have staged.

        If a decision is warranted, create an ADR and author its slots — a line per slot
        usually suffices; an ADR earns its keep by capturing the *why*, not by running
        long:

        {{ cli.create-adr }}

        Author its three required slots on the address `create` prints — `context` (the
        forces at play), `decision` (the call itself), `consequences` (tradeoffs and
        follow-on effects). Inside slot prose, the reserved heading depths are schema-relative to
        the address you write — the CLI owns the section, item, and sub-label heading
        levels there, so your headings sit below them; Setext headings are rejected at
        every depth, and a rejected write names the shallowest depth free at that
        address:

        jigc doc set-slot adr:<slug>#context --from-file - --task {{task.id}}
        jigc doc set-slot adr:<slug>#decision --from-file - --task {{task.id}}
        jigc doc set-slot adr:<slug>#consequences --from-file - --task {{task.id}}

        The `options` slot is optional — fill it only when alternatives were genuinely
        weighed. Its `## Options` heading renders either way; an empty optional slot is
        conformant and never blocks finalize:

        jigc doc set-slot adr:<slug>#options --from-file - --task {{task.id}}

        Before you finalize, verify the change actually works: build it and run the
        tests, and confirm the behaviour you set out to produce. Finalize commits your
        staged work; it does not check that the work is correct.

        Read your write back before you move on — with `--task` the read serves THIS
        task's staged copy, the write you just made, which the committed store does not
        carry yet:

        jigc doc show adr:<slug> --task {{task.id}}

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
        ---
        states-constraints: [finalize.left-out, finalize.nothing-staged, finalize.carried-staged]
        ---
        Validate and commit the task as one logical commit. Finalize commits only the
        staged set plus the docs it manages; unstaged edits and untracked files are left
        out, and with nothing staged over a dirty tree it refuses. Anything still staged
        from BEFORE this task was minted makes finalize refuse too (one blocking finding
        per carried path): unstage it, or pass `--carry-staged` to declare the carryover
        deliberate.

        To see what's left before committing, run `jigc task validate {{task.id}}` — it
        previews part of what finalize gates on (this task's content findings, the
        carryover gate, the owner-artifact causes that need no staging, and the
        granted-but-unused changelog gate), without committing anything; the staged set,
        promotion and the commit itself are decided at finalize.

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

        # --- placement-root — the parent dir of every NON-root placement home (M49) ---
        # The asymmetry this closes: a `location:` doctype's home resolves through `docs-root`,
        # a `placement:` doctype's home resolves through NOTHING (it is the literal
        # `placement.file` its schema declares), and since M38 that literal sits inside the
        # doctype's frozen `schema-hash`, so `jigc relocate` refuses to move it. An adopter whose
        # managed docs do not live under `docs/` therefore had no path at all to `docs/roadmap.md`
        # — not a knob, not a verb (storage.md → Placement; DECISIONS.md → the M49 Settle, D6).
        # THE RULE, which is also the scope answer: a declared home carrying a LEADING DIRECTORY
        # COMPONENT (`docs/roadmap.md`) resolves to `<placement-root>/<remainder>`; a home
        # declared AT the repo root (`VISION.md`, `CHANGELOG.md`) is NEVER re-rooted. The
        # ecosystem-idiomatic files stay unburiable by DERIVATION, so there is no doctype
        # allow-list here to go stale.
        # Default `""` = UNSET: every declared home stands, byte-identical to no knob at all.
        # `.` means the repo root (`docs/roadmap.md` → `roadmap.md`); `jigc config set
        # placement-root ""` canonicalizes to `.`, the same spelling `docs-root` accepts. A
        # tunable knob (no floor).
        # Declared IDENTICALLY in BOTH shipped packs — never a recorded divergence: a knob present
        # in only one file is invisible under the whole-file `knobs.yaml` shadow, so the resolved
        # home would vary by which pack won, and composition-invariance is precisely what the
        # placement design exists to protect. Pinned by cli::pack
        # placement_root_is_declared_in_both_packs_identically.
        placement-root:
          type: string
          default: ""

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
        # zero items IS the steady state (the staging area, a project before its first
        # cut release, a valid just-created state, a clean audit) — a matching token
        # suppresses the finding entirely. `changelog#releases` joined at M47: a young
        # corpus (unreleased changes staged, no release cut yet) is CORRECT, not hollow,
        # so the advisory fired on every greenfield project with no action behind it —
        # and an advisory with no action behind it is what teaches a reader to ignore
        # the channel (surface-contract.md → law 1).
        validation.schema-conformance.repeatable-populated.severity:
          type: enum
          of: [blocking, warning, advisory]
          default: advisory
        validation.schema-conformance.repeatable-populated.exempt:
          type: string
          default: "changelog#unreleased-changes changelog#releases milestone-record#tasks completion-record#findings"

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
    /// pack now ships its **own** `config/schema-manifest.yaml` listing all **eleven**
    /// shipped schemas — the ten persisted work-doc/design-altitude doctypes plus
    /// the transient `commit` shadow (the freeze assert is strict set-equality; the
    /// dev precedent lists its transient `commit`) — every one frozen at
    /// schema-version 1 (the v1 baseline) except the three bumped since:
    /// `deferral-ledger` (M41 F4, the `kind` rename), `completion-record` (M49 Inc-9, the
    /// audit vocabulary + the `detail` slot) — each at **2** — and `milestone-record`,
    /// bumped twice (M42 Inc-7, the `discarded` lifecycle member; M49 Inc-9, the
    /// per-sub-task `workflow` leaf) and therefore at **3**, shipping the snapshot store's
    /// first **two**-snapshot chain (`.v1.yaml` beside `.v2.yaml`) where the other two ship
    /// one `schema-snapshots/<ty>.v1.yaml` each — with every recomputed hash matching. The
    /// eleventh member, `planning-record` (M49 Inc-9 / T5), is the **new**-doctype shape of
    /// this gate: it joins the declared set at schema-version 1 and owes no snapshot at
    /// all. A methodology schema-shape change that bumps no version fails **here,
    /// loudly**, at build time.
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

        // The frozen set is exactly the eleven shipped methodology schemas, each at
        // schema-version 1 (the crystallizing v1 baseline) except three: `deferral-ledger`,
        // bumped to 2 by the M41 F4 v1→v2 `kind` enum-member rename (D/I → Decision/Idea
        // — the first methodology v1→v2 migration; `design/corpus-migration.md` → the
        // structural-auto / value-semantic-authored distinction), `milestone-record`,
        // bumped to 3 by two bumps (M42 Inc-7's lifecycle widening — `status` gains
        // `discarded` at both loci, an `EnumWidened` pair; then M49 Inc-9's durability bump
        // — a per-sub-task `workflow` leaf joins the `tasks` item block, an
        // `AddedItemField` for a `default`-less machine-maintained leaf, so the recorded
        // workflow becomes fresh-clone durable; `design/team-ready-state.md` → The
        // lifecycle / the committed record), and `completion-record`, bumped to 2 by the
        // M49 Inc-9 audit-vocabulary bump (`severity` widened to the superset
        // `[blocking, advisory, HIGH, MEDIUM, LOW]` plus an optional `detail` prose slot on
        // the findings item block — an `EnumWidened` + `AddedItemSlot` pair, both byte
        // no-ops; `completions/artifacts/M49/settle-record.md` → D10 · Tier 3).
        // `planning-record` (M49 Inc-9 / T5) joins the set NEW, at 1 — free at the freeze.
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
                "planning-record",
                "research",
                "roadmap",
                "vision",
            ],
            "the methodology freeze manifest must enumerate exactly the eleven shipped schemas",
        );
        for entry in &manifest.doctypes {
            let expected = match entry.ty.as_str() {
                "completion-record" | "deferral-ledger" => 2,
                "milestone-record" => 3,
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
    /// doctypes (the `roadmap`/`decisions-log`/`deferral-ledger` singletons and the
    /// per-milestone `planning-record`, which thereby gain a `---` block on mint) —
    /// and stays **inert** for the transient `commit` shadow
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
            "planning-record",
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
                    engine::tempname::unique_nanos(),
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
                    engine::tempname::unique_nanos(),
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
                    engine::tempname::unique_nanos(),
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

        /// **The marker composes WITH a listed pack, and the demotion holds for every
        /// governed id** (M49 Inc 6 / T1). This replaces the M42 refusal
        /// (`marker + packs:` was an error, which itself replaced a silent drop of the
        /// whole listed set): the marker is written into *every* `jigc setup` project,
        /// so refusing the combination left an adopter no way to add a house doctype at
        /// all. The composition is now `[listed… ▸ dev ▸ methodology]` with the freeze
        /// demotion, and this asserts the demotion over the **whole governed set** —
        /// read from the two embedded manifests, never hand-listed — rather than over
        /// one instance: a listed pack shipping a divergent schema for *every* governed
        /// doctype wins **none** of them, while the one id no manifest governs still
        /// resolves listed-first (the extension case the whole task exists for).
        #[test]
        fn a_listed_pack_shadows_no_governed_doctype_but_still_extends() {
            let dev = EmbeddedPack::new();
            let methodology = EmbeddedPack::methodology();
            let governed = governed_doctype_ids(&[&dev, &methodology]);
            assert!(
                governed.contains("commit") && governed.contains("idea"),
                "the governed set must span both embedded manifests; got: {governed:?}",
            );

            // A house pack that tries to shadow EVERY governed doctype, plus one
            // doctype no manifest governs (the genuine extension).
            const UNGOVERNED: &str = "house-note";
            let listed = TempDir::new();
            let schemas = listed.path().join("schemas");
            std::fs::create_dir_all(&schemas).expect("mk schemas/");
            let house_bytes = |ty: &str| format!("type: {ty}\n# the house shadow\n").into_bytes();
            for ty in governed.iter().map(String::as_str).chain([UNGOVERNED]) {
                std::fs::write(schemas.join(format!("{ty}.yaml")), house_bytes(ty))
                    .expect("seed the house schema");
            }

            let pack = make_pack_from_marker(None, vec![listed.path().to_path_buf()], true)
                .expect("marker + a listed pack composes");

            // Every governed id resolves to an EMBEDDED pack's bytes, at both the
            // `read` and the `origin_pack` seam — one ordering function feeds both, so
            // the freeze gate and every read surface agree on the winner.
            for ty in &governed {
                let id = ResourceId::from(ty.as_str());
                let bytes = pack
                    .read(PackResourceKind::Schemas, &id)
                    .expect("a governed doctype reads through the composite");
                assert_ne!(
                    bytes,
                    house_bytes(ty),
                    "`{ty}` is manifest-governed, so the listed pack must not win it",
                );
                assert_eq!(
                    pack.origin_pack(PackResourceKind::Schemas, &id)
                        .resolving_path(),
                    engine::packsource::EMBEDDED_PATH,
                    "`{ty}`'s origin pack must be an embedded pack, not the listed one",
                );
                // The shadowed loser is still enumerated, embedded-first — the freeze
                // walk and `--explain` read this plural view.
                let owners: Vec<String> = pack
                    .origin_packs(PackResourceKind::Schemas, &id)
                    .iter()
                    .map(|p| p.resolving_path())
                    .collect();
                assert_eq!(
                    owners.first().map(String::as_str),
                    Some(engine::packsource::EMBEDDED_PATH),
                    "`{ty}`'s enumeration must lead with the winner (embedded); got: {owners:?}",
                );
                assert_eq!(
                    owners.last(),
                    Some(&listed.path().display().to_string()),
                    "`{ty}`'s shadowed listed owner must still be enumerated, demoted to \
                     last; got: {owners:?}",
                );
            }

            // The extension case: an id no manifest governs keeps the ordinary
            // listed-highest convention.
            let id = ResourceId::from(UNGOVERNED);
            assert_eq!(
                pack.read(PackResourceKind::Schemas, &id)
                    .expect("the house doctype reads through the composite"),
                house_bytes(UNGOVERNED),
                "an ungoverned doctype must resolve to the listed pack — extension is \
                 the capability the refusal denied",
            );
            assert_eq!(
                pack.origin_pack(PackResourceKind::Schemas, &id)
                    .resolving_path(),
                listed.path().display().to_string(),
                "the ungoverned doctype's origin pack is the listed directory",
            );
        }

        /// The demotion is scoped to the **`Schemas`** id-space: a listed pack still
        /// wins every other kind under the marker (here a workflow, the id-space
        /// `multi-pack.md` calls precedence-override's home). Without this the
        /// "extension" the task ships would be schema-only.
        #[test]
        fn the_freeze_demotion_does_not_reach_other_id_spaces() {
            let listed = TempDir::new();
            let wf = listed.path().join("workflows");
            std::fs::create_dir_all(&wf).expect("mk workflows/");
            std::fs::write(wf.join("house-only.yaml"), b"when: from the house pack\n")
                .expect("seed the house workflow");

            let pack = make_pack_from_marker(None, vec![listed.path().to_path_buf()], true)
                .expect("marker + a listed pack composes");

            assert_eq!(
                pack.read(PackResourceKind::Workflows, &ResourceId::from("house-only"))
                    .expect("the house workflow reads through the composite"),
                b"when: from the house pack\n",
                "a listed pack's workflow must compose under the marker",
            );
        }

        /// Under an explicit `JIGC_PACK_DIR` the marker is inert
        /// ([`pack_dir_supersedes_the_marker`]), so listed packs compose over that base
        /// on the plain M14 path — no embedded pair, and therefore no freeze demotion:
        /// the `JIGC_PACK_DIR` channel is byte-for-byte the pre-marker composition.
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
                    engine::tempname::unique_nanos(),
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
                assert_schema_freeze(&clean, None)
                    .expect("a composite whose every manifest holds composes clean");

                let drifted = two_manifest_composite("0".repeat(64));
                let err = assert_schema_freeze(&drifted, None)
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
                    engine::tempname::unique_nanos(),
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

        /// (M49 Increment 7 / T1) **`placement-root` is declared in BOTH shipped packs, with
        /// an identical declaration** — the knob that gives a `placement:` doctype's home
        /// the project override a `location:` home has always had through `docs-root`
        /// (`design/storage.md` → Placement; `DECISIONS.md` → the M49 Settle, D6).
        ///
        /// Mirrored rather than dev-only **because the property the placement design exists
        /// to protect is composition-invariance**: `knobs.yaml` collides by whole-file shadow
        /// (`multi-pack.md` → Per-kind collision behavior), so a key declared in one pack
        /// alone is *invisible* whenever the other wins — and a placement home that stands in
        /// one composition and re-roots in another is exactly the variance the literal
        /// `placement.file` was introduced to end. `docs-root`'s dev-only divergence is not a
        /// precedent here: it re-points `location:` homes, and the methodology pack
        /// deliberately has none nested.
        ///
        /// The parity guard above would catch a *missing* key, but only as one entry in a
        /// list; this arm states the mint and its default so the reason survives the diff.
        #[test]
        fn placement_root_is_declared_in_both_packs_identically() {
            const KEY: &str = "placement-root";

            let load = |which: &str, pack: &EmbeddedPack| {
                engine::knobs::load_knobs(
                    &pack
                        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
                        .unwrap_or_else(|e| panic!("{which} knobs.yaml reads back: {e}")),
                )
                .unwrap_or_else(|e| panic!("{which} knobs.yaml loads: {e}"))
            };
            let dev = load("dev", &EmbeddedPack::new());
            let methodology = load("methodology", &EmbeddedPack::methodology());

            for (which, knobs) in [("dev", &dev), ("methodology", &methodology)] {
                let field = knobs.field(KEY).unwrap_or_else(|| {
                    panic!(
                        "{which}: `{KEY}` must be declared — a pack that omits it resolves the \
                         knob to nothing, so its placement homes silently ignore the override"
                    )
                });
                assert!(
                    matches!(field.ty, engine::schema::FieldType::String),
                    "{which}: `{KEY}` is a path fragment — a string knob",
                );
                assert_eq!(
                    knobs.base_scalars().get(KEY).map(String::as_str),
                    Some(""),
                    "{which}: the default is the UNSET sentinel — every declared \
                     `placement.file` stands, byte-identical to a build with no knob (the \
                     repo root is spelled `.`)",
                );
                assert_eq!(
                    knobs.floors().get(KEY),
                    None,
                    "{which}: a layout knob carries no demotion-lock floor",
                );
            }

            assert_eq!(
                dev.field(KEY),
                methodology.field(KEY),
                "`{KEY}` must be declared IDENTICALLY in both packs — under the whole-file \
                 `knobs.yaml` shadow a divergent declaration makes the resolved placement \
                 home depend on which pack won, which is the composition-variance the \
                 placement design forbids",
            );
        }
    }
}
