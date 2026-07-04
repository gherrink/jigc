//! `jigc migrate-corpus` — the managed-corpus schema-migration verb (M34 Inc-3 T4).
//!
//! The **corpus-side** of the productive-readiness pair: a committed managed corpus at an
//! old schema version is detected (the Inc-1 fifth `validate` family + Inc-3 T3
//! version-aware route), and this verb **migrates** it onto the current schema —
//! byte-stable, per-doc gated, the stamp flipped last — consuming the engine
//! [`engine::transform::migrate_corpus`] fold (`design/corpus-migration.md` → The
//! deterministic transform / Acceptance flows).
//!
//! Disjoint from `jigc migrate` (foreign-doc *adoption*, LLM re-author) and `jigc upgrade`
//! (config-delta reconciliation): this is a managed *structural* schema-version upgrade (v0→v1
//! and v1→v2) over N committed instances, **CLI-owned and deterministic** (the determinism
//! boundary — no LLM in the structural path).
//!
//! # The live `add-field` dogfood
//!
//! The one genuinely pending shape change post-freeze is the **schema-version stamp**
//! itself: the engine injects an `added-optional-field` declaration into every frozen
//! persisted doctype, so a committed v0 corpus (authored before the stamp existed) is a
//! real `added-field` migration target. This verb runs the real schema-diff classifier
//! over the genuine corpus and applies the engine transform's `added-optional-field`
//! branch — the live e2e, not synthetic coverage.
//!
//! # Prior-schema sourcing + the v1→v2 value-bump
//!
//! A **below-version** stamped doc (a real v1→v2 transition) cannot derive its prior shape
//! from the current schema, so the verb sources `from` per committed doc by stamp from the
//! versioned snapshot store (`schema-snapshots/<ty>.v<k>.yaml`, via
//! [`crate::pack::load_prior_schema`]) — the engine diffs **two real declared schemas**, the
//! determinism boundary in its strongest form. The stamp, present in both shapes, is
//! **value-bumped** `k → current` ([`bump_and_regate`]) rather than added; a missing snapshot
//! **blocks** the doc with a route (never a silent `already-current`, closing the Inc-3
//! detector/verb divergence — `design/corpus-migration.md` → Prior-schema sourcing).
//!
//! # Per-doc transaction + stamp-flips-last
//!
//! Each doc is migrated in a scratch buffer and written back **only on a clean conformance
//! gate** (the engine fold's per-doc granularity). A doc whose migration mints an empty
//! required slot (a Framing-A prose-needing change) **fails the gate and rolls back** —
//! it stays byte-identical v0, unstamped, and re-detects `migrate`. So the schema-version
//! stamp **flips last**: it only lands once every structural splice *and* every
//! prose-pending slot is authored and the doc gates clean (`design/corpus-migration.md` →
//! the stamp-flips-last rule). A blocked doc is **routed to the agent** to author the prose
//! (Framing A); the agent authors it through the write verbs and re-runs the migration.

use crate::cli::Format;
use crate::invocation_log::Outcome;
use crate::pack;
use crate::render;
use anyhow::{Context, Result};
use engine::file_state::{FileStateRecord, hash_bytes};
use engine::packsource::PackSource;
use engine::schema::{SCHEMA_VERSION_FIELD, Schema, SectionBody};
use engine::schema_diff::{SchemaChange, schema_diff};
use engine::transform::{CorpusDoc, CorpusMigration, DocOutcome, migrate_corpus};
use std::path::{Path, PathBuf};

/// One doctype's migration job: its current shape (`to`, stamp-injected) and its current
/// manifest schema-version (the value the stamp is filled/bumped to + the "already current"
/// threshold). The **prior** shape (`from`) is resolved **per committed doc by stamp** in
/// [`migrate_committed_corpus`] — stamp-absent (v0) docs derive it as `strip_stamp(to)`;
/// below-version (v1→v2) docs source it from the versioned snapshot store via
/// [`crate::pack::load_prior_schema`] — so it is not a per-doctype field.
pub(crate) struct DoctypeMigration {
    /// The doctype's id (the snapshot-store key + diagnostics).
    pub ty: String,
    /// The current schema shape, with the engine-injected schema-version stamp.
    pub to: Schema,
    /// The doctype's current manifest schema-version (the stamp value + currency floor).
    pub version: u32,
    /// The resolved `docs-root` prefix (`""` for a flat layout). Applied to the **relocation
    /// walk-home** only: a relocated doctype's committed instances sit under this prefix at
    /// the prior `location:` home, but the prior *snapshot* stores that location raw, so
    /// [`resolve_migration_homes`] re-applies the prefix. The in-place `to.location` is
    /// already docs-root-resolved (via `all_schemas`), so this is inert there.
    pub docs_root: String,
}

/// The outcome of a corpus migration run, rendered by [`render::corpus_migration`].
#[derive(Debug, serde::Serialize)]
pub struct CorpusMigrationReport {
    /// Docs migrated (stamped + structurally upgraded), written back byte-stable —
    /// repo-relative paths, sorted.
    pub migrated: Vec<String>,
    /// Docs already at the current schema-version (skipped, byte-untouched), sorted.
    pub already_current: Vec<String>,
    /// Docs that could not migrate cleanly (a prose-needing mint that blocks until
    /// authored, or a doc halted behind one), each with its route — sorted by path.
    pub blocked: Vec<(String, String)>,
}

/// Run `jigc migrate-corpus` against `cwd`: locate the repo + project layer, build the
/// frozen persisted doctypes' v0→v1 migration jobs from the pack, migrate the committed
/// corpus, render the report through `format`, and print it. A clean run (even with blocked
/// docs routed to the agent) exits 0 — the migration writes; blocked docs are an expected
/// interim state, not a failure. A locator error routes to stderr and exits non-zero.
pub fn run(cwd: &Path, format: Format) -> Outcome {
    match migrate_in_repo(cwd) {
        Ok(report) => {
            println!("{}", render::corpus_migration(format, &report));
            Outcome::success()
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            Outcome::failure()
        }
    }
}

/// Locate the repo + project layer, assemble the frozen persisted doctypes' migration jobs
/// (each carrying its current stamp-injected shape `to` + its manifest schema-version; the
/// prior `from` is resolved per committed doc by stamp inside [`migrate_committed_corpus`]),
/// and migrate the committed corpus.
fn migrate_in_repo(cwd: &Path) -> Result<CorpusMigrationReport> {
    let jigc_home = require_project_layer(cwd)?;
    let pack = pack::make_pack();
    let pack = pack.as_ref();
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);
    let schemas = defs.all_schemas(pack)?;
    // The doctype → manifest schema-version map gates which doctypes migrate (the frozen
    // persisted set) and supplies each stamp's value — the same authority the version-aware
    // detector routes against (`design/validation.md` → Version-aware routing).
    let versions = pack::frozen_doctype_versions(pack);
    // The resolved docs-root prefix — re-applied to a relocated doctype's prior (snapshot,
    // raw) `location:` walk-home so the committed instances under `docs-root` are found.
    let docs_root = crate::start::docs_root_prefix(&resolved).to_string();

    let mut doctypes = Vec::new();
    for (ty, to) in schemas {
        // Only the frozen persisted doctypes carry the stamp + a migration target; a
        // transient doctype (neither `location:` nor `placement:` — e.g. `commit`) or a
        // non-frozen one migrates nothing. **Persisted** is `location OR placement` — a
        // relocated doctype (M38 changelog: `placement: CHANGELOG.md`, `location: None`)
        // must reach the job list so its instances relocate (the sibling `pack.rs`
        // stamp-inject gate uses the same `location.is_some() || placement.is_some()`
        // idiom; `design/storage.md` → Placement). `resolve_migration_homes` then walks
        // the prior `location:` home and moves each instance to the placement `file`.
        let Some(&version) = versions.get(&ty) else {
            continue;
        };
        if to.location.is_none() && to.placement.is_none() {
            continue;
        }
        doctypes.push(DoctypeMigration {
            ty,
            to,
            version,
            docs_root: docs_root.clone(),
        });
    }
    // Deterministic doctype order (the committed walk + the fold both consume it in order).
    doctypes.sort_by(|a, b| a.ty.cmp(&b.ty));

    let jigc_root = jigc_home.join(".jigc");
    migrate_committed_corpus(pack, &jigc_home, &jigc_root, &doctypes)
}

/// Migrate the committed corpus under `repo_root` for each [`DoctypeMigration`], per-doc
/// gated and WIP-safe, re-baselining any rewritten doc that already carried a file-state
/// hash. `jigc_root` is the `.jigc/` dir holding the file-state record; `pack` is the
/// snapshot store the below-version prior shapes are sourced from.
///
/// The **prior shape (`from`) is resolved per committed doc by its schema-version stamp**
/// (`design/corpus-migration.md` → Prior-schema sourcing):
/// - **at-or-above** the doctype's current version → `already-current`, byte-untouched.
/// - **stamp absent** (the v0 corpus state) → `from = strip_stamp(to)`, the unchanged
///   `added-optional-field` stamp path (the live Inc-3 dogfood); the stamp is *added*.
/// - **below-version** (`1 ≤ k < current`, a v1→v2 transition) → `from` = the versioned
///   snapshot `schema-snapshots/<ty>.v<k>.yaml` via [`crate::pack::load_prior_schema`]; the
///   structural diff is applied **and** the stamp is **value-bumped** `k → current` (it
///   already exists, so this is a `set_field` splice, not an add-field). A **missing**
///   snapshot **blocks** that doc with a route — never a silent `already-current` (closing
///   the Inc-3 detector/verb divergence; [`DECISIONS.md`] → 2026-06-25 audit Finding 2).
///
/// Each doc's change list is the schema-diff filtered to what it still needs (a prose slot
/// already authored is dropped, so it flips cleanly on a re-run). The engine fold applies
/// them, gates each doc on conformance against the current schema, and commits only the
/// clean ones; this writes the migrated bytes back to disk and flips the file-state baseline.
pub(crate) fn migrate_committed_corpus(
    pack: &dyn PackSource,
    repo_root: &Path,
    jigc_root: &Path,
    doctypes: &[DoctypeMigration],
) -> Result<CorpusMigrationReport> {
    let mut report = CorpusMigrationReport {
        migrated: Vec::new(),
        already_current: Vec::new(),
        blocked: Vec::new(),
    };

    // Prepare every candidate doc across the corpus (heterogeneous: each carries its own
    // schema pair + per-doc change list), collected and path-sorted so the fold — which
    // halts at the first blocker — is deterministic.
    let mut prepared: Vec<PreparedDoc> = Vec::new();
    for dt in doctypes {
        // Resolve the FROM home to walk (where the committed instances actually sit) and,
        // for a doctype **relocated** to a single-file placement home, the literal TO path
        // each instance moves to. The corpus walk keys on the **from** home, never the
        // (dir-less) placement `to` home, so a relocated doctype's instances at the old
        // location are still found (`design/corpus-migration.md` → Relocation).
        let Some((walk_home, relocate_to)) = resolve_migration_homes(pack, dt) else {
            continue;
        };
        // The stamp's value comes from the doctype's manifest version: thread it in as the
        // field `default` so the v0 add-field branch (which has no value source for a bare
        // `set`-derived field) places it deterministically. Inert for the below-version
        // path (the stamp is already present there — it is value-bumped, not added).
        let to = with_stamp_default(&dt.to, dt.version);
        for slug in committed_slugs(repo_root, &walk_home) {
            let rel_key = format!("{walk_home}{slug}.md");
            // The destination: the placement `to` for a relocated doctype, else the doc's
            // own home (an in-place migration, `target_key == rel_key`).
            let target_key = relocate_to.clone().unwrap_or_else(|| rel_key.clone());
            let Ok(bytes) = std::fs::read(repo_root.join(&rel_key)) else {
                continue; // read race: skip; the next run re-checks.
            };
            let source = String::from_utf8_lossy(&bytes).into_owned();
            match read_stamp_from_source(&source) {
                // At or above the current version: already current, byte-untouched.
                Some(s) if s >= dt.version => report.already_current.push(rel_key),
                // A below-version stamp (a v1→v2 transition): source the prior shape from
                // the versioned snapshot store and value-bump the stamp `k → current`. A
                // missing snapshot blocks the doc — never a silent already-current (the
                // Finding-2 fix: a doc the version-aware detector routes `migrate` is
                // migrated by the verb).
                Some(k) => match crate::pack::load_prior_schema(pack, &dt.ty, k) {
                    Ok(from) => {
                        let changes = per_doc_changes(&schema_diff(&from, &to), &source, false);
                        // The stamp is value-bumped `k → current` **post-fold** (it already
                        // exists, so it is a `set_field` value splice, not an add-field) — on
                        // the gated v2 bytes, which are guaranteed to conform to `to` (the
                        // committed source's own shape may be the prior `from`, so it cannot be
                        // bumped directly; the gated v2 always can). `bump_to` carries the
                        // target version into the post-fold pass.
                        prepared.push(PreparedDoc {
                            rel_key,
                            target_key,
                            source,
                            from,
                            to: to.clone(),
                            changes,
                            bump_to: Some(dt.version),
                        });
                    }
                    Err(_) => {
                        let route = missing_snapshot_route(&rel_key, &dt.ty, k);
                        report.blocked.push((rel_key, route));
                    }
                },
                // Stamp absent (the v0 corpus state): the unchanged add-field path (the stamp
                // is *added* at the current value via its `default`, so no post-fold bump).
                None => {
                    let from = strip_stamp(&dt.to);
                    let changes = per_doc_changes(&schema_diff(&from, &to), &source, true);
                    if changes.is_empty() {
                        // Nothing this doc needs (its shape already matches): leave it.
                        report.already_current.push(rel_key);
                        continue;
                    }
                    prepared.push(PreparedDoc {
                        rel_key,
                        target_key,
                        source,
                        from,
                        to: to.clone(),
                        changes,
                        bump_to: None,
                    });
                }
            }
        }
    }
    prepared.sort_by(|a, b| a.rel_key.cmp(&b.rel_key));

    let corpus: Vec<CorpusDoc> = prepared
        .iter()
        .map(|p| CorpusDoc {
            id: &p.rel_key,
            old_schema: &p.from,
            new_schema: &p.to,
            source: &p.source,
            changes: &p.changes,
        })
        .collect();
    let result: CorpusMigration = migrate_corpus(&corpus);

    let mut record = FileStateRecord::load(jigc_root)
        .with_context(|| format!("loading the file-state record at {jigc_root:?}"))?;
    for (i, outcome) in result.docs.iter().enumerate() {
        match outcome {
            DocOutcome::Migrated { id, v2 } => {
                // A below-version doc value-bumps its schema-version stamp `k → current` on
                // the gated v2 bytes, then re-gates (the bump is a byte-stable value splice
                // that keeps the doc conformant — asserted, not assumed). A stamp-absent
                // (v0) doc already carries the current stamp from the add-field branch.
                let prep = &prepared[i];
                let v2 = match prep.bump_to {
                    Some(version) => bump_and_regate(&prep.to, v2, version)?,
                    None => v2.clone(),
                };
                // A **relocated** doctype's destination differs from its source home; an
                // in-place migration writes back to the same key (`target_key == rel_key`).
                let target = &prep.target_key;
                let moved = target != id;

                // WRITE-BEFORE-REMOVE (`corpus-migration.md` → Relocation: write-to-`to`
                // precedes remove-`from`, so an abort between strands neither copy). The
                // gated v2 bytes land at the destination **first** — a fault here leaves the
                // source at `id` untouched on disk (never zero copies). Only once `to`
                // exists is the old-home source removed and the file-state re-keyed, one
                // atomic per-doc unit.
                engine::state::persist(&repo_root.join(target), v2.as_bytes())
                    .with_context(|| format!("writing the migrated doc {target}"))?;
                if moved {
                    std::fs::remove_file(repo_root.join(id))
                        .with_context(|| format!("removing the relocated source {id}"))?;
                }

                // Re-baseline / re-key the file-state, paired **per doc** with this doc's
                // write so the on-disk baseline always matches the on-disk files (the
                // per-doc-gated transaction granularity — `corpus-migration.md` → Migration
                // atomicity / WIP-safety; `reconciliation.md` → the file-state re-key). A
                // **moved**, tracked doc drops its old-home key and re-registers at the
                // destination `from → to`, so the instance re-registers and none is orphaned;
                // an in-place tracked doc re-hashes at its key; an untracked doc adopts
                // nothing (the migration tracks nothing it didn't already track).
                if moved {
                    if record.forget(id) {
                        record.record(target.clone(), hash_bytes(v2.as_bytes()));
                        record.save(jigc_root).with_context(|| {
                            format!("saving the file-state record at {jigc_root:?}")
                        })?;
                    }
                } else if record.get(id).is_some() {
                    record.record(id.clone(), hash_bytes(v2.as_bytes()));
                    record.save(jigc_root).with_context(|| {
                        format!("saving the file-state record at {jigc_root:?}")
                    })?;
                }
                report.migrated.push(target.clone());
            }
            DocOutcome::Untouched { id, .. } => {
                let route = if result.halted_at == Some(i) {
                    prose_needing_route(id)
                } else {
                    deferred_route(id)
                };
                report.blocked.push((id.clone(), route));
            }
        }
    }

    report.migrated.sort();
    report.already_current.sort();
    report.blocked.sort();
    Ok(report)
}

/// One prepared migration job, owning its source + schema pair + per-doc change list (the
/// fold borrows them).
struct PreparedDoc {
    /// The doc's **source** home — where the committed instance sits (the fold's id, the
    /// bytes read, and the file removed after a relocation move).
    rel_key: String,
    /// The doc's **destination** home. Equal to `rel_key` for an in-place migration; the
    /// literal placement `to` for a relocated doctype (the file the gated v2 bytes are
    /// written to; a move iff `target_key != rel_key`).
    target_key: String,
    source: String,
    from: Schema,
    to: Schema,
    changes: Vec<SchemaChange>,
    /// `Some(version)` for a below-version doc whose schema-version stamp is value-bumped to
    /// `version` post-fold (over the gated v2 bytes); `None` for a stamp-absent (v0) doc,
    /// whose stamp is *added* at the current value by the add-field branch.
    bump_to: Option<u32>,
}

/// The per-doc change list: the doctype's `fixed` schema-diff filtered to what this doc
/// still needs. The schema-version stamp add-field is kept iff the doc carries no stamp; a
/// prose-needing slot is kept iff the doc does not already carry that section heading (an
/// already-authored slot is dropped, so the doc flips cleanly once the prose lands).
fn per_doc_changes(fixed: &[SchemaChange], source: &str, stamp_absent: bool) -> Vec<SchemaChange> {
    fixed
        .iter()
        .filter(|change| match change {
            SchemaChange::AddedOptionalField { field, .. } if field == SCHEMA_VERSION_FIELD => {
                stamp_absent
            }
            SchemaChange::ProseNeeding { section, .. } => !has_section_heading(source, section),
            _ => true,
        })
        .cloned()
        .collect()
}

/// Whether `source` carries a body section heading (`## …`) matching `section_id` under the
/// engine's heading↔id rule (`slugify(heading) == id`) — the same inverse the parser uses,
/// so a multi-word id round-trips. Used to decide whether a prose-needing slot is already
/// authored (independent of a full parse, which a still-incomplete doc fails).
fn has_section_heading(source: &str, section_id: &str) -> bool {
    source.lines().any(|line| {
        line.strip_prefix("## ")
            .is_some_and(|text| engine::slug::slugify(text.trim()) == section_id)
    })
}

/// Read a committed doc's schema-version stamp from its leading `---` front-matter block, if
/// present and integer-valued. Independent of a full parse so an incomplete (mid-migration)
/// doc's stamp is still legible. A doc with no front-matter fence, or no `schema-version`
/// line, yields `None` (the v0 corpus state).
fn read_stamp_from_source(source: &str) -> Option<u32> {
    let body = source.strip_prefix("---\n")?;
    let end = body.find("\n---")?;
    body[..end].lines().find_map(|line| {
        line.strip_prefix(&format!("{SCHEMA_VERSION_FIELD}:"))
            .and_then(|v| v.trim().parse::<u32>().ok())
    })
}

/// A clone of `schema` with the engine schema-version stamp field's `default` set to
/// `version` — so the transform's `added-optional-field` branch (which reads `default` for
/// the value) places the stamp deterministically. The stamp's shape is otherwise
/// unchanged; this is only the migration-time value source the CLI threads in.
fn with_stamp_default(schema: &Schema, version: u32) -> Schema {
    let mut out = schema.clone();
    for section in &mut out.sections {
        if let SectionBody::Simple { fields, .. } = &mut section.body
            && let Some(field) = fields.iter_mut().find(|f| f.id == SCHEMA_VERSION_FIELD)
        {
            field.default = Some(version.to_string());
        }
    }
    out
}

/// **Value-bump** a migrated doc's schema-version stamp to `version` and **re-gate** the
/// result — the v1→v2 stamp transition (`design/corpus-migration.md` → the stamp value-bump),
/// distinct from the v0→v1 add-field path (the field already exists, so `added-field` does not
/// cover it). Called **post-fold** over the gated v2 bytes, which conform to `schema` (the
/// committed source's own shape may still be the prior `from`, so it cannot be bumped
/// directly; the gated v2 always can).
///
/// The bump is [`engine::write::set_field`] — a present-field value splice (byte-stable: it
/// replaces only the value digits after `schema-version:`, every other byte intact). The
/// re-gate asserts the bumped bytes still conform (they do — the stamp's *value* is not a
/// conformance constraint), proving byte-stability + conformance rather than trusting reuse.
/// A failure (a snapshot drift the deferred snapshot-hash gate would otherwise catch) fails
/// the run **loudly** ([`DECISIONS.md`] → snapshot hashing deferred / fails loudly).
fn bump_and_regate(schema: &Schema, v2: &str, version: u32) -> Result<String> {
    let bumped = bump_stamp(schema, v2, version)?;
    let doc = engine::parse::parse_sections(schema, &bumped)
        .map_err(|e| anyhow::anyhow!("re-parsing the value-bumped doc: {e:?}"))?;
    let findings = engine::validate::schema_conformance(schema, &bumped, &doc);
    if !findings.is_empty() {
        anyhow::bail!("the value-bumped doc no longer conforms: {findings:?}");
    }
    Ok(bumped)
}

/// Splice a present schema-version stamp's value to `version` via [`engine::write::set_field`]
/// (a byte-stable value splice). `schema` is the shape the bumped `source` conforms to (its
/// header carries the stamp field). An absent stamp section / a non-conforming source is an
/// `Err` (loud, never a silent skip).
fn bump_stamp(schema: &Schema, source: &str, version: u32) -> Result<String> {
    let section = stamp_section_id(schema).ok_or_else(|| {
        anyhow::anyhow!("the schema declares no schema-version stamp section to value-bump")
    })?;
    engine::write::set_field(
        schema,
        source,
        &section,
        SCHEMA_VERSION_FIELD,
        &version.to_string(),
    )
    .map_err(|e| anyhow::anyhow!("value-bumping the schema-version stamp `{section}`: {e:?}"))
}

/// The id of the section carrying the schema-version stamp field — the existing header for a
/// header-bearing doctype, the injected `meta` header for a header-less one. The [`set_field`]
/// address for the value-bump.
///
/// [`set_field`]: engine::write::set_field
fn stamp_section_id(schema: &Schema) -> Option<String> {
    schema
        .sections
        .iter()
        .find_map(|section| match &section.body {
            SectionBody::Simple { fields, .. }
                if fields.iter().any(|f| f.id == SCHEMA_VERSION_FIELD) =>
            {
                Some(section.id.clone())
            }
            _ => None,
        })
}

/// The route for a below-version stamped doc whose prior-shape snapshot is **not shipped**:
/// the migration cannot source the `from` it would diff against, so the doc is blocked (never
/// a silent `already-current` — the detector routes it `migrate`). Ship the snapshot, re-run.
fn missing_snapshot_route(rel_key: &str, ty: &str, stamp: u32) -> String {
    format!(
        "blocked — `{rel_key}` is stamped schema-version {stamp}, below current, but no prior-schema \
         snapshot `schema-snapshots/{ty}.v{stamp}.yaml` is shipped to source the migration from; \
         ship the snapshot, then re-run `jigc migrate-corpus`"
    )
}

/// A clone of `schema` with the engine schema-version stamp field removed — the doctype's
/// **prior (v0) shape**, the schema-diff's `from`. For a header-bearing doctype this drops
/// just the appended stamp leaf; for a header-less one (whose stamp lives in the synthetic
/// `meta` header the injection minted) it leaves that header empty, so the diff classifies
/// the stamp as an `added-optional-field` that introduces the fence.
fn strip_stamp(schema: &Schema) -> Schema {
    let mut out = schema.clone();
    for section in &mut out.sections {
        if let SectionBody::Simple { fields, .. } = &mut section.body {
            fields.retain(|f| f.id != SCHEMA_VERSION_FIELD);
        }
    }
    out
}

/// The Framing-A route for a doc whose migration minted an empty required slot: author the
/// prose, then re-run — the stamp flips only once the doc gates clean.
fn prose_needing_route(rel_key: &str) -> String {
    format!(
        "author the new required prose in `{rel_key}` through the write verbs, then re-run \
         `jigc migrate-corpus` (the schema-version stamp flips only once it gates clean)"
    )
}

/// The route for a doc left untouched behind the run's first blocker (WIP-safety: the fold
/// halts at the first blocked doc, never half-transforming the rest).
fn deferred_route(rel_key: &str) -> String {
    format!(
        "deferred — `{rel_key}` will migrate once the blocker above is resolved; re-run `jigc migrate-corpus`"
    )
}

/// Resolve, for one doctype migration job, the **from** home the corpus walk enumerates
/// (where the committed instances actually sit) and — for a doctype **relocated** to a
/// single-file placement home — the literal **to** path each instance moves to
/// (`design/corpus-migration.md` → Relocation: the walk keys on the from home). `None` when
/// the doctype has no walkable home (a relocated doctype whose prior-location snapshot is
/// missing).
///
/// - A doctype whose **current** shape still declares a `location:` directory migrates
///   **in place** — walk that directory, no move (`relocate_to = None`).
/// - A doctype whose current shape is a single-file `placement:` (no `location:`) has
///   **relocated**: its committed instances still sit at the **prior** version's `location:`
///   home, sourced from the versioned snapshot at `version - 1` via
///   [`crate::pack::load_prior_schema`]. The walk enumerates that old directory; each instance
///   moves to the placement `file`. A missing / location-less prior snapshot yields `None`.
fn resolve_migration_homes(
    pack: &dyn PackSource,
    dt: &DoctypeMigration,
) -> Option<(String, Option<String>)> {
    if let Some(location) = &dt.to.location {
        return Some((location.clone(), None));
    }
    let placement = dt.to.placement.as_ref()?;
    let prior = crate::pack::load_prior_schema(pack, &dt.ty, dt.version.checked_sub(1)?).ok()?;
    let raw_home = prior.location?;
    // The prior snapshot stores its `location:` **raw** (docs-root-free), but the committed
    // instances sit under the resolved `docs-root` prefix — re-apply it (the in-place branch
    // above returns an already-resolved `to.location`, so this is the sole re-application
    // site; `crate::start::docs_root_prefix`).
    let from_home = if dt.docs_root.is_empty() {
        raw_home
    } else {
        format!("{}/{raw_home}", dt.docs_root)
    };
    Some((from_home, Some(placement.file.clone())))
}

/// The committed-doc slugs of a persisted type — the `.md` file stems under
/// `<repo_root>/<location>`, slug-sorted. A missing / unreadable location yields none.
fn committed_slugs(repo_root: &Path, location: &str) -> Vec<String> {
    let dir = repo_root.join(location);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut slugs: Vec<String> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(str::to_string))
        .collect();
    slugs.sort();
    slugs
}

/// Locate the repo root and its `.jigc/config/` project layer — the store-walk locate
/// preamble shared with `jigc validate` / `jigc ingest`. Errors with routed messages when
/// the repo or the project layer is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    let ctx = crate::locate::locate(cwd)?;
    if ctx.project_config.is_none() {
        anyhow::bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }
    Ok(ctx.jigc_home)
}

#[cfg(test)]
mod tests {
    //! Verb-core coverage the real-binary dogfood (adr, header-bearing add-field) cannot
    //! reach over the frozen dev pack: the **header-less fence-introducing** add-field, and
    //! the **combined stamp-flips-last** case (a v0→v1 that adds the stamp *and* a new
    //! required slot — no such change exists in the frozen-v1 dev pack, so it is exercised
    //! over synthetic doctypes through [`migrate_committed_corpus`], the same core the verb
    //! runs).

    use super::*;
    use engine::field_block::{Field, Value};
    use engine::parse::parse_sections;
    use engine::schema::{inject_schema_version_stamp, load_schema};
    use engine::validate::schema_conformance;
    use engine::write::{Instance, SectionContent, render};
    use std::fs;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-migrate-corpus-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
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

    /// Load a schema, then inject the engine schema-version stamp — the CLI pack-loader's
    /// v1 shape, reconstructed for a synthetic doctype.
    fn v1_schema(yaml: &[u8]) -> Schema {
        let mut schema = load_schema(yaml).expect("schema loads");
        inject_schema_version_stamp(&mut schema);
        schema
    }

    /// Build a [`DoctypeMigration`] from the stamp-injected current shape `to`, at `version`.
    /// The prior shape is resolved per committed doc by stamp inside
    /// [`migrate_committed_corpus`] (stamp-absent → `strip_stamp(to)`; below-version →
    /// the snapshot store), so it is not a field here.
    fn migration(to: Schema, version: u32) -> DoctypeMigration {
        DoctypeMigration {
            ty: to.ty.clone(),
            to,
            version,
            // The core tests seed at the flat repo-root home (no docs-root); the docs-root
            // re-application on the relocation walk-home is covered end-to-end by the
            // `corpus_migration` binary test.
            docs_root: String::new(),
        }
    }

    /// Build a throwaway pack dir carrying one prior-schema snapshot
    /// (`schema-snapshots/<ty>.v<version>.yaml`) + a freeze manifest declaring `<ty>` frozen,
    /// so [`crate::pack::load_prior_schema`] resolves field-types **and** injects the
    /// schema-version stamp into the snapshot identically to the current schema. Returns the
    /// owning [`TempDir`] (the caller keeps it alive so the files outlive the pack).
    fn snapshot_pack(ty: &str, version: u32, snapshot_yaml: &str) -> TempDir {
        let dir = TempDir::new("snap-pack");
        let snaps = dir.path().join("schema-snapshots");
        fs::create_dir_all(&snaps).expect("mk schema-snapshots");
        fs::write(snaps.join(format!("{ty}.v{version}.yaml")), snapshot_yaml).expect("write snap");
        let cfg = dir.path().join("config");
        fs::create_dir_all(&cfg).expect("mk config");
        // Only the doctype name gates stamp injection; the manifest version/hash are not read
        // on this path (the freeze gate does not run here), so they are placeholders.
        fs::write(
            cfg.join("schema-manifest.yaml"),
            format!(
                "doctypes:\n  - type: {ty}\n    schema-version: {version}\n    schema-hash: {}\n",
                "0".repeat(64)
            ),
        )
        .expect("write manifest");
        dir
    }

    /// Write `content` to `repo_root/<rel_key>`, creating the location dir.
    fn write_doc(repo_root: &Path, rel_key: &str, content: &str) {
        let path = repo_root.join(rel_key);
        fs::create_dir_all(path.parent().unwrap()).expect("mk location dir");
        fs::write(&path, content).expect("write doc");
    }

    /// Assert a doc's bytes conform to `schema` (zero conformance findings) and round-trip
    /// byte-identical.
    fn assert_conformant_and_stable(schema: &Schema, source: &str) {
        let doc = parse_sections(schema, source).expect("doc parses under the current schema");
        let findings = schema_conformance(schema, source, &doc);
        assert!(
            findings.is_empty(),
            "migrated doc must conform; got {findings:?}\nsource:\n{source}"
        );
        let inst = engine::write::instance_from_source(schema, source).expect("re-parse");
        assert_eq!(
            render(schema, &inst),
            source,
            "migrated doc must round-trip byte-identical"
        );
    }

    /// A header-less doctype (`note`): no `---` block until the stamp introduces one.
    fn note_v0_yaml() -> &'static [u8] {
        b"\
type: note
location: notes/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: success
    slot: { hint: \"s\" }
"
    }

    /// The header-less add-field case **through the verb core**: migrating a committed v0
    /// `note` introduces the `---` fence carrying `schema-version: 1` byte-stable, preserves
    /// the body slots, and reports it migrated.
    #[test]
    fn header_less_doc_migration_introduces_the_fence_byte_stable() {
        let repo = TempDir::new("fence");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());

        // A conformant v0 note authored through `render` over the **genuine** header-less
        // shape (the byte-stable form a real header-less committed doc has — `strip_stamp`
        // leaves an empty `meta` header, used only by the schema-diff, never to render).
        let v0 = render(
            &load_schema(note_v0_yaml()).expect("v0 note loads"),
            &Instance {
                title: "A Note".to_string(),
                sections: vec![
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("A clean flow.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "success".to_string(),
                        slot: Some("It works.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            !v0.starts_with("---"),
            "the v0 note carries no front-matter fence"
        );
        write_doc(repo.path(), "notes/a-note.md", &v0);

        // The stamp-absent (v0) path derives `from = strip_stamp(to)` internally and never
        // touches the snapshot store, so the pack is unused here.
        let pack = crate::pack::EmbeddedPack::new();
        let report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 1)])
                .expect("migration runs");

        assert_eq!(report.migrated, vec!["notes/a-note.md".to_string()]);
        assert!(
            report.blocked.is_empty(),
            "no blockers: {:?}",
            report.blocked
        );

        let migrated = fs::read_to_string(repo.path().join("notes/a-note.md")).expect("read");
        assert!(
            migrated.starts_with("---\nschema-version: 1\n---\n\n"),
            "the fence is introduced carrying the stamp; got:\n{migrated}"
        );
        assert!(
            migrated.contains("A clean flow.") && migrated.contains("It works."),
            "the body slots survive; got:\n{migrated}"
        );
        assert_conformant_and_stable(&to, &migrated);
    }

    /// An already-stamped doc is the false-positive guard: it is skipped, byte-untouched.
    #[test]
    fn already_current_doc_is_skipped_byte_untouched() {
        let repo = TempDir::new("current");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());

        // A note already stamped at the current version (rendered through the v1 schema).
        let stamped = render(
            &to,
            &Instance {
                title: "Done Note".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Already done.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "success".to_string(),
                        slot: Some("Stamped.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "notes/done-note.md", &stamped);

        let pack = crate::pack::EmbeddedPack::new();
        let report = migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to, 1)])
            .expect("migration runs");

        assert_eq!(
            report.already_current,
            vec!["notes/done-note.md".to_string()]
        );
        assert!(report.migrated.is_empty(), "nothing to migrate");
        let after = fs::read_to_string(repo.path().join("notes/done-note.md")).expect("read");
        assert_eq!(after, stamped, "an already-current doc is byte-untouched");
    }

    /// Per-doc baseline atomicity (M34 audit, `corpus-migration.md` → Migration atomicity /
    /// WIP-safety): the file write and its file-state baseline flip are **one per-doc unit**, so
    /// the on-disk baseline always matches the on-disk files. When an I/O fault aborts the write
    /// loop mid-corpus, the docs already written must already carry their refreshed baseline on
    /// disk — otherwise a subsequent `file-state` detect over the committed store mis-reports
    /// those correctly-migrated docs as out-of-band drift.
    ///
    /// Two tracked v0 notes: `a` sorts before `b`, so `a` persists first; `b`'s atomic write is
    /// forced to fail (its temp-sibling `notes/b.md.tmp` is pre-occupied by a directory, so
    /// [`engine::state::persist`]'s temp write errors), aborting the loop. After the abort `a` is
    /// on disk migrated, and its on-disk baseline must already equal the hash of those bytes.
    #[test]
    fn written_docs_are_baselined_per_doc_even_when_a_later_write_aborts() {
        let repo = TempDir::new("partial-abort");
        let jigc_root = repo.path().join(".jigc");

        let to = v1_schema(note_v0_yaml());
        let v0_schema = load_schema(note_v0_yaml()).expect("v0 note loads");
        let note = |title: &str| {
            render(
                &v0_schema,
                &Instance {
                    title: title.to_string(),
                    sections: vec![
                        SectionContent {
                            id: "vision".to_string(),
                            slot: Some("V.".to_string()),
                            ..Default::default()
                        },
                        SectionContent {
                            id: "success".to_string(),
                            slot: Some("S.".to_string()),
                            ..Default::default()
                        },
                    ],
                },
            )
        };
        let a0 = note("A Note");
        let b0 = note("B Note");
        write_doc(repo.path(), "notes/a.md", &a0);
        write_doc(repo.path(), "notes/b.md", &b0);

        // Both docs are already tracked in the file-state baseline (the migration re-baselines
        // only docs it already tracks) at their v0 hashes.
        let mut seed = FileStateRecord::new();
        seed.record("notes/a.md".to_string(), hash_bytes(a0.as_bytes()));
        seed.record("notes/b.md".to_string(), hash_bytes(b0.as_bytes()));
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        // Force `b`'s atomic write to fail: occupy its temp-sibling path with a directory, so
        // `persist` errors when it writes the temp file — the loop aborts after `a` is written.
        fs::create_dir_all(repo.path().join("notes/b.md.tmp")).expect("occupy temp sibling");

        let pack = crate::pack::EmbeddedPack::new();
        let result = migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to, 1)]);
        assert!(
            result.is_err(),
            "the aborted write surfaces as an error: {result:?}"
        );

        // `a` was written to disk migrated; its on-disk baseline must already reflect those
        // bytes (the per-doc commit), so a later `file-state` detect would NOT mis-report it.
        let on_disk_a = fs::read(repo.path().join("notes/a.md")).expect("a was written");
        assert_ne!(
            on_disk_a,
            a0.as_bytes(),
            "a was actually migrated (not still v0)"
        );
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("notes/a.md"),
            Some(hash_bytes(&on_disk_a).as_str()),
            "the already-written doc `a` must be baselined to its on-disk bytes before the abort"
        );
    }

    /// The v1 prior shape of the `memo` doctype (a single `vision` slot) — the snapshot the
    /// below-version path sources `from` from.
    fn memo_v1_yaml() -> &'static str {
        "\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
"
    }

    /// The v2 current shape of the `memo` doctype (a new required trailing `rationale` slot).
    fn memo_v2_yaml() -> &'static [u8] {
        b"\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: rationale
    slot: { hint: \"why\" }
"
    }

    /// The **stamp-flips-last** discipline on the below-version (v1→v2) snapshot path: a memo
    /// whose v1→v2 adds a new required `rationale` slot, sourced `from` the versioned snapshot
    /// store. The committed **v1-stamped** doc lacks the slot, so the migration mints it empty,
    /// **fails the gate, and rolls back** — the doc stays byte-identical v1 (stamp `1`, **not**
    /// value-bumped to `2`), routed to the agent. Once the prose is authored, the same doc
    /// migrates clean and the stamp **flips** `1→2` — proof the value-bump lands only on a
    /// clean gate (post-fold), never while prose is pending.
    #[test]
    fn below_version_prose_needing_stamp_flips_only_after_prose_authored() {
        let repo = TempDir::new("flips-last");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("memo", 1, memo_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(memo_v2_yaml());
        // The prior shape the verb sources from the snapshot store (vision + injected stamp)
        // — the byte form a committed v1 memo is rendered against.
        let from = crate::pack::load_prior_schema(&pack, "memo", 1).expect("the memo.v1 snapshot");

        // --- phase 1: the v1 doc lacks rationale → migration blocks, doc stays v1 ---
        let v1 = render(
            &from,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Move the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "memos/cache-memo.md", &v1);

        let blocked_report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("migration runs");

        assert!(
            blocked_report.migrated.is_empty(),
            "the prose-needing doc does not migrate: {:?}",
            blocked_report.migrated
        );
        assert_eq!(
            blocked_report.blocked.len(),
            1,
            "the doc is blocked, routed to the agent: {:?}",
            blocked_report.blocked
        );
        assert_eq!(blocked_report.blocked[0].0, "memos/cache-memo.md");
        assert!(
            blocked_report.blocked[0]
                .1
                .contains("re-run `jigc migrate-corpus`"),
            "the route is Framing-A author-then-re-run: {}",
            blocked_report.blocked[0].1
        );
        // STAMP-FLIPS-LAST: the on-disk doc is byte-identical v1 — stamp NOT bumped to 2 —
        // because the value-bump is post-fold and the doc never reached a clean gate, so
        // nothing was written.
        let still_v1 = fs::read_to_string(repo.path().join("memos/cache-memo.md")).expect("read");
        assert_eq!(still_v1, v1, "the blocked doc is byte-identical v1");
        assert!(
            still_v1.contains("schema-version: 1") && !still_v1.contains("schema-version: 2"),
            "the stamp did NOT flip while the prose is pending; got:\n{still_v1}"
        );

        // --- phase 2: the agent authors the rationale slot → migration flips the stamp 1→2 ---
        let authored = render(
            &to,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "vision".to_string(),
                        slot: Some("Move the cache.".to_string()),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "rationale".to_string(),
                        slot: Some("Latency wins.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "memos/cache-memo.md", &authored);

        let flipped_report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("re-migration runs");

        assert_eq!(
            flipped_report.migrated,
            vec!["memos/cache-memo.md".to_string()],
            "with the prose authored the doc migrates: {flipped_report:?}",
        );
        assert!(flipped_report.blocked.is_empty(), "no longer blocked");
        let flipped = fs::read_to_string(repo.path().join("memos/cache-memo.md")).expect("read");
        assert!(
            flipped.contains("schema-version: 2"),
            "the stamp flips 1→2 once the prose is authored and the doc gates clean; got:\n{flipped}"
        );
        assert!(
            flipped.contains("Move the cache.") && flipped.contains("Latency wins."),
            "the authored prose survives; got:\n{flipped}"
        );
        assert_conformant_and_stable(&to, &flipped);
    }

    /// The v1 prior shape of the `card` doctype: a single **fixed-slot** `body` section — the
    /// reconstructed-shape analog of the M25 `prd.requirements` fixed-slot→repeatable reshape.
    fn card_v1_yaml() -> &'static str {
        "\
type: card
location: cards/
id-from: title
sections:
  - id: body
    slot: { hint: \"the card body\" }
"
    }

    /// The v2 current shape of the `card` doctype: `body` promoted to a **repeatable**
    /// item-block (the old slot prose becomes the default first item).
    fn card_v2_yaml() -> &'static [u8] {
        b"\
type: card
location: cards/
id-from: title
sections:
  - id: body
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: statement, slot: { hint: \"one card\" } }
"
    }

    /// The headline T2 case: a **below-version stamped** doc (stamp `1`, current `2`) migrates
    /// through `migrate_committed_corpus` (the same core the verb runs) — `from` sourced from
    /// the `schema-snapshots/card.v1.yaml` snapshot, the `fixed-slot→repeatable` structural
    /// change applied, and the stamp **value-bumped** `1→2`. The result (a) round-trips
    /// byte-stable + (b) conforms under the current schema ([`assert_conformant_and_stable`]),
    /// (c) is **idempotent** on re-run (now stamp-2 ⇒ already-current, byte-untouched), and
    /// (d) **detector/verb agree**: a below-version doc the version-aware detector routes
    /// `migrate` (proven over the real binary in `tests/schema_conformance_routing.rs`) is
    /// **migrated** by the verb, never reported `already-current` (DECISIONS → audit Finding 2).
    #[test]
    fn below_version_doc_migrates_via_snapshot_with_stamp_value_bump() {
        let repo = TempDir::new("below-version");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("card", 1, card_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(card_v2_yaml());
        // The fixed-slot prior shape (with the injected stamp) — the form a committed v1 card
        // is rendered against, sourced from the snapshot store exactly as the verb sources it.
        let from = crate::pack::load_prior_schema(&pack, "card", 1).expect("the card.v1 snapshot");

        let v1 = render(
            &from,
            &Instance {
                title: "First Card".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Wire the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            v1.contains("schema-version: 1"),
            "the committed doc is stamped below current; got:\n{v1}"
        );
        write_doc(repo.path(), "cards/first-card.md", &v1);

        let report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("migration runs");

        // (d) detector/verb agree: a below-version doc is MIGRATED, never already-current.
        assert_eq!(
            report.migrated,
            vec!["cards/first-card.md".to_string()],
            "the below-version doc migrates: {report:?}"
        );
        assert!(
            report.already_current.is_empty() && report.blocked.is_empty(),
            "never already-current or blocked: {report:?}"
        );

        let migrated = fs::read_to_string(repo.path().join("cards/first-card.md")).expect("read");
        // The stamp is value-bumped 1→2 (the v1→v2 transition).
        assert!(
            migrated.contains("schema-version: 2") && !migrated.contains("schema-version: 1"),
            "the stamp is value-bumped 1→2; got:\n{migrated}"
        );
        // The structural change landed: the slot prose is preserved as the default item.
        assert!(
            migrated.contains("Wire the cache."),
            "the slot prose survives as the default item; got:\n{migrated}"
        );
        // (a) round-trip byte-stable + (b) conformant under the current schema.
        assert_conformant_and_stable(&to, &migrated);

        // (c) idempotent: a re-run finds the now-stamp-2 doc already current, byte-untouched.
        let rerun =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("re-migration runs");
        assert_eq!(
            rerun.already_current,
            vec!["cards/first-card.md".to_string()],
            "the migrated doc is already-current on a re-run: {rerun:?}"
        );
        assert!(
            rerun.migrated.is_empty(),
            "nothing migrates twice: {rerun:?}"
        );
        let after = fs::read_to_string(repo.path().join("cards/first-card.md")).expect("read");
        assert_eq!(after, migrated, "the re-run leaves the doc byte-untouched");
    }

    /// A below-version stamped doc whose prior-shape snapshot is **not shipped** is **blocked**
    /// with a route naming the missing snapshot — never a silent `already-current` (the
    /// detector routes it `migrate`, so the verb must surface it, not drop it).
    #[test]
    fn below_version_doc_with_a_missing_snapshot_is_blocked_with_a_route() {
        let repo = TempDir::new("missing-snap");
        let jigc_root = repo.path().join(".jigc");

        // A pack with NO `schema-snapshots/` entry, so `load_prior_schema(card, 1)` errors.
        let pack_dir = TempDir::new("empty-pack");
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(card_v2_yaml());
        // The committed v1 doc is rendered against the in-memory fixed-slot shape (the pack
        // ships no snapshot to source it from — that is the point of this case).
        let from_shape = {
            let mut s = load_schema(card_v1_yaml().as_bytes()).expect("v1 card loads");
            inject_schema_version_stamp(&mut s);
            s
        };
        let v1 = render(
            &from_shape,
            &Instance {
                title: "First Card".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Wire the cache.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        );
        write_doc(repo.path(), "cards/first-card.md", &v1);

        let report = migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to, 2)])
            .expect("migration runs");

        assert!(
            report.migrated.is_empty() && report.already_current.is_empty(),
            "a missing snapshot is neither migrated nor already-current: {report:?}"
        );
        assert_eq!(report.blocked.len(), 1, "the doc is blocked: {report:?}");
        assert_eq!(report.blocked[0].0, "cards/first-card.md");
        assert!(
            report.blocked[0]
                .1
                .contains("schema-snapshots/card.v1.yaml")
                && report.blocked[0].1.contains("re-run `jigc migrate-corpus`"),
            "the route names the missing snapshot + the re-run: {}",
            report.blocked[0].1
        );
        // The doc is left byte-untouched (never silently rewritten).
        let after = fs::read_to_string(repo.path().join("cards/first-card.md")).expect("read");
        assert_eq!(after, v1, "the blocked doc is byte-untouched");
    }

    /// The v1 prior shape of the `log` doctype: a folder-location home (`changelog/`) with a
    /// single `body` slot — the reconstructed pre-relocation changelog. The snapshot the
    /// relocation walk sources both the old home *and* the schema-diff `from` from.
    fn log_v1_yaml() -> &'static str {
        "\
type: log
location: changelog/
sections:
  - id: body
    slot: { hint: \"the log\" }
"
    }

    /// The v2 current shape of the `log` doctype: **relocated** to a single literal
    /// `placement:` home (`CHANGELOG.md`, no `location:`) and given a `display-title:
    /// Changelog` (so `# changelog` → `# Changelog`) — the reconstructed changelog v1→v2
    /// relocation.
    fn log_v2_yaml() -> &'static [u8] {
        b"\
type: log
placement: { file: CHANGELOG.md }
display-title: Changelog
sections:
  - id: body
    slot: { hint: \"the log\" }
"
    }

    /// Render a committed **v1-stamped** `log` at its old folder home against `from` (the
    /// snapshot shape, stamp injected): a lowercase `# changelog` H1 + the body slot. The
    /// byte form a real pre-relocation committed changelog has.
    fn log_v1_doc(from: &Schema) -> String {
        render(
            from,
            &Instance {
                title: "changelog".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "body".to_string(),
                        slot: Some("Released 1.0 with the new limiter.".to_string()),
                        ..Default::default()
                    },
                ],
            },
        )
    }

    /// The headline T2 relocation case through `migrate_committed_corpus` (the same core the
    /// verb runs): a committed **v1-stamped** `log` sits at its old folder home
    /// (`changelog/changelog.md`) with a lowercase `# changelog` H1. Its v2 shape is
    /// **relocated** to the literal `CHANGELOG.md` placement home + display-title `Changelog`.
    /// The move-arm (a) walks the **`from`** home (the walk keys on the old location, not the
    /// empty placement `to`), finds the doc, and moves it; (b) writes the gated v2 bytes to
    /// `CHANGELOG.md` byte-faithful — the H1 fixed to `# Changelog`, the stamp value-bumped
    /// `1→2`, the body preserved; (c) **removes** the old `changelog/changelog.md`; and (d)
    /// **re-keys** the file-state baseline `from → to` (the old-home key dropped, the target
    /// re-registered at the migrated hash — no orphan).
    #[test]
    fn relocated_doc_moves_to_placement_home_byte_faithful_with_rekey() {
        let repo = TempDir::new("relocate");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("log", 1, log_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(log_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "log", 1).expect("the log.v1 snapshot");

        let v1 = log_v1_doc(&from);
        assert!(
            v1.starts_with("---\nschema-version: 1\n---\n\n# changelog\n"),
            "the committed doc is stamped v1 with a lowercase H1; got:\n{v1}"
        );
        write_doc(repo.path(), "changelog/changelog.md", &v1);

        // The doc is tracked at its old home (the migration re-keys only what it tracks).
        let mut seed = FileStateRecord::new();
        seed.record(
            "changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        let report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("migration runs");

        // (a) the walk found+moved the doc at the from home; the report names the new home.
        assert_eq!(
            report.migrated,
            vec!["CHANGELOG.md".to_string()],
            "the relocated doc migrates to its placement home: {report:?}"
        );
        assert!(
            report.blocked.is_empty() && report.already_current.is_empty(),
            "never blocked or already-current: {report:?}"
        );

        // (c) the old-home source file is removed.
        assert!(
            !repo.path().join("changelog/changelog.md").exists(),
            "the old-home source is removed after the move"
        );
        // (b) the doc lives at the placement home, byte-faithful: H1 fixed, stamp bumped 1→2,
        //     body preserved.
        let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("relocated home");
        assert!(
            moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
            "the stamp value-bumps 1→2 and the H1 is fixed to `# Changelog`; got:\n{moved}"
        );
        assert!(
            moved.contains("Released 1.0 with the new limiter."),
            "the body prose survives the move; got:\n{moved}"
        );
        assert_conformant_and_stable(&to, &moved);

        // (d) the file-state is re-keyed from → to: the target carries the migrated hash and
        //     the old-home key is dropped (no orphaned baseline entry).
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("CHANGELOG.md"),
            Some(hash_bytes(moved.as_bytes()).as_str()),
            "the target home is baselined at the migrated bytes"
        );
        assert!(
            record.get("changelog/changelog.md").is_none(),
            "the old-home key is dropped (re-keyed, not orphaned)"
        );
    }

    /// Relocation-safety census row — **write-before-remove / abort-strands-neither**
    /// (`design/corpus-migration.md` → Relocation: write-to-`to` precedes remove-`from`, so an
    /// abort between strands neither copy). The move writes the gated v2 bytes to the target
    /// home **first**, then removes the old-home source. When the write to `to` is forced to
    /// fail (its atomic temp-sibling `CHANGELOG.md.tmp` is pre-occupied by a directory, so
    /// [`engine::state::persist`] errors before the rename), the old-home source is **never
    /// removed** — the doc still exists at `from` (never zero copies). Under the *wrong*
    /// (remove-first) ordering the source would already be gone **and** the write failed → the
    /// doc lost entirely; this test fails there, so it pins the ordering. The move never
    /// completed, so the file-state baseline is **not** prematurely re-keyed.
    #[test]
    fn relocation_abort_at_the_write_strands_neither_copy() {
        let repo = TempDir::new("relocate-abort");
        let jigc_root = repo.path().join(".jigc");

        let pack_dir = snapshot_pack("log", 1, log_v1_yaml());
        let pack = crate::pack::FilesystemPack::new(pack_dir.path().to_path_buf());

        let to = v1_schema(log_v2_yaml());
        let from = crate::pack::load_prior_schema(&pack, "log", 1).expect("the log.v1 snapshot");
        let v1 = log_v1_doc(&from);
        write_doc(repo.path(), "changelog/changelog.md", &v1);

        let mut seed = FileStateRecord::new();
        seed.record(
            "changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        // Force the write to the TO home to fail: occupy its atomic temp-sibling
        // `CHANGELOG.md.tmp` with a directory, so `persist` errors before the rename — the
        // move aborts AT the write, BEFORE the source removal (write-before-remove).
        fs::create_dir_all(repo.path().join("CHANGELOG.md.tmp")).expect("occupy temp sibling");

        let result = migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to, 2)]);
        assert!(
            result.is_err(),
            "the aborted write surfaces as an error: {result:?}"
        );

        // WRITE-BEFORE-REMOVE: the failed write to `to` means the source at `from` was NEVER
        // removed — the doc survives at its old home (never zero copies; nothing stranded).
        assert!(
            repo.path().join("changelog/changelog.md").exists(),
            "the from copy survives the aborted write (never zero copies)"
        );
        // The write failed before the rename, so the target home was not created.
        assert!(
            !repo.path().join("CHANGELOG.md").exists(),
            "the aborted write left no partial target"
        );
        // The move never completed, so the baseline is intact — no premature re-key.
        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("changelog/changelog.md"),
            Some(hash_bytes(v1.as_bytes()).as_str()),
            "the from-home baseline is intact (re-key not applied on abort)"
        );
        assert!(
            record.get("CHANGELOG.md").is_none(),
            "no premature re-key to the target home"
        );
    }

    // ---- (h) the REAL shipped changelog v1→v2 relocation over the EMBEDDED pack ----

    /// (M38 inc-5 T1, done-criterion) The **shipped** `changelog` doctype's v1→v2 shape
    /// change, classified over the embedded dev pack: the prior-version snapshot
    /// (`schema-snapshots/changelog.v1.yaml`, folder home `changelog/`, slug H1) diffed
    /// against the current shipped schema (`placement: CHANGELOG.md` + `display-title:
    /// Changelog`) classifies to **exactly** `[Relocated {changelog/ → CHANGELOG.md},
    /// DisplayTitleChanged {Changelog}]`. Both shapes load through [`crate::pack::load_pack_schema`]
    /// so the stamp rides both identically (a value-bump, not a spurious field-add) and the
    /// two KaC sections are byte-identical — the sole diffs are the doctype-level relocation
    /// and H1 re-title. This is the classifier driving the real relocation, not a synthetic
    /// `log` stand-in.
    #[test]
    fn embedded_changelog_v1_to_v2_classifies_exactly_relocated_and_display_title() {
        use engine::packsource::{PackResourceKind, PackSource, ResourceId};
        use engine::schema_diff::{SchemaChange, schema_diff};

        let pack = crate::pack::EmbeddedPack::new();
        let bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("changelog"))
            .expect("the embedded pack ships the changelog schema");
        let current =
            crate::pack::load_pack_schema(&pack, &bytes).expect("current changelog loads");
        let prior = crate::pack::load_prior_schema(&pack, "changelog", 1)
            .expect("the shipped changelog.v1 snapshot loads");

        let diff = schema_diff(&prior, &current);
        assert_eq!(
            diff,
            vec![
                SchemaChange::Relocated {
                    from: "changelog/".to_string(),
                    to: "CHANGELOG.md".to_string(),
                },
                SchemaChange::DisplayTitleChanged {
                    to: "Changelog".to_string(),
                },
            ],
            "the shipped changelog v1→v2 diff is exactly the relocation + the H1 re-title",
        );
    }

    /// (M38 inc-5 T1, done-criterion) A committed **v1-stamped** shipped `changelog`
    /// instance at the prior folder home (`changelog/changelog.md`, the docs-root-resolved
    /// `docs/changelog/changelog.md` old canonical) with a lowercase `# changelog` H1
    /// **relocates** to the literal root `CHANGELOG.md` through `migrate_committed_corpus`
    /// over the **embedded** pack (the same core the verb runs): byte-faithful — the H1
    /// fixed to `# Changelog`, the stamp value-bumped `1→2`, the two KaC section headers
    /// preserved; the old-home source removed; the file-state re-keyed. jigc's own repo has
    /// no managed changelog instance, so the proof rides this fixture.
    #[test]
    fn embedded_changelog_relocates_committed_doc_to_root_byte_faithful() {
        use engine::packsource::{PackResourceKind, PackSource, ResourceId};
        use engine::schema::SCHEMA_VERSION_FIELD;

        let repo = TempDir::new("changelog-relocate");
        let jigc_root = repo.path().join(".jigc");
        let pack = crate::pack::EmbeddedPack::new();

        let bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("changelog"))
            .expect("the embedded pack ships the changelog schema");
        let to = crate::pack::load_pack_schema(&pack, &bytes).expect("current changelog loads");
        let from = crate::pack::load_prior_schema(&pack, "changelog", 1)
            .expect("the shipped changelog.v1 snapshot loads");

        // A committed v1-stamped changelog at the OLD folder home, lowercase-slug H1 + the
        // two empty KaC section headers — the byte form a real pre-relocation instance has.
        let v1 = render(
            &from,
            &Instance {
                title: "changelog".to_string(),
                sections: vec![
                    SectionContent {
                        id: "meta".to_string(),
                        fields: vec![Field {
                            key: SCHEMA_VERSION_FIELD.to_string(),
                            value: Value::Scalar("1".to_string()),
                        }],
                        ..Default::default()
                    },
                    SectionContent {
                        id: "unreleased-changes".to_string(),
                        ..Default::default()
                    },
                    SectionContent {
                        id: "releases".to_string(),
                        ..Default::default()
                    },
                ],
            },
        );
        assert!(
            v1.starts_with("---\nschema-version: 1\n---\n\n# changelog\n"),
            "the committed doc is v1-stamped with a lowercase H1; got:\n{v1}"
        );
        write_doc(repo.path(), "changelog/changelog.md", &v1);

        let mut seed = FileStateRecord::new();
        seed.record(
            "changelog/changelog.md".to_string(),
            hash_bytes(v1.as_bytes()),
        );
        fs::create_dir_all(&jigc_root).expect("mk .jigc");
        seed.save(&jigc_root).expect("seed the baseline");

        let report =
            migrate_committed_corpus(&pack, repo.path(), &jigc_root, &[migration(to.clone(), 2)])
                .expect("migration runs");

        assert_eq!(
            report.migrated,
            vec!["CHANGELOG.md".to_string()],
            "the shipped changelog relocates to its root placement home: {report:?}"
        );
        assert!(
            !repo.path().join("changelog/changelog.md").exists(),
            "the old-home source is removed after the move"
        );
        let moved = fs::read_to_string(repo.path().join("CHANGELOG.md")).expect("relocated home");
        assert!(
            moved.starts_with("---\nschema-version: 2\n---\n\n# Changelog\n"),
            "the stamp value-bumps 1→2 and the H1 is fixed to `# Changelog`; got:\n{moved}"
        );
        assert_conformant_and_stable(&to, &moved);

        let record = FileStateRecord::load(&jigc_root).expect("reload baseline");
        assert_eq!(
            record.get("CHANGELOG.md"),
            Some(hash_bytes(moved.as_bytes()).as_str()),
            "the target home is baselined at the migrated bytes"
        );
        assert!(
            record.get("changelog/changelog.md").is_none(),
            "the old-home key is dropped (re-keyed, not orphaned)"
        );
    }
}
