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
//! { "severity": "blocking", "probe": "…", "check": "…", "code": "…",
//!   "key": { "code": "…", "target": "…" },
//!   "message": "…", "location": { "line": 1, "col": 1 }, "route": null }
//! ```
//!
//! `key` is the **stable per-instance identity** `(code, target)` a driver dedupes on
//! ([command-output-contract.md](../../../design/command-output-contract.md) → The stable
//! finding key). It is **derived** — from the finding's `code` and its
//! [`Location::address`] — never a stored field, so it can never drift from the address
//! the finding carries; the `Serialize` impl computes it at projection time. A `target` of
//! `null` is **not** a default for "no address yet": every finding that reaches a
//! serialization funnel carries one of the six declared target forms, and `null` is a
//! **declared singleton value** — reserved for the codes [`is_declared_singleton`] names,
//! whose emission path can yield at most one instance. [`debug_assert_targets_declared`]
//! enforces that at the funnels (the contract's third obligation — the closure claim is a
//! check, not a promise).
//!
//! `probe` / `check` are the **structured severity handle** the M6 post-pass keys on
//! ([validation.md](../../../design/validation.md) → Severity assignment — the M6
//! post-pass: *a `Finding` carries `(probe, check)`*). They are made **explicit
//! fields** rather than re-parsed from `code` on the read path, because the built
//! codes do not all split cleanly to `<probe>.<check>` — the `override-default`
//! classifier emits five descriptive codes that collapse onto three inventory checks
//! (`validation.md` → Code-id reconciliation). For every other producer the handle is
//! exactly the code's `<prefix>.<suffix>` split (populated once at construction); the
//! dotted `code` is retained for rendering but is no longer the severity handle.

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

impl Severity {
    /// The strictness rank in the total order `blocking > warning > advisory`
    /// (`design/overrides.md` → Soft-rejection: severities are totally ordered).
    /// A higher rank is stricter; the cascade soft-rejects a `scalar-set` whose
    /// value ranks below a key's `floor`. `Severity` derives no `Ord` (its variant
    /// order is not a severity claim), so the order is this explicit ranking.
    pub fn rank(self) -> u8 {
        match self {
            Severity::Advisory => 0,
            Severity::Warning => 1,
            Severity::Blocking => 2,
        }
    }

    /// Parse a severity from its kebab-case token (`"blocking"` / `"warning"` /
    /// `"advisory"`), or `None` for any other string. Lets the cascade rank an
    /// opaque scalar value against a floor without a serde round-trip.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "blocking" => Some(Severity::Blocking),
            "warning" => Some(Severity::Warning),
            "advisory" => Some(Severity::Advisory),
            _ => None,
        }
    }
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

/// The **stable per-instance identity** of a [`Finding`] — the `(code, target)` pair a
/// driver dedupes/tracks a finding across sweeps by, and a future acknowledge-ledger keys
/// on ([command-output-contract.md](../../../design/command-output-contract.md) → The
/// stable finding key). Unlike a [`Location`]'s `line`/`col` (which churn under edits and
/// are a convenience pointer only), the key is stable: `target` is the finding's
/// URI-normal-form address (`<type>:<slug>[#<fragment>]`), which survives reorder/retitle
/// and changes only through `jigc rename`. Derived — never stored — from a finding's `code`
/// and its [`Location::address`] ([`Finding::key`]), so it cannot drift from the address the
/// finding carries.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingKey {
    /// The dotted finding id (`<probe>.<check>`) — per-code dedup granularity.
    pub code: String,
    /// The stable address the finding concerns, in URI normal form; `null` for a finding
    /// raised before an address resolves (a purely-positional conformance diagnostic).
    pub target: Option<String>,
}

/// Whether `code` is a **declared singleton exception** — the one place `target: null` is a
/// *value of the pin* rather than an un-swept default
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The declared
/// singleton exception). The list is stated by its predicate — *a finding whose emission path
/// can yield at most one instance has nothing to discriminate against, so `(code, null)` is
/// already unique-per-instance* — and this is its **one home**:
///
/// - `store-version.binary-mismatch` — its subject is **the store**, of which a repo has one.
/// - every `setup.*` / `uninstall.*` — `setup::run` / `run_uninstall` are `Result<_, Finding>`
///   (fail-fast, **exactly one finding per invocation**; no `Vec<Finding>` anywhere), so two
///   instances of one code can never coexist in one output.
///
/// Anything else with no address is the un-swept state of a family nobody has looked at, and
/// [`debug_assert_targets_declared`] says so at the seam.
pub fn is_declared_singleton(code: &str) -> bool {
    code == "store-version.binary-mismatch"
        || code.starts_with("setup.")
        || code.starts_with("uninstall.")
}

/// The **membership test, made mechanical** — the check the contract's third obligation owes
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The membership
/// test): *every `Finding` reaching a serialization funnel carries `Some(Location::address)`,
/// except the declared singletons.* A finding is in the envelope-projecting set **iff** it is
/// serialized as a `Finding` — [`Finding`]'s `Serialize` writes `key` unconditionally and
/// derives `key.target` from [`Location::address`], so a null address **is** a null key. Call
/// it at each of the three funnels (`ValidationReport`'s `findings[]`, a `DocAck`'s
/// `findings[]`, the bare-`Finding` `setup_block`); a family that forgets its target form then
/// fails the suite instead of shipping a degenerate key.
///
/// Debug-only (`debug_assert`): the obligation is an invariant of jigc's **own** finding
/// producers — a build-time property the whole test suite exercises through the seam — not a
/// runtime condition on user input, so it must never turn a user's finding into a panic.
pub fn debug_assert_targets_declared(findings: &[Finding]) {
    for finding in findings {
        debug_assert!(
            finding.carries_declared_target(),
            "finding `{}` reaches a serialization funnel with no `key.target` — give it one \
             of the six declared target forms, or declare it an exception \
             (design/command-output-contract.md → The membership test)",
            finding.code,
        );
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
///
/// `Serialize` is hand-written (not derived) so the projection can carry the derived
/// [`FindingKey`] as a `key` field without storing it — see [`Finding::key`].
/// `Deserialize` is derived and ignores the extra `key` field on the wire.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Finding {
    /// How serious this finding is.
    pub severity: Severity,
    /// The probe (or synthetic category) that raised this finding — the prefix half of
    /// the structured severity handle the M6 post-pass keys on
    /// ([validation.md](../../../design/validation.md) → Severity assignment). For
    /// every producer except `override-default` this equals the `code`'s prefix; the
    /// classifier sets it explicitly. `#[serde(default)]` so older / hand-built JSON
    /// round-trips.
    #[serde(default)]
    pub probe: String,
    /// The check within the probe — the suffix half of the `(probe, check)` severity
    /// handle. For `override-default` this is the **canonical inventory check id**
    /// (`validation.md` → Code-id reconciliation: five codes → three checks), so it may
    /// differ from the `code` suffix; for every other producer it is the `code` suffix.
    #[serde(default)]
    pub check: String,
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

/// Split a dotted `code` into its `(probe, check)` handle on the **first** `.` — the
/// derivation every producer except `override-default` uses (`validation.md` →
/// Code-id reconciliation: for all probes except `override-default` the check is the
/// code suffix and the probe the prefix). A code with no `.` (none ship today)
/// degenerates to `(code, "")`.
fn split_code(code: &str) -> (String, String) {
    match code.split_once('.') {
        Some((probe, check)) => (probe.to_string(), check.to_string()),
        None => (code.to_string(), String::new()),
    }
}

impl Serialize for Finding {
    /// Project the pinned findings envelope, deriving the stable [`FindingKey`] as the
    /// `key` field right after `code`
    /// ([command-output-contract.md](../../../design/command-output-contract.md) → the
    /// findings envelope). Hand-written rather than derived so `key` is computed at
    /// projection time from `code` + [`Location::address`] — a single source of truth that
    /// cannot drift from the address the finding carries.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut st = serializer.serialize_struct("Finding", 8)?;
        st.serialize_field("severity", &self.severity)?;
        st.serialize_field("probe", &self.probe)?;
        st.serialize_field("check", &self.check)?;
        st.serialize_field("code", &self.code)?;
        st.serialize_field("key", &self.key())?;
        st.serialize_field("message", &self.message)?;
        st.serialize_field("location", &self.location)?;
        st.serialize_field("route", &self.route)?;
        st.end()
    }
}

impl Finding {
    /// The **stable per-instance key** `(code, target)` — `target` derived from this
    /// finding's [`Location::address`] (the URI-normal-form address a `path→URI` flip
    /// installs; `None` before an address resolves). Never stored, so it always tracks the
    /// carried address; serialized as the envelope's `key` field.
    pub fn key(&self) -> FindingKey {
        FindingKey {
            code: self.code.clone(),
            target: self.location.as_ref().and_then(|l| l.address.clone()),
        }
    }

    /// Whether this finding honours the target obligation: it carries a discriminating
    /// `key.target` (a [`Location::address`] in one of the six declared forms), **or** its
    /// code is a [`is_declared_singleton`] exception. The predicate
    /// [`debug_assert_targets_declared`] enforces at every serialization funnel.
    pub fn carries_declared_target(&self) -> bool {
        self.key().target.is_some() || is_declared_singleton(&self.code)
    }

    /// A blocking conformance finding at a [`Location`], with no route — the
    /// inc-2 parser's only producer. `probe` / `check` derive from the `code`'s
    /// `<prefix>.<suffix>` split ([`split_code`]).
    pub fn blocking(
        code: impl Into<String>,
        message: impl Into<String>,
        location: Location,
    ) -> Self {
        let code = code.into();
        let (probe, check) = split_code(&code);
        Self {
            severity: Severity::Blocking,
            probe,
            check,
            code,
            message: message.into(),
            location: Some(location),
            route: None,
        }
    }

    /// Override this finding's `check` to the **canonical inventory check id**, leaving
    /// the descriptive `code` (and everything else) untouched — the `override-default`
    /// rename surface where the emitted `code` does not equal its inventory check
    /// (`validation.md` → Code-id reconciliation). The only producer that needs it.
    #[must_use]
    pub fn with_check(mut self, check: impl Into<String>) -> Self {
        self.check = check.into();
        self
    }

    /// A hard block: a [`Severity::Blocking`] finding carrying a `route` (the
    /// next action) and no source location. The settled block-payload envelope
    /// (`DECISIONS.md` 2026-05-31 → a hard block is a blocking finding with a
    /// route, not a new type) — used by operations whose failure points at a
    /// human action rather than a source coordinate (e.g. `jigc setup`).
    pub fn block(
        code: impl Into<String>,
        message: impl Into<String>,
        route: impl Into<String>,
    ) -> Self {
        let code = code.into();
        let (probe, check) = split_code(&code);
        Self {
            severity: Severity::Blocking,
            probe,
            check,
            code,
            message: message.into(),
            location: None,
            route: Some(route.into()),
        }
    }

    /// A finding at an arbitrary [`Severity`] with an optional [`Location`] and
    /// `route`, deriving `probe` / `check` from the `code`'s `<prefix>.<suffix>` split
    /// ([`split_code`]) — the general constructor for producers that are not blocking,
    /// no-route conformance ([`Finding::blocking`]) or routed hard blocks
    /// ([`Finding::block`]). Centralizes handle population so no `Finding { … }` literal
    /// in the engine has to spell `probe` / `check`.
    pub fn graded(
        severity: Severity,
        code: impl Into<String>,
        message: impl Into<String>,
        location: Option<Location>,
        route: Option<String>,
    ) -> Self {
        let code = code.into();
        let (probe, check) = split_code(&code);
        Self {
            severity,
            probe,
            check,
            code,
            message: message.into(),
            location,
            route,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pinned envelope (`DECISIONS.md` 2026-05-31 → Finding shape; re-pinned M6
    /// for the `(probe, check)` handle): a blocking finding with a `code`, a located
    /// `message`, and no route projects to exactly `{severity:"blocking", probe, check,
    /// code, message, location:{line,col}, route:null}` — in that field order, no stray
    /// keys, `address` omitted when absent, `route` kept as `null`. The `probe`/`check`
    /// handle derives from the `code`'s `<prefix>.<suffix>` split (here the exempt
    /// parser code `conformance.heading-missing` → `("conformance", "heading-missing")`
    /// — it still *carries* a handle, it is merely not post-passed). The golden pins
    /// the serialized string (not a key-sorted value), so it also locks field *order*;
    /// a rename, a reorder, or a serde-attribute slip breaks it. That is the contract.
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
          "probe": "conformance",
          "check": "heading-missing",
          "code": "conformance.heading-missing",
          "key": {
            "code": "conformance.heading-missing",
            "target": null
          },
          "message": "required section heading `## Decision` is missing",
          "location": {
            "line": 1,
            "col": 1
          },
          "route": null
        }
        "#);
    }

    /// The stable key carries the finding's URI `target` when it has one: an addressed
    /// finding projects `key.target` = its [`Location::address`], derived (not stored) so
    /// it always tracks the address a `path→URI` flip installs. Pins the `{code, target}`
    /// shape the driver dedupes on (`command-output-contract.md` → the stable finding key).
    #[test]
    fn finding_key_carries_the_uri_target() {
        let finding = Finding::graded(
            Severity::Blocking,
            "schema-conformance.ref-resolves",
            "forward-ref integrity — target does not resolve",
            Some(Location::addressed("adr:cache#supersedes/adr:ghost", 1, 1)),
            None,
        );
        let json = serde_json::to_value(&finding).expect("serializes");
        assert_eq!(
            json["key"],
            serde_json::json!({
                "code": "schema-conformance.ref-resolves",
                "target": "adr:cache#supersedes/adr:ghost"
            }),
            "an addressed finding's key.target is its URI address",
        );
    }

    /// The exempt-code handle (`validation.md` → Severity inventory: membership is the
    /// keyed surface, not every emitted code). A finding whose `(probe, check)` is *not*
    /// an inventory row — here the synthetic `schema-conformance.unknown-type`, exempt
    /// from the M6 post-pass — still *carries* a `(probe, check)`, split from its code
    /// prefix/suffix. The post-pass (T2) matches on membership, never code-prefix, so an
    /// exempt sibling under a keyed probe is left untouched; this pins that it nonetheless
    /// carries a derivable handle.
    #[test]
    fn exempt_code_still_carries_probe_check() {
        let finding = Finding::blocking(
            "schema-conformance.unknown-type",
            "staged doc has a type the resolved cascade does not define",
            Location::addressed("docs/wat:x.md", 1, 1),
        );
        assert_eq!(finding.probe, "schema-conformance");
        assert_eq!(finding.check, "unknown-type");
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

    /// The severity total order `blocking > warning > advisory` (`design/overrides.md`
    /// → Soft-rejection): `rank` is strictly decreasing across the three, and
    /// `from_token` round-trips each kebab-case token while rejecting any other.
    #[test]
    fn severity_total_order_and_token_round_trip() {
        assert!(Severity::Blocking.rank() > Severity::Warning.rank());
        assert!(Severity::Warning.rank() > Severity::Advisory.rank());

        assert_eq!(Severity::from_token("blocking"), Some(Severity::Blocking));
        assert_eq!(Severity::from_token("warning"), Some(Severity::Warning));
        assert_eq!(Severity::from_token("advisory"), Some(Severity::Advisory));
        assert_eq!(Severity::from_token("nonsense"), None);
    }

    /// A hard block is a blocking finding that carries a route — not a new type.
    /// It round-trips through serde with the route preserved.
    #[test]
    fn finding_with_route_round_trips() {
        let block = Finding::graded(
            Severity::Blocking,
            "finalize.forward-ref-dangling",
            "`supersedes` target `adr:cache` does not exist",
            Some(Location::addressed("adr:new#decision", 5, 3)),
            Some("order the task that creates `adr:cache` first".into()),
        );

        let json = serde_json::to_value(&block).expect("serializes");
        let back: Finding = serde_json::from_value(json).expect("deserializes");
        assert_eq!(back, block);
    }
}
