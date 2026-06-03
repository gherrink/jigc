//! `jigc config <verb>` — authoring cascade deltas into the project layer
//! (`design/overrides.md` → Authoring deltas — the `jigc config` verbs).
//!
//! Only `config set <key> <value>` lands this increment (the `scalar-set` rung);
//! the `insert-step`/`replace-step`/`remove-step`/`fill`/`fork` rungs are later
//! increments. `set` writes **only** the project layer's
//! `.jigc/config/manifest.yaml` `scalar:` block — never a base definition, never
//! a fork.
//!
//! **Write-time adjudication** (`overrides.md` → Write-time vs resolve-time
//! split): the cheap, local checks run here so the human gets an immediate error
//! — the key must be a **declared knob** (the pack's `config/knobs.yaml` closed
//! surface) and its value must pass the **same** [`engine::write::check_value`]
//! the doc write path uses (no second type system). An undeclared key or a
//! wrong-type value is rejected with a routed blocking finding and **no** write;
//! whole-cascade consequences (cycles, orphaning) surface later at resolution
//! through `workflow-refs`.

use crate::pack::EmbeddedPack;
use anyhow::{Context, Result, bail};
use engine::field_block::Value;
use engine::finding::Finding;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// The `jigc config <verb>` subcommand tree. Only `set` exists this increment
/// (`design/overrides.md` → Authoring deltas).
#[derive(Debug, clap::Subcommand, PartialEq, Eq)]
pub enum ConfigCommand {
    /// Record a `scalar-set` delta — set a closed-surface knob in the project
    /// layer's `manifest.yaml` `scalar:` block. Adjudicated at write time against
    /// the pack's declared knob (`check_value`): an undeclared key or a wrong-type
    /// value is rejected non-zero with a routed finding.
    Set {
        /// The knob key to set — one of the pack's declared `config/knobs.yaml` keys.
        key: String,
        /// The value to set (adjudicated against the knob's declared type).
        value: String,
    },
}

impl ConfigCommand {
    /// Dispatch the parsed `config` verb against `cwd`, mapping the result to a
    /// process exit code. A blocking adjudication finding surfaces on stderr (with
    /// its route) and exits non-zero; a clean write exits 0.
    pub fn dispatch(self, cwd: &Path) -> ExitCode {
        let result = match self {
            ConfigCommand::Set { key, value } => run_set(cwd, &key, &value),
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err:#}");
                ExitCode::FAILURE
            }
        }
    }
}

/// `jigc config set <key> <value>` — record a `scalar-set` in the project layer.
///
/// Adjudicates the write locally (`overrides.md` → Write-time vs resolve-time
/// split), then records the value into the project manifest's `scalar:` block:
/// 1. `<key>` is a declared knob in the pack's `config/knobs.yaml` else reject;
/// 2. `<value>` passes `check_value` against the knob's declared type else reject;
/// 3. write `scalar.<key> = <value>` into `.jigc/config/manifest.yaml`
///    (last-write-wins; any existing `scalar:` entries and `deltas:` block are
///    preserved).
fn run_set(cwd: &Path, key: &str, value: &str) -> Result<()> {
    let repo_root = discover_repo_root(cwd)
        .with_context(|| format!("not inside a git repository (from {})", cwd.display()))?;
    let project_config = repo_root.join(".jigc").join("config");
    if !project_config.is_dir() {
        bail!(
            "this project isn't set up — run `jigc setup` (no `.jigc/config/` cascade layer found)"
        );
    }

    // Step 1 — the key must be a declared knob (the closed surface).
    let pack = EmbeddedPack::new();
    let knobs_bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .context("the embedded pack is missing `config/knobs`")?;
    let knobs = engine::knobs::load_knobs(&knobs_bytes).context("`config/knobs` is malformed")?;
    let Some(field) = knobs.field(key) else {
        return Err(finding_to_err(Finding::block(
            "config.undeclared-key",
            format!("`{key}` is not a settable knob — the cascade surface is closed"),
            "run `jigc start` to orient; settable knobs are declared by the pack",
        )));
    };

    // Step 2 — the value must pass the same `check_value` the doc write path uses.
    if let Err(reason) = engine::write::check_value(field, &Value::Scalar(value.to_owned())) {
        return Err(finding_to_err(Finding::block(
            "config.value-rejected",
            format!("`{value}` is not a valid value for `{key}`: {reason}"),
            "set a value matching the knob's declared type",
        )));
    }

    // Step 3 — record the `scalar-set` into the project manifest (last-write-wins).
    write_scalar(&project_config, key, value)
}

/// Record `scalar.<key> = <value>` into `<project_config>/manifest.yaml`,
/// preserving any existing top-level keys (the `deltas:` list) and `scalar:`
/// entries. A missing manifest is created with a single-entry `scalar:` block.
///
/// The manifest is round-tripped through `serde_yaml_ng::Value` so the loader
/// (`crate::start::load_project_layer`) reads back exactly the `scalar:` map this
/// writes — the one manifest both the write and read paths agree on.
fn write_scalar(project_config: &Path, key: &str, value: &str) -> Result<()> {
    use serde_yaml_ng::Value as Yaml;

    let manifest = project_config.join("manifest.yaml");
    let mut doc: Yaml = match std::fs::read_to_string(&manifest) {
        Ok(text) => serde_yaml_ng::from_str(&text)
            .with_context(|| format!("{} is not valid YAML", manifest.display()))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Yaml::Mapping(Default::default()),
        Err(e) => {
            return Err(e).with_context(|| format!("could not read {}", manifest.display()));
        }
    };

    // A present-but-non-mapping (or null/empty) manifest becomes a fresh map so the
    // `scalar:` block has a home.
    let map = match &mut doc {
        Yaml::Mapping(map) => map,
        _ => {
            doc = Yaml::Mapping(Default::default());
            let Yaml::Mapping(map) = &mut doc else {
                unreachable!("just set to a mapping")
            };
            map
        }
    };

    // Locate or create the `scalar:` sub-map, then set `<key> = <value>`.
    let scalar_key = Yaml::String("scalar".to_owned());
    let scalar = map
        .entry(scalar_key)
        .or_insert_with(|| Yaml::Mapping(Default::default()));
    if !scalar.is_mapping() {
        *scalar = Yaml::Mapping(Default::default());
    }
    let Yaml::Mapping(scalar_map) = scalar else {
        unreachable!("just ensured a mapping")
    };
    scalar_map.insert(Yaml::String(key.to_owned()), Yaml::String(value.to_owned()));

    let rendered = serde_yaml_ng::to_string(&doc)
        .with_context(|| format!("could not serialize {}", manifest.display()))?;
    std::fs::write(&manifest, rendered)
        .with_context(|| format!("could not write {}", manifest.display()))?;
    Ok(())
}

/// Map an engine [`Finding`] to an `anyhow` error carrying its message + route —
/// the same envelope `crate::start` / `crate::task` use for a blocking finding.
fn finding_to_err(finding: Finding) -> anyhow::Error {
    match finding.route {
        Some(route) => anyhow::anyhow!("{}\n  route: {route}", finding.message),
        None => anyhow::anyhow!("{}", finding.message),
    }
}

/// Walk up from `start` to the directory holding `.git` (the repo root) — the same
/// discovery `crate::start` / `crate::task` do.
fn discover_repo_root(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join(".git").exists())
        .map(PathBuf::from)
}
