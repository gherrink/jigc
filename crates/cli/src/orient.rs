//! Bare `jigc start` orientation (read-only) — the end-to-end handler.
//!
//! Wires the CLI-locates / engine-resolves spine for the two increment-1
//! orientation states (`design/bootstrap.md` → Orientation output examples):
//!   1. **unset project** — no project cascade layer (`.jigc/config/` absent):
//!      route the agent to `jigc setup`.
//!   2. **clean, no active task** — project layer present: resolve the cascade,
//!      build the workflow catalog from the embedded pack, and render the
//!      provenance header + catalog + routing footer.
//!
//! Read-only by construction: it locates, resolves, and renders — it never
//! writes, composes, or shells out to git (the `.git` marker is only *read* by
//! repo-root discovery). See `implementation/module-layout.md` → The I/O
//! boundary (CLI locates, engine resolves).

use crate::locate::{self, RunContext};
use crate::pack::make_pack;
use crate::start::selectable_workflows;
use anyhow::{Context, Result};
use engine::cascade::{self, OverrideLayer, PackDefaultLayer};
use engine::knobs::load_knobs;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::result::{Catalog, OrientationView};
use std::collections::BTreeMap;
use std::path::Path;

/// The pack-default config key that carries the pack's own cascade id — the
/// `Pack: <pack-id>/<version>` provenance segment. The pack names itself
/// (`overrides.md` → pack-default layer carries pack id); the version comes from
/// the pack binary.
const PACK_ID_KEY: &str = "pack-id";

/// Produce the structured bare-`start` orientation result for the repo `start`
/// is run from.
///
/// Locates the cascade layers, then branches on whether the project layer is
/// present: absent → the unset-project view; present → resolve the cascade over
/// the embedded pack and build the clean-no-task view. Presentation-free — the
/// CLI maps the returned [`OrientationView`] to a surface via `Format → render`.
pub fn orient(start: &Path) -> Result<OrientationView> {
    let ctx = locate::locate(start)?;
    let pack = make_pack();
    orient_with(&ctx, pack.as_ref())
}

/// The testable core of [`orient`]: build the orientation view from an
/// already-located [`RunContext`] and a [`PackSource`].
fn orient_with(ctx: &RunContext, pack: &dyn PackSource) -> Result<OrientationView> {
    let Some(project_config) = &ctx.project_config else {
        // State 1 — unset project: no project cascade layer is present.
        return Ok(OrientationView::unset_project());
    };

    // State 2 — clean, no active task: resolve the cascade and build the catalog.
    let pack_files: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .into_iter()
        .map(|id| id.as_str().to_owned())
        .collect();
    let pack_id = pack_id_from_config(pack)?;
    let scalars = pack_default_scalars(pack)?;
    let pack_default = PackDefaultLayer::new(pack_id, pack.pack_version(), scalars, pack_files);
    let project = OverrideLayer::empty().config_path(project_config.display().to_string());
    let resolved = cascade::resolve(&pack_default, None, Some(&project))?;

    // Orientation lists only the **selectable** (`creates-task: true`) work-
    // workflows — the same increment-1 filter the front door feeds the router's
    // `{{catalog}}` — so a bare-`start` reader sees the workflows a router selects
    // among, never the router itself (`workflow-dialect.md` → Workflow selection).
    let catalog = Catalog::new(selectable_workflows(pack)?);

    Ok(OrientationView::clean(
        compose_pack_header(pack, resolved.provenance()),
        catalog,
    ))
}

/// Render the orientation provenance header with the **composed-set** `Pack:`
/// segment substituted for the engine's single-pack one. The engine
/// [`Provenance`](engine::cascade::Provenance) carries only the precedence-winner's
/// `Pack: <id>/<version>` (the cascade resolves over one merged base); the
/// multi-pack provenance — every composed pack, highest-precedence first, joined by
/// ` | ` — is sourced from the pack-set itself ([`PackSource::provenance_segments`]).
/// The config-path tail the engine appended is preserved unchanged.
///
/// A single-pack set composes one segment, so `multi == single` and the header is
/// byte-identical to today (the floor on the provenance axis).
fn compose_pack_header(pack: &dyn PackSource, provenance: &engine::cascade::Provenance) -> String {
    let multi = pack
        .provenance_segments()
        .into_iter()
        .map(|(id, version)| format!("{id}/{version}"))
        .collect::<Vec<_>>()
        .join(" | ");
    let full = provenance.header();
    let tail = full
        .strip_prefix(&provenance.pack_segment())
        .unwrap_or(&full);
    format!("Pack: {multi}{tail}")
}

/// Read the pack's own cascade id from its `config/defaults` `pack-id` field.
/// The pack-default layer carries its identity, so the provenance header's pack
/// segment is cascade-sourced, never a CLI constant (`overrides.md`).
fn pack_id_from_config(pack: &dyn PackSource) -> Result<String> {
    let bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("defaults"))
        .context("the pack must ship a `config/defaults` resource")?;
    let text = String::from_utf8(bytes).context("`config/defaults` is not UTF-8")?;
    let value: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).context("`config/defaults` is not valid YAML")?;
    value
        .get(PACK_ID_KEY)
        .and_then(serde_yaml_ng::Value::as_str)
        .map(str::to_owned)
        .with_context(|| format!("`config/defaults` declares no `{PACK_ID_KEY}`"))
}

/// Seed the pack-default layer's scalar surface from the pack's `config/knobs`
/// declaration: the closed key set plus each knob's materialized default. This is
/// the base map the cascade resolves over (`overrides.md` → Scalar knobs: the
/// loader builds the pack-default layer's scalar surface). It replaces the empty
/// feed orientation used before the knob surface existed, so a no-override
/// `resolved.scalar(...)` returns the declared default rather than `None`.
fn pack_default_scalars(pack: &dyn PackSource) -> Result<BTreeMap<String, String>> {
    let bytes = pack
        .read(PackResourceKind::Config, &ResourceId::from("knobs"))
        .context("the pack must ship a `config/knobs` declaration")?;
    let knobs = load_knobs(&bytes).context("`config/knobs` is malformed")?;
    Ok(knobs.base_scalars())
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::packsource::{PackError, ResourceId};
    use std::path::PathBuf;

    /// A `PackSource` that serves one fenced-front-matter workflow plus a
    /// `config/defaults` declaring its own `pack-id`, so the orient core can be
    /// exercised without the binary-embedded pack. The `pack_id` field lets a
    /// test vary the declared id and assert the provenance header tracks config.
    struct FakePack {
        pack_id: &'static str,
    }

    impl FakePack {
        fn new() -> Self {
            FakePack { pack_id: "dev" }
        }

        fn with_pack_id(pack_id: &'static str) -> Self {
            FakePack { pack_id }
        }
    }

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            "v0.3.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            match kind {
                // Post-flip shape: the `creates-task: false` router alongside the
                // two selectable (`creates-task: true`) work-workflows, in list
                // order, so the orientation filter can be exercised.
                PackResourceKind::Workflows => vec![
                    ResourceId::from("router"),
                    ResourceId::from("single-task"),
                    ResourceId::from("quick-fix"),
                ],
                PackResourceKind::Config => {
                    vec![ResourceId::from("defaults"), ResourceId::from("knobs")]
                }
                _ => Vec::new(),
            }
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            match (kind, id.as_str()) {
                (PackResourceKind::Workflows, "router") => {
                    Ok(b"---\nwhen: help me pick a workflow\ncreates-task: false\n---\n".to_vec())
                }
                (PackResourceKind::Workflows, "single-task") => Ok(
                    b"---\nwhen: implement one scoped change end-to-end\ncreates-task: true\n---\n"
                        .to_vec(),
                ),
                (PackResourceKind::Workflows, "quick-fix") => {
                    Ok(b"---\nwhen: a small focused fix\ncreates-task: true\n---\n".to_vec())
                }
                (PackResourceKind::Config, "defaults") => Ok(format!(
                    "pack-id: {}\ndefault-workflow: router\n",
                    self.pack_id
                )
                .into_bytes()),
                (PackResourceKind::Config, "knobs") => Ok(format!(
                    "default-workflow:\n  type: enum\n  of: [router, single-task, quick-fix]\n  default: router\n{}",
                    crate::pack::intrinsic_knobs_yaml(),
                )
                .into_bytes()),
                _ => Err(PackError::NotFound {
                    kind,
                    id: id.clone(),
                }),
            }
        }
    }

    fn ctx(project_config: Option<PathBuf>) -> RunContext {
        RunContext {
            repo_root: PathBuf::from("/repo"),
            project_config,
            team_config: PathBuf::from("/home/u/.config/jigc"),
        }
    }

    /// A two-pack composite over [`FakePack`]s — highest-precedence first. Built
    /// from real `CompositePack` (not a hand-rolled segment list) so the header
    /// test drives the production enumeration, not a reconstruction.
    fn two_pack_composite(high: &'static str, low: &'static str) -> crate::pack::CompositePack {
        crate::pack::CompositePack::new(vec![
            Box::new(FakePack::with_pack_id(high)),
            Box::new(FakePack::with_pack_id(low)),
        ])
    }

    #[test]
    fn absent_project_layer_yields_unset_project_view() {
        let view = orient_with(&ctx(None), &FakePack::new()).expect("orient succeeds");
        assert_eq!(view, OrientationView::unset_project());
    }

    /// Under a two-pack set the `Pack:` header lists **both** packs,
    /// highest-precedence first, joined by ` | ` — the composed-set provenance
    /// (`multi-pack.md` → Provenance: the single segment becomes the composed set).
    /// The config-path tail is preserved unchanged after the multi-segment.
    #[test]
    fn two_pack_set_header_names_both_packs_highest_first() {
        let pack = two_pack_composite("methodology", "dev");
        let view = orient_with(&ctx(Some(PathBuf::from("/repo/.jigc/config"))), &pack)
            .expect("orient succeeds");
        let OrientationView::Clean { header, .. } = &view else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        assert!(
            header.starts_with("Pack: methodology/v0.3.0 | dev/v0.3.0"),
            "got:\n{header}",
        );
        assert!(
            header.contains(" · Project config: /repo/.jigc/config"),
            "the config-path tail must survive after the composed-pack segment; got:\n{header}",
        );
    }

    /// The single-segment floor on the provenance axis: a one-pack composite (the
    /// cold-start shape) renders `Pack: dev/<version>` byte-identical to today —
    /// no trailing ` | `, no second segment (increment-workflow hardening #5: the
    /// feature composed into a context that omits the second pack stays inert).
    #[test]
    fn one_pack_composite_header_is_byte_identical_to_today() {
        let pack = crate::pack::CompositePack::new(vec![Box::new(FakePack::new())]);
        let view = orient_with(&ctx(Some(PathBuf::from("/repo/.jigc/config"))), &pack)
            .expect("orient succeeds");
        let OrientationView::Clean { header, .. } = &view else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        assert_eq!(
            header,
            "Pack: dev/v0.3.0 · Project config: /repo/.jigc/config",
        );
    }

    #[test]
    fn present_project_layer_yields_clean_view_with_header_and_catalog() {
        let view = orient_with(
            &ctx(Some(PathBuf::from("/repo/.jigc/config"))),
            &FakePack::new(),
        )
        .expect("orient succeeds");
        let OrientationView::Clean {
            header, workflows, ..
        } = &view
        else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        // Provenance header: pack segment + the located project config path.
        assert!(header.contains("Pack: dev/v0.3.0"), "got:\n{header}");
        assert!(
            header.contains("Project config: /repo/.jigc/config"),
            "got:\n{header}",
        );
        // Catalog entry with its `when` hint.
        let entry = &workflows.entries()[0];
        assert_eq!(entry.id, "single-task");
        assert_eq!(entry.when, "implement one scoped change end-to-end");
    }

    /// Post-flip orientation lists only the **selectable** (`creates-task: true`)
    /// work-workflows — `single-task` + `quick-fix` — and **not** the router
    /// (`creates-task: false`), so a bare-`start` reader is pointed at the
    /// workflows a router selects among, never at the router itself. Reuses the
    /// increment-1 `creates-task: true` filter rather than re-deriving it
    /// (`workflow-dialect.md` → Workflow selection: the router never lists itself).
    #[test]
    fn orientation_catalog_lists_only_selectable_work_workflows() {
        let view = orient_with(
            &ctx(Some(PathBuf::from("/repo/.jigc/config"))),
            &FakePack::new(),
        )
        .expect("orient succeeds");
        let OrientationView::Clean { workflows, .. } = &view else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        let ids: Vec<&str> = workflows.entries().iter().map(|e| e.id.as_str()).collect();
        assert_eq!(
            ids,
            vec!["single-task", "quick-fix"],
            "orientation must list the selectable work-workflows and not the router",
        );
    }

    /// The provenance header's pack segment is sourced from the pack's own
    /// `config/defaults` `pack-id`, not a CLI constant: a pack that declares
    /// `pack-id: xyz` renders `Pack: xyz/<version>`. This is the read-path wire
    /// that closes the increment-6 hardcode.
    #[test]
    fn pack_segment_is_sourced_from_pack_config() {
        let view = orient_with(
            &ctx(Some(PathBuf::from("/repo/.jigc/config"))),
            &FakePack::with_pack_id("xyz"),
        )
        .expect("orient succeeds");
        let OrientationView::Clean { header, .. } = &view else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        assert!(header.contains("Pack: xyz/v0.3.0"), "got:\n{header}");
    }
}
