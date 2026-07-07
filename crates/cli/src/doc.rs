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
use engine::finding::{Finding, Location, Severity};
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::schema::{Schema, SectionBody};
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
    /// Mint a new managed instance (agent-initiated, create-gated). The id-source
    /// is supplied inline; the CLI mints + places per the schema.
    Create {
        /// The doctype to create (e.g. `adr`).
        r#type: String,
        /// The id-source the slug is minted from (inline, the `--<id-source>`
        /// form; `--title "…"` is the MVP surface for the title-slugged types).
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
    /// Remove a repeatable item — a top-level item (`<type>:<slug>#<section>/<id>`) or a
    /// nested one (`#<section>/<parent>/.../<nested-section>/<id>`) — without discarding
    /// the task (a general recovery verb over the engine's `remove_item` /
    /// `remove_nested_item`).
    RemoveItem {
        /// The item address — top-level `<type>:<slug>#<section>/<id>` or the nested
        /// section-qualified chain `#<section>/<parent>/.../<nested-section>/<id>`.
        addr: String,
        /// The active task to scope the write to (see `Create::task`).
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a field leaf's value (inline, adjudicated at write time). For a
    /// list-cardinality (`0..*`) ref, set ALL values in one call with the inline-list
    /// form `--value "[a, b, c]"` — repeated single-value calls replace the whole list
    /// (and are rejected once it is populated, to prevent silently dropping prior
    /// entries).
    SetField {
        /// The leaf address — `<type>:<slug>#<field>` (or `#<section>/<field>`).
        addr: String,
        /// The new value (inline — fields are short + escaping-safe). A
        /// list-cardinality (`0..*`) ref takes the inline-list form `"[a, b, c]"` to
        /// set multiple values in one call.
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
    /// Author a whole instance from one declarative payload — the doctype-general
    /// batch verb. Applies the equivalent `create` plus N `add-item` / `set-slot` /
    /// `set-field` over a single buffer, persisting once. In the payload, a slot
    /// value must be wrapped in literal `<<…>>` markers (`summary: "<<the prose>>"`)
    /// — required syntax that tags the value as slot prose, not a fill-me
    /// placeholder to delete; an inline field takes a bare value (wrapping one is
    /// rejected).
    ///
    /// Reach for `author` to write a whole instance in one shot (a migration, or any
    /// many-leaf doc) — it collapses what would be a `create` + N follow-up calls.
    /// Use `create` then `set-slot`/`set-field`/`add-item` for incremental,
    /// one-leaf-at-a-time authoring instead.
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
    /// Read a **committed** managed doc (or an addressed `#section`/item/leaf slice)
    /// through the canonical parse/render path. Plain text is the byte-exact committed
    /// view; `--format json` is the pinned stable shape (`design/team-ready-state.md` →
    /// The read surface): a whole-doc object `{ type, slug, fields, sections }` where a
    /// slot section serializes to its prose string and a repeatable section to its item
    /// array; a `#section` slice returns that section's value (item array / slot prose),
    /// an `#section/<id>` slice the item object, an `#section/<id>/<leaf>` slice the leaf.
    /// Reads the committed store — it takes **no** `--task` (unlike the write verbs).
    Show {
        /// The doc address — `<type>:<slug>`, or a `#section`/item/leaf slice of it.
        addr: String,
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
    /// an orchestration error to stderr through the shared operational-error funnel.
    pub fn dispatch(self, cwd: &Path, format: Format) -> Outcome {
        let result = match self {
            DocCommand::Create {
                r#type,
                title,
                slug,
                task,
            } => run_create(cwd, &r#type, &title, slug.as_deref(), task.as_deref()),
            DocCommand::AddItem { addr, title, task } => {
                run_add_item(cwd, &addr, &title, task.as_deref())
            }
            DocCommand::RemoveItem { addr, task } => {
                run_remove_item(cwd, &addr, task.as_deref(), format)
            }
            DocCommand::SetField { addr, value, task } => {
                run_set_field(cwd, &addr, &value, task.as_deref(), format)
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
            } => run_author(cwd, &doctype, &from_file, task.as_deref()),
            DocCommand::Show { addr } => run_show(cwd, &addr, format),
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
                let report = engine::result::ValidationReport::new(vec![finding], &resolved);
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

/// `jigc doc set-field <addr> --value <v>` — adjudicate + splice a field value.
fn run_set_field(
    cwd: &Path,
    addr: &str,
    value: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)
        .with_context(|| format!("no field addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = apply_field_target(&schema, &source, target, addr, value)?;

    persist(&path, &edited)?;
    // Confirm the landed value — the positive ack the silent verb was missing.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Field {
                address: addr.to_string(),
                value: value.to_string(),
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
    addr: &str,
    value: &str,
) -> Result<String, DocFailure> {
    Ok(match target {
        FieldTarget::Section { section, field } => set_field_validated(
            schema,
            source,
            &section,
            &field,
            &Value::Scalar(value.to_string()),
        )
        .map_err(|f| block(&f, "set-field", addr))?,
        FieldTarget::Item {
            section,
            item,
            field,
        } => set_item_field_or_insert(schema, source, &section, &item, &field, value).map_err(
            |e| {
                block(
                    &engine::write::generate_error_finding(&e),
                    "set-field",
                    addr,
                )
            },
        )?,
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            set_nested_item_field_or_insert(schema, source, &section, &item_ids, &field, value)
                .map_err(|e| {
                    block(
                        &engine::write::generate_error_finding(&e),
                        "set-field",
                        addr,
                    )
                })?
        }
    })
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
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        slot_target(&schema, &address).with_context(|| format!("no slot addressed by `{addr}`"))?;

    let prose = read_handoff(from_file)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = apply_slot_target(&schema, &source, target, addr, &prose)?;

    persist(&path, &edited)?;
    // Confirm the spliced prose by length — the slot bytes are too large to echo.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Slot {
                address: addr.to_string(),
                chars: prose.chars().count(),
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
    addr: &str,
    prose: &str,
) -> Result<String, DocFailure> {
    Ok(match target {
        SlotTarget::Section(section) => set_slot_validated(schema, source, &section, prose)
            .map_err(|f| block(&f, "set-slot", addr))?,
        SlotTarget::Item {
            section,
            item,
            leaf,
        } => set_item_slot(schema, source, &section, &item, &leaf, prose)
            .map_err(|e| block(&engine::write::splice_error_finding(&e), "set-slot", addr))?,
        SlotTarget::NestedItem {
            section,
            items,
            leaf,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            set_nested_item_slot(schema, source, &section, &item_ids, &leaf, prose)
                .map_err(|e| block(&engine::write::splice_error_finding(&e), "set-slot", addr))?
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
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        add_item_target(&address).with_context(|| format!("no section addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let (edited, minted_path) =
        apply_add_item_target(&schema, &source, target, addr, title, task.is_migration()?)?;

    persist(&path, &edited)?;
    // The minted item address — the next address an agent fills the item's slot/field
    // at (the same slugify the engine mints the `{#id}` from, never re-spelled).
    println!(
        "{}:{}#{}",
        address.r#type.as_str(),
        address.slug.as_str(),
        minted_path,
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
    addr: &str,
    title: &str,
    migration: bool,
) -> Result<(String, String), DocFailure> {
    // Write-time id-from-enum reject (`design/auto-migration.md` → Hardening #3;
    // write-commands.md → Two check times): when the destination repeatable's `id-from`
    // is an enum the `--title` re-slugs outside, block here — fast feedback at the point
    // of the mistake, not deferred to finalize. The batch (`apply_leaf`) inherits this
    // by sharing this path.
    if let Some(finding) = id_from_enum_block(schema, &target, title) {
        return Err(DocFailure::Block(finding));
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
                .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", addr))?;
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
            .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", addr))?;
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
/// slug-cased id-from address. A non-enum id-from / a member title yields `None` — the
/// inert path, mirroring finalize's exemption (so `Fixed`→`fixed` passes).
fn id_from_enum_block(schema: &Schema, target: &AddItemTarget, title: &str) -> Option<Finding> {
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
    Some(Finding::blocking(
        engine::validate::ID_FROM_ENUM_CODE,
        format!(
            "add-item rejected: `{slug}` is not an enum member of id-from field `{}`",
            repeatable.id_from
        ),
        Location::addressed(format!("{prefix}/{slug}/{}", repeatable.id_from), 1, 1),
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
    let address = parse_addr(addr)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        remove_item_target(&address).with_context(|| format!("no item addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let source = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let edited = match target {
        RemoveItemTarget::TopLevel { section, item } => {
            engine::write::remove_item(&schema, &source, &section, &item).map_err(|e| {
                block(
                    &engine::write::splice_error_finding(&e),
                    "remove-item",
                    addr,
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
                        addr,
                    )
                },
            )?
        }
    };

    persist(&path, &edited)?;
    // Confirm the removed item address — the positive ack the silent verb was missing.
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::RemovedItem {
                address: addr.to_string(),
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

/// Resolve the item a `remove-item` address targets. The two-hop `#section/id`
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
) -> Result<(), DocFailure> {
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
    println!("{}", created.address);
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
) -> Result<(), DocFailure> {
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
    // The create persists the empty doc through the gated path (gate + squatter seams).
    let created = state::create_gated(
        &task.dir,
        &schemas,
        &gate.allows_create,
        doctype,
        &plan.title,
        &task.jigc_home,
        &on_create,
        // `doc author` derives the slug from the payload title, never a `--slug` flag.
        None,
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
    println!("{}", created.address);
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

/// `jigc doc show <ref>` — read a **committed** managed doc (or an addressed slice)
/// from the store (`design/team-ready-state.md` → The read surface; `design/
/// introspection.md` → "Reading *filled* prose stays `jigc doc show`"). Unlike the
/// write verbs this is task-less: it resolves the cascade schema set + repo root the
/// same way `jigc validate` does, then serves the read through the canonical parse
/// path — plain text is the byte-exact [`engine::store::read_slice`] view; `--format
/// json` is the pinned stable shape (a 1.0 contract, [`show_json`]). A read-side block
/// (unknown type / not-found / unparseable / a `#fragment` naming nothing) routes
/// through the shared [`DocFailure`] envelope, non-zero exit + route, exactly like a
/// write block.
fn run_show(cwd: &Path, addr: &str, format: Format) -> Result<(), DocFailure> {
    let address = parse_addr(addr)?;
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let schemas = committed_schemas(&jigc_home)?;
    match format {
        Format::Json => {
            let value = show_json(&jigc_home, &schemas, &address)?;
            println!("{}", render::json(&value));
        }
        Format::Agent | Format::Human => {
            let slice = engine::store::read_slice(&jigc_home, &schemas, &address)
                .map_err(DocFailure::Block)?;
            println!("{slice}");
        }
    }
    Ok(())
}

/// The cascade-resolved schema set keyed by doctype the committed-store read resolves
/// against — the `project > team > pack-default` shadow set with each `location:`
/// nested under the resolved `docs-root` (the `jigc validate` store-read idiom; a
/// placement doctype homes at its literal file, docs-root inert). Reads the composed
/// pack from cwd, so a `[dev ▸ methodology]` repo's `vision`/`milestone-record` resolve
/// alongside the dev doctypes.
fn committed_schemas(jigc_home: &Path) -> Result<BTreeMap<String, Schema>> {
    let pack = make_pack();
    let project_config = jigc_home.join(".jigc").join("config");
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config)?;
    let defs = crate::start::CascadeDefs::new(&resolved, &project_config);
    defs.all_schemas(pack.as_ref())
}

/// Build the pinned `--format json` value for `address` (`design/team-ready-state.md` →
/// The read surface — the 1.0 stable contract, a one-way door). Reads the whole
/// committed doc through [`engine::store::read_slice`] (which surfaces every doc-level
/// block — unknown type / transient / not-found / unparseable — identically to the
/// plain path), re-parses it (guaranteed clean: `read_slice` just parsed it), and shapes
/// the value from the parsed structure. For a `#fragment`, a second `read_slice`
/// validates the fragment resolves so the json path blocks on a bad `#section`/item/leaf
/// exactly as plain does; the value itself is navigated over the parsed structure.
fn show_json(
    jigc_home: &Path,
    schemas: &BTreeMap<String, Schema>,
    address: &Address,
) -> Result<serde_json::Value, DocFailure> {
    let whole = Address {
        fragment: None,
        ..address.clone()
    };
    let source =
        engine::store::read_slice(jigc_home, schemas, &whole).map_err(DocFailure::Block)?;
    let schema = schemas
        .get(address.r#type.as_str())
        .expect("read_slice resolved the type, so it is in the schema set");
    let doc = engine::parse::parse_sections(schema, &source)
        .expect("read_slice already parsed the committed doc clean");
    match &address.fragment {
        None => Ok(whole_doc_json(schema, &doc, &source, address)),
        Some(fragment) => {
            engine::store::read_slice(jigc_home, schemas, address).map_err(DocFailure::Block)?;
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
                        fields.insert(field.key.clone(), field_json(&field.value));
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
/// array; `#section/<id>` → the item object; `#section/<id>/<leaf>` → the leaf value
/// (slot prose or field). `read_slice` already validated the fragment resolves, so each
/// lookup is infallible. Nested repeatable-in-item content (the M22 changelog shape) is
/// not part of the pinned read-surface contract — every witness + the milestone-record
/// are flat — so a deeper path degrades to the enclosing item object rather than
/// modelling an unpinned nested json shape.
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
        // A simple section only reaches here at the section level (`read_slice` blocks a
        // sub-hop into a simple section), so it is the slot-prose case.
        SectionBody::Simple { .. } => slot_json(parsed.and_then(|p| p.slot.as_ref()), source),
        SectionBody::Repeatable { repeatable } => {
            let items = parsed.map(|p| p.items.as_slice()).unwrap_or(&[]);
            match rest.split_first() {
                None => items_json(items, repeatable, source),
                Some((item_id, leaf_rest)) => {
                    let item = items
                        .iter()
                        .find(|it| &it.id == item_id)
                        .expect("read_slice validated the item");
                    match leaf_rest.first() {
                        None => item_json(item, repeatable, source),
                        Some(leaf) if leaf_rest.len() == 1 => {
                            if let Some(span) = item.slot_span(leaf) {
                                serde_json::Value::String(span.slice(source).trim().to_string())
                            } else if let Some(field) = item.fields.iter().find(|f| &f.key == leaf)
                            {
                                field_json(&field.value)
                            } else {
                                // A nested item id at leaf position (unpinned nested case).
                                item_json(item, repeatable, source)
                            }
                        }
                        // A deeper nested path — unpinned; degrade to the enclosing item.
                        Some(_) => item_json(item, repeatable, source),
                    }
                }
            }
        }
    }
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

/// One repeatable item as a json object of its leaves: the `id-from` leaf keyed by its
/// id → the item's heading value (its stable id-source), each other field keyed by leaf
/// id (scalar → string, list → array), each slot keyed by leaf id → its trimmed prose.
/// A single bare-prose slot carries no leaf id in the parsed item, so the block names it
/// ([`single_slot_leaf`]). Nested repeatable content is not serialized (the unpinned
/// nested case; see [`fragment_json`]).
fn item_json(
    item: &engine::parse::ParsedItem,
    repeatable: &engine::schema::Repeatable,
    source: &str,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
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
    /// `<repo>/.jigc/tasks/<id>/` directly, rejecting `no task <id>` if absent
    /// (mirroring `start::reenter_in_repo`). Else: the **single** active task
    /// directory under `<repo>/.jigc/tasks/`; **none** rejects with the start-a-task
    /// route, **more than one** with no `--task` rejects asking for the selector.
    fn resolve(cwd: &Path, task_id: Option<&str>) -> Result<Self> {
        let jigc_home = crate::start::jigc_home_or_repo(cwd)?;
        let jigc_root = jigc_home.join(".jigc");
        let tasks = jigc_root.join("tasks");

        if let Some(id) = task_id {
            let dir = tasks.join(id);
            if !dir.is_dir() {
                bail!("no task `{id}` — list live tasks with `jigc task list`");
            }
            return Ok(Self {
                id: id.to_string(),
                dir,
                jigc_home,
                pack: make_pack(),
            });
        }

        // The single enumeration source of truth (`state::list_active_task_ids`) the
        // ambiguous error and `jigc task list` share, so they never disagree.
        let mut ids = state::list_active_task_ids(&jigc_root);
        match ids.len() {
            0 => bail!("no active task — start one with `jigc start`"),
            1 => {
                let id = ids.pop().expect("one task id");
                let dir = tasks.join(&id);
                Ok(Self {
                    id,
                    dir,
                    jigc_home,
                    pack: make_pack(),
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
    // `map_err`, not `with_context`: the latter chains the raw I/O error's `os error 2`
    // tail into `{err:#}` — a dead end. The absent-instance case is the expected reason
    // this read fails, so surface the provision route alone (M36 Inc-4).
    std::fs::read_to_string(path).map_err(|_| {
        anyhow!(
            "no staged instance for `{addr}` — provision it first (`jigc start` / `jigc doc create`). \
             Note: `jigc doc create <type> --title \"X\"` derives the id from the title (`X` → slug), \
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
        let shipped = make_pack();
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
