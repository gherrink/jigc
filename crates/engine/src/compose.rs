//! Workflow composition — include expansion + deterministic placeholder
//! resolution into the four-class emitted format.
//!
//! See `design/workflow-dialect.md` and `design/command-catalog.md`.
//!
//! This module lands the composition **inputs**: parsing one workflow definition
//! file into a [`WorkflowDef`] and one step definition file into a [`StepDef`]
//! (`workflow-dialect.md` → On-disk definition format — a step is a file, its id
//! is the filename, its body is the verbatim prompt). A workflow definition is a
//! Markdown body under a `---`-fenced front-matter block (`workflow-dialect.md`
//! → On-disk definition format), where the front-matter is **config-family YAML**
//! (`when`, `creates-task` default `true`, `allows-create`) and the body is
//! **include-only at top level**: each non-blank, non-comment line must be a
//! `{{ include: step:<id> }}` placeholder. The ordered include ids are the
//! composition order (principle #2: the file *is* the ordered list; ids carry no
//! position). This is phase-1 input gathering for one layer's workflow file
//! (`overrides.md` → Resolution algorithm).

use crate::cascade::{Anchor, StructuralDelta};
use crate::finding::{Finding, Location};
use serde::{Deserialize, Serialize};

/// One `allows-create` entry: a doctype the agent may `jigc doc create` during
/// the task, bound to a context role (`{type: adr, as: decision}` →
/// `AllowsCreate { doc_type: "adr", as_role: "decision" }`). See
/// `workflow-dialect.md` → On-disk definition format and `write-commands.md` →
/// The create-gate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowsCreate {
    /// The doctype id the create-gate admits (e.g. `adr`).
    #[serde(rename = "type")]
    pub doc_type: String,
    /// The context role the created instance binds to (e.g. `decision`).
    #[serde(rename = "as")]
    pub as_role: String,
}

/// One `reads` entry: a context role bound from an existing committed doc via
/// `jigc task bind <role> <addr>` (`{role: spec, type: spec}` →
/// `Reads { role: "spec", doc_type: "spec" }`). The dual of [`AllowsCreate`] —
/// `allows-create` declares a role bound to a doc the task *creates*; `reads`
/// declares a role bound to a doc a *prior* task committed. Both make
/// `task.<role>` a declared `workflow-refs` root. See `workflow-dialect.md` →
/// On-disk definition format and `write-commands.md` → Binding a context role.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reads {
    /// The context role the bound instance fills (e.g. `spec`).
    pub role: String,
    /// The doctype id the bound instance must match (e.g. `spec`).
    #[serde(rename = "type")]
    pub doc_type: String,
}

/// A parsed workflow definition: its metadata front-matter plus the ordered
/// include id list lifted from the include-only body.
///
/// `includes` are step ids in physical body order — the composition order. The
/// `when` selection hint is carried for completeness but the catalog
/// ([`crate::catalog`]) is the surface that consumes it; composition consumes
/// `creates_task`, `allows_create`, and `includes`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowDef {
    /// The one-line `when` selection hint (router/catalog guidance).
    pub when: Option<String>,
    /// Authored prose: what this workflow *is* (one or two sentences). Optional —
    /// `None` when the front-matter omits it. Projected by the `describe`
    /// self-description surface (`introspection.md` → The authored fields).
    pub description: Option<String>,
    /// Authored prose: when/why you'd reach for this workflow. Optional — `None`
    /// when the front-matter omits it. Projected by `describe` (`introspection.md`).
    pub usage: Option<String>,
    /// Whether running this workflow mints a task. Default `true` when the
    /// front-matter omits the key (`workflow-dialect.md` → On-disk format).
    pub creates_task: bool,
    /// Whether this workflow is selectable from the router/orientation catalog.
    /// Default `true` when omitted. A `creates-task: true` workflow that can
    /// never reach a commit boundary on its own (e.g. the fan-out `sub-task`,
    /// whose only commit boundary is the parent milestone's `finalize`) sets
    /// this `false` so the router never offers it as a top-level pick.
    pub selectable: bool,
    /// The doctypes the agent may create during the task (default empty).
    pub allows_create: Vec<AllowsCreate>,
    /// The context roles bound from existing committed docs (default empty).
    pub reads: Vec<Reads>,
    /// The ordered step ids from the include-only body — the composition order.
    pub includes: Vec<String>,
}

/// The config-family front-matter of a workflow definition, as YAML.
///
/// `creates-task` defaults to `true` when omitted (`workflow-dialect.md`). The
/// catalog reads only `when` from the same block; this is composition's fuller
/// read.
#[derive(Deserialize)]
struct WorkflowFrontMatter {
    when: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    usage: Option<String>,
    #[serde(rename = "creates-task", default = "default_true")]
    creates_task: bool,
    #[serde(default = "default_true")]
    selectable: bool,
    #[serde(rename = "allows-create", default)]
    allows_create: Vec<AllowsCreate>,
    #[serde(default)]
    reads: Vec<Reads>,
}

fn default_true() -> bool {
    true
}

/// One argument of a catalog command-ref — the three arg kinds of
/// `command-catalog.md` → The three arg kinds.
///
/// - [`CommandArg::Literal`] — a plain YAML string: a fixed token, rendered as
///   itself (shell-quoted if needed).
/// - [`CommandArg::From`] — `{ from: <data-value-path> }`: resolved at
///   compose-time against the workflow's data-value context. The path text is
///   carried **verbatim** here (the workflow-dialect grammar string); parsing /
///   resolution is composition's job, not the loader's — this task is load-only.
/// - [`CommandArg::Agent`] — `{ agent: <name>, hint: <string> }`: left for the
///   agent to fill at run-time, rendered as an `<NAME>` marker. The `hint` is
///   **required and non-empty** (the agent needs to know what to substitute —
///   `command-catalog.md` → Validation).
///
/// Any other map shape (an unknown key, a `from` that is not a string, an
/// `agent` missing or with an empty `hint`) is a malformed entry rejected at
/// load time. The serde projection is internally tagged so the loaded catalog's
/// golden distinguishes the three kinds unambiguously.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CommandArg {
    /// A fixed literal token.
    Literal { literal: String },
    /// A `from:` data-value path, carried verbatim (resolved in composition).
    From { from: String },
    /// An `agent:` run-time substitution with a required non-empty hint.
    Agent { agent: String, hint: String },
}

/// One catalog command-ref: the `{{cli.<id>}}` entry the workflow's steps
/// resolve against (`command-catalog.md` → Catalog format).
///
/// The `id` is the map key in the [`CommandCatalog`], not a field here. `stdin`
/// documents what stdin carries (the renderer ignores it); `hint` is the
/// one-line documentation surfaced via tooling.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandRef {
    /// The executable (typically `jigc`).
    pub command: String,
    /// The ordered argument list — literals, `from:`, and `agent:` args.
    pub args: Vec<CommandArg>,
    /// Optional documentation of what stdin carries (renderer ignores it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
    /// One-line documentation of the command-ref.
    pub hint: String,
}

/// The workflow command catalog: command-refs keyed by `id`.
///
/// A [`BTreeMap`] so the serde projection (and any iteration) is deterministic
/// in id order — the composition-is-deterministic invariant applies to the
/// loaded catalog too. Built by [`load_command_catalog`] from the pack's
/// `commands.yaml`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandCatalog {
    /// The command-refs keyed by their `{{cli.<id>}}` id.
    pub commands: std::collections::BTreeMap<String, CommandRef>,
}

impl CommandCatalog {
    /// Look up a command-ref by its `{{cli.<id>}}` id.
    pub fn get(&self, id: &str) -> Option<&CommandRef> {
        self.commands.get(id)
    }
}

/// Parse `commands.yaml` raw bytes into a [`CommandCatalog`] keyed by id.
///
/// The file is the config-family YAML of `command-catalog.md` → Catalog format:
/// a top-level `commands:` list, each entry `{ id, command, args, stdin?, hint }`.
/// Each arg is one of the three kinds (`command-catalog.md` → The three arg
/// kinds); an unknown arg map shape — an unrecognized key, an `agent:` with an
/// empty `hint` — is a blocking conformance [`Finding`] (the settled block
/// envelope, `DECISIONS.md` 2026-05-31). A duplicate `id` is likewise rejected:
/// the catalog is keyed by id, so two entries under one key is a definition bug.
/// Load-only: `from:` paths are carried verbatim and resolved later in
/// composition.
pub fn load_command_catalog(bytes: &[u8]) -> Result<CommandCatalog, Finding> {
    let text = std::str::from_utf8(bytes).map_err(|_| {
        Finding::blocking(
            "workflow-refs.not-utf8",
            "command catalog is not valid UTF-8",
            Location::at(1, 1),
        )
    })?;
    let raw: RawCatalog = serde_yaml_ng::from_str(text).map_err(|source| {
        Finding::blocking(
            "workflow-refs.malformed-command-catalog",
            format!("command catalog is malformed: {source}"),
            Location::at(1, 1),
        )
    })?;

    let mut commands = std::collections::BTreeMap::new();
    for entry in raw.commands {
        let args = entry
            .args
            .into_iter()
            .map(parse_command_arg)
            .collect::<Result<Vec<_>, _>>()?;
        let command_ref = CommandRef {
            command: entry.command,
            args,
            stdin: entry.stdin,
            hint: entry.hint,
        };
        if commands.insert(entry.id.clone(), command_ref).is_some() {
            return Err(Finding::blocking(
                "workflow-refs.duplicate-command-id",
                format!("command catalog has two entries under id `{}`", entry.id),
                Location::at(1, 1),
            ));
        }
    }
    Ok(CommandCatalog { commands })
}

/// Convert one raw YAML arg value into a typed [`CommandArg`], rejecting any
/// unknown map shape.
fn parse_command_arg(raw: serde_yaml_ng::Value) -> Result<CommandArg, Finding> {
    use serde_yaml_ng::Value;
    match raw {
        Value::String(literal) => Ok(CommandArg::Literal { literal }),
        Value::Mapping(map) => parse_arg_mapping(map),
        other => Err(malformed_arg(format!(
            "command arg must be a string literal or a `from:`/`agent:` map, got {other:?}"
        ))),
    }
}

/// Convert one `from:`/`agent:` arg mapping into a typed [`CommandArg`].
///
/// Exactly one recognized shape is admitted: `{ from: <string> }` or
/// `{ agent: <string>, hint: <non-empty string> }`. Any other key, a wrong
/// value type, a missing `hint`, or an empty `hint` is a malformed arg.
fn parse_arg_mapping(map: serde_yaml_ng::Mapping) -> Result<CommandArg, Finding> {
    use serde_yaml_ng::Value;
    let str_at = |map: &serde_yaml_ng::Mapping, key: &str| -> Option<String> {
        match map.get(Value::from(key)) {
            Some(Value::String(s)) => Some(s.clone()),
            _ => None,
        }
    };

    if map.contains_key(Value::from("from")) {
        if map.len() != 1 {
            return Err(malformed_arg(
                "a `from:` arg takes only the `from` key".to_owned(),
            ));
        }
        let from = str_at(&map, "from")
            .ok_or_else(|| malformed_arg("`from:` must be a data-value-path string".to_owned()))?;
        return Ok(CommandArg::From { from });
    }

    if map.contains_key(Value::from("agent")) {
        let unknown = map
            .keys()
            .any(|k| !matches!(k, Value::String(s) if s == "agent" || s == "hint"));
        if unknown {
            return Err(malformed_arg(
                "an `agent:` arg takes only the `agent` and `hint` keys".to_owned(),
            ));
        }
        let agent = str_at(&map, "agent")
            .ok_or_else(|| malformed_arg("`agent:` must be a name string".to_owned()))?;
        let hint = str_at(&map, "hint").filter(|h| !h.trim().is_empty()).ok_or_else(|| {
            malformed_arg(format!(
                "`agent:` arg `{agent}` needs a non-empty `hint` (the agent must know what to substitute)"
            ))
        })?;
        return Ok(CommandArg::Agent { agent, hint });
    }

    Err(malformed_arg(
        "command arg map has no recognized `from:` or `agent:` key".to_owned(),
    ))
}

/// A blocking conformance finding for a malformed catalog arg.
fn malformed_arg(message: String) -> Finding {
    Finding::blocking(
        "workflow-refs.malformed-command-arg",
        message,
        Location::at(1, 1),
    )
}

/// The raw `commands:`-list shape of `commands.yaml`, deserialized before arg
/// kinds are typed. Args land as untyped [`serde_yaml_ng::Value`] so
/// [`parse_command_arg`] can reject unknown map shapes (serde's derive would
/// silently mis-route them).
#[derive(Deserialize)]
struct RawCatalog {
    commands: Vec<RawCommandRef>,
}

#[derive(Deserialize)]
struct RawCommandRef {
    id: String,
    command: String,
    args: Vec<serde_yaml_ng::Value>,
    #[serde(default)]
    stdin: Option<String>,
    hint: String,
}

/// Render a resolved command-ref to a shell-safe command line — the literal
/// command string **without** the `Run:` wrapper (the emitter adds that).
///
/// For each arg of the [`CommandRef`] (`command-catalog.md` → The three arg
/// kinds): a [`CommandArg::Literal`] renders as its token; a [`CommandArg::From`]
/// parses its data-value path and resolves it against `ctx` via the seq-5
/// resolver, then renders the resolved value (a scalar's text, an address's
/// canonical form, or empty text for an [`Resolution::Absent`]); a
/// [`CommandArg::Agent`] renders as an uppercase `<NAME>` marker left for the
/// agent to fill at run-time. Each rendered token is then POSIX-quoted per
/// [`shell_quote`], and the command + quoted args are joined by single spaces.
///
/// Rendering is a **pure function** of `(cmd, ctx)` — same inputs always yield
/// the same line (the determinism boundary). A `from:` path that fails to parse
/// or resolve surfaces the resolver's blocking [`Finding`].
pub fn render_command(
    cmd: &CommandRef,
    ctx: &crate::data_value::ComposeContext,
) -> Result<String, Finding> {
    let mut words = Vec::with_capacity(cmd.args.len() + 1);
    words.push(shell_quote(&cmd.command));
    for arg in &cmd.args {
        words.push(render_arg(arg, ctx)?);
    }
    Ok(words.join(" "))
}

/// Render one command-ref arg to its shell-quoted token.
///
/// - [`CommandArg::Literal`] → the literal text.
/// - [`CommandArg::From`] → the data-value path is parsed and resolved against
///   `ctx`; the resolved value's text is the token ([`Resolution::Scalar`] → its
///   value; [`Resolution::Address`] / [`Resolution::Content`] → the canonical
///   address string; [`Resolution::Absent`] → empty text, per
///   `workflow-dialect.md` → Empty vs unresolvable).
/// - [`CommandArg::Agent`] → an uppercase `<NAME>` marker (the agent fills it at
///   run-time); the marker is left **bare** by [`shell_quote`] since its
///   character set is the bare-allowed set (`command-catalog.md` → Shell-safe).
fn render_arg(
    arg: &CommandArg,
    ctx: &crate::data_value::ComposeContext,
) -> Result<String, Finding> {
    use crate::data_value::{Path, Resolution};
    let token = match arg {
        CommandArg::Literal { literal } => literal.clone(),
        // The `<NAME>` agent marker is emitted **bare** (it is the agent's
        // run-time substitution point, not a value to quote — `command-catalog.md`
        // → Shell-safe rendering). It bypasses `shell_quote` entirely.
        CommandArg::Agent { agent, .. } => return Ok(format!("<{}>", agent.to_uppercase())),
        CommandArg::From { from } => {
            let path = Path::parse(from).map_err(|source| {
                Finding::blocking(
                    "workflow-refs.malformed-data-value",
                    format!("command-ref `from:` path `{from}` is malformed: {source}"),
                    Location::at(1, 1),
                )
            })?;
            match path.resolve(ctx)? {
                Resolution::Scalar { value } => value,
                Resolution::Address { address } | Resolution::Content { address } => {
                    address.to_string()
                }
                Resolution::Absent => String::new(),
                Resolution::Catalog { .. }
                | Resolution::Store { .. }
                | Resolution::Milestone { .. } => {
                    return Err(collection_not_lone());
                }
            }
        }
    };
    Ok(shell_quote(&token))
}

/// POSIX single-arg quoting, deterministic (`command-catalog.md` → Shell-safe
/// rendering).
///
/// - A **non-empty** arg whose every char is in the bare charset
///   `[A-Za-z0-9._:#/=@+-]` is returned unchanged (addresses, identifiers, flag
///   names, single-dash stdin).
/// - Anything else (whitespace, a shell metachar, or the empty string) is
///   **single-quoted**; an embedded `'` is closed-escaped-reopened (`'it'\''s'`).
///
/// The `<NAME>` agent marker (`<` / `>` are metachars, *not* in the bare charset)
/// is never passed here — [`render_arg`] emits it bare directly.
fn shell_quote(arg: &str) -> String {
    let is_bare = !arg.is_empty() && arg.chars().all(is_bare_char);
    if is_bare {
        return arg.to_owned();
    }
    let mut out = String::with_capacity(arg.len() + 2);
    out.push('\'');
    for ch in arg.chars() {
        if ch == '\'' {
            // Close the quote, emit an escaped literal quote, reopen.
            out.push_str("'\\''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Whether `ch` is in the shell bare charset `[A-Za-z0-9._:#/=@+-]`
/// (`command-catalog.md` → Shell-safe rendering: addresses, identifiers, flag
/// names, single-dash stdin).
fn is_bare_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || "._:#/=@+-".contains(ch)
}

/// Emit one composed step body into the four-class emitted format — phases 8
/// (placeholder resolution) and 9 (line emission) of `overrides.md` →
/// Resolution algorithm, for one step body.
///
/// The body is walked line-by-line; each line maps to exactly one of the four
/// classes (`workflow-dialect.md` → Emitted format — the four classes, the four
/// rules):
///
/// - **Run** — a line that is a lone `{{cli.<id>}}` placeholder resolves the
///   command-ref in `catalog` via [`render_command`] and emits
///   `` Run: `<cmd>` `` (the command in backticks, machine-extractable by
///   `` ^Run: `(.+)`$ ``).
/// - **Content** — a line that is a lone `{{@<path>}}` data-value resolves via
///   the seq-5 resolver; a bound resolution ([`Resolution::Content`] /
///   [`Resolution::Address`]) emits a `> ` Markdown blockquote of the resolved
///   address; an [`Resolution::Absent`] (a declared-but-unbound role) emits an
///   **empty line** — empty-not-finding (`workflow-dialect.md` → Empty vs
///   unresolvable).
/// - **Author** — a line carrying `<<author: {{<path>}}>>` keeps the `<<…>>`
///   wrapper unchanged and resolves only the **embedded** `{{<path>}}` to its
///   address, emitting `<<author: <address>>` (rule 3: the wrapper survives
///   composition; only the placeholder resolves).
/// - **Reason** — everything else is bare prose, emitted verbatim (rule 4: the
///   default, no marker, no overhead).
///
/// Emission is a **pure function** of `(body, ctx, catalog)` — same inputs always
/// yield the same text (the determinism boundary; no I/O, clock, or LLM). A
/// command-ref id absent from `catalog`, a `{{cli.…}}`/`{{@…}}`/`<<author:>>`
/// whose data-value fails to resolve, surfaces the resolver/catalog's blocking
/// [`Finding`].
pub fn emit_step_body(
    body: &str,
    ctx: &crate::data_value::ComposeContext,
    catalog: &CommandCatalog,
) -> Result<String, Finding> {
    emit_step_body_with(body, ctx, catalog, None)
}

/// Emit one composed step body, dereferencing `{{@<path>}}` Content lines through
/// `store` when one is provided — the store-backed sibling of [`emit_step_body`].
///
/// With a `store`, an `@`-Content data-value that resolves to a bound role plus a
/// cross-doc `.relation` hop chain (the superseding-decision `{{@task.decision.
/// supersedes#decision}}`) is **dereferenced**: the role's bound doc is walked along
/// each relation edge (via the edge overlay), the landing doc's `#fragment` is
/// sliced through [`crate::store`], and the prose is emitted as a multi-line `> `
/// Content blockquote (`workflow-dialect.md` → Emitted format). An unbound role or
/// an unset relation short-circuits to an **empty line** (no finding — the
/// empty-vs-unresolvable contract). Without a `store` (the structural
/// `workflow_refs` gate, the pure tests) Content lines carry the resolved address
/// handle, the inc-3 stance.
pub fn emit_step_body_with(
    body: &str,
    ctx: &crate::data_value::ComposeContext,
    catalog: &CommandCatalog,
    store: Option<&dyn ContentStore>,
) -> Result<String, Finding> {
    let trailing_newline = body.ends_with('\n');
    let mut out_lines = Vec::new();
    for line in body.lines() {
        out_lines.push(emit_line(line, ctx, catalog, store)?);
    }
    let mut emitted = out_lines.join("\n");
    if trailing_newline {
        emitted.push('\n');
    }
    Ok(emitted)
}

/// Emit one body line into its four-class form.
fn emit_line(
    line: &str,
    ctx: &crate::data_value::ComposeContext,
    catalog: &CommandCatalog,
    store: Option<&dyn ContentStore>,
) -> Result<String, Finding> {
    let trimmed = line.trim();

    // Run: a lone `{{cli.<id>}}` placeholder.
    if let Some(id) = parse_cli_placeholder(trimmed) {
        let cmd = catalog.get(id).ok_or_else(|| {
            Finding::blocking(
                "workflow-refs.command-ref-resolves",
                format!("command-ref `{{{{cli.{id}}}}}` resolves to no catalog entry"),
                Location::at(1, 1),
            )
        })?;
        let rendered = render_command(cmd, ctx)?;
        return Ok(format!("Run: `{rendered}`"));
    }

    // Author: a `<<author: {{<path>}}>>` directive — wrapper preserved, only the
    // embedded `{{<path>}}` resolved to its address.
    if let Some(emitted) = emit_author_line(trimmed, ctx)? {
        return Ok(emitted);
    }

    // Source seam: a lone `{{source}}` read-only context placeholder — the
    // CLI-owned foreign bytes the `jigc migrate` verb stages into the task,
    // emitted **verbatim** (auto-migration.md → The source seam). Syntactically
    // distinct from a managed-doc `{{@…}}` deref: it carries non-managed bytes,
    // never an address, so it never reaches the data-value `Path` grammar. An
    // unfed seam (`ctx.source` is `None`) emits an empty line — empty-not-finding,
    // mirroring an unbound role.
    if parse_lone_placeholder(trimmed) == Some("source") {
        return Ok(ctx.source.clone().unwrap_or_default());
    }

    // Content: a lone `{{@<path>}}` data-value placeholder.
    if let Some(inner) = parse_lone_placeholder(trimmed)
        && let Some(path_text) = inner.strip_prefix('@')
    {
        // Re-attach the `@` the resolver expects.
        let path = parse_data_value(&format!("@{path_text}"))?;
        return emit_content(&path, ctx, store);
    }

    // Reason (resolved data-value): a lone bare `{{<path>}}` data-value — the
    // **scalar** form (`{{task.intent}}` → the scalar string) or a bare path's
    // **address** (`workflow-dialect.md` → Leaves: bare path = the reference).
    // It carries no `> ` marker (that is the `@`-Content class); the resolved
    // text replaces the placeholder inline, emitted as Reason prose. A lone
    // `{{include: …}}` never reaches here — includes are expanded in phase 7 and
    // split out of step bodies before emission. A lone `{{catalog}}` resolves to
    // the collection and renders as the router's option list (one `- <id> —
    // <when>` line per entry, fed order; an empty catalog → empty text, the
    // empty-not-finding stance — `workflow-dialect.md` → Workflow selection). A lone
    // `{{store.<doctype>}}` resolves to the committed-instance collection and renders
    // as a `> ` Content list (one address per line, fed order; an empty store →
    // empty text — `workflow-dialect.md` → data-value roots).
    if let Some(inner) = parse_lone_placeholder(trimmed) {
        use crate::data_value::Resolution;
        let path = parse_data_value(inner)?;
        match path.resolve(ctx)? {
            Resolution::Catalog { entries } => return Ok(render_catalog_options(&entries)),
            Resolution::Store { entries } => return Ok(render_store_list(&entries)),
            _ => {}
        }
        return emit_bare_data_value(&path, ctx);
    }

    // Reason: bare prose — passed through verbatim. Inline `{{<path>}}` data-value
    // tokens are NOT resolved here: that substitution runs as a pre-phase-5 pass
    // over step-file bodies only ([`resolve_inline_data_values`]), so phase-5
    // fill-applied (agent/project-authored) prose is never rewritten at emit
    // (`DECISIONS.md` 2026-06-12 — the determinism boundary applied).
    Ok(line.to_owned())
}

/// Resolve every **inline** `{{<path>}}` bare data-value token in a **step-file
/// body**, substituting in place with the lone-line Reason-resolved semantics
/// ([`emit_bare_data_value`]: scalar text / address handle / absent → empty) — the
/// compose-time mechanism a literal `jigc … --task {{task.id}}` authoring line
/// needs (`DECISIONS.md` M17 settle pre-fix #3: compose-time substitution, the
/// deterministic pack+compose surface; M17 inc-4 revised the M12-era lone-line-only
/// rule to inline resolution).
///
/// **Runs before phase 5** (fill application), on the body as the step file
/// authored it — pack-, team-, or project-shadow-authored *composed* prose. It
/// must never run over fill-applied text: slot-fill content is agent/project
/// prose, and the CLI owns structure, never the prose (`VISION.md` → the
/// determinism boundary; `DECISIONS.md` 2026-06-12). The frontend applies this
/// pass per fetched step, then splices fills ([`apply_slot_fills`]).
///
/// Line discipline: a line that is a lone `{{…}}` placeholder (any kind — those
/// are the lone-line emit classes, blocking semantics included) or an
/// `<<author: …>>` directive passes through **verbatim**, left to its own class
/// at emit. Every other line gets the token-anywhere scan
/// ([`emit_inline_data_values`]).
pub fn resolve_inline_data_values(body: &str, ctx: &crate::data_value::ComposeContext) -> String {
    let trailing_newline = body.ends_with('\n');
    let mut out_lines = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if parse_lone_placeholder(trimmed).is_some() || trimmed.starts_with("<<author:") {
            out_lines.push(line.to_owned());
        } else {
            out_lines.push(emit_inline_data_values(line, ctx));
        }
    }
    let mut resolved = out_lines.join("\n");
    if trailing_newline {
        resolved.push('\n');
    }
    resolved
}

/// The per-line token-anywhere scan behind [`resolve_inline_data_values`], the
/// [`next_fill_token`] discipline: a `{{…}}` whose
/// trimmed inner is `fill:`-shaped (phase 5's), `cli.`-prefixed, `@`-prefixed, or
/// `include:`-shaped (the lone-line classes) is **skipped** — the scan resumes
/// just past its `{{`, leaving the token for its own phase. Every other inner is
/// tried as a bare data-value path; one that fails — a malformed path, an
/// unresolvable path, or a **collection** resolution (`catalog` / `store.*` /
/// `milestone.*` — collections stay lone-line classes) — is left **verbatim**,
/// the same skip discipline. Inline tokens are **inert, never blocking**:
/// prose *mentioning* `{{…}}` syntax must not brick every later compose of the
/// workflow (M17 inc-4 validation fix). Blocking stays with the lone-line
/// classes. A line with no resolved token is returned byte-unchanged.
fn emit_inline_data_values(line: &str, ctx: &crate::data_value::ComposeContext) -> String {
    let mut rewritten = String::new();
    let mut cursor = 0;
    while let Some(rel_open) = line[cursor..].find("{{") {
        let open = cursor + rel_open;
        // The matching `}}` is the first one after this `{{` (data-value paths
        // carry no braces, so no nesting to balance). No closer: the rest of the
        // line is plain prose.
        let Some(rel_close) = line[open + 2..].find("}}") else {
            break;
        };
        let close = open + 2 + rel_close;
        let inner = line[open + 2..close].trim();
        // Skip the other placeholder kinds to their own phases.
        if inner.starts_with("fill:")
            || inner.starts_with("cli.")
            || inner.starts_with('@')
            || inner.starts_with("include:")
            || inner == "source"
        {
            rewritten.push_str(&line[cursor..open + 2]);
            cursor = open + 2;
            continue;
        }
        // An inner that fails to parse or resolve is left verbatim — the same
        // skip: inline tokens are inert (see the doc comment).
        let Some(resolved) = parse_data_value(inner)
            .ok()
            .and_then(|path| emit_bare_data_value(&path, ctx).ok())
        else {
            rewritten.push_str(&line[cursor..open + 2]);
            cursor = open + 2;
            continue;
        };
        rewritten.push_str(&line[cursor..open]);
        rewritten.push_str(&resolved);
        cursor = close + 2;
    }
    if cursor == 0 {
        return line.to_owned();
    }
    rewritten.push_str(&line[cursor..]);
    rewritten
}

/// A read-side dereference surface for `{{@<path>}}` Content lines: walk one
/// cross-doc relation edge, and slice a committed/working doc fragment to its prose.
///
/// The composer stays a pure function of its inputs (the determinism boundary); the
/// **I/O** of reading the committed store and the edge index lives behind this
/// trait, fed in by the frontend (CLI locates layers + builds the overlay; engine
/// resolves — `VISION.md` principle #4). The reference implementation is
/// [`StoreContext`], over the inc-5 [`crate::index::WorkingOverlay`] +
/// [`crate::store`]; a test may supply a fake.
pub trait ContentStore {
    /// Walk one `.relation` edge from `from` (a `<type>:<slug>` identity), returning
    /// the target identity, or `None` when the relation is unset on `from` (an
    /// **absent** value → empty text, not an error).
    fn walk_edge(&self, from: &str, relation: &str) -> Option<String>;

    /// Slice the committed/working doc named by `address` to its `#fragment` prose,
    /// or a blocking [`Finding`] when the target is missing/unparseable/unsliceable
    /// (the one block envelope, routed).
    fn read_slice(&self, address: &crate::address::Address) -> Result<String, Finding>;
}

/// The reference [`ContentStore`] — the inc-5 committed store + edge overlay.
///
/// Holds the cascade-resolved schema set, the repo root (committed store), and the
/// active task's [`WorkingOverlay`](crate::index::WorkingOverlay) (committed ∪ this
/// task's working edges). [`walk_edge`](ContentStore::walk_edge) walks the overlaid
/// graph; [`read_slice`](ContentStore::read_slice) reads the committed store
/// (`crate::store::read_slice` — identity-is-the-path). The CLI builds one per
/// compose; the engine consumes it through the trait.
pub struct StoreContext<'a> {
    /// The committed-store repo root (`<repo_root>/<location>/<slug>.md`).
    pub repo_root: &'a std::path::Path,
    /// The cascade-resolved schema set (type → schema).
    pub schemas: &'a std::collections::BTreeMap<String, crate::schema::Schema>,
    /// The active task's edge overlay (committed ∪ this task's working edges).
    pub overlay: &'a crate::index::WorkingOverlay,
}

impl ContentStore for StoreContext<'_> {
    fn walk_edge(&self, from: &str, relation: &str) -> Option<String> {
        self.overlay.walk_edge(from, relation)
    }

    fn read_slice(&self, address: &crate::address::Address) -> Result<String, Finding> {
        crate::store::read_slice(self.repo_root, self.schemas, address)
    }
}

/// Emit a resolved `{{@<path>}}` content line.
///
/// Without a `store` (the structural `workflow_refs` gate): a bound resolution emits
/// a `> ` blockquote of the resolved **address handle**, an [`Resolution::Absent`]
/// emits an empty line (the inc-3 stance).
///
/// With a `store` (the live read path): a bound role plus a cross-doc `.relation`
/// hop chain is **dereferenced** — the bound doc is walked along each relation edge
/// over the overlay, the landing doc's `#fragment` is sliced through the store, and
/// the prose is emitted as a multi-line `> ` Content blockquote (each prose line
/// prefixed, `workflow-dialect.md` → Emitted format). An unbound role
/// ([`Resolution::Absent`]) or an unset relation (a hop with no edge) short-circuits
/// to an **empty line** — no finding (`worked-examples.md` flow #1; the
/// empty-vs-unresolvable contract).
fn emit_content(
    path: &crate::data_value::Path,
    ctx: &crate::data_value::ComposeContext,
    store: Option<&dyn ContentStore>,
) -> Result<String, Finding> {
    use crate::data_value::Resolution;
    match path.resolve(ctx)? {
        Resolution::Content { address } => match store {
            // The live read path: dereference through the edge hops + the store.
            Some(store) => match deref_content(path, &address, store)? {
                Some(prose) => Ok(blockquote(&prose)),
                None => Ok(String::new()), // an unset relation → empty, no finding.
            },
            // No store (the structural gate): the resolved address handle.
            None => Ok(format!("> {address}")),
        },
        Resolution::Address { address } => Ok(format!("> {address}")),
        Resolution::Scalar { value } => Ok(format!("> {value}")),
        Resolution::Absent => Ok(String::new()),
        Resolution::Catalog { .. } | Resolution::Store { .. } | Resolution::Milestone { .. } => {
            Err(collection_not_lone())
        }
    }
}

/// Dereference a bound-role `@`-Content `path` to the prose its final address slices
/// to, walking the cross-doc `.relation` hops past the role over `store`'s overlay.
///
/// `bound` is the bound role's doc address ([`Resolution::Content`] built it from the
/// role binding). The path's hops are `[role, relation*]`: the **first** hop is the
/// role (already resolved into `bound`), the **rest** are cross-doc relation edges to
/// walk (e.g. `supersedes`). Each hop walks one edge over the overlay; an **unset**
/// relation (no edge) yields `Ok(None)` → the caller emits empty text. The landing
/// identity carries the path's own `#fragment`, which the store slices to its prose.
///
/// A walk that lands on a malformed identity, or a store read that fails (missing /
/// unparseable / unsliceable target), surfaces the store's blocking [`Finding`] —
/// the one routed block envelope, never a panic.
fn deref_content(
    path: &crate::data_value::Path,
    bound: &crate::address::Address,
    store: &dyn ContentStore,
) -> Result<Option<String>, Finding> {
    // The bound role's doc identity — `<type>:<slug>`, fragment dropped (it is the
    // *final* slice, re-attached after the hops land).
    let mut current = format!("{}:{}", bound.r#type, bound.slug);

    // Walk every cross-doc relation hop past the role (the first hop is the role).
    for hop in path.hops.iter().skip(1) {
        match store.walk_edge(&current, hop.as_str()) {
            Some(to) => current = to,
            None => return Ok(None), // unset relation → absent, empty text.
        }
    }

    // The landing doc, sliced at the path's own `#fragment`.
    let landing = match &path.fragment {
        Some(fragment) => format!("{current}#{fragment}"),
        None => current,
    };
    let address = crate::address::Address::parse(&landing).map_err(|source| {
        Finding::blocking(
            "workflow-refs.malformed-data-value",
            format!("the dereferenced target `{landing}` is not a valid address: {source}"),
            Location::at(1, 1),
        )
    })?;
    store.read_slice(&address).map(Some)
}

/// Render `prose` as a Markdown blockquote — each line prefixed `> ` (a blank line
/// becomes a bare `>`), so a multi-line doc-slice is a multi-line blockquote
/// (`workflow-dialect.md` → Emitted format: "a multi-line doc-slice is a multi-line
/// blockquote"). A trailing newline on `prose` is dropped (the slice is the slot
/// bytes; the blockquote is the emitted form).
fn blockquote(prose: &str) -> String {
    prose
        .trim_end_matches('\n')
        .lines()
        .map(|line| {
            if line.is_empty() {
                ">".to_string()
            } else {
                format!("> {line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Emit a resolved bare `{{<path>}}` data-value as inline Reason text: a
/// [`Resolution::Scalar`]'s string (`{{task.intent}}`), or a bare path's
/// resolved **address** (`workflow-dialect.md` → Leaves: bare path = the
/// reference). An [`Resolution::Absent`] (declared-but-unbound role) emits an
/// **empty line** (empty-not-finding). Unlike [`emit_content`] there is no `> `
/// blockquote — the bare class is the reference/scalar, not the dereferenced
/// content.
fn emit_bare_data_value(
    path: &crate::data_value::Path,
    ctx: &crate::data_value::ComposeContext,
) -> Result<String, Finding> {
    use crate::data_value::Resolution;
    match path.resolve(ctx)? {
        Resolution::Scalar { value } => Ok(value),
        Resolution::Address { address } | Resolution::Content { address } => {
            Ok(address.to_string())
        }
        Resolution::Absent => Ok(String::new()),
        Resolution::Catalog { .. } | Resolution::Store { .. } | Resolution::Milestone { .. } => {
            Err(collection_not_lone())
        }
    }
}

/// Emit a `fan-out` step's `Spawn:` directive block — one
/// `` Spawn: `cd <worktree> && jigc workflow <run> --task <id>` `` line per id of
/// the resolved `over:` collection, in the collection's (already id-sorted) order,
/// joined by newlines (`workflow-dialect.md` → Emitted format, rule 4: the 5th
/// class). The `cd <worktree>` prefix directs each sub-agent into its own detached
/// worktree (M31 WF4), the shared [`crate::milestone::worktree_path`] convention.
///
/// `over` carries the verbatim marker placeholder text (e.g. `{{ milestone.tasks }}`);
/// its `{{…}}` braces are stripped and the inner data-value path resolved against
/// `ctx` to a [`Resolution::Milestone`] collection. `run` is the marker's
/// `workflow:<id>` ref; the `workflow:` prefix is stripped to the bare workflow id
/// that names the re-entry command (`jigc workflow <id> --task <sub>`) — the BARE
/// CLI payload; the adapter launch wrapper is a later increment
/// ([assistant-adapter.md] → Bind the spawn mechanism). An **empty** collection
/// emits the empty string — zero directives, never a finding (the
/// empty-vs-unresolvable stance, same as an empty `catalog`/`store`).
///
/// A non-collection `over:` resolution (the path resolves to a scalar/address rather
/// than a `Milestone` collection) is the `collection`-shaped misuse — surfaced as the
/// resolver's structural [`Finding`] rather than a silent empty emit.
fn emit_fan_out_spawns(
    over: &str,
    run: &str,
    ctx: &crate::data_value::ComposeContext,
) -> Result<String, Finding> {
    use crate::data_value::Resolution;
    let inner = parse_lone_placeholder(over.trim()).ok_or_else(|| {
        Finding::blocking(
            "workflow-refs.malformed-data-value",
            format!("fan-out `over:` `{over}` is not a lone `{{{{<path>}}}}` placeholder"),
            Location::at(1, 1),
        )
    })?;
    let path = parse_data_value(inner)?;
    let ids = match path.resolve(ctx)? {
        Resolution::Milestone { ids } => ids,
        _ => {
            return Err(Finding::blocking(
                "workflow-refs.malformed-data-value",
                format!(
                    "fan-out `over:` `{over}` must resolve to a collection (e.g. `{{{{milestone.tasks}}}}`)"
                ),
                Location::at(1, 1),
            ));
        }
    };
    // The re-entry command names the bare workflow id (the `workflow:` prefix stripped).
    let workflow = run.strip_prefix("workflow:").unwrap_or(run);
    let lines: Vec<String> = ids
        .iter()
        .map(|id| {
            // Each spawn directs its sub-agent into its own detached worktree first
            // (M31 WF4) via the shared `worktree_path` convention `render_spawn` also
            // uses, then runs the bare re-entry payload there.
            let worktree = crate::milestone::worktree_path(id);
            let cmd = format!(
                "cd {} && jigc workflow {workflow} --task {id}",
                worktree.display()
            );
            format!("Spawn: `{cmd}`")
        })
        .collect();
    Ok(lines.join("\n"))
}

/// Render the resolved `catalog` collection as the router's option list: one
/// `- <id> — <when>` line per entry, in fed order, joined by newlines (the T3
/// line shape, `workflow-dialect.md` → Workflow selection / Emitted format). An
/// **empty** catalog renders the empty string — emitted as empty text, never a
/// finding (the empty-vs-unresolvable contract, same stance as an unbound role).
fn render_catalog_options(entries: &[crate::result::CatalogEntry]) -> String {
    entries
        .iter()
        .map(|entry| format!("- {} — {}", entry.id, entry.when))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render the resolved `store.<doctype>` collection as a readable **Content list**:
/// one `> <address>` blockquote line per committed instance, in fed order, joined by
/// newlines (`workflow-dialect.md` → data-value roots: "a collection interpolated in
/// an ordinary step emits as a readable Content list"; `worked-examples.md` → flow 6
/// `locate-from-spec`). The element is the bare `<type>:<slug>` address (the title
/// hint flow 6 shows is a deferred elaboration — `DECISIONS.md` 2026-06-01). An
/// **empty** store renders the empty string — emitted as empty text, never a finding
/// (the empty-vs-unresolvable contract, same stance as an empty `catalog`).
fn render_store_list(entries: &[crate::address::Address]) -> String {
    entries
        .iter()
        .map(|address| format!("> {address}"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// If `trimmed` is a `<<author: {{<path>}}>>` directive, resolve the embedded
/// `{{<path>}}` to its address and return `<<author: <address>>` — the `<<…>>`
/// wrapper preserved, only the placeholder resolved (rule 3). Returns `Ok(None)`
/// when the line is not an author directive (so the caller falls through).
fn emit_author_line(
    trimmed: &str,
    ctx: &crate::data_value::ComposeContext,
) -> Result<Option<String>, Finding> {
    use crate::data_value::Resolution;
    let Some(inner) = trimmed
        .strip_prefix("<<author:")
        .and_then(|s| s.strip_suffix(">>"))
    else {
        return Ok(None);
    };
    let inner = inner.trim();
    let Some(placeholder) = parse_lone_placeholder(inner) else {
        return Ok(None);
    };
    let path = parse_data_value(placeholder)?;
    let address = match path.resolve(ctx)? {
        Resolution::Address { address } | Resolution::Content { address } => address.to_string(),
        Resolution::Scalar { value } => value,
        // An unbound role (a doc created in-task is not yet bound on this compose)
        // would otherwise emit `<<author: >>` — an empty target the agent must
        // guess into. Name the slot from the path's own role + `#fragment` (e.g.
        // `arch-doc#overview`) so the directive still says which slot to author;
        // a re-compose after the create fills in the resolved slug.
        Resolution::Absent => pending_author_address(&path),
        Resolution::Catalog { .. } | Resolution::Store { .. } | Resolution::Milestone { .. } => {
            return Err(collection_not_lone());
        }
    };
    Ok(Some(format!("<<author: {address}>>")))
}

/// The best-effort slot reference for an `<<author: …>>` directive whose role is
/// still **unbound** (the target doc is created in-task, so the bound address —
/// with its slug — is not knowable on this compose). Built from the path's own
/// `.relation` hops + `#fragment` (the `task.` root dropped): `task.arch-doc#overview`
/// → `arch-doc#overview`. Names the slot to author into rather than emitting an
/// empty target; the resolved slug arrives on a re-compose once the doc exists.
fn pending_author_address(path: &crate::data_value::Path) -> String {
    use crate::data_value::Relation;
    let role = path
        .hops
        .iter()
        .map(Relation::as_str)
        .collect::<Vec<_>>()
        .join(".");
    match &path.fragment {
        Some(fragment) => format!("{role}#{fragment}"),
        None => role,
    }
}

/// Parse a data-value path string into a [`Path`](crate::data_value::Path),
/// mapping a parse failure to a blocking `workflow-refs.malformed-data-value`
/// [`Finding`].
fn parse_data_value(text: &str) -> Result<crate::data_value::Path, Finding> {
    crate::data_value::Path::parse(text).map_err(|source| {
        Finding::blocking(
            "workflow-refs.malformed-data-value",
            format!("data-value path `{text}` is malformed: {source}"),
            Location::at(1, 1),
        )
    })
}

/// The blocking finding for a collection data-value (`{{catalog}}` /
/// `{{store.<doctype>}}`) used anywhere other than as a lone-placeholder list — as a
/// command-ref arg, inside a `> ` Content / `<<author: …>>` directive, or inline. A
/// collection only renders as a lone placeholder (handled before these emitters in
/// [`emit_line`]: `catalog` → the router's option list, `store` → a Content list);
/// any other position is a structural misuse, not a value.
fn collection_not_lone() -> Finding {
    Finding::blocking(
        "workflow-refs.collection-not-lone",
        "a `{{catalog}}` / `{{store.<doctype>}}` collection renders only as a lone \
         placeholder list — it cannot be used as a command arg, content, or inline value"
            .to_owned(),
        Location::at(1, 1),
    )
}

/// If `trimmed` is a lone `{{ … }}` placeholder, return its inner (trimmed)
/// text; else `None`. A "lone" placeholder is the whole (trimmed) line — an
/// inline `{{…}}` inside prose is not a class line.
fn parse_lone_placeholder(trimmed: &str) -> Option<&str> {
    trimmed
        .strip_prefix("{{")?
        .strip_suffix("}}")
        .map(str::trim)
}

/// If `trimmed` is a lone `{{ cli.<id> }}` placeholder, return `<id>`; else
/// `None`. The id is the bare command-ref id (no further dots/whitespace).
fn parse_cli_placeholder(trimmed: &str) -> Option<&str> {
    let inner = parse_lone_placeholder(trimmed)?;
    let id = inner.strip_prefix("cli.")?.trim();
    if id.is_empty() || id.contains(char::is_whitespace) {
        return None;
    }
    Some(id)
}

/// A parsed step definition: its frozen id (the resource id the pack assigns from
/// the filename stem) and its verbatim prompt body.
///
/// A step is **one file** reusing the document-instance shape — a Markdown body
/// under an optional `---`-fenced front-matter block (`workflow-dialect.md` →
/// On-disk definition format). The **id is the filename**, never a front-matter
/// field; the **body is the prompt** (instruction prose with `{{placeholders}}`,
/// resolved later in composition). A *plain* step needs no front-matter at all —
/// it is just a prompt body. The body is carried **byte-for-byte verbatim** (the
/// post-fence remainder when a front-matter block is present, else the whole
/// file); placeholder resolution runs over it later, never at load time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StepDef {
    /// The frozen step id — the resource id the pack assigns (the filename stem).
    pub id: String,
    /// The verbatim prompt body (post-front-matter remainder, byte-for-byte).
    pub body: String,
    /// The step kind, parsed from the front-matter markers. A plain step (no
    /// `fan-out:`/`join:`/`checkpoint:` marker) is [`StepKind::Plain`].
    pub kind: StepKind,
}

/// A step's kind, parsed from its front-matter markers (`workflow-dialect.md` →
/// On-disk definition format). A plain step (no marker, or no front-matter at
/// all) is [`StepKind::Plain`]; a `fan-out:` marker is [`StepKind::FanOut`]; a
/// `join: {}` marker is [`StepKind::Join`]; a `checkpoint:` marker is
/// [`StepKind::Checkpoint`] (M15). The markers are **parsed and honored**, not
/// stripped — pre-M8 `load_step_def` discarded all step front-matter, so a
/// `fan-out:` marker silently composed as inert prose (`DECISIONS.md`
/// 2026-06-04). The kind reaches [`ComposedStep`] so the emit (`Spawn:`) and
/// `workflow-refs` passes can read it without re-parsing.
///
/// Internally tagged (like [`CommandArg`]) so the serde projection is an
/// unambiguous golden discriminant.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StepKind {
    /// A plain step: instruction prose, no repetition marker.
    Plain,
    /// A `fan-out` step: spawns the `run:` sub-workflow once per item of the
    /// `over:` collection. `over` carries the verbatim data-value path text (e.g.
    /// `{{ milestone.tasks }}`); `run` the `workflow:<id>` string. Both are
    /// required — resolution runs later, in composition.
    FanOut {
        /// The verbatim `over:` data-value path text (the fan-out list-source).
        over: String,
        /// The `run:` sub-workflow ref (`workflow:<id>`) each spawn runs.
        run: String,
    },
    /// A `join` step: the barrier that merges the fanned sub-task areas by
    /// task-id order. Carries no parameters (`join: {}`).
    Join,
    /// A `checkpoint` step (M15): a structural human-gate that halts execution to
    /// surface a decision. `reason` is a stable slug naming *which* halt point this
    /// is (`new-fork-at-plan`, …) — a label for the human/orchestrator, not a CLI
    /// condition. Required and non-empty. There is **no pairing rule** (unlike
    /// `fan-out`↔`join`): a checkpoint is standalone, valid anywhere in a body
    /// (`workflow-dialect.md` → The checkpoint step kind).
    Checkpoint {
        /// The `reason:` slug naming this halt point.
        reason: String,
    },
}

/// The config-family front-matter of a step definition, as YAML — the optional
/// `fan-out:` / `join:` / `checkpoint:` markers. All default absent (a plain step
/// needs no front-matter); the kind validation happens in [`load_step_def`].
#[derive(Deserialize)]
struct StepFrontMatter {
    #[serde(rename = "fan-out", default)]
    fan_out: Option<FanOutMarker>,
    #[serde(default)]
    join: Option<serde_yaml_ng::Value>,
    #[serde(default)]
    checkpoint: Option<CheckpointMarker>,
}

/// The `fan-out:` marker body: `over` (the list-source path) and `run` (the
/// sub-workflow ref). Both required — a missing key is a malformed kind.
#[derive(Deserialize)]
struct FanOutMarker {
    over: Option<String>,
    run: Option<String>,
}

/// The `checkpoint:` marker body: `reason` (the halt-point slug). Required and
/// non-empty — a missing/blank reason is a malformed kind (M15).
#[derive(Deserialize)]
struct CheckpointMarker {
    reason: Option<String>,
}

/// Parse one step definition's raw bytes into a [`StepDef`] under `id`.
///
/// The `id` is the resource id the caller already holds (the pack's filename stem,
/// per `packsource.rs` → `PackResourceKind::Steps`) — never read from the file.
/// The body is the prompt, carried **verbatim**: if the file opens with a
/// `---`-fenced front-matter block (config-family YAML — a step's optional config,
/// e.g. a `fan-out` marker), the body is the **post-fence remainder, byte-for-byte**;
/// otherwise the body is the **whole file, byte-for-byte** (a plain step needs no
/// front-matter). The only failure is non-UTF-8 bytes — a blocking conformance
/// [`Finding`] (the settled block envelope, `DECISIONS.md` 2026-05-31). The
/// front-matter is *not* parsed here: this task's single concern is the verbatim
/// id+body split; consuming a step's config lands when a step kind needs it.
pub fn load_step_def(id: impl Into<String>, bytes: &[u8]) -> Result<StepDef, Finding> {
    let text = std::str::from_utf8(bytes).map_err(|_| {
        Finding::blocking(
            "workflow-refs.not-utf8",
            "step definition is not valid UTF-8",
            Location::at(1, 1),
        )
    })?;
    let (front, body) = match split_front_matter(text) {
        // The front-matter (config-family YAML) sits on line 2, after the opening
        // `---` fence — where a malformed-kind finding points.
        Some((front, body)) => (Some(front), body),
        // No fence → a plain step, no front-matter to parse.
        None => (None, text),
    };
    let kind = match front {
        Some(front) => parse_step_kind(front)?,
        None => StepKind::Plain,
    };
    Ok(StepDef {
        id: id.into(),
        body: body.to_owned(),
        kind,
    })
}

/// Parse a step's front-matter YAML into its [`StepKind`], honoring the
/// `fan-out:` / `join:` / `checkpoint:` markers (`workflow-dialect.md` → On-disk
/// definition format). A step is **exactly one kind**: a blocking, **located**
/// `workflow-refs.step-kind-malformed` [`Finding`] (pointing at the front-matter,
/// line 2) rejects: malformed YAML, **more than one** marker present (the
/// three-marker mutual-exclusivity check, M15), a `fan-out` missing `over` or
/// `run` (both required), or a `checkpoint` missing/blank `reason`. Front-matter
/// that declares no marker — any other config a step might carry — is
/// [`StepKind::Plain`].
fn parse_step_kind(front: &str) -> Result<StepKind, Finding> {
    // The front-matter begins on line 2 (line 1 is the opening `---` fence).
    let malformed = |msg: &str| {
        Finding::blocking(
            "workflow-refs.step-kind-malformed",
            format!("step front-matter declares a malformed step kind: {msg}"),
            Location::at(2, 1),
        )
    };
    let meta: StepFrontMatter = serde_yaml_ng::from_str(front)
        .map_err(|source| malformed(&format!("not valid config-family YAML: {source}")))?;

    // Mutual exclusivity across all three markers: a step is exactly one kind.
    let marker_count =
        meta.fan_out.is_some() as u8 + meta.join.is_some() as u8 + meta.checkpoint.is_some() as u8;
    if marker_count > 1 {
        return Err(malformed(
            "a step is one kind, but more than one of `fan-out:` / `join:` / `checkpoint:` is present",
        ));
    }

    if let Some(fan_out) = meta.fan_out {
        let over = fan_out
            .over
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| malformed("`fan-out` requires a non-empty `over:`"))?;
        let run = fan_out
            .run
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| malformed("`fan-out` requires a non-empty `run:`"))?;
        return Ok(StepKind::FanOut { over, run });
    }
    if meta.join.is_some() {
        return Ok(StepKind::Join);
    }
    if let Some(checkpoint) = meta.checkpoint {
        let reason = checkpoint
            .reason
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| malformed("`checkpoint` requires a non-empty `reason:`"))?;
        return Ok(StepKind::Checkpoint { reason });
    }
    Ok(StepKind::Plain)
}

/// Parse one workflow definition's raw bytes into a [`WorkflowDef`].
///
/// Slices the `---`-fenced front-matter (config-family YAML) and the body, parses
/// the metadata, and reads the include-only body: each non-blank, non-comment
/// line must be a `{{ include: step:<id> }}` placeholder; blank lines and HTML
/// comment lines (`<!-- … -->`) are dropped. Any other top-level line — prose, an
/// ATX heading, a non-include placeholder — is a blocking conformance
/// [`Finding`] (`workflow-refs.body-include-only`, `workflow-dialect.md` →
/// Workflow body is include-only). The finding is the settled block envelope, not
/// a new error type (`DECISIONS.md` 2026-05-31 → block payload).
pub fn load_workflow_def(bytes: &[u8]) -> Result<WorkflowDef, Finding> {
    let text = std::str::from_utf8(bytes).map_err(|_| {
        Finding::blocking(
            "workflow-refs.not-utf8",
            "workflow definition is not valid UTF-8",
            Location::at(1, 1),
        )
    })?;
    let (front, body) = split_front_matter(text).ok_or_else(|| {
        Finding::blocking(
            "workflow-refs.missing-front-matter",
            "workflow definition has no `---`-fenced front-matter block",
            Location::at(1, 1),
        )
    })?;
    let meta: WorkflowFrontMatter = serde_yaml_ng::from_str(front).map_err(|source| {
        Finding::blocking(
            "workflow-refs.malformed-front-matter",
            format!("workflow front-matter is malformed config-family YAML: {source}"),
            Location::at(1, 1),
        )
    })?;

    // The body begins one line after the front-matter's closing `---` fence; that
    // closing fence sits on the line *after* the front-matter lines. Count the
    // lines consumed up to the body so findings point at real source lines.
    let body_start_line = text[..text.len() - body.len()].lines().count() + 1;
    let includes = parse_include_only_body(body, body_start_line)?;

    Ok(WorkflowDef {
        when: meta.when.filter(|w| !w.trim().is_empty()),
        description: meta.description,
        usage: meta.usage,
        creates_task: meta.creates_task,
        selectable: meta.selectable,
        allows_create: meta.allows_create,
        reads: meta.reads,
        includes,
    })
}

/// Split `text` into `(front_matter_yaml, body)` at the leading `---` fence.
///
/// Reuses the same fence recognition as [`crate::catalog::front_matter`] (one
/// recognizer), but also returns the body that follows the closing fence — what
/// composition needs that the catalog does not.
fn split_front_matter(text: &str) -> Option<(&str, &str)> {
    let front = crate::catalog::front_matter(text)?;
    // `front` is the slice strictly between the fences; the closing `---` fence
    // starts right after it, within the post-opening-fence remainder.
    let rest = text.strip_prefix("---\n")?;
    let after_close = &rest[front.len()..];
    let body = after_close
        .strip_prefix("---\n")
        .or_else(|| after_close.strip_prefix("---"))
        .unwrap_or(after_close);
    Some((front, body))
}

/// Read the include-only body into an ordered list of step ids.
///
/// Drops blank lines and HTML-comment lines; accepts `{{ include: step:<id> }}`
/// lines (whitespace-tolerant inside the braces); rejects anything else as a
/// blocking `body-include-only` finding located at the offending line.
fn parse_include_only_body(body: &str, body_start_line: usize) -> Result<Vec<String>, Finding> {
    let mut includes = Vec::new();
    for (offset, raw) in body.lines().enumerate() {
        let line_no = body_start_line + offset;
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("<!--") && line.ends_with("-->") {
            continue;
        }
        match parse_include_line(line) {
            Some(id) => includes.push(id),
            None => {
                return Err(Finding::blocking(
                    "workflow-refs.body-include-only",
                    format!(
                        "workflow body line is not an `{{{{ include: step:<id> }}}}`, blank, or comment: `{line}`"
                    ),
                    Location::at(line_no, 1),
                ));
            }
        }
    }
    Ok(includes)
}

/// Match one `{{ include: step:<id> }}` line and return `<id>`, or `None` if the
/// line is not a well-formed top-level include placeholder.
fn parse_include_line(line: &str) -> Option<String> {
    let inner = line.strip_prefix("{{")?.strip_suffix("}}")?.trim();
    let target = inner.strip_prefix("include:")?.trim();
    let id = target.strip_prefix("step:")?.trim();
    if id.is_empty() || id.contains(char::is_whitespace) {
        return None;
    }
    Some(id.to_owned())
}

/// Supplies a step definition by its resolved id — the engine's view of the
/// cascade-resolved step file set.
///
/// Per the engine invariant (CLI locates cascade layers, engine resolves —
/// `VISION.md` principle #4 / `CLAUDE.md`), the *frontend* maps a step id to a
/// file and loads its bytes into a [`StepDef`]. The engine consumes that mapping
/// through this trait, so [`expand_includes`] stays a pure function of
/// `(WorkflowDef, StepSource)` — feed-layers-in / assert-results-out. A `None` is
/// a *dangling* include id: the include names a step no layer provides.
///
/// The phase-2 by-id shadowing surface ([`crate::cascade::Resolved::file_owner`])
/// is *resolved* but **not yet consulted on this read path**: the live frontend
/// (`PackStepSource` in `cli`) reads step bytes from the pack-default layer
/// only. Routing step ids through `file_owner` so a project-layer file shadows a
/// pack step lands in a later increment (`overrides.md` phase 2 by-id shadowing).
pub trait StepSource {
    /// Resolve a step id to its [`StepDef`], or `None` if no layer provides it.
    fn step(&self, id: &str) -> Option<StepDef>;

    /// Scope subsequent pack-default step reads to the **origin pack** of the named
    /// composing workflow — the constituent pack that defines `workflow_id`'s
    /// top-level id, so every `{{include: step:X}}` this workflow expands resolves
    /// against ITS OWN pack rather than the precedence-winner's divergent step
    /// (`multi-pack.md` → Pack-local body-reference resolution → Steps). The
    /// store-sweep loop calls this once per enumerated workflow, before that
    /// workflow's refs are walked, mirroring `jigc start`'s per-compose scoping.
    ///
    /// **Default: a no-op.** A single-pack / origin-unaware source (every engine-test
    /// source) already reads against the only pack, so scoping is inert — the
    /// store-sweep path stays byte-identical there. Only the CLI's cascade source
    /// overrides it.
    fn scope_to_workflow(&self, _workflow_id: &str) {}
}

/// One leaf of the flattened composition: a step's id and its body, placeholders
/// **still unresolved** (phase 7 output, before phase 8 placeholder resolution).
///
/// A step whose body mixes prose with `{{include: …}}` lines expands **in place**
/// (`workflow-dialect.md` → Composition: "expanded recursively in place";
/// `worked-examples.md` 3a phase-7): the body splits at each include line into
/// ordered segments — a prose run becomes one leaf under *this* step's id, and an
/// include is replaced by the recursively-expanded child subtree at its position.
/// So a `before / {{include}} / after` body yields three leaves —
/// `[parent("before"), child…, parent("after")]` — preserving prose order around
/// the splice. The first leaf of each included step is a step boundary; the
/// continuation prose segments after an in-body include carry [`continues`] so the
/// emit join concatenates them to the prior segment with no injected blank line,
/// keeping the spliced body contiguous. For a *plain* step (no nested includes —
/// every MVP step), the body is the verbatim step body unchanged in a single leaf.
///
/// [`continues`]: ComposedStep::continues
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ComposedStep {
    /// The frozen step id this leaf came from.
    pub id: String,
    /// The step's prompt body, nested include lines removed, placeholders unresolved.
    pub body: String,
    /// `true` when this leaf is a *continuation* of the same on-disk step body
    /// across an in-place include splice (a prose segment after a nested include,
    /// or a deeper continuation) — the emit join concatenates it directly to the
    /// prior leaf instead of inserting the inter-step blank line, so the spliced
    /// body reads as one contiguous step. `false` for a step boundary (the first
    /// leaf of an included step).
    pub continues: bool,
    /// The step kind, carried from the originating [`StepDef`] (the doc-elaboration
    /// pin, `DECISIONS.md` 2026-06-04) so the emit (`Spawn:`) and `workflow-refs`
    /// passes read it without re-parsing front-matter. Only the **boundary** leaf
    /// (a step's first segment) inherits the step's kind; every **continuation**
    /// leaf ([`continues`] true) is [`StepKind::Plain`].
    ///
    /// [`continues`]: ComposedStep::continues
    pub kind: StepKind,
}

/// The flat, ordered composition tree of a workflow: its step bodies in
/// pre-order include traversal, placeholders unresolved.
///
/// This is the phase-7 output (`overrides.md` → Resolution algorithm). Phase 8
/// (placeholder resolution) and phase 9 (emit) consume it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Composition {
    /// The flattened step leaves, in pre-order include order.
    pub steps: Vec<ComposedStep>,
}

impl Composition {
    /// The flattened step ids, in order — the composition order a golden pins.
    pub fn step_ids(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.id.as_str()).collect()
    }
}

/// Apply a layer-ordered run of `structural-op` deltas to a workflow's include
/// id list — **phase 4** of `overrides.md` → Resolution algorithm, run *before*
/// include expansion (phase 7). A pure transform of the id `Vec`: same
/// `(includes, deltas)` in → same id list out (no I/O, no cascade consulted).
///
/// Deltas apply in fed order — the within-layer manifest order chained across
/// layers (pack → team → project), so an earlier delta's result is the later
/// delta's input (`overrides.md` → Within-layer manifest order). Each kind:
///
/// - [`StructuralDelta::Insert`] — splice the new step id at the [`Anchor::After`]
///   / [`Anchor::Before`] anchor's position.
/// - [`StructuralDelta::Replace`] — swap the id at the [`Anchor::At`] position for
///   the replacement step id (another step id, never inline content).
/// - [`StructuralDelta::Remove`] — drop the id at the [`Anchor::At`] position.
///
/// A delta whose anchor / target id is **absent** from the current list is an
/// **orphaned** `workflow-refs.structural-anchor-resolves` blocking [`Finding`]
/// (`overrides.md` → Within-layer manifest order: "a later delta targeting a
/// now-removed id surfaces an `orphaned` finding"). This precedes phase 7, so a
/// dangling *inserted* id (one no layer provides a file for) is caught later by
/// [`expand_includes`]'s `include-resolves` check, not here — phase 4 only
/// transforms the id list.
pub fn apply_structural_deltas(
    includes: &[String],
    deltas: &[StructuralDelta],
) -> Result<Vec<String>, Finding> {
    let mut ids: Vec<String> = includes.to_vec();
    for delta in deltas {
        match delta {
            StructuralDelta::Insert { target, step } => {
                let (anchor_id, offset) = match &target.anchor {
                    Anchor::After(id) => (id, 1),
                    Anchor::Before(id) => (id, 0),
                    Anchor::At(id) => (id, 0),
                };
                let pos = anchor_position(&ids, anchor_id)?;
                ids.insert(pos + offset, step.clone());
            }
            StructuralDelta::Replace { target, step } => {
                let pos = anchor_position(&ids, at_id(&target.anchor))?;
                ids[pos] = step.clone();
            }
            StructuralDelta::Remove { target } => {
                let pos = anchor_position(&ids, at_id(&target.anchor))?;
                ids.remove(pos);
            }
        }
    }
    Ok(ids)
}

/// The step id of an [`Anchor::At`] (replace / remove) target, falling back to
/// the carried id for the never-constructed-here `After` / `Before` cases so the
/// helper is total without an unreachable.
fn at_id(anchor: &Anchor) -> &str {
    match anchor {
        Anchor::At(id) | Anchor::After(id) | Anchor::Before(id) => id,
    }
}

/// The index of `anchor_id` in the current include list, or an **orphaned**
/// `workflow-refs.structural-anchor-resolves` blocking [`Finding`] when the
/// anchor / target id is absent (`overrides.md` → Within-layer manifest order).
///
/// The finding carries its repair **route**: an orphaned structural delta routes
/// the human to remove or re-target it (`validation.md` → route: `run-command`;
/// `overrides.md` → "`orphaned` → a `run-command` route to remove or re-target the
/// delta"). The block is located *and* routed — the settled block envelope keeps
/// both (`finding.rs`).
fn anchor_position(ids: &[String], anchor_id: &str) -> Result<usize, Finding> {
    ids.iter().position(|id| id == anchor_id).ok_or_else(|| {
        Finding::graded(
            crate::finding::Severity::Blocking,
            "workflow-refs.structural-anchor-resolves",
            format!(
                "structural-op anchor `{anchor_id}` resolves to no entry in the include list (orphaned)"
            ),
            Some(Location::at(1, 1)),
            Some(format!(
                "remove or re-target the structural-op anchored at `{anchor_id}` with `jigc config` (the anchor it names is not in the include list)"
            )),
        )
    })
}

/// Build the structural slice of the `--explain` resolution tree — layers 1–2 of
/// the output contract (`design/workflow-dialect.md` → `--explain` output
/// contract): the workflow's `overrides applied: N` provenance and the resolved
/// include-expansion tree, each step tagged with the cascade layer that owns its
/// file and any `replace-step` annotation, in **post-phase-4 composed order**.
///
/// A pure derivation over the inputs `compose` already holds — the pack
/// `includes` id list, the `scoped` `structural-op` deltas for *this* workflow,
/// the `workflow_layer` the definition file resolved to, and a `file_owner`
/// lookup ([`crate::cascade::Resolved::file_owner`]) closed over by the caller. It
/// replays phase 4 with the **same** position semantics as
/// [`apply_structural_deltas`] (so the built step order equals the composed
/// include order), tracking which slot each `replace` swapped so the renderer can
/// emit `← replaces <id> at position N`. No re-resolution, no I/O.
///
/// `overrides_applied` is the total applied-override count — `scoped` structural
/// deltas (that mutated this workflow's include list) **plus** the scalar-key
/// overrides — matching the agent-text `overrides applied: N` header. The
/// `rejected_demotions` ride layer 1 distinctly from the applied overrides
/// (`design/workflow-dialect.md` → `--explain` output contract): a below-floor
/// `scalar-set` the cascade soft-rejected (logged, not applied), so it is **not**
/// folded into `overrides_applied`. A delta whose anchor / target id is
/// absent surfaces the same **orphaned** blocking [`Finding`] as phase 4
/// ([`apply_structural_deltas`]); a layer with no file for a resolved id is a
/// dangling include the compose path reports, so the tree defaults its layer to
/// pack-default — the live caller only builds the tree on a clean compose.
pub fn build_resolution_tree(
    workflow: &str,
    workflow_layer: crate::cascade::LayerKind,
    includes: &[String],
    scoped: &[StructuralDelta],
    scalar_overrides: Vec<crate::result::ScalarOverride>,
    rejected_demotions: Vec<crate::result::RejectedDemotion>,
    file_owner: impl Fn(&str) -> Option<crate::cascade::LayerKind>,
) -> Result<crate::result::ResolutionTree, Finding> {
    use crate::result::{Replacement, ResolvedStep};

    // Replay phase 4 carrying a parallel provenance slot per id: `Some(replaced)`
    // marks a slot a `replace` swapped, so the post-phase-4 position is the
    // annotation's position. Mirrors `apply_structural_deltas` exactly.
    let mut ids: Vec<String> = includes.to_vec();
    let mut replaced: Vec<Option<String>> = vec![None; ids.len()];
    for delta in scoped {
        match delta {
            StructuralDelta::Insert { target, step } => {
                let (anchor_id, offset) = match &target.anchor {
                    Anchor::After(id) => (id, 1),
                    Anchor::Before(id) | Anchor::At(id) => (id, 0),
                };
                let pos = anchor_position(&ids, anchor_id)?;
                ids.insert(pos + offset, step.clone());
                replaced.insert(pos + offset, None);
            }
            StructuralDelta::Replace { target, step } => {
                let pos = anchor_position(&ids, at_id(&target.anchor))?;
                replaced[pos] = Some(ids[pos].clone());
                ids[pos] = step.clone();
            }
            StructuralDelta::Remove { target } => {
                let pos = anchor_position(&ids, at_id(&target.anchor))?;
                ids.remove(pos);
                replaced.remove(pos);
            }
        }
    }

    let steps = ids
        .iter()
        .zip(replaced)
        .enumerate()
        .map(|(idx, (id, replaced))| ResolvedStep {
            id: id.clone(),
            layer: file_owner(id).unwrap_or(crate::cascade::LayerKind::PackDefault),
            replaces: replaced.map(|replaced| Replacement {
                replaced,
                position: idx + 1,
            }),
        })
        .collect();

    // The header total counts *every* applied override — structural deltas plus
    // scalar-key overrides — so the JSON `overrides_applied` field matches the
    // agent-text `overrides applied: N` line exactly (the structural-only count
    // is recoverable as `overrides_applied - scalar_overrides.len()`).
    let overrides_applied = scoped.len() + scalar_overrides.len();
    Ok(crate::result::ResolutionTree::new(
        workflow,
        workflow_layer,
        overrides_applied,
        scalar_overrides,
        // The soft-rejected demotions ride layer 1 distinctly from the applied
        // overrides — logged, not applied, so never folded into `overrides_applied`.
        rejected_demotions,
        steps,
    ))
}

/// The cascade-resolved slot-fill content for a composition, keyed by
/// `(step_id, fill_id)` — the input to the **phase-5** fill-application pass
/// ([`apply_slot_fills`]).
///
/// One entry per `{{fill:<id>}}` point a `slot-fill` delta fills, the value being
/// the **higher-layer-wins** resolved content body (`overrides.md` → The
/// `{{fill:}}` placeholder: "Higher layer wins for the same `<fill-id>`"). The
/// frontend (CLI) reads each fill's native file bytes and folds the cascade into
/// this map (project over team over pack); the engine consumes it as a pure input,
/// so phase 5 stays a function of `(body, fills)` — fed-in / asserted-out. A
/// `(step_id, fill_id)` absent from the map is an **unfilled** point: it resolves
/// to the pack's default body (empty in M4), never a finding (an absent extension
/// point is the common case).
pub type ResolvedFills = std::collections::BTreeMap<(String, String), String>;

/// Apply the cascade's `slot-fill` content to one step body — **phase 5** of
/// `overrides.md` → Resolution algorithm, run *before* include expansion (phase 7)
/// and placeholder resolution (phase 8).
///
/// Each `{{fill:<id>}}` token in `body` — **anywhere in a line**, recognized by
/// [`next_fill_token`] — is replaced by the content `fills` carries for
/// `(step_id, <id>)`, or — when no `slot-fill` fills that point — the pack's
/// **default body** (empty in M4, never a finding: an unfilled extension point is
/// the common case, `overrides.md` → The `{{fill:}}` placeholder). A **lone-line**
/// token whose content is empty collapses the line to nothing (no blank line left
/// behind); a mid-line token whose content is empty drops just the token. Non-fill
/// text passes through byte-for-byte.
///
/// The pass **does not re-run**: a `{{fill:}}` *inside* the applied content is left
/// intact, surviving to phase 8 where `workflow-refs` flags it as a blocking
/// survivor (the no-nested-fills rule — `overrides.md`: "phase 5 does not re-run").
/// Substitution scans the *original* line only and never re-scans the spliced
/// content, so a nested `{{fill:}}` — lone-line or mid-line — survives intact.
/// Likewise the applied content's own `{{include:}}` / `{{cli.…}}` / `{{@…}}` are
/// left untouched, resolving in the later phases exactly as if the pack had written
/// them inline.
///
/// A pure function of `(step_id, body, fills)` — same inputs always yield the same
/// text (the determinism boundary; no I/O, clock, or LLM). The `Result` carries a
/// [`Finding`] for symmetry with the other phase passes; M4's pass has no failure
/// of its own (orphan/survivor detection is the `workflow-refs` probe's job, T3),
/// so it is presently always `Ok`.
pub fn apply_slot_fills(
    step_id: &str,
    body: &str,
    fills: &ResolvedFills,
) -> Result<String, Finding> {
    let resolve = |id: &str| -> &str {
        fills
            .get(&(step_id.to_owned(), id.to_owned()))
            .map(String::as_str)
            .unwrap_or("")
    };
    let trailing_newline = body.ends_with('\n');
    let mut out_lines = Vec::new();
    for line in body.lines() {
        // A lone-line `{{fill: <id>}}` keeps the byte-identical splice-and-collapse
        // semantics: the content's lines replace the fill line, an empty default
        // leaving no blank line behind.
        if let Some(fill_id) = fill_id_of(line.trim()) {
            for content_line in resolve(fill_id).lines() {
                out_lines.push(content_line.to_owned());
            }
            continue;
        }
        // Otherwise substitute every mid-line `{{fill:<id>}}` token in place,
        // scanning the *original* line only (the spliced content is never re-scanned,
        // so a nested fill survives). A line with no fill token passes through
        // byte-for-byte. Multi-line content splices in at the token's position.
        let mut rewritten = String::new();
        let mut cursor = 0;
        while let Some((range, id)) = next_fill_token(&line[cursor..]) {
            rewritten.push_str(&line[cursor..cursor + range.start]);
            rewritten.push_str(resolve(id));
            cursor += range.end;
        }
        if cursor == 0 {
            out_lines.push(line.to_owned());
        } else {
            rewritten.push_str(&line[cursor..]);
            for rewritten_line in rewritten.split('\n') {
                out_lines.push(rewritten_line.to_owned());
            }
        }
    }
    let mut applied = out_lines.join("\n");
    if trailing_newline {
        applied.push('\n');
    }
    Ok(applied)
}

/// Every `{{fill:<id>}}` point a step `body` declares — **token-anywhere**, in
/// occurrence order — the inline `{{fill:}}` recognizer ([`next_fill_token`]) swept
/// over the whole body, exposed for the **write-time** `config fill` checks
/// (`overrides.md` → the `jigc config` verbs: "the `{{fill:<fill-id>}}` point exists
/// in the resolved step body"; The `{{fill:}}` placeholder: "rejects fill content
/// containing `{{fill:}}` at write time").
///
/// Two uses, one recognizer: the verb checks the target fill-id is among the
/// resolved step body's points (else the orphan would land at resolution), and
/// rejects fill content whose own body declares any `{{fill:}}` — including a
/// **mid-line** one (the no-nested rule, since phase 5 does not re-run; a mid-line
/// nested fill would otherwise leak the literal token into agent output). The same
/// token discipline the phase-5 pass and the `slot-fill-orphan` / `fill-survivor`
/// checks use, so all paths agree on what counts as a fill point. Pure: no I/O, no
/// cascade consulted.
pub fn fill_ids_in(body: &str) -> Vec<&str> {
    let mut ids = Vec::new();
    let mut base = 0;
    while let Some((range, id)) = next_fill_token(&body[base..]) {
        ids.push(id);
        base += range.end;
    }
    ids
}

/// If `trimmed` is a lone `{{ fill: <id> }}` placeholder, return `<id>`; else
/// `None`. The id is the bare fill-id (no further whitespace) — the
/// fourth read-path placeholder kind (`workflow-dialect.md` → Leaves).
///
/// Distinct from [`parse_cli_placeholder`] (`cli.<id>`) and [`parse_include_line`]
/// (`include: step:<id>`): a `{{fill:}}` is its own leaf kind. This recognizer is
/// the **lone-line** form (the whole trimmed line is the placeholder); the
/// token-anywhere form is [`next_fill_token`].
fn fill_id_of(trimmed: &str) -> Option<&str> {
    let inner = parse_lone_placeholder(trimmed)?;
    fill_id_of_inner(inner)
}

/// The fill-id of an already-extracted placeholder inner (`fill: <id>`), or `None`.
/// Shared by the lone-line [`fill_id_of`] and the token scanner [`next_fill_token`]
/// so both agree on what is a well-formed `{{fill:}}`.
fn fill_id_of_inner(inner: &str) -> Option<&str> {
    let id = inner.trim().strip_prefix("fill:")?.trim();
    if id.is_empty() || id.contains(char::is_whitespace) {
        return None;
    }
    Some(id)
}

/// Find the first `{{fill:<id>}}` token anywhere in `s`, returning its byte range
/// (`{{`…`}}` inclusive) and the bare `<id>`. A `{{fill:}}` is recognized **inline**,
/// not only as a lone line (`overrides.md` → The `{{fill:}}` placeholder: "an inline
/// `{{fill:<id>}}` placeholder") — symmetric with how the substitution / no-nested /
/// survivor checks treat it. Only `{{fill:…}}` matches: a `{{…}}` whose inner is not
/// a well-formed `fill:<id>` (a command-ref, data-value, include, or malformed fill)
/// is **skipped**, the scan continuing after its `{{`, so those placeholders are left
/// for their own later phases.
fn next_fill_token(s: &str) -> Option<(std::ops::Range<usize>, &str)> {
    let mut search_from = 0;
    while let Some(rel_open) = s[search_from..].find("{{") {
        let open = search_from + rel_open;
        // The matching `}}` is the first one after this `{{` (fill ids carry no
        // braces, so no nesting to balance).
        if let Some(rel_close) = s[open + 2..].find("}}") {
            let close = open + 2 + rel_close;
            let inner = &s[open + 2..close];
            if let Some(id) = fill_id_of_inner(inner) {
                return Some((open..close + 2, id));
            }
        }
        // Not a fill token (or no closer): resume just past this `{{`.
        search_from = open + 2;
    }
    None
}

/// Expand a workflow's include list into a flat, ordered [`Composition`] —
/// phases 6 (cycle detection) and 7 (include expansion) of `overrides.md` →
/// Resolution algorithm, for the workflow-only path.
///
/// Walks `def.includes` in order; each id resolves to a [`StepDef`] via `source`
/// (the cascade-resolved file set — highest-precedence layer wins, already done
/// upstream). Each step is expanded **recursively**: a `{{include: step:<id>}}`
/// line inside a step body pulls that sub-step in at that point, flattened in
/// pre-order. The result is a flat list of step bodies, placeholders still
/// unresolved (phase 8's job).
///
/// Phase 6 runs **interleaved** with the walk: the active include path is
/// tracked, and re-entering an id already on it is a **cycle** — a blocking
/// `workflow-refs.include-cycle-absent` [`Finding`] (`overrides.md`: "cycles are
/// never broken automatically"). An include id `source` cannot resolve is a
/// **dangling** include — a blocking `workflow-refs.include-resolves`
/// [`Finding`]. Both reuse the settled block envelope (`DECISIONS.md`
/// 2026-05-31).
///
/// Expansion is a **pure function** of `(def, source)` — same resolved cascade
/// in → same composition out (the determinism boundary; no I/O, clock, or LLM).
pub fn expand_includes(def: &WorkflowDef, source: &dyn StepSource) -> Result<Composition, Finding> {
    let mut steps = Vec::new();
    let mut on_path = Vec::new();
    for id in &def.includes {
        expand_step(id, source, &mut steps, &mut on_path, true)?;
    }
    Ok(Composition { steps })
}

/// Recursively expand one step id into `out`, **in place**, with cycle detection.
///
/// `on_path` is the active include path (the DFS stack). If `id` is already on
/// it, this is an include cycle. Otherwise the step resolves and its body is split
/// into ordered [`BodySegment`]s at each `{{include:}}` line: a prose run is
/// emitted as a [`ComposedStep`] leaf under *this* `id`, and an include is
/// recursively expanded at its position — so the child subtree lands **between**
/// the surrounding prose, not appended after it (`workflow-dialect.md` →
/// Composition: "expanded recursively in place"; `worked-examples.md` 3a phase-7).
///
/// `boundary` is `true` when the first leaf this call emits begins a new on-disk
/// step (a top-level workflow include, or a nested include that is itself the
/// first segment of its parent) — that leaf is a step boundary the emit join
/// separates with a blank line. Every later leaf of the *same* on-disk body — a
/// prose segment after an in-body include, or a child include not in first
/// position — is a **continuation** (`continues = true`), concatenated with no
/// injected blank line so the spliced body stays contiguous. A plain step (no
/// nested includes — every MVP step) emits exactly one leaf, its body verbatim.
fn expand_step(
    id: &str,
    source: &dyn StepSource,
    out: &mut Vec<ComposedStep>,
    on_path: &mut Vec<String>,
    boundary: bool,
) -> Result<(), Finding> {
    if on_path.iter().any(|p| p == id) {
        let cycle = on_path
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(id))
            .collect::<Vec<_>>()
            .join(" -> ");
        return Err(Finding::blocking(
            "workflow-refs.include-cycle-absent",
            format!("include cycle: {cycle}"),
            Location::at(1, 1),
        ));
    }

    let step = source.step(id).ok_or_else(|| {
        Finding::blocking(
            "workflow-refs.include-resolves",
            format!("include `step:{id}` resolves to no step file in the cascade"),
            Location::at(1, 1),
        )
    })?;

    on_path.push(id.to_owned());
    // `first` tracks whether the next leaf this step emits is its very first — only
    // that one inherits the caller's `boundary`; all later leaves continue.
    let mut first = true;
    for segment in body_segments(&step.body) {
        let continues = if first { !boundary } else { true };
        match segment {
            // Only the boundary leaf (a step's first segment, `!continues`) carries
            // the step's kind; continuation prose leaves stay `Plain`.
            BodySegment::Prose(body) => out.push(ComposedStep {
                id: id.to_owned(),
                body,
                continues,
                kind: if continues {
                    StepKind::Plain
                } else {
                    step.kind.clone()
                },
            }),
            // A child include that is *not* the first segment is a continuation of
            // this body — its subtree's first leaf must not start a new blank-line
            // boundary. The first segment inherits this step's own `boundary`.
            BodySegment::Include(child) => {
                expand_step(&child, source, out, on_path, first && boundary)?;
            }
        }
        first = false;
    }
    on_path.pop();
    Ok(())
}

/// One ordered piece of a step body: a verbatim prose run, or a nested include.
enum BodySegment {
    /// A maximal run of non-include lines, kept byte-for-byte (with its newlines).
    Prose(String),
    /// A `{{include: step:<id>}}` line, carrying the resolved `<id>`.
    Include(String),
}

/// Split a step body into ordered [`BodySegment`]s at each `{{include:}}` line —
/// the basis of **in-place** expansion. Consecutive non-include lines coalesce
/// into one [`BodySegment::Prose`]; each include line becomes a
/// [`BodySegment::Include`] at its position. A line is an include when
/// [`parse_include_line`] accepts its trimmed form (the same recognizer the
/// workflow body uses). Prose — the step's `{{cli.…}}` / `{{…}}` / `<<author:…>>`
/// leaves and plain text — survives verbatim.
///
/// A body with **no** include lines (every MVP step) yields a single
/// [`BodySegment::Prose`] holding the body byte-for-byte, so a plain step composes
/// exactly as before (the no-override determinism invariant).
fn body_segments(body: &str) -> Vec<BodySegment> {
    // Fast path: no include line at all → one verbatim prose segment.
    if !body.lines().any(|l| parse_include_line(l.trim()).is_some()) {
        return vec![BodySegment::Prose(body.to_owned())];
    }
    let mut segments = Vec::new();
    let mut prose = String::new();
    for line in body.lines() {
        match parse_include_line(line.trim()) {
            Some(child) => {
                if !prose.is_empty() {
                    segments.push(BodySegment::Prose(std::mem::take(&mut prose)));
                }
                segments.push(BodySegment::Include(child));
            }
            None => {
                prose.push_str(line);
                prose.push('\n');
            }
        }
    }
    if !prose.is_empty() {
        segments.push(BodySegment::Prose(prose));
    }
    segments
}

/// A fully composed workflow: the ordered, four-class **emitted text** of every
/// step, placeholders resolved, ready for the agent to read.
///
/// This is the phase-9 output (`overrides.md` → Resolution algorithm) — the
/// *view* a `jigc start "<intent>"` / `jigc workflow <x> --task <id>` call emits
/// (`workflow-dialect.md` → The composed output is a view: ephemeral, derived,
/// never persisted). It carries the **footerless** body: the routing footer is a
/// presentation concern appended by the agent-text / human renderer in `cli`,
/// never by the engine (the engine stays presentation-free; JSON output carries
/// no footer — `workflow-dialect.md` → Routing footer). As an engine result
/// type it is `Serialize`/`Deserialize` (the renderer / JSON contract).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposedWorkflow {
    /// The full ordered four-class emitted text, every placeholder resolved.
    pub text: String,
}

/// Compose a workflow end-to-end — phases 7 (include expansion), 8 (placeholder
/// resolution), and 9 (emit) of `overrides.md` → Resolution algorithm, wired
/// into one deterministic step (`workflow-dialect.md` → The composed output is a
/// view).
///
/// Expands `def`'s includes against `source` into a flat [`Composition`]
/// ([`expand_includes`]), then emits each step body to the four-class format
/// ([`emit_step_body`]) against `ctx` (the live-state feed) and `catalog` (the
/// command-refs), joining the emitted step texts in include order with a blank
/// line between steps. The result is the [`ComposedWorkflow`] view — the
/// footerless emitted text; the agent-text renderer appends the routing footer.
///
/// Composition is a **pure function** of `(def, source, catalog, ctx)` — same
/// resolved cascade in → same workflow out (the determinism boundary; no I/O, no
/// clock, no LLM). A dangling/cyclic include, an unresolved command-ref, or a
/// malformed/structurally-invalid data-value surfaces as a blocking [`Finding`].
pub fn compose(
    def: &WorkflowDef,
    source: &dyn StepSource,
    catalog: &CommandCatalog,
    ctx: &crate::data_value::ComposeContext,
) -> Result<ComposedWorkflow, Finding> {
    compose_with_store(def, source, catalog, ctx, None)
}

/// Compose a workflow end-to-end, dereferencing `{{@<path>}}` Content lines through
/// `store` when one is provided — the store-backed sibling of [`compose`].
///
/// With a `store`, the superseding-decision `{{@task.decision.supersedes#decision}}`
/// is dereferenced to the superseded ADR's `#decision` prose, emitted as a `> `
/// Content blockquote (`worked-examples.md` → Superseding decision); without one,
/// Content lines carry the resolved address handle (the structural path). Everything
/// else is [`compose`]'s behavior. Still a pure function of its inputs once `store`'s
/// reads are fixed — the engine never reaches outside the fed-in surfaces.
pub fn compose_with_store(
    def: &WorkflowDef,
    source: &dyn StepSource,
    catalog: &CommandCatalog,
    ctx: &crate::data_value::ComposeContext,
    store: Option<&dyn ContentStore>,
) -> Result<ComposedWorkflow, Finding> {
    let composition = expand_includes(def, source)?;
    let mut emitted_steps = Vec::with_capacity(composition.steps.len());
    for step in &composition.steps {
        let mut emitted = emit_step_body_with(&step.body, ctx, catalog, store)?;
        // A `fan-out` step appends its `Spawn:` directives — one per id-sorted
        // sub-task of the resolved `over:` collection — after the step's reasoning
        // prose (`workflow-dialect.md` → Emitted format, rule 4: the 5th class).
        if let StepKind::FanOut { over, run } = &step.kind {
            let spawns = emit_fan_out_spawns(over, run, ctx)?;
            if !spawns.is_empty() {
                if !emitted.is_empty() && !emitted.ends_with('\n') {
                    emitted.push('\n');
                }
                emitted.push_str(&spawns);
            }
        }
        // A `checkpoint` step **prepends** its `Checkpoint: <reason>` halt directive
        // — the bare `reason` slug, no backticks — before the body prose (the inverse
        // of `fan-out`'s `Spawn:` append), so the halt is read before the prose that
        // explains when to honor it (`workflow-dialect.md` → Emitted format, rule 5:
        // the 6th class).
        if let StepKind::Checkpoint { reason } = &step.kind {
            emitted = format!("Checkpoint: {reason}\n{emitted}");
        }
        emitted_steps.push(emitted);
    }
    // Join the per-step emitted texts with a single blank line between **step
    // boundaries**, so the composed view reads as one ordered document. A leaf that
    // *continues* the same on-disk step body across an in-place include splice
    // (`ComposedStep::continues`) is concatenated directly — no blank line — so the
    // spliced prose-before / child / prose-after reads contiguous. Each step body
    // carries its own internal newlines; we trim a leaf's trailing newline before
    // the separator so a boundary separator is exactly one blank line, not two.
    let mut text = String::new();
    for (i, (step, emitted)) in composition.steps.iter().zip(&emitted_steps).enumerate() {
        if i > 0 && !step.continues {
            text.push('\n');
        }
        text.push_str(emitted.trim_end_matches('\n'));
        text.push('\n');
    }
    Ok(ComposedWorkflow { text })
}

/// The engine-native `workflow-refs` probe: validate a workflow definition + its
/// expanded composition tree at compose-time, before any output reaches the agent
/// (`validation.md` → Probes (`workflow-refs`) + Severity inventory; the seven
/// intrinsic-blocking checks).
///
/// One uniform pass runs the seven checks over `(workflow_bytes, source, catalog,
/// ctx)` — the same resolved-cascade inputs [`compose`] consumes — and returns the
/// blocking [`Finding`]s it found (empty ⇒ the definition is conformant). The
/// checks map onto the composition pipeline that already detects each break, so
/// the probe is that pipeline run as a **gate**:
///
/// - `body-include-only` — the workflow body is include-only at top level
///   ([`load_workflow_def`]). Also surfaces malformed/missing front-matter.
/// - `include-resolves` / `include-cycle-absent` — every `{{include}}` resolves
///   to a step in the cascade and the include graph is acyclic
///   ([`expand_includes`]).
/// - `run-marker-not-shadowed` / `spawn-marker-not-shadowed` — no step's
///   instruction prose starts a line with the composer-reserved `Run: ` / `Spawn: `
///   marker (`workflow-dialect.md` → Compose-time conformance); checked per expanded
///   step body by [`find_run_shadow`] / [`find_spawn_shadow`].
/// - `fan-out-join-paired` — each `fan-out` step pairs with a `join` step in the
///   workflow and vice-versa (M8), checked over the composition's per-step kinds by
///   [`find_fan_out_join_pairing`].
/// - `command-ref-resolves` / `placeholder-resolves` / `at-marker-on-non-scalar`
///   — every `{{cli.…}}` resolves against the catalog, every `{{…}}`/`{{@…}}`
///   data-value resolves (or is legitimately absent), and no `@` applies to a
///   scalar ([`emit_step_body`], which drives the seq-5 resolver + the seq-6/7
///   catalog).
///
/// A definition that fails to load or expand cannot be emitted, so those checks
/// short-circuit (one structural finding, no half-built tree to walk). Once the
/// tree expands, each step body is checked independently so multiple steps each
/// surface their own break. The probe is a **pure function** of its inputs (the
/// determinism boundary; no I/O, clock, or LLM) — the same resolved cascade in
/// yields the same findings out.
pub fn workflow_refs(
    workflow_bytes: &[u8],
    source: &dyn StepSource,
    catalog: &CommandCatalog,
    ctx: &crate::data_value::ComposeContext,
) -> Vec<Finding> {
    workflow_refs_with_deltas(workflow_bytes, &[], source, catalog, ctx)
}

/// The `workflow_refs` gate over a workflow's **post-phase-4** include list —
/// [`workflow_refs`] with the manifest's `structural-op` `deltas` applied before
/// the checks run, so a delta-introduced orphan / cycle / dangling surfaces at
/// resolution time through the same machinery (`overrides.md` → Resolution
/// algorithm phases 4–7; Write-time vs resolve-time split).
///
/// `deltas` are the deltas already **scoped to this workflow id** (the frontend
/// filters by `target().workflow_id` before calling — [`apply_structural_deltas`]
/// applies every delta it is fed). Phase 4 runs first: an orphaned anchor — a
/// delta whose anchor a same-manifest delta removed — short-circuits to that one
/// located [`Finding`] (`structural-anchor-resolves`, carrying its repair route),
/// with no half-built tree to walk. Otherwise the post-phase-4 id list flows into
/// [`expand_includes`]'s `include-resolves` / `include-cycle-absent` checks, so a
/// re-include of a *different* id is clean (flow 3a) while a re-include that loops
/// back is the located cycle finding — never a panic.
///
/// A pure function of its inputs (the determinism boundary); the empty-`deltas`
/// call is [`workflow_refs`]'s exact prior behavior (the no-override path stays
/// byte-identical).
pub fn workflow_refs_with_deltas(
    workflow_bytes: &[u8],
    deltas: &[StructuralDelta],
    source: &dyn StepSource,
    catalog: &CommandCatalog,
    ctx: &crate::data_value::ComposeContext,
) -> Vec<Finding> {
    // body-include-only (and malformed/missing front-matter): a definition that
    // does not load has no tree to walk.
    let mut def = match load_workflow_def(workflow_bytes) {
        Ok(def) => def,
        Err(finding) => return vec![finding],
    };

    // Phase 4 — apply the structural deltas to the include id list before
    // expansion. An orphaned anchor surfaces here as that one located finding.
    def.includes = match apply_structural_deltas(&def.includes, deltas) {
        Ok(includes) => includes,
        Err(finding) => return vec![finding],
    };

    // include-resolves / include-cycle-absent: a tree that does not expand cannot
    // be emitted. Run over the **post-phase-4** include list.
    let composition = match expand_includes(&def, source) {
        Ok(composition) => composition,
        Err(finding) => return vec![finding],
    };

    // Workflow-level: each `fan-out` step must pair with a `join` step (M8).
    let mut findings = Vec::new();
    if let Some(finding) = find_fan_out_join_pairing(&composition) {
        findings.push(finding);
    }

    // Per expanded step body: the reserved-marker shadow checks, then emission
    // (command-ref-resolves / placeholder-resolves / at-marker-on-non-scalar).
    for step in &composition.steps {
        if let Some(finding) = find_run_shadow(&step.body) {
            findings.push(finding);
        }
        if let Some(finding) = find_spawn_shadow(&step.body) {
            findings.push(finding);
        }
        if let Some(finding) = find_checkpoint_shadow(&step.body) {
            findings.push(finding);
        }
        if let Err(finding) = emit_step_body(&step.body, ctx, catalog) {
            findings.push(finding);
        }
    }
    findings
}

/// The **store-scope** `workflow-refs` target — the task-less complement to
/// [`workflow_refs`] (`validation.md` → Completing the envelope: workflow↔refs at
/// store scope). It re-checks a workflow definition's **task-independent**
/// referential integrity against the current pack, with **no task minted** and **no
/// data-value context** — the `jigc validate` store sweep, not the `jigc start`
/// compose gate.
///
/// One pass runs only the checks that are well-defined with no task in hand:
///
/// - `body-include-only` — the body is include-only at top level
///   ([`load_workflow_def`]; also surfaces malformed/missing front-matter).
/// - `include-resolves` / `include-cycle-absent` — every `{{include}}` resolves to a
///   step in the cascade and the include graph is acyclic ([`expand_includes`]).
/// - `run-` / `spawn-` / `checkpoint-marker-not-shadowed` — no step prose shadows a
///   composer-reserved marker ([`find_run_shadow`] / [`find_spawn_shadow`] /
///   [`find_checkpoint_shadow`]).
/// - `fan-out-join-paired` — each `fan-out` pairs with a `join` and vice-versa
///   ([`find_fan_out_join_pairing`]).
/// - `command-ref-resolves` — **catalog membership only** ([`find_command_ref_membership`]):
///   each lone `{{cli.<id>}}` names an entry the catalog `get`s, **without** rendering
///   its args. The store sweep deliberately does **not** call [`render_command`]: the
///   dev pack gives nearly every command-ref a `task.*` arg, so a task-less full
///   resolution would emit `task-ref-in-no-task-workflow` for essentially every
///   command-ref — **fabricated drift, not real** (`validation.md` → the membership-only
///   load-bearing line; the design-review B1 fix).
///
/// The **task-data-dependent** checks — `placeholder-resolves` over `{{task.*}}`,
/// `at-marker-on-non-scalar` — are inherently compose-time and **stay at `jigc start`**
/// ([`workflow_refs`]); they are not run here. Reuses the `workflow-refs.*` check ids
/// (no new id, no severity-inventory growth). A pure function of its inputs (the
/// determinism boundary; no I/O, clock, or LLM, and — by construction — no task).
pub fn workflow_refs_store(
    workflow_bytes: &[u8],
    source: &dyn StepSource,
    catalog: &CommandCatalog,
) -> Vec<Finding> {
    // body-include-only (and malformed/missing front-matter): a definition that does
    // not load has no tree to walk.
    let def = match load_workflow_def(workflow_bytes) {
        Ok(def) => def,
        Err(finding) => return vec![finding],
    };

    // include-resolves / include-cycle-absent: a tree that does not expand cannot be
    // emitted.
    let composition = match expand_includes(&def, source) {
        Ok(composition) => composition,
        Err(finding) => return vec![finding],
    };

    // Workflow-level: each `fan-out` step must pair with a `join` step (M8).
    let mut findings = Vec::new();
    if let Some(finding) = find_fan_out_join_pairing(&composition) {
        findings.push(finding);
    }

    // Per expanded step body: the reserved-marker shadow checks, then the
    // membership-only command-ref check (no `render_command`, no task data).
    for step in &composition.steps {
        if let Some(finding) = find_run_shadow(&step.body) {
            findings.push(finding);
        }
        if let Some(finding) = find_spawn_shadow(&step.body) {
            findings.push(finding);
        }
        if let Some(finding) = find_checkpoint_shadow(&step.body) {
            findings.push(finding);
        }
        findings.extend(find_command_ref_membership(&step.body, catalog));
    }
    findings
}

/// The **membership-only** command-ref check ([`workflow_refs_store`]): for each lone
/// `{{cli.<id>}}` line in `body`, a blocking `workflow-refs.command-ref-resolves`
/// [`Finding`] (located at that body-relative line) when the catalog has no entry for
/// `<id>`; an entry that exists yields none.
///
/// This is the store-scope half of `command-ref-resolves`: it checks **only** that the
/// id is in the catalog ([`CommandCatalog::get`]), never that the entry's args resolve
/// against a task — [`render_command`] is **not** called (`validation.md` → the
/// membership-only load-bearing line; the B1 fix). A command-ref whose args are all
/// `task.*` is therefore clean at store scope, where there is no task to resolve them
/// against. Mirrors the compose-time emit-path finding ([`emit_line`]) so the reused
/// check id carries the same message shape, minus the arg-resolution leg.
fn find_command_ref_membership(body: &str, catalog: &CommandCatalog) -> Vec<Finding> {
    body.lines()
        .enumerate()
        .filter_map(|(offset, line)| {
            let id = parse_cli_placeholder(line.trim())?;
            catalog.get(id).is_none().then(|| {
                Finding::blocking(
                    "workflow-refs.command-ref-resolves",
                    format!("command-ref `{{{{cli.{id}}}}}` resolves to no catalog entry"),
                    Location::at(offset + 1, 1),
                )
            })
        })
        .collect()
}

/// The fill-aware `workflow_refs` gate — [`workflow_refs_with_deltas`] extended with
/// the **two M4 fill checks**, run over **post-phase-5** step bodies (`overrides.md`
/// → The `{{fill:}}` placeholder: Orphan detection is M4; no nested fills).
///
/// After phase 4 (structural deltas) and phase 7 (include expansion), each composed
/// step body is fed through phase 5 ([`apply_slot_fills`]) against `fills` before the
/// per-step checks run, so they fire at the same resolution point as
/// `structural-anchor-resolves`. The two net-new checks:
///
/// - **`slot-fill-orphan`** — a `slot_fills` delta whose **target step** (via
///   `source`) either does not resolve or resolves but declares no `{{fill:<id>}}`
///   point for `fill_id` is blocking, carrying its repair **route**. Keyed on the
///   *target step* — the same question the write-time `check_fill_point_present` asks
///   — so a slot-fill whose target step is simply absent from the composed workflow is
///   **inert for that compose, not an orphan** (symmetric with the orphaned
///   `structural-anchor-resolves`, closing the closed-surface hole for both authoring
///   paths).
/// - **`fill-survivor`** — a `{{fill:}}` token (lone-line or mid-line) surviving into
///   a post-phase-5 body (a nested fill phase 5 did not re-run) is blocking,
///   **located at its line** ([`find_fill_survivor`]).
///
/// `deltas` (phase-4 structural-ops) and `slot_fills` (the slot-fill deltas) are both
/// already **scoped to this workflow** by the frontend. `fills` is the cascade-
/// resolved fill content ([`ResolvedFills`]). A pure function of its inputs (the
/// determinism boundary); the empty-`slot_fills`/`fills` call is
/// [`workflow_refs_with_deltas`]'s behavior over post-phase-5 bodies (no fill point →
/// phase 5 is a no-op, byte-identical).
#[allow(clippy::too_many_arguments)]
pub fn workflow_refs_with_fills(
    workflow_bytes: &[u8],
    deltas: &[StructuralDelta],
    slot_fills: &[crate::cascade::SlotFillDelta],
    fills: &ResolvedFills,
    source: &dyn StepSource,
    catalog: &CommandCatalog,
    ctx: &crate::data_value::ComposeContext,
) -> Vec<Finding> {
    // body-include-only / front-matter: a definition that does not load has no tree.
    let mut def = match load_workflow_def(workflow_bytes) {
        Ok(def) => def,
        Err(finding) => return vec![finding],
    };

    // Phase 4 — structural deltas on the include id list.
    def.includes = match apply_structural_deltas(&def.includes, deltas) {
        Ok(includes) => includes,
        Err(finding) => return vec![finding],
    };

    // Phase 7 — include expansion (and phase 6 cycle detection).
    let composition = match expand_includes(&def, source) {
        Ok(composition) => composition,
        Err(finding) => return vec![finding],
    };

    // slot-fill-orphan: a slot-fill is orphaned iff its **target step** resolves but
    // declares no `{{fill:<fill-id>}}` point (or the target step does not resolve at
    // all) — keyed on the *target step* via `source`, the same question the write-time
    // `check_fill_point_present` asks, evaluated regardless of whether the
    // currently-composed workflow includes that step. A slot-fill whose target step is
    // simply absent from this workflow is **inert for this compose, not an orphan**
    // (`overrides.md` → The `{{fill:}}` placeholder: slot-fill targets). Keying on the
    // composed workflow's bodies instead would falsely orphan-block every workflow that
    // omits the targeted step — including the bare-`jigc start` router.
    let mut findings = Vec::new();
    for delta in slot_fills {
        let declares_point = source
            .step(&delta.target.step_id)
            .is_some_and(|step| fill_ids_in(&step.body).contains(&delta.target.fill_id.as_str()));
        if !declares_point {
            findings.push(Finding::graded(
                crate::finding::Severity::Blocking,
                "workflow-refs.slot-fill-orphan",
                format!(
                    "slot-fill targets `step:{}#{}`, a `{{{{fill:}}}}` point no resolved step body declares (orphaned)",
                    delta.target.step_id, delta.target.fill_id
                ),
                Some(Location::at(1, 1)),
                Some(format!(
                    "remove or re-target the slot-fill on `step:{}#{}` with `jigc config fill` (the `{{{{fill:}}}}` point it names is not in the resolved step body)",
                    delta.target.step_id, delta.target.fill_id
                )),
            ));
        }
    }

    // Workflow-level: each `fan-out` step must pair with a `join` step (M8). The
    // kind rides on the boundary leaf untouched by phase 5, so it reads the same
    // composition the per-step loop walks.
    if let Some(finding) = find_fan_out_join_pairing(&composition) {
        findings.push(finding);
    }

    // Per expanded step body, run phase 5, then the post-phase-5 checks:
    // fill-survivor, run/spawn-marker shadow, and emission (command-ref / placeholder).
    for step in &composition.steps {
        let applied = match apply_slot_fills(&step.id, &step.body, fills) {
            Ok(applied) => applied,
            Err(finding) => {
                findings.push(finding);
                continue;
            }
        };
        // A surviving `{{fill:}}` is *this body's* break — it would otherwise be
        // mis-parsed as a `fill`-rooted data-value by the emitter, so the distinct
        // survivor finding supersedes the emit check for this body.
        if let Some(finding) = find_fill_survivor(&applied) {
            findings.push(finding);
            continue;
        }
        if let Some(finding) = find_run_shadow(&applied) {
            findings.push(finding);
        }
        if let Some(finding) = find_spawn_shadow(&applied) {
            findings.push(finding);
        }
        if let Some(finding) = find_checkpoint_shadow(&applied) {
            findings.push(finding);
        }
        if let Err(finding) = emit_step_body(&applied, ctx, catalog) {
            findings.push(finding);
        }
    }
    findings
}

/// If a step body line shadows the composer-reserved `Run: ` marker, return a
/// blocking `run-marker-not-shadowed` [`Finding`] located at that line; else
/// `None`.
///
/// `Run: ` at a line's left margin is the composer's directive, emitted only from
/// a resolved `{{cli.…}}` command-ref (`workflow-dialect.md` → Compose-time
/// conformance: "instruction prose in a step definition must not start a line with
/// `Run: `"). The body here is the de-included step prose with placeholders still
/// unresolved, so a literal `Run: ` line is authored prose, never a resolved
/// command-ref (which is `{{ cli.… }}` at this stage). The first shadowing line is
/// reported with a precise (step-body-relative) line pointer.
fn find_run_shadow(body: &str) -> Option<Finding> {
    body.lines().enumerate().find_map(|(offset, line)| {
        line.trim_start().starts_with("Run: ").then(|| {
            Finding::blocking(
                "workflow-refs.run-marker-not-shadowed",
                format!(
                    "step prose shadows the composer-reserved `Run: ` marker: `{}`",
                    line.trim()
                ),
                Location::at(offset + 1, 1),
            )
        })
    })
}

/// If a step body line shadows the composer-reserved `Spawn: ` marker, return a
/// blocking `spawn-marker-not-shadowed` [`Finding`] located at that line; else
/// `None`. The `Spawn: ` mirror of [`find_run_shadow`].
///
/// `Spawn: ` at a line's left margin is the composer's fan-out directive, emitted
/// only from a resolved `fan-out` step ([`emit_fan_out_spawns`]) — never authored
/// prose (`workflow-dialect.md` → Compose-time conformance: "instruction prose in a
/// step definition must not start a line with `Run: ` or `Spawn: `"). The body
/// here is the de-included step prose with the `fan-out` spawns not yet appended,
/// so a literal `Spawn: ` line is authored prose, never an emitted directive. The
/// first shadowing line is reported with a precise (step-body-relative) pointer.
fn find_spawn_shadow(body: &str) -> Option<Finding> {
    body.lines().enumerate().find_map(|(offset, line)| {
        line.trim_start().starts_with("Spawn: ").then(|| {
            Finding::blocking(
                "workflow-refs.spawn-marker-not-shadowed",
                format!(
                    "step prose shadows the composer-reserved `Spawn: ` marker: `{}`",
                    line.trim()
                ),
                Location::at(offset + 1, 1),
            )
        })
    })
}

/// If a step body line shadows the composer-reserved `Checkpoint: ` marker, return
/// a blocking `checkpoint-marker-not-shadowed` [`Finding`] located at that line;
/// else `None`. The `Checkpoint: ` mirror of [`find_spawn_shadow`] (M15).
///
/// `Checkpoint: ` at a line's left margin is the composer's halt directive, emitted
/// only from a resolved `checkpoint` step (prepended in [`compose_with_store`]) —
/// never authored prose (`workflow-dialect.md` → Compose-time conformance). The body
/// here is the de-included step prose **before** the directive is prepended, so a
/// real `checkpoint` step's own directive never self-trips and a literal
/// `Checkpoint: ` line is authored prose. The first shadowing line is reported with
/// a precise (step-body-relative) pointer.
fn find_checkpoint_shadow(body: &str) -> Option<Finding> {
    body.lines().enumerate().find_map(|(offset, line)| {
        line.trim_start().starts_with("Checkpoint: ").then(|| {
            Finding::blocking(
                "workflow-refs.checkpoint-marker-not-shadowed",
                format!(
                    "step prose shadows the composer-reserved `Checkpoint: ` marker: `{}`",
                    line.trim()
                ),
                Location::at(offset + 1, 1),
            )
        })
    })
}

/// The workflow-level `fan-out-join-paired` check over a composition's per-step
/// kinds: a `fan-out` step must have a matching `join` step in the same workflow
/// **and vice-versa** (`workflow-dialect.md` → Compose-time conformance; the M8
/// pairing rule). An unpaired `fan-out` (a spawn with no merge boundary) or an
/// unpaired `join` (a merge with nothing fanned out) is a blocking
/// `workflow-refs.fan-out-join-paired` [`Finding`]; a paired (or kind-free)
/// composition yields none.
///
/// The check is presence-pairing, not count-matching (one `join` rejoins any
/// number of fan-outs) — it reads only the [`StepKind`]s already on
/// [`ComposedStep`] (T2), so it re-parses no front-matter. The finding carries no
/// per-step location (it is a whole-workflow property); the route names the missing
/// counterpart.
fn find_fan_out_join_pairing(composition: &Composition) -> Option<Finding> {
    let has_fan_out = composition
        .steps
        .iter()
        .any(|s| matches!(s.kind, StepKind::FanOut { .. }));
    let has_join = composition
        .steps
        .iter()
        .any(|s| matches!(s.kind, StepKind::Join));
    match (has_fan_out, has_join) {
        (true, false) => Some(Finding::block(
            "workflow-refs.fan-out-join-paired",
            "a `fan-out` step has no matching `join` step in the workflow",
            "add a `join` step after the `fan-out` so the fanned sub-tasks have a merge boundary",
        )),
        (false, true) => Some(Finding::block(
            "workflow-refs.fan-out-join-paired",
            "a `join` step has no matching `fan-out` step in the workflow",
            "add a `fan-out` step before the `join`, or remove the unpaired `join`",
        )),
        _ => None,
    }
}

/// If a **post-phase-5** step body still carries a `{{fill:<id>}}` token — lone-line
/// **or mid-line** — return a blocking `workflow-refs.fill-survivor` [`Finding`]
/// located at that line; else `None`.
///
/// Phase 5 ([`apply_slot_fills`]) replaces every declared `{{fill:}}` point — filled
/// or defaulted — and **does not re-run**, so any `{{fill:}}` surviving into the
/// post-phase-5 body arrived *inside applied content* (a nested fill) and will never
/// resolve (`overrides.md` → no nested fills: "phase 5 does not re-run"). A surviving
/// `{{fill:}}` is its own break, distinct from the generic `placeholder-resolves` /
/// `undeclared-root` finding the emitter would otherwise raise. The first survivor is
/// reported with its body-relative line pointer. The body is scanned line-by-line by
/// the same [`next_fill_token`] recognizer phase 5 uses, so a mid-line nested fill
/// (which would otherwise leak its literal token into agent output) is caught too.
fn find_fill_survivor(body: &str) -> Option<Finding> {
    body.lines().enumerate().find_map(|(offset, line)| {
        next_fill_token(line).map(|(_, fill_id)| {
            Finding::blocking(
                "workflow-refs.fill-survivor",
                format!(
                    "a `{{{{fill: {fill_id}}}}}` survives composition unresolved (phase 5 does not re-run — fill content may not contain another `{{{{fill:}}}}`)"
                ),
                Location::at(offset + 1, 1),
            )
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `ComposeContext` for emitter tests: `task.intent` bound, `commit` bound
    /// (a `creates-task` workflow's commit doc is the task's own slug), and
    /// `decision` **declared but unbound** (the agent has not yet created the ADR).
    fn emit_ctx() -> crate::data_value::ComposeContext {
        let mut roles = std::collections::BTreeMap::new();
        roles.insert(
            "commit".to_owned(),
            Some(crate::address::Address::parse("commit:emit-four-classes").expect("valid")),
        );
        roles.insert("decision".to_owned(), None);
        crate::data_value::ComposeContext {
            task: Some(crate::data_value::TaskRoot {
                id: "emit-four-classes".to_owned(),
                intent: "emit a composed step body to the four-class format".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        }
    }

    /// Core done-criterion (`emit_four_classes`): a fixture step body carrying one
    /// `{{cli.…}}`, one bound `{{@…}}`, one unbound `{{@…}}`, one
    /// `<<author: {{…}}>>`, and plain prose emits exactly the four conventions —
    /// a `` Run: `<cmd>` `` line, a `> ` blockquote, an empty line (the unbound
    /// `@` → empty-not-finding), an `<<author: <address>>` directive (wrapper
    /// preserved, embedded `{{…}}` resolved), and bare Reason prose. The golden
    /// pins the emitted bytes.
    #[test]
    fn emit_four_classes() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = emit_ctx();
        let body = "\
Implement the change directly. When done, stage the commit prose:
{{ cli.set-commit-summary }}
<<author: {{ task.commit#summary }}>>

Here is the bound commit content:
{{ @task.commit#summary }}

If your decision supersedes an earlier one, here is that decision:
{{ @task.decision.supersedes#decision }}
";
        let emitted = emit_step_body(body, &ctx, &catalog).expect("emits");

        // The Run line matches the strict line-pattern `^Run: ` + backticked cmd.
        let run_line = emitted
            .lines()
            .find(|l| l.starts_with("Run: "))
            .expect("a Run line is emitted");
        assert!(
            run_line.starts_with("Run: `") && run_line.ends_with('`'),
            "Run line must be `^Run: `(.+)`$`, got {run_line:?}"
        );
        assert_eq!(
            run_line,
            "Run: `jigc doc set-slot commit:emit-four-classes#summary --from-file - --task emit-four-classes`"
        );

        // The `<<author:>>` wrapper survives; only the embedded `{{…}}` resolves.
        assert!(emitted.contains("<<author: commit:emit-four-classes#summary>>"));

        insta::assert_snapshot!(emitted, @r#"
        Implement the change directly. When done, stage the commit prose:
        Run: `jigc doc set-slot commit:emit-four-classes#summary --from-file - --task emit-four-classes`
        <<author: commit:emit-four-classes#summary>>

        Here is the bound commit content:
        > commit:emit-four-classes#summary

        If your decision supersedes an earlier one, here is that decision:

        "#);
    }

    /// The unbound-`@` → empty-line case in isolation (empty-not-finding): a
    /// declared-but-unbound `@`-path resolves to [`Resolution::Absent`], which the
    /// emitter writes as an empty line — never a `> ` blockquote, never a finding.
    #[test]
    fn emit_unbound_at_is_empty_line() {
        let ctx = emit_ctx();
        let catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        let emitted = emit_step_body(
            "before\n{{ @task.decision.supersedes#decision }}\nafter\n",
            &ctx,
            &catalog,
        )
        .expect("emits");
        insta::assert_snapshot!(emitted, @r###"
        before

        after
        "###);
    }

    /// An `<<author: …>>` directive on a **declared-but-unbound** role names the
    /// slot from the path's own role + `#fragment` rather than emitting an empty
    /// `<<author: >>` target the agent must guess into. The doc is created in-task,
    /// so the bound slug is not knowable on this compose — but the slot (`#fragment`)
    /// is, and a re-compose after the create fills the resolved address in.
    #[test]
    fn emit_unbound_author_names_the_slot() {
        let ctx = emit_ctx();
        let catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        // `decision` is declared-but-unbound in `emit_ctx`.
        let emitted = emit_step_body(
            "<<author: {{ task.decision#consequences }}>>\n",
            &ctx,
            &catalog,
        )
        .expect("emits");
        assert_eq!(emitted, "<<author: decision#consequences>>\n");
    }

    /// Core done-criterion (M23 inc-1 T1): the **source seam** — a lone
    /// `{{source}}` read-only context placeholder surfaces the foreign bytes fed
    /// on [`ComposeContext::source`] **verbatim** into the emitted step body, and
    /// the `workflow-refs` gate flags no finding for it. The seam is syntactically
    /// distinct from a managed-doc deref (`{{@…}}`): no `@`, no managed head — it
    /// carries non-managed raw bytes, never an address.
    #[test]
    fn emit_source_seam_verbatim() {
        let catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        let foreign = "\
# Changelog

## [1.2.0] - 2023-01-15
### Added
- A foreign feature line.

## [1.1.0] - 2022-08-01
### Fixed
- A foreign bugfix.";
        let ctx = crate::data_value::ComposeContext {
            source: Some(foreign.to_owned()),
            ..emit_ctx()
        };
        let body = "Here is the foreign file to rewrite:\n{{ source }}\nRewrite it now.\n";
        let emitted = emit_step_body(body, &ctx, &catalog).expect("emits");

        // The foreign content appears verbatim, between the framing prose lines.
        assert!(
            emitted.contains(foreign),
            "the foreign bytes must appear verbatim; got {emitted:?}"
        );
        insta::assert_snapshot!(emitted, @r###"
        Here is the foreign file to rewrite:
        # Changelog

        ## [1.2.0] - 2023-01-15
        ### Added
        - A foreign feature line.

        ## [1.1.0] - 2022-08-01
        ### Fixed
        - A foreign bugfix.
        Rewrite it now.
        "###);
    }

    /// The source seam is **inert when unfed** (empty-not-finding): a `{{source}}`
    /// in a composition whose `ComposeContext` carries no foreign bytes emits an
    /// empty line — never a finding, mirroring the unbound-`@` contract. This is
    /// the omitting-context guard: the seam composed into a non-migration context
    /// (`source: None`) is inert, not an error.
    #[test]
    fn emit_source_seam_unfed_is_empty_line() {
        let catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        let ctx = emit_ctx(); // source defaults to None
        let emitted =
            emit_step_body("before\n{{ source }}\nafter\n", &ctx, &catalog).expect("emits");
        insta::assert_snapshot!(emitted, @r###"
        before

        after
        "###);
    }

    /// The end-to-end `workflow-refs` gate emits **no finding** for a `{{source}}`
    /// seam line — driven through the real [`workflow_refs`] gate over a workflow
    /// whose step body carries the seam, both fed and unfed. A bare `source` is a
    /// known read-only-context placeholder, not a data-value path (which would
    /// surface an `undeclared-root` finding). The unfed arm is the omitting-context
    /// guard: the seam composed into a context with no foreign bytes is inert.
    #[test]
    fn source_seam_clears_workflow_refs_gate() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let src = MapSource::new(&[("only", "the foreign file:\n{{ source }}\nrewrite it.\n")]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";

        // Fed: the gate clears with the foreign bytes present.
        let fed_ctx = crate::data_value::ComposeContext {
            source: Some("foreign bytes".to_owned()),
            ..compose_ctx()
        };
        let fed = workflow_refs(wf, &src, &catalog, &fed_ctx);
        assert!(
            fed.is_empty(),
            "a fed source seam must clear the gate, got {fed:?}"
        );

        // Unfed: the gate clears with no foreign bytes (empty-not-finding).
        let unfed_ctx = compose_ctx(); // source defaults to None
        let unfed = workflow_refs(wf, &src, &catalog, &unfed_ctx);
        assert!(
            unfed.is_empty(),
            "an unfed source seam must clear the gate (empty-not-finding), got {unfed:?}"
        );
    }

    /// Core done-criterion (M17 inc-4 T2): an **inline** `{{<path>}}` data-value
    /// token in a step-file prose line resolves in place with the lone-line
    /// Reason-resolved semantics — a scalar substitutes its text (`{{task.id}}` →
    /// the minted slug, the literal `--task` disambiguation line T3 ships), a
    /// bound role substitutes its address handle, an absent role substitutes
    /// empty text (empty-not-finding). A line with no placeholder stays
    /// byte-unchanged. Since the 2026-06-12 adjudication this runs as the
    /// **pre-phase-5** pass over step-file bodies, never at emit.
    #[test]
    fn inline_data_value_token_resolves_in_prose_line() {
        let ctx = emit_ctx();
        let body = "\
jigc doc add-item x --task {{task.id}}
intent: {{ task.intent }} (inline scalar)
see {{task.commit}} and absent [{{task.decision}}] inline
plain prose, {single} braces, no tokens
";
        let resolved = resolve_inline_data_values(body, &ctx);
        insta::assert_snapshot!(resolved, @r"
        jigc doc add-item x --task emit-four-classes
        intent: emit a composed step body to the four-class format (inline scalar)
        see commit:emit-four-classes and absent [] inline
        plain prose, {single} braces, no tokens
        ");
    }

    /// An unresolvable **inline** token is left **verbatim** — inert, never
    /// blocking (M17 inc-4 validation fix: prose *mentioning* `{{…}}` syntax must
    /// not brick compose). An inline **collection** (`{{catalog}}`) is inert too —
    /// collections stay lone-line classes; blocking stays with the lone-line
    /// forms, and a resolvable token on the same line still substitutes.
    #[test]
    fn inline_unresolvable_path_and_collection_stay_inert_verbatim() {
        let ctx = emit_ctx();
        let line = "Note: template syntax like {{version}} appears in our docs; leave it as-is.\n";
        assert_eq!(
            resolve_inline_data_values(line, &ctx),
            line,
            "an unresolvable inline token stays verbatim"
        );

        let line = "see {{task.bogus}} here\n";
        assert_eq!(
            resolve_inline_data_values(line, &ctx),
            line,
            "an undeclared inline role stays verbatim"
        );

        let line = "pick from {{catalog}} now\n";
        assert_eq!(
            resolve_inline_data_values(line, &ctx),
            line,
            "an inline collection stays verbatim"
        );

        // A resolvable token still substitutes alongside an inert one.
        assert_eq!(
            resolve_inline_data_values("use {{version}} for task {{task.id}}\n", &ctx),
            "use {{version}} for task emit-four-classes\n",
            "resolvable inline tokens substitute; inert ones stay verbatim",
        );
    }

    /// The other placeholder kinds are **skipped** to their own phases (the
    /// `next_fill_token` discipline): a `{{fill:…}}`-shaped inner belongs to
    /// phase 5, a `{{cli.…}}` / `{{@…}}` / `{{include:…}}` inner is a lone-line
    /// class — all four pass through byte-for-byte, never resolved inline. A
    /// **lone-line** placeholder of any kind and an `<<author: …>>` directive
    /// pass verbatim too — those belong to the emit classes (blocking semantics
    /// included), not the inline pre-pass.
    #[test]
    fn inline_other_placeholder_kinds_pass_through_verbatim() {
        let ctx = emit_ctx();
        let body =
            "do {{fill: extra}} then {{cli.x}} and {{@task.commit}} and {{include: step:y}}\n";
        assert_eq!(
            resolve_inline_data_values(body, &ctx),
            body,
            "non-data-value inners are left to their phases"
        );

        let body = "{{ task.intent }}\n<<author: {{task.commit#summary}}>>\n{{nonsense}}\n";
        assert_eq!(
            resolve_inline_data_values(body, &ctx),
            body,
            "lone-line placeholders and author directives are left to their emit classes"
        );
    }

    /// The emit pass itself never rewrites Reason prose: an inline `{{<path>}}`
    /// token — resolvable or not — reaching [`emit_step_body`] (i.e. surviving to
    /// post-phase-5 text, where fill-applied prose lives) passes through
    /// byte-for-byte. Fill-authored prose is never rewritten — the determinism
    /// boundary applied (`DECISIONS.md` 2026-06-12).
    #[test]
    fn emit_leaves_inline_tokens_in_post_fill_prose_verbatim() {
        let ctx = emit_ctx();
        let catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        let body = "House rule: restate {{task.intent}} for task {{task.id}}.\n";
        let emitted = emit_step_body(body, &ctx, &catalog).expect("inline prose never blocks");
        assert_eq!(
            emitted, body,
            "emit never substitutes inline tokens — fill-applied prose stays verbatim"
        );
    }

    // -- The superseding-decision context-slice (inc-5 seq 5) ----------------
    //
    // `{{@task.decision.supersedes#decision}}` dereferenced through the bound
    // `task.decision` role + the `supersedes` edge hop + the committed store to the
    // superseded ADR's `#decision` prose, emitted as a `> ` Content blockquote
    // (`worked-examples.md` → Superseding decision). An unbound role or an unset
    // `supersedes` short-circuits to empty text — no finding (flow #1).

    use crate::address::Address;
    use crate::index::{self, WorkingOverlay};
    use crate::schema::Schema;
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    const ADR_YAML: &[u8] = include_bytes!("../../cli/pack/schemas/adr.yaml");

    /// A throwaway directory that removes itself on drop.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-compose-slice-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
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

    fn adr_schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "adr".to_string(),
            crate::schema::load_schema_with_types(ADR_YAML, &crate::schema::dev_pack_field_types())
                .expect("adr.yaml loads"),
        );
        m
    }

    /// The committed ADR the supersede points at (`decisions/single-node-cache.md`).
    const COMMITTED_ADR: &str = "\
---
status: accepted
date: 2026-05-23
---

# Single-node session cache

## Context
Session lookups must stay sub-millisecond.

## Decision
A single in-memory node keeps session lookups sub-millisecond and avoids a
network hop; acceptable because sessions are cheap to reconstruct on a cold node.

## Consequences
A cold node loses its sessions; clients re-authenticate.
";

    /// A working-area ADR `B` whose `supersedes` is `supersedes` (or unset when
    /// `None`).
    fn working_adr_b(supersedes: Option<&str>) -> String {
        let field = supersedes
            .map(|to| format!("supersedes: {to}\n"))
            .unwrap_or_default();
        format!(
            "\
---
status: accepted
date: 2026-05-30
{field}---

# Shared redis session cache

## Context
A single node is a single point of failure.

## Decision
Replicate the session cache across nodes.

## Consequences
Slightly higher write latency for resilience.
"
        )
    }

    /// Commit ADR `A` at `decisions/single-node-cache.md` under `repo_root`.
    fn commit_adr_a(repo_root: &Path) {
        let dir = repo_root.join("decisions");
        std::fs::create_dir_all(&dir).expect("mk decisions/");
        std::fs::write(dir.join("single-node-cache.md"), COMMITTED_ADR).expect("write A");
    }

    /// Stage ADR `B` at `<task_dir>/docs/adr:shared-redis-session-cache.md`.
    fn stage_adr_b(task_dir: &Path, supersedes: Option<&str>) {
        let docs = task_dir.join("docs");
        std::fs::create_dir_all(&docs).expect("mk docs/");
        std::fs::write(
            docs.join("adr:shared-redis-session-cache.md"),
            working_adr_b(supersedes),
        )
        .expect("stage B");
    }

    /// The `ComposeContext` with `task.decision` **bound** to the staged ADR `B`
    /// (the create-gate binding) — what the superseding task's resume sees.
    fn ctx_decision_bound() -> crate::data_value::ComposeContext {
        let mut roles = BTreeMap::new();
        roles.insert(
            "commit".to_owned(),
            Some(Address::parse("commit:shared-redis-session-cache").expect("valid")),
        );
        roles.insert(
            "decision".to_owned(),
            Some(Address::parse("adr:shared-redis-session-cache").expect("valid")),
        );
        crate::data_value::ComposeContext {
            task: Some(crate::data_value::TaskRoot {
                id: "shared-redis-session-cache".to_owned(),
                intent: "move the session cache to a shared redis cluster".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        }
    }

    /// Build the live [`StoreContext`] over the committed store + the task overlay.
    fn store_ctx<'a>(
        repo_root: &'a Path,
        schemas: &'a BTreeMap<String, Schema>,
        overlay: &'a WorkingOverlay,
    ) -> StoreContext<'a> {
        StoreContext {
            repo_root,
            schemas,
            overlay,
        }
    }

    /// HEADLINE: `{{@task.decision.supersedes#decision}}` dereferences the bound
    /// `task.decision` (`adr:shared-redis-session-cache`) → its `supersedes` edge
    /// (`adr:single-node-cache`, committed) → that ADR's `#decision` slice, emitted
    /// as a multi-line `> ` Content blockquote of the committed prose — byte-exact
    /// from the committed fixture (`worked-examples.md` → Superseding decision).
    #[test]
    fn superseded_context_slices_the_prior_decision() {
        let repo = TempRoot::new("slice-repo");
        let task = TempRoot::new("slice-task");
        commit_adr_a(repo.path());
        stage_adr_b(task.path(), Some("adr:single-node-cache"));

        let schemas = adr_schemas();
        let committed = index::rebuild_committed(repo.path(), &schemas, "HEAD");
        let overlay = index::overlay_working(&committed, task.path(), &schemas);
        let store = store_ctx(repo.path(), &schemas, &overlay);

        let catalog = CommandCatalog {
            commands: BTreeMap::new(),
        };
        let body = "{{ @task.decision.supersedes#decision }}\n";
        let emitted = emit_step_body_with(body, &ctx_decision_bound(), &catalog, Some(&store))
            .expect("emits");

        insta::assert_snapshot!(emitted, @r"
        > A single in-memory node keeps session lookups sub-millisecond and avoids a
        > network hop; acceptable because sessions are cheap to reconstruct on a cold node.
        ");
    }

    /// FLOW #1 / no-edge: with `task.decision` bound but `supersedes` **unset** (no
    /// edge), and with `task.decision` **unbound**, the same placeholder emits an
    /// empty line — no `> ` blockquote, zero findings (empty-vs-unresolvable).
    #[test]
    fn superseded_context_empty_when_no_edge() {
        let repo = TempRoot::new("empty-repo");
        let schemas = adr_schemas();
        let catalog = CommandCatalog {
            commands: BTreeMap::new(),
        };
        let body = "before\n{{ @task.decision.supersedes#decision }}\nafter\n";

        // (1) Bound decision, but `supersedes` unset → no edge → empty.
        let task_unset = TempRoot::new("empty-task-unset");
        stage_adr_b(task_unset.path(), None);
        let committed = index::rebuild_committed(repo.path(), &schemas, "HEAD");
        let overlay_unset = index::overlay_working(&committed, task_unset.path(), &schemas);
        let store_unset = store_ctx(repo.path(), &schemas, &overlay_unset);
        let emitted_unset =
            emit_step_body_with(body, &ctx_decision_bound(), &catalog, Some(&store_unset))
                .expect("emits");
        assert_eq!(
            emitted_unset, "before\n\nafter\n",
            "an unset `supersedes` emits an empty line, no `>`: {emitted_unset:?}"
        );

        // (2) Unbound decision role → absent → empty, even with a store present.
        let task_unbound = TempRoot::new("empty-task-unbound");
        let overlay_unbound = index::overlay_working(&committed, task_unbound.path(), &schemas);
        let store_unbound = store_ctx(repo.path(), &schemas, &overlay_unbound);
        let mut roles = BTreeMap::new();
        roles.insert("decision".to_owned(), None); // declared, unbound.
        let ctx_unbound = crate::data_value::ComposeContext {
            task: Some(crate::data_value::TaskRoot {
                id: "t".to_owned(),
                intent: "i".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        };
        let emitted_unbound =
            emit_step_body_with(body, &ctx_unbound, &catalog, Some(&store_unbound)).expect("emits");
        assert_eq!(
            emitted_unbound, "before\n\nafter\n",
            "an unbound decision role emits an empty line: {emitted_unbound:?}"
        );
        assert!(
            !emitted_unbound.contains('>'),
            "no blockquote when the role is unbound"
        );
    }

    // -- The `reads` cross-task binding spine (inc-2 T4 acceptance) ----------
    //
    // A `reads: [{role: spec, type: spec}]`-declaring workflow makes `task.spec`
    // a declared `workflow-refs` root; `jigc task bind spec spec:<slug>` records
    // the binding in `roles.json`; on resume re-compose `{{@task.spec#criteria}}`
    // dereferences the bound role straight to the committed spec's `#criteria`
    // slice (zero relation hops — the role *is* the target), emitted as a `> `
    // Content blockquote (`worked-examples.md` → flows 5/6; `write-commands.md` →
    // Binding a context role / Resolution timing). Declared-but-unbound → empty.

    /// A stub `reads`-declaring workflow def: `reads: [{role: spec, type: spec}]`,
    /// one include of the (in-test) `read-the-spec` step. The composition order
    /// is the include list — the step body is supplied by [`ReadsStepSource`].
    fn reads_spec_def() -> WorkflowDef {
        WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: Vec::new(),
            reads: vec![Reads {
                role: "spec".to_owned(),
                doc_type: "spec".to_owned(),
            }],
            includes: vec!["read-the-spec".to_owned()],
        }
    }

    /// The stub pack's step source: the lone `read-the-spec` step whose body is the
    /// bound-role context-slice placeholder.
    struct ReadsStepSource;

    impl StepSource for ReadsStepSource {
        fn step(&self, id: &str) -> Option<StepDef> {
            (id == "read-the-spec").then(|| StepDef {
                id: id.to_owned(),
                body: "Here are the acceptance criteria you must satisfy:\n\
                       {{ @task.spec#criteria }}\n"
                    .to_owned(),
                kind: StepKind::Plain,
            })
        }
    }

    /// A stub `spec` schema with a `header` section and a `criteria` **slot**
    /// section (so the committed `#criteria` slice is store-readable — `read_slice`
    /// slices slot sections). Persisted to `specs/`, the doctype-map location. This
    /// is the test pack's spec, NOT the shipped `spec.yaml` (whose `criteria` is
    /// repeatable) — the acceptance proves the binding spine, fixtures over pack.
    const STUB_SPEC_YAML: &[u8] = b"\
type: spec
location: specs/
id-from: title

sections:
  - id: header
    header: true
    fields:
      - { id: title, type: string }
  - id: criteria
    slot: { hint: \"The acceptance criteria, testably phrased.\" }
";

    fn spec_schemas() -> BTreeMap<String, Schema> {
        let mut m = BTreeMap::new();
        m.insert(
            "spec".to_string(),
            crate::schema::load_schema(STUB_SPEC_YAML).expect("stub spec schema loads"),
        );
        m
    }

    /// The committed spec the `reads` role binds to (`specs/payment-retry.md`),
    /// carrying the `#criteria` prose the slice must surface.
    const COMMITTED_SPEC: &str = "\
---
---

# Payment retry

## Criteria
A failed charge retries with exponential backoff, capped at five attempts.
";

    /// Commit the spec at `specs/payment-retry.md` under `repo_root`.
    fn commit_spec(repo_root: &Path) {
        let dir = repo_root.join("specs");
        std::fs::create_dir_all(&dir).expect("mk specs/");
        std::fs::write(dir.join("payment-retry.md"), COMMITTED_SPEC).expect("write spec");
    }

    /// Build the resume `ComposeContext` for a task whose workflow `reads` `def`,
    /// declaring each `reads` role and binding it iff `roles.json` recorded a
    /// `jigc task bind` — the engine-seam mirror of the CLI's `build_context`
    /// (`start.rs`), so the test exercises the same declared-vs-bound logic at
    /// the `compose_with_store` seam.
    fn reads_ctx(
        def: &WorkflowDef,
        bound: &crate::state::RolesRecord,
    ) -> crate::data_value::ComposeContext {
        let mut roles = BTreeMap::new();
        for entry in &def.reads {
            let binding = bound.get(&entry.role).and_then(|a| Address::parse(a).ok());
            roles.insert(entry.role.clone(), binding);
        }
        crate::data_value::ComposeContext {
            task: Some(crate::data_value::TaskRoot {
                id: "implement-payment-retry".to_owned(),
                intent: "implement the payment retry policy".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        }
    }

    /// HEADLINE (inc-2 T4): a `reads: [{role: spec, type: spec}]` workflow + a
    /// committed `specs/payment-retry.md`; `jigc task bind` recorded
    /// `spec -> spec:payment-retry` in `roles.json`; on resume re-compose,
    /// `{{@task.spec#criteria}}` dereferences the bound role straight to the
    /// committed spec's `#criteria` slice — emitted as a `> ` Content blockquote
    /// of the committed prose (NOT the bare `spec:payment-retry#criteria` handle).
    #[test]
    fn bound_reads_role_slice_resolves_on_resume() {
        let repo = TempRoot::new("reads-repo");
        let task = TempRoot::new("reads-task");
        commit_spec(repo.path());

        // The `jigc task bind spec spec:payment-retry` binding, persisted in the
        // task's `roles.json` (read back on resume).
        let mut bound = crate::state::RolesRecord::new();
        bound.bind("spec", "spec:payment-retry");
        bound.save(task.path()).expect("save roles.json");
        let bound = crate::state::RolesRecord::load(task.path()).expect("roles.json loads");

        let def = reads_spec_def();
        let schemas = spec_schemas();
        let committed = index::rebuild_committed(repo.path(), &schemas, "HEAD");
        let overlay = index::overlay_working(&committed, task.path(), &schemas);
        let store = store_ctx(repo.path(), &schemas, &overlay);
        let catalog = CommandCatalog {
            commands: BTreeMap::new(),
        };

        let composed = compose_with_store(
            &def,
            &ReadsStepSource,
            &catalog,
            &reads_ctx(&def, &bound),
            Some(&store),
        )
        .expect("composes");

        // The committed `#criteria` prose is sliced and emitted as a `> ` blockquote.
        assert!(
            composed.text.contains(
                "> A failed charge retries with exponential backoff, capped at five attempts."
            ),
            "the bound `reads` role must slice the committed spec's `#criteria` prose \
             as a `> ` blockquote; got:\n{}",
            composed.text
        );
        // NOT the bare address handle — the slice dereferences to prose.
        assert!(
            !composed.text.contains("> spec:payment-retry#criteria"),
            "the slice must dereference to prose, NOT emit the bare address handle; got:\n{}",
            composed.text
        );
    }

    /// The declared-but-unbound case: the same `reads: [{role: spec, type: spec}]`
    /// workflow with **no** `roles.json` binding resolves `{{@task.spec#criteria}}`
    /// to an **empty line** — no `> ` blockquote, zero findings (the
    /// empty-vs-unresolvable contract; `workflow-dialect.md` → `reads`).
    #[test]
    fn unbound_reads_role_resolves_empty() {
        let repo = TempRoot::new("reads-empty-repo");
        let task = TempRoot::new("reads-empty-task");
        commit_spec(repo.path());

        let def = reads_spec_def();
        let schemas = spec_schemas();
        let committed = index::rebuild_committed(repo.path(), &schemas, "HEAD");
        let overlay = index::overlay_working(&committed, task.path(), &schemas);
        let store = store_ctx(repo.path(), &schemas, &overlay);
        let catalog = CommandCatalog {
            commands: BTreeMap::new(),
        };

        // No `jigc task bind` ran → `roles.json` records nothing for `spec`.
        let bound = crate::state::RolesRecord::new();
        let composed = compose_with_store(
            &def,
            &ReadsStepSource,
            &catalog,
            &reads_ctx(&def, &bound),
            Some(&store),
        )
        .expect("composes");

        assert!(
            !composed.text.contains('>'),
            "a declared-but-unbound `reads` role emits no blockquote; got:\n{}",
            composed.text
        );
        // The `@`-slice resolves to nothing — none of the committed prose surfaces.
        assert!(
            !composed.text.contains("exponential backoff"),
            "an unbound `reads` role surfaces NO committed prose; got:\n{}",
            composed.text
        );
        // The surrounding instruction prose is still emitted; only the slice is empty.
        assert!(
            composed
                .text
                .contains("Here are the acceptance criteria you must satisfy:"),
            "the step's instruction prose survives the empty slice; got:\n{}",
            composed.text
        );
    }

    proptest::proptest! {
        /// Emission is a **pure function** of `(body, ctx, catalog)`: emitting the
        /// same inputs twice yields identical text (the determinism boundary). The
        /// generator interleaves the four line classes in arbitrary order.
        #[test]
        fn emit_is_pure(
            lines in proptest::collection::vec(
                proptest::sample::select(vec![
                    "plain reason prose",
                    "{{ cli.finalize-task }}",
                    "{{ @task.commit#summary }}",
                    "{{ @task.decision.supersedes#decision }}",
                    "<<author: {{ task.commit#summary }}>>",
                ]),
                0..12,
            )
        ) {
            let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
            let ctx = emit_ctx();
            let body = lines.join("\n");
            let first = emit_step_body(&body, &ctx, &catalog);
            let second = emit_step_body(&body, &ctx, &catalog);
            proptest::prop_assert_eq!(first, second);
        }
    }

    /// The shipped command catalog bytes — kept in sync with
    /// `crates/cli/pack/config/commands.yaml` (asserted byte-identical below).
    const COMMANDS_YAML: &[u8] = include_bytes!("../../cli/pack/config/commands.yaml");

    /// Core done-criterion: the shipped `commands.yaml` loads with all four
    /// command-refs present and each of the three arg kinds carried correctly —
    /// a `from:` arg carries its data-value-path string, an `agent:` arg carries
    /// a non-empty hint, a literal stays a string. The golden pins the full serde
    /// projection of the loaded catalog (id-keyed, deterministic), so a field
    /// rename, a reorder, or an arg-kind mis-route breaks it.
    #[test]
    fn command_catalog_loads_three_arg_kinds() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");

        // Every shipped command-ref is present under its id.
        for id in [
            "set-commit-summary",
            "validate-task",
            "finalize-task",
            "create-adr",
            "create-spec",
            "bind-spec",
            "recompose-task",
            "run-ingest",
        ] {
            assert!(catalog.get(id).is_some(), "missing command-ref `{id}`");
        }

        // A `from:` arg carries a data-value path (verbatim string).
        let set_commit = catalog.get("set-commit-summary").expect("present");
        assert!(set_commit.args.contains(&CommandArg::From {
            from: "task.commit#summary".to_owned(),
        }));

        // An `agent:` arg carries a non-empty hint.
        let create_adr = catalog.get("create-adr").expect("present");
        let agent_arg = create_adr
            .args
            .iter()
            .find_map(|a| match a {
                CommandArg::Agent { agent, hint } => Some((agent, hint)),
                _ => None,
            })
            .expect("create-adr has an agent arg");
        assert_eq!(agent_arg.0, "title");
        assert!(!agent_arg.1.trim().is_empty());

        // A literal stays a string.
        assert!(set_commit.args.contains(&CommandArg::Literal {
            literal: "doc".to_owned(),
        }));

        let json = serde_json::to_string_pretty(&catalog).expect("serializes");
        insta::assert_snapshot!(json, @r#"
        {
          "commands": {
            "bind-spec": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "task"
                },
                {
                  "kind": "literal",
                  "literal": "bind"
                },
                {
                  "kind": "literal",
                  "literal": "spec"
                },
                {
                  "kind": "agent",
                  "agent": "spec_id",
                  "hint": "the spec:<slug> id picked from the committed specs above"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Bind the spec this work implements to the task's spec role."
            },
            "create-adr": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "create"
                },
                {
                  "kind": "literal",
                  "literal": "adr"
                },
                {
                  "kind": "literal",
                  "literal": "--title"
                },
                {
                  "kind": "agent",
                  "agent": "title",
                  "hint": "short declarative sentence describing the decision"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Create a new ADR in the current task."
            },
            "create-arch-doc": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "create"
                },
                {
                  "kind": "literal",
                  "literal": "arch-doc"
                },
                {
                  "kind": "literal",
                  "literal": "--title"
                },
                {
                  "kind": "agent",
                  "agent": "title",
                  "hint": "short declarative title naming the part of the system the arch-doc documents"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Create a new arch-doc in the current task."
            },
            "create-changelog": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "create"
                },
                {
                  "kind": "literal",
                  "literal": "changelog"
                },
                {
                  "kind": "literal",
                  "literal": "--title"
                },
                {
                  "kind": "literal",
                  "literal": "Changelog"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Create-or-update the running changelog singleton in the current task."
            },
            "create-prd": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "create"
                },
                {
                  "kind": "literal",
                  "literal": "prd"
                },
                {
                  "kind": "literal",
                  "literal": "--title"
                },
                {
                  "kind": "agent",
                  "agent": "title",
                  "hint": "short declarative title naming the product the prd specifies"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Create a new prd in the current task."
            },
            "create-spec": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "create"
                },
                {
                  "kind": "literal",
                  "literal": "spec"
                },
                {
                  "kind": "literal",
                  "literal": "--title"
                },
                {
                  "kind": "agent",
                  "agent": "title",
                  "hint": "short declarative title naming what the spec delivers"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Create a new spec in the current task."
            },
            "finalize-task": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "task"
                },
                {
                  "kind": "literal",
                  "literal": "finalize"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Run validate + commit. One task → one commit."
            },
            "milestone-finalize": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "milestone"
                },
                {
                  "kind": "literal",
                  "literal": "finalize"
                },
                {
                  "kind": "agent",
                  "agent": "milestone_id",
                  "hint": "the milestone:<slug> id being executed"
                }
              ],
              "hint": "The milestone commit boundary — validate the merged join + commit per the squash knob."
            },
            "milestone-provision": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "milestone"
                },
                {
                  "kind": "literal",
                  "literal": "provision"
                },
                {
                  "kind": "agent",
                  "agent": "milestone_id",
                  "hint": "the milestone:<slug> id being executed"
                }
              ],
              "hint": "Provision one detached base-pin worktree per sub-task before the fan-out."
            },
            "recompose-task": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "start"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Re-compose the task to pick up the freshly bound slice."
            },
            "run-ingest": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "ingest"
                }
              ],
              "hint": "Scan the repo, classify candidate docs, and report the triage verdicts."
            },
            "set-commit-summary": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "set-slot"
                },
                {
                  "kind": "from",
                  "from": "task.commit#summary"
                },
                {
                  "kind": "literal",
                  "literal": "--from-file"
                },
                {
                  "kind": "literal",
                  "literal": "-"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "stdin": "the slot prose",
              "hint": "Stage the commit summary slot from stdin."
            },
            "set-commit-type": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "doc"
                },
                {
                  "kind": "literal",
                  "literal": "set-field"
                },
                {
                  "kind": "from",
                  "from": "task.commit#type"
                },
                {
                  "kind": "literal",
                  "literal": "--value"
                },
                {
                  "kind": "agent",
                  "agent": "commit_type",
                  "hint": "the Conventional-Commits type for what changed: feat, fix, docs, style, refactor, perf, test, build, ci, chore, or revert"
                },
                {
                  "kind": "literal",
                  "literal": "--task"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Set the required Conventional-Commits type."
            },
            "validate-task": {
              "command": "jigc",
              "args": [
                {
                  "kind": "literal",
                  "literal": "task"
                },
                {
                  "kind": "literal",
                  "literal": "validate"
                },
                {
                  "kind": "from",
                  "from": "task.id"
                }
              ],
              "hint": "Preview validation findings without committing."
            }
          }
        }
        "#);
    }

    /// An `agent:` arg with an empty `hint` is a typed malformed-arg error — the
    /// agent must know what to substitute (`command-catalog.md` → Validation).
    #[test]
    fn command_catalog_rejects_agent_arg_with_empty_hint() {
        let yaml = "\
commands:
  - id: bad
    command: jigc
    args:
      - { agent: title, hint: \"   \" }
    hint: a hint
";
        let err = load_command_catalog(yaml.as_bytes()).expect_err("empty hint rejected");
        assert_eq!(err.code, "workflow-refs.malformed-command-arg");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);
    }

    /// An arg map with an unknown key (neither `from:` nor `agent:`) is a typed
    /// malformed-arg error.
    #[test]
    fn command_catalog_rejects_unknown_arg_key() {
        let yaml = "\
commands:
  - id: bad
    command: jigc
    args:
      - { wat: nonsense }
    hint: a hint
";
        let err = load_command_catalog(yaml.as_bytes()).expect_err("unknown arg key rejected");
        assert_eq!(err.code, "workflow-refs.malformed-command-arg");
    }

    /// The in-test catalog const stays byte-identical to the shipped pack file.
    #[test]
    fn shipped_commands_yaml_is_byte_identical() {
        let shipped = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../cli/pack/config/commands.yaml"
        ))
        .expect("shipped commands.yaml reads");
        assert_eq!(COMMANDS_YAML, shipped.as_slice());
    }

    /// The shipped single-task definition bytes — the embedded pack file this
    /// composer loads. Kept in sync with `crates/cli/pack/workflows/single-task.yaml`.
    const SINGLE_TASK: &str = "\
---
when: implement one scoped change end-to-end
creates-task: true
allows-create: [{type: adr, as: decision}]
---
{{ include: step:locate }}
{{ include: step:implement }}
{{ include: step:superseded-context }}
{{ include: step:finalize }}
";

    /// Core done-criterion: loading the shipped single-task bytes yields a
    /// `WorkflowDef` with `creates_task=true`, `allows_create=[{adr, decision}]`,
    /// and the four includes in body order. The golden pins the full serde
    /// projection so a field rename or reorder breaks it.
    #[test]
    fn workflow_def_parses_includes_and_front_matter() {
        let def = load_workflow_def(SINGLE_TASK.as_bytes()).expect("loads");

        assert!(def.creates_task);
        assert_eq!(
            def.allows_create,
            vec![AllowsCreate {
                doc_type: "adr".to_owned(),
                as_role: "decision".to_owned(),
            }]
        );
        assert_eq!(
            def.includes,
            vec!["locate", "implement", "superseded-context", "finalize"]
        );

        let json = serde_json::to_string_pretty(&def).expect("serializes");
        insta::assert_snapshot!(json, @r#"
        {
          "when": "implement one scoped change end-to-end",
          "description": null,
          "usage": null,
          "creates_task": true,
          "selectable": true,
          "allows_create": [
            {
              "type": "adr",
              "as": "decision"
            }
          ],
          "reads": [],
          "includes": [
            "locate",
            "implement",
            "superseded-context",
            "finalize"
          ]
        }
        "#);
    }

    /// Authored `description:`/`usage:` front-matter prose round-trips onto the
    /// loaded `WorkflowDef` — the substrate `describe` projects. `WorkflowFrontMatter`
    /// is **not** `deny_unknown_fields`, so this proves the loader is *wired*
    /// rather than silently dropping the fields (the "authored it but describe
    /// shows nothing" trap; `introspection.md` → The authored fields).
    #[test]
    fn description_and_usage_round_trip() {
        let def = load_workflow_def(
            b"---\nwhen: x\ndescription: one end-to-end scoped change\nusage: when the work is one coherent change\n---\n{{ include: step:locate }}\n",
        )
        .expect("loads");
        assert_eq!(
            def.description.as_deref(),
            Some("one end-to-end scoped change")
        );
        assert_eq!(
            def.usage.as_deref(),
            Some("when the work is one coherent change")
        );
    }

    /// Omitting `description:`/`usage:` yields `None` on both — no error
    /// (skip-on-absent is the runtime contract; `introspection.md`).
    #[test]
    fn description_and_usage_default_none_when_omitted() {
        let def =
            load_workflow_def(b"---\nwhen: x\n---\n{{ include: step:locate }}\n").expect("loads");
        assert_eq!(def.description, None);
        assert_eq!(def.usage, None);
    }

    /// A `reads: [{role: spec, type: spec}]`-declaring workflow front-matter
    /// yields `WorkflowDef.reads == [Reads{role:"spec", type:"spec"}]` — the dual
    /// of `allows-create` (`workflow-dialect.md` → On-disk definition format).
    #[test]
    fn reads_declaration_parses() {
        let def = load_workflow_def(
            b"---\nwhen: implement from a spec\nreads: [{role: spec, type: spec}]\n---\n{{ include: step:locate-from-spec }}\n",
        )
        .expect("loads");
        assert_eq!(
            def.reads,
            vec![Reads {
                role: "spec".to_owned(),
                doc_type: "spec".to_owned(),
            }]
        );
    }

    /// An omitted `reads:` key defaults to an empty list (`workflow-dialect.md` →
    /// On-disk definition format: default empty).
    #[test]
    fn reads_defaults_empty_when_omitted() {
        let def =
            load_workflow_def(b"---\nwhen: pick a workflow\n---\n{{ include: step:present }}\n")
                .expect("loads");
        assert!(def.reads.is_empty());
    }

    /// `creates-task` defaults to `true` when the front-matter omits the key
    /// (`workflow-dialect.md` → On-disk format).
    #[test]
    fn creates_task_defaults_true_when_omitted() {
        let def =
            load_workflow_def(b"---\nwhen: pick a workflow\n---\n{{ include: step:present }}\n")
                .expect("loads");
        assert!(def.creates_task);
        assert!(def.allows_create.is_empty());
    }

    /// A `creates-task: false` workflow (the router shape) parses with the flag off.
    #[test]
    fn creates_task_false_parses() {
        let def = load_workflow_def(
            b"---\nwhen: help me pick\ncreates-task: false\n---\n{{ include: step:route }}\n",
        )
        .expect("loads");
        assert!(!def.creates_task);
    }

    /// Blank lines and HTML-comment lines are dropped; only include ids survive,
    /// in physical order.
    #[test]
    fn blanks_and_comments_are_dropped() {
        let def = load_workflow_def(
            b"---\nwhen: x\n---\n\n{{ include: step:a }}\n<!-- why b -->\n{{ include: step:b }}\n\n",
        )
        .expect("loads");
        assert_eq!(def.includes, vec!["a", "b"]);
    }

    /// A prose body line is a blocking `body-include-only` finding located at the
    /// offending source line — the `body-include-only` precursor.
    #[test]
    fn prose_body_line_is_a_blocking_finding() {
        let err = load_workflow_def(
            b"---\nwhen: x\n---\n{{ include: step:a }}\nthis is prose, not an include\n",
        )
        .expect_err("prose rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);
        // Front-matter is 3 lines (`---`, `when: x`, `---`); body line 1 is the
        // include, line 2 is the prose → source line 5.
        assert_eq!(err.location.as_ref().expect("located").line, 5);
    }

    /// An ATX heading at body top-level is rejected (it is not an include).
    #[test]
    fn heading_body_line_is_rejected() {
        let err =
            load_workflow_def(b"---\nwhen: x\n---\n## a heading\n").expect_err("heading rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
    }

    /// A non-include placeholder (a data-value, not an include) at top level is
    /// rejected — the body is include-*only*.
    #[test]
    fn non_include_placeholder_is_rejected() {
        let err = load_workflow_def(b"---\nwhen: x\n---\n{{ task.intent }}\n")
            .expect_err("data-value rejects");
        assert_eq!(err.code, "workflow-refs.body-include-only");
    }

    /// Missing front-matter is a clear blocking finding.
    #[test]
    fn missing_front_matter_is_a_blocking_finding() {
        let err = load_workflow_def(b"{{ include: step:a }}\n").expect_err("no front-matter");
        assert_eq!(err.code, "workflow-refs.missing-front-matter");
    }

    /// The shipped `locate` step body — kept in sync with
    /// `crates/cli/pack/steps/locate.yaml` (a plain, front-matter-less step).
    const STEP_LOCATE: &str = "\
Reason about the change. The intent is:
{{ task.intent }}

The relevant code paths are not yet known. Inspect the codebase to confirm
scope before implementing.
";

    /// The shipped `superseded-context` step body — kept in sync with
    /// `crates/cli/pack/steps/superseded-context.yaml` (also front-matter-less).
    const STEP_SUPERSEDED: &str = "\
If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).
{{ @task.decision.supersedes#decision }}
";

    /// Core done-criterion: a front-matter-less step loads with its full body as
    /// prose, verbatim; a step preceded by a `---`-fenced front-matter block splits
    /// cleanly, with the body preserved byte-for-byte (the post-fence remainder).
    /// Goldens over the two shipped plain step bodies pin the canonical bytes.
    #[test]
    fn step_def_loads_body_verbatim_with_optional_front_matter() {
        // A plain step (no leading `---` fence): id is the resource id, body is the
        // whole input verbatim.
        let locate = load_step_def("locate", STEP_LOCATE.as_bytes()).expect("loads");
        assert_eq!(locate.id, "locate");
        assert_eq!(locate.body, STEP_LOCATE);
        insta::assert_snapshot!(locate.body, @r###"
        Reason about the change. The intent is:
        {{ task.intent }}

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.
        "###);

        let superseded =
            load_step_def("superseded-context", STEP_SUPERSEDED.as_bytes()).expect("loads");
        assert_eq!(superseded.id, "superseded-context");
        assert_eq!(superseded.body, STEP_SUPERSEDED);
        insta::assert_snapshot!(superseded.body, @r###"
        If your decision supersedes an earlier one, set `supersedes` on the ADR; the
        superseded decision then appears below for reference, so your consequences can
        explain what changes (nothing appears if it supersedes none).
        {{ @task.decision.supersedes#decision }}
        "###);

        // A step *with* front-matter: the body is exactly the post-fence remainder,
        // byte-for-byte — the front-matter (config-family YAML) is parsed into the
        // step kind, the prose preserved verbatim including its internal blank lines.
        let fenced = load_step_def(
            "fan-out-step",
            b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-task per item.\n\nEach runs the sub-workflow.\n",
        )
        .expect("loads");
        assert_eq!(fenced.id, "fan-out-step");
        assert_eq!(
            fenced.body,
            "Spawn a sub-task per item.\n\nEach runs the sub-workflow.\n"
        );
        // The `fan-out:` marker is now **parsed and honored**, not stripped to inert
        // prose: the kind carries `over` (the verbatim `{{ milestone.tasks }}` text)
        // and `run` (the `workflow:sub-task` string) — M8 (`workflow-dialect.md` →
        // On-disk definition format; `DECISIONS.md` 2026-06-04).
        assert_eq!(
            fenced.kind,
            StepKind::FanOut {
                over: "{{ milestone.tasks }}".to_owned(),
                run: "workflow:sub-task".to_owned(),
            }
        );
    }

    /// A plain step (no front-matter) loads as `StepKind::Plain` — the kindless
    /// default (`workflow-dialect.md` → On-disk definition format: "a plain step
    /// stays kindless").
    #[test]
    fn plain_step_has_plain_kind() {
        let locate = load_step_def("locate", STEP_LOCATE.as_bytes()).expect("loads");
        assert_eq!(locate.kind, StepKind::Plain);
    }

    /// The done-criterion `join: {}` shape: a `join` step loads as `StepKind::Join`
    /// with its body intact (`workflow-dialect.md` → On-disk definition format).
    #[test]
    fn join_step_has_join_kind() {
        let join = load_step_def(
            "join-tasks",
            b"---\njoin: {}\n---\nAll sub-tasks are complete and merged by task-id order. Continue.\n",
        )
        .expect("loads");
        assert_eq!(join.kind, StepKind::Join);
        assert_eq!(
            join.body,
            "All sub-tasks are complete and merged by task-id order. Continue.\n"
        );
    }

    /// Both `fan-out` and `join` markers on one step is a blocking, **located**
    /// conformance finding — a step is one kind (`workflow-dialect.md` → On-disk
    /// definition format: the two step kinds are distinct markers).
    #[test]
    fn both_markers_is_blocking_located_finding() {
        let err = load_step_def(
            "confused",
            b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\njoin: {}\n---\nbody\n",
        )
        .expect_err("both markers rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// A `fan-out` marker missing `over` is a blocking located finding (both `over`
    /// and `run` are required — `workflow-dialect.md` → the `fan-out` step declares
    /// a list-source and a referenced sub-workflow).
    #[test]
    fn fan_out_missing_over_is_blocking_located_finding() {
        let err = load_step_def(
            "no-over",
            b"---\nfan-out:\n  run: workflow:sub-task\n---\nbody\n",
        )
        .expect_err("missing over rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// A `fan-out` marker missing `run` is a blocking located finding.
    #[test]
    fn fan_out_missing_run_is_blocking_located_finding() {
        let err = load_step_def(
            "no-run",
            b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n---\nbody\n",
        )
        .expect_err("missing run rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// The M15 done-criterion: a `checkpoint: {reason: <slug>}` step loads as
    /// `StepKind::Checkpoint { reason }` with its body intact (`workflow-dialect.md`
    /// → The checkpoint step kind — `checkpoint:` carries one field, `reason`).
    #[test]
    fn checkpoint_step_has_checkpoint_kind() {
        let cp = load_step_def(
            "plan-gate",
            b"---\ncheckpoint:\n  reason: new-fork-at-plan\n---\nIf planning surfaced a genuinely new fork, stop and surface it.\n",
        )
        .expect("loads");
        assert_eq!(
            cp.kind,
            StepKind::Checkpoint {
                reason: "new-fork-at-plan".to_owned(),
            }
        );
        assert_eq!(
            cp.body,
            "If planning surfaced a genuinely new fork, stop and surface it.\n"
        );
    }

    /// The serde-tagged golden discriminant for the new variant: `StepKind` is
    /// internally tagged (`#[serde(tag = "kind", rename_all = "kebab-case")]`), so
    /// `Checkpoint { reason }` projects to `{"kind": "checkpoint", "reason": …}` —
    /// an unambiguous discriminant a rename would break.
    #[test]
    fn checkpoint_kind_serde_discriminant() {
        let kind = StepKind::Checkpoint {
            reason: "new-fork-at-plan".to_owned(),
        };
        let json = serde_json::to_string_pretty(&kind).expect("serializes");
        insta::assert_snapshot!(json, @r#"
        {
          "kind": "checkpoint",
          "reason": "new-fork-at-plan"
        }
        "#);
    }

    /// T2 done-criterion (`workflow-dialect.md` → Emitted format, rule 5: the
    /// sixth emit class): a `checkpoint` step's emitted text **opens** with a
    /// `Checkpoint: <reason>\n` directive line — the slug bare, no backticks —
    /// then the body prose, the directive **prepended** before the body (the
    /// inverse of `fan-out`'s `Spawn:` append). The same body with the marker
    /// stripped (a plain step, byte-identical prose) composes **without** that
    /// line — the M8 face-#5 parsed-but-ignored guard at the engine seam: the
    /// composed output must differ with vs. without the marker.
    #[test]
    fn checkpoint_step_prepends_directive() {
        let body = "If planning surfaced a genuinely new fork, stop and surface it.\n";
        let with_marker = MapSource::new(&[(
            "plan-gate",
            "---\ncheckpoint:\n  reason: new-fork-at-plan\n---\nIf planning surfaced a genuinely new fork, stop and surface it.\n",
        )]);
        let without_marker = MapSource::new(&[("plan-gate", body)]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["plan-gate".to_owned()],
        };
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        let composed = compose(&def, &with_marker, &catalog, &ctx).expect("composes");
        assert_eq!(
            composed.text,
            format!("Checkpoint: new-fork-at-plan\n{body}"),
            "the directive prepends before the body prose, slug bare"
        );

        // The same body, marker stripped, composes WITHOUT the directive — the
        // with-vs-without delta at the engine seam.
        let plain = compose(&def, &without_marker, &catalog, &ctx).expect("composes");
        assert_eq!(plain.text, body, "a plain step emits no Checkpoint: line");
        assert_ne!(
            composed.text, plain.text,
            "composed output must differ with vs. without the checkpoint marker"
        );
    }

    /// `checkpoint` alongside any other marker (`fan-out` here) is a blocking,
    /// **located** conformance finding — a step is exactly one kind, now a
    /// three-marker mutual-exclusivity check (`workflow-dialect.md` → Engine
    /// reality: `parse_step_kind` becomes a three-marker mutual-exclusivity check).
    #[test]
    fn checkpoint_plus_fan_out_is_blocking_located_finding() {
        let err = load_step_def(
            "confused-cp",
            b"---\ncheckpoint:\n  reason: x\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\n---\nbody\n",
        )
        .expect_err("checkpoint + fan-out rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// `checkpoint` alongside `join` is equally rejected — the mutual-exclusivity
    /// holds across every pair of the three markers.
    #[test]
    fn checkpoint_plus_join_is_blocking_located_finding() {
        let err = load_step_def(
            "confused-cp-join",
            b"---\ncheckpoint:\n  reason: x\njoin: {}\n---\nbody\n",
        )
        .expect_err("checkpoint + join rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// A `checkpoint` marker with an empty `reason` is rejected — the `reason` slug
    /// is required (mirrors `fan-out`'s required `over`/`run`).
    #[test]
    fn checkpoint_empty_reason_is_blocking_located_finding() {
        let err = load_step_def(
            "blank-reason",
            b"---\ncheckpoint:\n  reason: \"   \"\n---\nbody\n",
        )
        .expect_err("empty reason rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    /// A `checkpoint` marker missing `reason` entirely is likewise rejected.
    #[test]
    fn checkpoint_missing_reason_is_blocking_located_finding() {
        let err = load_step_def("no-reason", b"---\ncheckpoint: {}\n---\nbody\n")
            .expect_err("missing reason rejected");
        assert_eq!(err.code, "workflow-refs.step-kind-malformed");
        assert!(err.location.is_some(), "finding is located");
    }

    proptest::proptest! {
        /// Body bytes survive load unchanged: for arbitrary UTF-8 prose with no
        /// leading `---\n` fence, `StepDef.body` equals the input verbatim.
        #[test]
        fn front_matter_less_body_survives_load_verbatim(
            prose in "[^\\x00]{0,200}"
        ) {
            // Exclude inputs that happen to open with a front-matter fence — those
            // are the *with-front-matter* case, exercised separately below.
            proptest::prop_assume!(!prose.starts_with("---\n"));
            let def = load_step_def("s", prose.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.body, prose);
        }

        /// Body bytes survive load unchanged for the *with-front-matter* case: for
        /// arbitrary prose preceded by a fenced front-matter block, `StepDef.body`
        /// equals the post-fence remainder verbatim.
        #[test]
        fn fenced_body_is_post_fence_remainder_verbatim(
            body in "[^\\x00]{0,200}"
        ) {
            // A body that itself opens with a `---` line would be ambiguous with
            // the front-matter's closing fence, so skip those; they cannot occur as
            // the first body content after a real closing fence anyway.
            proptest::prop_assume!(!body.starts_with("---"));
            let input = format!("---\nkey: value\n---\n{body}");
            let def = load_step_def("s", input.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.body, body);
        }
    }

    proptest::proptest! {
        /// Round-trip property: an include-only body of N shuffled `{{ include:
        /// step:<slug> }}` lines, interleaved with blanks and HTML comments, parses
        /// to exactly the input id list in order, with blanks/comments dropped.
        #[test]
        fn include_only_bodies_round_trip_to_ordered_ids(
            ids in proptest::collection::vec("[a-z][a-z0-9-]{0,7}", 0..8)
        ) {
            // Build a body: each id on its own include line, with a blank line and
            // a comment between consecutive ids (both must be dropped).
            let mut body = String::from("---\nwhen: x\n---\n");
            for (i, id) in ids.iter().enumerate() {
                if i > 0 {
                    body.push('\n');
                    body.push_str("<!-- note -->\n");
                }
                body.push_str(&format!("{{{{ include: step:{id} }}}}\n"));
            }
            let def = load_workflow_def(body.as_bytes()).expect("loads");
            proptest::prop_assert_eq!(def.includes, ids);
        }
    }

    use crate::data_value::{ComposeContext, TaskRoot};

    /// The `command-catalog.md` worked context: a task `add-rate-limiter` whose
    /// `commit` role is bound (a `creates-task` workflow's commit doc carries the
    /// task's own slug). `task.id` and `task.intent` are the engine-native scalars.
    fn add_rate_limiter_ctx() -> ComposeContext {
        let mut roles = std::collections::BTreeMap::new();
        roles.insert(
            "commit".to_owned(),
            Some(crate::address::Address::parse("commit:add-rate-limiter").expect("valid address")),
        );
        ComposeContext {
            task: Some(TaskRoot {
                id: "add-rate-limiter".to_owned(),
                intent: "add a rate limiter to the API".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        }
    }

    /// Core done-criterion (`command_ref_renders_shell_safe`): the shipped
    /// command-refs render to the exact shell-safe lines of `command-catalog.md`
    /// → What the renderer emits, with the `Run:` wrapper *not* included (the
    /// emitter's job). A `from:` arg resolves through the data-value resolver and
    /// renders bare (its value is in the bare charset); an `agent:` arg renders as
    /// an uppercase `<NAME>` marker, bare.
    #[test]
    fn command_ref_renders_shell_safe() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = add_rate_limiter_ctx();

        let set_commit = catalog.get("set-commit-summary").expect("present");
        assert_eq!(
            render_command(set_commit, &ctx).expect("renders"),
            "jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter"
        );

        let create_adr = catalog.get("create-adr").expect("present");
        assert_eq!(
            render_command(create_adr, &ctx).expect("renders"),
            "jigc doc create adr --title <TITLE> --task add-rate-limiter"
        );

        let finalize = catalog.get("finalize-task").expect("present");
        assert_eq!(
            render_command(finalize, &ctx).expect("renders"),
            "jigc task finalize add-rate-limiter"
        );

        let validate = catalog.get("validate-task").expect("present");
        assert_eq!(
            render_command(validate, &ctx).expect("renders"),
            "jigc task validate add-rate-limiter"
        );

        // Golden table over the four shipped command-refs → rendered line. The
        // golden pins the exact bytes so a quoting or join change breaks it.
        let mut rows = Vec::new();
        for id in [
            "create-adr",
            "finalize-task",
            "set-commit-summary",
            "validate-task",
        ] {
            let line = render_command(catalog.get(id).expect("present"), &ctx).expect("renders");
            rows.push(format!("{id}\n  => {line}"));
        }
        insta::assert_snapshot!(rows.join("\n"), @r#"
        create-adr
          => jigc doc create adr --title <TITLE> --task add-rate-limiter
        finalize-task
          => jigc task finalize add-rate-limiter
        set-commit-summary
          => jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter
        validate-task
          => jigc task validate add-rate-limiter
        "#);
    }

    /// Per-arg POSIX quoting (`command-catalog.md` → Shell-safe rendering): a bare
    /// charset arg stays unquoted; an arg with whitespace or a shell metachar is
    /// single-quoted; an embedded `'` is closed-escaped-reopened (`'it'\''s'`).
    #[test]
    fn shell_quotes_per_arg() {
        // Bare: the full bare charset stays unquoted.
        assert_eq!(
            shell_quote("commit:add-rate-limiter#summary"),
            "commit:add-rate-limiter#summary"
        );
        assert_eq!(shell_quote("--from-file"), "--from-file");
        assert_eq!(shell_quote("-"), "-");
        assert_eq!(shell_quote("<TITLE>"), "'<TITLE>'");

        // Whitespace / metachars → single-quoted.
        assert_eq!(shell_quote("two words"), "'two words'");
        assert_eq!(shell_quote("a;rm -rf"), "'a;rm -rf'");
        assert_eq!(shell_quote("a|b"), "'a|b'");

        // An embedded single quote: closed-escaped-reopened.
        assert_eq!(shell_quote("it's"), "'it'\\''s'");

        // The empty string renders as an explicit empty quoted word.
        assert_eq!(shell_quote(""), "''");
    }

    proptest::proptest! {
        /// Quoting predicate: an arg whose every char is in the bare charset
        /// `[A-Za-z0-9._:#/=@+-]` (and is non-empty) renders bare; anything else
        /// renders single-quoted, and a quoted rendering both opens and closes with
        /// `'`. The rendered token is always a single shell word.
        #[test]
        fn quoting_predicate_holds(arg in ".{0,40}") {
            let rendered = shell_quote(&arg);
            let is_bare = !arg.is_empty()
                && arg.chars().all(|c| {
                    c.is_ascii_alphanumeric() || "._:#/=@+-".contains(c)
                });
            if is_bare {
                proptest::prop_assert_eq!(&rendered, &arg, "bare arg should stay bare");
            } else {
                proptest::prop_assert!(
                    rendered.starts_with('\'') && rendered.ends_with('\''),
                    "non-bare arg {:?} should be single-quoted, got {:?}", arg, rendered
                );
            }
        }

        /// Purity / determinism: rendering the same `(command-ref, ctx)` twice
        /// yields an identical line. Rendering is a pure function of its inputs
        /// (the determinism boundary). Generated over the four shipped refs.
        #[test]
        fn render_is_pure(idx in 0usize..4) {
            let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
            let ctx = add_rate_limiter_ctx();
            let id = ["create-adr", "finalize-task", "set-commit-summary", "validate-task"][idx];
            let cmd = catalog.get(id).expect("present");
            let first = render_command(cmd, &ctx);
            let second = render_command(cmd, &ctx);
            proptest::prop_assert_eq!(first, second);
        }
    }

    /// A test [`StepSource`] backed by an id → body map. Stands in for the
    /// frontend's cascade-resolved step file set: a present key resolves to a
    /// [`StepDef`] with that body; an absent key is a dangling include.
    struct MapSource(std::collections::BTreeMap<String, String>);

    impl MapSource {
        fn new(pairs: &[(&str, &str)]) -> Self {
            MapSource(
                pairs
                    .iter()
                    .map(|(id, body)| ((*id).to_owned(), (*body).to_owned()))
                    .collect(),
            )
        }
    }

    impl StepSource for MapSource {
        fn step(&self, id: &str) -> Option<StepDef> {
            self.0
                .get(id)
                .map(|body| load_step_def(id, body.as_bytes()).expect("loads"))
        }
    }

    /// The four shipped single-task step bodies as the **post-phase-5** compose path
    /// feeds them — byte-identical to the pack files, except the pack `implement`
    /// step's `{{fill: extra-guidance}}` extension point is resolved through phase 5
    /// with an empty cascade (its empty pack default, the unfilled case), exactly as
    /// the CLI's `FillStepSource` does before this engine `compose`/`workflow_refs`
    /// runs. Without it the lone `{{fill:}}` point would reach phase 8 unresolved —
    /// the fill checks are the fill-aware [`workflow_refs_with_fills`]'s job, not the
    /// fill-blind compose these clean-body tests exercise.
    fn single_task_source() -> MapSource {
        let implement = apply_slot_fills(
            "implement",
            include_str!("../../cli/pack/steps/implement.yaml"),
            &ResolvedFills::new(),
        )
        .expect("empty-fill phase 5 over the pack implement body");
        MapSource::new(&[
            ("locate", STEP_LOCATE),
            ("implement", &implement),
            ("superseded-context", STEP_SUPERSEDED),
            (
                "finalize",
                include_str!("../../cli/pack/steps/finalize.yaml"),
            ),
        ])
    }

    /// Core done-criterion (`include_expansion_flattens_in_order`): the shipped
    /// single-task workflow expands to the four step bodies in include order —
    /// `[locate, implement, superseded-context, finalize]` — placeholders still
    /// unresolved. A dangling include id is a typed `include-resolves` error; a
    /// synthetic include cycle is a typed `include-cycle-absent` error. The golden
    /// pins the flattened step-id order.
    #[test]
    fn include_expansion_flattens_in_order() {
        let def = load_workflow_def(SINGLE_TASK.as_bytes()).expect("loads");
        let source = single_task_source();

        let composition = expand_includes(&def, &source).expect("expands");

        // The flattened step-id order is exactly the include-list order.
        assert_eq!(
            composition.step_ids(),
            vec!["locate", "implement", "superseded-context", "finalize"]
        );
        insta::assert_snapshot!(composition.step_ids().join(" -> "), @"locate -> implement -> superseded-context -> finalize");

        // Plain step bodies are carried verbatim, placeholders unresolved.
        let locate = &composition.steps[0];
        assert_eq!(locate.id, "locate");
        assert_eq!(locate.body, STEP_LOCATE);
        assert!(locate.body.contains("{{ task.intent }}"));

        // A dangling include id (no step file in the cascade) → typed error.
        let dangling = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["locate".to_owned(), "not-a-step".to_owned()],
        };
        let err = expand_includes(&dangling, &source).expect_err("dangling include rejects");
        assert_eq!(err.code, "workflow-refs.include-resolves");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);

        // A synthetic include cycle (A includes B includes A) → typed error.
        let cyclic_source = MapSource::new(&[
            ("a", "prose a\n{{ include: step:b }}\n"),
            ("b", "prose b\n{{ include: step:a }}\n"),
        ]);
        let cyclic = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["a".to_owned()],
        };
        let err = expand_includes(&cyclic, &cyclic_source).expect_err("cycle rejects");
        assert_eq!(err.code, "workflow-refs.include-cycle-absent");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);
    }

    /// The step kind reaches the [`ComposedStep`] (the doc-elaboration pin,
    /// `DECISIONS.md` 2026-06-04): a `fan-out` step's boundary leaf carries its
    /// [`StepKind::FanOut`], a `join` step its [`StepKind::Join`], a plain step
    /// [`StepKind::Plain`] — so the emit/check passes (T3/T4) can read the kind off
    /// the composition without re-parsing front-matter.
    #[test]
    fn step_kind_reaches_composed_step() {
        // A source that loads real step bytes (front-matter parsed) so the kind is
        // genuinely propagated, not hand-set.
        struct BytesSource(std::collections::BTreeMap<String, Vec<u8>>);
        impl StepSource for BytesSource {
            fn step(&self, id: &str) -> Option<StepDef> {
                self.0.get(id).map(|b| load_step_def(id, b).expect("loads"))
            }
        }
        let source = BytesSource(
            [
                (
                    "fan".to_owned(),
                    b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\n---\nfan body\n".to_vec(),
                ),
                ("join".to_owned(), b"---\njoin: {}\n---\njoin body\n".to_vec()),
                ("plain".to_owned(), b"plain body\n".to_vec()),
            ]
            .into_iter()
            .collect(),
        );
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: false,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["fan".to_owned(), "join".to_owned(), "plain".to_owned()],
        };
        let composition = expand_includes(&def, &source).expect("expands");
        assert_eq!(
            composition.steps[0].kind,
            StepKind::FanOut {
                over: "{{ milestone.tasks }}".to_owned(),
                run: "workflow:sub-task".to_owned(),
            }
        );
        assert_eq!(composition.steps[1].kind, StepKind::Join);
        assert_eq!(composition.steps[2].kind, StepKind::Plain);
    }

    /// A continuation leaf — a prose segment after an in-body include splice — stays
    /// [`StepKind::Plain`] even when its on-disk step carries a kind: only the
    /// boundary leaf (the step's first segment) inherits the step kind. (Fan-out/
    /// join steps are single-bodied by construction, but the rule is explicit so the
    /// invariant doesn't drift.)
    #[test]
    fn continuation_leaf_stays_plain() {
        struct BytesSource(std::collections::BTreeMap<String, Vec<u8>>);
        impl StepSource for BytesSource {
            fn step(&self, id: &str) -> Option<StepDef> {
                self.0.get(id).map(|b| load_step_def(id, b).expect("loads"))
            }
        }
        // `outer` carries a `join` kind and splices `inner` mid-body, so it emits a
        // boundary prose leaf, then `inner`, then a continuation prose leaf.
        let source = BytesSource(
            [
                (
                    "outer".to_owned(),
                    b"---\njoin: {}\n---\nbefore\n{{ include: step:inner }}\nafter\n".to_vec(),
                ),
                ("inner".to_owned(), b"inner body\n".to_vec()),
            ]
            .into_iter()
            .collect(),
        );
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: false,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["outer".to_owned()],
        };
        let composition = expand_includes(&def, &source).expect("expands");
        // [ outer("before", boundary, Join), inner(Plain), outer("after", continuation, Plain) ]
        assert_eq!(composition.steps[0].id, "outer");
        assert_eq!(composition.steps[0].kind, StepKind::Join);
        assert!(!composition.steps[0].continues);
        let last = composition.steps.last().expect("has leaves");
        assert_eq!(last.id, "outer");
        assert!(last.continues, "the after-include prose is a continuation");
        assert_eq!(last.kind, StepKind::Plain);
    }

    /// A nested include inside a step body expands **in place**: the parent's prose
    /// *before* the include comes first, then the nested step's body at the include
    /// line's position, then the parent's prose *after* — phase 7 recursive
    /// expansion (`overrides.md`; `workflow-dialect.md` → expanded in place). The
    /// nested `{{include:}}` line is replaced by the child subtree; the parent's
    /// prose segments survive verbatim, split into their own leaves under the
    /// parent id so each retains per-step fill/run-shadow attribution.
    #[test]
    fn nested_include_in_step_body_expands_in_place() {
        let source = MapSource::new(&[
            ("parent", "before\n{{ include: step:child }}\nafter\n"),
            ("child", "child body\n"),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["parent".to_owned()],
        };

        let composition = expand_includes(&def, &source).expect("expands");

        // The child is spliced *between* the parent's prose segments — not appended
        // after the whole parent body. Both prose leaves keep the parent id.
        assert_eq!(composition.step_ids(), vec!["parent", "child", "parent"]);
        assert_eq!(composition.steps[0].body, "before\n");
        assert_eq!(composition.steps[1].body, "child body\n");
        assert_eq!(composition.steps[2].body, "after\n");
    }

    /// A nested `{{include:}}` mixed with prose expands **in place**: the included
    /// body replaces the include line at its position, with the prose **before** and
    /// **after** preserved in order — `design/workflow-dialect.md` (Leaves /
    /// Composition: "expanded recursively in place") + `worked-examples.md` 3a
    /// phase-7. Reproduces the M4 audit MEDIUM finding: the prior pre-order
    /// flattening hoisted the included body to the **end** of the step, collapsing
    /// `before` and `after` above it. Asserts the *composed text* order, the
    /// byte-level contract the agent reads.
    #[test]
    fn nested_include_expands_in_place_not_hoisted_to_end() {
        let source = MapSource::new(&[
            (
                "parent",
                "LINE-BEFORE-INCLUDE\n{{ include: step:child }}\nLINE-AFTER-INCLUDE\n",
            ),
            ("child", "CHILD-BODY\n"),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["parent".to_owned()],
        };

        // Composed text: the child body lands **between** the two prose lines, not
        // after them. With prose and include continuous (no blank line injected),
        // the splice reads as one contiguous step body.
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");
        assert_eq!(
            composed.text, "LINE-BEFORE-INCLUDE\nCHILD-BODY\nLINE-AFTER-INCLUDE\n",
            "the include must expand in place: before, child, after — not before, after, child"
        );

        // The flattened leaves carry the same ids, the child spliced *between* the
        // parent's prose segments (not appended after the whole parent body).
        let composition = expand_includes(&def, &source).expect("expands");
        assert_eq!(composition.step_ids(), vec!["parent", "child", "parent"]);
        assert_eq!(composition.steps[0].body, "LINE-BEFORE-INCLUDE\n");
        assert_eq!(composition.steps[1].body, "CHILD-BODY\n");
        assert_eq!(composition.steps[2].body, "LINE-AFTER-INCLUDE\n");
    }

    /// The literal `worked-examples.md` 3a pattern: a `project-implement` body that
    /// is `{{ include: step:implement }}`, a blank line, then a house-rule line. The
    /// design (3a phase-7) requires the pack `implement` body to compose **followed
    /// by** the house rule. The hoisting bug emitted the house rule first.
    #[test]
    fn worked_example_3a_implement_body_then_house_rule() {
        let house_rule = "Before you finalize, run the project lint probe.";
        let source = MapSource::new(&[
            (
                "project-implement",
                &format!("{{{{ include: step:implement }}}}\n\n{house_rule}\n"),
            ),
            ("implement", "PACK-IMPLEMENT-BODY\n"),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["project-implement".to_owned()],
        };

        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");
        let body_at = composed
            .text
            .find("PACK-IMPLEMENT-BODY")
            .expect("the pack implement body composes");
        let rule_at = composed
            .text
            .find(house_rule)
            .expect("the house rule composes");
        assert!(
            body_at < rule_at,
            "3a requires the implement body FOLLOWED BY the house rule; got:\n{}",
            composed.text
        );
    }

    /// A self-including step (A includes A) is a cycle the moment it re-enters.
    #[test]
    fn self_include_is_a_cycle() {
        let source = MapSource::new(&[("a", "loop\n{{ include: step:a }}\n")]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["a".to_owned()],
        };
        let err = expand_includes(&def, &source).expect_err("self-cycle rejects");
        assert_eq!(err.code, "workflow-refs.include-cycle-absent");
    }

    /// A diamond (A→B, A→C, B→D, C→D) is **not** a cycle: D is reached twice but
    /// never while already on the active path, so it expands once per inclusion in
    /// pre-order. Cycle detection keys on the active DFS path, not on having seen
    /// an id before. With in-place expansion, the pure-container bodies (`a`, `b`,
    /// `c` — only includes, no prose) contribute **no text leaf** of their own: an
    /// include line is replaced by its child subtree, so only the prose-bearing `d`
    /// leaves remain. The walk still pushes `a`/`b`/`c` onto the DFS path, so cycle
    /// detection is unaffected.
    #[test]
    fn diamond_is_not_a_cycle() {
        let source = MapSource::new(&[
            ("a", "{{ include: step:b }}\n{{ include: step:c }}\n"),
            ("b", "{{ include: step:d }}\n"),
            ("c", "{{ include: step:d }}\n"),
            ("d", "leaf\n"),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: true,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["a".to_owned()],
        };
        let composition = expand_includes(&def, &source).expect("diamond expands");
        // Only the prose-bearing leaf `d` remains, once per inclusion in pre-order.
        assert_eq!(composition.step_ids(), vec!["d", "d"]);
        // `a`'s body is two *adjacent* include lines (no blank between), so the
        // in-place splice replaces each line with `d`'s body contiguously: the
        // second inclusion continues the first, never hoisted or blank-separated.
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");
        assert_eq!(composed.text, "leaf\nleaf\n");
    }

    proptest::proptest! {
        /// For an acyclic include forest, the flattened leaf order equals the
        /// pre-order include traversal. We generate a flat workflow over N distinct
        /// plain steps (no nested includes), so the pre-order traversal is exactly
        /// the include-list order — expansion must reproduce it.
        #[test]
        fn flattened_order_equals_include_order_for_acyclic(
            ids in proptest::collection::vec("[a-z][a-z0-9-]{0,7}", 0..8)
        ) {
            // Distinct ids only (a repeated include id is legitimate re-use, but
            // we want the pre-order == include-order invariant over a forest of
            // distinct plain leaves; duplicates would still satisfy it but muddy
            // the assertion).
            let mut seen = std::collections::BTreeSet::new();
            let ids: Vec<String> = ids.into_iter().filter(|id| seen.insert(id.clone())).collect();

            let pairs: Vec<(&str, &str)> = ids.iter().map(|id| (id.as_str(), "plain body\n")).collect();
            let source = MapSource::new(&pairs);
            let def = WorkflowDef {
                when: None,
                description: None,
                usage: None,
                creates_task: true,
                selectable: true,
                allows_create: vec![],
                reads: vec![],
                includes: ids.clone(),
            };

            let composition = expand_includes(&def, &source).expect("acyclic forest expands");
            let flattened: Vec<String> = composition.steps.iter().map(|s| s.id.clone()).collect();
            proptest::prop_assert_eq!(flattened, ids);
        }
    }

    /// The full `single-task` compose context: intent `add rate limiter`, the
    /// `commit` role bound to the task's own slug (`creates-task` workflow), and
    /// `decision` **declared but unbound** (no ADR created in this task — so the
    /// `superseded-context` step's `{{@task.decision.supersedes#decision}}`
    /// resolves to empty text).
    fn compose_ctx() -> ComposeContext {
        let mut roles = std::collections::BTreeMap::new();
        roles.insert(
            "commit".to_owned(),
            Some(crate::address::Address::parse("commit:add-rate-limiter").expect("valid")),
        );
        roles.insert("decision".to_owned(), None);
        ComposeContext {
            task: Some(TaskRoot {
                id: "add-rate-limiter".to_owned(),
                intent: "add rate limiter".to_owned(),
                roles,
            }),
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        }
    }

    /// Core done-criterion (`compose_single_task_emits_resolved_view`): composing
    /// the shipped `single-task` with intent `add rate limiter` yields the full
    /// four-class emitted text — the four steps (`locate`, `implement`,
    /// `superseded-context`, `finalize`) in include order, with `{{task.intent}}`
    /// resolved inline to its scalar, the `{{cli.…}}` command-refs rendered to
    /// `` Run: `…` `` lines, the `<<author: {{…}}>>` directive's embedded
    /// placeholder resolved (wrapper preserved), and the unbound
    /// `{{@task.decision.supersedes#decision}}` line emitted **empty** (no ADR
    /// bound). The golden pins the whole composed view; the routing footer is the
    /// renderer's job and is absent from the engine view.
    #[test]
    fn compose_single_task_emits_resolved_view() {
        let def = load_workflow_def(SINGLE_TASK.as_bytes()).expect("loads");
        let source = single_task_source();
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");

        // The resolved intent appears inline (scalar, no `> ` blockquote).
        assert!(
            composed.text.contains("add rate limiter"),
            "the resolved intent must appear in the composed view"
        );
        // The four steps' content appears in include order.
        let intent_at = composed
            .text
            .find("add rate limiter")
            .expect("intent present");
        let author_at = composed
            .text
            .find("<<author: commit:add-rate-limiter#summary>>")
            .expect("implement step present");
        let finalize_at = composed
            .text
            .find("jigc task finalize add-rate-limiter")
            .expect("finalize step present");
        assert!(
            intent_at < author_at && author_at < finalize_at,
            "steps must compose in include order: locate < implement < finalize"
        );
        // The unbound supersedes line is empty (empty-not-finding), never a `> `.
        assert!(
            !composed.text.contains("> commit:")
                && !composed.text.contains("@task.decision.supersedes"),
            "the unbound supersedes slice must emit empty text, not a finding/blockquote"
        );
        // The engine view carries no routing footer (that is the renderer's job).
        assert!(
            !composed.text.contains("— jigc ·"),
            "the engine view must not carry the routing footer"
        );

        insta::assert_snapshot!(composed.text, @"
        Reason about the change. The intent is:
        add rate limiter

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.

        Implement the change directly in the working tree. `git add` your code edits
        before finalize — it commits only what you have staged. When done, set the
        required Conventional-Commits type — your editorial call on what this change
        does — then stage the summary prose:

        Run: `jigc doc set-field commit:add-rate-limiter#type --value <COMMIT_TYPE> --task add-rate-limiter`
        Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter`
        <<author: commit:add-rate-limiter#summary>>

        The `scope` and `body` are optional: add a `scope` to name the area touched, or
        author a `body` to explain the motivation, only when they earn their place —

        jigc doc set-field commit:{{task.id}}#scope --value <area> --task {{task.id}}
        jigc doc set-slot commit:{{task.id}}#body --from-file - --task {{task.id}}

        If a decision is warranted, create an ADR and author its slots:

        Run: `jigc doc create adr --title <TITLE> --task add-rate-limiter`

        If your decision supersedes an earlier one, set `supersedes` on the ADR; the
        superseded decision then appears below for reference, so your consequences can
        explain what changes (nothing appears if it supersedes none).

        Validate and commit the task as one logical commit. Make sure your code edits
        are staged (`git add`) first — finalize commits only the staged set plus the
        docs it manages:

        Run: `jigc task finalize add-rate-limiter`
        ");
    }

    /// T3 done-criterion (`catalog_placeholder_emits_option_lines`): a lone
    /// `{{catalog}}` in a step body emits one `- <id> — <when>` option line per
    /// fed entry, **in fed order**, em-dash separator (`workflow-dialect.md` →
    /// Workflow selection / Emitted format; the T3 pin). An **empty** catalog
    /// emits empty text — no finding (the empty-vs-unresolvable contract, same
    /// stance as an unbound role).
    #[test]
    fn catalog_placeholder_emits_option_lines() {
        let empty_catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };

        // Two entries, fed order quick-fix-then-single-task (deliberately not the
        // single-task-first order to prove fed order, not sorted order, wins).
        let ctx = ComposeContext {
            task: None,
            catalog: vec![
                crate::result::CatalogEntry::new("quick-fix", "A small, localized fix."),
                crate::result::CatalogEntry::new(
                    "single-task",
                    "Implement one well-scoped change.",
                ),
            ],
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        };

        let emitted = emit_step_body("{{ catalog }}\n", &ctx, &empty_catalog).expect("emits");
        assert_eq!(
            emitted,
            "- quick-fix — A small, localized fix.\n\
             - single-task — Implement one well-scoped change.\n",
        );

        // An empty catalog emits empty text — no finding (empty-not-error).
        let empty_ctx = ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        };
        let emitted_empty =
            emit_step_body("{{ catalog }}\n", &empty_ctx, &empty_catalog).expect("emits");
        assert_eq!(emitted_empty, "\n");
    }

    /// Core done-criterion (the compose half): a lone `{{store.specs}}` renders a
    /// `> ` **Content list** — one `> <type>:<slug>` line per committed instance, in
    /// fed order (`workflow-dialect.md` → data-value roots; `worked-examples.md` →
    /// flow 6 `locate-from-spec`). An **empty** store emits empty text — no finding
    /// (empty-not-error, the same stance as an empty catalog).
    #[test]
    fn emit_store_collection_as_content_list() {
        let empty_catalog = CommandCatalog {
            commands: std::collections::BTreeMap::new(),
        };
        let mut store = std::collections::BTreeMap::new();
        // Keyed by the path-facing collection name (`store.specs`); see the
        // resolver test for the keying rationale.
        store.insert(
            "specs".to_owned(),
            vec![
                crate::address::Address::parse("spec:gateway-rate-limiting").expect("valid"),
                crate::address::Address::parse("spec:auth-token-rotation").expect("valid"),
            ],
        );
        let ctx = crate::data_value::ComposeContext {
            task: None,
            catalog: Vec::new(),
            store,
            milestone: Vec::new(),
            source: None,
        };

        // A two-spec store renders one `> <address>` line per instance, fed order.
        let emitted = emit_step_body("{{ store.specs }}\n", &ctx, &empty_catalog).expect("emits");
        assert_eq!(
            emitted,
            "> spec:gateway-rate-limiting\n> spec:auth-token-rotation\n",
        );

        // An unfed doctype (no committed instances) emits empty text — no finding.
        let empty_ctx = crate::data_value::ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: Vec::new(),
            source: None,
        };
        let emitted_empty =
            emit_step_body("{{ store.specs }}\n", &empty_ctx, &empty_catalog).expect("emits");
        assert_eq!(emitted_empty, "\n");
    }

    proptest::proptest! {
        /// Determinism (the load-bearing invariant): composing twice with the same
        /// resolved cascade (`def` + `source` + `catalog`) and the same `ctx` is
        /// byte-identical — same resolved cascade in → same workflow out. The
        /// generator varies the intent scalar; the structure is fixed.
        #[test]
        fn compose_is_deterministic_for_fixed_cascade_and_ctx(
            intent in "[a-z][a-z0-9 ]{0,40}"
        ) {
            let def = load_workflow_def(SINGLE_TASK.as_bytes()).expect("loads");
            let source = single_task_source();
            let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
            let mut ctx = compose_ctx();
            ctx.task.as_mut().expect("compose_ctx binds a task").intent = intent;

            let first = compose(&def, &source, &catalog, &ctx);
            let second = compose(&def, &source, &catalog, &ctx);
            proptest::prop_assert_eq!(first, second);
        }
    }

    /// Render a findings list to a compact `code @ line:col` table for goldens.
    fn finding_codes(findings: &[Finding]) -> String {
        findings
            .iter()
            .map(|f| {
                let loc = f
                    .location
                    .as_ref()
                    .map(|l| format!("{}:{}", l.line, l.col))
                    .unwrap_or_else(|| "-".to_owned());
                format!("{} @ {loc}", f.code)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Core done-criterion (`workflow_refs_flags_each_break`): the engine-native
    /// `workflow-refs` probe runs the seven intrinsic compose-time checks over a
    /// workflow definition + its expanded tree. A clean `single-task` yields zero
    /// findings; six minimal negative fixtures each trip **exactly one** check with
    /// a blocking [`Finding`] carrying the right `code`. The golden pins the
    /// findings list (code @ line:col) per fixture, and every finding is asserted
    /// `Severity::Blocking` (every `workflow-refs` check is intrinsic-blocking,
    /// `validation.md` → Severity inventory).
    #[test]
    fn workflow_refs_flags_each_break() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        // Clean: the shipped single-task composes with zero findings.
        let clean = workflow_refs(
            SINGLE_TASK.as_bytes(),
            &single_task_source(),
            &catalog,
            &ctx,
        );
        assert!(
            clean.is_empty(),
            "a clean single-task must yield zero findings, got {clean:?}"
        );
        insta::assert_snapshot!(finding_codes(&clean), @"");

        // 1) A dangling include id (no step file in the cascade) → include-resolves.
        let dangling_wf = b"---\nwhen: x\n---\n{{ include: step:not-a-step }}\n";
        let dangling = workflow_refs(dangling_wf, &single_task_source(), &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&dangling),
            @"workflow-refs.include-resolves @ 1:1"
        );

        // 2) A dangling `{{cli.unknown}}` command-ref → command-ref-resolves.
        let unknown_cli = MapSource::new(&[("only", "{{ cli.unknown }}\n")]);
        let unknown_cli_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let cli = workflow_refs(unknown_cli_wf, &unknown_cli, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&cli),
            @"workflow-refs.command-ref-resolves @ 1:1"
        );

        // 3) A step body line literally starting `Run: ` shadows the reserved
        //    composer marker → run-marker-not-shadowed.
        let shadow_src = MapSource::new(&[("only", "do the thing\nRun: jigc do-it\nthen stop\n")]);
        let shadow_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let shadow = workflow_refs(shadow_wf, &shadow_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&shadow),
            @"workflow-refs.run-marker-not-shadowed @ 2:1"
        );

        // 4) A workflow body with a prose line → body-include-only.
        let prose_wf = b"---\nwhen: x\n---\n{{ include: step:locate }}\nthis is prose\n";
        let prose = workflow_refs(prose_wf, &single_task_source(), &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&prose),
            @"workflow-refs.body-include-only @ 5:1"
        );

        // 5) An `@`-marker on a scalar-resolving path → at-marker-on-non-scalar.
        let at_scalar_src = MapSource::new(&[("only", "{{ @task.intent }}\n")]);
        let at_scalar_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let at_scalar = workflow_refs(at_scalar_wf, &at_scalar_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&at_scalar),
            @"workflow-refs.at-marker-on-non-scalar @ 1:1"
        );

        // 6) An include cycle (A includes B includes A) → include-cycle-absent.
        let cyclic_src = MapSource::new(&[
            ("a", "prose a\n{{ include: step:b }}\n"),
            ("b", "prose b\n{{ include: step:a }}\n"),
        ]);
        let cyclic_wf = b"---\nwhen: x\n---\n{{ include: step:a }}\n";
        let cyclic = workflow_refs(cyclic_wf, &cyclic_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&cyclic),
            @"workflow-refs.include-cycle-absent @ 1:1"
        );

        // Every finding the probe emits is intrinsic blocking.
        for findings in [&dangling, &cli, &shadow, &prose, &at_scalar, &cyclic] {
            assert_eq!(findings.len(), 1, "each fixture trips exactly one check");
            assert_eq!(
                findings[0].severity,
                crate::finding::Severity::Blocking,
                "every workflow-refs check is intrinsic-blocking"
            );
        }
    }

    // --- T4: the two M8 workflow-refs checks (intrinsic, blocking) ---

    /// `spawn-marker-not-shadowed` (M8): a step body line starting `Spawn: ` shadows
    /// the composer-reserved fan-out marker → blocking finding located at that line.
    /// A *genuine* fan-out step is clean: its `Spawn:` directives are emitted by the
    /// composer from the resolved `over:` collection (not present in the step body
    /// the probe walks), and its prose ("Spawn a sub-task per item.") does not start
    /// a line with the reserved prefix. The shadow finding is intrinsic-blocking.
    #[test]
    fn workflow_refs_flags_spawn_marker_shadow() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        // A step body literally starting a line `Spawn: ` shadows the marker.
        let shadow_src =
            MapSource::new(&[("only", "do the thing\nSpawn: a sub-task\nthen stop\n")]);
        let shadow_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let shadow = workflow_refs(shadow_wf, &shadow_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&shadow),
            @"workflow-refs.spawn-marker-not-shadowed @ 2:1"
        );
        assert_eq!(shadow.len(), 1, "exactly the shadow check trips");
        assert_eq!(shadow[0].severity, crate::finding::Severity::Blocking);

        // A genuine fan-out step (paired with a join) is clean — no spawn-shadow.
        let clean_src = MapSource::new(&[
            (
                "fan",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\n---\nSpawn a sub-task per item.\n",
            ),
            ("join", "---\njoin: {}\n---\nMerge the results.\n"),
        ]);
        let clean_wf = b"---\nwhen: x\n---\n{{ include: step:fan }}\n{{ include: step:join }}\n";
        let clean = workflow_refs(clean_wf, &clean_src, &catalog, &ctx);
        assert!(
            clean.is_empty(),
            "a genuine paired fan-out step yields zero findings, got {clean:?}"
        );
    }

    /// `checkpoint-marker-not-shadowed` (M15): a step body line starting
    /// `Checkpoint: ` shadows the composer-reserved halt marker → blocking finding
    /// located at that line, at BOTH gate sites (`workflow_refs` →
    /// `workflow_refs_with_deltas`, and the live `workflow_refs_with_fills`). A
    /// *genuine* `checkpoint` step is clean: its `Checkpoint:` directive is emitted
    /// by the composer (prepended post-check), so it is absent from the de-included
    /// `step.body`/applied surface the probe walks — its own directive never
    /// self-trips. Intrinsic-blocking. The `Spawn:` mirror, against the prepend
    /// directive (`workflow-dialect.md` → Compose-time conformance).
    #[test]
    fn workflow_refs_flags_checkpoint_marker_shadow_at_both_gates() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        // (a) A step body literally starting a line `Checkpoint: ` shadows the
        //     marker — at the delta gate (`workflow_refs`).
        let shadow_src =
            MapSource::new(&[("only", "do the thing\nCheckpoint: a halt\nthen stop\n")]);
        let shadow_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let shadow = workflow_refs(shadow_wf, &shadow_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&shadow),
            @"workflow-refs.checkpoint-marker-not-shadowed @ 2:1"
        );
        assert_eq!(shadow.len(), 1, "exactly the shadow check trips");
        assert_eq!(shadow[0].severity, crate::finding::Severity::Blocking);

        // (a, live path) The same shadow also trips at the fill-aware gate
        //     (`workflow_refs_with_fills`) — the live compose path.
        let live = workflow_refs_with_fills(
            shadow_wf,
            &[],
            &[],
            &ResolvedFills::new(),
            &shadow_src,
            &catalog,
            &ctx,
        );
        insta::assert_snapshot!(
            finding_codes(&live),
            @"workflow-refs.checkpoint-marker-not-shadowed @ 2:1"
        );
        assert_eq!(
            live.len(),
            1,
            "exactly the shadow check trips on the live path"
        );
        assert_eq!(live[0].severity, crate::finding::Severity::Blocking);

        // (b) A genuine checkpoint step (paired with no marker) is clean — its own
        //     `Checkpoint:` directive is prepended post-check, unread by the
        //     pre-emit shadow scope.
        let clean_src = MapSource::new(&[(
            "gate",
            "---\ncheckpoint:\n  reason: new-fork-at-plan\n---\nIf a fork surfaced, stop.\n",
        )]);
        let clean_wf = b"---\nwhen: x\n---\n{{ include: step:gate }}\n";
        let clean = workflow_refs(clean_wf, &clean_src, &catalog, &ctx);
        assert!(
            clean.is_empty(),
            "a genuine checkpoint step yields zero findings, got {clean:?}"
        );
        let clean_live = workflow_refs_with_fills(
            clean_wf,
            &[],
            &[],
            &ResolvedFills::new(),
            &clean_src,
            &catalog,
            &ctx,
        );
        assert!(
            clean_live.is_empty(),
            "a genuine checkpoint step yields zero findings on the live path, got {clean_live:?}"
        );
    }

    /// `fan-out-join-paired` (M8): a workflow with a `fan-out` step and no `join`
    /// step (and vice-versa) is a blocking finding; a paired workflow is clean. The
    /// check reads the composition's per-step kinds — presence-pairing, not
    /// count-matching. Intrinsic-blocking.
    #[test]
    fn workflow_refs_flags_unpaired_fan_out_join() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        let fan = (
            "fan",
            "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\n---\nSpawn a sub-task per item.\n",
        );
        let join = ("join", "---\njoin: {}\n---\nMerge the results.\n");

        // fan-out with no join → blocking pairing finding.
        let fan_only_src = MapSource::new(&[fan]);
        let fan_only_wf = b"---\nwhen: x\n---\n{{ include: step:fan }}\n";
        let fan_only = workflow_refs(fan_only_wf, &fan_only_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&fan_only),
            @"workflow-refs.fan-out-join-paired @ -"
        );
        assert_eq!(fan_only.len(), 1, "exactly the pairing check trips");
        assert_eq!(fan_only[0].severity, crate::finding::Severity::Blocking);

        // join with no fan-out → blocking pairing finding.
        let join_only_src = MapSource::new(&[join]);
        let join_only_wf = b"---\nwhen: x\n---\n{{ include: step:join }}\n";
        let join_only = workflow_refs(join_only_wf, &join_only_src, &catalog, &ctx);
        insta::assert_snapshot!(
            finding_codes(&join_only),
            @"workflow-refs.fan-out-join-paired @ -"
        );
        assert_eq!(join_only.len(), 1, "exactly the pairing check trips");
        assert_eq!(join_only[0].severity, crate::finding::Severity::Blocking);

        // A paired fan-out + join → clean.
        let paired_src = MapSource::new(&[fan, join]);
        let paired_wf = b"---\nwhen: x\n---\n{{ include: step:fan }}\n{{ include: step:join }}\n";
        let paired = workflow_refs(paired_wf, &paired_src, &catalog, &ctx);
        assert!(
            paired.is_empty(),
            "a paired fan-out + join yields zero findings, got {paired:?}"
        );
    }

    // --- M20 T1: the store-scope `workflow-refs` target (task-less) ---

    /// `workflow_refs_store` runs only the **task-independent** checks: a clean
    /// `single-task` yields zero findings, and each task-independent break trips
    /// **exactly one** blocking finding with the right `code` (and located line). The
    /// task-data checks (`placeholder-resolves`, `at-marker-on-non-scalar`) are NOT
    /// run here — they stay at `jigc start` (`validation.md` → Completing the
    /// envelope: the exact check set).
    #[test]
    fn workflow_refs_store_flags_each_task_independent_break() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");

        // Clean: the shipped single-task store-validates with zero findings.
        let clean = workflow_refs_store(SINGLE_TASK.as_bytes(), &single_task_source(), &catalog);
        assert!(
            clean.is_empty(),
            "a clean single-task must yield zero store findings, got {clean:?}"
        );
        insta::assert_snapshot!(finding_codes(&clean), @"");

        // 1) A dangling include id (no step file in the cascade) → include-resolves.
        let dangling_wf = b"---\nwhen: x\n---\n{{ include: step:not-a-step }}\n";
        let dangling = workflow_refs_store(dangling_wf, &single_task_source(), &catalog);
        insta::assert_snapshot!(
            finding_codes(&dangling),
            @"workflow-refs.include-resolves @ 1:1"
        );

        // 2) An include cycle (A includes B includes A) → include-cycle-absent.
        let cyclic_src = MapSource::new(&[
            ("a", "prose a\n{{ include: step:b }}\n"),
            ("b", "prose b\n{{ include: step:a }}\n"),
        ]);
        let cyclic_wf = b"---\nwhen: x\n---\n{{ include: step:a }}\n";
        let cyclic = workflow_refs_store(cyclic_wf, &cyclic_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&cyclic),
            @"workflow-refs.include-cycle-absent @ 1:1"
        );

        // 3) A workflow body with a prose line → body-include-only.
        let prose_wf = b"---\nwhen: x\n---\n{{ include: step:locate }}\nthis is prose\n";
        let prose = workflow_refs_store(prose_wf, &single_task_source(), &catalog);
        insta::assert_snapshot!(
            finding_codes(&prose),
            @"workflow-refs.body-include-only @ 5:1"
        );

        // 4) A step body line starting `Run: ` → run-marker-not-shadowed.
        let run_src = MapSource::new(&[("only", "do the thing\nRun: jigc do-it\nthen stop\n")]);
        let only_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let run = workflow_refs_store(only_wf, &run_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&run),
            @"workflow-refs.run-marker-not-shadowed @ 2:1"
        );

        // 5) A step body line starting `Spawn: ` → spawn-marker-not-shadowed.
        let spawn_src = MapSource::new(&[("only", "do the thing\nSpawn: a sub-task\nthen stop\n")]);
        let spawn = workflow_refs_store(only_wf, &spawn_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&spawn),
            @"workflow-refs.spawn-marker-not-shadowed @ 2:1"
        );

        // 6) A step body line starting `Checkpoint: ` → checkpoint-marker-not-shadowed.
        let cp_src = MapSource::new(&[("only", "do the thing\nCheckpoint: a halt\nthen stop\n")]);
        let cp = workflow_refs_store(only_wf, &cp_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&cp),
            @"workflow-refs.checkpoint-marker-not-shadowed @ 2:1"
        );

        // 7) An unpaired `fan-out` (no `join`) → fan-out-join-paired.
        let fan_src = MapSource::new(&[(
            "fan",
            "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run: workflow:sub-task\n---\nSpawn a sub-task per item.\n",
        )]);
        let fan_wf = b"---\nwhen: x\n---\n{{ include: step:fan }}\n";
        let fan = workflow_refs_store(fan_wf, &fan_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&fan),
            @"workflow-refs.fan-out-join-paired @ -"
        );

        // Every store finding is intrinsic blocking, exactly one per fixture.
        for findings in [&dangling, &cyclic, &prose, &run, &spawn, &cp, &fan] {
            assert_eq!(findings.len(), 1, "each fixture trips exactly one check");
            assert_eq!(
                findings[0].severity,
                crate::finding::Severity::Blocking,
                "every store workflow-refs check is intrinsic-blocking"
            );
        }
    }

    /// The store-scope command-ref path is **membership-only**: a `{{cli.<id>}}`
    /// naming a catalog-absent id is exactly one `command-ref-resolves` finding
    /// (located at its body line); a `{{cli.<id>}}` naming a present entry is clean.
    #[test]
    fn workflow_refs_store_flags_absent_command_ref_membership() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let only_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";

        // A dangling `{{cli.unknown}}` command-ref → command-ref-resolves at its line.
        let unknown_src = MapSource::new(&[("only", "first\n{{ cli.unknown }}\nlast\n")]);
        let unknown = workflow_refs_store(only_wf, &unknown_src, &catalog);
        insta::assert_snapshot!(
            finding_codes(&unknown),
            @"workflow-refs.command-ref-resolves @ 2:1"
        );
        assert_eq!(unknown.len(), 1, "exactly the membership check trips");
        assert_eq!(unknown[0].severity, crate::finding::Severity::Blocking);

        // A present command-ref id → clean (membership passes).
        let present_src = MapSource::new(&[("only", "{{ cli.validate-task }}\n")]);
        let present = workflow_refs_store(only_wf, &present_src, &catalog);
        assert!(
            present.is_empty(),
            "a catalog-present command-ref yields zero findings, got {present:?}"
        );
    }

    /// The load-bearing **B1** assertion: a command-ref whose entry carries `task.*`
    /// args (every doc-verb ref in the real dev pack) emits **NO** finding at store
    /// scope — the membership-only path does not call `render_command`, so no
    /// fabricated `placeholder-resolves` / `task-ref-in-no-task-workflow` drift over a
    /// task-less workflow (`validation.md` → the B1 fix). Asserted over the **real**
    /// catalog's `{ from: "task.id" }`-bearing refs, not a hand-built stub.
    #[test]
    fn workflow_refs_store_emits_no_task_ref_drift_for_task_args() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        // `validate-task` / `finalize-task` both render `{ from: "task.id" }`; at store
        // scope (no task) a full resolution would drift, but membership-only is clean.
        for id in ["validate-task", "finalize-task", "set-commit-summary"] {
            assert!(
                catalog.get(id).expect("ref present").args.iter().any(|a| {
                    matches!(a, CommandArg::From { from } if from.starts_with("task."))
                }),
                "fixture precondition: `{id}` must carry a task.* `from:` arg"
            );
            let src = MapSource::new(&[("only", &format!("{{{{ cli.{id} }}}}\n"))]);
            let only_wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
            let findings = workflow_refs_store(only_wf, &src, &catalog);
            assert!(
                findings.is_empty(),
                "store scope must emit NO finding for the task.*-arg command-ref `{id}` (B1), got {findings:?}"
            );
        }
    }

    /// `workflow_refs_store` is a pure function of `(workflow_bytes, source,
    /// catalog)` — the determinism boundary, by construction task-less. The same
    /// inputs twice yield byte-identical findings.
    #[test]
    fn workflow_refs_store_is_deterministic() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let src = single_task_source();
        let first = workflow_refs_store(SINGLE_TASK.as_bytes(), &src, &catalog);
        let second = workflow_refs_store(SINGLE_TASK.as_bytes(), &src, &catalog);
        assert_eq!(
            finding_codes(&first),
            finding_codes(&second),
            "same inputs → same store findings"
        );
    }

    // --- T5: resolution-time cycle/orphan over the post-phase-4 include list ---
    //
    // The phase-4 structural deltas mutate the include id list *before* expansion;
    // the gate must validate the **post-phase-4** list so a delta-introduced
    // orphan / cycle / dangling surfaces at resolution time through the same
    // `workflow-refs` machinery — all located findings, never a panic
    // (`overrides.md` → Write-time vs resolve-time split; Resolution algorithm
    // phases 6–7).

    /// (a) An orphaned-anchor delta — `remove-step #locate` then
    /// `insert-step --after locate` in the **same manifest** — surfaces a blocking
    /// `workflow-refs.structural-anchor-resolves` finding **with its route** (the
    /// `run-command` direction to remove or re-target the delta — `validation.md`
    /// → route; `overrides.md` → Within-layer manifest order). Located, never a
    /// panic.
    #[test]
    fn delta_orphaned_anchor_surfaces_located_finding_with_route() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let deltas = vec![
            remove("workflow:single-task#locate"),
            insert(
                "workflow:single-task",
                AnchorSpec::After("locate".to_owned()),
                "team-lint",
            ),
        ];

        let findings = workflow_refs_with_deltas(
            SINGLE_TASK.as_bytes(),
            &deltas,
            &single_task_source(),
            &catalog,
            &ctx,
        );

        assert_eq!(findings.len(), 1, "exactly the orphaned-anchor finding");
        let f = &findings[0];
        assert_eq!(f.code, "workflow-refs.structural-anchor-resolves");
        assert_eq!(f.severity, crate::finding::Severity::Blocking);
        assert_eq!(f.location, Some(Location::at(1, 1)), "located, not a panic");
        assert!(
            f.route.is_some(),
            "an orphaned delta carries its repair route, got {:?}",
            f.route
        );
        let route = f.route.as_deref().unwrap();
        assert!(
            route.contains("jigc config"),
            "the route directs the human to remove or re-target the delta, got {route:?}"
        );
    }

    /// (b) A structural delta that introduces an **include cycle** is caught at
    /// phase 6 through the same `include-cycle-absent` machinery: a `replace-step`
    /// swaps `implement` for a project step whose body re-includes a step that
    /// loops back, so the post-phase-4 list expands into a cycle. Located, never a
    /// panic.
    #[test]
    fn delta_introducing_a_cycle_is_caught_at_phase_6() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        // The project step `looping-implement` re-includes `back`, which re-includes
        // `looping-implement` — a cycle only the post-phase-4 list reaches.
        let src = MapSource::new(&[
            ("locate", STEP_LOCATE),
            (
                "looping-implement",
                "house rule\n{{ include: step:back }}\n",
            ),
            ("back", "{{ include: step:looping-implement }}\n"),
            ("superseded-context", STEP_SUPERSEDED),
            (
                "finalize",
                include_str!("../../cli/pack/steps/finalize.yaml"),
            ),
        ]);
        let deltas = vec![replace(
            "workflow:single-task#implement",
            "looping-implement",
        )];

        let findings =
            workflow_refs_with_deltas(SINGLE_TASK.as_bytes(), &deltas, &src, &catalog, &ctx);

        assert_eq!(findings.len(), 1, "the delta-introduced cycle, located");
        assert_eq!(findings[0].code, "workflow-refs.include-cycle-absent");
        assert_eq!(findings[0].severity, crate::finding::Severity::Blocking);
        assert_eq!(findings[0].location, Some(Location::at(1, 1)));
    }

    /// (c) Flow 3a's clean re-include: `replace-step #implement → project-implement`
    /// where `project-implement` re-includes `implement` (a **different** id). The
    /// post-phase-4 list expands cleanly — **no false cycle**, zero findings
    /// (`worked-examples.md` → 3a; `overrides.md` → `replace` vs `tracked-fork`).
    #[test]
    fn flow_3a_re_include_is_clean_no_false_cycle() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        // `project-implement` re-includes the pack `implement` (a different id), per
        // flow 3a — augmenting it, not forking. No cycle: implement does not loop back.
        // The pack `implement` body is fed post-phase-5 (empty-fill default), as the
        // live compose path does, so its `{{fill:}}` point does not reach phase 8.
        let implement = apply_slot_fills(
            "implement",
            include_str!("../../cli/pack/steps/implement.yaml"),
            &ResolvedFills::new(),
        )
        .expect("empty-fill phase 5 over the pack implement body");
        let src = MapSource::new(&[
            ("locate", STEP_LOCATE),
            ("implement", &implement),
            (
                "project-implement",
                "{{ include: step:implement }}\n\nRun the project lint probe before you finalize.\n",
            ),
            ("superseded-context", STEP_SUPERSEDED),
            (
                "finalize",
                include_str!("../../cli/pack/steps/finalize.yaml"),
            ),
        ]);
        let deltas = vec![replace(
            "workflow:single-task#implement",
            "project-implement",
        )];

        let findings =
            workflow_refs_with_deltas(SINGLE_TASK.as_bytes(), &deltas, &src, &catalog, &ctx);

        assert!(
            findings.is_empty(),
            "flow 3a's different-id re-include must be clean, got {findings:?}"
        );
    }

    /// The `placeholder-resolves` check: an undeclared data-value root (a
    /// structurally-invalid path, not a merely-absent value) is a blocking
    /// `workflow-refs` finding surfaced by the probe.
    #[test]
    fn workflow_refs_flags_undeclared_placeholder() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let src = MapSource::new(&[("only", "{{ nope.field }}\n")]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:only }}\n";
        let findings = workflow_refs(wf, &src, &catalog, &ctx);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "workflow-refs.undeclared-root");
        assert_eq!(findings[0].severity, crate::finding::Severity::Blocking);
    }

    // --- M4 fill checks: orphan + survivor (workflow_refs_with_fills) -----------

    /// Build a `slot-fill` delta from a `step:<id>#<fill-id>` target + the native
    /// fill file's content-id (basename).
    fn slot_fill(target: &str, content_id: &str) -> crate::cascade::SlotFillDelta {
        crate::cascade::SlotFillDelta {
            target: crate::cascade::SlotFillTarget::parse(target).expect("valid target"),
            content_id: content_id.to_owned(),
        }
    }

    /// Done-criterion (a): a `slot-fill` aimed at a fill-id **no resolved body
    /// declares** is a blocking `workflow-refs.slot-fill-orphan` carrying its repair
    /// route — symmetric with the orphaned `structural-anchor-resolves`, and closing
    /// the closed-surface hole for both authoring paths (`overrides.md` → Orphan
    /// detection is M4).
    #[test]
    fn workflow_refs_flags_slot_fill_orphan_with_route() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        // The `implement` step declares `extra-guidance`; the delta targets a
        // fill-id (`typo-id`) no body declares.
        let src = MapSource::new(&[("implement", "do it\n{{ fill: extra-guidance }}\nstop\n")]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:implement }}\n";
        let deltas = [slot_fill("step:implement#typo-id", "typo-id")];
        let fills = fills(&[]);

        let findings = workflow_refs_with_fills(wf, &[], &deltas, &fills, &src, &catalog, &ctx);

        assert_eq!(
            findings.len(),
            1,
            "exactly one orphan finding, got {findings:?}"
        );
        assert_eq!(findings[0].code, "workflow-refs.slot-fill-orphan");
        assert_eq!(findings[0].severity, crate::finding::Severity::Blocking);
        assert!(
            findings[0].route.is_some(),
            "an orphaned slot-fill carries its repair route"
        );
    }

    /// Done-criterion (b): a surviving lone `{{fill:}}` in a post-phase-5 body is a
    /// blocking `workflow-refs.fill-survivor` **located at the line** — whether it
    /// arrived inside applied fill content (a nested fill) or names a point no delta
    /// filled. Both sub-cases trip the same recognizer (`overrides.md` → no nested
    /// fills; the survivor is flagged at resolution, not parsed as a generic root).
    #[test]
    fn workflow_refs_flags_fill_survivor_at_line() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();

        // (b1) Applied fill content that itself carries a `{{fill:}}` — phase 5 does
        //      not re-run, so the nested fill survives on the body's line 2.
        let nested_src = MapSource::new(&[("implement", "head\n{{ fill: outer }}\ntail\n")]);
        let nested_wf = b"---\nwhen: x\n---\n{{ include: step:implement }}\n";
        let nested_deltas = [slot_fill("step:implement#outer", "outer")];
        let nested_fills = fills(&[("implement", "outer", "{{ fill: inner }}\nmore")]);
        let nested = workflow_refs_with_fills(
            nested_wf,
            &[],
            &nested_deltas,
            &nested_fills,
            &nested_src,
            &catalog,
            &ctx,
        );
        insta::assert_snapshot!(
            finding_codes(&nested),
            @"workflow-refs.fill-survivor @ 2:1"
        );

        // (b2) A lone `{{fill:}}` that no delta filled and whose point a hostile body
        //      re-declares inside applied content — same survivor recognizer. Here a
        //      `{{fill:}}` is injected by content for a *declared* point, so it
        //      survives as a bare line no later phase resolves.
        let survivor_src = MapSource::new(&[("implement", "{{ fill: p }}\n")]);
        let survivor_wf = b"---\nwhen: x\n---\n{{ include: step:implement }}\n";
        let survivor_deltas = [slot_fill("step:implement#p", "p")];
        let survivor_fills = fills(&[("implement", "p", "{{ fill: leftover }}")]);
        let survivor = workflow_refs_with_fills(
            survivor_wf,
            &[],
            &survivor_deltas,
            &survivor_fills,
            &survivor_src,
            &catalog,
            &ctx,
        );
        insta::assert_snapshot!(
            finding_codes(&survivor),
            @"workflow-refs.fill-survivor @ 1:1"
        );

        for findings in [&nested, &survivor] {
            assert_eq!(findings.len(), 1, "each fixture trips exactly one survivor");
            assert_eq!(findings[0].severity, crate::finding::Severity::Blocking);
        }
    }

    /// Done-criterion (c): a correctly-filled body + a matching declared point
    /// produce **no** finding — the fill resolves cleanly, no orphan, no survivor.
    #[test]
    fn workflow_refs_clean_fill_yields_no_finding() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let src = MapSource::new(&[("implement", "do it\n{{ fill: extra-guidance }}\nstop\n")]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:implement }}\n";
        let deltas = [slot_fill("step:implement#extra-guidance", "extra-guidance")];
        let fills = fills(&[("implement", "extra-guidance", "follow the house style")]);

        let findings = workflow_refs_with_fills(wf, &[], &deltas, &fills, &src, &catalog, &ctx);

        assert!(
            findings.is_empty(),
            "a correctly-filled body + declared point yields no finding, got {findings:?}"
        );
        insta::assert_snapshot!(finding_codes(&findings), @"");
    }

    /// Regression (M4 audit, HIGH): a slot-fill whose **target step is not in the
    /// composed workflow** is **inert, not an orphan**. The orphan check keys on the
    /// *target step's* resolved body (via `source`), not the composed workflow's
    /// bodies — so a workflow that omits the target step (here `other`, with no
    /// `implement` include) composes cleanly even though a slot-fill targets
    /// `step:implement#extra-guidance`, which `implement` (fetchable via `source`)
    /// genuinely declares. Keying on the composed bodies bricked the bare-`jigc start`
    /// router the moment any `implement` slot-fill was recorded.
    #[test]
    fn workflow_refs_slot_fill_for_omitted_step_is_inert_not_orphan() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        // `source` knows both steps; the workflow includes only `other`.
        let src = MapSource::new(&[
            ("implement", "do it\n{{ fill: extra-guidance }}\nstop\n"),
            ("other", "a different step body\n"),
        ]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:other }}\n";
        let deltas = [slot_fill("step:implement#extra-guidance", "extra-guidance")];
        let fills = fills(&[]);

        let findings = workflow_refs_with_fills(wf, &[], &deltas, &fills, &src, &catalog, &ctx);

        assert!(
            findings.is_empty(),
            "a slot-fill whose target step the composed workflow omits is inert, not an orphan, got {findings:?}"
        );
        insta::assert_snapshot!(finding_codes(&findings), @"");
    }

    /// Regression guard (keep catching real orphans): a slot-fill whose target step
    /// **does resolve but declares no such point** is still a blocking orphan, even
    /// when that step is absent from the composed workflow — so the check is not a
    /// no-op. Here `implement` declares only `extra-guidance`; the delta targets the
    /// undeclared `typo-id`, and the workflow includes only `other`.
    #[test]
    fn workflow_refs_orphan_for_omitted_step_still_blocks() {
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = compose_ctx();
        let src = MapSource::new(&[
            ("implement", "do it\n{{ fill: extra-guidance }}\nstop\n"),
            ("other", "a different step body\n"),
        ]);
        let wf = b"---\nwhen: x\n---\n{{ include: step:other }}\n";
        let deltas = [slot_fill("step:implement#typo-id", "typo-id")];
        let fills = fills(&[]);

        let findings = workflow_refs_with_fills(wf, &[], &deltas, &fills, &src, &catalog, &ctx);

        assert_eq!(
            findings.len(),
            1,
            "exactly one orphan finding, got {findings:?}"
        );
        assert_eq!(findings[0].code, "workflow-refs.slot-fill-orphan");
        assert_eq!(findings[0].severity, crate::finding::Severity::Blocking);
        assert!(
            findings[0].route.is_some(),
            "the orphan carries its repair route"
        );
    }

    // --- phase-4 structural-delta application (apply_structural_deltas) ---

    use crate::cascade::{AnchorSpec, StructuralTarget};

    /// The done-criterion fixture include list: `single-task`'s pack-default
    /// composition order.
    fn fixture_includes() -> Vec<String> {
        ["locate", "implement", "superseded-context", "finalize"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Build an `insert` delta from a `workflow:<id>` ref + an `after:` / `before:`
    /// anchor + the inserted step id (the basename rule of the verb surface).
    fn insert(workflow: &str, anchor: AnchorSpec, step: &str) -> StructuralDelta {
        StructuralDelta::Insert {
            target: StructuralTarget::parse(workflow, Some(anchor)).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `replace` delta from a `workflow:<id>#<step-id>` ref + the
    /// replacement step id (another step id, never inline content).
    fn replace(target: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget::parse(target, None).expect("valid target"),
            step: step.to_owned(),
        }
    }

    /// Build a `remove` delta from a `workflow:<id>#<step-id>` ref.
    fn remove(target: &str) -> StructuralDelta {
        StructuralDelta::Remove {
            target: StructuralTarget::parse(target, None).expect("valid target"),
        }
    }

    /// Golden: an `insert` with an `after:` anchor lands the new id at the slot
    /// **immediately after** the anchor's index — not at the list head/tail.
    #[test]
    fn insert_after_lands_at_anchor_plus_one() {
        let out = apply_structural_deltas(
            &fixture_includes(),
            &[insert(
                "workflow:single-task",
                AnchorSpec::After("locate".to_owned()),
                "team-lint",
            )],
        )
        .expect("anchor present");

        assert_eq!(
            out,
            vec![
                "locate".to_owned(),
                "team-lint".to_owned(),
                "implement".to_owned(),
                "superseded-context".to_owned(),
                "finalize".to_owned(),
            ],
        );
    }

    /// Golden: an `insert` with a `before:` anchor lands the new id **at** the
    /// anchor's index, pushing the anchor one slot later.
    #[test]
    fn insert_before_lands_at_anchor_index() {
        let out = apply_structural_deltas(
            &fixture_includes(),
            &[insert(
                "workflow:single-task",
                AnchorSpec::Before("finalize".to_owned()),
                "team-lint",
            )],
        )
        .expect("anchor present");

        assert_eq!(
            out,
            vec![
                "locate".to_owned(),
                "implement".to_owned(),
                "superseded-context".to_owned(),
                "team-lint".to_owned(),
                "finalize".to_owned(),
            ],
        );
    }

    /// Golden: `replace` swaps the id **at the target's position** for another
    /// step id, leaving every other slot untouched.
    #[test]
    fn replace_swaps_id_at_position() {
        let out = apply_structural_deltas(
            &fixture_includes(),
            &[replace(
                "workflow:single-task#implement",
                "project-implement",
            )],
        )
        .expect("target present");

        assert_eq!(
            out,
            vec![
                "locate".to_owned(),
                "project-implement".to_owned(),
                "superseded-context".to_owned(),
                "finalize".to_owned(),
            ],
        );
    }

    /// Golden: `remove` drops the targeted id, closing the gap.
    #[test]
    fn remove_drops_targeted_id() {
        let out = apply_structural_deltas(
            &fixture_includes(),
            &[remove("workflow:single-task#superseded-context")],
        )
        .expect("target present");

        assert_eq!(
            out,
            vec![
                "locate".to_owned(),
                "implement".to_owned(),
                "finalize".to_owned(),
            ],
        );
    }

    /// Golden: multiple deltas compose **in fed order** — a `replace` then a
    /// `remove` of the *same* id. The replace runs first (operating on the
    /// original `implement`), then the remove operates on the replace's result
    /// (`project-implement`), dropping it (`overrides.md` → Within-layer manifest
    /// order).
    #[test]
    fn deltas_compose_in_fed_order_replace_then_remove() {
        let out = apply_structural_deltas(
            &fixture_includes(),
            &[
                replace("workflow:single-task#implement", "project-implement"),
                remove("workflow:single-task#project-implement"),
            ],
        )
        .expect("each anchor present when its delta runs");

        assert_eq!(
            out,
            vec![
                "locate".to_owned(),
                "superseded-context".to_owned(),
                "finalize".to_owned(),
            ],
        );
    }

    /// A delta whose anchor a same-run earlier delta removed is **orphaned** — a
    /// blocking `workflow-refs.structural-anchor-resolves` finding (`overrides.md`
    /// → Within-layer manifest order).
    #[test]
    fn delta_targeting_a_removed_anchor_is_orphaned() {
        let finding = apply_structural_deltas(
            &fixture_includes(),
            &[
                remove("workflow:single-task#implement"),
                insert(
                    "workflow:single-task",
                    AnchorSpec::After("implement".to_owned()),
                    "team-lint",
                ),
            ],
        )
        .expect_err("the insert's anchor was removed");

        assert_eq!(finding.code, "workflow-refs.structural-anchor-resolves");
        assert_eq!(finding.severity, crate::finding::Severity::Blocking);
        assert_eq!(finding.location, Some(Location::at(1, 1)));
    }

    /// The no-delta path is the identity on the include list — byte-identical id
    /// `Vec` out (the no-override path stays unchanged).
    #[test]
    fn no_deltas_is_the_identity() {
        let out = apply_structural_deltas(&fixture_includes(), &[]).expect("no deltas");
        assert_eq!(out, fixture_includes());
    }

    // --- resolution-tree builder (build_resolution_tree) — layers 1–2 ---

    use crate::cascade::LayerKind;
    use crate::result::{Replacement, ResolvedStep};

    /// A `file_owner` lookup that reports the given ids as project-owned, every
    /// other id pack-default — the phase-2 by-id shadowing surface the tree tags
    /// each resolved step against.
    fn owner_of<'a>(project: &'a [&'a str]) -> impl Fn(&str) -> Option<LayerKind> + 'a {
        move |id: &str| {
            Some(if project.contains(&id) {
                LayerKind::Project
            } else {
                LayerKind::PackDefault
            })
        }
    }

    /// (a) The no-override single-task tree: `overrides_applied: 0`, every step
    /// tagged pack-default (its pack file owns it), in the pack include order, no
    /// `replaces` annotation anywhere.
    #[test]
    fn tree_no_override_all_pack_default() {
        let tree = build_resolution_tree(
            "single-task",
            LayerKind::PackDefault,
            &fixture_includes(),
            &[],
            Vec::new(),
            Vec::new(),
            owner_of(&[]),
        )
        .expect("builds");

        assert_eq!(tree.workflow, "single-task");
        assert_eq!(tree.workflow_layer, LayerKind::PackDefault);
        assert_eq!(tree.overrides_applied, 0);
        assert_eq!(
            tree.steps,
            vec![
                ResolvedStep {
                    id: "locate".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "implement".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "superseded-context".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "finalize".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
            ],
        );
    }

    /// (b) A `replace-step` over `implement`: `overrides_applied: 1`; the
    /// replacing `project-implement` step is tagged **project** at the **same
    /// position** (1-based 2) with a `replaces implement at position 2` annotation;
    /// every other step stays pack-default and unannotated (`worked-examples.md` →
    /// 3a).
    #[test]
    fn tree_replace_step_tags_project_with_annotation() {
        let tree = build_resolution_tree(
            "single-task",
            LayerKind::PackDefault,
            &fixture_includes(),
            &[replace(
                "workflow:single-task#implement",
                "project-implement",
            )],
            Vec::new(),
            Vec::new(),
            owner_of(&["project-implement"]),
        )
        .expect("builds");

        assert_eq!(tree.overrides_applied, 1);
        assert_eq!(
            tree.steps,
            vec![
                ResolvedStep {
                    id: "locate".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "project-implement".to_owned(),
                    layer: LayerKind::Project,
                    replaces: Some(Replacement {
                        replaced: "implement".to_owned(),
                        position: 2,
                    }),
                },
                ResolvedStep {
                    id: "superseded-context".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "finalize".to_owned(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
            ],
        );
    }

    /// (c) The built tree's step order equals the post-phase-4 composed include
    /// order — the tree is the *same* list `compose` expands. Driven over a
    /// non-trivial multi-delta cascade (insert + replace) so the equality is not a
    /// no-op identity: the tree ids are asserted **against
    /// `apply_structural_deltas`'s output**, the exact list the compose path
    /// consumes.
    #[test]
    fn tree_step_order_equals_post_phase4_include_order() {
        let deltas = [
            replace("workflow:single-task#implement", "project-implement"),
            insert(
                "workflow:single-task",
                AnchorSpec::After("locate".to_owned()),
                "team-lint",
            ),
        ];

        let composed = apply_structural_deltas(&fixture_includes(), &deltas).expect("applies");
        let tree = build_resolution_tree(
            "single-task",
            LayerKind::PackDefault,
            &fixture_includes(),
            &deltas,
            Vec::new(),
            Vec::new(),
            owner_of(&["project-implement", "team-lint"]),
        )
        .expect("builds");

        let tree_ids: Vec<&str> = tree.steps.iter().map(|s| s.id.as_str()).collect();
        let composed_ids: Vec<&str> = composed.iter().map(String::as_str).collect();
        assert_eq!(tree_ids, composed_ids);
    }

    // -- Phase 5: slot-fill application (`apply_slot_fills` / `fill_id_of`) --------

    /// Build a [`ResolvedFills`] from `(step, fill, content)` triples.
    fn fills(entries: &[(&str, &str, &str)]) -> ResolvedFills {
        entries
            .iter()
            .map(|(step, fill, content)| {
                ((step.to_string(), fill.to_string()), content.to_string())
            })
            .collect()
    }

    /// HEADLINE done-criterion: a body with two `{{fill:}}` points emits (a) the fed
    /// content inline at the **filled** point's line, and (b) the **empty** pack
    /// default at the unfilled point — and (c) the resolved content's own
    /// `{{include:}}` / `{{@…}}` are left **intact** for later phases (phase 5 runs
    /// before expansion + placeholder resolution). The golden pins the spliced bytes.
    #[test]
    fn fill_applies_filled_and_empty_default_leaving_nested_placeholders_intact() {
        let body = "\
Implement the change directly in the working tree.
{{ fill: extra-guidance }}
Then validate.
{{ fill: house-style }}
Done.
";
        // `extra-guidance` is filled with multi-line content that itself carries an
        // `{{include:}}` and an `{{@…}}` — both must survive phase 5 untouched.
        let filled = "\
Follow the house rule.
{{ include: step:team-lint }}
{{ @task.spec#criteria }}";
        let resolved = fills(&[("implement", "extra-guidance", filled)]);

        let applied = apply_slot_fills("implement", body, &resolved).expect("applies");

        // (c) the nested placeholders are carried through verbatim — not resolved,
        // not stripped — so later phases see them.
        assert!(applied.contains("{{ include: step:team-lint }}"));
        assert!(applied.contains("{{ @task.spec#criteria }}"));

        insta::assert_snapshot!(applied, @r"
        Implement the change directly in the working tree.
        Follow the house rule.
        {{ include: step:team-lint }}
        {{ @task.spec#criteria }}
        Then validate.
        Done.
        ");
    }

    /// A lone-line fill emits its content in place (the simplest (a) case —
    /// `x → "house rule"`), a **mid-line** `{{fill:}}` token resolves *in place*
    /// (token-anywhere, not only lone-line), and a body with no fill token at all
    /// passes through byte-for-byte (the no-fill common path).
    #[test]
    fn fill_single_line_inline_and_no_fill_is_identity() {
        let resolved = fills(&[("implement", "x", "house rule")]);

        let one = apply_slot_fills("implement", "before\n{{ fill: x }}\nafter\n", &resolved)
            .expect("applies");
        assert_eq!(one, "before\nhouse rule\nafter\n");

        // A mid-line `{{fill: x}}` token is substituted in place (not left intact).
        let mid = apply_slot_fills(
            "implement",
            "prose with a {{ fill: x }} mid-line.\n",
            &resolved,
        )
        .expect("applies");
        assert_eq!(mid, "prose with a house rule mid-line.\n");

        // No `{{fill:}}` token of any form → the body is returned unchanged. (The
        // `{{ cli.x }}` is a different placeholder kind, left for its own phase.)
        let plain = "Just prose with a {{ cli.x }} command-ref — no fill here.\n";
        let untouched = apply_slot_fills("implement", plain, &resolved).expect("applies");
        assert_eq!(untouched, plain);
    }

    /// A `{{fill:}}` **inside applied content** is left intact — the pass does not
    /// re-run, so the nested point survives to phase 8's `workflow-refs` survivor
    /// check (`overrides.md` → no nested fills: "phase 5 does not re-run").
    #[test]
    fn fill_does_not_recurse_into_applied_content() {
        let resolved = fills(&[("implement", "outer", "filled, but {{ fill: inner }} stays")]);
        let applied =
            apply_slot_fills("implement", "{{ fill: outer }}\n", &resolved).expect("applies");
        assert_eq!(applied, "filled, but {{ fill: inner }} stays\n");
    }

    /// A fill for a **different step** does not apply — the pass keys on
    /// `(step_id, fill_id)`, so an `other` step's `x` leaves `implement`'s `{{fill:
    /// x}}` at its empty default.
    #[test]
    fn fill_keys_on_step_id() {
        let resolved = fills(&[("other", "x", "wrong step")]);
        let applied =
            apply_slot_fills("implement", "a\n{{ fill: x }}\nb\n", &resolved).expect("applies");
        assert_eq!(applied, "a\nb\n");
    }

    /// `fill_id_of` recognizes a lone `{{fill: <id>}}` (whitespace-tolerant inside
    /// the braces) and rejects non-fill / malformed forms — distinct from the
    /// `cli.` and `include:` recognizers.
    #[test]
    fn fill_id_of_recognizes_only_lone_fill_placeholders() {
        assert_eq!(
            fill_id_of("{{fill: extra-guidance}}"),
            Some("extra-guidance")
        );
        assert_eq!(
            fill_id_of("{{ fill: extra-guidance }}"),
            Some("extra-guidance")
        );
        // Not a fill: other placeholder kinds, prose, malformed/empty ids.
        assert_eq!(fill_id_of("{{ cli.set-commit }}"), None);
        assert_eq!(fill_id_of("{{ include: step:x }}"), None);
        assert_eq!(fill_id_of("{{ fill: }}"), None);
        assert_eq!(fill_id_of("{{ fill: two words }}"), None);
        assert_eq!(fill_id_of("plain prose"), None);
    }

    /// `fill_ids_in` enumerates every `{{fill:}}` point a body declares — lone-line
    /// **and mid-line**, in occurrence order — and is empty for a body with none, the
    /// write-time recognizer the `config fill` checks drive (point-exists + no-nested).
    #[test]
    fn fill_ids_in_lists_declared_points_in_order() {
        let body = "intro\n{{fill: extra-guidance}}\nmid\n{{ fill: more }}\nend\n";
        assert_eq!(fill_ids_in(body), vec!["extra-guidance", "more"]);
        // Mid-line tokens count, in occurrence order, even multiple per line.
        let inline = "house rule with {{fill: a}} and {{ fill: b }} embedded\n";
        assert_eq!(fill_ids_in(inline), vec!["a", "b"]);
        // A body that declares no `{{fill:}}` point (other placeholder kinds don't count).
        assert!(fill_ids_in("plain\n{{ cli.x }}\n{{ include: step:y }}\n").is_empty());
    }

    /// `next_fill_token` finds a `{{fill:<id>}}` token **anywhere** in a string —
    /// mid-line included — and matches *only* `{{fill:…}}`, skipping other `{{…}}`
    /// placeholder kinds (command-refs, includes, data-values) and malformed fills.
    #[test]
    fn next_fill_token_matches_only_fill_anywhere() {
        let (range, id) = next_fill_token("pre {{fill: x}} post").expect("mid-line fill");
        assert_eq!(id, "x");
        assert_eq!(&"pre {{fill: x}} post"[range], "{{fill: x}}");

        // Other placeholder kinds are skipped, then the real fill is found past them.
        let (_, id) = next_fill_token("{{ cli.set }} then {{ include: step:y }} then {{fill: z}}")
            .expect("fill after non-fills");
        assert_eq!(id, "z");

        // No fill token at all (only other kinds / malformed fills) → None.
        assert!(next_fill_token("{{ cli.x }} and {{ fill: }} and {{ fill: two words }}").is_none());
        assert!(next_fill_token("plain prose, no braces").is_none());
    }

    // --- Spawn: the 5th emit class (fan-out step) (T3) ---

    /// A `creates-task: false` ctx fed the milestone's id-sorted sub-task ids — the
    /// `fan-out` step's `over:` list-source ([workflow-dialect.md] → data-value
    /// roots). No `task` root (a `fan-out`/`join` workflow operates on an existing
    /// milestone work-unit and mints no task).
    fn fan_out_ctx(ids: &[&str]) -> ComposeContext {
        ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: ids.iter().map(|s| (*s).to_owned()).collect(),
            source: None,
        }
    }

    /// The done-criterion fixture: a `fan-out` step over `{{ milestone.tasks }}` with
    /// `run: workflow:sub-task`, plus a later `join` step. Fed two ids it must emit
    /// **exactly** one `` Spawn: `cd <worktree> && jigc workflow sub-task --task <id>` ``
    /// per id in id order, no others; the `Spawn:` line matches the same strict
    /// line-start/backtick pattern `Run:` uses (`workflow-dialect.md` → Emitted format,
    /// rule 4).
    #[test]
    fn fan_out_emits_one_spawn_per_id_in_id_order() {
        let source = MapSource::new(&[
            (
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "---\njoin: {}\n---\nAll sub-tasks complete and merged by task-id order. Continue.\n",
            ),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: false,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["implement-tasks".to_owned(), "join-tasks".to_owned()],
        };
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        // Fed in id order: the resolver carries the already-id-sorted collection.
        let ctx = fan_out_ctx(&["alpha-fix", "zebra-fix"]);

        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");

        // Extract the emitted Spawn directives — the bytes the agent reads — and assert
        // on those, never a reconstruction.
        let spawns: Vec<&str> = composed
            .text
            .lines()
            .filter(|l| l.starts_with("Spawn: "))
            .collect();
        assert_eq!(
            spawns,
            vec![
                "Spawn: `cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`",
                "Spawn: `cd .jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix`",
            ],
            "exactly one Spawn per id, in id order, no others"
        );
        // The Spawn line matches the same strict line-start/backtick pattern `Run:` uses.
        for spawn in &spawns {
            assert!(
                spawn.starts_with("Spawn: `") && spawn.ends_with('`'),
                "Spawn line must be `^Spawn: `(.+)`$`, got {spawn:?}"
            );
        }
    }

    /// An **empty** milestone (no sub-tasks) → **zero** `Spawn:` lines — the
    /// empty-collection emits no directives (the empty-vs-unresolvable stance, same as
    /// an empty `catalog`/`store`).
    #[test]
    fn empty_milestone_emits_zero_spawn_lines() {
        let source = MapSource::new(&[
            (
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "---\njoin: {}\n---\nAll sub-tasks complete. Continue.\n",
            ),
        ]);
        let def = WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: false,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["implement-tasks".to_owned(), "join-tasks".to_owned()],
        };
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");
        let ctx = fan_out_ctx(&[]);

        let composed = compose(&def, &source, &catalog, &ctx).expect("composes");

        assert!(
            !composed.text.lines().any(|l| l.starts_with("Spawn: ")),
            "an empty milestone emits no Spawn directives, got:\n{}",
            composed.text
        );
    }

    // --- Determinism (#7) + the #5 face: order-invariant Spawn + marker honored (T5) ---

    /// The acceptance fixture for T5: the assembled T1–T4 spine — a `fan-out` step
    /// over `{{ milestone.tasks }}` (`run: workflow:sub-task`) paired with a later
    /// `join` step, the shape the increment ships. Reused by both T5 acceptance tests
    /// so the cross-order and marker-present/absent assertions run over one workflow.
    fn t5_fan_out_def() -> WorkflowDef {
        WorkflowDef {
            when: None,
            description: None,
            usage: None,
            creates_task: false,
            selectable: true,
            allows_create: vec![],
            reads: vec![],
            includes: vec!["implement-tasks".to_owned(), "join-tasks".to_owned()],
        }
    }

    /// The milestone ctx as the CLI actually feeds it: the sub-task id *set* run
    /// through [`crate::milestone::TaskList::enumerate`] (the id-sort boundary —
    /// Validation hardening #7), so insertion / feed order never reaches the resolver
    /// unsorted. Two divergent feed orders therefore produce the **same** collection.
    fn t5_ctx_via_enumerate(tasks: &[&str]) -> ComposeContext {
        let list = crate::milestone::TaskList {
            tasks: tasks.iter().map(|s| (*s).to_owned()).collect(),
        };
        ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: list.enumerate(),
            source: None,
        }
    }

    /// Validation hardening #7 (determinism by re-execution): composing the fan-out
    /// workflow over the milestone ctx fed `[zebra-fix, alpha-fix]` produces output
    /// **byte-identical** to the same ctx fed the reverse order `[alpha-fix,
    /// zebra-fix]` — fed through the `TaskList::enumerate()` id-sort boundary the CLI
    /// uses — with the `Spawn:` lines in id order in both. One green run is not a
    /// red→green: the two divergent feed orders are the proof the id-sort, not feed
    /// order, drives the emitted directive sequence.
    #[test]
    fn fan_out_spawns_byte_identical_across_divergent_feed_orders() {
        let source = MapSource::new(&[
            (
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "---\njoin: {}\n---\nAll sub-tasks complete and merged by task-id order. Continue.\n",
            ),
        ]);
        let def = t5_fan_out_def();
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");

        // Two divergent feed orders — minimum id-order and its reverse.
        let forward = compose(
            &def,
            &source,
            &catalog,
            &t5_ctx_via_enumerate(&["alpha-fix", "zebra-fix"]),
        )
        .expect("composes forward");
        let reversed = compose(
            &def,
            &source,
            &catalog,
            &t5_ctx_via_enumerate(&["zebra-fix", "alpha-fix"]),
        )
        .expect("composes reversed");

        // The whole composed view — every byte — is identical regardless of feed order.
        assert_eq!(
            forward.text, reversed.text,
            "compose output must be byte-identical across divergent feed orders"
        );

        // And the Spawn directives are the id-ordered sequence in both — extracted from
        // the emitted bytes the agent reads, never reconstructed.
        let spawns = |w: &ComposedWorkflow| -> Vec<String> {
            w.text
                .lines()
                .filter(|l| l.starts_with("Spawn: "))
                .map(str::to_owned)
                .collect()
        };
        let expected = vec![
            "Spawn: `cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`"
                .to_owned(),
            "Spawn: `cd .jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix`"
                .to_owned(),
        ];
        assert_eq!(
            spawns(&forward),
            expected,
            "forward feed: id-ordered Spawns"
        );
        assert_eq!(
            spawns(&reversed),
            expected,
            "reverse feed: id-ordered Spawns"
        );
    }

    /// Validation hardening #7, fed into the unit under test (not through the
    /// already-proven `enumerate()` sort): drive the public `compose` API with a
    /// **non-id-sorted** `ctx.milestone` directly and assert the emitted `Spawn:`
    /// directives come out **id-ordered** regardless. This is the genuine #7 proof —
    /// the divergent orders reach `compose`/`emit_fan_out_spawns`, so the emit path's
    /// own order-invariance (not the caller's boundary sort) is what is exercised. If
    /// the emit path leaked feed order, the directives would read `zebra-fix` before
    /// `alpha-fix` and this would fail.
    #[test]
    fn fan_out_spawns_id_ordered_from_unsorted_ctx() {
        let source = MapSource::new(&[
            (
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "---\njoin: {}\n---\nAll sub-tasks complete and merged by task-id order. Continue.\n",
            ),
        ]);
        let def = t5_fan_out_def();
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");

        // A milestone ctx whose ids are NOT id-sorted — fed straight to `compose`,
        // bypassing `TaskList::enumerate()`. Two divergent feed orders of the same set.
        let ctx_for = |order: &[&str]| ComposeContext {
            task: None,
            catalog: Vec::new(),
            store: std::collections::BTreeMap::new(),
            milestone: order.iter().map(|s| (*s).to_owned()).collect(),
            source: None,
        };
        let forward = compose(
            &def,
            &source,
            &catalog,
            &ctx_for(&["zebra-fix", "alpha-fix", "mid-fix"]),
        )
        .expect("composes forward");
        let reversed = compose(
            &def,
            &source,
            &catalog,
            &ctx_for(&["mid-fix", "alpha-fix", "zebra-fix"]),
        )
        .expect("composes reversed");

        let spawns = |w: &ComposedWorkflow| -> Vec<String> {
            w.text
                .lines()
                .filter(|l| l.starts_with("Spawn: "))
                .map(str::to_owned)
                .collect()
        };
        let expected = vec![
            "Spawn: `cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`"
                .to_owned(),
            "Spawn: `cd .jigc/worktrees/mid-fix && jigc workflow sub-task --task mid-fix`"
                .to_owned(),
            "Spawn: `cd .jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix`"
                .to_owned(),
        ];
        assert_eq!(
            spawns(&forward),
            expected,
            "emit path must id-order Spawns even from an unsorted ctx feed"
        );
        assert_eq!(spawns(&forward), spawns(&reversed));
        assert_eq!(
            forward.text, reversed.text,
            "compose output byte-identical across divergent unsorted feed orders"
        );
    }

    /// The M8 #5 face: a marker that parses but leaves compose output unchanged is
    /// parsed-but-ignored. The same step body composed **with** the `fan-out:`
    /// front-matter emits N `Spawn:` lines; composed with the marker **removed** (the
    /// identical instruction prose as a plain step) emits **zero** `Spawn:` lines and
    /// the two composed views **differ** — proving the marker is honored on the emit
    /// path, not merely recognized by the loader.
    #[test]
    fn marker_present_differs_from_marker_absent() {
        let ctx = t5_ctx_via_enumerate(&["alpha-fix", "zebra-fix"]);
        let catalog = load_command_catalog(COMMANDS_YAML).expect("loads");

        // The marker-present spine (a `fan-out`/`join` pair).
        let with_marker = MapSource::new(&[
            (
                "implement-tasks",
                "---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n  run:  workflow:sub-task\n---\nSpawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "---\njoin: {}\n---\nAll sub-tasks complete and merged by task-id order. Continue.\n",
            ),
        ]);
        // The marker-absent body — the *same* instruction prose, no `fan-out:`
        // front-matter (a plain step), and no `join` (a plain step is not a fan-out, so
        // the pairing check does not apply).
        let without_marker = MapSource::new(&[
            (
                "implement-tasks",
                "Spawn a sub-agent per task and implement it.\n",
            ),
            (
                "join-tasks",
                "All sub-tasks complete and merged by task-id order. Continue.\n",
            ),
        ]);
        let def = t5_fan_out_def();

        let present = compose(&def, &with_marker, &catalog, &ctx).expect("composes with marker");
        let absent =
            compose(&def, &without_marker, &catalog, &ctx).expect("composes without marker");

        let spawn_count = |w: &ComposedWorkflow| -> usize {
            w.text.lines().filter(|l| l.starts_with("Spawn: ")).count()
        };
        assert_eq!(
            spawn_count(&present),
            2,
            "marker present: one Spawn per id-sorted sub-task, got:\n{}",
            present.text
        );
        assert_eq!(
            spawn_count(&absent),
            0,
            "marker absent: zero Spawn lines (inert prose), got:\n{}",
            absent.text
        );
        assert_ne!(
            present.text, absent.text,
            "the marker must be honored — marker-present and marker-absent compose differently"
        );
    }
}
