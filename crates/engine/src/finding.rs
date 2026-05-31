//! The one finding shape — the conformance / diagnostic / block envelope.
//!
//! "One finding shape, two producers" ([implementation/parsing.md](../../../implementation/parsing.md)
//! → Conformance diagnostics): conformance diagnostics (the parser, this increment)
//! and, later, graded validation findings + hard blocks all surface through this
//! single envelope, so the agent and the renderers learn one shape. The layers
//! differ in *how* severity is assigned — conformance is binary/intrinsic, validation
//! is cascade-tunable ([design/validation.md](../../../design/validation.md), referenced,
//! not redefined) — but the carried shape is identical.
//!
//! A hard block is **not** a separate type: it is a [`Severity::Blocking`] finding
//! that carries a [`Finding::route`] directing the agent's next action (the settled
//! inc-4 block-payload gate, `DECISIONS.md` 2026-05-31).
//!
//! As an engine result type, [`Finding`]'s JSON projection is a stable contract:
//! field names are pinned with explicit `serde` attributes (the renderer / JSON /
//! future-MCP surface, like `result`). The pinned shape — a blocking finding with a
//! located message and no route — projects to:
//!
//! ```json
//! { "severity": "blocking", "code": "…", "message": "…",
//!   "location": { "line": 1, "col": 1 }, "route": null }
//! ```

use serde::{Deserialize, Serialize};

/// How serious a [`Finding`] is.
///
/// For *conformance* diagnostics (the parser) severity is always
/// [`Severity::Blocking`] — a malformed file is malformed regardless of project
/// policy ([parsing.md](../../../implementation/parsing.md) → Conformance is binary
/// and intrinsic). For *validation* findings the engine assigns the final severity
/// from the cascade ([validation.md](../../../design/validation.md) → severity is
/// engine-owned). The variants project to kebab-case strings (`"blocking"`,
/// `"warning"`, `"advisory"`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    /// Blocks the gate — a conformance failure or a blocking validation finding.
    Blocking,
    /// Surfaced, does not block.
    Warning,
    /// Informational only.
    Advisory,
}

/// Where a [`Finding`] points: an optional managed-artifact `address` plus the
/// source `line`/`col` it was raised at.
///
/// `address` is the URI-shaped target ([structural-grammar.md](../../../design/structural-grammar.md)
/// → Addressing) when one is known; conformance diagnostics that are purely
/// positional (raised before an address can be resolved) carry only `line`/`col`,
/// so `address` is `None` and is omitted from the projection. `line`/`col` are
/// 1-based source coordinates, free from the byte offsets the parser already tracks
/// ([parsing.md](../../../implementation/parsing.md) → Diagnostics are located).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// The URI-shaped target address, when one is known; omitted when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 1-based source line.
    pub line: usize,
    /// 1-based source column.
    pub col: usize,
}

impl Location {
    /// A positional location with no address — the common conformance-diagnostic
    /// case (raised at a source coordinate before an address resolves).
    pub fn at(line: usize, col: usize) -> Self {
        Self {
            address: None,
            line,
            col,
        }
    }

    /// An addressed location at a source coordinate.
    pub fn addressed(address: impl Into<String>, line: usize, col: usize) -> Self {
        Self {
            address: Some(address.into()),
            line,
            col,
        }
    }
}

/// The one envelope every problem surfaces through: a [`Severity`], a stable
/// machine `code`, a human-readable `message`, an optional [`Location`], and an
/// optional `route` directing the agent's next action.
///
/// - `code` — a stable identifier for the kind of problem (machine-actionable),
///   always present.
/// - `message` — human-readable, always present.
/// - `location` — where it points; `None` for findings with no source coordinate.
/// - `route` — an optional repair direction the engine **never executes**
///   ([validation.md](../../../design/validation.md) → route is a direction, not a
///   guarantee). A hard block is a [`Severity::Blocking`] finding carrying a route.
///
/// Inc-2 produces only conformance findings: [`Severity::Blocking`], a `code` and a
/// located `message`, `route` typically `None` at parse scope.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    /// How serious this finding is.
    pub severity: Severity,
    /// A stable, machine-actionable identifier for the kind of problem.
    pub code: String,
    /// A human-readable description, always present.
    pub message: String,
    /// Where this finding points, when a source coordinate is known.
    pub location: Option<Location>,
    /// An optional repair direction the engine never executes; present on a hard
    /// block. Projects as `null` when absent (the pinned envelope keeps the key).
    pub route: Option<String>,
}

impl Finding {
    /// A blocking conformance finding at a [`Location`], with no route — the
    /// inc-2 parser's only producer.
    pub fn blocking(
        code: impl Into<String>,
        message: impl Into<String>,
        location: Location,
    ) -> Self {
        Self {
            severity: Severity::Blocking,
            code: code.into(),
            message: message.into(),
            location: Some(location),
            route: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pinned envelope (`DECISIONS.md` 2026-05-31 → Finding shape): a blocking
    /// finding with a `code`, a located `message`, and no route projects to exactly
    /// `{severity:"blocking", code, message, location:{line,col}, route:null}` — in
    /// that field order, no stray keys, `address` omitted when absent, `route` kept
    /// as `null`. The golden pins the serialized string (not a key-sorted value), so
    /// it also locks field *order*; a rename, a reorder, or a serde-attribute slip
    /// breaks it. That is the contract.
    #[test]
    fn finding_json_projection_is_the_pinned_envelope() {
        let finding = Finding::blocking(
            "conformance.heading-missing",
            "required section heading `## Decision` is missing",
            Location::at(1, 1),
        );

        let json = serde_json::to_string_pretty(&finding).expect("serializes");

        insta::assert_snapshot!(json, @r#"
        {
          "severity": "blocking",
          "code": "conformance.heading-missing",
          "message": "required section heading `## Decision` is missing",
          "location": {
            "line": 1,
            "col": 1
          },
          "route": null
        }
        "#);
    }

    /// `Severity` projects to kebab-case strings and round-trips through serde.
    #[test]
    fn severity_round_trips_as_kebab_case() {
        for (variant, token) in [
            (Severity::Blocking, "\"blocking\""),
            (Severity::Warning, "\"warning\""),
            (Severity::Advisory, "\"advisory\""),
        ] {
            let json = serde_json::to_string(&variant).expect("serializes");
            assert_eq!(json, token);
            let back: Severity = serde_json::from_str(&json).expect("deserializes");
            assert_eq!(back, variant);
        }
    }

    /// `Location` round-trips through serde; an addressed location keeps its
    /// `address`, a positional one omits the key entirely.
    #[test]
    fn location_round_trips_through_serde() {
        let positional = Location::at(12, 4);
        let json = serde_json::to_value(&positional).expect("serializes");
        assert_eq!(json, serde_json::json!({ "line": 12, "col": 4 }));
        let back: Location = serde_json::from_value(json).expect("deserializes");
        assert_eq!(back, positional);

        let addressed = Location::addressed("adr:cache#decision", 3, 1);
        let json = serde_json::to_value(&addressed).expect("serializes");
        assert_eq!(
            json,
            serde_json::json!({ "address": "adr:cache#decision", "line": 3, "col": 1 })
        );
        let back: Location = serde_json::from_value(json).expect("deserializes");
        assert_eq!(back, addressed);
    }

    /// A hard block is a blocking finding that carries a route — not a new type.
    /// It round-trips through serde with the route preserved.
    #[test]
    fn finding_with_route_round_trips() {
        let block = Finding {
            severity: Severity::Blocking,
            code: "finalize.forward-ref-dangling".into(),
            message: "`supersedes` target `adr:cache` does not exist".into(),
            location: Some(Location::addressed("adr:new#decision", 5, 3)),
            route: Some("order the task that creates `adr:cache` first".into()),
        };

        let json = serde_json::to_value(&block).expect("serializes");
        let back: Finding = serde_json::from_value(json).expect("deserializes");
        assert_eq!(back, block);
    }
}
