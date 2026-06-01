//! `EmbeddedPack` — the MVP `PackSource` impl that serves the built-in dev pack
//! from bytes embedded in the `jigc` binary.
//!
//! Embed mechanism is `include_dir` (decided 2026-05-31; always-embedded, so what
//! you test is what ships). The pack tree lives in `crates/cli/pack/`, one
//! sub-directory per [`PackResourceKind`] (`workflows/`, `schemas/`, `steps/`,
//! `config/`); a resource's [`ResourceId`] is its file stem. The pack versions
//! with the release, so `pack_version` is the binary's `CARGO_PKG_VERSION`
//! (override-reconciliation: built-in pack-default version = binary version).
//! See `implementation/module-layout.md` → The dev pack's home.

use engine::packsource::{PackError, PackResourceKind, PackSource, ResourceId};
use include_dir::{Dir, include_dir};

/// The built-in dev pack, embedded at compile time from `crates/cli/pack/`.
static PACK: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/pack");

/// `PackSource` over the binary-embedded built-in dev pack.
pub struct EmbeddedPack;

impl EmbeddedPack {
    pub fn new() -> Self {
        EmbeddedPack
    }
}

impl Default for EmbeddedPack {
    fn default() -> Self {
        Self::new()
    }
}

/// The pack sub-directory that holds resources of `kind`.
fn kind_dir(kind: PackResourceKind) -> &'static str {
    match kind {
        PackResourceKind::Schemas => "schemas",
        PackResourceKind::Workflows => "workflows",
        PackResourceKind::Steps => "steps",
        PackResourceKind::Config => "config",
    }
}

impl PackSource for EmbeddedPack {
    fn pack_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
        let Some(dir) = PACK.get_dir(kind_dir(kind)) else {
            return Vec::new();
        };
        let mut ids: Vec<ResourceId> = dir
            .files()
            .filter_map(|f| f.path().file_stem())
            .filter_map(|stem| stem.to_str())
            .map(ResourceId::from)
            .collect();
        ids.sort();
        ids
    }

    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
        let dir = PACK.get_dir(kind_dir(kind));
        let bytes = dir.and_then(|dir| {
            dir.files()
                .find(|f| f.path().file_stem().and_then(|s| s.to_str()) == Some(id.as_str()))
                .map(|f| f.contents().to_vec())
        });
        bytes.ok_or_else(|| PackError::NotFound {
            kind,
            id: id.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_pack_lists_and_reads_the_shipped_workflows() {
        let pack = EmbeddedPack::new();

        let workflows = pack.list(PackResourceKind::Workflows);
        assert!(
            workflows.contains(&ResourceId::from("single-task")),
            "the embedded pack must ship the `single-task` workflow; got {workflows:?}",
        );

        let bytes = pack
            .read(
                PackResourceKind::Workflows,
                &ResourceId::from("single-task"),
            )
            .expect("the shipped `single-task` workflow reads back");
        assert!(
            !bytes.is_empty(),
            "the `single-task` workflow definition must be non-empty YAML",
        );
    }

    #[test]
    fn embedded_pack_read_of_a_missing_id_is_not_found() {
        let pack = EmbeddedPack::new();
        let err = pack
            .read(PackResourceKind::Workflows, &ResourceId::from("absent"))
            .expect_err("an id with no embedded file errors");
        assert_eq!(
            err,
            PackError::NotFound {
                kind: PackResourceKind::Workflows,
                id: ResourceId::from("absent"),
            },
        );
    }

    #[test]
    fn embedded_pack_version_is_the_binary_version() {
        let pack = EmbeddedPack::new();
        assert_eq!(pack.pack_version(), env!("CARGO_PKG_VERSION"));
    }

    /// Read a resource as UTF-8 text (pack definitions are text).
    fn read_text(pack: &EmbeddedPack, kind: PackResourceKind, id: &str) -> String {
        let bytes = pack
            .read(kind, &ResourceId::from(id))
            .unwrap_or_else(|e| panic!("resource `{id}` must read back: {e}"));
        String::from_utf8(bytes).expect("pack resources are UTF-8 text")
    }

    /// The single-task workflow ships the full MVP definition: spec-less `when`,
    /// `creates-task: true`, the `{type: adr, as: decision}` create-gate, and a
    /// body that is exactly the four ordered step includes. Golden over the bytes
    /// pins the canonical pack content (no serializer here — the file *is* the
    /// contract). See workflow-dialect.md → On-disk definition format.
    #[test]
    fn single_task_workflow_body_is_the_canonical_definition() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Workflows, "single-task");
        insta::assert_snapshot!(body, @r###"
        ---
        when: implement one scoped change end-to-end
        creates-task: true
        allows-create: [{type: adr, as: decision}]
        ---
        {{ include: step:locate }}
        {{ include: step:implement }}
        {{ include: step:superseded-context }}
        {{ include: step:finalize }}
        "###);
    }

    /// list(Steps) yields the MVP step ids, sorted (the pack lists in stem
    /// order). The composer's includes resolve against exactly these — the four
    /// `single-task` steps plus `implement-quick`, the ADR-free variant `quick-fix`
    /// includes.
    #[test]
    fn embedded_pack_lists_the_mvp_steps() {
        let pack = EmbeddedPack::new();
        let steps = pack.list(PackResourceKind::Steps);
        assert_eq!(
            steps,
            vec![
                ResourceId::from("finalize"),
                ResourceId::from("implement"),
                ResourceId::from("implement-quick"),
                ResourceId::from("locate"),
                ResourceId::from("superseded-context"),
            ],
        );
    }

    #[test]
    fn step_locate_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "locate");
        insta::assert_snapshot!(body, @r###"
        Reason about the change. The intent is:
        {{ task.intent }}

        The relevant code paths are not yet known. Inspect the codebase to confirm
        scope before implementing.
        "###);
    }

    #[test]
    fn step_implement_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "implement");
        insta::assert_snapshot!(body, @r###"
        Implement the change directly in the working tree. When done, stage the
        commit prose:

        {{ cli.set-commit-summary }}
        <<author: {{ task.commit#summary }}>>

        If a decision is warranted, create an ADR and author its slots:

        {{ cli.create-adr }}
        "###);
    }

    #[test]
    fn step_superseded_context_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "superseded-context");
        insta::assert_snapshot!(body, @r###"
        If your decision supersedes an earlier one, here is that decision for
        reference — make your consequences explain what changes:
        {{ @task.decision.supersedes#decision }}
        "###);
    }

    #[test]
    fn step_finalize_body_is_canonical() {
        let pack = EmbeddedPack::new();
        let body = read_text(&pack, PackResourceKind::Steps, "finalize");
        insta::assert_snapshot!(body, @r###"
        Validate and commit the task as one logical commit:

        {{ cli.finalize-task }}
        "###);
    }

    /// list(Config) carries the `defaults` resource whose `default-workflow`
    /// points at `single-task` (the MVP cascade knob — CLAUDE.md → MVP scope),
    /// plus the `commands` catalog. The composer reads both.
    #[test]
    fn embedded_pack_config_carries_defaults_and_commands() {
        let pack = EmbeddedPack::new();
        let config = pack.list(PackResourceKind::Config);
        assert!(
            config.contains(&ResourceId::from("defaults")),
            "the pack config layer must ship a `defaults` resource; got {config:?}",
        );
        assert!(
            config.contains(&ResourceId::from("commands")),
            "the pack config layer must ship the `commands` catalog; got {config:?}",
        );

        let defaults = read_text(&pack, PackResourceKind::Config, "defaults");
        insta::assert_snapshot!(defaults, @r###"
        pack-id: dev
        default-workflow: single-task
        "###);

        // The commands catalog is present and readable (its parser arrives with
        // the composer; here we only pin its presence + canonical bytes).
        let commands = read_text(&pack, PackResourceKind::Config, "commands");
        assert!(
            commands.contains("set-commit-summary"),
            "the commands catalog must define the workflow's command-refs; got:\n{commands}",
        );
    }

    /// The pack-default config layer declares the pack's own identity: its
    /// `pack-id` is `dev`. This is what makes the `Pack: dev/<version>`
    /// provenance segment cascade-sourced rather than a CLI constant — the pack
    /// names itself. See overrides.md → pack-default layer carries pack id.
    #[test]
    fn embedded_pack_config_declares_pack_id() {
        let pack = EmbeddedPack::new();
        let defaults = read_text(&pack, PackResourceKind::Config, "defaults");
        assert!(
            defaults.lines().any(|l| l.trim() == "pack-id: dev"),
            "the pack config must declare `pack-id: dev`; got:\n{defaults}",
        );
    }
}
