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
use engine::result::{Orientation, OrientationView, ValidationReport};
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
        "\nRun: `jigc start \"<intent>\"`   — composes the default workflow (single-task)\n",
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
            out.push_str("  - bootstrap line → ");
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
    /// next-step directive, and ends with the routing footer.
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

        Run: `jigc start "<intent>"`   — composes the default workflow (single-task)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        "#);

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
            Finding {
                severity: Severity::Blocking,
                code: "file-state.hash-matches".into(),
                message: "on-disk content of `docs/commit:x.md` differs".into(),
                location: Some(Location::addressed("docs/commit:x.md", 1, 1)),
                route: Some("reconcile docs/commit:x.md".into()),
            },
            Finding {
                severity: Severity::Blocking,
                code: "schema-conformance.required-slot-present".into(),
                message: "required slot in section `summary` is empty".into(),
                location: None,
                route: None,
            },
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
