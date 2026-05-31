//! Task/staging state, the per-task working area, base pinning, and the
//! `finalize` transaction.
//!
//! See `design/write-commands.md` (task origination, staging), `design/storage.md`
//! (`.jigc/` layout), and `design/finalize.md` (the seven phases).
//!
//! This module lands **task origination** (`write-commands.md` → Task
//! origination; `storage.md` → The per-task working area / base pinning): minting
//! a task slugs the intent into a frozen content-slug ([`crate::slug::slugify`],
//! empty → the type name), opens the gitignored working area at
//! `.jigc/tasks/<id>/`, and writes a **base-pin** file recording the commit the
//! task started against (full + short SHA). A serial collision — an active task
//! dir of that id already exists — **rejects** (`write-commands.md` → Task-id
//! collision & resume: *never silently suffixed, never silently reused*), surfacing
//! the existing task's status as a routed blocking [`Finding`]; nothing new is
//! created. The numeric `-2`/`-3` suffix is reserved for the post-MVP parallel
//! `fan-out`/`join` case only.
//!
//! The base SHA is supplied by the caller (the CLI reads HEAD via `git rev-parse`
//! — "CLI orchestrates, git executes"); the engine performs no I/O beyond the
//! working-area filesystem and never shells out, so minting is a pure function of
//! (jigc-root, intent, type-name, base) → on-disk effect, golden-testable.

use crate::finding::{Finding, Location, Severity};
use crate::schema::Schema;
use crate::write::{self, Instance, SectionContent};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The base-pin filename inside a task's working area.
const BASE_PIN_FILE: &str = "base.json";

/// The working-area sub-directory holding a task's staged doc instances
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout: a staged instance
/// lives at `.jigc/tasks/<id>/docs/<type>:<slug>.md`).
const DOCS_DIR: &str = "docs";

/// The bare filename of a staged doc instance: the `:`-joined address slug
/// (`<type>:<slug>.md`), the on-disk form the working-area layout pins
/// (`DECISIONS.md` 2026-05-31 → Task working-area on-disk layout).
fn instance_filename(type_name: &str, slug: &str) -> String {
    format!("{type_name}:{slug}.md")
}

/// The on-disk path a staged `<type>:<slug>` instance lives at within `task_dir`:
/// `<task_dir>/docs/<type>:<slug>.md`.
pub fn instance_path(task_dir: &Path, type_name: &str, slug: &str) -> PathBuf {
    task_dir
        .join(DOCS_DIR)
        .join(instance_filename(type_name, slug))
}

/// Build the **empty** in-memory instance for `schema` — the canonical template the
/// workflow provisions: the H1 title is the task-derived `slug`, and every schema
/// section is left content-free (no field values, no slot prose, no items). Rendered
/// through [`crate::write::render`], this yields the full skeleton — front-matter (no
/// values), the `# <slug>` H1, and every `## Heading` with an empty slot — the bytes
/// the agent then fills slot-by-slot (`design/write-commands.md` → Instance
/// provisioning: "the agent only fills slots").
fn empty_instance(schema: &Schema, slug: &str) -> Instance {
    Instance {
        title: slug.to_string(),
        sections: schema
            .sections
            .iter()
            .map(|s| SectionContent {
                id: s.id.clone(),
                ..Default::default()
            })
            .collect(),
    }
}

/// **Provision** a workflow-provisioned empty doc instance into the task working area
/// (`design/write-commands.md` → Instance provisioning → Workflow-provisioned;
/// `implementation/parsing.md` → The write pipeline → "Writes land in the task working
/// area"). Materializes `<task_dir>/docs/<type>:<slug>.md` with the canonical
/// empty-template bytes ([`crate::write::render`] of [`empty_instance`]), atomically
/// (temp + rename), and returns its path. The `docs/` dir is created on demand.
///
/// Pure working-area filesystem effect — no verbs, no git. The `type` name is read
/// from the schema; the `slug` is the task-derived id (`commit:<task-id>`).
pub fn provision_doc(task_dir: &Path, schema: &Schema, slug: &str) -> std::io::Result<PathBuf> {
    let path = instance_path(task_dir, &schema.ty, slug);
    let bytes = write::render(schema, &empty_instance(schema, slug));
    write_atomic(&path, bytes.as_bytes())?;
    Ok(path)
}

/// **Copy-in on first touch** of a pre-existing managed doc into the task working area
/// (`implementation/parsing.md` → The write pipeline → "an existing doc is copied in
/// (base-pinned) on first touch"; `DECISIONS.md` 2026-05-31 → copy-in-on-first-touch).
/// Applies the *only* permitted first-touch canonicalization
/// ([`crate::write::first_touch_canonicalize`] — BOM strip + single trailing newline,
/// EOL-preserving, everything else byte-for-byte) and persists the result atomically at
/// `<task_dir>/docs/<type>:<slug>.md`, returning its path. The committed source file is
/// untouched (this writes only the working copy).
pub fn copy_in(
    task_dir: &Path,
    type_name: &str,
    slug: &str,
    source: &str,
) -> std::io::Result<PathBuf> {
    let path = instance_path(task_dir, type_name, slug);
    let canonical = write::first_touch_canonicalize(source);
    write_atomic(&path, canonical.as_bytes())?;
    Ok(path)
}

/// **Atomic persist** of an edited buffer to a working-area doc path
/// (`implementation/parsing.md` → The write pipeline → "Atomic on disk — write temp +
/// rename"). Writes `bytes` to a sibling temp file, then `rename`s it over `path`, so a
/// reader never observes a partial write and no temp residue survives a successful
/// persist. The parent dir is created on demand.
pub fn persist(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    write_atomic(path, bytes)
}

/// Write `bytes` to `path` via the temp-file + `rename` dance (the atomic-on-disk
/// primitive shared by [`provision_doc`], [`copy_in`], and [`persist`]). The temp file
/// is a sibling (`<filename>.tmp`) so the `rename` stays on the same filesystem (atomic);
/// it is removed on a write failure and consumed by the rename on success — never left
/// behind. The parent dir is created on demand.
fn write_atomic(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = temp_sibling(path);
    if let Err(err) = std::fs::write(&tmp, bytes) {
        let _ = std::fs::remove_file(&tmp);
        return Err(err);
    }
    if let Err(err) = std::fs::rename(&tmp, path) {
        let _ = std::fs::remove_file(&tmp);
        return Err(err);
    }
    Ok(())
}

/// The sibling temp path for an atomic write of `path` — its filename with a `.tmp`
/// suffix (same directory, so `rename` is intra-filesystem and atomic).
fn temp_sibling(path: &Path) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(".tmp");
    match path.parent() {
        Some(parent) => parent.join(name),
        None => PathBuf::from(name),
    }
}

/// The base commit a task was started against: the full 40-char SHA and the
/// abbreviated short SHA, both as `git rev-parse` reports them.
///
/// Operating a task whose base ≠ the current checkout is detected and routed
/// (`storage.md` → A task is pinned to its base); this is the pinned value the
/// CLI compares HEAD against on resume.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BasePin {
    /// The full commit SHA HEAD pointed at when the task was minted.
    pub sha: String,
    /// The abbreviated short SHA (for human-facing divergence messages).
    pub short: String,
}

impl BasePin {
    /// A base pin from a full and short SHA.
    pub fn new(sha: impl Into<String>, short: impl Into<String>) -> Self {
        Self {
            sha: sha.into(),
            short: short.into(),
        }
    }
}

/// A freshly minted task: its frozen slug `id`, its working-area `dir`, and the
/// `base` it was pinned to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MintedTask {
    /// The frozen content-slug id (slugged from the intent, type-name fallback).
    pub id: String,
    /// The per-task working area, `<jigc_root>/tasks/<id>/`.
    pub dir: PathBuf,
    /// The base commit the task is pinned to.
    pub base: BasePin,
}

/// Mint a task: slug the `intent` (empty → `type_name`), reject on a serial
/// collision with an existing active task, else open `<jigc_root>/tasks/<id>/`
/// and write the base-pin file capturing `base`.
///
/// `jigc_root` is the project's `.jigc/` home (a temp root under test). On
/// success the working-area directory and its `base.json` exist on disk. On a
/// serial collision the returned [`Finding`] is `Severity::Blocking`, carries the
/// existing task's status in its message, and a `route` directing the agent to
/// resume or discard — nothing new is created.
pub fn mint_task(
    jigc_root: &Path,
    intent: &str,
    type_name: &str,
    base: BasePin,
) -> Result<MintedTask, Finding> {
    let id = mint_id(intent, type_name);
    let dir = jigc_root.join("tasks").join(&id);

    // Serial collision: an active task dir of that id already exists → reject,
    // surfacing its status, never silently suffixed or reused.
    if dir.exists() {
        return Err(collision_finding(&id));
    }

    std::fs::create_dir_all(&dir).map_err(|err| io_finding(&id, "open the working area", &err))?;

    let pin_path = dir.join(BASE_PIN_FILE);
    let body = render_base_pin(&base);
    std::fs::write(&pin_path, body).map_err(|err| io_finding(&id, "write the base pin", &err))?;

    Ok(MintedTask { id, dir, base })
}

/// Slug the id-source into the task id, applying the empty → type-name fallback.
///
/// The pure normalization is [`crate::slug::slugify`]; the fallback is the mint
/// site's concern (only the caller knows the type name), per the slug rule
/// (`DECISIONS.md` 2026-05-31 → Slug / minting normalization).
fn mint_id(intent: &str, type_name: &str) -> String {
    let slug = crate::slug::slugify(intent);
    if slug.is_empty() {
        crate::slug::slugify(type_name)
    } else {
        slug
    }
}

/// Render the base-pin file body — the frozen on-disk form (golden-locked).
fn render_base_pin(base: &BasePin) -> String {
    // Pretty JSON with a trailing newline; field order is the struct order
    // (`sha` then `short`), pinned by the golden.
    let mut s = serde_json::to_string_pretty(base).expect("BasePin serializes");
    s.push('\n');
    s
}

/// The serial-collision block: a blocking finding naming the existing task and
/// routing the agent to resume or discard (`write-commands.md` → Task-id collision
/// & resume).
fn collision_finding(id: &str) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "task.serial-collision".to_string(),
        message: format!("task `{id}` is already active"),
        location: Some(Location {
            address: Some(format!("task:{id}")),
            line: 1,
            col: 1,
        }),
        route: Some(format!(
            "resume with `jigc start --task {id}` or abandon with `jigc task discard {id}`"
        )),
    }
}

/// A blocking finding for a working-area I/O failure during minting.
fn io_finding(id: &str, doing: &str, err: &std::io::Error) -> Finding {
    Finding {
        severity: Severity::Blocking,
        code: "task.working-area-io".to_string(),
        message: format!("could not {doing} for task `{id}`: {err}"),
        location: Some(Location {
            address: Some(format!("task:{id}")),
            line: 1,
            col: 1,
        }),
        route: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A throwaway directory that removes itself on drop — keeps mint tests off
    /// any real `.jigc/` tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-mint-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp root");
            TempRoot(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// The done-criterion: minting "Add rate limiter" opens
    /// `.jigc/tasks/add-rate-limiter/` with a base-pin recording the supplied
    /// HEAD; a second mint of the same slug rejects with a route-bearing blocking
    /// finding naming the existing task and creates nothing new.
    #[test]
    fn mint_creates_task_area_and_base_pin() {
        let root = TempRoot::new("create");
        let base = BasePin::new("0123456789abcdef0123456789abcdef01234567", "0123456");

        let minted = mint_task(root.path(), "Add rate limiter", "commit", base.clone())
            .expect("first mint succeeds");

        assert_eq!(minted.id, "add-rate-limiter");
        let dir = root.path().join("tasks").join("add-rate-limiter");
        assert_eq!(minted.dir, dir);
        assert!(dir.is_dir(), "working area must exist");

        let pin = std::fs::read_to_string(dir.join(BASE_PIN_FILE)).expect("base pin written");
        // Golden over the frozen base-pin byte form for a fixed SHA.
        insta::assert_snapshot!(pin, @r#"
        {
          "sha": "0123456789abcdef0123456789abcdef01234567",
          "short": "0123456"
        }
        "#);
        // The pin round-trips back to the supplied base.
        let back: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(back, base);

        // Second mint of the same slug → serial reject, nothing new created.
        let before = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        let err = mint_task(root.path(), "Add rate limiter", "commit", base.clone())
            .expect_err("re-mint of the same slug rejects");
        assert_eq!(err.severity, Severity::Blocking);
        assert_eq!(err.code, "task.serial-collision");
        assert!(
            err.message.contains("add-rate-limiter"),
            "block must name the existing task: {err:?}"
        );
        assert!(
            err.route.is_some(),
            "serial collision carries a resume/discard route"
        );
        let after = std::fs::read_dir(root.path().join("tasks"))
            .expect("tasks dir")
            .count();
        assert_eq!(before, after, "no second dir created on collision");
    }

    /// An intent that normalizes to nothing falls back to the type name.
    #[test]
    fn empty_intent_falls_back_to_type_name() {
        let root = TempRoot::new("fallback");
        let base = BasePin::new("a".repeat(40), "aaaaaaa");

        let minted =
            mint_task(root.path(), "!!!___---", "adr", base).expect("fallback mint succeeds");
        assert_eq!(
            minted.id, "adr",
            "stripped-to-empty intent uses the type name"
        );
        assert!(root.path().join("tasks").join("adr").is_dir());
    }

    /// A stripped-whitespace intent also falls back.
    #[test]
    fn whitespace_intent_falls_back_to_type_name() {
        assert_eq!(mint_id("   ", "commit"), "commit");
        assert_eq!(mint_id("", "commit"), "commit");
    }

    proptest! {
        /// For any non-colliding, non-empty-slug intent, the minted id is exactly
        /// `slugify(intent)` — minting layers the fallback over the pure rule,
        /// nothing else.
        #[test]
        fn mint_id_is_slugify_for_non_empty(intent in "[A-Za-z0-9 _-]{1,40}") {
            let slug = crate::slug::slugify(&intent);
            prop_assume!(!slug.is_empty());
            prop_assert_eq!(mint_id(&intent, "commit"), slug);
        }
    }

    const COMMIT_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/commit.yaml");

    fn commit_schema() -> Schema {
        crate::schema::load_schema(COMMIT_YAML).expect("commit.yaml loads")
    }

    /// The done-criterion (`DECISIONS.md` 2026-05-31 → inc-4 working-area layout).
    /// Provisioning opens `.jigc/tasks/<id>/docs/commit:<id>.md` whose bytes equal
    /// `write::render` of the empty commit instance (golden); a subsequent atomic
    /// persist of an edited buffer replaces those bytes byte-for-byte, leaving **no**
    /// temp-file residue.
    #[test]
    fn working_area_provisions_and_persists_atomically() {
        let root = TempRoot::new("working-area");
        let schema = commit_schema();
        let task_dir = root.path().join("tasks").join("add-rate-limiter");

        // Provision: the empty commit template lands at docs/commit:<id>.md.
        let path =
            provision_doc(&task_dir, &schema, "add-rate-limiter").expect("provision succeeds");
        assert_eq!(
            path,
            task_dir.join("docs").join("commit:add-rate-limiter.md"),
            "provisioned at the `:`-joined address slug under docs/"
        );
        assert!(path.is_file(), "provisioned file must exist");

        let provisioned = std::fs::read_to_string(&path).expect("read provisioned");
        // The provisioned bytes ARE `write::render` of the empty commit instance.
        let expected = write::render(&schema, &empty_instance(&schema, "add-rate-limiter"));
        assert_eq!(
            provisioned, expected,
            "provisioned bytes equal write::render of the empty commit instance"
        );
        // Golden over the provisioned empty-commit byte form.
        insta::assert_snapshot!("provisioned_empty_commit", provisioned);

        // No temp residue from the atomic provision.
        let tmp = task_dir.join("docs").join("commit:add-rate-limiter.md.tmp");
        assert!(!tmp.exists(), "no leftover temp path after provisioning");

        // Persist an edited buffer: atomic temp+rename replaces the file byte-for-byte.
        let edited = "---\ntype: feat\n---\n\n# add-rate-limiter\n\n## Summary\n\nLimit at the gateway.\n\n## Body\n\n## Trailers\n";
        persist(&path, edited.as_bytes()).expect("persist succeeds");

        let after = std::fs::read_to_string(&path).expect("read persisted");
        assert_eq!(after, edited, "persisted bytes equal the input buffer");
        assert!(
            !tmp.exists(),
            "no leftover temp path after the atomic persist"
        );
    }

    /// Copy-in on first touch persists a pre-existing managed doc into the working
    /// area, applying the only first-touch canonicalization (BOM strip + single
    /// trailing newline, EOL-preserving) and leaving everything else byte-for-byte —
    /// the committed source is never the thing written. A BOM-prefixed, double-trailing
    /// -newline source lands canonicalized; an already-canonical source is byte-stable.
    #[test]
    fn copy_in_canonicalizes_on_first_touch() {
        let root = TempRoot::new("copy-in");
        let task_dir = root.path().join("tasks").join("supersede");

        let source =
            "\u{feff}---\nstatus: accepted\n---\n\n# Rate-limit\n\n## Context\n\nForces.\n\n\n";
        let path = copy_in(&task_dir, "adr", "rate-limit", source).expect("copy-in succeeds");

        assert_eq!(
            path,
            task_dir.join("docs").join("adr:rate-limit.md"),
            "copied in at the `:`-joined address slug under docs/"
        );
        let landed = std::fs::read_to_string(&path).expect("read copied-in");
        // BOM stripped, trailing newlines collapsed to exactly one; interior intact.
        assert_eq!(
            landed, "---\nstatus: accepted\n---\n\n# Rate-limit\n\n## Context\n\nForces.\n",
            "first-touch canonicalization: BOM strip + single trailing newline only"
        );
        assert_eq!(
            landed,
            write::first_touch_canonicalize(source),
            "copy-in IS first_touch_canonicalize of the source"
        );
    }
}
