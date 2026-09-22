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
        match &mut finding.location {
            Some(location) => location.address = Some(uri),
            None => finding.location = Some(Location::addressed(uri, 1, 1)),
        }
        // The route half of the flip (2026-07-17 surface-comprehension review, B1;
        // generalized to every derivable placeholder at M47's P6): a mechanical route
        // minted before the doc's identity was in hand carries `<PLACEHOLDER>`-class tokens
        // of the CLI-seam dummy table — and the identity just installed above determines
        // the derivable ones ([`ROUTE_PLACEHOLDERS`]), so they are rendered concrete and the
        // route becomes copy-runnable. Runs **after** the address lands, because the
        // substitution reads the finding's own `key.target`.
        finding.substitute_derivable_route_placeholders();
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
/// - `adapter-guide.user-modified` — its subject is **the adapter's owned artifact**, and a
///   profile declares at most one (`AdapterProfile.guide` is a single optional target), so
///   both doors that raise it (`jigc setup`, `jigc upgrade`) can emit at most one instance
///   per invocation. Its address is a repo path, not a managed-doc identity, so there is no
///   URI-normal-form target to carry either — the path rides the message and the route.
/// - `overrides.project-step-missing` — recorded into `CascadeStepSource`'s **one-slot**
///   error sink (`Option<Finding>`, last write wins) and drained as the verb's single
///   located error, so at most one instance reaches an output. Declared with the route
///   at the M43 completion audit, same rationale as the parse errors above.
/// - `task-discard.foreign-bytes` and `milestone.foreign-bytes` — the two non-`uninstall.`
///   members of M52 Increment 4's foreign-byte family, which the increment registers on the
///   **findings arm**. Each door's guard (`cli::task::refuse_over_foreign_bytes` and its
///   `cli::milestone` sibling) returns `Err` on the first of its two mutually-exclusive
///   producers — the fail-closed enumeration error or the listing — and the door bails, so
///   exactly one instance reaches an output, which is the `uninstall.*` rationale two bullets
///   up applied to the doors that share its subject. Enumerated, **not** namespaced:
///   `task-discard.` and `milestone.` both carry siblings (the staged-prose and
///   dirty-worktree guards) that keep the flattened arm and are not serialized as findings at
///   all, so a prefix here would exempt codes nobody has looked at. `milestone.foreign-bytes`
///   gained a **third** producer at M52 Increment 5 / T7 — the mint unwind, which can raise
///   one per seed area in a single `add-from-spec` — and that producer is **located at the
///   area** rather than admitted here: the exemption covers the two fail-fast guards whose
///   door bails on the first, and a producer that can emit several at once carries the target
///   that tells them apart.
///
/// - every `repo.*` — the repository-posture family, **at its `Here` site only**. The probe
///   returns the **first** breach of the checkout it is asked about and the asking door bails
///   on it (`cli::repo::adjudicated_breach`), so at most one instance per invocation reaches
///   an output — the `setup.*` / `uninstall.*` rationale, applied to a family whose subject is
///   *the repository the command was typed in* and which therefore has no address to carry.
///   At the family's **other** site the target exists and is carried: a breach in a fan-out
///   worktree (`cli::repo::BreachSite::FanOutWorktree`) is located at that worktree's
///   repo-relative path and lands in the **filesystem-path** form, so two breaching worktrees
///   are two discriminating keys. The split is by *site*, not by code, and both halves are
///   named here and at `design/command-output-contract.md` rather than one of them being left
///   to be discovered (the independent review of `986d5e0a`, MEDIUM 2).
///
/// Anything else with no address is the un-swept state of a family nobody has looked at, and
/// [`debug_assert_targets_declared`] says so at the seam.
pub fn is_declared_singleton(code: &str) -> bool {
    code == "store-version.binary-mismatch"
        || code == "adapter-guide.user-modified"
        || code.starts_with("setup.")
        || code.starts_with("uninstall.")
        || code.starts_with("structural-target.")
        || code.starts_with("slot-fill-target.")
        || code == "overrides.project-step-missing"
        || code == "task-discard.foreign-bytes"
        || code == "milestone.foreign-bytes"
        || code.starts_with("repo.")
}

/// The **route-exempt parse diagnostics** — one row per code, each with its reason, in the
/// order [`crate::parse`] raises them (frame → field group → item identity → slot prose).
/// This is the route floor's exemption list ([surface-contract.md](../../../design/surface-contract.md)
/// → The route fence; [validation.md](../../../design/validation.md) → The route floor), and
/// it is an **enumeration, not a namespace**: until M49 the predicate was
/// `starts_with("conformance.")`, so a code no producer emits was exempt by its *spelling*,
/// and a route-less blocking finding could be admitted at the seam by being named into the
/// family. Every member below is a real producer in `parse.rs`; the derivation arm beside the
/// route-floor sweep asserts the two sets agree.
///
/// **The criterion is *may* be route-less, and the whole family shares one reason.** Each of
/// these is raised by [`crate::parse::parse_sections`], which returns `Err` on *any* finding —
/// so a doc carrying one of them **does not parse**, and every `jigc doc` write verb resolves
/// its address through that same parse. A mechanical repair route here would name a command
/// that cannot run on the document it is printed about. What is left is the located message,
/// and since M49 that message is genuinely located on the surface an agent reads (`at:
/// <address> · line <n>`, `cli::render::finding_locus`) — the fact the exemption's rationale
/// always asserted and the agent text withheld until then. Exemption means *may*, not *must*:
/// a diagnostic that knows a **direction** takes a route and needs no row here, which is why
/// `conformance.duplicate-field` — repaired by deleting the stray line, and carrying the
/// shipped hand-repair sanction — is deliberately absent, and why the *same* code can be
/// routed at a different emission (the store sweep routes a **below-version** doc's frame
/// findings at `migrate`; `validate::store_sweep_routes_below_version_parse_failures_migrate`).
///
/// The rows, and what each message leaves the reader:
///
/// - `conformance.header-not-first`, `conformance.section-missing`,
///   `conformance.section-renamed` — the document's **frame**: the schema-fixed `##` heading
///   is absent, renamed, or the header section is not first. The message names the required
///   heading (or the found one), and no jigc verb rewrites a doc's own frame in place.
/// - `conformance.orphaned-sentinel`, `conformance.malformed-field-block`,
///   `conformance.unknown-field` — the **field group's bytes**: a `<!-- fields -->` sentinel
///   with no list under it, a bullet the field grammar cannot read, an undeclared key. The
///   message names the sentinel line, the malformed line, or the key **and the declared set
///   beside it** — and whether an undeclared key should be corrected or deleted is the
///   author's call, not a direction jigc holds.
/// - `conformance.item-heading-unanchored`, `conformance.item-anchor-malformed`,
///   `conformance.item-anchor-duplicate` — an item's **identity**: no `{#id}`, an `{#id}` that
///   is not a slug, or one id on two items. There is no address to route *at* — a write verb
///   takes an address, and the missing/invalid/ambiguous identity is exactly the defect. The
///   message names the heading and the depth reserved here, the bad anchor text, or the
///   duplicated id.
/// - `conformance.item-slot-label-missing`, `conformance.item-slot-delimiter-shadowed`,
///   `conformance.slot-setext-heading`, `conformance.slot-heading-depth` — a **slot's prose**:
///   a declared `####` sub-heading absent from a multi-slot item, or an authored line that
///   shadows the item-slot delimiter / sits at a schema-reserved depth. The message names the
///   offending line and the depth that *is* free at that address (context-derived, never a
///   global `####`), which is the whole repair.
const CONFORMANCE_PARSE_DIAGNOSTICS: &[&str] = &[
    "conformance.header-not-first",
    "conformance.section-missing",
    "conformance.section-renamed",
    "conformance.orphaned-sentinel",
    "conformance.malformed-field-block",
    "conformance.unknown-field",
    "conformance.item-heading-unanchored",
    "conformance.item-anchor-malformed",
    "conformance.item-anchor-duplicate",
    "conformance.item-slot-label-missing",
    "conformance.item-slot-delimiter-shadowed",
    "conformance.slot-setext-heading",
    "conformance.slot-heading-depth",
];

/// Whether `code` is **route-exempt** under the route floor — the floor's **one-home
/// exemption fn** (the [`is_declared_singleton`] pattern: a list of exceptions checkable
/// from the code alone, never a census of call sites). The floor
/// ([surface-contract.md](../../../design/surface-contract.md) → The route fence;
/// [validation.md](../../../design/validation.md) → The route floor) says `blocking ⇒
/// route present`, asserted on [`Finding`]'s `Serialize`; the exemption is exactly
/// [`CONFORMANCE_PARSE_DIAGNOSTICS`], whose doc-comment states the shared reason and what
/// each member's message leaves the reader.
///
/// **The hook-rejection identities are *not* an exemption, and since M52 they are not an
/// absence either.** They were both, on one rationale — an anyhow error path, never a
/// [`Finding`], so nothing here had to name them. M52 Increment 1 / T1 routes a reject that
/// carries a finding onto the findings envelope, so the ten `*.commit-rejected` identities
/// each reach this seam as a blocking [`Finding`] — and each carries a route, the frame's own
/// recovery sentence (what survived + this door's re-run), which is what the floor asks for.
/// git's stderr stays verbatim: it is the finding's `message`, not a wrap
/// ([finalize.md](../../../design/finalize.md)).
pub fn is_route_exempt(code: &str) -> bool {
    CONFORMANCE_PARSE_DIAGNOSTICS.contains(&code)
}

/// One row of the **route-placeholder derivability table** — the declaration behind **P6
/// route-followability** (M47; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8 + the
/// pre-decompose review's P6 rider). Sibling of [`is_route_exempt`] /
/// [`is_declared_singleton`]: a verdict checkable from the token alone, each with its
/// reason, never a census of call sites.
pub struct RoutePlaceholder {
    /// The `<PLACEHOLDER>`-class argv token — byte-identical to its row in the CLI-seam
    /// parse table (`crates/cli/src/route_fence.rs` → `DUMMY_SUBSTITUTIONS`), whose
    /// membership is fenced against this one there (the only place both are visible).
    pub token: &'static str,
    /// Whether the token's value is **derivable from the finding's own `key.target`** —
    /// and therefore *must* be substituted before the finding reaches a driver.
    pub derivable: bool,
    /// Why. A verdict without a reason is the un-swept state this table exists to end.
    pub reason: &'static str,
}

/// The **derivability table**: for every `<PLACEHOLDER>`-class token a mechanical route may
/// carry, whether the finding's own `key.target` determines it.
///
/// The M43 CLI-seam fence proves a mechanical route **parses**; it sits on
/// [`Route::mechanical`] and never sees the finding, so it cannot know whether the route an
/// agent reads is **followable**. A placeholder the finding could have filled and did not is
/// a route that names a command the driver cannot run — law 2's *nothing hides* failing at
/// the last inch. So: *derivable ⇒ substituted*, asserted on [`Finding`]'s `Serialize`
/// ([`Finding::route_placeholders_are_substituted`], the fourth assert on that seam).
///
/// The non-derivable rows are the honest half. `<task-id>` in particular is **not**
/// derivable under any rule — one doc is written from many tasks, so the address a finding
/// concerns names no task — and it therefore needs a *different* source (the CLI dispatch
/// that resolved the task supplies it at every enriched write-verb producer).
///
/// Rows are in the CLI-seam table's own order, so the two read as one; a test beside
/// `DUMMY_SUBSTITUTIONS` asserts exactly that (membership **and** order), because a token in
/// one table and not the other is a placeholder with no verdict or a verdict about a
/// placeholder no route may carry.
pub const ROUTE_PLACEHOLDERS: &[RoutePlaceholder] = &[
    RoutePlaceholder {
        token: "<task-id>",
        derivable: false,
        reason: "a task is not a property of the address: one doc is written from many \
                 tasks. It needs a different source — the CLI dispatch that resolved the \
                 task — so it stays outside the property rather than being faked from the \
                 target",
    },
    RoutePlaceholder {
        token: "<address>",
        derivable: true,
        reason: "the finding's own `key.target` — the URI-normal-form address the write \
                 grammar accepts verbatim; narrowed to the top showable hop on a `jigc doc \
                 show` route, which resolves a doc or a section and never the absent item a \
                 `write.not-present` names",
    },
    RoutePlaceholder {
        token: "<value>",
        derivable: false,
        reason: "the value is the agent's to author — the finding reports what is missing \
                 or malformed, never what it should become",
    },
    RoutePlaceholder {
        token: "<doctype>",
        derivable: true,
        reason: "the `<type>` head of `key.target` — every write / gate target is a \
                 `<type>:<slug>[#…]` URI, so the doctype whose schema answers the question \
                 is already carried",
    },
    RoutePlaceholder {
        token: "<milestone-id>",
        derivable: false,
        reason: "a milestone is a work unit, not a managed address; no doc target names one",
    },
    RoutePlaceholder {
        token: "<path>",
        derivable: false,
        reason: "a foreign source path is outside management by definition, so no managed \
                 target carries it",
    },
    RoutePlaceholder {
        token: "<type>",
        derivable: false,
        reason: "the doctype the agent chooses to provision next — deliberately not the \
                 doctype of the finding's own target (the `jigc doc create <type>` \
                 positional, named after that verb's usage line)",
    },
    RoutePlaceholder {
        token: "<intent>",
        derivable: false,
        reason: "free-text task intent — the agent's to write",
    },
    RoutePlaceholder {
        token: "<title>",
        derivable: false,
        reason: "free-text milestone title — the agent's to write; a title is author-owned \
                 prose, and no address determines it (the quoted-span form `\"<title>\"`, \
                 declared here since the CLI-seam fence learned to read it)",
    },
    RoutePlaceholder {
        token: "<key>",
        derivable: false,
        reason: "a cascade knob key names a config surface, not a managed document; no \
                 doc target carries one (the `jigc config get <key>` positional, reached \
                 by the read-shaped `config show` tip)",
    },
    RoutePlaceholder {
        token: "<workflow-id>",
        derivable: false,
        reason: "a workflow is not a property of the address — it is the door's own \
                 argument, and the finding that carries this token concerns no document at \
                 all. The set it must be filled from is named in the message instead, which \
                 is the only place the caller's choice can come from",
    },
];

/// Whether `token` is a placeholder a finding's own `key.target` determines
/// ([`ROUTE_PLACEHOLDERS`]). An unknown token is **not** derivable: the CLI-seam parse fence
/// already refuses an undeclared placeholder at construction, so this predicate stays a pure
/// lookup rather than a second gate.
pub fn is_derivable_route_placeholder(token: &str) -> bool {
    ROUTE_PLACEHOLDERS
        .iter()
        .any(|row| row.token == token && row.derivable)
}

/// The concrete value `token` takes on a route whose finding targets `target`, or `None`
/// when the target does not carry it (a non-URI target has no `<doctype>` head). `argv` is
/// the route's own argv, because the **verb decides what address it can accept**: see
/// [`showable_address`].
fn derive_route_placeholder(token: &str, target: &str, argv: &[String]) -> Option<String> {
    // A derived value that is not already [`shell_safe`] is quoted (M51 completion audit):
    // a `key.target` is a URI in the ordinary case, but the **path form** is a declared
    // target shape (`file-state.*`, `ingest.*`, `schema-conformance.unadopted-instance`),
    // and a substitution is written straight into an argv whose join IS the emitted bytes. A
    // repo-relative path holding a space or a quote would otherwise re-lex as two arguments
    // — the substitution turning a placeholder route into a broken concrete one.
    let derived = match token {
        "<address>" if is_show_route(argv) => Some(showable_address(target)),
        "<address>" => Some(target.to_owned()),
        "<doctype>" => target.split_once(':').map(|(head, _)| head.to_owned()),
        _ => None,
    };
    derived.map(|value| shell_operand(&value))
}

/// Whether `argv` is a `jigc doc show …` read route (argv\[0\] is the leading `jigc` the
/// fence requires).
fn is_show_route(argv: &[String]) -> bool {
    argv.get(1).map(String::as_str) == Some("doc")
        && argv.get(2).map(String::as_str) == Some("show")
}

/// The **top showable hop** of `target`: `<type>:<slug>#<section>` for a deeper fragment,
/// the target itself otherwise. A read verb resolves a doc or one of its sections; the
/// address a `write.not-present` carries is precisely the node that is *absent*, so echoing
/// it into a `doc show` route would mint a concrete dead end — worse than the placeholder it
/// replaced. This is the same containing-section read the CLI's `write.not-present`
/// enrichment computes, so the engine's defensive fallback and the enriched route agree
/// byte-for-byte on the address (`design/validation.md` → the `write.*` route split).
fn showable_address(target: &str) -> String {
    match target.split_once('#') {
        Some((head, fragment)) => match fragment.split('/').next() {
            Some(top) if !top.is_empty() => format!("{head}#{top}"),
            _ => head.to_owned(),
        },
        None => target.to_owned(),
    }
}

/// Whether `code` is a **declared non-unique exception** — a code whose key is *deliberately
/// collapsed below its subject's granularity*, so two instances sharing one `(code, target)`
/// in one emitted slice are the pin, not a defect
/// ([command-output-contract.md](../../../design/command-output-contract.md) → The declared
/// non-unique exceptions). Each carries a **non-null** target; what it forgoes is the
/// sub-discriminator, and each forgoes it for a stated reason — this is its **one home**:
///
/// - `conformance.item-heading-unanchored` — an item heading with no anchor **has no
///   identity**: the key that would discriminate is precisely the thing the finding reports
///   missing. (M45 — the code that carried this row was `item-anchor-missing`; it names the
///   reserved-depth *cause* now, and the key is unchanged because the reason is.)
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
        "conformance.item-heading-unanchored"
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
        fence_command_spans(&text);
        Self {
            text,
            kind: RouteKind::Mechanical { argv, tail },
        }
    }

    /// A human-judgment direction — the flat text verbatim.
    pub fn human(text: impl Into<String>) -> Self {
        let text = text.into();
        fence_command_spans(&text);
        Self {
            text,
            kind: RouteKind::Human,
        }
    }

    /// A "no action needed" notice — the flat text verbatim.
    pub fn informational(text: impl Into<String>) -> Self {
        let text = text.into();
        fence_command_spans(&text);
        Self {
            text,
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

/// Render one emitted command-line token into bytes a shell re-lexes as **exactly itself**:
/// **bare** when every byte is shell-inert (`[A-Za-z0-9._/@=:+-]`, the alphabet ids, slugs,
/// addresses and flags live in), **POSIX single-quoted** otherwise — `'` written `'\''`, the
/// one quoting form under which a shell performs no expansion at all.
///
/// **This is the one home of the quoting rule** (M51 completion audit). It lived in
/// `cli::task` while the engine mints routes of its own — `file_state`'s conflict block,
/// `validate`'s owner-artifact `git add`, `finalize`'s `git restore --staged` — so every
/// engine-side producer interpolated its path raw and the rule was enforced in exactly the
/// half of the codebase that did not need it. Quoting is a **lexical property of the emitted
/// bytes** and needs no CLI knowledge (unlike the *parse* half of the route fence, which
/// genuinely needs clap and stays injected from the CLI seam), so it belongs beside the
/// constructor that composes a route's text. `cli::task::shell_token` re-exports this.
///
/// The tokens this embeds are **author-owned prose and filesystem paths** — a milestone
/// title, a sub-task intent, a foreign file an operator named — i.e. input these doors are
/// designed to receive, not exotica. Double-quoting (the pre-M47-fix form) leaves `$` and
/// command substitution live and emits a whitespace-free token bare, so `Cache $HOME rework`
/// re-run as printed **exits 0 having created a different artifact** than the frame says it
/// recovers. The fence is a real shell: see `cli::task`'s
/// `shell_token_round_trips_through_a_real_shell_over_the_metachar_axis`.
pub fn shell_token(arg: &str) -> String {
    let inert = |c: char| {
        c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '/' | '@' | '=' | ':' | '+' | '-')
    };
    if !arg.is_empty() && arg.chars().all(inert) {
        arg.to_owned()
    } else {
        format!("'{}'", arg.replace('\'', r"'\''"))
    }
}

/// Render one emitted command-line **operand that may already be inert** — a `<type>:<slug>`
/// address, a repo-relative path — leaving it exactly as written when a shell re-lexes it as
/// itself, and quoting it otherwise.
///
/// It exists because [`shell_token`] and [`shell_safe`] disagree on one byte, deliberately:
/// `#` is an address fragment separator that a shell treats literally mid-word, so
/// `shell_safe` admits `adr:pick-a-db#context` bare — but `#` at the *start* of a word opens
/// a comment, and `shell_token`'s subject is author prose, where a title of exactly `#42`
/// would then vanish. Neither predicate is wrong; they answer about different subjects.
///
/// So: **author prose goes through [`shell_token`]** (quote first, ask later), and **an
/// address or path an inert grammar already governs goes through this** — which keeps every
/// shipped route byte-identical while still quoting the one that needs it. Using
/// `shell_token` here instead would re-spell `adr:x#context` as `'adr:x#context'` on every
/// surface that prints an address, for a safety none of them lacked.
pub fn shell_operand(value: &str) -> String {
    if shell_safe(value) {
        value.to_owned()
    } else {
        shell_token(value)
    }
}

/// Whether one emitted command-line token survives a real shell as **exactly itself** — the
/// predicate [`shell_token`] satisfies, and the quoting half of the M43 route fence.
///
/// Two accepted shapes and nothing else: **bare** (every byte in the shell-inert charset
/// `[A-Za-z0-9._:#/=@+-]`, the set `compose`'s `Run:`-line renderer already declares — ids,
/// slugs, addresses incl. a `#fragment`, flags, `-` for stdin), or **POSIX single-quoted**
/// with an embedded `'` written `'\''`.
pub fn shell_safe(token: &str) -> bool {
    let inert = |c: char| c.is_ascii_alphanumeric() || "._:#/=@+-".contains(c);
    if !token.is_empty() && token.chars().all(inert) {
        return true;
    }
    let Some(inner) = token
        .strip_prefix('\'')
        .and_then(|rest| rest.strip_suffix('\''))
    else {
        return false;
    };
    // Inside the quotes, a `'` may appear only as the close-escape-reopen splice.
    !inner.split(r"'\''").any(|span| span.contains('\''))
}

/// The **span fence**: the first token of a backticked `git …` / `jigc …` span in `text`
/// that a shell would not re-lex as itself, if any.
///
/// The M43 route fence checked a **mechanical** route's argv, token by token. It could not
/// see the other half of the same surface: a [`Route::human`] naming a command *in prose*
/// (`` stage it with `git add -- {path}` ``), and the prose **tail** a mechanical route
/// carries. Both are bytes an agent pastes into a shell, and both were interpolating raw
/// paths — driven at the M51 completion audit, a foreign `my notes.md` produced
/// `` `git add -- my notes.md` `` (exit 128, pathspec `my`) and `` `jigc migrate my notes.md
/// --as adr` `` (exit 2), a route that dead-ends when followed verbatim.
///
/// So the check moves from the argv to the **composed text**, which is where membership in
/// "bytes the reader runs" is actually decided: every route of every kind passes through
/// one of three constructors, and each hands its finished text here.
///
/// A span is a command iff its first token is `git` or `jigc` — a backticked `` `--force` ``
/// or `` `docs/decisions/x.md` `` names a flag or a file, not a command line. Within a
/// command span a `<placeholder>` the reader fills is legal in either the bare (`<task-id>`)
/// or quoted-span (`"<New Title>"`) form, exactly as the mechanical fence's
/// dummy-substitution seam accepts them; everything else must be [`shell_safe`].
fn unsafe_command_token(text: &str) -> Option<String> {
    for span in backticked_spans(text) {
        let mut tokens = command_tokens(span);
        let Some(head) = tokens.next() else { continue };
        if head != "git" && head != "jigc" {
            continue;
        }
        for token in tokens {
            // Everything the reader supplies — a `<…>` group, an elision — is removed
            // before the token is judged, so what is checked is the bytes actually emitted:
            // a token that is *nothing but* placeholder (`<task-id>`) or elision (`…`)
            // reduces to empty, and a placeholder glued to literal bytes
            // (`<doc-address>#context/date`) reduces to the literal half. This is the shape
            // the route prose itself calls "a form to fill, not a runnable command".
            let stripped = strip_unemitted(&token);
            if stripped.is_empty() || shell_safe(&stripped) || inert_double_quoted(&token) {
                continue;
            }
            return Some(token);
        }
    }
    None
}

/// `token` with everything that is **not emitted bytes** removed — every `<…>` placeholder
/// group the reader fills, and every elision (`…` / `...`) standing for arguments the prose
/// declined to spell. What is left is what a shell would actually receive, which is what the
/// fence is about.
///
/// The elision leg is the same judgment as the placeholder leg, in the other notation: a
/// route reading `` re-record the delta (e.g. `jigc config replace-step …`) `` is naming a
/// verb, not handing over a command line — it is an *example*, and a fence that reads `…` as
/// an argument reports a defect in a sentence that has none.
///
/// An unclosed `<` removes nothing: a malformed placeholder is judged as written rather than
/// silently swallowing the rest of the token.
fn strip_unemitted(token: &str) -> String {
    let mut out = String::with_capacity(token.len());
    let mut rest = token;
    while let Some(open) = rest.find('<') {
        let Some(close) = rest[open..].find('>') else {
            break;
        };
        out.push_str(&rest[..open]);
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    out.replace('\u{2026}', "").replace("...", "")
}

/// Whether `token` is a **double-quoted literal a shell re-lexes as its own inner bytes** —
/// the one acceptance the span fence grants that [`shell_safe`] does not.
///
/// The two predicates differ because their subjects do. [`shell_safe`] governs an **argv
/// token jigc generated**, where the M48 rule is absolute: prose goes in single quotes,
/// because `format!("{title:?}")` — the shape that produced `--to "Cache $HOME rework"` —
/// double-quotes by default and leaves `$` and `$( … )` live. A **prose command span** is
/// hand-written, and a hand-written double-quoted example is both idiomatic and correct:
/// `git config user.email "you@example.com"` runs as itself, and refusing it would push the
/// author to reword a line that was never broken.
///
/// So the rule is the shell's own: inside double quotes exactly `$`, `` ` ``, `\` and `"` are
/// still active. A token carrying none of them re-lexes to its inner bytes and nothing else;
/// one carrying any of them is the defect this fence exists for, quoting style regardless.
fn inert_double_quoted(token: &str) -> bool {
    token
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .is_some_and(|inner| !inner.contains(['$', '`', '\\', '"']))
}

/// The backticked spans of `text` — the odd-indexed pieces of a split on `` ` ``. An
/// unpaired trailing backtick opens no span, which is the conservative reading.
fn backticked_spans(text: &str) -> Vec<&str> {
    let mut pieces = text.split('`');
    let _ = pieces.next();
    let mut spans: Vec<&str> = Vec::new();
    while let Some(span) = pieces.next() {
        spans.push(span);
        if pieces.next().is_none() {
            spans.pop();
            break;
        }
    }
    spans
}

/// Split one backticked span into shell words, keeping a `'…'`, `"…"` or `<…>` group whole
/// — so a quoted token carrying a space (`'my notes.md'`, `"<New Title>"`) is one word,
/// which is the whole point of the quoting the fence is checking for, and so is an
/// **unquoted multi-word placeholder** (`--to <a title>`), which is a shape the reader
/// replaces rather than bytes anyone pastes. Splitting the latter on whitespace reports
/// `<a` as an unsafe token and refuses a line that was never broken.
fn command_tokens(span: &str) -> impl Iterator<Item = String> + '_ {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut started = false;
    // A backslash outside quotes escapes the next byte into the word. Without it the
    // tokenizer splits `'\''` — the close-escape-reopen splice [`shell_token`] ITSELF emits
    // for an embedded `'` — into three words and refuses its own correct output.
    let mut escaped = false;
    for ch in span.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            started = true;
            continue;
        }
        match quote {
            Some(q) => {
                current.push(ch);
                if ch == q {
                    quote = None;
                }
            }
            None if ch == '\\' => {
                current.push(ch);
                escaped = true;
                started = true;
            }
            None if ch == '\'' || ch == '"' => {
                quote = Some(ch);
                current.push(ch);
                started = true;
            }
            // An angle-bracket span is a placeholder unit in this prose grammar, whether or
            // not it holds a space; it closes at `>`.
            None if ch == '<' && !started => {
                quote = Some('>');
                current.push(ch);
                started = true;
            }
            None if ch.is_whitespace() => {
                if started {
                    words.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            None => {
                current.push(ch);
                started = true;
            }
        }
    }
    if started {
        words.push(current);
    }
    words.into_iter()
}

/// Fire the span fence at construction (debug posture, the seam-assert class — it rides the
/// suite, never a release-build panic, exactly like its parse sibling).
///
/// It is installed on **all three** constructors and not only on `mechanical`, because the
/// bytes a reader pastes do not care which kind minted them: the M51 audit's two driven
/// dead ends were a `human` route and a `mechanical` route's raw argv token, and a
/// mechanical route's prose **tail** can name a second command line the argv check never
/// sees.
fn fence_command_spans(text: &str) {
    #[cfg(debug_assertions)]
    if let Some(token) = unsafe_command_token(text) {
        panic!(
            "a route's backticked command span must be copy-runnable: token `{token}` is \
             not shell-safe as emitted — a path or an author-owned prose token embedded in \
             an emitted command line must be rendered through \
             `engine::finding::shell_token` (route text: {text:?}; \
             design/surface-contract.md \u{2192} The route fence)"
        );
    }
    #[cfg(not(debug_assertions))]
    let _ = text;
}

/// The span fence, asked as a **question** rather than as an assertion — `true` iff every
/// backticked `git …` / `jigc …` span in `text` is copy-runnable. Producers that *derive* a
/// route's text and cannot know in advance ask here, in every posture.
pub fn command_spans_are_shell_safe(text: &str) -> bool {
    unsafe_command_token(text).is_none()
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
        // P6 route-followability (M47) — the fourth assert on this seam: a mechanical route
        // may not reach a driver carrying a derivable placeholder. The parse fence proves
        // the argv runs; this proves it runs *here*, at the address the finding names.
        debug_assert!(
            self.route_placeholders_are_substituted(),
            "finding `{}` is serialized with a mechanical route carrying a derivable \
             placeholder — fill it from `key.target` where the target is established \
             (`Finding::substitute_derivable_route_placeholders`), or declare the \
             placeholder non-derivable with its reason (engine `finding::ROUTE_PLACEHOLDERS`; \
             design/surface-contract.md → The route fence)",
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

    /// **Fill every derivable placeholder on this finding's mechanical route** from its own
    /// `key.target` ([`ROUTE_PLACEHOLDERS`]) — the P6 substitution, applied wherever a
    /// target is established ([`readdress_to_uri`]; the CLI's write-failure target stamp).
    /// A target-less finding, a non-mechanical route, and a route carrying only
    /// non-derivable placeholders are all left exactly as they are.
    ///
    /// The route is rebuilt through the checked [`Route::mechanical`] constructor, so the
    /// CLI-seam parse fence re-adjudicates the **concrete** argv — a substitution that
    /// produced an unrunnable command could not be constructed.
    pub fn substitute_derivable_route_placeholders(&mut self) {
        let Some(target) = self.key().target else {
            return;
        };
        let Some(route) = &self.route else { return };
        let RouteKind::Mechanical { argv, tail } = route.kind() else {
            return;
        };
        if !argv.iter().any(|arg| is_derivable_route_placeholder(arg)) {
            return;
        }
        let substituted: Vec<String> = argv
            .iter()
            .map(|arg| {
                if is_derivable_route_placeholder(arg) {
                    derive_route_placeholder(arg, &target, argv).unwrap_or_else(|| arg.clone())
                } else {
                    arg.clone()
                }
            })
            .collect();
        self.route = Some(Route::mechanical(substituted, tail.clone()));
    }

    /// Whether this finding honours **P6 route-followability**: no token of its mechanical
    /// route's argv is a placeholder its own `key.target` could have filled
    /// ([`ROUTE_PLACEHOLDERS`]). Asserted on every projection by [`Finding`]'s `Serialize` —
    /// the fourth assert on that seam, and the one the M43 parse fence structurally cannot
    /// make (it never sees the finding).
    ///
    /// A finding with no target has nothing to derive from and passes; the non-derivable
    /// placeholders (`<task-id>` foremost) are outside the property by declaration, and
    /// human-route prose spans are outside it entirely — a route's *kind* decides whether it
    /// carries an argv at all.
    pub fn route_placeholders_are_substituted(&self) -> bool {
        let Some(route) = &self.route else {
            return true;
        };
        let RouteKind::Mechanical { argv, .. } = route.kind() else {
            return true;
        };
        self.key().target.is_none() || !argv.iter().any(|arg| is_derivable_route_placeholder(arg))
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
        fence_addressed_token_is_quoted(location.as_ref(), route.as_ref());
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

/// The **subject half** of the emitted-command fence (debug posture, M51 completion audit):
/// when this finding's own located address is a token a shell would not re-lex as itself, a
/// backticked `git …` / `jigc …` span in its route that names that token must name it
/// **quoted**.
///
/// [`unsafe_command_token`] alone cannot catch the commonest spelling of this defect,
/// because it checks *tokens* and an unquoted path with a space is not one bad token — it is
/// two perfectly inert ones. `` `git add -- my notes.md` `` passes every token check and
/// exits 128 when run (pathspec `my`). The missing information is the **word boundary**, and
/// the finding carries it: `Location::address` is the very token the route is talking about
/// ([`unadopted_instance`]'s path form, `file-state.*`'s, `ingest.*`'s). So the check is
/// derived from the finding's own subject rather than guessed from the text.
///
/// Inert wherever the address is already shell-safe, which is every ordinary corpus — it
/// fires only on the class it exists to catch.
///
/// **Declared bound:** it sits at construction, so a producer that assigns `finding.route`
/// *after* `graded` (the CLI's route-enrichment sites) is outside it. Those are still
/// covered by the span fence at the `Route` constructor, which is the half that does not
/// need the subject — what they forgo is only the word-boundary claim.
fn fence_addressed_token_is_quoted(location: Option<&Location>, route: Option<&Route>) {
    #[cfg(debug_assertions)]
    {
        let (Some(address), Some(route)) = (
            location.and_then(|l| l.address.as_deref()),
            route.map(Route::as_str),
        ) else {
            return;
        };
        if shell_safe(address) {
            return;
        }
        let quoted = shell_token(address);
        for span in backticked_spans(route) {
            let mut tokens = command_tokens(span);
            let Some(head) = tokens.next() else { continue };
            if (head != "git" && head != "jigc") || !span.contains(address) {
                continue;
            }
            assert!(
                span.contains(quoted.as_str()),
                "a route's command span names this finding's own subject `{address}`, which \
                 a shell does not re-lex as itself — it must be rendered through \
                 `engine::finding::shell_token` (as `{quoted}`); span: `{span}` \
                 (design/surface-contract.md \u{2192} The route fence)"
            );
        }
    }
    #[cfg(not(debug_assertions))]
    {
        let _ = (location, route);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The emitted-command quoting rule, over the shapes that break it** (M51 completion
    /// audit): every token [`shell_token`] renders survives a shell as exactly itself, and
    /// every token it leaves bare needed no quoting. The alphabet left bare is the one
    /// `compose`'s `Run:` renderer already declares, so an id, a slug, an address with a
    /// `#fragment` and a flag stay readable.
    #[test]
    fn shell_token_renders_every_unsafe_shape_into_a_token_a_shell_re_lexes_as_itself() {
        for raw in [
            "my notes.md",
            "it's an odd name.md",
            "*.md",
            ":colon.md",
            "Cache $HOME rework",
            "Cache $(touch PWNED) rework",
            "a\nnewline.md",
            "",
        ] {
            let token = shell_token(raw);
            assert!(
                shell_safe(&token),
                "`shell_token({raw:?})` must be shell-safe; got `{token}`"
            );
        }
        for bare in ["docs/decisions/x.md", "--to", "-", "adr", "task-id-1"] {
            assert_eq!(shell_token(bare), bare, "`{bare}` needs no quoting");
        }
        // The two predicates agree everywhere except `#`, which [`shell_safe`] reads bare
        // (an address fragment) and [`shell_token`] quotes. Quoting more than necessary is
        // never unsafe, so the asymmetry is left as it shipped and stated here instead of
        // silently widened — the one caller that would have re-spelled every address,
        // [`derive_route_placeholder`], asks `shell_safe` first.
        assert!(shell_safe("adr:pick-a-db#context"));
        assert_eq!(
            shell_token("adr:pick-a-db#context"),
            "'adr:pick-a-db#context'"
        );
    }

    /// **The span fence sees a command wherever it is written, not only in an argv.** A
    /// backticked `git …` / `jigc …` span carrying a metachar a shell would act on is the
    /// defect; a bare `` `--force` `` or a backticked file name is not a command span at all;
    /// and a `<placeholder>` the reader fills is legal bare or as a quoted span.
    #[test]
    fn the_span_fence_reads_backticked_command_spans_and_nothing_else() {
        for unsafe_text in [
            "stage it with `git add -- it's an odd name.md`",
            "run `jigc migrate *.md --as changelog`",
            "re-run `jigc doc rename adr:x --to \"Cache $HOME rework\"`",
        ] {
            assert!(
                !command_spans_are_shell_safe(unsafe_text),
                "the fence must refuse: {unsafe_text}"
            );
        }
        for ok in [
            "stage it with `git add -- 'it'\\''s an odd name.md'`",
            "run `jigc migrate <path> --as <doctype>`",
            "adopt it: `jigc rename adr:x --to \"<New Title>\"`",
            "pass `--carry-staged`, or restore `docs/decisions/x.md`",
            "an unpaired ` backtick opens no span and git add -- x is not one",
            // A hand-written double-quoted example, which is what a *prose* span is for:
            // nothing inside it is still active to a shell, so it runs as itself. The argv
            // rule is stricter for a reason that does not apply here — see
            // [`inert_double_quoted`].
            "tell git who you are — `git config user.email \"you@example.com\"`",
            // A multi-word placeholder, written unquoted because it is a SHAPE the reader
            // replaces — `<a title>` is one unit, not the two tokens `<a` and `title>`.
            "name the id yourself: `jigc rename adr:x --to <a title> --slug <new-slug>`",
            // A placeholder GLUED to literal bytes — a form to fill, which is what the
            // unfilled-leaf advisory's own prose calls it. Only the literal half is emitted,
            // so only the literal half is judged.
            "set one with `jigc doc set-field <doc-address>#context/date --value <value>`",
            // An elision names a verb rather than handing over a command line.
            "re-record the delta (e.g. `jigc config replace-step …`) to pin a basis",
        ] {
            assert!(
                command_spans_are_shell_safe(ok),
                "the fence must admit: {ok}"
            );
        }
    }

    /// **The subject half, driven** — the spelling the token check structurally cannot see.
    /// An unquoted path holding a space is two inert tokens, so `` `git add -- my notes.md` ``
    /// passes [`command_spans_are_shell_safe`] and exits 128 when run; the finding's own
    /// located address is the word boundary that says so, and [`Finding::graded`] asserts it.
    #[test]
    #[should_panic(expected = "which a shell does not re-lex as itself")]
    fn a_route_naming_its_own_unsafe_subject_unquoted_fires_the_subject_fence() {
        let path = "my notes.md";
        assert!(
            command_spans_are_shell_safe(&format!("`git add -- {path}`")),
            "the token check is blind here — that blindness is what this fence covers"
        );
        let _ = Finding::graded(
            Severity::Blocking,
            "migrate.source-untracked",
            "untracked",
            Some(Location::addressed(path, 1, 1)),
            Some(Route::human(format!("stage it with `git add -- {path}`"))),
        );
    }

    /// The same subject, quoted at the producer: the finding constructs, and the route is
    /// the bytes that actually run.
    #[test]
    fn the_same_route_constructs_once_its_subject_is_rendered_through_shell_token() {
        let path = "my notes.md";
        let finding = Finding::graded(
            Severity::Blocking,
            "migrate.source-untracked",
            "untracked",
            Some(Location::addressed(path, 1, 1)),
            Some(Route::human(format!(
                "stage it with `git add -- {}`",
                shell_token(path)
            ))),
        );
        assert_eq!(
            finding.route.expect("routed").as_str(),
            "stage it with `git add -- 'my notes.md'`"
        );
    }

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

    /// **P6 route-followability, the property itself** (M47 Inc 6 T3; `DECISIONS.md` →
    /// 2026-07-26 M47 Settle, Decision 8 + the pre-decompose review's P6 rider): the
    /// **fourth** assert on the finding-serialization seam. The CLI-seam parse fence
    /// proves a mechanical route *parses*; it can say nothing about whether the route an
    /// agent reads is **followable**, because it never sees the finding. This assert does:
    /// a mechanical route may not reach a driver carrying a placeholder the finding's own
    /// `key.target` could have filled.
    #[test]
    #[should_panic(expected = "derivable placeholder")]
    fn serializing_a_route_that_keeps_a_derivable_placeholder_fires_the_seam() {
        let unsubstituted = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot in section `decision` is empty",
            Some(Location::addressed("adr:use-sqlite#decision", 3, 1)),
            Some(Route::mechanical(
                ["jigc", "doc", "set-slot", "<address>", "--from-file", "-"],
                " to fill the empty slot",
            )),
        );

        let _ = serde_json::to_string(&unsubstituted);
    }

    /// **The honest carve-out**: `<task-id>` is *not* derivable from `key.target` under any
    /// rule — one doc is written from many tasks, so the address the finding concerns does
    /// not name a task. It therefore needs a different source (the CLI dispatch that
    /// resolved the task) and stays **outside** the property: the identical finding,
    /// carrying `<task-id>` instead of `<address>`, projects clean and keeps its
    /// placeholder verbatim.
    #[test]
    fn a_task_id_placeholder_is_outside_the_property_and_projects_verbatim() {
        let carve_out = Finding::graded(
            Severity::Blocking,
            "write.not-present",
            "write rejected: item `9-9-9` in section `releases` not present",
            Some(Location::addressed("changelog:changelog#releases", 3, 1)),
            Some(Route::mechanical(
                [
                    "jigc",
                    "doc",
                    "show",
                    "changelog:changelog#releases",
                    "--task",
                    "<task-id>",
                ],
                " to see the section's current item ids",
            )),
        );

        let json = serde_json::to_string(&carve_out).expect("the carve-out projects");
        assert!(
            json.contains("<task-id>"),
            "`<task-id>` is declared non-derivable, so it survives the seam verbatim; got:\n{json}",
        );
    }

    /// The two tables are **one membership set** stated twice — once as *what may appear*
    /// (the CLI-seam `DUMMY_SUBSTITUTIONS` parse table) and once as *what must be filled*
    /// (this crate's derivability table). This half asserts every declared row carries a
    /// non-empty reason and that exactly the two derivable rows are derivable; the
    /// cross-table membership fence lives beside `DUMMY_SUBSTITUTIONS`, the only place
    /// both are visible (`crates/cli/src/route_fence.rs`).
    #[test]
    fn the_derivability_table_declares_a_verdict_and_a_reason_per_row() {
        let derivable: Vec<&str> = ROUTE_PLACEHOLDERS
            .iter()
            .filter(|row| row.derivable)
            .map(|row| row.token)
            .collect();
        assert_eq!(
            derivable,
            ["<address>", "<doctype>"],
            "exactly the two placeholders a `key.target` determines are derivable",
        );
        for row in ROUTE_PLACEHOLDERS {
            assert!(
                row.token.starts_with('<') && row.token.ends_with('>'),
                "a row's token is `<PLACEHOLDER>`-class; got `{}`",
                row.token,
            );
            assert!(
                !row.reason.trim().is_empty(),
                "row `{}` declares a verdict with no reason",
                row.token,
            );
        }
    }

    /// The **derivation rules**, exercised through the shared method: `<address>` takes the
    /// finding's own `key.target`, `<doctype>` its `<type>` head — and on a `jigc doc show`
    /// route the address is **narrowed to the top showable hop**, because a read verb
    /// resolves a doc or a section, never the absent item a `write.not-present` names.
    #[test]
    fn the_shared_substitution_fills_address_and_doctype_from_the_target() {
        let mut shape_question = Finding::graded(
            Severity::Blocking,
            "write.wrong-shape",
            "write rejected: section `summary` is not repeatable",
            Some(Location::addressed("commit:log-it#summary", 1, 1)),
            Some(Route::mechanical(
                ["jigc", "doc", "schema", "<doctype>"],
                " to see the declared shape",
            )),
        );
        shape_question.substitute_derivable_route_placeholders();
        assert_eq!(
            shape_question.route.as_ref().map(Route::as_str),
            Some("`jigc doc schema commit` to see the declared shape"),
        );

        let mut deep_miss = Finding::graded(
            Severity::Blocking,
            "write.not-present",
            "write rejected: item `nonesuch` not present",
            Some(Location::addressed(
                "changelog:changelog#releases/1-3-0/changes/nonesuch/notes",
                1,
                1,
            )),
            Some(Route::mechanical(
                ["jigc", "doc", "show", "<address>", "--task", "<task-id>"],
                " to see the section's current item ids",
            )),
        );
        deep_miss.substitute_derivable_route_placeholders();
        assert_eq!(
            deep_miss.route.as_ref().map(Route::as_str),
            Some(
                "`jigc doc show changelog:changelog#releases --task <task-id>` to see the \
                 section's current item ids"
            ),
            "a `doc show` route narrows to the top showable hop; `<task-id>` is left to its \
             own source",
        );
    }

    /// The pinned envelope (`DECISIONS.md` 2026-05-31 → Finding shape; re-pinned M6
    /// for the `(probe, check)` handle): a blocking finding with a `code`, a located
    /// `message`, and no route projects to exactly `{severity:"blocking", probe, check,
    /// code, key, message, location:{address,line,col}, route:null}` — in that field order,
    /// no stray keys, `route` kept as `null`. The `probe`/`check` handle derives from the
    /// `code`'s `<prefix>.<suffix>` split (here the exempt parser code
    /// `conformance.section-missing` → `("conformance", "section-missing")` — it still
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
            "conformance.section-missing",
            "required section heading `## Decision` is missing",
            Location::addressed("adr:pick-a-db#decision", 1, 1),
        );

        let json = serde_json::to_string_pretty(&finding).expect("serializes");

        insta::assert_snapshot!(json, @r#"
        {
          "severity": "blocking",
          "probe": "conformance",
          "check": "section-missing",
          "code": "conformance.section-missing",
          "key": {
            "code": "conformance.section-missing",
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
            "conformance.section-missing",
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
            "conformance.section-missing",
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

    /// The exemption is an **enumeration, not a namespace** (M49 Increment 8 / D5). Under the
    /// `starts_with("conformance.")` predicate it replaced, *any* code spelled into that
    /// namespace was exempt — including one no producer emits — so a route-less blocking
    /// finding could be admitted at the seam by its **spelling**. The negative control is the
    /// whole property: an unenumerated `conformance.*` code is refused, and the criterion the
    /// enumeration is written to is checked on the one member it applies to — exemption means
    /// *may* be route-less, so `conformance.duplicate-field`, which carries a route, needs no
    /// row and does not have one.
    #[test]
    fn the_route_exemption_is_an_enumeration_not_a_namespace_prefix() {
        // Spelled in two pieces on purpose: the derivation arm below reads this file's raw
        // bytes, and a literal here would be a phantom `conformance.*` name in its input.
        let invented = format!("conformance.{}", "not-a-real-code");
        assert!(
            !is_route_exempt(&invented),
            "a code nobody emits is not exempt — the exemption is a stated list of parser \
             diagnostics, not the `conformance.` namespace",
        );

        assert!(
            !is_route_exempt("conformance.duplicate-field"),
            "the criterion is *may* be route-less: `duplicate-field` knows its direction and \
             carries a route, so it needs no row and takes none",
        );

        for code in CONFORMANCE_PARSE_DIAGNOSTICS {
            assert!(is_route_exempt(code), "`{code}` is an enumerated member");
        }
    }

    /// **The disposal arm** (M49 Increment 8 / D5): every `conformance.*` code *named* in the
    /// engine's own source has a **production producer**. A code name with no producer is a
    /// claim about the tool's vocabulary that the tool does not keep — the M45 rename left
    /// `item-anchor-missing` alive in a negative assertion, and a synthetic `heading-missing`
    /// stood in for a real code in this file's own envelope goldens. Neither could ever be
    /// emitted, and a reader (or a reason-per-member enumeration) has no way to tell them
    /// from the thirteen that can. *(Both are written here without their `conformance.`
    /// head — the convention [`is_declared_non_unique`]'s M45 row already uses: a retired
    /// name spelled in full is itself a name with no producer, and this arm reads prose.)*
    ///
    /// The name set is read from the **raw** bytes — comments and test modules included,
    /// because a doc-comment naming a retired code is exactly the lie this arm hunts — while
    /// the producer set comes from the same production-source constructor scan the route-floor
    /// sweep above uses. **Bound: the engine crate**, which is where `conformance.*` findings
    /// are produced and where the vocabulary is therefore claimed; a `conformance.*` string in
    /// a CLI fixture is an assertion about *output text*, and this arm cannot see it.
    #[test]
    fn every_conformance_code_named_in_engine_source_has_a_producer() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        collect_rs(&src, &mut files);

        let mut named: Vec<(String, String)> = Vec::new();
        let mut producers: Vec<String> = Vec::new();
        for file in &files {
            let text = std::fs::read_to_string(file).expect("read source");
            for name in conformance_names(&text) {
                named.push((name, file.display().to_string()));
            }
            collect_finding_codes(&production_code(&text), &mut producers);
        }
        producers.retain(|c| c.starts_with("conformance."));
        producers.sort();
        producers.dedup();

        // Sanity floor, the sibling of the sweep's: an empty difference is only trustworthy
        // if both scans reached their subject.
        assert!(
            producers.len() >= 13,
            "the producer scan found only {} `conformance.*` producers — the source lexer \
             likely regressed",
            producers.len(),
        );
        assert!(
            named.len() >= producers.len(),
            "the name scan found fewer names than producers — the name lexer likely regressed",
        );

        let orphans: Vec<String> = named
            .iter()
            .filter(|(name, _)| !producers.contains(name))
            .map(|(name, file)| format!("  {name} — named in {file}"))
            .collect();
        assert!(
            orphans.is_empty(),
            "these `conformance.*` code names have no production producer — give each one a \
             producer, or dispose of the name (design/validation.md → The route floor):\n{}",
            orphans.join("\n"),
        );
    }

    /// Every `conformance.<name>` token in `text` — the **raw** name scan (see the disposal
    /// arm). `schema-conformance.*` is a different family and is excluded by requiring the
    /// byte before `conformance` to be neither `-` nor alphanumeric.
    fn conformance_names(text: &str) -> Vec<String> {
        const HEAD: &str = "conformance.";
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut from = 0usize;
        while let Some(rel) = text[from..].find(HEAD) {
            let start = from + rel;
            from = start + HEAD.len();
            let preceded_by_name_byte = start > 0 && {
                let b = bytes[start - 1];
                b == b'-' || b.is_ascii_alphanumeric()
            };
            if preceded_by_name_byte {
                continue;
            }
            let mut end = from;
            while end < bytes.len()
                && (bytes[end].is_ascii_lowercase()
                    || bytes[end].is_ascii_digit()
                    || bytes[end] == b'-')
            {
                end += 1;
            }
            // A Rust path, not a code name: `conformance.is_empty()` on a local named
            // `conformance` would otherwise read as a name whose tail is `is`, terminated
            // by an identifier byte — which no finding code ever is.
            let is_rust_path =
                end < bytes.len() && (bytes[end] == b'_' || bytes[end].is_ascii_alphanumeric());
            if end > from && !is_rust_path {
                out.push(text[start..end].to_string());
            }
        }
        out
    }

    /// Every literal finding `code` constructed in the production `code` view — the producer
    /// set the disposal arm subtracts. Non-literal codes are skipped (the same bound the
    /// route-floor sweep declares).
    fn collect_finding_codes(code: &str, out: &mut Vec<String>) {
        for_each_finding_constructor(code, |is_graded, args| {
            let code_arg = if is_graded { args.get(1) } else { args.first() };
            if let Some(lit) = code_arg.and_then(|a| extract_str_lit(a)) {
                out.push(lit);
            }
        });
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

    /// Walk every `Finding::graded(…)` / `Finding::blocking(…)` call in the production `code`
    /// view, handing each one's constructor kind (`is_graded`) and top-level argument spans to
    /// `visit` — the one traversal both source-derived arms read (the route-floor sweep's
    /// route-less-blocking scan and the disposal arm's producer scan), so the two can never
    /// disagree about what a producer *is*.
    fn for_each_finding_constructor(code: &str, mut visit: impl FnMut(bool, &[String])) {
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
                visit(is_graded, &args);
            }
        }
    }

    /// Find every `Finding::graded(…)` / `Finding::blocking(…)` in the production `code` view and
    /// record any that constructs a route-less, non-[`is_route_exempt`] **blocking** finding.
    fn scan_blocking_constructors(
        code: &str,
        file: &std::path::Path,
        examined: &mut usize,
        violations: &mut Vec<String>,
    ) {
        for_each_finding_constructor(code, |is_graded, args| {
            *examined += 1;

            let is_blocking = if is_graded {
                args.first()
                    .is_some_and(|a| a.contains("Severity::Blocking"))
            } else {
                true
            };
            if !is_blocking {
                return;
            }
            let route_less = if is_graded {
                args.last().is_some_and(|a| a == "None")
            } else {
                // `Finding::blocking` is route-less by construction.
                true
            };
            if !route_less {
                return;
            }
            let code_arg = if is_graded { args.get(1) } else { args.first() };
            let finding_code = code_arg.and_then(|a| extract_str_lit(a));
            let exempt = finding_code.as_deref().is_some_and(is_route_exempt);
            if !exempt {
                let shown = finding_code.unwrap_or_else(|| "<non-literal code>".to_string());
                let marker = if is_graded {
                    "Finding::graded("
                } else {
                    "Finding::blocking("
                };
                violations.push(format!(
                    "  {}: {marker}… {shown}, route=None",
                    file.display()
                ));
            }
        });
    }
}
