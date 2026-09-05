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
    SlotAddress, set_field_validated, set_item_field_validated, set_slot_validated,
};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

/// The `jigc doc` **read** verbs — the members of the family with no write to make
/// (`design/doc-read-surface.md`: `doc show` is the contract-pinned content read,
/// `doc schema` the separately-versioned schema read, `doc list` the index read).
/// Declared here, once, as the code-side complement of [`doc_write_verbs`].
pub const DOC_READ_VERBS: &[&str] = &["show", "schema", "list"];

/// The `jigc doc` **write-verb** family, derived from the built clap tree: every
/// `doc` leaf minus [`DOC_READ_VERBS`]. Registry-derived, never a hand list — a new
/// `doc` verb added later joins the write family (or is declared a read) instead of
/// dodging every consumer.
///
/// Two consumers, one partition: the M48 read-back fence
/// (`crate::pack` → the staged-read-back tier — a catalog entry whose argv is
/// `jigc doc <write-verb>` is a write solicit, so the step carrying its
/// `{{cli.<id>}}` ref owes the read-back statement) and the rejected-write
/// exit-code sweep (`crates/cli/tests/exit_codes.rs`), which read it rather than
/// re-deriving it beside a second copy of the read set.
pub fn doc_write_verbs() -> Vec<String> {
    use clap::CommandFactory;
    let mut root = <crate::cli::Cli as CommandFactory>::command();
    root.build();
    let doc = root
        .get_subcommands()
        .find(|sub| sub.get_name() == "doc")
        .expect("the clap tree carries the `doc` subtree");
    doc.get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .filter(|name| name != "help" && !DOC_READ_VERBS.contains(&name.as_str()))
        .collect()
}

/// `doc set-slot`'s long help: the one-line lead plus the **address-independent**
/// form of the heading-depth ceiling rule, taken verbatim from the engine so the
/// help and the enforcing write path state one rule. Help is rendered before any
/// address exists, so no target's reserved depth is knowable here — it states the
/// rule and names no depth, and points at the `{{schema:}}` projection, the one
/// surface that *can* name the set (`design/surface-contract.md` → The stated-at
/// fence, the `set-slot --help` carve-out).
fn set_slot_long_about() -> String {
    format!(
        "Set a slot leaf's prose (multi-line, via stdin or a file).\n\n{}\n\nThe \
         `{{{{schema:<doctype>}}}}` projection at an authoring solicit renders the exact \
         reserved set for the slots it solicits.",
        engine::write::slot_ceiling_rule_statement(),
    )
}

/// The `add-item` long help. `add-item` mints an item's `{#id}` anchor from
/// `--title` the same way `create` mints a doc id, so this second soliciting
/// surface earns the same stated-at fence: the slug mint rule is stated at the
/// `--title` it binds (`design/surface-contract.md` → The stated-at fence,
/// seam-generated tier). The sentence is `slug::mint_statement` output — built
/// from the mint-rule constants, so it can never drift from what the mint
/// enforces (the doc-level `{{schema:}}` skeleton and `create --help` render the
/// same generator).
fn add_item_long_about() -> String {
    format!(
        "Mint a repeatable item into a section, id-slugged from `--title`.\n\n\
         The section is addressed `<type>:<slug>#<section>`; the CLI mints the \
         `{{#id}}` anchor + appends the item block. The `--title` is {} (`--slug` \
         overrides the mint, except where the block's `id-from` is an enum — there \
         the heading IS the member and the anchor equals it).",
        engine::slug::mint_statement("the item `{#id}` anchor"),
    )
}

/// The `create` long help. The doc-mint site — the third soliciting surface, and
/// the one whose statement was hand-typed: it named the two caps and nothing
/// else, so it described a rule the mint does not implement (B7). It renders the
/// same `slug::mint_statement` generator as `add-item` and the `{{schema:}}`
/// projection, so all three mint sites state one rule and none can drift from it.
fn create_long_about() -> String {
    format!(
        "Mint a new managed instance (agent-initiated, create-gated).\n\n\
         The title the slug is minted from is supplied inline with `--title` — always \
         literally `--title`, whatever the doctype's `id-from` field is named; the CLI \
         mints + places per the schema. The `--title` is {} (`--slug` overrides the \
         mint).\n\n\
         {}\n\n\
         {}",
        engine::slug::mint_statement("the doc id"),
        TITLE_CONTRACT,
        crate::cli::ARGUMENT_CONVENTION,
    )
}

/// **The title contract, stated before the write** — the law-3 ambush repair M47 made for
/// `write.already-present`, applied to its M48 sibling (`design/surface-contract.md` →
/// law 3, the ambush class; `design/write-commands.md` → The three-way write over a
/// committed doc). Both minting verbs render it, because both make the same two misses.
const TITLE_CONTRACT: &str = "\
A create over a doc this task ALREADY holds — the same id re-created, or a committed one \
copied in for update — is a create-or-update: it hands that body back and does NOT \
rewrite its `# H1`. So a title that would be silently dropped is refused \
(`write.title-ignored`), as is a singleton's, whose `# H1` is the schema's own \
`display-title`. And a title (or `--slug`) that would mint a DIFFERENT identity beside \
the one this task's create-gate role already binds is refused as well \
(`write.identity-change`) — that is a second document, not a correction. Both refusals \
route at `jigc doc rename`, the in-task title change; a genuinely separate second \
document is its own task.";

/// The `doc rename` long help. The fourth soliciting surface that mints a slug, so
/// it earns the same stated-at fence as `create` / `add-item` / the `{{schema:}}`
/// projection: the mint rule is stated at the flag it binds, rendered from the same
/// `slug::mint_statement` generator so none of the four can drift
/// (`design/surface-contract.md` → The stated-at fence, seam-generated tier).
fn rename_long_about() -> String {
    format!(
        "Retitle a staged doc — and re-slug it while its identity is uncommitted.\n\n\
         The doc is addressed `<type>:<slug>` (whole-doc only — retitle a repeatable \
         item with `jigc doc retitle-item`). When this task MINTED the doc, `--to` \
         rewrites its `# H1` and re-slugs it, moving the staged file and every \
         reference to it inside the task area. When the doc was copied in from the \
         COMMITTED store it is retitle-only: the `# H1` is corrected in place, and a \
         `--to` that would move the slug is refused and routed at `jigc rename`, the \
         task-less op that moves a committed identity and repoints its referrers. The \
         `--to` title is {} (`--slug` overrides the mint).\n\n\
         {}",
        engine::slug::mint_statement("the re-slugged doc id"),
        crate::cli::ARGUMENT_CONVENTION,
    )
}

/// The `jigc doc <verb>` subcommand tree. Each verb addresses a managed doc in
/// the active task's working area.
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum DocCommand {
    /// Mint a new managed instance (agent-initiated, create-gated).
    ///
    /// The title the slug is minted from is supplied inline with `--title` —
    /// always literally `--title`, whatever the doctype's `id-from` field is
    /// named; the CLI mints + places per the schema (`--slug` overrides the
    /// mint).
    #[command(long_about = create_long_about())]
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
    #[command(long_about = add_item_long_about())]
    AddItem {
        /// The section address — `<type>:<slug>#<section>` (the repeatable section the
        /// item is minted into).
        addr: String,
        /// The item id-source — slugged to the `{#id}` anchor (the MVP surface; the
        /// item's slot/fields are filled by later `set-slot`/`set-field` writes).
        #[arg(long)]
        title: String,
        /// Override the minted item `{#id}`, decoupling the item's id from its heading
        /// text. Taken **verbatim** and validated as a well-formed slug — a malformed
        /// value is rejected, never silently re-slugified (`design/write-commands.md`
        /// → `jigc rename`'s `--slug` precedent). Refused where the block's `id-from`
        /// is an ENUM: there the heading IS the member and the anchor equals it, so an
        /// override would be an identity change.
        #[arg(long)]
        slug: Option<String>,
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
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
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
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
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
        #[arg(long)]
        task: Option<String>,
    },
    /// Retitle a staged doc — and re-slug it while its identity is uncommitted.
    ///
    /// The in-task sibling of the top-level `jigc rename` (which is task-less and
    /// self-committing, and refuses outright while any task is in flight). It splits
    /// on **committed-store identity**: a doc this task minted has no committed
    /// referrers by construction, so `--to` rewrites its `# H1` **and** re-slugs it,
    /// moving the staged file and every reference to it inside the task area; a doc
    /// copied in from the committed store is **retitle-only** — its `# H1` is
    /// corrected in place and a divergent `--to` is refused, routed at `jigc rename`,
    /// which moves a committed identity transactionally and repoints its referrers.
    /// A doctype whose identity the CLI supplies — a `placement` / `display-title`
    /// singleton, or a transient sink like `commit` (whose slug IS the task id) —
    /// refuses: there is no author-owned title or slug to change.
    #[command(long_about = rename_long_about())]
    Rename {
        /// The doc address — `<type>:<slug>` (a singleton doctype may be named bare).
        /// Whole-doc only: retitle a repeatable item with `jigc doc retitle-item`.
        addr: String,
        /// The new title — the doc's `# H1`, and (while the identity is uncommitted)
        /// the source the new slug is minted from
        /// (`design/write-commands.md` → The argument convention).
        #[arg(long)]
        to: String,
        /// Override the re-slug's minted id, decoupling it from the title. Taken
        /// **verbatim** and validated as a well-formed slug. A committed identity
        /// cannot re-slug here at all: a value that would move it is refused and
        /// routed at `jigc rename`.
        #[arg(long)]
        slug: Option<String>,
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
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
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
        #[arg(long)]
        task: Option<String>,
    },
    /// Set a slot leaf's prose (multi-line, via stdin or a file).
    #[command(long_about = set_slot_long_about())]
    SetSlot {
        /// The slot address — `<type>:<slug>#<slot>`.
        addr: String,
        /// The prose source: a path, or `-` for stdin (prose never inline).
        #[arg(long)]
        from_file: String,
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
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
    /// one-leaf-at-a-time authoring instead. Run `author` instead of those verbs, or
    /// after them — the create `author` implies over a doc this task already staged
    /// acks `already existed — copied in for update` and changes nothing.
    ///
    /// Over a COMMITTED doc `author` copies the committed body in as the edit base
    /// and writes three ways: a payload item the doc does not hold is **appended**
    /// (the committed items stay untouched); an existing doc-level leaf you
    /// re-author overwrites **in place**; and a payload item whose title mints an id
    /// the doc ALREADY holds is refused (`write.already-present`) with the whole
    /// payload rejected and nothing staged — edit that item in place with
    /// `set-slot`/`set-field` instead of re-authoring it here.
    ///
    /// The fourth thing it does NOT do is rewrite the doc's title. The payload's
    /// `title:` is the create id-source; over a doc this task already holds (or a
    /// committed one copied in) the `# H1` stays as it is, so a `title:` that would be
    /// silently dropped is refused (`write.title-ignored`) — as is a singleton's, whose
    /// `# H1` is the schema's own `display-title`. A `title:` that mints a DIFFERENT id
    /// is refused too (`write.identity-change`): that is a second document, not a
    /// correction. Both reject the whole payload with nothing staged, and both route at
    /// `jigc doc rename`, the in-task title change.
    ///
    /// Payload shape (YAML; `--from-file`), mirroring the document's structure:
    ///
    /// ```yaml
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
    ///       - id: <section-id>        # a section that IS one slot (a simple section):
    ///         set: {<section-id>: <<…>>}   # its own slot, keyed by the section's own id
    /// ```
    #[command(verbatim_doc_comment)]
    Author {
        /// The doctype to author (e.g. `changelog`) — minted through the create-gate.
        doctype: String,
        /// The payload source: a path, or `-` for stdin (the whole-doc payload is
        /// large, so it arrives the same way slot prose does — never inline).
        #[arg(long = "from-file")]
        from_file: String,
        /// The active task to scope the write to. Optional: explicit wins; else the
        /// single active task; else (zero / more-than-one) the write rejects.
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
    ///
    /// The `{#id}` anchor on a rendered item heading IS that item's `<id>` — the
    /// address component every `#section/<id>` slice and every item write verb
    /// takes. It is minted from the title, not always equal to it (a release titled
    /// `1.0.0` has the id `1-0-0`), and it stays frozen across a retitle, so read the
    /// id off the anchor rather than slugging the title yourself; `--format json`
    /// carries the same value as each item object's `id` key.
    ///
    /// Stdout carries the addressed content and nothing else — no routing footer,
    /// no banner — so a read redirects or pipes straight into a file. Diagnostics,
    /// blocks, and the staged-elsewhere note ride stderr.
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
    /// (`contract-version: 5` — `design/doc-read-surface.md` → Why json is a contract
    /// here); plain text is a non-contractual human listing. Task-less — a schema
    /// projection is never task-scoped.
    Schema {
        /// The doctype whose resolved schema to project (e.g. `adr`).
        doctype: String,
    },
    /// List the store surface by identity, slug-sorted — **committed by default**.
    ///
    /// The fourth read surface, next to `describe` (the menu), `doc show` (the
    /// content read) and `doc schema` (the schema read). Every instance of one
    /// doctype (or of every persisted doctype) carries its `<type>:<slug>`
    /// identity, its repo-relative path, and its **registration state**:
    /// `managed` (jigc's own doc) or `unregistered` (a file at a managed home
    /// jigc never adopted — adopt it with `jigc ingest` / `jigc migrate <path>
    /// --as <doctype>`). `--format json` is the pinned shape
    /// `{"docs":[{id, path, state, item-count}]}` (no in-band version integer —
    /// `design/doc-read-surface.md` → the fourth read surface).
    ///
    /// **`--task <id>` lists what that task stages instead** — staged-only, the two
    /// views are never merged. Every staged row is `managed` (a staged working copy
    /// is jigc-written by construction), and its `path` is where the instance
    /// promotes to at finalize; a transient doctype (`commit:<task-id>`, sink = the
    /// git message) has no committed home, so it lists at its `<type>:<slug>`
    /// identity. A task-less listing served while an open task stages docs says so
    /// on **stderr** and hands over the staged listing; stdout is unchanged.
    List {
        /// The doctype to list (optional — omit to list every persisted doctype).
        doctype: Option<String>,
        /// List this open task's **staged** working copies instead of the committed
        /// store (same rows, staged-only).
        #[arg(long)]
        task: Option<String>,
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
    ///
    /// **The write-path P6 seam** (M47 Inc 6 T3; `design/surface-contract.md` → The route
    /// fence): this is the one door every `doc`-verb block passes through, and by the time
    /// it does, the target is established — either by the producer itself or by
    /// [`stamp_target`] one call earlier — so it is where a route's derivable placeholders
    /// are filled from the finding's own `key.target`
    /// ([`engine::finding::ROUTE_PLACEHOLDERS`]). Without it the engine's write routes reach
    /// an agent as `jigc doc schema <doctype>`: parseable, declared, and unrunnable.
    fn block(mut finding: Finding) -> Self {
        finding.substitute_derivable_route_placeholders();
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
            DocCommand::AddItem {
                addr,
                title,
                slug,
                task,
            } => run_add_item(cwd, &addr, &title, slug.as_deref(), task.as_deref(), format),
            DocCommand::RemoveItem { addr, task } => {
                run_remove_item(cwd, &addr, task.as_deref(), format)
            }
            DocCommand::RetitleItem { addr, title, task } => {
                run_retitle_item(cwd, &addr, &title, task.as_deref(), format)
            }
            DocCommand::Rename {
                addr,
                to,
                slug,
                task,
            } => run_doc_rename(cwd, &addr, &to, slug.as_deref(), task.as_deref(), format),
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
            DocCommand::List { doctype, task } => {
                run_list(cwd, doctype.as_deref(), task.as_deref(), format)
            }
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
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    // The **parsed** address in URI normal form — the target every block on this write keys
    // at (`design/command-output-contract.md` → the `write.*` row). Never the raw `addr`: a
    // bare singleton head is legal at the verb boundary, so `vision#thesis` would key a
    // slug-less address no driver can resolve.
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-field", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target (before `target` is consumed by the apply) + the written
    // value shaped through the same scalar/list grammar the read path re-parses it with,
    // so a write-ack and a `doc show` read-back agree on shape (a `0..*` bracket-list
    // value → a JSON array; `design/command-output-contract.md` §2).
    let ack_target = field_ack_target(&address, &target);
    let value_json = field_json(&engine::field_block::parse_value(value));

    // A mis-named item → `write.not-present`, enriched to the followable containing
    // section ([`enrich_not_present_route`]). It runs **past** the empty-value repoint: an
    // absent item outranks the clear-a-field footgun, since `--unset` at an item that was
    // never minted is no more writable than `--value ""` was.
    let edited = apply_field_target(&schema, &source, target, &uri, value)
        .map_err(|e| repoint_empty_value(e, addr, value))
        .map_err(|e| enrich_not_present_route(e, &uri, &task.id))?;

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
                copied_in,
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
///
/// Because it is the shared seam, the engine's **undeclared-address guard** reaches both
/// doors from one place: a `set-field` at a field leaf the schema does not declare is
/// rejected `write.unknown-field` before any bytes move, whether it arrives as the
/// per-leaf verb or as one lowered leaf of a `doc author` payload (M47 — the
/// undeclared-address table; `crates/cli/tests/undeclared_address_writes.rs`).
fn apply_field_target(
    schema: &Schema,
    source: &str,
    target: FieldTarget,
    uri: &str,
    value: &str,
) -> Result<String, DocFailure> {
    // **Item presence outranks both CLI-side guards below** (M47 — the write-verb ×
    // item-id-miss axis). Both are *schema-only* pre-checks: they answer from the doctype
    // alone and never look at the corpus, so at an item that was never minted they assert
    // a property of a nonexistent item (a law-1 lie) and hand back a route whose first
    // verb blocks on the same absence — costing a second hop and answering nothing. Gated
    // once **here**, at the shared seam, rather than inside each guard: the per-leaf verb
    // and the `doc author` batch inherit the ordering together, and a guard added to this
    // seam later is ranked by construction. Falling through leaves the engine's own
    // `write.not-present` + [`enrich_not_present_route`] to answer — the same
    // shape → presence → leaf order the engine's item-field doors keep
    // ([`engine::write::item_chain_absent`]).
    let item_present = !addressed_item_absent(schema, source, &target);
    // The set-field id-from guard (`design/write-commands.md` → The set-field id-from
    // guard): a heading-derived field is never written through `set-field` — living
    // here, the per-leaf verb AND the `doc author` batch (via `apply_leaf`) inherit
    // the reject in one place, killing the or-insert corruption shapes.
    if item_present && let Some(finding) = id_from_field_guard(schema, &target, uri, value) {
        return Err(DocFailure::block(finding));
    }
    // The set-field machine-maintained guard (`design/write-commands.md` → The set-field
    // machine-maintained guard): a `set:`-derived **absolute** (the freeze stamp / a
    // milestone transition) is CLI-owned and never author-writable — sharing this seam,
    // both the per-leaf verb and the `doc author` batch refuse the forge. `set: on-create`
    // is NOT absolute (a mint-time default the author may override), so it passes. The
    // presence rank is inert for today's members (the freeze stamp is doc-level, and a
    // milestone-record is refused whole upstream) — it is the seam's rule, not a patch.
    if item_present && let Some(finding) = machine_maintained_field_guard(schema, &target, uri) {
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
        } => set_item_field_validated(schema, source, &section, &[item.as_str()], &field, value)
            .map_err(|f| block(&f, "set-field", uri))?,
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            set_item_field_validated(schema, source, &section, &item_ids, &field, value)
                .map_err(|f| block(&f, "set-field", uri))?
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
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-field --unset", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = field_target(&schema, &address)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    let ack_target = field_ack_target(&address, &target);
    let outcome = apply_unset_target(&schema, &source, target, &uri)
        .map_err(|e| enrich_not_present_route(e, &uri, &task.id))?;

    // The no-op writes nothing: the state the agent asked for already held, and the
    // first-touch copy-in (announced by `copied_in`) is the only byte this call moved.
    let (edited, already_absent) = match outcome {
        engine::write::UnsetOutcome::Removed(edited) => {
            persist(&path, &edited)?;
            (edited, false)
        }
        engine::write::UnsetOutcome::AlreadyAbsent => (source, true),
    };
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
                already_absent,
                findings,
                copied_in,
            },
        )
    );
    Ok(())
}

/// Apply a resolved `--unset` (field removal) to `source`, dispatching by target kind to
/// the engine's byte-stable splice-remove — the `--unset` counterpart to
/// [`apply_field_target`]. An ineligible-field engine [`Finding`] surfaces through the
/// shared block envelope (its guard route preserved, a routeless splice error given the
/// generic retry route).
///
/// An **already-absent** field is not a failure and never reaches that envelope: the
/// engine answers [`engine::write::UnsetOutcome::AlreadyAbsent`] and the caller acks the
/// no-op (M49 Increment 1 T4).
fn apply_unset_target(
    schema: &Schema,
    source: &str,
    target: FieldTarget,
    uri: &str,
) -> Result<engine::write::UnsetOutcome, DocFailure> {
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

/// **Scope a boundary-door repair route to the task it must run in** (M49 Increment 8 / T3;
/// `design/validation.md` → The route floor; `design/surface-contract.md` → The route fence,
/// law 2). The gate blocks minted by `engine::validate::conformance_route` route a
/// `jigc doc` write at the finding's own address — a route the M43 fence proves *parses* and
/// M47's P6 proves carries no undeliverable placeholder, but which the **boundary doors**
/// (`crate::render::BOUNDARY_DOORS`) emit into a repo that may hold more than one active
/// task, where a task-selector-less `jigc doc` write exits 1 on `more than one active task`.
/// The door already holds the id — `jigc task validate <id>` / `jigc task finalize <id>` were
/// *given* it, and the milestone door reads it off the merged doc's contributing sub-task —
/// so `<task-id>`'s declared non-derivability (`engine::finding::ROUTE_PLACEHOLDERS`: "it
/// needs a different source — the CLI dispatch that resolved the task") is satisfied here,
/// the same CLI-side post-pass position [`enrich_not_present_route`] occupies.
///
/// **The subject is the route's shape, not a list of codes.** Any mechanical route naming a
/// `jigc doc` verb that `crate::cli::VERB_KINDS` classifies `Write` is a write into a task's
/// staged area and needs the selector; the classification is read from that clap-fenced
/// registry, so a doc write-verb added later is swept without an edit here. A route that
/// already names `--task` (the `write.not-present` enrichment above, the changelog-gate
/// advisory) is left exactly as it is.
///
/// **It can only ever emit a route that runs.** The derived argv is put back through the
/// fence's own predicate ([`crate::route_fence::accepts`]) before it is adopted, in every
/// build posture — so a doc write verb that takes no `--task`, or a task id that is not
/// shell-safe as emitted, leaves the original route standing rather than replacing it with a
/// second unrunnable one.
///
/// The selector is inserted **before the first flag**, after the positional run, so the
/// author-owned trailing placeholder a route ends on (`--value <value>`, `--from-file -`)
/// stays last — where an agent's eye and cursor already are.
pub(crate) fn scope_repair_route_to_task(finding: &mut Finding, task_id: &str) {
    let Some(route) = finding.route.as_ref() else {
        return;
    };
    let engine::finding::RouteKind::Mechanical { argv, tail } = route.kind() else {
        return;
    };
    if argv.first().map(String::as_str) != Some("jigc")
        || argv.get(1).map(String::as_str) != Some("doc")
    {
        return;
    }
    let Some(verb) = argv.get(2) else {
        return;
    };
    if crate::cli::verb_kind(&["doc", verb.as_str()]) != Some(crate::cli::VerbKind::Write) {
        return;
    }
    if argv.iter().any(|arg| arg == "--task") {
        return;
    }
    let at = argv
        .iter()
        .position(|arg| arg.starts_with('-'))
        .unwrap_or(argv.len());
    let mut scoped = argv.clone();
    scoped.splice(at..at, ["--task".to_owned(), task_id.to_owned()]);
    if !crate::route_fence::accepts(&scoped) {
        return;
    }
    let tail = tail.clone();
    finding.route = Some(engine::finding::Route::mechanical(scoped, tail));
}

/// Enrich a `write.not-present` reject's route to the **followable containing section**
/// (`jigc doc show <type>:<slug>#<section> --task <id>`) — the M44 Inc 2 route split
/// (`design/validation.md` → the `write.*` route split; `design/surface-contract.md` law 2:
/// nothing hides). A not-present is an **item-id** miss: the agent addressed an item that
/// was never minted, so the schema (the engine's defensive `jigc doc schema` route) is a
/// dead end — it names the shape, never the corpus's real item ids. The recovery is to
/// *show the containing section*, whose live item ids reveal the right address.
///
/// The section is the write address's fragment **top hop**, so a nested / field-leaf
/// address strips to the top **showable** section, never an unshowable field-leaf (the N2
/// pin).
///
/// Applied as a post-pass at **every write-verb dispatch that can produce a
/// `write.not-present`** — the site is where the real URI + resolved task id are in scope,
/// so the enrichment cannot live deeper. The applied set is the whole write-verb ×
/// item-id-miss axis (`crates/cli/tests/write_miss_shape_axis.rs`; M47 Inc 6 T2):
/// `run_set_slot` · `run_remove_item` (M44's two) and `run_set_field` (past
/// [`repoint_empty_value`]) · `run_unset_field` · `run_retitle_item` · `run_add_item` (the
/// four M47 wired, two of them turned into not-present producers by T1's engine flip).
/// **Adding a write verb means adding its call here** — the enumeration is fenced by that
/// suite's row-per-cell axis, not by this comment.
///
/// **Narrowed at its other edge (M49 Inc 1 T4):** `--unset`'s **field**-miss cell no
/// longer reaches here, because it is no longer a miss — an eligible field that is
/// already absent is the state the caller asked for, so the engine answers
/// [`engine::write::UnsetOutcome::AlreadyAbsent`] and the door acks the no-op. What used
/// to arrive was a present item's absent bullet wearing the *item*-miss route: "show the
/// section's item ids" over a section whose ids were fine, so the route terminated
/// nowhere and the re-run reproduced the error. The item-miss cell is untouched — this
/// enrichment is right for it, and only for it.
///
/// The widened domain's response holds at its edge: a `write.not-present` raised over a
/// **section-level** target (an absent structural home rather than an absent item id) gets
/// the same containing-section read — the section it could not find — the shape
/// `run_set_slot`'s own section-slot arm could already raise and enrich under M44, not a
/// new one invented here.
///
/// Deliberately **not** threaded through the shared `apply_slot_target` / `apply_field_target`
/// (the M44 rationale), so the batch `doc author` path keeps the engine's defensive
/// fallback route rather than a route built from an address it does not resolve per leaf. A
/// non-`not-present` block, or an orchestration failure, passes through unchanged.
fn enrich_not_present_route(failure: DocFailure, uri: &str, task_id: &str) -> DocFailure {
    let DocFailure::Block(mut finding) = failure else {
        return failure;
    };
    if finding.code != "write.not-present" {
        return DocFailure::Block(finding);
    }
    if let Some((_, fragment)) = uri.split_once('#')
        && let Some(section) = fragment.split('/').next()
    {
        let show_addr = format!("{}#{section}", doc_head(uri));
        finding.route = Some(engine::finding::Route::mechanical(
            ["jigc", "doc", "show", &show_addr, "--task", task_id],
            " to see the section's current item ids, then re-run the write at an existing item",
        ));
    }
    DocFailure::Block(finding)
}

/// Is the item a [`FieldTarget`] addresses **absent** from `source`? The
/// [`engine::write::item_chain_absent`] presence question, asked in the target's own
/// vocabulary: a section-level target names no item (never absent), an item / nested-item
/// target hands its parent-scoped chain straight through. A shape miss and an unparseable
/// source both answer "not absent", leaving those diagnoses to the splice path.
fn addressed_item_absent(schema: &Schema, source: &str, target: &FieldTarget) -> bool {
    match target {
        FieldTarget::Section { .. } => false,
        FieldTarget::Item { section, item, .. } => {
            engine::write::item_chain_absent(schema, source, section, &[item.as_str()])
        }
        FieldTarget::NestedItem { section, items, .. } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::item_chain_absent(schema, source, section, &item_ids)
        }
    }
}

/// The [`addressed_item_absent`] sibling over a [`RemoveItemTarget`] — the item forms the
/// `retitle-item` / `remove-item` doors resolve. Same presence question, same three
/// not-absent answers.
fn removable_item_absent(schema: &Schema, source: &str, target: &RemoveItemTarget) -> bool {
    match target {
        RemoveItemTarget::TopLevel { section, item } => {
            engine::write::item_chain_absent(schema, source, section, &[item.as_str()])
        }
        RemoveItemTarget::Nested { section, items } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::item_chain_absent(schema, source, section, &item_ids)
        }
    }
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

/// The set-field machine-maintained guard (`design/write-commands.md` → The set-field
/// machine-maintained guard; DECISIONS.md 2026-07-23 → M45 Increment 3). A `set:`-derived
/// **absolute** field — the freeze stamp (`set: schema-version`) or a milestone transition
/// (`set: on-transition`) — is **CLI-owned**: forging it through `set-field` committed a
/// value the deriver skips forever (a forged `schema-version: 99 ≥ current` made
/// `migrate-corpus` byte-untouch the doc and `version_currency_break` never report).
/// Refused with a blocking finding whose route names the legitimate deriver. `set:
/// on-create` is **not** absolute ([`engine::schema::is_machine_maintained_absolute`]
/// returns false) — the CLI merely defaults it at mint and the changelog-migration
/// historical-date path overwrites it — so it passes (the inert path). A non-`set:`
/// field, or an unresolvable address, likewise yields `None`.
///
/// Shares [`apply_field_target`] with the id-from guard, so the per-leaf `set-field` verb
/// and the batch `doc author` apply (via [`apply_leaf`]) inherit the reject in one place —
/// **not** the engine write primitives, which `transform.rs`'s optional-field fill and
/// `migrate_corpus`'s own stamp bump legitimately write through. `milestone-record` is
/// refused **whole** by [`machine_maintained_guard`] above this, so its `on-transition`
/// leaves never reach here; this per-leaf split guards the freeze stamp on every other
/// frozen doctype.
fn machine_maintained_field_guard(
    schema: &Schema,
    target: &FieldTarget,
    uri: &str,
) -> Option<Finding> {
    let field = declared_field(schema, target)?;
    if !engine::schema::is_machine_maintained_absolute(&field) {
        return None;
    }
    let route = if field.set.as_deref() == Some(engine::schema::SCHEMA_VERSION_SET) {
        "the schema-version stamp records which schema version this instance was authored \
         against — it is set at create and advanced only by `jigc migrate-corpus`, never a \
         manual write"
    } else {
        "the milestone-record's machine-maintained leaves change only through the milestone \
         verbs (`jigc milestone add-task` / `jigc milestone finalize`), never a manual write"
    };
    Some(Finding::graded(
        Severity::Blocking,
        "write.machine-maintained-field",
        format!(
            "set-field rejected: `{}` is a machine-maintained field — its value is \
             CLI-derived (`set: {}`) and is never written through a `jigc doc` verb",
            field.id,
            field.set.as_deref().unwrap_or_default()
        ),
        Some(Location::addressed(uri, 1, 1)),
        Some(route.into()),
    ))
}

/// Resolve the schema [`engine::schema::Field`] a [`FieldTarget`] addresses — the shared
/// lookup the machine-maintained guard reads the `set:` kind off, keyed on the **field**
/// (not the target position), so the guard is a fence over the declared field wherever it
/// lives. Returns an owned clone (the nested arm's repeatable is materialized by
/// [`engine::write::nested_repeatable`]); `None` for an unresolvable address / a
/// non-simple-or-repeatable body — the inert path (the engine splice adjudicates presence
/// as before).
fn declared_field(schema: &Schema, target: &FieldTarget) -> Option<engine::schema::Field> {
    let in_block = |repeatable: &Repeatable, field: &str| {
        repeatable.block.iter().find_map(|leaf| match leaf {
            Leaf::Field(f) if f.id == field => Some((**f).clone()),
            _ => None,
        })
    };
    match target {
        FieldTarget::Section { section, field } => {
            let SectionBody::Simple { fields, .. } =
                &schema.sections.iter().find(|s| &s.id == section)?.body
            else {
                return None;
            };
            fields.iter().find(|f| &f.id == field).cloned()
        }
        FieldTarget::Item { section, field, .. } => {
            let SectionBody::Repeatable { repeatable } =
                &schema.sections.iter().find(|s| &s.id == section)?.body
            else {
                return None;
            };
            in_block(repeatable, field)
        }
        FieldTarget::NestedItem {
            section,
            items,
            field,
        } => {
            // `items` is parents ++ [nested-section, item-id] (the `field_target` Deep
            // contract, mirrored in `id_from_field_guard`).
            let (_, rest) = items.split_last()?;
            let (nested_section, parents) = rest.split_last()?;
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            let repeatable =
                engine::write::nested_repeatable(schema, section, &parent_ids, nested_section)?;
            in_block(&repeatable, field)
        }
    }
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
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "set-slot", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target = slot_target(&schema, &address)?;

    let prose = read_handoff(from_file)?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the apply.
    let ack_target = slot_ack_target(&address, &target);

    let edited = apply_slot_target(&schema, &source, target, &uri, &prose)
        .map_err(|e| enrich_not_present_route(e, &uri, &task.id))?;

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
                copied_in,
            },
        )
    );
    Ok(())
}

/// Splice resolved slot prose into the in-memory `source`, returning the edited
/// buffer — the source→source transform shared by the per-leaf `set-slot` verb and
/// the batch `doc author` apply. No I/O (the [`apply_field_target`] sibling).
///
/// **Every arm goes through the one gated engine seam** (`set_slot_validated`, M45):
/// the CLI resolves the address and the engine derives that address's reserved
/// heading depth, enforces it before touching bytes, and validate-afters the write —
/// so an item-slot write can no longer land reserved-depth prose the section-slot
/// write would have refused (`implementation/parsing.md` → Slot heading-depth ceiling).
fn apply_slot_target(
    schema: &Schema,
    source: &str,
    target: SlotTarget,
    uri: &str,
    prose: &str,
) -> Result<String, DocFailure> {
    // The item arms' chains, kept alive for the borrowed `SlotAddress`.
    let (section, chain, leaf) = match &target {
        SlotTarget::Section(section) => (section.as_str(), Vec::new(), ""),
        SlotTarget::Item {
            section,
            item,
            leaf,
        } => (section.as_str(), vec![item.as_str()], leaf.as_str()),
        SlotTarget::NestedItem {
            section,
            items,
            leaf,
        } => (
            section.as_str(),
            items.iter().map(String::as_str).collect(),
            leaf.as_str(),
        ),
    };
    let address = if chain.is_empty() {
        SlotAddress::Section { section }
    } else {
        SlotAddress::Item {
            section,
            chain: &chain,
            leaf,
        }
    };
    set_slot_validated(schema, source, address, prose).map_err(|f| block(&f, "set-slot", uri))
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
    slug_override: Option<&str>,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    reject_malformed_slug(slug_override)?;
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "add-item", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        add_item_target(&address).with_context(|| format!("no section addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the apply: `section` +
    // the **minted** leaf-most item id (the new item — contract §2).
    let ack_target = add_item_ack_target(&address, &target, title, slug_override);

    let (edited, minted_path) = apply_add_item_target(
        &schema,
        &source,
        target,
        &uri,
        title,
        slug_override,
        &task.id,
        task.is_migration()?,
        // The staged instance was read (or copied in) above and a rejected write persists
        // nothing, so it is still there for the reject's route to name — with exactly
        // these bytes, since this verb writes once and the reject is before that write.
        RejectDestination::Reachable { surviving: &source },
    )
    .map_err(|e| enrich_not_present_route(e, &uri, &task.id))?;

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
                copied_in,
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
#[allow(clippy::too_many_arguments)]
fn apply_add_item_target(
    schema: &Schema,
    source: &str,
    target: AddItemTarget,
    uri: &str,
    title: &str,
    slug_override: Option<&str>,
    task_id: &str,
    migration: bool,
    destination: RejectDestination<'_>,
) -> Result<(String, String), DocFailure> {
    // Write-time id-from-enum reject (`design/auto-migration.md` → Hardening #3;
    // write-commands.md → Two check times): when the destination repeatable's `id-from`
    // is an enum the `--title` re-slugs outside, block here — fast feedback at the point
    // of the mistake, not deferred to finalize. The batch (`apply_leaf`) inherits this
    // by sharing this path.
    if let Some(finding) = id_from_enum_block(schema, doc_head(uri), &target, title, slug_override)
    {
        return Err(DocFailure::block(finding));
    }
    // The collision locus, captured before the apply consumes `target` — the section and
    // the parent-scoped chain prefix an `already-present` reject's recovery needs.
    let locus = add_item_locus(schema, &target, title, slug_override);
    let applied = match target {
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
            let edited = engine::write::add_item(
                schema,
                source,
                &section,
                title,
                slug_override,
                None,
                &on_create,
            )
            .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", uri));
            let minted = format!("{}/{}", section, minted_item_id(title, slug_override));
            edited.map(|edited| (edited, minted))
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
                slug_override,
                None,
                &on_create,
            )
            .map_err(|e| block(&engine::write::generate_error_finding(&e), "add-item", uri));
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
                minted_item_id(title, slug_override),
            );
            edited.map(|edited| (edited, minted))
        }
    };
    applied.map_err(|failure| {
        enrich_already_present_route(
            failure,
            schema,
            source,
            locus.as_ref(),
            uri,
            task_id,
            destination,
        )
    })
}

/// Where an `add-item` mint lands, in the vocabulary an **`already-present`** reject's
/// recovery needs: the section, the parent-scoped chain **prefix** above the new item
/// (empty for a top-level mint; `parents… + <nested-section>` for a nested one, the
/// section-qualified form [`engine::write::item_chain_absent`] takes), the id the call
/// **minted**, and the `--title` as one shell-safe token.
struct AddItemLocus {
    section: String,
    prefix: Vec<String>,
    minted: String,
    title_token: String,
}

/// Resolve the [`AddItemLocus`] of a mint, or `None` where a `--slug` recovery **is not
/// available** — which is the whole point of asking: an enum `id-from` block's heading IS
/// the member and its anchor equals it, so an override there is refused as an identity
/// change ([`slug_override_enum_refusal`]). Emitting a `--slug` route into that block
/// would hand back a command that blocks when run, which is the un-followable route the
/// write-verb × miss-shape axis exists to forbid — so the collision keeps its shipped
/// human route there ("edit it in place"), which is the true answer when the id *is* the
/// category. An unresolvable destination yields `None` for the same reason.
fn add_item_locus(
    schema: &Schema,
    target: &AddItemTarget,
    title: &str,
    slug_override: Option<&str>,
) -> Option<AddItemLocus> {
    let (repeatable, _) = add_item_destination(schema, target)?;
    if id_from_is_enum(&repeatable) {
        return None;
    }
    let (section, prefix) = match target {
        AddItemTarget::TopLevel { section } => (section.clone(), Vec::new()),
        AddItemTarget::Nested {
            section,
            parents,
            nested_section,
        } => {
            let mut prefix = parents.clone();
            prefix.push(nested_section.clone());
            (section.clone(), prefix)
        }
    };
    Some(AddItemLocus {
        section,
        prefix,
        minted: minted_item_id(title, slug_override),
        title_token: crate::task::shell_token(title),
    })
}

/// **What becomes of the destination doc once this write's failure is unwound** — the
/// one input an `already-present` reject's recovery cannot derive from the buffer it was
/// handed, and must therefore be told.
///
/// The enriched route is a *mint into a doc*, so it is only a recovery while there is a
/// doc to mint into. That holds at the per-leaf `add-item` verb by construction (it read
/// or copied in the staged instance, and a rejected write persists nothing, so the
/// instance stands) — and it does **not** hold on `doc author`'s **create** arm, whose
/// whole-or-nothing rollback removes the very file the create provisioned
/// ([`engine::state::CreatedDoc::rollback`]). The batch is the shape every `migrate-*`
/// workflow drives, and there the shipped enrichment printed a copy-runnable-looking
/// command that exits 1 — `surface-contract.md` law 1, in a route (M49 Increment 5, the
/// T5 triage).
///
/// Asked at the caller because only the caller knows: `create_gated`'s `existed`
/// discriminator answers it for the batch (a same-identity **staged** copy is restored
/// from its pre-image; a **committed** instance is still at its canonical home for
/// copy-on-first-touch; a fresh mint leaves neither), and it is a constant at the verb.
///
/// The reachable arm carries the surviving **bytes**, not merely the fact of survival:
/// *which item ids are there afterwards* is the second thing the recovery cannot derive
/// from the buffer it was handed, and it is a different question. The batch chains its
/// leaves over one in-progress buffer, so an id an **earlier payload item** minted is in
/// that buffer and is **not** in the doc the rollback restores — probing the buffer both
/// asserts an item that is not there and suffixes around a bare id that is free (M49
/// Increment 5, the T5 triage's own validation finding). The caller supplies the batch's
/// **pre-chain** buffer, which is exactly what the rollback leaves: the pre-image it
/// restores over a staged copy, and the still-untouched committed body it copied in.
#[derive(Clone, Copy)]
enum RejectDestination<'a> {
    /// The doc is still addressable after the reject — staged, or committed at its
    /// canonical home — and `surviving` is its content **as of then**. A mint route
    /// naming it runs, and is probed against these bytes.
    Reachable { surviving: &'a str },
    /// The write provisioned the doc itself and its failure discards it: nothing is
    /// staged and nothing is committed, so **no** route naming that doc runs.
    Discarded,
}

/// Enrich a `write.already-present` **item** collision's route into the mint that
/// actually recovers it (M49 Increment 5, T5) — the [`enrich_not_present_route`] sibling,
/// applied at the one shared site both doors funnel through
/// ([`apply_add_item_target`]), so the per-leaf `add-item` verb and the `doc author`
/// batch cannot disagree about it.
///
/// The shipped route was a [`engine::finding::Route::human`] — *"the target already
/// exists — edit it in place"* — which is the right answer for a **correction** and the
/// wrong one for a **second, distinct entry**: two genuinely different titles can slug
/// alike (`1-3-0` and `1.3.0` mint one id), and until `add-item` gained `--slug` (T4 of
/// this increment) there was no command that recovered it at all. Where the destination
/// survives the reject the route is now that same mint under the **first free**
/// `<minted>-N` id, argv-complete and copy-runnable, with the edit-in-place branch kept
/// as the tail's alternative rather than dropped.
///
/// **Every claim the mint makes is adjudicated against the bytes the reject leaves
/// behind** ([`RejectDestination`]), never the buffer the write was attempted on — for
/// `doc author` those are two different documents, and the enrichment shipped probing the
/// wrong one. Two arms fall out, and both were live defects the batch reached (the T5
/// triage and its validation finding):
///
///   * the destination does not survive at all — `doc author`'s create arm rejects the
///     batch whole and rolls its own provisioning back, so the mint (like the shipped
///     edit-in-place) would name a doc that is not there;
///   * the destination survives but the **colliding id does not** — the id was minted by
///     an earlier item of this same payload, which the rollback took with it, so *"beside
///     the item already there"* is false and the `-N` suffix is derived around a bare id
///     that is free. Run verbatim it exits 0 and lands one mis-suffixed item while the
///     rest of the payload stays unauthored. This is the deliverable's own headline case
///     (890 titles minting 884 slugs), and the acceptance was shaped around the other one.
///
/// Both are told the recovery that exists, which is the payload's: the batch landed
/// nothing, so revising it and re-running is the complete repair, where a single mint
/// would be a partial one.
///
/// Deliberately narrow at three edges. It fires only on `write.already-present`; only
/// where the collision is **confirmed** to be the minted item id (the reject's other
/// possible subject — a structural home the generator found present — is left with the
/// route it has); and only where a `--slug` would actually be accepted (`locus` is `None`
/// over an enum `id-from`, see [`add_item_locus`]). The **doc-level** `create` collision
/// is a different subject with its own meaning and its own text, and never reaches here.
fn enrich_already_present_route(
    failure: DocFailure,
    schema: &Schema,
    source: &str,
    locus: Option<&AddItemLocus>,
    uri: &str,
    task_id: &str,
    destination: RejectDestination<'_>,
) -> DocFailure {
    let DocFailure::Block(mut finding) = failure else {
        return failure;
    };
    if finding.code != "write.already-present" {
        return DocFailure::Block(finding);
    }
    // A discarded destination out-ranks the mint: no command naming that doc runs, so
    // both the mint and the shipped *"edit it in place"* would be advice about a file
    // that is not there. The recovery that does exist is the payload's — the batch landed
    // nothing, so revising it and re-running is a complete repair, and it is stated
    // without asserting which subject collided (an earlier payload item, or a body the
    // create copied in), because the route is emitted for both.
    let surviving = match destination {
        RejectDestination::Discarded => {
            finding.route = Some(engine::finding::Route::human(
                "nothing landed — `jigc doc author` rejects the batch whole and discards \
                 the doc it provisioned, so there is nothing staged to edit in place or \
                 to mint beside: revise the payload so this write lands (an item id is \
                 slugged from its title, so two titles that slug alike claim one id) and \
                 re-run the whole `jigc doc author`",
            ));
            return DocFailure::Block(finding);
        }
        RejectDestination::Reachable { surviving } => surviving,
    };
    // The collision is only *this* enrichment's subject where the minted id is what the
    // write actually hit — asked against `source`, the buffer the reject was adjudicated
    // on. Everything after is asked against `surviving`, because everything after is a
    // claim about the doc the agent will find.
    if let Some(locus) = locus
        && item_taken(schema, source, locus, &locus.minted)
    {
        if !item_taken(schema, surviving, locus, &locus.minted) {
            // The id is taken in the buffer and free in what survives: the duplicate came
            // from an **earlier item of this same payload**, which the whole-or-nothing
            // rollback took with it. There is nothing to mint beside, the bare id is not
            // in fact claimed, and a single `add-item` would land one item of the payload
            // under a suffix nothing asked for while the rest stayed unauthored — so the
            // recovery offered is the payload's, the same one the discarded arm prints.
            finding.route = Some(engine::finding::Route::human(
                "nothing from this payload landed — `jigc doc author` rejects the batch \
                 whole and unwinds it, and the id that collided is not in the doc it \
                 leaves behind: the duplicate is inside the payload itself (an item id is \
                 slugged from its title, so two titles that slug alike claim one id). \
                 Revise the payload so each item mints its own id and re-run the whole \
                 `jigc doc author`",
            ));
        } else if let Some(free) = first_free_item_id(schema, surviving, locus) {
            finding.route = Some(engine::finding::Route::mechanical(
                [
                    "jigc",
                    "doc",
                    "add-item",
                    uri,
                    "--title",
                    &locus.title_token,
                    "--slug",
                    &free,
                    "--task",
                    task_id,
                ],
                " mints it beside the item already there under an id of your own — \
                 `--slug` drives the item id verbatim, so two titles that slug alike can \
                 coexist. If this is a correction of that item rather than a second \
                 entry, edit it in place with `jigc doc set-slot` / `jigc doc set-field` \
                 instead",
            ));
        }
    }
    DocFailure::Block(finding)
}

/// The first free `<minted>-N` sibling id at `locus`, or `None`.
///
/// `None` is the **honest** answer twice over, and both matter: when the minted id is not
/// in fact present the reject was about something other than an item collision, so a
/// route offering a fresh item id would be inventing a diagnosis; and when no suffix in
/// the probed range is free the route would name an id that itself collides — a command
/// that blocks when run. Presence is asked through the shared
/// [`engine::write::item_chain_absent`], the same predicate the item-miss doors rank on,
/// so "taken" means here exactly what it means there. Runs on the **reject** path only.
fn first_free_item_id(schema: &Schema, source: &str, locus: &AddItemLocus) -> Option<String> {
    if !item_taken(schema, source, locus, &locus.minted) {
        return None;
    }
    (2..=99)
        .map(|n| format!("{}-{n}", locus.minted))
        .find(|candidate| !item_taken(schema, source, locus, candidate))
}

/// Is `id` occupied at `locus` in `source`? Asked through the shared
/// [`engine::write::item_chain_absent`], the same predicate the item-miss doors rank on,
/// so "taken" means here exactly what it means there. The **source is a parameter on
/// purpose**: the collision's subject is adjudicated against the buffer the write hit,
/// while every claim the route makes is adjudicated against the bytes the reject leaves
/// behind, and for the batch those are two different documents.
fn item_taken(schema: &Schema, source: &str, locus: &AddItemLocus, id: &str) -> bool {
    let mut chain: Vec<&str> = locus.prefix.iter().map(String::as_str).collect();
    chain.push(id);
    !engine::write::item_chain_absent(schema, source, &locus.section, &chain)
}

/// The destination repeatable an [`AddItemTarget`] mints into, with the address **tail**
/// naming it (`<section>` for a top-level mint, the section-qualified
/// `<section>/<parents…>/<nested-section>` chain for a nested one) — the navigation the
/// mint path itself uses (the section's own body, or the engine's
/// [`engine::write::nested_repeatable`]). `None` when the address names no declared
/// repeatable, leaving that diagnosis to the splice path.
fn add_item_destination(schema: &Schema, target: &AddItemTarget) -> Option<(Repeatable, String)> {
    match target {
        AddItemTarget::TopLevel { section } => {
            let body = &schema.sections.iter().find(|s| &s.id == section)?.body;
            let SectionBody::Repeatable { repeatable } = body else {
                return None;
            };
            Some((repeatable.clone(), section.clone()))
        }
        AddItemTarget::Nested {
            section,
            parents,
            nested_section,
        } => {
            let parent_ids: Vec<&str> = parents.iter().map(String::as_str).collect();
            let repeatable =
                engine::write::nested_repeatable(schema, section, &parent_ids, nested_section)?;
            Some((
                repeatable,
                format!("{section}/{}/{nested_section}", parents.join("/")),
            ))
        }
    }
}

/// The write-time id-from reject for an `add-item` mint (`design/auto-migration.md` →
/// Hardening #3; write-commands.md → Two check times). Resolves the destination
/// repeatable — the section's own for a top-level mint, the named nested one (via the
/// engine's [`engine::write::nested_repeatable`], the same navigation the mint path uses)
/// for a nested mint — and runs the shared [`engine::validate::id_from_enum_violation`]
/// adjudicator over the `--title`. It returns a blocking finding carrying the **shared**
/// code [`engine::validate::ID_FROM_ENUM_CODE`] (identical to finalize's) for either arm:
/// a **shape** violation (the universal rule — every id-from's `--title` must be a stable
/// single-line non-blank string, so this fires ahead of the engine's slug guard and names
/// the shape reason), a **trailer-key** violation (the commit-scoped rule — a `commit`
/// trailer `key` must be a well-shaped git-trailer token, no internal whitespace or colon),
/// or an **enum-member** miss (the title re-slugs outside the declared
/// members). Addressed at the slug-cased id-from address, **qualified by the doc head**
/// `doc` (`<type>:<slug>`) into the URI normal form its stable key targets. A clean title
/// over a non-enum id-from yields `None` — the inert path, mirroring finalize's exemption
/// (so `Fixed`→`fixed` passes).
fn id_from_enum_block(
    schema: &Schema,
    doc: &str,
    target: &AddItemTarget,
    title: &str,
    slug_override: Option<&str>,
) -> Option<Finding> {
    let (repeatable, prefix) = add_item_destination(schema, target)?;
    // The **override** refusal, ahead of the membership test and independent of the
    // title: where the id-source is an enum, the heading IS the member and the anchor
    // equals it, so a `--slug` is a second identity beside the member — the same
    // adjudication `retitle-item` makes on the same block shape
    // ([`retitle_id_from_refusal`]), converging on the shipped `write.identity-change`
    // rather than minting a code (`design/write-commands.md` → Identity divergence).
    if let Some(slug) = slug_override
        && id_from_is_enum(&repeatable)
    {
        return Some(slug_override_enum_refusal(
            doc,
            &prefix,
            &repeatable.id_from,
            title,
            slug,
        ));
    }
    let violation = engine::validate::id_from_enum_violation(&repeatable, title, &schema.ty)?;
    // The route floor (M43): every arm names a followable repair. A **shape** violation's
    // fix is the `--title` value itself (a human correction jigc cannot execute); an
    // **enum-member** miss routes the mechanical `doc schema` read for the declared members.
    let (message, route, slug) = match &violation {
        engine::validate::IdFromViolation::Shape(reason) => (
            format!(
                "add-item rejected: the heading {reason} (id-from field `{}`)",
                repeatable.id_from
            ),
            engine::finding::Route::human(
                "re-run `add-item` with a non-empty, single-line `--title` \
                 without leading or trailing whitespace",
            ),
            engine::slug::slugify(title),
        ),
        engine::validate::IdFromViolation::TrailerKeyShape(reason) => {
            return Some(trailer_key_refusal(
                "add-item",
                reason,
                &repeatable.id_from,
                format!(
                    "{doc}#{prefix}/{}/{}",
                    engine::slug::slugify(title),
                    repeatable.id_from
                ),
            ));
        }
        engine::validate::IdFromViolation::NotEnumMember(slug) => (
            format!(
                "add-item rejected: `{slug}` is not an enum member of id-from field `{}`",
                repeatable.id_from
            ),
            engine::finding::Route::mechanical(
                ["jigc", "doc", "schema", "<doctype>"],
                " to see the declared members, then re-run `add-item` with a member title",
            ),
            slug.clone(),
        ),
    };
    Some(Finding::graded(
        engine::finding::Severity::Blocking,
        engine::validate::ID_FROM_ENUM_CODE,
        message,
        Some(Location::addressed(
            format!("{doc}#{prefix}/{slug}/{}", repeatable.id_from),
            1,
            1,
        )),
        Some(route),
    ))
}

/// Whether a repeatable's `id-from` names an **enum** field of its own block — the one
/// predicate both enum-id-from refusals read, so the two doors cannot disagree about
/// what an enum id-source is: `add-item`'s `--slug` refusal ([`id_from_enum_block`])
/// and `retitle-item`'s member-change refusal ([`retitle_id_from_refusal`]). A block
/// that does not declare its own id-from leaf (the loader never requires it) is not an
/// enum source — the inert answer both doors already took.
fn id_from_is_enum(repeatable: &engine::schema::Repeatable) -> bool {
    repeatable.block.iter().any(|leaf| match leaf {
        engine::schema::Leaf::Field(f) => {
            f.id == repeatable.id_from && f.ty == engine::schema::FieldType::Enum
        }
        _ => false,
    })
}

/// The `add-item --slug` refusal over an **enum** id-source. `prefix` is the
/// destination block's address tail (the section for a top-level mint, the
/// section-qualified nested chain for a nested one), so `<doc>#<prefix>` is exactly the
/// address the call named and the route is the same mint minus the override — a
/// mechanical re-run, argv-checked like every other mechanical route.
fn slug_override_enum_refusal(
    doc: &str,
    prefix: &str,
    id_from: &str,
    title: &str,
    slug: &str,
) -> Finding {
    let dest = format!("{doc}#{prefix}");
    Finding::graded(
        Severity::Blocking,
        "write.identity-change",
        format!(
            "add-item rejected: `{prefix}` derives its item id from enum field \
             `{id_from}`, so the heading IS the member and the anchor equals it — \
             `--slug {slug}` would mint a second identity beside the member, not an id"
        ),
        Some(Location::addressed(
            format!("{doc}#{prefix}/{slug}/{id_from}"),
            1,
            1,
        )),
        Some(engine::finding::Route::mechanical(
            [
                "jigc",
                "doc",
                "add-item",
                &dest,
                "--title",
                &crate::task::shell_token(title),
            ],
            " mints the member itself; a name of your own belongs in the item's prose",
        )),
    )
}

/// The commit-trailer key-shape refusal both title-writing doors compose — `add-item`
/// ([`id_from_enum_block`]) and `retitle-item` ([`retitle_id_from_refusal`]) — from the
/// shared adjudicator's [`engine::validate::IdFromViolation::TrailerKeyShape`] arm. One
/// verb-parameterized constructor (message + route + the shared
/// [`engine::validate::ID_FROM_ENUM_CODE`]) so the two doors' refusal shape cannot
/// drift (the confidence-audit wave — sibling-hunt finding 6: the retitle door shipped
/// without the rule, so a well-shaped trailer key could be retitled to `BREAKING
/// CHANGE` at write time and only blocked later at the task gate). `address` is the
/// full-URI id-from leaf address the door targets — the mint-time slug form at
/// add-item, the frozen item path at retitle.
fn trailer_key_refusal(verb: &str, reason: &str, id_from: &str, address: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        engine::validate::ID_FROM_ENUM_CODE,
        format!("{verb} rejected: the trailer key {reason} (id-from field `{id_from}`)"),
        Some(Location::addressed(address, 1, 1)),
        Some(engine::finding::Route::human(format!(
            "re-run `{verb}` with a trailer key that is a single git-trailer token \
             — no whitespace or colon (e.g. `Co-Authored-By`)"
        ))),
    )
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
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "remove-item", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    let target =
        remove_item_target(&address).with_context(|| format!("no item addressed by `{addr}`"))?;

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // The decomposed ack target, before `target` is consumed by the removal match.
    let ack_target = item_ack_target(&address, &target);

    let edited = match target {
        RemoveItemTarget::TopLevel { section, item } => {
            engine::write::remove_item(&schema, &source, &section, &item)
        }
        RemoveItemTarget::Nested { section, items } => {
            let item_ids: Vec<&str> = items.iter().map(String::as_str).collect();
            engine::write::remove_nested_item(&schema, &source, &section, &item_ids)
        }
    }
    // A mis-named item → `write.not-present`, enriched to the followable containing section
    // (M44 Inc 2, [`enrich_not_present_route`]); the resolved task id + real URI are in hand.
    .map_err(|e| {
        enrich_not_present_route(
            block(
                &engine::write::splice_error_finding(&e),
                "remove-item",
                &uri,
            ),
            &uri,
            &task.id,
        )
    })?;

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
                copied_in,
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
/// The **id-from refusals** run first ([`retitle_id_from_refusal`]): an item whose id
/// derives from an enum field has its heading AS the enum member and its anchor EQUAL
/// to it, so any member change is an identity change, not a retitle — refused with a
/// blocking finding routing to `remove-item` + `add-item` under the target category
/// (the Settle-decided route); and a `commit` trailer item's new title must be a
/// well-shaped git-trailer token (the same shared adjudicator the `add-item` door
/// runs — the confidence-audit wave, sibling-hunt finding 6). Both fire before any
/// bytes are **moved**, and both are outranked by **item presence** (M47 — the
/// write-verb × item-id-miss axis): they are schema-only checks, so at an item that was
/// never minted they would describe a nonexistent item and route to a verb that blocks
/// on the same absence. The doctype-keyed milestone-record refusal above them is
/// deliberately **not** presence-gated: "no `jigc doc` write applies to this doctype" is
/// true whether or not the item exists, and its route is informational — it dead-ends
/// nowhere.
fn run_retitle_item(
    cwd: &Path,
    addr: &str,
    title: &str,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
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

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;

    // **Item presence outranks the id-from refusals** (M47 — the write-verb ×
    // item-id-miss axis), which is why they run after the read rather than before it: an
    // enum id-from refusal at an item that was never minted asserts what a nonexistent
    // item derives its id from, and routes to a `remove-item` that blocks on the same
    // absence. At an absent item the engine's `write.not-present` +
    // [`enrich_not_present_route`] below answer instead. Nothing is *moved* by the read —
    // a rejected retitle still persists nothing.
    if !removable_item_absent(&schema, &source, &target)
        && let Some(finding) = retitle_id_from_refusal(&schema, &address, &target, addr, title)
    {
        return Err(DocFailure::block(finding));
    }

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
        enrich_not_present_route(
            block(
                &engine::write::generate_error_finding(&e),
                "retitle-item",
                &uri,
            ),
            &uri,
            &task.id,
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
                copied_in,
            },
        )
    );
    Ok(())
}

/// The id-from refusals for `retitle-item`, two arms over one resolved repeatable
/// (`design/write-commands.md` → `jigc doc retitle-item`; DECISIONS.md 2026-07-10 →
/// the REFUSE + remove/add settled fork; the confidence-audit wave — sibling-hunt
/// finding 6). Resolves the repeatable the addressed item lives in — the section's own
/// for a top-level item, the named nested one (via the engine's
/// [`engine::write::nested_repeatable`], the [`id_from_enum_block`] navigation) for a
/// chain — then:
///
/// 1. The **unconditional enum-id-from refusal**: when the `id-from` field is an
///    **enum**, refuse *regardless of the new title* (unlike `add-item`'s membership
///    test) — the heading IS the member and the anchor equals it, so a
///    member-to-member change is an identity change, not a retitle. The route names
///    `doc remove-item` on the item + `doc add-item` under the target category,
///    moving the prose in the same motion.
/// 2. The **commit-trailer key-shape refusal** (non-enum arm): the same shared
///    [`engine::validate::id_from_enum_violation`] adjudicator the `add-item` door
///    runs, over the engine-trimmed title — a `commit` trailer key bearing internal
///    whitespace or a colon would break the `%(trailers)` block, so it is refused at
///    the point of the mistake with the wired door's refusal shape
///    ([`trailer_key_refusal`]), not deferred to the task gate.
///
/// A non-enum, non-trailer-violating id-from (arch-doc's `title`, changelog's release
/// `title`) yields `None` — the inert path.
fn retitle_id_from_refusal(
    schema: &Schema,
    address: &Address,
    target: &RemoveItemTarget,
    addr: &str,
    title: &str,
) -> Option<Finding> {
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
    if !id_from_is_enum(&repeatable) {
        // The non-enum arm — the commit-trailer key-shape refusal (sibling-hunt
        // finding 6): run the SAME shared adjudicator the add-item door runs
        // ([`id_from_enum_block`]), over the engine-trimmed title (the engine
        // [`engine::write::retitle_item`] trims before splicing, so edge whitespace
        // never lands — but internal whitespace/colon would, and its `check_value`
        // on a plain string key passes them). Only the trailer arm blocks here:
        // `Shape` on a trimmed title (empty / embedded control char) is the engine's
        // own reject contract, and `NotEnumMember` is unreachable (an enum id-from
        // is refused below, title-independent).
        if let Some(engine::validate::IdFromViolation::TrailerKeyShape(reason)) =
            engine::validate::id_from_enum_violation(&repeatable, title.trim(), &schema.ty)
        {
            return Some(trailer_key_refusal(
                "retitle-item",
                reason,
                &repeatable.id_from,
                format!("{doc}#{item_path}/{}", repeatable.id_from),
            ));
        }
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
    let value = if field.ty == FieldType::Date
        && field.set.as_deref() == Some(engine::schema::SET_ON_CREATE)
    {
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
pub(crate) fn on_create_doc_fields(
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
pub(crate) fn stamp_schema_version(pack: &dyn PackSource, doctype: &str) -> u32 {
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

/// `jigc doc rename <addr> --to "<New Title>"` (optional `--slug`) — the **in-task
/// doc-level title change**, the staged sibling of the top-level [`crate::rename`]
/// (`design/write-commands.md` → `jigc doc rename`; `design/storage.md` → Identity).
///
/// The split is **committed-store identity**, which is the identity model's own subject
/// (*"identity is the path"*): a doc this task minted has no committed referrers **by
/// construction**, so re-slugging it is the same act as minting it correctly and the
/// whole referrer set is the CLI-owned task area; a doc **copied in** from the committed
/// store has referrers this verb cannot see and a path other clones already hold, so it
/// is retitle-only and a genuine re-slug routes at `jigc rename` — task-less,
/// transactional, referrer-repointing.
///
/// **The discriminator is the persisted provenance**, not a fresh `canonical_path`
/// probe: `docs/provenance.json` records the outcome of `state::create`'s *own* copy-in
/// predicate at stage time ([`state::Provenance::EditedFromBase`] iff the committed body
/// was carried in), **including its in-location-squatter exception** — a migration whose
/// source path IS the doctype's canonical destination stages `created` over a *foreign*
/// file, and a bare `canonical_path(..).is_file()` would read that squatter as a
/// committed managed identity and hand back a `jigc rename` route that blocks on a doc
/// it cannot find. Same predicate, read where it was already answered.
///
/// Ranking (M47 — the write-verb miss axis): the **argument-shape** reject (a `#fragment`
/// at a whole-doc verb) and the **doctype-wide** refusals run before any read — they
/// claim nothing about the addressed instance — while the identity split runs after it,
/// because it is a question about the corpus.
fn run_doc_rename(
    cwd: &Path,
    addr: &str,
    to: &str,
    slug_override: Option<&str>,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    reject_malformed_slug(slug_override)?;
    let task = ActiveTask::resolve(cwd, task_id)?;
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
    // Argument shape, above everything: `rename` addresses a whole doc. Without this the
    // trailing hop would be silently dropped and the verb would ack a rename of the
    // container the agent did not name — the exact "success over a silent no-op" shape
    // this increment exists to close.
    if address.fragment.is_some() {
        let doc = format!("{}:{}", address.r#type.as_str(), address.slug.as_str());
        return Err(DocFailure::Orchestration(anyhow!(
            "`{addr}` addresses part of a doc — `jigc doc rename` renames the whole doc \
             (its `# H1`, and its slug while the identity is uncommitted); a repeatable \
             item is retitled with `jigc doc retitle-item`\n  route: run {}",
            engine::finding::Route::mechanical(
                [
                    "jigc",
                    "doc",
                    "rename",
                    &doc,
                    "--to",
                    &crate::task::shell_token(to),
                ],
                "",
            ),
        )));
    }
    let uri = address.to_string();
    machine_maintained_guard(address.r#type.as_str(), "rename", &uri)?;
    let schema = task.schema(address.r#type.as_str())?;
    // The doctype-wide refusals: a doctype whose id and `# H1` the CLI supplies has no
    // author-owned title to move, whether or not an instance is staged.
    if let Some(finding) = fixed_identity_refusal(&schema, &uri) {
        return Err(DocFailure::block(finding));
    }

    let new_slug = match slug_override {
        Some(slug) => slug.to_string(),
        None => engine::slug::slugify(to),
    };
    if new_slug.is_empty() {
        return Err(DocFailure::Orchestration(anyhow!(
            "`--to {to:?}` slugs to nothing — pass an explicit `--slug <slug>`"
        )));
    }

    let path = staged_path(&task.dir, &address, &task.id)?;
    let EditBase { source, copied_in } = task.read_or_copy_in(&path, &schema, &address, addr)?;
    let committed_identity = matches!(
        state::ProvenanceRecord::load(&task.dir)
            .context("could not read the task's staged-doc provenance")?
            .get(&uri),
        Some(state::Provenance::EditedFromBase)
    );
    let reslugged = new_slug != address.slug.as_str();
    if committed_identity && reslugged {
        return Err(DocFailure::block(committed_reslug_refusal(
            &uri,
            &new_slug,
            to,
            slug_override,
        )));
    }

    let new_uri = format!("{}:{new_slug}", address.r#type.as_str());
    // The destination-occupancy guard runs **before a byte is written** — a refused
    // rename must not leave the source doc retitled at its old id (the half-applied
    // state the pre-hoist order produced: `rewrite_h1` had already landed the new `# H1`
    // when the collision refusal fired).
    let new_path = if reslugged {
        Some(free_destination(
            &task, &schema, &new_slug, &new_uri, &uri, to,
        )?)
    } else {
        None
    };

    let retitled = crate::rename::rewrite_h1(&source, to)
        .ok_or_else(|| anyhow!("the staged doc `{uri}` carries no `# H1` to retitle"))?;
    persist(&path, &retitled)?;

    if let Some(new_path) = new_path {
        move_staged_identity(&task, &address, &path, &new_path, &new_slug, &new_uri)?;
    }

    let target = whole_doc_ack_target(&new_uri)?;
    let findings = write_ack_findings(&schema, &retitled, &target.doctype, &target.slug);
    println!(
        "{}",
        render::doc_ack(
            format,
            &render::DocAck::Renamed {
                address: new_uri,
                target,
                title: to.to_string(),
                from: uri,
                reslugged,
                committed_identity,
                findings,
                copied_in,
            },
        )
    );
    Ok(())
}

/// The **fixed-identity refusal**: a doctype whose id *and* `# H1` the CLI supplies has
/// no author-owned title for `doc rename` to move, so the verb refuses by **doctype**,
/// above any instance question. Two shapes, one code — `write.identity-change`, the
/// shipped *"you are changing identity through a verb that cannot"* member (its other
/// producer is `retitle-item` under an enum `id-from`):
///
/// * a **singleton** ([`Schema::fixed_title`] — the one home of this rule, keyed on
///   `singleton:` itself and never re-derived from the knobs a shipped singleton happens
///   to carry): its slug IS the type id and its H1 is the schema's, which is exactly why
///   `jigc rename` calls a reslug **undefined** there rather than merely blocked. It is
///   the same predicate the create/author door refuses on (`write.title-ignored`), so the
///   two doors cannot disagree about which doctypes own their title;
/// * a **transient sink** (no `location`, no `placement`, no doc-level `id-from` — today
///   `commit`): it never lands as a repo file, and its slug IS the task id, so moving it
///   would sever the doc from the task whose finalize renders it.
///
/// Both routes are `Human`: the correction is a schema edit or a different verb, never a
/// command this doc's agent can re-run.
fn fixed_identity_refusal(schema: &Schema, uri: &str) -> Option<Finding> {
    let ty = &schema.ty;
    let (what, route) = if let Some(fixed) = schema.fixed_title() {
        (
            format!(
                "`{ty}` is a singleton — its slug IS the type id and its `# H1` is supplied \
                 by the schema (`{fixed}`), so it carries no author-owned title or slug"
            ),
            format!(
                "nothing to rename: the name is part of the `{ty}` schema, so a genuinely \
                 wrong one is a pack change, not a write; edit the doc's prose with \
                 `jigc doc set-slot`"
            ),
        )
    } else if schema.location.is_none() {
        (
            format!(
                "`{ty}` is a transient doctype — it never lands as a repo file, and its \
                 slug IS the task id it is rendered for"
            ),
            format!(
                "nothing to rename: a `{ty}` doc is addressed by its task id for the life \
                 of that task; write its content with `jigc doc set-slot` / `jigc doc \
                 set-field`"
            ),
        )
    } else {
        return None;
    };
    Some(Finding::graded(
        Severity::Blocking,
        "write.identity-change",
        format!("rename rejected: {what}"),
        Some(Location::addressed(uri, 1, 1)),
        Some(route.into()),
    ))
}

/// The **committed-identity refusal**: the addressed doc was copied in from the committed
/// store, so its path is its identity everywhere outside this task — referrers in other
/// docs, other clones' history, `git log --follow`. Moving it is `jigc rename`'s job, and
/// the route is that verb, argv-complete (the `--slug` override carried through when the
/// agent supplied one), with the precondition stated: `rename` is task-less and
/// self-committing, so it runs once this task is out of flight.
fn committed_reslug_refusal(
    uri: &str,
    new_slug: &str,
    to: &str,
    slug_override: Option<&str>,
) -> Finding {
    let mut argv = vec![
        "jigc".to_string(),
        "rename".to_string(),
        uri.to_string(),
        "--to".to_string(),
        crate::task::shell_token(to),
    ];
    if let Some(slug) = slug_override {
        argv.push("--slug".to_string());
        argv.push(slug.to_string());
    }
    Finding::graded(
        Severity::Blocking,
        "write.identity-change",
        format!(
            "rename rejected: `{uri}` is committed, so `--to {to:?}` would move its \
             identity to `{new_slug}` — a committed doc's path IS its identity, and \
             referrers outside this task point at the old one. A same-slug retitle of \
             the staged copy is supported; a re-slug is not"
        ),
        Some(Location::addressed(uri, 1, 1)),
        Some(engine::finding::Route::mechanical(
            argv,
            " moves it for real — repointing every committed referrer in one \
                 transaction — once this task is finalized or discarded (it is a \
                 task-less, self-committing store op)",
        )),
    )
}

/// **The destination-occupancy guard** — the re-slug's *"is this identity free?"*, asked
/// over **both homes a doc lives in**, before a byte is written. Returns the staged path
/// the move may claim; blocks with `write.already-present` keyed at the destination
/// identity otherwise.
///
/// A re-slug that lands on an identity something already answers to is a collision in
/// either home, and each home loses different bytes:
///
/// * **staged** — another doc in this task's own working area. The move would overwrite
///   it, discarding that doc's authored content **now**;
/// * **committed** — an instance in the store. Nothing is lost at the write, so this arm
///   read as harmless and shipped staged-only: the rename **acked success at exit 0**,
///   `task validate` stayed clean, and the state was refused three commands later by
///   `finalize.promote-clobber`. That is a write ack over a state the task cannot
///   complete, and a `task validate` that did not preview what `finalize` gates on
///   (`design/surface-contract.md` → law 1; M47 Increment 4). Refusing here makes the
///   two doors agree by making the state unreachable, rather than by teaching a third
///   surface to describe it.
///
/// The committed arm is [`state::create_incumbent`], **not** a second committed-existence
/// check of its own — the same predicate `doc create` / `doc author` adjudicate their
/// title pre-check with. Reusing it is what buys the parity: its committed arm carries
/// the **in-location-squatter carve-out** (a migration whose recorded `source-path` IS
/// this slug's canonical destination is rewriting that very file in place), which is the
/// carve-out `engine::finalize::plan_clobber_guard` makes at the committing door. A bare
/// committed-existence probe — [`state::bound_instance_present`], the two-home predicate
/// the sibling title pre-check reads — answers the wrong question here: it is about a
/// **role binding**'s document and carries no carve-out, so it would refuse an in-place
/// migration rename that `finalize` lands happily — the same lie pointed the other way.
///
/// `create_incumbent`'s **staged** arm is the same `docs/<type>:<slug>.md` probe as the
/// one above it, so this call reads purely as the committed answer: the staged home has
/// already been adjudicated (and with the write-time barrier applied, which
/// `create_incumbent` does not know about).
fn free_destination(
    task: &ActiveTask,
    schema: &Schema,
    new_slug: &str,
    new_uri: &str,
    old_uri: &str,
    to: &str,
) -> Result<PathBuf, DocFailure> {
    let new_address = parse_addr(new_uri)?;
    let new_path = staged_path(&task.dir, &new_address, &task.id)?;
    if new_path.exists() {
        return Err(DocFailure::block(occupied_destination_refusal(
            new_uri,
            old_uri,
            to,
            format!(
                "this task already stages `{new_uri}` — a rename onto it would discard \
                 that doc's authored content"
            ),
            format!("remove the staged `{new_uri}` first if it was minted by mistake"),
        )));
    }
    // The **second home**. `id_source` is the slug itself and `slug_override` carries it
    // verbatim, so the probed identity is exactly `new_uri` — the title has already minted
    // the slug up in `run_doc_rename`, and re-deriving it here could only disagree.
    let incumbent = state::create_incumbent(
        &task.dir,
        schema,
        schema.ty.as_str(),
        new_slug,
        Some(new_slug),
        &task.jigc_home,
    )
    .map_err(DocFailure::block)?;
    if let Some(committed) = incumbent.incumbent {
        let at = committed
            .strip_prefix(&task.jigc_home)
            .unwrap_or(&committed)
            .display()
            .to_string();
        return Err(DocFailure::block(occupied_destination_refusal(
            new_uri,
            old_uri,
            to,
            format!(
                "the committed store already holds `{new_uri}` at `{at}` — landing this \
                 task's doc under that identity would overwrite it at finalize"
            ),
            format!(
                "if `{new_uri}` is the doc you meant to work on, read it with `jigc doc \
                 show {new_uri}` and edit that one instead of minting a second under its \
                 identity"
            ),
        )));
    }
    Ok(new_path)
}

/// The re-slug's occupancy refusal — one code and one `(code, target)` key for both
/// homes ([`free_destination`]), keyed at the **destination identity** the write refused
/// to claim. `what` states the home; `alternative` states that home's own second exit.
///
/// The route leads with the exit that is this agent's own command back, one flag longer:
/// a re-slug is *chosen*, not derived, so an explicit `--slug` corrects the title without
/// moving onto an occupied id. It is a **`Human`** route, like the committing door's
/// `finalize.promote-clobber` sibling it previews: the free slug is the agent's to pick,
/// and a mechanical argv may only carry a declared placeholder
/// (`engine::finding::ROUTE_PLACEHOLDERS`), which a slug is not.
fn occupied_destination_refusal(
    new_uri: &str,
    old_uri: &str,
    to: &str,
    what: String,
    alternative: String,
) -> Finding {
    let to = crate::task::shell_token(to);
    Finding::graded(
        Severity::Blocking,
        "write.already-present",
        format!("rename rejected: {what}"),
        Some(Location::addressed(new_uri, 1, 1)),
        Some(
            format!(
                "give this doc an id nothing else answers to — re-run `jigc doc rename \
                 {old_uri} --to {to} --slug <other-slug>`; or {alternative}"
            )
            .into(),
        ),
    )
}

/// Move a **never-committed** staged doc's identity within the task working area: the
/// staged body, and every in-task reference to it. The set is **derived, never hand
/// listed** — the whole point of confining the re-slug to the uncommitted case is that
/// the CLI owns every referrer there:
///
/// 1. the staged `docs/<type>:<slug>.md` body (through the same write-time barrier the
///    other verbs stage through, so a `--slug` can no more escape the area than an
///    address can);
/// 2. the `docs/provenance.json` entry, re-keyed with its recorded provenance intact
///    (the join classifies by it, so losing it would change the clash rule);
/// 3. every `roles.json` binding pointing at the old address;
/// 4. every **staged doc's** `ref` fields — including the task's own transient `commit`
///    doc, which `jigc rename` can only *report* on because there it is not a file, and
///    which here is simply another doc in the area;
/// 5. a migration task's recorded `slug-override`, when it held the old slug — the id the
///    author path re-mints from, which would otherwise re-create the doc at the old id.
///
/// The destination must be **free in both homes** before any of it runs — see
/// [`free_destination`], which is where that adjudication lives and which hands this
/// function the `new_path` it moves to.
fn move_staged_identity(
    task: &ActiveTask,
    address: &Address,
    old_path: &Path,
    new_path: &Path,
    new_slug: &str,
    new_uri: &str,
) -> Result<(), DocFailure> {
    let old_uri = address.to_string();
    std::fs::rename(old_path, new_path)
        .with_context(|| format!("could not move the staged `{old_uri}` to `{new_uri}`"))?;

    // 2. the provenance manifest — re-keyed, the recorded value preserved.
    let mut provenance = state::ProvenanceRecord::load(&task.dir)
        .context("could not read the task's staged-doc provenance")?;
    if let Some(recorded) = provenance.docs.remove(&old_uri) {
        provenance.docs.insert(new_uri.to_string(), recorded);
        state::persist(
            &state::ProvenanceRecord::path_in(&task.dir),
            provenance.to_bytes().as_bytes(),
        )
        .context("could not record the renamed doc's provenance")?;
    }

    // 3. the bound context roles.
    let mut roles =
        state::RolesRecord::load(&task.dir).context("could not read the bound roles")?;
    let rebound: Vec<String> = roles
        .roles
        .iter()
        .filter(|(_, bound)| bound.as_str() == old_uri)
        .map(|(role, _)| role.clone())
        .collect();
    if !rebound.is_empty() {
        for role in rebound {
            roles.bind(role, new_uri.to_string());
        }
        roles
            .save(&task.dir)
            .context("could not re-bind the renamed doc's context roles")?;
    }

    // 4. every staged doc's `ref` fields (the renamed doc included — a self-reference is
    //    still a reference). `repoint_ref` answers `NotPresent` for a relation the doc
    //    does not carry the old id in, which is the no-op case, not a failure.
    let schemas = task.schemas()?;
    for (staged_path, staged_uri) in staged_instances(&task.dir)? {
        let Some(doc_schema) = staged_uri
            .split_once(':')
            .and_then(|(ty, _)| schemas.get(ty))
        else {
            continue;
        };
        let mut source = std::fs::read_to_string(&staged_path)
            .with_context(|| format!("could not read the staged `{staged_uri}`"))?;
        let mut touched = false;
        for relation in ref_relations(doc_schema) {
            if let Ok(edited) =
                engine::write::repoint_ref(doc_schema, &source, &relation, &old_uri, new_uri)
            {
                source = edited;
                touched = true;
            }
        }
        if touched {
            persist(&staged_path, &source)?;
        }
    }

    // 5. a migration task's recorded slug override, when it named the old id.
    let override_path = task.dir.join(state::SLUG_OVERRIDE_FILE);
    if state::read_slug_override(&task.dir)
        .context("could not read the task's migration slug override")?
        .as_deref()
        == Some(address.slug.as_str())
    {
        state::persist(&override_path, new_slug.as_bytes())
            .context("could not move the task's migration slug override")?;
    }
    Ok(())
}

/// Every staged instance in a task's `docs/` area as `(path, <type>:<slug>)`, sorted —
/// the working-area walk `finalize`'s owner-artifact scan performs, kept local because
/// this one needs the address, not the parsed document.
fn staged_instances(task_dir: &Path) -> Result<Vec<(PathBuf, String)>> {
    let docs = state::instance_path(task_dir, "_", "_")
        .parent()
        .expect("a staged instance path has a `docs/` parent")
        .to_path_buf();
    let entries = match std::fs::read_dir(&docs) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err).with_context(|| format!("reading staged docs in {docs:?}")),
    };
    let mut out: Vec<(PathBuf, String)> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|x| x.to_str()) == Some("md"))
        .filter_map(|path| {
            let stem = path.file_stem()?.to_str()?.to_string();
            stem.contains(':').then_some((path, stem))
        })
        .collect();
    out.sort();
    Ok(out)
}

/// The ids of a schema's doc-level `ref` fields — the relations [`engine::write::repoint_ref`]
/// can rewrite (it locates a `ref` on a *simple* section; no shipped doctype declares a
/// `ref` inside a repeatable item block, the same bound `jigc rename` carries).
fn ref_relations(schema: &Schema) -> Vec<String> {
    schema
        .sections
        .iter()
        .filter_map(|section| match &section.body {
            SectionBody::Simple { fields, .. } => Some(fields),
            _ => None,
        })
        .flatten()
        .filter(|field| field.ty == FieldType::Ref)
        .map(|field| field.id.clone())
        .collect()
}

/// **The title pre-check** — the one seam `jigc doc create` and `jigc doc author` share,
/// closing the write path's *"success over a title that never landed"* class
/// (`design/write-commands.md` → The four-way write over a committed doc, the **fourth**
/// member; `DECISIONS.md` → 2026-08-13 the Settle, F2). It runs **before either verb
/// persists anything** — `run_create` calls it ahead of `create_gated`, `run_author`
/// ahead of the same call, so the whole payload is rejected with nothing staged and no
/// rollback to perform.
///
/// Two shapes, and they are **not** the same defect:
///
/// * **identity divergence** — the workflow gate entry's `as:` role is already bound to
///   `<type>:<slug>` **that is actually there** ([`state::bound_instance_present`] — staged
///   in the working area, or committed in the store) and this call would mint or re-point a
///   *different* identity. That is an identity change made through a verb that only mints,
///   so it converges on the shipped **`write.identity-change`** (its other producers:
///   `retitle-item` under an enum `id-from`, and `doc rename`'s two refusals). A binding
///   whose document is in neither home is **stale, not an incumbent**: refusing on it says
///   the task holds a doc it does not (a `design/surface-contract.md` law-1 lie) and routes
///   at a `jigc doc rename` that cannot run.
/// * **the silent no-op** — the call lands on the identity the task already holds (or on
///   a committed doc it would copy in), so the create hands that body back **as found**
///   and the supplied title is never written. This is *not* an identity change:
///   `design/storage.md` → Identity calls an ordinary retitle identity-**stable**, so
///   firing `write.identity-change` here would be a law-1 lie. It earns its own member,
///   **`write.title-ignored`** — *the title you supplied will not become this doc's
///   `# H1`* — true of exactly the cells it fires on, and keyed at a different target
///   than the divergence shape.
///
/// A **fixed-title doctype** ([`Schema::fixed_title`] — a `placement` / `display-title`
/// singleton) is the no-op shape one rank higher: its `# H1` is the schema's, so a
/// divergent `--title` is dropped whatever the corpus holds. It is adjudicated from the
/// **doctype alone**, before any instance is read, and then returns — the two instance
/// arms below are inert for it (its slug is fixed, so it cannot diverge) and arm 3 would
/// route at `jigc doc rename`, which refuses a singleton outright (a dead end).
///
/// **Ranking:** doctype admission (unknown / gate-blocked, [`state::create_admission`])
/// outranks all of it — a title complaint about a doctype this workflow cannot create is
/// a misdirection — and the payload-shape parse outranks that, unchanged.
fn title_pre_check(
    task: &ActiveTask,
    schema: &Schema,
    entry: &engine::compose::AllowsCreate,
    verb: &str,
    title: &str,
    slug_override: Option<&str>,
) -> Result<(), DocFailure> {
    let ty = schema.ty.as_str();
    // Rank 1 — the doctype-wide refusal: a singleton's `# H1` is the schema's own.
    if let Some(fixed) = schema.fixed_title() {
        if title != fixed {
            return Err(DocFailure::block(fixed_title_refusal(
                task, ty, verb, title, &fixed,
            )));
        }
        return Ok(());
    }

    let incumbent =
        state::create_incumbent(&task.dir, schema, ty, title, slug_override, &task.jigc_home)
            .map_err(|f| block(&f, verb, ty))?;

    // Rank 2 — identity divergence. The bound role is the identity this task **holds**;
    // a call that would mint another one is a second document, not a correction. A
    // binding naming a different doctype is another entry's role and is not ours to read.
    //
    // **"Holds" is probed, never assumed** ([`state::bound_instance_present`]): `roles.json`
    // is a record, and a binding outlives its document whenever a write that bound the role
    // rolls its staged file back — `create_gated` binds *before* `run_author` applies the
    // payload leaves, so one bad leaf leaves the role pointing at nothing. Reading that
    // orphan as an incumbent refused the retry with a sentence naming a doc that is not
    // there and a `jigc doc rename` route that could not run. A binding whose document is
    // in neither home is stale, so the mint proceeds and re-points it.
    if !entry.as_role.is_empty() {
        let roles =
            state::RolesRecord::load(&task.dir).context("could not read the task's bound roles")?;
        if let Some(bound) = roles.get(&entry.as_role)
            && bound.starts_with(&format!("{ty}:"))
            && bound != incumbent.address
            && state::bound_instance_present(&task.dir, schema, bound, &task.jigc_home)
        {
            return Err(DocFailure::block(identity_divergence_refusal(
                task,
                verb,
                bound,
                &incumbent.address,
                &entry.as_role,
                title,
                slug_override,
            )));
        }
    }

    // Rank 3 — the silent no-op: an incumbent body means the create writes no title.
    if let Some(path) = &incumbent.incumbent {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("could not read the existing `{}`", incumbent.address))?;
        // Compare the **title text**, not the bytes around it. The guard's claim is that
        // the supplied title will not become the doc's `# H1`, so its two sides must be
        // exactly what the mint would render (`engine::write::render` — `# {title.trim()}`)
        // and what the incumbent's H1 already says ([`crate::rename::read_h1`] — BOM- and
        // EOL-tolerant, because humans own these bytes). Otherwise a supported checkout
        // shape (a CRLF clone) or the tool's own trimming refuses a byte-identical title
        // and prints the same string on both sides of the refusal — a `design/surface-
        // contract.md` law-1 self-contradiction.
        if let Some(current) = crate::rename::read_h1(&source)
            && current.trim() != title.trim()
        {
            let staged = path.starts_with(&task.dir);
            return Err(DocFailure::block(title_ignored_refusal(
                task,
                verb,
                &incumbent.address,
                current.trim(),
                title,
                staged,
            )));
        }
    }
    Ok(())
}

/// The **fixed-title refusal**: a `placement` / `display-title` singleton carries the
/// schema's own `# H1`, so a divergent supplied title is dropped on the floor. The route
/// differs by verb because the correction does: `create` takes its title on the command
/// line, so the fix is one runnable command; `author` takes it from a payload file, so a
/// mechanical route would promise a re-run that fixes nothing.
fn fixed_title_refusal(
    task: &ActiveTask,
    ty: &str,
    verb: &str,
    title: &str,
    fixed: &str,
) -> Finding {
    let uri = format!("{ty}:{ty}");
    let route = if verb == "create" {
        engine::finding::Route::mechanical(
            [
                "jigc",
                "doc",
                "create",
                ty,
                "--title",
                &crate::task::shell_token(fixed),
                "--task",
                &task.id,
            ],
            " mints it under the title the schema fixes; a genuinely wrong name there is \
             a pack change, not a write",
        )
    } else {
        engine::finding::Route::human(format!(
            "set the payload's `title:` to `{fixed}` (or drop the line — a `{ty}` payload \
             defaults to the title its schema fixes) and re-run the same `jigc doc author \
             {ty} --from-file <payload> --task {}`",
            task.id,
        ))
    };
    Finding::graded(
        Severity::Blocking,
        "write.title-ignored",
        format!(
            "{verb} rejected: `{ty}` is a singleton — its `# H1` is supplied by the schema \
             (`{fixed}`), never by the author, so `{title}` would be dropped silently and \
             the doc would still read `# {fixed}`"
        ),
        Some(Location::addressed(&uri, 1, 1)),
        Some(route),
    )
}

/// The **identity-divergence refusal**: this task's `<role>` already names a doc, and the
/// call would mint a different one beside it rather than correct it. The route is the
/// in-task title change (M48's `jigc doc rename`), argv-complete — carrying the `--slug`
/// through when the agent supplied one, so the correction that runs is the one asked for.
///
/// The tail states the **declared behaviour change** rather than ambushing with it: every
/// shipped workflow is one doc per role by its own prose, so a hand-driven second mint is
/// what stops working here, and the message says where a genuinely separate second
/// document goes.
fn identity_divergence_refusal(
    task: &ActiveTask,
    verb: &str,
    bound: &str,
    minted: &str,
    role: &str,
    title: &str,
    slug_override: Option<&str>,
) -> Finding {
    let mut argv = vec![
        "jigc".to_string(),
        "doc".to_string(),
        "rename".to_string(),
        bound.to_string(),
        "--to".to_string(),
        crate::task::shell_token(title),
    ];
    if let Some(slug) = slug_override {
        argv.push("--slug".to_string());
        argv.push(slug.to_string());
    }
    argv.push("--task".to_string());
    argv.push(task.id.clone());
    Finding::graded(
        Severity::Blocking,
        "write.identity-change",
        format!(
            "{verb} rejected: this task's `{role}` is already `{bound}`, and this call \
             would mint `{minted}` instead — a second document beside the first, not a \
             correction of it"
        ),
        Some(Location::addressed(bound, 1, 1)),
        Some(engine::finding::Route::mechanical(
            argv,
            " moves the doc this task already holds onto the title (and id) you asked \
             for; a genuinely separate second document is its own task — finalize or \
             discard this one first",
        )),
    )
}

/// The **silent-no-op refusal**: the call lands on an identity that already has a body,
/// so the create hands that body back and the supplied title is never written. Not an
/// identity change (the id does not move — `design/storage.md` → Identity), hence its own
/// code; the route is the verb that *does* move a staged doc's title, after which this
/// very write re-runs unchanged.
fn title_ignored_refusal(
    task: &ActiveTask,
    verb: &str,
    address: &str,
    current: &str,
    title: &str,
    staged: bool,
) -> Finding {
    let held = if staged {
        "is already staged in this task"
    } else {
        "is already committed and would be copied in for update"
    };
    Finding::graded(
        Severity::Blocking,
        "write.title-ignored",
        format!(
            "{verb} rejected: `{address}` {held} as `# {current}`, and this call mints the \
             same id — so `{title}` would never become its `# H1` and the write would ack \
             a retitle that did not happen"
        ),
        Some(Location::addressed(address, 1, 1)),
        Some(engine::finding::Route::mechanical(
            [
                "jigc",
                "doc",
                "rename",
                address,
                "--to",
                &crate::task::shell_token(title),
                "--task",
                &task.id,
            ],
            " retitles the doc in place; then re-run this write unchanged",
        )),
    )
}

/// The `--slug` grammar reject the three `jigc doc` mint doors share — `create`,
/// `rename` and `add-item`. An override **drives the minted id verbatim** and is
/// therefore never silently re-slugified: a value that is not a well-formed slug is
/// refused **before** the door resolves a task or reads a byte, so a rejected mint
/// stages nothing (`design/write-commands.md` → `jigc rename`'s `--slug` precedent;
/// `DECISIONS.md` 2026-07-06 M39 planning → Slug (G6)). `None` is inert.
///
/// One function rather than a per-door copy: the three doors state the same grammar,
/// and a fourth spelling of "what a slug is" is exactly the drift the surface contract
/// forbids. (`jigc start` / `jigc migrate` mint **work-unit** ids through their own
/// crates and keep their own copies — the same rule, a different family.)
fn reject_malformed_slug(slug_override: Option<&str>) -> Result<(), DocFailure> {
    if let Some(slug) = slug_override
        && !engine::slug::is_slug(slug)
    {
        return Err(DocFailure::Orchestration(anyhow!(
            "`--slug {slug:?}` is not a valid slug — use lowercase letters, digits, and single hyphens (no leading, trailing, or doubled `-`)"
        )));
    }
    Ok(())
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
    reject_malformed_slug(slug_override)?;
    let task = ActiveTask::resolve(cwd, task_id)?;
    let schemas = task.schemas()?;
    let gate = task.workflow_gate()?;
    // Admission (unknown doctype / the create-gate) first, then the title pre-check —
    // both **before** `create_gated` persists, so a rejected create stages nothing and
    // has nothing to roll back.
    let (schema, entry) = state::create_admission(&schemas, &gate.allows_create, type_name)
        .map_err(|f| block(&f, "create", type_name))?;
    title_pre_check(&task, schema, entry, "create", title, slug_override)?;
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
    let plan = crate::author::parse_author_payload(schemas.get(doctype), &payload)?;
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
    // Admission then the title pre-check — the same seam `run_create` lowers through,
    // reached **before** `create_gated` persists the empty doc, so a rejected payload
    // stages nothing at all (there is no `CreatedDoc` to roll back yet). Ranked below the
    // payload parse above: an unparseable payload is an argument-shape defect.
    let (schema, entry) = state::create_admission(&schemas, &gate.allows_create, doctype)
        .map_err(|f| block(&f, "author", doctype))?;
    title_pre_check(
        &task,
        schema,
        entry,
        "author",
        &plan.title,
        slug_override.as_deref(),
    )?;
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

    // Chain every leaf over the single in-memory buffer, no persist between leaves.
    // Atomicity (`design/auto-migration.md` → Hardening #1): `create_gated` already
    // persisted the empty instance before the chain, so a mid-chain leaf failure must
    // **also** discard that staged file — otherwise an empty doc leaks for a batch that
    // "persisted nothing". Rollback = drop the in-memory buffer + [`CreatedDoc::rollback`],
    // which restores exactly what the create found: it removes the file the create
    // provisioned, and **restores the captured pre-image** when the create merely handed
    // back an already-staged working copy (M45 Inc 5 T2's create-or-update — authoring
    // over a staged doc merges into it, so an unconditional removal here deleted the
    // session's prior work, silently, on the path the reject's own route invites the
    // agent to re-run; M47 Increment 6, the triage fix). The block finding propagates
    // unchanged either way.
    let mut buffer = read_staged(&created.path, &created.address)?;
    // What the rollback above leaves behind, for a reject's route to be adjudicated
    // against ([`RejectDestination`]): `existed` is exactly that question — it is `true`
    // for both branches that find a body already there (a staged copy handed back as
    // found, restored from its pre-image; a committed instance copied in, still at its
    // canonical home for copy-on-first-touch) and `false` for the fresh mint, whose file
    // this call wrote and the rollback removes.
    //
    // And *with which content*: the *pre-chain* buffer, captured here before the first
    // leaf edits it. Over a staged copy that is byte-for-byte the pre-image
    // [`CreatedDoc::rollback`] restores (the create wrote nothing); over a committed
    // copy-in it is the committed body, which the rollback leaves at its canonical home
    // untouched. Either way it is the doc an agent finds after the reject — which is not
    // the in-progress `buffer`, and the difference is exactly one payload's own items.
    let surviving = buffer.clone();
    let destination = if created.existed {
        RejectDestination::Reachable {
            surviving: &surviving,
        }
    } else {
        RejectDestination::Discarded
    };
    for leaf in &plan.leaves {
        match apply_leaf(
            schema,
            &buffer,
            &created.address,
            leaf,
            &task.id,
            migration,
            destination,
        ) {
            Ok(edited) => buffer = edited,
            Err(failure) => {
                created.rollback();
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

/// Apply one lowered batch [`Leaf`](crate::author::Leaf) over the in-memory `source`,
/// returning the edited buffer. The leaf's `fragment` is the address tail relative to
/// the created instance; prepending `head` (`<doctype>:<slug>`) reconstitutes the full
/// address the existing per-leaf target resolvers (`field_target` / `slot_target` /
/// `add_item_target`) accept verbatim — so the batch reuses the same resolution +
/// splice primitives the per-leaf verbs do (the shared `apply_*_target` helpers).
fn apply_leaf(
    schema: &Schema,
    source: &str,
    head: &str,
    leaf: &crate::author::Leaf,
    task_id: &str,
    migration: bool,
    destination: RejectDestination<'_>,
) -> Result<String, DocFailure> {
    use crate::author::Leaf;
    match leaf {
        Leaf::AddItem { fragment, title } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = add_item_target(&address)
                .with_context(|| format!("no section addressed by `{addr}`"))?;
            // No `slug_override`: the batch payload grammar carries no `slug:` key —
            // `--slug` is the per-leaf `add-item` flag, and a payload collision is
            // routed at it rather than silently overridden here.
            let (edited, _minted) = apply_add_item_target(
                schema,
                source,
                target,
                &addr,
                title,
                None,
                task_id,
                migration,
                destination,
            )?;
            Ok(edited)
        }
        Leaf::SetField { fragment, value } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = field_target(schema, &address)?;
            apply_field_target(schema, source, target, &addr, value)
        }
        Leaf::SetSlot { fragment, prose } => {
            let addr = format!("{head}#{fragment}");
            let address = parse_addr(&addr)?;
            let target = slot_target(schema, &address)?;
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
    // The project layer is located **before** the address is parsed: a bare singleton
    // head expands only for a doctype the *resolved* cascade homes at a literal file, so
    // the address grammar cannot be adjudicated without the layer that decides it. The
    // one visible consequence is ordering — a malformed address in a repo that was never
    // set up now reports the setup gate first, which is the fault the caller must fix
    // first anyway.
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let address = parse_verb_addr(pack.as_ref(), &jigc_home.join(".jigc").join("config"), addr)?;
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
///
/// **Phrased for a reader who may *be* the staging task** (M47 Inc 10 / T5 — D5;
/// `design/surface-contract.md` → the style guide). A task-less read carries no task
/// id, so the hint cannot know whose task it names — and the commonest case in the
/// field is that the only stager is the reader's own open task, which the old
/// *"the committed copy served here may be stale"* framing read as a third-party
/// warning about someone else's edit. The wording is true in both readings: it says
/// which copy this read served, that edits staged in the named task are not in it
/// (**"any edits"** — the check is existence-only, so a just-copied-in, still
/// identical staged copy is not asserted to differ), and it hands the reader the
/// staged read under an explicit *if that task is yours* clause.
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
    let (plural, whose) = if staged_in.len() == 1 {
        ("", "if that task is yours")
    } else {
        ("s", "if one of them is yours")
    };
    eprintln!(
        "note: `{doc}` is also staged in open task{plural} {} — this read served the committed \
         copy, so any edits staged there are not shown; {whose}, read your staged work: {}",
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
    let address = parse_verb_addr(task.pack.as_ref(), &task.project_config(), addr)?;
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
    if !engine::validate::is_unadopted_foreign(
        ty,
        address.slug.as_str(),
        schema,
        &source,
        &versions,
        &priors,
    ) {
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
    crate::start::resolved_schemas(pack, &jigc_home.join(".jigc").join("config"))
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
        return Err(unknown_doctype_block(doctype));
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
///
/// **`--task <id>` lists the task's staged surface instead** ([`run_list_staged`]) — the
/// index read's counterpart to `doc show --task`, always an explicit id, never inferred.
fn run_list(
    cwd: &Path,
    doctype: Option<&str>,
    task_id: Option<&str>,
    format: Format,
) -> Result<(), DocFailure> {
    if let Some(task_id) = task_id {
        return run_list_staged(cwd, doctype, task_id, format);
    }
    let jigc_home = crate::ingest::require_project_layer(cwd)?;
    let pack = make_pack()?;
    let schemas = committed_schemas(pack.as_ref(), &jigc_home)?;
    if let Some(ty) = doctype
        && !schemas.contains_key(ty)
    {
        return Err(unknown_doctype_block(ty));
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
            // The `<slug>` half of the identity the enumerator derived — the same
            // identity the row prints, so the state answers about the row.
            let slug = id.split_once(':').map_or("", |(_, slug)| slug);
            let state = if engine::validate::is_unadopted_foreign(
                ty, slug, schema, &source, &versions, &priors,
            ) {
                "unregistered"
            } else {
                "managed"
            };
            // Best-effort item count: parse against the current schema and sum top-level
            // repeatable items; a foreign/unregistered or stale-shape instance that does not
            // parse counts 0 (never a block — `doc list` is a report, and the row already
            // carries `state` to tell an agent the file is not adopted).
            let item_count = engine::parse::parse_sections(schema, &source)
                .map(|doc| item_count(&doc))
                .unwrap_or(0);
            docs.push(DocRow {
                id,
                path: path
                    .strip_prefix(&jigc_home)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned(),
                state,
                item_count,
            });
        }
    }
    // The empty-set line (M43 Inc 7 / T5; `design/surface-contract.md` → the style
    // guide) — zero rows print a stated empty set, never zero bytes (the `task list`
    // empty-roster mold), naming the filtered doctype when one scoped the listing.
    // Exit 0: an empty store is a legitimate state, not an error.
    let empty_line = match doctype {
        Some(ty) => format!("jigc doc list — no committed `{ty}` docs"),
        None => "jigc doc list — no committed docs".to_string(),
    };
    render_listing(format, &docs, &empty_line);
    staged_listing_hint(&jigc_home, doctype);
    Ok(())
}

/// The **staged arm** of `jigc doc list` — `--task <id>` lists what that task stages
/// (`design/doc-read-surface.md` → the fourth read surface, the staged arm). The index
/// read's counterpart to [`run_show_staged`]: same row shape, same doctype narrowing,
/// and **staged-only** — a committed doc the task has not copied in is not in it, the
/// same non-merging the staged content read makes (`store::read_slice_staged` blocks
/// rather than falling back). The task resolves via [`ActiveTask::resolve`], so a bad
/// id gets the shared `jigc task list` route.
///
/// Three row facts the staged arm settles, each reusing a shipped rule rather than
/// minting one:
///
/// - **`state` is `managed` on every row.** A staged working copy is jigc-written by
///   construction, never a foreign squatter — the same reason the staged content read
///   runs no [`reroute_unadopted`]. The managed-vs-foreign discriminator adjudicates
///   the *committed* store, and is not consulted here.
/// - **`path` is where the instance promotes to at finalize** ([`engine::store::canonical_path`],
///   the one owner of the committed layout, rooted at the empty path so the placement
///   branch is included) — the M43 A14 staged display rule: a printed path is repo-real
///   or a typed identity, never the working-area fiction. A **transient** doctype
///   (`commit:<task-id>`, sink = the git message) and a type the resolved cascade no
///   longer defines have no committed home, so they list at their `<type>:<slug>`
///   identity — the second of law 1's two legal forms.
/// - **`item-count` is the same best-effort parse** the committed arm makes; an
///   unknown-type instance has no schema to parse against and counts 0.
fn run_list_staged(
    cwd: &Path,
    doctype: Option<&str>,
    task_id: &str,
    format: Format,
) -> Result<(), DocFailure> {
    let task = ActiveTask::resolve(cwd, Some(task_id))?;
    let schemas = committed_schemas(task.pack.as_ref(), &task.jigc_home)?;
    if let Some(ty) = doctype
        && !schemas.contains_key(ty)
    {
        return Err(unknown_doctype_block(ty));
    }
    // The staged `docs/<type>:<slug>.md` set as sorted `<type>:<slug>` identities — the
    // CLI's one staged-doc enumerator, shared with `task discard`'s dropped-doc ack and
    // the `uninstall` prose guard. Filename-sorted ⇒ (type, slug)-sorted, matching the
    // committed arm's order by construction.
    let staged = crate::task::staged_doc_ids(&task.dir.join("docs"))
        .with_context(|| format!("listing the docs staged in task `{}`", task.id))?;
    let mut docs = Vec::new();
    for id in staged {
        let Some((ty, slug)) = id.split_once(':') else {
            continue; // not the `<type>:<slug>` layout: nothing this projection can key on.
        };
        if doctype.is_some_and(|want| want != ty) {
            continue;
        }
        let schema = schemas.get(ty);
        let path = schema
            .and_then(|schema| engine::store::canonical_path(Path::new(""), schema, slug))
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| id.clone());
        let mut count = 0;
        if let Some(schema) = schema {
            let file = state::instance_path(&task.dir, ty, slug);
            let bytes = std::fs::read(&file)
                .with_context(|| format!("reading the staged doc at {file:?}"))?;
            let source = String::from_utf8_lossy(&bytes);
            count = engine::parse::parse_sections(schema, &source)
                .map(|doc| item_count(&doc))
                .unwrap_or(0);
        }
        docs.push(DocRow {
            id,
            path,
            state: "managed",
            item_count: count,
        });
    }
    let empty_line = match doctype {
        Some(ty) => format!("jigc doc list — no `{ty}` docs staged in task {}", task.id),
        None => format!("jigc doc list — no docs staged in task {}", task.id),
    };
    render_listing(format, &docs, &empty_line);
    Ok(())
}

/// Render one listing — the **shared** renderer both arms print through, so the committed
/// and staged surfaces can never grow two column vocabularies (the same one-primitive rule
/// the enumeration side already obeys). Only the empty-set line differs, and the caller
/// supplies it because only the caller knows what set was empty.
///
/// The column header (M47 Inc 10 / T5 — P4-5/C3; `design/surface-contract.md` → law 2:
/// nothing hides): three bare columns left the reader to infer what the third one meant —
/// `managed`/`unregistered` reads as a state only once something says `state`. It names the
/// columns in row order, in the pinned json's own key spelling, so the plain arm teaches the
/// machine arm's vocabulary. **Row-gated**: it prints where rows do, never over the empty-set
/// line (a header above nothing names nothing), and never on `--format json`, whose keys
/// *are* the shape.
fn render_listing(format: Format, docs: &[DocRow], empty_line: &str) {
    match format {
        Format::Json => println!("{}", render::json(&DocListing { docs })),
        Format::Agent | Format::Human => {
            if docs.is_empty() {
                println!("{empty_line}");
            } else {
                println!("id  path  state");
            }
            for row in docs {
                println!("{}  {}  {}", row.id, row.path, row.state);
            }
        }
    }
}

/// The shared `store.unknown-type` block both `doc list` arms raise — the same routed
/// refusal the read-side `doc show` / `doc schema` raise, so a doctype the cascade does
/// not define gets one answer whichever surface asked.
fn unknown_doctype_block(doctype: &str) -> DocFailure {
    DocFailure::block(engine::store::unknown_doctype(doctype))
}

/// The **staged-listing hint** — the index read's counterpart to [`stale_read_hint`]
/// (`design/doc-read-surface.md` → the fourth read surface, the staged arm;
/// `design/surface-contract.md` → the route floor). A task-less `doc list` answers *"which
/// docs are committed"*, which is a different question from *"which docs does my task
/// hold"* — and six consecutive trials went to the filesystem to read their own in-flight
/// work rather than ask the second one. So when an open task stages docs, the listing says
/// which surface it just served and hands over the staged listing.
///
/// **Stdout is untouched**: the note rides **stderr**, so the pinned json and the plain
/// listing — empty-set line included — are byte-identical with and without an open task.
/// Existence check only, over [`state::list_active_task_ids`] (the single task enumeration
/// source) + the CLI's one staged-doc enumerator: no doc is parsed and no content is read,
/// so the note claims only that the task stages *something*.
///
/// **The reader's scope survives into the route** — a `doc list <doctype>` routes at the
/// same doctype's staged listing, never a wider one — and the sentence is phrased for a
/// reader who may *be* the staging task (the M47 D5 revision applied to its sibling): a
/// task-less listing carries no task id, so it says *if that task is yours* rather than
/// warning about someone else's edit. With more than one staging task the ids are listed
/// and the command carries the shared `<task-id>` placeholder.
fn staged_listing_hint(jigc_home: &Path, doctype: Option<&str>) {
    let jigc_root = jigc_home.join(".jigc");
    let tasks = jigc_root.join("tasks");
    let staging: Vec<String> = state::list_active_task_ids(&jigc_root)
        .into_iter()
        .filter(|id| {
            crate::task::staged_doc_ids(&tasks.join(id).join("docs"))
                .is_ok_and(|staged| !staged.is_empty())
        })
        .collect();
    let task_arg = match staging.as_slice() {
        [] => return,
        [id] => id.as_str(),
        _ => "<task-id>",
    };
    let (plural, whose) = if staging.len() == 1 {
        ("", "if that task is yours")
    } else {
        ("s", "if one of them is yours")
    };
    let mut argv = vec!["jigc", "doc", "list"];
    if let Some(ty) = doctype {
        argv.push(ty);
    }
    argv.extend(["--task", task_arg]);
    eprintln!(
        "note: docs are also staged in open task{plural} {} — this listing is the committed \
         store; {whose}, list what it stages: {}",
        staging.join(", "),
        engine::finding::Route::mechanical(argv, ""),
    );
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
    /// The additive **`item-count`** key (M44 — `design/doc-read-surface.md` → the item-count
    /// additive key): the parsed count of the doc's top-level repeatable items ([`item_count`]).
    /// `doc list` did not parse instances before this — the count is a new parse, and it is
    /// **best-effort**: an instance that does not parse against its current schema (a foreign /
    /// `unregistered` file, or a stale-shape managed doc) counts **0**.
    #[serde(rename = "item-count")]
    item_count: usize,
}

/// The `jigc doc schema --format json` shape — the **separately-pinned, explicitly
/// versioned** contract (`design/doc-read-surface.md` → Why json is a contract
/// here). The keys/structure are the pin (`contract-version` bumps on any
/// structural change to this projection); the *values* track the resolved schemas
/// as they evolve. Golden-pinned at ship (`crates/cli/tests/doc_schema.rs`).
#[derive(serde::Serialize)]
struct SchemaContract<'a> {
    /// The projection's own version — 4 since M45 (the three settability states: an
    /// id-from leaf's `add-item`/`retitle-item` pair, a `set: on-create` stamp's
    /// `set-field`; 3 was the M43 rc.7 write-verb address join, 2 the M41 rc.5
    /// `of`/`section` join; bumps on any structural change to these keys).
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
/// default?, set?, section?, set-field? | (add-item? + retitle-item?)}`. `required` is
/// presence-in-a-conformant-instance — the author must supply it OR the CLI stamps it
/// (`default:`/`set:`); `author-required` is the shared engine predicate
/// ([`engine::validate::is_author_required`]) — the same authority the create
/// skeleton pre-stamps from, so the mint and this projection can never drift apart
/// (M40 Settle #4). `of` carries an `enum`'s legal members (universal across
/// depths, so an agent reads the legal values without a failed-write probe);
/// `section` names the field's owning simple-section — **top-level only** (an item
/// field carries its section structurally, under `sections[].item`) (M41 rc.5,
/// V3+V6+V11 the `doc author` discoverability cluster).
///
/// **The write-verb address carries the field's settability state** (M45 Inc 3 — the
/// three-state split, `design/doc-read-surface.md` → the settability states):
/// - `set-field` — the field is **directly settable**: a plain author-supplied field,
///   or a `set: on-create` **author-overridable** stamp (the CLI defaults it at mint
///   but the changelog-migration historical-date path overwrites it, so it IS
///   settable). Absent on a **machine-maintained absolute** (`set: schema-version` /
///   `on-transition`), whose value the CLI owns and the write path refuses.
/// - `add-item` + `retitle-item` — the field is the block's **`id-from` leaf** (the
///   item's identity: the heading IS the value). It is supplied at mint by
///   `add-item --title` (the block address, always) and changed afterward by
///   `retitle-item` (the **item** address, one hop shallower than the leaf) — **except**
///   an **enum** id-from, which `retitle-item` refuses unconditionally (a member change
///   is an identity change), so it carries `add-item` **alone**.
///
/// The set-field / (add-item + retitle-item) keys are mutually exclusive by
/// construction (an id-from leaf is never `set-field`-settable), and **all three are
/// absent** when the doctype is machine-maintained whole (`milestone-record`) — the
/// doctype exclusion wins over the per-leaf states.
///
/// **`write-key` names the payload key an id-source leaf's value is supplied under**
/// (M48 Inc 7 — F12; `design/doc-read-surface.md` → the id-source names its write key).
/// An address says which **verb** takes the leaf; on the id-source it did not say which
/// **key** carries the value, and the leaf's own id is the misleading answer — a
/// `changelog` change-group projects `category` while both its verbs take
/// [`ID_SOURCE_WRITE_KEY`], whatever the field is called
/// (`design/write-commands.md` → The argument convention). It rides the id-source state
/// **alone**: a `set-field` entry's payload is that verb's `--value`/`--unset` pair and a
/// slot's is `--from-file <path|->` — neither is one key, and neither is shadowed by a
/// field id, so naming one there would understate the verb rather than teach it.
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
    #[serde(rename = "add-item", skip_serializing_if = "Option::is_none")]
    add_item: Option<String>,
    #[serde(rename = "retitle-item", skip_serializing_if = "Option::is_none")]
    retitle_item: Option<String>,
    #[serde(rename = "write-key", skip_serializing_if = "Option::is_none")]
    write_key: Option<&'static str>,
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
    // The whole-doctype write suppression (M45 Inc 3): a `milestone-record` is
    // machine-maintained in full — [`machine_maintained_guard`] refuses EVERY `jigc
    // doc` write to it (`design/team-ready-state.md` → The record is not writable) — so
    // the projection advertises no write address at all, by a **doctype** exclusion
    // rather than the per-leaf `set:`-kind rule, keeping advertised-set == accepted-set
    // (`design/doc-read-surface.md` → the settability states; the parity caveat). This
    // is the same doctype key the write-path guard uses; a milestone-record carries no
    // prose slot / nested block (every leaf is a CLI-`set:` field), so gating the
    // Option-valued addresses covers every address the record's shape can produce.
    let suppress = schema.ty == crate::milestone::MILESTONE_RECORD_TYPE;
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
                    // source is its H1 title, never a declared header field. It is
                    // directly settable unless the doctype is machine-maintained whole
                    // or the field is a machine-maintained absolute — a `set: on-create`
                    // stamp IS settable (author-overridable).
                    let address = (!suppress
                        && !engine::schema::is_machine_maintained_absolute(field))
                    .then(|| format!("{doc}#{}/{}", section.id, field.id));
                    let mut field = contract_field(field, address, None, None);
                    field.section = Some(&section.id);
                    field
                }));
                if let Some(slot) = slot {
                    sections.push(ContractSection {
                        id: &section.id,
                        kind: "slot",
                        optional: slot.optional,
                        set_slot: (!suppress).then(|| format!("{doc}#{}", section.id)),
                        add_item: None,
                        item: None,
                    });
                }
            }
            SectionBody::Repeatable { repeatable } => {
                let block_addr = format!("{doc}#{}", section.id);
                sections.push(ContractSection {
                    id: &section.id,
                    kind: "repeatable",
                    optional: false,
                    set_slot: None,
                    add_item: (!suppress).then(|| block_addr.clone()),
                    item: Some(contract_item(repeatable, &block_addr, suppress)),
                });
            }
        }
    }
    SchemaContract {
        contract_version: 5,
        ty: &schema.ty,
        schema_version,
        fields,
        sections,
    }
}

/// The payload key an **id-source** leaf's value is supplied under — at mint by
/// `jigc doc add-item --title`, afterwards by `jigc doc retitle-item --title`. It is
/// **always literally `--title`**, whatever the doctype's `id-from` field is called: the
/// flag names the role (the title being minted from), not the field
/// (`design/write-commands.md` → The argument convention). Exported so the projection's
/// claim can be fenced against the real clap tree rather than re-typed beside it.
pub const ID_SOURCE_WRITE_KEY: &str = "--title";

/// Project one schema field into its pinned [`ContractField`]. The three write-verb
/// addresses are **caller-computed** (the caller knows the field's position, which the
/// settability state keys on; see [`ContractField`]): `set_field` for a directly
/// settable field, the `add_item` + `retitle_item` pair for a block's `id-from` leaf,
/// and all three `None` for a machine-maintained absolute or a machine-maintained-whole
/// doctype. The `write-key` marker is **derived from that same call**, never passed
/// separately: a field carrying `add_item` IS the block's id-source leaf (the only
/// state that sets it), so the key and the address it belongs to cannot drift apart —
/// and a suppressed leaf, advertised under no verb, names no key either.
fn contract_field(
    field: &engine::schema::Field,
    set_field: Option<String>,
    add_item: Option<String>,
    retitle_item: Option<String>,
) -> ContractField<'_> {
    let author_required = engine::validate::is_author_required(field);
    let write_key = add_item.is_some().then_some(ID_SOURCE_WRITE_KEY);
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
        add_item,
        retitle_item,
        write_key,
    }
}

/// Project a repeatable's item template, recursing into nested repeatables.
/// `block_addr` is the block's placeheld `add-item` address
/// (`<type>:<slug>#<section>`, or the nested section-qualified chain); the item's
/// leaf-address prefix is `{block_addr}/<id>` (one `/<id>` per depth). `suppress`
/// propagates the machine-maintained-whole doctype exclusion — when set, no leaf
/// carries a write address.
fn contract_item<'a>(
    repeatable: &'a Repeatable,
    block_addr: &str,
    suppress: bool,
) -> ContractItem<'a> {
    // The item address (`retitle-item`'s target, and each leaf's set-field parent) —
    // one `/<id>` deeper than the block's `add-item` address.
    let prefix = format!("{block_addr}/<id>");
    let mut fields = Vec::new();
    let mut slots = Vec::new();
    let mut nested = Vec::new();
    for leaf in &repeatable.block {
        match leaf {
            Leaf::Field(field) => {
                // The three settability states (M45 Inc 3; `design/doc-read-surface.md`
                // → the settability states):
                let (set_field, add_item, retitle_item) = if suppress {
                    // Machine-maintained whole (milestone-record): no write address.
                    (None, None, None)
                } else if field.id == repeatable.id_from {
                    // The `id-from` leaf — the item's identity (the heading IS the
                    // value). Supplied at mint via `add-item --title` (the block
                    // address, always), changed afterward via `retitle-item` (the
                    // ITEM address, one hop shallower) — EXCEPT an enum id-from, which
                    // `retitle-item` refuses unconditionally (a member change is an
                    // identity change), so it carries `add-item` alone.
                    let retitle = (field.ty != FieldType::Enum).then(|| prefix.clone());
                    (None, Some(block_addr.to_string()), retitle)
                } else if engine::schema::is_machine_maintained_absolute(field) {
                    // A machine-maintained absolute — CLI-owned, unadvertised.
                    (None, None, None)
                } else {
                    // Directly settable, incl. a `set: on-create` author-overridable
                    // stamp (the changelog-migration historical-date path overwrites it).
                    (Some(format!("{prefix}/{}", field.id)), None, None)
                };
                fields.push(contract_field(field, set_field, add_item, retitle_item));
            }
            Leaf::Slot { id, slot } => slots.push(ContractSlot {
                id,
                optional: slot.optional,
                set_slot: format!("{prefix}/{id}"),
            }),
            Leaf::Repeatable { id, repeatable } => {
                let nested_block = format!("{prefix}/{id}");
                nested.push(ContractNested {
                    id,
                    add_item: nested_block.clone(),
                    item: contract_item(repeatable, &nested_block, suppress),
                });
            }
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
///
/// **The `*` legend is a listing-level line, printed iff a `*` actually renders**
/// (M47 Inc 10 / T5 — N16; `design/surface-contract.md` → laws 1 + 2). It used to
/// ride the `fields (…)` header, which got both halves wrong: [`push_field_line`]
/// marks **item leaves under `sections:`** too, so the legend sat away from most of
/// its markers (`spec`'s only `*` is on `criteria/<id>`'s title); and four shipped
/// doctypes — `adr`, `research`, `vision`, `milestone-record` — have fields but
/// **no** author-required leaf anywhere, so the legend announced a convention the
/// listing never used. The predicate is read off the **rendered body**, not
/// recomputed from the contract: `push_field_line` is the only writer of a `" *"`
/// marker and always closes the line, so `" *\n"` in the body is exactly "a marker
/// rendered" — a second walk of the contract could drift from what the reader sees.
fn schema_listing(schema: &Schema, schema_version: Option<u32>) -> String {
    let contract = schema_contract(schema, schema_version);
    let mut out = match contract.schema_version {
        Some(version) => format!("doctype: {} (schema-version {version})\n", contract.ty),
        None => format!("doctype: {}\n", contract.ty),
    };
    let mut body = String::new();
    if !contract.fields.is_empty() {
        body.push_str("fields:\n");
        for field in &contract.fields {
            push_field_line(&mut body, field, 1);
        }
    }
    if !contract.sections.is_empty() {
        body.push_str("sections:\n");
        for section in &contract.sections {
            body.push_str(&format!("  - {}: {}", section.id, section.kind));
            if section.optional {
                body.push_str(" (optional)");
            }
            // The section's write-verb address, mirrored from the pinned json
            // (non-contractual presentation — only the json shape is the pin).
            if let Some(addr) = &section.set_slot {
                body.push_str(&format!(" (set-slot: {addr})"));
            }
            if let Some(addr) = &section.add_item {
                body.push_str(&format!(" (add-item: {addr})"));
            }
            body.push('\n');
            if let Some(item) = &section.item {
                push_item_lines(&mut body, item, 2);
            }
        }
    }
    if body.contains(" *\n") {
        out.push_str("* = author-required\n");
    }
    out.push_str(&body);
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
    // The field's write addresses, mirrored from the pinned json (non-contractual
    // presentation): `set-field` for a directly settable field, else the
    // `add-item` + `retitle-item` pair of an `id-from` leaf (the id-source state); a
    // machine-maintained field's line stays address-less — inert.
    if let Some(addr) = &field.set_field {
        out.push_str(&format!(" (set-field: {addr})"));
    }
    if let Some(addr) = &field.add_item {
        out.push_str(&format!(" (add-item: {addr})"));
    }
    if let Some(addr) = &field.retitle_item {
        out.push_str(&format!(" (retitle-item: {addr})"));
    }
    // The id-source's payload key, mirrored from the pinned json in the json's own key
    // spelling (the plain arm teaches the machine arm's vocabulary rather than
    // inventing a second one) — the field the listing names `category` is written
    // `--title`, and the listing is where a reader meets that id first.
    if let Some(key) = field.write_key {
        out.push_str(&format!(" (write-key: {key})"));
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
///
/// Two additive top-level keys ride beside them, both on **every** whole-doc serve and
/// on **no** fragment slice (a slice is a bare value with no object to hang a key on):
/// [`item_count`], and the doc's own **`schema-version`** stamp as a json **number**
/// ([`stamped_schema_version`]).
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
    let schema_version = stamped_schema_version(&fields);
    serde_json::json!({
        "type": schema.ty,
        "slug": address.slug.as_str(),
        "item-count": item_count(doc),
        "schema-version": schema_version,
        "fields": serde_json::Value::Object(fields),
        "sections": serde_json::Value::Object(sections),
    })
}

/// The doc's **own schema-version stamp** as a json **number** — the additive top-level
/// `schema-version` key on the whole-doc `--format json` serve (`design/doc-read-surface.md`
/// → One name, two json types). It is the doc's **actual** stamp, where `jigc doc schema`'s
/// identically-named integer is the doctype's **expected** version from the freeze manifest:
/// comparing the two is the upgrade check a driver automates (*is this doc's stamp behind
/// its schema?*), and it is the reason this key is a number — before it, that comparison
/// needed a cast, because the only stamp on this surface was the string in `fields`.
///
/// **Read back off the already-built `fields` map on purpose**, not re-walked from the
/// parsed doc: the two keys then cannot disagree by construction, and `fields` stays
/// uniformly stringy — [`field_json`]'s *"a scalar field serializes as its string"* holds
/// for every key in the map, so a driver iterating it is unaffected.
///
/// `None` (serializing to `null`) when the doctype carries **no stamp field** — the
/// transient `commit`, which the pack loader's injection gate skips (`crate::pack` → the
/// stamp injection is `location`/`placement`-gated) — and equally when a stamp on disk is
/// not an integer, which only a hand edit produces and which the `int` field-type check
/// blocks at validate. Never a coerced `0`, and never an absent key: the raw bytes stay
/// readable in `fields` either way.
fn stamped_schema_version(fields: &serde_json::Map<String, serde_json::Value>) -> Option<u64> {
    fields
        .get(engine::schema::SCHEMA_VERSION_FIELD)?
        .as_str()?
        .parse::<u64>()
        .ok()
}

/// The doc's **top-level repeatable item count** — the total number of items summed across
/// every repeatable section (a simple section contributes none; a **nested** item is a member
/// of its enclosing item object and is *not* counted). This is the value of the additive
/// **`item-count`** key on the whole-doc `--format json` serve (riding **both** the committed
/// and staged serves, so `staged` stays the sole staged/committed differentiator — absent on
/// a fragment slice, which is a bare value) and on each `jigc doc list` row
/// (`design/doc-read-surface.md` → the item-count additive key).
fn item_count(doc: &engine::parse::Document) -> usize {
    doc.sections.iter().map(|section| section.items.len()).sum()
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
    // NOT derivable from the heading (it is minted once and frozen; a retitle diverges
    // the two permanently, and the mint-time caps, a `-2` collision suffix, or a
    // slug-rule generation diverge them at mint). Without it the contract is not closed
    // under its own address grammar (M42, `design/doc-read-surface.md` → The item `id`
    // closes the json contract). The key is reserved at schema load (`schema::RESERVED_ITEM_ID_KEY`), so
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
    slug_override: Option<&str>,
) -> render::AckTarget {
    let section = match target {
        AddItemTarget::TopLevel { section } | AddItemTarget::Nested { section, .. } => {
            section.clone()
        }
    };
    ack_target(
        address,
        Some(section),
        Some(minted_item_id(title, slug_override)),
        None,
    )
}

/// The id an `add-item` mints, **CLI-side**: `slug_override` verbatim when supplied,
/// else [`engine::slug::slugify`] of the title — the same derivation the engine's
/// `mint_item_id` runs, kept in ONE place here because three CLI sites re-spell the
/// minted id after the write returns (the acked `target.item`, the acked top-level
/// address, and the acked nested chain). Before the override existed all three read
/// `slugify(title)` directly; an override that moved the anchor but not those three
/// would ack an address the doc does not hold — a write-verb miss the read surface
/// could not answer (`design/command-output-contract.md` §2).
fn minted_item_id(title: &str, slug_override: Option<&str>) -> String {
    match slug_override {
        Some(slug) => slug.to_owned(),
        None => engine::slug::slugify(title),
    }
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

/// The staged bytes an edit verb splices into, plus whether reading them **copied a
/// base-committed doc into the task** — the copy-on-first-touch fact each of the five
/// edit verbs states on its ack (M47 Inc 10 T3; `design/command-output-contract.md` §2 →
/// the first-touch copy-in note). Returned by [`ActiveTask::read_or_copy_in`], the only
/// production caller of `state::copy_in`, so no verb can reach the seam and lose the bit.
struct EditBase {
    source: String,
    copied_in: bool,
}

impl EditBase {
    /// The steady-state read: the instance was already staged (or is absent entirely, in
    /// which case the read itself rejects) — nothing was copied in.
    fn staged(source: String) -> Self {
        EditBase {
            source,
            copied_in: false,
        }
    }

    /// The first touch: the committed body was copied into the working area.
    fn copied_in(source: String) -> Self {
        EditBase {
            source,
            copied_in: true,
        }
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
    /// `<repo>/.jigc/tasks/<id>/` directly, and answers it in **two** ways
    /// (`design/structural-grammar.md` → Work-units and runtime identity, resolution): a
    /// **malformed** id is refused as not-an-id before the join
    /// (`crate::task::reject_malformed_work_unit_id`); a **well-formed** id with no such
    /// area rejects with the shared task-list route (`crate::task::no_such_task`).
    /// Else: the **single** active task
    /// directory under `<repo>/.jigc/tasks/`; **none** rejects with the start-a-task
    /// route, **more than one** with no `--task` rejects asking for the selector.
    fn resolve(cwd: &Path, task_id: Option<&str>) -> Result<Self> {
        // Before anything joins an explicit `--task <id>` onto a path (M50 Inc 1 / T1).
        // The enumerated branch below reads ids off the filesystem, so only the caller's
        // own token needs asking.
        if let Some(id) = task_id {
            crate::task::reject_malformed_work_unit_id(id)?;
        }
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
    ///
    /// The returned [`EditBase`] carries which of those cases ran: case 2 is the one the
    /// verb's ack states (M47 Inc 10 T3), cases 1 and 3 copy nothing in.
    fn read_or_copy_in(
        &self,
        path: &Path,
        schema: &Schema,
        address: &Address,
        addr: &str,
    ) -> Result<EditBase, DocFailure> {
        if path.is_file() {
            return Ok(EditBase::staged(read_staged(path, addr)?));
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
            // The copy-on-write staged the committed doc; bind the workflow's object-form
            // `allows-create` role for this doctype so the set-field-first / edit-first
            // revise path resolves `task.<role>` (the `@`-slice and the `<<author:>>`
            // address) without a prior explicit `doc create` (M45 Inc 5 T2).
            self.bind_role_on_copy_in(address)?;
            return Ok(EditBase::copied_in(read_staged(path, addr)?));
        }
        // Neither staged nor committed → the absent-instance reject, naming the
        // provisioning act this task actually has for the doctype (B2-1).
        Ok(EditBase::staged(read_staged_routed(
            path,
            addr,
            self.provision_route(address),
        )?))
    }

    /// Which provisioning act the absent-instance refusal may honestly name for
    /// `address`'s doctype **in this task** — the B2-1 repair: the shipped refusal
    /// offered `jigc doc create <type>` unconditionally, and under a workflow whose
    /// `allows-create:` gate forbids that doctype the offered command answers
    /// `create.gate-blocked` (`design/surface-contract.md` → law 1 on printed routes).
    ///
    /// The gate arm is [`engine::state::create_admission`] — the very predicate
    /// `create.gate-blocked` fires from — so the refusal and the create can never
    /// disagree about what this task may create (M45's one-predicate rule; a second
    /// membership test here would be the drift).
    fn provision_route(&self, address: &Address) -> ProvisionRoute {
        let type_name = address.r#type.as_str();
        // Both reads are fallible and both own their own doors. A failure here must not
        // convert an absent-instance refusal into a different failure — that would answer
        // a question nobody asked — so an unreadable gate keeps the shipped hint.
        let Ok(def) = self.workflow_gate() else {
            return ProvisionRoute::Create;
        };
        let Ok(schemas) = self.schemas() else {
            return ProvisionRoute::Create;
        };
        if engine::state::create_admission(&schemas, &def.allows_create, type_name).is_ok() {
            return ProvisionRoute::Create;
        }
        if crate::start::provisions_at_compose(&def, type_name) {
            return ProvisionRoute::AtCompose {
                type_name: type_name.to_owned(),
                task: self.id.clone(),
            };
        }
        ProvisionRoute::GateForbids {
            type_name: type_name.to_owned(),
            task: self.id.clone(),
            allowed: def
                .allows_create
                .iter()
                .map(|e| e.doc_type.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    /// Bind the workflow's object-form `allows-create` role for `address`'s doctype
    /// **iff it is currently unbound** — the tail of a **copy-on-first-touch** stage
    /// ([`read_or_copy_in`], M45 Inc 5 T2). So an edit verb that copy-on-writes a
    /// committed doc (the revise path — set a field or add an item before any
    /// `doc create`) still binds the role, and a later re-compose resolves
    /// `task.<role>` to it: `form-vision`'s `{{ @task.vision.grounded-in#findings }}`
    /// renders its grounding, and `<<author: {{ task.arch-doc#overview }}>>` renders
    /// the real `<type>:<slug>#slot` address rather than the slug-less pending form.
    ///
    /// **Bind-if-unbound, not the create-gate's last-write-wins** (`DECISIONS.md`
    /// 2026-07-23 M45 planning → Fork 2, the builder sub-decision): a copy-on-write is
    /// an *incidental* side effect of an edit verb, so it fills a role only when nothing
    /// has claimed it — it never clobbers an explicit `doc create --as` binding, which
    /// keeps last-write-wins (`state::create_gated`). A doctype not carried by an
    /// object-form gate entry (a bare-form or absent entry) binds nothing — inert.
    fn bind_role_on_copy_in(&self, address: &Address) -> Result<()> {
        let gate = self.workflow_gate()?;
        let Some(entry) = gate
            .allows_create
            .iter()
            .find(|e| e.doc_type == address.r#type.as_str() && !e.as_role.is_empty())
        else {
            return Ok(());
        };
        let mut roles = state::RolesRecord::load(&self.dir)
            .with_context(|| "could not read the task's bound roles")?;
        if roles.get(&entry.as_role).is_none() {
            roles.bind(
                entry.as_role.clone(),
                format!("{}:{}", address.r#type.as_str(), address.slug.as_str()),
            );
            roles
                .save(&self.dir)
                .with_context(|| "could not record the copy-on-write role binding")?;
        }
        Ok(())
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

    /// The project layer's committed config dir — the layer every schema read on this
    /// task routes through.
    fn project_config(&self) -> PathBuf {
        self.jigc_home.join(".jigc").join("config")
    }

    /// The **cascade-resolved** schema for `type_name` — a project `schemas/<ty>.yaml`
    /// whole-file shadow wins, and the `location:` is nested under the resolved
    /// `docs-root`.
    ///
    /// Resolved, not pack-read: the copy-in resolves `canonical_path` against this
    /// `location:`, so a shadow-blind read here staged a **blank skeleton** over a
    /// committed document whose prose the store still held — `doc create` acked a fresh
    /// mint for a doc the read surfaces were serving (M49 Increment 3 T2).
    fn schema(&self, type_name: &str) -> Result<Schema, DocFailure> {
        // The unknown doctype is a **block**, not an orchestration error. Before M49
        // Increment 11 / T1 it fell through to `with_context` over the pack read, so all
        // six write verbs answered *"unknown doctype `x`: the embedded pack is missing
        // `x`: no pack resource of kind Schemas with id `x`"* — no finding code, no route,
        // and `PackResourceKind`'s `{:?}` at a reader. It is the same fault the read
        // surfaces already block on, so it gets the same answer
        // ([`unknown_doctype_block`]; `design/surface-contract.md` → law 2 / the route
        // fence).
        //
        // Membership is the **pack's schema listing**, which is exactly the set
        // `CascadeDefs::declared_schemas` builds the doctype map from (a project layer
        // shadows ids the pack ships; it never introduces one) — cheap, and it adds no
        // failure mode: an unrelated doctype's malformed shadow must not make every write
        // to a different doctype fail.
        if !self
            .pack
            .list(PackResourceKind::Schemas)
            .iter()
            .any(|id| id.as_str() == type_name)
        {
            return Err(unknown_doctype_block(type_name));
        }
        Ok(
            crate::start::resolved_schema(self.pack.as_ref(), &self.project_config(), type_name)
                .with_context(|| format!("could not resolve the `{type_name}` schema"))?,
        )
    }

    /// Every cascade-resolved schema, keyed by doctype — the set `create_gated`
    /// adjudicates the unknown-doctype check against.
    fn schemas(&self) -> Result<BTreeMap<String, Schema>> {
        crate::start::resolved_schemas(self.pack.as_ref(), &self.project_config())
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
/// A rejection is adjudicated per fault by [`address_parse_guidance`], never flattened into
/// one sentence: the doctype hop it hands over is read off the **expanded** address, which a
/// fragment fault has already parsed (see that function's own note).
///
/// `engine::address::Address::parse` is **untouched**: the grammar is not widened, this is
/// an expansion at the verb boundary — the one place a human/agent types an address.
fn parse_verb_addr(pack: &dyn PackSource, project_config: &Path, addr: &str) -> Result<Address> {
    let expanded = expand_bare_singleton(pack, project_config, addr);
    let address = Address::parse(&expanded).map_err(|err| {
        let (explanation, route) = address_parse_guidance(&err, head_doctype(&expanded));
        anyhow!("malformed address `{addr}`: {err} — {explanation}\n  route: {route}")
    })?;
    // The grammar accepted the shape; the **slug head** still has to be a slug, because it
    // is what names the file (M50 Inc 2 / T1 — `crate::task::reject_malformed_slug_head`).
    crate::task::reject_malformed_slug_head(addr, address.slug.as_str())?;
    Ok(address)
}

/// The `<type>` hop of an address whose **head** parsed — `""` when it did not.
///
/// The three fragment faults are reached only after `<type>:<slug>` was accepted
/// (`engine::address::Address::parse` splits on `#`, adjudicates the head, and parses the
/// fragment last), so each of them has a real doctype in hand for its route. The three head
/// faults never read this.
fn head_doctype(expanded: &str) -> &str {
    expanded
        .split('#')
        .next()
        .and_then(|head| head.split_once(':'))
        .map(|(ty, _)| ty)
        .unwrap_or_default()
}

/// The explanation + route for one address [`engine::address::ParseError`], split on **whose
/// fault it is** (`design/surface-contract.md` → law 1 + the route floor; the M47 write-verb
/// × miss-shape precedent applied to the addressing seam).
///
/// The match is **exhaustive over the enum by construction**: a seventh variant cannot
/// compile without an author choosing its sentence and its route — which is the whole repair,
/// since all six previously shared one of each.
///
///   * The **head** faults (`EmptyType` / `MissingColon` / `EmptySlug`) are about
///     `<type>:<slug>`, so they keep the `<type>:<slug>` form and the `jigc describe`
///     doctype surface — true and followable where the caller got the head wrong.
///   * The **fragment** faults (`EmptyFragment` / `EmptyHop` / `TooManyHops`) sit over a head
///     that already parsed, so that sentence describes a part the caller typed correctly and
///     `jigc describe` — which lists doctypes, never addresses — answers nothing. They state
///     the fragment grammar and route at `jigc doc schema <type>`: the surface that, since
///     M49 Increment 5 T1, lists only addresses the write path accepts. `TooManyHops` names
///     the **depth budget** it actually broke, generated from the two constants T1 tied
///     together, so the sentence cannot go stale the day the grammar moves.
fn address_parse_guidance(err: &engine::address::ParseError, doctype: &str) -> (String, String) {
    use engine::address::ParseError;

    let head = || {
        (
            "a doc is addressed as `<type>:<slug>`, e.g. `adr:single-node-cache` (a singleton \
             doctype like `changelog` or `vision` may be named bare)"
                .to_owned(),
            format!(
                "run {} for the doctype surface",
                engine::finding::Route::mechanical(["jigc", "describe"], ""),
            ),
        )
    };
    let fragment = |explanation: String| {
        (
            explanation,
            format!(
                "run {} for the addresses this doctype accepts",
                engine::finding::Route::mechanical(doc_schema_argv(doctype), ""),
            ),
        )
    };

    match err {
        ParseError::EmptyType | ParseError::MissingColon | ParseError::EmptySlug => head(),
        ParseError::EmptyFragment => fragment(
            "the `#` fragment names a place inside the doc: `#<section>`, \
             `#<section>/<field>` or `#<section>/<item>/<field>` — drop the `#` to address \
             the whole doc"
                .to_owned(),
        ),
        ParseError::EmptyHop => fragment(
            "a fragment is `#<section>`, `#<section>/<field>` or \
             `#<section>/<item>/<field>`, with no doubled and no trailing `/`"
                .to_owned(),
        ),
        ParseError::TooManyHops => fragment(format!(
            "the grammar admits at most {} hops: a section, then two per nesting level \
             (the item id and the nested section that declares the next block), then the \
             leaf — so the deepest addressable leaf write is {} nesting levels deep",
            engine::address::MAX_FRAGMENT_HOPS,
            engine::schema::MAX_NESTING_DEPTH,
        )),
    }
}

/// The `jigc doc schema <doctype>` argv a fragment fault routes at.
///
/// The doctype hop is **user bytes** — it comes out of the address the caller typed — and a
/// mechanical route's text is `argv.join(" ")`, i.e. bytes an agent pastes into a shell. So
/// it is rendered through [`crate::task::shell_token`], and a token that re-lexes as a
/// **flag** (reachable as `jigc doc show -- -x:slug#`) is placed after clap's `--`
/// end-of-flags separator, so the emitted line runs as printed instead of failing the route
/// fence's own argv parse.
fn doc_schema_argv(doctype: &str) -> Vec<String> {
    let token = crate::task::shell_token(doctype);
    let mut argv = vec!["jigc".to_owned(), "doc".to_owned(), "schema".to_owned()];
    if token.starts_with('-') {
        argv.push("--".to_owned());
    }
    argv.push(token);
    argv
}

/// Expand a bare **singleton** head to its canonical `<type>:<type>` spelling, carrying any
/// `#fragment` through (`changelog#releases` → `changelog:changelog#releases`); every other
/// address passes through verbatim.
///
/// The singleton set is the **placement** doctypes (`design/storage.md` → Placement): their
/// slug is fixed to the type id by construction — `engine::store::read_slice` refuses every
/// *other* slug for one — so the bare type names the instance unambiguously.
fn expand_bare_singleton(pack: &dyn PackSource, project_config: &Path, addr: &str) -> String {
    let (head, fragment) = match addr.split_once('#') {
        Some((head, fragment)) => (head, Some(fragment)),
        None => (addr, None),
    };
    if head.contains(':') || !is_singleton_type(pack, project_config, head) {
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
///
/// Asked of the **resolved** schema: `placement:` is a project-shadowable key, so a
/// pack-only answer left the bare spelling refused as malformed for a doctype the
/// cascade had made a singleton — while `<ty>:<ty>`, the spelling this expansion exists
/// to save the caller from, read the document (M49 Increment 3 T2).
fn is_singleton_type(pack: &dyn PackSource, project_config: &Path, ty: &str) -> bool {
    crate::start::resolved_schema(pack, project_config, ty)
        .is_ok_and(|schema| schema.placement.is_some())
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

/// The provisioning act an absent-instance refusal may honestly name — see
/// [`ActiveTask::provision_route`], which derives it from the task's own create-gate.
enum ProvisionRoute {
    /// The task's gate admits the addressed doctype: the shipped create hint, unchanged.
    Create,
    /// The addressed doctype is the one the task's **compose** provisions (its commit
    /// doc), which bypasses the gate rather than being granted by it — so no
    /// `jigc doc create` exists for it in any task.
    AtCompose { type_name: String, task: String },
    /// The gate forbids the addressed doctype and compose provisions none: nothing in
    /// this task can provision it at all.
    GateForbids {
        type_name: String,
        task: String,
        allowed: String,
    },
}

/// Read the staged instance bytes, mapping an absent instance to an actionable
/// error (the write verbs require the instance to already exist —
/// `design/write-commands.md` → Instance provisioning). The provisioning route the
/// refusal names is the caller's, because only the caller knows the task's gate; the
/// callers that cannot raise (the instance is present by construction) pass
/// [`ProvisionRoute::Create`] through [`read_staged`].
fn read_staged_routed(path: &Path, addr: &str, route: ProvisionRoute) -> Result<String> {
    // `map_err`, not `with_context`: the latter chains the raw I/O error's `os error 2`
    // tail into `{err:#}` — a dead end. The absent-instance case is the expected reason
    // this read fails, so surface the provision route alone (M36 Inc-4).
    std::fs::read_to_string(path).map_err(|_| match route {
        ProvisionRoute::Create => {
            let start = engine::finding::Route::mechanical(["jigc", "start"], "");
            // The full, parseable form — the old bare `jigc doc create` span never parsed
            // (required args short), which the T7 parse fence surfaced and forces honest.
            // The sample title is **single**-quoted, the one form a shell expands nothing
            // inside — the route fence's quoting half refuses the double-quoted span,
            // because a reader who substitutes their own prose into a `"…"` span pastes
            // live `$` and command substitution (M48 inc-2 triage).
            let create = engine::finding::Route::mechanical(
                ["jigc", "doc", "create", "<type>", "--title", "'X'"],
                "",
            );
            anyhow!(
                "no staged instance for `{addr}` — provision it first ({start} / {create}). \
                 Note: {create} derives the id from the title (`X` → slug), \
                 not the task id — address writes at that title-derived id"
            )
        }
        // Compose already provisioned this doctype's one instance, at the task-derived id
        // — so the honest next act is to read what the task holds, not to create a second
        // one through a door that is closed to every workflow.
        ProvisionRoute::AtCompose { type_name, task } => {
            let list =
                engine::finding::Route::mechanical(["jigc", "doc", "list", "--task", &task], "");
            anyhow!(
                "no staged instance for `{addr}` — task `{task}`'s workflow provisions its \
                 `{type_name}` doc at compose and grants no in-task create for it; list what \
                 task `{task}` stages with {list}"
            )
        }
        // Nothing in this task provisions the doctype, so every in-task route is a dead
        // end: name the gate that closed it and the catalog of workflows that open it.
        ProvisionRoute::GateForbids {
            type_name,
            task,
            allowed,
        } => {
            let start = engine::finding::Route::mechanical(["jigc", "start"], "");
            anyhow!(
                "no staged instance for `{addr}` — task `{task}`'s workflow grants no in-task \
                 create for `{type_name}` (its `allows-create:` gate lists [{allowed}]), so \
                 nothing in this task provisions it; create `{type_name}` from a task minted \
                 on a workflow that grants it ({start} lists the catalog)"
            )
        }
    })
}

/// [`read_staged_routed`] for the call sites whose instance is present by construction
/// (a just-created doc, a just-copied-in one) — the refusal is unreachable there, so the
/// shipped create hint stands in.
fn read_staged(path: &Path, addr: &str) -> Result<String> {
    read_staged_routed(path, addr, ProvisionRoute::Create)
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

/// **Rank 1 at the CLI's own target resolvers: is the addressed section declared at all?**
///
/// The item-addressing write doors ask this in the engine (`undeclared_section_splice`,
/// rank 1 of shape → presence → leaf), which is why an undeclared section hop reads
/// `write.unknown-section` at all six of them. The **section-level** address forms never
/// reach a door to ask: `set-slot` at `#<section>` and `set-field` at `#<section>/<leaf>`
/// bottom out in [`slot_target`] / [`field_target`], which search the schema for a slot or
/// a field and — finding neither, because the section they would live in does not exist —
/// used to refuse with a bare `no slot addressed by …`: **code-less, route-less, outside the
/// finding envelope**, so a driver keying on `(code, target)` saw nothing and an agent got
/// no recovery. `jigc doc author` reached both resolvers through its batch lowering and so
/// produced the bare form twice more.
///
/// The miss is the same miss, so it earns the same code, the same sentence and the same
/// `jigc doc schema <doctype>` read — asked here at rank 1, from the engine's own predicate,
/// rather than inferred from the absent leaf below (M49 Increment 11 / T3, closing M47
/// increment 6's advisory 2; `design/validation.md` → The `write.*` route split).
fn undeclared_section_guard(schema: &Schema, section: &str, uri: &str) -> Result<(), DocFailure> {
    match engine::write::undeclared_section_splice(schema, section) {
        None => Ok(()),
        Some(err) => {
            let mut finding = engine::write::splice_error_finding(&err);
            stamp_target(&mut finding, uri);
            Err(DocFailure::block(finding))
        }
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
///
/// Returns a [`DocFailure`] rather than `None` since M49 Increment 11 / T3, so the one
/// resolver answers every door that consumes it identically: the two-hop form's
/// [`undeclared_section_guard`] mints a located, routed `write.unknown-section`, and the
/// address that genuinely names no declared field keeps the resolver's own sentence — in
/// the URI normal form the write-path findings key at, not the raw argument.
fn field_target(schema: &Schema, address: &Address) -> Result<FieldTarget, DocFailure> {
    let uri = address.to_string();
    let no_field = || -> DocFailure { anyhow::anyhow!("no field addressed by `{uri}`").into() };
    let Some(fragment) = address.fragment.as_ref() else {
        return Err(no_field());
    };
    match fragment {
        Fragment::Unit(field) => {
            let field = field.as_str();
            schema
                .sections
                .iter()
                .find_map(|s| match &s.body {
                    SectionBody::Simple { fields, .. } if fields.iter().any(|f| f.id == field) => {
                        Some(FieldTarget::Section {
                            section: s.id.clone(),
                            field: field.to_string(),
                        })
                    }
                    _ => None,
                })
                .ok_or_else(no_field)
        }
        Fragment::UnitLeaf(section, field) => {
            let (section, field) = (section.as_str(), field.as_str());
            undeclared_section_guard(schema, section, &uri)?;
            schema
                .sections
                .iter()
                .find(|s| s.id == section)
                .map(|s| FieldTarget::Section {
                    section: s.id.clone(),
                    field: field.to_string(),
                })
                .ok_or_else(no_field)
        }
        // The item-leaf field hop. The CLI only extracts the `(section, item, field)`
        // triple; the engine `set_item_field_or_insert` adjudicates shape (item/section
        // presence), the **declaration** of the addressed field leaf, AND the value's
        // declared type — the 2026-06-07 parity gap is closed (M24 inc-2 T2): a malformed
        // item-field value is rejected at the write verb with finalize's
        // `schema-conformance.field-value-conformant` code, and an **undeclared** leaf
        // with `write.unknown-field` before any bytes move (M47 — the undeclared-address
        // table; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9).
        Fragment::UnitItemLeaf(section, item, field) => Ok(FieldTarget::Item {
            section: section.as_str().to_string(),
            item: item.as_str().to_string(),
            field: field.as_str().to_string(),
        }),
        // A bare item hop (`#<section>/<item>`) addresses no field leaf.
        Fragment::UnitItem(_, _) => Err(no_field()),
        // A **nested** path (`#section/item/child/.../field`): the leading hop is the
        // section, the trailing hop is the field leaf, and the hops between are the
        // parent-scoped item id chain the engine locator walks (review finding S1).
        Fragment::Deep(hops) => {
            let Some((section, rest)) = hops.split_first() else {
                return Err(no_field());
            };
            let Some((field, items)) = rest.split_last() else {
                return Err(no_field());
            };
            if items.is_empty() {
                return Err(no_field());
            }
            Ok(FieldTarget::NestedItem {
                section: section.clone(),
                items: items.to_vec(),
                field: field.clone(),
            })
        }
    }
}

/// The resolved destination of a `set-slot` address: a **section-level** slot or an
/// **item-level** per-item slot on a (possibly nested) repeatable item, addressed
/// through its item-id chain. Both lower to one `engine::write::SlotAddress` and one
/// gated call ([`apply_slot_target`]).
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
/// `(section, item)` pair for the item form; the engine's gated `set_slot_validated`
/// adjudicates item/section presence.
///
/// **The section arm's undeclared-address guard** (M47 — the undeclared-address table;
/// `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9): a simple section's prose slot
/// **is** the section, addressed `#<section>` with no leaf hop, so a trailing hop over
/// one names a leaf the schema does not declare there. This used to *drop* that hop and
/// splice the section's real slot at exit 0 — the section-leaf face of the item arms'
/// `slot_span` fallback — so it is now rejected here, before the write, with the same
/// `write.unknown-field` code (and route) the item arms and the field verbs emit. The
/// message names the leaf-less address form, since an agent that reached here was aiming
/// at prose that does have a home. (A simple section declares **at most one** slot, and it
/// is unnamed — the grammar's `unit/leaf` depth reaches only its *fields* today. If
/// sub-labelled section slots are ever added, this arm is where they are declared
/// resolvable; the guard would then admit a declared sub-label rather than reject it.)
fn slot_target(schema: &Schema, address: &Address) -> Result<SlotTarget, DocFailure> {
    let uri = address.to_string();
    let no_slot = || -> DocFailure { anyhow::anyhow!("no slot addressed by `{uri}`").into() };
    let Some(fragment) = address.fragment.as_ref() else {
        return Err(no_slot());
    };
    let section_id = match fragment {
        Fragment::Unit(u) => u.as_str(),
        Fragment::UnitLeaf(section, leaf) => {
            let section = section.as_str();
            // Rank 1: an undeclared **section** outranks the leaf question, exactly as it
            // does at the item-addressing doors — there is no block on which the leaf
            // could be declared (M49 Increment 11 / T3).
            undeclared_section_guard(schema, section, &uri)?;
            // Only a **simple** section can be the section arm's target, so only there is
            // the trailing hop an undeclared leaf; over a repeatable section it is an item
            // id with no leaf, which addresses no slot at all (today's message).
            let Some(declared) = schema
                .sections
                .iter()
                .find(|s| s.id == section && matches!(s.body, SectionBody::Simple { .. }))
            else {
                return Err(no_slot());
            };
            // Where the prose *does* have a home, name it — an agent that reached here was
            // aiming at something real. Where the section declares no slot at all, say so
            // instead of pointing at an address that would fail too.
            let at = if matches!(&declared.body, SectionBody::Simple { slot: Some(_), .. }) {
                format!(
                    "section {section:?} (its prose slot is the section itself — address it \
                     as `#{section}`, with no leaf hop)"
                )
            } else {
                format!("section {section:?} (which declares no prose slot)")
            };
            return Err(block(
                &engine::write::splice_error_finding(&engine::write::SpliceError::UnknownLeaf {
                    leaf: leaf.as_str().to_string(),
                    at,
                }),
                "set-slot",
                &uri,
            ));
        }
        Fragment::UnitItemLeaf(section, item, leaf) => {
            return Ok(SlotTarget::Item {
                section: section.as_str().to_string(),
                item: item.as_str().to_string(),
                leaf: leaf.as_str().to_string(),
            });
        }
        // A **nested** path (`#section/item/child/.../leaf`): split off the section
        // (leading) and the slot leaf (trailing); the hops between are the parent-scoped
        // item id chain.
        Fragment::Deep(hops) => {
            let Some((section, rest)) = hops.split_first() else {
                return Err(no_slot());
            };
            let Some((leaf, items)) = rest.split_last() else {
                return Err(no_slot());
            };
            if items.is_empty() {
                return Err(no_slot());
            }
            return Ok(SlotTarget::NestedItem {
                section: section.clone(),
                items: items.to_vec(),
                leaf: leaf.clone(),
            });
        }
        _ => return Err(no_slot()),
    };
    // The bare `#<section>` form's rank-1 declaredness check — the [`Fragment::Unit`] arm
    // is the only one that falls through to here, and it is the form the batch lowering
    // gives a payload section's own slot key.
    undeclared_section_guard(schema, section_id, &uri)?;
    schema
        .sections
        .iter()
        .find_map(|s| match &s.body {
            SectionBody::Simple { slot: Some(_), .. } if s.id == section_id => {
                Some(SlotTarget::Section(s.id.clone()))
            }
            _ => None,
        })
        .ok_or_else(no_slot)
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
    // **The if-absent rule's fence** (M49). What the clobber-guard lets stand must be a
    // target the contract *declares*: the subject itself (the doctype-scoped `create`
    // keys — `create.gate-blocked` → `adr`) or an already-URI-normal address
    // (`create.serial-collision`'s instance address, `<type>:<slug>`). Anything else is a
    // bare **fragment** that hitch-hiked out of a producer which could not resolve its own
    // identity — the shape `engine::write::splice_error_finding` shipped, where a parse
    // break's `header/bogus` survived this stamp untouched and reached a driver as a
    // `key.target` no read verb can resolve. Without the fence the guard cannot tell a
    // resolved target from a leaked hop, so the next producer to leak one is silent again.
    //
    // Debug-only, exactly like the membership test it protects
    // (`engine::finding::debug_assert_keys_discriminate`): this is an invariant of jigc's
    // **own** finding producers, exercised by the whole suite, never a runtime condition on
    // a user's document.
    debug_assert!(
        match finding.location.as_ref().and_then(|l| l.address.as_deref()) {
            Some(address) => address == subject || address.contains(':'),
            None => false,
        },
        "a write finding's target must be URI-normal or the subject itself, never a bare \
         fragment; code `{}` carried {:?} at subject `{subject}`",
        finding.code,
        finding.location.as_ref().and_then(|l| l.address.as_deref()),
    );
}

/// The doc head (`<type>:<slug>`) of a **normal-form** address — the URI prefix every
/// write-path finding's target carries, and the head the guards rebuild their route
/// addresses from. An address with no `#fragment` is already the head. `pub(crate)` since
/// M49 Inc 8 / T3: the milestone boundary door asks the same question of a merged doc's
/// target to find the sub-task that contributed it.
pub(crate) fn doc_head(addr: &str) -> &str {
    addr.split_once('#').map_or(addr, |(head, _)| head)
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope the front door uses (`crate::start::finding_to_err`).
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
                engine::tempname::unique_nanos(),
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
                Ok(FieldTarget::Section { section, field }) if section == "header" && field == "implements"
            ),
            "the canonical section-qualified fragment resolves to (header, implements)",
        );

        let alias = parse_addr("commit:add-rate-limiter#implements").expect("valid");
        assert!(
            matches!(
                field_target(&schema, &alias),
                Ok(FieldTarget::Section { section, field }) if section == "header" && field == "implements"
            ),
            "the flat single-hop alias resolves identically",
        );
    }

    /// The **read surface** picks one canonical form for the aliased commit header
    /// addresses (RC alpha3 findings §43 — the "printed `#type`/`#scope` errors"
    /// claim, REFUTED then pinned). Both the single-hop `#type` and the
    /// section-qualified `#header/type` resolve identically at the write boundary
    /// (proven by [`implements_resolves_section_qualified_and_flat_alias`]) — but no
    /// read surface *stated* the alias, so the two forms could drift apart across
    /// printed surfaces. `doc schema`'s projection — the read surface an agent learns
    /// write addresses from — advertises the **section-qualified** form alone, never
    /// the bare single-hop, so the surface states the canonical form exactly once.
    /// Driven on the emitted [`schema_contract`] projection itself.
    #[test]
    fn schema_read_surface_picks_the_qualified_commit_address() {
        let schema = load_schema(COMMIT_YAML).expect("commit.yaml loads");
        let contract = schema_contract(&schema, None);

        for id in ["type", "scope"] {
            let field = contract
                .fields
                .iter()
                .find(|f| f.id == id)
                .unwrap_or_else(|| panic!("commit projects a `{id}` field"));
            let expected = format!("commit:<slug>#header/{id}");
            assert_eq!(
                field.set_field.as_deref(),
                Some(expected.as_str()),
                "the read surface advertises the section-qualified write address for `{id}`",
            );
        }

        // The bare single-hop alias is advertised nowhere — the canonical form is
        // stated exactly once, so no two printed surfaces can disagree on the wire.
        for field in &contract.fields {
            if let Some(addr) = field.set_field.as_deref() {
                assert!(
                    !addr.ends_with("#type") && !addr.ends_with("#scope"),
                    "no field advertises the bare single-hop alias; got `{addr}`",
                );
            }
        }
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
