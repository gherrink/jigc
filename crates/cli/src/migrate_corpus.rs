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
//! (config-delta reconciliation): this is a managed v0→v1 *structural* upgrade over N
//! committed instances, **CLI-owned and deterministic** (the determinism boundary — no LLM
//! in the structural path).
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
use crate::pack;
use crate::render;
use anyhow::{Context, Result};
use engine::file_state::{FileStateRecord, hash_bytes};
use engine::schema::{SCHEMA_VERSION_FIELD, Schema, SectionBody};
use engine::schema_diff::{SchemaChange, schema_diff};
use engine::transform::{CorpusDoc, CorpusMigration, DocOutcome, migrate_corpus};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// One doctype's v0→v1 migration job: its prior shape (`from`), its current shape (`to`,
/// stamp-injected), and the doctype's current manifest schema-version (the value the stamp
/// is filled with + the "already current" threshold).
pub(crate) struct DoctypeMigration {
    /// The doctype's id (for diagnostics).
    pub ty: String,
    /// The prior (v0) schema shape the committed corpus was authored against.
    pub from: Schema,
    /// The current (v1) schema shape, with the engine-injected schema-version stamp.
    pub to: Schema,
    /// The doctype's current manifest schema-version (the stamp value + currency floor).
    pub version: u32,
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
pub fn run(cwd: &Path, format: Format) -> ExitCode {
    match migrate_in_repo(cwd) {
        Ok(report) => {
            println!("{}", render::corpus_migration(format, &report));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", render::operational_error(format, &err));
            ExitCode::FAILURE
        }
    }
}

/// Locate the repo + project layer, assemble the frozen persisted doctypes' migration jobs
/// (v0 = the current shape minus the engine stamp; v1 = the stamp-injected shape; version =
/// the doctype's manifest schema-version), and migrate the committed corpus.
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

    let mut doctypes = Vec::new();
    for (ty, to) in schemas {
        // Only the frozen persisted doctypes carry the stamp + a migration target; a
        // transient (location-less) or non-frozen doctype migrates nothing.
        let (Some(_), Some(&version)) = (to.location.as_deref(), versions.get(&ty)) else {
            continue;
        };
        let from = strip_stamp(&to);
        doctypes.push(DoctypeMigration {
            ty,
            from,
            to,
            version,
        });
    }
    // Deterministic doctype order (the committed walk + the fold both consume it in order).
    doctypes.sort_by(|a, b| a.ty.cmp(&b.ty));

    let jigc_root = jigc_home.join(".jigc");
    migrate_committed_corpus(&jigc_home, &jigc_root, &doctypes)
}

/// Migrate the committed corpus under `repo_root` for each [`DoctypeMigration`], per-doc
/// gated and WIP-safe, re-baselining any rewritten doc that already carried a file-state
/// hash. `jigc_root` is the `.jigc/` dir holding the file-state record.
///
/// For each candidate committed doc (one whose schema-version stamp is absent or below the
/// doctype's current version) it computes the **per-doc** change list — the doctype's
/// schema-diff (the live `added-optional-field` stamp + any prose-needing slot), filtered
/// to the changes that doc still needs (a slot already authored is dropped, so an
/// already-authored doc flips cleanly on a re-run). The engine fold applies them, gates each
/// doc on conformance against the v1 schema, and commits only the clean ones; this writes
/// the committed v2 bytes back to disk and flips the file-state baseline.
pub(crate) fn migrate_committed_corpus(
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
        let Some(location) = dt.to.location.as_deref() else {
            continue;
        };
        // The stamp's value comes from the doctype's manifest version: thread it in as the
        // field `default` so the transform's `added-optional-field` branch (which has no
        // value source for a bare `set`-derived field) places it deterministically.
        let transform_to = with_stamp_default(&dt.to, dt.version);
        let fixed = schema_diff(&dt.from, &transform_to);
        for slug in committed_slugs(repo_root, location) {
            let rel_key = format!("{location}{slug}.md");
            let Ok(bytes) = std::fs::read(repo_root.join(&rel_key)) else {
                continue; // read race: skip; the next run re-checks.
            };
            let source = String::from_utf8_lossy(&bytes).into_owned();
            let stamp = read_stamp_from_source(&source);
            if stamp.is_some_and(|s| s >= dt.version) {
                report.already_current.push(rel_key);
                continue;
            }
            let changes = per_doc_changes(&fixed, &source, stamp.is_none());
            if changes.is_empty() {
                // Nothing this doc needs (e.g. a below-version stamp the add-field branch
                // cannot value-bump): leave it, already at its shape.
                report.already_current.push(rel_key);
                continue;
            }
            prepared.push(PreparedDoc {
                rel_key,
                source,
                from: dt.from.clone(),
                to: transform_to.clone(),
                changes,
            });
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
    let mut record_dirty = false;
    for (i, outcome) in result.docs.iter().enumerate() {
        match outcome {
            DocOutcome::Migrated { id, v2 } => {
                engine::state::persist(&repo_root.join(id), v2.as_bytes())
                    .with_context(|| format!("writing the migrated doc {id}"))?;
                // Re-baseline an already-tracked doc so its rewritten bytes are not
                // mis-reported as out-of-band drift (the rollback inventory's file-state
                // re-hash — `corpus-migration.md`). An un-baselined doc stays un-baselined
                // (the migration adopts nothing it didn't already track).
                if record.get(id).is_some() {
                    record.record(id.clone(), hash_bytes(v2.as_bytes()));
                    record_dirty = true;
                }
                report.migrated.push(id.clone());
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
    if record_dirty {
        record
            .save(jigc_root)
            .with_context(|| format!("saving the file-state record at {jigc_root:?}"))?;
    }

    report.migrated.sort();
    report.already_current.sort();
    report.blocked.sort();
    Ok(report)
}

/// One prepared migration job, owning its source + schema pair + per-doc change list (the
/// fold borrows them).
struct PreparedDoc {
    rel_key: String,
    source: String,
    from: Schema,
    to: Schema,
    changes: Vec<SchemaChange>,
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

    /// Build a [`DoctypeMigration`] from an explicit prior shape `from` + the stamp-injected
    /// `to`, at `version`.
    fn migration(from: Schema, to: Schema, version: u32) -> DoctypeMigration {
        DoctypeMigration {
            ty: to.ty.clone(),
            from,
            to,
            version,
        }
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
        let from = strip_stamp(&to);

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

        let report =
            migrate_committed_corpus(repo.path(), &jigc_root, &[migration(from, to.clone(), 1)])
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
        let from = strip_stamp(&to);

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

        let report = migrate_committed_corpus(repo.path(), &jigc_root, &[migration(from, to, 1)])
            .expect("migration runs");

        assert_eq!(
            report.already_current,
            vec!["notes/done-note.md".to_string()]
        );
        assert!(report.migrated.is_empty(), "nothing to migrate");
        let after = fs::read_to_string(repo.path().join("notes/done-note.md")).expect("read");
        assert_eq!(after, stamped, "an already-current doc is byte-untouched");
    }

    /// The combined **stamp-flips-last** case over a synthetic doctype whose v0→v1 adds the
    /// stamp **and** a new required trailing slot (`rationale`). A v0 doc that lacks the slot
    /// migrates to a minted-empty slot that **fails the gate**, so the whole doc rolls back —
    /// it stays byte-identical v0, **unstamped**, routed to the agent. Once the prose is
    /// authored, the same doc migrates clean and the stamp **flips**.
    #[test]
    fn combined_change_stamp_flips_only_after_prose_authored() {
        let repo = TempDir::new("flips-last");
        let jigc_root = repo.path().join(".jigc");

        // v1 adds a NEW required trailing `rationale` slot AND (via injection) the stamp.
        let to = v1_schema(
            b"\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
  - id: rationale
    slot: { hint: \"why\" }
",
        );
        // The genuine v0 prior shape: just `vision` (no rationale, no stamp).
        let from = load_schema(
            b"\
type: memo
location: memos/
id-from: title
sections:
  - id: vision
    slot: { hint: \"v\" }
",
        )
        .expect("v0 memo loads");
        // The intermediate shape an agent authors against (v1 minus the stamp): vision +
        // the now-present rationale, used to render the authored doc.
        let authored_shape = strip_stamp(&to);

        // --- phase 1: the v0 doc lacks rationale → migration blocks, doc stays v0 ---
        let v0 = render(
            &from,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![SectionContent {
                    id: "vision".to_string(),
                    slot: Some("Move the cache.".to_string()),
                    ..Default::default()
                }],
            },
        );
        write_doc(repo.path(), "memos/cache-memo.md", &v0);

        let blocked_report = migrate_committed_corpus(
            repo.path(),
            &jigc_root,
            &[migration(from.clone(), to.clone(), 1)],
        )
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
        // STAMP-FLIPS-LAST: the on-disk doc is byte-identical v0 — unstamped — because the
        // whole scratch (stamp included) rolled back on the failed gate.
        let still_v0 = fs::read_to_string(repo.path().join("memos/cache-memo.md")).expect("read");
        assert_eq!(still_v0, v0, "the blocked doc is byte-identical v0");
        assert!(
            !still_v0.contains("schema-version"),
            "the stamp did NOT flip while the prose is pending; got:\n{still_v0}"
        );

        // --- phase 2: the agent authors the rationale slot → migration flips the stamp ---
        let authored = render(
            &authored_shape,
            &Instance {
                title: "Cache Memo".to_string(),
                sections: vec![
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
            migrate_committed_corpus(repo.path(), &jigc_root, &[migration(from, to.clone(), 1)])
                .expect("re-migration runs");

        assert_eq!(
            flipped_report.migrated,
            vec!["memos/cache-memo.md".to_string()],
            "with the prose authored the doc migrates: {flipped_report:?}",
        );
        assert!(flipped_report.blocked.is_empty(), "no longer blocked");
        let flipped = fs::read_to_string(repo.path().join("memos/cache-memo.md")).expect("read");
        assert!(
            flipped.contains("schema-version: 1"),
            "the stamp flips once the prose is authored and the doc gates clean; got:\n{flipped}"
        );
        assert!(
            flipped.contains("Move the cache.") && flipped.contains("Latency wins."),
            "the authored prose survives; got:\n{flipped}"
        );
        assert_conformant_and_stable(&to, &flipped);
    }
}
