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
use engine::cascade::{Anchor, StructuralDelta, StructuralTarget};
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

/// Append one `structural-op` delta to `<project_config>/manifest.yaml`'s
/// `deltas:` list, preserving any existing top-level keys (the `scalar:` map) and
/// prior `deltas:` entries. A missing manifest is created; a missing/blank
/// `deltas:` block becomes a fresh list (`design/overrides.md` → Delta
/// representation; `design/storage.md` → Config layout).
///
/// The on-disk spelling mirrors what [`crate::start::load_project_layer`] reads
/// back (`design/overrides.md` → Delta targets / the `jigc config` verbs):
/// - `kind:` — `insert-step` / `replace-step` / `remove-step`;
/// - `target:` — `workflow:<id>` for an insert (the anchor rides `after:`/`before:`),
///   `workflow:<id>#<step-id>` for a replace/remove ([`Anchor::At`]);
/// - `after:`/`before:` — the insert anchor step id;
/// - `with:` — `step:<basename>`, the native step file's id (insert/replace only).
///
/// The manifest is round-tripped through `serde_yaml_ng::Value` so the loader
/// reads back exactly the `deltas:` list this writes (the preserve discipline
/// [`write_scalar`] uses for the `scalar:` sub-map, mirrored for `deltas:`).
// Consumed by the `insert-step` / `replace-step` / `remove-step` dispatch (this
// increment's T3/T4) + the test below; `allow` covers the non-test build until
// that wiring lands.
#[allow(dead_code)]
fn append_delta(project_config: &Path, delta: &StructuralDelta) -> Result<()> {
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
    // `deltas:` list has a home.
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

    // Locate or create the `deltas:` sequence, then push the new entry.
    let deltas_key = Yaml::String("deltas".to_owned());
    let deltas = map
        .entry(deltas_key)
        .or_insert_with(|| Yaml::Sequence(Vec::new()));
    if !deltas.is_sequence() {
        *deltas = Yaml::Sequence(Vec::new());
    }
    let Yaml::Sequence(list) = deltas else {
        unreachable!("just ensured a sequence")
    };
    list.push(delta_to_yaml(delta));

    let rendered = serde_yaml_ng::to_string(&doc)
        .with_context(|| format!("could not serialize {}", manifest.display()))?;
    std::fs::write(&manifest, rendered)
        .with_context(|| format!("could not write {}", manifest.display()))?;
    Ok(())
}

/// Render one [`StructuralDelta`] to its manifest-entry mapping — the inverse of
/// [`crate::start::load_project_layer`]'s `parse_one_delta`. The `with:` step id
/// is written with the `step:` prefix (the manifest spelling the loader strips).
fn delta_to_yaml(delta: &StructuralDelta) -> serde_yaml_ng::Value {
    use serde_yaml_ng::Value as Yaml;

    let mut entry = serde_yaml_ng::Mapping::new();
    let mut put = |k: &str, v: String| {
        entry.insert(Yaml::String(k.to_owned()), Yaml::String(v));
    };
    let wf = |id: &str| format!("workflow:{id}");
    match delta {
        StructuralDelta::Insert { target, step } => {
            put("kind", "insert-step".to_owned());
            put("target", wf(&target.workflow_id));
            match &target.anchor {
                Anchor::After(id) => put("after", id.clone()),
                Anchor::Before(id) => put("before", id.clone()),
                Anchor::At(id) => put("after", id.clone()),
            }
            put("with", format!("step:{step}"));
        }
        StructuralDelta::Replace { target, step } => {
            put("kind", "replace-step".to_owned());
            put(
                "target",
                format!("{}#{}", wf(&target.workflow_id), at_step(target)),
            );
            put("with", format!("step:{step}"));
        }
        StructuralDelta::Remove { target } => {
            put("kind", "remove-step".to_owned());
            put(
                "target",
                format!("{}#{}", wf(&target.workflow_id), at_step(target)),
            );
        }
    }
    Yaml::Mapping(entry)
}

/// The `#<step-id>` step a replace/remove target names (`Anchor::At`). Insert
/// targets never reach here; an `After`/`Before` on a replace/remove is a writer
/// bug, so the id is still emitted rather than silently dropped.
fn at_step(target: &StructuralTarget) -> String {
    match &target.anchor {
        Anchor::At(id) | Anchor::After(id) | Anchor::Before(id) => id.clone(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A throwaway directory that removes itself on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "jigc-config-delta-{tag}-{}-{:?}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
            ));
            fs::create_dir_all(&path).expect("create temp dir");
            TempDir(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn insert_after(workflow: &str, anchor: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Insert {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::After(anchor.to_owned()),
            },
            step: step.to_owned(),
        }
    }

    fn replace_at(workflow: &str, at: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(at.to_owned()),
            },
            step: step.to_owned(),
        }
    }

    /// T1 done-criterion: append an insert-step then a replace-step into a temp
    /// manifest that already carries a `scalar:` entry; the round-tripped YAML
    /// carries both delta entries **and** the original `scalar:` entry, and
    /// `start::load_project_layer` parses the result back to the two expected
    /// [`StructuralDelta`]s — preserve discipline over the `deltas:` block, the
    /// inverse of the loader's `parse_one_delta` (`design/overrides.md` → Delta
    /// representation; `design/storage.md` → Config layout).
    #[test]
    fn append_delta_preserves_scalar_and_prior_deltas_and_round_trips() {
        let dir = TempDir::new("append");
        let project_config = dir.path();
        // Seed a manifest already carrying a `scalar:` entry (the writer must not
        // clobber it when it grows the `deltas:` list).
        fs::write(
            project_config.join("manifest.yaml"),
            "scalar:\n  default-workflow: single-task\n",
        )
        .expect("seed manifest");

        let insert = insert_after("single-task", "implement", "project-validate");
        let replace = replace_at("single-task", "validate", "project-validate");
        append_delta(project_config, &insert).expect("append insert-step");
        append_delta(project_config, &replace).expect("append replace-step");

        // The original `scalar:` entry survives the two appends.
        let text =
            fs::read_to_string(project_config.join("manifest.yaml")).expect("read manifest back");
        let doc: serde_yaml_ng::Value = serde_yaml_ng::from_str(&text).expect("manifest is YAML");
        assert_eq!(
            doc.get("scalar")
                .and_then(|s| s.get("default-workflow"))
                .and_then(serde_yaml_ng::Value::as_str),
            Some("single-task"),
            "the original scalar: entry must survive the delta appends\n{text}"
        );
        let count = doc
            .get("deltas")
            .and_then(serde_yaml_ng::Value::as_sequence)
            .map(Vec::len)
            .unwrap_or(0);
        assert_eq!(count, 2, "both delta entries must be present\n{text}");

        // The loader parses the written manifest back to the two deltas, in order.
        let (_layer, deltas) =
            crate::start::load_project_layer(project_config).expect("loader parses the manifest");
        assert_eq!(
            deltas,
            vec![insert, replace],
            "load_project_layer must round-trip the two appended deltas in order\n{text}"
        );
    }
}
