//! Bare `jigc start` orientation — the end-to-end handler.
//!
//! Wires the CLI-locates / engine-resolves spine for the three orientation states
//! (`design/bootstrap.md` → Orientation output examples):
//!   1. **unset project** — no project cascade layer (`.jigc/config/` absent):
//!      route the agent to `jigc setup`.
//!   2. **clean, no active task** — project layer present and the active set empty:
//!      resolve the cascade, build the workflow catalog from the embedded pack, and
//!      render the provenance header + catalog + routing footer.
//!   3. **active task** — project layer present and at least one task live: the
//!      active set, one row per task, with the routes that finish or abandon it
//!      (`bootstrap.md`'s states 3 and 4, collapsed — M50 → the Settle, D3).
//!
//! **It reports, it never acts**: it mints nothing, stages nothing and composes no
//! side-effectful workflow. It is not, since state 3, *inert*: reporting a live task's
//! findings runs the shipped task-scope sweep, which shells out to `git`, may spawn the
//! `doc-code` probe, and materializes the `.jigc/index/edges.json` cache — a
//! self-healing derived cache that is a pure function of the committed store, which is
//! the one carve-out the read/write rule allows (`crate::cli::VerbKind`; `jigc start` is
//! a [`VerbKind::Write`](crate::cli::VerbKind) verb regardless, since with an intent it
//! mints). A task whose sweep cannot run is reported with its findings **unknown**,
//! never with an error and never as clean: the bootstrap door degrades.
//! See `implementation/module-layout.md` → The I/O boundary (CLI locates, engine
//! resolves).

use crate::locate::{self, RunContext};
use crate::pack::make_pack;
use crate::start::selectable_workflows;
use anyhow::{Context, Result};
use engine::cascade::{self, OverrideLayer, PackDefaultLayer};
use engine::knobs::load_knobs;
use engine::packsource::{PackResourceKind, PackSource, ResourceId};
use engine::result::{ActiveTask, Catalog, NextStep, OrientationView};
use std::collections::BTreeMap;
use std::path::Path;

/// The **off-catalog** entry verbs orientation surfaces as next steps when the
/// composed pack-set provides them — `(id, gist)`, in display order
/// (`design/project-setup.md` → Off-catalog discoverability (G6)). Each is an
/// entry workflow deliberately absent from the selectable `Available workflows:`
/// catalog (`planning` is `selectable: false`, off the router by the M16
/// invariant — *not* flipped; `ingest-existing` is `creates-task: false`), so a
/// bare-`start` reader can find them only if orientation names them. A verb is
/// named **iff** the pack-set ships it (gated below on `pack.list(Workflows)`
/// membership) — naming one unconditionally would route to a non-resolving
/// workflow in a pack-set that omits it (`ingest-existing` ships in the
/// always-present dev pack; `planning` only under the embedded methodology pack).
const OFF_CATALOG_VERBS: &[(&str, &str)] = &[
    (
        "planning",
        "plan a milestone — decompose it into increments and tasks",
    ),
    (
        "ingest-existing",
        "bring an existing repo's docs under management",
    ),
];

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
    let pack = make_pack()?;
    orient_with(&ctx, pack.as_ref())
}

/// The testable core of [`orient`]: build the orientation view from an
/// already-located [`RunContext`] and a [`PackSource`].
fn orient_with(ctx: &RunContext, pack: &dyn PackSource) -> Result<OrientationView> {
    let Some(project_config) = &ctx.project_config else {
        // State 1 — unset project: no project cascade layer is present.
        return Ok(OrientationView::unset_project());
    };

    // States 2 and 3 both carry the provenance header, so the cascade resolves either
    // way; only what rides beside it differs.
    let workflow_ids: Vec<String> = pack
        .list(PackResourceKind::Workflows)
        .into_iter()
        .map(|id| id.as_str().to_owned())
        .collect();
    let pack_files = workflow_ids.clone();
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

    // Off-catalog discoverability (G6): name each off-catalog entry verb **iff**
    // the composed pack-set actually ships it — gated on `pack.list(Workflows)`
    // membership, so a pack-set that omits one (the dev-only floor omits
    // `planning`) routes to no non-resolving verb. The data is presentation-free;
    // `render::orientation_clean` formats the route-prose line per verb.
    let next_steps: Vec<NextStep> = OFF_CATALOG_VERBS
        .iter()
        .filter(|(id, _)| workflow_ids.iter().any(|w| w == id))
        .map(|(id, gist)| NextStep::new(*id, *gist))
        .collect();

    let header = compose_pack_header(pack, resolved.provenance());

    // State 3 — at least one task is live. The active set decides the variant, so the
    // `clean` tag can never be emitted over a repo holding work in progress (D3). The
    // catalog rides it too: `create.gate-blocked` routes with "`jigc start` lists the
    // catalog" and can only fire while a task is live, so dropping it here would break an
    // existing route in exactly the state that prints it.
    let active = active_tasks(ctx);
    if !active.is_empty() {
        return Ok(OrientationView::active_task(
            header, active, catalog, next_steps,
        ));
    }

    Ok(OrientationView::clean(header, catalog, next_steps))
}

/// Build the **active set** — one [`ActiveTask`] row per live task under the project's
/// `.jigc/` home, in the shipped enumerator's stable (sorted-id) order.
///
/// Every row is assembled from enumerators that already ship
/// ([`state::list_active_task_ids`](engine::state::list_active_task_ids) and the three
/// working-area readers beside it, plus [`crate::task::staged_doc_ids`]), so orientation
/// reports the same task state every other door reads rather than a second opinion about
/// it.
///
/// **Nothing here can fail the door.** A working area that will not answer degrades to
/// the absent case for that fact — an unreadable intent to the empty string, an
/// unreadable base pin to [`None`], a sweep that cannot run to `findings: null` plus the
/// reason. A task that exists is always named: a task you cannot see is worse than a pin
/// you cannot read, and this door runs at every `SessionStart`.
fn active_tasks(ctx: &RunContext) -> Vec<ActiveTask> {
    let jigc_root = ctx.jigc_home.join(".jigc");
    engine::state::list_active_task_ids(&jigc_root)
        .into_iter()
        .map(|id| {
            let dir = jigc_root.join("tasks").join(&id);
            let (findings, findings_unavailable) =
                match crate::task::sweep_for_orientation(&ctx.repo_root, &id) {
                    Ok(findings) => (Some(findings), None),
                    Err(reason) => (None, Some(reason)),
                };
            ActiveTask {
                workflow: engine::state::read_workflow_id(&dir).ok().flatten(),
                intent: engine::state::read_intent(&dir).unwrap_or_default(),
                // A milestone's sub-tasks are ordinary areas under `.jigc/tasks/`, so the
                // enumerator above lists them — and a sub-task's commit boundary is the
                // milestone door, `jigc task finalize` refusing outright. Asked here so the
                // renderer can route to the door that runs (`design/surface-contract.md` →
                // the route floor).
                milestone: engine::milestone::owning_milestone(&jigc_root, &id),
                base: engine::state::read_base_pin(&dir).ok(),
                staged: crate::task::staged_doc_ids(&dir.join("docs")).unwrap_or_default(),
                id,
                findings,
                findings_unavailable,
            }
        })
        .collect()
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
            jigc_home: PathBuf::from("/repo"),
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

    /// A `PackSource` that adds the two off-catalog entry verbs (`planning`,
    /// `ingest-existing`) on top of [`FakePack`]'s router + selectable set, so the
    /// next-step gating can be exercised over a pack-set that **provides** them.
    struct OffCatalogPack;

    impl PackSource for OffCatalogPack {
        fn pack_version(&self) -> String {
            "v0.3.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            match kind {
                PackResourceKind::Workflows => vec![
                    ResourceId::from("router"),
                    ResourceId::from("single-task"),
                    ResourceId::from("planning"),
                    ResourceId::from("ingest-existing"),
                ],
                other => FakePack::new().list(other),
            }
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            match (kind, id.as_str()) {
                // `planning` is `creates-task: true` but `selectable: false` — off the
                // selectable catalog by the M16 invariant (named in route-prose only).
                (PackResourceKind::Workflows, "planning") => Ok(
                    b"---\nwhen: plan a milestone\ncreates-task: true\nselectable: false\n---\n"
                        .to_vec(),
                ),
                // `ingest-existing` is `creates-task: false` — orient/route shape.
                (PackResourceKind::Workflows, "ingest-existing") => {
                    Ok(b"---\nwhen: bring an existing repo under management\ncreates-task: false\n---\n".to_vec())
                }
                other => FakePack::new().read(other.0, id),
            }
        }
    }

    /// Off-catalog discoverability (G6): when the pack-set ships `planning` +
    /// `ingest-existing`, the clean view's `next_steps` names BOTH (in display
    /// order), while neither leaks into the selectable `workflows` catalog
    /// (`planning` is `selectable: false`; `ingest-existing` is
    /// `creates-task: false`). The catalog filter and the off-catalog naming are
    /// independent surfaces.
    #[test]
    fn clean_view_next_steps_name_present_off_catalog_verbs() {
        let view = orient_with(
            &ctx(Some(PathBuf::from("/repo/.jigc/config"))),
            &OffCatalogPack,
        )
        .expect("orient succeeds");
        let OrientationView::Clean {
            workflows,
            next_steps,
            ..
        } = &view
        else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        let step_ids: Vec<&str> = next_steps.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(
            step_ids,
            vec!["planning", "ingest-existing"],
            "both present off-catalog verbs are named as next steps, in display order",
        );
        // Neither off-catalog verb leaks into the selectable catalog.
        let catalog_ids: Vec<&str> = workflows.entries().iter().map(|e| e.id.as_str()).collect();
        assert!(
            !catalog_ids.contains(&"planning") && !catalog_ids.contains(&"ingest-existing"),
            "an off-catalog verb must NOT appear in the selectable catalog; got {catalog_ids:?}",
        );
    }

    /// The omitting-context check (increment-workflow hardening #5): a pack-set that
    /// ships **neither** off-catalog verb (the bare [`FakePack`] — router + two
    /// selectable work-workflows only) carries an **empty** `next_steps`, naming no
    /// non-resolving verb. The feature stays inert where the verbs are absent.
    #[test]
    fn clean_view_next_steps_inert_when_off_catalog_verbs_absent() {
        let view = orient_with(
            &ctx(Some(PathBuf::from("/repo/.jigc/config"))),
            &FakePack::new(),
        )
        .expect("orient succeeds");
        let OrientationView::Clean { next_steps, .. } = &view else {
            panic!("expected the clean view, got:\n{view:?}");
        };
        assert!(
            next_steps.is_empty(),
            "a pack-set omitting the off-catalog verbs names none; got {next_steps:?}",
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
