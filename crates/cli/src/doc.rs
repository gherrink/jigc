//! `jigc doc <verb> <addr>` — the write-path surface over the engine's gated
//! verbs (`design/write-commands.md` → The verbs / Content handoff / Worked
//! example). CLI wiring only: resolve the active task from cwd, parse the
//! address, map its fragment to a `(section, leaf)`, hand the bytes to the engine
//! verb (`set_field_validated` / `set_slot_validated` / `create_gated`), persist
//! the returned buffer atomically, and surface a blocking [`Finding`] (with its
//! route) on stderr + a non-zero exit.
//!
//! The slot/field handoff split falls out of the leaf kind
//! (`design/write-commands.md` → Content handoff): fields are short + adjudicable
//! so they arrive **inline** (`--value`); slots are multi-line prose so they
//! arrive via **stdin / `--from-file`**. The engine owns placement, validation,
//! and the canonical byte form; the CLI owns I/O and presentation (the route the
//! agent acts on next, surfaced per the settled block-payload envelope).

use crate::cli::Format;
use crate::pack::make_pack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::address::{Address, Fragment};
use engine::compose::{WorkflowDef, load_workflow_def};
use engine::field_block::Value;
use engine::finding::{Finding, Location, Severity};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Schema, SectionBody};
use engine::state;
use engine::write::{
    set_field_validated, set_item_field_or_insert, set_item_slot, set_slot_validated,
};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;

/// The `jigc doc <verb>` subcommand tree. Each verb addresses a managed doc in
/// the active task's working area (`design/write-commands.md` → The verbs).
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum DocCommand {
    /// Mint a new managed instance (agent-initiated, create-gated). The id-source
    /// is supplied inline; the CLI mints + places per the schema.
    Create {
        /// The doctype to create (e.g. `adr`).
        r#type: String,
        /// The id-source the slug is minted from (inline, the `--<id-source>`
        /// form; `--title "…"` is the MVP surface for the title-slugged types).
        #[arg(long)]
        title: String,
        /// The active task to scope the write to (`design/write-commands.md` →
        /// The write-time `--task`-scoped barrier). Optional: explicit wins; else
        /// the single active task; else (zero / more-than-one) the write rejects.
        #[arg(long)]
        task: Option<String>,
    },
    /// Mint a repeatable item into a section (`<type>:<slug>#<section>`), id-slugged
    /// from `--title`. The CLI mints the `{#id}` anchor + appends the item block.
    AddItem {
        /// The section address — `<type>:<slug>#<section>` (the repeatable section the
        /// item is minted into).
        addr: String,
        /// The item id-source — slugged to the `{#id}` anchor (the MVP surface; the
        /// item's slot/fields are filled by later `set-slot`/`set-field` writes).
        #[arg(long)]
        title: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a field leaf's value (inline, adjudicated at write time).
    SetField {
        /// The leaf address — `<type>:<slug>#<field>` (or `#<section>/<field>`).
        addr: String,
        /// The new value (inline — fields are short + escaping-safe).
        #[arg(long)]
        value: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a slot leaf's prose (multi-line, via stdin or a file).
    SetSlot {
        /// The slot address — `<type>:<slug>#<slot>`.
        addr: String,
        /// The prose source: a path, or `-` for stdin (prose never inline).
        #[arg(long)]
        from_file: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
}

/// A `doc` verb's failure: a write-time **block** (a structured [`Finding`],
/// rendered through `--format` so an agent on `--format json` gets a parseable
/// envelope), or an **orchestration** error (git/IO/usage — plain text). The
/// blocking finding is the same envelope the rest of the CLI uses; only the
/// happy-path *output* of `doc` stays plain (the staged buffer / new address).
enum DocFailure {
    Block(Finding),
    Orchestration(anyhow::Error),
}

impl From<anyhow::Error> for DocFailure {
    fn from(err: anyhow::Error) -> Self {
        DocFailure::Orchestration(err)
    }
}

impl DocCommand {
    /// Dispatch the parsed `doc` verb against the active task in `cwd`, mapping a
    /// blocking [`Finding`] to a non-zero exit — rendered through `--format` (JSON
    /// envelope under `--format json`, the located message + route otherwise) — and
    /// an orchestration error to plain stderr.
    pub fn dispatch(self, cwd: &Path, format: Format) -> ExitCode {
        let result = match self {
            DocCommand::Create {
                r#type,
                title,
                task,
            } => run_create(cwd, &r#type, &title, task.as_deref()),
            DocCommand::AddItem { addr, title, task } => {
                run_add_item(cwd, &addr, &title, task.as_deref())
            }
            DocCommand::SetField { addr, value, task } => {
                run_set_field(cwd, &addr, &value, task.as_deref())
            }
            DocCommand::SetSlot {
                addr,
                from_file,
                task,
            } => run_set_slot(cwd, &addr, &from_file, task.as_deref()),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(DocFailure::Block(finding)) => {
                // The write-time block is not an inventory check, so the severity
                // post-pass is a no-op over it; a no-delta cascade keeps it byte-identical.
                let resolved = match crate::cascade_util::no_delta_resolved() {
                    Ok(resolved) => resolved,
                    Err(err) => {
                        eprintln!("{err:#}");
                        return ExitCode::FAILURE;
                    }
                };
                let report = engine::result::ValidationReport::new(vec![finding], &resolved);
                eprint!("{}", render::validation(format, &report));
                if format != Format::Json {
                    eprintln!();
                }
                ExitCode::FAILURE
            }
            Err(DocFailure::Orchestration(err)) => {
                eprintln!("{err:#}");
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc doc set-field <addr> --value <v>` — adjudicate + splice a field value.
fn run_set_field(
    cwd: &Path,
    addr: &str,
    value: &str,
    task_id: Option<&str>,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)
        .with_context(|| format!("no field addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = match target {
        FieldTarget::Section { section, field } => set_field_validated(
            &schema,
            &source,
            &section,
            &field,
            &Value::Scalar(value.to_string()),
        )
        .map_err(|f| block(&f, "set-field", addr))?,
        FieldTarget::Item {
            section,
            item,
            field,
        } => set_item_field_or_insert(&schema, &source, &section, &item, &field, value).map_err(
            |e| {
                block(
                    &engine::write::generate_error_finding(&e),
                    "set-field",
                    addr,
                )
            },
        )?,
    };

    persist(&path, &edited)?;
    Ok(())
}

/// `jigc doc set-slot <addr> --from-file <path|->` — splice slot prose (stdin/file).
fn run_set_slot(
    cwd: &Path,
    addr: &str,
    from_file: &str,
    task_id: Option<&str>,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        slot_target(&schema, &address).with_context(|| format!("no slot addressed by `{addr}`"))?;

    let prose = read_handoff(from_file)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = match target {
        SlotTarget::Section(section) => set_slot_validated(&schema, &source, &section, &prose)
            .map_err(|f| block(&f, "set-slot", addr))?,
        SlotTarget::Item { section, item } => {
            set_item_slot(&schema, &source, &section, &item, &prose)
                .map_err(|e| block(&engine::write::splice_error_finding(&e), "set-slot", addr))?
        }
    };

    persist(&path, &edited)?;
    Ok(())
}

/// `jigc doc add-item <addr>#<section> --title <…>` — mint a repeatable item into a
/// section. Clones the `run_create`/`run_set_slot` shape: resolve the active task,
/// parse the address, resolve the section from the fragment's leading hop, read (or
/// copy-in) the staged instance, call the proven engine `add_item` (mint-empty —
/// `--title` only, no slot/fields yet), persist, and print the minted item address
/// `<type>:<slug>#<section>/<slug(title)>` (the next address an agent fills the item's
/// slot/field at). A non-repeatable section / unknown section routes the engine's
/// [`engine::write::GenerateError`] through the shared blocking [`Finding`] mapping
/// (`design/write-commands.md`; `architecture-documentation.md` → add-item ergonomics).
fn run_add_item(
    cwd: &Path,
    addr: &str,
    title: &str,
    task_id: Option<&str>,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let section_id =
        section_hop(&address).with_context(|| format!("no section addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = engine::write::add_item(&schema, &source, &section_id, title, None, &[])
        .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", addr))?;

    persist(&path, &edited)?;
    // The minted item address — the section hop plus the slugger-minted anchor (the
    // same slugify the engine mints the `{#id}` from, never re-spelled).
    println!(
        "{}:{}#{}/{}",
        address.r#type.as_str(),
        address.slug.as_str(),
        section_id,
        engine::slug::slugify(title),
    );
    Ok(())
}

/// `jigc doc create <type> --title <…>` — agent-initiated, create-gated mint.
fn run_create(
    cwd: &Path,
    type_name: &str,
    title: &str,
    task_id: Option<&str>,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let schemas = task.schemas()?;
    let gate = task.workflow_gate()?;
    let created = state::create_gated(
        &task.dir,
        &schemas,
        &gate.allows_create,
        type_name,
        title,
        &task.repo_root,
    )
    .map_err(|f| block(&f, "create", type_name))?;
    println!("{}", created.address);
    Ok(())
}

/// The active task: its working-area directory + the embedded pack to resolve
/// schemas and the workflow gate against.
struct ActiveTask {
    /// The resolved task id (the directory name under `.jigc/tasks/`) — the name
    /// the write-time barrier scopes the staged-doc destination to.
    id: String,
    dir: PathBuf,
    /// The repo root the task lives under — the anchor copy-on-first-touch resolves
    /// a base-committed `<location>/<slug>.md` against (`canonical_path`).
    repo_root: PathBuf,
    pack: Box<dyn PackSource>,
}

impl ActiveTask {
    /// Resolve the active task from `cwd` + an optional explicit `--task <id>`
    /// (`design/write-commands.md` → The write-time `--task`-scoped barrier:
    /// active-task resolution). **Explicit `--task <id>` wins** — it resolves
    /// `<repo>/.jigc/tasks/<id>/` directly, rejecting `no task <id>` if absent
    /// (mirroring `start::reenter_in_repo`). Else: the **single** active task
    /// directory under `<repo>/.jigc/tasks/`; **none** rejects with the start-a-task
    /// route, **more than one** with no `--task` rejects asking for the selector.
    fn resolve(cwd: &Path, task_id: Option<&str>) -> Result<Self> {
        let repo_root = discover_repo_root(cwd)
            .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
        let tasks = repo_root.join(".jigc").join("tasks");

        if let Some(id) = task_id {
            let dir = tasks.join(id);
            if !dir.is_dir() {
                bail!("no task `{id}` — list live tasks with `jigc start`");
            }
            return Ok(Self {
                id: id.to_string(),
                dir,
                repo_root,
                pack: make_pack(),
            });
        }

        let mut dirs: Vec<PathBuf> = match std::fs::read_dir(&tasks) {
            Ok(entries) => entries
                .filter_map(std::result::Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect(),
            Err(_) => Vec::new(),
        };
        dirs.sort();
        match dirs.len() {
            0 => bail!("no active task — start one with `jigc start`"),
            1 => {
                let dir = dirs.pop().expect("one task dir");
                let id = dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .expect("a task dir under .jigc/tasks/ has a utf-8 name")
                    .to_string();
                Ok(Self {
                    id,
                    dir,
                    repo_root,
                    pack: make_pack(),
                })
            }
            _ => bail!("more than one active task — name one with `--task <id>`"),
        }
    }

    /// Read the staged instance bytes the edit verb splices into, applying
    /// **copy-on-first-touch** when the doc is base-committed but not yet staged
    /// (`design/write-commands.md` → copy-on-first-touch).
    ///
    /// Three cases, in order:
    /// 1. **Already staged** — read the staged body (the steady-state path: a
    ///    `created` doc, or a base doc already copied in by an earlier edit). A
    ///    second edit therefore never re-copies, so the prior edit survives.
    /// 2. **Absent from the area but committed at base** — the slug's canonical
    ///    `<location>/<slug>.md` exists under `repo_root`: copy that committed body
    ///    in via [`state::copy_in`] (which records `edited-from-base` write-once),
    ///    then read the copied-in body. This is the *only* new wiring T4 adds — the
    ///    first production caller of `copy_in`.
    /// 3. **Neither staged nor committed** — reject with the unchanged
    ///    "no staged instance" error (`read_staged`).
    fn read_or_copy_in(
        &self,
        path: &Path,
        schema: &Schema,
        address: &Address,
        addr: &str,
    ) -> Result<String, DocFailure> {
        if path.is_file() {
            return Ok(read_staged(path, addr)?);
        }
        // The copy-in trigger predicate: the slug is absent from the area AND its
        // committed `<location>/<slug>.md` exists at base — resolved by the same
        // `schema.location`-keyed path the committed store / `task bind` use.
        let slug = address.slug.as_str();
        if let Some(committed) = engine::store::canonical_path(&self.repo_root, schema, slug)
            && committed.is_file()
        {
            let body = std::fs::read_to_string(&committed)
                .with_context(|| format!("could not read committed `{addr}`"))?;
            state::copy_in(&self.dir, address.r#type.as_str(), slug, &body)
                .with_context(|| format!("could not copy in `{addr}` for editing"))?;
            return Ok(read_staged(path, addr)?);
        }
        // Neither staged nor committed → the unchanged absent-instance reject.
        Ok(read_staged(path, addr)?)
    }

    /// Load the schema for `type_name` from the embedded pack.
    fn schema(&self, type_name: &str) -> Result<Schema> {
        let bytes = self
            .pack
            .read(PackResourceKind::Schemas, &ResourceId::from(type_name))
            .with_context(|| format!("unknown doctype `{type_name}`"))?;
        crate::pack::load_pack_schema(self.pack.as_ref(), &bytes)
            .with_context(|| format!("the `{type_name}` schema is malformed"))
    }

    /// Load every shipped schema, keyed by doctype — the set `create_gated`
    /// adjudicates the unknown-doctype check against.
    fn schemas(&self) -> Result<BTreeMap<String, Schema>> {
        let mut out = BTreeMap::new();
        for id in self.pack.list(PackResourceKind::Schemas) {
            let bytes = self
                .pack
                .read(PackResourceKind::Schemas, &id)
                .with_context(|| format!("the `{}` schema reads back", id.as_str()))?;
            let schema = crate::pack::load_pack_schema(self.pack.as_ref(), &bytes)
                .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
            out.insert(schema.ty.clone(), schema);
        }
        Ok(out)
    }

    /// Load the task's **bound** workflow definition — the create-gate's
    /// `allows-create` lives on its front-matter. The workflow is the one the task
    /// was minted on (recorded at `.jigc/tasks/<id>/workflow`), never a hardcoded
    /// default: a `plan` task's gate must read `plan`'s `allows-create`, not
    /// `single-task`'s. Mirrors `start::resume_in_repo`'s bound-workflow read.
    fn workflow_gate(&self) -> Result<WorkflowDef> {
        let workflow_id = state::read_workflow_id(&self.dir)
            .context("could not read the task's recorded workflow")?
            .context(
                "the active task has no recorded workflow — discard it with `jigc task discard <id>` and re-start with `jigc start`",
            )?;
        let bytes = self
            .pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from(workflow_id.as_str()),
            )
            .with_context(|| format!("the embedded pack is missing `{workflow_id}`"))?;
        load_workflow_def(&bytes).map_err(finding_to_err)
    }
}

/// Parse an address string, mapping a grammar error to an actionable message.
fn parse_addr(addr: &str) -> Result<Address> {
    Address::parse(addr).with_context(|| format!("malformed address `{addr}`"))
}

/// The staged on-disk path of `address`'s instance within the task working area,
/// **guarded by the write-time barrier** (`design/write-commands.md` → The
/// write-time `--task`-scoped barrier; `design/storage.md` → The by-task-id join:
/// the write-time complement to the join-time isolation check).
///
/// The address slug is structural, not sanitized ([`engine::address`] splits on
/// `:` / `#` / `/` only), so a slug like `../../<sibling>/docs/x` would make the
/// destination escape this area. The barrier rejects up front any destination that
/// lexically lands **outside** `<task_dir>/docs/` — a sub-agent physically cannot
/// stage into a sibling's area. (Lexical, not on-disk `canonicalize`: an escaping
/// destination does not exist, so it has no real path to canonicalize.)
fn staged_path(task_dir: &Path, address: &Address, task_id: &str) -> Result<PathBuf, DocFailure> {
    // The area is the parent of any non-escaping staged-instance path — sourced
    // from `instance_path` itself so the `docs/` layout constant stays owned by
    // the engine, never restated here.
    let area = state::instance_path(task_dir, "_", "_")
        .parent()
        .expect("a staged instance path has a `docs/` parent")
        .to_path_buf();
    let dest = state::instance_path(task_dir, address.r#type.as_str(), address.slug.as_str());
    if within_area(&area, &dest) {
        Ok(dest)
    } else {
        Err(DocFailure::Block(barrier_block(task_id, address)))
    }
}

/// The destination-containment predicate the barrier is built on: does `dest`
/// lexically resolve to a path **within** `area`? Both are normalized by folding
/// `.` / `..` syntactically (an escaping `..` that climbs above `area` fails the
/// `starts_with`), never touching the filesystem — the dest of a refused write
/// does not exist, so there is nothing to `canonicalize`.
fn within_area(area: &Path, dest: &Path) -> bool {
    let dest = lexically_normalize(dest);
    let area = lexically_normalize(area);
    dest.starts_with(&area)
}

/// Lexically fold a path's `.` (dropped) and `..` (pop the prior real component)
/// components, leaving roots/prefixes intact. A leading `..` that cannot pop is
/// preserved, so a path that climbs above its anchor keeps the `..` and fails a
/// `starts_with` against any anchor below it.
fn lexically_normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                // Pop a prior normal component; if the tail is a `..` (or there is
                // nothing to pop), keep the `..` so the escape stays visible.
                if matches!(out.components().next_back(), Some(Component::Normal(_))) {
                    out.pop();
                } else {
                    out.push(Component::ParentDir.as_os_str());
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// The write-time barrier block (`design/write-commands.md` → The write-time
/// `--task`-scoped barrier) — the write-time complement to the M7
/// `join.area-isolation` finding. Blocking, **located** at the offending address +
/// **routed**, emitted directly with no `knobs.yaml` row (the blocking-but-untunable
/// precedent; mirrors [`engine::milestone`]'s `join.area-isolation`).
fn barrier_block(task_id: &str, address: &Address) -> Finding {
    let address = address.to_string();
    Finding::graded(
        Severity::Blocking,
        "write.area-barrier",
        format!(
            "barrier — the staged destination for `{address}` lands outside task \
             `{task_id}`'s area (`tasks/{task_id}/docs/`); a staging write must stay \
             within its own sub-area"
        ),
        Some(Location::addressed(&address, 1, 1)),
        Some(format!(
            "address the doc with a slug inside task `{task_id}`'s own area"
        )),
    )
}

/// Read the staged instance bytes, mapping an absent instance to an actionable
/// error (the write verbs require the instance to already exist —
/// `design/write-commands.md` → Instance provisioning).
fn read_staged(path: &Path, addr: &str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| {
        format!("no staged instance for `{addr}` — provision it first (`jigc start` / `jigc doc create`)")
    })
}

/// Persist the engine's returned buffer atomically into the working area.
fn persist(path: &Path, bytes: &str) -> Result<()> {
    state::persist(path, bytes.as_bytes())
        .with_context(|| format!("could not persist `{}`", path.display()))
}

/// Read the slot handoff: `-` ⇒ stdin, else a file path (`design/write-commands.md`
/// → Content handoff: slots via stdin / `--from-file`, never inline). Shared with
/// `jigc config fill`, whose fill content arrives the same way.
pub(crate) fn read_handoff(from_file: &str) -> Result<String> {
    if from_file == "-" {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("could not read slot prose from stdin")?;
        Ok(buf)
    } else {
        std::fs::read_to_string(from_file)
            .with_context(|| format!("could not read slot prose from `{from_file}`"))
    }
}

/// The resolved destination of a `set-field` address: a **section-level** field
/// (`(section, field)`, adjudicated via `set_field_validated`) or an **item-level**
/// field on a repeatable item (`(section, item, field)`, spliced via
/// `set_item_field_or_insert`). The item hop disambiguates two items that carry
/// identically-keyed field leaves.
enum FieldTarget {
    Section {
        section: String,
        field: String,
    },
    Item {
        section: String,
        item: String,
        field: String,
    },
}

/// Resolve the destination a `set-field` address targets.
///
/// Three address forms: the single-hop `#<field>` (the MVP worked-example surface —
/// search every section for a field of that id), the explicit two-hop
/// `#<section>/<field>`, and the item-leaf three-hop `#<section>/<item>/<field>`
/// (M13 Increment 3 — the per-item field, addressed through the item id).
fn field_target(schema: &Schema, address: &Address) -> Option<FieldTarget> {
    match address.fragment.as_ref()? {
        Fragment::Unit(field) => {
            let field = field.as_str();
            schema.sections.iter().find_map(|s| match &s.body {
                SectionBody::Simple { fields, .. } if fields.iter().any(|f| f.id == field) => {
                    Some(FieldTarget::Section {
                        section: s.id.clone(),
                        field: field.to_string(),
                    })
                }
                _ => None,
            })
        }
        Fragment::UnitLeaf(section, field) => {
            let (section, field) = (section.as_str(), field.as_str());
            schema
                .sections
                .iter()
                .find(|s| s.id == section)
                .map(|s| FieldTarget::Section {
                    section: s.id.clone(),
                    field: field.to_string(),
                })
        }
        // The item-leaf field hop. The CLI only extracts the `(section, item, field)`
        // triple; the engine `set_item_field_or_insert` adjudicates shape (item/section
        // presence). PARITY GAP (DECISIONS.md 2026-06-07): item-leaf field writes carry
        // NO value-type / heading-ceiling adjudication — unlike the section-level
        // `set_field_validated` path — so a malformed item-field value is not rejected here.
        Fragment::UnitItemLeaf(section, item, field) => Some(FieldTarget::Item {
            section: section.as_str().to_string(),
            item: item.as_str().to_string(),
            field: field.as_str().to_string(),
        }),
        // A bare item hop (`#<section>/<item>`) addresses no field leaf.
        Fragment::UnitItem(_, _) => None,
    }
}

/// The resolved destination of a `set-slot` address: a **section-level** slot
/// (spliced via `set_slot_validated`) or an **item-level** per-item slot on a
/// repeatable item (spliced via `set_item_slot`, addressed through the item id).
enum SlotTarget {
    Section(String),
    Item { section: String, item: String },
}

/// Resolve the destination a `set-slot` address targets — the simple section whose
/// id is the fragment's leading hop and which declares a `slot`, or the per-item
/// slot of a repeatable item (`#<section>/<item>/<slot>`). The CLI extracts the
/// `(section, item)` pair for the item form; the engine `set_item_slot` adjudicates
/// item/section presence.
fn slot_target(schema: &Schema, address: &Address) -> Option<SlotTarget> {
    let section_id = match address.fragment.as_ref()? {
        Fragment::Unit(u) => u.as_str(),
        Fragment::UnitLeaf(u, _) => u.as_str(),
        Fragment::UnitItemLeaf(section, item, _) => {
            return Some(SlotTarget::Item {
                section: section.as_str().to_string(),
                item: item.as_str().to_string(),
            });
        }
        _ => return None,
    };
    schema.sections.iter().find_map(|s| match &s.body {
        SectionBody::Simple { slot: Some(_), .. } if s.id == section_id => {
            Some(SlotTarget::Section(s.id.clone()))
        }
        _ => None,
    })
}

/// Resolve the `section_id` an `add-item` address targets — the fragment's leading
/// hop. The CLI only extracts the named section; the engine `add_item` adjudicates
/// the *shape* (a non-repeatable / unknown section surfaces as a routed
/// [`engine::write::GenerateError`]), so this never re-checks repeatability here.
fn section_hop(address: &Address) -> Option<String> {
    match address.fragment.as_ref()? {
        Fragment::Unit(u) => Some(u.as_str().to_string()),
        Fragment::UnitLeaf(u, _) | Fragment::UnitItem(u, _) | Fragment::UnitItemLeaf(u, _, _) => {
            Some(u.as_str().to_string())
        }
    }
}

/// Wrap a blocking [`Finding`] as a [`DocFailure::Block`], ensuring it carries a
/// route. A hard block is a blocking-severity finding carrying a route
/// (`DECISIONS.md` 2026-05-31 → blocked/error payload). Where a write-time
/// adjudication finding carries none (the engine's `write.malformed-value` is
/// routeless), the CLI supplies the actionable retry route — presentation the CLI
/// owns, the determinism boundary unaffected. `dispatch` renders it through
/// `--format` (JSON envelope under `--format json`).
fn block(finding: &Finding, verb: &str, addr: &str) -> DocFailure {
    let mut finding = finding.clone();
    if finding.route.is_none() {
        finding.route = Some(format!(
            "retry `jigc doc {verb} {addr}` with a conforming value"
        ));
    }
    DocFailure::Block(finding)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope the front door uses (`crate::start::finding_to_err`).
fn finding_to_err(finding: Finding) -> anyhow::Error {
    let route = finding
        .route
        .map(|r| format!("\n  route: {r}"))
        .unwrap_or_default();
    anyhow::anyhow!("{}{route}", finding.message)
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the
/// same discovery `crate::start` / `crate::locate` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::schema::load_schema;

    /// The shipped `commit` schema, loaded from the embedded pack source tree.
    const COMMIT_YAML: &[u8] = include_bytes!("../pack/schemas/commit.yaml");
    /// The shipped `prd` schema (M9 new-project doc-type), loaded from the
    /// embedded pack source tree so the round-trip pins exactly the bytes that ship.
    const PRD_YAML: &[u8] = include_bytes!("../pack/schemas/prd.yaml");

    /// A throwaway directory that removes itself on drop — keeps the prd round-trip
    /// off any real repo tree.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-prd-{tag}-{}-{:?}",
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
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// RED STEP (G2, DECISIONS.md 2026-06-06): the `prd` schema loads, and a
    /// hand-authored `prd` instance with all three **single-word** prose slots
    /// (`vision`/`requirements`/`context`) filled **canonical-writes then re-parses
    /// idempotently** — proving the single-word-id constraint holds for the new
    /// shape, never trusting it. The round-trip is `write::render` → write to the
    /// canonical `prds/<slug>.md` → `store::read_slice` (which re-parses the
    /// committed bytes against the real schema and slices each section by its id):
    /// a multi-word id would trip the logged title-case reparse defect — the writer
    /// emits a title-cased `## <Heading>` the parser's flat id-compare misses, so the
    /// slice would fail. Single-word ids route around it, so every slot resolves
    /// byte-for-byte to its source prose.
    #[test]
    fn prd_schema_loads_and_single_word_slots_round_trip() {
        let schema = load_schema(PRD_YAML).expect("prd.yaml loads");
        assert_eq!(schema.ty, "prd");
        assert_eq!(schema.location.as_deref(), Some("prds/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        let vision = "A deterministic context compiler for coding agents.";
        let requirements =
            "- Assemble exactly the slices a task needs.\n\n- Own every structural write.";
        let context = "Static rules files drift; this replaces them.";

        let instance = engine::write::Instance {
            title: "Context compiler".to_string(),
            sections: vec![
                engine::write::SectionContent {
                    id: "vision".to_string(),
                    slot: Some(vision.to_string()),
                    ..Default::default()
                },
                engine::write::SectionContent {
                    id: "requirements".to_string(),
                    slot: Some(requirements.to_string()),
                    ..Default::default()
                },
                engine::write::SectionContent {
                    id: "context".to_string(),
                    slot: Some(context.to_string()),
                    ..Default::default()
                },
            ],
        };

        let bytes = engine::write::render(&schema, &instance);

        // Commit the rendered bytes at the prd's canonical path, then re-read.
        let root = TempRoot::new("round-trip");
        let path = root.0.join("prds").join("context-compiler.md");
        std::fs::create_dir_all(path.parent().unwrap()).expect("mk prds/");
        std::fs::write(&path, &bytes).expect("write committed prd");

        let mut schemas = BTreeMap::new();
        schemas.insert("prd".to_string(), schema);

        for (section, expected) in [
            ("vision", vision),
            ("requirements", requirements),
            ("context", context),
        ] {
            let address =
                Address::parse(&format!("prd:context-compiler#{section}")).expect("valid address");
            let got = engine::store::read_slice(root.0.as_path(), &schemas, &address)
                .expect("conformant prd slice re-parses and resolves");
            // The writer trim_ends prose and the parser trims the span, so the
            // recorded slot bytes are the trimmed prose; the canonical render places
            // it contiguously, so trimmed == the prose itself.
            assert_eq!(
                got,
                expected.trim(),
                "the `{section}` single-word slot round-trips byte-for-byte",
            );
        }
    }

    /// The `implements` ref on `commit` resolves through `field_target` by both
    /// addressing forms: the canonical section-qualified `commit:<slug>#header/implements`
    /// and the flat single-hop alias `commit:<slug>#implements` map identically to
    /// `(header, implements)` (DECISIONS.md 2026-06-01 → M3 Increment 1 T3, review #7).
    #[test]
    fn implements_resolves_section_qualified_and_flat_alias() {
        let schema = load_schema(COMMIT_YAML).expect("commit.yaml loads");

        let canonical = parse_addr("commit:add-rate-limiter#header/implements").expect("valid");
        assert!(
            matches!(
                field_target(&schema, &canonical),
                Some(FieldTarget::Section { section, field }) if section == "header" && field == "implements"
            ),
            "the canonical section-qualified fragment resolves to (header, implements)",
        );

        let alias = parse_addr("commit:add-rate-limiter#implements").expect("valid");
        assert!(
            matches!(
                field_target(&schema, &alias),
                Some(FieldTarget::Section { section, field }) if section == "header" && field == "implements"
            ),
            "the flat single-hop alias resolves identically",
        );
    }

    /// The destination-containment predicate the barrier is built on
    /// (`design/write-commands.md` → The write-time `--task`-scoped barrier): an
    /// in-area staged destination is accepted; a slug that path-escapes the area
    /// (climbs above `tasks/<id>/docs/`) is rejected — the lexical complement to the
    /// M7 join-time isolation check (`design/storage.md` → The by-task-id join).
    #[test]
    fn barrier_predicate_accepts_in_area_and_rejects_escaping() {
        let task_dir = Path::new("/repo/.jigc/tasks/move-cache-to-redis");
        let area = state::instance_path(task_dir, "_", "_")
            .parent()
            .expect("area parent")
            .to_path_buf();

        // The ordinary in-area destination — `tasks/<id>/docs/commit:<id>.md`.
        let in_area = state::instance_path(task_dir, "commit", "move-cache-to-redis");
        assert!(
            within_area(&area, &in_area),
            "a staged instance under the task's own `docs/` is in-area",
        );

        // A slug that path-escapes into a sibling sub-area — refused. The
        // `<type>:` prefix on the filename absorbs one `..`, so escaping the
        // `docs/` boundary takes two leading `..` past it.
        let escaping = state::instance_path(
            task_dir,
            "commit",
            "../../../evict-stale-keys/docs/commit:pwned",
        );
        assert!(
            !within_area(&area, &escaping),
            "a destination that climbs above the area's `docs/` is out-of-area",
        );

        // Escaping the task dir entirely is also refused.
        let far = state::instance_path(task_dir, "commit", "../../../../../../tmp/pwned");
        assert!(
            !within_area(&area, &far),
            "a destination climbing above the repo is out-of-area",
        );
    }
}
