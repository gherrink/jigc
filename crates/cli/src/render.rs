//! Renderers over the engine's result types: agent-text (default), json
//! (generic via serde), human-pretty. The TUI is a post-MVP `render::tui`.
//!
//! See `implementation/module-layout.md` → Renderers.
//!
//! JSON is **generic** over `serde` (one function over any `Serialize` result,
//! no per-type code); agent-text is **per-type** here. The routing footer rides
//! only on agent-text / human output — never on JSON (consumed by tooling, not
//! the agent's reading flow). See `design/bootstrap.md` → Context compaction
//! resilience and `design/workflow-dialect.md` → Routing footer.

use crate::cli::Format;
use crate::setup::SetupSummary;
use engine::compose::ComposedWorkflow;
use engine::finding::{Finding, Severity};
use engine::result::{Orientation, OrientationView, ResolutionTree, ValidationReport};
use serde::Serialize;

/// The one-line routing footer appended to every agent-text / human CLI output.
/// Self-reinforcing: every CLI call re-shows the routing pointer, so context
/// compaction can drop one bootstrap injection but not the footer the agent
/// re-sees on each call. Verbatim from `design/workflow-dialect.md` → Routing
/// footer.
pub const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// Render an orientation result to the surface `format` selects: `agent` /
/// `human` map to the per-state agent-text (with the routing footer); `json`
/// maps to the **generic** JSON renderer (no footer). This is the one
/// `Format → renderer` mapping for orientation (`implementation/module-layout.md`
/// → Renderers / Format selection). MVP human-pretty is agent-text + light
/// styling, so it currently renders identically to agent (the TUI is post-MVP).
pub fn orientation(format: Format, view: &OrientationView) -> String {
    match format {
        Format::Json => json(view),
        Format::Agent | Format::Human => match view {
            OrientationView::UnsetProject { .. } => orientation_unset(),
            OrientationView::Clean {
                header, workflows, ..
            } => orientation_clean(header, &Orientation::new(workflows.clone())),
        },
    }
}

/// Render any `Serialize` result type to pretty JSON — the **generic** JSON
/// renderer. Carries no routing footer (JSON is consumed by tooling, not the
/// agent's reading flow).
pub fn json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect("engine result types serialize")
}

/// Render an `Orientation` to **agent-text**: one line per workflow (`id — when`)
/// followed by the routing footer.
///
/// Superseded for orientation by [`orientation_clean`] (which adds the
/// provenance header + `Available workflows:` framing per `design/bootstrap.md`);
/// retained as the leaner per-entry shape for callers that want just the list.
#[allow(dead_code)]
pub fn orientation_agent_text(orientation: &Orientation) -> String {
    let mut out = String::new();
    for entry in orientation.workflows.entries() {
        out.push_str(entry.id.as_str());
        out.push_str(" — ");
        out.push_str(entry.when.as_str());
        out.push('\n');
    }
    out.push_str(ROUTING_FOOTER);
    out
}

/// Render the **clean, no active task** orientation (`design/bootstrap.md` →
/// Orientation output examples, state 2): the cascade/provenance `header`, the
/// available-workflows catalog (`  - id — when` per entry), the `jigc start`
/// next-step directive, then the universal routing footer.
///
/// Branch/HEAD and the "Recent: …" finalization line from the design example are
/// deferred — they need git-HEAD inspection and task state, neither of which
/// exists in increment 1 (bare `start` is read-only, no task store yet).
pub fn orientation_clean(header: &str, orientation: &Orientation) -> String {
    let mut out = String::from("jigc — orientation\n\n");
    out.push_str(header);
    out.push_str("\n\nAvailable workflows:\n");
    for entry in orientation.workflows.entries() {
        out.push_str("  - ");
        out.push_str(entry.id.as_str());
        out.push_str(" — ");
        out.push_str(entry.when.as_str());
        out.push('\n');
    }
    out.push_str(
        "\nRun: `jigc start \"<intent>\"`   — routes among the workflows above; pick one, then re-run with `--workflow <chosen>`\n",
    );
    out.push_str(ROUTING_FOOTER);
    out
}

/// Render a composed workflow to the surface `format` selects: `agent` / `human`
/// emit the engine's four-class composed text followed by the routing footer
/// (`design/workflow-dialect.md` → Routing footer — every composed workflow
/// output in agent-text and human-pretty ends with the one-line footer); `json`
/// emits the **generic** JSON projection of the [`ComposedWorkflow`] with **no**
/// footer (consumed by tooling, not the agent's reading flow).
///
/// The footer is appended here, in the frontend — never by the engine, which
/// stays presentation-free (the engine view carries the footerless text). The
/// composed text already ends with a trailing newline; the footer follows it on
/// its own line.
///
/// Wired into the `jigc start "<intent>"` dispatch (`crate::cli::run_compose`),
/// which mints a task and emits this composed view.
pub fn composed(format: Format, view: &ComposedWorkflow) -> String {
    match format {
        Format::Json => json(view),
        Format::Agent | Format::Human => {
            let mut out = String::with_capacity(view.text.len() + ROUTING_FOOTER.len() + 1);
            out.push_str(&view.text);
            if !view.text.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render the `--explain` [`ResolutionTree`] to the surface `format` selects:
/// `agent` / `human` emit the agent-text tree (the workflow line with its winning
/// layer + pack label, the `overrides applied: N` line, the resolved include list
/// with each step's source layer and any `← replaces … at position` annotation),
/// followed by the routing footer; `json` emits the **generic** JSON projection of
/// the tree with **no** footer (tooling-consumed). The `pack_label`
/// (`<pack-id>/v<version>`) is CLI-side framing — the engine tree carries only the
/// structural fact (which layer won), not the displayed label
/// (`design/workflow-dialect.md` → `--explain` output contract; `design/worked-
/// examples.md` → 3a). Sibling of [`composed`].
pub fn explain(format: Format, tree: &ResolutionTree, pack_label: &str) -> String {
    match format {
        Format::Json => json(tree),
        Format::Agent | Format::Human => {
            let mut out = explain_agent_text(tree, pack_label);
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The agent-text body of the `--explain` tree (footer appended by [`explain`]).
/// The workflow line names the resolved workflow + its winning layer (with the
/// pack label), the `overrides applied:` line reports the **total** override count —
/// structural deltas plus applied scalar-key overrides — (`none` when zero, the
/// worked-example spelling), one `<key> = <value>  (<layer>)` line per applied
/// scalar-key override (layer 1 of the output contract — `design/workflow-dialect.md`
/// → "any scalar-key overrides applied with their source layer"), and one line per
/// resolved include carrying the step's id, its source layer, and — for a
/// `replace-step` slot — the `← replaces <id> at position N` annotation.
fn explain_agent_text(tree: &ResolutionTree, pack_label: &str) -> String {
    let mut out = format!(
        "workflow:{}    ({} · {pack_label})\n",
        tree.workflow,
        tree.workflow_layer.label(),
    );
    let total_overrides = tree.overrides_applied;
    if total_overrides == 0 {
        out.push_str("  overrides applied: none\n");
    } else {
        out.push_str(&format!("  overrides applied: {total_overrides}\n"));
    }
    for scalar in &tree.scalar_overrides {
        out.push_str(&format!(
            "    {} = {}    ({})\n",
            scalar.key,
            scalar.value,
            scalar.layer.label(),
        ));
    }
    out.push_str("  includes:\n");
    for step in &tree.steps {
        out.push_str(&format!("    step:{}    ({})", step.id, step.layer.label(),));
        if let Some(replacement) = &step.replaces {
            out.push_str(&format!(
                "  ← replaces step:{} at position {}",
                replacement.replaced, replacement.position,
            ));
        }
        out.push('\n');
    }
    out
}

/// Render a [`ValidationReport`] to the surface `format` selects: `agent` / `human`
/// emit one line per finding (`severity · code — message`, with an indented
/// `route:` line where the finding carries one) followed by the routing footer;
/// `json` emits the **generic** JSON projection of the report with **no** footer
/// (consumed by tooling, not the agent's reading flow). A clean report renders a
/// single `no findings` line so the agent sees a positive signal.
///
/// This is the `task validate` view (`design/validation.md` → How it gates
/// `finalize`: validate previews what finalize blocks on). The exit code — which
/// tracks `report.has_blocking()` — is the dispatcher's concern, not the renderer's.
pub fn validation(format: Format, report: &ValidationReport) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = String::new();
            if report.findings.is_empty() {
                out.push_str("no findings — the task validates clean\n");
            } else {
                for finding in &report.findings {
                    out.push_str(&finding_line(finding));
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// One agent-text finding line: `<severity> · <code> — <message>`, plus an indented
/// `route:` line when the finding carries a repair direction (the settled
/// block-payload envelope — a hard block is a blocking finding carrying a route).
fn finding_line(finding: &Finding) -> String {
    let severity = match finding.severity {
        Severity::Blocking => "blocking",
        Severity::Warning => "warning",
        Severity::Advisory => "advisory",
    };
    let mut line = format!("{severity} · {} — {}\n", finding.code, finding.message);
    if let Some(route) = &finding.route {
        line.push_str("  route: ");
        line.push_str(route);
        line.push('\n');
    }
    line
}

/// Render a successful `jigc setup` install to the surface `format` selects:
/// `agent` / `human` emit a one-line-per-target summary of what was installed,
/// followed by the routing footer; `json` emits a generic object naming the two
/// host targets, with no footer (tooling-consumed). The agent-text summary tells
/// the agent the install is done and where it landed.
pub fn setup_success(format: Format, summary: &SetupSummary) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "installed": true,
            "line_file": summary.line_file,
            "allowlist_file": summary.allowlist_file,
        })),
        Format::Agent | Format::Human => {
            let mut out = String::from("jigc setup — adapter installed\n\n");
            out.push_str("  - bootstrap reference → ");
            out.push_str(&summary.line_file);
            out.push('\n');
            out.push_str("  - jigc allowlist → ");
            out.push_str(&summary.allowlist_file);
            out.push('\n');
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc setup` failure to the surface `format` selects: `agent` /
/// `human` emit the blocking finding line (`severity · code — message` + the
/// indented `route:` line) followed by the routing footer; `json` emits the
/// **generic** finding projection (the stable block-payload envelope) with no
/// footer. A hard block is a blocking finding carrying a route (`DECISIONS.md`
/// 2026-05-31), so this is the same envelope `validate` blocks surface through.
pub fn setup_block(format: Format, finding: &Finding) -> String {
    match format {
        Format::Json => json(finding),
        Format::Agent | Format::Human => {
            let mut out = finding_line(finding);
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render the **unset project** orientation (`design/bootstrap.md` → Orientation
/// output examples, state 1): no project layer is set up, so route the agent to
/// `jigc setup` and end with the universal routing footer.
pub fn orientation_unset() -> String {
    let mut out = String::from("jigc — orientation\n\n");
    out.push_str(
        "This project isn't set up. No domain pack is installed; the cascade has only engine defaults.\n\n",
    );
    out.push_str("Run: `jigc setup`\n\n");
    out.push_str(
        "The setup workflow walks the pack choice, the project config dir, and the first workflow.\n",
    );
    out.push_str(ROUTING_FOOTER);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::result::{Catalog, CatalogEntry, Orientation};

    fn fixture() -> Orientation {
        Orientation::new(Catalog::new(vec![
            CatalogEntry::new("single-task", "Implement one well-scoped change."),
            CatalogEntry::new(
                "project-setup",
                "Set up the development pack on a fresh repo.",
            ),
        ]))
    }

    /// The agent-text rendering lists each workflow with its `when` hint and ends
    /// with the routing footer line.
    #[test]
    fn render_orientation_agent_text_ends_with_routing_footer() {
        let text = orientation_agent_text(&fixture());

        insta::assert_snapshot!(text, @r"
        single-task — Implement one well-scoped change.
        project-setup — Set up the development pack on a fresh repo.
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        assert!(text.ends_with(ROUTING_FOOTER));
    }

    /// The clean-no-task orientation (state 2) prints the provenance header, the
    /// available-workflows catalog with each `when` hint, the `jigc start`
    /// next-step directive, and ends with the routing footer. Post-flip the
    /// next-step leads to the **routing** flow (`jigc start "<intent>"` composes
    /// the router, which routes among the workflows above), not a direct
    /// single-task mint (`workflow-dialect.md` → Workflow selection).
    #[test]
    fn render_orientation_clean_has_header_catalog_and_footer() {
        let text = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &fixture(),
        );

        insta::assert_snapshot!(text, @r#"
        jigc — orientation

        Pack: dev/v0.3.0 · Project config: .jigc/config

        Available workflows:
          - single-task — Implement one well-scoped change.
          - project-setup — Set up the development pack on a fresh repo.

        Run: `jigc start "<intent>"`   — routes among the workflows above; pick one, then re-run with `--workflow <chosen>`
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        "#);

        // The next-step leads to the routing flow, not a direct single-task mint.
        assert!(
            text.contains("routes among the workflows above"),
            "got:\n{text}",
        );
        assert!(
            !text.contains("default workflow (single-task)"),
            "post-flip orientation must not claim a direct single-task mint; got:\n{text}",
        );
        assert!(text.ends_with(ROUTING_FOOTER));
    }

    /// The unset-project orientation (state 1) routes the agent to `jigc setup`
    /// and ends with the routing footer.
    #[test]
    fn render_orientation_unset_routes_to_setup() {
        let text = orientation_unset();

        insta::assert_snapshot!(text, @r"
        jigc — orientation

        This project isn't set up. No domain pack is installed; the cascade has only engine defaults.

        Run: `jigc setup`

        The setup workflow walks the pack choice, the project config dir, and the first workflow.
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        assert!(text.ends_with(ROUTING_FOOTER));
    }

    /// A composed workflow rendered to agent-text ends with the routing footer
    /// (`design/workflow-dialect.md` → Routing footer), appended in the frontend
    /// after the engine's footerless four-class text. JSON carries none.
    #[test]
    fn render_composed_agent_text_ends_with_routing_footer() {
        let view = ComposedWorkflow {
            text: "Reason about the change.\nRun: `jigc task finalize add-rate-limiter`\n"
                .to_string(),
        };

        let agent = composed(Format::Agent, &view);
        assert!(agent.ends_with(ROUTING_FOOTER));
        assert!(agent.contains("Run: `jigc task finalize add-rate-limiter`"));

        // Human renders identically to agent in the MVP (TUI is post-MVP).
        assert_eq!(composed(Format::Human, &view), agent);

        // JSON is the generic projection of the result type — no footer.
        let json_out = composed(Format::Json, &view);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: ComposedWorkflow = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, view);
    }

    /// The `--explain` tree renders to agent-text as the workflow line (winning
    /// layer + pack label), the `overrides applied: N` line, and one line per
    /// resolved include with its source layer + any `← replaces … at position`
    /// annotation, ending with the routing footer; JSON is the generic projection
    /// with no footer and the same provenance.
    #[test]
    fn render_explain_tree_agent_text_and_json() {
        use engine::cascade::LayerKind;
        use engine::result::{Replacement, ResolutionTree, ResolvedStep, ScalarOverride};

        let tree = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            // overrides_applied is the total: 1 structural (the replace below) + 1 scalar.
            2,
            vec![ScalarOverride {
                key: "default-workflow".to_string(),
                value: "single-task".to_string(),
                layer: LayerKind::Project,
            }],
            vec![
                ResolvedStep {
                    id: "locate".to_string(),
                    layer: LayerKind::PackDefault,
                    replaces: None,
                },
                ResolvedStep {
                    id: "project-implement".to_string(),
                    layer: LayerKind::Project,
                    replaces: Some(Replacement {
                        replaced: "implement".to_string(),
                        position: 2,
                    }),
                },
            ],
        );

        let agent = explain(Format::Agent, &tree, "dev/v0.0.0");
        assert!(
            agent.contains("workflow:single-task    (pack-default · dev/v0.0.0)"),
            "got:\n{agent}",
        );
        // One structural delta + one scalar override fold into the total count.
        assert!(agent.contains("overrides applied: 2"), "got:\n{agent}");
        // The per-knob scalar-override line carries the key, value, and winning layer.
        assert!(
            agent.contains("default-workflow = single-task    (project)"),
            "got:\n{agent}",
        );
        assert!(
            agent.contains("step:locate    (pack-default)"),
            "got:\n{agent}",
        );
        assert!(
            agent.contains(
                "step:project-implement    (project)  ← replaces step:implement at position 2"
            ),
            "got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER), "got:\n{agent}");

        // A no-override tree spells the count `none`.
        let clean = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(explain(Format::Agent, &clean, "dev/v0.0.0").contains("overrides applied: none"),);

        // JSON is the generic projection — parseable, same provenance, no footer.
        let json_out = explain(Format::Json, &tree, "dev/v0.0.0");
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: ResolutionTree = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, tree);
    }

    /// The JSON rendering of the same value is valid JSON of the result type and
    /// carries no footer.
    #[test]
    fn render_orientation_json_is_valid_and_carries_no_footer() {
        let rendered = json(&fixture());

        insta::assert_snapshot!(rendered, @r#"
        {
          "schema_version": 1,
          "workflows": [
            {
              "id": "single-task",
              "when": "Implement one well-scoped change."
            },
            {
              "id": "project-setup",
              "when": "Set up the development pack on a fresh repo."
            }
          ]
        }
        "#);

        // Valid JSON that round-trips back to the result type.
        let back: Orientation = serde_json::from_str(&rendered).expect("valid JSON");
        assert_eq!(back, fixture());

        // No footer in JSON output.
        assert!(!rendered.contains(ROUTING_FOOTER));
    }

    /// A blocking [`ValidationReport`] renders one `severity · code — message` line
    /// per finding (with an indented `route:` line where present) and ends with the
    /// routing footer; the clean report renders a positive `no findings` line + the
    /// footer. JSON is the generic projection with no footer.
    #[test]
    fn render_validation_lists_findings_or_clean_and_footers_agent_text() {
        use engine::finding::{Finding, Location, Severity};

        let report = ValidationReport::new(vec![
            Finding::graded(
                Severity::Blocking,
                "file-state.hash-matches",
                "on-disk content of `docs/commit:x.md` differs",
                Some(Location::addressed("docs/commit:x.md", 1, 1)),
                Some("reconcile docs/commit:x.md".into()),
            ),
            Finding::graded(
                Severity::Blocking,
                "schema-conformance.required-slot-present",
                "required slot in section `summary` is empty",
                None,
                None,
            ),
        ]);

        let agent = validation(Format::Agent, &report);
        insta::assert_snapshot!(agent, @r"
        blocking · file-state.hash-matches — on-disk content of `docs/commit:x.md` differs
          route: reconcile docs/commit:x.md
        blocking · schema-conformance.required-slot-present — required slot in section `summary` is empty
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        assert_eq!(validation(Format::Human, &report), agent);

        // A clean report renders the positive line + the footer.
        let clean = validation(Format::Agent, &ValidationReport::new(Vec::new()));
        insta::assert_snapshot!(clean, @r"
        no findings — the task validates clean
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        // JSON is the generic projection of the report — no footer.
        let json_out = validation(Format::Json, &report);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: ValidationReport = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, report);
    }
}
