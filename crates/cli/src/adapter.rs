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
use std::path::Path;

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

/// The marker-fenced bootstrap block injected into the always-loaded file
/// (`CLAUDE.md`).
///
/// A pure function of the embedded profile contract — no filesystem. Returns the
/// canonical routing sentence (`design/bootstrap.md` → The sentence, verbatim)
/// wrapped in stable HTML-comment idempotency markers. `jigc setup` writes this
/// block into the always-loaded file; on a re-run the markers let it locate and
/// replace its own block instead of appending a duplicate.
pub fn bootstrap_block() -> String {
    format!("{BOOTSTRAP_START_MARKER}\n{BOOTSTRAP_SENTENCE}\n{BOOTSTRAP_END_MARKER}\n")
}

/// Idempotently write the marker-fenced bootstrap block ([`bootstrap_block`])
/// into the host project's always-loaded file (`<repo_root>/CLAUDE.md`).
///
/// `jigc setup`'s static-line floor ([`assistant-adapter.md`] → Inject the
/// bootstrap; [`bootstrap.md`] → Placement is the adapter). Idempotent by the
/// marker fence: a missing file is created with just the block; a file without
/// the markers gets the block **appended** (surrounding content untouched); a
/// file already carrying the markers has only the fenced region **replaced**,
/// leaving every byte outside the markers identical. Run twice ⇒ byte-identical
/// file. Regenerated on upgrade, so the integration can't rot.
pub fn inject_line(repo_root: &Path) -> std::io::Result<()> {
    let target = repo_root.join("CLAUDE.md");
    let block = bootstrap_block();

    let next = match std::fs::read_to_string(&target) {
        // No file yet: it is exactly the block.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => block,
        Err(e) => return Err(e),
        Ok(existing) => match marker_span(&existing) {
            // Already has our fence: replace only the fenced region, leaving
            // every byte outside it identical. `block` ends in one `\n`; the
            // byte after the end marker (a `\n` or EOF) lives in `suffix`, so
            // splice the block *without* its trailing newline to avoid doubling.
            Some((start, end)) => {
                let mut out = String::with_capacity(existing.len() + block.len());
                out.push_str(&existing[..start]);
                out.push_str(block.trim_end_matches('\n'));
                out.push_str(&existing[end..]);
                out
            }
            // No fence: append the block, separated from existing content by a
            // blank line. An empty file degenerates to just the block.
            None => {
                if existing.is_empty() {
                    block
                } else {
                    let mut out = String::with_capacity(existing.len() + block.len() + 2);
                    out.push_str(&existing);
                    if !out.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push('\n');
                    out.push_str(&block);
                    out
                }
            }
        },
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

    let mut settings: serde_json::Value = match std::fs::read_to_string(&target) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            serde_json::Value::Object(serde_json::Map::new())
        }
        Err(e) => return Err(e),
        Ok(text) => serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
    };

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

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = serde_json::to_string_pretty(&settings)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    out.push('\n');
    std::fs::write(&target, out)
}

/// Locate the byte span of the marker-fenced bootstrap block in `content`, from
/// the start of the start marker to the end of the end marker (exclusive of any
/// following newline). Returns `None` when either marker is absent. The first
/// occurrence of each marker is used; on a re-run we wrote exactly one fence, so
/// a conformant file has exactly one.
fn marker_span(content: &str) -> Option<(usize, usize)> {
    let start = content.find(BOOTSTRAP_START_MARKER)?;
    let end_marker = content[start..].find(BOOTSTRAP_END_MARKER)?;
    let end = start + end_marker + BOOTSTRAP_END_MARKER.len();
    Some((start, end))
}

/// Opening idempotency marker for the bootstrap block (decided 2026-05-31). An
/// HTML comment so it is invisible in rendered markdown; the `jigc:` namespace
/// keeps it unambiguous against any other tool's markers.
const BOOTSTRAP_START_MARKER: &str = "<!-- jigc:bootstrap:start -->";

/// Closing idempotency marker for the bootstrap block. `jigc setup` replaces the
/// span between the start and end markers, making re-injection idempotent.
const BOOTSTRAP_END_MARKER: &str = "<!-- jigc:bootstrap:end -->";

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

    /// Golden over [`bootstrap_block`]: the canonical routing sentence
    /// (`design/bootstrap.md` → The sentence, verbatim) fenced by the pinned
    /// idempotency markers. Byte-exact: start marker, blank line, the sentence,
    /// blank line, end marker, trailing newline. This is the exact text
    /// `jigc setup` injects into `CLAUDE.md`; the markers let a re-run find and
    /// replace its own block rather than appending a duplicate.
    #[test]
    fn bootstrap_block_is_the_marker_fenced_sentence() {
        insta::assert_snapshot!(bootstrap_block(), @r###"
        <!-- jigc:bootstrap:start -->
        `jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.
        <!-- jigc:bootstrap:end -->
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
    /// just the block; a second inject is a no-op at the byte level (run twice ⇒
    /// byte-identical). The created-file form is golden-locked: exactly the
    /// fenced block, nothing else.
    #[test]
    fn inject_creates_then_is_idempotent() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");

        inject_line(dir.path()).expect("first inject");
        assert!(claude_md.exists(), "inject creates CLAUDE.md when absent");
        let after_first = std::fs::read_to_string(&claude_md).expect("read after first");

        inject_line(dir.path()).expect("second inject");
        let after_second = std::fs::read_to_string(&claude_md).expect("read after second");

        assert_eq!(
            after_first, after_second,
            "inject is idempotent: a second run leaves the file byte-identical",
        );

        insta::assert_snapshot!(after_first, @r###"
        <!-- jigc:bootstrap:start -->
        `jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.
        <!-- jigc:bootstrap:end -->
        "###);
    }

    /// Inject into a `CLAUDE.md` that already has user prose **and** a stale
    /// block replaces only the fenced region: the user content above and below
    /// the markers stays byte-identical, only the block's interior is refreshed.
    /// A re-run is then idempotent.
    #[test]
    fn inject_replaces_only_between_markers() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");

        // A file with user prose around a *stale* block (a marker fence whose
        // interior differs from the current bootstrap sentence).
        let preexisting = "\
# My Project

Some project rules a human wrote.

<!-- jigc:bootstrap:start -->
STALE sentence from an older jigc version.
<!-- jigc:bootstrap:end -->

More rules below, also human-authored.
";
        std::fs::write(&claude_md, preexisting).expect("seed CLAUDE.md");

        inject_line(dir.path()).expect("inject over a stale block");
        let after = std::fs::read_to_string(&claude_md).expect("read after inject");

        assert!(
            after.starts_with("# My Project\n\nSome project rules a human wrote.\n"),
            "user prose above the block is preserved byte-for-byte, got:\n{after}",
        );
        assert!(
            after.ends_with("More rules below, also human-authored.\n"),
            "user prose below the block is preserved byte-for-byte, got:\n{after}",
        );
        assert!(
            !after.contains("STALE sentence"),
            "the stale block interior is replaced, got:\n{after}",
        );
        assert_eq!(
            after.matches(BOOTSTRAP_START_MARKER).count(),
            1,
            "exactly one block — no duplicate appended, got:\n{after}",
        );
        assert!(
            after.contains("never read or edit managed docs directly"),
            "the current bootstrap sentence is present, got:\n{after}",
        );

        // Idempotent: a second inject over the now-current block is a no-op.
        inject_line(dir.path()).expect("second inject");
        let after_second = std::fs::read_to_string(&claude_md).expect("read after second");
        assert_eq!(after, after_second, "re-inject is byte-identical");

        insta::assert_snapshot!(after, @r###"
        # My Project

        Some project rules a human wrote.

        <!-- jigc:bootstrap:start -->
        `jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.
        <!-- jigc:bootstrap:end -->

        More rules below, also human-authored.
        "###);
    }

    /// Append into a `CLAUDE.md` that has user prose but **no** block adds the
    /// fenced block at the end, preserving the existing content, and re-running
    /// does not duplicate it.
    #[test]
    fn inject_appends_when_no_marker_present() {
        let dir = TempDir::new();
        let claude_md = dir.path().join("CLAUDE.md");
        std::fs::write(&claude_md, "# My Project\n\nHuman rules.\n").expect("seed CLAUDE.md");

        inject_line(dir.path()).expect("first inject");
        let after_first = std::fs::read_to_string(&claude_md).expect("read after first");
        assert!(
            after_first.starts_with("# My Project\n\nHuman rules.\n"),
            "existing content is preserved, got:\n{after_first}",
        );
        assert_eq!(
            after_first.matches(BOOTSTRAP_START_MARKER).count(),
            1,
            "the block is appended once, got:\n{after_first}",
        );

        inject_line(dir.path()).expect("second inject");
        let after_second = std::fs::read_to_string(&claude_md).expect("read after second");
        assert_eq!(
            after_first, after_second,
            "appending is idempotent: run twice ⇒ byte-identical",
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
