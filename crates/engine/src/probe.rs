//! The `Probe` seam — the read-only `check(target, ctx) -> [finding]` interface every
//! probe rides ([validation.md](../../../design/validation.md) → The engine / probe
//! boundary, The wire contract).
//!
//! **M10 reshapes the seam.** The M6 seam shipped `check(&self, ctx)` with **no
//! `target`** and a single, live, non-serializable ctx ([`OverrideCtx`] — a borrowed
//! `(deltas, pack)` handle), sized to its one consumer (`override-default`). The wire
//! contract the subprocess (pack-probe) seam rides requires a **`target`** and an
//! **effective-state ctx the engine can serialize** (a live graph in-process, its
//! serialized read-only projection + a path-ref out-of-process — `validation.md` → The
//! wire contract). M10 reshapes the seam to `check(target, ctx)` so the doc-code shape
//! is admissible, **without** coercing `override-default`'s live non-task ctx into a
//! serializable form it has no need to be ([DECISIONS.md](../../../DECISIONS.md)
//! 2026-06-06, M10 inc-2 / T1).
//!
//! **The carrier: a `target` parameter + two associated types** (`Target`, `Ctx<'_>`).
//! Each probe declares *exactly* its own target shape and ctx shape, so the seam admits
//! both shapes with no coercion (the over-generalization trap M3 paid for is avoided by
//! per-probe associated types, never one generic ctx forced over both):
//! - `override-default` is **non-task**: `Target = ()` (target-less) and `Ctx<'a> =
//!   OverrideCtx<'a>` (a live, non-serializable handle) — its M5 reconciliation is
//!   unchanged; the reshape moves only the *call shape*.
//! - the doc-code probe (M10 inc 3+) declares `Target` an [`crate::address::Address`]
//!   and `Ctx` a **serializable** effective-state snapshot (built in inc 3), so an
//!   in-process and a subprocess impl consume the *same logical input* (`validation.md`
//!   → The wire contract). `Ctx<'_>` is a GAT so a borrowing ctx (`OverrideCtx<'a>`) and
//!   a borrowed-snapshot ctx are both expressible.
//!
//! **Severity stays engine-owned, assigned once at the post-pass.** A [`Probe`] only
//! *emits* findings (each carrying its `(probe, check)` handle); the resolved-cascade
//! severity lookup (`validation.override-default.<check>.severity`) is applied
//! downstream by [`crate::result::ValidationReport::new`], never threaded into the
//! probe site (`validation.md` → severity is assigned in one engine-owned pass). So the
//! seam carries no `Resolved`: routing through it changes nothing about how `upgrade`'s
//! findings already tune.

use crate::finding::{Finding, Findings};
use crate::override_default::{RecordedDeltas, classify};
use crate::packsource::PackSource;
use crate::result::SCHEMA_VERSION;
use crate::target_surface::TargetAnchor;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The **request** envelope the engine sends a subprocess pack probe — JSON on the
/// probe's stdin ([validation.md](../../../design/validation.md) → The wire contract,
/// line 106: `{ probe_id, target, effective_state: { snapshot_path }, config,
/// schema_version }`). It is the **concrete Rust projection of the locked envelope**;
/// the struct shapes + serde form are a doc-elaboration pin within it
/// ([DECISIONS.md](../../../DECISIONS.md) 2026-06-06, M10 inc-3 / T1).
///
/// The snapshot is carried **by path-ref** ([`ProbeEffectiveState::snapshot_path`]),
/// never inlined — the engine materialized the [`EffectiveStateSnapshot`] in the
/// probe's read scope (inc 2) and the request names its path (`validation.md`:101:
/// "the request carries a path-ref to it"). The CLI invoker (T2) writes this to the
/// probe's stdin; the probe never calls back.
///
/// **`target`** is the one address grammar ([structural-grammar.md](../../../design/structural-grammar.md#addressing))
/// in its **string form** — the wire carries the flat URI a probe reads, not the
/// engine's internal [`crate::address::Address`] enum (the snapshot already addresses
/// each anchor as a string, [`TargetAnchor::address`]). **`schema_version`** is present
/// **as a field** so the contract can evolve without silently breaking a probe built
/// against an older engine; the *versioning policy itself stays deferred*
/// (`validation.md`:112; [module-layout.md](../../../implementation/module-layout.md)
/// → Open questions).
///
/// **Field order is pinned** (`probe_id`, `target`, `effective_state`, `config`,
/// `schema_version`) — the serialized form is the contract a probe is built against, so
/// a reorder is a breaking change the golden ([`tests::request_json_projection_is_the_pinned_envelope`])
/// catches.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeRequest {
    /// The probe being invoked (e.g. `doc-code`) — the `probe` half of the `(probe,
    /// check)` handle the engine keys severity on.
    pub probe_id: String,
    /// The address the probe adjudicates, in the one address grammar's string form.
    pub target: String,
    /// The read-only effective-state the probe reads — carried **by path-ref**.
    pub effective_state: ProbeEffectiveState,
    /// The cascade-resolved probe config (an opaque JSON object the probe interprets);
    /// empty when the cascade resolves none.
    pub config: serde_json::Map<String, serde_json::Value>,
    /// The wire-contract schema version (the contract-evolution marker; policy deferred).
    pub schema_version: u32,
}

/// The `effective_state` member of a [`ProbeRequest`]: the **path-ref** to the
/// materialized [`EffectiveStateSnapshot`] (`validation.md`:101 — a subprocess holds no
/// live graph handle and reaches the engine over no socket, so the engine materializes
/// a read-only snapshot in the probe's read scope and the request names its path).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeEffectiveState {
    /// The filesystem path of the materialized snapshot the probe reads.
    pub snapshot_path: PathBuf,
}

/// The **response** envelope a subprocess pack probe returns — JSON on the probe's
/// stdout ([validation.md](../../../design/validation.md) → The wire contract, line
/// 107: `{ findings: [<Finding>], schema_version }`). The engine deserializes this from
/// the probe's stdout (T3 ingests `findings` into the [`crate::result::ValidationReport`]).
///
/// **`findings`** are the engine's **one [`Finding`] shape** ([finding.rs](finding.rs))
/// — a probe emits findings; the engine assigns the final severity downstream
/// (severity is engine-owned, never baked into a probe — `validation.md`:111). A
/// response **round-trips** serialize → deserialize → equal
/// ([`tests::response_round_trips_through_serde`]). **`schema_version`** mirrors the
/// request's contract-evolution marker.
///
/// **Field order is pinned** (`findings`, `schema_version`) for the same contract reason
/// as [`ProbeRequest`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeResponse {
    /// The findings the probe emits — the engine's one [`Finding`] shape, carried in the
    /// [`Findings`] collection projection (the uniqueness half of the membership test rides
    /// its `Serialize`; the wire shape is the plain array, unchanged).
    pub findings: Findings,
    /// The wire-contract schema version (mirrors the request's marker).
    pub schema_version: u32,
}

impl ProbeRequest {
    /// Build a request for `probe_id` over `target`, carrying the snapshot by
    /// `snapshot_path` and the cascade-resolved `config`, stamping the current
    /// [`SCHEMA_VERSION`]. The engine's request-construction step (the CLI invoker, T2,
    /// serializes this to the probe's stdin).
    pub fn new(
        probe_id: impl Into<String>,
        target: impl Into<String>,
        snapshot_path: PathBuf,
        config: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            probe_id: probe_id.into(),
            target: target.into(),
            effective_state: ProbeEffectiveState { snapshot_path },
            config,
            schema_version: SCHEMA_VERSION,
        }
    }
}

impl ProbeResponse {
    /// A response carrying `findings`, stamping the current [`SCHEMA_VERSION`] — the
    /// shape a well-behaved probe emits (T3 deserializes the probe's stdout into this).
    pub fn new(findings: Vec<Finding>) -> Self {
        Self {
            findings: findings.into(),
            schema_version: SCHEMA_VERSION,
        }
    }
}

/// The read-only **effective-state snapshot** the engine materializes for a doc-code
/// probe — the serializable ctx the wire contract carries **by path-ref**
/// (`validation.md` → The wire contract; line 103: "What the snapshot must carry for
/// `doc-code`"). It is built from the [`crate::target_surface`] enumeration: the
/// `(target-address, anchor-value, check-id)` pairs to adjudicate plus the
/// **working-tree root** the anchors resolve against.
///
/// The snapshot carries **no code** — by rule 4 of the determinism contract the
/// probe reads the cited code directly from the repo at [`working_tree_root`](Self::working_tree_root)
/// (`validation.md`:103: "the code is read directly from the repo per rule 4, not
/// copied into the snapshot"). So an in-process and a subprocess probe consume the
/// *same logical input*: the in-process probe holds the live struct, the subprocess
/// probe reads its serialized form from the path-ref (inc 3) — the serde form is the
/// shared contract, unit-proven by [`tests::snapshot_round_trips_through_serde`].
///
/// **Field order is pinned** (`anchors`, `working_tree_root`, then `root_kind`) — a
/// doc-elaboration pin within the locked snapshot model ([DECISIONS.md](../../../DECISIONS.md)
/// 2026-06-06, M10 inc-2 / T3): the serialized form is a stable contract a probe is
/// built against, so a reorder is a breaking change a golden must catch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveStateSnapshot {
    /// The enumerated target surface — every `code-anchor` leaf over the task's
    /// effective-state docs, address-sorted ([`crate::target_surface::enumerate_target_surface`]).
    pub anchors: Vec<TargetAnchor>,
    /// The code-tree root the anchors' `<path>#<symbol>` values resolve against
    /// (the code is read from here directly, never copied into the snapshot).
    pub working_tree_root: PathBuf,
    /// **Which** root [`working_tree_root`](Self::working_tree_root) is — so a finding can
    /// name the tree it actually read (`validation.md` → The finding must name the root it
    /// read, and route to `git add`).
    pub root_kind: RootKind,
}

/// The **kind** of code tree a [`EffectiveStateSnapshot`] hands the probe — the
/// discriminator that keeps a `doc-code` finding's message and route honest about what
/// was read (`validation.md`:279). The two roots are correct by design (`validation.md`:265):
/// task scope adjudicates the tree `finalize` commits, store scope the tree on disk. What
/// the probe cannot know from the path alone is *which*, so the engine says it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RootKind {
    /// The **materialized git index** — exactly the bytes `finalize` commits (task scope).
    /// A file present on disk but never `git add`ed is *absent* here, and the only repair
    /// is `git add`.
    StagedIndex,
    /// The **on-disk working tree** — the task-less store sweep's root, which has no index
    /// to commit.
    WorkingTree,
}

impl EffectiveStateSnapshot {
    /// Build the snapshot from the enumerated `anchors`, the `working_tree_root` they
    /// resolve against, and **which kind of root** that is — the engine's materialization
    /// step (the wire carries it by path-ref out-of-process; inc 3 writes the path-ref
    /// hand-off).
    pub fn new(
        anchors: Vec<TargetAnchor>,
        working_tree_root: PathBuf,
        root_kind: RootKind,
    ) -> Self {
        Self {
            anchors,
            working_tree_root,
            root_kind,
        }
    }
}

/// How a probe subprocess ended, observed from the invoker boundary — the engine-side
/// mirror of the CLI invoker's raw status ([invoke.rs](../../cli/src/invoke.rs) →
/// `ProbeStatus`). The engine owns response parse + meta-finding synthesis but **never
/// shells out** ([finalize.md](../../../design/finalize.md); the engine is shell-free),
/// so the CLI invoker produces the live outcome and translates it into this engine type
/// at the inc-5 wiring seam — the synthesis fn ([`ingest_probe_run`]) is proven in
/// isolation against it here (T3). It mirrors the invoker's shape — an `Exited { code }`
/// (`None` when terminated by signal with no code) or a `TimedOut` — plus the invoker's
/// spawn error, as `CouldNotStart` (M54 Inc 2 T2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProbeRunStatus {
    /// The child exited on its own within the budget, carrying its exit code (`None`
    /// when terminated by signal with no code). A zero code is the clean candidate; a
    /// non-zero code (or `None`) is a `crash` candidate.
    Exited {
        /// The process exit code, or `None` if terminated without one (e.g. a signal).
        code: Option<i32>,
    },
    /// The child exceeded the wall-clock budget and was killed by the invoker — the
    /// `timeout` meta-finding ([validation.md](../../../design/validation.md) → Failure
    /// semantics).
    TimedOut,
    /// The program could not be spawned at all (absent, not executable, …) — no process
    /// ran, so there is no exit code and no output. It stays the `crash` meta-finding (no
    /// new check id), but its message says what happened instead of the *exited non-zero
    /// (exit-code signal)* a spawn failure used to read as (M54 Inc 2 T2; DECISIONS.md →
    /// *M54 settled*, S1 and S4).
    CouldNotStart {
        /// The spawn error's text, verbatim (`io::Error`'s `Display`).
        error: String,
    },
}

/// The most bytes of a probe's **stderr** that ride a finding message (M54 Inc 2 T2 pin):
/// the invoker keeps no more than this and drains the rest, so a chatty child can never
/// fill the pipe and deadlock, and [`ingest_probe_run`] cuts at it again, so an invoker
/// that kept more still yields a bounded message. 4 KiB holds a panic line or a skewed
/// child's version reason many times over and keeps one finding readable.
pub const PROBE_STDERR_BOUND: usize = 4096;

/// The raw outcome of one probe invocation as the engine consumes it — the engine-side
/// mirror of the CLI invoker's `ProbeOutcome`: the child's stdout bytes (verbatim,
/// **unclassified** — may be a valid response, garbage, or empty) and how it ended.
/// [`ingest_probe_run`] is the sole consumer; it decides what each shape means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeRun {
    /// The bytes the probe wrote to stdout, captured verbatim (empty if it wrote none).
    pub stdout: Vec<u8>,
    /// The first bytes the probe wrote to stderr (empty if it wrote none, or never
    /// started) — at most [`PROBE_STDERR_BOUND`] of them are ever rendered.
    pub stderr: Vec<u8>,
    /// How the process ended.
    pub status: ProbeRunStatus,
}

/// Ingest one probe invocation's raw outcome into the findings the engine carries into
/// the [`crate::result::ValidationReport`] — the response-ingestion + meta-finding
/// synthesis step ([validation.md](../../../design/validation.md) → Failure semantics —
/// meta-findings, lines 114-125). The five outcome shapes the invoker (T2) produces map:
///
/// - **well-behaved** (exit 0 + parseable [`ProbeResponse`]) → the probe's own
///   `findings`, **unchanged** (severity is left for the engine's post-pass — this fn
///   does not grade); **no** meta-finding.
/// - **timed out** → exactly one blocking `timeout` meta-finding (partial stdout is
///   discarded — a timed-out probe is untrusted).
/// - **non-zero exit** (or signal-terminated, `code: None`) → one blocking `crash`
///   meta-finding (the exit code rides the descriptive message, and so does the child's
///   stderr, cut at [`PROBE_STDERR_BOUND`], when it wrote any).
/// - **could not start** (the spawn itself failed) → one blocking `crash` meta-finding
///   too, whose message reads *could not start: `<io error>`* (M54 Inc 2 T2).
/// - **exit 0 but unparseable** (garbage / empty / valid-JSON-but-not-a-response) → one
///   blocking `malformed-output` meta-finding (the parse error rides the message).
///
/// Each meta-finding's **severity handle** is the canonical `(probe, check) =
/// ("pack-probe-integrity", <reason>)` — set via [`Finding::with_check`] over a
/// descriptive `probe-failure` `code` (a **code-id reconciliation** exactly like
/// `override-default`'s: the post-pass + floor key on `(probe, check)`, the descriptive
/// `code` is rendering-only — `validation.md` → Code-id reconciliation). The synthesized
/// severity is `Blocking`, the intrinsic floor these checks are locked to (T4); the fn
/// **never panics** on any stdout shape or status.
pub fn ingest_probe_run(probe_id: &str, run: &ProbeRun) -> Vec<Finding> {
    match &run.status {
        ProbeRunStatus::TimedOut => vec![meta_finding(
            probe_id,
            "timeout",
            format!("probe `{probe_id}` exceeded its time budget and was killed"),
        )],
        ProbeRunStatus::Exited { code: Some(0) } => {
            // Wire contract (validation.md → The wire contract): a clean run MUST emit
            // `{findings: [], schema_version}` — empty/unparseable stdout on a zero exit
            // is `malformed-output`, never "no findings", so a probe that silently did
            // nothing cannot masquerade as a pass.
            match serde_json::from_slice::<ProbeResponse>(&run.stdout) {
                Ok(response) => response.findings.into_vec(),
                Err(err) => vec![meta_finding(
                    probe_id,
                    "malformed-output",
                    format!("probe `{probe_id}` exited 0 but emitted unparseable output: {err}"),
                )],
            }
        }
        ProbeRunStatus::CouldNotStart { error } => vec![meta_finding(
            probe_id,
            "crash",
            format!("probe `{probe_id}` could not start: {error}"),
        )],
        ProbeRunStatus::Exited { code } => {
            let exit = code.map_or_else(|| "signal".to_string(), |c| c.to_string());
            let mut message = format!(
                "probe `{probe_id}` exited non-zero (exit-code {exit}) with no usable output"
            );
            let stderr = bounded_stderr(&run.stderr);
            if !stderr.is_empty() {
                message.push_str("; stderr: ");
                message.push_str(&stderr);
            }
            vec![meta_finding(probe_id, "crash", message)]
        }
    }
}

/// The child's stderr as a finding carries it: the first [`PROBE_STDERR_BOUND`] bytes,
/// decoded lossily (a cut through a multi-byte character shows as `U+FFFD`, never a
/// panic) and trimmed of surrounding whitespace.
fn bounded_stderr(stderr: &[u8]) -> String {
    let kept = &stderr[..stderr.len().min(PROBE_STDERR_BOUND)];
    String::from_utf8_lossy(kept).trim().to_string()
}

/// Build one intrinsic-blocking `pack-probe-integrity` meta-finding: a descriptive
/// `probe-failure` `code` carrying the human reason, re-keyed via [`Finding::with_check`]
/// onto the canonical `(probe, check) = ("pack-probe-integrity", <reason>)` handle the
/// post-pass + floor lock on.
///
/// It is **located at the offending probe's id** — the **pack-resource** target form
/// ([command-output-contract.md](../../../design/command-output-contract.md) → the form
/// table), which is what makes the finding's stable key `(code, target)` discriminating.
/// A misbehaving subprocess has no *source coordinate* the engine can cite, but it has an
/// identity, and it is in hand at every call: all three reasons share the one descriptive
/// `probe-failure` code, so without the probe id **every** misbehaving probe in a sweep
/// collides on one key — on the family that is *also* the store-scope exit-flip class, the
/// one an operator most needs to tell apart. A run yields at most one status per probe, so
/// the probe id alone restores uniqueness (the failure *reason* stays on the `(probe,
/// check)` severity handle, where the post-pass and the floor read it).
fn meta_finding(probe_id: &str, reason: &str, message: String) -> Finding {
    Finding::graded(
        crate::finding::Severity::Blocking,
        "pack-probe-integrity.probe-failure",
        message,
        Some(crate::finding::Location::addressed(probe_id, 1, 1)),
        // The route floor (M43): a misbehaving probe subprocess is an install/binary
        // problem, not a corpus problem — judgment-shaped (which half is broken is the
        // operator's call), so the route is human.
        Some(crate::finding::Route::human(
            "the probe subprocess misbehaved (the message carries the reason) — repair or \
             re-install the probe binary (`jigc setup` reinstalls the shipped probes), then \
             re-run the sweep",
        )),
    )
    .with_check(reason)
}

/// The read-only context an override-scoped [`Probe`] checks: the recorded deltas to
/// reconcile and the current (env-selected) pack to reconcile them against — the
/// `(recorded deltas, pack)` shape `override-default` consumes (`validation.md` → one
/// whose ctx is `(recorded deltas, pack)`). This is the **non-task** ctx; the seam
/// admits it as `OverrideDefaultProbe`'s associated `Ctx` without coercing it into the
/// serializable effective-state shape the doc-code probe declares.
pub struct OverrideCtx<'a> {
    /// The project layer's recorded deltas, borrowed for the duration of the check.
    pub deltas: RecordedDeltas<'a>,
    /// The current pack the deltas are reconciled against (pack-direct reads).
    pub pack: &'a dyn PackSource,
}

/// A probe: a read-only `check(target, ctx) -> [finding]` (`validation.md` → The engine
/// / probe boundary). Each probe declares its own [`Target`](Probe::Target) shape and
/// [`Ctx`](Probe::Ctx) shape via associated types, so the seam admits both the non-task
/// `(deltas, pack)` ctx and a serializable effective-state ctx without forcing one
/// generic over both. The engine assigns final severity downstream (the post-pass), so a
/// probe only *suggests* by emitting findings carrying their `(probe, check)` handle
/// ("the engine assigns, the probe suggests").
pub trait Probe {
    /// The address surface this probe checks — an [`crate::address::Address`] for a
    /// targeted probe (doc-code), or `()` for a non-task probe (`override-default`)
    /// whose ctx already names what it reconciles.
    type Target;
    /// The read-only context this probe checks. A GAT so a borrowing ctx
    /// ([`OverrideCtx`]) and a borrowed serializable-snapshot ctx are both expressible.
    type Ctx<'a>;

    /// Check `target` against `ctx`, returning one [`Finding`] per problem found (none
    /// for a clean context). Read-only: a probe reads its inputs and writes nothing.
    fn check<'a>(&self, target: &Self::Target, ctx: Self::Ctx<'a>) -> Vec<Finding>;
}

/// The `override-default` probe routed onto the seam — the seam's first (and, pre-M10,
/// only) real consumer. It delegates to the M5 [`classify`] logic: the reshape changes
/// the *call shape* (a `target` it ignores — its ctx already names what it reconciles),
/// not the reconciliation, so the byte-stable upgrade path is unaffected (`validation.md`
/// → `override-default` is the retrofit).
pub struct OverrideDefaultProbe;

impl Probe for OverrideDefaultProbe {
    /// Target-less: `override-default`'s ctx (`deltas`) already names the override
    /// surface it reconciles, so the seam's `target` is the unit type.
    type Target = ();
    type Ctx<'a> = OverrideCtx<'a>;

    fn check<'a>(&self, _target: &(), ctx: OverrideCtx<'a>) -> Vec<Finding> {
        classify(ctx.deltas, ctx.pack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;
    use crate::cascade::{Anchor, StructuralDelta, StructuralTarget};
    use crate::finding::{Location, Severity};
    use crate::packsource::{PackError, PackResourceKind, ResourceId};
    use std::collections::HashMap;

    /// The **request** envelope is the pinned wire contract (the T1 done-criterion): a
    /// request projects to exactly `{ probe_id, target, effective_state: {
    /// snapshot_path }, config, schema_version }` — in that field order, the snapshot
    /// carried **by path-ref** (never inlined), `config` an object, `schema_version`
    /// present. The golden pins the serialized *string* (not a key-sorted value), so it
    /// also locks field order; a rename, a reorder, an inlined snapshot, or a serde slip
    /// breaks it. That is the contract both sides bind to.
    #[test]
    fn request_json_projection_is_the_pinned_envelope() {
        let config = serde_json::Map::from_iter([(
            "is-a-test".to_string(),
            serde_json::Value::String("rust-first".to_string()),
        )]);
        let request = ProbeRequest::new(
            "doc-code",
            "adr:single-node-cache#status/cites-code",
            PathBuf::from("/scratch/snapshot.json"),
            config,
        );

        let json = serde_json::to_string_pretty(&request).expect("serializes");

        insta::assert_snapshot!(json, @r#"
        {
          "probe_id": "doc-code",
          "target": "adr:single-node-cache#status/cites-code",
          "effective_state": {
            "snapshot_path": "/scratch/snapshot.json"
          },
          "config": {
            "is-a-test": "rust-first"
          },
          "schema_version": 3
        }
        "#);
    }

    /// A request with **no resolved config** projects `config` as an empty object `{}`
    /// (the cascade resolved none) — the wire member is always present, never absent,
    /// so a probe reads a stable shape. And the request **round-trips** serialize →
    /// deserialize → equal.
    #[test]
    fn request_round_trips_with_empty_config() {
        let request = ProbeRequest::new(
            "doc-code",
            "spec:rate-limiting#criteria/rate-limit/maps-to-test",
            PathBuf::from("/scratch/snapshot.json"),
            serde_json::Map::new(),
        );

        let json = serde_json::to_value(&request).expect("serializes");
        assert_eq!(
            json["config"],
            serde_json::json!({}),
            "an unresolved config is an empty object, not absent",
        );

        let back: ProbeRequest = serde_json::from_value(json).expect("deserializes");
        assert_eq!(
            back, request,
            "the request round-trips serialize -> deserialize -> equal"
        );
    }

    /// The **response** envelope (the T1 done-criterion): a response carrying the
    /// engine's one [`Finding`] shape projects to exactly `{ findings: [<Finding>],
    /// schema_version }` in that field order, and **round-trips** serialize →
    /// deserialize → equal. The `findings` member deserializes back into the engine's
    /// one `Finding` shape — proving a probe's emitted findings ingest unchanged (T3).
    #[test]
    fn response_round_trips_through_serde() {
        let response = ProbeResponse::new(vec![Finding::graded(
            Severity::Blocking,
            "doc-code.symbol-exists",
            "anchor `crates/engine/src/missing.rs#nope` resolves to no symbol",
            Some(Location::addressed(
                "adr:single-node-cache#status/cites-code",
                1,
                1,
            )),
            Some(crate::finding::Route::human(
                "update the citation to match the renamed/moved code, or restore the cited symbol",
            )),
        )]);

        let json = serde_json::to_string_pretty(&response).expect("serializes");
        let back: ProbeResponse = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(
            back, response,
            "the response round-trips serialize -> deserialize -> equal",
        );

        insta::assert_snapshot!(json, @r#"
        {
          "findings": [
            {
              "severity": "blocking",
              "probe": "doc-code",
              "check": "symbol-exists",
              "code": "doc-code.symbol-exists",
              "key": {
                "code": "doc-code.symbol-exists",
                "target": "adr:single-node-cache#status/cites-code"
              },
              "message": "anchor `crates/engine/src/missing.rs#nope` resolves to no symbol",
              "location": {
                "address": "adr:single-node-cache#status/cites-code",
                "line": 1,
                "col": 1
              },
              "route": "update the citation to match the renamed/moved code, or restore the cited symbol"
            }
          ],
          "schema_version": 3
        }
        "#);
    }

    /// The response's `findings` deserialize into the engine's one [`Finding`] shape
    /// from a **probe-authored** JSON document (the bytes a subprocess writes to stdout,
    /// not an engine-built struct) — proving an external probe's output binds to the
    /// engine's `Finding` envelope. A probe may omit `probe`/`check` (`#[serde(default)]`),
    /// so this fixture does; they default empty and the engine derives them downstream.
    #[test]
    fn response_findings_deserialize_into_the_one_finding_shape() {
        let wire = r#"{
          "findings": [
            {
              "severity": "blocking",
              "code": "doc-code.symbol-exists",
              "message": "dangling anchor",
              "location": { "address": "adr:cache#status/cites-code", "line": 1, "col": 1 },
              "route": null
            }
          ],
          "schema_version": 3
        }"#;

        let response: ProbeResponse =
            serde_json::from_str(wire).expect("probe-authored JSON deserializes");

        assert_eq!(response.findings.len(), 1);
        let finding = &response.findings[0];
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "doc-code.symbol-exists");
        assert_eq!(
            finding.location.as_ref().and_then(|l| l.address.as_deref()),
            Some("adr:cache#status/cites-code"),
        );
    }

    /// Two enumerated anchors — a header `cites-code` (`symbol-exists`) and a criterion
    /// `maps-to-test` (`criterion-maps-to-test`) — the same two shapes T2 enumerates.
    fn sample_anchors() -> Vec<TargetAnchor> {
        vec![
            TargetAnchor {
                address: "adr:single-node-cache#status/cites-code".to_string(),
                anchor_value: "crates/engine/src/validate.rs#validate_task".to_string(),
                check_id: "symbol-exists".to_string(),
            },
            TargetAnchor {
                address: "spec:rate-limiting#criteria/rate-limit/maps-to-test".to_string(),
                anchor_value: "crates/engine/src/missing.rs#nope".to_string(),
                check_id: "criterion-maps-to-test".to_string(),
            },
        ]
    }

    /// Serialization is unit-proven (the T3 done-criterion): the snapshot serialises and
    /// **deserialises back equal** (serialize → deserialize → equal), and its JSON pins
    /// the field order (`anchors`, `working_tree_root`, `root_kind`) the wire contract
    /// carries — a reorder or a serde-attribute slip breaks the golden. This is the shared
    /// form an in-process and a subprocess probe both consume; `root_kind` is what lets the
    /// probe's finding name the root it read (`validation.md`:279).
    #[test]
    fn snapshot_round_trips_through_serde() {
        let snapshot = EffectiveStateSnapshot::new(
            sample_anchors(),
            PathBuf::from("/repo/root"),
            RootKind::StagedIndex,
        );

        let json = serde_json::to_string_pretty(&snapshot).expect("serialises");
        let back: EffectiveStateSnapshot = serde_json::from_str(&json).expect("deserialises");

        assert_eq!(
            back, snapshot,
            "the snapshot round-trips serialize -> deserialize -> equal",
        );

        insta::assert_snapshot!(json, @r#"
        {
          "anchors": [
            {
              "address": "adr:single-node-cache#status/cites-code",
              "anchor_value": "crates/engine/src/validate.rs#validate_task",
              "check_id": "symbol-exists"
            },
            {
              "address": "spec:rate-limiting#criteria/rate-limit/maps-to-test",
              "anchor_value": "crates/engine/src/missing.rs#nope",
              "check_id": "criterion-maps-to-test"
            }
          ],
          "working_tree_root": "/repo/root",
          "root_kind": "staged-index"
        }
        "#);
    }

    /// An **in-process** doc-code probe test double (an `impl Probe`, NOT a subprocess):
    /// it declares `Target = Address` (the doc-code target shape) and `Ctx` a borrowed
    /// [`EffectiveStateSnapshot`], and emits one finding per snapshot anchor whose
    /// `anchor_value` names a file the working tree lacks — a stand-in for the real
    /// tree-sitter resolve (inc 4). It reads **only** the snapshot's enumerated pairs;
    /// it never re-parses a doc.
    struct InProcessDocCodeProbe;

    impl Probe for InProcessDocCodeProbe {
        type Target = Address;
        type Ctx<'a> = &'a EffectiveStateSnapshot;

        fn check(&self, _target: &Address, ctx: &EffectiveStateSnapshot) -> Vec<Finding> {
            ctx.anchors
                .iter()
                .filter(|anchor| {
                    // Resolve the anchor's `<path>#<symbol>` file against the snapshot's
                    // working-tree root; a missing file is a dangling anchor.
                    let path = anchor.anchor_value.split('#').next().unwrap_or("");
                    !ctx.working_tree_root.join(path).exists()
                })
                .map(|anchor| {
                    Finding::graded(
                        crate::finding::Severity::Blocking,
                        format!("doc-code.{}", anchor.check_id),
                        format!("anchor `{}` resolves to no file", anchor.anchor_value),
                        Some(crate::finding::Location::addressed(
                            anchor.address.clone(),
                            1,
                            1,
                        )),
                        Some(crate::finding::Route::human(
                            "update the citation to match the renamed/moved code, or restore the cited symbol",
                        )),
                    )
                })
                .collect()
        }
    }

    /// The reshaped seam admits a probe consuming the **serialized** effective-state ctx
    /// end-to-end (the T3 done-criterion): the engine materializes the snapshot, it is
    /// round-tripped through serialization (serialize -> deserialize — the exact bytes
    /// the wire carries by path-ref), and the in-process probe is handed a `(target,
    /// ctx)` over the *deserialized* snapshot. It returns findings derived from the pairs
    /// it reads — one for the anchor naming a file the working tree lacks, none for the
    /// anchor naming a present file — proving the probe consumes the serialized snapshot,
    /// not a live handle.
    #[test]
    fn in_process_probe_consumes_serialized_snapshot_end_to_end() {
        // A working tree where the first anchor's file EXISTS and the second's does not.
        let root = std::env::temp_dir().join(format!(
            "jigc-snapshot-probe-{}-{:?}",
            std::process::id(),
            crate::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(root.join("crates/engine/src")).expect("mk tree");
        std::fs::write(
            root.join("crates/engine/src/validate.rs"),
            b"fn validate_task() {}\n",
        )
        .expect("present file");

        let snapshot =
            EffectiveStateSnapshot::new(sample_anchors(), root.clone(), RootKind::WorkingTree);

        // Cross the wire: materialized -> serialized -> deserialized (the bytes the
        // path-ref carries). The probe consumes the DESERIALIZED snapshot, never the live one.
        let bytes = serde_json::to_vec(&snapshot).expect("serialises");
        let over_wire: EffectiveStateSnapshot =
            serde_json::from_slice(&bytes).expect("deserialises");

        let target: Address = "adr:single-node-cache#status/cites-code"
            .parse()
            .expect("address parses");
        let findings = InProcessDocCodeProbe.check(&target, &over_wire);

        assert_eq!(
            findings.len(),
            1,
            "exactly the dangling anchor surfaces (the present one is clean): {findings:?}",
        );
        assert_eq!(findings[0].code, "doc-code.criterion-maps-to-test");
        assert_eq!(
            findings[0]
                .location
                .as_ref()
                .and_then(|l| l.address.as_deref()),
            Some("spec:rate-limiting#criteria/rate-limit/maps-to-test"),
            "the finding is derived from the snapshot pair the probe read",
        );

        let _ = std::fs::remove_dir_all(&root);
    }

    /// A trivial in-memory `PackSource` carrying step bodies — the engine test-double
    /// pattern, here seeded as the "current pack" the seam classifies deltas against.
    struct FakePack(HashMap<(PackResourceKind, ResourceId), Vec<u8>>);

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            "v2".to_owned()
        }
        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            self.0
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect()
        }
        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            self.0
                .get(&(kind, id.clone()))
                .cloned()
                .ok_or_else(|| PackError::NotFound {
                    kind,
                    id: id.clone(),
                })
        }
    }

    /// **Well-behaved** (zero exit + parseable JSON): the probe's own findings ingest
    /// into the report **unchanged** (severity left for the post-pass — T3 does not
    /// grade), and **no** meta-finding is synthesized. The fixture's response is the
    /// bytes a well-behaved probe writes to stdout (a `ProbeResponse` serialized), so the
    /// ingestion path reads exactly the wire shape an inc-4 probe will emit.
    #[test]
    fn well_behaved_outcome_ingests_findings_and_synthesizes_no_meta_finding() {
        let emitted = Finding::graded(
            Severity::Blocking,
            "doc-code.symbol-exists",
            "anchor `crates/engine/src/missing.rs#nope` resolves to no symbol",
            Some(Location::addressed(
                "adr:single-node-cache#status/cites-code",
                1,
                1,
            )),
            Some(crate::finding::Route::human(
                "update the citation to match the renamed/moved code, or restore the cited symbol",
            )),
        );
        let stdout = serde_json::to_vec(&ProbeResponse::new(vec![emitted.clone()])).unwrap();

        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout,
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            },
        );

        assert_eq!(
            findings,
            vec![emitted],
            "the probe's findings ingest verbatim, no meta-finding, severity untouched",
        );
        assert!(
            findings.iter().all(|f| f.probe != "pack-probe-integrity"),
            "a well-behaved probe synthesizes no meta-finding: {findings:?}",
        );
    }

    /// A well-behaved probe with an **empty** `findings` array (a clean adjudication)
    /// ingests as **no findings at all** — proving zero-exit + parseable-empty is the
    /// clean case, never a crash or malformed-output meta-finding.
    #[test]
    fn well_behaved_empty_findings_ingest_as_clean() {
        let stdout = serde_json::to_vec(&ProbeResponse::new(vec![])).unwrap();

        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout,
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            },
        );

        assert!(
            findings.is_empty(),
            "a clean probe (zero exit, empty findings) ingests nothing: {findings:?}",
        );
    }

    /// **Timed out**: exactly one **blocking** meta-finding keyed `(probe,
    /// check)=("pack-probe-integrity","timeout")`, regardless of any partial stdout (a
    /// timed-out probe is untrusted, so its output — here garbage — is discarded). The
    /// `(probe, check)` is the post-pass + floor handle; the descriptive `code`
    /// (`probe-failure`) is for rendering only.
    #[test]
    fn timed_out_synthesizes_one_blocking_timeout_meta_finding() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: b"partial junk before the kill".to_vec(),
                stderr: Vec::new(),
                status: ProbeRunStatus::TimedOut,
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.probe, "pack-probe-integrity");
        assert_eq!(f.check, "timeout");
        assert!(
            f.message.contains("doc-code"),
            "the meta-finding names the offending probe: {}",
            f.message,
        );
    }

    /// **Crash** (non-zero exit, no parseable JSON): one **blocking** meta-finding keyed
    /// `(probe, check)=("pack-probe-integrity","crash")`. The exit code rides the
    /// descriptive message (`probe-failure { … exit-code }`, `validation.md`:121), the
    /// handle stays the canonical `crash`.
    #[test]
    fn non_zero_exit_synthesizes_one_blocking_crash_meta_finding() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(2) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.probe, "pack-probe-integrity");
        assert_eq!(f.check, "crash");
        assert!(
            f.message.contains('2'),
            "the exit code rides the message: {}",
            f.message,
        );
    }

    /// A child terminated **by signal** (no exit code) with no JSON is a **crash** too —
    /// `code: None` is non-zero by construction (not a clean exit). The engine must not
    /// panic on the `None` code, and must not mistake it for a clean run.
    #[test]
    fn signal_terminated_no_output_synthesizes_crash() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: None },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        assert_eq!(findings[0].probe, "pack-probe-integrity");
        assert_eq!(findings[0].check, "crash");
    }

    /// **Could not start** (M54 Inc 2 T2): a program that never spawned yields exactly one
    /// blocking `pack-probe-integrity.probe-failure`, check `crash` — the same finding and
    /// the same check as a non-zero exit, so the 34-check census does not move — whose
    /// message says what happened and carries the io error. Red before: the spawn failure
    /// read as *exited non-zero (exit-code signal)*, which is how the gate's `ENOENT` race
    /// was misread (DECISIONS.md → *M54 settled*, S1).
    #[test]
    fn could_not_start_synthesizes_one_blocking_crash_naming_the_io_error() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::CouldNotStart {
                    error: "Permission denied (os error 13)".to_string(),
                },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.code, "pack-probe-integrity.probe-failure");
        assert_eq!(f.probe, "pack-probe-integrity");
        assert_eq!(f.check, "crash");
        assert!(
            f.message.contains("could not start")
                && f.message.contains("Permission denied (os error 13)"),
            "the message says the probe could not start, and why: {}",
            f.message,
        );
        assert!(
            !f.message.contains("exited non-zero"),
            "a probe that never ran did not exit: {}",
            f.message,
        );
    }

    /// A **non-zero exit that wrote to stderr** carries that stderr on its `crash`
    /// message — the channel a skewed child states its reason on (M54 Inc 2 T2; S4).
    #[test]
    fn non_zero_exit_carries_its_stderr_on_the_crash_message() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: b"doc-code: built for jigc 0.0.0-skew, run by jigc 1.0.0\n".to_vec(),
                status: ProbeRunStatus::Exited { code: Some(2) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        assert_eq!(findings[0].check, "crash");
        assert!(
            findings[0]
                .message
                .contains("doc-code: built for jigc 0.0.0-skew, run by jigc 1.0.0"),
            "the child's stderr rides the message: {}",
            findings[0].message,
        );
    }

    /// Stderr past [`PROBE_STDERR_BOUND`] is **cut at the bound**: every byte up to it
    /// rides the message and none after it — so a child that floods stderr still yields
    /// one readable finding, whatever the invoker handed over.
    #[test]
    fn stderr_past_the_bound_is_cut_at_the_bound() {
        let mut stderr = vec![b'a'; PROBE_STDERR_BOUND];
        stderr.extend_from_slice(b"PAST-THE-BOUND");

        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr,
                status: ProbeRunStatus::Exited { code: Some(1) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        let message = &findings[0].message;
        assert!(
            message.contains(&"a".repeat(PROBE_STDERR_BOUND)),
            "every byte up to the bound rides the message: {} bytes",
            message.len(),
        );
        assert!(
            !message.contains(&"a".repeat(PROBE_STDERR_BOUND + 1)) && !message.contains("PAST"),
            "no byte past the bound rides the message: {} bytes",
            message.len(),
        );
    }

    /// **Malformed output** (zero exit, but stdout is **unparseable** JSON): one
    /// **blocking** meta-finding keyed `(probe, check)=("pack-probe-integrity",
    /// "malformed-output")`. A probe that exited cleanly but emitted garbage cannot be
    /// trusted to have validated anything — distinct from `crash` (which exited
    /// non-zero). The parse error rides the descriptive message (`validation.md`:122).
    #[test]
    fn zero_exit_unparseable_json_synthesizes_one_blocking_malformed_output_meta_finding() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: b"this is not json {".to_vec(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        let f = &findings[0];
        assert_eq!(f.severity, Severity::Blocking);
        assert_eq!(f.probe, "pack-probe-integrity");
        assert_eq!(f.check, "malformed-output");
    }

    /// A probe that exits **zero** but writes **no output at all** is malformed-output,
    /// not clean — empty stdout is not a valid `{ findings, schema_version }` response.
    /// (A clean adjudication emits `{"findings":[],…}`, never an empty stream.)
    #[test]
    fn zero_exit_empty_output_synthesizes_malformed_output() {
        let findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        assert_eq!(findings[0].probe, "pack-probe-integrity");
        assert_eq!(findings[0].check, "malformed-output");
    }

    /// **Two misbehaving probes in one sweep are two distinct findings** (the M42
    /// done-criterion): a timed-out `doc-code` and a crashed `commit-msg` yield findings
    /// whose stable `key.target`s are their **probe ids**, so a driver can tell them
    /// apart, dedupe them, and eventually acknowledge them separately
    /// ([command-output-contract.md](../../../design/command-output-contract.md) → the
    /// form table, the pack-resource row). Red before M42: both keyed `(code, null)` —
    /// one degenerate key for every misbehaving probe in a sweep, on the family that is
    /// *also* the store-scope exit-flip class, i.e. the one an operator most needs to tell
    /// apart. The `(probe, check)` severity handle carries the *reason* and is unchanged;
    /// `key` is `(code, target)` and all three reasons share one descriptive `code`, so
    /// the probe id is what restores uniqueness.
    #[test]
    fn two_misbehaving_probes_in_one_sweep_key_at_their_own_probe_ids() {
        let mut findings = ingest_probe_run(
            "doc-code",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::TimedOut,
            },
        );
        findings.extend(ingest_probe_run(
            "commit-msg",
            &ProbeRun {
                stdout: Vec::new(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(2) },
            },
        ));

        assert_eq!(findings.len(), 2, "one per probe: {findings:?}");
        let keys: Vec<_> = findings.iter().map(Finding::key).collect();
        assert_ne!(
            keys[0], keys[1],
            "two misbehaving probes must not collide on one key: {keys:?}",
        );
        assert_eq!(
            keys.iter()
                .map(|k| k.target.clone())
                .collect::<Vec<Option<String>>>(),
            vec![Some("doc-code".to_string()), Some("commit-msg".to_string()),],
            "each meta-finding targets the probe it is about: {keys:?}",
        );
        assert_eq!(
            keys.iter().map(|k| k.code.as_str()).collect::<Vec<_>>(),
            vec![
                "pack-probe-integrity.probe-failure",
                "pack-probe-integrity.probe-failure",
            ],
            "the descriptive code is shared — only the target discriminates",
        );

        // The `(probe, check)` severity handle is the failure *reason*, untouched.
        assert_eq!(
            findings
                .iter()
                .map(|f| (f.probe.as_str(), f.check.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("pack-probe-integrity", "timeout"),
                ("pack-probe-integrity", "crash"),
            ],
        );
    }

    /// A **malformed-output** meta-finding keys at its probe id too — the third reason
    /// shares the one descriptive `code`, so the target is the whole discriminator across
    /// all three (`timeout` / `crash` / `malformed-output`).
    #[test]
    fn malformed_output_meta_finding_keys_at_the_probe_id() {
        let findings = ingest_probe_run(
            "commit-msg",
            &ProbeRun {
                stdout: b"not json {".to_vec(),
                stderr: Vec::new(),
                status: ProbeRunStatus::Exited { code: Some(0) },
            },
        );

        assert_eq!(findings.len(), 1, "exactly one meta-finding: {findings:?}");
        assert_eq!(findings[0].check, "malformed-output");
        assert_eq!(findings[0].key().target.as_deref(), Some("commit-msg"));
    }

    /// The engine **never panics** on any misbehavior — sweep every adversarial shape
    /// (each invoker outcome, plus boundary stdout: empty, valid-but-not-a-response JSON,
    /// truncated, binary) and assert ingestion always returns (never unwinds). The
    /// done-criterion's hard floor: a misbehaving probe degrades to a meta-finding, never
    /// a crash of the engine itself.
    #[test]
    fn ingestion_never_panics_on_any_misbehavior() {
        let stdouts: Vec<Vec<u8>> = vec![
            Vec::new(),
            b"".to_vec(),
            b"{".to_vec(),
            b"not json".to_vec(),
            b"[]".to_vec(),
            b"42".to_vec(),
            b"{\"unexpected\":true}".to_vec(),
            vec![0x00, 0xff, 0x80, 0x01],
            serde_json::to_vec(&ProbeResponse::new(vec![])).unwrap(),
        ];
        let statuses = [
            ProbeRunStatus::Exited { code: Some(0) },
            ProbeRunStatus::Exited { code: Some(1) },
            ProbeRunStatus::Exited { code: None },
            ProbeRunStatus::TimedOut,
            ProbeRunStatus::CouldNotStart {
                error: String::new(),
            },
        ];
        for stdout in &stdouts {
            for status in &statuses {
                let _ = ingest_probe_run(
                    "doc-code",
                    &ProbeRun {
                        stdout: stdout.clone(),
                        stderr: Vec::new(),
                        status: status.clone(),
                    },
                );
            }
        }
    }

    /// The seam delegates faithfully: `OverrideDefaultProbe::check` over an
    /// [`OverrideCtx`] returns **exactly** what [`classify`] returns over the same
    /// `(deltas, pack)`. A `remove` delta over a step the pack omits orphans, so the
    /// probe must surface that one finding — proving the seam routes the real
    /// reconciliation, not a reconstruction. Hardening #5 — the pack OMITS the target.
    #[test]
    fn override_default_probe_delegates_to_classify() {
        let pack = FakePack(HashMap::from([(
            (PackResourceKind::Steps, ResourceId::from("present")),
            b"present body\n".to_vec(),
        )]));

        let structural = vec![StructuralDelta::Remove {
            target: StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::At("gone".to_owned()),
            },
        }];
        let mk = || RecordedDeltas {
            structural: &structural,
            forks: &[],
            bases: &[],
            slot_fills: &[],
            scalars: &[],
        };

        let via_seam = OverrideDefaultProbe.check(
            &(),
            OverrideCtx {
                deltas: mk(),
                pack: &pack,
            },
        );
        let direct = classify(mk(), &pack);

        assert_eq!(
            via_seam, direct,
            "the seam returns exactly what classify returns over the same ctx",
        );
        assert_eq!(
            via_seam.len(),
            1,
            "the omitted target orphans: {via_seam:?}"
        );
        assert_eq!(via_seam[0].code, "override-default.target-exists");
    }
}
