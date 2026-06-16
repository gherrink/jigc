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
use crate::setup::{SetupSummary, UninstallSummary};
use engine::compose::ComposedWorkflow;
use engine::finding::{Finding, Severity};
use engine::introspect::{DefinitionKind, Description};
use engine::milestone::JoinOutcome;
use engine::result::{NextStep, Orientation, OrientationView, ResolutionTree, ValidationReport};
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
        "\nRun: `jigc start \"<intent>\"`   — routes among the workflows above; pick one, then re-run with `--workflow <chosen>`\n",
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
            let mut out = String::from(
                "jigc setup — adapter installed\n\njigc is now wired into this project; two host files were updated:\n",
            );
            out.push_str("  - bootstrap reference → ");
            out.push_str(&summary.line_file);
            out.push_str("   (orients your assistant to `jigc start` each session)\n");
            out.push_str("  - jigc allowlist → ");
            out.push_str(&summary.allowlist_file);
            out.push_str("   (pre-approves the `jigc` commands the agent runs)\n");
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

/// Render a successful `jigc uninstall` teardown to the surface `format` selects
/// (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5)): `agent` /
/// `human` emit a one-line-per-target summary of what was torn down, followed by the
/// routing footer; `json` emits a generic object naming the two host targets, with no
/// footer (tooling-consumed). The summary states the repo-local footprint was removed
/// (the machine-global `doc-code` probe is left in place — B2).
pub fn uninstall_success(format: Format, summary: &UninstallSummary) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "uninstalled": true,
            "line_file": summary.line_file,
            "allowlist_file": summary.allowlist_file,
        })),
        Format::Agent | Format::Human => {
            let mut out = String::from("jigc uninstall — repo-local install removed\n\n");
            out.push_str("  - removed .jigc/\n");
            out.push_str("  - unwired bootstrap reference ← ");
            out.push_str(&summary.line_file);
            out.push('\n');
            out.push_str("  - removed jigc allowlist ← ");
            out.push_str(&summary.allowlist_file);
            out.push('\n');
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
            for row in &report.rows {
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
                if let Some(finding) = &row.finding {
                    out.push_str("  ");
                    out.push_str(&finding_line(finding));
                }
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
        "matches no managed schema; left as-is — bring it under management with `jigc migrate <path> --as <doctype>`.",
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
                    Some(id) => format!(
                        "unmanaged {} ({}) — dropped its file-state baseline + forward edges; the file is left on disk\n",
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
/// rewritten in lockstep) followed by the routing footer; `json` emits the
/// **generic** projection of the [`JoinOutcome`], with no footer (tooling-consumed).
///
/// The suffix decision is read straight off the merged overlay (a pure function of
/// it, like the merge itself): an entry is a collision suffix iff its address ends
/// `-<N>` (N ≥ 2) **and** the de-suffixed base address is also in the overlay (the
/// bare instance the lower task id kept); the self-ref rewrite is named iff that
/// suffixed instance carries an edge back to its own (suffixed) address. The verb
/// commits nothing — wiring the suffix-resolved overlay into `finalize` is a later
/// increment (`design/storage.md` → The by-task-id join; `design/worked-examples.md`
/// → flow 9).
pub fn milestone_join(format: Format, milestone_id: &str, outcome: &JoinOutcome) -> String {
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
            // reviewer needn't eyeball that N of M releases survived. The canonical side is
            // conformant (release versions read off its `### …` item headings); the foreign
            // side is non-conformant, so it is a HEURISTIC version-scan. Negative guard
            // (DECISIONS C4, Framing A): display-only — labeled fuzzy, feeds no gate, no
            // agent logic, no structural decision; never a second structural authority.
            let dropped = dropped_release_versions(foreign, rewrites);
            if !dropped.is_empty() {
                out.push_str(&format!(
                    "fidelity (heuristic version-scan — fuzzy, advisory; feeds no gate, no \
                     structural decision): source releases absent from the rewrite: {}\n\n",
                    dropped.join(", "),
                ));
            }
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
/// absent from the conformant `rewrites`' release-item headings, sorted + de-duplicated
/// for a stable display. Fuzzy by construction (the foreign side is non-conformant, so
/// the scan can miss or invent a release); the result is **display-only** and feeds no
/// structural decision (DECISIONS C4, Framing A).
fn dropped_release_versions(foreign: &str, rewrites: &[(String, String)]) -> Vec<String> {
    // The conformant side: release versions live on the `### …` item headings (an H3
    // repeatable item — `### 1.0.0  {#100}`); a deeper `#### …` change-group heading and
    // the H2 section headings carry no version token, so this naturally excludes them.
    let mut kept: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (_destination, rendered) in rewrites {
        for line in rendered.lines() {
            if let Some(title) = line.trim_start().strip_prefix("### ") {
                kept.extend(scan_version_tokens(title));
            }
        }
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
/// trailing/doubled dot. Dash-separated dates (`2021-06-01`) carry no `.` and so never
/// match. Heuristic only — see [`dropped_release_versions`].
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
            let token = &text[start..i];
            if token.contains('.')
                && !token.starts_with('.')
                && !token.ends_with('.')
                && !token.contains("..")
            {
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
/// the woven definition sentences flow as running prose under light *unkeyed*
/// section transitions ("The workflows you can compose here …", "The doc-types you
/// can author …", "And the commands jigc gives you …"), the routing footer last.
/// The shape is deliberately **hostile to parsing** (the non-contractual format
/// contract — `introspection.md` → Non-contractual by design): no key-shaped lines,
/// no bullet rows, no per-definition extractable handle — describe is a menu, not an
/// API. `json` still routes through the generic serde renderer, but that projection
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
                out.push_str(&workflows.join(" "));
                out.push_str("\n\n");
            }
            if !doctypes.is_empty() {
                out.push_str("The doc-types you can author. ");
                out.push_str(&doctypes.join(" "));
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
            .find("routes among the workflows above")
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
        use engine::finding::{Finding, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let report = ValidationReport::new(
            vec![Finding::graded(
                Severity::Warning,
                "override-default.target-unchanged",
                "override target `workflow:single-task#implement` changed in the current pack",
                None,
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
            findings: Vec::new(),
        };

        let agent = milestone_join(Format::Agent, "cache-rework", &outcome);
        insta::assert_snapshot!(agent, @r"
        joined milestone:cache-rework — 3 doc(s) merged
          - adr:cache-strategy  (created · from area-low)
          - adr:cache-strategy-2  (created · from area-zed)  ← suffixed -2 on collision; self-ref rewritten
          - adr:eviction-policy  (edited-from-base · from evict-stale-keys)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        // The disjoint doc carries no suffix annotation.
        assert!(
            !agent.contains("adr:eviction-policy  (edited-from-base · from evict-stale-keys)  ←"),
            "a disjoint doc must not be annotated as suffixed; got:\n{agent}",
        );

        // JSON is the generic projection — round-trips, no footer.
        let json_out = milestone_join(Format::Json, "cache-rework", &outcome);
        assert!(!json_out.contains(ROUTING_FOOTER));
        let back: JoinOutcome = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(back, outcome);
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
                },
                TriageRow {
                    file: "decisions/rate-limit.md".to_string(),
                    best_match: Some("adr".to_string()),
                    verdict: "adoptable",
                    finding: None,
                    adopted: true,
                },
                TriageRow {
                    file: "docs/notes.md".to_string(),
                    best_match: None,
                    verdict: "unmanaged",
                    finding: None,
                    adopted: false,
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
        unmanaged docs/notes.md → (parses against no schema — left untouched)

        What the verdicts above mean, and what to do next:
          adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
          needs-reconcile — parses as the named type but conflicts; fix it per the row's route, then re-run `jigc ingest`.
          unmanaged — matches no managed schema; left as-is — bring it under management with `jigc migrate <path> --as <doctype>`.
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
        assert!(!json_out.contains("bring it under management"));
        assert!(json_out.contains("\"verdict\": \"needs-reconcile\""));
        assert!(json_out.contains("\"code\": \"conformance.section-missing\""));
        assert!(json_out.contains("\"adopted\": true"));
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

    /// The successful-setup summary names each installed target **and what it is for**
    /// (#9c — less-terse setup output), ending with the routing footer; the JSON shape
    /// is unchanged (the `installed`/`line_file`/`allowlist_file` keys, no explanatory
    /// prose — regression watch).
    #[test]
    fn render_setup_success_names_what_was_installed() {
        let summary = SetupSummary {
            line_file: "CLAUDE.md".to_string(),
            allowlist_file: ".claude/settings.json".to_string(),
        };

        let agent = setup_success(Format::Agent, &summary);
        insta::assert_snapshot!(agent, @r"
        jigc setup — adapter installed

        jigc is now wired into this project; two host files were updated:
          - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
          - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
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
            ],
            &resolved,
        );

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
}
