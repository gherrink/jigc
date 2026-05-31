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
use crate::pack::EmbeddedPack;
use crate::render;
use anyhow::{Context, Result, bail};
use engine::address::{Address, Fragment};
use engine::compose::{WorkflowDef, load_workflow_def};
use engine::field_block::Value;
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Schema, SectionBody, load_schema};
use engine::state;
use engine::write::{set_field_validated, set_slot_validated};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
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
    },
    /// Set a field leaf's value (inline, adjudicated at write time).
    SetField {
        /// The leaf address — `<type>:<slug>#<field>` (or `#<section>/<field>`).
        addr: String,
        /// The new value (inline — fields are short + escaping-safe).
        #[arg(long)]
        value: String,
    },
    /// Set a slot leaf's prose (multi-line, via stdin or a file).
    SetSlot {
        /// The slot address — `<type>:<slug>#<slot>`.
        addr: String,
        /// The prose source: a path, or `-` for stdin (prose never inline).
        #[arg(long)]
        from_file: String,
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
            DocCommand::Create { r#type, title } => run_create(cwd, &r#type, &title),
            DocCommand::SetField { addr, value } => run_set_field(cwd, &addr, &value),
            DocCommand::SetSlot { addr, from_file } => run_set_slot(cwd, &addr, &from_file),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(DocFailure::Block(finding)) => {
                let report = engine::result::ValidationReport::new(vec![finding]);
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
fn run_set_field(cwd: &Path, addr: &str, value: &str) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let (section_id, field_key) = field_target(&schema, &address)
        .with_context(|| format!("no field addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address);
    let source = read_staged(&path, addr)?;

    let edited = set_field_validated(
        &schema,
        &source,
        &section_id,
        &field_key,
        &Value::Scalar(value.to_string()),
    )
    .map_err(|f| block(&f, "set-field", addr))?;

    persist(&path, &edited)?;
    Ok(())
}

/// `jigc doc set-slot <addr> --from-file <path|->` — splice slot prose (stdin/file).
fn run_set_slot(cwd: &Path, addr: &str, from_file: &str) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let section_id =
        slot_target(&schema, &address).with_context(|| format!("no slot addressed by `{addr}`"))?;

    let prose = read_handoff(from_file)?;

    let path = staged_path(&task.dir, &address);
    let source = read_staged(&path, addr)?;

    let edited = set_slot_validated(&schema, &source, &section_id, &prose)
        .map_err(|f| block(&f, "set-slot", addr))?;

    persist(&path, &edited)?;
    Ok(())
}

/// `jigc doc create <type> --title <…>` — agent-initiated, create-gated mint.
fn run_create(cwd: &Path, type_name: &str, title: &str) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd)?;
    let schemas = task.schemas()?;
    let gate = task.workflow_gate()?;
    let created = state::create_gated(&task.dir, &schemas, &gate.allows_create, type_name, title)
        .map_err(|f| block(&f, "create", type_name))?;
    println!("{}", created.address);
    Ok(())
}

/// The active task: its working-area directory + the embedded pack to resolve
/// schemas and the workflow gate against.
struct ActiveTask {
    dir: PathBuf,
    pack: EmbeddedPack,
}

impl ActiveTask {
    /// Resolve the active task from `cwd` (`design/write-commands.md` → The
    /// create-gate, step 1: "`.jigc/tasks/<id>/` from cwd"). MVP: the single
    /// active task directory under `<repo>/.jigc/tasks/`; **none** rejects with
    /// the start-a-task route, **more than one** rejects asking for `--task` (the
    /// explicit selector lands with `jigc task`).
    fn resolve(cwd: &Path) -> Result<Self> {
        let repo_root = discover_repo_root(cwd)
            .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
        let tasks = repo_root.join(".jigc").join("tasks");
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
            1 => Ok(Self {
                dir: dirs.pop().expect("one task dir"),
                pack: EmbeddedPack::new(),
            }),
            _ => bail!("more than one active task — name one with `--task <id>`"),
        }
    }

    /// Load the schema for `type_name` from the embedded pack.
    fn schema(&self, type_name: &str) -> Result<Schema> {
        let bytes = self
            .pack
            .read(PackResourceKind::Schemas, &ResourceId::from(type_name))
            .with_context(|| format!("unknown doctype `{type_name}`"))?;
        load_schema(&bytes).with_context(|| format!("the `{type_name}` schema is malformed"))
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
            let schema = load_schema(&bytes)
                .with_context(|| format!("the `{}` schema parses", id.as_str()))?;
            out.insert(schema.ty.clone(), schema);
        }
        Ok(out)
    }

    /// Load the task's workflow definition — the create-gate's `allows-create`
    /// lives on its front-matter. MVP: the single shipped work-workflow,
    /// `single-task` (the cascade has no project workflow override yet).
    fn workflow_gate(&self) -> Result<WorkflowDef> {
        let bytes = self
            .pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from("single-task"),
            )
            .context("the embedded pack is missing `single-task`")?;
        load_workflow_def(&bytes).map_err(finding_to_err)
    }
}

/// Parse an address string, mapping a grammar error to an actionable message.
fn parse_addr(addr: &str) -> Result<Address> {
    Address::parse(addr).with_context(|| format!("malformed address `{addr}`"))
}

/// The staged on-disk path of `address`'s instance within the task working area.
fn staged_path(task_dir: &Path, address: &Address) -> PathBuf {
    state::instance_path(task_dir, address.r#type.as_str(), address.slug.as_str())
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
/// → Content handoff: slots via stdin / `--from-file`, never inline).
fn read_handoff(from_file: &str) -> Result<String> {
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

/// Resolve the `(section_id, field_key)` a `set-field` address targets.
///
/// Two address forms: the single-hop `#<field>` (the MVP worked-example surface —
/// search every section for a field of that id) and the explicit two-hop
/// `#<section>/<field>`.
fn field_target(schema: &Schema, address: &Address) -> Option<(String, String)> {
    match address.fragment.as_ref()? {
        Fragment::Unit(field) => {
            let field = field.as_str();
            schema.sections.iter().find_map(|s| match &s.body {
                SectionBody::Simple { fields, .. } if fields.iter().any(|f| f.id == field) => {
                    Some((s.id.clone(), field.to_string()))
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
                .map(|s| (s.id.clone(), field.to_string()))
        }
        _ => None,
    }
}

/// Resolve the `section_id` a `set-slot` address targets — the simple section
/// whose id is the fragment's leading hop and which declares a `slot`.
fn slot_target(schema: &Schema, address: &Address) -> Option<String> {
    let section_id = match address.fragment.as_ref()? {
        Fragment::Unit(u) => u.as_str(),
        Fragment::UnitLeaf(u, _) => u.as_str(),
        _ => return None,
    };
    schema.sections.iter().find_map(|s| match &s.body {
        SectionBody::Simple { slot: Some(_), .. } if s.id == section_id => Some(s.id.clone()),
        _ => None,
    })
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
