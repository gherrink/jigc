//! `jigc describe` — the self-description surface (M11 Increment 2, T2).
//!
//! Locate → build the **pack-only** resolved definitions → assemble → (the
//! dispatch then renders the projection as free prose through the global
//! `--format`). Mirrors the `run_ingest` / `run_orient` shape: locate the repo +
//! project layer, [`make_pack`], then build the resolved definitions **pack-only**
//! (the *unfiltered* workflow set — every workflow, not the
//! `creates-task && selectable` catalog — plus the full doctype set + the command
//! catalogs of **every** constituent pack that ships one, since composition resolves
//! each workflow's command-refs against its own origin pack) and hand them to the
//! engine's [`Description::assemble`] whole-menu projection assembler
//! (`design/introspection.md` → Command surface). *Pack-only* here means **outside
//! the project/team cascade**, not "one pack": the pack-set may be composed.
//!
//! Read-only by construction: it locates, reads the cascade-resolved definitions,
//! and assembles — it composes nothing, mints nothing, and writes nothing. The
//! definition reads route through the resolved cascade ([`CascadeDefs`]), so a
//! project whole-file shadow of a workflow/doctype wins on the projection exactly as
//! it wins on compose: describe **reflects the resolved cascade** and cannot drift
//! from it (`overrides.md` → Authored metadata on a definition resolves by whole-file
//! shadow; `worked-examples.md` → flow 14). The project layer is also the "is this
//! project set up" setup gate, exactly as `jigc ingest` requires it.

use anyhow::Result;
use std::path::{Path, PathBuf};

use engine::compose::{CommandCatalog, WorkflowDef, load_workflow_def};
use engine::finding::Finding;
use engine::introspect::{DefinitionKind, Description};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::Schema;

use crate::pack::make_pack;
use crate::start::{CascadeDefs, load_catalog, resolve_severity_cascade};

/// Which **kinds of entry** the menu returns — the `--workflows` / `--doctypes` /
/// `--commands` selection (`design/introspection.md` → Command surface).
///
/// The filter selects **membership**, never prose: describe's prose tier is fenced
/// non-contractual by design, so a selected entry reads exactly as it reads in the
/// whole menu, and an unselected one is simply absent. It is applied to the assembled
/// projection, so the prose surface and the `--format json` envelope return the same
/// membership by construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Kinds {
    /// Return the workflow entries.
    workflows: bool,
    /// Return the doctype entries.
    doctypes: bool,
    /// Return the command-ref entries.
    commands: bool,
}

impl Kinds {
    /// The selection the three flags express. **No flag selects everything** — the
    /// whole menu stays the default, so `jigc describe` is byte-unchanged; a flag
    /// narrows, it never turns a full tour into an empty one.
    pub(crate) fn from_flags(workflows: bool, doctypes: bool, commands: bool) -> Self {
        if !(workflows || doctypes || commands) {
            return Self {
                workflows: true,
                doctypes: true,
                commands: true,
            };
        }
        Self {
            workflows,
            doctypes,
            commands,
        }
    }

    /// Keep only the selected entries of an assembled projection.
    fn select(self, description: Description) -> Description {
        let Description {
            schema_version,
            mut definitions,
            mut commands,
        } = description;
        definitions.retain(|definition| match definition.kind {
            DefinitionKind::Workflow => self.workflows,
            DefinitionKind::Doctype => self.doctypes,
        });
        if !self.commands {
            commands.clear();
        }
        Description {
            schema_version,
            definitions,
            commands,
        }
    }
}

/// Assemble the [`Description`] projection for the repo `describe` is run from —
/// **through the resolved cascade**, narrowed to the requested [`Kinds`]. Locates the
/// repo + project layer (the setup gate), resolves the cascade, reads the unfiltered
/// workflows + all doctypes **layer-aware** (a project whole-file shadow wins) + the
/// command catalog, and runs the engine's [`Description::assemble`]. Presentation-free
/// — the dispatch maps the result to the free-prose surface via `Format → render`.
pub(crate) fn run(cwd: &Path, kinds: Kinds) -> Result<Description> {
    let repo_root = require_project_layer(cwd)?;
    let project_config = repo_root.join(".jigc").join("config");
    let pack = make_pack()?;
    let pack = pack.as_ref();

    // The resolved cascade's by-id `file_owner` surface is what makes the projection
    // cascade-reflecting: it routes each definition read to the highest-precedence
    // layer that owns the id (a project `workflows/`/`schemas/` shadow wins at file
    // granularity, no field-merge).
    let resolved = resolve_severity_cascade(pack, &project_config)?;
    let defs = CascadeDefs::new(&resolved, &project_config);

    // The unfiltered workflow set — every workflow, not the `creates-task &&
    // selectable` catalog (`introspection.md` → enumeration is over the unfiltered
    // set). Each id paired with its cascade-resolved definition, so the assembler
    // reads the authored `description:` / `usage:` fields the resolved layer carries.
    let workflows: Vec<(String, WorkflowDef, Option<String>)> = load_workflow_defs(pack, &defs)?;
    let schemas: Vec<(Schema, Option<String>)> = load_schemas(pack, &defs)?;
    let catalogs = load_catalogs(pack)?;

    Ok(kinds.select(Description::assemble(
        workflows
            .iter()
            .map(|(id, def, origin)| (id.as_str(), def, origin.as_deref())),
        schemas
            .iter()
            .map(|(schema, origin)| (schema, origin.as_deref())),
        catalogs
            .iter()
            .map(|(pack_id, cat)| (pack_id.as_str(), cat)),
    )))
}

/// The command catalog of **every** pack in the composed set that ships one, paired
/// with that pack's `pack-id` — the union the projection carries, attributed.
///
/// Not the precedence winner alone: `CompositePack::read` resolves `config/commands`
/// winner-take-all whole-file, but composition never uses that resolution — a
/// workflow's `{{cli.<id>}}` refs resolve against **its own** origin pack's catalog
/// (`start.rs` → `origin_pack(Workflows, …)` → [`load_catalog`]; `multi-pack.md` →
/// Pack-local body-reference resolution). So under `[dev ▸ methodology]` every
/// methodology command-ref is genuinely composed and reachable while the
/// winner-take-all read left eleven of them on no menu at all — law 2, on the surface
/// that exists to name what is available (M49 Increment 11 / T6).
///
/// Enumerated with [`PackSource::origin_packs`], the same accessor the pack-load
/// freeze/front-matter fences use to reach **every** manifest-shipping constituent
/// rather than the winner, in precedence order.
fn load_catalogs(pack: &dyn PackSource) -> Result<Vec<(String, CommandCatalog)>> {
    let owners = pack.origin_packs(PackResourceKind::Config, &ResourceId::from("commands"));
    if owners.is_empty() {
        // `origin_packs` is empty exactly when no constituent's `read` succeeds, so the
        // composite read faults too: surface that fault verbatim rather than degrading a
        // missing catalog into a silently empty menu.
        load_catalog(pack)?;
    }
    owners
        .into_iter()
        .map(|owner| Ok((owner.own_pack_id(), load_catalog(owner)?)))
        .collect()
}

/// Load every workflow definition through the cascade — the **unfiltered** set (every
/// workflow id the pack lists, read layer-aware via [`CascadeDefs::read_workflow`] so
/// a project shadow wins, then parsed for its front-matter), each paired with the
/// [`CascadeDefs::origin_pack_id`] of whatever actually provided it. The assembler skips
/// any that carry neither authored field, so no filtering happens here.
fn load_workflow_defs(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
) -> Result<Vec<(String, WorkflowDef, Option<String>)>> {
    let mut out = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = defs.read_workflow(pack, id.as_str())?;
        let def = load_workflow_def(&bytes).map_err(finding_to_err)?;
        let origin = defs.origin_pack_id(pack, PackResourceKind::Workflows, id.as_str());
        out.push((id.as_str().to_owned(), def, origin));
    }
    Ok(out)
}

/// Load every doctype schema through the cascade — the full doctype set the projection
/// narrates, read layer-aware via [`CascadeDefs::all_schemas`] so a project shadow
/// wins (the engine stays domain-empty; the CLI feeds the cascade-resolved pack in) —
/// each paired with the origin that provided it.
///
/// The attribution is keyed by the **resource id** a schema is read from, and paired to
/// the loaded schema by its `ty`: everything that resolves a doctype schema addresses it
/// by `<ty>.yaml` ([`CascadeDefs::schema`], [`crate::start::resolved_schema`]), so the two
/// agree for every schema those surfaces can reach. A schema file whose declared `ty`
/// diverges from its stem therefore resolves to **no attribution** rather than to the
/// attribution of a neighbouring id — the safe half of the failure, and the only one
/// worth having: an unattributed entry says nothing, a misattributed one lies.
fn load_schemas(
    pack: &dyn PackSource,
    defs: &CascadeDefs<'_>,
) -> Result<Vec<(Schema, Option<String>)>> {
    let origins: std::collections::BTreeMap<String, Option<String>> = pack
        .list(PackResourceKind::Schemas)
        .into_iter()
        .map(|id| {
            let origin = defs.origin_pack_id(pack, PackResourceKind::Schemas, id.as_str());
            (id.as_str().to_owned(), origin)
        })
        .collect();
    Ok(defs
        .all_schemas(pack)?
        .into_values()
        .map(|schema| {
            let origin = origins.get(&schema.ty).cloned().flatten();
            (schema, origin)
        })
        .collect())
}

/// Locate the repo root and its `.jigc/config/` project layer — the same setup gate
/// `jigc ingest` uses. Errors with routed messages when the repo or the project layer
/// is absent.
fn require_project_layer(cwd: &Path) -> Result<PathBuf> {
    // The `.jigc/` project layer binds to jigc_home (the main checkout), so a worktree
    // resolves the one shared layer (M31 Inc 2 / WF3). `describe` reads only the cascade
    // + pack, so jigc_home is the single base it needs (no git, no worktree code).
    let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
    if !jigc_home.join(".jigc").join("config").is_dir() {
        return Err(crate::locate::not_set_up());
    }
    Ok(jigc_home)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route — the
/// same envelope the rest of the CLI surfaces blocking findings through.
fn finding_to_err(finding: Finding) -> anyhow::Error {
    // The locus rides between the message and the route, exactly as it does on the findings
    // surface ([`crate::render::finding_line`]) — one funnel must not describe a break in
    // fewer facts than another (M49 Increment 8 / T4).
    let at = crate::render::finding_locus(&finding)
        .map(|locus| format!("\n  at: {locus}"))
        .unwrap_or_default();
    let route = finding
        .route
        .map(|r| format!("\n  route: {r}"))
        .unwrap_or_default();
    anyhow::anyhow!("{}{at}{route}", finding.message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-describe-{tag}-{}-{:?}",
                std::process::id(),
                engine::tempname::unique_nanos(),
            );
            path.push(unique);
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

    /// Mark `root` as a git repo + project layer — the setup gate `run` requires.
    /// Returns the `.jigc/config/` project layer dir the shadows are authored under.
    fn set_up_repo(root: &Path) -> PathBuf {
        fs::create_dir_all(root.join(".git")).expect("create .git marker");
        let config = root.join(".jigc").join("config");
        fs::create_dir_all(&config).expect("create project layer");
        config
    }

    /// The prose narrating a given definition id, or `None` if it is not narrated.
    fn prose_for<'a>(description: &'a Description, id: &str) -> Option<&'a str> {
        description
            .definitions
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.prose.as_str())
    }

    /// T3 done-criterion — a project `workflows/single-task.yaml` whole-file shadow
    /// carrying an edited `usage:` wins on the describe projection: the narration for
    /// `single-task` carries the PROJECT prose and NOT the pack prose, while another
    /// shipped definition (`adr`) still narrates its PACK prose unchanged. The
    /// override CHANGED the output — a "describe mentions single-task" assertion would
    /// be masking; what is asserted is the project string in and the pack string out.
    #[test]
    fn describe_reflects_the_project_workflow_shadow() {
        let repo = TempDir::new("shadow");
        let project_config = set_up_repo(repo.path());

        // The verbatim pack `usage:` clause for single-task — the string that must be
        // GONE once the project shadow wins.
        let pack_usage = "the work is one coherent change you can hold in your head";
        // The project shadow's edited `usage:` — the string that must APPEAR.
        let project_usage = "you want to prove the cascade visibly reflects in describe";

        let workflows = project_config.join("workflows");
        fs::create_dir_all(&workflows).expect("mk workflows shadow dir");
        fs::write(
            workflows.join("single-task.yaml"),
            format!(
                "---\nwhen: a scoped change\ndescription: An end-to-end scoped change.\nusage: {project_usage}\ncreates-task: true\n---\n{{{{ include: step:noop }}}}\n"
            ),
        )
        .expect("write workflow shadow");

        let description = run(repo.path(), Kinds::from_flags(false, false, false))
            .expect("describe runs over the shadowed cascade");

        let single_task = prose_for(&description, "single-task")
            .expect("single-task is still narrated through the cascade");
        assert!(
            single_task.contains(project_usage),
            "the project shadow's usage must win in the projection; got: {single_task:?}"
        );
        assert!(
            !single_task.contains(pack_usage),
            "the pack usage must NOT survive once the project shadows the definition (whole-file replace); got: {single_task:?}"
        );

        // An unshadowed definition keeps its pack prose — the shadow is inert for ids
        // the project does not own.
        let adr = prose_for(&description, "adr").expect("adr is narrated from the pack");
        assert!(
            adr.contains("dated architectural decision record"),
            "the unshadowed adr doctype must keep its pack prose; got: {adr:?}"
        );
    }

    /// M43 T6 — describe over a pack with a `selectable: false` workflow prints
    /// that workflow's suppressed reason in its entry (`surface-contract.md` →
    /// The suppression fence, last sentence). A project shadow hides
    /// `single-task` with a declared `suppressed:` block; the projection's entry
    /// must say it is hidden from the router catalog and carry the reason —
    /// while an unsuppressed definition's narration stays clause-free (the
    /// omitting context is inert).
    #[test]
    fn describe_prints_the_suppressed_reason_for_a_hidden_workflow() {
        let repo = TempDir::new("suppressed");
        let project_config = set_up_repo(repo.path());

        let reason = "held back while the shadow trial runs";
        let workflows = project_config.join("workflows");
        fs::create_dir_all(&workflows).expect("mk workflows shadow dir");
        fs::write(
            workflows.join("single-task.yaml"),
            format!(
                "---\nwhen: a scoped change\ndescription: An end-to-end scoped change.\nusage: the work is one coherent change.\ncreates-task: true\nselectable: false\nsuppressed:\n  reason: {reason}\n  expires: the shadow trial concludes\n---\n{{{{ include: step:noop }}}}\n"
            ),
        )
        .expect("write hidden workflow shadow");

        let description = run(repo.path(), Kinds::from_flags(false, false, false))
            .expect("describe runs over the hidden shadow");

        let single_task = prose_for(&description, "single-task")
            .expect("the hidden workflow is still narrated (describe is the unfiltered set)");
        assert!(
            single_task.contains("hidden from the router catalog"),
            "the entry must say the workflow is hidden from the router catalog; got: {single_task:?}"
        );
        assert!(
            single_task.contains(reason),
            "the entry must carry the declared suppression reason; got: {single_task:?}"
        );

        // The omitting context: an unsuppressed pack definition narrates with NO
        // hidden clause — the projection change is scoped to suppressed workflows.
        let adr = prose_for(&description, "adr").expect("adr is narrated from the pack");
        assert!(
            !adr.contains("hidden from the router catalog"),
            "an unsuppressed definition must not gain the hidden clause; got: {adr:?}"
        );
    }

    /// A both-fields-absent project shadow leaves that definition NOT narrated:
    /// skip-on-absent holds THROUGH the cascade (the resolved definition is what the
    /// assembler reads, so a shadow that strips both authored fields removes the
    /// narration the pack definition would have produced).
    #[test]
    fn describe_skip_on_absent_holds_through_the_shadow() {
        let repo = TempDir::new("absent");
        let project_config = set_up_repo(repo.path());

        let workflows = project_config.join("workflows");
        fs::create_dir_all(&workflows).expect("mk workflows shadow dir");
        // A whole-file shadow with NEITHER authored field — the resolved single-task
        // carries no description/usage, so the assembler skips it.
        fs::write(
            workflows.join("single-task.yaml"),
            "---\nwhen: a scoped change\ncreates-task: true\n---\n{{ include: step:noop }}\n",
        )
        .expect("write field-stripped shadow");

        let description = run(repo.path(), Kinds::from_flags(false, false, false))
            .expect("describe runs over the stripped shadow");

        assert!(
            prose_for(&description, "single-task").is_none(),
            "a both-fields-absent shadow must leave single-task un-narrated (skip-on-absent through the cascade)",
        );
        // The pack-prose proof: an unshadowed definition is still narrated, so the
        // skip above is the shadow's doing, not describe narrating nothing at all.
        assert!(
            prose_for(&description, "adr").is_some(),
            "an unshadowed definition is still narrated (the skip is the shadow's effect)",
        );
    }
}
