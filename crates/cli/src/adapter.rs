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
use std::fmt;
use std::path::Path;

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

/// Render the spawn launch line: a lexical two-token substitution of the
/// `{{workflow}}` and `{{task_id}}` placeholders in `template` with `workflow`
/// and `task_id`, leaving every other byte verbatim.
///
/// Plain string replacement, **not** the engine `{{…}}` data-value grammar — the
/// spawn template's tokens are launch-line placeholders the adapter fills, never
/// data-value refs (`design/assistant-adapter.md` → Bind the spawn mechanism). So
/// it does not route through `data_value`/`compose`. Consumed by the launch path
/// (this increment's later tasks).
#[allow(dead_code)]
pub fn render_spawn(template: &str, workflow: &str, task_id: &str) -> String {
    template
        .replace("{{workflow}}", workflow)
        .replace("{{task_id}}", task_id)
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
        }
    }
}

impl std::error::Error for ProfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProfileError::Malformed { source, .. } => Some(source),
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
            // Append the section, separated from existing content by a blank
            // line. An empty file degenerates to just the section.
            if existing.is_empty() {
                section
            } else {
                let mut out = String::with_capacity(existing.len() + section.len() + 2);
                out.push_str(&existing);
                if !out.ends_with('\n') {
                    out.push('\n');
                }
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
/// subdirs (`tasks/`/`index/`/`state/`) so `config/`, its `.gitkeep`, and
/// `AGENT.md` are committed while the working area is not (mirrors
/// `task.rs::ensure_jigc_gitignore`). Idempotent: an existing `.gitignore` is
/// left untouched, and the empty `.gitkeep` is rewritten byte-identically.
pub fn init_project_layer(repo_root: &Path) -> std::io::Result<()> {
    let config_dir = repo_root.join(".jigc").join("config");
    std::fs::create_dir_all(&config_dir)?;
    std::fs::write(config_dir.join(".gitkeep"), b"")?;

    let gitignore = repo_root.join(".jigc").join(".gitignore");
    if !gitignore.exists() {
        std::fs::write(&gitignore, "tasks/\nindex/\nstate/\n")?;
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

/// Load the embedded adapter profile for `assistant` (its file stem under
/// `adapters/`).
///
/// An unknown assistant name yields [`ProfileError::NotFound`]; malformed bytes
/// yield a typed error, never a panic — the profile is fed in exactly like a
/// pack resource, off the engine's presentation-free surface.
pub fn load_profile(assistant: &str) -> Result<AdapterProfile, ProfileError> {
    let file_name = format!("{assistant}.yaml");
    let file = ADAPTERS
        .get_file(&file_name)
        .ok_or_else(|| ProfileError::NotFound(assistant.to_owned()))?;
    let text = std::str::from_utf8(file.contents()).map_err(|_| ProfileError::NotUtf8 {
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
          permit: ["jigc *"]
        spawn:
          template: "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`"
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
            vec!["jigc *".to_string()],
            "the allowlist permits the `jigc *` command pattern",
        );

        let spawn = profile
            .spawn()
            .expect("the profile declares a spawn launch template");
        assert_eq!(
            spawn.template,
            "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`",
            "the spawn template is the shipped one-line Claude Code launch template",
        );
    }

    /// [`render_spawn`] is a lexical two-token substitution: it replaces both
    /// `{{workflow}}` and `{{task_id}}` with the given values, leaving the rest of
    /// the template (the wrapping prose and the backticks) verbatim. The rendered
    /// launch line carries the backticked `jigc workflow … --task …` invocation
    /// with no residual placeholder tokens.
    #[test]
    fn render_spawn_substitutes_both_tokens() {
        let template = "Use your Task tool to run: `jigc workflow {{workflow}} --task {{task_id}}`";
        let rendered = render_spawn(template, "sub-task", "alpha-fix");

        assert_eq!(
            rendered, "Use your Task tool to run: `jigc workflow sub-task --task alpha-fix`",
            "both tokens are substituted, wrapping prose preserved",
        );
        assert!(
            rendered.contains("`jigc workflow sub-task --task alpha-fix`"),
            "the backticked invocation is present, got:\n{rendered}",
        );
        assert!(
            !rendered.contains("{{workflow}}") && !rendered.contains("{{task_id}}"),
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

        insta::assert_snapshot!(after_first, @r###"
        {
          "permissions": {
            "allow": [
              "jigc *"
            ]
          }
        }
        "###);
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

        insta::assert_snapshot!(after, @r###"
        {
          "enabledPlugins": {
            "rust-analyzer-lsp@claude-plugins-official": true
          },
          "permissions": {
            "allow": [
              "Bash(ls:*)",
              "jigc *"
            ]
          }
        }
        "###);
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

        insta::assert_snapshot!(after, @r###"
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
              "jigc *"
            ]
          }
        }
        "###);
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
}
