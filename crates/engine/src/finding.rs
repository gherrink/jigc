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
//! whose emission path can yield at most one instance.
//!
//! **The seam is structural, not a funnel list.** The membership predicate is *a finding
//! projects a key **iff** it is serialized as a `Finding`* — so the guard lives **on the
//! serialization**: [`Finding`]'s `Serialize` impl asserts the **presence** half itself, and
//! the [`Findings`] collection newtype — the sanctioned way a *set* of findings serializes —
//! asserts the **uniqueness** half, which needs the set. Neither can be bypassed, and there
//! is nothing to enumerate: a new surface that serializes a finding passes the check *by
//! construction* (the M42 lesson, one level up — a hand-listed set of funnels is a census,
//! and a census rots as the set grows; the funnel list missed `jigc ingest`'s triage row).
//!
//! The **route floor** rides the same seam (M43,
//! [surface-contract.md](../../../design/surface-contract.md) → The route fence;
//! [validation.md](../../../design/validation.md) → The route floor): `blocking ⇒ route
//! present`, asserted on [`Finding`]'s `Serialize` alongside the target presence — a
//! blocked gate that names no recovery cannot be serialized at all. The one exemption is
//! the purely-positional parser diagnostic ([`is_route_exempt`], the floor's one-home
//! exemption fn).
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

/// The **`path→URI` flip**: re-address every finding in `findings` at the owning doc's
/// `identity` (its `<type>:<slug>` URI — a **placement** doctype's `<type>:<type>` singleton
/// included), turning the bare per-instance checks' fragment-only / absent addresses into the
/// URI normal form the stable key is defined in
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The stable
/// finding key). A fragment-bearing address becomes `<identity>#<fragment>`; a fragment-less
/// or location-less finding is addressed at the bare `<identity>`. **`line`/`col` are
/// preserved** — re-addressing never moves the source coordinate the check raised.
///
/// **Precondition**: the findings are *raw* per-instance output (a parse / conformance sweep
/// against one doc), whose addresses are fragment-only — never already-flipped URIs, which
/// would double-prefix. Every producer that holds a doc's identity flips at that boundary:
/// the store + task conformance sweeps (`validate::attribute_to_doc`, which prefixes the
/// message with the doc's path on the way through) and `jigc ingest`'s near-miss row
/// re-derivation — so one defect in one doc projects **one** key, whichever verb reports it.
pub fn readdress_to_uri(findings: &mut [Finding], identity: &str) {
    for finding in findings {
        let uri = match finding.location.as_ref().and_then(|l| l.address.as_deref()) {
            Some(fragment) => format!("{identity}#{fragment}"),
            None => identity.to_string(),
        };
        // The route half of the flip (2026-07-17 surface-comprehension review, B1): a
        // mechanical route minted before the doc's identity was in hand carries the
        // `<address>` placeholder of the CLI-seam dummy table — but here the finding's
        // real URI address *is* the write address (`<type>:<slug>#<section[/leaf]>` is
        // exactly the grammar `set-slot`/`set-field` accept), so the placeholder is
        // rendered concrete and the route becomes copy-runnable. Rebuilt through
        // [`Route::mechanical`], so the parse fence re-proves the concrete argv.
        if let Some(route) = &finding.route
            && let RouteKind::Mechanical { argv, tail } = route.kind()
            && argv.iter().any(|arg| arg == "<address>")
        {
            let argv: Vec<String> = argv
                .iter()
                .map(|arg| {
                    if arg == "<address>" {
                        uri.clone()
                    } else {
                        arg.clone()
                    }
                })
                .collect();
            finding.route = Some(Route::mechanical(argv, tail.clone()));
        }
        match &mut finding.location {
            Some(location) => location.address = Some(uri),
            None => finding.location = Some(Location::addressed(uri, 1, 1)),
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
/// - every `structural-target.*` / `slot-fill-target.*` — `StructuralTarget::parse` /
///   `SlotFillTarget::parse` are `Result<_, Finding>` (fail-fast, exactly one finding per
///   invocation) and every caller bails the verb on the first error, so two instances of
///   one code can never coexist in one output. Declared at the M43 completion audit,
///   where the family gained its route: seam-admissible by decision, not by the accident
///   of every caller rendering through `finding_to_err`.
/// - `overrides.project-step-missing` — recorded into `CascadeStepSource`'s **one-slot**
///   error sink (`Option<Finding>`, last write wins) and drained as the verb's single
///   located error, so at most one instance reaches an output. Declared with the route
///   at the M43 completion audit, same rationale as the parse errors above.
///
/// Anything else with no address is the un-swept state of a family nobody has looked at, and
/// [`debug_assert_targets_declared`] says so at the seam.
pub fn is_declared_singleton(code: &str) -> bool {
    code == "store-version.binary-mismatch"
        || code.starts_with("setup.")
        || code.starts_with("uninstall.")
        || code.starts_with("structural-target.")
        || code.starts_with("slot-fill-target.")
        || code == "overrides.project-step-missing"
}

/// Whether `code` is **route-exempt** under the route floor — the floor's **one-home
/// exemption fn** (the [`is_declared_singleton`] pattern: a list of exceptions checkable
/// from the code alone, never a census of call sites). The route floor
/// ([surface-contract.md](../../../design/surface-contract.md) → The route fence;
/// [validation.md](../../../design/validation.md) → The route floor) says `blocking ⇒
/// route present`, asserted on [`Finding`]'s `Serialize`; the exemption covers exactly the
/// **purely-positional parser `conformance.*` diagnostics** — a malformed byte at a source
/// coordinate, where *the located message is the repair* (fix the named line; no CLI verb
/// repairs a hand-broken byte) and any at-parse route would be a guess. Exemption means
/// *may be route-less*: a parser diagnostic that does know a direction (the below-version
/// parse failure routed `migrate`) still carries it.
///
/// The **hook-rejection error identity** (`finalize.commit-rejected`) is the floor's other
/// re-affirmed exemption, but it is an anyhow error path, not a [`Finding`] — git's
/// verbatim stderr *is* the correction signal ([finalize.md](../../../design/finalize.md))
/// — so it never reaches this seam and needs no entry here.
pub fn is_route_exempt(code: &str) -> bool {
    code.starts_with("conformance.")
}

/// Whether `code` is a **declared non-unique exception** — a code whose key is *deliberately
/// collapsed below its subject's granularity*, so two instances sharing one `(code, target)`
/// in one emitted slice are the pin, not a defect
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The declared
/// non-unique exceptions). Each carries a **non-null** target; what it forgoes is the
/// sub-discriminator, and each forgoes it for a stated reason — this is its **one home**:
///
/// - `conformance.item-anchor-missing` — an item with no anchor **has no identity**: the key
///   that would discriminate is precisely the thing the finding reports missing.
/// - `conformance.item-anchor-malformed` — the same reason, one step along: the item's anchor
///   text **is not an identity, it is the defect**, so keying on it (`#<section>/<found>`)
///   discriminates the *bad text*, not the item — and two items carrying the same bad anchor
///   collapse. Both are still reported (unlike a duplicated *id*, two broken anchors are two
///   items and two repairs); what is forgone is telling them apart by key, because the only
///   sub-discriminators on offer are heading prose (unstable) and position (excluded by the
///   never-positions invariant).
/// - `conformance.item-slot-delimiter-shadowed`, `conformance.slot-setext-heading`,
///   `conformance.slot-heading-depth` — the subject is a **slot's prose**; the only
///   sub-discriminator on offer is prose text, and a prose-derived key churns under exactly
///   the edits the key exists to survive.
/// - every `workflow-refs.*` — keyed at the **pack resource**: a workflow-def load failure
///   means the pack is broken, a defect a pack author fixes once, not a corpus finding a
///   driver tracks across sweeps.
///
/// Anything else that collides is a degenerate key, and [`debug_assert_targets_declared`]
/// says so at the seam.
pub fn is_declared_non_unique(code: &str) -> bool {
    matches!(
        code,
        "conformance.item-anchor-missing"
            | "conformance.item-anchor-malformed"
            | "conformance.item-slot-delimiter-shadowed"
            | "conformance.slot-setext-heading"
            | "conformance.slot-heading-depth"
    ) || code.starts_with("workflow-refs.")
}

/// The **discriminating half** of the membership test, over one emitted slice
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The membership
/// test): no two findings in `findings` share one `(code, target)`, except the
/// [`is_declared_non_unique`] codes (collapsed on purpose, with a reason).
///
/// The closure claim — *`(code, target)` is **unique-per-instance*** — is **two** properties,
/// and checking only the first is how the class reopens. The *presence* half rides on
/// [`Finding`]'s own `Serialize` (a finding cannot be projected without it); *uniqueness*
/// needs the **set**, so it cannot live in a single element's `serialize` — it lives here, on
/// [`Findings`]'s `Serialize`, the sanctioned way a collection of findings projects.
///
/// Presence alone passes a **degenerate** key: `schema-conformance.required-slot-present`
/// shipped three byte-identical `(code, adr:<slug>)` keys for one pristine ADR *through* the
/// seam, because its target was non-null — under-discriminating, never address-less. A driver
/// deserializing that array cannot tell the findings apart, which is exactly what the key
/// exists to let it do. So this half is keyed on the property the contract **claims**
/// (*discriminating*), not on the symptom that made the class visible (*null*).
///
/// Debug-only (`debug_assert`): the obligation is an invariant of jigc's **own** finding
/// producers — a build-time property the whole test suite exercises through the seam — not a
/// runtime condition on user input, so it must never turn a user's finding into a panic.
///
/// Public because a container that holds findings **indirectly** (a report of rows, each
/// carrying an `Option<Finding>` — `jigc ingest`'s triage report) cannot reach [`Findings`]'s
/// impl but still emits one slice; it runs this over its own set from its own `Serialize`.
pub fn debug_assert_keys_discriminate(findings: &[Finding]) {
    // Cheap enough to skip entirely in release: `debug_assert!` compiles its condition out,
    // but the uniqueness pass is a statement, so gate it the same way.
    if !cfg!(debug_assertions) {
        return;
    }
    let mut seen: std::collections::HashSet<FindingKey> = std::collections::HashSet::new();
    for finding in findings {
        if is_declared_non_unique(&finding.code) {
            continue;
        }
        let key = finding.key();
        let target = key.target.clone().unwrap_or_else(|| "null".to_owned());
        debug_assert!(
            seen.insert(key),
            "two findings in one serialized slice collide on one key (`{}`, `{}`) — give the \
             code a discriminating `#<fragment>`, or declare it non-unique \
             (design/command-output-contract.md → The membership test)",
            finding.code,
            target,
        );
    }
}

/// A **collection of findings, as it serializes** — the one sanctioned way a *set* of
/// [`Finding`]s projects ([command-output-contract.md](../../../design/command-output-contract.md)
/// → The membership test). Its `Serialize` runs [`debug_assert_keys_discriminate`] over the
/// slice and then projects it as a plain JSON array — byte-identical to the `Vec<Finding>` it
/// replaces, so no wire shape moves.
///
/// It exists so the uniqueness half of the membership test has a **structural** home: the
/// property needs the set, so it cannot ride an element's `serialize` — but a *list of places
/// to call a checker* is a census, and M42's lesson is that a census rots. A findings
/// collection that projects through this type **cannot** skip the check; a driver therefore
/// never receives an array whose keys it cannot tell apart.
///
/// Transparent on the wire and derefs to `[Finding]`, so it reads like the `Vec` it wraps.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct Findings(Vec<Finding>);

impl Serialize for Findings {
    /// The **uniqueness half of the seam** — run over the set, then project the plain array
    /// (each element's own `Serialize` runs the presence half).
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        debug_assert_keys_discriminate(&self.0);
        self.0.serialize(serializer)
    }
}

impl Findings {
    /// The wrapped findings, by value — for the (rare) consumer that must own the `Vec`.
    pub fn into_vec(self) -> Vec<Finding> {
        self.0
    }

    /// Append a finding to the collection — the post-report append the CLI's store sweep does
    /// (the binary-mismatch / orphan advisories it derives outside the engine families). The
    /// seam is unaffected: the checks ride the projection, not the construction.
    pub fn push(&mut self, finding: Finding) {
        self.0.push(finding);
    }
}

impl From<Vec<Finding>> for Findings {
    fn from(findings: Vec<Finding>) -> Self {
        Self(findings)
    }
}

impl FromIterator<Finding> for Findings {
    fn from_iter<I: IntoIterator<Item = Finding>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Findings {
    type Item = Finding;
    type IntoIter = std::vec::IntoIter<Finding>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Findings {
    type Item = &'a Finding;
    type IntoIter = std::slice::Iter<'a, Finding>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl std::ops::Deref for Findings {
    type Target = [Finding];
    fn deref(&self) -> &[Finding] {
        &self.0
    }
}

/// An agent-facing repair direction, as [`Finding::route`] carries it — the **internal
/// route value** of the M43 route fence
/// ([surface-contract.md](../../../design/surface-contract.md) → The route fence). Three
/// kinds ([`RouteKind`]): `Mechanical` (a copy-runnable command), `Human` (a direction
/// only a human judgment can take), `Informational` ("no action needed"). The kind is
/// **internal**: on the wire a route is still today's flat string — `Serialize` projects
/// [`Route::as_str`] **byte-identical** to the pre-M43 `Option<String>`, so the pinned
/// envelope golden does not move, and `Deserialize` maps any string to `Human` (a
/// reloaded `Mechanical` degrades to `Human` but serializes identically, so nothing is
/// lost on the wire). The wire tagged-union stays deferred with its recorded trigger
/// ([decisions-pending.md](../../../implementation/decisions-pending.md) → `route`
/// tagged-union promotion).
///
/// Fields are private, so a route exists only through its constructors: the flat `text`
/// is composed **once**, from the kind's own parts, and cannot drift from them.
/// `From<String>` / `From<&str>` map to `Human`, so producer migration is per-producer —
/// an un-migrated call site keeps compiling (and keeps its exact bytes) via `Into`.
/// Derefs to `str`, so readers that render the route as a string are unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Route {
    /// The flat string this route projects to — the wire shape.
    text: String,
    /// The kind taxonomy — internal until the wire union is promoted.
    kind: RouteKind,
}

/// The route **kind taxonomy** — the act-vs-inform discrimination a driver needs,
/// recorded internally ahead of the deferred wire promotion
/// ([surface-contract.md](../../../design/surface-contract.md) → The route fence:
/// `Mechanical{argv}` subsumes the retired `run-command` sketch). The route's prose
/// lives on [`Route`]'s `text`, not the variant, so the flat wire projection has one
/// home for every kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteKind {
    /// A copy-runnable command: `argv` is the exact argv (leading `jigc`) the flat text
    /// backticks; `tail` is trailing prose appended **verbatim** after the closing
    /// backtick — it carries its own leading separator, and is empty for a bare command.
    /// The CLI-seam parse fence asserts the argv parses against the real CLI at
    /// construction time; the engine stores it clap-blind.
    Mechanical { argv: Vec<String>, tail: String },
    /// A direction only a human can take — and the `Deserialize` / `From<String>`
    /// default, so an un-migrated producer's route lands here.
    Human,
    /// "No action needed" — informs, directs nothing.
    Informational,
}

/// The installed CLI-seam argv validator for [`Route::mechanical`] — the parse half of the
/// M43 route fence ([surface-contract.md](../../../design/surface-contract.md) → The route
/// fence, pinned mechanics). The engine is clap-blind by layering (no CLI dependency), so it
/// cannot check "this argv parses against the real CLI" itself; the CLI installs the check at
/// process start ([`install_mechanical_argv_validator`], the injected-validator shape) and the
/// constructor consults it debug-only — enforcement rides the suite, never a release panic.
static MECHANICAL_ARGV_VALIDATOR: std::sync::OnceLock<MechanicalArgvValidator> =
    std::sync::OnceLock::new();

/// The shape of the CLI-seam argv validator: `Ok(())` if the argv parses against the real
/// CLI, `Err(reason)` otherwise.
pub type MechanicalArgvValidator = fn(&[String]) -> Result<(), String>;

/// Install the CLI-seam argv validator [`Route::mechanical`] consults (the parse assert of
/// the M43 route fence). Idempotent — the first install wins: a process has exactly one CLI,
/// so `main` and the CLI test seam install the same function.
pub fn install_mechanical_argv_validator(validator: MechanicalArgvValidator) {
    let _ = MECHANICAL_ARGV_VALIDATOR.set(validator);
}

impl Route {
    /// A copy-runnable command route. The flat text is composed here and only here —
    /// `` `<argv joined by spaces>` `` + `tail` verbatim — the one composition rule,
    /// so a mechanical route's text can never drift from its argv.
    ///
    /// **The parse fence** (debug posture, the seam-assert class): a mechanical route whose
    /// `argv` does not parse against the real CLI **cannot be constructed** — the CLI-installed
    /// validator ([`install_mechanical_argv_validator`]) panics the construction in debug
    /// builds, so the fence rides the suite; a release binary never pays or panics.
    pub fn mechanical<I, S>(argv: I, tail: impl Into<String>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let argv: Vec<String> = argv.into_iter().map(Into::into).collect();
        // The parse fence (debug posture — compiled out of release builds, like the
        // sibling key-seam asserts): consult the CLI-installed validator, if any.
        #[cfg(debug_assertions)]
        if let Some(validate) = MECHANICAL_ARGV_VALIDATOR.get()
            && let Err(reason) = validate(&argv)
        {
            panic!(
                "a `Route::mechanical` argv must parse against the real CLI: {reason} — \
                 argv {argv:?} (design/surface-contract.md → The route fence)"
            );
        }
        let tail = tail.into();
        let text = format!("`{}`{}", argv.join(" "), tail);
        Self {
            text,
            kind: RouteKind::Mechanical { argv, tail },
        }
    }

    /// A human-judgment direction — the flat text verbatim.
    pub fn human(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: RouteKind::Human,
        }
    }

    /// A "no action needed" notice — the flat text verbatim.
    pub fn informational(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: RouteKind::Informational,
        }
    }

    /// The flat string this route projects to (also reachable via `Deref<Target = str>`).
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// The kind taxonomy this route was constructed as.
    pub fn kind(&self) -> &RouteKind {
        &self.kind
    }
}

impl std::ops::Deref for Route {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

impl From<String> for Route {
    /// An un-migrated producer's string route is a [`RouteKind::Human`] direction.
    fn from(text: String) -> Self {
        Route::human(text)
    }
}

impl From<&str> for Route {
    /// See [`From<String>`] — the same per-producer migration seam.
    fn from(text: &str) -> Self {
        Route::human(text)
    }
}

impl Serialize for Route {
    /// **Byte-identical to the pre-M43 string**: a route serializes as its flat text
    /// only — the kind never reaches the wire (the tagged union stays deferred), so the
    /// pinned envelope golden does not move.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.text)
    }
}

impl<'de> Deserialize<'de> for Route {
    /// The wire carries the flat string only, so a reloaded route is [`RouteKind::Human`]
    /// — a `Mechanical` route degrades to `Human` on a round-trip but serializes
    /// identically, so the wire bytes are preserved exactly.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Route::human(String::deserialize(deserializer)?))
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
    /// A repair direction the engine never executes. Internally a [`Route`]
    /// (kind-carrying, M43); on the wire still the flat string. Under the route floor
    /// (M43) a **blocking** finding must carry one to serialize, unless its code is
    /// [`is_route_exempt`]; projects as `null` when absent (the pinned envelope keeps
    /// the key).
    pub route: Option<Route>,
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
    ///
    /// **This is the seam.** The membership predicate is *a finding projects a key iff it is
    /// serialized as a `Finding`* — so the **presence** half of the membership test is
    /// asserted right here, on the projection itself, where it cannot be bypassed and there
    /// is nothing to enumerate (the uniqueness half needs the set: [`Findings`]). Debug-only,
    /// like every other clause of the test: this is an invariant of jigc's **own** producers,
    /// never a runtime condition on user input.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        debug_assert!(
            self.carries_declared_target(),
            "finding `{}` is serialized with no `key.target` — give it one of the six declared \
             target forms, or declare it an exception (design/command-output-contract.md → The \
             membership test)",
            self.code,
        );
        // The route floor's presence half (M43) — the third assert on this seam: a blocking
        // finding names its recovery, or its code is a declared parser-diagnostic exemption.
        debug_assert!(
            self.severity != Severity::Blocking
                || self.route.is_some()
                || is_route_exempt(&self.code),
            "blocking finding `{}` is serialized with no `route` — name its recovery, or (only \
             if it is a purely-positional parser diagnostic) declare it in `is_route_exempt` \
             (design/surface-contract.md → The route fence)",
            self.code,
        );
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

    /// Whether this finding honours the **presence** half of the target obligation: it carries
    /// a `key.target` (a [`Location::address`] in one of the six declared forms), **or** its
    /// code is a [`is_declared_singleton`] exception. Asserted on every projection by
    /// [`Finding`]'s `Serialize`. Presence alone is *not* the closure claim — a non-null target
    /// can still fail to discriminate — so [`Findings`]'s `Serialize` pairs it with the
    /// **uniqueness** pass over the slice ([`debug_assert_keys_discriminate`]); the two halves
    /// together are the membership test.
    pub fn carries_declared_target(&self) -> bool {
        self.key().target.is_some() || is_declared_singleton(&self.code)
    }

    /// A blocking conformance finding at a [`Location`], with no route — suited to the
    /// route floor's [`is_route_exempt`] parser diagnostics (the `conformance.*` family,
    /// its main producer). A non-exempt blocking producer must construct a routed
    /// finding instead ([`Finding::graded`] / [`Finding::block`]) or the serialization
    /// seam refuses it. `probe` / `check` derive from the `code`'s `<prefix>.<suffix>`
    /// split ([`split_code`]).
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
        route: impl Into<Route>,
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
        route: Option<Route>,
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

    /// The route half of the `path→URI` flip (B1, 2026-07-17 surface review): once
    /// [`readdress_to_uri`] has the doc's identity in hand, a mechanical route minted
    /// with the `<address>` placeholder is rebuilt with the finding's **real** URI
    /// address — copy-runnable, since the URI form is exactly the write grammar
    /// (`jigc doc set-slot adr:use-sqlite#decision …`). A human route and a mechanical
    /// route without the placeholder pass through untouched, and the derived
    /// `key.target` is unchanged by the substitution (the route is not part of the key).
    #[test]
    fn readdress_renders_the_address_placeholder_concrete_in_mechanical_routes() {
        let mut findings = [
            Finding::graded(
                Severity::Blocking,
                "schema-conformance.required-slot-present",
                "required slot in section `decision` is empty",
                Some(Location::addressed("decision", 3, 1)),
                Some(Route::mechanical(
                    ["jigc", "doc", "set-slot", "<address>", "--from-file", "-"],
                    " to fill the empty slot",
                )),
            ),
            Finding::graded(
                Severity::Blocking,
                "schema-conformance.unknown-type",
                "staged doc has an unknown type",
                None,
                Some(Route::human("check the type against `jigc describe`")),
            ),
        ];

        readdress_to_uri(&mut findings, "adr:use-sqlite");

        let route = findings[0].route.as_ref().expect("route kept");
        assert_eq!(
            route.as_str(),
            "`jigc doc set-slot adr:use-sqlite#decision --from-file -` to fill the empty slot",
            "the placeholder is rendered as the finding's real write address"
        );
        assert_eq!(
            findings[0].key().target.as_deref(),
            Some("adr:use-sqlite#decision"),
            "the stable key is the flipped URI, unaffected by the route substitution"
        );
        // The human route is untouched — substitution reaches only a mechanical
        // route carrying the placeholder.
        assert_eq!(
            findings[1].route.as_ref().map(|r| r.as_str()),
            Some("check the type against `jigc describe`"),
        );
    }

    /// The pinned envelope (`DECISIONS.md` 2026-05-31 → Finding shape; re-pinned M6
    /// for the `(probe, check)` handle): a blocking finding with a `code`, a located
    /// `message`, and no route projects to exactly `{severity:"blocking", probe, check,
    /// code, key, message, location:{address,line,col}, route:null}` — in that field order,
    /// no stray keys, `route` kept as `null`. The `probe`/`check` handle derives from the
    /// `code`'s `<prefix>.<suffix>` split (here the exempt parser code
    /// `conformance.heading-missing` → `("conformance", "heading-missing")` — it still
    /// *carries* a handle, it is merely not post-passed). The golden pins the serialized
    /// string (not a key-sorted value), so it also locks field *order*; a rename, a reorder,
    /// or a serde-attribute slip breaks it. That is the contract.
    ///
    /// The finding is **addressed**, because that is the only shape a non-exempt finding can
    /// be *projected* in: the presence half of the membership test rides this very
    /// `Serialize`, so an address-less `conformance.*` finding cannot reach a driver at all
    /// (the parser emits fragment-only addresses; the owning doc's identity is flipped in by
    /// `validate::attribute_to_doc` before the projection — the URI form pinned here). The
    /// legitimate `target: null` shape — a **declared singleton** — is pinned by
    /// `result::tests::the_report_seam_passes_the_declared_singleton`.
    #[test]
    fn finding_json_projection_is_the_pinned_envelope() {
        let finding = Finding::blocking(
            "conformance.heading-missing",
            "required section heading `## Decision` is missing",
            Location::addressed("adr:pick-a-db#decision", 1, 1),
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
            "target": "adr:pick-a-db#decision"
          },
          "message": "required section heading `## Decision` is missing",
          "location": {
            "address": "adr:pick-a-db#decision",
            "line": 1,
            "col": 1
          },
          "route": null
        }
        "#);
    }

    /// **The seam is structural** (M42 Inc 9, the fix): the presence half of the membership
    /// test rides [`Finding`]'s own `Serialize`, so a finding with no `key.target` cannot be
    /// projected **at all** — not through a report, not through a write ack, not through a
    /// triage row, not through a surface invented next year. There is no funnel list to keep
    /// current: the check is on the projection, and *being projected* is what makes a finding a
    /// member. Red before the move: the same finding serialized happily to `target: null`
    /// anywhere outside the three hand-listed funnels (which is how `jigc ingest` slipped
    /// through).
    #[test]
    #[should_panic(expected = "is serialized with no `key.target`")]
    fn serializing_a_targetless_finding_fires_the_seam() {
        let degenerate = Finding::blocking(
            "conformance.unknown-field",
            "undeclared field key `bogus-key`",
            Location::at(12, 1),
        );

        let _ = serde_json::to_string(&degenerate);
    }

    /// The **route floor's presence half at the seam** (M43, `design/surface-contract.md` →
    /// The route fence): `blocking ⇒ route present` — a route-less non-exempt blocking
    /// finding cannot be serialized **at all**. Same structural posture as the key seam:
    /// the assert rides [`Finding`]'s own `Serialize`, so there is no funnel list to keep
    /// current — a blocked gate that names no recovery is unrepresentable on any surface
    /// the suite exercises.
    #[test]
    #[should_panic(expected = "with no `route`")]
    fn serializing_a_routeless_blocking_finding_fires_the_seam() {
        let unrouted = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot in section `context` is empty",
            Some(Location::addressed("adr:pick-a-db#context", 1, 1)),
            None,
        );

        let _ = serde_json::to_string(&unrouted);
    }

    /// The route floor's **one-home exemption** (the [`is_declared_singleton`] pattern): a
    /// purely-positional parser `conformance.*` diagnostic serializes route-less — the
    /// located message *is* the repair — and non-blocking severities are outside the floor
    /// entirely (the advisory half is carried by the producers, not this assert).
    #[test]
    fn route_exempt_parser_diagnostics_and_non_blocking_pass_the_seam() {
        // A parser conformance diagnostic: blocking, route-less, exempt by its one-home fn.
        let parser_diagnostic = Finding::blocking(
            "conformance.heading-missing",
            "required section heading `## Decision` is missing",
            Location::addressed("adr:pick-a-db#decision", 1, 1),
        );
        assert!(is_route_exempt(&parser_diagnostic.code));
        let json = serde_json::to_value(&parser_diagnostic).expect("an exempt code projects");
        assert_eq!(json["route"], serde_json::Value::Null);

        // A non-blocking finding is outside the presence assert (the floor's advisory half
        // is a producer obligation, not a seam panic).
        let advisory = Finding::graded(
            Severity::Advisory,
            "schema-completeness.inverse-cardinality",
            "below the inverse-card minimum",
            Some(Location::addressed("prd:checkout#spec", 1, 1)),
            None,
        );
        let json = serde_json::to_value(&advisory).expect("a non-blocking finding projects");
        assert_eq!(json["route"], serde_json::Value::Null);
    }

    /// A **declared singleton** is the one legal `target: null` — its emission path yields at
    /// most one instance, so `(code, null)` is already unique-per-instance. It projects
    /// through the same seam untouched (the exception list is a list of *exceptions to a
    /// rule*, checkable from the code alone — not a list of *places to check*).
    #[test]
    fn serializing_a_declared_singleton_passes_the_seam() {
        let singleton = Finding::block(
            "setup.repo-root",
            "`jigc setup` must run inside a git repository",
            "run `git init` first",
        );

        let json = serde_json::to_value(&singleton).expect("a declared singleton projects");
        assert_eq!(json["key"]["target"], serde_json::Value::Null);
    }

    /// The **uniqueness half**, on the collection newtype: a [`Findings`] cannot be projected
    /// with two findings sharing one `(code, target)` — the property needs the *set*, so it
    /// lives on the only sanctioned way a set of findings serializes. Same structural bar as
    /// the presence half: you cannot hand a driver an array whose keys it cannot tell apart.
    #[test]
    #[should_panic(expected = "collide on one key")]
    fn serializing_colliding_findings_fires_the_seam() {
        let collide = |line| {
            Finding::graded(
                Severity::Blocking,
                "schema-conformance.required-slot-present",
                "required slot is empty",
                Some(Location::addressed("adr:pick-a-db", line, 1)),
                None,
            )
        };
        let findings = Findings::from(vec![collide(12), collide(20)]);

        let _ = serde_json::to_string(&findings);
    }

    /// The newtype moves **no wire shape**: a [`Findings`] projects as the plain JSON array the
    /// `Vec<Finding>` it replaced did — the check is invisible to every consumer.
    #[test]
    fn findings_project_as_the_plain_array() {
        let finding = Finding::blocking(
            "conformance.heading-missing",
            "required section heading `## Decision` is missing",
            Location::addressed("adr:pick-a-db#decision", 1, 1),
        );
        let wrapped = Findings::from(vec![finding.clone()]);

        assert_eq!(
            serde_json::to_value(&wrapped).expect("serializes"),
            serde_json::json!([serde_json::to_value(&finding).expect("serializes")]),
            "a findings collection is a plain array on the wire — the seam adds no key",
        );
        let back: Findings =
            serde_json::from_value(serde_json::to_value(&wrapped).expect("serializes"))
                .expect("deserializes");
        assert_eq!(back, wrapped, "and it round-trips");
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
            // Routed, as the production `dangling` constructor routes it (the route floor:
            // a serialized blocking finding names its recovery).
            Some("fix the reference, create the target in this task, or drop the field".into()),
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

    /// The M43 route fence, first slice: each [`Route`] kind projects to its **flat
    /// string** — byte-identical to the pre-M43 `Option<String>` wire, so no driver
    /// sees a shape change (the tagged union stays deferred,
    /// `implementation/decisions-pending.md` → `route` tagged-union promotion). A
    /// `Mechanical` route composes as the backticked argv + verbatim tail (empty tail
    /// adds nothing); `Human` / `Informational` are their text verbatim.
    #[test]
    fn route_variants_project_to_their_flat_string() {
        let mechanical =
            Route::mechanical(["jigc", "task", "list"], " to see every minted task id");
        assert_eq!(
            serde_json::to_value(&mechanical).expect("serializes"),
            serde_json::json!("`jigc task list` to see every minted task id"),
            "a mechanical route projects as the backticked argv + verbatim tail",
        );
        assert_eq!(
            serde_json::to_value(Route::mechanical(["jigc", "task", "list"], ""))
                .expect("serializes"),
            serde_json::json!("`jigc task list`"),
            "an empty tail appends nothing",
        );

        let human = Route::human("order the task that creates `adr:cache` first");
        assert_eq!(
            serde_json::to_value(&human).expect("serializes"),
            serde_json::json!("order the task that creates `adr:cache` first"),
            "a human route projects as its text verbatim",
        );

        let informational = Route::informational("no action needed — staying plain is correct");
        assert_eq!(
            serde_json::to_value(&informational).expect("serializes"),
            serde_json::json!("no action needed — staying plain is correct"),
            "an informational route projects as its text verbatim",
        );
    }

    /// The envelope path: a [`Finding`] carrying a `Mechanical` route serializes its
    /// `route` field as the flat string — on the wire, indistinguishable from a `Human`
    /// route with the same text (the internal kind is invisible until the wire union is
    /// promoted).
    #[test]
    fn finding_route_field_is_the_flat_string_for_every_kind() {
        let mechanical = Finding::graded(
            Severity::Blocking,
            "finalize.left-out",
            "the staged set leaves tracked changes out",
            Some(Location::addressed("adr:new#decision", 1, 1)),
            Some(Route::mechanical(
                ["jigc", "task", "validate", "<id>"],
                " to see what's left",
            )),
        );
        let json = serde_json::to_value(&mechanical).expect("serializes");
        assert_eq!(
            json["route"],
            serde_json::json!("`jigc task validate <id>` to see what's left"),
            "the envelope's route field is the flat string",
        );

        let mut as_human = mechanical.clone();
        as_human.route = Some(Route::human("`jigc task validate <id>` to see what's left"));
        assert_eq!(
            serde_json::to_value(&as_human).expect("serializes")["route"],
            json["route"],
            "a human route with the same text is byte-identical on the wire",
        );
    }

    /// `Deserialize` maps the flat wire string to [`RouteKind::Human`] — a reloaded
    /// `Mechanical` route **degrades to Human** and re-serializes byte-identical, so
    /// nothing is lost on the wire (the accepted internal-slice bound; the wire union
    /// that would preserve the kind stays deferred). `From<String>` is the same seam:
    /// an un-migrated producer's string route lands as `Human`.
    #[test]
    fn route_deserializes_to_human() {
        let mechanical = Route::mechanical(["jigc", "task", "list"], "");
        let wire = serde_json::to_string(&mechanical).expect("serializes");
        let back: Route = serde_json::from_str(&wire).expect("deserializes");
        assert!(matches!(back.kind(), RouteKind::Human));
        assert_eq!(back.as_str(), mechanical.as_str());
        assert_eq!(
            serde_json::to_string(&back).expect("serializes"),
            wire,
            "a reloaded route re-serializes byte-identical",
        );

        let from_string = Route::from("run the corpus migration".to_string());
        assert!(matches!(from_string.kind(), RouteKind::Human));
        assert_eq!(from_string.as_str(), "run the corpus migration");
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

    /// **The route-floor seam-sweep** — the load-bearing half of closing the route-fence
    /// class, discharging the rule the M43 retrospective minted (`DECISIONS.md` 2026-07-17 →
    /// the seam-sweep rule: *a fence landing at a seam owes, in the same increment, a sweep
    /// that pushes every existing producer through that seam — or an argued enumeration of the
    /// producers that cannot reach it*). The route floor's presence half rides
    /// [`Finding`]'s `Serialize`, so its strength is exactly the suite's *traffic* through the
    /// seam — and route-floor violations shipped under a green suite precisely because certain
    /// producers were never driven through serde (four at M43 completion; seven more found in
    /// the M43 surface census — all four `finalize.*` I/O faults, `doc-code.multi-valued-anchor`,
    /// `milestone.area-io`, `task.working-area-io`, two of which render only via `Display` and
    /// so **structurally cannot** reach the serialize seam).
    ///
    /// A per-producer census of *tests* would rot as producers are added. This closes the class
    /// at the **construction source** instead: it scans the engine's own production source for
    /// every `Finding::graded(Severity::Blocking, …, None)` and every `Finding::blocking(…)`
    /// (route-less by definition) and asserts each carries a route-exempt code. A new blocking
    /// producer that ships route-less — whether or not any test ever serializes it — fails here,
    /// so it must declare its bucket: route it, or (only if it is a purely-positional parser
    /// diagnostic) place its code under [`is_route_exempt`]. Bound: engine scope only (the
    /// contract's home crate; the CLI's few direct producers route through helpers), and it
    /// reads `code` string literals — a route-less blocking finding built with a non-literal
    /// code is flagged for manual bucketing rather than resolved.
    #[test]
    fn every_production_blocking_finding_is_routed_or_exempt() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect_rs(&src, &mut files);
        assert!(
            !files.is_empty(),
            "found no engine source to scan under {src:?}"
        );

        let mut examined = 0usize;
        let mut violations = Vec::new();
        for file in &files {
            let text = std::fs::read_to_string(file).expect("read source");
            let code = production_code(&text);
            scan_blocking_constructors(&code, file, &mut examined, &mut violations);
        }

        // Sanity floor: the engine builds dozens of blocking findings, so an empty
        // `violations` is only trustworthy if the source lexer actually reached them. If a
        // lexer regression silently dropped every call site, this fires before the emptiness
        // could read as a false all-clear.
        assert!(
            examined >= 30,
            "the seam-sweep scan examined only {examined} blocking constructors — the source \
             lexer likely regressed"
        );
        assert!(
            violations.is_empty(),
            "these PRODUCTION sites construct a route-less, non-exempt blocking finding — a \
             blocked gate that names no recovery (design/surface-contract.md → The route fence; \
             DECISIONS.md 2026-07-17 → the seam-sweep rule). Route each (name its recovery), or \
             — only for a purely-positional parser diagnostic — place its code under \
             `is_route_exempt`:\n{}",
            violations.join("\n"),
        );
    }

    /// Recursively collect `.rs` files under `dir` — the seam-sweep scan's file set.
    fn collect_rs(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("read dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                collect_rs(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }

    /// Neutralize a source byte that would confuse paren-/comma-matching if it hid inside a
    /// string or char literal — every structural byte becomes a space; a finding `code`
    /// literal (only letters, digits, `.`, `-`) is untouched.
    fn neutralize(c: u8) -> u8 {
        match c {
            b'(' | b')' | b'{' | b'}' | b'[' | b']' | b',' | b';' => b' ',
            _ => c,
        }
    }

    /// A view of Rust source with **comments and `#[cfg(test)] mod … {…}` blocks removed** and
    /// the structural bytes inside string / char literals neutralized (see [`neutralize`]) — so
    /// the constructor scan never trips on a brace in a test fixture, a comment, or a message
    /// string, while a finding's `code` literal survives verbatim. Not a full parser: it handles
    /// line / nested-block comments, normal and raw (`r#"…"#`) strings, and byte/char literals
    /// (disambiguated from lifetimes) — the whole alphabet the engine's source uses here.
    fn production_code(text: &str) -> String {
        let b = text.as_bytes();
        let n = b.len();
        let mut out = String::new();
        let mut i = 0usize;
        let mut depth = 0usize;
        // Brace depths at which an open `#[cfg(test)] mod … {` region began; source is test
        // code while any marker is live. A stack tolerates (rare) nesting.
        let mut test_markers: Vec<usize> = Vec::new();
        let mut pending_test = false;
        let is_ident = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
        while i < n {
            let in_test = !test_markers.is_empty();
            let c = b[i];
            // Line comment.
            if c == b'/' && i + 1 < n && b[i + 1] == b'/' {
                i += 2;
                while i < n && b[i] != b'\n' {
                    i += 1;
                }
                continue;
            }
            // Block comment (Rust block comments nest).
            if c == b'/' && i + 1 < n && b[i + 1] == b'*' {
                let mut d = 1usize;
                i += 2;
                while i < n && d > 0 {
                    if b[i] == b'/' && i + 1 < n && b[i + 1] == b'*' {
                        d += 1;
                        i += 2;
                    } else if b[i] == b'*' && i + 1 < n && b[i + 1] == b'/' {
                        d -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                continue;
            }
            // Raw string: `(b)? r #* "` … `" #*` (boundary-guarded so `for` / `br` in idents
            // do not match).
            {
                let boundary = i == 0 || !is_ident(b[i - 1]);
                let mut p = i;
                if boundary && p < n && b[p] == b'b' {
                    p += 1;
                }
                if boundary && p < n && b[p] == b'r' {
                    let mut h = 0usize;
                    let mut q = p + 1;
                    while q < n && b[q] == b'#' {
                        h += 1;
                        q += 1;
                    }
                    if q < n && b[q] == b'"' {
                        let mut k = q + 1;
                        loop {
                            if k >= n {
                                break;
                            }
                            if b[k] == b'"' {
                                let mut hh = 0usize;
                                while k + 1 + hh < n && b[k + 1 + hh] == b'#' {
                                    hh += 1;
                                }
                                if hh >= h {
                                    k = k + 1 + h;
                                    break;
                                }
                            }
                            k += 1;
                        }
                        if !in_test {
                            out.push(' ');
                        }
                        i = k;
                        continue;
                    }
                }
            }
            // Normal string.
            if c == b'"' {
                let mut k = i + 1;
                let mut buf = String::from('"');
                while k < n {
                    if b[k] == b'\\' && k + 1 < n {
                        buf.push('\\');
                        buf.push(neutralize(b[k + 1]) as char);
                        k += 2;
                        continue;
                    }
                    if b[k] == b'"' {
                        buf.push('"');
                        k += 1;
                        break;
                    }
                    buf.push(neutralize(b[k]) as char);
                    k += 1;
                }
                if !in_test {
                    out.push_str(&buf);
                }
                i = k;
                continue;
            }
            // Char / byte literal vs. lifetime.
            {
                let mut p = i;
                if c == b'b' && i + 1 < n && b[i + 1] == b'\'' {
                    p += 1;
                }
                if p < n && b[p] == b'\'' {
                    let is_char =
                        (p + 1 < n && b[p + 1] == b'\\') || (p + 2 < n && b[p + 2] == b'\'');
                    if is_char {
                        let mut k = p + 1;
                        if k < n && b[k] == b'\\' {
                            k += 2;
                        } else {
                            k += 1;
                        }
                        if k < n && b[k] == b'\'' {
                            k += 1;
                        }
                        if !in_test {
                            out.push(' ');
                        }
                        i = k;
                        continue;
                    }
                    // Otherwise a lifetime (`'a`, `'_`, `'static`) — fall through and emit `'`.
                }
            }
            // `#[cfg(test)]` — arm the next brace as a test-region opener.
            if b[i..].starts_with(b"#[cfg(test)]") {
                pending_test = true;
                i += "#[cfg(test)]".len();
                continue;
            }
            if c == b'{' {
                depth += 1;
                if pending_test {
                    test_markers.push(depth);
                    pending_test = false;
                }
                if test_markers.is_empty() {
                    out.push('{');
                }
                i += 1;
                continue;
            }
            if c == b'}' {
                if test_markers.is_empty() {
                    out.push('}');
                }
                if test_markers.last() == Some(&depth) {
                    test_markers.pop();
                }
                depth = depth.saturating_sub(1);
                i += 1;
                continue;
            }
            if !in_test {
                out.push(c as char);
            }
            i += 1;
        }
        out
    }

    /// Split a call's argument text on **top-level** commas (depth-0 across `()[]{}`), trimming
    /// and dropping the empty tail a trailing comma leaves.
    fn top_level_split(s: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut depth = 0i32;
        let mut last = 0usize;
        for (k, c) in s.char_indices() {
            match c {
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(s[last..k].to_string());
                    last = k + 1;
                }
                _ => {}
            }
        }
        parts.push(s[last..].to_string());
        parts
            .into_iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    }

    /// The first `"…"` literal inside `arg` (the finding `code`), or `None` for a non-literal.
    fn extract_str_lit(arg: &str) -> Option<String> {
        let start = arg.find('"')?;
        let rest = &arg[start + 1..];
        let end = rest.find('"')?;
        Some(rest[..end].to_string())
    }

    /// Find every `Finding::graded(…)` / `Finding::blocking(…)` in the production `code` view and
    /// record any that constructs a route-less, non-[`is_route_exempt`] **blocking** finding.
    fn scan_blocking_constructors(
        code: &str,
        file: &std::path::Path,
        examined: &mut usize,
        violations: &mut Vec<String>,
    ) {
        let bytes = code.as_bytes();
        for (marker, is_graded) in [("Finding::graded(", true), ("Finding::blocking(", false)] {
            let mut from = 0usize;
            while let Some(rel) = code[from..].find(marker) {
                let open = from + rel + marker.len() - 1; // index of the '('
                let mut d = 0i32;
                let mut j = open;
                let mut close = code.len();
                while j < bytes.len() {
                    match bytes[j] {
                        b'(' => d += 1,
                        b')' => {
                            d -= 1;
                            if d == 0 {
                                close = j;
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                from = (open + 1).max(close);
                let args = top_level_split(&code[open + 1..close]);
                *examined += 1;

                let is_blocking = if is_graded {
                    args.first()
                        .is_some_and(|a| a.contains("Severity::Blocking"))
                } else {
                    true
                };
                if !is_blocking {
                    continue;
                }
                let route_less = if is_graded {
                    args.last().is_some_and(|a| a == "None")
                } else {
                    // `Finding::blocking` is route-less by construction.
                    true
                };
                if !route_less {
                    continue;
                }
                let code_arg = if is_graded { args.get(1) } else { args.first() };
                let finding_code = code_arg.and_then(|a| extract_str_lit(a));
                let exempt = finding_code.as_deref().is_some_and(is_route_exempt);
                if !exempt {
                    let shown = finding_code.unwrap_or_else(|| "<non-literal code>".to_string());
                    violations.push(format!(
                        "  {}: {marker}… {shown}, route=None",
                        file.display()
                    ));
                }
            }
        }
    }
}
