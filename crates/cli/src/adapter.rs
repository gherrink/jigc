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

    /// The **guide** target — where this assistant wants jigc's own shipped guides,
    /// and the assistant-specific header they carry (`design/assistant-adapter.md`
    /// → The adapter's owned artifacts). **Optional**: an assistant with no place to
    /// put a guide declares none and the install is inert there, never an error.
    #[serde(default)]
    pub guide: Option<GuideTarget>,
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

    /// The profile's **guide** target, if it declares one — the adapter-owned artifact
    /// `setup` writes, stamps, and names in its install commit. `None` is the ordinary
    /// omitting context, not a degraded profile.
    pub fn guide(&self) -> Option<&GuideTarget> {
        self.guide.as_ref()
    }
}

/// The guide target: where the assistant wants jigc's shipped guides, and the header
/// that file has to open with for the assistant to recognize it.
///
/// The **path is assistant knowledge** (`.claude/skills/jigc/SKILL.md` is Claude Code's
/// skill convention, not jigc's), and so is the header — which is why both live in the
/// profile rather than in `setup`'s neutral core: the core copies what the profile
/// declares and appends only its own stamp keys ([`crate::setup::guide_artifact`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTarget {
    /// The repo-relative path the artifact is written to (e.g.
    /// `.claude/skills/jigc/SKILL.md`).
    pub file: String,

    /// The assistant's own front-matter keys, emitted verbatim above jigc's stamp lines.
    /// A `BTreeMap` so the rendered header is deterministic (same profile in → same bytes
    /// out, which is what makes the artifact's hash stable across runs). `#[serde(default)]`
    /// so a profile whose assistant wants no header stays valid.
    #[serde(default, rename = "front-matter")]
    pub front_matter: std::collections::BTreeMap<String, String>,
}

/// Why a [`GuideTarget`] is rejected at install — a precise typed clause, mirroring
/// [`SpawnTemplateReason`]: the violated clause *is* the install error's route, so the
/// profile author is told which rule to fix rather than that "something is wrong".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideTargetReason {
    /// The declared path is empty.
    EmptyPath,
    /// The declared path is absolute — the artifact is written under the repo root, so
    /// an absolute path would escape the repo the install is committing from.
    AbsolutePath,
    /// The declared path climbs out of the repo (a `..` component).
    EscapingPath,
    /// A front-matter key collides with one of jigc's own stamp keys — the profile would
    /// be forging the version/hash the ownership check reads.
    ReservedKey,
    /// A front-matter key is empty or carries a character outside `[A-Za-z0-9_-]`, so it
    /// could not be emitted as a plain YAML key.
    MalformedKey,
}

impl fmt::Display for GuideTargetReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clause = match self {
            GuideTargetReason::EmptyPath => "the guide target's `file` must name a path",
            GuideTargetReason::AbsolutePath => {
                "the guide target's `file` must be repo-relative, not absolute"
            }
            GuideTargetReason::EscapingPath => {
                "the guide target's `file` must stay inside the repo (no `..` component)"
            }
            GuideTargetReason::ReservedKey => {
                "the guide target's `front-matter` must not declare a `jigc-` stamp key \
                 (jigc appends `jigc-version:` and `jigc-body-blake3:` itself)"
            }
            GuideTargetReason::MalformedKey => {
                "each `front-matter` key must be a non-empty run of `[A-Za-z0-9_-]`"
            }
        };
        f.write_str(clause)
    }
}

/// The stamp keys jigc appends to every guide artifact's front matter — reserved against
/// the profile, so the ownership check reads jigc's own record and never a profile's.
pub const GUIDE_RESERVED_KEYS: [&str; 2] = ["jigc-version", "jigc-body-blake3"];

/// Gate a [`GuideTarget`] against the decidable install-time rules, mirroring
/// [`validate_spawn_template`]: purely lexical, no judgement, run **before any write** so
/// a broken profile touches nothing on disk.
pub fn validate_guide_target(guide: &GuideTarget) -> Result<(), GuideTargetReason> {
    if guide.file.trim().is_empty() {
        return Err(GuideTargetReason::EmptyPath);
    }
    if guide.file.starts_with('/') {
        return Err(GuideTargetReason::AbsolutePath);
    }
    if guide.file.split('/').any(|component| component == "..") {
        return Err(GuideTargetReason::EscapingPath);
    }
    for key in guide.front_matter.keys() {
        if GUIDE_RESERVED_KEYS.contains(&key.as_str()) {
            return Err(GuideTargetReason::ReservedKey);
        }
        if key.is_empty()
            || !key
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(GuideTargetReason::MalformedKey);
        }
    }
    Ok(())
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

    /// The command patterns to permit (e.g. `["Bash(jigc:*)"]`).
    pub permit: Vec<String>,

    /// The **safety-floor** deny patterns merged into the settings file's
    /// `permissions.deny` — a bounded blocklist against catastrophic/exfil actions
    /// (destructive/exfil shell + secret-file reads on both the `Read` and `Bash`
    /// surfaces), deliberately *orthogonal* to the managed-doc boundary
    /// (`design/assistant-adapter.md` → the `deny` safety floor). Consumed on teardown
    /// by [`remove_deny`]; the install-side `inject` reuses this same list (a later
    /// increment). `#[serde(default)]` so a profile that declares none stays valid.
    #[serde(default)]
    pub deny: Vec<String>,
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

// **There is no `render_spawn` here any more** (M53 post-review-fix review, LOW 9).
//
// It was a `#[allow(dead_code)]` second renderer of the launch line, substituting
// `{{worktree}}` with the repo-relative `.jigc/worktrees/<task_id>` — the exact spelling
// `a8318211` declared broken, because that line is pasted into a shell of unknown cwd. Its
// doc-comment said *"consumed by the launch path (this increment's later tasks)"*, and no
// such consumer ever arrived: the launch line the product emits is composed by
// `engine::compose::emit_fan_out_spawns`, which owns the rule (the absolute worktree the
// caller resolved, rendered through `engine::finding::shell_operand`).
//
// Deleted rather than corrected, because correcting it would keep two renderers of one line
// — which is the defect HIGH 1 of the same review is: the `Spawn:` line and the refusal that
// sends an agent to the same directory had drifted into two spellings, and nothing noticed
// until a path with a space in it made them disagree out loud. The `spawn.template` profile
// key and its `{{worktree}}` placeholder stay; a future launch path that fills it takes its
// value from `emit_fan_out_spawns`' rule, not from a helper that predates it.

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

/// The managed bootstrap file's contents: the canonical routing sentence, the
/// read rule, the context-compiler framing, the output contract, then the
/// machine-output contract — each its own paragraph, with a trailing newline.
///
/// A pure function of the embedded contract — no filesystem. The file is wholly
/// CLI-owned and rewritten in full each `setup`, so it needs no in-file
/// idempotency markers. The routing sentence stays *routing, not content*
/// (`design/bootstrap.md` → The sentence); the four lines beneath it are
/// *use-the-interface* facts — which files are `jigc`'s, what `jigc` is, how to
/// read its signals, and how to consume its machine output — not routing rules.
pub fn bootstrap_file() -> String {
    format!(
        "{BOOTSTRAP_SENTENCE}\n\n{BOOTSTRAP_READ_RULE}\n\n{BOOTSTRAP_FRAMING}\n\n{BOOTSTRAP_PATHS_AND_CWD}\n\n{BOOTSTRAP_OUTPUT_CONTRACT}\n\n{BOOTSTRAP_MACHINE_OUTPUT}\n"
    )
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

/// Idempotently merge the profile's `deny` **safety floor** into the host project's
/// assistant settings file (`<repo_root>/<profile.allowlist.file>`, e.g.
/// `.claude/settings.json`) — the install-side twin of [`remove_deny`], the same
/// structure-aware merge as [`inject_allowlist`].
///
/// `jigc setup`'s safety-floor step (`design/assistant-adapter.md` → the `deny`
/// safety floor: "Merged, never clobbered — mirror `inject_allowlist`"). A
/// **structure-aware** JSON merge, not a text splice: parse (or create) the settings
/// object, ensure every floor pattern from the profile is present in the
/// `permissions.deny` array, and write back pretty JSON. Idempotent by **structural
/// presence** — a pattern already in the array is a no-op, so a re-run is
/// byte-identical; every user `deny` entry, unrelated permissions, and unrelated
/// top-level keys are preserved (a missing file is created with just the floor; a
/// missing `permissions` object or `deny` array is created). A profile declaring **no**
/// floor is a clean no-op — it never creates an empty `permissions.deny`.
pub fn inject_deny(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<()> {
    // A profile with no deny floor: nothing to merge, and never materialize an empty
    // `permissions.deny` array (mirrors `remove_deny`'s empty-floor guard).
    if profile.allowlist.deny.is_empty() {
        return Ok(());
    }
    let target = repo_root.join(&profile.allowlist.file);

    let mut settings = read_settings(&target)?;

    // Navigate/create `permissions.deny`, then ensure each floor pattern is present.
    let deny = settings
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
        .entry("deny")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "`permissions.deny` is not a JSON array",
            )
        })?;

    for pattern in &profile.allowlist.deny {
        let present = deny.iter().any(|v| v.as_str() == Some(pattern.as_str()));
        if !present {
            deny.push(serde_json::Value::String(pattern.clone()));
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
///
/// Returns `Ok(true)` when a jigc-injected reference was actually removed (so `uninstall`
/// reports it in the teardown summary), `Ok(false)` when there was nothing to unwire (an
/// absent file, or a `CLAUDE.md` carrying no jigc section) — the honest no-op signal.
pub fn unwire_reference(repo_root: &Path) -> std::io::Result<bool> {
    let target = repo_root.join("CLAUDE.md");
    let existing = match std::fs::read_to_string(&target) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(false),
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
        return Ok(false);
    };

    if next.is_empty() {
        // setup created the file; remove it so uninstall leaves no jigc trace.
        match std::fs::remove_file(&target) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
            Ok(()) => Ok(true),
        }
    } else {
        std::fs::write(&target, next).map(|()| true)
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
///
/// Returns `Ok(true)` when a permit was actually dropped, `Ok(false)` on the no-op —
/// the honest signal the teardown summary reports on.
pub fn remove_allowlist(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<bool> {
    let target = repo_root.join(&profile.allowlist.file);

    // Absent settings file: nothing to remove.
    if !target.exists() {
        return Ok(false);
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
        return Ok(false);
    };

    let before = allow.len();
    allow.retain(|v| {
        v.as_str()
            .is_none_or(|s| !profile.allowlist.permit.iter().any(|p| p == s))
    });
    // No permit was present: leave the file byte-untouched (idempotent no-op).
    if allow.len() == before {
        return Ok(false);
    }

    write_settings(&target, &settings).map(|()| true)
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

/// Idempotently **remove** the profile's session-event hook from the host project's
/// assistant settings file — the structure-aware inverse of [`inject_hook`] for
/// `jigc uninstall` (`design/project-setup.md` → Flow 2 hardening → Teardown, the M36
/// symmetry fix: "both hooks must come out").
///
/// **Surgical, not a clobber.** Under `hooks.<event>`, drops every inner-`hooks`
/// command entry whose `command == <profile hook.run>` and prunes any matcher whose
/// inner array is thereby emptied — so a matcher jigc added (a lone `jigc start`
/// command) disappears entirely, while a **foreign** `SessionStart` hook sharing the
/// same event array is preserved verbatim, exactly as `remove_allowlist` drops one
/// permit from a possibly-shared `allow` array. **Idempotent + non-destructive:** an
/// absent file, an absent `hooks`/`<event>`, or an event carrying no matching command
/// is a clean no-op (nothing written, so a second `uninstall` is a no-op). A no-op for
/// a profile that declares no hook.
///
/// Returns `Ok(true)` when jigc's hook command was actually dropped, `Ok(false)` on the
/// no-op — the honest signal the teardown summary reports on.
pub fn remove_hook(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<bool> {
    let Some(hook) = profile.hook() else {
        return Ok(false);
    };
    let target = repo_root.join(&profile.allowlist.file);

    // Absent settings file: nothing to remove.
    if !target.exists() {
        return Ok(false);
    }
    let mut settings = read_settings(&target)?;

    // Navigate to `hooks.<event>` if present; absent → a clean no-op.
    let Some(event_matchers) = settings
        .as_object_mut()
        .and_then(|root| root.get_mut("hooks"))
        .and_then(|hooks| hooks.as_object_mut())
        .and_then(|hooks| hooks.get_mut(&hook.event))
        .and_then(|arr| arr.as_array_mut())
    else {
        return Ok(false);
    };

    // Nothing to remove: no matcher under the event runs our command → byte-untouched.
    let present = event_matchers.iter().any(|matcher| {
        matcher
            .get("hooks")
            .and_then(|h| h.as_array())
            .is_some_and(|inner| {
                inner
                    .iter()
                    .any(|cmd| cmd.get("command").and_then(|c| c.as_str()) == Some(&hook.run))
            })
    });
    if !present {
        return Ok(false);
    }

    // Drop our command from every matcher, then prune matchers emptied by that drop
    // (a matcher jigc added). Foreign matchers and foreign commands stay verbatim.
    for matcher in event_matchers.iter_mut() {
        if let Some(inner) = matcher.get_mut("hooks").and_then(|h| h.as_array_mut()) {
            inner.retain(|cmd| cmd.get("command").and_then(|c| c.as_str()) != Some(&hook.run));
        }
    }
    event_matchers.retain(|matcher| {
        matcher
            .get("hooks")
            .and_then(|h| h.as_array())
            .is_none_or(|inner| !inner.is_empty())
    });

    write_settings(&target, &settings).map(|()| true)
}

/// Idempotently **remove** the profile's `deny` safety floor from the host project's
/// assistant settings file — the structure-aware inverse of the install-side deny
/// injection for `jigc uninstall` (`design/assistant-adapter.md` → the `deny` safety
/// floor: "merged, never clobbered — mirror `inject_allowlist`"; the teardown mirror
/// of [`remove_allowlist`]).
///
/// Drops each profile deny pattern from `permissions.deny`, preserving every user
/// deny entry, the `permissions` object, and unrelated keys, and leaving the file
/// valid JSON. **Idempotent + non-destructive:** an empty profile deny set, an absent
/// file, an absent `permissions`/`deny`, or an array carrying none of the floor
/// patterns is a clean no-op (nothing written, so a second `uninstall` is a no-op).
///
/// Returns `Ok(true)` when a floor pattern was actually dropped, `Ok(false)` on the
/// no-op — the honest signal the teardown summary reports on.
pub fn remove_deny(repo_root: &Path, profile: &AdapterProfile) -> std::io::Result<bool> {
    if profile.allowlist.deny.is_empty() {
        return Ok(false);
    }
    let target = repo_root.join(&profile.allowlist.file);

    // Absent settings file: nothing to remove.
    if !target.exists() {
        return Ok(false);
    }
    let mut settings = read_settings(&target)?;

    // Navigate to `permissions.deny` if present; absent → a clean no-op.
    let Some(deny) = settings
        .as_object_mut()
        .and_then(|root| root.get_mut("permissions"))
        .and_then(|perms| perms.as_object_mut())
        .and_then(|perms| perms.get_mut("deny"))
        .and_then(|deny| deny.as_array_mut())
    else {
        return Ok(false);
    };

    let before = deny.len();
    deny.retain(|v| {
        v.as_str()
            .is_none_or(|s| !profile.allowlist.deny.iter().any(|p| p == s))
    });
    // No floor pattern was present: leave the file byte-untouched (idempotent no-op).
    if deny.len() == before {
        return Ok(false);
    }

    write_settings(&target, &settings).map(|()| true)
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
/// subdirs so `config/`, its `.gitkeep`, and `AGENT.md` are committed while the working
/// area and the M36 invocation log are not. The entry set is the **canonical** one
/// [`crate::gitignore::ensure`] writes — the single source of truth shared with the
/// finalize (`task.rs`) and milestone-create (`milestone.rs`) writers — so the gitignore
/// `setup` commits is already final and a later `finalize` never has to amend it.
/// Idempotent: the `.gitignore` is amended only if it lacks an entry, and the empty
/// `.gitkeep` is rewritten byte-identically.
///
/// **Returns what the amend did** ([`crate::gitignore::Ensured`]), because this is one of
/// the four doors that owes an ack for it (`crate::gitignore::IGNORE_DOORS`): `setup`
/// *commits* what it writes here, so an entry appended to an older build's committed file
/// lands in the install commit, and the install summary has to say so (M51 Increment 4 /
/// T2). The `.gitkeep` needs no such report — it is jigc's own empty marker at a path
/// nothing else owns.
pub fn init_project_layer(repo_root: &Path) -> std::io::Result<crate::gitignore::Ensured> {
    let config_dir = repo_root.join(".jigc").join("config");
    std::fs::create_dir_all(&config_dir)?;
    std::fs::write(config_dir.join(".gitkeep"), b"")?;

    crate::gitignore::ensure(&repo_root.join(".jigc"))
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

/// The read rule stated beneath the routing sentence (RC lacon trial B17,
/// 2026-07-17): the crisp discriminator the prohibition above depends on —
/// which docs are "managed" (the `jigc doc list` set), the sanctioned read
/// path for them (`jigc doc show`), and the stated complement (everything
/// else is read freely). It names only stable verbs, never a location
/// convention or a doc enumeration, so it cannot rot
/// (`design/assistant-adapter.md` → Inject the bootstrap). The 2026-07-17
/// surface-comprehension review (B4) added the two edges the flat rule did
/// not compose with: a doc **staged in the open task** is not yet in the
/// committed `doc list` set (so the rule read as "read-free" — its sanctioned
/// read is the task-scoped show, never the file), and an **`unregistered`**
/// `doc list` row is not yet managed — directly readable until adopted. The
/// final sentence is the M44 Inc 4 read-rule amendment (change 2, RC rc.7
/// discoverability rerun, 2026-07-20): project source stays freely readable,
/// but `jigc`'s **own** behavior is learned from the installed binary
/// (`--help`/`describe`/`doc schema`), never a checked-out jigc or pack source
/// tree — which need not match the binary in use. It closes the stale-source
/// reach the flat "read source freely" clause otherwise licensed, without
/// contradicting it (the freedom is scoped to *project* source). The
/// staged-read clause gains its **inverse** at M45 Inc 10 (RC alpha3
/// findings §70): the staged read is stated one direction only — dropping
/// `--task` reads the committed copy — so the agent that read the staged
/// working copy knows how to reach the landed one.
const BOOTSTRAP_READ_RULE: &str = "Managed docs are exactly the `jigc doc list` set — the committed set; read one with `jigc doc show <doc>`. A doc staged in your open task is read with `jigc doc show <doc> --task <id>`, not from the file; drop --task to read the committed copy. An `unregistered` row is not yet managed — readable directly until adopted. Everything else — source, tests, any file not in that set — you read freely. That freedom is for *project* source; to learn how `jigc` itself behaves, ask the installed binary (`jigc --help`, `jigc describe`, `jigc doc schema`), never a checked-out jigc or pack source tree — it need not match the binary you run.";

/// The context-compiler framing stated beneath the read rule (RC greenfield
/// trial A4, 2026-07-06): one stable line on what `jigc` *is* and the
/// division of labor, giving the agent the mental model behind the prohibition.
/// Framing, not content — it names the thesis (categories of ownership), never a
/// convention or doc list, so it cannot rot (`design/assistant-adapter.md` →
/// Inject the bootstrap). The doc-slices promise is scoped **per-tier** (RC
/// lacon trial B2, 2026-07-17): a workflow pulls the slices *it declares*, and
/// a quick fix legitimately declares none — the unscoped "exactly the … slices
/// your task needs" read as broken on the zero-slice tier.
///
/// **The `commits` claim is made true rather than softened (F-10).** *"owns every
/// structural write — … commits"* was false at exactly one point: a commit that
/// had already landed with a wrong message. The rc.14 trial's **only** adapter
/// bypass was a worker who hit that point, found no jigc path, and reached for
/// raw `git commit --amend` — typing `<type>(<scope>): <summary>` by hand, the
/// one structural operation this sentence promises jigc owns. So the clause names
/// the door rather than qualifying the claim, and it names it *here* — six trials
/// running, the verified lens is that the model is reliable at push and
/// unreliable at pull, so a capability no preloaded surface names is a capability
/// nobody finds. It is one clause on the paragraph that already makes the claim,
/// not a seventh paragraph: the floor is short on purpose.
const BOOTSTRAP_FRAMING: &str = "`jigc` is a context compiler: it assembles the workflow steps for your task plus the doc slices that workflow declares (a quick fix may declare none), and owns every structural write — placement, cross-references, commits. You author only the prose. That includes a commit that has already landed: repair a wrong commit message with `jigc task amend`, which re-authors it as a commit doc jigc renders — never with `git commit --amend` by hand.";

/// The output contract stated beneath the framing (M36, narrowed 2026-07-06 —
/// RC greenfield trial A4): the behavioral core — stop on non-zero, follow the
/// output, never retry blindly — plus, since the 2026-07-17
/// surface-comprehension review (B4), the **one-line exit-code taxonomy**,
/// amended 2026-07-23 (M45 Settle, Decision 6):
/// 1 error · 2 usage · 3 blocking findings at a task-scope gate
/// ([`crate::task::EXIT_VALIDATION_BLOCKED`]) · 4 migration review hold
/// ([`crate::task::EXIT_REVIEW_PENDING`]). The added qualifier *at a task-scope
/// gate* — plus the store-scope-blind clause that follows it — repairs a claim
/// that was true of the gate that minted exit 3 and false as a general one: a
/// store-scope `jigc validate` reports blocking content findings at **exit 0**,
/// and a rejected write blocks at **exit 1**. The 2026-07-06 narrowing retired
/// the per-code *meanings enumeration* (each outcome's meaning rides in the
/// command's own output — blocking findings carry `route:` lines, route-less
/// advisories say "no action needed" — a meanings table here was content that
/// rots), and that stands; what the review reinstated is the stable numeric key
/// alone, which both independent readers otherwise had to reverse-engineer from
/// observed behavior. The taxonomy portion is **seam-generated** — asserted ==
/// [`crate::task::bootstrap_taxonomy_line`] by [`bootstrap_line_is_the_table`],
/// so the line cannot drift from the [`crate::task::EXIT_CODES`] table
/// (statement == constant, replacing the earlier "frozen CLI constants" claim
/// that was aspirational when only 3/4 were named).
///
/// **The report-only stance carries its exception (M47 T1 — RC-alpha4 A3).**
/// The store-scope-blind clause shipped *unqualified*, and four days after it was
/// written the confidence-audit fix (2026-07-24) minted an
/// [`engine::validate::SCHEMA_VERSION_AHEAD_CODE`] break that **flips** the sweep's
/// exit — the trial planted exactly that state and read the preload as a lie. The
/// clause is now scoped to the real predicate
/// ([`crate::render::validation_store_exit_flips`]) — but **not** to a cause class
/// (the M47 completion audit's correction): the shipped scoping said *unless the
/// sweep itself could not be trusted*, which is true of three of the four
/// conditions and **false** of `reconciliation.rename`, where the sweep worked and
/// is reporting a real structural-identity change. A class that excludes a real
/// member is the same law-1 lie one tier down. The clause now states only what
/// holds over the whole axis — *a few conditions flip that exit*, unenumerated
/// (four items in a preloaded paragraph is content that rots, the rule that retired
/// the meanings table) — plus the two things that are true of every member: the
/// **flipped exit** ([`crate::render::STORE_EXIT_FLIP_PHRASE`]) and the **closing
/// line** that names which condition fired and why. The one the agent actually
/// meets is still **named**, in [`crate::render::AHEAD_STAMP_PHRASE`] — the
/// trailer's own words — with [`bootstrap_names_the_ahead_exception`] asserting
/// that agreement and [`bootstrap_clause_covers_every_store_exit_flip`] holding the
/// clause true over **every** [`crate::render::STORE_EXIT_FLIPS`] member, so
/// neither can preload and output say opposite things about one exit, nor can the
/// clause quietly shrink to the member its author had in mind.
const BOOTSTRAP_OUTPUT_CONTRACT: &str = "Read every command's output; a non-zero exit means stop and follow what the output says — never retry blindly. Exit codes: 1 error · 2 usage · 3 blocking findings at a task-scope gate · 4 migration review hold. A store-scope `jigc validate` is report-only — it exits 0 even when it surfaces findings — unless one of a few conditions flips that exit, such as a doc stamped above this build's schema-version: then it exits non-zero and its closing line names the condition and why.";

/// **Where the paths jigc prints are rooted, and where jigc may be run** — the paragraph
/// M53's cwd census found missing (`completions/artifacts/M53/cwd-census.md` → C3-03, the
/// census's own *"cheapest single fix in the whole census"*).
///
/// The preload told the agent which files are jigc's and never said what the paths naming
/// them are **relative to**. Every `at:` locus, every `jigc doc list` row and every path a
/// finding names is repo-relative by law 1 (`design/surface-contract.md` → The printed-path
/// fence) — a rule stated to the implementer and to nobody else. An agent reading
/// `docs/spec/x.md` from a subdirectory or a fan-out worktree resolves nothing, and the
/// census's ranked list puts this one sentence upstream of four other rows.
///
/// It says both halves, because either alone is a trap: the paths are rooted at the
/// repository, **and** the binary may be run from anywhere inside it — so an agent that is
/// not at the root neither has to `cd` before invoking jigc nor may assume a printed path
/// resolves where it stands.
///
/// **The exception is stated as a rule, not as a count** (M53 post-review-fix review,
/// LOW 6). It shipped saying *"a `git` command … is the one exception"*, and jigc prints at
/// least three other absolutes: the `cd` of a fan-out `Spawn:` line (which this file's own
/// doc-comment already named, so preload and code contradicted each other in the same
/// commit), the `jigc migrate <absolute source>` operand of every adoption route since the
/// same review's HIGH 2, and the line naming which linked worktree a `finalize` committed in
/// (`crate::render` → `commit_site_line`). A count goes stale the next time a producer joins
/// the class; the two rules behind it do not — **bytes jigc prints for you to run** carry
/// absolutes wherever a relative one would resolve against your directory instead of the
/// repository's (`design/surface-contract.md` → The printed-path fence, the *pasteable shell
/// bytes* rule), and a path naming **a checkout that is not the repository root** is absolute
/// because no repo-relative spelling reaches it.
///
/// The paragraph still names `git` concretely, because that is the one a reader meets on an
/// ordinary gate refusal; the others are covered by the rule rather than enumerated.
///
/// It names no verb and no path, so it cannot rot; it is fenced by the whole-body golden
/// like its five siblings.
const BOOTSTRAP_PATHS_AND_CWD: &str = "Every path jigc prints — an `at:` locus, a `jigc doc list` row, a path inside a finding's message — is relative to the **repository root**, not to your current directory. Two kinds of printed path are absolute instead, and both are on purpose. First, bytes jigc prints for you to **run**: a backticked command carries absolute paths wherever a relative one would resolve against your directory rather than the repository's — a `git` command leads with `git -C <the repository's absolute path>`, and the `cd` of a fan-out `Spawn:` line names the worktree outright — so you can paste it from wherever you are standing and it acts on what the finding is about. Second, a path naming a checkout that is not the repository root, such as the linked worktree a commit landed in: nothing repo-relative reaches it. You may run `jigc` from any directory inside the repository, including a fan-out worktree; it finds the project itself. So resolve a printed path against the repository root, and pass your own file arguments the way you would to any other command — relative to where you are.";

/// The machine-output contract stated as the fifth paragraph (M44 Inc 4,
/// change 1 — RC rc.7 discoverability rerun, 2026-07-20): the preload tier now
/// carries the two facts a driver otherwise reverse-engineers by scraping text.
/// **Every verb** speaks `--format json` on a **successful or validation**
/// outcome — the scope is deliberate (the N4 carve-out): a usage error rejected
/// by clap *before* the JSON funnel prints plain text, so the claim is Law-1
/// true only on the success/validation path. **Composed producers**
/// (`jigc start`/`jigc workflow`/`jigc migrate`) return the minted task id at
/// `.task` — the pinned `{task, text}` compose shape (`render.rs` →
/// `compose_json`); `migrate` rides it because it composes the `migrate-<type>`
/// workflow over the minted task (the rc.7 baseline refuted "migrate lacks
/// `.task`"). It names only stable global-flag + JSON-key facts, never a per-verb
/// enumeration, so it cannot rot (`design/assistant-adapter.md` → Inject the
/// bootstrap). Fenced by the whole-body golden plus [`format_is_a_global_arg`].
const BOOTSTRAP_MACHINE_OUTPUT: &str = "Every verb speaks `--format json` on a successful or validation outcome: pass it and parse the structured result — do not scrape the human-readable lines (a usage error rejected before parsing still prints plain text, not JSON). The composed producers — `jigc start`, `jigc workflow`, `jigc migrate` — return the minted task id at `.task`; read it there, never from the human line.";

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

    /// The shipped Claude Code **guide target** passes `validate_guide_target` and declares
    /// the Claude Code skill path — the canonical-pass anchor for the second decidable rule,
    /// and the one place the shipped path is asserted from the profile rather than re-typed.
    #[test]
    fn shipped_guide_target_passes_validation() {
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        let guide = profile
            .guide()
            .expect("the shipped profile declares a guide target");
        validate_guide_target(guide).expect("the shipped guide target passes the decidable rule");
        assert_eq!(guide.file, ".claude/skills/jigc/SKILL.md");
        assert_eq!(
            guide.front_matter.get("name").map(String::as_str),
            Some("jigc"),
            "the assistant's own header keys ride in the profile, not in the neutral core",
        );
    }

    /// Each broken guide target **fails on its own clause** — the full clause coverage, the
    /// `broken_spawn_templates_fail_on_their_clause` discipline applied to the second gate.
    /// Asserting the *specific* clause is what stops a clause swap passing by failing on a
    /// sibling.
    #[test]
    fn broken_guide_targets_fail_on_their_clause() {
        use GuideTargetReason::*;

        let target = |file: &str, key: &str| GuideTarget {
            file: file.to_string(),
            front_matter: if key.is_empty() {
                std::collections::BTreeMap::new()
            } else {
                std::collections::BTreeMap::from([(key.to_string(), "v".to_string())])
            },
        };

        let cases: &[(GuideTarget, GuideTargetReason)] = &[
            (target("   ", ""), EmptyPath),
            (target("/etc/skills/SKILL.md", ""), AbsolutePath),
            (target("../outside/SKILL.md", ""), EscapingPath),
            // Both stamp keys are reserved: a profile that could name one would forge the
            // record the ownership check reads.
            (target("a/SKILL.md", "jigc-version"), ReservedKey),
            (target("a/SKILL.md", "jigc-body-blake3"), ReservedKey),
            (target("a/SKILL.md", "not a key"), MalformedKey),
        ];
        for (guide, expected) in cases {
            assert_eq!(
                validate_guide_target(guide),
                Err(*expected),
                "guide target {guide:?} must fail on its own clause",
            );
        }

        // …and the shape the shipped profile uses passes, so the gate is not vacuous.
        validate_guide_target(&target(".claude/skills/jigc/SKILL.md", "name"))
            .expect("a well-formed guide target passes");
    }

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
    /// `SessionStart` hook + the allowlist (permit + the M36 `deny` safety floor) +
    /// the **spawn** launch template, and nothing else (no `Resume` hook — that stays
    /// post-MVP). The `deny` floor pairs each secret pattern's `Read` deny with a
    /// `Bash(cat …)` twin (`design/assistant-adapter.md` → the `deny` safety floor).
    #[test]
    fn claude_code_profile_bytes_are_canonical() {
        let bytes = include_str!("../adapters/claude-code.yaml");
        insta::assert_snapshot!(bytes, @r###"
        assistant: claude-code
        # Harness bindings below reflect Claude Code as of 2026-07 — the `SessionStart`
        # hook event and the `.claude/settings.json` permissions schema are the harness's,
        # not jigc's; revisit them if a later Claude Code release changes either.
        inject:
          - reference: { file: CLAUDE.md, to: .jigc/AGENT.md, syntax: at-import }
          - hook: { event: SessionStart, run: "jigc start" }
        allowlist:
          file: .claude/settings.json
          permit: ["Bash(jigc:*)", "Bash(git add:*)"]
          deny:
            - "Bash(rm -rf:*)"
            - "Bash(curl:*)"
            - "Bash(wget:*)"
            - "Bash(git push --force:*)"
            - "Read(./.env)"
            - "Bash(cat ./.env:*)"
            - "Read(./.env.*)"
            - "Bash(cat ./.env.*:*)"
            - "Read(./**/*.pem)"
            - "Bash(cat ./**/*.pem:*)"
            - "Read(./**/*.key)"
            - "Bash(cat ./**/*.key:*)"
            - "Read(./**/id_rsa*)"
            - "Bash(cat ./**/id_rsa*:*)"
            - "Read(./**/id_ed25519*)"
            - "Bash(cat ./**/id_ed25519*:*)"
            - "Read(./**/credentials)"
            - "Bash(cat ./**/credentials:*)"
            - "Read(./**/.npmrc)"
            - "Bash(cat ./**/.npmrc:*)"
            # The two human-owned destroying doors (M50): each deletes bytes that exist in
            # no object DB, and the `--force` past their refusal is the human's consent to
            # spend them, not the agent's.
            - "Bash(jigc uninstall:*)"
            - "Bash(jigc milestone discard:*)"
        spawn:
          template: "Use your Task tool to run: `cd {{worktree}} && jigc workflow {{workflow}} --task {{task_id}}`"
        # The adapter's own owned artifact: the shipped guides, installed as a Claude Code
        # skill, stamped with the binary version and replaced by `setup` (M48 Increment 10).
        # Optional — an assistant with no place to put a guide simply declares none and the
        # install is inert there. `file` is the assistant-specific path; `front-matter` is the
        # assistant's own header, copied verbatim above jigc's `jigc-version:` /
        # `jigc-body-blake3:` stamp lines (which no profile may name).
        guide:
          file: .claude/skills/jigc/SKILL.md
          front-matter:
            name: jigc
            description: How to work in a jigc-managed repository — the setup/start/finalize loop, adopting an existing project, and upgrading a corpus.
        "###);
    }

    /// The floor's **two human-owned destroyers** (M50 Increment 3, Settle D9). This
    /// increment gives `jigc task discard` and `jigc uninstall` the same staged-prose
    /// refusal — but a refusal an agent can consent past with `--force` is not a floor.
    /// Both doors delete bytes that exist in no object DB, so spending them is the
    /// **human's** call, and the profile's `deny` list is the one surface that can say
    /// so: `deny` **blocks**, it does not prompt (the profile has no prompt key), and
    /// [`inject_deny`] is append-if-absent, so this reaches already-installed repos on
    /// their next `setup` — whereas narrowing the `permit` (a prefix pattern, injected
    /// purely additively) would have reached zero installs without pruning a
    /// user-owned file.
    ///
    /// `jigc milestone provision` is deliberately **absent**: M48's fail-closed
    /// leftover classifier already guards it behind `--force`, and denying it would
    /// park an unattended fan-out on a decision nobody is watching.
    ///
    /// Asserted against the **loaded** profile rather than the golden bytes above, so
    /// a later profile edit that reshuffles or rewrites the floor cannot drop either
    /// pattern silently — the golden would simply be re-accepted.
    #[test]
    fn the_deny_floor_carries_the_two_human_owned_destroyers() {
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        for pattern in ["Bash(jigc uninstall:*)", "Bash(jigc milestone discard:*)"] {
            assert!(
                profile.allowlist.deny.iter().any(|p| p == pattern),
                "the deny floor must carry `{pattern}` — a destroying door whose consent \
                 belongs to the human, not the agent; got:\n{:#?}",
                profile.allowlist.deny,
            );
        }

        assert!(
            !profile
                .allowlist
                .deny
                .iter()
                .any(|p| p.contains("milestone provision")),
            "`jigc milestone provision` must stay OFF the floor — M48's fail-closed \
             classifier already guards it, and denying it would park the unattended \
             fan-out; got:\n{:#?}",
            profile.allowlist.deny,
        );
    }

    /// The loader deserializes the shipped profile and exposes its surfaces: the
    /// inject **reference** target points `CLAUDE.md` at the managed
    /// `.jigc/AGENT.md` (the universal floor), the inject **hook** binds
    /// `SessionStart` to `jigc start` (the primary injection), the **allowlist**
    /// file is `.claude/settings.json` with the `Bash(jigc:*)` permit, and the **spawn**
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
            vec!["Bash(jigc:*)".to_string(), "Bash(git add:*)".to_string()],
            "the allowlist permits the `Bash(jigc:*)` command pattern and `Bash(git add:*)` staging",
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

    /// Golden over [`bootstrap_file`]: the canonical routing sentence
    /// (`design/bootstrap.md` → The sentence, verbatim), the read rule (with the
    /// M44 Inc 4 binary-derived-behavior amendment), the context-compiler
    /// framing, **the paths-and-cwd paragraph** (M53 — the cwd census, C3-03), the
    /// output contract, and the machine-output contract (M44 Inc 4) as the managed
    /// file body — six paragraphs, with a single trailing newline. This is the
    /// exact content `jigc setup` writes into `.jigc/AGENT.md` (rewritten whole
    /// each run — no markers).
    #[test]
    fn bootstrap_file_is_the_sentence_body() {
        insta::assert_snapshot!(bootstrap_file(), @r###"
        `jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.

        Managed docs are exactly the `jigc doc list` set — the committed set; read one with `jigc doc show <doc>`. A doc staged in your open task is read with `jigc doc show <doc> --task <id>`, not from the file; drop --task to read the committed copy. An `unregistered` row is not yet managed — readable directly until adopted. Everything else — source, tests, any file not in that set — you read freely. That freedom is for *project* source; to learn how `jigc` itself behaves, ask the installed binary (`jigc --help`, `jigc describe`, `jigc doc schema`), never a checked-out jigc or pack source tree — it need not match the binary you run.

        `jigc` is a context compiler: it assembles the workflow steps for your task plus the doc slices that workflow declares (a quick fix may declare none), and owns every structural write — placement, cross-references, commits. You author only the prose. That includes a commit that has already landed: repair a wrong commit message with `jigc task amend`, which re-authors it as a commit doc jigc renders — never with `git commit --amend` by hand.

        Every path jigc prints — an `at:` locus, a `jigc doc list` row, a path inside a finding's message — is relative to the **repository root**, not to your current directory. Two kinds of printed path are absolute instead, and both are on purpose. First, bytes jigc prints for you to **run**: a backticked command carries absolute paths wherever a relative one would resolve against your directory rather than the repository's — a `git` command leads with `git -C <the repository's absolute path>`, and the `cd` of a fan-out `Spawn:` line names the worktree outright — so you can paste it from wherever you are standing and it acts on what the finding is about. Second, a path naming a checkout that is not the repository root, such as the linked worktree a commit landed in: nothing repo-relative reaches it. You may run `jigc` from any directory inside the repository, including a fan-out worktree; it finds the project itself. So resolve a printed path against the repository root, and pass your own file arguments the way you would to any other command — relative to where you are.

        Read every command's output; a non-zero exit means stop and follow what the output says — never retry blindly. Exit codes: 1 error · 2 usage · 3 blocking findings at a task-scope gate · 4 migration review hold. A store-scope `jigc validate` is report-only — it exits 0 even when it surfaces findings — unless one of a few conditions flips that exit, such as a doc stamped above this build's schema-version: then it exits non-zero and its closing line names the condition and why.

        Every verb speaks `--format json` on a successful or validation outcome: pass it and parse the structured result — do not scrape the human-readable lines (a usage error rejected before parsing still prints plain text, not JSON). The composed producers — `jigc start`, `jigc workflow`, `jigc migrate` — return the minted task id at `.task`; read it there, never from the human line.
        "###);
    }

    /// The managed bootstrap body states the output contract: the behavioral
    /// core (stop on non-zero, follow the output, never retry blindly — M36,
    /// narrowed 2026-07-06) plus the context-compiler framing, and — since the
    /// 2026-07-17 surface-comprehension review (B4) — the **one-line exit-code
    /// taxonomy** (1 error · 2 usage · 3 blocking findings · 4 migration
    /// review hold): both independent readers had to reverse-engineer the
    /// codes from observed behavior. The 2026-07-06 retirement of the per-code
    /// *meanings enumeration* stands — each outcome's meaning still rides the
    /// command's own output; what returns is the stable numeric key alone.
    /// Driven on the emitted body itself — the bytes `setup` writes into
    /// `.jigc/AGENT.md`.
    #[test]
    fn bootstrap_file_states_the_output_contract() {
        let body = bootstrap_file();
        assert!(
            body.contains("never retry blindly"),
            "the bootstrap body must state the behavioral output contract; got:\n{body}",
        );
        assert!(
            body.contains("context compiler"),
            "the bootstrap body must carry the what-jigc-is framing; got:\n{body}",
        );
        assert!(
            body.contains(
                "1 error · 2 usage · 3 blocking findings at a task-scope gate · 4 migration review hold"
            ),
            "the bootstrap body states the one-line exit-code taxonomy, amended with the \
             task-scope qualifier (M45 Decision 6); got:\n{body}",
        );
        assert!(
            body.contains("store-scope `jigc validate` is report-only")
                && body.contains("exits 0 even when it surfaces findings")
                && body.contains("unless one of a few conditions flips that exit"),
            "the bootstrap body carries the store-scope-blind clause the qualifier rests \
             on (M45 Decision 6) — with the exit-flipping exceptions it is scoped by \
             (M47 T1, corrected by the M47 completion audit to a class that covers the \
             whole axis; asserted in full by `bootstrap_names_the_ahead_exception` and \
             `bootstrap_clause_covers_every_store_exit_flip`); got:\n{body}",
        );
        // (M44 Inc 4, change 1) The machine-output paragraph: every verb speaks
        // `--format json` on a successful/validation outcome (the clap-error
        // carve-out), and composed producers return the minted id at `.task`,
        // never scraped from the human line.
        assert!(
            body.contains("`--format json`"),
            "the bootstrap body states the machine-output contract — every verb \
             speaks `--format json`; got:\n{body}",
        );
        assert!(
            body.contains("`.task`"),
            "the machine-output paragraph names the `.task` id field composed \
             producers return; got:\n{body}",
        );
        assert!(
            body.contains("`jigc migrate`"),
            "the machine-output paragraph names all three composed producers \
             (start/workflow/migrate); got:\n{body}",
        );
        assert!(
            body.contains("never") && body.contains("human line"),
            "the machine-output paragraph forbids scraping the human line; got:\n{body}",
        );
    }

    /// Statement == constant (M45 Decision 6, the M43 corollary): the exit-code
    /// taxonomy stated in the managed bootstrap body is the **rendering of the
    /// [`crate::task::EXIT_CODES`] table**, context-parameterized for the
    /// bootstrap surface — so a code, a label, or the whole line cannot drift
    /// from the table a driver actually sees. Driven on the emitted body itself
    /// (the bytes `setup` writes into `.jigc/AGENT.md`), not the raw const.
    #[test]
    fn bootstrap_line_is_the_table() {
        let body = bootstrap_file();
        let line = crate::task::bootstrap_taxonomy_line();
        assert_eq!(
            line,
            "1 error · 2 usage · 3 blocking findings at a task-scope gate · 4 migration review hold",
            "the table renders the amended taxonomy line (M45 Decision 6)",
        );
        assert!(
            body.contains(&line),
            "the bootstrap body's exit-code line must be the EXIT_CODES rendering \
             (statement == constant); rendered:\n{line}\nbody:\n{body}",
        );
    }

    /// The preload tier names the **exit-flipping exception an agent actually meets**
    /// (M47 T1 — RC-alpha4 A3): the unqualified *"a store-scope `jigc validate` is
    /// report-only — it exits 0 even when it surfaces findings"* was four days staler than
    /// the deliberate exit flip the 2026-07-24 confidence-audit fix minted, and the trial
    /// planted exactly that state — a doc stamped above this build's schema-version, which
    /// the sweep cannot adjudicate, so it exits non-zero. The binary was honest at the point
    /// of contradiction (its own trailer says "exits non-zero" and why); the lie lived only
    /// in the preloaded contract, which is the tier an agent trusts *without* re-checking.
    ///
    /// The assert is an **agreement** assert, not a wording one: the emitted body and the
    /// rendered [`crate::render::ahead_corpus_trailer`] must both carry
    /// [`crate::render::AHEAD_STAMP_PHRASE`] and both state the non-zero exit, so neither
    /// surface can be reworded away from the other. Driven on the emitted body — the bytes
    /// `setup` writes into `.jigc/AGENT.md`.
    #[test]
    fn bootstrap_names_the_ahead_exception() {
        let body = bootstrap_file();
        let phrase = crate::render::AHEAD_STAMP_PHRASE;
        let trailer = crate::render::ahead_corpus_trailer();

        assert!(
            trailer.contains(phrase) && trailer.contains("exits non-zero"),
            "the store trailer states the above-current condition and its non-zero exit; \
             got:\n{trailer}",
        );
        assert!(
            body.contains(phrase),
            "the preloaded output contract must name the ahead-stamp exception in the same \
             words the store trailer uses ({phrase:?}); got:\n{body}",
        );
        assert!(
            body.contains("exits non-zero"),
            "the preloaded output contract must state that the exception flips the exit, \
             agreeing with the trailer the sweep prints; got:\n{body}",
        );
        assert!(
            body.contains("store-scope `jigc validate` is report-only"),
            "the report-only stance the exception qualifies is still stated; got:\n{body}",
        );
    }

    /// **The axis fence over the exit-flip class** (M47 completion audit): the preload's
    /// report-only clause is held true against **every** condition that flips the store
    /// sweep's exit ([`crate::render::STORE_EXIT_FLIPS`]), not against the one member its
    /// author had in mind.
    ///
    /// It shipped narrowed — *"unless the sweep itself could not be trusted"* — which is
    /// **false** of `reconciliation.rename`: that sweep worked and is reporting a real
    /// structural-identity change it found (its own trailer says exactly that). One member,
    /// one fence ([`bootstrap_names_the_ahead_exception`], which checks the ahead stamp), one
    /// clause that excluded a real member — the incomplete-fix shape the complete-fix
    /// contract exists to prevent (`implementation/pinning.md`). So this fence iterates the
    /// axis, derived from the same table [`crate::render::validation_store_exit_flips`]
    /// decides the exit with, and a fifth condition reddens it rather than silently
    /// falsifying the preload again:
    ///
    /// - every member really flips the exit through the shared predicate, and its **really
    ///   rendered** closing line delivers what the preload promises — the flipped exit in
    ///   [`crate::render::STORE_EXIT_FLIP_PHRASE`] plus the words naming which condition
    ///   fired;
    /// - the preload states that promise in those same words;
    /// - and while **any** member is a sweep that worked, the preload may not characterize
    ///   the class as an untrustworthy sweep — the narrowing that shipped.
    #[test]
    fn bootstrap_clause_covers_every_store_exit_flip() {
        use engine::result::ValidationReport;

        let body = bootstrap_file();
        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let flip_phrase = crate::render::STORE_EXIT_FLIP_PHRASE;

        for flip in crate::render::STORE_EXIT_FLIPS {
            let id = flip.id;
            let report = ValidationReport::new(vec![(flip.witness)()], &resolved);
            assert!(
                crate::render::validation_store_exit_flips(&report),
                "{id}: this axis member must flip the store sweep's exit through the shared \
                 predicate — otherwise the table and the predicate disagree",
            );
            let rendered = crate::render::validation_store(
                crate::cli::Format::Agent,
                &report,
                &std::collections::BTreeSet::new(),
            );
            assert!(
                rendered.contains(flip_phrase),
                "{id}: the rendered sweep must state the flipped exit in the words the \
                 preload promises ({flip_phrase:?}); got:\n{rendered}",
            );
            assert!(
                rendered.contains(flip.cause),
                "{id}: its closing line must name which condition fired ({:?}) — the \
                 preload sends the agent there for the reason; got:\n{rendered}",
                flip.cause,
            );
        }

        assert!(
            body.contains(flip_phrase) && body.contains("closing line"),
            "the preload must state the promise the whole axis delivers — the flipped exit \
             ({flip_phrase:?}) and the closing line that names the condition; got:\n{body}",
        );

        // The narrowing guard. `sweep_untrustworthy` is declared per member beside its
        // matcher, so a new condition answers the question to compile: while any member is a
        // sweep that *worked* (today `reconciliation.rename`), a clause stating the class as
        // an untrustworthy sweep excludes it — a law-1 lie on the tier an agent trusts
        // without re-checking.
        if let Some(trustworthy) = crate::render::STORE_EXIT_FLIPS
            .iter()
            .find(|flip| !flip.sweep_untrustworthy)
        {
            for claim in [
                "could not be trusted",
                "cannot be trusted",
                "not trustworthy",
            ] {
                assert!(
                    !body.contains(claim),
                    "the exit-flip class must not be stated as {claim:?}: `{}` is a sweep \
                     that worked and is reporting a real event, so the class is wider than \
                     an untrustworthy sweep; got:\n{body}",
                    trustworthy.id,
                );
            }
        }
    }

    /// `--format` is a **global** clap arg — the structural half of the fact the
    /// machine-output paragraph (M44 Inc 4, change 1) rests on: "every verb speaks
    /// `--format json`" is only *possible* because a single global flag reaches
    /// every subcommand.
    ///
    /// **This fence is shape-limited, and deliberately named as such** (M47 Inc 7
    /// T4): it asserts only that the arg *is* global — that every dispatch arm can
    /// see it. It stays green if every arm parses the flag and then **ignores** it,
    /// which is not hypothetical: `TaskCommand::Diff` did exactly that until M47
    /// Inc 7 T1, printing plain text at exit 0 under `--format json` while this
    /// assertion passed. The **behavioural half** — every leaf verb driven to a real
    /// success and asserted to emit exactly one JSON document on stdout and none on
    /// stderr, enumerated from this same clap tree — lives in
    /// `crates/cli/tests/format_json_success_axis.rs`; its reject-surface sibling is
    /// `crates/cli/tests/machine_output.rs`.
    #[test]
    fn format_is_a_global_arg() {
        use clap::CommandFactory;
        let cmd = crate::cli::Cli::command();
        let format = cmd
            .get_arguments()
            .find(|a| a.get_id() == "format")
            .expect("the `--format` arg is defined on the root command");
        assert!(
            format.is_global_set(),
            "`--format` must be a global arg so every dispatch arm honors it — \
             the ground the machine-output paragraph stands on",
        );
    }

    /// The managed bootstrap body states the crisp read rule (M43, RC lacon
    /// trial B17/B2): managed = the `jigc doc list` set, read via
    /// `jigc doc show`; everything else is read freely (the complement,
    /// stated) — and the context-compiler framing scopes the doc-slices
    /// promise per-tier (a workflow pulls the slices *it declares*; a
    /// quick-fix legitimately declares none), so the promise no longer reads
    /// as broken on a zero-slice tier. Driven on the regenerated file itself
    /// — the bytes `setup` writes at `.jigc/AGENT.md`.
    #[test]
    fn bootstrap_file_states_the_read_rule_and_scopes_the_slices_promise() {
        let dir = TempDir::new();
        write_bootstrap_file(dir.path()).expect("write bootstrap file");
        let body = std::fs::read_to_string(dir.path().join(".jigc/AGENT.md"))
            .expect("read the regenerated AGENT.md");

        assert!(
            body.contains("`jigc doc list`"),
            "the read rule names the managed-set discriminator; got:\n{body}",
        );
        assert!(
            body.contains("`jigc doc show"),
            "the read rule names the managed read path; got:\n{body}",
        );
        assert!(
            body.contains("read freely"),
            "the complement rule is stated — everything outside the managed set \
             is read freely; got:\n{body}",
        );
        // (B4, 2026-07-17 surface review) The two edges the flat rule did not
        // compose with: a doc staged in the open task (not yet in the committed
        // `doc list` set — its sanctioned read is the task-scoped show, never the
        // file) and an `unregistered` row (not yet managed — directly readable
        // until adopted).
        assert!(
            body.contains("--task <id>"),
            "the read rule names the staged read for a doc in the open task; got:\n{body}",
        );
        // (M45 Inc 10, findings §70) The staged-read clause's inverse: dropping
        // `--task` reads the committed copy — the direction an agent that read the
        // staged working copy needs to reach the landed one.
        assert!(
            body.contains("drop --task to read the committed copy"),
            "the read rule states the staged-read inverse (drop --task → committed copy); \
             got:\n{body}",
        );
        assert!(
            body.contains("`unregistered` row"),
            "the read rule states the unregistered-row edge; got:\n{body}",
        );
        assert!(
            !body.contains("exactly the workflow steps and doc slices your task needs"),
            "the unscoped doc-slices promise is retired — the framing scopes \
             slices to what the workflow declares; got:\n{body}",
        );
        assert!(
            body.contains("declares"),
            "the framing scopes the doc-slices promise per-tier — a workflow \
             pulls the slices it declares; got:\n{body}",
        );
        // (M44 Inc 4, change 2) The read-rule amendment: jigc's own behavior is
        // derived from the installed binary (`--help`/`describe`/`doc schema`),
        // never from a checked-out jigc/pack source tree — reconciled with the
        // preserved "read project source freely" clause.
        assert!(
            body.contains("installed binary"),
            "the read rule derives jigc's behavior from the installed binary; got:\n{body}",
        );
        assert!(
            body.contains("`jigc doc schema`"),
            "the read rule names the binary-introspection path (doc schema); got:\n{body}",
        );
        assert!(
            body.contains("checked-out"),
            "the read rule forbids deriving jigc's behavior from a checked-out \
             jigc/pack source tree; got:\n{body}",
        );
        assert!(
            body.contains("read freely"),
            "the amendment preserves the existing read-project-source-freely \
             clause; got:\n{body}",
        );
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
                engine::tempname::unique_nanos(),
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
            !allow.iter().any(|x| x == "Bash(jigc:*)"),
            "the `Bash(jigc:*)` permit must be dropped; got:\n{after}",
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

    /// [`remove_hook`] drops jigc's `SessionStart` command **surgically** from a
    /// `hooks` object that also carries a **foreign** `SessionStart` hook: the foreign
    /// hook (and any unrelated event) survives byte-for-byte while jigc's matcher is
    /// pruned, and a second remove is a clean byte-identical no-op. The RED obligation:
    /// surgical, not a clobber.
    #[test]
    fn remove_hook_drops_jigc_matcher_preserving_foreign_and_is_idempotent() {
        let dir = TempDir::new();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).expect("create .claude");
        let settings = claude_dir.join("settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        // Seed a settings file carrying a FOREIGN SessionStart hook, then install
        // jigc's alongside it (the real shared-file case).
        let seeded = serde_json::json!({
            "hooks": {
                "SessionStart": [
                    { "hooks": [ { "type": "command", "command": "my-own-tool --greet" } ] }
                ],
                "PreToolUse": [
                    { "hooks": [ { "type": "command", "command": "audit-log" } ] }
                ]
            }
        });
        std::fs::write(
            &settings,
            format!("{}\n", serde_json::to_string_pretty(&seeded).unwrap()),
        )
        .unwrap();
        inject_hook(dir.path(), &profile).expect("inject jigc hook alongside the foreign one");

        remove_hook(dir.path(), &profile).expect("remove jigc hook");
        let after = std::fs::read_to_string(&settings).expect("read after remove");
        let parsed: serde_json::Value =
            serde_json::from_str(&after).expect("settings stays valid JSON");

        let session = parsed["hooks"]["SessionStart"]
            .as_array()
            .expect("SessionStart stays an array");
        let has_command = |arr: &[serde_json::Value], cmd: &str| {
            arr.iter().any(|matcher| {
                matcher["hooks"]
                    .as_array()
                    .is_some_and(|inner| inner.iter().any(|c| c["command"].as_str() == Some(cmd)))
            })
        };
        assert!(
            !has_command(session, "jigc start"),
            "jigc's SessionStart command must be dropped; got:\n{after}",
        );
        assert!(
            has_command(session, "my-own-tool --greet"),
            "the foreign SessionStart hook must survive; got:\n{after}",
        );
        assert!(
            parsed["hooks"]["PreToolUse"]
                .as_array()
                .is_some_and(|a| { has_command(a, "audit-log") }),
            "an unrelated event must survive untouched; got:\n{after}",
        );

        // Idempotent: a second remove (jigc command already gone) is byte-identical.
        remove_hook(dir.path(), &profile).expect("second remove");
        assert_eq!(
            std::fs::read_to_string(&settings).expect("present"),
            after,
            "a second remove over a jigc-command-free hooks object is byte-identical",
        );
    }

    /// [`remove_hook`] over an **absent** settings file is a clean no-op (no file is
    /// created) — the idempotent teardown over a project that never had one.
    #[test]
    fn remove_hook_over_absent_settings_is_a_no_op() {
        let dir = TempDir::new();
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        remove_hook(dir.path(), &profile).expect("remove over absent settings");
        assert!(
            !dir.path().join(".claude/settings.json").exists(),
            "remove over an absent settings file must not create one",
        );
    }

    /// [`remove_deny`] drops every profile deny-floor pattern from `permissions.deny`
    /// while a **foreign** deny entry (and an unrelated top-level key) survives, and a
    /// second remove is a clean byte-identical no-op. Driven against a **pre-seeded**
    /// settings file (the install-side deny injection lands in a later increment —
    /// `design/project-setup.md` → the DENY COUPLING note), so this is the teardown
    /// contract on its own.
    #[test]
    fn remove_deny_drops_floor_preserving_foreign_and_is_idempotent() {
        let dir = TempDir::new();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).expect("create .claude");
        let settings = claude_dir.join("settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        assert!(
            !profile.allowlist.deny.is_empty(),
            "the shipped profile must carry a non-empty deny floor",
        );

        // Seed a settings file whose `permissions.deny` carries a FOREIGN entry plus
        // every jigc floor pattern (as the install-side inject will one day leave it).
        let mut deny: Vec<serde_json::Value> =
            vec![serde_json::Value::String("Bash(shutdown:*)".to_string())];
        for p in &profile.allowlist.deny {
            deny.push(serde_json::Value::String(p.clone()));
        }
        let seeded = serde_json::json!({
            "model": "claude-sonnet-4",
            "permissions": { "deny": deny },
        });
        std::fs::write(
            &settings,
            format!("{}\n", serde_json::to_string_pretty(&seeded).unwrap()),
        )
        .unwrap();

        remove_deny(dir.path(), &profile).expect("remove deny floor");
        let after = std::fs::read_to_string(&settings).expect("read after remove");
        let parsed: serde_json::Value =
            serde_json::from_str(&after).expect("settings stays valid JSON");
        let deny_after = parsed["permissions"]["deny"].as_array().unwrap();
        for p in &profile.allowlist.deny {
            assert!(
                !deny_after.iter().any(|x| x.as_str() == Some(p.as_str())),
                "the floor pattern `{p}` must be dropped; got:\n{after}",
            );
        }
        assert!(
            deny_after.iter().any(|x| x == "Bash(shutdown:*)"),
            "the foreign deny entry must survive; got:\n{after}",
        );
        assert_eq!(
            parsed["model"], "claude-sonnet-4",
            "the unrelated top-level key must survive; got:\n{after}",
        );

        // Idempotent: a second remove (floor already gone) is byte-identical.
        remove_deny(dir.path(), &profile).expect("second remove");
        assert_eq!(
            std::fs::read_to_string(&settings).expect("present"),
            after,
            "a second remove over a floor-free deny list is byte-identical",
        );
    }

    /// [`remove_deny`] over an **absent** settings file is a clean no-op (no file is
    /// created).
    #[test]
    fn remove_deny_over_absent_settings_is_a_no_op() {
        let dir = TempDir::new();
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        remove_deny(dir.path(), &profile).expect("remove over absent settings");
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
    /// golden-locked: pretty JSON, the `Bash(jigc:*)` permit present once.
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
            after_first.matches("\"Bash(jigc:*)\"").count(),
            1,
            "the permit appears exactly once — no duplicate, got:\n{after_first}",
        );

        insta::assert_snapshot!(after_first, @r#"
        {
          "permissions": {
            "allow": [
              "Bash(jigc:*)",
              "Bash(git add:*)"
            ]
          }
        }
        "#);
    }

    /// Merging into a `.claude/settings.json` that already holds an unrelated
    /// top-level key **and** an unrelated permission preserves both: the `Bash(jigc:*)`
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
            after.contains("\"Bash(jigc:*)\""),
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
            after.matches("\"Bash(jigc:*)\"").count(),
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
              "Bash(jigc:*)",
              "Bash(git add:*)"
            ]
          }
        }
        "#);
    }

    /// First deny-floor merge into a project with **no** `.claude/settings.json`
    /// creates the file with a `permissions.deny` array holding every profile floor
    /// pattern (in profile order, each exactly once); a second merge is byte-identical
    /// (run twice ⇒ byte-identical). The created form is golden-locked — the exact
    /// bytes `jigc setup` merges (`design/assistant-adapter.md` → the `deny` safety
    /// floor: each secret `Read` deny pairs with a `Bash(cat …)` twin).
    #[test]
    fn deny_floor_added_then_idempotent() {
        let dir = TempDir::new();
        let settings = dir.path().join(".claude/settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");
        assert!(
            !profile.allowlist.deny.is_empty(),
            "the shipped profile must carry a non-empty deny floor",
        );

        inject_deny(dir.path(), &profile).expect("first deny merge");
        assert!(
            settings.exists(),
            "merge creates .claude/settings.json when absent",
        );
        let after_first = std::fs::read_to_string(&settings).expect("read after first");

        // Every floor pattern is present exactly once.
        let parsed: serde_json::Value = serde_json::from_str(&after_first).unwrap();
        let deny = parsed["permissions"]["deny"].as_array().unwrap();
        for p in &profile.allowlist.deny {
            assert_eq!(
                deny.iter()
                    .filter(|x| x.as_str() == Some(p.as_str()))
                    .count(),
                1,
                "the floor pattern `{p}` must appear exactly once, got:\n{after_first}",
            );
        }

        inject_deny(dir.path(), &profile).expect("second deny merge");
        let after_second = std::fs::read_to_string(&settings).expect("read after second");
        assert_eq!(
            after_first, after_second,
            "deny merge is idempotent: a second run leaves the file byte-identical",
        );

        insta::assert_snapshot!(after_first, @r#"
        {
          "permissions": {
            "deny": [
              "Bash(rm -rf:*)",
              "Bash(curl:*)",
              "Bash(wget:*)",
              "Bash(git push --force:*)",
              "Read(./.env)",
              "Bash(cat ./.env:*)",
              "Read(./.env.*)",
              "Bash(cat ./.env.*:*)",
              "Read(./**/*.pem)",
              "Bash(cat ./**/*.pem:*)",
              "Read(./**/*.key)",
              "Bash(cat ./**/*.key:*)",
              "Read(./**/id_rsa*)",
              "Bash(cat ./**/id_rsa*:*)",
              "Read(./**/id_ed25519*)",
              "Bash(cat ./**/id_ed25519*:*)",
              "Read(./**/credentials)",
              "Bash(cat ./**/credentials:*)",
              "Read(./**/.npmrc)",
              "Bash(cat ./**/.npmrc:*)",
              "Bash(jigc uninstall:*)",
              "Bash(jigc milestone discard:*)"
            ]
          }
        }
        "#);
    }

    /// Merging the floor into a `.claude/settings.json` that already holds a **foreign**
    /// `permissions.deny` entry (plus an unrelated top-level key and the allowlist)
    /// preserves all of them — **merged, never clobbered**: every floor pattern is added
    /// once, the foreign deny entry stays, and the unrelated key + allow list are
    /// untouched. A re-run is byte-identical.
    #[test]
    fn deny_floor_preserves_existing_and_is_idempotent() {
        let dir = TempDir::new();
        let claude_dir = dir.path().join(".claude");
        std::fs::create_dir_all(&claude_dir).expect("create .claude");
        let settings = claude_dir.join("settings.json");
        let profile = load_profile("claude-code").expect("the shipped profile loads");

        // A settings file with an unrelated top-level key, an existing allow list, and a
        // FOREIGN deny entry the human wrote.
        let preexisting = r#"{
  "model": "claude-sonnet-4",
  "permissions": {
    "allow": [
      "Bash(ls:*)"
    ],
    "deny": [
      "Bash(shutdown:*)"
    ]
  }
}
"#;
        std::fs::write(&settings, preexisting).expect("seed settings.json");

        inject_deny(dir.path(), &profile).expect("merge the floor into existing settings");
        let after = std::fs::read_to_string(&settings).expect("read after merge");
        let parsed: serde_json::Value =
            serde_json::from_str(&after).expect("settings stays valid JSON");

        let deny = parsed["permissions"]["deny"].as_array().unwrap();
        assert!(
            deny.iter().any(|x| x == "Bash(shutdown:*)"),
            "the foreign deny entry must survive (never clobbered), got:\n{after}",
        );
        for p in &profile.allowlist.deny {
            assert!(
                deny.iter().any(|x| x.as_str() == Some(p.as_str())),
                "the floor pattern `{p}` must be merged in, got:\n{after}",
            );
        }
        assert_eq!(
            parsed["model"], "claude-sonnet-4",
            "the unrelated top-level key must survive, got:\n{after}",
        );
        assert!(
            parsed["permissions"]["allow"]
                .as_array()
                .is_some_and(|a| a.iter().any(|x| x == "Bash(ls:*)")),
            "the unrelated allow list must survive, got:\n{after}",
        );

        // Idempotent: a second merge over the now-current file is byte-identical.
        inject_deny(dir.path(), &profile).expect("second merge");
        let after_second = std::fs::read_to_string(&settings).expect("read after second");
        assert_eq!(after, after_second, "re-merge is byte-identical");
    }

    /// A profile with an **empty** deny floor is inert — [`inject_deny`] writes nothing
    /// and never materializes an empty `permissions.deny` (the empty-floor guard, the
    /// mirror of [`remove_deny`]'s). Covers the omitting context, not just the shipped
    /// happy path.
    #[test]
    fn deny_floor_empty_is_inert() {
        let dir = TempDir::new();
        let settings = dir.path().join(".claude/settings.json");
        let mut profile = load_profile("claude-code").expect("the shipped profile loads");
        profile.allowlist.deny.clear();

        inject_deny(dir.path(), &profile).expect("inject over an empty floor");
        assert!(
            !settings.exists(),
            "an empty deny floor must not create a settings file / empty deny array",
        );
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
            after.contains("\"Bash(jigc:*)\""),
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
              "Bash(jigc:*)",
              "Bash(git add:*)"
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
              \x20 permit: [\"Bash(jigc:*)\"]\n\
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
