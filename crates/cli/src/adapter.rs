//! Assistant-adapter generation from embedded profiles + the engine catalog.
//!
//! MVP scope: the static bootstrap line + the `jigc` allowlist (hooks and spawn
//! binding are post-MVP). See `design/assistant-adapter.md`.
//!
//! Adapter **profiles** are embedded config-family data, the same pattern as the
//! pack (`implementation/module-layout.md` → Adapter (in `cli`): "Profiles are
//! embedded data … same pattern as the pack"). Each profile is the
//! assistant-neutral→assistant shim: the source of *where/how* to inject the
//! bootstrap and *how* to allowlist `jigc`. This module is the **typed model +
//! loader** only — it generates no host files yet (`jigc setup` arrives in a
//! later increment). The MVP model carries exactly the two in-scope surfaces:
//! the inject **line** target (the universal floor) and the **allowlist** target
//! (the path-of-least-resistance the bootstrap depends on); the **hook** and
//! **spawn** profile fields are post-MVP and intentionally absent from both the
//! model and the shipped profile bytes, so what we test is what ships.

use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::fmt;

/// The built-in adapter profiles, embedded at compile time from
/// `crates/cli/adapters/`. Mirrors `pack::PACK` (`include_dir`, decided
/// 2026-05-31): always-embedded, so what you test is what ships.
static ADAPTERS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/adapters");

/// A parsed adapter profile: the assistant-specific shim telling the CLI *where
/// and how* to wire into one coding assistant.
///
/// Deserialized from the config-family YAML at `adapters/<assistant>.yaml`. The
/// MVP surface is the inject **line** floor + the **allowlist**; hooks and the
/// spawn launch template are post-MVP and absent here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterProfile {
    /// The assistant this profile wires into (e.g. `claude-code`). Matches the
    /// file stem the loader resolves by.
    pub assistant: String,

    /// The bootstrap injection targets. MVP ships exactly one: the static
    /// **line** floor in the always-loaded file.
    pub inject: Vec<InjectTarget>,

    /// Where and what to allowlist so `jigc` runs without friction.
    pub allowlist: Allowlist,
}

/// One bootstrap injection target. MVP carries only the `line` variant (the
/// universal floor); the `hook` / `resume` variants are post-MVP.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InjectTarget {
    /// The static-line floor: inject the bootstrap line into this file.
    pub line: LineTarget,
}

/// The static-line floor target: which always-loaded file the bootstrap line
/// lands in, and at what scope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LineTarget {
    /// The always-loaded file to inject into (e.g. `CLAUDE.md`).
    pub file: String,

    /// The scope the file is resolved at (e.g. `project-root`).
    pub scope: String,
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
    /// golden pins exactly the MVP bytes that ship: the inject line floor + the
    /// allowlist, and nothing else (no hook/spawn — those are post-MVP).
    #[test]
    fn claude_code_profile_bytes_are_canonical() {
        let bytes = include_str!("../adapters/claude-code.yaml");
        insta::assert_snapshot!(bytes, @r###"
        assistant: claude-code
        inject:
          - line: { file: CLAUDE.md, scope: project-root }
        allowlist:
          file: .claude/settings.json
          permit: ["jigc *"]
        "###);
    }

    /// The loader deserializes the shipped profile and exposes the two MVP
    /// surfaces: the inject **line** target is `CLAUDE.md` (the universal floor),
    /// the **allowlist** file is `.claude/settings.json`, and the permit pattern
    /// is `jigc *`.
    #[test]
    fn claude_code_profile_loads_with_inject_and_allowlist_targets() {
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        assert_eq!(profile.assistant, "claude-code");

        assert_eq!(
            profile.inject.len(),
            1,
            "MVP ships exactly the static-line floor; got {:?}",
            profile.inject,
        );
        assert_eq!(
            profile.inject[0].line.file, "CLAUDE.md",
            "the inject line floor targets CLAUDE.md",
        );

        assert_eq!(
            profile.allowlist.file, ".claude/settings.json",
            "the allowlist targets the Claude Code settings file",
        );
        assert_eq!(
            profile.allowlist.permit,
            vec!["jigc *".to_string()],
            "the allowlist permits the `jigc *` command pattern",
        );
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
