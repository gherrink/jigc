//! `jigc start "<intent>"` — task minting + composition (the front door).
//!
//! The CLI reads HEAD (`git rev-parse`) and hands the engine an explicit base
//! SHA; the engine slugs the intent, opens `.jigc/tasks/<id>/`, and writes the
//! base pin (`design/write-commands.md` → Task origination; `design/storage.md`
//! → The per-task working area / base pinning). "CLI orchestrates, git executes"
//! ([storage.md](../../../design/storage.md) → CLI and git): the git invocation
//! lives here, never in the engine, so the engine stays a pure function of its
//! inputs.
//!
//! After minting, [`compose_in_repo`] composes the cascade's default workflow
//! (`single-task`) with `{{task.intent}}` = the intent — locating the cascade
//! (CLI locates, engine resolves), running the `workflow-refs` gate at
//! compose-time, then emitting the four-class composed view
//! ([write-commands.md](../../../design/write-commands.md) → Task origination;
//! [workflow-dialect.md](../../../design/workflow-dialect.md) → The composed
//! output is a view). Composition is deterministic and makes no LLM call (same
//! resolved cascade in → same workflow out).

use crate::pack::EmbeddedPack;
use anyhow::{Context, Result, bail};
use engine::address::Address;
use engine::compose::{
    self, CommandCatalog, ComposedWorkflow, StepDef, StepSource, StoreContext, WorkflowDef,
    load_command_catalog, load_step_def, load_workflow_def,
};
use engine::data_value::{ComposeContext, TaskRoot};
use engine::finding::{Finding, Severity};
use engine::index;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Schema, load_schema};
use engine::state::{self, BasePin, MintedTask, RolesRecord};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

/// The default doc-type name minting falls back to when the intent slugs to
/// nothing. `single-task` is a `commit`-producing work-workflow, so its task-id
/// fallback is the `commit` type name (`write-commands.md` → Instance
/// provisioning: the task's commit doc).
const FALLBACK_TYPE: &str = "commit";

/// Mint a task in the repo containing `start`: read HEAD, then open the working
/// area + base pin under `<repo_root>/.jigc/`.
///
/// Repo-root discovery reuses the same `.git`-ancestor walk as cascade location
/// ([`crate::locate`]). A serial collision (an active task of the slugged id
/// already exists) surfaces as the engine's routed blocking finding, mapped here
/// to an `anyhow` error carrying that route.
pub fn mint_in_repo(start: &Path, intent: &str) -> Result<MintedTask> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let jigc_root = repo_root.join(".jigc");
    let base = read_head(&repo_root)?;

    state::mint_task(&jigc_root, intent, FALLBACK_TYPE, base).map_err(finding_to_err)
}

/// Provision the task's workflow-provisioned **commit** doc into the working
/// area as a **fillable form**: the empty skeleton with every schema header field
/// pre-stamped as an empty `key:` line, materialized at
/// `<task_dir>/docs/commit:<id>.md`. This is the deterministic structural act the
/// workflow owns — the agent then fills the `summary`/`body` slots (`set-slot`)
/// and splices the `type`/`scope` fields (`set-field`) into the present lines.
///
/// The header-field lines are pre-stamped because the engine's `set-field` is a
/// **surgical splice** of a present value line ([`engine::write::set_field`]):
/// front-matter has no generate-a-new-line path (the header is not a generatable
/// body section), so the form must carry the line for the agent to fill. (Body
/// slots provision empty and `set-slot` generates the section's home on demand —
/// `write-commands.md` → Instance provisioning; `DECISIONS.md` 2026-05-31 → inc-4
/// fillable-form provisioning.)
fn provision_commit_doc(pack: &EmbeddedPack, minted: &MintedTask) -> Result<()> {
    let bytes = read_pack(pack, PackResourceKind::Schemas, FALLBACK_TYPE)?;
    let schema = engine::schema::load_schema(&bytes)
        .map_err(|e| anyhow::anyhow!("the `{FALLBACK_TYPE}` schema is malformed: {e}"))?;
    let instance = fillable_form(&schema, &minted.id);
    let path = state::instance_path(&minted.dir, &schema.ty, &minted.id);
    let rendered = engine::write::render(&schema, &instance);
    state::persist(&path, rendered.as_bytes())
        .with_context(|| format!("could not provision the commit doc for `{}`", minted.id))?;
    Ok(())
}

/// Build the **fillable** empty instance for `schema`: the H1 title is the task id,
/// every body section is content-free, and the header section's declared fields are
/// pre-stamped as empty `key:` lines (the surface `set-field` splices). The empty
/// value is `Value::Scalar("")`, which renders the canonical `key:` line.
fn fillable_form(schema: &engine::schema::Schema, slug: &str) -> engine::write::Instance {
    use engine::field_block::{Field, Value};
    use engine::schema::SectionBody;
    use engine::write::SectionContent;

    let sections = schema
        .sections
        .iter()
        .map(|section| {
            let fields = if section.header {
                match &section.body {
                    SectionBody::Simple { fields, .. } => fields
                        .iter()
                        .map(|f| Field {
                            key: f.id.clone(),
                            value: Value::Scalar(String::new()),
                        })
                        .collect(),
                    SectionBody::Repeatable { .. } => Vec::new(),
                }
            } else {
                Vec::new()
            };
            SectionContent {
                id: section.id.clone(),
                fields,
                ..Default::default()
            }
        })
        .collect();
    engine::write::Instance {
        title: slug.to_string(),
        sections,
    }
}

/// The cascade knob the default-workflow id is read from
/// (`design/overrides.md`; `design/write-commands.md` → Task origination: the
/// `default-workflow: <id>` cascade knob the front door composes). The MVP pack
/// ships `default-workflow: single-task` in its `config/defaults` resource.
const DEFAULT_WORKFLOW_KEY: &str = "default-workflow";

/// Mint a task from `intent`, then compose the cascade's default workflow over
/// it — the `jigc start "<intent>"` front door.
///
/// Locates the cascade (a missing project layer routes to `jigc setup`), reads
/// `default-workflow` from the pack's `config/defaults`, mints the task (seq 12),
/// builds the [`ComposeContext`] binding `{{task.intent}}`/`{{task.id}}` and
/// declaring the workflow's `allows-create` roles unbound, runs the
/// `workflow-refs` gate at compose-time, and on a clean gate emits the
/// deterministic four-class composed view. A blocking gate finding short-circuits
/// with that finding's message + route — nothing is emitted past the gate.
///
/// Returns the composed view; the caller renders it through the selected format.
pub fn compose_in_repo(start: &Path, intent: &str) -> Result<ComposedWorkflow> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    let pack = EmbeddedPack::new();
    let workflow_id = default_workflow_id(&pack)?;
    let workflow_bytes = read_pack(&pack, PackResourceKind::Workflows, &workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let catalog = load_catalog(&pack)?;

    // Mint the task (reads HEAD) — `single-task` declares `creates-task: true`,
    // so the front door mints in one call. Minting after the definition loads so
    // a malformed pack never leaves a task dir behind.
    let minted = mint_in_repo(&repo_root, intent)?;

    // Workflow-provisioned instance — a `creates-task` work-workflow provisions
    // the task's commit doc (the sink of its `<<author: {{task.commit#summary}}>>`
    // slot), so the agent only fills slots (`write-commands.md` → Instance
    // provisioning → Workflow-provisioned). The empty skeleton stages here, ready
    // for the `jigc doc set-field`/`set-slot` write loop.
    provision_commit_doc(&pack, &minted)?;

    // At mint there are no bound context roles yet (the agent binds them in-task,
    // e.g. an ADR via the create-gate); resume re-reads them from roles.json.
    let ctx = build_context(&minted.id, intent, &def, &RolesRecord::new());
    let source = PackStepSource { pack: &pack };

    // Compose-time `workflow-refs` gate: validate every placeholder / include /
    // command-ref / marker before any output reaches the agent. A blocking
    // finding short-circuits.
    let findings = compose::workflow_refs(&workflow_bytes, &source, &catalog, &ctx);
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    compose::compose(&def, &source, &catalog, &ctx).map_err(finding_to_err)
}

/// Resume an existing task by `id` and re-compose its default workflow — the
/// `jigc start --task <id>` form (`design/write-commands.md` → Task-id collision &
/// resume; `DECISIONS.md` 2026-05-31 → Four `jigc start` forms).
///
/// Unlike [`compose_in_repo`] this **mints nothing**: it resolves the existing
/// `.jigc/tasks/<id>/` working area, reads its persisted state (base pin, the
/// original intent, the bound context roles in `roles.json`), verifies the task's
/// base still matches the current checkout (the CLI never operates a task off its
/// pinned base — `storage.md` → A task is pinned to its base), then re-composes
/// with the bound roles in scope. A role bound since the task was minted (e.g. an
/// ADR created in-task through the create-gate) now resolves in the composed view
/// — the surface the superseding-decision context-slice reads
/// (`worked-examples.md` → Superseding decision).
///
/// A nonexistent id rejects with `no task \`<id>\``; a base mismatch rejects with
/// the divergence-routing prompt (`write-commands.md` → Base mismatch on an
/// existing task).
pub fn resume_in_repo(start: &Path, id: &str) -> Result<ComposedWorkflow> {
    let repo_root = discover_repo_root(start)
        .with_context(|| format!("not inside a git repository (from {})", start.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    let task_dir = repo_root.join(".jigc").join("tasks").join(id);
    if !task_dir.is_dir() {
        bail!("no task `{id}` — list live tasks with `jigc start`");
    }

    // The task is pinned to its base; never operate it off its pinned commit.
    let pinned = state::read_base_pin(&task_dir)
        .with_context(|| format!("could not read the base pin for task `{id}`"))?;
    let head = read_head(&repo_root)?;
    if pinned.sha != head.sha {
        bail!(
            "task `{id}` is pinned to base {} but you're on {} — switch back with `git checkout {}` or `jigc task discard {id}`",
            pinned.short,
            head.short,
            pinned.short,
        );
    }

    let intent = state::read_intent(&task_dir)
        .with_context(|| format!("could not read intent for `{id}`"))?;
    let bound =
        RolesRecord::load(&task_dir).with_context(|| format!("could not read roles for `{id}`"))?;

    let pack = EmbeddedPack::new();
    let workflow_id = default_workflow_id(&pack)?;
    let workflow_bytes = read_pack(&pack, PackResourceKind::Workflows, &workflow_id)?;
    let def = load_workflow_def(&workflow_bytes).map_err(finding_to_err)?;
    let catalog = load_catalog(&pack)?;

    let ctx = build_context(id, &intent, &def, &bound);
    let source = PackStepSource { pack: &pack };

    let findings = compose::workflow_refs(&workflow_bytes, &source, &catalog, &ctx);
    if let Some(finding) = findings
        .into_iter()
        .find(|f| f.severity == Severity::Blocking)
    {
        return Err(finding_to_err(finding));
    }

    // Wire the committed store + edge overlay so a context-slice over a persisted ADR
    // (`{{@task.decision.supersedes#decision}}`) dereferences to the prior decision's
    // prose — the superseding-decision read path (`worked-examples.md` → Task 2). The
    // CLI locates the layers (committed-store root + the `.jigc/` index home), the
    // engine resolves through the `ContentStore` trait (`VISION.md` principle #4).
    let jigc_root = repo_root.join(".jigc");
    let schemas = all_schemas(&pack)?;
    let committed = index::load_committed(&repo_root, &jigc_root, &schemas, &head.sha);
    let overlay = index::overlay_working(&committed, &task_dir, &schemas);
    let store = StoreContext {
        repo_root: &repo_root,
        schemas: &schemas,
        overlay: &overlay,
    };

    compose::compose_with_store(&def, &source, &catalog, &ctx, Some(&store)).map_err(finding_to_err)
}

/// Load every shipped schema from the embedded pack, keyed by doctype — the
/// cascade-resolved schema set the committed store + edge overlay resolve `<type>`
/// prefixes against (`crate::task` loads the same set for the finalize sweep).
fn all_schemas(pack: &EmbeddedPack) -> Result<BTreeMap<String, Schema>> {
    let mut out = BTreeMap::new();
    for id in pack.list(PackResourceKind::Schemas) {
        let bytes = read_pack(pack, PackResourceKind::Schemas, id.as_str())?;
        let schema = load_schema(&bytes)
            .with_context(|| format!("the `{}` schema is malformed", id.as_str()))?;
        out.insert(schema.ty.clone(), schema);
    }
    Ok(out)
}

/// Build the [`ComposeContext`] for the composed workflow.
///
/// Binds the engine-native `task.id`/`task.intent` scalars, the task's **commit**
/// role to `commit:<id>` (the always-present sink of a `creates-task` work-
/// workflow — its `<<author: {{task.commit#summary}}>>` slot must resolve to that
/// address), and declares each `allows-create` role from the workflow gate. A
/// role **bound** in `roles` (persisted in the task's `roles.json` once the agent
/// created the doc through the create-gate) binds to its recorded address; an
/// **unbound** gate role declares as `None`, so `{{@task.<role>.…}}` resolves to
/// absent/empty (`design/workflow-dialect.md` → Empty vs unresolvable). At mint
/// `roles` is empty (nothing is bound yet); on `--task <id>` resume it carries
/// the binds recorded since (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role binding
/// at create).
fn build_context(id: &str, intent: &str, def: &WorkflowDef, bound: &RolesRecord) -> ComposeContext {
    let mut roles: BTreeMap<String, Option<Address>> = BTreeMap::new();
    // The task's commit doc — the engine-native sink, bound to `commit:<id>`.
    let commit =
        Address::parse(&format!("commit:{id}")).expect("`commit:<slug>` is a valid address");
    roles.insert("commit".to_owned(), Some(commit));
    // Each `allows-create` role is declared; bound iff `roles.json` recorded it.
    for entry in &def.allows_create {
        let binding = bound
            .get(&entry.as_role)
            .and_then(|addr| Address::parse(addr).ok());
        roles.entry(entry.as_role.clone()).or_insert(binding);
    }
    ComposeContext {
        task: Some(TaskRoot {
            id: id.to_owned(),
            intent: intent.to_owned(),
            roles,
        }),
    }
}

/// A [`StepSource`] that resolves step ids against the embedded pack. The MVP
/// cascade has no project step overrides, so the pack-default layer owns every
/// step — the engine consumes this mapping and stays a pure function of it.
struct PackStepSource<'a> {
    pack: &'a EmbeddedPack,
}

impl StepSource for PackStepSource<'_> {
    fn step(&self, id: &str) -> Option<StepDef> {
        let bytes = self
            .pack
            .read(PackResourceKind::Steps, &ResourceId::from(id))
            .ok()?;
        load_step_def(id, &bytes).ok()
    }
}

/// Read the `default-workflow` id from the pack's `config/defaults` resource.
fn default_workflow_id(pack: &EmbeddedPack) -> Result<String> {
    let bytes = read_pack(pack, PackResourceKind::Config, "defaults")?;
    let text = String::from_utf8(bytes).context("`config/defaults` is not UTF-8")?;
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).context("`config/defaults` is not valid YAML")?;
    value
        .get(DEFAULT_WORKFLOW_KEY)
        .and_then(serde_yaml_ng::Value::as_str)
        .map(str::to_owned)
        .with_context(|| format!("`config/defaults` declares no `{DEFAULT_WORKFLOW_KEY}`"))
}

/// Load and parse the pack's command catalog (`config/commands`).
fn load_catalog(pack: &EmbeddedPack) -> Result<CommandCatalog> {
    let bytes = read_pack(pack, PackResourceKind::Config, "commands")?;
    load_command_catalog(&bytes).map_err(finding_to_err)
}

/// Read a pack resource by kind + id, mapping a missing resource to an error.
fn read_pack(pack: &EmbeddedPack, kind: PackResourceKind, id: &str) -> Result<Vec<u8>> {
    pack.read(kind, &ResourceId::from(id))
        .with_context(|| format!("the embedded pack is missing `{id}`"))
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope minting already uses (a hard block is a blocking-severity
/// finding carrying a route, `DECISIONS.md` 2026-05-31).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let route = finding
        .route
        .map(|r| format!("\n  route: {r}"))
        .unwrap_or_default();
    anyhow::anyhow!("{}{route}", finding.message)
}

/// Read HEAD as a [`BasePin`] (full + short SHA) by shelling out to the user's
/// `git` (`DECISIONS.md` 2026-05-31 → Git invocation: shell out to the `git`
/// binary, not git2/gitoxide).
fn read_head(repo_root: &Path) -> Result<BasePin> {
    let sha = git_rev_parse(repo_root, &["rev-parse", "HEAD"])?;
    let short = git_rev_parse(repo_root, &["rev-parse", "--short", "HEAD"])?;
    Ok(BasePin::new(sha, short))
}

/// Run `git <args>` in `repo_root` and return the single trimmed line of stdout.
fn git_rev_parse(repo_root: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .context("could not run `git` (is it on PATH?)")?;
    if !out.status.success() {
        anyhow::bail!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim(),
        );
    }
    let line = String::from_utf8(out.stdout)
        .context("`git` produced non-UTF-8 output")?
        .trim()
        .to_string();
    Ok(line)
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the
/// same discovery [`crate::locate`] does, kept local so minting needs no
/// `RunContext`.
fn discover_repo_root(start: &Path) -> Option<std::path::PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(std::path::PathBuf::from)
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
                "jigc-start-mint-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
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

    /// Initialize a real git repo with one commit, returning its HEAD SHAs.
    fn init_repo_with_commit(root: &Path) -> (String, String) {
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .args(args)
                .current_dir(root)
                .output()
                .expect("run git");
            assert!(
                out.status.success(),
                "git {args:?} failed: {}",
                String::from_utf8_lossy(&out.stderr)
            );
            String::from_utf8(out.stdout)
                .expect("utf-8")
                .trim()
                .to_string()
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "test@example.com"]);
        git(&["config", "user.name", "Test"]);
        fs::write(root.join("README.md"), "hello\n").expect("write file");
        git(&["add", "."]);
        git(&["commit", "-q", "-m", "initial"]);
        let sha = git(&["rev-parse", "HEAD"]);
        let short = git(&["rev-parse", "--short", "HEAD"]);
        (sha, short)
    }

    /// The CLI integration done-criterion: minting under a real temp git repo
    /// creates the working dir and a base pin recording the repo's actual HEAD.
    #[test]
    fn mint_opens_working_dir_and_base_pin_under_a_temp_git_repo() {
        let repo = TempDir::new("repo");
        let (sha, short) = init_repo_with_commit(repo.path());

        let minted = mint_in_repo(repo.path(), "Add rate limiter").expect("mint succeeds");

        assert_eq!(minted.id, "add-rate-limiter");
        let dir = repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("add-rate-limiter");
        assert!(dir.is_dir(), "working dir must appear under .jigc/tasks/");
        assert_eq!(minted.dir, dir);

        let pin = fs::read_to_string(dir.join("base.json")).expect("base pin written");
        let base: BasePin = serde_json::from_str(&pin).expect("pin parses");
        assert_eq!(base.sha, sha, "base pin records HEAD's full SHA");
        assert_eq!(base.short, short, "base pin records HEAD's short SHA");

        // A serial re-mint of the same intent rejects, surfacing the route.
        let err = mint_in_repo(repo.path(), "Add rate limiter").expect_err("re-mint rejects");
        let msg = err.to_string();
        assert!(
            msg.contains("add-rate-limiter") && msg.contains("route:"),
            "serial collision must name the task and carry a route; got: {msg}"
        );
    }
}
