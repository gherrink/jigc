//! Assistant-adapter generation from embedded profiles + the engine catalog.
//!
//! Scope: the bootstrap floor (by reference) + the `SessionStart` hook (the
//! primary injection) + the `jigc` allowlist + the **spawn** launch template (the
//! `Resume` hook is post-MVP). See `design/assistant-adapter.md`.
//!
//! Adapter **profiles** are embedded config-family data, the same pattern as the
//! pack (`implementation/module-layout.md` → Adapter (in `cli`): "Profiles are
//! embedded data … same pattern as the pack"). Each profile is the
//! assistant-neutral→assistant shim: the source of *where/how* to inject the
//! bootstrap and *how* to allowlist `jigc`. The MVP model carries the in-scope
//! surfaces: the inject **reference** target (the universal floor — a managed
//! bootstrap file plus an `@`-import pointer), the inject **hook** target (the
//! primary injection — a `SessionStart` event running the front door), and the
//! **allowlist** target (the path-of-least-resistance the bootstrap depends on),
//! and the **spawn** launch template (the assistant-specific fan-out launch line);
//! the `Resume` hook is post-MVP and intentionally absent from both the model and
//! the shipped profile bytes, so what we test is what ships.
//!
//! The bootstrap floor is **by reference, not inlined** (`DECISIONS.md`
//! 2026-05-31 → adapter install reworked): `jigc setup` writes the canonical
//! sentence into a managed `.jigc/AGENT.md` (regenerated whole each run) and
//! injects a bare `@.jigc/AGENT.md` import into the always-loaded file, idempotent
//! on that exact reference line — no marker-fenced block in `CLAUDE.md`.

use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

/// The built-in adapter profiles, embedded at compile time from
/// `crates/cli/adapters/`. Mirrors `pack::PACK` (`include_dir`, decided
/// 2026-05-31): always-embedded, so what you test is what ships.
static ADAPTERS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/adapters");

/// A parsed adapter profile: the assistant-specific shim telling the CLI *where
/// and how* to wire into one coding assistant.
///
/// Deserialized from the config-family YAML at `adapters/<assistant>.yaml`. The
/// surface is the inject **reference** floor + the **hook** + the **allowlist** +
/// the **spawn** launch template; the `Resume` hook is post-MVP and absent here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterProfile {
    /// The assistant this profile wires into (e.g. `claude-code`). Matches the
    /// file stem the loader resolves by.
    pub assistant: String,

    /// The bootstrap injection targets. MVP ships two: the **reference** floor
    /// (a managed bootstrap file plus an `@`-import pointer in the always-loaded
    /// file) and the **hook** (the primary injection — a `SessionStart` event
    /// running the front door).
    pub inject: Vec<InjectTarget>,

    /// Where and what to allowlist so `jigc` runs without friction.
    pub allowlist: Allowlist,

    /// The **spawn** launch template — the assistant-specific bit the CLI renders
    /// its fan-out dispatch (`workflow` + `task_id`) through to reach the
    /// assistant's launch primitive (for Claude Code, a Task-tool invocation
    /// running `jigc workflow … --task …`). The locked seam: CLI owns the payload,
    /// the adapter owns the launch.
    pub spawn: SpawnTarget,
}

impl AdapterProfile {
    /// The first inject **reference** floor target, if the profile declares one —
    /// the always-loaded-file pointer `setup` injects.
    pub fn reference(&self) -> Option<&ReferenceTarget> {
        self.inject.iter().find_map(|t| match t {
            InjectTarget::Reference { reference } => Some(reference),
            InjectTarget::Hook { .. } => None,
        })
    }

    /// The first inject **hook** target, if the profile declares one — the
    /// session-event binding `setup` installs into the assistant settings file.
    pub fn hook(&self) -> Option<&HookTarget> {
        self.inject.iter().find_map(|t| match t {
            InjectTarget::Hook { hook } => Some(hook),
            InjectTarget::Reference { .. } => None,
        })
    }

    /// The profile's **spawn** launch template — the fan-out launch line the CLI
    /// renders through to reach the assistant's launch primitive. Mirrors
    /// [`reference`](Self::reference)/[`hook`](Self::hook) as the accessor for the
    /// spawn surface (always present: `spawn` is a required field). Consumed by
    /// the install-time validation + launch path (this increment's later tasks).
    #[allow(dead_code)]
    pub fn spawn(&self) -> Option<&SpawnTarget> {
        Some(&self.spawn)
    }
}

/// One bootstrap injection target — an **untagged** variant keyed by its single
/// YAML map key (`reference:` or `hook:`), the natural config-family shape (each
/// list entry is a one-key map). MVP carries the `reference` floor and the `hook`
/// (the primary `SessionStart` injection); the `resume` hook and the spawn variant
/// are post-MVP. Untagged (not serde's externally-tagged enum) because the YAML is
/// a map-with-one-key, not a `!Tag`; the two variants' fields are disjoint, so
/// disambiguation is unambiguous.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InjectTarget {
    /// The reference floor: write the bootstrap into a managed file and point the
    /// always-loaded file at it with an import line.
    Reference {
        /// The reference floor target.
        reference: ReferenceTarget,
    },

    /// The hook injection: bind an assistant session event (e.g. `SessionStart`)
    /// to a `jigc` command so the bootstrap reaches the agent at session start.
    Hook {
        /// The hook injection target.
        hook: HookTarget,
    },
}

/// The reference floor target: which always-loaded file gets the import line,
/// the managed bootstrap file it points `to`, and the import `syntax`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceTarget {
    /// The always-loaded file the import line lands in (e.g. `CLAUDE.md`).
    pub file: String,

    /// The managed bootstrap file the import points at (e.g. `.jigc/AGENT.md`).
    pub to: String,

    /// The import syntax the assistant understands (e.g. `at-import` →
    /// `@<to>`).
    pub syntax: String,
}

/// The hook injection target: the assistant session `event` to bind and the
/// `jigc` command to `run` on it (e.g. `SessionStart` → `jigc start`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HookTarget {
    /// The assistant session event to bind (e.g. `SessionStart`). Matches the key
    /// the assistant settings file groups hooks under.
    pub event: String,

    /// The `jigc` command the hook runs on the event (e.g. `jigc start` —
    /// read-only orientation, the primary bootstrap injection).
    pub run: String,
}

/// The allowlist target: which settings file to edit and which command patterns
/// to permit so the agent runs `jigc` without a prompt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Allowlist {
    /// The assistant's permission/settings file (e.g. `.claude/settings.json`).
    pub file: String,

    /// The command patterns to permit (e.g. `["jigc *"]`).
    pub permit: Vec<String>,
}

/// The spawn launch target: the launch `template` the CLI renders its fan-out
/// dispatch through. The template carries the two lexical placeholders
/// `{{workflow}}` and `{{task_id}}` (plain template tokens, **not** the engine
/// `{{…}}` data-value grammar) wrapped in the assistant's launch primitive
/// (`design/assistant-adapter.md` → Bind the spawn mechanism).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpawnTarget {
    /// The launch template — one line of wrapping prose around a backticked
    /// `jigc workflow {{workflow}} --task {{task_id}}` invocation.
    pub template: String,
}

/// Render the spawn launch line: a lexical three-token substitution of the
/// `{{worktree}}`, `{{workflow}}`, and `{{task_id}}` placeholders in `template`,
/// leaving every other byte verbatim. `{{worktree}}` resolves to the sub-task's
/// deterministic worktree path (`.jigc/worktrees/<task_id>`, the shared
/// [`engine::milestone::worktree_path`] convention the fan-out emit also uses) so
/// the launched sub-agent `cd`s into its own detached checkout (M31 WF4).
///
/// Plain string replacement, **not** the engine `{{…}}` data-value grammar — the
/// spawn template's tokens are launch-line placeholders the adapter fills, never
/// data-value refs (`design/assistant-adapter.md` → Bind the spawn mechanism). So
/// it does not route through `data_value`/`compose`. Consumed by the launch path
/// (this increment's later tasks).
#[allow(dead_code)]
pub fn render_spawn(template: &str, workflow: &str, task_id: &str) -> String {
    let worktree = engine::milestone::worktree_path(task_id);
    template
        .replace("{{worktree}}", &worktree.display().to_string())
        .replace("{{workflow}}", workflow)
        .replace("{{task_id}}", task_id)
}

/// Which clause of the decidable spawn-template rule a template violated —
/// the typed pointer the install error surfaces.
///
/// A small typed enum (not a `String` message) so the install path can route a
/// precise pointer to the violated clause and the unit table can assert the
/// *specific* clause (`DECISIONS.md` inc-4 planning pin (ii) — typed reason vs a
/// `Result<(), String>`). The rule is purely lexical and decidable
/// (`design/assistant-adapter.md` → Bind the spawn mechanism); each variant is one
/// clause of that rule. `Display` carries the human pointer the install surfaces.
/// Consumed by the install path (`setup::install`); exercised here by the unit
/// clause-coverage table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnTemplateReason {
    /// The template does not literally contain the `{{task_id}}` placeholder.
    MissingTaskIdPlaceholder,

    /// The template does not literally contain the `{{workflow}}` placeholder.
    MissingWorkflowPlaceholder,

    /// The template carries no `jigc workflow … --task …` invocation.
    MissingInvocation,

    /// The template contains more than one newline (the ceiling is ≤ 1).
    TooManyNewlines,

    /// The template does not contain exactly one backticked span.
    NotExactlyOneBacktickSpan,

    /// The template carries a forbidden marker — a `<<author:` slot-author marker,
    /// an ATX heading (`#`/`##` at a line start), or a `Run:`/`Spawn:`/`> `
    /// directive marker — signalling re-authored workflow-body prose.
    ForbiddenMarker,

    /// The backticked span carries shell beyond the permitted shape: the only
    /// allowed prefix on the `jigc workflow …` invocation is a single
    /// `cd <one-token> &&` (the worktree `cd`); any other command, a multi-token
    /// `cd` argument, or trailing `&&`-chaining is rejected (the M31 scoped
    /// relaxation — one structured token, validated structurally).
    DisallowedShellPrefix,
}

impl fmt::Display for SpawnTemplateReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pointer = match self {
            SpawnTemplateReason::MissingTaskIdPlaceholder => {
                "missing the required `{{task_id}}` placeholder"
            }
            SpawnTemplateReason::MissingWorkflowPlaceholder => {
                "missing the required `{{workflow}}` placeholder"
            }
            SpawnTemplateReason::MissingInvocation => {
                "missing the required `jigc workflow … --task …` invocation"
            }
            SpawnTemplateReason::TooManyNewlines => {
                "contains more than one newline (the launch line must be a single line)"
            }
            SpawnTemplateReason::NotExactlyOneBacktickSpan => {
                "must contain exactly one backticked span (the invocation)"
            }
            SpawnTemplateReason::ForbiddenMarker => {
                "contains a forbidden marker (`<<author:`, an ATX heading, or a `Run:`/`Spawn:`/`> ` directive)"
            }
            SpawnTemplateReason::DisallowedShellPrefix => {
                "carries disallowed shell — the invocation may take only a single `cd <worktree> &&` prefix"
            }
        };
        write!(f, "spawn template {pointer}")
    }
}

/// Validate a spawn launch template against the **decidable** install-time rule
/// (`design/assistant-adapter.md` → Bind the spawn mechanism; `DECISIONS.md`
/// 2026-06-04 — install-time launch-template validation = full, a decidable
/// forbidden-pattern rule, not a heuristic).
///
/// A pure function of the **whole** (unrendered) template string — no I/O. The
/// rule in one place, purely lexical and decidable (no judgement):
///
/// 1. both `{{task_id}}` and `{{workflow}}` literally present;
/// 2. a `jigc workflow … --task …` invocation present;
/// 3. **≤ 1 newline** total, **exactly one** backticked span, and **no** forbidden
///    marker — a `<<author:` slot-author marker, an ATX heading (`#`/`##` at a line
///    start), or a `Run:`/`Spawn:`/`> ` directive marker — anywhere.
///
/// On failure returns the [`SpawnTemplateReason`] naming the violated clause (the
/// install error's precise pointer). Clauses are checked in rule order, so the
/// first violated clause is reported.
///
/// Consumed by the install path (`setup::install`); exercised here by the unit
/// clause-coverage table.
pub fn validate_spawn_template(template: &str) -> Result<(), SpawnTemplateReason> {
    // (1) required placeholders, literally present.
    if !template.contains("{{task_id}}") {
        return Err(SpawnTemplateReason::MissingTaskIdPlaceholder);
    }
    if !template.contains("{{workflow}}") {
        return Err(SpawnTemplateReason::MissingWorkflowPlaceholder);
    }

    // (2) the required invocation pattern: `jigc workflow` … `--task` (in order).
    let invokes = template
        .find("jigc workflow")
        .and_then(|wf| template[wf..].find("--task").map(|_| ()))
        .is_some();
    if !invokes {
        return Err(SpawnTemplateReason::MissingInvocation);
    }

    // (3a) ≤ 1 newline total.
    if template.matches('\n').count() > 1 {
        return Err(SpawnTemplateReason::TooManyNewlines);
    }

    // (3b) exactly one backticked span — an even, nonzero count of backticks with
    // exactly one opening one (2 backticks = one span).
    if template.matches('`').count() != 2 {
        return Err(SpawnTemplateReason::NotExactlyOneBacktickSpan);
    }

    // (3c) no forbidden marker anywhere.
    if template.contains("<<author:") || template.lines().any(line_carries_forbidden_marker) {
        return Err(SpawnTemplateReason::ForbiddenMarker);
    }

    // (4) the backticked span itself carries only the permitted shape — the bare
    //     `jigc workflow … --task …` invocation, optionally preceded by a single
    //     `cd <one-token> &&` worktree prefix, and no other shell (the M31 scoped
    //     relaxation, validated structurally). The exactly-one-span clause above
    //     guarantees the span sits between the two backticks.
    let span = template
        .split('`')
        .nth(1)
        .expect("exactly one backticked span is guaranteed above");
    if !spawn_span_is_permitted(span) {
        return Err(SpawnTemplateReason::DisallowedShellPrefix);
    }

    Ok(())
}

/// Whether the backticked launch span is the permitted shape: the bare
/// `jigc workflow …` invocation, optionally preceded by a single `cd <one-token> &&`
/// worktree prefix, and with **no** further shell chaining — the structural half of
/// the M31 spawn-template reopen (`design/assistant-adapter.md` → Bind the spawn
/// mechanism). The `cd` argument is one whitespace-delimited token validated
/// structurally (it is a CLI-filled `{{worktree}}` placeholder), never matched
/// against a value — the determinism rationale preserved.
fn spawn_span_is_permitted(span: &str) -> bool {
    // Strip an optional single `cd <token> &&` prefix; the remainder must be the
    // bare invocation with no residual shell chaining.
    let invocation = match span.split_once("&&") {
        None => span.trim(),
        Some((prefix, rest)) => {
            let Some(arg) = prefix.trim().strip_prefix("cd ") else {
                return false;
            };
            // Exactly one whitespace-delimited token after `cd`.
            if arg.split_whitespace().count() != 1 {
                return false;
            }
            rest.trim()
        }
    };
    // The remainder is exactly the invocation: it starts with `jigc workflow` and
    // chains nothing further (the single permitted `&&` is already consumed).
    invocation.starts_with("jigc workflow") && !invocation.contains("&&")
}

/// Whether a single line opens with an ATX heading (`#`/`##` …) or a
/// `Run:`/`Spawn:`/`> ` directive marker, after stripping leading whitespace —
/// the line-anchored half of the forbidden-marker clause of
/// [`validate_spawn_template`].
fn line_carries_forbidden_marker(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with('#')
        || trimmed.starts_with("Run:")
        || trimmed.starts_with("Spawn:")
        || trimmed.starts_with("> ")
}

/// Why loading an adapter profile failed.
///
/// A small typed enum (not an `anyhow` message) because the caller must
/// distinguish the **unknown-assistant** case — `jigc setup` routes it to a
/// clear "no profile for X" install error, and the loader test matches on it.
/// Hand-rolled `Display`/`Error` keep `cli` on `anyhow` for everything else; no
/// `thiserror` dep is added to the crate (the engine/CLI error split holds).
#[derive(Debug)]
pub enum ProfileError {
    /// No embedded profile ships for the requested assistant name.
    NotFound(String),

    /// The profile bytes were not valid UTF-8 (profiles are text).
    NotUtf8 {
        /// The assistant whose profile failed to decode.
        assistant: String,
    },

    /// The YAML did not match the [`AdapterProfile`] model.
    Malformed {
        /// The assistant whose profile failed to parse.
        assistant: String,
        /// The underlying deserialization error.
        source: serde_yaml_ng::Error,
    },

    /// A `JIGC_ADAPTERS_DIR`-selected profile file could not be read from disk
    /// (e.g. the directory or `<assistant>.yaml` is absent). Only reachable when
    /// the override is set; the embedded path never hits the filesystem.
    Io {
        /// The assistant whose profile failed to read.
        assistant: String,
        /// The underlying IO error.
        source: std::io::Error,
    },
}

impl fmt::Display for ProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileError::NotFound(assistant) => {
                write!(f, "no adapter profile for assistant `{assistant}`")
            }
            ProfileError::NotUtf8 { assistant } => {
                write!(f, "adapter profile for `{assistant}` is not valid UTF-8")
            }
            ProfileError::Malformed { assistant, source } => {
                write!(f, "malformed adapter profile for `{assistant}`: {source}")
            }
            ProfileError::Io { assistant, source } => {
                write!(f, "cannot read adapter profile for `{assistant}`: {source}")
            }
        }
    }
}

impl std::error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProfileError::Malformed { source, .. } => Some(source),
            ProfileError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// The managed bootstrap file's contents: the canonical routing sentence as the
/// file body, with a trailing newline.
///
/// A pure function of the embedded contract — no filesystem. The file is wholly
/// CLI-owned and rewritten in full each `setup`, so it needs no in-file
/// idempotency markers; the body is just the sentence.
pub fn bootstrap_file() -> String {
    format!("{BOOTSTRAP_SENTENCE}\n")
}

/// Write the managed bootstrap file (`<repo_root>/.jigc/AGENT.md`) with the
/// canonical sentence, creating `.jigc/` if needed.
///
/// `jigc setup`'s reference floor ([`assistant-adapter.md`] → Inject the
/// bootstrap). Fully CLI-owned: rewritten **whole** on every run, so a re-run is
/// byte-identical and the content stays rot-proof (the *structure-is-generated*
/// discipline applied to the floor). Regenerated on upgrade.
pub fn write_bootstrap_file(repo_root: &Path) -> std::io::Result<()> {
    let target = repo_root.join(BOOTSTRAP_FILE_REL);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&target, bootstrap_file())
}

/// Idempotently inject the bare `@.jigc/AGENT.md` import line into the host
/// project's always-loaded file (`<repo_root>/CLAUDE.md`).
///
/// `jigc setup`'s reference floor ([`assistant-adapter.md`] → Inject the
/// bootstrap; [`bootstrap.md`] → Placement is the adapter). Idempotent by
/// detecting the exact reference line: a missing file is created with the heading
/// and the import line; a file already carrying the exact `@.jigc/AGENT.md` line
/// is left **byte-identical** (no duplicate); a file without it gets the section
/// **appended** (surrounding content untouched). No marker comments — the floor
/// is a single pointer line, with the content in the managed file.
pub fn inject_reference(repo_root: &Path) -> std::io::Result<()> {
    let target = repo_root.join("CLAUDE.md");
    let section = format!("## Project interface\n\n{BOOTSTRAP_IMPORT_LINE}\n");

    let next = match std::fs::read_to_string(&target) {
        // No file yet: it is exactly the section.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => section,
        Err(e) => return Err(e),
        Ok(existing) => {
            if contains_import_line(&existing) {
                // The exact reference is already present: a byte-for-byte no-op.
                return Ok(());
            }
            // Append the section after a **fixed** `"\n"` separator — never
            // normalizing the existing trailing newline — so [`unwire_reference`]
            // can strip exactly `"\n" + section` and restore the pre-setup bytes
            // byte-for-byte regardless of whether the original ended in `\n`. A
            // newline-terminated original (`X\n`) gets a blank line before the
            // section (`X\n\nsection`); a non-terminated one (`X`) gets the section
            // on the next line (`X\nsection`). An empty file degenerates to just
            // the section.
            if existing.is_empty() {
                section
            } else {
                let mut out = String::with_capacity(existing.len() + section.len() + 1);
                out.push_str(&existing);
                out.push('\n');
                out.push_str(&section);
                out
            }
        }
    };

    std::fs::write(&target, next)
}

/// Idempotently merge the profile's allowlist permits into the host project's
/// assistant settings file (`<repo_root>/<profile.allowlist.file>`, e.g.
/// `.claude/settings.json`).
///
/// `jigc setup`'s allowlist step ([`assistant-adapter.md`] → Make `jigc`
/// frictionless: the path-of-least-resistance the bootstrap depends on). A
/// **structure-aware** JSON merge, not a text splice: parse (or create) the
/// settings object, ensure every permit pattern from the profile is present in
/// the `permissions.allow` array, and write back pretty JSON. Idempotent by
/// **structural presence** (decided 2026-05-31), not by text markers — a permit
/// already in the array is a no-op, so a re-run is byte-identical; unrelated
/// top-level keys and unrelated permissions are preserved (a missing file is
/// created with just the permit(s); a missing `permissions` object or `allow`
/// array is created). Regenerated on upgrade, so the integration can't rot.
pub fn inject_allowlist(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<()> {
    let target = repo_root.join(&profile.allowlist.file);

    let mut settings = read_settings(&target)?;

    // Navigate/create `permissions.allow`, then ensure each permit is present.
    let allow = settings
        .as_object_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "settings root is not a JSON object",
            )
        })?
        .entry("permissions")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
        .as_object_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "`permissions` is not a JSON object",
            )
        })?
        .entry("allow")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "`permissions.allow` is not a JSON array",
            )
        })?;

    for permit in &profile.allowlist.permit {
        let present = allow.iter().any(|v| v.as_str() == Some(permit.as_str()));
        if !present {
            allow.push(serde_json::Value::String(permit.clone()));
        }
    }

    write_settings(&target, &settings)
}

/// Idempotently **unwire** the bootstrap reference from the host project's
/// always-loaded file (`<repo_root>/CLAUDE.md`) — the inverse of
/// [`inject_reference`] for `jigc uninstall` (`design/project-setup.md` → Flow 2
/// hardening → Teardown / cleanup (G5), bullet (b)).
///
/// Removes the jigc-injected `## Project interface` section (the heading, its blank
/// line, and the bare `@.jigc/AGENT.md` import line) that [`inject_reference`]
/// appends, restoring the human's pre-existing content **byte-for-byte**. Matches
/// the exact appended form — a leading separator newline + the section block — so a
/// `setup`→`uninstall` round-trip returns `CLAUDE.md` to its pre-setup bytes; if the
/// file is exactly the section (setup created it), it is removed entirely.
///
/// **Idempotent + non-destructive:** a file without the section is left untouched
/// (a clean no-op, so a second `uninstall` is a no-op), an absent file is a no-op,
/// and only the exact jigc-injected section is stripped — a human who hand-wrote the
/// `@.jigc/AGENT.md` line under their own heading keeps their surrounding prose
/// (only the bare import line + any jigc `## Project interface` block we recognize is
/// removed; see the matching below).
pub fn unwire_reference(repo_root: &Path) -> std::io::Result<()> {
    let target = repo_root.join("CLAUDE.md");
    let existing = match std::fs::read_to_string(&target) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
        Ok(existing) => existing,
    };

    // The exact section [`inject_reference`] writes, and the form it appends it in.
    let section = format!("## Project interface\n\n{BOOTSTRAP_IMPORT_LINE}\n");

    let next = if existing == section {
        // setup created the file (it is exactly the section): remove it entirely.
        String::new()
    } else if let Some(stripped) = existing.strip_suffix(&format!("\n{section}")) {
        // The append form: `inject_reference` writes `existing + "\n" + section`
        // with a **fixed** `"\n"` separator (never normalizing the existing
        // trailing newline), so stripping `"\n" + section` restores `existing`'s
        // pre-setup bytes exactly — whether or not the original ended in `\n`.
        String::from(stripped)
    } else {
        // No jigc-injected section to remove: a clean no-op (idempotent — a second
        // `uninstall`, or a file that never carried our section, is untouched).
        return Ok(());
    };

    if next.is_empty() {
        // setup created the file; remove it so uninstall leaves no jigc trace.
        match std::fs::remove_file(&target) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            other => other,
        }
    } else {
        std::fs::write(&target, next)
    }
}

/// Idempotently **remove** the profile's allowlist permits from the host project's
/// assistant settings file (`<repo_root>/<profile.allowlist.file>`) — the
/// structure-aware-JSON inverse of [`inject_allowlist`] for `jigc uninstall`
/// (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5), bullet
/// (b)).
///
/// Parses the settings object, drops each profile permit pattern from the
/// `permissions.allow` array, and writes back pretty JSON — leaving every unrelated
/// permit, the `permissions` object, and unrelated top-level keys intact, and the
/// file valid JSON. **Idempotent + non-destructive:** an absent file, an absent
/// `permissions`/`allow`, or an array that no longer carries the permit is a clean
/// no-op (nothing is written, so a second `uninstall` is a no-op).
pub fn remove_allowlist(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<()> {
    let target = repo_root.join(&profile.allowlist.file);

    // Absent settings file: nothing to remove.
    if !target.exists() {
        return Ok(());
    }
    let mut settings = read_settings(&target)?;

    // Navigate to `permissions.allow` if present; absent → a clean no-op.
    let Some(allow) = settings
        .as_object_mut()
        .and_then(|root| root.get_mut("permissions"))
        .and_then(|perms| perms.as_object_mut())
        .and_then(|perms| perms.get_mut("allow"))
        .and_then(|allow| allow.as_array_mut())
    else {
        return Ok(());
    };

    let before = allow.len();
    allow.retain(|v| {
        v.as_str()
            .is_none_or(|s| !profile.allowlist.permit.iter().any(|p| p == s))
    });
    // No permit was present: leave the file byte-untouched (idempotent no-op).
    if allow.len() == before {
        return Ok(());
    }

    write_settings(&target, &settings)
}

/// Idempotently install the profile's session-event **hook** into the host
/// project's assistant settings file (`<repo_root>/<profile.allowlist.file>`,
/// e.g. `.claude/settings.json`).
///
/// `jigc setup`'s primary bootstrap injection (`design/assistant-adapter.md` →
/// Inject the bootstrap: hook (primary); `DECISIONS.md` 2026-05-31 → pull the
/// SessionStart hook into the MVP adapter). The same **structure-aware** JSON
/// merge as the allowlist: ensure a `hooks.<event>` matcher exists whose inner
/// `hooks` array carries a `{ type: command, command: <run> }` entry. Idempotent
/// by **structural presence** — if any matcher under the event already runs the
/// command, it is a no-op, so a re-run is byte-identical; unrelated events,
/// matchers, and top-level keys are preserved. Shares the file with the allowlist
/// merge: each is an independent structural no-op on a re-run, so running both
/// leaves the file byte-stable. A no-op for a profile that declares no hook.
pub fn inject_hook(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<()> {
    let Some(hook) = profile.hook() else {
        return Ok(());
    };
    let target = repo_root.join(&profile.allowlist.file);

    let mut settings = read_settings(&target)?;

    // Navigate/create `hooks.<event>` (an array of matcher objects).
    let event_matchers = settings
        .as_object_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "settings root is not a JSON object",
            )
        })?
        .entry("hooks")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()))
        .as_object_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "`hooks` is not a JSON object",
            )
        })?
        .entry(hook.event.clone())
        .or_insert_with(|| serde_json::Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "`hooks.<event>` is not a JSON array",
            )
        })?;

    // Idempotent by structural presence: bail if any matcher already runs the
    // command.
    let already_present = event_matchers.iter().any(|matcher| {
        matcher
            .get("hooks")
            .and_then(|h| h.as_array())
            .is_some_and(|inner| {
                inner
                    .iter()
                    .any(|cmd| cmd.get("command").and_then(|c| c.as_str()) == Some(&hook.run))
            })
    });
    if !already_present {
        event_matchers.push(serde_json::json!({
            "hooks": [ { "type": "command", "command": hook.run } ]
        }));
    }

    write_settings(&target, &settings)
}

/// Read the assistant settings file as a JSON value, treating an absent file as
/// an empty object — the shared open step for the structure-aware settings merges
/// ([`inject_allowlist`], [`inject_hook`]).
fn read_settings(target: &Path) -> std::io::Result<serde_json::Value> {
    match std::fs::read_to_string(target) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(serde_json::Value::Object(serde_json::Map::new()))
        }
        Err(e) => Err(e),
        Ok(text) => serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}

/// Write `settings` back as pretty JSON with a single trailing newline, creating
/// the parent dir if needed — the shared write step for the settings merges.
fn write_settings(target: &Path, settings: &serde_json::Value) -> std::io::Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = serde_json::to_string_pretty(settings)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    out.push('\n');
    std::fs::write(target, out)
}

/// Initialize the in-repo **project cascade layer** so the project resolves as
/// *set up* (`design/assistant-adapter.md` → Generated, minimal, regenerated;
/// `DECISIONS.md` 2026-05-31 → adapter install reworked).
///
/// Creates `<repo_root>/.jigc/config/` (the project layer — orientation's
/// clean/unset discriminator keys on its presence, `crate::locate` →
/// `PROJECT_CONFIG_REL`) with a tracked `.gitkeep` so git keeps the otherwise
/// empty dir, and ensures `<repo_root>/.jigc/.gitignore` ignores the transient
/// subdirs (`tasks/`/`index/`/`state/`/`milestones/`) so `config/`, its `.gitkeep`,
/// and `AGENT.md` are committed while the working area is not. The entry set is the
/// **canonical** one `task.rs::ensure_jigc_gitignore` writes — including `milestones/`
/// — so that the gitignore `setup` commits is already final and a later `finalize`
/// never has to amend it (its `needs_write` keys on `milestones/` being present).
/// Idempotent: an existing `.gitignore` is left untouched, and the empty `.gitkeep`
/// is rewritten byte-identically.
pub fn init_project_layer(repo_root: &Path) -> std::io::Result<()> {
    let config_dir = repo_root.join(".jigc").join("config");
    std::fs::create_dir_all(&config_dir)?;
    std::fs::write(config_dir.join(".gitkeep"), b"")?;

    let gitignore = repo_root.join(".jigc").join(".gitignore");
    if !gitignore.exists() {
        std::fs::write(&gitignore, "tasks/\nindex/\nstate/\nmilestones/\n")?;
    }
    Ok(())
}

/// Whether `content` already carries the exact bare `@.jigc/AGENT.md` import
/// line — the idempotency check for [`inject_reference`]. Matches a line that is
/// exactly the import (after trimming surrounding whitespace), so a re-run that
/// finds it is a byte-for-byte no-op.
fn contains_import_line(content: &str) -> bool {
    content
        .lines()
        .any(|line| line.trim() == BOOTSTRAP_IMPORT_LINE)
}

/// The managed bootstrap file's repo-relative path (`.jigc/AGENT.md`) — the
/// `to` target the always-loaded file imports. Kept out of the cascade config
/// dir `.jigc/config/` so the cascade loader never trips on a markdown file
/// (`DECISIONS.md` 2026-05-31 → adapter install reworked).
const BOOTSTRAP_FILE_REL: &str = ".jigc/AGENT.md";

/// The bare `@`-import line injected into the always-loaded file — a Claude Code
/// import pointing at the managed bootstrap file. The whole floor in `CLAUDE.md`.
const BOOTSTRAP_IMPORT_LINE: &str = "@.jigc/AGENT.md";

/// The canonical routing sentence, verbatim from `design/bootstrap.md` → The
/// sentence. The backticks are content (they fence the command tokens, as in the
/// routing footer); the doc's surrounding bold is blockquote presentation, not
/// part of the sentence.
const BOOTSTRAP_SENTENCE: &str = "`jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.";

/// The env var that selects a directory adapter-profile source over the
/// binary-embedded default. The adapter analogue of [`crate::pack`]'s
/// `JIGC_PACK_DIR` (`DECISIONS.md` 2026-06-05 → the `JIGC_ADAPTERS_DIR` seam).
const ADAPTERS_DIR_ENV: &str = "JIGC_ADAPTERS_DIR";

/// Load the adapter profile for `assistant` (its file stem under `adapters/`).
///
/// The **single** production profile-load point — the adapter analogue of
/// `pack::make_pack`. With `JIGC_ADAPTERS_DIR` set to a directory, the profile
/// YAML is read live from `<dir>/<assistant>.yaml`; unset, the binary-embedded
/// profile is used, so output is byte-identical to a build without the seam.
/// Routing every production load through this one seam makes the obligation
/// "every load honors the env," not a per-site count (the `JIGC_PACK_DIR`
/// lesson). The seam exists so a deliberately-broken spawn template can be driven
/// through the real `jigc setup` and observed to reject.
///
/// An unknown assistant name yields [`ProfileError::NotFound`]; malformed bytes
/// yield a typed error, never a panic — the profile is fed in exactly like a
/// pack resource, off the engine's presentation-free surface.
pub fn load_profile(assistant: &str) -> Result<AdapterProfile, ProfileError> {
    load_profile_from(assistant, std::env::var_os(ADAPTERS_DIR_ENV))
}

/// The testable core of [`load_profile`]: select on an already-read env value
/// rather than reading the process environment, so the selection logic is
/// exercised without mutating global state (parallel-test-safe). A set, non-empty
/// directory reads `<dir>/<assistant>.yaml` from disk; otherwise the embedded
/// bytes are used.
fn load_profile_from(
    assistant: &str,
    adapters_dir: Option<OsString>,
) -> Result<AdapterProfile, ProfileError> {
    let file_name = format!("{assistant}.yaml");
    let bytes = match adapters_dir {
        Some(dir) if !dir.is_empty() => std::fs::read(PathBuf::from(dir).join(&file_name))
            .map_err(|source| ProfileError::Io {
                assistant: assistant.to_owned(),
                source,
            })?,
        _ => ADAPTERS
            .get_file(&file_name)
            .ok_or_else(|| ProfileError::NotFound(assistant.to_owned()))?
            .contents()
            .to_vec(),
    };
    let text = std::str::from_utf8(&bytes).map_err(|_| ProfileError::NotUtf8 {
        assistant: assistant.to_owned(),
    })?;
    serde_yaml_ng::from_str(text).map_err(|source| ProfileError::Malformed {
        assistant: assistant.to_owned(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped Claude Code spawn template **passes** `validate_spawn_template`:
    /// it carries both placeholders, the `jigc workflow … --task` invocation, sits
    /// on one line, and wraps exactly one backticked span in free prose with no
    /// forbidden marker. This is the canonical-pass anchor for the decidable rule.
    #[test]
    fn shipped_spawn_template_passes_validation() {
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        validate_spawn_template(&profile.spawn().expect("spawn target").template)
            .expect("the shipped template passes the decidable rule");
    }

    /// Each broken template **fails on the expected clause** — the full clause
    /// coverage. A precise typed reason (the violated clause) is the install
    /// error's pointer; the table asserts the *specific* clause, not merely that it
    /// failed, so a clause swap can't pass by failing on a sibling clause.
    #[test]
    fn broken_spawn_templates_fail_on_their_clause() {
        use SpawnTemplateReason::*;

        let cases: &[(&str, SpawnTemplateReason)] = &[
            // (1) missing {{task_id}}.
            (
                "Use your Task tool to run: `jigc workflow {{workflow}} --task X`",
                MissingTaskIdPlaceholder,
            ),
            // (2) missing {{workflow}}.
            (
                "Use your Task tool to run: `jigc workflow W --task {{task_id}}`",
                MissingWorkflowPlaceholder,
            ),
            // (3) no `jigc workflow … --task …` invocation (placeholders present so
            //     the earlier clauses pass; the invocation verb is absent).
            (
                "Run the task `{{workflow}} for {{task_id}}` somewhere",
                MissingInvocation,
            ),
            // (4) two newlines (one newline is allowed; two is over the ceiling).
            (
                "Use your Task tool to run:\n`jigc workflow {{workflow}} --task {{task_id}}`\n",
                TooManyNewlines,
            ),
            // (5) zero backtick spans.
            (
                "Use your Task tool to run: jigc workflow {{workflow}} --task {{task_id}}",
                NotExactlyOneBacktickSpan,
            ),
            // (6) two backtick spans.
            (
                "Use `your` Task tool: `jigc workflow {{workflow}} --task {{task_id}}`",
                NotExactlyOneBacktickSpan,
            ),
            // (7) a <<author: slot-author marker.
            (
                "<<author: note >> `jigc workflow {{workflow}} --task {{task_id}}`",
                ForbiddenMarker,
            ),
            // (8) an ATX heading at a line start.
            (
                "# Heading `jigc workflow {{workflow}} --task {{task_id}}`",
                ForbiddenMarker,
            ),
            // (9a) a Run: directive marker.
            (
                "Run: `jigc workflow {{workflow}} --task {{task_id}}`",
                ForbiddenMarker,
            ),
            // (9b) a Spawn: directive marker.
            (
                "Spawn: `jigc workflow {{workflow}} --task {{task_id}}`",
                ForbiddenMarker,
            ),
            // (9c) a blockquote `> ` directive marker.
            (
                "> `jigc workflow {{workflow}} --task {{task_id}}`",
                ForbiddenMarker,
            ),
        ];

        for (template, expected) in cases {
            let err = validate_spawn_template(template)
                .expect_err(&format!("template should fail: {template:?}"));
            assert_eq!(
                &err, expected,
                "template {template:?} failed on {err:?}, expected {expected:?}",
            );
        }
    }

    /// The M31 reopen (scoped relaxation, determinism rationale preserved): the
    /// backtick span may carry **only** an optional single `cd <one-token> &&` prefix
    /// before the `jigc workflow … --task …` invocation. The bare invocation still
    /// passes; the worktree-`cd` prefix passes; **any other shell** (e.g.
    /// `rm -rf x && jigc workflow …`) is rejected structurally — the load-bearing red
    /// (the old purely-lexical rule accepted it).
    #[test]
    fn spawn_template_permits_only_the_cd_worktree_prefix() {
        // (a) the worktree `cd` prefix PASSES — exactly one structured token.
        validate_spawn_template(
            "Use your Task tool to run: `cd {{worktree}} && jigc workflow {{workflow}} --task {{task_id}}`",
        )
        .expect("the `cd {{worktree}} &&` prefix is permitted");

        // (b) the bare invocation still PASSES (no prefix).
        validate_spawn_template(
            "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`",
        )
        .expect("the bare invocation still passes");

        // (c) any OTHER shell prefix FAILS — the structural rejection (red: the old
        //     lexical rule accepted `rm -rf x && jigc workflow …`).
        let err = validate_spawn_template(
            "Use your Task tool to run: `rm -rf x && jigc workflow {{workflow}} --task {{task_id}}`",
        )
        .expect_err("a non-`cd` shell prefix must be rejected");
        assert_eq!(
            err,
            SpawnTemplateReason::DisallowedShellPrefix,
            "the rejection must name the disallowed-shell-prefix clause",
        );

        // (d) a chained shell AFTER the invocation also FAILS (no trailing `&&`).
        validate_spawn_template(
            "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}} && rm -rf x`",
        )
        .expect_err("trailing shell chaining must be rejected");

        // (e) a multi-token `cd` argument FAILS (only one structured token permitted).
        validate_spawn_template(
            "Use your Task tool to run: `cd a b && jigc workflow {{workflow}} --task {{task_id}}`",
        )
        .expect_err("a multi-token `cd` argument must be rejected");
    }

    /// Golden over the embedded `claude-code.yaml` bytes — the canonical profile
    /// contract. The file *is* the source of truth (no serializer here), so the
    /// golden pins exactly the bytes that ship: the inject reference floor + the
    /// `SessionStart` hook + the allowlist + the **spawn** launch template, and
    /// nothing else (no `Resume` hook — that stays post-MVP).
    #[test]
    fn claude_code_profile_bytes_are_canonical() {
        let bytes = include_str!("../adapters/claude-code.yaml");
        insta::assert_snapshot!(bytes, @r###"
        assistant: claude-code
        inject:
          - reference: { file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }
          - hook: { event: SessionStart, run: "jigc start" }
        allowlist:
          file: .claude/settings.json
          permit: ["jigc *", "git add *"]
        spawn:
          template: "Use your Task tool to run: `cd {{worktree}} && jigc workflow {{workflow}} --task {{task_id}}`"
        "###);
    }

    /// The loader deserializes the shipped profile and exposes its surfaces: the
    /// inject **reference** target points `CLAUDE.md` at the managed
    /// `.jigc/AGENT.md` (the universal floor), the inject **hook** binds
    /// `SessionStart` to `jigc start` (the primary injection), the **allowlist**
    /// file is `.claude/settings.json` with the `jigc *` permit, and the **spawn**
    /// launch template is the shipped one-line Claude Code template.
    #[test]
    fn claude_code_profile_loads_with_inject_and_allowlist_targets() {
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        assert_eq!(profile.assistant, "claude-code");

        let reference = profile
            .reference()
            .expect("the profile declares a reference floor");
        assert_eq!(
            reference.file, "CLAUDE.md",
            "the inject reference floor targets CLAUDE.md",
        );
        assert_eq!(
            reference.to, ".jigc/AGENT.md",
            "the inject reference points at the managed `.jigc/AGENT.md`",
        );
        assert_eq!(
            reference.syntax, "at-import",
            "the inject reference uses the at-import syntax",
        );

        let hook = profile
            .hook()
            .expect("the profile declares a SessionStart hook");
        assert_eq!(
            hook.event, "SessionStart",
            "the hook binds the SessionStart event"
        );
        assert_eq!(hook.run, "jigc start", "the hook runs bare `jigc start`");

        assert_eq!(
            profile.allowlist.file, ".claude/settings.json",
            "the allowlist targets the Claude Code settings file",
        );
        assert_eq!(
            profile.allowlist.permit,
            vec!["jigc *".to_string(), "git add *".to_string()],
            "the allowlist permits the `jigc *` command pattern and `git add *` staging",
        );

        let spawn = profile
            .spawn()
            .expect("the profile declares a spawn launch template");
        assert_eq!(
            spawn.template,
            "Use your Task tool to run: `cd {{worktree}} && jigc workflow {{workflow}} --task {{task_id}}`",
            "the spawn template is the shipped one-line Claude Code launch template",
        );
    }

    /// [`render_spawn`] is a lexical three-token substitution: it replaces
    /// `{{worktree}}`, `{{workflow}}`, and `{{task_id}}` with the given values,
    /// leaving the rest of the template (the wrapping prose and the backticks)
    /// verbatim. The rendered launch line carries the backticked
    /// `cd .jigc/worktrees/<id> && jigc workflow … --task …` invocation with no
    /// residual placeholder tokens, and the rendered span passes the install rule.
    #[test]
    fn render_spawn_substitutes_all_tokens() {
        let template = "Use your Task tool to run: `cd {{worktree}} && jigc workflow {{workflow}} --task {{task_id}}`";
        let rendered = render_spawn(template, "sub-task", "alpha-fix");

        assert_eq!(
            rendered,
            "Use your Task tool to run: `cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`",
            "all three tokens are substituted, wrapping prose preserved",
        );
        assert!(
            rendered.contains(
                "`cd .jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix`"
            ),
            "the backticked worktree-`cd` invocation is present, got:\n{rendered}",
        );
        assert!(
            !rendered.contains("{{worktree}}")
                && !rendered.contains("{{workflow}}")
                && !rendered.contains("{{task_id}}"),
            "no residual placeholder tokens, got:\n{rendered}",
        );
    }

    /// Golden over [`bootstrap_file`]: the canonical routing sentence
    /// (`design/bootstrap.md` → The sentence, verbatim) as the managed file body,
    /// with a single trailing newline. This is the exact content `jigc setup`
    /// writes into `.jigc/AGENT.md` (rewritten whole each run — no markers).
    #[test]
    fn bootstrap_file_is_the_sentence_body() {
        insta::assert_snapshot!(bootstrap_file(), @r###"
        `jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.
        "###);
    }

    /// A throwaway directory that removes itself on drop — keeps the injection
    /// tests off the developer's repo (the project's hand-rolled temp-dir
    /// pattern; no `tempfile` dep).
    struct TempDir(std::path::PathBuf);

    impl TempDir {
        fn new() -> Self {
            let mut path = std::env::temp_dir();
            let unique = format!(
                "jigc-inject-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            );
            path.push(unique);
            std::fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// First inject into a project with **no** `CLAUDE.md` creates the file with
    /// the heading + the bare import line; a second inject is a no-op at the byte
    /// level (run twice ⇒ byte-identical). The created-file form is golden-locked:
    /// the `## Project interface` heading and the bare `@.jigc/AGENT.md` line, no
    /// marker comments.
    #[test]
    fn inject_creates_then_is_idempotent() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");

        inject_reference(dir.path()).expect("first inject");
        assert!(claude_md.exists(), "inject creates CLAUDE.md when absent");
        let after_first = std::fs::read_to_string(&claude_md).expect("read after first");

        inject_reference(dir.path()).expect("second inject");
        let after_second = std::fs::read_to_string(&claude_md).expect("read after second");

        assert_eq!(
            after_first, after_second,
            "inject is idempotent: a second run leaves the file byte-identical",
        );
        assert!(
            !after_first.contains("<!-- jigc:bootstrap"),
            "no marker comments in CLAUDE.md, got:\n{after_first}",
        );

        insta::assert_snapshot!(after_first, @r###"
        ## Project interface

        @.jigc/AGENT.md
        "###);
    }

    /// Inject into a `CLAUDE.md` that already has user prose but **no** reference
    /// appends the section at the end, preserving the existing content byte-for-
    /// byte, and re-running does not duplicate it.
    #[test]
    fn inject_appends_when_reference_absent() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        std::fs::write(&claude_md, "# My Project\n\nHuman rules.\n").expect("seed CLAUDE.md");

        inject_reference(dir.path()).expect("first inject");
        let after_first = std::fs::read_to_string(&claude_md).expect("read after first");
        assert!(
            after_first.starts_with("# My Project\n\nHuman rules.\n"),
            "existing content is preserved, got:\n{after_first}",
        );
        assert_eq!(
            after_first.matches(BOOTSTRAP_IMPORT_LINE).count(),
            1,
            "the reference is appended once, got:\n{after_first}",
        );

        inject_reference(dir.path()).expect("second inject");
        let after_second = std::fs::read_to_string(&claude_md).expect("read after second");
        assert_eq!(
            after_first, after_second,
            "appending is idempotent: run twice ⇒ byte-identical",
        );

        insta::assert_snapshot!(after_first, @r###"
        # My Project

        Human rules.

        ## Project interface

        @.jigc/AGENT.md
        "###);
    }

    /// A `CLAUDE.md` that already carries the exact bare `@.jigc/AGENT.md` line —
    /// even under a different heading the human wrote — is left **byte-identical**:
    /// the idempotency keys on the bare reference line, not on our heading.
    #[test]
    fn inject_leaves_existing_reference_untouched() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        let preexisting = "# My Project\n\n## Imports\n\n@.jigc/AGENT.md\n\nMore rules.\n";
        std::fs::write(&claude_md, preexisting).expect("seed CLAUDE.md");

        inject_reference(dir.path()).expect("inject over an existing reference");
        let after = std::fs::read_to_string(&claude_md).expect("read after inject");

        assert_eq!(
            after, preexisting,
            "a file already carrying the reference is left byte-identical, got:\n{after}",
        );
    }

    /// [`unwire_reference`] is the inverse of [`inject_reference`]: injecting then
    /// unwiring restores the human's pre-existing content **byte-for-byte**, and a
    /// second unwire (no section present) is a clean no-op.
    #[test]
    fn unwire_reference_round_trips_to_preexisting_bytes() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        let preexisting = "# My Project\n\nHuman rules.\n";
        std::fs::write(&claude_md, preexisting).expect("seed CLAUDE.md");

        inject_reference(dir.path()).expect("inject");
        assert!(
            std::fs::read_to_string(&claude_md)
                .unwrap()
                .contains(BOOTSTRAP_IMPORT_LINE),
            "inject must add the import line",
        );

        unwire_reference(dir.path()).expect("unwire");
        let after = std::fs::read_to_string(&claude_md).expect("read after unwire");
        assert_eq!(
            after, preexisting,
            "inject→unwire must restore the pre-existing bytes; got:\n{after}",
        );

        // Idempotent: a second unwire over a section-free file is a clean no-op.
        unwire_reference(dir.path()).expect("second unwire");
        assert_eq!(
            std::fs::read_to_string(&claude_md).expect("still present"),
            preexisting,
            "a second unwire leaves the restored file byte-identical",
        );
    }

    /// A pre-existing `CLAUDE.md` whose content has **no trailing newline** must
    /// also round-trip byte-for-byte: inject appends a fixed `"\n" + section`
    /// separator, and unwire strips it, restoring the human's exact bytes.
    #[test]
    fn unwire_reference_round_trips_without_a_trailing_newline() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        let preexisting = "# My Project\n\nHuman rules, no trailing newline.";
        std::fs::write(&claude_md, preexisting).expect("seed CLAUDE.md");

        inject_reference(dir.path()).expect("inject");
        assert!(
            std::fs::read_to_string(&claude_md)
                .unwrap()
                .contains(BOOTSTRAP_IMPORT_LINE),
            "inject must add the import line",
        );

        unwire_reference(dir.path()).expect("unwire");
        let after = std::fs::read_to_string(&claude_md).expect("read after unwire");
        assert_eq!(
            after, preexisting,
            "inject→unwire must restore the pre-existing bytes exactly, even with \
             no trailing newline; got:\n{after}",
        );
    }

    /// When `setup` **created** `CLAUDE.md` (the file is exactly the injected
    /// section), [`unwire_reference`] removes the file entirely — no orphan jigc
    /// trace. An absent file is then a clean no-op.
    #[test]
    fn unwire_reference_removes_a_setup_created_file() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");

        inject_reference(dir.path()).expect("inject creates the file");
        assert!(claude_md.exists(), "inject creates CLAUDE.md when absent");

        unwire_reference(dir.path()).expect("unwire");
        assert!(
            !claude_md.exists(),
            "unwiring a setup-created CLAUDE.md removes it entirely",
        );

        // Absent-file no-op.
        unwire_reference(dir.path()).expect("unwire over absent file is a no-op");
        assert!(!claude_md.exists(), "an absent CLAUDE.md stays absent");
    }

    /// [`unwire_reference`] leaves a `CLAUDE.md` that carries **no** jigc section
    /// byte-untouched — it strips only the recognized injected section, never a
    /// human's unrelated prose.
    #[test]
    fn unwire_reference_leaves_a_section_free_file_untouched() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        let human = "# My Project\n\nNo jigc here.\n";
        std::fs::write(&claude_md, human).expect("seed CLAUDE.md");

        unwire_reference(dir.path()).expect("unwire over a section-free file");
        assert_eq!(
            std::fs::read_to_string(&claude_md).expect("present"),
            human,
            "a file with no jigc section is left byte-identical",
        );
    }

    /// [`remove_allowlist`] drops the profile's permit from `permissions.allow`,
    /// leaving unrelated permits + unrelated top-level keys intact and the file valid
    /// JSON; a second remove (permit already gone) is a clean byte-identical no-op.
    #[test]
    fn remove_allowlist_drops_permit_preserving_unrelated_and_is_idempotent() {
        let dir = TempDir::new();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).expect("create .claude");
        let settings = claude_dir.join("settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        // Inject, then remove — the allowlist round-trip.
        inject_allowlist(dir.path(), &profile).expect("inject allowlist");
        // Seed an unrelated permit + key alongside (simulating a human's settings).
        let mut v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&settings).unwrap()).unwrap();
        v["model"] = serde_json::Value::String("claude-sonnet-4".to_string());
        v["permissions"]["allow"]
            .as_array_mut()
            .unwrap()
            .insert(0, serde_json::Value::String("git status".to_string()));
        std::fs::write(
            &settings,
            format!("{}\n", serde_json::to_string_pretty(&v).unwrap()),
        )
        .unwrap();

        remove_allowlist(dir.path(), &profile).expect("remove allowlist");
        let after = std::fs::read_to_string(&settings).expect("read after remove");
        let parsed: serde_json::Value =
            serde_json::from_str(&after).expect("settings stays valid JSON");
        let allow = parsed["permissions"]["allow"].as_array().unwrap();
        assert!(
            !allow.iter().any(|x| x == "jigc *"),
            "the `jigc *` permit must be dropped; got:\n{after}",
        );
        assert!(
            allow.iter().any(|x| x == "git status"),
            "the unrelated permit must survive; got:\n{after}",
        );
        assert_eq!(
            parsed["model"], "claude-sonnet-4",
            "the unrelated top-level key must survive; got:\n{after}",
        );

        // Idempotent: a second remove (permit already gone) is byte-identical.
        remove_allowlist(dir.path(), &profile).expect("second remove");
        assert_eq!(
            std::fs::read_to_string(&settings).expect("present"),
            after,
            "a second remove over a permit-free allow list is byte-identical",
        );
    }

    /// [`remove_allowlist`] over an **absent** settings file is a clean no-op (no
    /// file is created) — the idempotent teardown over a project that never had one.
    #[test]
    fn remove_allowlist_over_absent_settings_is_a_no_op() {
        let dir = TempDir::new();
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        remove_allowlist(dir.path(), &profile).expect("remove over absent settings");
        assert!(
            !dir.path().join(".claude/settings.json").exists(),
            "remove over an absent settings file must not create one",
        );
    }

    /// [`write_bootstrap_file`] writes the managed `.jigc/AGENT.md` with the
    /// sentence body, creating `.jigc/` when absent, and a re-run is byte-identical
    /// (the file is rewritten whole each time).
    #[test]
    fn bootstrap_file_written_then_idempotent() {
        let dir = TempDir::new();
        let agent_md = dir.path().join(".jigc/AGENT.md");

        write_bootstrap_file(dir.path()).expect("first write");
        assert!(
            agent_md.exists(),
            "writes `.jigc/AGENT.md`, creating `.jigc/`"
        );
        let after_first = std::fs::read_to_string(&agent_md).expect("read after first");
        assert_eq!(after_first, bootstrap_file());

        write_bootstrap_file(dir.path()).expect("second write");
        let after_second = std::fs::read_to_string(&agent_md).expect("read after second");
        assert_eq!(
            after_first, after_second,
            "rewriting whole is byte-identical on a re-run",
        );
    }

    /// First allowlist merge into a project with **no** `.claude/settings.json`
    /// creates the file (and the `.claude/` dir) with a `permissions.allow` array
    /// holding exactly the profile's permit pattern; a second merge is a no-op at
    /// the byte level (run twice ⇒ byte-identical). The created form is
    /// golden-locked: pretty JSON, the `jigc *` permit present once.
    #[test]
    fn allowlist_added_then_idempotent() {
        let dir = TempDir::new();
        let settings = dir.path().join(".claude/settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        inject_allowlist(dir.path(), &profile).expect("first allowlist merge");
        assert!(
            settings.exists(),
            "merge creates .claude/settings.json when absent",
        );
        let after_first = std::fs::read_to_string(&settings).expect("read after first");

        inject_allowlist(dir.path(), &profile).expect("second allowlist merge");
        let after_second = std::fs::read_to_string(&settings).expect("read after second");

        assert_eq!(
            after_first, after_second,
            "allowlist merge is idempotent: a second run leaves the file byte-identical",
        );
        assert_eq!(
            after_first.matches("\"jigc *\"").count(),
            1,
            "the permit appears exactly once — no duplicate, got:\n{after_first}",
        );

        insta::assert_snapshot!(after_first, @r#"
        {
          "permissions": {
            "allow": [
              "jigc *",
              "git add *"
            ]
          }
        }
        "#);
    }

    /// Merging into a `.claude/settings.json` that already holds an unrelated
    /// top-level key **and** an unrelated permission preserves both: the `jigc *`
    /// permit is added once to the existing allow list, the unrelated permission
    /// stays, and the unrelated top-level key is untouched. A re-run is then
    /// idempotent.
    #[test]
    fn allowlist_preserves_existing_settings() {
        let dir = TempDir::new();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).expect("create .claude");
        let settings = claude_dir.join("settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        // A settings file with an unrelated top-level key and an unrelated
        // already-present permission.
        let preexisting = r#"{
  "enabledPlugins": {
    "rust-analyzer-lsp@claude-plugins-official": true
  },
  "permissions": {
    "allow": [
      "Bash(ls:*)"
    ]
  }
}
"#;
        std::fs::write(&settings, preexisting).expect("seed settings.json");

        inject_allowlist(dir.path(), &profile).expect("merge into existing settings");
        let after = std::fs::read_to_string(&settings).expect("read after merge");

        assert!(
            after.contains("\"jigc *\""),
            "the jigc permit is added, got:\n{after}",
        );
        assert!(
            after.contains("\"Bash(ls:*)\""),
            "the unrelated permission is preserved, got:\n{after}",
        );
        assert!(
            after.contains("rust-analyzer-lsp@claude-plugins-official"),
            "the unrelated top-level key is preserved, got:\n{after}",
        );
        assert_eq!(
            after.matches("\"jigc *\"").count(),
            1,
            "the permit is added exactly once, got:\n{after}",
        );

        // Idempotent: a second merge over the now-current file is a no-op.
        inject_allowlist(dir.path(), &profile).expect("second merge");
        let after_second = std::fs::read_to_string(&settings).expect("read after second");
        assert_eq!(after, after_second, "re-merge is byte-identical");

        insta::assert_snapshot!(after, @r#"
        {
          "enabledPlugins": {
            "rust-analyzer-lsp@claude-plugins-official": true
          },
          "permissions": {
            "allow": [
              "Bash(ls:*)",
              "jigc *",
              "git add *"
            ]
          }
        }
        "#);
    }

    /// First hook install into a project with **no** `.claude/settings.json`
    /// creates the file with a `hooks.SessionStart` matcher running `jigc start`;
    /// a second install is a no-op at the byte level (run twice ⇒ byte-identical).
    /// The created form is golden-locked: the Claude Code SessionStart shape.
    #[test]
    fn hook_added_then_idempotent() {
        let dir = TempDir::new();
        let settings = dir.path().join(".claude/settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        inject_hook(dir.path(), &profile).expect("first hook install");
        assert!(
            settings.exists(),
            "hook install creates .claude/settings.json when absent",
        );
        let after_first = std::fs::read_to_string(&settings).expect("read after first");

        inject_hook(dir.path(), &profile).expect("second hook install");
        let after_second = std::fs::read_to_string(&settings).expect("read after second");

        assert_eq!(
            after_first, after_second,
            "hook install is idempotent: a second run leaves the file byte-identical",
        );
        assert_eq!(
            after_first.matches("\"jigc start\"").count(),
            1,
            "the hook command appears exactly once — no duplicate, got:\n{after_first}",
        );

        insta::assert_snapshot!(after_first, @r###"
        {
          "hooks": {
            "SessionStart": [
              {
                "hooks": [
                  {
                    "command": "jigc start",
                    "type": "command"
                  }
                ]
              }
            ]
          }
        }
        "###);
    }

    /// Installing the hook into a `.claude/settings.json` that already holds the
    /// allowlist (the real `setup` ordering: allowlist first, then hook) adds the
    /// hook **alongside** the permissions without clobbering them, and a re-run is
    /// byte-identical.
    #[test]
    fn hook_install_preserves_allowlist() {
        let dir = TempDir::new();
        let settings = dir.path().join(".claude/settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        inject_allowlist(dir.path(), &profile).expect("allowlist first");
        inject_hook(dir.path(), &profile).expect("hook second");
        let after = std::fs::read_to_string(&settings).expect("read after both");

        assert!(
            after.contains("\"jigc *\""),
            "the allowlist permit is preserved, got:\n{after}",
        );
        assert!(
            after.contains("\"SessionStart\""),
            "the hook event is present, got:\n{after}",
        );

        // Idempotent across both merges: re-running both is byte-identical.
        inject_allowlist(dir.path(), &profile).expect("re-allowlist");
        inject_hook(dir.path(), &profile).expect("re-hook");
        let after_second = std::fs::read_to_string(&settings).expect("read after re-run");
        assert_eq!(
            after, after_second,
            "re-running both merges is byte-identical"
        );

        insta::assert_snapshot!(after, @r#"
        {
          "hooks": {
            "SessionStart": [
              {
                "hooks": [
                  {
                    "command": "jigc start",
                    "type": "command"
                  }
                ]
              }
            ]
          },
          "permissions": {
            "allow": [
              "jigc *",
              "git add *"
            ]
          }
        }
        "#);
    }

    /// An unknown assistant name is a clear not-found error, never a panic.
    #[test]
    fn unknown_assistant_yields_not_found() {
        let err = load_profile("nonexistent").expect_err("no profile ships for `nonexistent`");
        assert!(
            matches!(&err, ProfileError::NotFound(name) if name == "nonexistent"),
            "expected NotFound(\"nonexistent\"), got {err:?}",
        );
    }

    /// The selection core of [`load_profile`], exercised without mutating the
    /// process environment (parallel-test-safe; the `make_pack_from` idiom). With
    /// the env value unset, the profile loads from the embedded bytes — equal to
    /// what `load_profile` reads with no override, so a no-env build is inert.
    #[test]
    fn load_profile_from_unset_reads_the_embedded_profile() {
        let embedded = load_profile_from("claude-code", None).expect("embedded profile loads");
        assert_eq!(embedded.assistant, "claude-code");
        validate_spawn_template(&embedded.spawn().expect("spawn target").template)
            .expect("the embedded template passes the rule");
    }

    /// An empty `JIGC_ADAPTERS_DIR=` falls through to the embedded default rather
    /// than reading an empty path (an unset-equivalent value is inert).
    #[test]
    fn load_profile_from_empty_env_reads_the_embedded_profile() {
        let profile =
            load_profile_from("claude-code", Some(OsString::new())).expect("embedded loads");
        assert_eq!(profile.assistant, "claude-code");
    }

    /// With a directory value set, the profile is read live from
    /// `<dir>/<assistant>.yaml` — a profile seeded only on disk loads back, proving
    /// an alternate profile drives the loader (the seam the install-time reject
    /// proof rides on).
    #[test]
    fn load_profile_from_dir_reads_the_on_disk_profile() {
        let dir = TempDir::new();
        std::fs::write(
            dir.path().join("claude-code.yaml"),
            b"assistant: claude-code\n\
              inject:\n\
              \x20 - reference: { file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }\n\
              allowlist:\n\
              \x20 file: .claude/settings.json\n\
              \x20 permit: [\"jigc *\"]\n\
              spawn:\n\
              \x20 template: \"on-disk only `jigc workflow {{workflow}} --task {{task_id}}`\"\n",
        )
        .expect("seed on-disk profile");

        let profile = load_profile_from("claude-code", Some(OsString::from(dir.path())))
            .expect("the on-disk profile loads via the seam");
        assert_eq!(
            profile.spawn().expect("spawn target").template,
            "on-disk only `jigc workflow {{workflow}} --task {{task_id}}`",
            "the set-env loader must read the on-disk profile, not the embedded one",
        );
    }

    /// A set directory that lacks the requested profile file is a typed IO error
    /// (only reachable on the override path), never a panic.
    #[test]
    fn load_profile_from_dir_missing_file_is_io_error() {
        let dir = TempDir::new();
        let err = load_profile_from("claude-code", Some(OsString::from(dir.path())))
            .expect_err("a missing on-disk profile errors");
        assert!(
            matches!(&err, ProfileError::Io { assistant, .. } if assistant == "claude-code"),
            "expected Io {{ assistant: \"claude-code\", .. }}, got {err:?}",
        );
    }
}
