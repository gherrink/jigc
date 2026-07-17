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
use crate::ingest::IngestReport;
use crate::setup::{InstallCommit, SetupSummary, UninstallSummary};
use crate::start::Composition;
use crate::task::TaskListRow;
use engine::finding::{Finding, Findings, Severity};
use engine::introspect::{DefinitionKind, Description};
use engine::milestone::JoinOutcome;
use engine::result::{NextStep, Orientation, OrientationView, ResolutionTree, ValidationReport};
use serde::Serialize;
use std::collections::BTreeSet;

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
                header,
                workflows,
                next_steps,
                ..
            } => orientation_clean(header, &Orientation::new(workflows.clone()), next_steps),
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
/// next-step directive, the **off-catalog** next-step verbs (one route-prose line
/// per present verb — `design/project-setup.md` → Off-catalog discoverability
/// (G6)), then the universal routing footer.
///
/// The `next_steps` are the off-catalog entry verbs the composed pack-set actually
/// provides (gated CLI-side on pack membership — `planning`, `ingest-existing`),
/// each named so a bare-`start` reader discovers it without already knowing the
/// verb. An empty `next_steps` renders no extra line — the omitting context stays
/// inert (the dev-only floor omits `planning`).
///
/// Branch/HEAD and the "Recent: …" finalization line from the design example are
/// deferred — they need git-HEAD inspection and task state, neither of which
/// exists in increment 1 (bare `start` is read-only, no task store yet).
pub fn orientation_clean(
    header: &str,
    orientation: &Orientation,
    next_steps: &[NextStep],
) -> String {
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
        "\nRun: `jigc start \"<intent>\"`   — presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it\n",
    );
    // The off-catalog next-step verbs — one route-prose line per verb the pack-set
    // ships (same shape for each), so milestone planning + the existing-project
    // on-ramp are discoverable though they are not in the catalog above.
    for step in next_steps {
        out.push_str("Run: `jigc start --workflow ");
        out.push_str(&step.id);
        out.push_str("`   — ");
        out.push_str(&step.gist);
        out.push('\n');
    }
    out.push_str(ROUTING_FOOTER);
    out
}

/// Render a composed workflow to the surface `format` selects: `agent` / `human`
/// emit the [`minted_header`], then the engine's four-class composed text, then the
/// [`task_state_lines`], then the [`create_gates_line`], then the routing footer
/// (`design/workflow-dialect.md` → Routing footer — every composed workflow output in
/// agent-text and human-pretty ends with the one-line footer); `json` emits the
/// **generic** JSON projection of the
/// [`ComposedWorkflow`](engine::compose::ComposedWorkflow) with **no** footer and
/// **no** presentation lines (consumed by tooling, not the agent's reading flow).
///
/// All the presentation lines are appended here, in the frontend — never by the engine,
/// which stays presentation-free (the engine view carries the bare text). The composed
/// text already ends with a trailing newline; the task-state lines, the gates line, and
/// the footer follow it, each on its own line, and the header precedes it.
///
/// The header + task-state + gates ride on the CLI-side [`Composition`], **not** on the
/// engine's composed view: the composed-output JSON is pinned at exactly `{task, text}`
/// (`design/command-output-contract.md` §1), so the agent-facing gate list, mint
/// announcement, and task-state affordances are presentation, and add no key to the
/// contract.
///
/// Wired into the `jigc start "<intent>"` dispatch (`crate::cli::run_compose`),
/// which mints a task and emits this composed view.
pub fn composed(format: Format, view: &Composition) -> String {
    match format {
        // Exactly the pinned `{task, text}` projection — the presentation lines below
        // reach no tooling consumer.
        Format::Json => json(&view.view),
        Format::Agent | Format::Human => {
            let text = &view.view.text;
            let header = minted_header(view);
            let state = task_state_lines(view);
            let gates = create_gates_line(&view.gates);
            let mut out = String::with_capacity(
                header.len() + text.len() + state.len() + gates.len() + ROUTING_FOOTER.len() + 1,
            );
            out.push_str(&header);
            out.push_str(text);
            if !text.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&state);
            out.push_str(&gates);
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The `task minted: <id>` header a **work-minting** compose opens with (M42) — the id
/// every subsequent call in the loop requires (`jigc doc … --task <id>`, `jigc task
/// finalize <id>`), stated on its own line.
///
/// Before it, agent text revealed the minted id **only inside a `Run:` command string** in
/// some step's body — structurally present in `--format json` since M41-V2, but never
/// *stated* on the surface an agent actually reads.
///
/// It states a **mint**, so it renders exactly where this invocation minted the id — keyed
/// on [`Composition::minted`], **not** on `view.task.is_some()`: a resume / sub-agent
/// re-entry carries the *given* id and mints nothing, and a `creates-task: false` compose
/// (the router) mints nothing at all. Both render **no bytes** — the omitting context stays
/// inert, and the composed text opens exactly as before.
fn minted_header(view: &Composition) -> String {
    match (view.minted, view.view.task.as_deref()) {
        (true, Some(id)) => format!("task minted: {id}\n\n"),
        _ => String::new(),
    }
}

/// The three task-state lines an **id-carrying** compose appends below the composed
/// text (M43 Inc 7 / B3+B4, `design/surface-contract.md` → law 2 — resume and
/// what's-left are named by the surfaces producing the state):
///
/// - `resume:` — `jigc start --task <id>`, the designated recovery after context loss;
/// - `what's-left:` — `jigc task validate <id>`, the preview of what finalize gates on;
/// - `task scope:` — the B3 statement: `jigc doc` writes default to the **single**
///   active task, and the explicit `--task <id>` is the override that wins when
///   several are active (`crate::doc`'s task-resolution contract, stated where the
///   state is produced instead of learned from the more-than-one rejection).
///
/// Unlike [`minted_header`] — which states an **invocation fact** (this run minted)
/// and so stays off a resume — these state **standing affordances of the active-task
/// state**, as true (and as needed: a resume happens exactly when context was lost) on
/// a re-compose as on a mint. So they key on the **id's presence**, not on
/// [`Composition::minted`]; the id-less contexts (the router, a plain named compose)
/// render **no bytes** — the omitting context stays inert.
fn task_state_lines(view: &Composition) -> String {
    let Some(id) = view.view.task.as_deref() else {
        return String::new();
    };
    format!(
        "resume: `jigc start --task {id}`   — re-composes this workflow if context is lost\n\
         what's-left: `jigc task validate {id}`   — previews the findings finalize will gate on\n\
         task scope: `jigc doc` writes default to the single active task; `--task {id}` is the explicit override and wins when several are active\n"
    )
}

/// The `create-gates:` line a composed task carries immediately before the
/// [`ROUTING_FOOTER`] (M42) — the doctypes the composing workflow's `allows-create`
/// grants, in declaration order (`create-gates: adr, changelog`).
///
/// Before this line, the **only** surface in the binary that named a task's gates was
/// the `create.gate-blocked` **refusal** — an agent learned its gates by tripping one.
/// It joins the footer's mold (frontend-appended presentation, agent/human only) rather
/// than the JSON contract, which stays pinned at `{task, text}`.
///
/// A workflow that grants **no** gate (`quick-fix`, a `creates-task: false` router)
/// renders **no bytes at all** — the line is omitted, never printed as `none`: the
/// omitting context stays inert, and the footer follows the composed text exactly as
/// before.
fn create_gates_line(gates: &[String]) -> String {
    if gates.is_empty() {
        return String::new();
    }
    format!("create-gates: {}\n", gates.join(", "))
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
/// → "any scalar-key overrides applied with their source layer"), one
/// `rejected demotion: <key> = <attempted> below floor <floor>  (<layer>)` line per
/// below-floor `scalar-set` the cascade soft-rejected — shown **distinctly** from
/// the applied overrides (`design/workflow-dialect.md` → "any **rejected**
/// scalar-sets … shown distinctly … with the attempted value, the floor, and its
/// source layer"; `design/overrides.md` → Soft-rejection), and one line per
/// resolved include carrying the step's id, its source layer, and — for a
/// `replace-step` slot — the `← replaces <id> at position N` annotation.
fn explain_agent_text(tree: &ResolutionTree, pack_label: &str) -> String {
    let mut out = format!(
        "workflow:{}    ({} · {pack_label})\n",
        tree.workflow,
        tree.workflow_layer.label(),
    );
    // The adjudicated top-level cross-pack collision winners — one line per id-space
    // a multi-pack composition precedence-resolved, naming the winning pack
    // (`design/multi-pack.md` → Provenance). A single-pack composition carries none,
    // so no line renders and the output is byte-identical to today.
    for winner in &tree.collision_winners {
        out.push_str(&format!(
            "  collision: {} → won by {}/{}\n",
            winner.collision, winner.pack_id, winner.pack_version,
        ));
    }
    // The composed pack-set's per-pack provenance inputs — one `Pack input:` line
    // per composed pack, naming its resolving path + blake3 content-hash (after the
    // collision-winner lines, highest-precedence first), so the human sees the exact
    // pack input behind a deterministic outcome — id/version alone is not the
    // identity (`design/multi-pack.md` → Provenance under N packs;
    // `design/worked-examples.md` → flow 17 assertion 5). A single-pack composition
    // carries one entry; a tree with none renders no line, byte-identical to today.
    for input in &tree.pack_inputs {
        out.push_str(&format!(
            "  Pack input: {}/{} = {}  (blake3 {})\n",
            input.pack_id, input.pack_version, input.path, input.content_hash,
        ));
    }
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
    // The soft-rejected below-floor `scalar-set`s — shown distinctly from the applied
    // overrides above, with the attempted value, the floor it ranked below, and the
    // source layer (`design/workflow-dialect.md` → `--explain` output contract;
    // `design/overrides.md` → Soft-rejection). Absent entirely when no demotion was
    // floor-rejected.
    for rejected in &tree.rejected_demotions {
        out.push_str(&format!(
            "    rejected demotion: {} = {} below floor {}    ({})\n",
            rejected.key,
            rejected.attempted,
            rejected.floor,
            rejected.layer.label(),
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
    validation_scoped(
        format,
        report,
        "no findings — the task validates clean",
        None,
        None,
    )
}

/// Render the `jigc upgrade` sweep's [`ValidationReport`]: like [`validation`] but the
/// clean line names **what was checked** — the recorded config deltas re-applied
/// against the current pack — never the task-scoped wording (round-2 D4: `upgrade`
/// checks config deltas, not a task, and "no recorded deltas" is a different clean
/// state than "N deltas re-apply clean").
pub fn validation_upgrade(format: Format, report: &ValidationReport, checked: usize) -> String {
    let clean = if checked == 0 {
        "no findings — no recorded config deltas to check against the current pack".to_string()
    } else {
        format!(
            "no findings — {checked} recorded config delta(s) re-apply clean against the current pack"
        )
    };
    validation_scoped(format, report, &clean, None, None)
}

/// Render a [`ValidationReport`] for the **store-scope** sweep (`jigc validate`): like
/// [`validation`] but task-less, so (a) the clean line is store-scoped — it validates the
/// committed store, not a task, and must not reuse the "the task validates clean" wording
/// (M26 shakedown #10b) — and (b) it carries a **report-only clarification** so the
/// exit-code contract is unambiguous from the output. The store sweep is detect-and-report:
/// content findings are *listed on `blocking · …` lines* (the doc's **cascade** severity —
/// what would gate at `finalize`) yet the run **exits 0**; only the three exit-flipping
/// exceptions go non-zero (`design/validation.md` → Exit semantics — report-only, with three
/// exit-flipping exceptions). Without a trailer a human eyeballing `blocking`, or a script
/// chaining `jigc validate && deploy`, misreads a report-only store finding as a gate
/// failure. So the agent/human view appends a [`store_trailer`] naming where these findings
/// actually gate, and the JSON adds a machine-readable `report_only` (+ `scope`) signal —
/// the per-finding severity token is left untouched (it is meaningful).
pub fn validation_store(
    format: Format,
    report: &ValidationReport,
    unbaselined: &BTreeSet<String>,
) -> String {
    // The three exit-non-zero exceptions (`validation.md` → Exit semantics): a
    // `pack-probe-integrity.*` meta-finding (the probe could not be trusted, so the sweep
    // cannot claim a result), a `reconciliation.rename` finding (an out-of-band `git mv`,
    // a structural-identity event this commit introduced), and — M42 — a version-currency
    // break (the corpus is below its manifest version, so *every other family* adjudicated
    // docs against a schema they were never written to: the same untrustworthy-sweep
    // criterion as the crashed probe). Any of the three flips `report_only` false; the
    // trailer distinguishes the three wordings. The exit decision is the shared
    // [`validation_store_exit_flips`] both this renderer's `report_only` field and
    // `run_validate_store`'s exit code key on, so all three stay truthful in lockstep.
    let probe_unreliable = report
        .findings
        .iter()
        .any(|f| f.probe == "pack-probe-integrity");
    let oob_rename = report
        .findings
        .iter()
        .any(|f| f.code == "reconciliation.rename");
    let unmigrated_corpus = report
        .findings
        .iter()
        .any(|f| f.code == engine::validate::SCHEMA_VERSION_CURRENT_CODE);
    match format {
        Format::Json => {
            let mut value = serde_json::to_value(report).expect("validation report serializes");
            if let Some(object) = value.as_object_mut() {
                object.insert(
                    "scope".to_string(),
                    serde_json::Value::String("store".to_string()),
                );
                object.insert(
                    "report_only".to_string(),
                    serde_json::Value::Bool(!validation_store_exit_flips(report)),
                );
            }
            json(&value)
        }
        Format::Agent | Format::Human => {
            let trailer = store_trailer(
                report,
                probe_unreliable,
                oob_rename,
                unmigrated_corpus,
                unbaselined,
            );
            validation_scoped(
                format,
                report,
                "no findings — the committed store validates clean",
                Some(&trailer),
                Some(unbaselined),
            )
        }
    }
}

/// Whether the store-scope sweep's exit flips non-zero — the **three exit-flipping
/// exceptions** to the report-only stance (`validation.md` → Exit semantics): a
/// `pack-probe-integrity.*` meta-finding (the probe could not be trusted), a
/// `reconciliation.rename` finding (an out-of-band `git mv` — a structural-identity event
/// this commit introduced), or (M42) an [`engine::validate::SCHEMA_VERSION_CURRENT_CODE`]
/// break — a **managed** committed instance below its doctype's manifest version, i.e. an
/// **unmigrated corpus**, where every other family adjudicated docs against a schema they
/// were never written to. All three meet the class's own recorded criterion: *the sweep
/// could not produce a trustworthy result*. The report-only rule for **content** findings is
/// untouched — an invalid enum, a malformed date, a dangling ref keep their codes and their
/// exit 0.
///
/// **The "managed arm only" condition needs no extra test here**: `SCHEMA_VERSION_CURRENT_CODE`
/// is emitted *only* on the managed arm of the fifth family's discriminator (a foreign squatter
/// at a placement home takes the advisory `schema-conformance.unadopted-instance` instead), so
/// keying on the code **is** the condition — a stock brownfield repo that has only run `jigc
/// setup` stays exit-0 (`crates/cli/tests/managed_vs_foreign.rs`, the foreign arm).
///
/// The single source of truth shared by the dispatcher's exit code ([`crate::cli`]'s
/// `run_validate_store`), the JSON `report_only` field, and the human/agent trailer, so all
/// three stay truthful in lockstep.
pub(crate) fn validation_store_exit_flips(report: &ValidationReport) -> bool {
    report.findings.iter().any(|f| {
        f.probe == "pack-probe-integrity"
            || f.code == "reconciliation.rename"
            || f.code == engine::validate::SCHEMA_VERSION_CURRENT_CODE
    })
}

/// The **store-scope-only** check ids — the findings that gate **nowhere**, and which the
/// report-only trailer must therefore never claim a task-scope gate for (`validation.md` →
/// The trailer must not claim a gate that does not exist, M42).
///
/// **Derived from the emit sites, not from prose.** Each code below is emitted only on a
/// store-scope path and by **no** task-scope path, so neither `jigc task validate` nor the
/// `finalize` preflight (both [`engine::validate::validate_task`]) can ever see it:
///
/// - `schema-conformance.{mention-resolves, repeatable-populated, surplus-sections-absent,
///   unadopted-instance}` and `schema-completeness.inverse-cardinality` — the store-only
///   families of `validate_store_families` (`mention_resolves_store`, `hollow_surplus_store`,
///   `schema_conformance_store`'s foreign arm, `inverse_cardinality_store`; completeness and
///   in-prose mentions depend on *other* tasks, so they are by design never a per-task gate).
/// - `file-state.un-baselined` — the read-only committed-store twin's UNKNOWN outcome
///   (`file_state::detect_committed_store`); the task path *adopts* a baseline instead
///   (`file-state.baseline-adopt`), so this code never reaches a gate.
/// - `file-state.{orphaned-doc, unregistered-doc}` and `store-version.binary-mismatch` — the
///   CLI-minted store advisories (`crate::cli`'s orphan tiers, `crate::setup`), un-keyed and
///   store-scope by construction.
///
/// - `schema-conformance.schema-version-current` — the version-currency break
///   (`validate_store_families`' fifth family, managed arm). It gates **nowhere** in the system:
///   at task scope a stale committed doc downgrades to `advisory ·
///   reconciliation.conformance-block`, and a finalize over a stale corpus was proven to commit.
///   It never reaches the *trailer's* report-only branch (its presence takes the unmigrated-corpus
///   case above), but it **does** reach the **per-finding gate label** (M42 Inc 12 / T5) — which
///   is why it belongs on this list rather than in the trailer's branch alone: it is blocking and
///   `Location`-less, so without it the label would be granted (the conservative
///   no-address-no-suppression direction) and claim a finalize gate that does not exist.
///
/// **This list is necessary but not sufficient** — it answers *"can any task-scope path emit this
/// **code**?"*, while the claim the trailer makes is about a **finding**: *does a gate exist for
/// **this** finding?* [`gates_at_task`] asks the second question; the per-doc conformance families
/// need the baseline discriminator on top of this list.
const GATES_NOWHERE: &[&str] = &[
    "schema-conformance.mention-resolves",
    "schema-conformance.repeatable-populated",
    "schema-conformance.surplus-sections-absent",
    "schema-conformance.unadopted-instance",
    "schema-completeness.inverse-cardinality",
    "file-state.un-baselined",
    "file-state.orphaned-doc",
    "file-state.unregistered-doc",
    "store-version.binary-mismatch",
    engine::validate::SCHEMA_VERSION_CURRENT_CODE,
];

/// Whether a **gate exists for this finding** — the criterion the report-only trailer's claim
/// actually asserts (`validation.md` → The trailer must not claim a gate that does not exist,
/// M42). Two conditions, and the second is why the code-level [`GATES_NOWHERE`] list alone is not
/// enough:
///
/// 1. **The code reaches no task-scope path at all** ([`GATES_NOWHERE`]) — store-scope-only by
///    construction, so neither `jigc task validate` nor the `finalize` preflight can ever see it.
///
/// 2. **The finding is a per-doc conformance break over an *un-baselined* committed doc.** The
///    store sweep parses every committed instance itself, so it raises `conformance.*` (the
///    `parse_sections` `Err` arm) and `schema-conformance.*` findings **directly over committed
///    docs**. `engine::validate::validate_task` never does: it parses only the task's **staged**
///    instances, and routes the *committed* store through
///    `file_state::reconcile_committed_store`, which grades a nonconformant committed file by
///    **baseline membership** —
///    - **baselined** (a recorded hash) and drifted → **blocking** `reconciliation.conformance-block`:
///      a real gate. The claim stands.
///    - **un-baselined** (no record — a fresh clone, a brownfield adoption, any hand-authored
///      corpus, i.e. the dominant `jigc validate` corpus) → **advisory**
///      `reconciliation.conformance-block` (`conformance_advisory_finding` — *routed, not
///      recorded*). Nothing gates on it, ever. The claim is a lie.
///
///    So the discriminator is `FileStateRecord` membership — the same one M40's two-tier orphan
///    route keys on. `unbaselined` carries the `<type>:<slug>` identities of the committed
///    instances with no record entry, and a store-scope per-doc finding is addressed at exactly
///    that identity (`attribute_to_doc` rewrites its address to `<identity>` or
///    `<identity>#<fragment>`), so the two join on the address's identity part.
///
/// A finding addressed at nothing, or at a doc that is not a committed instance, takes no
/// suppression — the conservative direction: the claim is only ever *withdrawn* on a proof that
/// no gate exists, never granted on the absence of one.
///
/// [`gates_at_finalize`] narrows this to the findings whose gate is a **finalize block** — the
/// claim the per-finding label makes.
fn gates_at_task(finding: &Finding, unbaselined: &BTreeSet<String>) -> bool {
    if GATES_NOWHERE.contains(&finding.code.as_str()) {
        return false;
    }
    let per_doc_conformance = finding.code.starts_with("conformance.")
        || finding.code.starts_with("schema-conformance.")
        || finding.code.starts_with("schema-completeness.");
    if !per_doc_conformance {
        return true;
    }
    let Some(address) = finding.location.as_ref().and_then(|l| l.address.as_deref()) else {
        return true;
    };
    let identity = address.split('#').next().unwrap_or(address);
    !unbaselined.contains(identity)
}

/// Whether **this** store-scope finding is one a `finalize` will actually stop on — the claim the
/// per-finding gate label makes (M42 Inc 12 / T5; `validation.md` → The trailer must not claim a
/// gate that does not exist, whose closing note pins this label as a *dependency* on that fix,
/// never a papercut: printed over the un-corrected criterion it would put the false gate claim on
/// **every row**).
///
/// The store sweep prints each finding at its **cascade severity** while exiting 0, so `blocking ·`
/// on its own tells the reader nothing about whether anything ever *stops* on it. Two conditions,
/// both necessary:
///
/// 1. **A gate exists for the finding at all** — [`gates_at_task`], the trailer's own criterion
///    (the store-scope-only codes, and the un-baselined-committed-doc discriminator).
/// 2. **The finding's cascade severity is `blocking`.** A `warning`/`advisory` finding is surfaced
///    at the task boundary but never blocks the transaction, so labelling it *"gates at finalize"*
///    would be exactly the falsehood this label exists to retire (the cascade can demote any
///    non-intrinsic check — `flow13_contract_and_severity.rs` demotes a `doc-code` break to
///    `warning`, and that demoted row must claim no gate).
fn gates_at_finalize(finding: &Finding, unbaselined: &BTreeSet<String>) -> bool {
    matches!(finding.severity, Severity::Blocking) && gates_at_task(finding, unbaselined)
}

/// The store-scope clarifying trailer appended after the findings (`jigc validate`), so
/// exit-0-with-`blocking`-findings is unambiguous. Four cases, matching the three
/// exit-flipping exceptions (`validation.md` → Exit semantics): for the
/// `pack-probe-integrity.*` exception (`probe_unreliable`) it says the sweep could not
/// complete and exits non-zero; for a store-scope `reconciliation.rename` (`oob_rename`,
/// M35) it says an out-of-band rename was detected and the sweep exits non-zero; for a
/// version-currency break (`unmigrated_corpus`, M42) it says the corpus is unmigrated — so
/// every other finding in the report was adjudicated against the wrong schema — and names
/// `jigc migrate-corpus`, the verb that clears it; otherwise the content findings are
/// **report-only** at store scope (exit 0) and it names where they actually gate.
/// Probe-unreliability dominates (it taints the whole result); the unmigrated corpus is next
/// (it taints every *content* verdict below it). Ends with a newline so the caller appends
/// the routing footer on its own line.
///
/// **The report-only branch claims a gate only where one exists (M42).** Its blanket sentence
/// — *"these gate at `jigc task validate` / `jigc task finalize`"* — is **false** for every
/// [`GATES_NOWHERE`] code (emitted by no task-scope path, so no gate can ever see them: a stock
/// brownfield repo, whose only finding is the adoption advisory, was told to go look for a gate
/// that will never fire) **and** for a per-doc conformance break over an **un-baselined**
/// committed doc (at task scope the reconciler grades that doc *advisory*, never blocking — the
/// dominant `jigc validate` corpus). [`gates_at_task`] decides per finding; this branch counts
/// the findings that *do* carry a gate and scopes the claim to them: all → the original sentence;
/// none → no gate claim at all; mixed → how many, and that the rest gate nowhere.
fn store_trailer(
    report: &ValidationReport,
    probe_unreliable: bool,
    oob_rename: bool,
    unmigrated_corpus: bool,
    unbaselined: &BTreeSet<String>,
) -> String {
    if probe_unreliable {
        "pack-probe-integrity finding(s) present — the sweep could not complete and exits \
         non-zero; the store result is not trustworthy.\n"
            .to_string()
    } else if oob_rename {
        "out-of-band rename detected — a structural-identity change this commit introduced; \
         the sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).\n"
            .to_string()
    } else if unmigrated_corpus {
        "the committed corpus is below its schema-version — every other finding above was \
         adjudicated against a schema those docs were never written to, so the sweep exits \
         non-zero; run `jigc migrate-corpus`, then re-validate.\n"
            .to_string()
    } else {
        let n = report.findings.len();
        let gating = report
            .findings
            .iter()
            .filter(|f| gates_at_task(f, unbaselined))
            .count();
        if gating == 0 {
            format!(
                "{n} finding(s) — report-only at store scope (exit 0); each gates nowhere — a \
                 store-scope advisory, actionable through its own route above.\n"
            )
        } else if gating == n {
            format!(
                "{n} finding(s) — report-only at store scope (exit 0); these gate at \
                 `jigc task validate` / `jigc task finalize`.\n"
            )
        } else {
            format!(
                "{n} finding(s) — report-only at store scope (exit 0); {gating} of them gate at \
                 `jigc task validate` / `jigc task finalize`; the rest are store-scope advisories \
                 that gate nowhere — follow each finding's route above.\n"
            )
        }
    }
}

/// Shared body for the validation views: emit one line per finding (or `clean_line`
/// when the report is empty), then an optional `trailer` (only when findings are present),
/// followed by the routing footer; JSON is the generic projection with no footer. The
/// scope-dependent surfaces are the clean line, the trailer, and the per-finding gate label.
///
/// `store_gates` is `Some(unbaselined)` for the **store** view only (M42 Inc 12 / T5): each
/// finding that a `finalize` would actually stop on ([`gates_at_finalize`]) renders `blocking
/// (gates at finalize) · …` rather than a bare `blocking ·` that exits 0. The **task** view passes
/// `None` — there the severity token already *is* the verdict (`blocking` blocks the transaction
/// in hand), so a gate label would be noise, and the surface stays byte-identical.
fn validation_scoped(
    format: Format,
    report: &ValidationReport,
    clean_line: &str,
    trailer: Option<&str>,
    store_gates: Option<&BTreeSet<String>>,
) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = String::new();
            if report.findings.is_empty() {
                out.push_str(clean_line);
                out.push('\n');
            } else {
                for finding in &report.findings {
                    let gates = store_gates.is_some_and(|u| gates_at_finalize(finding, u));
                    out.push_str(&finding_line(finding, gates));
                }
                if let Some(trailer) = trailer {
                    out.push_str(trailer);
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The landed-commit facts a successful `task finalize` confirms back to the user
/// (M26 post-completion shakedown): the short commit `hash` + its `subject`, each
/// persisted doc `promoted` to its canonical repo location, and the `files` count of
/// the landed commit. Serializes as the `committed` object the JSON envelope carries.
#[derive(Serialize)]
pub struct Landed {
    /// The landed commit's abbreviated hash (`git rev-parse --short HEAD`).
    pub hash: String,
    /// The landed commit's subject line (the rendered commit doc's first line).
    pub subject: String,
    /// Each promoted persisted doc's canonical repo-relative path (empty when a
    /// commit-only task promotes nothing).
    pub promoted: Vec<String>,
    /// The number of files the landed commit touched (`= manifest.len()`).
    pub files: usize,
    /// The **included** manifest — every path the landed commit carried (its `git show
    /// --name-status HEAD` delta), tagged by how it entered. The per-task `IndexHonoring`
    /// stage lands the index, so this is the staged set (a promoted doc, a modified/deleted
    /// tracked file, the staged side of an `MM` path).
    pub manifest: Vec<ManifestEntry>,
    /// The **left-out** residual the index commit left behind (post-commit `git status
    /// --porcelain` worktree column — M30 G3): unstaged/untracked WIP the agent must
    /// `git add` to include. Rendered identically to the dry-run forecast's `left_out`
    /// (the measurement envelope's dry-run/landed symmetry). A staged-then-further-modified
    /// (`MM`) path lands its staged side in `manifest` and its worktree residual here, so it
    /// appears in **both**.
    pub left_out: Vec<ManifestEntry>,
}

/// How a path entered the finalize commit set in the pre-commit manifest (B1 dirty-tree
/// sweep): a managed doc `Promoted` to its canonical location, a tracked `Modified` file,
/// a tracked `Deleted` file, a newly-`Added` (deliberately staged) file, or an `Untracked`
/// file left out of the commit (the stray-file signal a tester needs). `Added` and
/// `Untracked` are distinct on purpose: a staged new file the agent `git add`-ed is an
/// **included** add (nothing was swept), while `Untracked` only ever tags a **left-out**
/// file the commit excluded.
///
/// `CarriedOver` (M43, `design/surface-contract.md` → The carryover gate) labels a
/// **pre-task staged** entry (add, modify, or delete) riding the commit under a declared
/// `--carry-staged` — a label over the same set, never a membership change, rendered
/// identically at all four sites (dry-run forecast, pre-commit print, landed text,
/// landed JSON). The JSON value `carried-over` is a pre-1.0 additive enum extension,
/// declared in `design/command-output-contract.md` → Evolution posture.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ManifestKind {
    Promoted,
    Modified,
    Deleted,
    Added,
    Untracked,
    CarriedOver,
}

/// One entry in the finalize pre-commit manifest: a repo-relative `path` and the `kind`
/// of change that placed it in the commit set.
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ManifestEntry {
    /// The repo-relative path.
    pub path: String,
    /// How the path entered the commit set.
    pub kind: ManifestKind,
}

/// One agent-text manifest line for an **included** commit member: `  promoted <path>` /
/// `  modified <path>` / `  deleted <path>` / `  added <path>` (a deliberately-staged new
/// file) / `  carried-over <path>` (a pre-task staged entry riding under `--carry-staged`).
/// No trailing newline — the caller joins / closes it. `Untracked` never reaches
/// the included path (it tags only left-out files, rendered by [`left_out_lines`]); a
/// defensive arm renders it under the left-out wording rather than the retired "swept".
fn manifest_line(entry: &ManifestEntry) -> String {
    match entry.kind {
        ManifestKind::Promoted => format!("  promoted {}", entry.path),
        ManifestKind::Modified => format!("  modified {}", entry.path),
        ManifestKind::Deleted => format!("  deleted {}", entry.path),
        ManifestKind::Added => format!("  added {}", entry.path),
        ManifestKind::Untracked => format!("  untracked {}", entry.path),
        ManifestKind::CarriedOver => format!("  carried-over {}", entry.path),
    }
}

/// The left-out section a finalize manifest appends when the working tree carries
/// unstaged/untracked changes the commit (the index) leaves behind (M30 G3): a guidance
/// header naming the set, then one indented path per [`ManifestEntry`]. Empty when nothing
/// is left out. Shared by the dry-run forecast and the landed residual so both surfaces
/// render the left-out set identically.
fn left_out_lines(left_out: &[ManifestEntry]) -> Vec<String> {
    if left_out.is_empty() {
        return Vec::new();
    }
    let mut lines = vec!["  left-out (unstaged/untracked — git add to include):".to_string()];
    lines.extend(left_out.iter().map(|entry| format!("    {}", entry.path)));
    lines
}

/// The **pre-commit** `left-out` advisory a landing `task finalize` prints *before* it
/// commits (M42, `design/finalize.md` → "The `left-out` advisory prints BEFORE the commit
/// too"): a titled line, then the same [`left_out_lines`] section the dry-run forecast and
/// the landed residual render — so all three surfaces name the left-out set identically.
/// Empty (no bytes) when the tree leaves nothing out; ends with a newline (the caller
/// emits it as-is on the stream the format selects). It **surfaces only** — the commit
/// still lands; the block stays reserved for the empty-index case.
pub fn left_out_advisory(left_out: &[ManifestEntry]) -> String {
    let lines = left_out_lines(left_out);
    if lines.is_empty() {
        return String::new();
    }
    let mut out = String::from("finalize — committing the index; leaving out:\n");
    for line in lines {
        out.push_str(&line);
        out.push('\n');
    }
    out
}

/// The **pre-commit** carried-over print a landing `task finalize` emits before it
/// commits (M43, `design/surface-contract.md` → The carryover gate): on a
/// `--carry-staged` run the pre-task staged set is about to ride the whole-index
/// commit by declaration, and this print names each carried path with the same
/// [`manifest_line`] label (`carried-over`) the dry-run forecast and the landed
/// manifest render — the four sites move together. Empty (no bytes) when nothing is
/// carried (a refused run never reaches this print); ends with a newline. Sits beside
/// [`left_out_advisory`] (the M42 print, untouched) on the same stream discipline.
pub fn carried_over_advisory(carried: &[ManifestEntry]) -> String {
    if carried.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "finalize — committing the index; carrying over (staged before this task existed — \
         declared with `--carry-staged`):\n",
    );
    for entry in carried {
        out.push_str(&manifest_line(entry));
        out.push('\n');
    }
    out
}

/// Frame a commit-phase **hook/git rejection** with the recoverability it always had but
/// never stated (M42, `design/finalize.md` → 6. Commit): git's `rejection` bytes stay
/// **verbatim and unwrapped** (the hook output *is* the correction signal — the M40
/// refinement's routed wrap covers jigc's *own* staging acts, never the user's hook
/// channel), and jigc's own sentence is added *around* them — the task survives a
/// rejection intact, so the same `finalize` re-run lands the commit once the hook's
/// complaint is fixed. Emitted on **stderr** by the caller; `json` carries the framed text
/// in the [`operational_error`] envelope so a tooling consumer still parses it.
pub fn commit_rejected(format: Format, task_id: &str, rejection: &str) -> String {
    let framed = format!(
        "{rejection}\n\ntask {task_id} is intact — nothing was committed and your staged \
         changes are still staged. Fix the hook's complaint, then re-run `jigc task finalize \
         {task_id}`."
    );
    match format {
        Format::Json => json(&serde_json::json!({ "error": framed })),
        Format::Agent | Format::Human => framed,
    }
}

/// Render the `task finalize --dry-run` pre-commit manifest to the surface `format`
/// selects (M30 G3 — name what is **included** in the commit vs **left out** of it): `json`
/// emits `{ "dry_run": true, "manifest": [{path,kind}…], "left_out": [{path,kind}…] }`
/// (tooling-consumed, no footer); `agent` / `human` emit a titled block, one
/// [`manifest_line`] per included entry, then the [`left_out_lines`] section, with **no
/// trailing newline** — the caller's `println!` closes it, symmetric with [`landed_summary`].
pub fn finalize_manifest(
    format: Format,
    included: &[ManifestEntry],
    left_out: &[ManifestEntry],
) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "dry_run": true,
            "manifest": included,
            "left_out": left_out,
        })),
        Format::Agent | Format::Human => {
            let mut lines =
                vec!["finalize --dry-run — pre-commit manifest (nothing committed)".to_string()];
            lines.extend(included.iter().map(manifest_line));
            lines.extend(left_out_lines(left_out));
            lines.join("\n")
        }
    }
}

/// Render a **landed** `task finalize` to the surface `format` selects, pairing the
/// preflight findings envelope with a success summary that confirms what landed (M26
/// post-completion shakedown — a successful finalize was previously near-silent, leaving
/// a user to run `git log` to tell it worked):
///
/// - `agent` / `human` emit the [`validation`] envelope (findings + routing footer)
///   followed by a delimited success section naming the landed commit (short hash +
///   subject), each promoted persisted doc, and the file count — the same after-the-result
///   placement the hook relay uses (review S1), so the footer + the relay's invariants hold;
/// - `json` emits the **same** report envelope with a `committed` object added (the landed
///   facts), so a JSON consumer parsing the report keeps its `findings` array and gains
///   the commit facts as fields — no second object, no shape break.
pub fn finalize_landed(format: Format, report: &ValidationReport, landed: &Landed) -> String {
    match format {
        Format::Json => {
            let mut value = serde_json::to_value(report).expect("validation report serializes");
            if let Some(object) = value.as_object_mut() {
                object.insert(
                    "committed".to_string(),
                    serde_json::to_value(landed).expect("landed summary serializes"),
                );
            }
            json(&value)
        }
        Format::Agent | Format::Human => {
            let mut out = validation(format, report);
            out.push_str("\n\n");
            out.push_str(&landed_summary(landed));
            out
        }
    }
}

/// The agent-text success section a landed finalize appends after the routing footer:
/// `finalized <hash> — <subject>`, the [`manifest_line`] for each **included** path in the
/// commit (promoted / modified / deleted), a `  <n> file(s) committed` tally, then the
/// [`left_out_lines`] residual section naming the unstaged/untracked WIP the index commit
/// left behind (M30 G3 — rendered identically to the dry-run forecast). Ends without a
/// trailing newline — the caller's `println!` closes the line, symmetric with [`validation`].
fn landed_summary(landed: &Landed) -> String {
    let mut out = format!("finalized {} — {}\n", landed.hash, landed.subject);
    for entry in &landed.manifest {
        out.push_str(&manifest_line(entry));
        out.push('\n');
    }
    let noun = if landed.files == 1 { "file" } else { "files" };
    out.push_str(&format!("  {} {noun} committed", landed.files));
    for line in left_out_lines(&landed.left_out) {
        out.push('\n');
        out.push_str(&line);
    }
    out
}

/// A successful single-write `jigc doc` verb's confirmation — the positive ack the
/// silent write verbs were missing, so a write outcome is visible without re-reading
/// the working-area file (dogfood papercut). Rendered through `--format` by
/// [`doc_ack`]: a terse one-line confirmation on agent-text / human, a small
/// structured object on JSON. The plain-happy-path doc surface carries **no** routing
/// footer (the `create` / `add-item` sibling verbs emit a bare line too — only the
/// composed reading surfaces carry the footer).
///
/// Every variant carries `findings` — the **intrinsic single-doc advisories** the write
/// reports as data (`design/command-output-contract.md` §2 — findings-as-data on write;
/// today the `schema-conformance.surplus-sections-absent` check computed from the staged
/// buffer after persist). It projects into the JSON ack's `findings[]` (empty on a clean
/// write); completeness + cross-doc families stay store-scope, so `findings: []` means "no
/// intrinsic single-doc advisory," not "validated."
pub enum DocAck {
    /// A `set-field` landed `value` at `address`/`target`. `value` is the written field
    /// value shaped as `doc show`'s `fields` project it (scalar → string, list → array).
    Field {
        address: String,
        target: AckTarget,
        value: serde_json::Value,
        findings: Findings,
    },
    /// A `set-field --unset` cleared the field at `address`/`target` (its line/bullet
    /// removed). No `value` key — the effect is the field's absence, read back via `doc show`.
    UnsetField {
        address: String,
        target: AckTarget,
        findings: Findings,
    },
    /// A `set-slot` spliced `chars` characters of prose at `address`/`target`.
    Slot {
        address: String,
        target: AckTarget,
        chars: usize,
        findings: Findings,
    },
    /// A `remove-item` dropped the item at `address`/`target`.
    RemovedItem {
        address: String,
        target: AckTarget,
        findings: Findings,
    },
    /// A `retitle-item` retitled the item at `address`/`target` (anchor frozen) to `title`.
    RetitledItem {
        address: String,
        target: AckTarget,
        title: String,
        findings: Findings,
    },
    /// A `create` minted a whole doc at `address`/`target`. The target is the head only
    /// (`doctype`+`slug`, no fragment); its effect is the whole created doc, read back via
    /// `doc show` (so no per-op effect key — `design/command-output-contract.md` §2).
    /// `existed` is the **always-present** create-or-update discriminator (M43 inc-7 T1,
    /// contract §2): `false` on a fresh mint, `true` when the committed doc at the slug's
    /// canonical home was copied in for update — law 1's "an ack that says 'created'
    /// distinguishes created from already-existed" (`design/surface-contract.md`).
    Created {
        address: String,
        target: AckTarget,
        existed: bool,
        findings: Findings,
    },
    /// An `add-item` minted the item at `address`/`target`. `target.item` is the **minted**
    /// leaf-most id (the new item, not the bare section — the contract's "add-item's target
    /// is the new item"); its effect is that item, read back via `doc show` (no effect key).
    AddedItem {
        address: String,
        target: AckTarget,
        findings: Findings,
    },
    /// An `author` authored a whole doc at `address`/`target`. The target is the head only
    /// (`doctype`+`slug`), like [`DocAck::Created`]; its effect is the whole authored doc,
    /// read back via `doc show` (no effect key).
    Authored {
        address: String,
        target: AckTarget,
        findings: Findings,
    },
}

/// The decomposed write address a [`DocAck`] carries in `--format json` — the same
/// depth ladder `doc show`'s `#fragment` projects (`design/command-output-contract.md`
/// §2): `doctype` + `slug` always; `section`/`item`/`leaf` present exactly when the
/// address reaches that depth (a section-level slot carries only `section`; a
/// section-item write adds `item`; a leaf write adds `leaf`). A nested write flattens to
/// the leaf-most item id under `item` (the pinned flat 5-key shape). Absent hops are
/// omitted from the JSON, not emitted null.
#[derive(Serialize)]
pub struct AckTarget {
    pub doctype: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<String>,
}

/// Render a successful single-write `jigc doc` verb confirmation ([`DocAck`]) to the
/// surface `format` selects: `agent` / `human` emit a terse one-line confirmation (no
/// footer — symmetric with the bare line `doc create` / `doc add-item` emit); `json`
/// emits a small structured ack object on stdout (the house serde-object shape, like
/// [`milestone`]), so an agent on `--format json` gets a parseable confirmation.
pub fn doc_ack(format: Format, ack: &DocAck) -> String {
    // No seam call: the ack's `findings[]` is a `Findings`, so the membership test rides its
    // projection (`command-output-contract.md` → The membership test) — the presence half on
    // each `Finding`, the uniqueness half on the collection.
    match format {
        Format::Json => match ack {
            // The command-output contract (`design/command-output-contract.md` §2):
            // `op` + the decomposed `target` + the op's retained effect key +
            // `findings` (the intrinsic single-doc advisories, `[]` on a clean write —
            // each projects the pinned findings envelope via `Finding`'s `Serialize`).
            DocAck::Field {
                target,
                value,
                findings,
                ..
            } => json(&serde_json::json!({
                "op": "set-field", "target": target, "value": value, "findings": findings,
            })),
            // `--unset` shares the `set-field` op, with `unset: true` in place of a value.
            DocAck::UnsetField {
                target, findings, ..
            } => json(&serde_json::json!({
                "op": "set-field", "target": target, "unset": true, "findings": findings,
            })),
            DocAck::Slot {
                target,
                chars,
                findings,
                ..
            } => json(&serde_json::json!({
                "op": "set-slot", "target": target, "chars": chars, "findings": findings,
            })),
            DocAck::RemovedItem {
                target, findings, ..
            } => json(&serde_json::json!({
                "op": "remove-item", "target": target, "removed": true, "findings": findings,
            })),
            DocAck::RetitledItem {
                target,
                title,
                findings,
                ..
            } => json(&serde_json::json!({
                "op": "retitle-item", "target": target, "title": title, "findings": findings,
            })),
            // `create` / `add-item` / `author` join the envelope: `op` + the decomposed
            // `target` + `findings`. No per-op effect key — their effect is the whole
            // doc / the new item, read back via `doc show` (contract §2). `create`
            // additionally carries the always-present `existed` discriminator.
            DocAck::Created {
                target,
                existed,
                findings,
                ..
            } => json(&serde_json::json!({
                "op": "create", "target": target, "existed": existed, "findings": findings,
            })),
            DocAck::AddedItem {
                target, findings, ..
            } => json(&serde_json::json!({
                "op": "add-item", "target": target, "findings": findings,
            })),
            DocAck::Authored {
                target, findings, ..
            } => json(&serde_json::json!({
                "op": "author", "target": target, "findings": findings,
            })),
        },
        Format::Agent | Format::Human => match ack {
            DocAck::Field { address, value, .. } => {
                format!("set {address} = {}", ack_value_display(value))
            }
            DocAck::UnsetField { address, .. } => format!("unset {address}"),
            DocAck::Slot { address, chars, .. } => format!("set slot {address} ({chars} chars)"),
            DocAck::RemovedItem { address, .. } => format!("removed item {address}"),
            DocAck::RetitledItem { address, title, .. } => {
                format!("retitled item {address} to {title:?} (anchor frozen)")
            }
            // Agent-text is the bare address the verbs printed before joining the envelope
            // (byte-identical): the minted doc address (`create`/`author`) or the minted
            // item address (`add-item`) — the next address an agent drives. A copy-in
            // `create` appends the create-or-update note (M43 inc-7 T1) so the surface
            // states the effect: the committed body was carried in, not minted fresh.
            DocAck::Created {
                address,
                existed: true,
                ..
            } => format!("{address} (already existed — copied in for update)"),
            DocAck::Created { address, .. }
            | DocAck::AddedItem { address, .. }
            | DocAck::Authored { address, .. } => address.clone(),
        },
    }
}

/// A successful **task-state** mutation's confirmation — the ack `jigc task bind` and
/// `jigc task discard` join at M42 (`design/command-output-contract.md` §2 → The
/// task-state verbs join the envelope). Both mutate durable state and confirmed it with
/// silence in *every* format; "success is silence" is not a posture, it is the absence
/// of one.
///
/// The envelope is the write-ack shape ([`DocAck`]) with the subject a doc-write does not
/// have: the **work unit** — `task`, the id. Neither verb writes managed content, so the
/// intrinsic single-doc advisory has nothing to compute over; the `findings` key is
/// emitted as the empty array anyway, so a driver deserializes one envelope shape.
pub enum TaskAck {
    /// A `task bind` bound the doc at `target` to the task's `role`. `address` is the
    /// bound doc's URI (the agent-text line); `target` is that address decomposed —
    /// `doctype` + `slug` only, a bind targets a whole doc — stamped from the **parsed**
    /// address, never the raw CLI argument.
    Bound {
        task: String,
        role: String,
        address: String,
        target: AckTarget,
    },
    /// A `task discard` removed the task's working area. It carries no `target` (it
    /// addresses no doc) and **no effect key**: discarding an absent id never reaches the
    /// removal (it rejects at `TaskArea::resolve`, exit 1), so a `discarded` key could
    /// only ever hold the constant `true`. "Removed" vs "was already gone" is carried one
    /// layer up — exit 0 + this ack vs exit 1 + the error envelope
    /// (`design/command-output-contract.md` §2 → the ⚠ correction).
    ///
    /// `dropped` enumerates the **staged docs the removal threw away** (B3, 2026-07-17
    /// surface review — the style guide's own ack rule, "what a discard threw away":
    /// the working area's `docs/` set, `<type>:<slug>` identities, sorted). The
    /// agent-text line marks a transient doctype's instance `(transient)` — it was never
    /// going to commit as a file anyway; JSON carries the bare identities (an additive
    /// key, declared in `design/command-output-contract.md` §2).
    Discarded {
        task: String,
        dropped: Vec<DroppedStaged>,
    },
}

/// One staged doc a `task discard` threw away: its `<type>:<slug>` identity, plus
/// whether its doctype is **transient** (no `location:`/`placement:` — the instance
/// never persists past finalize, so dropping it loses no would-be-committed content).
pub struct DroppedStaged {
    pub doc: String,
    pub transient: bool,
}

/// Render a successful task-state verb's confirmation ([`TaskAck`]) to the surface
/// `format` selects: `agent` / `human` emit a terse one-line ack (symmetric with the
/// bare-line doc acks), `json` the structured envelope (`op` + `task` + `findings`, plus
/// `role` + the decomposed `target` on a bind).
pub fn task_ack(format: Format, ack: &TaskAck) -> String {
    match format {
        Format::Json => match ack {
            TaskAck::Bound {
                task, role, target, ..
            } => json(&serde_json::json!({
                "op": "task-bind", "task": task, "role": role, "target": target,
                "findings": [],
            })),
            TaskAck::Discarded { task, dropped } => json(&serde_json::json!({
                "op": "task-discard", "task": task,
                "dropped": dropped.iter().map(|d| d.doc.as_str()).collect::<Vec<_>>(),
                "findings": [],
            })),
        },
        Format::Agent | Format::Human => match ack {
            TaskAck::Bound {
                task,
                role,
                address,
                ..
            } => format!("bound {role} = {address} (task {task})"),
            TaskAck::Discarded { task, dropped } => {
                if dropped.is_empty() {
                    format!("discarded task {task}")
                } else {
                    let list: Vec<String> = dropped
                        .iter()
                        .map(|d| {
                            if d.transient {
                                format!("{} (transient)", d.doc)
                            } else {
                                d.doc.clone()
                            }
                        })
                        .collect();
                    format!(
                        "discarded task {task} — dropped staged edits to: {}",
                        list.join(", ")
                    )
                }
            }
        },
    }
}

/// The agent-text display of a shaped ack value: a scalar → its bare string, a list →
/// the inline `[a, b]` form (the on-disk authoring shape), so the terse confirmation
/// reads naturally where the JSON ack carries the structured `value`.
fn ack_value_display(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Array(elems) => {
            let parts: Vec<String> = elems.iter().map(ack_value_display).collect();
            format!("[{}]", parts.join(", "))
        }
        other => other.to_string(),
    }
}

/// A successful `jigc config <verb>` cascade-authoring write's confirmation — the
/// positive ack the six `config` verbs were missing (they mapped `Ok(())` to silence in
/// every format, inconsistent with every doc-write ack; the M43 surface census, Law 1
/// "acks state the effect"). Rendered through `--format` by [`config_ack`]: a terse
/// `config: <effect>` one-line confirmation on agent-text / human, a small structured
/// object on JSON.
///
/// No routing footer — a cascade-authoring write is not a composed reading surface (the
/// [`DocAck`] / [`TaskAck`] mold, whose bare-line acks likewise carry none). A `set`
/// that relocates committed docs prints its relocation lines separately, on stderr
/// (`config::route_docs_root_repoint_orphans`); this ack does not restate them.
pub enum ConfigAck {
    /// `config set <key> <value>` recorded a `scalar-set`.
    Set { key: String, value: String },
    /// `config insert-step` spliced a native `step` into `workflow`, `side`
    /// (`"after"`/`"before"`) the `anchor` step id.
    InsertStep {
        workflow: String,
        step: String,
        side: &'static str,
        anchor: String,
    },
    /// `config replace-step` swapped the `workflow:<id>#<step-id>` `target` for the
    /// native `step` (its basename).
    ReplaceStep { target: String, step: String },
    /// `config remove-step` dropped the `workflow:<id>#<step-id>` `target`.
    RemoveStep { target: String },
    /// `config fill` injected content into the `step:<id>#<fill-id>` `target`.
    Fill { target: String },
    /// `config fork` copied the `workflow:<id>#<step-id>` `target`'s body into a native
    /// `path`, pinning `base` (the blake3 prefix of the copied bytes).
    Fork {
        target: String,
        path: String,
        base: String,
    },
}

/// Render a successful `jigc config <verb>` confirmation ([`ConfigAck`]) to the surface
/// `format` selects: `agent` / `human` emit the terse `config: <effect>` line (no
/// footer — symmetric with the bare-line doc/task acks), `json` a small structured
/// object (`op` + the effect fields), so a `--format json` caller gets a parseable
/// confirmation instead of empty success.
pub fn config_ack(format: Format, ack: &ConfigAck) -> String {
    match format {
        Format::Json => match ack {
            ConfigAck::Set { key, value } => json(&serde_json::json!({
                "op": "config-set", "key": key, "value": value,
            })),
            ConfigAck::InsertStep {
                workflow,
                step,
                side,
                anchor,
            } => json(&serde_json::json!({
                "op": "config-insert-step", "workflow": workflow, "step": step,
                "side": side, "anchor": anchor,
            })),
            ConfigAck::ReplaceStep { target, step } => json(&serde_json::json!({
                "op": "config-replace-step", "target": target, "step": step,
            })),
            ConfigAck::RemoveStep { target } => json(&serde_json::json!({
                "op": "config-remove-step", "target": target,
            })),
            ConfigAck::Fill { target } => json(&serde_json::json!({
                "op": "config-fill", "target": target,
            })),
            ConfigAck::Fork { target, path, base } => json(&serde_json::json!({
                "op": "config-fork", "target": target, "path": path, "base": base,
            })),
        },
        Format::Agent | Format::Human => match ack {
            ConfigAck::Set { key, value } => format!("config: set `{key}` = `{value}`"),
            ConfigAck::InsertStep {
                workflow,
                step,
                side,
                anchor,
            } => format!("config: inserted step `{step}` into `{workflow}` {side} `{anchor}`"),
            ConfigAck::ReplaceStep { target, step } => {
                format!("config: replaced `{target}` with `{step}`")
            }
            ConfigAck::RemoveStep { target } => format!("config: removed `{target}`"),
            ConfigAck::Fill { target } => format!("config: filled `{target}`"),
            ConfigAck::Fork { target, path, base } => {
                format!("config: forked `{target}` -> {path} (pinned base {base})")
            }
        },
    }
}

/// One agent-text finding line: `<severity> · <code> — <message>`, plus an indented
/// `route:` line when the finding carries a repair direction (the settled
/// block-payload envelope — a hard block is a blocking finding carrying a route).
/// A route-less **advisory** is purely informational, and says so — the agent must
/// never be left inferring whether output wants something from it.
///
/// `gates` annotates the severity token as `blocking (gates at finalize)` (M42 Inc 12 / T5) — the
/// **store** view's per-finding gate label, decided by [`gates_at_finalize`]. Every other surface
/// passes `false`: at task scope the severity token already *is* the verdict, and the `setup` /
/// `ingest` finding lines are not store-sweep rows at all.
fn finding_line(finding: &Finding, gates: bool) -> String {
    let severity = match finding.severity {
        Severity::Blocking if gates => "blocking (gates at finalize)",
        Severity::Blocking => "blocking",
        Severity::Warning => "warning",
        Severity::Advisory => "advisory",
    };
    let mut line = format!("{severity} · {} — {}", finding.code, finding.message);
    if finding.route.is_none() && matches!(finding.severity, Severity::Advisory) {
        line.push_str("   (no action needed)");
    }
    line.push('\n');
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
        Format::Json => {
            let install_commit = match &summary.install_commit {
                InstallCommit::Committed(sha) => serde_json::Value::String(sha.clone()),
                InstallCommit::Nothing | InstallCommit::Skipped => serde_json::Value::Null,
            };
            json(&serde_json::json!({
                "installed": true,
                "line_file": summary.line_file,
                "allowlist_file": summary.allowlist_file,
                "install_commit": install_commit,
            }))
        }
        Format::Agent | Format::Human => {
            let mut out = String::from(
                "jigc setup — adapter installed\n\njigc is now wired into this project; setup installed:\n",
            );
            out.push_str("  - bootstrap reference → ");
            out.push_str(&summary.line_file);
            out.push_str("   (orients your assistant to `jigc start` each session)\n");
            out.push_str("  - jigc allowlist → ");
            out.push_str(&summary.allowlist_file);
            out.push_str("   (pre-approves the `jigc` commands the agent runs)\n");
            out.push_str("  - SessionStart hook → ");
            out.push_str(&summary.allowlist_file);
            out.push_str("   (runs `jigc start` to orient your assistant each session)\n");
            out.push_str("  - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)\n");
            // When setup committed its own install (M26), name that commit so the user
            // knows the scaffolding landed on its own, not in their first feature commit.
            if let InstallCommit::Committed(sha) = &summary.install_commit {
                out.push_str("  - install commit → ");
                out.push_str(sha);
                out.push_str("   (setup's install files are committed on their own, off your first feature commit)\n");
            }
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
    // No seam call: a bare `Finding` carries the presence half on its own `Serialize`
    // (`command-output-contract.md` → The membership test). Every `setup.*` / `uninstall.*`
    // code is a **declared singleton** (one finding per invocation), so the projection passes
    // them at `target: null` by the pin; anything else riding this surface owes a target.
    match format {
        Format::Json => json(finding),
        Format::Agent | Format::Human => {
            let mut out = finding_line(finding, false);
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a successful `jigc uninstall` teardown to the surface `format` selects
/// (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5)): `agent` /
/// `human` emit one bullet per artifact **actually** removed — the real teardown set,
/// so a no-op (a second `uninstall`, nothing present) reports a clean "nothing to
/// remove" line rather than over-claiming — followed by the routing footer; `json`
/// emits a generic object naming the two host targets plus the per-artifact `removed`
/// flags, with no footer (tooling-consumed). The machine-global `doc-code` probe is
/// left in place — B2.
pub fn uninstall_success(format: Format, summary: &UninstallSummary) -> String {
    let removed = &summary.removed;
    match format {
        Format::Json => json(&serde_json::json!({
            "uninstalled": true,
            "line_file": summary.line_file,
            "allowlist_file": summary.allowlist_file,
            // The real removal set — one flag per repo-local artifact, honest about
            // which were present vs already-absent (never over-claiming a no-op).
            "removed": {
                "jigc_dir": removed.jigc_dir,
                "reference": removed.reference,
                "allowlist": removed.allowlist,
                "hook": removed.hook,
                "deny": removed.deny,
                "precommit": removed.precommit,
            },
        })),
        Format::Agent | Format::Human => {
            let mut out = String::from("jigc uninstall — repo-local install removed\n\n");
            // One bullet per artifact **actually** removed — so the summary matches the
            // real teardown and never claims to remove what wasn't there (M36 honesty
            // fix). An all-absent teardown (a second `uninstall`) reports a clean no-op.
            if removed.is_empty() {
                out.push_str("  (nothing to remove — no repo-local jigc install was present)\n");
            } else {
                if removed.jigc_dir {
                    out.push_str("  - removed .jigc/\n");
                }
                if removed.reference {
                    out.push_str("  - unwired bootstrap reference ← ");
                    out.push_str(&summary.line_file);
                    out.push('\n');
                }
                if removed.allowlist {
                    out.push_str("  - removed jigc allowlist permit ← ");
                    out.push_str(&summary.allowlist_file);
                    out.push('\n');
                }
                if removed.hook {
                    out.push_str("  - removed SessionStart hook ← ");
                    out.push_str(&summary.allowlist_file);
                    out.push('\n');
                }
                if removed.deny {
                    out.push_str("  - removed deny safety floor ← ");
                    out.push_str(&summary.allowlist_file);
                    out.push('\n');
                }
                if removed.precommit {
                    out.push_str("  - removed pre-commit hook\n");
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a [`IngestReport`] to the surface `format` selects: `agent` / `human`
/// emit one row per discovered candidate (`<verdict> <file> → <type>`, in sorted
/// candidate order) followed by the routing footer; a `needs-reconcile` row carries
/// its routed finding indented beneath (the `severity · code — message` line + the
/// `route:` line — the same envelope OOB conflicts route through). `json` emits the
/// **generic** projection of the report with **no** footer (tooling-consumed). This
/// is the adopt-and-triage surface — adopt is register-only (the edge index +
/// file-state baseline; no candidate file is moved or rewritten), an adopted row is
/// marked distinctly (`design/project-setup.md` → Flow 2; `design/worked-examples.md`
/// → flow 12).
pub fn ingest(format: Format, report: &IngestReport) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = format!(
                "jigc ingest — {} candidate(s) classified  (sorted — deterministic report order)\n\n",
                report.rows.len(),
            );
            // V9 (M41 Inc 8): the `unmanaged` rows are the noise floor — a
            // 220-candidate adoption scan is mostly unmanaged, so one text line per
            // file blows the report up. Collapse them into per-directory counts keyed
            // on the directory (a `BTreeMap` — sorted, order-invariant output), keeping
            // the actionable `adoptable` / `needs-reconcile` rows itemized. The JSON
            // arm above stays full-rows (the tooling contract).
            let mut unmanaged_by_dir: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            for row in &report.rows {
                if row.verdict == "unmanaged" {
                    let dir = match row.file.rfind('/') {
                        Some(i) => row.file[..=i].to_string(),
                        None => "./".to_string(),
                    };
                    *unmanaged_by_dir.entry(dir).or_insert(0) += 1;
                    continue;
                }
                out.push_str(row.verdict);
                out.push(' ');
                out.push_str(&row.file);
                match &row.best_match {
                    Some(ty) => {
                        out.push_str(" → ");
                        out.push_str(ty);
                    }
                    None => out.push_str(" → (parses against no schema — left untouched)"),
                }
                // The adopt-confirmation marker (pinned shape): a schema-gated,
                // register-only adoption indexed the doc's edges + recorded its
                // file-state baseline. Only an adopted row carries it.
                if row.adopted {
                    out.push_str("  (adopted — indexed + baselined, no file moved)");
                }
                out.push('\n');
                // The adopt-time triage annotations (M40 F4): one indented line per
                // pinned-shape annotation ("adopted — structurally empty: 0 <items>" /
                // "adopted — N surplus trailing sections") — fixed-advisory visibility,
                // never a verdict flip; the same strings ride the JSON row.
                for annotation in &row.annotations {
                    out.push_str("  ");
                    out.push_str(annotation);
                    out.push('\n');
                }
                if let Some(finding) = &row.finding {
                    out.push_str("  ");
                    out.push_str(&finding_line(finding, false));
                }
            }
            // The collapsed unmanaged summary — one line per directory, in sorted
            // directory order (deterministic, independent of row-encounter order).
            // A8 (M43): the line blesses staying plain as a legitimate end-state —
            // `unmanaged` describes, it never implies everything must migrate
            // (`design/surface-contract.md` → the style guide, the verdict-words
            // bullet).
            for (dir, count) in &unmanaged_by_dir {
                out.push_str("unmanaged ");
                out.push_str(dir);
                out.push_str(&format!(
                    " — {count} file(s) parse against no schema (left untouched — fine to stay plain)\n"
                ));
            }
            // The verdict legend (#9c — less-terse triage): one line per verdict class
            // **actually present**, each stating why a row classified that way and the
            // next action. Keyed to the rows (never a static menu that claims an absent
            // class), in a fixed display order, so a report with one class names only
            // that one and an empty report renders no legend at all (inert).
            if !report.rows.is_empty() {
                out.push_str("\nWhat the verdicts above mean, and what to do next:\n");
                for (verdict, gloss) in VERDICT_LEGEND {
                    if report.rows.iter().any(|row| row.verdict == *verdict) {
                        out.push_str("  ");
                        out.push_str(verdict);
                        out.push_str(" — ");
                        out.push_str(gloss);
                        out.push('\n');
                    }
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render the active-task roster `jigc task list` enumerates to the surface `format`
/// selects (M26 shakedown: the in-tool way to find a live task id). `json` emits the
/// rows as a structured array (tooling-consumed, no footer); `agent` / `human` emit a
/// header, one line per task (`<id>  [<workflow>]  <intent>`), and the routing footer.
/// The empty roster renders a clean "no active tasks" line (exit 0), never an error.
pub fn task_list(format: Format, rows: &[TaskListRow]) -> String {
    match format {
        Format::Json => json(&rows),
        Format::Agent | Format::Human => {
            let mut out = if rows.is_empty() {
                "jigc task list — no active tasks\n".to_string()
            } else {
                let mut s = format!("jigc task list — {} active task(s)\n\n", rows.len());
                for row in rows {
                    s.push_str("  ");
                    s.push_str(&row.id);
                    if let Some(workflow) = &row.workflow {
                        s.push_str("  [");
                        s.push_str(workflow);
                        s.push(']');
                    }
                    if !row.intent.is_empty() {
                        s.push_str("  ");
                        s.push_str(&row.intent);
                    }
                    s.push('\n');
                }
                s
            };
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The verdict legend (`design/auto-migration.md` → Hardening #9c): the why + next
/// action for each ingest verdict class, in fixed display order. Only the entries
/// whose verdict is present in a report render, so the legend never claims a class
/// the scan didn't produce.
const VERDICT_LEGEND: &[(&str, &str)] = &[
    (
        "adoptable",
        "conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).",
    ),
    (
        "needs-reconcile",
        "parses as the named type but conflicts; fix it per the row's route, then re-run `jigc ingest`.",
    ),
    (
        "unmanaged",
        "matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.",
    ),
];

/// Render a `jigc unmanage <path>` outcome to the surface `format` selects (M21
/// Increment 4 / T1; `design/project-setup.md` → Flow 2 hardening → Teardown / cleanup
/// (G5)). `json` emits the report object (tooling-consumed, no footer); `agent` /
/// `human` emit one summary line distinguishing a real drop (the doc's edges +
/// baseline were dropped, the file left on disk) from an idempotent no-op (the doc was
/// already unmanaged), then the routing footer.
pub fn unmanage(format: Format, report: &crate::unmanage::UnmanageReport) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = if report.dropped {
                match &report.identity {
                    // The still-at-the-managed-home honesty clause (round-2 D2): the
                    // relocation sweeps (a `docs-root` re-point, `jigc relocate`) walk
                    // committed truth under the home — deliberately index-blind, so a
                    // fresh clone still relocates — which means an unmanaged file left
                    // at the home is still carried by home-wide ops. Say so at the one
                    // moment the operator makes that state.
                    Some(id) => format!(
                        "unmanaged {} ({}) — dropped its file-state baseline + forward edges; the file is left on disk. It still sits at the managed home, so home-wide ops (e.g. a `docs-root` re-point, which relocates every committed doc under the old home — managed or not) still carry it; move it out of the managed location to fully detach it\n",
                        report.path, id,
                    ),
                    None => format!(
                        "unmanaged {} — dropped its file-state baseline; the file is left on disk\n",
                        report.path,
                    ),
                }
            } else {
                format!("no-op: {} is not managed (nothing to drop)\n", report.path,)
            };
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc rename` outcome to the surface `format` selects: `agent` / `human` emit
/// the move summary (old→new identity + path, the count of repointed referrers, each
/// listed), followed by the routing footer; `json` emits the generic projection (no
/// footer). The verb owns the structural rewrite + commit; the CLI only formats the report.
pub fn rename(format: Format, report: &crate::rename::RenameReport) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = format!(
                "renamed {} -> {} ({} -> {}), repointed {} referrer(s)\n",
                report.from,
                report.to,
                report.old_path,
                report.new_path,
                report.referrers.len(),
            );
            for referrer in &report.referrers {
                out.push_str(&format!("  repointed {referrer}\n"));
            }
            if !report.prose_mentions.is_empty() {
                out.push_str(&format!(
                    "{} prose/unmanaged mention(s) of `{}` remain — advisory, not rewritten \
                     (the CLI authors no prose; fix these by hand):\n",
                    report.prose_mentions.len(),
                    report.from,
                ));
                for mention in &report.prose_mentions {
                    out.push_str(&format!("  {mention}\n"));
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc migrate-corpus` outcome to the surface `format` selects: `agent` /
/// `human` emit a per-doc summary — one `migrated`/`already current` line per doc plus, for
/// each blocked doc, its Framing-A route — then the **landed commit** (the verb commits its
/// own migration; absent when nothing was committed) and the routing footer; `json` emits the
/// generic projection of the report (tooling-consumed, no footer), whose `commit` field
/// carries the same sha (`null` when nothing was committed). The verb writes the migrated
/// docs; the CLI only formats the report the migration returns (the determinism boundary —
/// the structural transform is CLI-owned, no LLM).
pub fn corpus_migration(
    format: Format,
    report: &crate::migrate_corpus::CorpusMigrationReport,
) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            // A `--dry-run` suppressed the write, so it must not speak in the past tense: the
            // header says what the run IS (nothing written) and every migrated path is framed
            // `would migrate` — byte-distinct from an applying run, which the pre-F2 renderer
            // was not (M43 surface census, F2 — Law 1 "acks state the effect").
            let mut out = if report.dry_run {
                format!(
                    "corpus migration (dry run — nothing written): {} would migrate, {} already current, {} blocked\n",
                    report.migrated.len(),
                    report.already_current.len(),
                    report.blocked.len(),
                )
            } else {
                format!(
                    "corpus migration: {} migrated, {} already current, {} blocked\n",
                    report.migrated.len(),
                    report.already_current.len(),
                    report.blocked.len(),
                )
            };
            for path in &report.migrated {
                if report.dry_run {
                    out.push_str(&format!("  would migrate {path}\n"));
                } else {
                    out.push_str(&format!("  migrated   {path}\n"));
                }
            }
            for path in &report.already_current {
                out.push_str(&format!("  current    {path}\n"));
            }
            // A refused doc is a real `Finding` (M42 completion audit, Finding 2): it names its
            // stable target (the doc's path), the `migrate-corpus.*` code a driver keys on, the
            // diagnosis, and — separately — the route. The two halves print on their own lines:
            // fusing them is what let the route carry a diagnosis and say nothing actionable.
            for finding in &report.blocked {
                let path = finding
                    .location
                    .as_ref()
                    .and_then(|l| l.address.as_deref())
                    .unwrap_or("<unaddressed>");
                out.push_str(&format!("  blocked    {path}\n"));
                out.push_str(&format!("    {}: {}\n", finding.code, finding.message));
                if let Some(route) = &finding.route {
                    out.push_str(&format!("    route: {route}\n"));
                }
            }
            // The commit-status line — the three run modes each say what they did with the
            // writes (F2: all three honestly distinguishable). A dry run wrote nothing and
            // names the real run that would land it; an applying run either committed (naming
            // the sha) or, under `--no-commit`, left the writes on disk unstaged and says so.
            if report.dry_run {
                if !report.migrated.is_empty() {
                    out.push_str(
                        "nothing was written — re-run without `--dry-run` to apply the migration\n",
                    );
                }
            } else if let Some(sha) = &report.commit {
                // The commit boundary (`design/corpus-migration.md` → The commit boundary): the
                // verb lands its own migration, so the report names *where* it landed.
                out.push_str(&format!(
                    "committed {sha} — only the migrated paths were staged\n"
                ));
            } else if report.no_commit && !report.migrated.is_empty() {
                out.push_str(
                    "written but NOT committed (`--no-commit`) — the migrated paths are on disk \
                     and unstaged; stage and commit them yourself\n",
                );
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc relocate <type> --from <prior>` outcome (the freeze-exempt relocation
/// path) to the surface `format` selects: `agent` / `human` emit one `moved from -> to` line
/// per relocated instance, one `displaced` line per foreign squatter moved into the workbench,
/// and each blocked doc's reason, followed by the routing footer; `json` emits the generic
/// projection of the report (tooling-consumed, no footer). The CLI only formats the report the
/// relocation returns — the move itself is the deterministic T1 primitive
/// (`design/corpus-migration.md` → Relocation: freeze-exempt).
pub fn freeze_exempt_relocation(
    format: Format,
    report: &crate::relocate::RelocationReport,
) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human => {
            let mut out = format!(
                "freeze-exempt relocation: {} moved, {} displaced, {} blocked\n",
                report.moved.len(),
                report.displaced.len(),
                report.blocked.len(),
            );
            for (from, to) in &report.moved {
                out.push_str(&format!("  moved     {from} -> {to}\n"));
            }
            for (from, to) in &report.displaced {
                out.push_str(&format!(
                    "  displaced {from} -> {to} (foreign squatter → workbench)\n"
                ));
            }
            for (path, reason) in &report.blocked {
                out.push_str(&format!("  blocked   {path}\n    {reason}\n"));
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a successful `jigc milestone <verb>` action to the surface `format`
/// selects: `agent` / `human` emit the action summary line (e.g. `minted
/// milestone:<id> …`) followed by the routing footer; `json` emits a generic
/// object carrying the summary text, with no footer (tooling-consumed). The
/// determinism boundary is unaffected — the engine mints; the CLI only formats the
/// summary it returns (`design/write-commands.md` → Minting a milestone).
pub fn milestone(format: Format, summary: &str) -> String {
    match format {
        Format::Json => json(&serde_json::json!({ "text": summary })),
        Format::Agent | Format::Human => {
            let mut out = String::from(summary);
            out.push('\n');
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a **successful** `jigc milestone join` outcome to the surface `format`
/// selects: `agent` / `human` emit the merged-overlay summary (one `  - <address>`
/// line per merged doc, id-sorted, naming each doc's provenance + contributing
/// sub-task, and — for a collision-suffixed instance — the `← suffixed -N on
/// collision` decision plus `; self-ref rewritten` when its own reference was
/// rewritten in lockstep), then — C3 (round-2 surface fixes) — a `no docs staged
/// from:` line naming every sub-task in `sub_tasks` (the milestone's full id-sorted
/// list) that contributed NO merged doc, so the ack states its effect fully instead
/// of silently crediting a no-work sub-task; followed by the routing footer. The line
/// says "no docs" deliberately — the join merges docs only, and a sub-task may still
/// carry staged worktree code the finalize folds. `json` emits the **generic**
/// projection of the [`JoinOutcome`], with no footer (tooling-consumed).
///
/// The suffix decision is read straight off the merged overlay (a pure function of
/// it, like the merge itself): an entry is a collision suffix iff its address ends
/// `-<N>` (N ≥ 2) **and** the de-suffixed base address is also in the overlay (the
/// bare instance the lower task id kept); the self-ref rewrite is named iff that
/// suffixed instance carries an edge back to its own (suffixed) address. The verb
/// commits nothing — wiring the suffix-resolved overlay into `finalize` is a later
/// increment (`design/storage.md` → The by-task-id join; `design/worked-examples.md`
/// → flow 9).
pub fn milestone_join(
    format: Format,
    milestone_id: &str,
    outcome: &JoinOutcome,
    sub_tasks: &[String],
) -> String {
    match format {
        Format::Json => json(outcome),
        Format::Agent | Format::Human => {
            let mut out = format!(
                "joined milestone:{milestone_id} — {} doc(s) merged\n",
                outcome.overlay.len(),
            );
            for (address, doc) in &outcome.overlay {
                out.push_str("  - ");
                out.push_str(address);
                out.push_str("  (");
                out.push_str(provenance_label(doc.provenance));
                out.push_str(" · from ");
                out.push_str(&doc.source_task);
                out.push(')');
                if let Some(n) = suffix_of(address, &outcome.overlay) {
                    out.push_str(&format!("  ← suffixed -{n} on collision"));
                    if doc.edges.iter().any(|e| &e.to == address) {
                        out.push_str("; self-ref rewritten");
                    }
                }
                out.push('\n');
            }
            // The doc-less sub-tasks, in the caller's (id-sorted) order — named, never
            // silently credited by omission.
            let contributed: std::collections::BTreeSet<&str> = outcome
                .overlay
                .values()
                .map(|doc| doc.source_task.as_str())
                .collect();
            let absent: Vec<&str> = sub_tasks
                .iter()
                .map(String::as_str)
                .filter(|id| !contributed.contains(id))
                .collect();
            if !absent.is_empty() {
                out.push_str("  no docs staged from: ");
                out.push_str(&absent.join(", "));
                out.push('\n');
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The agent-text label for a merged doc's provenance.
fn provenance_label(provenance: engine::state::Provenance) -> &'static str {
    match provenance {
        engine::state::Provenance::Created => "created",
        engine::state::Provenance::EditedFromBase => "edited-from-base",
    }
}

/// One sub-task's contribution to a landed milestone boundary (C2, round-2 surface
/// fixes): its merged-doc count (the materialize's address→source map) and its
/// worktree's staged-code file count — both computed PRE-commit, while the fan-out
/// worktrees are still provisioned. `docs: 0, code_files: 0` is the visible no-work
/// sub-task ("nothing staged"), never a silent credit.
#[derive(Serialize)]
pub struct SubTaskContribution {
    /// The sub-task's work-unit id.
    pub id: String,
    /// Merged docs this sub-task's area contributed (transient commit doc included).
    pub docs: usize,
    /// Staged code files in this sub-task's fan-out worktree (`git diff --cached
    /// --name-only`); 0 for a never-provisioned or code-less sub-task.
    pub code_files: usize,
}

/// The landed-boundary facts a successful `jigc milestone finalize` confirms back
/// (C2, round-2 surface fixes — the highest-stakes commit boundary previously
/// succeeded with EMPTY stdout, driving the reader around the tool to raw `git
/// show`): the `jigc task finalize` mold's short `hash` + `subject` (the synthesized
/// milestone message's first line), the whole boundary's landed-file `manifest`
/// (`git diff --name-status <pre-boundary-HEAD> HEAD`, so the `squash: false` N+1
/// chain reads as one set), its `files` count, and the per-sub-task contribution
/// line. Serializes as the `committed` object of the JSON arm — one struct feeds
/// both arms, so the text/JSON sets are identical by construction.
#[derive(Serialize)]
pub struct MilestoneLanded {
    /// The landed boundary's abbreviated HEAD hash.
    pub hash: String,
    /// The landed HEAD subject (the synthesized `Finalize milestone <id> (…)` line).
    pub subject: String,
    /// The number of files the boundary landed (`= manifest.len()`).
    pub files: usize,
    /// Every path the boundary landed, tagged by how it entered ([`ManifestKind`]:
    /// promoted docs, added/modified/deleted code from the combined worktrees).
    pub manifest: Vec<ManifestEntry>,
    /// Each sub-task's contribution, id-sorted — the no-work one visible.
    pub sub_tasks: Vec<SubTaskContribution>,
}

/// Render a **landed** `jigc milestone finalize` to the surface `format` selects
/// (surfacing, never blocking — the M42 print posture): `agent` / `human` emit
/// `finalized <hash> — <subject>`, one [`manifest_line`] per landed path, the
/// `  <n> file(s) committed` tally, and the `  sub-tasks:` contribution line
/// (`<id>: 1 doc, 1 code file · <id>: nothing staged`), followed by the routing
/// footer; `json` emits `{"committed": {…}}` — the same [`MilestoneLanded`]
/// projection, no footer (tooling-consumed).
pub fn milestone_finalized(format: Format, landed: &MilestoneLanded) -> String {
    match format {
        Format::Json => json(&serde_json::json!({ "committed": landed })),
        Format::Agent | Format::Human => {
            let mut out = format!("finalized {} — {}\n", landed.hash, landed.subject);
            for entry in &landed.manifest {
                out.push_str(&manifest_line(entry));
                out.push('\n');
            }
            let noun = if landed.files == 1 { "file" } else { "files" };
            out.push_str(&format!("  {} {noun} committed\n", landed.files));
            if !landed.sub_tasks.is_empty() {
                let parts: Vec<String> = landed.sub_tasks.iter().map(contribution_label).collect();
                out.push_str(&format!("  sub-tasks: {}\n", parts.join(" · ")));
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// One sub-task's agent-text contribution label: `<id>: 1 doc, 2 code files`,
/// either half omitted at zero, and the fully-empty case named `nothing staged`.
fn contribution_label(contribution: &SubTaskContribution) -> String {
    if contribution.docs == 0 && contribution.code_files == 0 {
        return format!("{}: nothing staged", contribution.id);
    }
    let mut parts = Vec::new();
    if contribution.docs > 0 {
        let noun = if contribution.docs == 1 {
            "doc"
        } else {
            "docs"
        };
        parts.push(format!("{} {noun}", contribution.docs));
    }
    if contribution.code_files > 0 {
        let noun = if contribution.code_files == 1 {
            "code file"
        } else {
            "code files"
        };
        parts.push(format!("{} {noun}", contribution.code_files));
    }
    format!("{}: {}", contribution.id, parts.join(", "))
}

/// The collision-suffix index `N` (≥ 2) of `address`, iff it ends `-<N>` and the
/// de-suffixed base address is also present in the merged `overlay` — i.e. the bare
/// instance the lower task id kept. Returns `None` for a non-suffixed (disjoint)
/// address, so a plain doc whose slug merely ends in a number is never mislabeled.
fn suffix_of(
    address: &str,
    overlay: &std::collections::BTreeMap<String, engine::milestone::MergedDoc>,
) -> Option<usize> {
    let (base, num) = address.rsplit_once('-')?;
    let n: usize = num.parse().ok()?;
    if n >= 2 && overlay.contains_key(base) {
        Some(n)
    } else {
        None
    }
}

/// Render a **migration review gate** block (`design/auto-migration.md` → The review
/// gate): a finalize over a migration task that has not been `--approve`d. `agent` /
/// `human` emit the fidelity diff — both inputs visible — so the human can judge whether
/// the agent's rewrite faithfully preserved the foreign content (the strict parse
/// guarantees *structure*, never *content-faithfulness*; the human is its only check),
/// followed by the routing footer; `json` emits the generic projection (both inputs +
/// the destinations), no footer (tooling-consumed). The diff format is an elaboration —
/// the contract is that both the foreign source and each canonical rewrite are visible.
/// Each rewrite block prefixes the foreign lines `-` and the canonical lines `+`: a
/// migration is a wholesale rewrite, so the honest framing is that the human reviews the
/// whole of both sides. The command commits nothing (exit 4); the agent never approves
/// (a human-only gate).
pub fn migration_review(
    format: Format,
    task_id: &str,
    foreign: &str,
    rewrites: &[(String, String)],
) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "review": "pending",
            "task": task_id,
            "source": foreign,
            "rewrites": rewrites
                .iter()
                .map(|(destination, rendered)| {
                    serde_json::json!({ "destination": destination, "rendered": rendered })
                })
                .collect::<Vec<_>>(),
        })),
        Format::Agent | Format::Human => {
            let mut out = format!(
                "migration review required — nothing committed. Re-run \
                 `jigc task finalize {task_id} --approve` to write the canonical doc, \
                 retire the foreign original, and commit.\n\nThe rewrite is the agent's; \
                 the CLI guarantees structure, never content-faithfulness — review the \
                 fidelity diff below, then approve.\n\n",
            );
            // The structural fidelity summary (`design/auto-migration.md` → Hardening #5):
            // a release-level delta naming the source releases the rewrite dropped, so a
            // reviewer needn't eyeball that N of M releases survived. Both sides are
            // HEURISTIC version-scans over the whole text (the M40 calibration fix — a
            // heading-only kept-set false-alarmed non-changelog doctypes). Negative guard
            // (DECISIONS C4, Framing A): display-only — labeled fuzzy, feeds no gate, no
            // agent logic, no structural decision; never a second structural authority.
            let dropped = dropped_release_versions(foreign, rewrites);
            // Always render the line — the affirmative `(none)` form on the nothing-dropped
            // happy path (matching `design/worked-examples.md` flow 26) gives the reviewer a
            // trustworthy positive signal that the scan ran and found nothing missing; an
            // absent line is ambiguous. Display-only either way (DECISIONS C4, Framing A).
            let absent = if dropped.is_empty() {
                "(none)".to_string()
            } else {
                dropped.join(", ")
            };
            out.push_str(&format!(
                "fidelity (heuristic version-scan — fuzzy, advisory; feeds no gate, no \
                 structural decision): source releases absent from the rewrite: {absent}\n\n",
            ));
            for (destination, rendered) in rewrites {
                out.push_str("--- foreign source (staged seam)\n");
                for line in foreign.lines() {
                    out.push_str("- ");
                    out.push_str(line);
                    out.push('\n');
                }
                out.push_str(&format!("+++ canonical rewrite → {destination}\n"));
                for line in rendered.lines() {
                    out.push_str("+ ");
                    out.push_str(line);
                    out.push('\n');
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The heuristic release-delta for the fidelity summary (`design/auto-migration.md` →
/// Hardening #5): the version-like tokens scanned out of the `foreign` source that are
/// absent from the **whole** `rewrites` text, sorted + de-duplicated for a stable
/// display. Fuzzy by construction (both sides are heuristic scans, so the delta can
/// miss or invent a release); the result is **display-only** and feeds no structural
/// decision (DECISIONS C4, Framing A).
fn dropped_release_versions(foreign: &str, rewrites: &[(String, String)]) -> Vec<String> {
    // The kept-set scans the WHOLE rewrite text (the M40 calibration fix,
    // `design/auto-migration.md` → Hardening #5): a heading-only scan was a
    // changelog-shaped assumption (there, releases *are* `### …` headings) that
    // false-alarmed on every other doctype — a version token kept in body prose
    // still read as "dropped". A kept token anywhere in the rewrite is kept.
    let mut kept: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (_destination, rendered) in rewrites {
        kept.extend(scan_version_tokens(rendered));
    }
    let mut dropped: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for token in scan_version_tokens(foreign) {
        if !kept.contains(&token) {
            dropped.insert(token);
        }
    }
    dropped.into_iter().collect()
}

/// Every maximal dotted-numeric run in `text` (e.g. `1.0.0`, `0.9`) — a deliberately
/// fuzzy version-token scan: at least one `.` with a digit on each side, no leading or
/// doubled dot; a trailing dot is **trimmed**, not rejected (the M40 calibration fix:
/// a sentence-final `since 1.5.` yields `1.5`, no longer an under-report). Dash-separated
/// dates (`2021-06-01`) carry no `.` and so never match. Heuristic only — see
/// [`dropped_release_versions`].
fn scan_version_tokens(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let token = text[start..i].trim_end_matches('.');
            if token.contains('.') && !token.starts_with('.') && !token.contains("..") {
                tokens.push(token.to_string());
            }
        } else {
            i += 1;
        }
    }
    tokens
}

/// Render an **operational error** (an orchestration/`anyhow` failure — not a
/// validation outcome) to the surface `format` selects: `json` emits the single-key
/// envelope `{"error": "<anyhow chain>"}` (so a tooling consumer on `--format json`
/// gets a parseable error, never plain text — `design/measurement.md` → The capture
/// substrate, item 2); `agent` / `human` emit the `{err:#}` chain byte-identical to
/// the historic `eprintln!("{err:#}")` funnels this replaces (no routing footer — an
/// error is not a composed reading surface). The exit code (1, never 3) stays the
/// dispatcher's concern; every format-bearing error funnel (`task` / `cli` /
/// `milestone` dispatch arms) routes through here, emitted on **stderr**.
pub fn operational_error(format: Format, err: &anyhow::Error) -> String {
    match format {
        Format::Json => json(&serde_json::json!({ "error": format!("{err:#}") })),
        Format::Agent | Format::Human => format!("{err:#}"),
    }
}

/// Render the **unset project** orientation (`design/bootstrap.md` → Orientation
/// output examples, state 1): no project layer is set up, so route the agent to
/// `jigc setup` and end with the universal routing footer.
pub fn orientation_unset() -> String {
    let mut out = String::from("jigc — orientation\n\n");
    out.push_str(
        "This project isn't set up. No project config layer is present; the cascade has only pack defaults.\n\n",
    );
    out.push_str("Run: `jigc setup`\n\n");
    out.push_str(
        "`jigc setup` installs jigc into this project (the adapter, the `jigc` allowlist, the `.jigc/config/` layer). Then `jigc start` orients you to the setup workflows — `project-setup` (develop a new project's idea into its first requirements) or `ingest-existing` (bring an existing repo's docs under management).\n",
    );
    out.push_str(ROUTING_FOOTER);
    out
}

/// Render a [`Description`] whole-menu projection to the surface `format` selects.
///
/// **Free-prose renderer** — net-new, distinct from every other (line-structured)
/// arm. `agent` / `human` emit the projection as **discursive prose paragraphs**:
/// **one blank-line-separated paragraph per narrated definition**, under light
/// *unkeyed* section transitions ("The workflows you can compose here …", "The
/// doc-types you can author …", "And the commands jigc gives you …") that lead their
/// group's first paragraph, the routing footer last. The shape is deliberately
/// **hostile to parsing** (the non-contractual format contract — `introspection.md`
/// → Non-contractual by design): no key-shaped lines, no bullet rows, no
/// per-definition extractable handle — a blank line is none of those, it is only a
/// reading aid, so the paragraph break keeps the posture while sparing the reader the
/// wall a whole-group `join(" ")` printed. describe is a menu, not an API. The
/// command hints stay one paragraph: they are short one-line sentences, and breaking
/// each onto its own line would edge the surface toward the `id → hint` table the
/// format contract forbids. `json` still routes through the generic serde renderer,
/// but that projection
/// is **not** the surface this command's contract is about (describe's whole point is
/// not to be JSON-shaped — `introspection.md` → Command surface); it carries no
/// footer (tooling-consumed).
///
/// The engine has already woven each definition into a full sentence (`X is …. Reach
/// for it when ….`) and projected each command-ref `hint` verbatim; this renderer
/// only frames those sentences into paragraphs and appends the footer — it composes
/// no prose of its own beyond the unkeyed transitions.
pub fn describe(format: Format, description: &Description) -> String {
    match format {
        Format::Json => json(description),
        Format::Agent | Format::Human => {
            let mut out = String::from(
                "jigc describe — a tour of what this project lets you compose and author.\n\n",
            );

            let workflows: Vec<&str> = description
                .definitions
                .iter()
                .filter(|d| d.kind == DefinitionKind::Workflow)
                .map(|d| d.prose.as_str())
                .collect();
            let doctypes: Vec<&str> = description
                .definitions
                .iter()
                .filter(|d| d.kind == DefinitionKind::Doctype)
                .map(|d| d.prose.as_str())
                .collect();

            if !workflows.is_empty() {
                out.push_str("The workflows you can compose here. ");
                out.push_str(&workflows.join("\n\n"));
                out.push_str("\n\n");
            }
            if !doctypes.is_empty() {
                out.push_str("The doc-types you can author. ");
                out.push_str(&doctypes.join("\n\n"));
                out.push_str("\n\n");
            }
            if !description.commands.is_empty() {
                out.push_str("And the commands jigc hands you along the way. ");
                let sentences: Vec<String> = description
                    .commands
                    .iter()
                    .map(|c| format!("{} {}", c.id, c.hint))
                    .collect();
                out.push_str(&sentences.join(" "));
                out.push_str("\n\n");
            }

            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::compose::ComposedWorkflow;
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

    /// The membership test **through the write-ack surface** (`command-output-contract.md` →
    /// The membership test): a `DocAck`'s `findings[]` projects the pinned envelope, so an
    /// address-less non-exempt finding riding a write ack would ship `key.target: null`.
    /// [`doc_ack`] fires on it — and since the fix it fires **without this surface knowing the
    /// seam exists**: the check rides `Finding`'s own `Serialize`, so it cannot be bypassed and
    /// no funnel has to remember to call it (the census, retired).
    #[test]
    #[should_panic(expected = "is serialized with no `key.target`")]
    fn the_ack_seam_fires_on_an_address_less_finding() {
        let degenerate = Finding::graded(
            engine::finding::Severity::Advisory,
            "schema-conformance.surplus-sections-absent",
            "a surplus section is present",
            Some(engine::finding::Location::at(9, 1)),
            Some("remove the surplus section".into()),
        );

        let _ = doc_ack(
            Format::Json,
            &DocAck::Slot {
                address: "adr:use-rust#decision".to_owned(),
                target: AckTarget {
                    doctype: "adr".to_owned(),
                    slug: "use-rust".to_owned(),
                    section: Some("decision".to_owned()),
                    item: None,
                    leaf: None,
                },
                chars: 12,
                findings: vec![degenerate].into(),
            },
        );
    }

    /// The membership test **through the bare-`Finding` surface** (`setup_block`). Every
    /// `setup.*` code is a **declared singleton** (fail-fast `Result<_, Finding>`: at most
    /// one per invocation), so it passes the check with `target: null` — by the pin, not by
    /// omission (`command-output-contract.md` → The declared singleton exception).
    #[test]
    fn the_setup_block_seam_passes_the_declared_singleton() {
        let singleton = Finding::block(
            "setup.repo-root",
            "`jigc setup` must run inside a git repository",
            "run `git init` first",
        );

        let json = setup_block(Format::Json, &singleton);

        let value: serde_json::Value = serde_json::from_str(&json).expect("parses");
        assert_eq!(value["key"]["code"], "setup.repo-root");
        assert_eq!(
            value["key"]["target"],
            serde_json::Value::Null,
            "a declared singleton keys at null and passes the seam",
        );
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
    /// next-step directive, and ends with the routing footer. The directive
    /// claims no CLI selection work (`surface-contract.md` law 1 — behaviour
    /// claims match knobs): `jigc start "<intent>"` *presents* the catalog, the
    /// *agent* picks, and `--workflow <chosen>` composes — never "routes among".
    #[test]
    fn render_orientation_clean_has_header_catalog_and_footer() {
        // No off-catalog verbs present → no extra route-prose line (the omitting
        // context stays inert, byte-identical to before the G6 line).
        let text = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &fixture(),
            &[],
        );

        insta::assert_snapshot!(text, @r#"
        jigc — orientation

        Pack: dev/v0.3.0 · Project config: .jigc/config

        Available workflows:
          - single-task — Implement one well-scoped change.
          - project-setup — Set up the development pack on a fresh repo.

        Run: `jigc start "<intent>"`   — presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        "#);

        // The next-step tells the truth: the CLI presents, the agent picks.
        assert!(
            text.contains("presents the workflows above"),
            "got:\n{text}",
        );
        assert!(
            !text.contains("routes among"),
            "orientation must not claim the CLI does selection work; got:\n{text}",
        );
        assert!(
            !text.contains("default workflow (single-task)"),
            "post-flip orientation must not claim a direct single-task mint; got:\n{text}",
        );
        // No off-catalog next-step line when none is present.
        assert!(
            !text.contains("--workflow planning") && !text.contains("--workflow ingest-existing"),
            "the omitting context names no off-catalog verb; got:\n{text}",
        );
        assert!(text.ends_with(ROUTING_FOOTER));
    }

    /// Off-catalog discoverability (G6): each present off-catalog verb renders one
    /// `Run: jigc start --workflow <id>   — <gist>` route-prose line (same shape for
    /// each), after the catalog's routing directive and before the footer — so a
    /// bare-`start` reader finds `planning` + `ingest-existing` though neither is in
    /// the `Available workflows:` catalog (`design/project-setup.md` → Off-catalog
    /// discoverability (G6)).
    #[test]
    fn render_orientation_clean_names_off_catalog_next_step_verbs() {
        let text = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &fixture(),
            &[
                NextStep::new("planning", "plan a milestone"),
                NextStep::new("ingest-existing", "bring an existing repo under management"),
            ],
        );

        assert!(
            text.contains("Run: `jigc start --workflow planning`   — plan a milestone\n"),
            "the `planning` verb renders a route-prose line; got:\n{text}",
        );
        assert!(
            text.contains(
                "Run: `jigc start --workflow ingest-existing`   — bring an existing repo under management\n"
            ),
            "the `ingest-existing` verb renders a route-prose line; got:\n{text}",
        );
        // The off-catalog lines follow the catalog's routing directive.
        let routing_at = text
            .find("presents the workflows above")
            .expect("routing directive present");
        let planning_at = text
            .find("--workflow planning")
            .expect("planning line present");
        assert!(
            planning_at > routing_at,
            "the off-catalog lines follow the catalog routing directive; got:\n{text}",
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

        This project isn't set up. No project config layer is present; the cascade has only pack defaults.

        Run: `jigc setup`

        `jigc setup` installs jigc into this project (the adapter, the `jigc` allowlist, the `.jigc/config/` layer). Then `jigc start` orients you to the setup workflows — `project-setup` (develop a new project's idea into its first requirements) or `ingest-existing` (bring an existing repo's docs under management).
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        assert!(text.contains("project-setup"), "got:\n{text}");
        assert!(text.contains("ingest-existing"), "got:\n{text}");
        assert!(
            !text.contains("domain pack is installed"),
            "stale over-promise copy; got:\n{text}",
        );
        assert!(
            !text.contains("walks the pack choice"),
            "stale over-promise copy; got:\n{text}",
        );
        assert!(text.ends_with(ROUTING_FOOTER));
    }

    /// A composed workflow rendered to agent-text carries the `create-gates:` line then
    /// the routing footer (`design/workflow-dialect.md` → Routing footer), both appended
    /// in the frontend after the engine's footerless four-class text. JSON carries
    /// neither, and stays the pinned `{task, text}` projection of the composed view.
    #[test]
    fn render_composed_agent_text_names_the_gates_then_the_footer() {
        let view = ComposedWorkflow {
            task: Some("add-rate-limiter".to_string()),
            text: "Reason about the change.\nRun: `jigc task finalize add-rate-limiter`\n"
                .to_string(),
        };
        let granting = Composition {
            view: view.clone(),
            gates: vec!["adr".to_string(), "changelog".to_string()],
            minted: true,
        };

        let agent = composed(Format::Agent, &granting);
        assert!(agent.ends_with(ROUTING_FOOTER));
        assert!(agent.contains("Run: `jigc task finalize add-rate-limiter`"));
        assert!(
            agent.contains("create-gates: adr, changelog\n— jigc"),
            "the gates line sits immediately before the footer; got:\n{agent}",
        );

        // Human renders identically to agent in the MVP (TUI is post-MVP).
        assert_eq!(composed(Format::Human, &granting), agent);

        // A gate-less workflow renders no line at all — omitted, never `none`.
        let gateless = Composition {
            view: view.clone(),
            gates: Vec::new(),
            minted: true,
        };
        let bare = composed(Format::Agent, &gateless);
        assert!(!bare.contains("create-gates"), "got:\n{bare}");
        assert!(bare.ends_with(ROUTING_FOOTER));

        // JSON is the generic projection of the composed view — the pinned `{task, text}`
        // (`design/command-output-contract.md` §1): no footer, and no gates key or line.
        let json_out = composed(Format::Json, &granting);
        assert!(!json_out.contains(ROUTING_FOOTER));
        assert!(!json_out.contains("create-gates"));
        let back: ComposedWorkflow = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, view);
    }

    /// A **work-minting** compose opens agent/human text with the `task minted: <id>`
    /// header (M42) — the id the loop's every subsequent call needs, stated rather than
    /// buried inside a step body's `Run:` string. It is keyed on the *mint*, not on the
    /// id's presence: a **re-compose** (resume / sub-agent re-entry) carries the given id
    /// but minted nothing, and a `creates-task: false` compose has no id at all — both
    /// render no header. The pinned `{task, text}` JSON never carries it.
    #[test]
    fn render_composed_agent_text_announces_only_a_real_mint() {
        let text = "Reason about the change.\n".to_string();
        let minted = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: true,
        };

        let agent = composed(Format::Agent, &minted);
        assert!(
            agent.starts_with("task minted: add-rate-limiter\n\nReason about the change.\n"),
            "the header opens the view, above the composed text; got:\n{agent}",
        );
        assert_eq!(composed(Format::Human, &minted), agent);

        // A re-compose carries the *given* id — announcing a mint here would state one
        // that never happened.
        let resumed = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: false,
        };
        let re = composed(Format::Agent, &resumed);
        assert!(!re.contains("task minted"), "got:\n{re}");
        assert!(re.starts_with("Reason about the change.\n"), "got:\n{re}");

        // The `creates-task: false` (router) arm mints nothing and has no id: inert.
        let router = Composition {
            view: ComposedWorkflow { task: None, text },
            gates: Vec::new(),
            minted: false,
        };
        let routed = composed(Format::Agent, &router);
        assert!(!routed.contains("task minted"), "got:\n{routed}");

        // The pinned contract is untouched — presentation adds no key, and no bytes.
        let json_out = composed(Format::Json, &minted);
        assert!(!json_out.contains("task minted"), "got:\n{json_out}");
        let back: ComposedWorkflow = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back.task.as_deref(), Some("add-rate-limiter"));
        assert!(!back.text.contains("task minted"));
    }

    /// An **id-carrying** compose appends the three task-state lines (M43 Inc 7 /
    /// B3+B4): `resume:`, `what's-left:`, and the `task scope:` B3 statement — keyed on
    /// the id's **presence** (standing affordances of the active-task state), so a
    /// resume carries them too, while the id-less router renders no bytes. They sit
    /// between the composed text and the gates line, and never reach the JSON
    /// projection (the pinned `{task, text}` contract).
    #[test]
    fn render_composed_agent_text_states_the_task_affordances() {
        let text = "Reason about the change.\n".to_string();
        let minted = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: text.clone(),
            },
            gates: vec!["adr".to_string()],
            minted: true,
        };

        let agent = composed(Format::Agent, &minted);
        assert!(
            agent.contains("resume: `jigc start --task add-rate-limiter`"),
            "got:\n{agent}",
        );
        assert!(
            agent.contains("what's-left: `jigc task validate add-rate-limiter`"),
            "got:\n{agent}",
        );
        let scope = agent
            .lines()
            .find(|l| l.starts_with("task scope:"))
            .unwrap_or_else(|| panic!("a task-scope line renders; got:\n{agent}"));
        assert!(
            scope.contains("single active task")
                && scope.contains("--task add-rate-limiter")
                && scope.contains("wins"),
            "the B3 statement: single-active-task default, explicit `--task` wins; \
             got:\n{scope}",
        );
        // Stack order: text, then the task-state lines, then gates, then footer.
        assert!(
            agent.contains(
                "task scope: `jigc doc` writes default to the single active task; \
                 `--task add-rate-limiter` is the explicit override and wins when \
                 several are active\ncreate-gates: adr\n— jigc"
            ),
            "the task-state lines sit above the gates line + footer; got:\n{agent}",
        );
        assert_eq!(composed(Format::Human, &minted), agent);

        // A resume carries the same standing affordances (no mint header).
        let resumed = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: false,
        };
        let re = composed(Format::Agent, &resumed);
        assert!(!re.contains("task minted"), "got:\n{re}");
        assert!(
            re.contains("what's-left: `jigc task validate add-rate-limiter`"),
            "got:\n{re}",
        );

        // The id-less router renders no bytes — the omitting context stays inert.
        let router = Composition {
            view: ComposedWorkflow {
                task: None,
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: false,
        };
        let routed = composed(Format::Agent, &router);
        for needle in ["resume:", "what's-left:", "task scope:"] {
            assert!(!routed.contains(needle), "got:\n{routed}");
        }

        // The pinned `{task, text}` JSON never carries the lines.
        let json_out = composed(Format::Json, &minted);
        assert!(!json_out.contains("resume:"), "got:\n{json_out}");
        assert!(!json_out.contains("task scope:"), "got:\n{json_out}");
        let back: ComposedWorkflow = serde_json::from_str(&json_out).expect("valid JSON");
        assert!(!back.text.contains("what's-left:"));
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
            Vec::new(),
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

        // The applied-override tree shows no rejected-demotion line (none present).
        assert!(
            !agent.contains("rejected demotion:"),
            "an applied-override tree carries no rejected-demotion line; got:\n{agent}",
        );

        // A no-override tree spells the count `none`.
        let clean = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
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

    /// The `--explain` collision-winner line (T5): over a tree carrying one
    /// adjudicated top-level cross-pack collision per id-space (the `default-workflow`
    /// knob; the `commit` doctype), the agent-text body renders **one line per
    /// collision** naming the **winning pack** (its `(pack-id, version)`). A
    /// single-pack tree carries **no** collision winners, so **no** winner line
    /// renders and the output is byte-identical to today (the byte-identity floor;
    /// hardening #5 — the omitting context). JSON omits the empty `collision_winners`
    /// (skip-empty) and projects the populated one.
    #[test]
    fn render_explain_names_collision_winner_per_top_level_collision() {
        use engine::cascade::LayerKind;
        use engine::result::{CollisionWinner, ResolutionTree};

        // A two-pack composition: methodology wins both adjudicated top-level ids.
        let colliding = ResolutionTree::with_collisions(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![
                CollisionWinner {
                    collision: "default-workflow".to_string(),
                    pack_id: "methodology".to_string(),
                    pack_version: "0.1.0".to_string(),
                },
                CollisionWinner {
                    collision: "doctype:commit".to_string(),
                    pack_id: "methodology".to_string(),
                    pack_version: "0.1.0".to_string(),
                },
            ],
        );

        let agent = explain(Format::Agent, &colliding, "methodology/v0.1.0 | dev/v0.0.0");
        // One winner line per adjudicated collision, naming the winning pack.
        assert!(
            agent.contains("collision: default-workflow → won by methodology/0.1.0"),
            "the knob collision must name the winning pack; got:\n{agent}",
        );
        assert!(
            agent.contains("collision: doctype:commit → won by methodology/0.1.0"),
            "the doctype collision must name the winning pack; got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER), "got:\n{agent}");

        // A single-pack tree carries no collision winners → NO winner line renders,
        // byte-identical to the no-collision composition (hardening #5).
        let single = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let single_agent = explain(Format::Agent, &single, "dev/v0.0.0");
        assert!(
            !single_agent.contains("collision:"),
            "a single-pack composition renders NO winner line; got:\n{single_agent}",
        );

        // JSON: the populated tree projects `collision_winners`; the single-pack tree
        // omits it (skip-empty), and both round-trip.
        let json_out = explain(Format::Json, &colliding, "methodology/v0.1.0 | dev/v0.0.0");
        assert!(!json_out.contains(ROUTING_FOOTER));
        assert!(json_out.contains("\"collision\": \"default-workflow\""));
        let back: ResolutionTree = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, colliding);
        let single_json = explain(Format::Json, &single, "dev/v0.0.0");
        assert!(
            !single_json.contains("collision_winners"),
            "an empty collision_winners is omitted; got:\n{single_json}",
        );
    }

    /// The `--explain` `Pack input:` provenance line (T2): over a tree carrying
    /// `pack_inputs` (one entry per composed pack — id/version + resolving path +
    /// blake3 content-hash), the agent-text body renders **one `Pack input:` line per
    /// pack** naming its path + hash, **after** the collision-winner lines (the
    /// inc-2 surface) and highest-precedence first. A single-pack tree (no
    /// `pack_inputs`) renders **no** `Pack input:` line, so its agent-text is
    /// byte-identical to today (hardening #5 — the additive line is omitted in the
    /// omitting context). JSON omits the empty `pack_inputs` (skip-empty) and projects
    /// the populated one (`design/multi-pack.md` → Provenance under N packs;
    /// `design/worked-examples.md` → flow 17 assertion 5).
    #[test]
    fn render_explain_names_pack_input_path_and_hash_per_composed_pack() {
        use engine::cascade::LayerKind;
        use engine::result::{CollisionWinner, PackInput, ResolutionTree};

        // A two-pack composition: both inputs + both collision winners present.
        let mut tree = ResolutionTree::with_collisions(
            "dev-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![CollisionWinner {
                collision: "default-workflow".to_string(),
                pack_id: "methodology".to_string(),
                pack_version: "0.1.0".to_string(),
            }],
        );
        tree.pack_inputs = vec![
            PackInput {
                pack_id: "methodology".to_string(),
                pack_version: "0.1.0".to_string(),
                path: "/abs/packs/methodology".to_string(),
                content_hash: "a3f9deadbeef".to_string(),
            },
            PackInput {
                pack_id: "dev".to_string(),
                pack_version: "0.0.0".to_string(),
                path: "<embedded>".to_string(),
                content_hash: "71c2cafef00d".to_string(),
            },
        ];

        let agent = explain(Format::Agent, &tree, "methodology/v0.1.0 | dev/v0.0.0");

        // One `Pack input:` line per composed pack, naming its path + blake3 hash.
        assert!(
            agent.contains(
                "Pack input: methodology/0.1.0 = /abs/packs/methodology  (blake3 a3f9deadbeef)"
            ),
            "the highest-precedence pack's input line names its path + hash; got:\n{agent}",
        );
        assert!(
            agent.contains("Pack input: dev/0.0.0 = <embedded>  (blake3 71c2cafef00d)"),
            "the embedded base's input line names the `<embedded>` path + its hash; got:\n{agent}",
        );

        // The `Pack input:` lines render AFTER the collision-winner lines (the inc-2
        // surface) — the planner's ordering: provenance follows the collision winners.
        let collision_at = agent
            .find("collision: default-workflow")
            .expect("collision-winner line present");
        let input_at = agent
            .find("Pack input: methodology/0.1.0")
            .expect("pack-input line present");
        assert!(
            input_at > collision_at,
            "the `Pack input:` lines must follow the collision-winner lines; got:\n{agent}",
        );
        // Highest-precedence first.
        let dev_at = agent
            .find("Pack input: dev/0.0.0")
            .expect("dev input line present");
        assert!(
            dev_at > input_at,
            "the pack inputs render highest-precedence first; got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER), "got:\n{agent}");

        // A single-pack tree carries no `pack_inputs` → NO `Pack input:` line, the
        // existing agent-text byte-identical to today (the omitting context).
        let single = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        let single_agent = explain(Format::Agent, &single, "dev/v0.0.0");
        assert!(
            !single_agent.contains("Pack input:"),
            "a single-pack tree renders NO `Pack input:` line; got:\n{single_agent}",
        );

        // JSON: the populated tree projects `pack_inputs`; the single-pack tree omits
        // it (skip-empty), and both round-trip.
        let json_out = explain(Format::Json, &tree, "methodology/v0.1.0 | dev/v0.0.0");
        assert!(json_out.contains("\"path\": \"/abs/packs/methodology\""));
        let back: ResolutionTree = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, tree);
        let single_json = explain(Format::Json, &single, "dev/v0.0.0");
        assert!(
            !single_json.contains("pack_inputs"),
            "an empty pack_inputs is omitted; got:\n{single_json}",
        );
    }

    /// A below-floor `scalar-set` the cascade soft-rejected renders as a
    /// **rejected-demotion** line in the `--explain` agent-text body — carrying the
    /// key, the attempted value, the floor it ranked below, and the source layer —
    /// **distinct** from the applied-override lines (`design/workflow-dialect.md` →
    /// `--explain` output contract; `design/overrides.md` → Soft-rejection). A
    /// rejected demotion is **not** counted in `overrides applied: N` (logged, not
    /// applied). A tree with no rejection shows no such line.
    #[test]
    fn render_explain_shows_a_rejected_demotion_line_distinct_from_applied_overrides() {
        use engine::cascade::LayerKind;
        use engine::result::{RejectedDemotion, ResolutionTree, ScalarOverride};

        let tree = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            // One applied scalar override; the rejected demotion is NOT in the total.
            1,
            vec![ScalarOverride {
                key: "default-workflow".to_string(),
                value: "single-task".to_string(),
                layer: LayerKind::Project,
            }],
            vec![RejectedDemotion {
                key: "validation.workflow-refs.placeholder-resolves.severity".to_string(),
                attempted: "advisory".to_string(),
                floor: "blocking".to_string(),
                layer: LayerKind::Project,
            }],
            Vec::new(),
        );

        let agent = explain(Format::Agent, &tree, "dev/v0.0.0");

        // The rejected-demotion line carries key · attempted · floor · source layer.
        assert!(
            agent.contains(
                "rejected demotion: validation.workflow-refs.placeholder-resolves.severity = advisory below floor blocking    (project)"
            ),
            "the rejected-demotion line must show key/attempted/floor/layer; got:\n{agent}",
        );
        // It is DISTINCT from the applied-override line (the applied scalar still shows).
        assert!(
            agent.contains("default-workflow = single-task    (project)"),
            "the applied scalar override must still render distinctly; got:\n{agent}",
        );
        // The rejection is logged, not applied — the total stays at the one applied override.
        assert!(
            agent.contains("overrides applied: 1"),
            "a rejected demotion is not folded into the applied-override count; got:\n{agent}",
        );

        // A tree with no rejection shows no rejected-demotion line.
        let no_rejection = ResolutionTree::new(
            "single-task",
            LayerKind::PackDefault,
            0,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        assert!(
            !explain(Format::Agent, &no_rejection, "dev/v0.0.0").contains("rejected demotion:"),
            "no rejected demotion → no rejected-demotion line",
        );
    }

    /// A `warning` finding is a **live third tier** (M6): it flows produce → render →
    /// gate as **non-blocking**. A report whose only non-clean finding is
    /// `Severity::Warning` renders a `warning · …` line that is textually DISTINCT
    /// from the `advisory` tier, and `has_blocking()` — the predicate the upgrade gate
    /// maps to exit 0 — is false (`design/validation.md` → `warning` is a live third
    /// tier). This pins the END-TO-END contract the upgrade dispatch leans on: a
    /// warning-only report exits non-blocking, not merely that the render arm exists.
    #[test]
    fn render_warning_finding_is_non_blocking_and_distinct_from_advisory() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let report = ValidationReport::new(
            // The cascade-delta target form (T6): the delta *is* the subject, and its
            // target string is its identity (`command-output-contract.md` → the sixth form).
            vec![Finding::graded(
                Severity::Warning,
                "override-default.target-unchanged",
                "override target `workflow:single-task#implement` changed in the current pack",
                Some(Location::addressed("workflow:single-task#implement", 1, 1)),
                Some("re-review the delta on `workflow:single-task#implement`".into()),
            )],
            &resolved,
        );

        // The gate the upgrade dispatch maps to its exit code: a warning-only report
        // does NOT block, so `run_upgrade` returns `ExitCode::SUCCESS` (exit 0).
        assert!(
            !report.has_blocking(),
            "a warning-only report must be non-blocking (the upgrade gate exits 0); got:\n{:?}",
            report.findings,
        );

        let agent = validation(Format::Agent, &report);
        // The warning tier renders a `warning · …` line — present and DISTINCT from
        // the `advisory` tier (so a demoted finding is visibly a warning, not silent
        // and not mislabeled advisory).
        assert!(
            agent.contains(
                "warning · override-default.target-unchanged — override target `workflow:single-task#implement` changed in the current pack"
            ),
            "the warning finding must render a `warning · code — message` line; got:\n{agent}",
        );
        assert!(
            !agent.contains("advisory · "),
            "the warning tier must be textually distinct from `advisory`; got:\n{agent}",
        );
        // The block-payload envelope: the warning carries its indented `route:` line.
        assert!(
            agent.contains("\n  route: re-review the delta on `workflow:single-task#implement`"),
            "the warning finding must carry its indented `route:` line; got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER), "got:\n{agent}");
    }

    /// A route-less advisory says so explicitly (RC greenfield trial F5: an advisory
    /// the agent can't act on must carry a "no action needed" clause — otherwise it
    /// reads as an open question). A *routed* advisory and a route-less *blocking*
    /// finding stay unchanged: the route IS the action cue for the former, and a
    /// blocking finding is always actionable.
    #[test]
    fn routeless_advisory_carries_the_no_action_cue() {
        use engine::finding::{Finding, Severity};

        let routeless = Finding::graded(
            Severity::Advisory,
            "file-state.baseline-adopt",
            "baseline adopted: `docs/x.md`",
            None,
            None,
        );
        assert_eq!(
            finding_line(&routeless, false),
            "advisory · file-state.baseline-adopt — baseline adopted: `docs/x.md`   (no action needed)\n",
            "a route-less advisory must end with the no-action cue",
        );

        let routed = Finding::graded(
            Severity::Advisory,
            "reconciliation.orphaned-docs",
            "file outside the resolved roots",
            None,
            Some("ingest or move the file".into()),
        );
        assert!(
            !finding_line(&routed, false).contains("no action needed"),
            "a routed advisory's route is its action cue — no suffix",
        );

        let blocking = Finding::graded(
            Severity::Blocking,
            "schema-conformance.required-slot-present",
            "required slot is empty",
            None,
            None,
        );
        assert!(
            !finding_line(&blocking, false).contains("no action needed"),
            "a blocking finding is always actionable — no suffix",
        );
    }

    /// The `jigc milestone join` summary names every merged doc (id-sorted, with
    /// provenance + contributing sub-task) and — for a collision-suffixed `created`
    /// instance — the `← suffixed -2 on collision` decision plus `; self-ref
    /// rewritten` when the suffixed instance references its own (rewritten) address.
    /// A disjoint doc carries no suffix annotation; JSON is the generic projection
    /// with no footer.
    #[test]
    fn render_milestone_join_names_suffix_decision_and_self_ref_rewrite() {
        use engine::index::Edge;
        use engine::milestone::{JoinOutcome, MergedDoc};
        use engine::state::Provenance;
        use std::collections::BTreeMap;

        let mut overlay: BTreeMap<String, MergedDoc> = BTreeMap::new();
        // A disjoint edited-from-base doc (no suffix annotation).
        overlay.insert(
            "adr:eviction-policy".to_string(),
            MergedDoc {
                provenance: Provenance::EditedFromBase,
                source_task: "evict-stale-keys".to_string(),
                edges: Vec::new(),
            },
        );
        // The bare `created` instance the lower task id kept.
        overlay.insert(
            "adr:cache-strategy".to_string(),
            MergedDoc {
                provenance: Provenance::Created,
                source_task: "area-low".to_string(),
                edges: vec![Edge {
                    from: "adr:cache-strategy".to_string(),
                    relation: "supersedes".to_string(),
                    to: "adr:cache-strategy".to_string(),
                }],
            },
        );
        // The suffixed instance, its own self-ref rewritten to the suffixed slug.
        overlay.insert(
            "adr:cache-strategy-2".to_string(),
            MergedDoc {
                provenance: Provenance::Created,
                source_task: "area-zed".to_string(),
                edges: vec![Edge {
                    from: "adr:cache-strategy-2".to_string(),
                    relation: "supersedes".to_string(),
                    to: "adr:cache-strategy-2".to_string(),
                }],
            },
        );
        let outcome = JoinOutcome {
            overlay,
            findings: Vec::new().into(),
        };

        // The milestone's full id-sorted sub-task list — `area-idle` staged nothing, so
        // the ack must name it as doc-less (C3) rather than silently crediting it.
        let sub_tasks: Vec<String> = ["area-idle", "area-low", "area-zed", "evict-stale-keys"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let agent = milestone_join(Format::Agent, "cache-rework", &outcome, &sub_tasks);
        insta::assert_snapshot!(agent, @r"
        joined milestone:cache-rework — 3 doc(s) merged
          - adr:cache-strategy  (created · from area-low)
          - adr:cache-strategy-2  (created · from area-zed)  ← suffixed -2 on collision; self-ref rewritten
          - adr:eviction-policy  (edited-from-base · from evict-stale-keys)
          no docs staged from: area-idle
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        // The disjoint doc carries no suffix annotation.
        assert!(
            !agent.contains("adr:eviction-policy  (edited-from-base · from evict-stale-keys)  ←"),
            "a disjoint doc must not be annotated as suffixed; got:\n{agent}",
        );

        // JSON is the generic projection — round-trips, no footer.
        let json_out = milestone_join(Format::Json, "cache-rework", &outcome, &sub_tasks);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: JoinOutcome = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, outcome);
    }

    /// C2 — the `jigc milestone finalize` landing manifest: `finalized <hash> —
    /// <subject>`, one manifest line per landed path, the file tally, and the
    /// per-sub-task contribution line with the no-work sub-task visible (`nothing
    /// staged`). JSON is the same [`MilestoneLanded`] under a `committed` key — one
    /// struct feeds both arms, so the sets are identical by construction.
    #[test]
    fn render_milestone_finalized_names_manifest_and_contributions() {
        let landed = MilestoneLanded {
            hash: "b546ca8".to_string(),
            subject: "Finalize milestone cache-rework (2 sub-tasks)".to_string(),
            files: 3,
            manifest: vec![
                ManifestEntry {
                    path: "docs/milestone-records/cache-rework.md".to_string(),
                    kind: ManifestKind::Modified,
                },
                ManifestEntry {
                    path: "docs/decisions/eviction-policy.md".to_string(),
                    kind: ManifestKind::Promoted,
                },
                ManifestEntry {
                    path: "lru.py".to_string(),
                    kind: ManifestKind::Added,
                },
            ],
            sub_tasks: vec![
                SubTaskContribution {
                    id: "implement-lru-eviction".to_string(),
                    docs: 1,
                    code_files: 1,
                },
                SubTaskContribution {
                    id: "wire-cache-metrics-into".to_string(),
                    docs: 0,
                    code_files: 0,
                },
            ],
        };

        let agent = milestone_finalized(Format::Agent, &landed);
        insta::assert_snapshot!(agent, @r"
        finalized b546ca8 — Finalize milestone cache-rework (2 sub-tasks)
          modified docs/milestone-records/cache-rework.md
          promoted docs/decisions/eviction-policy.md
          added lru.py
          3 files committed
          sub-tasks: implement-lru-eviction: 1 doc, 1 code file · wire-cache-metrics-into: nothing staged
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));

        // JSON — the same struct under `committed`, no footer.
        let json_out = milestone_finalized(Format::Json, &landed);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["committed"]["hash"], "b546ca8");
        assert_eq!(value["committed"]["files"], 3);
        assert_eq!(value["committed"]["manifest"][2]["kind"], "added");
        assert_eq!(
            value["committed"]["sub_tasks"][1]["id"],
            "wire-cache-metrics-into"
        );
        assert_eq!(value["committed"]["sub_tasks"][1]["docs"], 0);
    }

    /// The `jigc ingest` triage report renders in sorted candidate order: one row
    /// per candidate (`<verdict> <file> → <type>`), an **adopted** `adoptable` row
    /// carrying the adopt-confirmation marker (no finding), a `needs-reconcile` row
    /// carrying its indented routed finding (`blocking · code — message` + the
    /// `route:` line — the OOB-conflict envelope), and an `unmanaged` row naming the
    /// untouched candidate; ends with the routing footer. JSON is the generic
    /// projection with no footer.
    #[test]
    fn render_ingest_lists_rows_with_routed_finding_and_footer() {
        use crate::ingest::{IngestReport, TriageRow};
        use engine::finding::{Finding, Location, Severity};

        let report = IngestReport {
            rows: vec![
                TriageRow {
                    file: "decisions/auth-choice.md".to_string(),
                    best_match: Some("adr".to_string()),
                    verdict: "needs-reconcile",
                    finding: Some(Finding::graded(
                        Severity::Blocking,
                        "conformance.section-missing",
                        "required section heading `## context` is missing",
                        Some(Location::addressed("decisions/auth-choice.md", 1, 1)),
                        Some("reconcile decisions/auth-choice.md against the `adr` schema".into()),
                    )),
                    adopted: false,
                    annotations: Vec::new(),
                },
                TriageRow {
                    file: "decisions/rate-limit.md".to_string(),
                    best_match: Some("adr".to_string()),
                    verdict: "adoptable",
                    finding: None,
                    adopted: true,
                    // The two M40 adopt-time annotations, in their pinned shapes —
                    // indented lines under the adopted row, and row-carried in JSON.
                    annotations: vec![
                        "adopted — structurally empty: 0 milestones".to_string(),
                        "adopted — 2 surplus trailing sections".to_string(),
                    ],
                },
                TriageRow {
                    file: "docs/notes.md".to_string(),
                    best_match: None,
                    verdict: "unmanaged",
                    finding: None,
                    adopted: false,
                    annotations: Vec::new(),
                },
            ],
        };

        let agent = ingest(Format::Agent, &report);
        insta::assert_snapshot!(agent, @r"
        jigc ingest — 3 candidate(s) classified  (sorted — deterministic report order)

        needs-reconcile decisions/auth-choice.md → adr
          blocking · conformance.section-missing — required section heading `## context` is missing
          route: reconcile decisions/auth-choice.md against the `adr` schema
        adoptable decisions/rate-limit.md → adr  (adopted — indexed + baselined, no file moved)
          adopted — structurally empty: 0 milestones
          adopted — 2 surplus trailing sections
        unmanaged docs/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)

        What the verdicts above mean, and what to do next:
          adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
          needs-reconcile — parses as the named type but conflicts; fix it per the row's route, then re-run `jigc ingest`.
          unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        // The legend names only the verdicts actually present, each with its why +
        // next action (less-terse triage, #9c) — and never claims an absent verdict.
        assert!(
            agent.contains("unmanaged — matches no managed schema")
                && agent.contains("jigc migrate <path> --as <doctype>"),
            "the unmanaged legend states why + the next action; got:\n{agent}",
        );
        assert!(
            agent.contains("adoptable — conformant at its managed location"),
            "the adoptable legend states why it was adopted; got:\n{agent}",
        );
        // Human renders identically to agent in the MVP.
        assert_eq!(ingest(Format::Human, &report), agent);

        // JSON is the generic projection — parseable, no footer, and carries NONE of
        // the agent-text legend prose (the structured shape is unchanged — #9c regression
        // watch).
        let json_out = ingest(Format::Json, &report);
        assert!(!json_out.contains(ROUTING_FOOTER));
        assert!(!json_out.contains("What the verdicts"));
        assert!(!json_out.contains("legitimate end-state"));
        assert!(json_out.contains("\"verdict\": \"needs-reconcile\""));
        assert!(json_out.contains("\"code\": \"conformance.section-missing\""));
        assert!(json_out.contains("\"adopted\": true"));
        // The M40 adopt-time annotations are row-carried in the JSON projection too
        // (every output format — the agent path is the dominant consumer).
        assert!(json_out.contains("\"adopted — structurally empty: 0 milestones\""));
        assert!(json_out.contains("\"adopted — 2 surplus trailing sections\""));
    }

    /// A report carrying only one verdict class names **only that verdict** in the
    /// legend — the legend is keyed to the verdicts actually present, never a static
    /// menu that claims absent classes (#9c — "why *this* row classified that way").
    #[test]
    fn render_ingest_legend_names_only_present_verdicts() {
        use crate::ingest::{IngestReport, TriageRow};

        let report = IngestReport {
            rows: vec![TriageRow {
                file: "docs/notes.md".to_string(),
                best_match: None,
                verdict: "unmanaged",
                finding: None,
                adopted: false,
                annotations: Vec::new(),
            }],
        };

        let agent = ingest(Format::Agent, &report);
        assert!(
            agent.contains("unmanaged — matches no managed schema"),
            "the present verdict is explained; got:\n{agent}",
        );
        assert!(
            !agent.contains("adoptable —") && !agent.contains("needs-reconcile —"),
            "the legend names no absent verdict; got:\n{agent}",
        );
    }

    /// A8 (M43 Inc 7): the `unmanaged` surfaces **bless staying plain as a legitimate
    /// end-state** — the collapsed per-directory line says the files are fine to stay
    /// plain, and the legend gloss says "no action needed" before *offering* `jigc
    /// migrate` (never the old imperative that implied everything must migrate) —
    /// `design/surface-contract.md` → the style guide, the verdict-words bullet. The
    /// at-home squatter's adopt-it story (`adoptable`/`needs-reconcile` rows) is
    /// untouched.
    #[test]
    fn render_ingest_blesses_staying_plain_for_unmanaged() {
        use crate::ingest::{IngestReport, TriageRow};

        let report = IngestReport {
            rows: vec![TriageRow {
                file: "docs/notes.md".to_string(),
                best_match: None,
                verdict: "unmanaged",
                finding: None,
                adopted: false,
                annotations: Vec::new(),
            }],
        };

        let agent = ingest(Format::Agent, &report);
        // The collapsed line blesses the end-state, not just the non-action.
        assert!(
            agent.contains(
                "unmanaged docs/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)"
            ),
            "the collapsed unmanaged line blesses staying plain; got:\n{agent}",
        );
        // The legend gloss: ignorable in the first breath, migrate offered after.
        assert!(
            agent.contains("staying a plain file is a legitimate end-state — no action needed"),
            "the unmanaged gloss blesses staying plain as a legitimate end-state; got:\n{agent}",
        );
        assert!(
            agent.contains("To bring one under management: `jigc migrate <path> --as <doctype>`"),
            "`jigc migrate` stays the offered option; got:\n{agent}",
        );
        // The old imperative phrasing — an implied obligation — is gone.
        assert!(
            !agent.contains("left as-is — bring it under management with"),
            "no wording implies migration is obligatory; got:\n{agent}",
        );
    }

    /// An empty report renders no legend block (the omitting context stays inert — no
    /// dangling "What the verdicts mean" heading over zero rows).
    #[test]
    fn render_ingest_empty_report_renders_no_legend() {
        use crate::ingest::IngestReport;

        let agent = ingest(Format::Agent, &IngestReport { rows: Vec::new() });
        assert!(
            !agent.contains("What the verdicts"),
            "no rows → no legend heading; got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER));
    }

    /// V9 (M41 Inc 8): the text arm collapses `unmanaged` rows into per-directory
    /// count lines (not one line per file — a 220-candidate report must stay
    /// agent-readable), keeps a non-unmanaged row itemized, and the JSON arm still
    /// emits every row (the tooling contract is unchanged).
    #[test]
    fn render_ingest_collapses_unmanaged_into_per_directory_counts() {
        use crate::ingest::{IngestReport, TriageRow};

        let unmanaged = |file: &str| TriageRow {
            file: file.to_string(),
            best_match: None,
            verdict: "unmanaged",
            finding: None,
            adopted: false,
            annotations: Vec::new(),
        };
        let report = IngestReport {
            rows: vec![
                TriageRow {
                    file: "decisions/keep.md".to_string(),
                    best_match: Some("adr".to_string()),
                    verdict: "adoptable",
                    finding: None,
                    adopted: true,
                    annotations: Vec::new(),
                },
                unmanaged("docs/a.md"),
                unmanaged("docs/b.md"),
                unmanaged("docs/c.md"),
                unmanaged("src/x.md"),
                unmanaged("src/y.md"),
            ],
        };

        let agent = ingest(Format::Agent, &report);
        insta::assert_snapshot!(agent, @r"
        jigc ingest — 6 candidate(s) classified  (sorted — deterministic report order)

        adoptable decisions/keep.md → adr  (adopted — indexed + baselined, no file moved)
        unmanaged docs/ — 3 file(s) parse against no schema (left untouched — fine to stay plain)
        unmanaged src/ — 2 file(s) parse against no schema (left untouched — fine to stay plain)

        What the verdicts above mean, and what to do next:
          adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
          unmanaged — matches no managed schema; staying a plain file is a legitimate end-state — no action needed. To bring one under management: `jigc migrate <path> --as <doctype>`.
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        // No per-file unmanaged line survives in the text arm (the blow-up we fixed).
        assert!(
            !agent.contains("docs/a.md") && !agent.contains("src/y.md"),
            "unmanaged rows must not itemize per file in text; got:\n{agent}",
        );
        // The actionable non-unmanaged row stays itemized.
        assert!(agent.contains("adoptable decisions/keep.md → adr"));

        // JSON keeps every row (tooling contract): all five unmanaged paths present.
        let json_out = ingest(Format::Json, &report);
        for f in [
            "docs/a.md",
            "docs/b.md",
            "docs/c.md",
            "src/x.md",
            "src/y.md",
        ] {
            assert!(
                json_out.contains(f),
                "JSON must emit every unmanaged row; missing {f} in:\n{json_out}",
            );
        }
        assert!(!json_out.contains(ROUTING_FOOTER));

        // Order-invariant: the same rows fed with the unmanaged rows in reverse
        // relative order render byte-identical text (per-directory aggregation is
        // keyed on the directory, never row-encounter order).
        let reversed = IngestReport {
            rows: vec![
                report.rows[0].clone(),
                unmanaged("src/y.md"),
                unmanaged("src/x.md"),
                unmanaged("docs/c.md"),
                unmanaged("docs/b.md"),
                unmanaged("docs/a.md"),
            ],
        };
        assert_eq!(
            ingest(Format::Agent, &reversed),
            agent,
            "per-directory counts must be order-invariant",
        );
    }

    /// The successful-setup summary names each installed target **and what it is for**
    /// (#9c — less-terse setup output), ending with the routing footer; the JSON shape
    /// is unchanged (the `installed`/`line_file`/`allowlist_file` keys, no explanatory
    /// prose — regression watch).
    #[test]
    fn render_setup_success_names_what_was_installed() {
        let summary = SetupSummary {
            line_file: "CLAUDE.md".to_string(),
            allowlist_file: ".claude/settings.json".to_string(),
            install_commit: InstallCommit::Skipped,
        };

        let agent = setup_success(Format::Agent, &summary);
        insta::assert_snapshot!(agent, @r"
        jigc setup — adapter installed

        jigc is now wired into this project; setup installed:
          - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
          - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
          - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
          - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        // Human renders identically to agent in the MVP.
        assert_eq!(setup_success(Format::Human, &summary), agent);

        // JSON is unchanged: the three stable keys, none of the explanatory prose.
        let json_out = setup_success(Format::Json, &summary);
        assert!(!json_out.contains(ROUTING_FOOTER));
        assert!(!json_out.contains("wired into this project"));
        assert!(!json_out.contains("orients your assistant"));
        assert!(json_out.contains("\"installed\": true"));
        assert!(json_out.contains("\"line_file\": \"CLAUDE.md\""));
        assert!(json_out.contains("\"allowlist_file\": \".claude/settings.json\""));
    }

    /// The free-prose `describe` renderer frames the engine's woven definition
    /// sentences and projected command `hint`s into discursive paragraphs, carrying
    /// the authored strings verbatim and ending with the routing footer. This is the
    /// renderer that *defines* the output shape the T3 format predicate asserts over —
    /// so this test pins that the authored prose survives into the rendered surface
    /// (the predicate then pins the shape is hostile-to-parsing). Human renders
    /// identically to agent in the MVP; JSON carries no footer.
    #[test]
    fn render_describe_weaves_authored_prose_with_footer() {
        use engine::introspect::{CommandHint, DefinitionKind, DefinitionProse, Description};
        use engine::result::SCHEMA_VERSION;

        let description = Description {
            schema_version: SCHEMA_VERSION,
            definitions: vec![
                DefinitionProse {
                    kind: DefinitionKind::Workflow,
                    id: "single-task".to_string(),
                    prose: "single-task is one end-to-end scoped change. Reach for it when the work is one coherent change you can hold in your head.".to_string(),
                },
                DefinitionProse {
                    kind: DefinitionKind::Doctype,
                    id: "adr".to_string(),
                    prose: "adr is a dated architectural decision record. Reach for it when a choice is worth preserving with its rationale.".to_string(),
                },
            ],
            commands: vec![CommandHint {
                id: "finalize".to_string(),
                hint: "Validate, render the commit, and commit the task.".to_string(),
            }],
        };

        let agent = describe(Format::Agent, &description);

        // The authored prose survives into the rendered surface verbatim.
        assert!(
            agent.contains(
                "single-task is one end-to-end scoped change. Reach for it when the work is one coherent change you can hold in your head."
            ),
            "the workflow's woven sentence is carried verbatim; got:\n{agent}",
        );
        assert!(
            agent.contains(
                "adr is a dated architectural decision record. Reach for it when a choice is worth preserving with its rationale."
            ),
            "the doctype's woven sentence is carried verbatim; got:\n{agent}",
        );
        assert!(
            agent.contains("Validate, render the commit, and commit the task."),
            "the command-ref hint is carried verbatim; got:\n{agent}",
        );
        assert!(agent.ends_with(ROUTING_FOOTER), "got:\n{agent}");

        // Human renders identically to agent in the MVP (TUI is post-MVP).
        assert_eq!(describe(Format::Human, &description), agent);

        // JSON carries no footer (tooling-consumed, not the contract surface).
        let json_out = describe(Format::Json, &description);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: Description = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, description);
    }

    /// Skip-on-absent **through the assemble→render path**, plus the renderer-unit
    /// shape check: a definition that carries *neither* authored field is dropped by
    /// the engine's [`engine::introspect::Description::assemble`] (never reaching the
    /// renderer), so its id never appears in the rendered bytes — and the rendered
    /// prose carries no key-shaped line and no bullet row. This is the colocated
    /// counterpart to the integration-level format predicate (`tests/describe.rs`):
    /// the integration test holds the *real binary's* bytes to the full predicate;
    /// this pins, at the renderer unit, that an absent-field def is skipped end-to-end
    /// and the rendered surface stays prose-shaped (`introspection.md` → The authored
    /// fields, skip-on-absent; The operational format contract).
    #[test]
    fn render_describe_skips_absent_field_def_and_stays_prose_shaped() {
        use engine::compose::{CommandCatalog, WorkflowDef};
        use std::collections::BTreeMap;

        // A minimal `WorkflowDef` carrying only the two authored fields the weave
        // reads; the rest is inert for this projection.
        fn workflow(description: Option<&str>, usage: Option<&str>) -> WorkflowDef {
            WorkflowDef {
                when: None,
                description: description.map(str::to_owned),
                usage: usage.map(str::to_owned),
                creates_task: true,
                selectable: true,
                suppressed: None,
                allows_create: Vec::new(),
                reads: Vec::new(),
                includes: Vec::new(),
            }
        }

        let narrated = workflow(
            Some("a narrated workflow you can compose from intent to commit."),
            None,
        );
        let silent = workflow(None, None); // neither field → skip-on-absent
        let catalog = CommandCatalog {
            commands: BTreeMap::new(),
        };
        let description = Description::assemble(
            [("narrated", &narrated), ("silent-workflow", &silent)],
            std::iter::empty(),
            &catalog,
        );

        let agent = describe(Format::Agent, &description);

        // The narrated def survives; the both-absent def never appears in the bytes.
        assert!(
            agent.contains("a narrated workflow you can compose from intent to commit."),
            "the narrated definition's prose must render; got:\n{agent}",
        );
        assert!(
            !agent.contains("silent-workflow"),
            "a both-absent definition must be skipped end-to-end (never rendered); got:\n{agent}",
        );

        // The rendered surface stays prose-shaped: no key-shaped line, no bullet row
        // (the renderer-unit echo of the integration format predicate).
        for line in agent.trim_end().trim_end_matches(ROUTING_FOOTER).lines() {
            let trimmed = line.trim_start();
            if let Some(colon) = trimmed.find(':') {
                let key = &trimmed[..colon];
                let key_shaped = !key.is_empty()
                    && key
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                    && trimmed[colon + 1..]
                        .chars()
                        .next()
                        .is_some_and(char::is_whitespace);
                assert!(
                    !key_shaped,
                    "a key-shaped line leaked into the prose: {line:?}"
                );
            }
            let mut chars = trimmed.chars();
            let bullet = matches!(chars.next(), Some('-') | Some('*'))
                && chars.next().is_some_and(char::is_whitespace);
            assert!(!bullet, "a bullet row leaked into the prose: {line:?}");
        }
    }

    /// The shared operational-error funnel (M17 inc-1 T3): `json` emits the
    /// single-key envelope `{"error": "<anyhow chain>"}` (parseable by a tooling
    /// consumer); `agent` / `human` emit the `{err:#}` chain byte-identical to the
    /// historic `eprintln!("{err:#}")` funnels (no footer — an error is not a
    /// composed reading surface). The integration counterpart
    /// (`tests/finalize_outcome_surface.rs::operational_error_honors_json`) holds
    /// the real binary's stderr to the same contract.
    #[test]
    fn render_operational_error_json_envelope_and_plain_agent_bytes() {
        let err = anyhow::anyhow!("could not run `git` (is it on PATH?)")
            .context("validating task at `.jigc/tasks/x`");

        // Agent: exactly the alternate-chain bytes, nothing else.
        let agent = operational_error(Format::Agent, &err);
        assert_eq!(
            agent,
            "validating task at `.jigc/tasks/x`: could not run `git` (is it on PATH?)",
        );
        assert!(!agent.contains(ROUTING_FOOTER));

        // Human renders identically to agent in the MVP (TUI is post-MVP).
        assert_eq!(operational_error(Format::Human, &err), agent);

        // JSON: the single-key envelope carrying the same chain, no footer.
        let json_out = operational_error(Format::Json, &err);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        let object = value.as_object().expect("an object envelope");
        assert_eq!(object.keys().collect::<Vec<_>>(), ["error"]);
        assert_eq!(value["error"], serde_json::Value::String(agent));
    }

    /// The JSON rendering of the same value is valid JSON of the result type and
    /// carries no footer.
    #[test]
    fn render_orientation_json_is_valid_and_carries_no_footer() {
        let rendered = json(&fixture());

        insta::assert_snapshot!(rendered, @r#"
        {
          "schema_version": 2,
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

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let report = ValidationReport::new(
            vec![
                Finding::graded(
                    Severity::Blocking,
                    "file-state.hash-matches",
                    "on-disk content of `decisions/x.md` differs",
                    Some(Location::addressed("decisions/x.md", 1, 1)),
                    Some("reconcile decisions/x.md".into()),
                ),
                Finding::graded(
                    Severity::Blocking,
                    "schema-conformance.required-slot-present",
                    "required slot in section `summary` is empty",
                    Some(Location::addressed("commit:x#summary", 1, 1)),
                    // Routed, as the production gate block routes it (the route floor).
                    Some(
                        "`jigc doc set-slot <address> --from-file -` to fill the empty slot".into(),
                    ),
                ),
            ],
            &resolved,
        );

        let agent = validation(Format::Agent, &report);
        insta::assert_snapshot!(agent, @"
        blocking · file-state.hash-matches — on-disk content of `decisions/x.md` differs
          route: reconcile decisions/x.md
        blocking · schema-conformance.required-slot-present — required slot in section `summary` is empty
          route: `jigc doc set-slot <address> --from-file -` to fill the empty slot
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        assert_eq!(validation(Format::Human, &report), agent);

        // A clean report renders the positive line + the footer.
        let clean = validation(Format::Agent, &ValidationReport::new(Vec::new(), &resolved));
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

    /// The **store-scope** sweep view is task-less: its clean line is store-scoped, not
    /// the task-scoped "the task validates clean" wording (M26 shakedown #10b). Findings
    /// render identically to the task view — only the clean line differs.
    #[test]
    fn render_validation_store_clean_line_is_store_scoped() {
        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");

        let clean = validation_store(
            Format::Agent,
            &ValidationReport::new(Vec::new(), &resolved),
            &BTreeSet::new(),
        );
        insta::assert_snapshot!(clean, @r"
        no findings — the committed store validates clean
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        // The store view must NOT reuse the task-scoped wording.
        assert!(!clean.contains("the task validates clean"));
    }

    /// The store sweep is **report-only** for content findings (exit 0) — a human or
    /// script must be able to tell that from the output, so the per-finding cascade-severity
    /// token (`blocking · doc-code…`) is not misread as a gate failure. The view appends a
    /// clarifying trailer naming where these findings *actually* gate, and the JSON carries a
    /// machine-readable `report_only` signal. A `pack-probe-integrity.*` meta-finding (the
    /// one exit-non-zero exception) flips both surfaces so it stays clearly distinguished.
    #[test]
    fn render_validation_store_trailer_clarifies_report_only_vs_probe_integrity() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");

        // A blocking-cascade content finding at store scope is report-only (exit 0).
        let content = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                "doc-code.symbol-exists",
                "cited symbol `evict_lru` not found",
                Some(Location::addressed("adr:cache#status/cites-code", 1, 1)),
                // Routed, as the real probe routes it (the route floor).
                Some("update the citation, or restore the cited symbol".into()),
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &content, &BTreeSet::new());
        insta::assert_snapshot!(agent, @"
        blocking (gates at finalize) · doc-code.symbol-exists — cited symbol `evict_lru` not found
          route: update the citation, or restore the cited symbol
        1 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize`.
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert_eq!(
            validation_store(Format::Human, &content, &BTreeSet::new()),
            agent
        );

        // JSON carries the machine-readable report-only signal + the scope tag.
        let json_out = validation_store(Format::Json, &content, &BTreeSet::new());
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["scope"], "store");
        assert_eq!(value["report_only"], serde_json::Value::Bool(true));
        // The findings array is still the generic projection — no shape break.
        assert_eq!(
            value["findings"].as_array().expect("findings array").len(),
            1
        );

        // A pack-probe-integrity meta-finding (the exit-non-zero exception) flips both
        // surfaces: the trailer distinguishes it and `report_only` goes false.
        let probe = ValidationReport::new(
            // The pack-resource target form (T3): the probe id.
            vec![Finding::graded(
                Severity::Blocking,
                "pack-probe-integrity.crash",
                "the doc-code probe exited 2",
                Some(Location::addressed("doc-code", 1, 1)),
                // Routed, as the production meta-finding routes it (the route floor).
                Some("repair or re-install the probe binary, then re-run the sweep".into()),
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &probe, &BTreeSet::new());
        assert!(
            agent.contains("the sweep could not complete and exits"),
            "the probe-integrity path must be distinguished from a report-only content \
             finding: {agent}",
        );
        assert!(
            !agent.contains("report-only at store scope"),
            "the probe-integrity trailer must not claim report-only: {agent}",
        );
        let json_out = validation_store(Format::Json, &probe, &BTreeSet::new());
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["report_only"], serde_json::Value::Bool(false));

        // A store-scope OOB-rename finding (exit-flipping exception #2, M35) flips both
        // surfaces just like probe-integrity: the trailer states the sweep exits non-zero
        // and `report_only` goes false — an OOB `git mv` is a structural-identity event,
        // not report-only content drift.
        let rename = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                "reconciliation.rename",
                "tracked managed doc adr:cache (decisions/cache.md) is missing",
                Some(Location::addressed("decisions/cache.md", 1, 1)),
                // Routed, as the production rename detector routes it (the route floor).
                Some("restore decisions/cache.md, or adopt the move with `jigc rename`".into()),
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &rename, &BTreeSet::new());
        assert!(
            agent.contains("exits non-zero"),
            "the rename trailer must announce the exits-non-zero contract: {agent}",
        );
        assert!(
            !agent.contains("report-only at store scope"),
            "the rename trailer must not claim report-only: {agent}",
        );
        let json_out = validation_store(Format::Json, &rename, &BTreeSet::new());
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["report_only"], serde_json::Value::Bool(false));
    }

    /// **The trailer claims a gate only where one exists** (M42 Inc 4 T3; `validation.md` →
    /// The trailer must not claim a gate that does not exist). The blanket sentence *"these
    /// gate at `jigc task validate` / `jigc task finalize`"* is **false** for every
    /// store-scope-only finding: those codes are emitted by no task-scope path at all, so
    /// neither gate can ever see them. A report whose findings **all** gate nowhere must make
    /// no gate claim; a report carrying a genuinely task-gating finding still names where it
    /// gates; a mixed report says how many of each.
    #[test]
    fn render_validation_store_trailer_claims_a_gate_only_where_one_exists() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");

        // Written out literally — **the contract, not a re-read of the constant**: every code a
        // store sweep can emit that `engine::validate::validate_task` never emits (the shared
        // body of `jigc task validate` and the `finalize` preflight). Dropping any one of these
        // from the shipped set puts the false gate claim back on the wire for it.
        let advisory = |code: &str| {
            Finding::graded(
                Severity::Advisory,
                code,
                format!("a store-scope finding under `{code}`"),
                Some(Location::addressed("decisions/cache.md", 1, 1)),
                Some("follow the finding's own route".into()),
            )
        };
        let gate_nowhere = ValidationReport::new(
            vec![
                advisory("schema-conformance.mention-resolves"),
                advisory("schema-conformance.repeatable-populated"),
                advisory("schema-conformance.surplus-sections-absent"),
                advisory("schema-conformance.unadopted-instance"),
                advisory("schema-completeness.inverse-cardinality"),
                advisory("file-state.un-baselined"),
                advisory("file-state.orphaned-doc"),
                advisory("file-state.unregistered-doc"),
                advisory("store-version.binary-mismatch"),
            ],
            &resolved,
        );

        let agent = validation_store(Format::Agent, &gate_nowhere, &BTreeSet::new());
        assert!(
            !agent.contains("jigc task validate") && !agent.contains("jigc task finalize"),
            "every finding here gates nowhere — the trailer must not name a gate: {agent}",
        );
        assert!(
            agent.contains("report-only at store scope (exit 0)"),
            "it is still report-only, and still says so: {agent}",
        );
        assert!(
            agent.contains("9 finding(s)") && agent.contains("gates nowhere"),
            "and it states the truth — none of them gates anywhere: {agent}",
        );
        // The exit contract is untouched: gating nowhere is not the same as flipping the exit.
        assert!(!validation_store_exit_flips(&gate_nowhere));

        // A genuinely task-gating finding still names where it gates.
        let gating = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                "doc-code.symbol-exists",
                "cited symbol `evict_lru` not found",
                Some(Location::addressed("adr:cache#status/cites-code", 1, 1)),
                None,
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &gating, &BTreeSet::new());
        assert!(
            agent.contains("these gate at `jigc task validate` / `jigc task finalize`"),
            "a doc-code break DOES gate at the task boundary — say so: {agent}",
        );

        // Mixed: the gate claim is scoped to the findings that carry one.
        let mixed = ValidationReport::new(
            vec![
                advisory("schema-conformance.mention-resolves"),
                Finding::graded(
                    Severity::Blocking,
                    "doc-code.symbol-exists",
                    "cited symbol `evict_lru` not found",
                    Some(Location::addressed("adr:cache#status/cites-code", 1, 1)),
                    None,
                ),
            ],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &mixed, &BTreeSet::new());
        assert!(
            agent.contains("1 of them gate at `jigc task validate` / `jigc task finalize`"),
            "the claim covers the gating finding only: {agent}",
        );
        assert!(
            agent.contains("the rest are store-scope advisories that gate nowhere"),
            "and disowns the gate for the store-scope-only one: {agent}",
        );
    }

    /// **The criterion is per *finding*, not per *code*** (M42 Inc 4 — the correction to T3;
    /// `validation.md` → The trailer must not claim a gate that does not exist).
    ///
    /// The code-level [`GATES_NOWHERE`] list asks *"can any task-scope path emit this code?"* —
    /// and for the store sweep's per-doc conformance breaks the answer is *"yes, over a **staged**
    /// doc"*, which says **nothing** about the **committed** doc this finding is actually
    /// addressed at. `engine::validate::validate_task` routes the committed store through
    /// `file_state::reconcile_committed_store`, which grades it by **baseline membership**:
    ///
    /// - **un-baselined** (no `FileStateRecord` entry — a fresh clone, a brownfield adoption, any
    ///   hand-authored corpus) → **advisory** `reconciliation.conformance-block`. It gates
    ///   nowhere, permanently, and the trailer must not send the reader to a gate for it.
    /// - **baselined** → the drift really does reach the **blocking** arm. The claim stands, and
    ///   suppressing it here would be the opposite lie.
    ///
    /// One and the same finding, both ways — so the test cannot pass by keying on the code, which
    /// is what the shipped criterion did. (The end-to-end proof, on the corpus a real repo
    /// produces, is `crates/cli/tests/managed_vs_foreign.rs`.)
    #[test]
    fn render_validation_store_gate_claim_turns_on_the_docs_baseline_not_on_the_code() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");

        // Exactly what the store sweep emits over a committed ADR missing `## Consequences`:
        // the parse-failure arm's `conformance.*` break, attributed to the doc's identity, plus
        // family 3's un-baselined advisory for the same doc.
        let report = || {
            ValidationReport::new(
                vec![
                    Finding::graded(
                        Severity::Blocking,
                        "conformance.section-missing",
                        "`docs/decisions/cache-it.md`: required section heading \
                         `## consequences` is missing",
                        Some(Location::addressed("adr:cache-it", 1, 1)),
                        None,
                    ),
                    Finding::graded(
                        Severity::Advisory,
                        "file-state.un-baselined",
                        "committed doc `docs/decisions/cache-it.md` is not yet baselined",
                        Some(Location::addressed("docs/decisions/cache-it.md", 1, 1)),
                        Some("no action needed".into()),
                    ),
                ],
                &resolved,
            )
        };

        // (i) UN-BASELINED — the dominant `jigc validate` corpus. No gate exists for either
        // finding, so the trailer names none.
        let unbaselined: BTreeSet<String> = ["adr:cache-it".to_string()].into_iter().collect();
        let agent = validation_store(Format::Agent, &report(), &unbaselined);
        assert!(
            !agent.contains("jigc task validate") && !agent.contains("jigc task finalize"),
            "at task scope this committed doc is graded ADVISORY by the reconciler — the \
             conformance break gates nowhere, and the trailer must not claim it does: {agent}",
        );
        assert!(
            agent.contains("2 finding(s)") && agent.contains("gates nowhere"),
            "and it states the truth for both: {agent}",
        );

        // (ii) BASELINED — the same finding, the same code, the opposite verdict: the drift
        // reaches the reconciler's blocking arm, so the gate is real and the claim stands.
        let agent = validation_store(Format::Agent, &report(), &BTreeSet::new());
        assert!(
            agent.contains("1 of them gate at `jigc task validate` / `jigc task finalize`"),
            "a baselined doc's drift DOES block at the task boundary — withdrawing the claim \
             here would be the opposite lie: {agent}",
        );

        // (iii) The address joins on the doc identity, fragment and all — a finding addressed at
        // a *slice* of the un-baselined doc is the same doc.
        let sliced = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                "schema-conformance.field-value-conformant",
                "`docs/decisions/cache-it.md`: `status` is not one of the enum members",
                Some(Location::addressed("adr:cache-it#header/status", 1, 1)),
                None,
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &sliced, &unbaselined);
        assert!(
            !agent.contains("jigc task validate"),
            "the identity is the address up to the `#` — a sliced address is the same doc: \
             {agent}",
        );
    }

    /// **The per-finding gate label claims a finalize block only where one lands** (M42 Inc 12 /
    /// T5; `validation.md` → The trailer must not claim a gate that does not exist, whose closing
    /// note pins this label as a dependency on that fix). The store sweep prints every finding at
    /// its **cascade** severity while exiting 0, so `blocking ·` alone says nothing about whether
    /// anything ever *stops* on it — the label answers that, per row. Two suppressions the
    /// end-to-end arms (`crates/cli/tests/validate_command.rs`) cannot reach:
    ///
    /// - a **cascade-demoted** finding: `warning`/`advisory` is surfaced at the task boundary but
    ///   blocks no transaction, so *"gates at finalize"* would be false (the cascade demotes
    ///   `doc-code` breaks in the field — `flow13_contract_and_severity.rs`);
    /// - the **version-currency break**: blocking, `Location`-less, and it gates **nowhere** (at
    ///   task scope a stale doc downgrades to an advisory), so the conservative
    ///   no-address-no-suppression direction would hand it the label unless the code is on
    ///   [`GATES_NOWHERE`] — the reason it is listed there and not only in the trailer's branch.
    #[test]
    fn render_validation_store_gate_label_names_only_a_gate_that_lands() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let doc_code = |severity: Severity| {
            Finding::graded(
                severity,
                "doc-code.symbol-exists",
                "cited symbol `evict_lru` not found",
                Some(Location::addressed("adr:cache#status/cites-code", 1, 1)),
                None,
            )
        };

        // A cascade-DEMOTED break over the very doc whose blocking twin earns the label: no
        // transaction stops on a warning, so no gate is claimed.
        let demoted = ValidationReport::new(vec![doc_code(Severity::Warning)], &resolved);
        let agent = validation_store(Format::Agent, &demoted, &BTreeSet::new());
        assert!(
            agent.starts_with("warning · doc-code.symbol-exists"),
            "a demoted finding renders at its cascade severity, unlabelled — `finalize` never \
             stops on it: {agent}",
        );
        assert!(
            !agent.contains("gates at finalize"),
            "and it must claim no finalize gate: {agent}",
        );

        // The version-currency break — blocking, address-less, gating nowhere in the system.
        let stale = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                engine::validate::SCHEMA_VERSION_CURRENT_CODE,
                "field `schema-version` is schema-version 1, below the current schema-version 2",
                None,
                Some("run `jigc migrate-corpus` to upgrade it".into()),
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &stale, &BTreeSet::new());
        assert!(
            !agent.contains("gates at finalize"),
            "a stale-corpus break gates NOWHERE (task scope downgrades it to an advisory) — the \
             row must not send the reader to `finalize`: {agent}",
        );
        assert!(
            agent.contains("blocking · schema-conformance.schema-version-current"),
            "it still renders at its bare cascade severity — the exit flip is the trailer's \
             claim, not the row's: {agent}",
        );
    }

    /// The dry-run manifest renders a titled block listing each entry by kind — an
    /// untracked sweep flagged distinctly — with no trailing newline; JSON carries
    /// `dry_run: true` and a `manifest[]` of `{path,kind}` (kebab-case kinds).
    #[test]
    fn render_finalize_manifest_flags_untracked_and_json_carries_dry_run() {
        let included = vec![ManifestEntry {
            path: "docs/decisions/x.md".to_string(),
            kind: ManifestKind::Promoted,
        }];
        let left_out = vec![ManifestEntry {
            path: "scratch.txt".to_string(),
            kind: ManifestKind::Untracked,
        }];

        let agent = finalize_manifest(Format::Agent, &included, &left_out);
        insta::assert_snapshot!(agent, @r"
        finalize --dry-run — pre-commit manifest (nothing committed)
          promoted docs/decisions/x.md
          left-out (unstaged/untracked — git add to include):
            scratch.txt");
        assert!(
            !agent.ends_with('\n'),
            "no trailing newline — the caller closes it"
        );
        assert_eq!(
            finalize_manifest(Format::Human, &included, &left_out),
            agent
        );

        let json_out = finalize_manifest(Format::Json, &included, &left_out);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["dry_run"], serde_json::Value::Bool(true));
        let manifest = value["manifest"].as_array().expect("manifest array");
        assert_eq!(manifest[0]["path"], "docs/decisions/x.md");
        assert_eq!(manifest[0]["kind"], "promoted");
        let left = value["left_out"].as_array().expect("left_out array");
        assert_eq!(left[0]["path"], "scratch.txt");
        assert_eq!(left[0]["kind"], "untracked");
    }

    /// An *included* deliberately-staged new file renders `added <path>` (M30 audit finding
    /// 3), never the retired `swept (was untracked)` wording — that string is gone from the
    /// included path entirely; JSON serializes the kebab-case kind `added`.
    #[test]
    fn render_finalize_manifest_renders_staged_new_file_as_added() {
        let included = vec![ManifestEntry {
            path: "src/feature.rs".to_string(),
            kind: ManifestKind::Added,
        }];

        let agent = finalize_manifest(Format::Agent, &included, &[]);
        assert!(
            agent.contains("  added src/feature.rs"),
            "a staged new file renders `added`; agent:\n{agent}",
        );
        assert!(
            !agent.contains("swept"),
            "the retired `swept` wording must not appear on the included path; agent:\n{agent}",
        );

        let json_out = finalize_manifest(Format::Json, &included, &[]);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["manifest"][0]["kind"], "added");
    }

    /// A carried entry renders `carried-over <path>` in the manifest text, serializes the
    /// kebab-case JSON kind `carried-over` (the M43 additive enum value,
    /// `design/command-output-contract.md` → Evolution posture), and the pre-commit
    /// [`carried_over_advisory`] names each carried path under the declared-carry header —
    /// empty (no bytes) when nothing is carried.
    #[test]
    fn render_manifest_carried_over_kind_and_pre_commit_advisory() {
        let carried = vec![ManifestEntry {
            path: "foreign-a.txt".to_string(),
            kind: ManifestKind::CarriedOver,
        }];

        let agent = finalize_manifest(Format::Agent, &carried, &[]);
        assert!(
            agent.contains("  carried-over foreign-a.txt"),
            "a carried entry renders `carried-over`; agent:\n{agent}",
        );
        let json_out = finalize_manifest(Format::Json, &carried, &[]);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["manifest"][0]["kind"], "carried-over");

        let advisory = carried_over_advisory(&carried);
        insta::assert_snapshot!(advisory, @r"
        finalize — committing the index; carrying over (staged before this task existed — declared with `--carry-staged`):
          carried-over foreign-a.txt
        ");
        assert_eq!(
            carried_over_advisory(&[]),
            "",
            "nothing carried ⇒ no bytes at all"
        );
    }

    /// A landed finalize's JSON `committed` object carries the **included** `manifest[]`
    /// alongside the `files` count *and* the **left_out[]** residual (M30 G3); the agent-text
    /// summary lists each included entry by kind, then the left-out section rendered
    /// identically to the dry-run forecast. The uncommitted stray is in `left_out`, not the
    /// included manifest.
    #[test]
    fn render_finalize_landed_carries_the_manifest() {
        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let report = ValidationReport::new(Vec::new(), &resolved);
        let landed = Landed {
            hash: "abc1234".to_string(),
            subject: "feat: surface the manifest".to_string(),
            promoted: vec!["docs/decisions/x.md".to_string()],
            files: 1,
            manifest: vec![ManifestEntry {
                path: "docs/decisions/x.md".to_string(),
                kind: ManifestKind::Promoted,
            }],
            left_out: vec![ManifestEntry {
                path: "scratch.txt".to_string(),
                kind: ManifestKind::Untracked,
            }],
        };

        let agent = finalize_landed(Format::Agent, &report, &landed);
        assert!(agent.contains("promoted docs/decisions/x.md"));
        assert!(agent.contains("1 file committed"));
        // The left-out residual is rendered identically to the dry-run (the shared helper).
        assert!(
            agent.contains("left-out (unstaged/untracked — git add to include):")
                && agent.contains("    scratch.txt"),
            "the landed summary names the left-out stray identically to the dry-run; agent:\n{agent}",
        );
        // The stray is NOT a swept inclusion — it is left out, not in the committed set.
        assert!(
            !agent.contains("swept (was untracked) scratch.txt"),
            "the stray is left-out, never a swept inclusion; agent:\n{agent}",
        );

        let json_out = finalize_landed(Format::Json, &report, &landed);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        let committed = &value["committed"];
        assert_eq!(committed["files"], 1);
        let manifest = committed["manifest"].as_array().expect("manifest array");
        assert_eq!(manifest.len(), 1);
        assert_eq!(manifest[0]["path"], "docs/decisions/x.md");
        assert!(
            !manifest.iter().any(|e| e["path"] == "scratch.txt"),
            "the stray is absent from committed.manifest; manifest:\n{manifest:?}",
        );
        let left_out = committed["left_out"].as_array().expect("left_out array");
        assert_eq!(left_out[0]["path"], "scratch.txt");
        assert_eq!(left_out[0]["kind"], "untracked");
    }

    /// The fidelity kept-set scans the **whole rewrite text**, not only `### ` headings
    /// (`design/auto-migration.md` → Hardening #5, the M40 calibration fix): a prd-shaped
    /// rewrite that keeps `2.0` in body prose — no `### ` item heading anywhere — must
    /// report `(none)`, never a false "dropped 2.0" (the heading-only kept-set was a
    /// changelog-shaped assumption that false-alarmed every other doctype).
    #[test]
    fn render_migration_review_fidelity_kept_set_scans_whole_rewrite_text() {
        let foreign = "# Product Requirements\n\nTargets the 2.0 platform release.\n";
        let rewrites = vec![(
            "docs/prd/platform.md".to_string(),
            "# Platform PRD\n\n## Context\n\nThis effort targets the 2.0 platform release.\n"
                .to_string(),
        )];

        let out = migration_review(Format::Agent, "task-1", foreign, &rewrites);
        let summary = out
            .lines()
            .find(|l| l.contains("source releases absent from the rewrite"))
            .unwrap_or_else(|| panic!("the review must render the fidelity summary; got:\n{out}"));
        assert!(
            summary.contains("(none)"),
            "2.0 kept in body prose is not dropped — the summary must be (none); got:\n{summary}",
        );
        assert!(
            !summary.contains("2.0"),
            "the kept 2.0 must not be reported as dropped; got:\n{summary}",
        );
    }

    /// A trailing sentence-dot no longer makes a version token escape the scan
    /// (Hardening #5, the M40 calibration fix): a foreign `since 1.5.` sentence yields a
    /// reportable `1.5` (previously the whole `1.5.` token was rejected → under-report).
    #[test]
    fn render_migration_review_fidelity_scan_trims_trailing_sentence_dot() {
        let foreign = "# Notes\n\nSupported since 1.5.\n";
        let rewrites = vec![(
            "docs/prd/notes.md".to_string(),
            "# Notes PRD\n\n## Context\n\nNo version mentioned here.\n".to_string(),
        )];

        let out = migration_review(Format::Agent, "task-1", foreign, &rewrites);
        let summary = out
            .lines()
            .find(|l| l.contains("source releases absent from the rewrite"))
            .unwrap_or_else(|| panic!("the review must render the fidelity summary; got:\n{out}"));
        assert!(
            summary.trim_end().ends_with(": 1.5"),
            "the dropped 1.5 is reported without the sentence dot; got:\n{summary}",
        );
    }
}
