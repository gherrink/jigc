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

use engine::result::Orientation;
use serde::Serialize;

/// The one-line routing footer appended to every agent-text / human CLI output.
/// Self-reinforcing: every CLI call re-shows the routing pointer, so context
/// compaction can drop one bootstrap injection but not the footer the agent
/// re-sees on each call. Verbatim from `design/workflow-dialect.md` → Routing
/// footer.
pub const ROUTING_FOOTER: &str =
    "— jigc · run `jigc start` for orientation; all writes through `jigc`.";

/// Render any `Serialize` result type to pretty JSON — the **generic** JSON
/// renderer. Carries no routing footer (JSON is consumed by tooling, not the
/// agent's reading flow).
pub fn json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect("engine result types serialize")
}

/// Render an `Orientation` to **agent-text**: one line per workflow (`id — when`)
/// followed by the routing footer.
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
}
