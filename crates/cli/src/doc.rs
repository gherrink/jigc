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
use crate::invocation_log::Outcome;
use crate::pack::make_pack;
use crate::render;
use anyhow::{Context, Result, anyhow, bail};
use engine::address::{Address, Fragment};
use engine::compose::{WorkflowDef, load_workflow_def};
use engine::field_block::Value;
use engine::finding::{Finding, Findings, Location, Severity};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{FieldType, Leaf, Repeatable, Schema, SectionBody};
use engine::state;
use engine::write::{
    set_field_validated, set_item_field_or_insert, set_item_slot, set_nested_item_field_or_insert,
    set_nested_item_slot, set_slot_validated,
};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

/// The `jigc doc <verb>` subcommand tree. Each verb addresses a managed doc in
/// the active task's working area.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum DocCommand {
    /// Mint a new managed instance (agent-initiated, create-gated).
    ///
    /// The title the slug is minted from is supplied inline with `--title` —
    /// always literally `--title`, whatever the doctype's `id-from` field is
    /// named; the CLI mints + places per the schema. The minted slug is the
    /// title slugged lowercase-kebab, capped at the first 5 words / 50 chars
    /// (`--slug` overrides the mint).
    Create {
        /// The doctype to create (e.g. `adr`).
        r#type: String,
        /// The title the slug is minted from — the flag is always literally
        /// `--title`, whatever the doctype's `id-from` field is named
        /// (`design/write-commands.md` → The argument convention).
        #[arg(long)]
        title: String,
        /// Override the minted doc slug (`<type>:<slug>`), decoupling the id from the
        /// title. Taken **verbatim** and validated as a well-formed slug — a malformed
        /// value is rejected, never silently re-slugified (`design/write-commands.md`
        /// → `jigc rename`'s `--slug` precedent). Inert for a singleton doctype (its
        /// slug is fixed to the type id).
        #[arg(long)]
        slug: Option<String>,
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
        #[arg(long)]
        task: Option<String>,
    },
    /// Mint a repeatable item into a section, id-slugged from `--title`.
    ///
    /// The section is addressed `<type>:<slug>#<section>`; the CLI mints the
    /// `{#id}` anchor + appends the item block.
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
    /// Remove a repeatable item (top-level or nested) without discarding the task.
    ///
    /// Addresses a top-level item (`<type>:<slug>#<section>/<id>`) or a nested one
    /// (`#<section>/<parent>/.../<nested-section>/<id>`) — a general recovery verb
    /// over the engine's `remove_item` / `remove_nested_item`.
    RemoveItem {
        /// The item address — top-level `<type>:<slug>#<section>/<id>` or the nested
        /// section-qualified chain `#<section>/<parent>/.../<nested-section>/<id>`.
        addr: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Retitle a repeatable item's heading — its `{#id}` anchor stays frozen.
    ///
    /// The retitle-without-reslug invariant's verb at item level. Addresses the
    /// same item forms as `remove-item`: top-level `<type>:<slug>#<section>/<id>`
    /// or the nested section-qualified chain. An item whose id derives from an
    /// **enum** field (e.g. a changelog change-group's `category`) refuses
    /// unconditionally — a member change is an identity change — routing to
    /// `remove-item` + `add-item` under the target category.
    RetitleItem {
        /// The item address — top-level `<type>:<slug>#<section>/<id>` or the nested
        /// section-qualified chain `#<section>/<parent>/.../<nested-section>/<id>`.
        addr: String,
        /// The new heading title (the item's `{#id}` anchor stays frozen).
        #[arg(long)]
        title: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a field leaf's value (inline, adjudicated at write time).
    ///
    /// For a list-cardinality (`0..*`) ref, set ALL values in one call with the
    /// inline-list form `--value "[a, b, c]"` — repeated single-value calls
    /// replace the whole list (and are rejected once it is populated, to prevent
    /// silently dropping prior entries).
    SetField {
        /// The leaf address — `<type>:<slug>#<field>` (or `#<section>/<field>`).
        addr: String,
        /// The new value (inline — fields are short + escaping-safe). A
        /// list-cardinality (`0..*`) ref takes the inline-list form `"[a, b, c]"` to
        /// set multiple values in one call. Exactly one of `--value` / `--unset` is
        /// required.
        #[arg(long, conflicts_with = "unset", required_unless_present = "unset")]
        value: Option<String>,
        /// Clear the field entirely — remove its line/bullet (an optional field re-conforms
        /// absent). Refused for author-required / defaulted / CLI-`set:` fields.
        #[arg(long)]
        unset: bool,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a slot leaf's prose (multi-line, via stdin or a file).
    ///
    /// Inside slot prose, headings must sit at `####` depth or deeper —
    /// `##`/`###` are schema-reserved, and Setext headings are rejected.
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
    /// Author a whole instance from one declarative payload — the batch verb.
    ///
    /// The doctype-general batch write: applies the equivalent `create` plus N
    /// `add-item` / `set-slot` / `set-field` over a single buffer, persisting once.
    /// In the payload, a slot value is a YAML block scalar wrapped in literal
    /// `<<…>>` markers (`summary: |` then, indented beneath it, `<<the prose>>`) —
    /// required syntax that tags the value as slot prose, not a fill-me
    /// placeholder to delete; the block scalar keeps multi-paragraph and bulleted
    /// prose intact where a quoted flow scalar would fold the line breaks. An
    /// inline field takes a bare value (wrapping one is rejected).
    ///
    /// Reach for `author` to write a whole instance in one shot (a migration, or any
    /// many-leaf doc) — it collapses what would be a `create` + N follow-up calls.
    /// Use `create` then `set-slot`/`set-field`/`add-item` for incremental,
    /// one-leaf-at-a-time authoring instead. Run `author` INSTEAD of those verbs,
    /// never after them — a doc already staged by `create` rejects the second
    /// create `author` implies. (A committed doc is fine: `author` copies it in
    /// and updates it.)
    ///
    /// Payload shape (YAML; `--from-file`), mirroring the document's structure:
    ///
    ///     title: <the create id-source>
    ///     sections:
    ///       - id: <section-id>
    ///         set:                    # doc-level leaves: fields + slots
    ///           status: accepted      # inline field — a bare value
    ///           summary: |
    ///             <<the slot prose>>   # slot — <<…>>-wrapped block scalar
    ///         items:                  # repeatable rows under this section
    ///           - title: <item id-source>
    ///             set:
    ///               date: 2026-07-11
    ///             sections:           # nested repeatable level, parented by the item
    ///               - id: <nested-section-id>
    ///                 set: { note: <<inline slot>> }
    #[command(verbatim_doc_comment)]
    Author {
        /// The doctype to author (e.g. `changelog`) — minted through the create-gate.
        doctype: String,
        /// The payload source: a path, or `-` for stdin (the whole-doc payload is
        /// large, so it arrives the same way slot prose does — never inline).
        #[arg(long = "from-file")]
        from_file: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Read a managed doc, or an addressed `#section`/item/leaf slice of it.
    ///
    /// Served through the canonical parse/render path. **Committed by default**:
    /// task-less, it reads the **committed** store — the view a fresh session or a
    /// teammate on a clone sees.
    /// A doc still staged in an open task is not committed yet; read it with
    /// `--task <id>`, which serves that task's **staged** working copy through the
    /// identical parse/slice path (the read-back of an in-flight write) — including a
    /// **transient** doc's staged copy (`commit:<task-id>`, which never commits to a
    /// repo file). Plain text is the canonical render; `--format json` is the pinned
    /// stable shape (`design/doc-read-surface.md`): a whole-doc object
    /// `{ type, slug, fields, sections }` — a staged serve adds the one `staged` key
    /// carrying the task id — where a slot section serializes to its prose string and
    /// a repeatable section to its item array; a `#section` slice returns that
    /// section's value (item array / slot prose), an `#section/<id>` slice the item
    /// object, an `#section/<id>/<leaf>` slice the leaf.
    Show {
        /// The doc address — `<type>:<slug>`, or a `#section`/item/leaf slice of it.
        addr: String,
        /// Read this open task's **staged** working copy instead of the committed
        /// store (same address, identical parse/slice path).
        #[arg(long)]
        task: Option<String>,
    },
    /// Project a doctype's **resolved** schema (the cascade-composed shape as loaded).
    ///
    /// Injected stamp field included — the third read surface, next to `describe`
    /// (the non-contractual menu) and `doc show` (the committed-content read).
    /// `--format json` is the separately-pinned, explicitly versioned contract
    /// (`contract-version: 3` — `design/doc-read-surface.md` → Why json is a contract
    /// here); plain text is a non-contractual human listing. Task-less — a schema
    /// projection is never task-scoped.
    Schema {
        /// The doctype whose resolved schema to project (e.g. `adr`).
        doctype: String,
    },
    /// List the **committed** store surface by identity, slug-sorted.
    ///
    /// The fourth read surface, next to `describe` (the menu), `doc show` (the
    /// content read) and `doc schema` (the schema read). Every instance of one
    /// doctype (or of every persisted doctype) carries its `<type>:<slug>`
    /// identity, its repo-relative path, and its **registration state**:
    /// `managed` (jigc's own doc) or `unregistered` (a file at a managed home
    /// jigc never adopted — adopt it with `jigc ingest` / `jigc migrate <path>
    /// --as <doctype>`). `--format json` is the pinned shape
    /// `{"docs":[{id, path, state}]}` (no in-band version integer —
    /// `design/doc-read-surface.md` → the fourth read surface). Task-less: it
    /// reads the committed store, never an open task's staged buffer.
    List {
        /// The doctype to list (optional — omit to list every persisted doctype).
        doctype: Option<String>,
    },
}

/// A `doc` verb's failure: a write-time **block** (a structured [`Finding`],
/// rendered through `--format` so an agent on `--format json` gets a parseable
/// envelope), or an **orchestration** error (git/IO/usage — the shared
/// operational-error funnel: `{"error": …}` under `--format json`, plain text
/// otherwise). The
/// blocking finding is the same envelope the rest of the CLI uses; only the
/// happy-path *output* of `doc` stays plain (the staged buffer / new address).
enum DocFailure {
    /// Boxed because [`Finding`] is much larger than the orchestration variant
    /// (the `large_enum_variant` lint — it grew when `route` became the
    /// kind-carrying [`engine::finding::Route`]).
    Block(Box<Finding>),
    Orchestration(anyhow::Error),
}

impl DocFailure {
    /// A blocking finding, boxed into the [`DocFailure::Block`] arm.
    fn block(finding: Finding) -> Self {
        DocFailure::Block(Box::new(finding))
    }
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
    /// an orchestration error to stderr through the shared operational-error funnel.
    pub fn dispatch(self, cwd: &Path, format: Format) -> Outcome {
        let result = match self {
            DocCommand::Create {
                r#type,
                title,
                slug,
                task,
            } => run_create(
                cwd,
                &r#type,
                &title,
                slug.as_deref(),
                task.as_deref(),
                format,
            ),
            DocCommand::AddItem { addr, title, task } => {
                run_add_item(cwd, &addr, &title, task.as_deref(), format)
            }
            DocCommand::RemoveItem { addr, task } => {
                run_remove_item(cwd, &addr, task.as_deref(), format)
            }
            DocCommand::RetitleItem { addr, title, task } => {
                run_retitle_item(cwd, &addr, &title, task.as_deref(), format)
            }
            DocCommand::SetField {
                addr,
                value,
                unset,
                task,
            } => {
                if unset {
                    run_unset_field(cwd, &addr, task.as_deref(), format)
                } else {
                    // `value` is `Some` whenever `--unset` is absent (clap's
                    // `required_unless_present`), so a bare `--value` is safe to unwrap.
                    run_set_field(
                        cwd,
                        &addr,
                        &value.unwrap_or_default(),
                        task.as_deref(),
                        format,
                    )
                }
            }
            DocCommand::SetSlot {
                addr,
                from_file,
                task,
            } => run_set_slot(cwd, &addr, &from_file, task.as_deref(), format),
            DocCommand::Author {
                doctype,
                from_file,
                task,
            } => run_author(cwd, &doctype, &from_file, task.as_deref(), format),
            DocCommand::Show { addr, task } => run_show(cwd, &addr, task.as_deref(), format),
            DocCommand::Schema { doctype } => run_schema(cwd, &doctype, format),
            DocCommand::List { doctype } => run_list(cwd, doctype.as_deref(), format),
        };
        match result {
            Ok(()) => Outcome::success(),
            Err(DocFailure::Block(finding)) => {
                // The write-time block is not an inventory check, so the severity
                // post-pass is a no-op over it; a no-delta cascade keeps it byte-identical.
                let resolved = match crate::cascade_util::no_delta_resolved() {
                    Ok(resolved) => resolved,
                    Err(err) => {
                        eprintln!("{}", render::operational_error(format, &err));
                        return Outcome::failure();
                    }
                };
                let report = engine::result::ValidationReport::new(vec![*finding], &resolved);
                eprint!("{}", render::validation(format, &report));
                if format != Format::Json {
                    eprintln!();
                }
                Outcome::with_findings(1, &report.findings)
            }
            Err(DocFailure::Orchestration(err)) => {
                eprintln!("{}", render::operational_error(format, &err));
                Outcome::failure()
            }
        }
    }
}

/// The **machine-maintained refusal** (A18; `design/team-ready-state.md` → The record is
/// not writable through the `jigc doc` verbs): a `milestone-record` has **no author-owned
/// leaf** — every leaf is `set:`-bearing (no prose slot, no author-required field) — so no
/// `jigc doc` **write** verb has anything legitimate to write. Unguarded, a `--task`-less
/// `jigc doc set-field milestone-record:<id>#status --value joined` exits 0: it silently
/// auto-selects the sole live task (a milestone sub-task), copies the committed record
/// into that area, and stages it to promote at that sub-task's finalize — the committed
/// record mutated outside the milestone verbs, by a verb that never consults milestone
/// state.
///
/// Keyed on the doctype like its already-shipped siblings (`jigc rename`'s reslug
/// refusal, `jigc doc retitle-item`'s item refusal — which carries its own, item-specific
/// wording), sharing their `write.machine-maintained` code, and fired **before any bytes
/// are read or copied in** (the `read_or_copy_in` staging is itself part of the defect).
/// The **read** verbs (`doc show` / `doc schema`) stay open — that uniformity is why the
/// record is a doctype rather than a raw-JSON island.
///
/// `target` is the refusal's declared key target: the **doc URI** (parsed normal form) for
/// an address-bearing write verb, the **bare doctype id** for `create` / `author`
/// (`design/command-output-contract.md` → the form table).
fn machine_maintained_guard(doctype: &str, verb: &str, target: &str) -> Result<(), DocFailure> {
    if doctype != crate::milestone::MILESTONE_RECORD_TYPE {
        return Ok(());
    }
    Err(DocFailure::block(Finding::graded(
        Severity::Blocking,
        "write.machine-maintained",
        format!(
            "{verb} rejected: `{target}` is a milestone-record — the record is \
             machine-maintained (every leaf is CLI-`set:`, so it carries no author-owned \
             slot or field), and no `jigc doc` write applies to it"
        ),
        Some(Location::addressed(target, 1, 1)),
        Some(
            "leave the record to the milestone verbs — `jigc milestone create` opens it, \
             `jigc milestone add-task` appends sub-tasks, `jigc milestone finalize` joins \
             it and advances every sub-task's status, and `jigc milestone discard` settles \
             an abandoned one; read it with `jigc doc show`"
                .into(),
        ),
    )))
}

/// `jigc doc set-field <addr> --value <v>` — adjudicate + splice a field value.
fn run_set_field(
    cwd: &Path,
    addr: &str,
    value: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    // The **parsed** address in URI normal form — the target every block on this write keys
    // at (`design/command-output-contract.md` → the `write.*` row). Never the raw `addr`: a
    // bare singleton head is legal at the verb boundary, so `vision#thesis` would key a
    // slug-less address no driver can resolve.
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-field", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)
        .with_context(|| format!("no field addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target (before `target` is consumed by the apply) + the written
    // value shaped through the same scalar/list grammar the read path re-parses it with,
    // so a write-ack and a `doc show` read-back agree on shape (a `0..*` bracket-list
    // value → a JSON array; `design/command-output-contract.md` §2).
    let ack_target = field_ack_target(&address, &target);
    let value_json = field_json(&engine::field_block::parse_value(value));

    let edited = apply_field_target(&schema, &source, target, &uri, value)
        .map_err(|e| repoint_empty_value(e, addr, value))?;

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    // Confirm the landed value — the positive ack the silent verb was missing.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Field {
                address: addr.to_string(),
                target: ack_target,
                value: value_json,
                findings,
            },
        )
    );
    Ok(())
}

/// Splice a resolved field write into the in-memory `source`, returning the edited
/// buffer — the source→source transform shared by the per-leaf `set-field` verb and
/// the batch `doc author` apply (the chain-the-primitives B1 path). It does **no**
/// I/O: the caller reads the buffer (per-leaf: from the staged file; batch: the
/// running in-memory buffer) and persists the result.
fn apply_field_target(
    schema: &Schema,
    source: &str,
    target: FieldTarget,
    uri: &str,
    value: &str,
) -> Result<String, DocFailure> {
    // The set-field id-from guard (`design/write-commands.md` → The set-field id-from
    // guard): a heading-derived field is never written through `set-field` — living
    // here, the per-leaf verb AND the `doc author` batch (via `apply_leaf`) inherit
    // the reject in one place, killing the or-insert corruption shapes.
    if let Some(finding) = id_from_field_guard(schema, &target, uri, value) {
        return Err(DocFailure::block(finding));
    }
    Ok(match target {
        FieldTarget::Section { section, field } => set_field_validated(
            schema,
            source,
            &section,
            &field,
            &Value::Scalar(value.to_string()),
        )
        .map_err(|f| block(&f, "set-field", uri))?,
        FieldTarget::Item {
            section,
            item,
            field,
        } => set_item_field_or_insert(schema, source, &section, &item, &field, value)
            .map_err(|e| block(&engine::write::generate_error_finding(&e), "set-field", uri))?,
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            set_nested_item_field_or_insert(schema, source, &section, &item_ids, &field, value)
                .map_err(|e| block(&engine::write::generate_error_finding(&e), "set-field", uri))?
        }
    })
}

/// `jigc doc set-field <addr> --unset` — clear a field: remove its line/bullet (an
/// optional field re-conforms absent). The `--unset` sibling of [`run_set_field`], routed
/// here when the flag is set. The eligibility guard (author-required / defaulted / `set:`
/// fields refused) lives in the engine's `unset_*_validated`.
fn run_unset_field(
    cwd: &Path,
    addr: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-field --unset", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)
        .with_context(|| format!("no field addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let ack_target = field_ack_target(&address, &target);
    let edited = apply_unset_target(&schema, &source, target, &uri)?;

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::UnsetField {
                address: addr.to_string(),
                target: ack_target,
                findings,
            },
        )
    );
    Ok(())
}

/// Apply a resolved `--unset` (field removal) to `source`, dispatching by target kind to
/// the engine's byte-stable splice-remove — the `--unset` counterpart to
/// [`apply_field_target`]. An ineligible-field / absent-field engine [`Finding`] surfaces
/// through the shared block envelope (its guard route preserved, a routeless splice error
/// given the generic retry route).
fn apply_unset_target(
    schema: &Schema,
    source: &str,
    target: FieldTarget,
    uri: &str,
) -> Result<String, DocFailure> {
    let map = |f: &Finding| block(f, "set-field", uri);
    Ok(match target {
        FieldTarget::Section { section, field } => {
            engine::write::unset_field_validated(schema, source, &section, &field)
                .map_err(|f| map(&f))?
        }
        FieldTarget::Item {
            section,
            item,
            field,
        } => engine::write::unset_item_field_validated(schema, source, &section, &[&item], &field)
            .map_err(|f| map(&f))?,
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::unset_item_field_validated(schema, source, &section, &item_ids, &field)
                .map_err(|f| map(&f))?
        }
    })
}

/// Repoint an empty-`--value` write reject at the `--unset` verb: `--value ""` is the
/// clear-a-field footgun (the opaque-scalar floor rejects an empty string), so its route
/// names `jigc doc set-field <addr> --unset` — the actual way to clear a field. A
/// non-empty value, or a non-block failure, is passed through untouched.
fn repoint_empty_value(failure: DocFailure, addr: &str, value: &str) -> DocFailure {
    if value.is_empty()
        && let DocFailure::Block(mut finding) = failure
    {
        finding.route = Some(format!(
            "to clear a field, use `jigc doc set-field {addr} --unset` (an empty value is not a clear)"
        ).into());
        return DocFailure::Block(finding);
    }
    failure
}

/// The set-field id-from guard (`design/write-commands.md` → The set-field id-from
/// guard; DECISIONS.md 2026-07-10 → M40 Settle #7). A repeatable item's `id-from`
/// field is **heading-derived** — its value lives in the item heading line, not a
/// field bullet — so `set-field` on it previously or-inserted a contradictory bullet
/// under the heading (three live corruption shapes: non-reparseable wedging · silent
/// divergence committed clean · a recovery trap loop). Rejected with a blocking
/// finding whose route is **type-aware**: a plain-string id-from names
/// `jigc doc retitle-item <item-addr>` (the heading verb, anchor frozen); an enum
/// id-from names `remove-item` + `add-item` under the target category (a category
/// change is an identity change — the [`retitle_enum_refusal`] route). Resolves the
/// destination repeatable with the same navigation as that guard (section lookup /
/// [`engine::write::nested_repeatable`]); a non-id-from field, a section-level
/// target, or an unresolvable chain yields `None` — the inert path (the engine
/// splice adjudicates presence/shape as before).
fn id_from_field_guard(
    schema: &Schema,
    target: &FieldTarget,
    uri: &str,
    value: &str,
) -> Option<Finding> {
    use engine::schema::FieldType;

    // The doc head (`<type>:<slug>`) the route addresses — and the finding's own key
    // target — are rebuilt from.
    let doc = doc_head(uri);
    let (repeatable, field, item_path, dest) = match target {
        // No shipped simple section carries an id-from; the guard is item-scoped.
        FieldTarget::Section { .. } => return None,
        FieldTarget::Item {
            section,
            item,
            field,
        } => {
            let body = &schema.sections.iter().find(|s| &s.id == section)?.body;
            let SectionBody::Repeatable { repeatable } = body else {
                return None;
            };
            (
                repeatable.clone(),
                field,
                format!("{section}/{item}"),
                format!("{doc}#{section}"),
            )
        }
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            // `items` is the parent-scoped chain down to the item: parents ++
            // [nested-section, item-id] (the [`field_target`] Deep contract).
            let (_, rest) = items.split_last()?;
            let (nested_section, parents) = rest.split_last()?;
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            let repeatable =
                engine::write::nested_repeatable(schema, section, &parent_ids, nested_section)?;
            (
                repeatable,
                field,
                format!("{section}/{}", items.join("/")),
                format!("{doc}#{section}/{}/{nested_section}", parents.join("/")),
            )
        }
    };
    if *field != repeatable.id_from {
        return None;
    }
    // The type-aware route: milestone-record → the machine-maintained milestone verbs
    // (NEVER `retitle-item`, which refuses on the record — the divergence-producing
    // command must not be routed to); enum id-from → identity change (remove + re-add
    // under the target category); string (or heading-implicit) id-from → the retitle
    // verb.
    let declared = repeatable.block.iter().find_map(|leaf| match leaf {
        engine::schema::Leaf::Field(f) if f.id == repeatable.id_from => Some(f),
        _ => None,
    });
    let route = if schema.ty == crate::milestone::MILESTONE_RECORD_TYPE {
        format!(
            "the milestone-record is machine-maintained — `{field}` mirrors the \
             sub-task's work-unit id and changes only through the milestone verbs \
             (`jigc milestone add-task` / `jigc milestone finalize`), never a manual write"
        )
    } else if declared.is_some_and(|f| f.ty == FieldType::Enum) {
        // The `remove-item` span goes through the checked constructor (F4 — the route-fence
        // seam-sweep); the `add-item … --title "<value>"` span stays inline (its quoted,
        // possibly multi-word title is not a flat-argv token). Text is unchanged.
        let remove_addr = format!("{doc}#{item_path}");
        let remove = engine::finding::Route::mechanical(
            ["jigc", "doc", "remove-item", remove_addr.as_str()],
            "",
        );
        format!(
            "run {remove} then `jigc doc add-item {dest} \
             --title \"{value}\"` under the target category, moving the prose in the \
             same motion"
        )
    } else {
        format!(
            "run `jigc doc retitle-item {doc}#{item_path} --title \"{value}\"` — the \
             heading retitles with its `{{#id}}` anchor frozen"
        )
    };
    Some(Finding::graded(
        Severity::Blocking,
        "write.id-from-field",
        format!(
            "set-field rejected: `{field}` is the heading-derived id-from field of item \
             `{item_path}` — its value lives in the item heading, not a field bullet"
        ),
        // The FULL URI, not the bare `<section>/<item>/<field>` fragment it emitted — a
        // half-normalized target is the same broken key one step short
        // (`design/command-output-contract.md` → the `write.*` row).
        Some(Location::addressed(
            format!("{doc}#{item_path}/{field}"),
            1,
            1,
        )),
        Some(route.into()),
    ))
}

/// `jigc doc set-slot <addr> --from-file <path|->` — splice slot prose (stdin/file).
fn run_set_slot(
    cwd: &Path,
    addr: &str,
    from_file: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-slot", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        slot_target(&schema, &address).with_context(|| format!("no slot addressed by `{addr}`"))?;

    let prose = read_handoff(from_file)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the apply.
    let ack_target = slot_ack_target(&address, &target);

    let edited = apply_slot_target(&schema, &source, target, &uri, &prose)?;

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    // Confirm the spliced prose by length — the slot bytes are too large to echo.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Slot {
                address: addr.to_string(),
                target: ack_target,
                chars: prose.chars().count(),
                findings,
            },
        )
    );
    Ok(())
}

/// Splice resolved slot prose into the in-memory `source`, returning the edited
/// buffer — the source→source transform shared by the per-leaf `set-slot` verb and
/// the batch `doc author` apply. No I/O (the [`apply_field_target`] sibling).
fn apply_slot_target(
    schema: &Schema,
    source: &str,
    target: SlotTarget,
    uri: &str,
    prose: &str,
) -> Result<String, DocFailure> {
    Ok(match target {
        SlotTarget::Section(section) => set_slot_validated(schema, source, &section, prose)
            .map_err(|f| block(&f, "set-slot", uri))?,
        SlotTarget::Item {
            section,
            item,
            leaf,
        } => set_item_slot(schema, source, &section, &item, &leaf, prose)
            .map_err(|e| block(&engine::write::splice_error_finding(&e), "set-slot", uri))?,
        SlotTarget::NestedItem {
            section,
            items,
            leaf,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            set_nested_item_slot(schema, source, &section, &item_ids, &leaf, prose)
                .map_err(|e| block(&engine::write::splice_error_finding(&e), "set-slot", uri))?
        }
    })
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
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "add-item", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        add_item_target(&address).with_context(|| format!("no section addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the apply: `section` +
    // the **minted** leaf-most item id (the new item — contract §2).
    let ack_target = add_item_ack_target(&address, &target, title);

    let (edited, minted_path) =
        apply_add_item_target(&schema, &source, target, &uri, title, task.is_migration()?)?;

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    // The minted item address — the next address an agent fills the item's slot/field
    // at (the same slugify the engine mints the `{#id}` from, never re-spelled).
    let ack_address = format!(
        "{}:{}#{}",
        address.r#type.as_str(),
        address.slug.as_str(),
        minted_path,
    );
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::AddedItem {
                address: ack_address,
                target: ack_target,
                findings,
            },
        )
    );
    Ok(())
}

/// Mint a repeatable item into the resolved `target` over the in-memory `source`,
/// returning `(edited buffer, minted item fragment)` — the source→source transform
/// shared by the per-leaf `add-item` verb (which prints the minted fragment) and the
/// batch `doc author` apply (which discards it). No I/O (the [`apply_field_target`]
/// sibling). The minted fragment is the canonical section-qualified chain.
///
/// `migration` suppresses the `set: on-create` **date** stamp (`design/auto-migration.md`
/// → Hardening #6): a release migrated from a *dateless* foreign file must render with
/// **no date** rather than fabricating the migration day as false history — so in
/// migration mode the date stamp (and only the date stamp) is dropped, top-level AND
/// nested, and an absent `set: on-create` date finalizes clean (it is not
/// author-required). The scoping is date-specific by predicate, not a blanket drop of
/// every create-time field: a non-date `set: on-create`/`default` leaf (none ships on the
/// changelog release block today, but M25 generalizes this path to adr/spec/prd) still
/// materializes under migration. Authoring (`migration = false`) keeps stamping today; an
/// explicit `set-field date` is a separate leaf, unaffected either way.
fn apply_add_item_target(
    schema: &Schema,
    source: &str,
    target: AddItemTarget,
    uri: &str,
    title: &str,
    migration: bool,
) -> Result<(String, String), DocFailure> {
    // Write-time id-from-enum reject (`design/auto-migration.md` → Hardening #3;
    // write-commands.md → Two check times): when the destination repeatable's `id-from`
    // is an enum the `--title` re-slugs outside, block here — fast feedback at the point
    // of the mistake, not deferred to finalize. The batch (`apply_leaf`) inherits this
    // by sharing this path.
    if let Some(finding) = id_from_enum_block(schema, doc_head(uri), &target, title) {
        return Err(DocFailure::block(finding));
    }
    Ok(match target {
        AddItemTarget::TopLevel { section } => {
            // Materialize the item block's `set: on-create` fields at mint, mirroring the
            // doc-level on-create contract: a `date` leaf declared `set: on-create` inside
            // the repeatable block is stamped with the current date here (the CLI owns the
            // clock — the engine stays a pure function; `write.rs` names this "the CLI
            // `set: on-create` deriver"). Single-slot / no-on-create items pass no fields.
            // Migration mode drops the date stamp (and only the date stamp — no
            // false-history date) inside the deriver, leaving any other create-time
            // field materializing normally.
            let on_create = on_create_item_fields(schema, &section, migration);
            let edited = engine::write::add_item(schema, source, &section, title, None, &on_create)
                .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", uri))?;
            let minted = format!("{}/{}", section, engine::slug::slugify(title));
            (edited, minted)
        }
        AddItemTarget::Nested {
            section,
            parents,
            nested_section,
        } => {
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            // Materialize the nested block's `set: on-create` fields at mint, symmetric
            // with the top-level branch above (which materializes via
            // `on_create_item_fields`) — a `date` leaf declared `set: on-create` inside
            // the nested block is stamped here. A nested block with no such leaf passes
            // none (the shipped `changes` groups carry only `category` + `notes`).
            // Migration mode drops the date stamp (and only the date stamp), symmetric
            // with the top-level branch — inert for the dateless `changes` groups, kept
            // for parity should a nested on-create date leaf ever ship.
            let on_create = on_create_nested_item_fields(
                schema,
                &section,
                &parents,
                &nested_section,
                migration,
            );
            let edited = engine::write::add_nested_item(
                schema,
                source,
                &section,
                &parent_ids,
                &nested_section,
                title,
                None,
                &on_create,
            )
            .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", uri))?;
            // The minted nested item address is the **section-qualified** chain: the
            // section, the parent-scoped id chain, the nested-section name, then the
            // slugger-minted anchor — `#section/parent/.../nested-section/<slug>`. This is
            // the canonical form the design settled (review finding S1,
            // `design/changelog.md` → engine work #1) and the exact address the
            // subsequent `set-slot`/`set-field` accept and the validate findings name, so
            // an agent drives the emitted address verbatim with no re-spelling.
            let minted = format!(
                "{}/{}/{}/{}",
                section,
                parents.join("/"),
                nested_section,
                engine::slug::slugify(title)
            );
            (edited, minted)
        }
    })
}

/// The write-time id-from-enum reject for an `add-item` mint (`design/auto-migration.md`
/// → Hardening #3; write-commands.md → Two check times). Resolves the destination
/// repeatable — the section's own for a top-level mint, the named nested one (via the
/// engine's [`engine::write::nested_repeatable`], the same navigation the mint path uses)
/// for a nested mint — and runs the shared [`engine::validate::id_from_enum_violation`]
/// adjudicator over the `--title`. When the id-from is an enum the title re-slugs outside
/// its members, returns a blocking finding carrying the **shared** code
/// [`engine::validate::ID_FROM_ENUM_CODE`] (identical to finalize's) addressed at the
/// slug-cased id-from address, **qualified by the doc head** `doc` (`<type>:<slug>`) into the
/// URI normal form its stable key targets. A non-enum id-from / a member title yields `None`
/// — the inert path, mirroring finalize's exemption (so `Fixed`→`fixed` passes).
fn id_from_enum_block(
    schema: &Schema,
    doc: &str,
    target: &AddItemTarget,
    title: &str,
) -> Option<Finding> {
    let (repeatable, prefix) = match target {
        AddItemTarget::TopLevel { section } => {
            let body = &schema.sections.iter().find(|s| &s.id == section)?.body;
            let SectionBody::Repeatable { repeatable } = body else {
                return None;
            };
            (repeatable.clone(), section.clone())
        }
        AddItemTarget::Nested {
            section,
            parents,
            nested_section,
        } => {
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            let repeatable =
                engine::write::nested_repeatable(schema, section, &parent_ids, nested_section)?;
            (
                repeatable,
                format!("{section}/{}/{nested_section}", parents.join("/")),
            )
        }
    };
    let slug = engine::validate::id_from_enum_violation(&repeatable, title)?;
    Some(Finding::graded(
        engine::finding::Severity::Blocking,
        engine::validate::ID_FROM_ENUM_CODE,
        format!(
            "add-item rejected: `{slug}` is not an enum member of id-from field `{}`",
            repeatable.id_from
        ),
        Some(Location::addressed(
            format!("{doc}#{prefix}/{slug}/{}", repeatable.id_from),
            1,
            1,
        )),
        // The route floor (M43): the refused mint's repair is a member title, not the
        // generic set-field route the finalize-time sibling of this code carries.
        Some(engine::finding::Route::mechanical(
            ["jigc", "doc", "schema", "<doctype>"],
            " to see the declared members, then re-run `add-item` with a member title",
        )),
    ))
}

/// The resolved destination of an `add-item` address: a **top-level** section
/// (`#section`) or a **nested** repeatable inside a parent item chain
/// (`#section/parent/.../nested-section` — the M22 lift). The trailing hop of a nested
/// target names *which* nested repeatable receives the item; the hops between section
/// and it are the parent-scoped item id chain.
enum AddItemTarget {
    TopLevel {
        section: String,
    },
    Nested {
        section: String,
        parents: Vec<String>,
        nested_section: String,
    },
}

/// Resolve the destination an `add-item` address targets. `#section` (1-hop) is a
/// top-level mint; `#section/parent/nested-section` (3-hop) and deeper name a nested
/// repeatable inside a parent item chain. The CLI only extracts the hops; the engine
/// `add_item` / `add_nested_item` adjudicate the *shape* (repeatability, presence).
fn add_item_target(address: &Address) -> Option<AddItemTarget> {
    match address.fragment.as_ref()? {
        Fragment::Unit(u) => Some(AddItemTarget::TopLevel {
            section: u.as_str().to_string(),
        }),
        // `#section/parent/nested-section`: one parent item, the nested repeatable named
        // by the trailing hop.
        Fragment::UnitItemLeaf(section, parent, nested) => Some(AddItemTarget::Nested {
            section: section.as_str().to_string(),
            parents: vec![parent.as_str().to_string()],
            nested_section: nested.as_str().to_string(),
        }),
        // `#section/parent/.../nested-section`: the leading hop is the section, the
        // trailing hop names the nested repeatable, the hops between are the parent chain.
        Fragment::Deep(hops) => {
            let (section, rest) = hops.split_first()?;
            let (nested_section, parents) = rest.split_last()?;
            if parents.is_empty() {
                return None;
            }
            Some(AddItemTarget::Nested {
                section: section.clone(),
                parents: parents.to_vec(),
                nested_section: nested_section.clone(),
            })
        }
        // A bare `#section/item` (no nested-section hop) addresses no repeatable to mint
        // into; a `#section/leaf` 2-hop likewise names no section to add to.
        Fragment::UnitLeaf(_, _) | Fragment::UnitItem(_, _) => None,
    }
}

/// `jigc doc remove-item <addr>` — remove a repeatable item without discarding the
/// task (a general recovery verb). Clones the `run_add_item` shape: resolve the active
/// task, parse the address, map its fragment to a top-level or nested target, read (or
/// copy-in) the staged instance, call the proven engine `remove_item` /
/// `remove_nested_item` (both byte-stable, including the last/only block), and persist.
/// An absent item / non-repeatable section routes the engine's [`engine::write::SpliceError`]
/// through the shared blocking [`Finding`] mapping (`design/write-commands.md`).
fn run_remove_item(
    cwd: &Path,
    addr: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "remove-item", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        remove_item_target(&address).with_context(|| format!("no item addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the removal match.
    let ack_target = item_ack_target(&address, &target);

    let edited = match target {
        RemoveItemTarget::TopLevel { section, item } => {
            engine::write::remove_item(&schema, &source, &section, &item).map_err(|e| {
                block(
                    &engine::write::splice_error_finding(&e),
                    "remove-item",
                    &uri,
                )
            })?
        }
        RemoveItemTarget::Nested { section, items } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::remove_nested_item(&schema, &source, &section, &item_ids).map_err(
                |e| {
                    block(
                        &engine::write::splice_error_finding(&e),
                        "remove-item",
                        &uri,
                    )
                },
            )?
        }
    };

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    // Confirm the removed item address — the positive ack the silent verb was missing.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::RemovedItem {
                address: addr.to_string(),
                target: ack_target,
                findings,
            },
        )
    );
    Ok(())
}

/// The resolved destination of a `remove-item` address: a **top-level** repeatable item
/// (`#section/id`, removed via `remove_item`) or a **nested** one (`#section/parent/.../
/// nested-section/id`, removed via `remove_nested_item`). For the nested form `items` is
/// the parent-scoped id chain from the section root down to the item being removed (the
/// whole chain after the section), matching the chain the engine locator walks.
enum RemoveItemTarget {
    TopLevel { section: String, item: String },
    Nested { section: String, items: Vec<String> },
}

/// Resolve the item a `remove-item` address targets (shared verbatim by
/// `retitle-item`, whose addresses are the same item forms). The two-hop `#section/id`
/// ([`Fragment::UnitLeaf`]) is a top-level item; a deeper section-qualified chain
/// ([`Fragment::Deep`]) is a nested item — the leading hop is the section, every hop
/// after it is the parent-scoped id chain down to the removed item. The CLI only
/// extracts the hops; the engine `remove_item` / `remove_nested_item` adjudicate
/// presence (an absent item / wrong-parent chain → `SpliceError::NotPresent`).
fn remove_item_target(address: &Address) -> Option<RemoveItemTarget> {
    match address.fragment.as_ref()? {
        Fragment::UnitLeaf(section, item) => Some(RemoveItemTarget::TopLevel {
            section: section.as_str().to_string(),
            item: item.as_str().to_string(),
        }),
        Fragment::Deep(hops) => {
            let (section, items) = hops.split_first()?;
            if items.is_empty() {
                return None;
            }
            Some(RemoveItemTarget::Nested {
                section: section.clone(),
                items: items.to_vec(),
            })
        }
        // A bare `#section` (no item hop) names no item to remove; a `#section/item/leaf`
        // mix without the section-qualified chain is not a remove target.
        Fragment::Unit(_) | Fragment::UnitItem(_, _) | Fragment::UnitItemLeaf(_, _, _) => None,
    }
}

/// `jigc doc retitle-item <addr> --title "<new>"` — retitle a repeatable item's
/// heading with its `{#id}` anchor **frozen** (the retitle-without-reslug invariant's
/// verb at item level; `design/write-commands.md` → `jigc doc retitle-item`). Clones
/// the `run_remove_item` shape — the address forms are the same item forms
/// [`remove_item_target`] resolves — and hands the section-qualified chain to the
/// engine [`engine::write::retitle_item`] heading-line splice.
///
/// The **unconditional enum-id-from refusal** runs first ([`retitle_enum_refusal`]):
/// an item whose id derives from an enum field has its heading AS the enum member and
/// its anchor EQUAL to it, so any member change is an identity change, not a retitle
/// — refused with a blocking finding routing to `remove-item` + `add-item` under the
/// target category (the Settle-decided route), before any bytes are read or moved.
fn run_retitle_item(
    cwd: &Path,
    addr: &str,
    title: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let uri = address.to_string();
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        remove_item_target(&address).with_context(|| format!("no item addressed by `{addr}`"))?;

    // The **milestone-record refusal** (the A4.4 doc-level reslug guard's item-level
    // mirror; `design/write-commands.md` → `jigc doc retitle-item` / Milestone-record
    // reslug): the record is machine-maintained — a `tasks` item's heading IS the
    // sub-task's work-unit id (`id-from: task-id`, a plain string, so the enum refusal
    // below is inert here) — and a retitle would sever the committed record from its
    // work unit while committing clean. Keyed on the doctype like the rename guard,
    // CLI-side like its siblings, before any bytes are read or moved.
    if schema.ty == crate::milestone::MILESTONE_RECORD_TYPE {
        let item_path = match &target {
            RemoveItemTarget::TopLevel { section, item } => format!("{section}/{item}"),
            RemoveItemTarget::Nested { section, items } => {
                format!("{section}/{}", items.join("/"))
            }
        };
        return Err(DocFailure::block(Finding::graded(
            Severity::Blocking,
            "write.machine-maintained",
            format!(
                "retitle-item rejected: item `{item_path}` lives in a milestone-record — \
                 the record is machine-maintained and an item's heading IS the sub-task's \
                 work-unit id, so a retitle would sever the record from its work unit"
            ),
            // The addressed item, in URI normal form — the item's key target.
            Some(Location::addressed(&uri, 1, 1)),
            Some(
                "leave the record to the milestone verbs — `jigc milestone add-task` \
                 appends sub-tasks and `jigc milestone finalize` advances their status; \
                 no manual retitle applies"
                    .into(),
            ),
        )));
    }

    if let Some(finding) = retitle_enum_refusal(&schema, &address, &target, addr, title) {
        return Err(DocFailure::block(finding));
    }

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = match &target {
        RemoveItemTarget::TopLevel { section, item } => {
            engine::write::retitle_item(&schema, &source, section, &[item.as_str()], title)
        }
        RemoveItemTarget::Nested { section, items } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::retitle_item(&schema, &source, section, &item_ids, title)
        }
    }
    .map_err(|e| {
        block(
            &engine::write::generate_error_finding(&e),
            "retitle-item",
            &uri,
        )
    })?;

    persist(&path, &edited)?;
    let findings = write_ack_findings(
        &schema,
        &edited,
        address.r#type.as_str(),
        address.slug.as_str(),
    );
    // Confirm the retitled item address — the anchor (hence the address) is frozen,
    // so the echoed address remains the one every follow-up write lands at.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::RetitledItem {
                address: addr.to_string(),
                target: item_ack_target(&address, &target),
                title: title.to_string(),
                findings,
            },
        )
    );
    Ok(())
}

/// The unconditional enum-id-from refusal for `retitle-item` (`design/write-commands.md`
/// → `jigc doc retitle-item`; DECISIONS.md 2026-07-10 → the REFUSE + remove/add settled
/// fork). Resolves the repeatable the addressed item lives in — the section's own for a
/// top-level item, the named nested one (via the engine's
/// [`engine::write::nested_repeatable`], the [`id_from_enum_block`] navigation) for a
/// chain — and refuses when its `id-from` field is an **enum**, *regardless of the new
/// title* (unlike `add-item`'s membership test): the heading IS the member and the
/// anchor equals it, so a member-to-member change is an identity change, not a retitle.
/// The route names `doc remove-item` on the item + `doc add-item` under the target
/// category, moving the prose in the same motion. A non-enum id-from (arch-doc's
/// `title`, changelog's release `title`) yields `None` — the inert path.
fn retitle_enum_refusal(
    schema: &Schema,
    address: &Address,
    target: &RemoveItemTarget,
    addr: &str,
    title: &str,
) -> Option<Finding> {
    use engine::schema::FieldType;

    // The repeatable the item lives in, its item path (for the finding address), and
    // the add-item destination the route re-mints under.
    let doc = format!("{}:{}", address.r#type.as_str(), address.slug.as_str());
    let (repeatable, item_path, dest) = match target {
        RemoveItemTarget::TopLevel { section, item } => {
            let body = &schema.sections.iter().find(|s| &s.id == section)?.body;
            let SectionBody::Repeatable { repeatable } = body else {
                return None;
            };
            (
                repeatable.clone(),
                format!("{section}/{item}"),
                format!("{doc}#{section}"),
            )
        }
        RemoveItemTarget::Nested { section, items } => {
            // `items` is the parent-scoped chain down to the item: parents ++
            // [nested-section, item-id] (the [`remove_item_target`] contract).
            let (_, rest) = items.split_last()?;
            let (nested_section, parents) = rest.split_last()?;
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            let repeatable =
                engine::write::nested_repeatable(schema, section, &parent_ids, nested_section)?;
            (
                repeatable,
                format!("{section}/{}", items.join("/")),
                format!("{doc}#{section}/{}/{nested_section}", parents.join("/")),
            )
        }
    };
    let field = repeatable.block.iter().find_map(|leaf| match leaf {
        engine::schema::Leaf::Field(f) if f.id == repeatable.id_from => Some(f),
        _ => None,
    })?;
    if field.ty != FieldType::Enum {
        return None;
    }
    Some(Finding::graded(
        Severity::Blocking,
        "write.identity-change",
        format!(
            "retitle-item rejected: item `{item_path}` derives its id from enum field \
             `{}` — a member change is an identity change, not a retitle",
            repeatable.id_from
        ),
        // The FULL URI (the `doc` head is in hand from the parsed address) — a bare
        // fragment is not a stable key (`design/command-output-contract.md`).
        Some(Location::addressed(
            format!("{doc}#{item_path}/{}", repeatable.id_from),
            1,
            1,
        )),
        Some(
            {
                // The `remove-item` span goes through the checked constructor (F4 — the
                // route-fence seam-sweep); the quoted `add-item … --title "<title>"` span
                // stays inline. Text is unchanged.
                let remove =
                    engine::finding::Route::mechanical(["jigc", "doc", "remove-item", addr], "");
                format!(
                    "run {remove} then `jigc doc add-item {dest} \
             --title \"{title}\"` under the target category, moving the prose in the \
             same motion"
                )
            }
            .into(),
        ),
    ))
}

/// One repeatable-block field leaf's create-time materialization, shared by the
/// item and nested-item on-create derivers (the per-item mirror of the doc-level
/// [`on_create_doc_fields`] leaf logic): a `set: on-create` `date` leaf stamps
/// [`today_iso`], a field carrying a literal `default:` stamps that default, any other
/// leaf yields nothing. `migration` drops the date stamp — and **only** the date stamp
/// (the dateless-history concern, `design/auto-migration.md` → Hardening #6, scoped to
/// the date by predicate, not a blanket drop of every create-time field): a non-date
/// `set: on-create`/`default` leaf still materializes under migration, so when M25
/// generalizes this path to adr/spec/prd a non-date create-time field is not silently
/// suppressed.
fn on_create_block_field(
    field: &engine::schema::Field,
    today: &str,
    migration: bool,
) -> Option<engine::field_block::Field> {
    use engine::schema::FieldType;

    // The schema-version stamp (`set: schema-version`) is materialized one level up by
    // [`on_create_doc_fields`] (it needs the doctype's manifest version, which the
    // item/nested derivers that also call this helper never carry), so this shared
    // leaf materializer handles only the `set: on-create` date + literal `default:`.
    let value = if field.ty == FieldType::Date && field.set.as_deref() == Some("on-create") {
        if migration {
            return None;
        }
        today.to_owned()
    } else {
        field.default.clone()?
    };
    Some(engine::field_block::Field {
        key: field.id.clone(),
        value: Value::Scalar(value),
    })
}

/// The create-time fields the repeatable `section_id`'s item block declares, each
/// materialized to its value at mint time via [`on_create_block_field`] — a `set:
/// on-create` `date` (stamped with [`today_iso`], suppressed under `migration`) and any
/// literal `default:` leaf. A non-repeatable / unknown section, or a block with no such
/// leaf, yields no fields (the existing single-slot/no-date `add-item` behavior is
/// unchanged).
fn on_create_item_fields(
    schema: &Schema,
    section_id: &str,
    migration: bool,
) -> Vec<engine::field_block::Field> {
    use engine::schema::Leaf;

    let Some(section) = schema.sections.iter().find(|s| s.id == section_id) else {
        return Vec::new();
    };
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return Vec::new();
    };
    let today = today_iso();
    repeatable
        .block
        .iter()
        .filter_map(|leaf| match leaf {
            Leaf::Field(field) => on_create_block_field(field, &today, migration),
            _ => None,
        })
        .collect()
}

/// The create-time fields the **nested** repeatable named `nested_section_id` (reached
/// by walking the section-qualified `parents` chain from `section_id`) declares, each
/// materialized at mint time via [`on_create_block_field`] (`set: on-create` date —
/// suppressed under `migration` — plus any literal `default:`) — the nested mirror of
/// [`on_create_item_fields`], so a nested `add-item` is symmetric with the top-level one
/// (before this the nested branch passed no fields, silently dropping a nested `set:
/// on-create` date — inert for the shipped changelog `changes` groups, but a latent
/// asymmetry). The schema walk mirrors the engine's nested-block resolution: descend the
/// parent chain's **named** nested-section ids. A section/chain that resolves to no
/// nested repeatable, or a block with no create-time leaf, yields no fields.
fn on_create_nested_item_fields(
    schema: &Schema,
    section_id: &str,
    parents: &[String],
    nested_section_id: &str,
    migration: bool,
) -> Vec<engine::field_block::Field> {
    use engine::schema::Leaf;

    let Some(section) = schema.sections.iter().find(|s| s.id == section_id) else {
        return Vec::new();
    };
    let SectionBody::Repeatable { repeatable } = &section.body else {
        return Vec::new();
    };
    // The named nested-section chain from the section's own block to the target nested
    // repeatable: the parent chain alternates `item, nested-section, …` starting at an
    // item, so the nested-section ids are at odd indices; we then descend by each.
    let mut chain: Vec<&str> = parents.iter().map(String::as_str).collect();
    chain.push(nested_section_id);
    let mut current = repeatable;
    for segment in chain.iter().skip(1).step_by(2) {
        let Some(next) = current.block.iter().find_map(|leaf| match leaf {
            Leaf::Repeatable { id, repeatable } if id == *segment => Some(repeatable),
            _ => None,
        }) else {
            return Vec::new();
        };
        current = next;
    }
    let today = today_iso();
    current
        .block
        .iter()
        .filter_map(|leaf| match leaf {
            Leaf::Field(field) => on_create_block_field(field, &today, migration),
            _ => None,
        })
        .collect()
}

/// The doc-level (header / simple-section) fields a schema declares with a
/// derived value, each materialized at create time before render — the
/// engine-clock-free mirror of the proven item-level [`on_create_item_fields`].
/// Walks the simple-section bodies' `fields` (the header is one such section) for
/// a `type: date, set: on-create` leaf (stamped with [`today_iso`]) and any field
/// carrying a literal `default:` (stamped with that value), in schema field order.
/// A field with neither yields nothing — so a doctype declaring no such field
/// (e.g. `commit`) is left byte-unchanged (the materializer is inert). Honors the
/// `adr` doc-level `date` (set-on-create) + `status: proposed` (default) promises
/// (`design/changelog.md` → engine work #4). `migration` suppresses the
/// `set: on-create` **date** header stamp — and only that stamp (delegating the
/// per-field decision to the proven [`on_create_block_field`]) — so a *dateless*
/// foreign ADR migrates with no date rather than fabricating the migration day as
/// false decision history (`design/auto-migration.md` → Doc-level date-suppression).
/// An explicit payload date still overwrites the (now-absent) stamp on the write path.
///
/// `schema_version` is the value stamped into the doctype's `set: schema-version`
/// header field (M34): the **caller** resolves it from the freeze manifest via
/// [`stamp_schema_version`], so it is `schema`'s own doctype's current manifest
/// version (uniformly 1 for the shipped frozen-v1 set). Unlike the date, it is
/// **not** migration-suppressed — a freshly-adopted foreign doc IS authored against
/// the current version, so the stamp is genuine, never false history
/// (`design/corpus-migration.md` → The schema-version stamp). A doctype carrying no
/// such field (it was not injected — the non-persisted/non-frozen case) ignores it.
fn on_create_doc_fields(
    schema: &Schema,
    migration: bool,
    schema_version: u32,
) -> Vec<engine::field_block::Field> {
    use engine::schema::SCHEMA_VERSION_SET;

    let today = today_iso();
    schema
        .sections
        .iter()
        .filter_map(|section| match &section.body {
            SectionBody::Simple { fields, .. } => Some(fields),
            SectionBody::Repeatable { .. } => None,
        })
        .flatten()
        .filter_map(|field| {
            if field.set.as_deref() == Some(SCHEMA_VERSION_SET) {
                return Some(engine::field_block::Field {
                    key: field.id.clone(),
                    value: Value::Scalar(schema_version.to_string()),
                });
            }
            on_create_block_field(field, &today, migration)
        })
        .collect()
}

/// The schema-version value stamped into a newly-created persisted doc of `doctype` —
/// the **manifest-aware** CLI-side `set: schema-version` deriver
/// (`engine::schema::SCHEMA_VERSION_SET`), the version analog of [`today_iso`]. It reads
/// `doctype`'s current schema-version from the pack's freeze manifest — the same
/// authority the version-aware `migrate`/detector read
/// ([`crate::pack::frozen_doctype_versions`]) — so a newly-created doc is stamped its
/// doctype's *declared* version, never a constant. Falls back to 1 only when `doctype`
/// is absent from the manifest (the unversioned / methodology case), matching the
/// version map's best-effort shape. For the shipped frozen-v1 set every doctype resolves
/// to 1, so the stamp is byte-identical to before; the lookup removes the v2-regime
/// footgun where a manifest bump would otherwise still stamp 1
/// (`design/corpus-migration.md` → The schema-version stamp).
fn stamp_schema_version(pack: &dyn PackSource, doctype: &str) -> u32 {
    crate::pack::frozen_doctype_versions(pack)
        .get(doctype)
        .copied()
        .unwrap_or(1)
}

/// The current UTC date as an ISO `YYYY-MM-DD` string — the CLI-side `set: on-create`
/// date deriver (`write.rs` → `is_iso_date`: "the canonical-date authority is the CLI
/// `set: on-create` deriver"). The engine stays a pure function, so clock access lives
/// CLI-side. Derived from [`SystemTime`](std::time::SystemTime) via the civil-from-days
/// algorithm (Howard Hinnant's `civil_from_days`) — no date-crate dependency, the same
/// `std::time` source the CLI already reads elsewhere.
fn today_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400) as i64; // days since 1970-01-01 (UTC)
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Convert a count of days since the Unix epoch (1970-01-01) to a `(year, month, day)`
/// proleptic-Gregorian civil date — Howard Hinnant's `civil_from_days` (the standard
/// branch-free algorithm), correct for all civil dates. Used only by [`today_iso`].
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// `jigc doc create <type> --title <…>` (optional `--slug`) — agent-initiated,
/// create-gated mint. A `--slug` override drives the minted doc id verbatim
/// (decoupled from the title); it is validated here as a well-formed slug and
/// **never silently re-slugified** (`DECISIONS.md` 2026-07-06 M39 planning → Slug
/// (G6)), then handed to `create_gated` so a colliding override rejects through the
/// settled instance-collision route.
fn run_create(
    cwd: &Path,
    type_name: &str,
    title: &str,
    slug_override: Option<&str>,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    machine_maintained_guard(type_name, "create", type_name)?;
    if let Some(slug) = slug_override
        && !engine::slug::is_slug(slug)
    {
        return Err(DocFailure::Orchestration(anyhow!(
            "`--slug {slug:?}` is not a valid slug — use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"
        )));
    }
    let task = ActiveTask::resolve(cwd, task_id)?;
    let schemas = task.schemas()?;
    let gate = task.workflow_gate()?;
    // Materialize the doctype's doc-level `default:` / `set: on-create` header fields
    // (clock-side CLI work) so the created instance carries them before render. In
    // migration mode the `set: on-create` date is suppressed (no fabricated history).
    let migration = task.is_migration()?;
    let schema_version = stamp_schema_version(task.pack.as_ref(), type_name);
    let on_create = schemas
        .get(type_name)
        .map(|s| on_create_doc_fields(s, migration, schema_version))
        .unwrap_or_default();
    let created = state::create_gated(
        &task.dir,
        &schemas,
        &gate.allows_create,
        type_name,
        title,
        &task.jigc_home,
        &on_create,
        slug_override,
    )
    .map_err(|f| block(&f, "create", type_name))?;
    // A whole-doc create carries only the target head (`doctype`+`slug`) — contract §2.
    let target = whole_doc_ack_target(&created.address)?;
    // A fresh `create` mints the schema-generated skeleton (only the declared sections,
    // any in-location squatter blank-seeded), so it is structurally surplus-free; a
    // copy-in carries the committed body, whose conformance/drift are finalize-
    // preflight's concern — either way the ack's `findings` are always empty
    // (`design/command-output-contract.md` §2). `existed` is the create-or-update
    // discriminator the engine copy-in decides (M43 inc-7 T1).
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Created {
                address: created.address,
                target,
                existed: created.existed,
                findings: Findings::default(),
            },
        )
    );
    Ok(())
}

/// `jigc doc author <doctype> --from-file <payload>` — author a **whole** instance from one
/// declarative payload (`design/write-commands.md` → Batch authoring; `design/
/// auto-migration.md` → Hardening #1). **Path A** (`DECISIONS.md` 2026-06-16, review
/// B1): the create runs through the shared [`state::create_gated`] (so the create-gate
/// is enforced and the in-location-squatter blank-seed fix applies — both live in
/// `state::create`), which persists the empty doc; then each lowered `add-item` /
/// `set-field` / `set-slot` leaf is chained over a **single in-memory buffer** with
/// **no persist between leaves**, persisting **once** at the end. This is exactly the
/// per-leaf verb chain minus the intermediate persists (which are byte no-ops), so
/// byte-stability + the create-gate + the squatter seam are inherited unchanged — it
/// deliberately does **not** build an `Instance` and `render` it. The boundary holds:
/// the agent authors the payload (the prose + which-content-goes-where); the CLI places
/// every leaf.
fn run_author(
    cwd: &Path,
    doctype: &str,
    from_file: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    machine_maintained_guard(doctype, "author", doctype)?;
    let task = ActiveTask::resolve(cwd, task_id)?;
    let payload = read_handoff(from_file)?;
    let schemas = task.schemas()?;
    let gate = task.workflow_gate()?;
    // Parse runs before any persist: a structurally-malformed payload — or a `set`
    // value whose `<<…>>` form contradicts the schema-declared leaf-kind (the
    // silent-misroute guard) — is rejected whole here, nothing staged
    // (`design/write-commands.md` → Batch authoring). The doctype schema is passed for
    // that cross-check; an unknown doctype (absent here) skips it and is rejected by
    // the create-gate below.
    let plan = ::cli::author::parse_author_payload(schemas.get(doctype), &payload)?;
    // Materialize the doctype's doc-level `default:` / `set: on-create` header fields
    // (the same clock-side CLI work `run_create` does) so the created instance carries
    // them before the leaves chain over it. The migration discriminator (reused below
    // for the per-leaf chain) suppresses the `set: on-create` date stamp in migration
    // mode (no fabricated history for a dateless foreign doc).
    let migration = task.is_migration()?;
    let schema_version = stamp_schema_version(task.pack.as_ref(), doctype);
    let on_create = schemas
        .get(doctype)
        .map(|s| on_create_doc_fields(s, migration, schema_version))
        .unwrap_or_default();
    // A migration minted with `jigc migrate … --slug <s>` recorded the override into
    // the task working area (M43 Inc 6 T2); read-if-present and drive the created
    // doc's id verbatim through the same `create_gated` override `doc create --slug`
    // uses. File-presence keying suffices: a migrate workflow's `allows-create` is a
    // single `{type: <target>}` entry, so any gated author here IS the target doctype.
    // Absent on every non-migration task (and a slug-less migrate) — `None`, the
    // title-derived slug, byte-identical to before.
    let slug_override = state::read_slug_override(&task.dir)
        .context("could not read the task's migration slug override")?;
    // The create persists the empty doc through the gated path (gate + squatter seams).
    let created = state::create_gated(
        &task.dir,
        &schemas,
        &gate.allows_create,
        doctype,
        &plan.title,
        &task.jigc_home,
        &on_create,
        slug_override.as_deref(),
    )
    .map_err(|f| block(&f, "author", doctype))?;
    // `create_gated` admitted the doctype, so it is in the loaded set — resolve the
    // schema from there rather than re-reading the pack.
    let schema = schemas
        .get(doctype)
        .expect("create_gated admitted the doctype, so it is in the schema set");

    // Chain every leaf over the single in-memory buffer, no persist between leaves.
    // Atomicity (`design/auto-migration.md` → Hardening #1): `create_gated` already
    // persisted the empty instance before the chain, so a mid-chain leaf failure must
    // **also** discard that staged file — otherwise an empty doc leaks for a batch that
    // "persisted nothing". Rollback = drop the in-memory buffer + remove the staged
    // file `create_gated` provisioned; the block finding propagates unchanged.
    let mut buffer = read_staged(&created.path, &created.address)?;
    for leaf in &plan.leaves {
        match apply_leaf(schema, &buffer, &created.address, leaf, migration) {
            Ok(edited) => buffer = edited,
            Err(failure) => {
                let _ = std::fs::remove_file(&created.path);
                return Err(failure);
            }
        }
    }
    // Persist once: the single write the batch promises.
    persist(&created.path, &buffer)?;
    // A whole-doc author carries only the target head (`doctype`+`slug`) — like create.
    let target = whole_doc_ack_target(&created.address)?;
    let findings = write_ack_findings(schema, &buffer, &target.doctype, &target.slug);
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Authored {
                address: created.address,
                target,
                findings,
            },
        )
    );
    Ok(())
}

/// Apply one lowered batch [`Leaf`](::cli::author::Leaf) over the in-memory `source`,
/// returning the edited buffer. The leaf's `fragment` is the address tail relative to
/// the created instance; prepending `head` (`<doctype>:<slug>`) reconstitutes the full
/// address the existing per-leaf target resolvers (`field_target` / `slot_target` /
/// `add_item_target`) accept verbatim — so the batch reuses the same resolution +
/// splice primitives the per-leaf verbs do (the shared `apply_*_target` helpers).
fn apply_leaf(
    schema: &Schema,
    source: &str,
    head: &str,
    leaf: &::cli::author::Leaf,
    migration: bool,
) -> Result<String, DocFailure> {
    use ::cli::author::Leaf;
    match leaf {
        Leaf::AddItem { fragment, title } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = add_item_target(&address)
                .with_context(|| format!("no section addressed by `{addr}`"))?;
            let (edited, _minted) =
                apply_add_item_target(schema, source, target, &addr, title, migration)?;
            Ok(edited)
        }
        Leaf::SetField { fragment, value } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = field_target(schema, &address)
                .with_context(|| format!("no field addressed by `{addr}`"))?;
            apply_field_target(schema, source, target, &addr, value)
        }
        Leaf::SetSlot { fragment, prose } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = slot_target(schema, &address)
                .with_context(|| format!("no slot addressed by `{addr}`"))?;
            apply_slot_target(schema, source, target, &addr, prose)
        }
    }
}

/// `jigc doc show <ref>` — read a managed doc (or an addressed slice) through the
/// canonical parse path, **committed by default** (`design/team-ready-state.md` → The
/// read surface; `design/doc-read-surface.md` → What it reads, the M43 R7 revision).
/// Task-less, it resolves the cascade schema set + repo root the same way `jigc
/// validate` does and reads the **committed** store; with `--task <id>` it serves that
/// task's **staged** working copy instead ([`run_show_staged`]). Plain text is the
/// [`engine::store::read_slice`] view (the **canonical render of the addressed node**:
/// the whole doc is the store's own bytes, a slice is re-rendered from the parse —
/// `design/doc-read-surface.md` → the retired byte-exactness claim); `--format json` is
/// the pinned stable shape (a 1.0 contract, [`show_json`]). A read-side block (unknown
/// type / not-found / unparseable / a `#fragment` naming nothing) routes through the
/// shared [`DocFailure`] envelope, non-zero exit + route, exactly like a write block.
fn run_show(
    cwd: &Path,
    addr: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    if let Some(task_id) = task_id {
        return run_show_staged(cwd, addr, task_id, format);
    }
    let pack = make_pack()?;
    let address = parse_verb_addr(pack.as_ref(), addr)?;
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let schemas = committed_schemas(pack.as_ref(), &jigc_home)?;
    let read = match format {
        Format::Json => {
            show_json(&jigc_home, &schemas, &address, None).map(|value| render::json(&value))
        }
        Format::Agent | Format::Human => {
            engine::store::read_slice(&jigc_home, &schemas, &address).map_err(DocFailure::block)
        }
    };
    match read {
        Ok(out) => {
            println!("{out}");
            stale_read_hint(&jigc_home, &address);
            Ok(())
        }
        Err(failure) => Err(reroute_unadopted(
            pack.as_ref(),
            &jigc_home,
            &schemas,
            &address,
            failure,
        )),
    }
}

/// The **stale-read hint** (M43; `design/surface-contract.md` → law 2 + the style
/// guide; `design/doc-read-surface.md` → The stale-read hint): a task-less `doc show`
/// serves the **committed** copy, but when the addressed doc is also **staged in an
/// open task**, that serve may be behind the staged working copy — so the read prints
/// one advisory line on **stderr** naming the open task id(s) + the staged-read
/// command. Stdout stays the canonical render / the pinned json, byte-identical (no
/// second additive key rides the committed shape).
///
/// Existence check only, over [`state::list_active_task_ids`] (the single task
/// enumeration source) + [`state::instance_path`] (the one owner of the
/// `docs/<type>:<slug>.md` layout) — no new enumerator, no content read. The
/// staged-read command is a [`engine::finding::Route::mechanical`], so the route
/// fence proves it parses; with more than one staging task, the ids are listed and
/// the command carries the `<task-id>` placeholder (the shared placeholder form).
fn stale_read_hint(jigc_home: &Path, address: &Address) {
    let jigc_root = jigc_home.join(".jigc");
    let tasks = jigc_root.join("tasks");
    let staged_in: Vec<String> = state::list_active_task_ids(&jigc_root)
        .into_iter()
        .filter(|id| {
            state::instance_path(
                &tasks.join(id),
                address.r#type.as_str(),
                address.slug.as_str(),
            )
            .is_file()
        })
        .collect();
    let doc = format!("{}:{}", address.r#type, address.slug);
    let addr = address.to_string();
    let task_arg = match staged_in.as_slice() {
        [] => return,
        [id] => id.as_str(),
        _ => "<task-id>",
    };
    eprintln!(
        "note: `{doc}` is also staged in open task{} {} — the committed copy served here may \
         be stale; staged read: {}",
        if staged_in.len() == 1 { "" } else { "s" },
        staged_in.join(", "),
        engine::finding::Route::mechanical(
            ["jigc", "doc", "show", addr.as_str(), "--task", task_arg],
            "",
        ),
    );
}

/// The **staged arm** of `jigc doc show` — `--task <id>` serves the task's staged
/// working copy through the identical parse/slice path
/// ([`engine::store::read_slice_staged`]; `design/surface-contract.md` → law 2: the
/// staged read). The task resolves via [`ActiveTask::resolve`] (a bad id gets the
/// shared `jigc task list` route), and the schema set is the same cascade-resolved
/// set the committed arm reads against — identical-path applies to schema resolution
/// too. Two conscious differences from the committed arm:
///
/// - **no [`reroute_unadopted`]**: the foreign-adoption discriminator adjudicates the
///   *committed* store; a staged working copy is jigc-written by construction, never
///   a foreign squatter, and its `store.unparseable` route already says "fix the
///   staged working copy";
/// - the whole-doc `--format json` serve carries the one additive `staged` marker key
///   ([`show_json`]).
fn run_show_staged(
    cwd: &Path,
    addr: &str,
    task_id: &str,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, Some(task_id))?;
    let address = parse_verb_addr(task.pack.as_ref(), addr)?;
    let schemas = committed_schemas(task.pack.as_ref(), &task.jigc_home)?;
    let out = match format {
        Format::Json => show_json(
            &task.jigc_home,
            &schemas,
            &address,
            Some((&task.dir, &task.id)),
        )
        .map(|value| render::json(&value))?,
        Format::Agent | Format::Human => {
            engine::store::read_slice_staged(&task.jigc_home, &task.dir, &schemas, &address)
                .map_err(DocFailure::block)?
        }
    };
    println!("{out}");
    Ok(())
}

/// Split the read-side **`store.unparseable`** block's *route* on the **managed-vs-foreign
/// discriminator** (M42, T7; `design/doc-read-surface.md` → `jigc doc list`: *"`doc show`'s
/// block on an unregistered instance routes to adoption, not to hand-repair"*).
///
/// The shipped route — *"fix the committed file so it conforms to its schema"*
/// (`engine::store::read_slice`) — is **true of a corrupted managed doc and a lie about a
/// never-adopted foreign one**: a stock brownfield repo's own Keep-a-Changelog `CHANGELOG.md`
/// squats at the `changelog` placement home, so `doc show changelog:changelog` sent its owner to
/// hand-repair a file jigc never wrote, while `doc list` already called it `unregistered` and
/// `jigc validate` already called it foreign — three surfaces, three stories about one file. The
/// route now splits on the **one** discriminator ([`engine::validate::is_unadopted_foreign`]) and
/// serves the **one** adoption route ([`engine::validate::adoption_route`]), so the three tell one.
///
/// It sits at the **verb boundary**, where the discriminator's inputs (the manifest version map,
/// the shipped prior-version shapes) are in hand: `read_slice`'s signature and the compose-deref
/// path are untouched — this is the read *verb*'s route, not a new engine judgement. Every
/// non-`store.unparseable` failure (and every doc the discriminator adjudicates **managed** —
/// the corrupt-but-stamped ADR) passes through unchanged: a blanket swap would be the
/// mirror-image lie.
fn reroute_unadopted(
    pack: &dyn PackSource,
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &engine::address::Address,
    failure: DocFailure,
) -> DocFailure {
    let DocFailure::Block(mut finding) = failure else {
        return failure;
    };
    // Only the unparseable block can name a foreign file: `is_unadopted_foreign` is `Foreign`
    // exactly when the committed bytes parse against no known version of the schema, so a doc
    // that reads clean (or is missing, or names a transient type) is never this case.
    if finding.code != "store.unparseable" {
        return DocFailure::Block(finding);
    }
    let ty = address.r#type.as_str();
    let Some(schema) = schemas.get(ty) else {
        return DocFailure::Block(finding);
    };
    let Some(path) = engine::store::canonical_path(jigc_home, schema, address.slug.as_str()) else {
        return DocFailure::Block(finding);
    };
    let Ok(bytes) = std::fs::read(&path) else {
        return DocFailure::Block(finding); // read race: the block stands as raised.
    };
    let source = String::from_utf8_lossy(&bytes);
    let versions = crate::pack::frozen_doctype_versions(pack);
    let priors = crate::pack::prior_doctype_schemas(pack, &versions);
    if !engine::validate::is_unadopted_foreign(ty, schema, &source, &versions, &priors) {
        return DocFailure::Block(finding);
    }
    let rel = path
        .strip_prefix(jigc_home)
        .unwrap_or(&path)
        .to_string_lossy()
        .into_owned();
    // The M40 two-tier rule, applied here as the store sweep applies it: name the
    // doctype-directed adoption verb only when the `migrate-<ty>` workflow it composes ships.
    let migratable = pack
        .list(engine::packsource::PackResourceKind::Workflows)
        .iter()
        .any(|id| *id == engine::packsource::ResourceId::from(format!("migrate-{ty}").as_str()));
    finding.route = Some(engine::validate::adoption_route(ty, &rel, migratable).into());
    DocFailure::Block(finding)
}

/// The cascade-resolved schema set keyed by doctype the committed-store read resolves
/// against — the `project > team > pack-default` shadow set with each `location:`
/// nested under the resolved `docs-root` (the `jigc validate` store-read idiom; a
/// placement doctype homes at its literal file, docs-root inert). Reads the composed
/// pack from cwd, so a `[dev ▸ methodology]` repo's `vision`/`milestone-record` resolve
/// alongside the dev doctypes.
fn committed_schemas(pack: &dyn PackSource, jigc_home: &Path) -> Result<BTreeMap<String, Schema>> {
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack, &project_config)?;
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);
    defs.all_schemas(pack)
}

/// `jigc doc schema <doctype>` — project the doctype's **resolved** schema, the
/// third read surface (`design/doc-read-surface.md` → Why json is a contract here;
/// `design/introspection.md` — describe is the non-contractual menu, `doc show` the
/// 1.0-pinned content read, `doc schema` this separately versioned structural
/// projection). Task-less: it resolves the cascade schema set exactly as `doc show`
/// does, then renders the schema **as loaded** — the injected dev-pack
/// schema-version stamp field included — so the projection is what every other
/// surface actually composes against, never the raw YAML. `--format json` is the
/// pinned [`SchemaContract`]; plain text is a non-contractual listing. An unknown
/// doctype blocks with a routed finding, exactly like a read-side `show` block.
fn run_schema(cwd: &Path, doctype: &str, format: Format) -> Result<(), DocFailure> {
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let pack = make_pack()?;
    let schemas = committed_schemas(pack.as_ref(), &jigc_home)?;
    let Some(schema) = schemas.get(doctype) else {
        return Err(DocFailure::block(Finding::graded(
            Severity::Blocking,
            "store.unknown-type",
            format!("unknown doctype `{doctype}`"),
            Some(Location::addressed(doctype, 1, 1)),
            Some("list the available doctypes with `jigc describe`".into()),
        )));
    };
    // The pinned top-level `schema-version`: the doctype's freeze-manifest version
    // when its owning pack declares one (the dev frozen set), else null (the
    // manifest-less methodology doctypes) — never the stamp deriver's 1-fallback.
    let schema_version = crate::pack::frozen_doctype_versions(pack.as_ref())
        .get(doctype)
        .copied();
    match format {
        Format::Json => println!("{}", render::json(&schema_contract(schema, schema_version))),
        Format::Agent | Format::Human => print!("{}", schema_listing(schema, schema_version)),
    }
    Ok(())
}

/// `jigc doc list [<doctype>]` — project the **committed store surface by identity**, the
/// fourth read surface (`design/doc-read-surface.md` → `jigc doc list` — the fourth read
/// surface). `doc show` presupposes you already know a doc exists; nothing answered *"which
/// docs exist"*, so an agent's only route to the corpus was to guess a slug or read the
/// filesystem — the raw read the adapter rule forbids.
///
/// **One primitive, two consumers**: the enumeration is [`engine::index::committed_instances`],
/// the same placement-aware census the store sweep walks (a placement doctype's singleton at
/// its literal file, a located doctype's `<location>/*.md`, a transient doctype nothing), and
/// each row's **registration state** is adjudicated by the one discriminator
/// [`engine::validate::is_unadopted_foreign`] — `managed` (jigc's own doc, by its committed
/// stamp/parse) or `unregistered` (a foreign file squatting at a managed home, the brownfield
/// `CHANGELOG.md` — route: adoption). A second rule here would be a second story about one
/// file; there is exactly one.
fn run_list(cwd: &Path, doctype: Option<&str>, format: Format) -> Result<(), DocFailure> {
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let pack = make_pack()?;
    let schemas = committed_schemas(pack.as_ref(), &jigc_home)?;
    if let Some(ty) = doctype
        && !schemas.contains_key(ty)
    {
        return Err(DocFailure::block(Finding::graded(
            Severity::Blocking,
            "store.unknown-type",
            format!("unknown doctype `{ty}`"),
            Some(Location::addressed(ty, 1, 1)),
            Some("list the available doctypes with `jigc describe`".into()),
        )));
    }
    // The discriminator's two inputs — the manifest version map (its precondition: it answers
    // only for a doctype the CLI stamps) and the shipped prior-version shapes its parse arm
    // reads against — resolved exactly as the store sweep resolves them.
    let versions = crate::pack::frozen_doctype_versions(pack.as_ref());
    let priors = crate::pack::prior_doctype_schemas(pack.as_ref(), &versions);

    // `schemas` is keyed by doctype (BTreeMap → type-sorted) and the enumerator is
    // slug-sorted, so the listing is (type, slug)-sorted by construction.
    let mut docs = Vec::new();
    for (ty, schema) in schemas
        .iter()
        .filter(|(ty, _)| doctype.is_none_or(|want| want == ty.as_str()))
    {
        for (id, path) in engine::index::committed_instances(&jigc_home, ty, schema) {
            let bytes = std::fs::read(&path)
                .with_context(|| format!("reading the committed doc at {path:?}"))?;
            let source = String::from_utf8_lossy(&bytes);
            let state = if engine::validate::is_unadopted_foreign(
                ty, schema, &source, &versions, &priors,
            ) {
                "unregistered"
            } else {
                "managed"
            };
            docs.push(DocRow {
                id,
                path: path
                    .strip_prefix(&jigc_home)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
                state,
            });
        }
    }
    match format {
        Format::Json => println!("{}", render::json(&DocListing { docs: &docs })),
        Format::Agent | Format::Human => {
            if docs.is_empty() {
                // The empty-set line (M43 Inc 7 / T5; `design/surface-contract.md` →
                // the style guide) — zero rows print a stated empty set, never zero
                // bytes (the `task list` empty-roster mold), naming the filtered
                // doctype when one scoped the listing. Exit 0: an empty store is a
                // legitimate state, not an error. JSON keeps the pinned wrapper.
                match doctype {
                    Some(ty) => println!("jigc doc list — no committed `{ty}` docs"),
                    None => println!("jigc doc list — no committed docs"),
                }
            }
            for row in &docs {
                println!("{}  {}  {}", row.id, row.path, row.state);
            }
        }
    }
    Ok(())
}

/// The `jigc doc list --format json` shape — **pinned at ship** with its posture declared
/// (`design/doc-read-surface.md` → the fourth read surface; the M41 lesson: an undeclared
/// output calcifies into a de-facto contract). It is an **index/identity** projection, not a
/// content read, so it rides `doc show`'s posture, **not** `doc schema`'s: **no in-band
/// version integer**, one pinned shape, additive keys permitted **pre-1.0 only**, evolving
/// after the pin solely by an explicitly versioned extension. Golden-pinned in
/// `crates/cli/tests/doc_list.rs`.
///
/// An **object wrapper**, never a bare array: a bare top-level array can never take the
/// additive key the pre-1.0 window permits.
#[derive(serde::Serialize)]
struct DocListing<'a> {
    docs: &'a [DocRow],
}

/// One listed instance: its identity, its repo-relative home, and its registration state.
/// `type`/`slug` are **omitted** — both are derivable from `id`, and a contract does not
/// carry the same fact twice.
#[derive(serde::Serialize)]
struct DocRow {
    /// The `<type>:<slug>` identity — the address every `doc` verb takes.
    id: String,
    /// The instance's repo-relative path.
    path: String,
    /// `managed` | `unregistered` — [`engine::validate::is_unadopted_foreign`]'s verdict.
    state: &'static str,
}

/// The `jigc doc schema --format json` shape — the **separately-pinned, explicitly
/// versioned** contract (`design/doc-read-surface.md` → Why json is a contract
/// here). The keys/structure are the pin (`contract-version` bumps on any
/// structural change to this projection); the *values* track the resolved schemas
/// as they evolve. Golden-pinned at ship (`crates/cli/tests/doc_schema.rs`).
#[derive(serde::Serialize)]
struct SchemaContract<'a> {
    /// The projection's own version — 3 since M43 rc.7 (the settable write-verb
    /// addresses joined the projection; 2 was the M41 rc.5 `of`/`section` join;
    /// bumps on any structural change to these keys).
    #[serde(rename = "contract-version")]
    contract_version: u32,
    /// The doctype id.
    #[serde(rename = "type")]
    ty: &'a str,
    /// The doctype's freeze-manifest schema-version, else null — always emitted.
    #[serde(rename = "schema-version")]
    schema_version: Option<u32>,
    /// Every simple section's fields flattened (header front-matter + body field
    /// groups), in schema-declared order.
    fields: Vec<ContractField<'a>>,
    /// One entry per slot section and per repeatable section, in schema-declared
    /// order (a header/fields-only section contributes to `fields` alone).
    sections: Vec<ContractSection<'a>>,
}

/// One field of the pinned projection: `{id, type, of?, required, author-required,
/// default?, set?, section?, set-field?}`. `required` is presence-in-a-conformant-instance —
/// the author must supply it OR the CLI stamps it (`default:`/`set:`);
/// `author-required` is the shared engine predicate
/// ([`engine::validate::is_author_required`]) — the same authority the create
/// skeleton pre-stamps from, so the mint and this projection can never drift apart
/// (M40 Settle #4). `of` carries an `enum`'s legal members (universal across
/// depths, so an agent reads the legal values without a failed-write probe);
/// `section` names the field's owning simple-section — **top-level only** (an item
/// field carries its section structurally, under `sections[].item`) (M41 rc.5,
/// V3+V6+V11 the `doc author` discoverability cluster); `set-field` is the field's
/// concrete write-verb address (instance parts placeheld: `<slug>`, `<id>`) —
/// **absent** when the field is not directly settable: a `set:`-derived field is
/// CLI-stamped (writing it fights the deriver / the freeze gate), and a block's
/// `id-from` leaf is the item's identity (the heading IS the value — the write-time
/// guard routes to `retitle-item` / remove+add) (M43 rc.7, Settle #10).
#[derive(serde::Serialize)]
struct ContractField<'a> {
    id: &'a str,
    #[serde(rename = "type")]
    ty: &'a FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    of: Option<&'a [String]>,
    required: bool,
    #[serde(rename = "author-required")]
    author_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    section: Option<&'a str>,
    #[serde(rename = "set-field", skip_serializing_if = "Option::is_none")]
    set_field: Option<String>,
}

/// One section of the pinned projection: `{id, kind: "slot"|"repeatable",
/// optional?, set-slot?|add-item?, item?}` — `optional` only on an optional slot
/// section; `set-slot` (the section's write address) only on a slot, `add-item`
/// and `item` only on a repeatable (M43 rc.7: every section entry names its
/// owning write verb by carrying its address under that verb's key).
#[derive(serde::Serialize)]
struct ContractSection<'a> {
    id: &'a str,
    kind: &'static str,
    #[serde(skip_serializing_if = "is_false")]
    optional: bool,
    #[serde(rename = "set-slot", skip_serializing_if = "Option::is_none")]
    set_slot: Option<String>,
    #[serde(rename = "add-item", skip_serializing_if = "Option::is_none")]
    add_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<ContractItem<'a>>,
}

/// A repeatable's item template `{fields, slots, nested}` — **recursive**: each
/// nested repeatable carries its own item (`design/doc-read-surface.md`: the
/// `item` object is recursive for nested repeatables).
#[derive(serde::Serialize)]
struct ContractItem<'a> {
    fields: Vec<ContractField<'a>>,
    slots: Vec<ContractSlot<'a>>,
    nested: Vec<ContractNested<'a>>,
}

/// One item slot: `{id, optional?, set-slot}` — an item slot is always directly
/// settable, so its write address is unconditional.
#[derive(serde::Serialize)]
struct ContractSlot<'a> {
    id: &'a str,
    #[serde(skip_serializing_if = "is_false")]
    optional: bool,
    #[serde(rename = "set-slot")]
    set_slot: String,
}

/// One nested repeatable inside an item: `{id, add-item, item}` (the recursion
/// point; `add-item` is the nested block's write address).
#[derive(serde::Serialize)]
struct ContractNested<'a> {
    id: &'a str,
    #[serde(rename = "add-item")]
    add_item: String,
    item: ContractItem<'a>,
}

/// serde `skip_serializing_if` helper for the skip-on-false optional markers.
fn is_false(value: &bool) -> bool {
    !*value
}

/// Build the pinned [`SchemaContract`] over the resolved `schema`, deterministically
/// in schema-declared order (fields, sections, and item leaves each walk the loaded
/// definition top to bottom — no hash-container order ever reaches the output).
/// Addresses are **type-level**: the instance parts are placeheld (`<slug>` for the
/// doc, `<id>` for each enclosing item's minted id at that depth — the real values
/// come from `doc list` / `doc show`'s item `id` key), so the projection stays a
/// schema read, never an instance read.
fn schema_contract(schema: &Schema, schema_version: Option<u32>) -> SchemaContract<'_> {
    let doc = format!("{}:<slug>", schema.ty);
    let mut fields = Vec::new();
    let mut sections = Vec::new();
    for section in &schema.sections {
        match &section.body {
            SectionBody::Simple {
                slot,
                fields: declared,
            } => {
                fields.extend(declared.iter().map(|field| {
                    // A top-level field carries its owning simple-section id; an item
                    // field carries its section structurally (`contract_item` leaves
                    // `section: None`). No doc-level `id-from` exclusion: a doc's id
                    // source is its H1 title, never a declared header field.
                    let address = field
                        .set
                        .is_none()
                        .then(|| format!("{doc}#{}/{}", section.id, field.id));
                    let mut field = contract_field(field, address);
                    field.section = Some(&section.id);
                    field
                }));
                if let Some(slot) = slot {
                    sections.push(ContractSection {
                        id: &section.id,
                        kind: "slot",
                        optional: slot.optional,
                        set_slot: Some(format!("{doc}#{}", section.id)),
                        add_item: None,
                        item: None,
                    });
                }
            }
            SectionBody::Repeatable { repeatable } => sections.push(ContractSection {
                id: &section.id,
                kind: "repeatable",
                optional: false,
                set_slot: None,
                add_item: Some(format!("{doc}#{}", section.id)),
                item: Some(contract_item(
                    repeatable,
                    &format!("{doc}#{}/<id>", section.id),
                )),
            }),
        }
    }
    SchemaContract {
        contract_version: 3,
        ty: &schema.ty,
        schema_version,
        fields,
        sections,
    }
}

/// Project one schema field into its pinned [`ContractField`]. `set_field` is the
/// caller-computed write address — `None` when the field is not directly settable
/// (a `set:`-derived field, or the block's `id-from` leaf; see [`ContractField`]).
fn contract_field(field: &engine::schema::Field, set_field: Option<String>) -> ContractField<'_> {
    let author_required = engine::validate::is_author_required(field);
    ContractField {
        id: &field.id,
        ty: &field.ty,
        of: field.of.as_deref(),
        required: author_required || field.default.is_some() || field.set.is_some(),
        author_required,
        default: field.default.as_deref(),
        set: field.set.as_deref(),
        // Set by `schema_contract` for a top-level field; item fields stay None.
        section: None,
        set_field,
    }
}

/// Project a repeatable's item template, recursing into nested repeatables.
/// `prefix` is the item's placeheld chain address (`<type>:<slug>#<section>/<id>`,
/// one `/<id>` per depth) — each leaf's write address hangs off it.
fn contract_item<'a>(repeatable: &'a Repeatable, prefix: &str) -> ContractItem<'a> {
    let mut fields = Vec::new();
    let mut slots = Vec::new();
    let mut nested = Vec::new();
    for leaf in &repeatable.block {
        match leaf {
            Leaf::Field(field) => {
                // The block's `id-from` leaf is the item's identity — the heading IS
                // the value, so it is `retitle-item` territory, never `set-field`.
                let address = (field.set.is_none() && field.id != repeatable.id_from)
                    .then(|| format!("{prefix}/{}", field.id));
                fields.push(contract_field(field, address));
            }
            Leaf::Slot { id, slot } => slots.push(ContractSlot {
                id,
                optional: slot.optional,
                set_slot: format!("{prefix}/{id}"),
            }),
            Leaf::Repeatable { id, repeatable } => nested.push(ContractNested {
                id,
                add_item: format!("{prefix}/{id}"),
                item: contract_item(repeatable, &format!("{prefix}/{id}/<id>")),
            }),
        }
    }
    ContractItem {
        fields,
        slots,
        nested,
    }
}

/// The plain (`agent`/`human`) schema listing — **non-contractual** presentation
/// (only the `--format json` shape is the pin): the doctype + its version, then
/// each field (`*` marks author-required) and each section, item leaves indented
/// under their repeatable.
fn schema_listing(schema: &Schema, schema_version: Option<u32>) -> String {
    let contract = schema_contract(schema, schema_version);
    let mut out = match contract.schema_version {
        Some(version) => format!("doctype: {} (schema-version {version})\n", contract.ty),
        None => format!("doctype: {}\n", contract.ty),
    };
    if !contract.fields.is_empty() {
        out.push_str("fields (* = author-required):\n");
        for field in &contract.fields {
            push_field_line(&mut out, field, 1);
        }
    }
    if !contract.sections.is_empty() {
        out.push_str("sections:\n");
        for section in &contract.sections {
            out.push_str(&format!("  - {}: {}", section.id, section.kind));
            if section.optional {
                out.push_str(" (optional)");
            }
            // The section's write-verb address, mirrored from the pinned json
            // (non-contractual presentation — only the json shape is the pin).
            if let Some(addr) = &section.set_slot {
                out.push_str(&format!(" (set-slot: {addr})"));
            }
            if let Some(addr) = &section.add_item {
                out.push_str(&format!(" (add-item: {addr})"));
            }
            out.push('\n');
            if let Some(item) = &section.item {
                push_item_lines(&mut out, item, 2);
            }
        }
    }
    out
}

/// Append one field's listing line at `depth` (two spaces per level). A **top-level**
/// field names its owning simple-section — the field group an agent must address to
/// write it (the `doc author` payload is section-keyed, and `set-field` takes
/// `#<section>/<field>`); an **item** field carries its section structurally (printed
/// indented under its repeatable), so its line stays section-less.
fn push_field_line(out: &mut String, field: &ContractField<'_>, depth: usize) {
    out.push_str(&"  ".repeat(depth));
    out.push_str(&format!("- {}: {}", field.id, field.ty_name()));
    if let Some(members) = field.of {
        out.push_str(&format!(" [{}]", members.join("|")));
    }
    if let Some(section) = field.section {
        out.push_str(&format!(" (section: {section})"));
    }
    if let Some(default) = field.default {
        out.push_str(&format!(" (default: {default})"));
    }
    if let Some(set) = field.set {
        out.push_str(&format!(" (set: {set})"));
    }
    // The field's write address, mirrored from the pinned json (non-contractual
    // presentation); a non-settable field's line stays address-less — inert.
    if let Some(addr) = &field.set_field {
        out.push_str(&format!(" (set-field: {addr})"));
    }
    if field.author_required {
        out.push_str(" *");
    }
    out.push('\n');
}

impl ContractField<'_> {
    /// The field type's bare on-disk spelling (via its serde form — the engine
    /// keeps the spelling private).
    fn ty_name(&self) -> String {
        match serde_json::to_value(self.ty) {
            Ok(serde_json::Value::String(name)) => name,
            _ => String::from("?"),
        }
    }
}

/// Append an item template's leaves at `depth`, recursing into nested repeatables.
fn push_item_lines(out: &mut String, item: &ContractItem<'_>, depth: usize) {
    for field in &item.fields {
        push_field_line(out, field, depth);
    }
    for slot in &item.slots {
        out.push_str(&"  ".repeat(depth));
        out.push_str(&format!("- {}: slot", slot.id));
        if slot.optional {
            out.push_str(" (optional)");
        }
        out.push_str(&format!(" (set-slot: {})", slot.set_slot));
        out.push('\n');
    }
    for nested in &item.nested {
        out.push_str(&"  ".repeat(depth));
        out.push_str(&format!(
            "- {}: repeatable (add-item: {})\n",
            nested.id, nested.add_item
        ));
        push_item_lines(out, &nested.item, depth + 1);
    }
}

/// Build the pinned `--format json` value for `address` (`design/team-ready-state.md` →
/// The read surface — the 1.0 stable contract, a one-way door). Reads the whole doc
/// through the addressed source arm — [`engine::store::read_slice`] task-less,
/// [`engine::store::read_slice_staged`] when `staged` carries a task's `(dir, id)` —
/// which surfaces every doc-level block (unknown type / transient / not-found /
/// not-staged / unparseable) identically to the plain path, re-parses it (guaranteed
/// clean: the read just parsed it), and shapes the value from the parsed structure.
/// For a `#fragment`, a second read validates the fragment resolves so the json path
/// blocks on a bad `#section`/item/leaf exactly as plain does; the value itself is
/// navigated over the parsed structure.
///
/// **The staged marker key** (`design/doc-read-surface.md` → The staged marker key): a
/// **staged whole-doc** serve inserts the one additive top-level key
/// `"staged": "<task-id>"`, so a driver can never mistake a staged read-back for
/// committed state; a committed serve's shape is byte-identical to the pin. A fragment
/// slice is a bare value (prose string / item array / leaf) with no object to hang the
/// key on — the conscious bound, pinned in the design revision.
fn show_json(
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &Address,
    staged: Option<(&Path, &str)>,
) -> Result<serde_json::Value, DocFailure> {
    let read = |a: &Address| match staged {
        Some((task_dir, _)) => engine::store::read_slice_staged(jigc_home, task_dir, schemas, a),
        None => engine::store::read_slice(jigc_home, schemas, a),
    };
    let whole = Address {
        fragment: None,
        ..address.clone()
    };
    let source = read(&whole).map_err(DocFailure::block)?;
    let schema = schemas
        .get(address.r#type.as_str())
        .expect("the read resolved the type, so it is in the schema set");
    let doc = engine::parse::parse_sections(schema, &source)
        .expect("the read already parsed the doc clean");
    match &address.fragment {
        None => {
            let mut value = whole_doc_json(schema, &doc, &source, address);
            if let Some((_, task_id)) = staged {
                value
                    .as_object_mut()
                    .expect("the whole-doc json is an object")
                    .insert(
                        "staged".to_string(),
                        serde_json::Value::String(task_id.to_string()),
                    );
            }
            Ok(value)
        }
        Some(fragment) => {
            read(address).map_err(DocFailure::block)?;
            Ok(fragment_json(schema, &doc, &source, fragment))
        }
    }
}

/// The whole-doc json wrapper `{ type, slug, fields, sections }`. `fields` flattens
/// every simple section's fields (the header's front-matter + any body trailing group)
/// keyed by leaf id; `sections` carries one entry per slot section (its prose string)
/// and per repeatable section (its item array) — a header/fields-only section
/// contributes to `fields` alone. A scalar field serializes as its string, a list-
/// cardinality field as a json array; slot prose is trimmed (the clean machine value —
/// the byte-exact form stays the plain path).
fn whole_doc_json(
    schema: &Schema,
    doc: &engine::parse::Document,
    source: &str,
    address: &Address,
) -> serde_json::Value {
    let mut fields = serde_json::Map::new();
    let mut sections = serde_json::Map::new();
    for section in &schema.sections {
        let parsed = doc.sections.iter().find(|s| s.id == section.id);
        match &section.body {
            SectionBody::Simple { slot, .. } => {
                if let Some(parsed) = parsed {
                    for field in &parsed.fields {
                        fields.insert(
                            field.key.clone(),
                            header_field_json(schema, &field.key, &field.value),
                        );
                    }
                }
                if slot.is_some() {
                    sections.insert(
                        section.id.clone(),
                        slot_json(parsed.and_then(|p| p.slot.as_ref()), source),
                    );
                }
            }
            SectionBody::Repeatable { repeatable } => {
                let items = parsed.map(|p| p.items.as_slice()).unwrap_or(&[]);
                sections.insert(section.id.clone(), items_json(items, repeatable, source));
            }
        }
    }
    serde_json::json!({
        "type": schema.ty,
        "slug": address.slug.as_str(),
        "fields": serde_json::Value::Object(fields),
        "sections": serde_json::Value::Object(sections),
    })
}

/// The json value an addressed `#fragment` slice resolves to (the sub-node of the
/// whole-doc shape): a slot section → its prose string; a repeatable section → its item
/// array; further hops resolve over the **section-qualified write grammar** — the chain
/// alternates `item, nested-section, item, …` (the CLI mirror of the engine's
/// `store::slice_fragment` / `write::physical_item_chain` walk; one canonical address,
/// no second grammar — M40, `design/doc-read-surface.md` → Nested repeatables join the
/// pin): an odd chain ends on an **item** (its object), an even chain on a declared
/// **nested section** (its array of recursive item objects), and a chain minus a
/// trailing **leaf** hop ends on that leaf's value (slot prose / the block's `id-from`
/// → the item's heading / a field, shaped as in the item object). `read_slice` already
/// validated the fragment resolves — a bad hop blocked before this runs — so navigation
/// is infallible: correct node or the honest upstream block, never a wrong node.
fn fragment_json(
    schema: &Schema,
    doc: &engine::parse::Document,
    source: &str,
    fragment: &Fragment,
) -> serde_json::Value {
    let hops = fragment_hops(fragment);
    let (section_id, rest) = hops
        .split_first()
        .expect("a fragment carries a section hop");
    let section = schema
        .sections
        .iter()
        .find(|s| &s.id == section_id)
        .expect("read_slice validated the section");
    let parsed = doc.sections.iter().find(|s| &s.id == section_id);
    match &section.body {
        // A simple section resolves at two depths: the section itself — its slot prose, or
        // (M42, a **fields-only**/header section, which has no slot) an object of its
        // leaves keyed by leaf id, each shaped exactly as the whole-doc `fields` project
        // them — and a **leaf** on its field group (`#section/<leaf>`, the grammar's
        // `unit/leaf` depth). A deeper hop never reaches here: `read_slice` blocked it.
        SectionBody::Simple { slot, .. } => match rest.split_first() {
            None if slot.is_none() => serde_json::Value::Object(
                parsed
                    .map(|p| p.fields.as_slice())
                    .unwrap_or(&[])
                    .iter()
                    .map(|f| (f.key.clone(), header_field_json(schema, &f.key, &f.value)))
                    .collect(),
            ),
            None => slot_json(parsed.and_then(|p| p.slot.as_ref()), source),
            Some((leaf, _)) => parsed
                .and_then(|p| p.fields.iter().find(|f| &f.key == leaf))
                .map(|f| header_field_json(schema, &f.key, &f.value))
                .expect("read_slice validated the section leaf resolves"),
        },
        SectionBody::Repeatable { repeatable } => {
            let items = parsed.map(|p| p.items.as_slice()).unwrap_or(&[]);
            if rest.is_empty() {
                return items_json(items, repeatable, source);
            }
            // 1. The whole hop chain as a write-grammar item path: an odd chain ends
            //    on an item (its object), an even chain on a declared nested section
            //    (its items array) — the engine's first interpretation.
            if let Some(end) = walk_write_chain(repeatable, items, rest) {
                return if end.at_nested_section {
                    items_json(&end.item.items, end.template, source)
                } else {
                    item_json(end.item, end.template, source)
                };
            }
            // 2. The chain minus a trailing leaf hop: the leaf resolves on the chain's
            //    item against the template its block bottoms out in.
            let (leaf, chain) = rest.split_last().expect("rest is non-empty");
            walk_write_chain(repeatable, items, chain)
                .and_then(|end| leaf_json(end.item, end.template, leaf, source))
                .expect("read_slice validated the fragment resolves")
        }
    }
}

/// The terminus of a section-qualified write-grammar hop chain walk.
struct ChainEnd<'a> {
    /// The chain's last traversed item.
    item: &'a engine::parse::ParsedItem,
    /// The repeatable template the chain bottoms out in: the item's own block for an
    /// odd chain, the declared nested block for an even one.
    template: &'a Repeatable,
    /// Whether the chain ended on a nested-section hop (an even chain) — the terminus
    /// is then `item`'s nested items under `template`, not `item` itself.
    at_nested_section: bool,
}

/// Walk `hops` over the parsed `items` under `repeatable` per the write grammar: hops
/// alternate item id, nested-section id, item id, … (starting at an item). Each item
/// id is matched **within its parent's items** (parent-scoped — a same-anchor item
/// under a different parent is never returned) and each nested-section id must name a
/// `Leaf::Repeatable` declared in the current block (never treated as an item id —
/// the engine's S1 rule). `None` when any hop names nothing (the caller then tries
/// the trailing-leaf interpretation; a genuinely bad address never reaches here —
/// `read_slice` blocked it).
fn walk_write_chain<'a>(
    repeatable: &'a Repeatable,
    items: &'a [engine::parse::ParsedItem],
    hops: &[&str],
) -> Option<ChainEnd<'a>> {
    let mut template = repeatable;
    let mut scope = items;
    let mut current = None;
    let mut expect_item = true;
    for hop in hops {
        if expect_item {
            let item = scope.iter().find(|it| &it.id == hop)?;
            scope = &item.items;
            current = Some(item);
        } else {
            template = template.block.iter().find_map(|leaf| match leaf {
                Leaf::Repeatable { id, repeatable } if id == hop => Some(repeatable),
                _ => None,
            })?;
        }
        expect_item = !expect_item;
    }
    Some(ChainEnd {
        item: current?,
        template,
        at_nested_section: expect_item,
    })
}

/// One leaf's json value on `item`, resolved against the `template` its chain bottoms
/// out in (the engine's `store::resolve_leaf` order): a **declared** slot's trimmed
/// prose (declared-only — the block's `id-from` wins over a bare-prose slot), the
/// template's `id-from` leaf → the item's heading (its stable id-source), or a
/// per-item field shaped as in the item object. `None` for a leaf the template does
/// not carry.
fn leaf_json(
    item: &engine::parse::ParsedItem,
    template: &Repeatable,
    leaf: &str,
    source: &str,
) -> Option<serde_json::Value> {
    let declares_slot = template
        .block
        .iter()
        .any(|l| matches!(l, Leaf::Slot { id, .. } if id == leaf));
    if declares_slot && let Some(span) = item.slot_span(leaf) {
        return Some(serde_json::Value::String(
            span.slice(source).trim().to_string(),
        ));
    }
    if template.id_from == leaf {
        return Some(serde_json::Value::String(item.title.clone()));
    }
    item.fields
        .iter()
        .find(|f| f.key == leaf)
        .map(|f| field_json(&f.value))
}

/// Split a [`Fragment`] into its ordered hop strings (section id first) — the read
/// path is purely structural, so it resolves each hop over the parsed data (the CLI
/// mirror of the engine's own `store::fragment_hops`).
fn fragment_hops(fragment: &Fragment) -> Vec<&str> {
    match fragment {
        Fragment::Unit(u) => vec![u.as_str()],
        Fragment::UnitLeaf(u, l) => vec![u.as_str(), l.as_str()],
        Fragment::UnitItem(u, i) => vec![u.as_str(), i.as_str()],
        Fragment::UnitItemLeaf(u, i, l) => vec![u.as_str(), i.as_str(), l.as_str()],
        Fragment::Deep(hops) => hops.iter().map(String::as_str).collect(),
    }
}

/// A repeatable section's items as a json array of item objects.
fn items_json(
    items: &[engine::parse::ParsedItem],
    repeatable: &engine::schema::Repeatable,
    source: &str,
) -> serde_json::Value {
    serde_json::Value::Array(
        items
            .iter()
            .map(|item| item_json(item, repeatable, source))
            .collect(),
    )
}

/// One repeatable item as a json object of its leaves: the item's minted **`id`**, the
/// `id-from` leaf keyed by its id → the item's heading value (its stable id-source),
/// each other field keyed by leaf id (scalar → string, list → array), each slot keyed by
/// leaf id → its trimmed prose, and each declared **nested repeatable** keyed by its
/// block id → the array of recursive item objects (M40, `design/doc-read-surface.md` →
/// Nested repeatables join the pin). A single bare-prose slot carries no leaf id in the
/// parsed item, so the block names it ([`single_slot_leaf`]).
fn item_json(
    item: &engine::parse::ParsedItem,
    repeatable: &engine::schema::Repeatable,
    source: &str,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    // The item's minted, frozen id — the handle every address into the item takes, and
    // NOT derivable from the heading (a release titled `1.0.0` mints `100`; a retitle
    // diverges the two permanently). Without it the contract is not closed under its own
    // address grammar (M42, `design/doc-read-surface.md` → The item `id` closes the json
    // contract). The key is reserved at schema load (`schema::RESERVED_ITEM_ID_KEY`), so
    // no declared leaf below can collide with it.
    map.insert(
        engine::schema::RESERVED_ITEM_ID_KEY.to_owned(),
        serde_json::Value::String(item.id.clone()),
    );
    // The id-from leaf is the item's `###` heading (its id-source), not a bullet field,
    // so it is carried by `item.title` — key it under the block's declared `id-from`.
    map.insert(
        repeatable.id_from.clone(),
        serde_json::Value::String(item.title.clone()),
    );
    for field in &item.fields {
        map.insert(field.key.clone(), field_json(&field.value));
    }
    if !item.slots.is_empty() {
        for (leaf_id, span) in &item.slots {
            map.insert(
                leaf_id.clone(),
                serde_json::Value::String(span.slice(source).trim().to_string()),
            );
        }
    } else if let Some(span) = &item.slot
        && let Some(leaf_id) = single_slot_leaf(repeatable)
    {
        map.insert(
            leaf_id.to_string(),
            serde_json::Value::String(span.slice(source).trim().to_string()),
        );
    }
    // Nested repeatable groups join the item object: each declared nested block keys
    // the item by its block id → the array of recursive item objects. The parse
    // carries one physical nested list (`item.items` — a nested section is a purely
    // logical schema hop), so it is the declared block's items. A flat block declares
    // no nested repeatable and gains no key, so flat item json is byte-unchanged (the
    // additive guard).
    for leaf in &repeatable.block {
        if let Leaf::Repeatable {
            id,
            repeatable: nested,
        } = leaf
        {
            map.insert(id.clone(), items_json(&item.items, nested, source));
        }
    }
    serde_json::Value::Object(map)
}

/// The id of a repeatable block's single bare-prose slot leaf — the key the parsed
/// item's un-keyed `slot` span serializes under (the multi-slot case keys itself via
/// `slots`). `None` for a slot-less block.
fn single_slot_leaf(repeatable: &engine::schema::Repeatable) -> Option<&str> {
    repeatable.block.iter().find_map(|leaf| match leaf {
        engine::schema::Leaf::Slot { id, .. } => Some(id.as_str()),
        _ => None,
    })
}

/// A slot section's json value — its prose, trimmed (the clean machine value; the
/// byte-exact form is the plain path). An unfilled/absent slot is the empty string.
fn slot_json(span: Option<&engine::parse::Span>, source: &str) -> serde_json::Value {
    serde_json::Value::String(
        span.map(|s| s.slice(source).trim().to_string())
            .unwrap_or_default(),
    )
}

/// The doctype (`schema.ty`) and header-field id of the one **compound** field in the
/// pinned json read-surface: the milestone-record's `base` pin. Every other field
/// stays a scalar; only this pair renders as a structured `{ sha, short }` object
/// (`DECISIONS.md` 2026-07-07 → the pinned json renders `milestone-record.base` as
/// structured `{sha, short}`).
const COMPOUND_BASE_DOCTYPE: &str = "milestone-record";
const COMPOUND_BASE_FIELD: &str = "base";

/// A **simple-section header** field's json value. The milestone-record's `base` field
/// is the one compound leaf in the pinned read surface: its stored value is the
/// space-joined `<sha> <short>` scalar the `.md` renders (the lossless round-trip that
/// reconstructs `BasePin`), but json projects it as a structured `{ "sha": …, "short": … }`
/// object so a consumer reads the two SHAs without splitting on an undocumented delimiter
/// (`DECISIONS.md` 2026-07-07; `design/team-ready-state.md` → The read surface). The `.md`
/// scalar + its read-back are unchanged — this is json-projection-only. A malformed base
/// value (no space) degrades to the plain scalar rather than fabricating an empty `short`;
/// the normal path is always `<sha> <short>`. Every other field falls through to the
/// scalar/list [`field_json`].
fn header_field_json(schema: &Schema, key: &str, value: &Value) -> serde_json::Value {
    if schema.ty == COMPOUND_BASE_DOCTYPE
        && key == COMPOUND_BASE_FIELD
        && let Value::Scalar(raw) = value
        && let Some((sha, short)) = raw.split_once(' ')
    {
        return serde_json::json!({ "sha": sha, "short": short });
    }
    field_json(value)
}

/// One field value as json: a scalar → its string (an enum member is already its
/// lowercase string), a list-cardinality value → a json array of its elements.
fn field_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Scalar(s) => serde_json::Value::String(s.clone()),
        Value::List(elems) => serde_json::Value::Array(
            elems
                .iter()
                .cloned()
                .map(serde_json::Value::String)
                .collect(),
        ),
    }
}

/// The intrinsic single-doc advisories a successful managed-write reports as data on its
/// [`render::DocAck`] (`design/command-output-contract.md` §2 — findings-as-data on write):
/// the `schema-conformance.surplus-sections-absent` check — the one pure-function advisory
/// that takes just `(schema, staged-buffer)`, needs no index or subprocess, and is
/// authoring-stage-agnostic (an undeclared trailing section is an anomaly at *any* stage).
/// Computed from the just-persisted `edited` buffer; each finding's [`Location::address`]
/// fragment (the surplus heading's slug) is flipped to the write's URI target
/// (`<type>:<slug>#<fragment>`) so its derived `key.target` reads in URI normal form
/// (`command-output-contract.md` → the stable finding key), the same `path→URI` flip the
/// store-scope `attribute_to_doc` installs. **Completeness** (`repeatable-populated`,
/// `required-slot`) + the cross-doc / subprocess families are deliberately excluded — they
/// answer "is the *corpus* complete," which one mid-authoring write cannot adjudicate — so
/// `findings: []` means "no intrinsic single-doc advisory," not "validated."
fn write_ack_findings(schema: &Schema, edited: &str, doctype: &str, slug: &str) -> Findings {
    let mut findings = engine::validate::surplus_sections_absent(schema, edited);
    engine::finding::readdress_to_uri(&mut findings, &format!("{doctype}:{slug}"));
    findings.into()
}

/// Stamp the doc head (`doctype` + `slug`) of a write-ack's decomposed `target`
/// (`design/command-output-contract.md` §2). The per-verb builders below supply the
/// section/item/leaf depth from their already-resolved write-target enums (no address
/// re-parse); this shared constructor reads the head off the parsed `address`.
fn ack_target(
    address: &Address,
    section: Option<String>,
    item: Option<String>,
    leaf: Option<String>,
) -> render::AckTarget {
    render::AckTarget {
        doctype: address.r#type.as_str().to_string(),
        slug: address.slug.as_str().to_string(),
        section,
        item,
        leaf,
    }
}

/// The decomposed ack target of a resolved `set-field` write: the field id is the leaf;
/// an item-scoped write carries the **leaf-most** item id under `item` (the flat 5-key
/// shape holds at nested depth — the field/slot-vs-section split is read off the resolved
/// target, never hop count).
fn field_ack_target(address: &Address, target: &FieldTarget) -> render::AckTarget {
    let (section, item, leaf) = match target {
        FieldTarget::Section { section, field } => (section.clone(), None, field.clone()),
        FieldTarget::Item {
            section,
            item,
            field,
        } => (section.clone(), Some(item.clone()), field.clone()),
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => (section.clone(), items.last().cloned(), field.clone()),
    };
    ack_target(address, Some(section), item, Some(leaf))
}

/// The decomposed ack target of a resolved `set-slot` write: a section-level slot reaches
/// only the section; an item-level slot carries the leaf-most item id + the slot leaf.
fn slot_ack_target(address: &Address, target: &SlotTarget) -> render::AckTarget {
    let (section, item, leaf) = match target {
        SlotTarget::Section(section) => (section.clone(), None, None),
        SlotTarget::Item {
            section,
            item,
            leaf,
        } => (section.clone(), Some(item.clone()), Some(leaf.clone())),
        SlotTarget::NestedItem {
            section,
            items,
            leaf,
        } => (section.clone(), items.last().cloned(), Some(leaf.clone())),
    };
    ack_target(address, Some(section), item, leaf)
}

/// The decomposed ack target of an `add-item` mint (`design/command-output-contract.md`
/// §2): `section` = the top-level section, `item` = the **minted** leaf-most id
/// (`slugify(title)`, the same anchor the engine mints — the *new* item, per the contract's
/// "add-item's target is the new item, not the bare section"), no leaf. Borrows `target`
/// before the apply consumes it.
fn add_item_ack_target(
    address: &Address,
    target: &AddItemTarget,
    title: &str,
) -> render::AckTarget {
    let section = match target {
        AddItemTarget::TopLevel { section } | AddItemTarget::Nested { section, .. } => {
            section.clone()
        }
    };
    ack_target(
        address,
        Some(section),
        Some(engine::slug::slugify(title)),
        None,
    )
}

/// The decomposed ack target of a freshly created/authored **whole doc** (`create`/`author`):
/// just the head — `doctype` + `slug`, no section/item/leaf — parsed off the minted
/// `<type>:<slug>` address (`design/command-output-contract.md` §2: a whole-doc write carries
/// only the target head). The address just came from `create_gated`, so the parse is
/// infallible in practice; a malformed one funnels through the shared orchestration error.
fn whole_doc_ack_target(address: &str) -> Result<render::AckTarget, DocFailure> {
    let parsed = parse_addr(address)?;
    Ok(ack_target(&parsed, None, None, None))
}

/// The decomposed ack target of a resolved `remove-item` / `retitle-item` write (both
/// resolve through [`RemoveItemTarget`]): section + the leaf-most item id, no leaf.
fn item_ack_target(address: &Address, target: &RemoveItemTarget) -> render::AckTarget {
    let (section, item) = match target {
        RemoveItemTarget::TopLevel { section, item } => (section.clone(), Some(item.clone())),
        RemoveItemTarget::Nested { section, items } => (section.clone(), items.last().cloned()),
    };
    ack_target(address, Some(section), item, None)
}

/// The active task: its working-area directory + the embedded pack to resolve
/// schemas and the workflow gate against.
struct ActiveTask {
    /// The resolved task id (the directory name under `.jigc/tasks/`) — the name
    /// the write-time barrier scopes the staged-doc destination to.
    id: String,
    dir: PathBuf,
    /// **jigc_home** — the main checkout the `.jigc/` working area + committed doc-store
    /// bind to (M31 Inc 2 / WF3). Copy-on-first-touch resolves a base-committed
    /// `<location>/<slug>.md` against it (`canonical_path`); `doc` verbs do no git I/O, so
    /// jigc_home is the single base this surface needs.
    jigc_home: PathBuf,
    pack: Box<dyn PackSource>,
}

impl ActiveTask {
    /// Resolve the active task from `cwd` + an optional explicit `--task <id>`
    /// (`design/write-commands.md` → The write-time `--task`-scoped barrier:
    /// active-task resolution). **Explicit `--task <id>` wins** — it resolves
    /// `<repo>/.jigc/tasks/<id>/` directly, rejecting with the shared task-list
    /// route (`crate::task::no_such_task`) if absent. Else: the **single** active task
    /// directory under `<repo>/.jigc/tasks/`; **none** rejects with the start-a-task
    /// route, **more than one** with no `--task` rejects asking for the selector.
    fn resolve(cwd: &Path, task_id: Option<&str>) -> Result<Self> {
        let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
        let jigc_root = jigc_home.join(".jigc");
        let tasks = jigc_root.join("tasks");

        if let Some(id) = task_id {
            let dir = tasks.join(id);
            if !dir.is_dir() {
                return Err(crate::task::no_such_task(id));
            }
            return Ok(Self {
                id: id.to_string(),
                dir,
                jigc_home,
                pack: make_pack()?,
            });
        }

        // The single enumeration source of truth (`state::list_active_task_ids`) the
        // ambiguous error and `jigc task list` share, so they never disagree.
        let mut ids = state::list_active_task_ids(&jigc_root);
        match ids.len() {
            0 => bail!(
                "no active task — start one with {}",
                engine::finding::Route::mechanical(["jigc", "start"], ""),
            ),
            1 => {
                let id = ids.pop().expect("one task id");
                let dir = tasks.join(&id);
                Ok(Self {
                    id,
                    dir,
                    jigc_home,
                    pack: make_pack()?,
                })
            }
            // Enumerate the live ids so the user can copy one into `--task <id>`
            // (M26 shakedown: the error must name what it asks you to pass).
            _ => bail!(
                "more than one active task — name one with `--task <id>`: {}",
                ids.join(", ")
            ),
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
        if let Some(committed) = engine::store::canonical_path(&self.jigc_home, schema, slug)
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

    /// Whether this task is a **migration** task — minted by `jigc migrate`, the only
    /// writer of the recorded `source-path` (`migrate.rs`; `jigc start` never writes
    /// it). The discriminator the on-create date suppression reads
    /// (`design/auto-migration.md` → Hardening #6): a migration's dateless release must
    /// not fabricate the migration day, so the `set: on-create` stamp is dropped here.
    fn is_migration(&self) -> Result<bool> {
        Ok(state::read_source_path(&self.dir)
            .context("could not read the task's migration source path")?
            .is_some())
    }

    /// Resolve the project cascade for this task — the `docs-root` (and severity)
    /// surface the schema-load applies. A missing project layer resolves to the
    /// pack-default base (the no-override case).
    fn resolved(&self) -> Result<engine::cascade::Resolved> {
        crate::start::resolve_severity_cascade(
            self.pack.as_ref(),
            &self.jigc_home.join(".jigc/config"),
        )
    }

    /// Load the schema for `type_name` from the embedded pack, nesting its `location:`
    /// under the resolved `docs-root` (a schema-load surface — the copy-in resolves
    /// `canonical_path` against this `location:`, so it must match the finalize-promote
    /// write path or warm copy-in reads the wrong dir).
    fn schema(&self, type_name: &str) -> Result<Schema> {
        let bytes = self
            .pack
            .read(PackResourceKind::Schemas, &ResourceId::from(type_name))
            .with_context(|| format!("unknown doctype `{type_name}`"))?;
        let mut schema = crate::pack::load_pack_schema(self.pack.as_ref(), &bytes)
            .with_context(|| format!("the `{type_name}` schema is malformed"))?;
        crate::start::apply_docs_root(&self.resolved()?, std::iter::once(&mut schema));
        Ok(schema)
    }

    /// Load every shipped schema, keyed by doctype — the set `create_gated`
    /// adjudicates the unknown-doctype check against. Nests each `location:` under the
    /// resolved `docs-root` (a schema-load surface, kept consistent with `schema`).
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
        crate::start::apply_docs_root(&self.resolved()?, out.values_mut());
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
            .with_context(|| {
                // The discard span names the concrete task id (the style guide's "the
                // exact next command for the state at hand"), riding the checked
                // constructor like every T7 span.
                format!(
                    "the active task has no recorded workflow — discard it with {} and re-start with {}",
                    engine::finding::Route::mechanical(["jigc", "task", "discard", &self.id], ""),
                    engine::finding::Route::mechanical(["jigc", "start"], ""),
                )
            })?;
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

/// Parse an address string, mapping a grammar error to an actionable message. The pure
/// grammar parse — for addresses the CLI itself composes (already `<type>:<slug>`-headed).
/// A **user-supplied** address arrives through [`parse_verb_addr`] instead.
fn parse_addr(addr: &str) -> Result<Address> {
    Address::parse(addr).with_context(|| format!("malformed address `{addr}`"))
}

/// Parse a **user-supplied** address at a `doc` verb boundary — the schema-aware layer over
/// the pure grammar: a bare **singleton** head (`changelog`, `vision#thesis`) expands to
/// its canonical `<type>:<type>` spelling before the grammar sees it, and any other
/// slug-less address keeps the actionable route (the `<type>:<slug>` form + the
/// `jigc describe` pointer — the M41 V12 repair, which reached `rename` only).
///
/// `engine::address::Address::parse` is **untouched**: the grammar is not widened, this is
/// an expansion at the verb boundary — the one place a human/agent types an address.
fn parse_verb_addr(pack: &dyn PackSource, addr: &str) -> Result<Address> {
    Address::parse(&expand_bare_singleton(pack, addr)).map_err(|err| {
        anyhow!(
            "malformed address `{addr}`: {err} — a doc is addressed as `<type>:<slug>`, \
             e.g. `adr:single-node-cache` (a singleton doctype like `changelog` or `vision` \
             may be named bare)\n  route: run {} for the doctype surface",
            engine::finding::Route::mechanical(["jigc", "describe"], ""),
        )
    })
}

/// Expand a bare **singleton** head to its canonical `<type>:<type>` spelling, carrying any
/// `#fragment` through (`changelog#releases` → `changelog:changelog#releases`); every other
/// address passes through verbatim.
///
/// The singleton set is the **placement** doctypes (`design/storage.md` → Placement): their
/// slug is fixed to the type id by construction — `engine::store::read_slice` refuses every
/// *other* slug for one — so the bare type names the instance unambiguously.
fn expand_bare_singleton(pack: &dyn PackSource, addr: &str) -> String {
    let (head, fragment) = match addr.split_once('#') {
        Some((head, fragment)) => (head, Some(fragment)),
        None => (addr, None),
    };
    if head.contains(':') || !is_singleton_type(pack, head) {
        return addr.to_string();
    }
    match fragment {
        Some(fragment) => format!("{head}:{head}#{fragment}"),
        None => format!("{head}:{head}"),
    }
}

/// Does `ty` name a **placement** doctype — one whose single instance homes at a literal
/// file and whose slug is therefore fixed to the type id? An unknown or malformed doctype
/// answers `false`, so the address falls through to the grammar's own rejection.
fn is_singleton_type(pack: &dyn PackSource, ty: &str) -> bool {
    pack.read(PackResourceKind::Schemas, &ResourceId::from(ty))
        .ok()
        .and_then(|bytes| crate::pack::load_pack_schema(pack, &bytes).ok())
        .is_some_and(|schema| schema.placement.is_some())
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
        Err(DocFailure::block(barrier_block(task_id, address)))
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
        Some(format!("address the doc with a slug inside task `{task_id}`'s own area").into()),
    )
}

/// Read the staged instance bytes, mapping an absent instance to an actionable
/// error (the write verbs require the instance to already exist —
/// `design/write-commands.md` → Instance provisioning).
fn read_staged(path: &Path, addr: &str) -> Result<String> {
    // `map_err`, not `with_context`: the latter chains the raw I/O error's `os error 2`
    // tail into `{err:#}` — a dead end. The absent-instance case is the expected reason
    // this read fails, so surface the provision route alone (M36 Inc-4).
    std::fs::read_to_string(path).map_err(|_| {
        let start = engine::finding::Route::mechanical(["jigc", "start"], "");
        // The full, parseable form — the old bare `jigc doc create` span never parsed
        // (required args short), which the T7 parse fence surfaced and forces honest.
        let create = engine::finding::Route::mechanical(
            ["jigc", "doc", "create", "<type>", "--title", "\"X\""],
            "",
        );
        anyhow!(
            "no staged instance for `{addr}` — provision it first ({start} / {create}). \
             Note: {create} derives the id from the title (`X` → slug), \
             not the task id — address writes at that title-derived id"
        )
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
    /// A **nested** repeatable-item field, addressed by its parent-scoped id chain
    /// (`#section/release/change-group/field` and deeper — the M22 multi-level lift).
    /// `items` is the id chain from the section root; `field` is the trailing leaf.
    NestedItem {
        section: String,
        items: Vec<String>,
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
        // presence) AND the value's declared type — the 2026-06-07 parity gap is closed
        // (M24 inc-2 T2): a malformed item-field value is rejected at the write verb with
        // finalize's `schema-conformance.field-value-conformant` code.
        Fragment::UnitItemLeaf(section, item, field) => Some(FieldTarget::Item {
            section: section.as_str().to_string(),
            item: item.as_str().to_string(),
            field: field.as_str().to_string(),
        }),
        // A bare item hop (`#<section>/<item>`) addresses no field leaf.
        Fragment::UnitItem(_, _) => None,
        // A **nested** path (`#section/item/child/.../field`): the leading hop is the
        // section, the trailing hop is the field leaf, and the hops between are the
        // parent-scoped item id chain the engine locator walks (review finding S1).
        Fragment::Deep(hops) => {
            let (section, rest) = hops.split_first()?;
            let (field, items) = rest.split_last()?;
            if items.is_empty() {
                return None;
            }
            Some(FieldTarget::NestedItem {
                section: section.clone(),
                items: items.to_vec(),
                field: field.clone(),
            })
        }
    }
}

/// The resolved destination of a `set-slot` address: a **section-level** slot
/// (spliced via `set_slot_validated`) or an **item-level** per-item slot on a
/// repeatable item (spliced via `set_item_slot`, addressed through the item id).
enum SlotTarget {
    Section(String),
    Item {
        section: String,
        item: String,
        leaf: String,
    },
    /// A **nested** repeatable-item slot, addressed by its parent-scoped id chain
    /// (`#section/release/change-group/notes` and deeper — the M22 lift). `items` is the
    /// id chain from the section root; `leaf` is the trailing slot leaf.
    NestedItem {
        section: String,
        items: Vec<String>,
        leaf: String,
    },
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
        Fragment::UnitItemLeaf(section, item, leaf) => {
            return Some(SlotTarget::Item {
                section: section.as_str().to_string(),
                item: item.as_str().to_string(),
                leaf: leaf.as_str().to_string(),
            });
        }
        // A **nested** path (`#section/item/child/.../leaf`): split off the section
        // (leading) and the slot leaf (trailing); the hops between are the parent-scoped
        // item id chain.
        Fragment::Deep(hops) => {
            let (section, rest) = hops.split_first()?;
            let (leaf, items) = rest.split_last()?;
            if items.is_empty() {
                return None;
            }
            return Some(SlotTarget::NestedItem {
                section: section.clone(),
                items: items.to_vec(),
                leaf: leaf.clone(),
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

/// Wrap a blocking [`Finding`] as a [`DocFailure::Block`], ensuring it carries a
/// route **and a stable key target**. A hard block is a blocking-severity finding
/// carrying a route (`DECISIONS.md` 2026-05-31 → blocked/error payload). Where a
/// write-time adjudication finding carries none (the engine's `write.malformed-value` is
/// routeless), the CLI supplies the actionable retry route — presentation the CLI
/// owns, the determinism boundary unaffected. `dispatch` renders it through
/// `--format` (JSON envelope under `--format json`).
///
/// `subject` is the finding's **declared target form**
/// (`design/command-output-contract.md` → the form table): the **doc URI** for the
/// address-bearing writes — in the parsed normal form, never the raw CLI argument (a bare
/// singleton head expands: `vision#thesis` → `vision:vision#thesis`) — and the **bare
/// doctype id** for the doctype-scoped `create` / `author` blocks. It doubles as the
/// route's address echo.
fn block(finding: &Finding, verb: &str, subject: &str) -> DocFailure {
    let mut finding = finding.clone();
    if finding.route.is_none() {
        finding.route =
            Some(format!("retry `jigc doc {verb} {subject}` with a conforming value").into());
    }
    stamp_target(&mut finding, subject);
    DocFailure::block(finding)
}

/// Stamp `subject` as the finding's [`Location::address`] — the string its stable
/// `(code, target)` key derives from (`design/command-output-contract.md` → The stable
/// finding key) — **if it carries none**. The engine's fifteen write constructors cannot
/// self-address (they hold a section + a field, never a `type:slug`), so they emit
/// `Location::at(line, col)` with no address and every failed write of one code collided on
/// the degenerate key `(code, null)`; the CLI holds the parsed address, so the CLI stamps it
/// outward — the same post-pass shape the store walk's `attribute_to_doc` flip installs.
///
/// **If-absent, never a clobber:** a finding that already resolved its own target keeps it
/// — the doctype-scoped `create` keys (`create.gate-blocked` → `adr`) and
/// `create.serial-collision`'s instance address are its declared forms, not defaults. The
/// source coordinate is preserved (it is a human's pointer, never part of the key).
fn stamp_target(finding: &mut Finding, subject: &str) {
    match &mut finding.location {
        Some(location) if location.address.is_none() => {
            location.address = Some(subject.to_string());
        }
        Some(_) => {}
        None => finding.location = Some(Location::addressed(subject, 1, 1)),
    }
}

/// The doc head (`<type>:<slug>`) of a **normal-form** address — the URI prefix every
/// write-path finding's target carries, and the head the guards rebuild their route
/// addresses from. An address with no `#fragment` is already the head.
fn doc_head(addr: &str) -> &str {
    addr.split_once('#').map_or(addr, |(head, _)| head)
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

    /// Re-derive an [`engine::write::Instance`] from a parsed prd over `source`,
    /// owning every slot's prose (re-slicing the spans) — the bridge that lets the
    /// cold/empty spike assert `render(parse(bytes)) == bytes` on the new repeatable
    /// shape. prd items are flat single-level (a `title` heading + a single
    /// `statement` slot, no fields, no nesting), so the re-derive copies those leaves
    /// verbatim.
    fn prd_reparse_to_instance(schema: &Schema, source: &str) -> engine::write::Instance {
        let doc = engine::parse::parse_sections(schema, source).expect("rendered prd parses");
        let title = source
            .lines()
            .find_map(|l| l.strip_prefix("# "))
            .unwrap_or("")
            .to_string();
        let sections = doc
            .sections
            .iter()
            .map(|s| engine::write::SectionContent {
                id: s.id.clone(),
                slot: s.slot.as_ref().map(|sp| sp.slice(source).to_string()),
                fields: s.fields.clone(),
                items: s
                    .items
                    .iter()
                    .map(|it| engine::write::ItemContent {
                        id: it.id.clone(),
                        title: it.title.clone(),
                        slot: it.slot.as_ref().map(|sp| sp.slice(source).to_string()),
                        slots: it
                            .slots
                            .iter()
                            .map(|(k, sp)| (k.clone(), sp.slice(source).to_string()))
                            .collect(),
                        fields: it.fields.clone(),
                        items: Vec::new(),
                    })
                    .collect(),
            })
            .collect();
        engine::write::Instance { title, sections }
    }

    /// M25 Inc 5 (T1): the `prd` schema loads with `requirements` as a **repeatable
    /// section** (per-requirement `title` field + `statement` slot, mirroring
    /// `spec.criteria` minus the code-anchor), while `vision`/`context` stay fixed
    /// slots. A hand-authored instance with two requirement items **canonical-writes
    /// then re-parses** so that each requirement's `#requirements/<id>/statement`
    /// resolves byte-for-byte to its source prose, and the two fixed slots round-trip
    /// through `store::read_slice`. (`add-item` + the multi-word-heading parser fix
    /// cleared the M9 blockers; `design/auto-migration.md` → prd.)
    #[test]
    fn prd_schema_loads_and_repeatable_requirements_round_trip() {
        let schema = load_schema(PRD_YAML).expect("prd.yaml loads");
        assert_eq!(schema.ty, "prd");
        assert_eq!(schema.location.as_deref(), Some("prds/"));
        assert_eq!(schema.id_from.as_deref(), Some("title"));

        let vision = "A deterministic context compiler for coding agents.";
        let context = "Static rules files drift; this replaces them.";
        let req_one = "Assemble exactly the slices a task needs, just-in-time.";
        let req_two = "Own every structural write, leaving the LLM only the prose.";

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
                    items: vec![
                        engine::write::ItemContent {
                            id: "just-in-time-slices".to_string(),
                            title: "Just-in-time slices".to_string(),
                            slot: Some(req_one.to_string()),
                            ..Default::default()
                        },
                        engine::write::ItemContent {
                            id: "own-every-write".to_string(),
                            title: "Own every write".to_string(),
                            slot: Some(req_two.to_string()),
                            ..Default::default()
                        },
                    ],
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
        schemas.insert("prd".to_string(), schema.clone());

        // The two fixed slots round-trip through the single-hop store-read path.
        for (section, expected) in [("vision", vision), ("context", context)] {
            let address =
                Address::parse(&format!("prd:context-compiler#{section}")).expect("valid address");
            let got = engine::store::read_slice(root.0.as_path(), &schemas, &address)
                .expect("conformant prd slice re-parses and resolves");
            assert_eq!(
                got,
                expected.trim(),
                "the `{section}` fixed slot round-trips byte-for-byte",
            );
        }

        // Each requirement's `#requirements/<id>/statement` re-reads byte-for-byte:
        // re-parse the committed bytes and resolve each item leaf by its frozen id.
        let committed = std::fs::read_to_string(&path).expect("re-read committed prd");
        let doc = engine::parse::parse_sections(&schema, &committed)
            .expect("committed repeatable prd re-parses");
        let reqs = doc
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements section present");
        assert_eq!(
            reqs.items.len(),
            2,
            "exactly the two minted requirements re-parse"
        );
        for (id, expected) in [
            ("just-in-time-slices", req_one),
            ("own-every-write", req_two),
        ] {
            let item = reqs
                .items
                .iter()
                .find(|i| i.id == id)
                .unwrap_or_else(|| panic!("requirement {id} present"));
            let statement = item
                .slot
                .as_ref()
                .map(|sp| sp.slice(&committed))
                .unwrap_or("");
            assert_eq!(
                statement.trim(),
                expected.trim(),
                "`#requirements/{id}/statement` re-reads byte-for-byte",
            );
        }
    }

    /// M25 Inc 5 (T1) cold/empty spike: a prd minted with **zero** `requirements`
    /// items renders + reparses byte-stable, and so does the same prd after **one**
    /// requirement is added. The repeatable item carries no on-create / default field
    /// at mint (title is a plain `string`, statement a slot), so the new shape does
    /// not trip the empty-slot+field-group byte-instability — proven here, not
    /// trusted.
    #[test]
    fn prd_repeatable_requirements_cold_then_one_item_round_trip() {
        let schema = load_schema(PRD_YAML).expect("prd.yaml loads");

        // Cold: the `requirements` section has zero items.
        let cold = engine::write::Instance {
            title: "Empty prd".to_string(),
            sections: vec![
                engine::write::SectionContent {
                    id: "vision".to_string(),
                    slot: Some("A one-line vision.".to_string()),
                    ..Default::default()
                },
                engine::write::SectionContent {
                    id: "requirements".to_string(),
                    ..Default::default()
                },
                engine::write::SectionContent {
                    id: "context".to_string(),
                    slot: Some("The shaping constraints.".to_string()),
                    ..Default::default()
                },
            ],
        };
        let cold_bytes = engine::write::render(&schema, &cold);
        let cold_again =
            engine::write::render(&schema, &prd_reparse_to_instance(&schema, &cold_bytes));
        assert_eq!(
            cold_bytes, cold_again,
            "a zero-item prd renders + reparses byte-stable",
        );

        // One added requirement: same byte-stability with an item present.
        let mut warm = cold;
        warm.sections[1].items.push(engine::write::ItemContent {
            id: "single-tap-log".to_string(),
            title: "Single-tap log".to_string(),
            slot: Some("Log a habit in one tap.".to_string()),
            ..Default::default()
        });
        let warm_bytes = engine::write::render(&schema, &warm);
        let warm_again =
            engine::write::render(&schema, &prd_reparse_to_instance(&schema, &warm_bytes));
        assert_eq!(
            warm_bytes, warm_again,
            "a one-item prd renders + reparses byte-stable (no empty-slot+field-group drift)",
        );
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

    /// `civil_from_days` is the `today_iso` date deriver's core — pin it against
    /// known epoch-day anchors (the epoch itself, leap-day boundaries, and a
    /// post-2000 century-rule case) so the stamped `set: on-create` date is correct
    /// independent of the wall clock.
    #[test]
    fn civil_from_days_matches_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1), "the Unix epoch");
        assert_eq!(
            civil_from_days(-1),
            (1969, 12, 31),
            "the day before the epoch"
        );
        // 2000-02-29 — a leap day across the divide-by-400 century rule.
        assert_eq!(civil_from_days(11_016), (2000, 2, 29), "the 2000 leap day");
        assert_eq!(civil_from_days(11_017), (2000, 3, 1), "the day after");
        // 2026-06-09 (the day this regression was fixed) — 20_613 days post-epoch.
        assert_eq!(civil_from_days(20_613), (2026, 6, 9), "a contemporary date");
    }

    /// `today_iso` emits a well-formed ISO `YYYY-MM-DD` the engine's date-conformance
    /// check accepts (four-digit year, `01..=12` month, `01..=31` day).
    #[test]
    fn today_iso_is_a_well_formed_iso_date() {
        let s = today_iso();
        let parts: Vec<&str> = s.split('-').collect();
        assert_eq!(parts.len(), 3, "ISO date has three `-`-joined parts: {s:?}");
        assert_eq!(parts[0].len(), 4, "four-digit year: {s:?}");
        let month: u32 = parts[1].parse().expect("numeric month");
        let day: u32 = parts[2].parse().expect("numeric day");
        assert!((1..=12).contains(&month), "month in range: {s:?}");
        assert!((1..=31).contains(&day), "day in range: {s:?}");
    }

    /// `on_create_item_fields` materializes exactly the repeatable block's `date`
    /// leaves declared `set: on-create` — and nothing for a block without one (the
    /// existing single-slot/no-date `add-item` behavior is untouched).
    #[test]
    fn on_create_item_fields_stamps_only_on_create_date_leaves() {
        // The shipped `spec` doctype's `criteria` block carries NO `set: on-create`
        // field, so `add-item` over it passes no fields (the regression-safe path).
        const SPEC_YAML: &[u8] = include_bytes!("../pack/schemas/spec.yaml");
        let types = vec![engine::schema::PackTypeDecl {
            name: "code-anchor".to_owned(),
            adjudicator: "doc-code".to_owned(),
            check: "symbol-exists".to_owned(),
        }];
        let spec = engine::schema::load_schema_with_types(SPEC_YAML, &types).expect("spec loads");
        assert!(
            on_create_item_fields(&spec, "criteria", false).is_empty(),
            "a block with no `set: on-create` field stamps nothing",
        );

        // A fixture block WITH an on-create date stamps exactly that one field.
        let yaml = br#"
type: ledger
location: ledger/
id-from: title
description: A fixture running ledger.
usage: pin the on-create date materialization.
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: date, type: date, set: on-create }
        - { id: body, slot: { hint: "what" } }
"#;
        let schema = load_schema(yaml).expect("fixture ledger loads");
        let fields = on_create_item_fields(&schema, "entries", false);
        assert_eq!(fields.len(), 1, "exactly the one on-create date field");
        assert_eq!(fields[0].key, "date", "the stamped field is `date`");
        match &fields[0].value {
            Value::Scalar(v) => assert_eq!(v, &today_iso(), "stamped with today's date"),
            other => panic!("the date is a scalar, got {other:?}"),
        }
        // An unknown / non-repeatable section yields nothing.
        assert!(
            on_create_item_fields(&schema, "no-such-section", false).is_empty(),
            "an unknown section stamps nothing",
        );
    }

    /// Migration mode (`design/auto-migration.md` → Hardening #6) suppresses the
    /// `set: on-create` **date** stamp — and ONLY the date stamp — so a release migrated
    /// from a *dateless* foreign file renders with no date rather than fabricating the
    /// migration day as false history. The suppression is scoped to the date by
    /// predicate, not a blanket drop of every create-time field: a non-date `default:`
    /// leaf in the same item block still materializes under migration (the latent trap
    /// M25 inherits when it generalizes this path to adr/spec/prd — proved here on the
    /// reference). The shipped changelog release block carries only a `date` on-create
    /// field, so this fixture augments it with a non-date `default:` leaf to make the
    /// scoping observable; the byte-identical authoring path (`migration = false`) keeps
    /// stamping today.
    #[test]
    fn migration_suppresses_only_the_on_create_date_not_other_create_fields() {
        // A fixture item block carrying BOTH a `set: on-create` date AND a non-date
        // field with a literal `default:` — the two create-time leaf kinds.
        let yaml = br#"
type: ledger
location: ledger/
id-from: title
description: A fixture running ledger.
usage: pin date-scoped migration suppression.
sections:
  - id: entries
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: date, type: date, set: on-create }
        - { id: kind, type: enum, of: [note, fix], default: note }
        - { id: body, slot: { hint: "what" } }
"#;
        let schema = load_schema(yaml).expect("fixture ledger loads");

        // Authoring mode stamps BOTH the on-create date and the default.
        let authored = on_create_item_fields(&schema, "entries", false);
        assert_eq!(
            authored.len(),
            2,
            "authoring stamps the on-create date AND the default"
        );
        assert!(
            authored
                .iter()
                .any(|f| f.key == "date" && f.value == Value::Scalar(today_iso())),
            "authoring stamps today's date",
        );
        assert!(
            authored
                .iter()
                .any(|f| f.key == "kind" && f.value == Value::Scalar("note".into())),
            "authoring stamps the default",
        );

        // Migration mode drops the date stamp but KEEPS the non-date default.
        let migrated = on_create_item_fields(&schema, "entries", true);
        assert_eq!(
            migrated.len(),
            1,
            "migration drops the date stamp but keeps the non-date default"
        );
        assert_eq!(
            migrated[0].key, "kind",
            "the surviving field is the non-date default, not the date"
        );
        assert_eq!(
            migrated[0].value,
            Value::Scalar("note".into()),
            "the default materializes unchanged under migration"
        );
        assert!(
            !migrated.iter().any(|f| f.key == "date"),
            "no false-history date is fabricated under migration",
        );
    }

    /// `on_create_nested_item_fields` materializes a **nested** repeatable block's
    /// `date` leaf declared `set: on-create` — symmetric with the top-level
    /// `on_create_item_fields`. Before the fix the nested `add-item` branch passed no
    /// fields, so a nested on-create date was silently dropped (inert for the shipped
    /// changelog `changes` groups, which carry only `category` + `notes`).
    #[test]
    fn on_create_nested_item_fields_stamps_nested_on_create_date() {
        // A fixture: a top-level `releases` repeatable nesting a `changes` repeatable
        // whose block carries a `set: on-create` date (the latent target).
        let yaml = br#"
type: log
location: logs/
id-from: title
description: A fixture two-level log.
usage: pin nested on-create date materialization.
sections:
  - id: releases
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: at, type: date, set: on-create }
              - { id: notes, slot: { hint: "what" } }
"#;
        let schema = load_schema(yaml).expect("fixture log loads");

        let fields = on_create_nested_item_fields(
            &schema,
            "releases",
            &["1-0-0".to_string()],
            "changes",
            false,
        );
        assert_eq!(
            fields.len(),
            1,
            "exactly the one nested on-create date field"
        );
        assert_eq!(fields[0].key, "at", "the stamped nested field is `at`");
        match &fields[0].value {
            Value::Scalar(v) => assert_eq!(v, &today_iso(), "stamped with today's date"),
            other => panic!("the date is a scalar, got {other:?}"),
        }

        // The shipped changelog `changes` groups carry NO on-create date — symmetric
        // regression-safety with the top-level path (no fields stamped).
        const CHANGELOG_YAML: &[u8] = include_bytes!("../pack/schemas/changelog.yaml");
        let changelog = load_schema(CHANGELOG_YAML).expect("changelog loads");
        assert!(
            on_create_nested_item_fields(
                &changelog,
                "releases",
                &["1-0-0".to_string()],
                "changes",
                false,
            )
            .is_empty(),
            "the shipped `changes` group declares no on-create date — stamps nothing",
        );
    }

    /// `on_create_doc_fields` materializes a doc-level header field's `default:` and
    /// `set: on-create` — and is **inert** for a header that declares neither (the
    /// omitting-context guard, M22 engine work #4).
    #[test]
    fn on_create_doc_fields_materializes_default_and_on_create() {
        const ADR_YAML: &[u8] = include_bytes!("../pack/schemas/adr.yaml");
        let types = vec![engine::schema::PackTypeDecl {
            name: "code-anchor".to_owned(),
            adjudicator: "doc-code".to_owned(),
            check: "symbol-exists".to_owned(),
        }];
        let adr = engine::schema::load_schema_with_types(ADR_YAML, &types).expect("adr loads");
        let fields = on_create_doc_fields(&adr, false, 1);
        // Exactly `status` (default: proposed) then `date` (set: on-create), in schema
        // field order — `supersedes`/`cites-code` carry neither, so they are omitted.
        assert_eq!(
            fields.len(),
            2,
            "exactly status (default) + date (on-create)"
        );
        assert_eq!(fields[0].key, "status");
        assert_eq!(fields[0].value, Value::Scalar("proposed".into()));
        assert_eq!(fields[1].key, "date");
        assert_eq!(fields[1].value, Value::Scalar(today_iso()));

        // The shipped `commit` header (fields `type`/`scope`/`implements`, none
        // carrying default or set) is the inert witness: the materializer stamps
        // nothing, so its rendered front-matter is byte-unchanged.
        let commit = load_schema(COMMIT_YAML).expect("commit loads");
        assert!(
            on_create_doc_fields(&commit, false, 1).is_empty(),
            "a header with no default/set field stamps nothing (inert)",
        );
    }

    /// Migration mode (`design/auto-migration.md` → Doc-level date-suppression)
    /// suppresses the doc-level `set: on-create` **date** header stamp — and ONLY that
    /// stamp — so a *dateless* foreign ADR migrates with no date rather than fabricating
    /// the migration day as false decision history (the doc-level twin of the proven
    /// item-level fix). The default-bearing `status: proposed` still materializes under
    /// migration (scoped suppression, not a blanket drop). The byte-identical authoring
    /// path (`migration = false`) keeps stamping today, so a non-migration `doc create
    /// adr` is unaffected. `spec`/`prd` carry no date field, so the flag is invariant.
    #[test]
    fn migration_suppresses_only_the_doc_level_on_create_date() {
        let types = vec![engine::schema::PackTypeDecl {
            name: "code-anchor".to_owned(),
            adjudicator: "doc-code".to_owned(),
            check: "symbol-exists".to_owned(),
        }];

        const ADR_YAML: &[u8] = include_bytes!("../pack/schemas/adr.yaml");
        let adr = engine::schema::load_schema_with_types(ADR_YAML, &types).expect("adr loads");

        // Migration mode drops the date stamp but KEEPS the `status: proposed` default.
        let migrated = on_create_doc_fields(&adr, true, 1);
        assert_eq!(
            migrated.len(),
            1,
            "migration drops the date stamp but keeps the default"
        );
        assert_eq!(
            migrated[0].key, "status",
            "the surviving field is the default"
        );
        assert_eq!(migrated[0].value, Value::Scalar("proposed".into()));
        assert!(
            !migrated.iter().any(|f| f.key == "date"),
            "no false-history date is fabricated under migration",
        );

        // Authoring mode (the regression witness) still stamps BOTH.
        let authored = on_create_doc_fields(&adr, false, 1);
        assert_eq!(authored.len(), 2, "authoring stamps status + the date");
        assert!(
            authored
                .iter()
                .any(|f| f.key == "date" && f.value == Value::Scalar(today_iso())),
            "authoring stamps today's date",
        );

        // `spec` carries no date field, so the flag is invariant (moot for spec/prd).
        const SPEC_YAML: &[u8] = include_bytes!("../pack/schemas/spec.yaml");
        let spec = engine::schema::load_schema_with_types(SPEC_YAML, &types).expect("spec loads");
        assert_eq!(
            on_create_doc_fields(&spec, true, 1),
            on_create_doc_fields(&spec, false, 1),
            "a doctype with no date field is flag-invariant",
        );

        // `commit` (no default/set header field) stays inert under both flags.
        let commit = load_schema(COMMIT_YAML).expect("commit loads");
        assert!(on_create_doc_fields(&commit, true, 1).is_empty());
        assert!(on_create_doc_fields(&commit, false, 1).is_empty());
    }

    /// The schema-version stamp (M34 audit) is derived from each doctype's **manifest**
    /// version, not a constant: a newly-created doc of a doctype the freeze manifest
    /// reports at version N is stamped `schema-version: N`. Drives the real create-path
    /// seam — [`stamp_schema_version`] resolving from a fixture pack's manifest, then
    /// [`on_create_doc_fields`] materializing that value into the injected stamp field.
    /// A fixture manifest reporting `widget` at v2 stamps `2`; a doctype absent from the
    /// manifest, and the shipped all-v1 set, both stamp `1` (shipped behaviour unchanged
    /// — the regression witness). Fails against the prior hardcoded `1` deriver.
    #[test]
    fn newly_created_doc_is_stamped_its_manifest_schema_version() {
        use engine::schema::SCHEMA_VERSION_FIELD;

        // A throwaway fixture pack: a freeze manifest reporting `widget` at v2 (the hash
        // is unread by the version map / stamp-injection — only `assert_schema_freeze`
        // recomputes it, which this path does not call) + a persisted `widget` doctype
        // so `load_pack_schema` injects the stamp into its header.
        let root = TempRoot::new("schema-version-stamp");
        std::fs::create_dir_all(root.0.join("config")).expect("config dir");
        std::fs::create_dir_all(root.0.join("schemas")).expect("schemas dir");
        std::fs::write(
            root.0.join("config/schema-manifest.yaml"),
            b"doctypes:\n  - type: widget\n    schema-version: 2\n    schema-hash: 0000000000000000000000000000000000000000000000000000000000000000\n",
        )
        .expect("write manifest");
        std::fs::write(
            root.0.join("schemas/widget.yaml"),
            b"type: widget\nlocation: widgets/\nid-from: title\nsections:\n  - id: meta\n    header: true\n    fields: []\n  - id: body\n    slot: { hint: \"x\" }\n",
        )
        .expect("write widget schema");
        let pack = crate::pack::FilesystemPack::new(root.0.clone());

        // The deriver resolves `widget`'s declared version (2) from the manifest, and
        // falls back to 1 for a doctype the manifest does not list.
        assert_eq!(
            stamp_schema_version(&pack, "widget"),
            2,
            "the stamp value is the doctype's manifest schema-version, not a constant",
        );
        assert_eq!(
            stamp_schema_version(&pack, "absent"),
            1,
            "a doctype absent from the manifest falls back to 1",
        );

        // End-to-end: the injected stamp field is materialized with the resolved value.
        let widget_bytes = pack
            .read(PackResourceKind::Schemas, &ResourceId::from("widget"))
            .expect("widget schema reads");
        let widget = crate::pack::load_pack_schema(&pack, &widget_bytes).expect("widget loads");
        let version = stamp_schema_version(&pack, "widget");
        let fields = on_create_doc_fields(&widget, false, version);
        let stamp = fields
            .iter()
            .find(|f| f.key == SCHEMA_VERSION_FIELD)
            .expect("the persisted doctype carries an injected schema-version stamp");
        assert_eq!(
            stamp.value,
            Value::Scalar("2".into()),
            "a v2-manifest doctype is stamped schema-version 2",
        );

        // Regression witness: a still-frozen-v1 shipped doctype (`spec`) stamps 1, while the
        // M36-bumped `adr` now stamps its manifest v2 — every shipped doctype resolves to its
        // own manifest version (`design/corpus-migration.md` → the adr v1→v2 flow).
        let shipped = make_pack().expect("the shipped pack passes its own freeze gate");
        assert_eq!(
            stamp_schema_version(shipped.as_ref(), "spec"),
            1,
            "the shipped frozen-v1 `spec` still stamps 1",
        );
        assert_eq!(
            stamp_schema_version(shipped.as_ref(), "adr"),
            2,
            "the M36-bumped shipped `adr` stamps its manifest schema-version 2",
        );
        let adr_bytes = shipped
            .read(PackResourceKind::Schemas, &ResourceId::from("adr"))
            .expect("shipped adr reads");
        let adr = crate::pack::load_pack_schema(shipped.as_ref(), &adr_bytes).expect("adr loads");
        let adr_stamp =
            on_create_doc_fields(&adr, false, stamp_schema_version(shipped.as_ref(), "adr"))
                .into_iter()
                .find(|f| f.key == SCHEMA_VERSION_FIELD)
                .expect("shipped adr carries the injected stamp");
        assert_eq!(
            adr_stamp.value,
            Value::Scalar("2".into()),
            "the M36-bumped shipped `adr` materializes schema-version 2",
        );
    }
}
