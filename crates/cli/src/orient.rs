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
use crate::pack::EmbeddedPack;
use anyhow::Result;
use engine::cascade::{self, OverrideLayer, PackDefaultLayer};
use engine::catalog::build_catalog;
use engine::packsource::{PackResourceKind, PackSource};
use engine::result::OrientationView;
use std::collections::BTreeMap;
use std::path::Path;

/// The embedded dev pack's cascade id — the `Pack: dev/<version>` provenance
/// segment. The built-in pack has no `config/` layer declaring its own id yet
/// (increment 6), so the CLI names it here; the version comes from the pack.
const DEV_PACK_ID: &str = "dev";

/// Produce the structured bare-`start` orientation result for the repo `start`
/// is run from.
///
/// Locates the cascade layers, then branches on whether the project layer is
/// present: absent → the unset-project view; present → resolve the cascade over
/// the embedded pack and build the clean-no-task view. Presentation-free — the
/// CLI maps the returned [`OrientationView`] to a surface via `Format → render`.
pub fn orient(start: &Path) -> Result<OrientationView> {
    let ctx = locate::locate(start)?;
    let pack = EmbeddedPack::new();
    orient_with(&ctx, &pack)
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
    let pack_default = PackDefaultLayer::new(
        DEV_PACK_ID,
        pack.pack_version(),
        BTreeMap::new(),
        pack_files,
    );
    let project = OverrideLayer::empty().config_path(project_config.display().to_string());
    let resolved = cascade::resolve(&pack_default, None, Some(&project))?;

    let catalog = build_catalog(pack)?;

    Ok(OrientationView::clean(
        resolved.provenance().header(),
        catalog,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::packsource::{PackError, ResourceId};
    use std::path::PathBuf;

    /// A `PackSource` that serves one fenced-front-matter workflow, so the
    /// orient core can be exercised without the binary-embedded pack.
    struct FakePack;

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            "v0.3.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            match kind {
                PackResourceKind::Workflows => vec![ResourceId::from("single-task")],
                _ => Vec::new(),
            }
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            if kind == PackResourceKind::Workflows && id.as_str() == "single-task" {
                return Ok(b"---\nwhen: implement one scoped change end-to-end\n---\n".to_vec());
            }
            Err(PackError::NotFound {
                kind,
                id: id.clone(),
            })
        }
    }

    fn ctx(project_config: Option<PathBuf>) -> RunContext {
        RunContext {
            repo_root: PathBuf::from("/repo"),
            project_config,
            team_config: PathBuf::from("/home/u/.config/jigc"),
        }
    }

    #[test]
    fn absent_project_layer_yields_unset_project_view() {
        let view = orient_with(&ctx(None), &FakePack).expect("orient succeeds");
        assert_eq!(view, OrientationView::unset_project());
    }

    #[test]
    fn present_project_layer_yields_clean_view_with_header_and_catalog() {
        let view = orient_with(&ctx(Some(PathBuf::from("/repo/.jigc/config"))), &FakePack)
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
}
