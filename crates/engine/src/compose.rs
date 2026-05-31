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
    /// Whether running this workflow mints a task. Default `true` when the
    /// front-matter omits the key (`workflow-dialect.md` → On-disk format).
    pub creates_task: bool,
    /// The doctypes the agent may create during the task (default empty).
    pub allows_create: Vec<AllowsCreate>,
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
    #[serde(rename = "creates-task", default = "default_creates_task")]
    creates_task: bool,
    #[serde(rename = "allows-create", default)]
    allows_create: Vec<AllowsCreate>,
}

fn default_creates_task() -> bool {
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
    let trailing_newline = body.ends_with('\n');
    let mut out_lines = Vec::new();
    for line in body.lines() {
        out_lines.push(emit_line(line, ctx, catalog)?);
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

    // Content: a lone `{{@<path>}}` data-value placeholder.
    if let Some(inner) = parse_lone_placeholder(trimmed)
        && let Some(path_text) = inner.strip_prefix('@')
    {
        // Re-attach the `@` the resolver expects.
        let path = parse_data_value(&format!("@{path_text}"))?;
        return emit_content(&path, ctx);
    }

    // Reason (resolved data-value): a lone bare `{{<path>}}` data-value — the
    // **scalar** form (`{{task.intent}}` → the scalar string) or a bare path's
    // **address** (`workflow-dialect.md` → Leaves: bare path = the reference).
    // It carries no `> ` marker (that is the `@`-Content class); the resolved
    // text replaces the placeholder inline, emitted as Reason prose. A lone
    // `{{include: …}}` never reaches here — includes are expanded in phase 7 and
    // split out of step bodies before emission.
    if let Some(inner) = parse_lone_placeholder(trimmed) {
        let path = parse_data_value(inner)?;
        return emit_bare_data_value(&path, ctx);
    }

    // Reason: bare prose, verbatim.
    Ok(line.to_owned())
}

/// Emit a resolved `{{@<path>}}` content line: a `> ` blockquote of the resolved
/// address, or an **empty line** when the path resolves to
/// [`Resolution::Absent`] (a declared-but-unbound role; empty-not-finding).
fn emit_content(
    path: &crate::data_value::Path,
    ctx: &crate::data_value::ComposeContext,
) -> Result<String, Finding> {
    use crate::data_value::Resolution;
    match path.resolve(ctx)? {
        Resolution::Content { address } | Resolution::Address { address } => {
            Ok(format!("> {address}"))
        }
        Resolution::Scalar { value } => Ok(format!("> {value}")),
        Resolution::Absent => Ok(String::new()),
    }
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
    }
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
        Resolution::Absent => String::new(),
    };
    Ok(Some(format!("<<author: {address}>>")))
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
    let body = strip_optional_front_matter(text);
    Ok(StepDef {
        id: id.into(),
        body: body.to_owned(),
    })
}

/// Return the step's verbatim body: the post-front-matter remainder when `text`
/// opens with a `---`-fenced block, else `text` unchanged.
///
/// Reuses the one fence recognizer ([`crate::catalog::front_matter`]); the body is
/// the bytes after the closing `---` fence line's terminating newline (or after a
/// closing `---` at end-of-file). A file that does not open with a fence has no
/// front-matter, so its whole content is the body — byte-for-byte.
fn strip_optional_front_matter(text: &str) -> &str {
    match split_front_matter(text) {
        Some((_front, body)) => body,
        None => text,
    }
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
        creates_task: meta.creates_task,
        allows_create: meta.allows_create,
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
/// `VISION.md` principle #4 / `CLAUDE.md`), the *frontend* maps a step id to the
/// highest-precedence present layer's file (`overrides.md` phase 2 by-id
/// shadowing, already built in [`crate::cascade::Resolved::file_owner`]) and
/// loads its bytes into a [`StepDef`]. The engine consumes that mapping through
/// this trait, so [`expand_includes`] stays a pure function of `(WorkflowDef,
/// StepSource)` — feed-layers-in / assert-results-out. A `None` is a *dangling*
/// include id: the include names a step no layer provides.
pub trait StepSource {
    /// Resolve a step id to its [`StepDef`], or `None` if no layer provides it.
    fn step(&self, id: &str) -> Option<StepDef>;
}

/// One leaf of the flattened composition: a step's id and its body, placeholders
/// **still unresolved** (phase 7 output, before phase 8 placeholder resolution).
///
/// `body` is the step's verbatim prompt with its own nested `{{include: …}}`
/// lines removed — those nested steps are flattened into their own
/// [`ComposedStep`]s in pre-order immediately after this one (`overrides.md`
/// phase 7: "recursively expand each `{{include: step:foo}}` … Result: a flat
/// composition"). For a *plain* step (no nested includes — every MVP step), the
/// body is the verbatim step body unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ComposedStep {
    /// The frozen step id this leaf came from.
    pub id: String,
    /// The step's prompt body, nested include lines removed, placeholders unresolved.
    pub body: String,
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
        expand_step(id, source, &mut steps, &mut on_path)?;
    }
    Ok(Composition { steps })
}

/// Recursively expand one step id into `out`, pre-order, with cycle detection.
///
/// `on_path` is the active include path (the DFS stack). If `id` is already on
/// it, this is an include cycle. Otherwise the step resolves, its body's nested
/// include lines are split out, the de-included body is emitted as this step's
/// [`ComposedStep`], and each nested include is expanded in physical order
/// immediately after — yielding a pre-order flattening.
fn expand_step(
    id: &str,
    source: &dyn StepSource,
    out: &mut Vec<ComposedStep>,
    on_path: &mut Vec<String>,
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

    let (body, nested) = split_nested_includes(&step.body);
    out.push(ComposedStep {
        id: id.to_owned(),
        body,
    });

    on_path.push(id.to_owned());
    for child in &nested {
        expand_step(child, source, out, on_path)?;
    }
    on_path.pop();
    Ok(())
}

/// Split a step body into `(de_included_body, nested_ids)`: the body with its
/// `{{include: step:<id>}}` lines removed, and the ordered ids those lines named.
///
/// A line is a nested include when [`parse_include_line`] accepts its trimmed
/// form (the same recognizer the workflow body uses). Non-include lines — the
/// step's prose, its `{{cli.…}}` / `{{…}}` / `<<author:…>>` leaves — are kept
/// verbatim. A body with no nested includes (every MVP step) returns unchanged.
fn split_nested_includes(body: &str) -> (String, Vec<String>) {
    // Fast path: no include line at all → body is returned byte-for-byte.
    if !body.lines().any(|l| parse_include_line(l.trim()).is_some()) {
        return (body.to_owned(), Vec::new());
    }
    let mut kept = String::with_capacity(body.len());
    let mut nested = Vec::new();
    for line in body.lines() {
        match parse_include_line(line.trim()) {
            Some(id) => nested.push(id),
            None => {
                kept.push_str(line);
                kept.push('\n');
            }
        }
    }
    (kept, nested)
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
    let composition = expand_includes(def, source)?;
    let mut emitted_steps = Vec::with_capacity(composition.steps.len());
    for step in &composition.steps {
        emitted_steps.push(emit_step_body(&step.body, ctx, catalog)?);
    }
    // Join the per-step emitted texts with a single blank line between steps, so
    // the composed view reads as one ordered document. Each step body already
    // carries its own internal newlines; we trim a step's trailing newline before
    // the separator so the separator is exactly one blank line, not two.
    let mut text = String::new();
    for (i, emitted) in emitted_steps.iter().enumerate() {
        if i > 0 {
            text.push('\n');
        }
        text.push_str(emitted.trim_end_matches('\n'));
        text.push('\n');
    }
    Ok(ComposedWorkflow { text })
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
            task: crate::data_value::TaskRoot {
                id: "emit-four-classes".to_owned(),
                intent: "emit a composed step body to the four-class format".to_owned(),
                roles,
            },
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
            "Run: `jigc doc set-slot commit:emit-four-classes#summary --from-file -`"
        );

        // The `<<author:>>` wrapper survives; only the embedded `{{…}}` resolves.
        assert!(emitted.contains("<<author: commit:emit-four-classes#summary>>"));

        insta::assert_snapshot!(emitted, @r#"
        Implement the change directly. When done, stage the commit prose:
        Run: `jigc doc set-slot commit:emit-four-classes#summary --from-file -`
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

        // All four shipped command-refs are present under their ids.
        for id in [
            "set-commit-summary",
            "validate-task",
            "finalize-task",
            "create-adr",
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
                }
              ],
              "hint": "Create a new ADR in the current task."
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
                }
              ],
              "stdin": "the slot prose",
              "hint": "Stage the commit summary slot from stdin."
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
          "creates_task": true,
          "allows_create": [
            {
              "type": "adr",
              "as": "decision"
            }
          ],
          "includes": [
            "locate",
            "implement",
            "superseded-context",
            "finalize"
          ]
        }
        "#);
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
If your decision supersedes an earlier one, here is that decision for
reference — make your consequences explain what changes:
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
        If your decision supersedes an earlier one, here is that decision for
        reference — make your consequences explain what changes:
        {{ @task.decision.supersedes#decision }}
        "###);

        // A step *with* front-matter: the body is exactly the post-fence remainder,
        // byte-for-byte — the front-matter (config-family YAML) is stripped, the
        // prose preserved verbatim including its internal blank lines.
        let fenced = load_step_def(
            "fan-out-step",
            b"---\nfan-out:\n  over: \"{{ milestone.tasks }}\"\n---\nSpawn a sub-task per item.\n\nEach runs the sub-workflow.\n",
        )
        .expect("loads");
        assert_eq!(fenced.id, "fan-out-step");
        assert_eq!(
            fenced.body,
            "Spawn a sub-task per item.\n\nEach runs the sub-workflow.\n"
        );
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
            task: TaskRoot {
                id: "add-rate-limiter".to_owned(),
                intent: "add a rate limiter to the API".to_owned(),
                roles,
            },
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
            "jigc doc set-slot commit:add-rate-limiter#summary --from-file -"
        );

        let create_adr = catalog.get("create-adr").expect("present");
        assert_eq!(
            render_command(create_adr, &ctx).expect("renders"),
            "jigc doc create adr --title <TITLE>"
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
          => jigc doc create adr --title <TITLE>
        finalize-task
          => jigc task finalize add-rate-limiter
        set-commit-summary
          => jigc doc set-slot commit:add-rate-limiter#summary --from-file -
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
            self.0.get(id).map(|body| StepDef {
                id: id.to_owned(),
                body: body.clone(),
            })
        }
    }

    /// The four shipped single-task step bodies, byte-identical to the pack files
    /// (the plain ones are asserted equal to the in-module consts above).
    fn single_task_source() -> MapSource {
        MapSource::new(&[
            ("locate", STEP_LOCATE),
            (
                "implement",
                include_str!("../../cli/pack/steps/implement.yaml"),
            ),
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
            creates_task: true,
            allows_create: vec![],
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
            creates_task: true,
            allows_create: vec![],
            includes: vec!["a".to_owned()],
        };
        let err = expand_includes(&cyclic, &cyclic_source).expect_err("cycle rejects");
        assert_eq!(err.code, "workflow-refs.include-cycle-absent");
        assert_eq!(err.severity, crate::finding::Severity::Blocking);
    }

    /// A nested include inside a step body is expanded in pre-order: the parent
    /// step's de-included body comes first, then the nested step's body — phase 7
    /// recursive expansion (`overrides.md`). The nested `{{include:}}` line is
    /// removed from the parent body; the parent's other prose survives verbatim.
    #[test]
    fn nested_include_in_step_body_flattens_pre_order() {
        let source = MapSource::new(&[
            ("parent", "before\n{{ include: step:child }}\nafter\n"),
            ("child", "child body\n"),
        ]);
        let def = WorkflowDef {
            when: None,
            creates_task: true,
            allows_create: vec![],
            includes: vec!["parent".to_owned()],
        };

        let composition = expand_includes(&def, &source).expect("expands");

        assert_eq!(composition.step_ids(), vec!["parent", "child"]);
        // The parent body keeps its prose but drops the nested include line.
        assert_eq!(composition.steps[0].body, "before\nafter\n");
        assert_eq!(composition.steps[1].body, "child body\n");
    }

    /// A self-including step (A includes A) is a cycle the moment it re-enters.
    #[test]
    fn self_include_is_a_cycle() {
        let source = MapSource::new(&[("a", "loop\n{{ include: step:a }}\n")]);
        let def = WorkflowDef {
            when: None,
            creates_task: true,
            allows_create: vec![],
            includes: vec!["a".to_owned()],
        };
        let err = expand_includes(&def, &source).expect_err("self-cycle rejects");
        assert_eq!(err.code, "workflow-refs.include-cycle-absent");
    }

    /// A diamond (A→B, A→C, B→D, C→D) is **not** a cycle: D is reached twice but
    /// never while already on the active path, so it expands once per inclusion in
    /// pre-order. Cycle detection keys on the active DFS path, not on having seen
    /// an id before.
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
            creates_task: true,
            allows_create: vec![],
            includes: vec!["a".to_owned()],
        };
        let composition = expand_includes(&def, &source).expect("diamond expands");
        // Pre-order: a, then b's subtree (b, d), then c's subtree (c, d).
        assert_eq!(composition.step_ids(), vec!["a", "b", "d", "c", "d"]);
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
                creates_task: true,
                allows_create: vec![],
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
            task: TaskRoot {
                id: "add-rate-limiter".to_owned(),
                intent: "add rate limiter".to_owned(),
                roles,
            },
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

        insta::assert_snapshot!(composed.text, @r#"
        Reason about the change. The intent is:
        add rate limiter

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.

        Implement the change directly in the working tree. When done, stage the
        commit prose:

        Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -`
        <<author: commit:add-rate-limiter#summary>>

        If a decision is warranted, create an ADR and author its slots:

        Run: `jigc doc create adr --title <TITLE>`

        If your decision supersedes an earlier one, here is that decision for
        reference — make your consequences explain what changes:

        Validate and commit the task as one logical commit:

        Run: `jigc task finalize add-rate-limiter`
        "#);
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
            ctx.task.intent = intent;

            let first = compose(&def, &source, &catalog, &ctx);
            let second = compose(&def, &source, &catalog, &ctx);
            proptest::prop_assert_eq!(first, second);
        }
    }
}
