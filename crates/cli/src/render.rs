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
use crate::gitignore::Ensured;
use crate::ingest::IngestReport;
use crate::milestone::MilestoneCreated;
use crate::setup::{InstallCommit, SetupSummary, UninstallSummary};
use crate::start::Composition;
use crate::task::TaskListRow;
use engine::finding::{Finding, Findings, Location, Route, Severity};
use engine::introspect::{DefinitionKind, Description};
use engine::milestone::JoinOutcome;
use engine::result::{
    ActiveTask, NextStep, Orientation, OrientationView, ResolutionTree, ValidationReport,
};
use engine::state::BasePin;
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
            OrientationView::ActiveTask {
                header,
                tasks,
                workflows,
                next_steps,
                ..
            } => orientation_active(
                header,
                tasks,
                &Orientation::new(workflows.clone()),
                next_steps,
            ),
        },
    }
}

/// Render the **active-task** orientation (`design/bootstrap.md` → Orientation output
/// examples, states 3 and 4 collapsed; M50 → the Settle, D3): the cascade/provenance
/// `header`, then one block per live task — what it is, what it holds, what the sweep
/// found — then the [`catalog_block`] the clean view also renders, then the universal
/// routing footer.
///
/// **Four `Run:` directives per task**, delivering the routing inside the push channel
/// the agent is already reading: resume, preview, commit, abandon. Two of the four are
/// **state-dependent, not shape-dependent**, because a route this binary's own guard
/// blocks is a route-floor defect: the commit directive names the milestone door for a
/// sub-task (`jigc task finalize <sub>` refuses outright), and the abandon directive
/// carries the `--force` consent exactly when the staged set makes the door demand it
/// (M50 → the Settle, D1/B9). The printed argv is the one that runs, in every state this
/// view can render.
///
/// **The findings render through [`finding_line`]** — the house shape, `severity ·
/// code — message` with its `at:` locus and its `route:` — so this surface gains no
/// private finding renderer to drift from the others.
fn orientation_active(
    header: &str,
    tasks: &[ActiveTask],
    orientation: &Orientation,
    next_steps: &[NextStep],
) -> String {
    let mut out = String::from("jigc — orientation\n\n");
    out.push_str(header);
    out.push('\n');
    for task in tasks {
        let id = &task.id;
        out.push_str(&format!("\nActive task: {id}\n"));
        out.push_str(&format!(
            "  workflow: {}\n",
            task.workflow.as_deref().unwrap_or(NO_RECORDED_WORKFLOW)
        ));
        // Each fact is omitted rather than printed empty when the working area does not
        // answer for it: an omitting context stays inert, and a blank value beside a
        // label reads as a value.
        if !task.intent.trim().is_empty() {
            out.push_str(&format!("  intent:   {}\n", task.intent.trim()));
        }
        if let Some(base) = &task.base {
            out.push_str(&format!("  base:     {}\n", base.short));
        }
        out.push_str(&format!("  staged:   {}\n", staged_summary(&task.staged)));
        out.push_str(&format!("  findings: {}\n", findings_summary(task)));
        for finding in task.findings.iter().flat_map(|f| f.iter()) {
            out.push_str(&finding_line(finding, false));
        }
        out.push('\n');
        out.push_str(&format!(
            "Run: `jigc start --task {id}`   — resume: re-composes this task's own workflow where it left off\n"
        ));
        out.push_str(&format!(
            "Run: `jigc task validate {id}`   — {}\n",
            crate::gate_coverage::whats_left_coverage()
        ));
        // The commit boundary is the milestone door for a sub-task, and `jigc task
        // finalize <sub>` refuses outright there — so the directive names the door that
        // runs rather than the one that matches the shape of the others.
        match task.milestone.as_deref() {
            Some(milestone) => out.push_str(&format!(
                "Run: `jigc milestone finalize {milestone}`   — validate + commit: this is a sub-task of milestone `{milestone}`, whose door is its only commit boundary — `jigc task finalize {id}` refuses here\n"
            )),
            None => out.push_str(&format!(
                "Run: `jigc task finalize {id}`   — validate + commit\n"
            )),
        }
        // The abandon directive is **B9's disjunction taken per row**: the consent is
        // printed exactly when the door needs it. A task staging a doc no commit has a
        // copy of is refused without `--force` (M50 → the Settle, D1), so omitting it
        // there would print a route this binary's own guard blocks; a task staging
        // nothing is not, so printing it there would train the flag into a reflex over a
        // door that never asked for it.
        if task.staged.is_empty() {
            out.push_str(&format!(
                "Run: `jigc task discard {id}`   — abandon: removes the working area\n"
            ));
        } else {
            out.push_str(&format!(
                "Run: `jigc task discard {id} --force`   — abandon: removes the working area and the doc(s) staged in it, which no commit has a copy of; `--force` is the consent this door refuses without\n"
            ));
        }
    }
    // The catalog, beneath the work in progress: the shipped `create.gate-blocked`
    // refusal routes at "`jigc start` lists the catalog" and can fire ONLY while a task
    // is live, so a view that dropped it here would break that route in exactly the state
    // that prints it — and several open tasks are legal, so the set is still a live
    // affordance rather than noise.
    out.push('\n');
    catalog_block(&mut out, orientation, next_steps);
    out.push_str(ROUTING_FOOTER);
    out
}

/// What the `workflow:` line says for a task whose working area records none — the
/// genuine absent case, never a silent fall-through to the cascade default.
const NO_RECORDED_WORKFLOW: &str = "none recorded";

/// The `staged:` value: the task's staged doc identities, or the stated empty case.
fn staged_summary(staged: &[String]) -> String {
    if staged.is_empty() {
        return String::from("nothing staged yet");
    }
    staged.join(", ")
}

/// The `findings:` tally — by severity, in descending severity order — or the stated
/// **unknown** case carrying why the sweep could not run. *Unknown* is never rendered as
/// *none*: that substitution is the lie this whole variant exists to retire.
fn findings_summary(task: &ActiveTask) -> String {
    let Some(findings) = &task.findings else {
        let reason = task
            .findings_unavailable
            .as_deref()
            .unwrap_or("the task-scope sweep did not run");
        return format!("unknown — {reason}");
    };
    let count = |severity: Severity| findings.iter().filter(|f| f.severity == severity).count();
    let tally: Vec<String> = [
        (Severity::Blocking, "blocking"),
        (Severity::Warning, "warning"),
        (Severity::Advisory, "advisory"),
    ]
    .into_iter()
    .filter_map(|(severity, label)| match count(severity) {
        0 => None,
        n => Some(format!("{n} {label}")),
    })
    .collect();
    if tally.is_empty() {
        return String::from("none");
    }
    tally.join(", ")
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
/// still deferred — each needs a git-HEAD read this view does not make. The premise
/// that once stood here (*"no task store yet"*) has been false since M1 and was the
/// reason a whole orientation state went unbuilt for 49 milestones; the remainder is
/// keyed to its trigger in `implementation/decisions-pending.md` instead of resting on
/// an expired claim about what exists (M50 Increment 5 / T2).
pub fn orientation_clean(
    header: &str,
    orientation: &Orientation,
    next_steps: &[NextStep],
) -> String {
    let mut out = String::from("jigc — orientation\n\n");
    out.push_str(header);
    out.push_str("\n\n");
    catalog_block(&mut out, orientation, next_steps);
    out.push_str(ROUTING_FOOTER);
    out
}

/// The **catalog block** — `Available workflows:` through the off-catalog next-step
/// lines — appended to `out`. One home, two callers: the clean view and the active-task
/// view, which carries the same block beneath its active set because
/// `create.gate-blocked` routes at *"`jigc start` lists the catalog"* and can only fire
/// while a task is live.
fn catalog_block(out: &mut String, orientation: &Orientation, next_steps: &[NextStep]) {
    out.push_str("Available workflows:\n");
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
    // The preview route — a read of a workflow's step text before minting. Gated on a
    // non-empty catalog (there is nothing to preview otherwise, so the omitting context
    // stays inert). Names the M44 `jigc workflow <id> --preview` surface, which shipped
    // with nothing routing to it (findings §69).
    //
    // **Scoped to the workflows the verb serves** (M47 Inc 10 / T6, D1): `--preview` is a
    // mint-free compose of a `creates-task: true` workflow and refuses the rest — correctly,
    // since for a non-minting member the direct run *is* the preview. The bare `<id>` here
    // quantified over the whole catalog while nothing in it marks which members mint, so a
    // reader could not predict which ids the verb would refuse (law 1). The line names the
    // scope and the fallback for everything outside it.
    if !orientation.workflows.entries().is_empty() {
        out.push_str(
            "Preview: `jigc workflow <id> --preview`   — read a task-minting workflow's step text without minting a task; a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either\n",
        );
    }
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
            let also_open = also_open_block(&view.also_open);
            let mut out = String::with_capacity(
                header.len()
                    + text.len()
                    + state.len()
                    + gates.len()
                    + also_open.len()
                    + ROUTING_FOOTER.len()
                    + 1,
            );
            out.push_str(&header);
            out.push_str(text);
            if !text.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&state);
            out.push_str(&gates);
            out.push_str(&also_open);
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a **preview** compose — `jigc workflow <id> --preview` (M44 Inc 3) — the
/// read surface that composes a `creates-task: true` workflow's step text **without
/// minting a task** (`design/surface-contract.md` → law 2: nothing hides; the
/// read-surface spine of `introspection.md` / `doc-read-surface.md`).
///
/// The `json` arm is **byte-identical to [`composed`]'s** — the pinned `{task, text}`
/// contract, with `task: null` (the preview `view.task` is `None`, so nothing is
/// minted and nothing lies — `surface-contract.md` → law 1). The banner is
/// presentation and never joins the contract.
///
/// The `agent` / `human` arm leads with a **mint-first banner** (law 3 — nothing
/// ambushes: the reader learns *before* the body that this composed nothing and how to
/// mint for real), then the identical [`composed`] body. That body already renders no
/// [`minted_header`] (keyed on `view.minted`, `false` here) and no [`task_state_lines`]
/// (keyed on `view.task`, `None` here) — so `composed` alone would already tell no lie;
/// the only thing owed over it is the banner, which frames the body's `--task
/// your-task-id` identity (the `start`-side preview-slug render) as a placeholder for
/// the id a real mint assigns.
pub fn composed_preview(format: Format, view: &Composition, workflow_id: &str) -> String {
    match format {
        // The pinned `{task: null, text}` projection — identical to `composed`'s json
        // arm; the banner is presentation and reaches no tooling consumer.
        Format::Json => json(&view.view),
        Format::Agent | Format::Human => {
            let banner = format!(
                "preview: workflow `{workflow_id}` — no task minted. This shows what it will ask \
                 before you commit to running it.\n\
                 To run it for real: `jigc start --workflow {workflow_id} \"<intent>\"`   — mints \
                 the task and composes this.\n\
                 Below, `--task your-task-id` marks where the minted id goes.\n\n"
            );
            format!("{banner}{}", composed(format, view))
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
/// - `resume:` — the designated recovery after context loss, **keyed on unit kind**
///   (M51 Inc 9 / T1, law 1): a top-level task names `jigc start --task <id>`, whose
///   area was provisioned at its mint; a **milestone sub-task** names the re-entry
///   door `jigc workflow <W> --task <id>` and the worktree it runs in, because that
///   is the door that provisions a sub-task's write-ready area
///   ([`crate::start`]'s `provision_on_first_entry`) and `jigc start --task <sub>`
///   composes over it without provisioning anything;
/// - `what's-left:` — `jigc task validate <id>`, the preview of **part** of the
///   finalize gate: the [`crate::gate_coverage::Tier::Previewed`] members. It says so —
///   the staged set, promotion and the commit itself are decided only at
///   `finalize`, and law 1 forbids the line implying otherwise. That clause is
///   **generated** from [`crate::gate_coverage`] (M46 Inc 6 / T2), not spelled
///   here: eight surfaces state this same split, and a member joining the
///   previewed set has to reach all of them — this one by construction, the rest
///   by the per-token fence;
/// - `task scope:` — the B3 statement: `jigc doc` writes default to the **single**
///   active task, and the explicit `--task <id>` is the override that wins when
///   several are active (`crate::doc`'s task-resolution contract, stated where the
///   state is produced instead of learned from the more-than-one rejection). It also
///   names the deliberate-parallelize affordance (findings §71): several open tasks
///   are **legal**, each addressed by its own `--task` — the single-active default is
///   a convenience, not a one-task cap. The parallel claim is **scoped to what holds**
///   (M47 Inc 8 / T2, law 1, the Inc 4 T4 mold): parallel tasks are carried while their
///   work stays disjoint, and a task whose base moved on **overlapping** paths blocks
///   and names them — the identical `decide_base_repin` decision the resume door named
///   two lines above and `finalize` both make (`design/finalize.md` → Parallel
///   hand-editing). Before Inc 8 the claim was flat, and the resume door it invites the
///   agent back through refused the sequence outright. **Scoped per unit kind**: that
///   divergence clause holds of a *top-level* task only — a **milestone sub-task**'s
///   `jigc task finalize` refuses outright and its one real commit boundary blocks on
///   *any* moved history, so a sub-task compose names the milestone instead
///   ([`Composition::sub_task_of`]). Stating the top-level behaviour there was the
///   second face of Inc 8's N7 finding.
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
    let (resume, divergence) = match view.sub_task_of.as_ref() {
        // A milestone sub-task, on both counts.
        //
        // The **door**: `jigc start --task <sub>` re-composes a sub-task's workflow and
        // provisions nothing, while the sub-task's write-ready `docs/` area is provisioned
        // by the re-entry door on first entry (`crate::start`'s
        // `provision_on_first_entry`; `design/worked-examples.md` → the fan-out, which
        // already states that `jigc workflow <W> --task <sub>` *"provisions its write-ready
        // area on first entry"*). Driven before M51 Inc 9 / T1: from the provisioned
        // worktree the shipped line's own door exits 0 over an area with no `docs/` in it,
        // and the composed body's next write then answered `no staged instance`. So the
        // line names the door that provisions — and **where** it runs, which the `Spawn:`
        // line has always named and this one did not (`design/surface-contract.md` → law 1).
        //
        // The **divergence**: `jigc task finalize <sub>` refuses outright, and the only
        // commit boundary it has blocks on *any* moved history (the sub-agent worktrees
        // were cut from the milestone's pin), so both of its read doors stay blanket-strict
        // too. The top-level clause below would state the opposite of all three.
        Some(sub) => {
            let milestone = &sub.milestone;
            let workflow = &sub.workflow;
            let worktree = engine::milestone::worktree_path(id);
            (
                format!(
                    "resume: `jigc workflow {workflow} --task {id}`   — re-composes this \
                     workflow and provisions this sub-task's write-ready docs area on first \
                     entry; run it from this sub-task's own worktree at `{}`, where its work \
                     happens",
                    worktree.display(),
                ),
                format!(
                    "this task is a sub-task of milestone `{milestone}`, whose `jigc milestone \
                     finalize {milestone}` is its only commit boundary — every door here stays \
                     pinned to the milestone's base"
                ),
            )
        }
        None => (
            format!(
                "resume: `jigc start --task {id}`   — re-composes this workflow if context is lost"
            ),
            "once a sibling task commits a path this one also touches, resuming or \
             finalizing here blocks and names the overlapping paths"
                .to_string(),
        ),
    };
    // The coverage clause is **generated** from the gate-coverage table, never spelled
    // here: this line and seven other surfaces state the same split, and a member that
    // joins the previewed set must reach all eight or none (`crate::gate_coverage`).
    let coverage = crate::gate_coverage::whats_left_coverage();
    format!(
        "{resume}\n\
         what's-left: `jigc task validate {id}`   — {coverage}\n\
         task scope: `jigc doc` writes default to the single active task; `--task {id}` is the explicit override and wins when several are active — several open tasks are legal, each addressed by its own `--task`, so you can run them in parallel while their work stays disjoint; {divergence}\n"
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
///
/// **And it says what a gate is** (M47 Inc 10 / T6). M42 moved the *list* off the
/// refusal; the *noun* stayed on it — no compose, `describe`, or help surface defined
/// `create-gates:`, so a reader who had never tripped `create.gate-blocked` met a bare
/// list of doctype ids. The trailing clause carries the refusal's own two facts (what the
/// gate grants, and that anything else is refused) into the surface that lists them.
///
/// It names the **permission**, not the verb: a `jigc doc create …` fragment here would be
/// a command-shaped string carrying no `--task <id>`, on a surface whose every composed
/// `jigc doc` line is task-disambiguated by contract (`methodology_pack_compose`'s
/// every-line assert). A definition that reads as a runnable command is an invitation to
/// run it, and this one would be missing the argument that decides where the write lands.
fn create_gates_line(gates: &[String]) -> String {
    if gates.is_empty() {
        return String::new();
    }
    format!(
        "create-gates: {}   — the doc-types this task is allowed to create; any other type is refused\n",
        gates.join(", ")
    )
}

/// The `also open:` block a **work-starting** `jigc start` form appends when other tasks
/// were already live when it ran (M50 Increment 5 / T3 — the trial's F-5: both forms that
/// start work were silent about the work already open, and one of them changes the repo).
///
/// One lead line carrying the count and the legality claim once, then **one row per task**
/// — the active *set*, never a singular claim that reports one and hides the rest (Settle
/// B8). Each row names the id, the workflow that minted it, and the door that re-composes
/// it, so the reader can act on the row without a second lookup.
///
/// It states plainly that **this call left them alone**: the block is an addition to what
/// the surface says, never a change to what it did. The mint is not suppressed — the
/// [`task_state_lines`] `task scope:` line just above composes *"several open tasks are
/// legal … you can run them in parallel"* into every task, and a refusal here would
/// contradict it.
///
/// The routes are backticked, not `Run:` directives: a `Run:` here would read as the next
/// step of *this* task and send the agent off to resume a different one.
///
/// An **empty** set renders no bytes at all — omitted, never printed as `none`, the same
/// inert-omitting-context mold as [`create_gates_line`]. That is every context but the two
/// filling forms, plus those two over a repo holding no other live task.
fn also_open_block(open: &[crate::start::OpenTask]) -> String {
    if open.is_empty() {
        return String::new();
    }
    let lead = if open.len() == 1 {
        String::from("1 other task was already open before this call — nothing here touched it")
    } else {
        format!(
            "{} other tasks were already open before this call — nothing here touched them",
            open.len(),
        )
    };
    let mut out = format!(
        "also open: {lead}; several open tasks are legal, each addressed by its own \
         `--task`:\n"
    );
    for task in open {
        let workflow = match &task.workflow {
            Some(id) => format!(" (workflow `{id}`)"),
            None => String::new(),
        };
        out.push_str(&format!(
            "  - `{id}`{workflow} — resume it with `jigc start --task {id}`\n",
            id = task.id,
        ));
    }
    out
}

/// Render the `--explain` [`ResolutionTree`] to the surface `format` selects:
/// `agent` / `human` emit the agent-text tree (the workflow line with its winning
/// layer + pack label, the `overrides applied: N` line, the resolved include list
/// with each step's source layer and any `← replaces … at position` annotation),
/// followed by the routing footer; `json` emits the **generic** JSON projection of
/// the tree with **no** footer (tooling-consumed). The `pack_label`
/// (`<pack-id>/<version>` — the one spelling every pack-naming surface renders, the
/// `collision:` and `Pack input:` lines below included) is CLI-side framing — the
/// engine tree carries only the structural fact (which layer won), not the label
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
///
/// The JSON arm carries that same width as **`checked`** beside the report (M48 Inc 7 T5 —
/// the judgment-tier census's second close; `design/command-output-contract.md` → Evolution
/// posture). It is not re-derivable from `findings[]`: a clean sweep over **zero** recorded
/// deltas and a clean sweep over twelve serialize identically, so a driver reading the
/// envelope could not tell *"nothing to check"* from *"everything checks out"* — the fact the
/// text arm has always distinguished in prose. Additive under the pre-1.0 window, in the
/// shape [`validation_store`]'s `scope` / `report_only` set.
///
/// **The clean line widens with the sweep (M48 Increment 10 / T2).** `upgrade` now also reads
/// the adapter's **owned guide artifact** — reporting, never replacing, because this verb
/// writes nothing (`design/overrides.md` → The `jigc upgrade` command). `guide` is the
/// repo-relative path of the artifact the sweep actually read, or `None` when there is none
/// installed to read; naming it is the same round-2 D4 honesty the delta count carries, and
/// a sweep that widened in silence would understate its own scope. Its value is keyed
/// beside `checked` for the same reason.
pub fn validation_upgrade(
    format: Format,
    report: &ValidationReport,
    checked: usize,
    guide: Option<&str>,
) -> String {
    match format {
        Format::Json => {
            let mut value = serde_json::to_value(report).expect("validation report serializes");
            if let Some(object) = value.as_object_mut() {
                object.insert("checked".to_string(), serde_json::Value::from(checked));
                object.insert("guide".to_string(), serde_json::Value::from(guide));
            }
            json(&value)
        }
        Format::Agent | Format::Human => {
            let mut clean = if checked == 0 {
                "no findings — no recorded config deltas to check against the current pack"
                    .to_string()
            } else {
                format!(
                    "no findings — {checked} recorded config delta(s) re-apply clean against the current pack"
                )
            };
            // Reached only when the report is empty, i.e. when the artifact was read and
            // found to be jigc's own — so the clause states that, not merely "checked".
            if let Some(guide) = guide {
                clean.push_str(&format!(
                    ", and the adapter's guide artifact `{guide}` is still jigc's own"
                ));
            }
            validation_scoped(format, report, &clean, None, None)
        }
    }
}

/// Render a [`ValidationReport`] for the **store-scope** sweep (`jigc validate`): like
/// [`validation`] but task-less, so (a) the clean line is store-scoped — it validates the
/// committed store, not a task, and must not reuse the "the task validates clean" wording
/// (M26 shakedown #10b) — and (b) it carries a **report-only clarification** so the
/// exit-code contract is unambiguous from the output. The store sweep is detect-and-report:
/// content findings are *listed on `blocking · …` lines* (the doc's **cascade** severity —
/// what would gate at `finalize`) yet the run **exits 0**; only the exit-flipping exceptions
/// [`STORE_EXIT_FLIPS`] enumerates go non-zero (`design/validation.md` → Exit semantics).
/// Without a trailer a human eyeballing `blocking`, or a script
/// chaining `jigc validate && deploy`, misreads a report-only store finding as a gate
/// failure. So the agent/human view appends a [`store_trailer`] naming where these findings
/// actually gate, and the JSON adds a machine-readable `report_only` (+ `scope`) signal —
/// the per-finding severity token is left untouched (it is meaningful).
///
/// The JSON carries a third top-level key, **`blocking_probes`** (M47, `design/command-output-contract.md`
/// → The store sweep's envelope): the sorted, de-duplicated probe names of the findings whose
/// resolved severity is `Blocking`, `[]` when none. It exists so a **shell** consumer — the
/// installed warn-only pre-commit hook above all — can key on *severity* without reaching into
/// a finding object: a flat array of plain strings is a `]`-delimited region a bounded match
/// cannot escape, where the findings list is a sequence of nested objects. Store sweep only;
/// [`validation`]'s task-scope envelope is untouched (its exit code already carries the
/// blocking verdict).
pub fn validation_store(
    format: Format,
    report: &ValidationReport,
    unbaselined: &BTreeSet<String>,
) -> String {
    // The exit-non-zero exceptions (`validation.md` → Exit semantics) live in **one**
    // enumerable table, [`STORE_EXIT_FLIPS`] — the axis, not a set of ad-hoc booleans. The
    // first member the report matches (table order **is** precedence) both flips
    // `report_only` false and supplies the closing line; absent a match the report-only
    // branch renders. The exit decision is the shared [`validation_store_exit_flips`] that
    // this renderer's `report_only` field, the trailer, and `run_validate_store`'s exit code
    // all key on, so they stay truthful in lockstep.
    let flip = first_store_exit_flip(report);
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
                    serde_json::Value::Bool(flip.is_none()),
                );
                // Sorted + de-duplicated by construction: a `BTreeSet` over the probe names of
                // the *blocking* findings only. Severity is the engine's resolved (post-cascade)
                // one, so a project that demotes a check demotes it here too.
                let blocking: BTreeSet<&str> = report
                    .findings
                    .iter()
                    .filter(|finding| finding.severity == Severity::Blocking)
                    .map(|finding| finding.probe.as_str())
                    .collect();
                object.insert(
                    "blocking_probes".to_string(),
                    serde_json::Value::Array(
                        blocking
                            .into_iter()
                            .map(|probe| serde_json::Value::String(probe.to_string()))
                            .collect(),
                    ),
                );
            }
            json(&value)
        }
        Format::Agent | Format::Human => {
            let trailer = store_trailer(report, flip, unbaselined);
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

/// One **exit-flipping condition** of the store-scope sweep — a member of the axis
/// `validation.md` → Exit semantics owns, made *enumerable* (M47 completion audit) so every
/// surface that speaks about the class can be checked against **all** of it, not against the
/// member its author had in mind. The axis-iterating form of the complete-fix contract
/// (`implementation/pinning.md`): the preload tier's report-only clause had been narrowed to
/// "the sweep itself could not be trusted", which is false of [`reconciliation.rename`], and
/// the fence that was supposed to hold the clause honest checked one member.
///
/// Each member carries what a fence needs to drive it end-to-end: a matcher, a **witness**
/// finding, the closing line it renders, the words that line names the condition with, and
/// whether the flip means the sweep's own result cannot be trusted. A further member cannot
/// join without supplying all five (the compiler asks), so it joins the fences that iterate
/// this table in the same motion.
///
/// Four of the five members exist **for** the fences (the renderer needs only `matches` and
/// `trailer`) — declared as the axis's testable surface rather than dropped.
///
/// **Public since M48 Inc 7 / T5**, together with the table below: the judgment-tier census
/// derives the `jigc validate` member's entry from *this* registry rather than hand-listing
/// four conditions (`crates/cli/tests/text_json_parity_axis.rs` →
/// `validate_census_entry`), which an integration suite outside the crate can only do
/// against a `pub` surface. The promotion is read-only reach: `matches` stays private, so
/// the table remains the single place a condition can be declared.
pub struct StoreExitFlip {
    /// Stable id — fence diagnostics only, never rendered to a surface.
    pub id: &'static str,
    /// Whether this finding **is** the condition.
    matches: fn(&Finding) -> bool,
    /// A finding that is this condition — what an axis-iterating fence feeds the real
    /// renderer, so no member is testable only in principle.
    ///
    /// It carries its **production target** (M48 Inc 7 T5): the membership seam refuses to
    /// serialize a target-less finding, so a witness without one is drivable through the
    /// agent arm and *never* through `--format json` — testable-in-principle on half the
    /// surface, which is the shape this table exists to prevent.
    pub witness: fn() -> Finding,
    /// The closing line this condition renders instead of the report-only sentence.
    pub trailer: fn() -> String,
    /// The words that closing line names **which** condition fired with — the checkable half
    /// of the preload's promise that the closing line says why.
    pub cause: &'static str,
    /// Whether the flip means *the sweep's own result cannot be trusted*. **False** for a
    /// condition where the sweep worked and is reporting a real event it found
    /// (`reconciliation.rename`) — which is exactly why no surface may state the class as an
    /// untrustworthy sweep while such a member is on the axis.
    pub sweep_untrustworthy: bool,
}

/// The exit-flipping conditions of the store sweep, **in precedence order** — the single
/// source [`validation_store_exit_flips`], the JSON `report_only` field, [`store_trailer`],
/// and [`crate::cli`]'s `run_validate_store` exit code all key on, so they stay truthful in
/// lockstep, and the one place a fence derives the axis from.
///
/// Precedence: probe-unreliability dominates (it taints the whole result); the unmigrated
/// corpus is next (it taints every *content* verdict below it, and clearing it first is right
/// even when an ahead doc coexists). The report-only rule for **content** findings is
/// untouched by any of them — an invalid enum, a malformed date, a dangling ref keep their
/// codes and their exit 0.
///
/// **The "managed arm only" condition is about which *code* fires, not about the exit**
/// (M46 Inc 3 / T1, re-derived where it lives). `SCHEMA_VERSION_CURRENT_CODE` is emitted
/// *only* on the managed arm of the fifth family's discriminator — a foreign squatter at a
/// placement home takes the advisory `schema-conformance.unadopted-instance` instead — so
/// keying on the code **is** that condition, and each home keeps its own closing line and its
/// own route (`jigc migrate-corpus` for the unmigrated managed doc, `jigc ingest` for the file
/// jigc was never handed). What no longer follows from it is the **exit**: the squatter is the
/// last member below, so a stock brownfield repo carrying an un-adopted file at a managed home
/// exits **non-zero**. Reporting it at exit 0 was a green over a home jigc has never been
/// handed — the false all-clear that member exists to retire
/// (`crates/cli/tests/managed_vs_foreign.rs`, the two foreign arms).
///
/// **Public since M48 Inc 7 / T5** — see [`StoreExitFlip`] for why: a census entry derived
/// from the registry cannot fall behind it, where a hand-list can.
pub const STORE_EXIT_FLIPS: &[StoreExitFlip] = &[
    // A `pack-probe-integrity.*` meta-finding: the probe crashed, so the sweep cannot claim
    // a result at all.
    StoreExitFlip {
        id: "probe-unreliable",
        // The witness carries the **production** code and target (M48 Inc 7 T5):
        // `engine::probe::meta_finding` emits `probe-failure` located at the offending probe's
        // id — the pack-resource target form.
        matches: |f| f.probe == "pack-probe-integrity",
        witness: || {
            Finding::graded(
                Severity::Blocking,
                "pack-probe-integrity.probe-failure",
                "the probe exited non-zero without a report",
                Some(engine::finding::Location::addressed("doc-code", 1, 1)),
                Some(Route::human("repair the probe, then re-validate")),
            )
        },
        trailer: probe_unreliable_trailer,
        cause: "pack-probe-integrity finding(s) present",
        sweep_untrustworthy: true,
    },
    // An out-of-band `git mv` (M35): a structural-identity change this commit introduced.
    // The sweep worked — it is reporting a real event — so this member is **not** an
    // untrustworthy sweep, and the class may not be stated as one.
    StoreExitFlip {
        id: "oob-rename",
        matches: |f| f.code == "reconciliation.rename",
        witness: || {
            Finding::graded(
                Severity::Blocking,
                "reconciliation.rename",
                "`decisions/use-sqlite.md` was renamed out of band",
                Some(engine::finding::Location::addressed(
                    "decisions/use-sqlite.md",
                    1,
                    1,
                )),
                Some(Route::human(
                    "revert the `git mv` or adopt it via `jigc rename`",
                )),
            )
        },
        trailer: oob_rename_trailer,
        cause: "out-of-band rename detected",
        sweep_untrustworthy: false,
    },
    // (M42) A version-currency break — a **managed** committed instance below its doctype's
    // manifest version, i.e. an unmigrated corpus, where every other family adjudicated docs
    // against a schema they were never written to.
    StoreExitFlip {
        id: "unmigrated-corpus",
        matches: |f| f.code == engine::validate::SCHEMA_VERSION_CURRENT_CODE,
        witness: || {
            Finding::graded(
                Severity::Blocking,
                engine::validate::SCHEMA_VERSION_CURRENT_CODE,
                "`CHANGELOG.md` is stamped schema-version 1, below the manifest's 2",
                Some(engine::finding::Location::addressed("CHANGELOG.md", 1, 1)),
                Some(Route::human("run `jigc migrate-corpus`, then re-validate")),
            )
        },
        trailer: unmigrated_corpus_trailer,
        cause: "the committed corpus is below its schema-version",
        sweep_untrustworthy: true,
    },
    // (2026-07-24) Its above-current sibling: a future/foreign stamp this build has no schema
    // for, so the sweep could not adjudicate the doc.
    StoreExitFlip {
        id: "ahead-corpus",
        matches: |f| f.code == engine::validate::SCHEMA_VERSION_AHEAD_CODE,
        witness: || {
            Finding::graded(
                Severity::Blocking,
                engine::validate::SCHEMA_VERSION_AHEAD_CODE,
                "`CHANGELOG.md` is stamped schema-version 3, above this build's 2",
                Some(engine::finding::Location::addressed("CHANGELOG.md", 1, 1)),
                Some(Route::human(
                    "upgrade jigc, or restore the stamp from git history",
                )),
            )
        },
        trailer: ahead_corpus_trailer,
        cause: AHEAD_STAMP_PHRASE,
        sweep_untrustworthy: true,
    },
    // (M51 Inc 8 / T3) A committed doc jigc stamped that no resolved doctype claims — the
    // pack that defined its type left the composition, or that type is composed and homed
    // somewhere this path is not. Either way the sweep never read those files against a
    // schema, so it refuses the green rather than reporting that it found nothing in them.
    // (M52 Inc 8 / T4: the second cause is why neither this comment nor the trailer may say
    // *no schema in the composed set* — driven, the schema was composed and `jigc ingest`
    // named it at the same commit.)
    StoreExitFlip {
        id: "orphaned-instance",
        matches: |f| f.code == crate::orphan::ORPHANED_INSTANCE_CODE,
        // The witness IS the producer (M51 Inc 8 / T3): a hand-built copy of the production
        // finding is a second place for the code, the target form and the route to drift, and
        // this member's whole claim is that the three cannot.
        witness: || crate::orphan::orphaned_instance_finding("docs/roadmap.md"),
        trailer: orphaned_instance_trailer,
        cause: ORPHANED_INSTANCE_CAUSE,
        sweep_untrustworthy: true,
    },
    // (M52 Inc 7 / T6) A **declared home the repository committed into that is now empty** —
    // a resolved doctype's exact declared path whose history is non-empty while no committed
    // instance is there. Like the squatter below it the sweep **worked**: it read the schema's
    // declared home, the committed census and `git log`, and is reporting a standing fact about
    // the corpus rather than an inability to adjudicate — so `sweep_untrustworthy: false`, and
    // nothing may state this class as a sweep that failed.
    //
    // **Its position is a decision, and the reason is the reader's, not the appender's.** Both
    // this and the squatter are standing facts about the corpus, so the table's stated ordering
    // rule (what taints the result · what this commit introduced · standing facts) does not
    // separate them; what does is which closing line a reader is handed when both fire — and
    // this one names a managed document the repository's own history says it *had* and no
    // longer has, where the squatter names a file jigc was never handed. **Loss outranks
    // non-adoption**, so this member precedes it. Both findings are on the report either way,
    // each with its own route, so the precedence buys only the closing sentence — which is why
    // it is decided on that and driven
    // (`crates/cli/tests/home_vacated.rs` → `a_vacated_home_closes_the_report_ahead_of_a_never_adopted_file`).
    StoreExitFlip {
        id: "home-vacated",
        matches: |f| f.code == crate::orphan::HOME_VACATED_CODE,
        // The witness IS the producer (`orphaned-instance`'s precedent): a hand-built copy of
        // the production finding is a second place for the code, the target form and the route
        // to drift, and this member's whole claim is that they cannot.
        witness: || {
            crate::orphan::home_vacated_finding(&crate::orphan::FixedHome {
                ty: "changelog".to_string(),
                path: "CHANGELOG.md".to_string(),
            })
        },
        trailer: home_vacated_trailer,
        cause: HOME_VACATED_CAUSE,
        sweep_untrustworthy: false,
    },
    // (M46 Inc 3 / T1) The **foreign** arm of that same discriminator: a committed file at a
    // managed doctype's home that jigc was never handed. Last in precedence — every condition
    // above it either taints the sweep's own result, is a change this commit introduced, or is
    // the standing fact that outranks this one (a managed document the store has lost). Like
    // `reconciliation.rename` the sweep **worked**: it found the file and named it foreign, so
    // this member is not an untrustworthy sweep either.
    StoreExitFlip {
        id: "foreign-squatter",
        matches: |f| f.code == UNADOPTED_INSTANCE_CODE,
        witness: || {
            Finding::graded(
                Severity::Advisory,
                UNADOPTED_INSTANCE_CODE,
                "committed file `CHANGELOG.md` sits at the `changelog` home but was never \
                 adopted by jigc",
                // The **file-path** target, which is the production form: a foreign file has
                // no managed identity to claim (`engine::validate::unadopted_instance`).
                Some(engine::finding::Location::addressed("CHANGELOG.md", 1, 1)),
                Some(Route::human(engine::validate::adoption_route(
                    "changelog",
                    "CHANGELOG.md",
                    true,
                ))),
            )
        },
        trailer: unadopted_squatter_trailer,
        cause: UNADOPTED_SQUATTER_CAUSE,
        sweep_untrustworthy: false,
    },
];

/// The first [`STORE_EXIT_FLIPS`] member this report matches — table order **is** precedence
/// — or `None` when the sweep stays report-only.
pub(crate) fn first_store_exit_flip(report: &ValidationReport) -> Option<&'static StoreExitFlip> {
    STORE_EXIT_FLIPS
        .iter()
        .find(|flip| report.findings.iter().any(|f| (flip.matches)(f)))
}

/// Whether the store-scope sweep's exit flips non-zero — the report-only stance's exceptions
/// (`validation.md` → Exit semantics), asked of the [`STORE_EXIT_FLIPS`] axis rather than of a
/// hand-repeated disjunction, so the predicate and every fence over it read the same set.
pub(crate) fn validation_store_exit_flips(report: &ValidationReport) -> bool {
    first_store_exit_flip(report).is_some()
}

/// The **store-scope-only** check ids — the findings that gate **nowhere**, and which the
/// report-only trailer must therefore never claim a task-scope gate for (`validation.md` →
/// The trailer must not claim a gate that does not exist, M42).
///
/// **Derived from the emit sites, not from prose — and the derivation is *"no gate blocks on
/// it"*, not *"no task-scope path emits it"*.** That distinction was inert until M48 Inc 4 / T1,
/// when `schema-conformance.unadopted-instance` gained a task-scope producer while staying a
/// member of this list; stating the old, narrower derivation would now be a false claim about a
/// code that still belongs here. For each code below, walk its producers and confirm that none
/// of them can reach a **blocking** verdict at `jigc task validate`, the `finalize` preflight or
/// the milestone-boundary gate:
///
/// - `schema-conformance.{mention-resolves, repeatable-populated, surplus-sections-absent}` and
///   `schema-completeness.inverse-cardinality` — the store-only families of
///   `validate_store_families` (`mention_resolves_store`, `hollow_surplus_store`,
///   `inverse_cardinality_store`), with **no** task-scope producer at all; completeness and
///   in-prose mentions depend on *other* tasks, so they are by design never a per-task gate.
/// - `schema-conformance.unadopted-instance` — the adoption advisory, with **two** producers
///   since M48 Inc 4 / T1: `schema_conformance_store`'s foreign arm and
///   `file_state::reconcile_committed`'s `UNKNOWN` + non-conformant arm (reached by
///   [`engine::validate::validate_task`], i.e. by every task-scope door). Both mint it
///   **advisory** through the one `unadopted_instance` producer, and the task-scope doors gate
///   on `has_blocking()` — so a foreign squatter is reported at those doors and blocks at none
///   of them. Membership therefore stands on the emit sites, not on their absence.
/// - `file-state.un-baselined` — the read-only committed-store twin's UNKNOWN outcome
///   (`file_state::detect_committed_store`); the task path *adopts* a baseline instead
///   (`file-state.baseline-adopt`), so this code never reaches a gate.
/// - `file-state.{orphaned-doc, unregistered-doc}` and `store-version.binary-mismatch` — the
///   CLI-minted store advisories (`crate::cli`'s orphan tiers, `crate::setup`), un-keyed and
///   store-scope by construction.
/// - `schema-conformance.orphaned-instance` — their blocking sibling (M51 Inc 8 / T3),
///   minted by `crate::cli`'s store sweep alone from `crate::orphan::orphaned_instances`. No
///   task-scope door can reach it: a task addresses docs **through** their doctype, and this
///   condition is that the doctype resolves to nothing. Membership is load-bearing rather
///   than tidy — the code is `schema-conformance.*` and located at a path, so without the row
///   the per-finding gate label would claim a `finalize` gate that does not exist.
///
/// - `schema-conformance.home-vacated` — the vacated-declared-home break (M52 Inc 7 / T5),
///   minted by `crate::cli`'s store sweep alone from `crate::orphan::vacated_homes`. Its
///   subject is a **home**, not a doc: the condition is that nothing is at the path, so there
///   is no document for a task to address and no task-scope door that could raise it.
///   Membership is load-bearing on the same footing as its sibling above — the code is
///   `schema-conformance.*` and located at a path, so without the row the per-finding gate
///   label would claim a `finalize` gate that does not exist.
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
    crate::orphan::ORPHANED_INSTANCE_CODE,
    crate::orphan::HOME_VACATED_CODE,
    "schema-conformance.mention-resolves",
    "schema-conformance.repeatable-populated",
    "schema-conformance.surplus-sections-absent",
    UNADOPTED_INSTANCE_CODE,
    "schema-completeness.inverse-cardinality",
    "file-state.un-baselined",
    "file-state.orphaned-doc",
    "file-state.unregistered-doc",
    "store-version.binary-mismatch",
    engine::validate::SCHEMA_VERSION_CURRENT_CODE,
    // The above-current sibling (2026-07-24): emitted only by the store sweep's fifth
    // family, exactly like the below-version break — no task-scope path can mint it, so
    // the per-finding gate label must never claim a finalize gate for it.
    engine::validate::SCHEMA_VERSION_AHEAD_CODE,
    // The M51 resolved-half advisory: emitted by the store sweep's fifth family alone (a
    // doctype's manifest entry is a pack fact the task doors never adjudicate), so the
    // per-finding gate label must claim no boundary gate for it either.
    engine::validate::UNVERSIONED_DOCTYPE_CODE,
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
///      recorded*), or — where the managed-vs-foreign discriminator adjudicates the file
///      **never adopted** (M48 Inc 4 / T1) — the advisory `unadopted-instance`, which this
///      condition never has to reach because [`GATES_NOWHERE`] already carries it. Nothing
///      gates on either, ever. The claim is a lie.
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
/// on its own tells the reader nothing about whether anything ever *stops* on it. Three
/// conditions, all necessary:
///
/// 1. **A gate exists for the finding at all** — [`gates_at_task`], the trailer's own criterion
///    (the store-scope-only codes, and the un-baselined-committed-doc discriminator).
/// 2. **The finding's cascade severity is `blocking`.** A `warning`/`advisory` finding is surfaced
///    at the task boundary but never blocks the transaction, so labelling it *"gates at finalize"*
///    would be exactly the falsehood this label exists to retire (the cascade can demote any
///    non-intrinsic check — `flow13_contract_and_severity.rs` demotes a `doc-code` break to
///    `warning`, and that demoted row must claim no gate). This is N10's first arm (M47 inc-8 /
///    T4): the trailer counted with [`gates_at_task`], which never consults severity, so an
///    advisory-by-default `doc-code.title-names-symbol` printed the row `advisory · …` and the
///    trailer *"these gate at …"* — the row and the trailer disagreeing in one report.
/// 3. **The finding's gate is a *boundary* gate** — not [`gates_at_compose`]. A `workflow-refs.*`
///    break has a real, hard gate, but it is **compose**: its emit sites are the store sweep's
///    family 2 and `crate::start`, and `engine::validate::validate_task` — the shared body of
///    `jigc task validate`, finalize's preflight, **and** the milestone-boundary gate — never runs
///    the workflow↔refs family at all. Labelling it *"gates at finalize"* is the same lie one
///    family over.
fn gates_at_finalize(finding: &Finding, unbaselined: &BTreeSet<String>) -> bool {
    matches!(finding.severity, Severity::Blocking)
        && !gates_at_compose(finding)
        && gates_at_task(finding, unbaselined)
}

/// Every door the shared `engine::validate::validate_task` entry stands behind — the two task
/// doors (`jigc task validate`, the `finalize` preflight) **and** the milestone-boundary gate
/// (`crate::milestone`'s `milestone_boundary_gate`, which drives that same entry over the merged
/// effective state under the same cascade and blocks exit 3). N10's second arm (M47 inc-8 / T4):
/// the trailer named only the two task doors while the milestone door gates on the same findings,
/// so a fan-out operator was told the boundary they were actually standing at could not stop them.
/// Each door as its **argv path** (the `jigc`-less leaf-verb path `crate::cli::VERB_KINDS`
/// keys on), so a consumer can *drive* a door rather than only name it: the M49 Inc 8 / T3
/// route-followability axis crosses these doors with
/// `engine::validate::conformance_repair_codes` and runs every emitted repair route at every
/// one of them. Rendered for prose by [`boundary_doors_phrase`], which reproduces the joined
/// literal this replaced byte-for-byte.
pub const BOUNDARY_DOORS: &[&[&str]] = &[
    &["task", "validate"],
    &["task", "finalize"],
    &["milestone", "finalize"],
];

/// [`BOUNDARY_DOORS`] as the trailer's prose — `` `jigc <door>` `` joined by `" / "`, the
/// exact bytes the flat constant carried before the registry replaced it.
pub(crate) fn boundary_doors_phrase() -> String {
    BOUNDARY_DOORS
        .iter()
        .map(|door| format!("`jigc {}`", door.join(" ")))
        .collect::<Vec<_>>()
        .join(" / ")
}

/// What a `finalize.*` finding's `key.target` names — the **subject split** that is the whole
/// point of the family (`design/command-output-contract.md` → The `finalize.*` family).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinalizeSubject {
    /// The **work unit**: `task:<id>` / `milestone:<id>`, built by `engine::finalize`'s
    /// `Unit::location` (or [`crate::task`]'s `work_unit_location` for the two CLI members).
    WorkUnit,
    /// A **file**: the path, built by `engine::finalize`'s `file_location`. A promote
    /// destination or a carried path may be foreign, with no URI identity — the `file-state`
    /// reason exactly.
    FilePath,
}

impl FinalizeSubject {
    /// The subject's name as the contract's `target` column spells it — so prose and table
    /// are rendered from the same value rather than re-worded per home.
    #[must_use]
    pub fn target_form(self) -> &'static str {
        match self {
            FinalizeSubject::WorkUnit => "the work-unit ref",
            FinalizeSubject::FilePath => "the file path",
        }
    }
}

/// One member of the **`finalize.*` family** — a blocked-finalize finding code, its producing
/// module, and the target form it keys at.
///
/// **The membership predicate, stated once and derived from nowhere else:** a code is a member
/// **iff a production (non-`#[cfg(test)]`) constructor mints it as a [`Finding`] whose code lies
/// in the `finalize.` namespace.** Everything else that *looks* like a member — a cascade knob
/// key, an invocation-log error identity, a declared contract identifier no producer mints — is
/// enumerated with its reason in [`FINALIZE_NON_MEMBERS`] instead.
///
/// **Why a table and not a count.** Four homes carried four different numerals for this family
/// (`design/command-output-contract.md` said *nine* three times and *eleven* once,
/// `implementation/roadmap.md` said *twelve*), and every one of them was wrong — including the
/// paragraph that had already written down the reason (*a census keyed on where you expect the
/// members to live will miss the ones that live somewhere else*) and then re-committed the error
/// by publishing a corrected count. The settled rule is `design/validation.md` → *Exit semantics*:
/// **the table is the enumeration; do not restate its size.** This is that table, on the
/// [`STORE_EXIT_FLIPS`] / [`BOUNDARY_DOORS`] idiom, and
/// `crates/cli/tests/finalize_family_registry.rs` holds it to the predicate above by scanning
/// production source — so a new producer that does not join this table reddens, wherever it lives.
///
/// **The predicate, not the prose, decides — and it moved a member at M52.**
/// `finalize.commit-rejected` was enumerated in [`FINALIZE_NON_MEMBERS`] on the reason *"an
/// `anyhow` path, never a `Finding`"*; M52 Increment 1 / T1 routes a reject that carries a
/// finding onto the findings envelope, so a production constructor now mints it as one and it
/// is a member here. Nothing about the log changed: the same string is still the door's
/// route-exempt error identity in [`crate::invocation_log::ERROR_CODE_REGISTRY`].
///
/// The M49 sweep the predicate found, which no count included: **`finalize.milestone-sub-task`**
/// lives in `engine::milestone`, in no doc row and inside no numeral — the third crate-or-file the
/// file-scoped census did not think to look in. And the contract's sub-table carried a row for
/// `finalize.forward-ref-dangling`, which **no production producer mints at all**: it survives
/// only as a unit-test fixture in `engine::finding`, so it was a member of the count and of no
/// code path.
pub struct FinalizeCode {
    /// The finding code, verbatim.
    pub code: &'static str,
    /// The module whose production source mints it — `<crate>::<module>`, checked against the
    /// scan so a member that moves crates cannot keep a stale home here.
    pub producer: &'static str,
    /// The target form it keys at.
    pub subject: FinalizeSubject,
    /// What that subject *is*, in the words the contract's sub-table uses.
    pub subject_note: &'static str,
}

/// The family, sorted by `code` (the sort is asserted, so the table has one order and a
/// diff over it reads as a membership change rather than a reshuffle).
pub const FINALIZE_FAMILY: &[FinalizeCode] = &[
    FinalizeCode {
        code: "finalize.base-mismatch",
        producer: "engine::finalize",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit's base pin — two constructors (pin diverged, overlap), \
                       mutually exclusive, so one instance per finalize",
    },
    FinalizeCode {
        code: "finalize.carried-staged",
        producer: "engine::finalize",
        subject: FinalizeSubject::FilePath,
        subject_note: "the pre-task staged file (add, modify or deletion) the whole-index \
                       commit would silently absorb — one finding per carried path",
    },
    FinalizeCode {
        code: "finalize.commit-rejected",
        producer: "cli::render",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit whose commit a hook — or git itself — refused; one \
                       refusal per run, so the work-unit ref alone keys it",
    },
    FinalizeCode {
        code: "finalize.empty-commit",
        producer: "engine::finalize",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit, whose staged set produced no diff",
    },
    FinalizeCode {
        code: "finalize.migration-no-replacement",
        producer: "engine::finalize",
        subject: FinalizeSubject::FilePath,
        subject_note: "the foreign source file the migration would retire with nothing to \
                       put in its place",
    },
    FinalizeCode {
        code: "finalize.milestone-sub-task",
        producer: "engine::milestone",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the sub-task whose own finalize would land a commit outside the \
                       milestone's one commit boundary",
    },
    FinalizeCode {
        code: "finalize.no-task",
        producer: "engine::finalize",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit whose task directory is absent",
    },
    FinalizeCode {
        code: "finalize.nothing-staged",
        producer: "cli::task",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit — the tree is dirty and the narrowed index is empty",
    },
    FinalizeCode {
        code: "finalize.promote-clobber",
        producer: "engine::finalize",
        subject: FinalizeSubject::FilePath,
        subject_note: "the destination file it refused to overwrite, which may be foreign \
                       and carry no URI identity",
    },
    FinalizeCode {
        code: "finalize.promote-io",
        producer: "engine::finalize",
        subject: FinalizeSubject::FilePath,
        subject_note: "the staged doc it could not read",
    },
    FinalizeCode {
        code: "finalize.provenance-io",
        producer: "engine::finalize",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit whose provenance manifest could not be read",
    },
    FinalizeCode {
        code: "finalize.render-io",
        producer: "engine::finalize",
        subject: FinalizeSubject::FilePath,
        subject_note: "the staged commit doc",
    },
    FinalizeCode {
        code: "finalize.retire-untrackable",
        producer: "cli::task",
        subject: FinalizeSubject::FilePath,
        subject_note: "the recorded migration source the sink refused to unlink — a path this \
                       repository cannot record or recover, so it may carry no repo-relative \
                       spelling at all",
    },
    FinalizeCode {
        code: "finalize.rollback-conflict",
        producer: "cli::rollback",
        subject: FinalizeSubject::FilePath,
        subject_note: "the config-layer file the transaction rewrote and could not put back — \
                       its bytes changed while the finalize was running, so one finding per \
                       raced path, never one `(code, null)` for the rollback",
    },
    FinalizeCode {
        code: "finalize.source-path-io",
        producer: "engine::finalize",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit whose recorded source path could not be read",
    },
    FinalizeCode {
        code: "finalize.stage-failed",
        producer: "cli::task",
        subject: FinalizeSubject::WorkUnit,
        subject_note: "the work unit whose own stage phase git rejected",
    },
];

/// The identifiers in the `finalize.` namespace that are **not** family members, each with the
/// reason it is not one. Without this list the predicate reads as a namespace — and a namespace
/// is exactly what let a knob key and a declared-but-unminted contract identifier be counted as
/// codes (the M49 Increment 8 lesson: *an exemption is an enumeration, not a namespace prefix*).
///
/// A member of this list that ever gains a production `Finding` producer must move into
/// [`FINALIZE_FAMILY`]; the registry suite asserts that in both directions — **and it has
/// happened**: `finalize.commit-rejected` sat here for three waves as *"an invocation-log error
/// identity … an `anyhow` path, never a `Finding`"*, which M52 Increment 1 / T1 made false by
/// emitting a reject that carries a finding on the findings envelope. It moved rather than
/// having its reason reworded, which is the direction this pair of tables exists to make cheap.
pub const FINALIZE_NON_MEMBERS: &[(&str, &str)] = &[
    (
        "finalize.fan-out",
        "a cascade **knob-key namespace** (`finalize.fan-out.squash`), not a finding code at all",
    ),
    (
        "finalize.forward-ref-dangling",
        "named as a member by the contract's sub-table for two waves and minted by **no** \
         production producer at all — it survives only as a unit-test fixture in \
         `engine::finding`, so it was a member of the count and of no code path (M49 \
         Increment 11 / T11). Declared here rather than merely deleted, so the record can \
         name it and so re-adding it as a member is a change to this table",
    ),
    (
        "finalize.left-out",
        "a declared **contract identifier** only (`crate::pack`'s ambush-class set): the M42 \
         settle chose print-over-refuse, so the staging contract's sole production surface is \
         the pre/post-commit left-out print and no producer mints it as a `Finding`",
    ),
];

/// The door a [`gates_at_compose`] finding really has — the compose gate `jigc start` runs before
/// it hands a task its steps.
const COMPOSE_DOOR: &str = "`jigc start`";

/// Whether this store-scope finding's gate is **compose**, never the task/milestone boundary
/// (M47 inc-8 / T4). One family qualifies: `workflow-refs.*`, whose checks run at
/// [`crate::start`]'s compose gate and in the store sweep's family 2
/// (`engine::validate::validate_store_families`) — and nowhere else. Derived from the emit sites,
/// per [`GATES_NOWHERE`]'s own rule: `engine::validate::validate_task` runs staged-instance
/// conformance, the committed-store reconciler, `ref_resolves`, and `doc-code` — no workflow↔refs
/// pass — so neither task door nor `milestone_boundary_gate` (which drives that same entry) can
/// ever see one. It is **not** a [`GATES_NOWHERE`] member: a gate does exist for it, so the claim
/// is scoped to the right door rather than withdrawn.
fn gates_at_compose(finding: &Finding) -> bool {
    finding.code.starts_with("workflow-refs.")
}

/// The wording of the **above-current stamp** condition, shared by the two surfaces that
/// state it (M47 T1 — RC-alpha4 A3). [`ahead_corpus_trailer`] interpolates it at the
/// point of contradiction, and `adapter.rs`'s `BOOTSTRAP_OUTPUT_CONTRACT` states it in the
/// preload tier as the named exception to the store sweep's report-only stance; the
/// adapter's `bootstrap_names_the_ahead_exception` asserts **both** against this constant
/// (the same statement-==-constant idiom the exit-code line already uses), so the preloaded
/// contract an agent trusts without re-checking cannot drift from the trailer the sweep
/// actually prints — it had, for one wave: the preload claimed exit 0 unconditionally while
/// the trailer said "exits non-zero".
pub(crate) const AHEAD_STAMP_PHRASE: &str = "stamped above this build's schema-version";

/// The words **every** [`STORE_EXIT_FLIPS`] member's closing line states the flipped exit in
/// (M47 completion audit) — the one observable the whole axis shares, and therefore the only
/// honest thing a preloaded surface can promise about the class without enumerating it: *then
/// it `exits non-zero` and its closing line says which condition and why*. Interpolated into
/// each trailer below, so a reworded trailer cannot silently stop delivering the promise.
pub(crate) const STORE_EXIT_FLIP_PHRASE: &str = "exits non-zero";

/// The store trailer's above-current-stamp sentence (2026-07-24), composed from
/// [`AHEAD_STAMP_PHRASE`] so the condition has one source and the preload tier can be
/// asserted to agree with it. Byte-identical to the literal it replaced.
pub(crate) fn ahead_corpus_trailer() -> String {
    format!(
        "a committed doc is {AHEAD_STAMP_PHRASE} — it was written to a schema this jigc \
         build does not know, so the sweep could not adjudicate it and {STORE_EXIT_FLIP_PHRASE}; \
         upgrade jigc, or restore the stamp from git history, then re-validate.\n"
    )
}

/// The store trailer for a crashed pack probe: the sweep could not complete, so it claims no
/// result. Byte-identical to the literal it replaced.
pub(crate) fn probe_unreliable_trailer() -> String {
    format!(
        "pack-probe-integrity finding(s) present — the sweep could not complete and \
         {STORE_EXIT_FLIP_PHRASE}; the store result is not trustworthy.\n"
    )
}

/// The store trailer for an out-of-band rename (M35). Note what it says: the sweep **worked**
/// and is reporting a real structural-identity change — the axis member that is *not* an
/// untrustworthy sweep. Byte-identical to the literal it replaced.
pub(crate) fn oob_rename_trailer() -> String {
    format!(
        "out-of-band rename detected — a structural-identity change this commit introduced; \
         the sweep {STORE_EXIT_FLIP_PHRASE} (revert the `git mv` or adopt it via \
         `jigc rename`).\n"
    )
}

/// The store trailer for a version-currency break (M42): the corpus is unmigrated, so every
/// other finding above was adjudicated against the wrong schema. Names `jigc migrate-corpus`,
/// the verb that clears it. Byte-identical to the literal it replaced.
pub(crate) fn unmigrated_corpus_trailer() -> String {
    format!(
        "the committed corpus is below its schema-version — every other finding above was \
         adjudicated against a schema those docs were never written to, so the sweep \
         {STORE_EXIT_FLIP_PHRASE}; run `jigc migrate-corpus`, then re-validate.\n"
    )
}

/// The check id of the **adoption advisory** — the finding that *is* the foreign-squatter
/// condition ([`STORE_EXIT_FLIPS`]), named once so the matcher, the witness and
/// [`GATES_NOWHERE`] cannot drift apart on a string.
pub(crate) const UNADOPTED_INSTANCE_CODE: &str = "schema-conformance.unadopted-instance";

/// The words the foreign-squatter closing line names its condition with — deliberately **not**
/// a substring of the finding's own message, so a fence asserting the trailer says this is
/// driving the trailer and not the finding line above it.
pub(crate) const UNADOPTED_SQUATTER_CAUSE: &str = "a never-adopted file sits at a managed home";

/// The words the orphaned-instance closing line names its condition with — deliberately
/// **not** a substring of the finding's own message, so a fence asserting the trailer says
/// this is driving the trailer and not the finding line above it.
pub(crate) const ORPHANED_INSTANCE_CAUSE: &str =
    "a stamped committed doc is claimed by no resolved doctype";

/// The store trailer for committed instances no resolved doctype claims (M51 Inc 8 / T3).
/// Like the two version-stamp members it *is* an untrustworthy sweep — the sweep reads a
/// doctype's instances at that doctype's home, so a path no doctype claims was never read
/// against a schema and every other verdict about it is an absence rather than a result — and
/// the closing line says so instead of letting a green stand for *"I stopped looking at these
/// files"*.
///
/// **What it may not say** (M52 Inc 8 / T4): that no schema in the composed set knows those
/// files. That was this line's wording and its finding's, and it is false in the
/// relocated-home cell — so the line names the claim the producer computed and hands the
/// reader to the door that tells the two causes apart, exactly as the finding's own route does.
pub(crate) fn orphaned_instance_trailer() -> String {
    format!(
        "{ORPHANED_INSTANCE_CAUSE} — no resolved doctype claims those paths, so the sweep \
         never read them against a schema and {STORE_EXIT_FLIP_PHRASE}; ask `jigc ingest` \
         which of them a resolved schema still accepts, restore what claims the rest (re-add \
         the pack that defines the type), or follow each finding's own route above, then \
         re-validate.\n"
    )
}

/// The words the vacated-home closing line names its condition with — deliberately **not** a
/// substring of the finding's own message, so a fence asserting the trailer says this is
/// driving the trailer and not the finding line above it.
pub(crate) const HOME_VACATED_CAUSE: &str =
    "a declared home the repository committed into is empty";

/// The store trailer for a declared home the repository committed into and then emptied (M52
/// Inc 7 / T6). Like the squatter's, it says nothing about an untrustworthy sweep: this sweep
/// **worked** — it read the declared home, the committed census and the repository's history,
/// and what it refuses is a green over a managed document the store has lost. The repair is
/// per-home, so the line hands the reader on to each finding's own route rather than naming one
/// verb: a locator, a restore and a re-register, which `crate::orphan::home_vacated_finding`
/// composes against the path that actually went.
pub(crate) fn home_vacated_trailer() -> String {
    format!(
        "{HOME_VACATED_CAUSE} — the repository's history says that home held a document and \
         nothing is there now, so the sweep {STORE_EXIT_FLIP_PHRASE} rather than report a green \
         over a managed document the store has lost; restore it at the declared home (each \
         finding above carries the locator for where it went), then re-validate.\n"
    )
}

/// The store trailer for a never-adopted file at a managed home (M46 Inc 3 / T1). Note what it
/// does **not** say: nothing about an untrustworthy sweep. This sweep worked — it found the
/// file and named it foreign; what it refuses is to report a green over a document jigc has
/// never been handed. Routes at the adoption front door, the same one every finding carries.
pub(crate) fn unadopted_squatter_trailer() -> String {
    format!(
        "{UNADOPTED_SQUATTER_CAUSE} — jigc was never handed it, so the sweep \
         {STORE_EXIT_FLIP_PHRASE} rather than report a green over a document it has never \
         seen; run `jigc ingest` to route it (each finding above carries its own route), then \
         re-validate.\n"
    )
}

/// The store-scope clarifying trailer appended after the findings (`jigc validate`), so
/// exit-0-with-`blocking`-findings is unambiguous. Two shapes: an **exit-flipping**
/// condition renders its own closing line — the [`STORE_EXIT_FLIPS`] member `flip` names,
/// which owns both the precedence and the wording (`validation.md` → Exit semantics) —
/// otherwise the content findings are **report-only** at store scope (exit 0) and the line
/// names where they actually gate. Ends with a newline so the caller appends the routing
/// footer on its own line.
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
///
/// **N10 (M47 inc-8 / T4) — the count is by severity, and the door list is complete and true.**
/// Two corrections, both on the same claim:
///
/// - the count moves from [`gates_at_task`] to [`gates_at_finalize`], so an **advisory** finding
///   is never counted as gating (the shipped trailer counted by probe family: a sweep whose only
///   finding was the advisory-by-default `doc-code.title-names-symbol` printed `advisory · …` and
///   then *"these gate at …"*). The row label and the trailer now share one predicate outright;
/// - the door list names **`jigc milestone finalize`** beside the two task doors —
///   `crate::milestone`'s boundary gate drives the *same* `engine::validate::validate_task` entry
///   under the same cascade and blocks exit 3 on the same families — and a **compose**-gated
///   family ([`gates_at_compose`]) gets its own count naming `jigc start`, rather than riding a
///   boundary claim that is false for it.
///
/// The all/none/mixed shapes are unchanged; the mixed sentence gains a compose clause only when a
/// compose-gated finding is present, so a store without one renders exactly as before (modulo the
/// door list).
fn store_trailer(
    report: &ValidationReport,
    flip: Option<&'static StoreExitFlip>,
    unbaselined: &BTreeSet<String>,
) -> String {
    if let Some(flip) = flip {
        (flip.trailer)()
    } else {
        let n = report.findings.len();
        let gating = report
            .findings
            .iter()
            .filter(|f| gates_at_finalize(f, unbaselined))
            .count();
        let compose = report
            .findings
            .iter()
            .filter(|f| matches!(f.severity, Severity::Blocking) && gates_at_compose(f))
            .count();
        let claim = if gating == n {
            format!("these gate at {}.", boundary_doors_phrase())
        } else if compose == n {
            format!(
                "these gate at compose ({COMPOSE_DOOR}), never at the task or milestone boundary."
            )
        } else if gating == 0 && compose == 0 {
            "each gates nowhere — a store-scope advisory, actionable through its own route above."
                .to_string()
        } else {
            let mut clauses = Vec::new();
            if gating > 0 {
                clauses.push(format!(
                    "{gating} of them gate at {}",
                    boundary_doors_phrase()
                ));
            }
            if compose > 0 {
                clauses.push(format!(
                    "{compose} of them gate at compose ({COMPOSE_DOOR})"
                ));
            }
            if n - gating - compose > 0 {
                clauses.push("the rest are store-scope advisories that gate nowhere".to_string());
            }
            format!(
                "{} — follow each finding's route above.",
                clauses.join("; ")
            )
        };
        format!("{n} finding(s) — report-only at store scope (exit 0); {claim}\n")
    }
}

/// The store view's one-line lead when an `advisory` row is among the findings (M47 Inc 10 /
/// T6 — RC-alpha4 D12): **the severity label is this scope's grading, not a verdict
/// everywhere.** The trial read two identically-labelled `advisory ·` rows in one report and
/// could not tell the never-gates one from the gates-later one; M47 inc-8 corrected the
/// trailer's *count*, and this is the residue on the label itself. It leads the rows rather
/// than following them (law 3 — the reader meets the qualifier before the labels it qualifies)
/// and hands the reader on to [`store_trailer`], the surface that does carry gate information.
///
/// Keyed on an `advisory` row being present: that is the label whose scope-relativity the
/// finding is about, so a report without one renders no bytes (the omitting-context floor),
/// and the **task** view never renders it — there the severity token *is* the verdict for the
/// transaction in hand, so this note would be false rather than merely noisy.
///
/// It quotes **neither a command nor the `(gates at finalize)` marker**. The doors belong to
/// the trailer's own claim, which states them only where a gate exists; and the marker is a
/// *per-row* label — printing its literal in a lead the sweep emits over reports that carry
/// no marked row would re-mint, one register up, the false-gate-claim class M42 closed (and
/// would defeat the whole-stdout proxy `validate_leaves_a_gateless_store_finding_unlabelled`
/// pins that closure with).
const SCOPE_RELATIVE_NOTE: &str = "note: severity is scope-relative — `advisory` is this sweep's grading of the row, and the \
     same break can still gate at a task or milestone door; the trailer below says which, where \
     a gate exists.\n";

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
///
/// The same `Some` also leads the store rows with [`SCOPE_RELATIVE_NOTE`] when an `advisory`
/// row is among them (M47 Inc 10 / T6, D12).
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
                if store_gates.is_some()
                    && report
                        .findings
                        .iter()
                        .any(|f| matches!(f.severity, Severity::Advisory))
                {
                    out.push_str(SCOPE_RELATIVE_NOTE);
                }
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
    /// The captured non-blocking hook output the landed commit's hooks emitted
    /// (`design/command-output-contract.md` → Stream discipline — the M45 `hook_output`
    /// key). **Present-always, the empty string when no hook spoke**, and the *same*
    /// captured string the stderr `--- hook output ---` relay carries (one capture, two
    /// channels — the relay trims it for its delimited section, this key carries the seam's
    /// string unchanged). It rides the JSON `committed` object so a driver that merged git's
    /// two fds reads the hook text from the one document it already parses, never off a
    /// stderr stream it may have folded into stdout.
    pub hook_output: String,
    /// Every byte jigc did **not** write into the task's working area, **moved aside**
    /// rather than destroyed when phase 7 tore that area down (M52 Increment 4 / T3;
    /// `design/storage.md` → The per-task working area). **Present always, `[]` on the
    /// ordinary path**, sorted by `from`: a key that appeared only when something moved
    /// would leave a driver guessing which of the two it was reading.
    ///
    /// This door takes no consent flag — a `task finalize --force` would be a new
    /// capability — and it cannot narrate a loss on the landed-boundary warrant either,
    /// because the commit carries the promoted docs and the index and nothing at all out of
    /// the working area. So it keeps the bytes: [`crate::task::displace_foreign_area`] moves
    /// each entry to `.jigc/displaced/<task-id>/<relative>` before the teardown, and this is
    /// what it moved — the same repo-relative pairs the stderr narration names.
    pub displaced: Vec<Displaced>,
}

/// One entry a displacing door **moved aside** instead of destroying, both halves
/// repo-relative (law 1: a surface prints no host filesystem).
///
/// The unit is a working area's complement **entry**, never a file walk: a foreign directory
/// moves whole, so `analysis/perf.txt` rides its own `analysis` pair and the subtree is
/// preserved by the move itself (`design/team-ready-state.md` → The working area's two
/// populations).
#[derive(Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Displaced {
    /// Where the entry was — its working-area path, repo-relative.
    pub from: String,
    /// Where it is now — under `.jigc/displaced/<unit-id>/`, repo-relative.
    pub to: String,
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

impl ManifestKind {
    /// Every kind, in declaration order — the vocabulary's **whole** value space, which is
    /// what the JSON `kind` key can carry (`design/command-output-contract.md` → The M43
    /// additive kind). Its completeness is fenced against the enum's own declaration by
    /// `crates/cli/tests/foldback_truth.rs`, because the exhaustive matches below force a
    /// seventh member to be *classified* and not to *join this array*.
    pub const ALL: [ManifestKind; 6] = [
        ManifestKind::Promoted,
        ManifestKind::Modified,
        ManifestKind::Deleted,
        ManifestKind::Added,
        ManifestKind::Untracked,
        ManifestKind::CarriedOver,
    ];

    /// The kind's **wire tag** — one home for the spelling every surface prints: the agent
    /// text ([`manifest_line`]) and the JSON `kind` value serde derives from the variant
    /// name. The two must agree, and a divergence is invisible at runtime, so the fence
    /// above compares this spelling against the declared variant kebab-cased.
    pub const fn tag(self) -> &'static str {
        match self {
            ManifestKind::Promoted => "promoted",
            ManifestKind::Modified => "modified",
            ManifestKind::Deleted => "deleted",
            ManifestKind::Added => "added",
            ManifestKind::Untracked => "untracked",
            ManifestKind::CarriedOver => "carried-over",
        }
    }

    /// Whether this kind can tag a path the commit **included** — the partition the enum's
    /// doc-comment above already states (`Untracked` only ever tags a left-out file), as
    /// code-side data rather than a second hand list. It is what makes the vocabulary's two
    /// true sentences both sayable: `design/finalize.md` names the **committed set**'s five,
    /// `design/command-output-contract.md` the JSON value space's six, and a prose fence
    /// owing all six everywhere would force `untracked` into the committed-set sentence.
    ///
    /// Exhaustive by construction: a seventh member does not compile without an arm here.
    pub const fn in_commit(self) -> bool {
        match self {
            ManifestKind::Promoted
            | ManifestKind::Modified
            | ManifestKind::Deleted
            | ManifestKind::Added
            | ManifestKind::CarriedOver => true,
            ManifestKind::Untracked => false,
        }
    }
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
/// No trailing newline — the caller joins / closes it. The label is
/// [`ManifestKind::tag`], the one home for the spelling this line and the JSON `kind`
/// value both put on the wire — never a second list here. `Untracked` never reaches the
/// included path (it tags only left-out files, rendered by [`left_out_lines`]); reaching
/// it anyway renders its own tag rather than the retired "swept".
fn manifest_line(entry: &ManifestEntry) -> String {
    format!("  {} {}", entry.kind.tag(), entry.path)
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
///
/// **The header states the intent, never the deed** (M48 Inc 9 / T3, RC-pre-1.0 → F13;
/// `design/surface-contract.md` → law 1). It read *"finalize — committing the index"*,
/// which the commit had not yet done and, on a run a hook rejects, never would: the header
/// stood above the rejection and *"read as done until the next line contradicted it"*.
/// **Reworded, never moved** — the print's position is settled (`design/finalize.md` → "The
/// `left-out` advisory prints BEFORE the commit too"), so the fix is in the words alone.
pub fn left_out_advisory(left_out: &[ManifestEntry]) -> String {
    let lines = left_out_lines(left_out);
    if lines.is_empty() {
        return String::new();
    }
    let mut out = String::from("finalize — about to commit the index; leaving out:\n");
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
/// [`left_out_advisory`] (the M42 print) on the same stream discipline — and, since M48
/// Inc 9 / T3, on the same **intent-not-deed** header: the sibling stem was reworded over
/// the whole pair, because F13's lie is the *stem's*, and it was carried by both prints.
pub fn carried_over_advisory(carried: &[ManifestEntry]) -> String {
    if carried.is_empty() {
        return String::new();
    }
    let mut out = String::from(
        "finalize — about to commit the index; carrying over (staged before this task existed \
         — declared with `--carry-staged`):\n",
    );
    for entry in carried {
        out.push_str(&manifest_line(entry));
        out.push('\n');
    }
    out
}

/// Frame a commit-phase **hook rejection** with the recoverability it always had but
/// never stated (M42, `design/finalize.md` → 6. Commit): git's `rejection` bytes stay
/// **verbatim and unwrapped** (the hook output *is* the correction signal — the M40
/// refinement's routed wrap covers jigc's *own* staging acts, never the user's hook
/// channel), and jigc's own sentence is added *around* them. Emitted on **stderr** by the
/// caller; `json` carries the framed text in the [`operational_error`] envelope so a tooling
/// consumer still parses it.
///
/// **Generalized off the task idiom at M47 Inc 3 T7** (`DECISIONS.md` → 2026-07-26 M47 the
/// Settle, Decision 6): the frame belongs to every one of the committing doors, not just
/// `jigc task finalize`, so the two door-specific halves are parameters —
///
/// - `survived`: the door's **state-truth** clause, no trailing period (this function adds
///   it). It must describe the state the *rejection* actually leaves, which differs per door
///   (the task door's staged set is still staged; a rejected `milestone create` unwound its
///   whole mint), and it may only claim recoverability the door genuinely has.
/// - `rerun`: the door's **own** copy-runnable command line — the argv an operator lifts out
///   of this sentence verbatim, never the task door's.
///
/// The one sentence shape is deliberate: it keeps the task door's bytes unchanged (its
/// `survived`/`rerun` reproduce M42's text exactly) while giving the other eight the same
/// three-part contract the acceptance suite (`tests/commit_rejected_axis.rs`) reads off the
/// emitted bytes.
///
/// The re-run span is **code-fenced the CommonMark way** ([`code_fence`]): the door's argv
/// embeds author-owned prose, and prose may carry backticks of its own, so a fixed single
/// backtick would let a title like ``Cache `rework` now`` truncate the very span a reader
/// lifts. The fence grows past the longest backtick run inside the command line, so the
/// delimiter is unambiguous for every prose the door can receive.
pub fn commit_rejected(rejection: &str, survived: &str, rerun: &str) -> String {
    format!("{rejection}\n\n{}", commit_rejection_route(survived, rerun))
}

/// The **recovery half** of the rejection frame, on its own — everything the frame says after
/// git's verbatim bytes: what survived, and this door's own copy-runnable re-run.
///
/// It is split out because the two surfaces need the same sentence in two shapes (M52
/// Increment 1 / T1). The agent-text arm concatenates it after the cause, exactly as it always
/// did — [`commit_rejected`] above is that concatenation and nothing else, so those bytes
/// cannot drift. The `--format json` arm puts the cause in a [`Finding`]'s `message` and this
/// sentence in its `route`, which is the decomposition the driver contract asks for: a route is
/// data a driver can act on, not prose it has to cut out of an error string.
pub fn commit_rejection_route(survived: &str, rerun: &str) -> String {
    let fence = code_fence(rerun);
    format!("{survived}. Fix the hook's complaint, then re-run {fence}{rerun}{fence}.")
}

/// Frame a commit-transaction failure that **no hook caused** — [`commit_rejected`]'s sibling
/// for N20's cell (M51 Increment 2 / T5; `completions/artifacts/M51/charter.md` → N20,
/// `gap-findings.md` → G-47).
///
/// It says the same three things the rejection frame says — the cause verbatim, what survived,
/// and this door's own copy-runnable re-run — and it deliberately does **not** say the fourth:
/// `commit_rejected` closes with *"Fix the hook's complaint"*, which in this cell is both a
/// false diagnosis (no hook spoke; git's `--ff-only` refused, or a stale index lock met the
/// stage) and a route an operator cannot follow. So this arm names no cause at all — *"resolve
/// the cause above"* is true whatever the cause was, and the cause itself is printed verbatim
/// immediately above it.
///
/// A second caller of [`commit_rejected`] would have been the cheap fix and would have shipped
/// that lie at ten doors at once, which is why the cell got its own arm rather than a parameter.
///
/// **Its second caller is git's own refusal of the commit** (M52 Increment 3 / T5): git refusing
/// to sign, or to make a partial commit during a pick, is no more a hook's complaint than a
/// stale index lock is. That caller passes the *hook* cell's state clause — everything before
/// the commit succeeded there — which is why the clause is this function's parameter and not
/// its business.
///
/// The closing `then re-run …` shape is deliberately the rejection frame's, so the one lifter
/// (`tests/commit_rejected_axis::lift_rerun`) reads the emitted bytes of both cells.
pub fn commit_failed(cause: &str, survived: &str, rerun: &str) -> String {
    format!("{cause}\n\n{}", commit_failure_route(survived, rerun))
}

/// [`commit_rejection_route`]'s sibling for the non-hook cell — same split, same reason, and
/// the one word that differs is the one this cell may not borrow: nothing here blames a hook,
/// because in this cell none spoke.
pub fn commit_failure_route(survived: &str, rerun: &str) -> String {
    let fence = code_fence(rerun);
    format!("{survived}. Resolve the cause above, then re-run {fence}{rerun}{fence}.")
}

/// The **machine arm** of the two frames above: a committing door's refusal as the [`Finding`]
/// the `--format json` reject document carries (M52 Increment 1 / T1; `settle-record.md` → D6
/// as amended by §3).
///
/// **Why the identity is a finding now, and why that is not a second name for one event.**
/// `code` is the door's own [`COMMITTING_DOORS`](crate::invocation_log::COMMITTING_DOORS)
/// error identity — the string the invocation log has recorded since M47 — and it stays that
/// on the log side unchanged. What changes is only that the **machine surface** stops carrying
/// it inside a message: a driver that hits a rejected commit under `--format json` used to get
/// `{"error": "<the whole frame>"}`, so the code it needed to branch on and the path to its own
/// raced bytes were prose. The contract's reject row has said since M45 that stderr carries
/// *exactly one document*; this is the shape that makes it true when the reject has something
/// to say (`design/command-output-contract.md` → Stream discipline; The two reject arms).
///
/// The recorded M42 reservation — *"deliberately NOT a `Finding` — a Finding would force a
/// mandatory route and wrap the hook's stderr, which the design pins as verbatim and
/// unwrapped"* — is answered rather than overridden: `message` is the hook's bytes **verbatim**
/// (JSON escaping is the envelope's, not a wrap), and the mandatory route is the frame's own
/// recovery sentence, which the door already printed. The agent-text arm is untouched.
pub fn commit_rejection_finding(code: &str, target: &str, cause: &str, route: String) -> Finding {
    Finding::graded(
        Severity::Blocking,
        code,
        cause.to_string(),
        Some(Location::addressed(target.to_string(), 1, 1)),
        Some(Route::human(route)),
    )
}

/// The backtick run that delimits `content` as a code span: one longer than the longest run
/// inside it (CommonMark's rule), so the span closes where it means to even when the content
/// carries backticks. Ordinary content is fenced with the ordinary single backtick.
fn code_fence(content: &str) -> String {
    let mut longest = 0;
    let mut run = 0;
    for c in content.chars() {
        run = if c == '`' { run + 1 } else { 0 };
        longest = longest.max(run);
    }
    "`".repeat(longest + 1)
}

/// Render the `task finalize --dry-run` pre-commit manifest to the surface `format`
/// selects (M30 G3 — name what is **included** in the commit vs **left out** of it): `json`
/// emits `{ "dry_run": true, "subject": "…", "manifest": [{path,kind}…], "left_out":
/// [{path,kind}…] }` (tooling-consumed, no footer); `agent` / `human` emit a titled block,
/// the `would commit — <subject>` forecast line, one [`manifest_line`] per included entry,
/// then the [`left_out_lines`] section, with **no trailing newline** — the caller's
/// `println!` closes it, symmetric with [`landed_summary`].
///
/// **The `subject` (M50 Inc 12 / F-7).** `subject` is
/// [`engine::finalize::FinalizePlan::subject`] — the first line of the message phase 3
/// already rendered, forecast on both surfaces so the flag whose job is *"tell me what
/// this finalize will do"* names the commit's most-read fact instead of computing and
/// discarding it. It is the same string [`Landed::subject`] carries after the commit
/// lands. **Declared bound:** this is jigc's forecast — the subject it will hand git —
/// never what git ends up with; a `commit-msg` hook may still rewrite the message.
///
/// **The `findings` (M51 Inc 5 / T5, EC-20).** The envelope carries the findings the door
/// it forecasts reports — the equal-set rider: `jigc task validate`, this forecast and the
/// committing door's emission (landed *or* blocked) are **one** code set, fenced
/// behaviourally over every [`crate::gate_coverage::Tier::Previewed`] member in
/// `crates/cli/tests/dry_run_findings_equal_set.rs`. Until M51 the forecast emitted no
/// findings channel at all: the report existed three statements above the caller's branch
/// and was discarded, so the flag QUICKSTART presents *beside* `task validate` as one
/// preview surface was silent about every advisory that surface reports.
///
/// **It is a JSON-only key, deliberately.** The standing text/JSON parity fence runs text
/// → envelope (*a value the text prints but the envelope withholds is a gap*), so an
/// additive envelope key is inside it; the agent/human forecast keeps the shape it has,
/// and `jigc task validate` remains the text surface for the findings themselves.
pub fn finalize_manifest(
    format: Format,
    subject: &str,
    included: &[ManifestEntry],
    left_out: &[ManifestEntry],
    findings: &Findings,
) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "dry_run": true,
            "subject": subject,
            "manifest": included,
            "left_out": left_out,
            "findings": findings,
        })),
        Format::Agent | Format::Human => {
            let mut lines = vec![
                "finalize --dry-run — pre-commit manifest (nothing committed)".to_string(),
                format!("would commit — {subject}"),
            ];
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
///
/// The five **edit** verbs additionally carry `copied_in` — `true` when this write was the
/// task's **first touch** of a base-committed doc, so reading it copied the committed body
/// into the working area (`doc.rs` → `Task::read_or_copy_in`). It states the same effect
/// [`DocAck::Created`]'s `existed` states on the create door (M47 Inc 10 T3, RC-alpha4
/// P4-6: the M43 fix was incomplete over the *verb* axis — the five edit doors stayed
/// silent, and the trial's worker learned of the staging from the finalize output). Since
/// M48 it is **both** surfaces: the text appends its note, and every JSON ack carries
/// `copied_in` — the parity rule the pre-1.0 additive-key window closes on (*a value the
/// text prints but the envelope withholds is a gap*;
/// `design/command-output-contract.md` §2 → the first-touch copy-in note).
pub enum DocAck {
    /// A `set-field` landed `value` at `address`/`target`. `value` is the written field
    /// value shaped as `doc show`'s `fields` project it (scalar → string, list → array).
    Field {
        address: String,
        target: AckTarget,
        value: serde_json::Value,
        findings: Findings,
        copied_in: bool,
    },
    /// A `set-field --unset` cleared the field at `address`/`target` (its line/bullet
    /// removed). No `value` key — the effect is the field's absence, read back via `doc show`.
    ///
    /// `already_absent` is the **always-present** discriminator of which conformant end
    /// the verb reached: `false` when a bullet/line was removed, `true` when the field was
    /// never there and the call changed no bytes (M49 Increment 1 T4 — the no-op that used
    /// to block with a route reproducing its own error). Law 1's *"an ack that says
    /// 'created' distinguishes created from already-existed"*, applied to removal, and it
    /// rides both surfaces: the agent-text line appends `(already absent — nothing
    /// changed)` and the envelope carries the key.
    UnsetField {
        address: String,
        target: AckTarget,
        already_absent: bool,
        findings: Findings,
        copied_in: bool,
    },
    /// A `set-slot` spliced `chars` characters of prose at `address`/`target`.
    Slot {
        address: String,
        target: AckTarget,
        chars: usize,
        findings: Findings,
        copied_in: bool,
    },
    /// A `remove-item` dropped the item at `address`/`target`.
    RemovedItem {
        address: String,
        target: AckTarget,
        findings: Findings,
        copied_in: bool,
    },
    /// A `doc rename` retitled the doc now at `address`/`target` to `title`, `from` its
    /// prior address. `reslugged` is the **always-present** discriminator of the effect:
    /// `true` when the identity moved (`address != from`), `false` on a retitle-only
    /// (`address == from`). Both keys ride the envelope because the agent-text line names
    /// both facts, and the contract withholds nothing the text prints.
    ///
    /// `committed_identity` is the doc's **provenance** cell ([`state::Provenance::
    /// EditedFromBase`](engine::state::Provenance) — copied in from the committed store),
    /// which is *why* a retitle-only kept the id, not *that* it did: a committed slug is
    /// the committed path and is frozen here, while a never-committed doc simply landed on
    /// the id it already had. The rendered sentence states the cell it is actually in — a
    /// retitle-only is reached from **both** (`design/surface-contract.md` law 1). Since M48
    /// it rides the envelope as `committed_identity` too: `reslugged: false` alone cannot
    /// tell the two cells apart, so a driver would lose a distinction the text makes.
    Renamed {
        address: String,
        target: AckTarget,
        title: String,
        from: String,
        reslugged: bool,
        committed_identity: bool,
        findings: Findings,
        copied_in: bool,
    },
    /// A `retitle-item` retitled the item at `address`/`target` (anchor frozen) to `title`.
    RetitledItem {
        address: String,
        target: AckTarget,
        title: String,
        findings: Findings,
        copied_in: bool,
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
        copied_in: bool,
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
        Format::Json => {
            let mut envelope = match ack {
                // The command-output contract (`design/command-output-contract.md` §2):
                // `op` + the decomposed `target` + the op's retained effect key +
                // `findings` (the intrinsic single-doc advisories, `[]` on a clean write —
                // each projects the pinned findings envelope via `Finding`'s `Serialize`).
                DocAck::Field {
                    target,
                    value,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "set-field", "target": target, "value": value, "findings": findings,
                }),
                // `--unset` shares the `set-field` op, with `unset: true` in place of a value.
                DocAck::UnsetField {
                    target,
                    already_absent,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "set-field", "target": target, "unset": true,
                    "already_absent": already_absent, "findings": findings,
                }),
                DocAck::Slot {
                    target,
                    chars,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "set-slot", "target": target, "chars": chars, "findings": findings,
                }),
                DocAck::RemovedItem {
                    target, findings, ..
                } => serde_json::json!({
                    "op": "remove-item", "target": target, "removed": true, "findings": findings,
                }),
                DocAck::RetitledItem {
                    target,
                    title,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "retitle-item", "target": target, "title": title, "findings": findings,
                }),
                // `committed_identity` rides the rename ack because the **text** states it
                // ("the committed identity keeps its slug" vs "the id is unchanged"), and a
                // driver reading `reslugged: false` alone cannot tell the two cells apart
                // (M48, the pre-1.0 additive-key window — a value the text prints and the
                // envelope withholds is a gap).
                DocAck::Renamed {
                    target,
                    title,
                    from,
                    reslugged,
                    committed_identity,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "rename", "target": target, "title": title, "from": from,
                    "reslugged": reslugged, "committed_identity": committed_identity,
                    "findings": findings,
                }),
                // `create` / `add-item` / `author` join the envelope: `op` + the decomposed
                // `target` + `findings`. No per-op effect key — their effect is the whole
                // doc / the new item, read back via `doc show` (contract §2). `create`
                // additionally carries the always-present `existed` discriminator.
                DocAck::Created {
                    target,
                    existed,
                    findings,
                    ..
                } => serde_json::json!({
                    "op": "create", "target": target, "existed": existed, "findings": findings,
                }),
                DocAck::AddedItem {
                    target, findings, ..
                } => serde_json::json!({
                    "op": "add-item", "target": target, "findings": findings,
                }),
                DocAck::Authored {
                    target, findings, ..
                } => serde_json::json!({
                    "op": "author", "target": target, "findings": findings,
                }),
            };
            // The wire half of the first-touch copy-in note (M48): the same fact
            // [`COPY_IN_NOTE`] states in prose, as the key a driver reads. Inserted in
            // **one** place off the same [`DocAck::copied_in`] the text arm reads, so a verb
            // cannot join the copy-in seam and stay silent on either surface. Present on
            // every arm — including the two whose own door states it differently
            // (`create`'s `existed`, `author`'s whole-doc write, both `false` here) — so a
            // driver deserializes one envelope shape.
            envelope
                .as_object_mut()
                .expect("each doc ack envelope is a JSON object")
                .insert(
                    "copied_in".to_owned(),
                    serde_json::Value::Bool(ack.copied_in()),
                );
            json(&envelope)
        }
        Format::Agent | Format::Human => {
            let line = match ack {
                DocAck::Field { address, value, .. } => {
                    format!("set {address} = {}", ack_value_display(value))
                }
                DocAck::UnsetField {
                    address,
                    already_absent,
                    ..
                } => {
                    if *already_absent {
                        format!("unset {address} (already absent — nothing changed)")
                    } else {
                        format!("unset {address}")
                    }
                }
                DocAck::Slot { address, chars, .. } => {
                    format!("set slot {address} ({chars} chars)")
                }
                DocAck::RemovedItem { address, .. } => format!("removed item {address}"),
                DocAck::RetitledItem { address, title, .. } => {
                    format!("retitled item {address} to {title:?} (anchor frozen)")
                }
                // The renamed doc's **new** address leads — the one every follow-up write
                // lands at — with the cell the verb took stated after it, so a retitle-only
                // never reads as a move that did not happen.
                DocAck::Renamed {
                    address,
                    title,
                    from,
                    reslugged: true,
                    ..
                } => format!("{address} (renamed to {title:?} from {from})"),
                // A retitle-only, stating the cell it is in rather than one cell's reason
                // over both: a committed identity's slug is the committed path (moving it
                // is `jigc rename`'s task-less, referrer-repointing job), while a
                // never-committed doc — the cell the `write.title-ignored` guard routes
                // here — simply landed on the id it already had.
                DocAck::Renamed {
                    address,
                    title,
                    committed_identity: true,
                    ..
                } => format!(
                    "{address} (retitled {title:?} — the committed identity keeps its slug)"
                ),
                DocAck::Renamed { address, title, .. } => {
                    format!("{address} (retitled {title:?} — the id is unchanged)")
                }
                // Agent-text is the bare address the verbs printed before joining the
                // envelope (byte-identical): the minted doc address (`create`/`author`) or
                // the minted item address (`add-item`) — the next address an agent drives.
                // A copy-in `create` appends the create-or-update note (M43 inc-7 T1) so
                // the surface states the effect: the committed body was carried in, not
                // minted fresh.
                DocAck::Created {
                    address,
                    existed: true,
                    ..
                } => format!("{address} (already existed — copied in for update)"),
                DocAck::Created { address, .. }
                | DocAck::AddedItem { address, .. }
                | DocAck::Authored { address, .. } => address.clone(),
            };
            // The edit doors state the same effect the create door states, once per task:
            // this write's read copied the committed doc into the working area, so the
            // task now owns (and at finalize re-promotes) it. Appended in ONE place, so a
            // verb cannot join the seam and stay silent (M47 Inc 10 T3).
            match ack.copied_in() {
                true => format!("{line} {COPY_IN_NOTE}"),
                false => line,
            }
        }
    }
}

/// The **first-touch copy-in note** the five edit verbs append to their agent/human ack
/// — [`DocAck::Created`]'s `(already existed — copied in for update)` in the edit doors'
/// mold, stating what the write's read did (`design/write-commands.md` →
/// copy-on-first-touch: the committed body is copied into the working area, carries
/// `edited-from-base` provenance, and re-promotes as an ordinary update at finalize).
const COPY_IN_NOTE: &str = "(copied in for update — the committed doc is now this task's staged copy, \
     re-promoted at finalize)";

impl DocAck {
    /// Whether this write's read **copied a base-committed doc into the task** (its first
    /// touch). Only the five edit verbs ride that seam; `create` carries the same fact in
    /// its own always-present `existed` discriminator, and `author` writes a whole doc
    /// through the batch create path, so both answer `false` here and keep their own lines.
    fn copied_in(&self) -> bool {
        match self {
            DocAck::Field { copied_in, .. }
            | DocAck::UnsetField { copied_in, .. }
            | DocAck::Slot { copied_in, .. }
            | DocAck::RemovedItem { copied_in, .. }
            | DocAck::RetitledItem { copied_in, .. }
            | DocAck::Renamed { copied_in, .. }
            | DocAck::AddedItem { copied_in, .. } => *copied_in,
            DocAck::Created { .. } | DocAck::Authored { .. } => false,
        }
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
    /// **The record-only commit a sub-task discard lands**, short sha (M51 Increment 9 / T8).
    /// This door is [`COMMITTING_DOORS`](crate::invocation_log::COMMITTING_DOORS)' tenth
    /// member — a sub-task discard settles its milestone's committed record
    /// ([`crate::milestone::settle_discarded_sub_task`]) and moves `HEAD` on the operator's
    /// behalf — yet its success line named no sha while every sibling committing door
    /// prints one (`setup`'s *install commit*, `milestone create`'s *record commit*,
    /// `migrate-corpus`'s *committed <sha>*).
    ///
    /// `None` in both of its own cells, and the enclosing surface tells them apart: an
    /// **ordinary** task discard is workbench-local and commits nothing, and a settled
    /// sub-task whose post-commit read-back failed names no sha rather than a wrong one.
    /// The text arm omits the line; the envelope carries `null`, the honest absent this
    /// family already uses for a sha that does not exist (`setup`'s `install_commit`,
    /// `migrate-corpus`'s own `commit`), never a key a driver has to know is missing.
    Discarded {
        task: String,
        dropped: Vec<DroppedStaged>,
        commit: Option<String>,
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
/// `format` selects: `agent` / `human` emit a terse ack (symmetric with the bare-line doc
/// acks — one line, plus the landed `record commit:` line when a sub-task discard committed
/// one), `json` the structured envelope (`op` + `task` + `findings`, plus `role` + the
/// decomposed `target` on a bind, and `commit` on a discard).
pub fn task_ack(format: Format, ack: &TaskAck) -> String {
    match format {
        Format::Json => match ack {
            TaskAck::Bound {
                task, role, target, ..
            } => json(&serde_json::json!({
                "op": "task-bind", "task": task, "role": role, "target": target,
                "findings": [],
            })),
            TaskAck::Discarded {
                task,
                dropped,
                commit,
            } => json(&serde_json::json!({
                "op": "task-discard", "task": task,
                "dropped": dropped.iter().map(|d| d.doc.as_str()).collect::<Vec<_>>(),
                "commit": commit,
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
            TaskAck::Discarded {
                task,
                dropped,
                commit,
            } => {
                let landed = commit.as_ref().map_or_else(String::new, |sha| {
                    format!(
                        "\n{}",
                        record_commit_line(
                            sha,
                            "this sub-task's milestone record, settled to `discarded` and \
                             committed on its own",
                        )
                    )
                });
                if dropped.is_empty() {
                    format!("discarded task {task}{landed}")
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
                        "discarded task {task} — dropped staged edits to: {}{landed}",
                        list.join(", ")
                    )
                }
            }
        },
    }
}

/// One staged doc in a [`TaskDiffView`]: its `<type>:<slug>` **identity** — which *is*
/// the address `jigc doc show <id> --task <task-id>` takes — plus the staged `body` the
/// agent-text arm prints verbatim.
///
/// The JSON arm carries the identity and **never** the body: echoing bodies would mint a
/// second, unversioned managed-doc content-read path beside the separately-versioned
/// `doc show` (`design/doc-read-surface.md` → The version/posture map). It carries no
/// `address` key either — the identity *is* the address, and a contract does not carry
/// the same fact twice (`DECISIONS.md` → 2026-08-05, the increment-7 plan halt).
pub struct StagedDoc {
    /// The `<type>:<slug>` identity, the working-area filename minus `.md`.
    pub id: String,
    /// The staged file's bytes (agent-text only).
    pub body: String,
}

/// What `jigc task diff <id>` reports: the task's own base pin, the working tree's diff
/// against it, and the docs its working area has staged.
pub struct TaskDiffView<'a> {
    /// The task id.
    pub task: &'a str,
    /// The task's recorded base pin — `{sha, short}` on the wire.
    pub base: &'a BasePin,
    /// The working tree's diff against `base.sha`; empty when there is none.
    pub code_diff: &'a str,
    /// The working area's staged docs, sorted by identity.
    pub staged: &'a [StagedDoc],
}

/// Render `jigc task diff`'s report to the surface `format` selects
/// (`design/command-output-contract.md` §2 → `jigc task diff`).
///
/// **Agent-text is byte-unchanged** from the pre-M47 surface: a `# code changes vs base
/// <short>` section when the diff is non-empty, then a `# staged docs` section listing
/// `--- <id>.md` + the verbatim body per staged doc — each section omitted entirely when
/// its content is empty, which is why the empty state renders as silence there.
///
/// **JSON is the settled envelope** (`DECISIONS.md` → 2026-07-26 M47 Settle, Decision 11):
/// `op` + `task` + `base` + `code_diff` + `staged_docs` + `findings`. `code_diff` and
/// `staged_docs` are **present-always** (`""` / `[]`) under the contract's own
/// absent-versus-empty rule — a discrimination a driver should never have to infer — and
/// that is what makes the **empty state non-empty**: the verb printed zero bytes on both
/// streams at exit 0 before this, silence a driver cannot tell from a crash. `findings`
/// is **structurally always empty**: `task diff` is a pure reader and the exit-code
/// taxonomy restricts the blocking verdict to the three gate verbs, so this verb raises
/// none; the key is present anyway so a driver deserializes one envelope shape.
pub fn task_diff(format: Format, view: &TaskDiffView<'_>) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "op": "task-diff",
            "task": view.task,
            "base": view.base,
            "code_diff": view.code_diff,
            "staged_docs": view
                .staged
                .iter()
                .map(|doc| serde_json::json!({ "id": doc.id }))
                .collect::<Vec<_>>(),
            "findings": [],
        })),
        Format::Agent | Format::Human => {
            let mut out = String::new();
            if !view.code_diff.trim().is_empty() {
                out.push_str(&format!("# code changes vs base {}\n", view.base.short));
                out.push_str(view.code_diff);
                if !view.code_diff.ends_with('\n') {
                    out.push('\n');
                }
            }
            if !view.staged.is_empty() {
                out.push_str("# staged docs\n");
                for doc in view.staged {
                    out.push_str(&format!("--- {}.md\n", doc.id));
                    out.push_str(&doc.body);
                    if !doc.body.ends_with('\n') {
                        out.push('\n');
                    }
                }
            }
            out
        }
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

/// One committed doc a `jigc config set` **relocated** — the staged `git mv` a root-knob
/// re-point landed, `from` its prior home `to` its new one (M51 Increment 5 / T4;
/// `completions/artifacts/M51/envelope-key-census.md` § 4.4 → EC-4).
///
/// **Why it is an object and not the positional pair [`crate::relocate::RelocationReport`]
/// uses.** That report's pairs ride an envelope whose *text* arm renders `from -> to` in
/// three labelled lists, so a driver reading `moved[0]` has a rendering to read the order
/// off. These moves are never rendered on the ack's text arm at all — they narrate on
/// **stderr** as they happen — so the wire is the only place the two halves can be named,
/// and they are named.
#[derive(Debug, Serialize)]
pub struct Relocated {
    /// The doc's prior repo-relative home — the path `git mv` moved away from.
    pub from: String,
    /// Its new repo-relative home under the re-pointed root.
    pub to: String,
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
/// that relocates committed docs prints its relocation *lines* on stderr, as each move
/// lands (`config::route_docs_root_repoint_orphans`); the **text** arm does not restate
/// them, and the JSON arm carries the same moves as data on [`ConfigAck::Set`]'s
/// `relocated` — a driver reads only stdout, so a fact that exists solely on stderr is
/// a fact the contract surface withholds (M51 Increment 5 / T4, EC-4).
#[derive(Debug)]
pub enum ConfigAck {
    /// `config set <key> <value>` recorded a `scalar-set`, having **relocated** the
    /// committed docs the write stranded (empty on every non-root knob, and on a root
    /// knob whose re-point stranded nothing).
    ///
    /// **It carries the moves that LANDED, never the ones that were attempted.** A
    /// per-doc failure is surfaced on stderr with its own route (`could not relocate …
    /// — move it by hand`) and is deliberately absent here: an ack that counted a
    /// failure as a relocation would name a `to` no file is at, which is the law-1 lie
    /// EC-4 exists to close. A foreign squatter **displaced** into the gitignored
    /// workbench is likewise absent — it is a different subject (a file parked out of
    /// the way, not a managed doc rehomed), and stderr narrates it as one.
    Set {
        key: String,
        value: String,
        relocated: Vec<Relocated>,
        /// The spelling the operator **typed**, when the door folded it into a different
        /// `value` — `None` when what landed is what was typed (M52 Increment 6 / T6).
        ///
        /// Only a root knob folds (`config::normalize_root_value`), and until this field the
        /// two coercions it performs landed **silently**: `config set docs-root ""` acked
        /// `= `.`` and `config set placement-root x/../y` acked `= `y``, each at exit 0 with
        /// nothing saying a fold had happened. The empty one is the sharper: `CLAUDE.md` →
        /// M50 records the design as *"`""` unset and `.` the repo root kept unmixable at the
        /// type level"*, and the door mixed them — choosing *the repository root* for an
        /// operator whose token said *unset*. The fold itself is kept (an empty
        /// `placement-root` landing verbatim would read back as never-set and silently do
        /// nothing); what changes is that the ack says so.
        ///
        /// **Text-only, by declaration** — the inverse of `relocated` beside it, and the
        /// declaration the parity fence (`crates/cli/tests/text_json_parity_axis.rs`) carries.
        /// The envelope's `value` is the value that **landed**, which is the fact a driver
        /// needs and the one half it cannot compute; the other half is the driver's own
        /// argument, which it sent and still holds. So no key is withheld from the wire — the
        /// clause is the *human* reading of a comparison a driver makes by construction, and
        /// `ConfigAck::Set`'s pinned `ENVELOPE_ARMS` shape does not move for it.
        folded_from: Option<String>,
    },
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

/// One member of the [`ConfigAck`] axis: an authoring verb, the `op` its envelope
/// names, and a **witness** of the variant that verb returns.
///
/// The [`STORE_EXIT_FLIPS`] mold — a table a fence can iterate rather than a set of
/// hand-repeated cases. It carries witness *constructors* because every `ConfigAck`
/// variant holds data, so a bare value list could not be a `const`.
pub struct ConfigAckArm {
    /// The `config` subcommand this ack confirms, as the clap tree spells it.
    pub verb: &'static str,
    /// The `op` value this variant's JSON envelope carries — what discriminates it
    /// on the wire.
    pub op: &'static str,
    /// A sample of the variant, so an axis-iterating fence feeds the real renderer
    /// and no member is testable only in principle.
    pub witness: fn() -> ConfigAck,
}

impl ConfigAck {
    /// Every cascade-authoring verb's ack, in `config --help` order — the axis the
    /// M48 parity fence iterates (`crates/cli/tests/config_ack_uncommitted.rs`),
    /// asserted there to biject against the clap `config` verb tree minus its two
    /// read verbs. A seventh authoring verb owes an arm here and reddens that fence
    /// until it has one.
    pub const ALL: &'static [ConfigAckArm] = &[
        ConfigAckArm {
            verb: "set",
            op: "config-set",
            witness: || ConfigAck::Set {
                key: "docs-root".to_owned(),
                value: "docs".to_owned(),
                // The fold is a **root-knob** event with no wire half, and the witness feeds
                // the wire fences; the clause it renders is fenced over a real corpus by
                // `crates/cli/tests/root_knob_rules.rs`, where the door does the folding.
                folded_from: None,
                // A **populated** relocation, so the parity fence proves the key carries
                // this field's own value rather than proving that two empty lists match.
                relocated: vec![Relocated {
                    from: "documents/decisions/cache-strategy.md".to_owned(),
                    to: "docs/decisions/cache-strategy.md".to_owned(),
                }],
            },
        },
        ConfigAckArm {
            verb: "insert-step",
            op: "config-insert-step",
            witness: || ConfigAck::InsertStep {
                workflow: "single-task".to_owned(),
                step: "team-extra".to_owned(),
                side: "after",
                anchor: "implement".to_owned(),
            },
        },
        ConfigAckArm {
            verb: "replace-step",
            op: "config-replace-step",
            witness: || ConfigAck::ReplaceStep {
                target: "workflow:single-task#implement".to_owned(),
                step: "project-implement".to_owned(),
            },
        },
        ConfigAckArm {
            verb: "remove-step",
            op: "config-remove-step",
            witness: || ConfigAck::RemoveStep {
                target: "workflow:single-task#implement".to_owned(),
            },
        },
        ConfigAckArm {
            verb: "fill",
            op: "config-fill",
            witness: || ConfigAck::Fill {
                target: "step:implement#extra-guidance".to_owned(),
            },
        },
        ConfigAckArm {
            verb: "fork",
            op: "config-fork",
            witness: || ConfigAck::Fork {
                target: "workflow:single-task#implement".to_owned(),
                path: ".jigc/config/steps/implement.yaml".to_owned(),
                base: "0a1b2c3d".to_owned(),
            },
        },
    ];
}

/// The clause **every** [`ConfigAck`] closes its text line with (M48, F5): the write
/// landed in `.jigc/config/` — the one *committed* corner of the otherwise gitignored
/// `.jigc/` workbench (`design/overrides.md` → The cascade) — and **jigc committed
/// nothing**, so the change sits in the worktree until the operator commits it.
///
/// Measured three times in the pre-1.0 trial: a `config set` rides an unrelated feature
/// commit, or a fresh clone inherits no config at all, and no surface said so. It is one
/// clause on all six verbs rather than a `set`-only sentence because the fact is a
/// property of the *layer*, not of the knob — the structural verbs write native step and
/// fill files into the same uncommitted layer.
///
/// The prose is the `docs-root` relocation lines' (`config::route_docs_root_repoint_orphans`)
/// — **prose reuse, not code reuse**: that line is a bare `eprintln!` firing for one knob
/// and only on a non-empty stranded set, so there is no shared function to extend. Its
/// subject is also different (the staged `git mv`s), which is why both may print.
const CONFIG_ACK_UNCOMMITTED: &str =
    "written to `.jigc/config/`, uncommitted — commit it with your next commit";

/// What the ack says about a root value the door **folded** — the clause
/// [`ConfigAck::Set`]'s `folded_from` earns (M52 Increment 6 / T6).
///
/// Two sentences, because the two coercions are different facts and one generic *"was
/// normalized"* would state neither. An **empty** value is not a spelling of the value that
/// landed at all — it is the operator asking for *unset* and getting *the repository root*, a
/// distinction `CLAUDE.md` → M50 records as kept unmixable at the type level and the door
/// mixes by necessity (a root knob has no unset spelling: an empty `placement-root` landing
/// verbatim reads back as never-set). Every other fold is one spelling of one home, and the
/// operator's question is only *why does it read back differently* — answered by naming both
/// halves and what the fold buys.
fn fold_clause(typed: &str, landed: &str) -> String {
    if typed.is_empty() {
        return format!(
            "you typed an empty value, which names the repository root `{landed}` — a root \
             knob has no unset spelling"
        );
    }
    format!(
        "you typed `{typed}`, which folds to `{landed}` — the one spelling every reader \
         resolves"
    )
}

/// Render a successful `jigc config <verb>` confirmation ([`ConfigAck`]) to the surface
/// `format` selects: `agent` / `human` emit the terse `config: <effect>` line (no
/// footer — symmetric with the bare-line doc/task acks), closed by
/// [`CONFIG_ACK_UNCOMMITTED`]; `json` a small structured object (`op` + the effect
/// fields + `committed: false`, the same fact on the wire), so a `--format json` caller
/// gets a parseable confirmation instead of empty success.
pub fn config_ack(format: Format, ack: &ConfigAck) -> String {
    match format {
        Format::Json => {
            let mut envelope = match ack {
                // `folded_from` is deliberately absent: `value` is what landed, which is
                // what a driver cannot compute, and the typed spelling is the driver's own
                // argument (see the field's own declaration).
                ConfigAck::Set {
                    key,
                    value,
                    relocated,
                    folded_from: _,
                } => serde_json::json!({
                    "op": "config-set", "key": key, "value": value,
                    "relocated": relocated,
                }),
                ConfigAck::InsertStep {
                    workflow,
                    step,
                    side,
                    anchor,
                } => serde_json::json!({
                    "op": "config-insert-step", "workflow": workflow, "step": step,
                    "side": side, "anchor": anchor,
                }),
                ConfigAck::ReplaceStep { target, step } => serde_json::json!({
                    "op": "config-replace-step", "target": target, "step": step,
                }),
                ConfigAck::RemoveStep { target } => serde_json::json!({
                    "op": "config-remove-step", "target": target,
                }),
                ConfigAck::Fill { target } => serde_json::json!({
                    "op": "config-fill", "target": target,
                }),
                ConfigAck::Fork { target, path, base } => serde_json::json!({
                    "op": "config-fork", "target": target, "path": path, "base": base,
                }),
            };
            // The wire half of the uncommitted state (M48): the same fact
            // [`CONFIG_ACK_UNCOMMITTED`] states in prose, as the key a driver reads —
            // a value the text prints and the envelope withholds is a gap. Constant
            // `false` because **no** `config` verb commits; the key exists so that
            // stays a statement a driver can read rather than one it must know.
            envelope
                .as_object_mut()
                .expect("each config ack envelope is a JSON object")
                .insert("committed".to_owned(), serde_json::Value::Bool(false));
            json(&envelope)
        }
        Format::Agent | Format::Human => {
            let effect = match ack {
                // `relocated` is deliberately not restated here: each move is narrated on
                // **stderr** by the floor that performed it, at the moment it landed and
                // with its own failure routing, and this ack's own doc says it does not
                // repeat those lines. The wire needs the fact because a driver reads only
                // stdout; the text arm already had it.
                ConfigAck::Set {
                    key,
                    value,
                    relocated: _,
                    folded_from,
                } => format!(
                    "config: set `{key}` = `{value}`{}",
                    folded_from
                        .as_deref()
                        .map(|typed| format!(" ({})", fold_clause(typed, value)))
                        .unwrap_or_default(),
                ),
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
            };
            format!("{effect} — {CONFIG_ACK_UNCOMMITTED}")
        }
    }
}

/// One declared knob's **resolved reading** — the row `jigc config get` prints for one
/// key and `jigc config list` prints for the whole closed surface
/// (`design/overrides.md` → Reading the resolved cascade).
///
/// The three facts a reader needs to explain a resolved value: the value itself, the
/// **layer that won it** (`pack-default` / `team` / `project` — [`LayerKind::label`]),
/// and, when a `scalar-set` for this key was **soft-rejected** by the knob's `floor`,
/// the [`RejectedSet`] that was dropped. Without that last field a floored demotion is
/// invisible from the read side: the delta is in the manifest, the value is unchanged,
/// and nothing says why (`overrides.md` → Soft-rejection, not abort).
///
/// [`LayerKind::label`]: engine::cascade::LayerKind::label
#[derive(Debug)]
pub struct KnobReading {
    /// The declared knob key.
    pub key: String,
    /// Its resolved value after the cascade folds every applied `scalar-set`.
    pub value: String,
    /// The layer that won the resolved value (`pack-default` when none overrode it).
    pub layer: &'static str,
    /// The `scalar-set` this key's `floor` dropped, if any.
    pub rejected: Option<RejectedSet>,
}

/// One soft-rejected `scalar-set` as the read rung reports it: the value a layer
/// attempted, the `floor` it ranked below, and the layer that attempted it — the
/// [`engine::cascade::Resolved::rejected_scalar_sets`] record, rendered.
#[derive(Debug)]
pub struct RejectedSet {
    /// The value the layer tried to set.
    pub attempted: String,
    /// The knob's demotion-lock floor the attempt ranked below.
    pub floor: String,
    /// The layer that attempted the dropped set.
    pub layer: &'static str,
}

/// One `key = value  (layer)` reading line, plus an indented `dropped:` line when a
/// `scalar-set` for the key was soft-rejected — the shared agent/human row shape of
/// both read verbs, so a one-key `get` and a whole-surface `list` read identically.
fn knob_reading_line(reading: &KnobReading) -> String {
    let mut line = format!("{} = {}  ({})\n", reading.key, reading.value, reading.layer);
    if let Some(rejected) = &reading.rejected {
        line.push_str(&format!(
            "  dropped: `{}` set in {} ranks below this knob's `{}` floor, so it was not applied\n",
            rejected.attempted, rejected.layer, rejected.floor,
        ));
    }
    line
}

/// The JSON projection of one [`KnobReading`] — the row shape both read envelopes
/// carry, so a driver parses `get` and `list` with one reader.
fn knob_reading_json(reading: &KnobReading) -> serde_json::Value {
    serde_json::json!({
        "key": reading.key,
        "value": reading.value,
        "layer": reading.layer,
        "rejected": reading.rejected.as_ref().map(|r| serde_json::json!({
            "attempted": r.attempted,
            "floor": r.floor,
            "layer": r.layer,
        })),
    })
}

/// Render `jigc config get <key>` — one knob's resolved reading. `agent` / `human`
/// emit the shared `key = value  (layer)` line (plus the `dropped:` line when a
/// below-floor set was soft-rejected); `json` emits the row object under `op:
/// config-get`. No routing footer — a knob read is not a composed reading surface,
/// the same posture [`config_ack`] takes.
pub fn config_get(format: Format, reading: &KnobReading) -> String {
    match format {
        Format::Json => {
            let mut doc = knob_reading_json(reading);
            doc["op"] = serde_json::Value::String("config-get".to_owned());
            json(&doc)
        }
        Format::Agent | Format::Human => knob_reading_line(reading).trim_end().to_owned(),
    }
}

/// Render `jigc config list` — every declared knob with its resolved reading, in the
/// declared (sorted) key order. `agent` / `human` emit one [`knob_reading_line`] per
/// knob; `json` emits them as the `knobs` array under `op: config-list`. This is the
/// **closed surface** the cascade will accept a `scalar-set` against, so it is
/// enumerated from the pack's declared knobs and never from a curated excerpt.
pub fn config_list(format: Format, readings: &[KnobReading]) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "op": "config-list",
            "knobs": readings.iter().map(knob_reading_json).collect::<Vec<_>>(),
        })),
        Format::Agent | Format::Human => readings
            .iter()
            .map(knob_reading_line)
            .collect::<String>()
            .trim_end()
            .to_owned(),
    }
}

/// A filesystem path as a **surface** may name it — **the CLI's door to the one home**,
/// [`engine::path::repo_relative`], which owns the rule and its whole rationale.
///
/// The rule moved into the engine at the M50 completion audit (finding 3): the engine's own
/// `store.*` read blocks name a committed doc's path on the 1.0-pinned `doc show --format
/// json` contract, and a crate that cannot reach the rule's home cannot obey it. This wrapper
/// stays because every CLI site already spells the call `render::repo_relative` and the
/// standing fence in `crates/cli/tests/repo_relative_paths.rs` reads that name — one home,
/// one door per crate, no second implementation.
pub(crate) fn repo_relative(repo_root: &std::path::Path, path: &std::path::Path) -> String {
    engine::path::repo_relative(repo_root, path)
}

/// The **locus** of a located [`Finding`] — the one renderer of `location`, so every text
/// surface says *where* in the same words (M49 Increment 8 / T4).
///
/// The `conformance.*` route exemption ([validation.md](../../../design/validation.md) → the
/// route floor) grants a purely-positional parser diagnostic a route-less pass on one stated
/// rationale: *the located message **is** the repair*. That was false of the surface an agent
/// reads — the JSON envelope carried `location: {address, line}` while every text render
/// printed the message alone — so the exemption rested on a fact the text withheld. This
/// function is the fix's single seam; a site that renders a finding calls it, and the source
/// sweep in `crates/cli/tests/located_finding_text.rs` holds the axis.
///
/// **Line 1 is this codebase's "no coordinate known".** Every producer that synthesizes a
/// location purely to carry an address stamps `1:1` — `finding::readdress_to_uri`,
/// `compose::at_resource`, `doc::stamp_target`, `migrate_corpus::blocked_finding` — so
/// rendering `line 1` would be a coordinate the finding does not actually claim. A line-1
/// location therefore contributes its address and nothing more, and a line-1 location with no
/// address contributes nothing at all: `None`, and the caller prints as it did before.
pub fn finding_locus(finding: &Finding) -> Option<String> {
    let location = finding.location.as_ref()?;
    let line = (location.line > 1).then(|| format!("line {}", location.line));
    match (&location.address, line) {
        (Some(address), Some(line)) => Some(format!("{address} · {line}")),
        (Some(address), None) => Some(address.clone()),
        (None, line) => line,
    }
}

/// The **head** of an agent-text finding line: `<severity> · <code>`, the identity the
/// ` — <message>` follows.
///
/// It is a named renderer for the same reason [`finding_locus`] is, and the split falls
/// exactly where the defect did: the head is the `code` half of the `(code, target)` key a
/// text-scraping driver reads (`design/command-output-contract.md` → The stable finding
/// key), and until M50 Increment 10 four of the six `finding_to_err` funnels printed
/// `finding.message` **alone** — the finding's own words with no identity in front of them
/// — while two more hand-copied this `format!`. Eight sites re-implementing one shape, four
/// of them dropping the identity. Every one of them now reaches this through
/// [`finding_line`], and `crates/cli/tests/located_finding_text.rs` checks the head exactly
/// as it checks the locus: a `Carries` site owes **both** renderers, because a check of one
/// is a check of a strictly weaker claim.
///
/// `gates` annotates the severity token as `blocking (gates at finalize)` (M42 Inc 12 / T5) — the
/// **store** view's per-finding gate label, decided by [`gates_at_finalize`]. Every other surface
/// passes `false`: at task scope the severity token already *is* the verdict, and the `setup` /
/// `ingest` finding lines are not store-sweep rows at all.
fn finding_head(finding: &Finding, gates: bool) -> String {
    let severity = match finding.severity {
        Severity::Blocking if gates => "blocking (gates at finalize)",
        Severity::Blocking => "blocking",
        Severity::Warning => "warning",
        Severity::Advisory => "advisory",
    };
    format!("{severity} · {}", finding.code)
}

/// One agent-text finding line: the [`finding_head`], plus an indented
/// `route:` line when the finding carries a repair direction (the settled
/// block-payload envelope — a hard block is a blocking finding carrying a route), and an
/// indented `at:` line between the two when the finding is **located** ([`finding_locus`]).
/// A route-less **advisory** is purely informational, and says so — the agent must
/// never be left inferring whether output wants something from it.
///
/// **The one house shape, reachable from outside this module** (M50 Increment 10 / T1):
/// `milestone`'s two stderr loops render each blocking finding through it rather than
/// re-deriving the three lines, and every `finding_to_err` funnel reaches it through
/// [`finding_error`]'s [`BlockedFinding`] carrier. The line ends in `\n`, so a caller
/// prints it with `print!`/`eprint!`, not `println!`.
pub(crate) fn finding_line(finding: &Finding, gates: bool) -> String {
    let mut line = format!("{} — {}", finding_head(finding, gates), finding.message);
    if finding.route.is_none() && matches!(finding.severity, Severity::Advisory) {
        line.push_str("   (no action needed)");
    }
    line.push('\n');
    // Where, then what to do about it: the locus precedes the route, because a route that
    // says "fix the named line" is unreadable until the line is named ([`finding_locus`]).
    if let Some(locus) = finding_locus(finding) {
        line.push_str("  at: ");
        line.push_str(&locus);
        line.push('\n');
    }
    if let Some(route) = &finding.route {
        line.push_str("  route: ");
        line.push_str(route);
        line.push('\n');
    }
    line
}

/// Render a single advisory finding for the agent/human presentation surface, **beside** a
/// composed view rather than inside it. Delegates to the house [`finding_line`] so the
/// advisory reads exactly like every other agent-text finding (`advisory · <code> —
/// <message>`, its `at:` locus, its `route:` line).
///
/// **Presentation-only**: the pinned `{task, text}` JSON contract [`composed`] projects
/// never carries it — its producer emits this beside that render, never inside it, and
/// under `--format json` sends it to **stderr** so the document on stdout still parses as
/// exactly one JSON value (`design/command-output-contract.md` → Stream discipline).
///
/// **One renderer, two producers** (M49 Increment 10 / T5): the migrate byte-floor
/// triviality nudge (M44 Inc 5, `design/auto-migration.md` → The byte-floor advisory) and
/// `jigc milestone execute`'s partially-provisioned report. A second copy of this one-line
/// shape is a second place for the advisory presentation to drift, which is why the name
/// is the *shape* rather than either producer.
pub fn advisory_line(finding: &Finding) -> String {
    finding_line(finding, false)
}

/// The line a door prints when [`crate::gitignore::ensure`] **amended**
/// `.jigc/.gitignore` — which canonical entries it appended, and that everything already
/// in the file was kept (M51 Increment 4 / T2; `design/surface-contract.md` → law 1).
///
/// **One renderer, four doors** (`crate::gitignore::IGNORE_DOORS`), because the fact is
/// one fact: `jigc setup`, `jigc task finalize` (the site the two `jigc milestone
/// finalize` arms reach as well), `jigc milestone create` and `jigc milestone provision`
/// all write into a file the user legitimately co-owns, and a second copy of this
/// sentence is a second place for it to drift. Each door frames it in its own mold — at
/// most an indent — and the line itself is identical everywhere.
///
/// **`None` is the silent case, and it is two shapes, not one.**
/// [`Ensured::Unchanged`] appended nothing, so there is nothing to say.
/// [`Ensured::Created`] appended nothing *to anything*: the file did not exist, so jigc
/// wrote its own file with its own entries and touched no line of the user's — which is
/// the ordinary fresh `jigc setup`, and a door narrating it on every install would be
/// noise standing where a real change belongs.
pub fn gitignore_amend_line(ensured: &Ensured) -> Option<String> {
    let Ensured::Amended { appended } = ensured else {
        return None;
    };
    // An `Amended` carrying nothing is unreachable by construction (`ensure` returns
    // `Unchanged` for it) — but the renderer must not compose "appended " either way.
    if appended.is_empty() {
        return None;
    }
    Some(format!(
        ".jigc/.gitignore → appended {}   (jigc's transient-runtime entries; every line \
         already in the file was kept)",
        appended.join(", "),
    ))
}

/// Render a successful `jigc setup` install to the surface `format` selects:
/// `agent` / `human` emit a one-line-per-target summary of what was installed,
/// followed by the routing footer; `json` emits a generic object naming the two
/// host targets, with no footer (tooling-consumed). The agent-text summary tells
/// the agent the install is done and where it landed.
///
/// **The envelope carries no `installed` flag** (M51 Increment 5 / T2). It held a
/// literal `true` this function could not vary, duplicating a fact the layer above
/// already carries: exit 0 **is** this envelope, and a failed install is exit 1 with
/// `{error}`. Removed under the pre-pin removal rule — the key was declared nowhere, and
/// shipping the 1.0 pin over it would bless it by omission for the life of `1.x`
/// (`design/command-output-contract.md` → Evolution posture (declared); the
/// retired-`discarded` precedent, verbatim).
pub fn setup_success(format: Format, summary: &SetupSummary) -> String {
    match format {
        Format::Json => {
            let install_commit = match &summary.install_commit {
                InstallCommit::Committed(sha) => serde_json::Value::String(sha.clone()),
                InstallCommit::Nothing | InstallCommit::Skipped => serde_json::Value::Null,
            };
            json(&serde_json::json!({
                "line_file": summary.line_file,
                "allowlist_file": summary.allowlist_file,
                // The hooks dir git **resolved** (D4), the same value the agent text
                // names — a driver that has to find the installed hook reads it here
                // rather than assuming the `.git/hooks` literal a `core.hooksPath` or a
                // linked worktree makes a lie. An additive key inside the still-open
                // pre-1.0 window (`design/command-output-contract.md` → Evolution
                // posture, the M48 additive key).
                "hook_file": summary.hook_file,
                // Whether that hook rode the install commit (M48 — the hooks-dir shape
                // the first sweep left silent). `false` is the ordinary answer on the
                // default `.git/hooks`: the hook works here and is in no commit, so a
                // clone has no drift backstop until `jigc setup` runs there. A driver
                // reads the fact rather than inferring it from the printed path's shape.
                // An additive key inside the still-open pre-1.0 window
                // (`design/command-output-contract.md` → Evolution posture).
                "hook_committed": summary.hook_committed,
                // The adapter's owned guide artifact, or `null` for a profile that
                // declares no guide target (M48 Increment 10) — a driver reads where the
                // guides landed rather than assuming an assistant-specific path. An
                // additive key inside the still-open pre-1.0 window
                // (`design/command-output-contract.md` → Evolution posture).
                "guide_file": summary.guide_file,
                // The install's non-blocking findings (M48 Increment 10 / T2) — today the
                // user-modified guide advisory. Findings-as-data on the success path
                // (`design/command-output-contract.md` → findings-as-data): a driver reads
                // the advisory and its route as a value, never by grepping the summary
                // prose. `[]` on an ordinary install.
                "findings": summary.findings,
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
            // The hooks dir git **resolved** (D4) — `core.hooksPath` and a linked
            // worktree's common hooks dir both make the `.git/hooks` literal a lie.
            out.push_str("  - pre-commit hook → ");
            out.push_str(&summary.hook_file);
            out.push_str("   (warn-only doc↔code drift backstop)\n");
            // …and, when the hook is in no commit, the consequence of that — the line above
            // is true (it IS installed) and incomplete, which on the default `.git/hooks`
            // is the shape almost every adopter is on. Keyed on the install commit's own
            // answer ([`cli::setup::SetupSummary::hook_committed`]), never on re-reading
            // the printed path, so the sentence and the commit cannot disagree.
            if !summary.hook_committed {
                out.push_str(
                    "    local to this checkout — git cannot track this path, so the hook is \
                     not in the install commit; a clone gets no drift backstop until `jigc \
                     setup` runs there\n",
                );
            }
            // The adapter's owned guide artifact (M48 Increment 10) — named only when the
            // profile declares one, so an assistant without a guide target says nothing
            // about a file it did not install.
            if let Some(guide_file) = &summary.guide_file {
                out.push_str("  - jigc guides → ");
                out.push_str(guide_file);
                out.push_str(
                    "   (jigc's own quickstart + migration notes, stamped with this build)\n",
                );
            }
            // When setup committed its own install (M26), name that commit so the user
            // knows the scaffolding landed on its own, not in their first feature commit.
            if let InstallCommit::Committed(sha) = &summary.install_commit {
                out.push_str("  - install commit → ");
                out.push_str(sha);
                out.push_str("   (setup's install files are committed on their own, off your first feature commit)\n");
            }
            // The install's non-blocking findings (M48 Increment 10 / T2) — an artifact jigc
            // declined to overwrite is *said*, never silently skipped, and it reads exactly
            // like every other agent-text finding (`advisory · <code> — <message>` + its
            // `route:` line) through the house [`finding_line`]. This block *was* the shape
            // [`run_findings`] is named after; it is the same bytes, from one home since M52
            // Increment 8 / T3 gave it a second and third producer.
            out.push_str(&run_findings(&summary.findings));
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

// `setup_block` lived here until M52 Increment 1 / T2, and it was the binary's **third**
// reject root shape: its `Format::Json` arm serialized the bare `Finding` itself, so
// `jigc --format json setup` outside a git repository and `jigc --format json uninstall` over
// unsaved workbench bytes answered with `{"severity": …, "probe": …, "code": …}` at the root —
// neither the `{error}` arm nor the `{findings, schema_version}` one, and described by no
// `ENVELOPE_ARMS` row (`settle-record.md` → D6.2 as amended by §3).
//
// Both doors now raise their refusal through [`envelope_finding_error`] and render it at the
// shared reject funnel ([`crate::invocation_log::operational_failure`]) — the pair M51
// Increment 6 built for its 25 unknown-work-unit cells. That is a deletion rather than a
// rewrite because the funnel already emits **exactly** what this function did on both arms:
// its text side renders a one-finding [`ValidationReport`] through [`validation`], whose
// single-finding body is [`finding_line`] plus [`ROUTING_FOOTER`] — this function's agent/human
// arm, byte for byte, the footer included. The doors also gain what they never had: the
// invocation log now records the identity they print, instead of one more anonymous exit 1.

/// Render a successful `jigc uninstall` teardown to the surface `format` selects
/// (`design/project-setup.md` → Flow 2 hardening → Teardown / cleanup (G5)): `agent` /
/// `human` emit one bullet per artifact **actually** removed — the real teardown set,
/// so a no-op (a second `uninstall`, nothing present) reports a clean "nothing to
/// remove" line rather than over-claiming — followed by the routing footer; `json`
/// emits a generic object naming the two host targets plus the per-artifact `removed`
/// flags, with no footer (tooling-consumed). The machine-global `doc-code` probe is
/// left in place — B2.
///
/// Both surfaces also carry what the teardown **declined** to remove: a user-modified guide
/// artifact is left standing, so it is neither a removal bullet nor a silence — it rides
/// `findings` as an advisory with its route (M48 Increment 10 / T3).
///
/// **The envelope carries no `uninstalled` flag** — `setup`'s case exactly (M51 Increment
/// 5 / T2): a literal `true` duplicating exit 0, removed under the pre-pin removal rule.
/// What a teardown script actually reads is `removed`, which is a real per-artifact
/// ledger rather than a constant.
pub fn uninstall_success(format: Format, summary: &UninstallSummary) -> String {
    let removed = &summary.removed;
    match format {
        Format::Json => json(&serde_json::json!({
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
                // The adapter's owned guide artifact (M48 Increment 10) — `false` also
                // covers the copy this teardown deliberately left standing, which the
                // `findings` below say in full. An additive key inside the still-open
                // pre-1.0 window (`design/command-output-contract.md` → Evolution posture).
                "guide": removed.guide,
            },
            // The teardown's non-blocking findings — what it declined to remove and why.
            // Findings-as-data on the success path: a driver reads the advisory and its
            // route as a value, never by grepping the summary prose. `[]` on an ordinary
            // teardown.
            "findings": summary.findings,
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
                if removed.guide {
                    out.push_str("  - removed jigc guide artifact\n");
                }
            }
            // What the teardown declined to remove, and why (M48 Increment 10 / T3) — a
            // file left standing is *said*, never silently skipped, and it reads exactly
            // like every other agent-text finding through the house [`finding_line`], from
            // the shared trailing block [`run_findings`] (same bytes; one home since M52
            // Increment 8 / T3 found this shape written out four times).
            out.push_str(&run_findings(&summary.findings));
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
/// **generic** projection of the report with **no** footer (tooling-consumed) — the
/// full per-row `rows` array plus the derived `summary` rollup (verdict-class counts +
/// the per-directory unmanaged breakdown; M44 T4). This
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
            // The run's own advisories (M52 Increment 8 / T3) — an out-of-band edit this
            // scan absorbed into a baseline is *said*, never silently taken, and it reads
            // exactly like every other agent-text finding through the house
            // [`finding_line`], the `setup` precedent.
            out.push_str(&run_findings(&report.findings));
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
        // The gloss names the **three** things that can conflict, because three producers
        // raise this verdict and only two of them are about the file's content: a near-miss
        // (`ingest.needs-reconcile` / the home schema's own parse break), a conformant doc
        // outside its home (`ingest.wrong-location`), and — since M51 Inc 9 / T4 — a
        // conformant doc at its home under a name no `<type>:<slug>` address reaches
        // (`ingest.unaddressable-identity`). The bare "conflicts" read as a content conflict
        // and sent the reader looking for a break in bytes that are clean.
        "needs-reconcile",
        "parses as the named type but conflicts — in its content, its location, or its name; fix it per the row's route, then re-run `jigc ingest`.",
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
                    // relocation sweeps (a `docs-root` re-point, a `placement-root`
                    // re-point, `jigc relocate`) walk committed truth under the home —
                    // deliberately index-blind, so a fresh clone still relocates — which
                    // means an unmanaged file left at the home is still carried by
                    // home-wide ops. Say so at the one moment the operator makes that
                    // state.
                    //
                    // The clause names the op **per home kind** rather than asserting the
                    // `docs-root` universal (M51 Increment 9 / T5, EC-16): `identity_of`
                    // resolves a placement doctype's literal `placement.file` too, and that
                    // home moves with `placement-root`, never with `docs-root`. One sentence
                    // covering both is honest for every doc this branch renders, and needs
                    // no schema lookup to stay that way.
                    Some(id) => format!(
                        "unmanaged {} ({}) — dropped its file-state baseline + forward edges; the file is left on disk. It still sits at the managed home, so the op that relocates that home — a `docs-root` re-point for a `location:` doctype, a `placement-root` re-point for a placement one — still carries it, managed or not; move it out of the managed location to fully detach it\n",
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

/// The trailing **non-blocking findings block** a success surface appends before its routing
/// footer: a blank line, then each finding through the house [`finding_line`]. Empty string
/// for an empty set, so an ordinary run's bytes are unchanged.
///
/// **One shape, four producers**: `setup_success`'s install advisories, `uninstall_success`'s
/// declined removals, and — since M52 Increment 8 / T3 — the two doors that report an absorbed
/// out-of-band edit ([`ingest`], [`rename`]). The first two had written it out inline, so the
/// shape already existed in two copies before this had a name; a copy of a shape is a place
/// for it to drift, which is why the name is the shape rather than either producer.
///
/// Not every agent-text finding render is this shape, and the two that are not stay where they
/// are: `jigc start`'s orientation view lists a task's findings under a `findings:` **label**
/// (no leading blank line, inside the task block), and [`ingest`]'s per-row finding is
/// **indented under its row**. Both say something about a listed subject; this block says
/// something about the run.
fn run_findings(findings: &engine::finding::Findings) -> String {
    if findings.is_empty() {
        return String::new();
    }
    let mut out = String::from("\n");
    for finding in findings {
        out.push_str(&finding_line(finding, false));
    }
    out
}

/// Render a `jigc rename` outcome to the surface `format` selects: `agent` / `human` emit
/// the move summary (old→new identity + path, the count of repointed referrers, each
/// listed), followed by the routing footer; `json` emits the generic projection (no
/// footer). The verb owns the structural rewrite + commit; the CLI only formats the report.
///
/// An **idempotent** run — `report.commit` is `None`, so nothing was staged and nothing was
/// committed — says exactly that instead of reporting a rename that did not happen (M48
/// Increment 8; law 1, "acks state the effect"). It is the only way to reach `None`: a
/// genuine reslug always stages the move's two ends, so the identity there is unchanged and
/// the ack names the title the doc already holds.
pub fn rename(format: Format, report: &crate::rename::RenameReport) -> String {
    match format {
        Format::Json => json(report),
        Format::Agent | Format::Human if report.commit.is_none() => {
            let mut out = format!(
                "no-op: {} already holds the title {:?} at {} — nothing renamed, nothing \
                 committed\n",
                report.from, report.title, report.old_path,
            );
            // …and the absorb block, which this arm owes exactly as much as the arm below:
            // an idempotent run stages nothing and commits nothing, and the move primitive
            // re-keys the file-state baseline anyway, so a drifted doc is absorbed *here*
            // too (M52 Increment 8 / T3, driven).
            out.push_str(&run_findings(&report.findings));
            out.push_str(ROUTING_FOOTER);
            out
        }
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
            // The transaction's absorbed out-of-band edits — the renamed doc's and every
            // repointed referrer's (M52 Increment 8 / T3).
            out.push_str(&run_findings(&report.findings));
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc migrate-corpus` outcome to the surface `format` selects: `agent` /
/// `human` emit a per-doc summary — one `migrated` / `recovered`-or-`unlanded` / `already
/// current` line per doc plus, for each blocked doc, its Framing-A route, and for each file
/// the run **declined to claim** (a never-adopted foreign file at a managed home) the store
/// door's own adoption advisory — then the **landed commit** (the verb commits its
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
            // THE RECOVERY CLAUSE (M48 Inc 9, F11 — Law 1 "the headline states what the run
            // did"). An earlier run's migration that was written and never landed (its commit
            // rejected, or `--no-commit`) is current *on disk* and absent from `HEAD`, so the
            // pre-F11 headline counted it `already current` and listed it `current` — a run that
            // **landed** it described the scan instead, while the only true sentence sat in the
            // trailing commit line. Whether it *is* landed is the run mode's answer, not the
            // corpus's: a committing run recovers those paths (the commit below names the sha),
            // a `--dry-run` / `--no-commit` run leaves them exactly as unlanded as it found them.
            // Absent entirely when there is nothing unlanded, so an ordinary run's bytes are
            // unchanged.
            let landed = report.commit.is_some();
            let recovery = if report.unlanded.is_empty() {
                String::new()
            } else if landed {
                format!("{} recovered from an earlier run, ", report.unlanded.len())
            } else {
                format!("{} unlanded from an earlier run, ", report.unlanded.len())
            };
            // A `--dry-run` suppressed the write, so it must not speak in the past tense: the
            // header says what the run IS (nothing written) and every migrated path is framed
            // `would migrate` — byte-distinct from an applying run, which the pre-F2 renderer
            // was not (M43 surface census, F2 — Law 1 "acks state the effect").
            // THE ADOPTION CLAUSE (M46 Inc 3 / T2). A committed file at a managed home that
            // jigc was never handed is excluded from the fold — it is not this verb's subject
            // — but it is **reported**, or the verb saw a file and never mentioned it. Absent
            // entirely when nothing was excluded, so an ordinary run's bytes are unchanged.
            let not_adopted = if report.unadopted.is_empty() {
                String::new()
            } else {
                format!(", {} not adopted", report.unadopted.len())
            };
            // THE LOUDNESS CLAUSE (M46 Inc 4 / T2). A `set:`-derived leaf an added-field fold had
            // no deterministic value for is left absent — conformantly — instead of blocking the
            // doc, so the run states what it left undone rather than reporting a clean migration
            // and nothing else. Absent entirely when nothing was left unfilled, so an ordinary
            // run's bytes are unchanged.
            let left_unfilled = if report.unfilled.is_empty() {
                String::new()
            } else {
                format!(", {} left unfilled", report.unfilled.len())
            };
            let mut out = if report.dry_run {
                format!(
                    "corpus migration (dry run — nothing written; {exception}): {} would migrate, {recovery}{} already current, {} blocked{not_adopted}{left_unfilled}\n",
                    report.migrated.len(),
                    report.already_current.len(),
                    report.blocked.len(),
                    exception = crate::cli::NO_WRITE_EXCEPTION,
                )
            } else {
                format!(
                    "corpus migration: {} migrated, {recovery}{} already current, {} blocked{not_adopted}{left_unfilled}\n",
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
            // The second render site the headline's word must agree with — it carried `current`
            // for a doc the run had just committed (F11).
            for path in &report.unlanded {
                if landed {
                    out.push_str(&format!("  recovered  {path}\n"));
                } else {
                    out.push_str(&format!("  unlanded   {path}\n"));
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
                // The row head IS the locus, rendered from the finding's own location through
                // the one renderer ([`finding_locus`]) — so a producer that ever raises this
                // family at a real source coordinate says so here, rather than the reader
                // losing the half the JSON keeps (M49 Increment 8 / T4).
                let path = finding_locus(finding).unwrap_or_else(|| "<unaddressed>".to_owned());
                out.push_str(&format!("  blocked    {path}\n"));
                out.push_str(&format!("    {}: {}\n", finding.code, finding.message));
                if let Some(route) = &finding.route {
                    out.push_str(&format!("    route: {route}\n"));
                }
            }
            // An excluded file prints exactly as a blocked one does — path, then code +
            // message, then route on their own lines — because it *is* a real `Finding`, the
            // store door's own (`schema-conformance.unadopted-instance`, advisory, routed at
            // adoption), rendered from the one producer rather than re-worded here.
            for finding in &report.unadopted {
                // The row head IS the locus, rendered from the finding's own location through
                // the one renderer ([`finding_locus`]) — so a producer that ever raises this
                // family at a real source coordinate says so here, rather than the reader
                // losing the half the JSON keeps (M49 Increment 8 / T4).
                let path = finding_locus(finding).unwrap_or_else(|| "<unaddressed>".to_owned());
                out.push_str(&format!("  unadopted  {path}\n"));
                out.push_str(&format!("    {}: {}\n", finding.code, finding.message));
                if let Some(route) = &finding.route {
                    out.push_str(&format!("    route: {route}\n"));
                }
            }
            // An unfilled `set:` leaf prints in the same three-line shape as a blocked or an
            // unadopted one — address, then code + message, then route — because it *is* a real
            // `Finding`: the address carries the leaf fragment (`<path>#<section>/<field>`), so a
            // reader sees WHICH leaf, and the route says whether anyone may write it at all.
            for finding in &report.unfilled {
                // The row head IS the locus, rendered from the finding's own location through
                // the one renderer ([`finding_locus`]) — so a producer that ever raises this
                // family at a real source coordinate says so here, rather than the reader
                // losing the half the JSON keeps (M49 Increment 8 / T4).
                let address = finding_locus(finding).unwrap_or_else(|| "<unaddressed>".to_owned());
                out.push_str(&format!("  unfilled   {address}\n"));
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
                //
                // A commit with **nothing migrated this run** is the N2 recovery: an earlier
                // run's migration was written but never landed (its commit rejected, or
                // `--no-commit`), and this run re-staged its pathspec. Saying *"only the
                // migrated paths were staged"* over `0 migrated` is a Law-1 lie about which
                // work the commit carries, so the recovery names itself.
                if report.migrated.is_empty() {
                    out.push_str(&format!(
                        "committed {sha} — an earlier run's migration was written but never \
                         landed; only its paths were staged\n"
                    ));
                } else {
                    out.push_str(&format!(
                        "committed {sha} — only the migrated paths were staged\n"
                    ));
                }
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
                out.push_str(&format!("  blocked   {path}\n"));
                // Indent **every** line of the reason, not only the first: since M52
                // Increment 8 / T6 a blocked row can carry a whole house-rendered finding
                // (`ingest.unaddressable-identity`'s head, `at:` locus and `route:`), and
                // an unindented continuation reads as a new top-level row.
                for line in reason.trim_end().lines() {
                    out.push_str(&format!("    {line}\n"));
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The **`record commit: <sha>` line** every record-only committing door's ack prints — one
/// home since M52 Increment 10 / T2 (D-2).
///
/// Four doors land a path-scoped commit of a milestone's committed record, and until this
/// increment two of them printed the line and two printed nothing at all
/// (`completions/artifacts/M52/baseline-surfaces.md` §4.4). The line's shape — the lead, the
/// three-space gutter, the `— <tail>` — is the fact one home owes; the `tail` stays the
/// caller's, because what the commit *was* differs per door (the milestone opening, a
/// sub-task appended, a record settled to `discarded`).
pub fn record_commit_line(sha: &str, tail: &str) -> String {
    format!("record commit: {sha}   — {tail}")
}

/// The `jigc milestone create` summary — the mint line, what the mint **landed**, and the
/// step that follows (M47 Inc 8 / T5; `completions/artifacts/M47/baseline.md` § 3c → N13,
/// `design/surface-contract.md` → law 2, nothing hides). It is the summary text
/// [`milestone`] then wraps, so the pinned `--format json` envelope stays exactly
/// `{text, hook_output}` — the growth rides inside `text`.
///
/// The mold is [`setup_success`], which already names the install commit it lands: a door that
/// commits on the operator's behalf says which commit, so the commit is reviewable instead of
/// discovered later in a `git log`. `create` landed one silently through rc.9.
///
/// **The record half is named only when it exists.** Under a `[dev ▸ methodology]` project
/// `create` materializes `docs/milestone-records/<id>.md` and lands a record-only commit;
/// dev-only it does neither, so those two lines render **no bytes at all** — the omitting
/// context stays inert, and an ack naming a path and a sha that do not exist would be the
/// law-1 lie this line exists to close. The sha line is likewise omitted (not printed as
/// `unknown`) when the post-commit read-back failed.
///
/// The `next:` step rides the checked [`engine::finding::Route::mechanical`] constructor, so
/// the CLI-seam parse fence asserts the printed argv parses against the real CLI — the quoted
/// `"<intent>"` placeholder is the same span form the sibling `create`-route uses.
pub fn milestone_created(created: &MilestoneCreated) -> String {
    let mut out = format!(
        "minted milestone:{} (shared base {})\n",
        created.id, created.base_short
    );
    if let Some(record) = &created.record {
        out.push_str(&format!(
            "record: {}   — the committed record this milestone's state lives in\n",
            record.path
        ));
        if let Some(sha) = &record.commit {
            out.push_str(&record_commit_line(
                sha,
                "the record on its own; anything else you had staged stayed staged",
            ));
            out.push('\n');
        }
    }
    // The ignore amend, when there was one (M51 Increment 4 / T2) — above the `next:`
    // route, which is the last line by convention.
    if let Some(line) = gitignore_amend_line(&created.ignore) {
        out.push_str(&line);
        out.push('\n');
    }
    let next = engine::finding::Route::mechanical(
        ["jigc", "milestone", "add-task", &created.id, "\"<intent>\""],
        "",
    );
    out.push_str(&format!(
        "next: {next}   — add the milestone's first sub-task"
    ));
    out
}

/// Render a successful `jigc milestone <verb>` action to the surface `format`
/// selects: `agent` / `human` emit the action summary (one line for most verbs;
/// `create`'s is the multi-line [`milestone_created`] ack) followed by the routing
/// footer; `json` emits a generic object carrying the summary text, with no footer
/// (tooling-consumed).
///
/// **Two arms, and `hook_output` is what separates them** (M51 Increment 5 / T3;
/// `completions/artifacts/M51/envelope-key-census.md` §4.3). `Some(stream)` adds the
/// `hook_output` key — the record-only commit's captured non-blocking hook stream,
/// **present-always** on that arm (the empty string when no hook spoke; the hook_output
/// producer axis, `design/command-output-contract.md` → Stream discipline). `None`
/// **omits the key**, which is the arm a verb that runs no commit takes: the key's
/// declaration is scoped to the landed-commit envelopes and the milestone record-only op
/// acks, and a read verb is neither — no hook can ever speak into it, so `""` would be a
/// standing claim that one could have. Which arm a verb takes is **not** decided here and
/// **not** decided by its identity: the dispatch site looks the verb's own leaf path up in
/// [`crate::cli::VERB_KINDS`], so the classification that already governs the whole clap
/// tree governs this envelope too ([`crate::milestone::MilestoneCommand::dispatch`]).
///
/// The stderr/stdout relay of the same string is the dispatch site's job
/// ([`crate::task::relay_hook_output`]), so the agent-text arm here stays the bare
/// summary. The determinism boundary is unaffected — the engine mints; the CLI only
/// formats the summary it returns (`design/write-commands.md` → Minting a milestone).
pub fn milestone(format: Format, summary: &str, hook_output: Option<&str>) -> String {
    match format {
        Format::Json => match hook_output {
            Some(hook_output) => {
                json(&serde_json::json!({ "text": summary, "hook_output": hook_output }))
            }
            None => json(&serde_json::json!({ "text": summary })),
        },
        Format::Agent | Format::Human => {
            let mut out = String::from(summary);
            out.push('\n');
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// Render a `jigc milestone join` outcome to the surface `format` selects — the door
/// runs to completion whether or not the merge is legal, so this renders both the clean
/// join and the **blocked** one. `agent` / `human` emit the merged-overlay summary (one
/// `  - <address>` line per merged doc, id-sorted, naming each doc's provenance +
/// contributing sub-task, and — for a collision-suffixed instance — the `← suffixed -N on
/// collision` decision plus `; self-ref rewritten` when its own reference was
/// rewritten in lockstep), then — C3 (round-2 surface fixes) — a `no docs staged
/// from:` line naming every sub-task that staged **no doc at all**, so the ack states
/// its effect fully instead of silently crediting a no-work sub-task; followed by the
/// routing footer. `staged_by_sub_task` is the milestone's full live sub-task set (the
/// map's key order *is* the id-sorted order the line prints in), each id paired with
/// whether its own area staged anything — read from that area's `provenance.json`, the
/// join's own input ([`crate::milestone`]'s join dispatch). The line
/// says "no docs" deliberately — the join merges docs only, and a sub-task may still
/// carry staged worktree code the finalize folds. `json` emits the **generic**
/// projection of the [`JoinOutcome`] plus the result-contract `schema_version`, with no
/// footer (tooling-consumed).
///
/// **The headline states the verdict** (M50 Increment 10 / T2). An outcome carrying a
/// blocking finding — a same-doc clash, a cross-area ref, a code collision — merged
/// nothing and committed nothing, so it opens `join blocked: milestone:<id> — <k>
/// blocking finding(s); <n> doc(s) would merge, nothing committed` rather than `joined
/// milestone:<id>`. Until then both verdicts printed the same success-shaped line and
/// the block reached the reader only on stderr, *after* the routing footer
/// ([`crate::milestone`]'s join dispatch prints the findings first for the same reason).
/// The body below the headline is unchanged either way: under a block it is the merge
/// the join **would** have produced, which is the diagnostic the contenders are read
/// against.
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
    staged_by_sub_task: &std::collections::BTreeMap<String, bool>,
) -> String {
    match format {
        Format::Json => {
            // The [`JoinOutcome`] projection, plus the two facts the **text** states that the
            // outcome alone does not carry: the milestone the join ran over, and the
            // doc-less sub-task set the text names on its `no docs staged from:` line (M48,
            // the pre-1.0 additive-key window — a value the text prints and the envelope
            // withholds is a gap). Both are additive top-level keys beside the outcome's own,
            // so a driver already parsing `overlay`/`findings` is unaffected.
            let mut envelope = serde_json::to_value(outcome).expect("the join outcome serializes");
            let object = envelope
                .as_object_mut()
                .expect("the join outcome serializes as an object");
            // The result-contract version marker every other envelope this contract rides
            // already carries (`design/command-output-contract.md` → The third version
            // integer). The join's envelope carried `findings` without it — the one
            // findings-bearing surface a driver could not version-check. The integer is
            // **read**, never retyped: it is global, so a literal here would silently
            // survive a bump (M50 Increment 10 / T2; the Settle's ordering constraint 1
            // sends any item that *moves* the integer to Increment 5, and this moves it
            // nowhere).
            object.insert(
                "schema_version".to_owned(),
                serde_json::Value::from(engine::result::SCHEMA_VERSION),
            );
            object.insert(
                "milestone".to_owned(),
                serde_json::Value::String(milestone_id.to_owned()),
            );
            object.insert(
                "no_docs_from".to_owned(),
                serde_json::Value::Array(
                    doc_less_sub_tasks(staged_by_sub_task)
                        .into_iter()
                        .map(|id| serde_json::Value::String(id.to_owned()))
                        .collect(),
                ),
            );
            json(&envelope)
        }
        Format::Agent | Format::Human => {
            // Law 1: a join carrying a blocking finding merged nothing and committed
            // nothing, so the headline may not open `joined milestone:…` (M50 Increment
            // 10 / T2; `design/surface-contract.md` → law 1). The overlay is still
            // reported — under a block it is the merge the join *would* have produced,
            // which is exactly the diagnostic the contending sub-tasks are read against.
            let blocking = outcome
                .findings
                .iter()
                .filter(|finding| finding.severity == Severity::Blocking)
                .count();
            let mut out = if blocking == 0 {
                format!(
                    "joined milestone:{milestone_id} — {} doc(s) merged\n",
                    outcome.overlay.len(),
                )
            } else {
                format!(
                    "join blocked: milestone:{milestone_id} — {blocking} blocking finding(s); \
                     {} doc(s) would merge, nothing committed\n",
                    outcome.overlay.len(),
                )
            };
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
            // silently credited by omission. Derived once, for both surfaces.
            let absent = doc_less_sub_tasks(staged_by_sub_task);
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

/// The milestone's sub-tasks that staged **no doc**, in id-sorted order — the set
/// [`milestone_join`]'s `no docs staged from:` line names and its envelope's
/// `no_docs_from` key carries. One derivation for both surfaces, so the text and the wire
/// cannot disagree about who staged nothing.
///
/// **The subject is what each sub-task staged, not what survived the merge** (M51
/// Increment 9 / T6; charter Tier 2 **EC-15**). This used to read the merge outcome's
/// `overlay` and treat every id absent from it as having staged nothing — but a
/// `join.same-doc-clash` keeps the whole contending address group **out** of the overlay,
/// so a sub-task whose only staged doc lost to the block was reported, on both surfaces,
/// as having staged nothing: the two surfaces agreed with each other and both disagreed
/// with the disk (`design/surface-contract.md` → law 1). The caller now supplies the
/// per-sub-task staged bit from each area's own `provenance.json`, so the fact stated is a
/// fact about the disk under every verdict. `overlay` is untouched by this: under a block
/// it stays the merge the join *would* have produced.
fn doc_less_sub_tasks(staged_by_sub_task: &std::collections::BTreeMap<String, bool>) -> Vec<&str> {
    staged_by_sub_task
        .iter()
        .filter(|(_, staged)| !**staged)
        .map(|(id, _)| id.as_str())
        .collect()
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
    /// Whether this sub-task had a **live** fan-out worktree at the boundary — one git
    /// reads as a worktree of its own, registered here or not (M47 Inc 3, call (b)(ii);
    /// the path-subject widening, M46 Inc 2 T1). `false` makes the degrade visible
    /// rather than silent: the worktree is the sole place a sub-agent's code can live,
    /// so a sub-task without one contributed no code — a fact `code_files: 0` alone
    /// cannot distinguish from "staged nothing."
    pub provisioned: bool,
    /// Whether a directory sits at this sub-task's worktree path that git **cannot read
    /// as a worktree of its own** — the `mv`-ed copy whose admin directory is gone, or a
    /// plain directory left at the path. It is a third fact, not a shade of
    /// `provisioned: false`: something is there, nothing could be counted out of it, and
    /// nothing from it was committed. Reading it as never-provisioned would claim the
    /// sub-task contributed nothing **by construction**, which is exactly what cannot be
    /// said here (M46 Inc 2 T1).
    pub worktree_unreadable: bool,
    /// The work in this sub-task's fan-out worktree the landed teardown **destroys** —
    /// path-sorted, empty when everything the worktree held was staged (M47 Inc 3, call
    /// (c)). The boundary commits only the staged set and then removes the whole
    /// checkout, so these bytes are lost at exit 0; naming them is the law-1 minimum.
    /// **Bound: visible, not prevented** — a `discard`-style refusal is new surface
    /// (M46).
    pub discarded: Vec<DiscardedWork>,
    /// This sub-task's **own** boundary commit — the abbreviated sha of the commit that
    /// carries its worktree code, or `None` when the boundary minted no commit for it
    /// (M50 Inc 11 / N12). `Some` only on the `squash: false` chain, and only for a
    /// **code-carrying** sub-task: `squash: true` folds every sub-task into one aggregate,
    /// and a sub-task that staged nothing contributes no commit to attribute. The merged
    /// docs are a milestone-level artifact and ride the aggregate, so this names the code
    /// channel alone — which is exactly what a fix-round orchestrator asks the envelope
    /// for ("which commit landed this fix?").
    pub hash: Option<String>,
}

/// One path a landed fan-out teardown destroys, with the reason it was not committed —
/// the unit of the law-1 loss narration on both channels (the stderr loss warning
/// and the landing manifest).
#[derive(Serialize)]
pub struct DiscardedWork {
    /// The worktree-relative path, verbatim as `git status --porcelain -z` reports it
    /// (never display-quoted — a path holding a space must survive the round trip).
    pub path: String,
    /// Why the boundary could not carry it.
    pub state: DiscardState,
}

/// Why a path in a fan-out worktree is **not** carried by the boundary commit — the three
/// reportable cells of the `git status --porcelain` **index-column** partition. The fourth
/// cell, *wholly staged* (index column set, worktree column clean), is deliberately
/// absent: those bytes land in the commit, so reporting them would be the over-report
/// that makes the whole narration untrustworthy.
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DiscardState {
    /// Nothing of this path is in the index (`??` untracked, or a worktree-only ` M`/` D`
    /// change) — the commit carries none of it.
    NeverStaged,
    /// The index holds an earlier version and the worktree has moved on (`MM`, `AM`, an
    /// unmerged cell) — the commit carries the staged half and the rest dies.
    PartlyStaged,
    /// git **ignores** this path (`!!`) — so no commit could ever carry it, and the
    /// teardown destroys it exactly as hard as an untracked one (M46 Inc 2). Reported at
    /// the ignore rule's **matching level**: `build/`, not each file under it.
    Ignored,
}

impl DiscardState {
    /// The agent-text parenthetical both channels print — one source of truth, so the
    /// stderr warning and the landing manifest cannot drift apart.
    pub fn label(self) -> &'static str {
        match self {
            DiscardState::NeverStaged => "never staged",
            DiscardState::PartlyStaged => "staged only in part",
            DiscardState::Ignored => "ignored by git",
        }
    }
}

/// One commit a landed `jigc milestone finalize` boundary made, with the paths **that**
/// commit landed (M50 Inc 11 / N12).
///
/// The boundary is not always one commit: `squash: false` lands one commit per
/// code-carrying sub-task in id order and then the merged-docs aggregate
/// (`design/finalize.md` → `fan-out` finalize). Until M50 the ack named the aggregate
/// alone beside a boundary-wide `manifest` — so it attributed every sub-task's files to a
/// sha that does not contain them, on the pinned envelope a fix-round orchestrator reads.
#[derive(Serialize)]
pub struct LandedCommit {
    /// The commit's abbreviated sha.
    pub hash: String,
    /// The commit's subject line.
    pub subject: String,
    /// The repo-relative paths this ONE commit changed (`git show --name-only`), as git
    /// reports them. A path may appear under more than one commit — see
    /// [`MilestoneLanded::manifest`] for the resolution rule.
    pub paths: Vec<String>,
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
    ///
    /// **Membership is boundary-wide and de-duplicated** — one entry per path, from the
    /// single whole-range `git diff --name-status <pre-boundary-HEAD> HEAD`. Under
    /// `squash: false` a path can be landed by more than one chain commit (a sub-agent
    /// that staged a file at a promoted doc's destination, say); it is still **one**
    /// entry here, and its **owning sha is the LAST [`commits`](Self::commits) member
    /// that lists it** — the commit whose bytes are the ones at HEAD. Reading the first
    /// claimant instead would name a sha whose version of the path was overwritten before
    /// the boundary ended.
    pub manifest: Vec<ManifestEntry>,
    /// **Every** commit the boundary made, oldest first — the chain's per-sub-task
    /// commits followed by the merged-docs aggregate under `squash: false`, and the one
    /// aggregate under `squash: true` (M50 Inc 11 / N12). Read from git over
    /// `<pre-boundary-HEAD>..HEAD`, so it is complete by construction rather than by the
    /// commit path remembering to report itself. This is the attribution channel:
    /// [`manifest`](Self::manifest) says what the boundary landed, `commits` says which
    /// sha landed it.
    pub commits: Vec<LandedCommit>,
    /// Each sub-task's contribution, id-sorted — the no-work one visible.
    pub sub_tasks: Vec<SubTaskContribution>,
    /// The captured non-blocking hook output the landed boundary's hooks emitted
    /// (`design/command-output-contract.md` → Stream discipline — the M45 `hook_output`
    /// key). Same shape and contract as [`Landed::hook_output`]: **present-always, the empty
    /// string when no hook spoke**, the *same* captured string the stderr relay carries. On
    /// the `squash: true` path this is the one combine boundary commit's stream; on
    /// `squash: false` it is **every** chain commit's stream (the N per-sub-task code
    /// commits + the aggregate) folded in commit order (the hook_output producer axis —
    /// the envelope must not carry the aggregate's stream alone).
    pub hook_output: String,
    /// Every byte jigc did **not** write into a **sub-task's** working area, **moved aside**
    /// rather than destroyed when the landed boundary tore those areas down (M52 Increment 4
    /// / T4; `settle-record.md` → §18). Same shape and same contract as
    /// [`Landed::displaced`] — `{from, to}` repo-relative pairs, sorted by `from`, **present
    /// always** (`[]` on the ordinary boundary) — and the same reason: the boundary commit
    /// carries the promoted docs, the merged record and the sub-agents' staged code and
    /// takes nothing at all out of a working area, so the *landed-boundary* warrant that
    /// lets this door's siblings merely narrate a loss does not hold here either.
    ///
    /// The union over **every** sub-task the boundary settled, so one key answers for the
    /// whole boundary; the stderr narration is per area, which is where the sub-task each
    /// move came from is already legible in the `from` path.
    pub displaced: Vec<Displaced>,
}

/// Render a **landed** `jigc milestone finalize` to the surface `format` selects
/// (surfacing, never blocking — the M42 print posture): `agent` / `human` emit
/// `finalized <hash> — <subject>`, one [`manifest_line`] per landed path, the
/// `  <n> file(s) committed` tally, the `  commits (oldest first, …)` attribution block
/// when the boundary made more than one commit ([`LandedCommit`]), and the
/// `  sub-tasks:` contribution line
/// (`<id>: 1 doc, 1 code file · <id>: nothing staged, no worktree provisioned`), then —
/// only when the teardown destroyed something — the `  discarded with the fan-out
/// worktrees` block naming each lost path and why it was not committed
/// ([`DiscardedWork`]), followed by the routing
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
            // The attribution block (M50 Inc 11 / N12) — printed only when the boundary
            // made MORE than one commit. With a single commit the header's `finalized
            // <hash>` already names it and every manifest line above belongs to it, so
            // the block would restate the manifest under a heading; its presence is
            // therefore itself the signal that the landed set is partitioned across
            // several shas. The machine arm carries `commits` either way.
            if landed.commits.len() > 1 {
                out.push_str("  commits (oldest first, each with the paths it landed):\n");
                for commit in &landed.commits {
                    out.push_str(&format!("    {} {}\n", commit.hash, commit.subject));
                    for path in &commit.paths {
                        out.push_str(&format!("      {path}\n"));
                    }
                }
            }
            if !landed.sub_tasks.is_empty() {
                let parts: Vec<String> = landed.sub_tasks.iter().map(contribution_label).collect();
                out.push_str(&format!("  sub-tasks: {}\n", parts.join(" · ")));
            }
            // The teardown's loss, named (M47 Inc 3, call (c)) — the boundary commits only
            // the staged set and then removes each worktree, so anything else the
            // sub-agent left there is gone. Omitted entirely when nothing was lost, so the
            // block's presence is itself the signal.
            let losers: Vec<&SubTaskContribution> = landed
                .sub_tasks
                .iter()
                .filter(|sub| !sub.discarded.is_empty())
                .collect();
            if !losers.is_empty() {
                out.push_str(
                    "  discarded with the fan-out worktrees (not committed, not recoverable):\n",
                );
                for sub in losers {
                    let paths: Vec<String> = sub
                        .discarded
                        .iter()
                        .map(|work| format!("{} ({})", work.path, work.state.label()))
                        .collect();
                    out.push_str(&format!("    {}: {}\n", sub.id, paths.join(" · ")));
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// One sub-task's agent-text contribution label: `<id>: 1 doc, 2 code files`,
/// either half omitted at zero, and the fully-empty case named `nothing staged`.
/// A sub-task with **no provisioned worktree** carries the `no worktree provisioned`
/// clause (M47 Inc 3, call (b)(ii)) — on every such sub-task, contributing or not,
/// because the fact it states is about the *code* channel being absent, not about
/// what was staged.
fn contribution_label(contribution: &SubTaskContribution) -> String {
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
    // `nothing staged` is a measurement, so it is withheld where nothing could be
    // measured — an unreadable worktree path leaves the code channel unread, and
    // claiming the sub-agent staged nothing would be the law-1 lie.
    if parts.is_empty() && !contribution.worktree_unreadable {
        parts.push("nothing staged".to_owned());
    }
    if contribution.worktree_unreadable {
        // Name the path, so the reader can go look at what the boundary walked past —
        // the worktree home is `.jigc/worktrees/<sub-task-id>` by construction
        // (`engine::milestone::worktree_path`).
        parts.push(format!(
            "unreadable worktree at .jigc/worktrees/{}, no code counted",
            contribution.id,
        ));
    } else if !contribution.provisioned {
        parts.push("no worktree provisioned".to_owned());
    }
    // The sub-task's own commit, where the boundary minted one (`squash: false` over a
    // code-carrying sub-task) — the prose half of `sub_tasks[].hash` (M50 Inc 11 / N12),
    // so the reader can go straight to the sha that carries this sub-task's code instead
    // of re-deriving it from the chain's order.
    if let Some(hash) = &contribution.hash {
        parts.push(format!("committed as {hash}"));
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
///
/// **`retires` names the files `--approve` will delete** (M51 Increment 1 / T7,
/// `settle-record.md` → D1 part 4). This is a human gate over a **destructive** act, and
/// until now it promised "retire the foreign original" without ever naming it — the one
/// fact a reviewer needs in order to consent was on no surface, text or machine. Each
/// entry is the adjudicated, repo-relative path the retire sink itself will unlink (the
/// caller reads it off the same `ValidatedRetirement`), so the gate and the sink cannot
/// name different files.
///
/// The set is **empty on an in-place, same-path migration** (the canonical destination IS
/// the foreign source, so `plan_retirements` skips it) — and the empty case renders its
/// own **affirmative** sentence rather than the retire sentence with nothing behind it,
/// on the same reasoning as the fidelity summary's `(none)` form below: an absent clause
/// is ambiguous, and a promise the run will not keep is a law-1 lie.
///
/// **`foreign` is `None` on the source-less hold** (M52 Increment 9 / T4). The hold's
/// subject is the promise the task's composed body makes — a body including
/// `step:migration-finalize` states that a plain finalize commits nothing and holds at
/// exit 4 — and that promise is composable without `jigc migrate`, which is the only
/// door that stages a source seam. In that cell there is nothing to be faithful *to*, so
/// the render drops the fidelity scan and the `-`/`+` diff and **says so**, showing the
/// canonical doc alone: an empty foreign half would read as a rewrite that dropped
/// everything, and a `(none)` fidelity line would claim a scan that never ran. The JSON
/// key set is unchanged — `source` is `null`, which no prior document could carry
/// because no prior state produced this arm.
///
/// **The envelope carries no `review` key** (M51 Increment 5 / T2). It could only ever
/// hold the string `"pending"`, and the hold's exit code — 4, declared in the exit-code
/// taxonomy as *the* review-hold code — already says the same thing at the layer a driver
/// reads first. Removed under the pre-pin removal rule, the key having been declared
/// nowhere (`design/command-output-contract.md` → Evolution posture (declared)).
pub fn migration_review(
    format: Format,
    task_id: &str,
    foreign: Option<&str>,
    rewrites: &[(String, String)],
    retires: &[String],
) -> String {
    match format {
        Format::Json => json(&serde_json::json!({
            "task": task_id,
            "source": foreign,
            "rewrites": rewrites
                .iter()
                .map(|(destination, rendered)| {
                    serde_json::json!({ "destination": destination, "rendered": rendered })
                })
                .collect::<Vec<_>>(),
            // Always present, empty on the in-place arm: a key that disappears turns
            // "this migration deletes nothing" into "nobody asked", which a driver
            // cannot tell from a missing feature.
            "retires": retires,
        })),
        Format::Agent | Format::Human => {
            let consequence = if retires.is_empty() {
                match foreign {
                    // The source-less cell (M52 Inc 9 / T4): nothing was staged to be
                    // faithful to and nothing is deleted, so the sentence says what
                    // approving does here rather than borrowing the in-place one, which
                    // names a foreign original this task never had.
                    None => "write the canonical doc and commit — this migration deletes \
                             nothing (no foreign source was staged for it)"
                        .to_string(),
                    Some(_) => "rewrite the foreign original in place and commit — this \
                                migration deletes nothing (the canonical destination IS \
                                the foreign source)"
                        .to_string(),
                }
            } else {
                let named = retires
                    .iter()
                    .map(|path| format!("`{path}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let noun = if retires.len() == 1 {
                    "the foreign original"
                } else {
                    "the foreign originals"
                };
                format!("write the canonical doc, DELETE {noun} {named}, and commit")
            };
            // The whole premise paragraph varies, not a spliced tail: with no source
            // there is no *rewrite* either — the agent authored the doc from the
            // workflow, not from a foreign file — so the sentence names what is actually
            // on the table. Stated, never silently omitted: the hold still fires (the
            // composed body promised it), and what a reviewer needs is that the one thing
            // the hold usually shows is absent, and why.
            let premise = match foreign {
                Some(_) => {
                    "The rewrite is the agent's; the CLI guarantees structure, never \
                     content-faithfulness — review the fidelity diff below, then approve."
                }
                None => {
                    "The doc is the agent's; the CLI guarantees structure, never \
                     content-faithfulness. No foreign source was staged for this task, so \
                     there is no fidelity diff to render — review the canonical doc below, \
                     then approve."
                }
            };
            let mut out = format!(
                "migration review required — nothing committed. Re-run \
                 `jigc task finalize {task_id} --approve` to {consequence}.\n\n\
                 {premise}\n\n",
            );
            // The structural fidelity summary (`design/auto-migration.md` → Hardening #5):
            // a release-level delta naming the source versions the rewrite dropped, split
            // into package-qualified (`pkg@version`) and bare version categories (the M44
            // report-split), so a reviewer needn't eyeball that N of M survived and sees
            // the package binding a bare scan drops. Both sides are HEURISTIC version-scans
            // over the whole text (the M40 calibration fix — a heading-only kept-set
            // false-alarmed non-changelog doctypes). Negative guard (DECISIONS C4, Framing
            // A): display-only — labeled fuzzy, feeds no gate, no agent logic, no
            // structural decision; never a second structural authority.
            // The scan is a foreign-vs-rewrite *delta*, so the source-less cell has no
            // delta to report and prints no fidelity block at all — a `(none)` there would
            // be the affirmative "the scan ran and found nothing" signal over a scan that
            // never ran.
            if let Some(foreign) = foreign {
                let dropped = dropped_release_versions(foreign, rewrites);
                // The report is SPLIT (`design/auto-migration.md` → Hardening #5, the M44
                // report-split): a package-qualified `pkg@version` and a bare version token
                // render under two distinct labels, so the reviewer sees the package binding a
                // bare scan drops. Always render both lines — the affirmative `(none)` form on
                // a nothing-dropped category (matching `design/worked-examples.md` flow 26)
                // gives the reviewer a trustworthy positive signal that the scan ran; an absent
                // line is ambiguous. Display-only either way (DECISIONS C4, Framing A).
                let package_absent = if dropped.packaged.is_empty() {
                    "(none)".to_string()
                } else {
                    dropped.packaged.join(", ")
                };
                let token_absent = if dropped.bare.is_empty() {
                    "(none)".to_string()
                } else {
                    dropped.bare.join(", ")
                };
                out.push_str(&format!(
                    "fidelity (heuristic version-scan — fuzzy, advisory; feeds no gate, \
                     no structural decision):\n  package@version absent from the rewrite: \
                     {package_absent}\n  version-like token absent from the rewrite: \
                     {token_absent}\n\n",
                ));
            }
            for (destination, rendered) in rewrites {
                // The `-`/`+` pairing is the diff; with no source there is only the
                // canonical doc, so it renders unprefixed rather than as a `+` half whose
                // missing counterpart reads as a deletion.
                match foreign {
                    Some(foreign) => {
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
                    None => {
                        out.push_str(&format!("=== canonical doc → {destination}\n"));
                        out.push_str(rendered);
                        if !rendered.ends_with('\n') {
                            out.push('\n');
                        }
                    }
                }
            }
            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

/// The heuristic release-delta for the fidelity summary (`design/auto-migration.md` →
/// Hardening #5), **report-split** (the M44 report-split): the version-like tokens
/// scanned out of the `foreign` source that are absent from the **whole** `rewrites`
/// text, partitioned into package-qualified (`pkg@version`) and bare version categories
/// so the reviewer sees the package binding a bare scan would drop. Each category is
/// sorted + de-duplicated over its own token space for a stable display. Fuzzy by
/// construction (both sides are heuristic scans, so the delta can miss or invent a
/// release); the result is **display-only** and feeds no structural decision (DECISIONS
/// C4, Framing A).
struct DroppedFidelity {
    /// Package-qualified `pkg@version` mentions absent from the rewrite.
    packaged: Vec<String>,
    /// Bare version tokens absent from the rewrite.
    bare: Vec<String>,
}

fn dropped_release_versions(foreign: &str, rewrites: &[(String, String)]) -> DroppedFidelity {
    // The kept-set scans the WHOLE rewrite text (the M40 calibration fix,
    // `design/auto-migration.md` → Hardening #5): a heading-only scan was a
    // changelog-shaped assumption (there, releases *are* `### …` headings) that
    // false-alarmed on every other doctype — a version token kept in body prose
    // still read as "dropped". A kept mention anywhere in the rewrite is kept. Each
    // category (packaged / bare) is matched over its own token space (the M44
    // report-split), so a `pkg@version` demoted to a bare version in the rewrite
    // surfaces as a dropped package binding — the whole point of the split.
    let mut kept_packaged: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut kept_bare: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (_destination, rendered) in rewrites {
        for mention in scan_version_mentions(rendered) {
            match mention {
                VersionMention::Packaged(token) => kept_packaged.insert(token),
                VersionMention::Bare(token) => kept_bare.insert(token),
            };
        }
    }
    let mut dropped_packaged: std::collections::BTreeSet<String> =
        std::collections::BTreeSet::new();
    let mut dropped_bare: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for mention in scan_version_mentions(foreign) {
        match mention {
            VersionMention::Packaged(token) if !kept_packaged.contains(&token) => {
                dropped_packaged.insert(token);
            }
            VersionMention::Bare(token) if !kept_bare.contains(&token) => {
                dropped_bare.insert(token);
            }
            _ => {}
        }
    }
    DroppedFidelity {
        packaged: dropped_packaged.into_iter().collect(),
        bare: dropped_bare.into_iter().collect(),
    }
}

/// A version-like mention classified by whether it carries a package binding — the
/// axis of the M44 report-split (`design/auto-migration.md` → Hardening #5).
enum VersionMention {
    /// A package-qualified mention `<name>@<version>` (e.g. `lodash@1.0.0`) — the
    /// version run is preceded by `@` and a package name precedes that `@`.
    Packaged(String),
    /// A bare dotted-numeric version token (e.g. `1.0.0`, `0.9`).
    Bare(String),
}

/// Whether `b` may appear in a package name for the packaged-mention scan — the common
/// `[A-Za-z0-9_-]` set (cargo/npm/pip). A `.`, `/`, or scoped-`@` prefix is deliberately
/// omitted: the scan is a fuzzy advisory aid, and the narrow set avoids grabbing a
/// preceding version's dots.
fn is_pkg_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

/// Classify every maximal dotted-numeric run in `text` (e.g. `1.0.0`, `0.9`) into a
/// [`VersionMention`] — a deliberately fuzzy version scan: at least one `.` with a digit
/// on each side, no leading or doubled dot; a trailing dot is **trimmed**, not rejected
/// (the M40 calibration fix: a sentence-final `since 1.5.` yields `1.5`, no longer an
/// under-report). A run glued to a preceding ASCII-alphanumeric or `-` byte is a
/// slug/identifier fragment (`project-alpha-2.0`, `dev2.0`), not a version, and is rejected
/// (the M44 word-boundary guard) — **except** a lone `v`/`V` version marker at a word
/// boundary (`v1.0.0`), which is the conventional version prefix and yields a bare
/// version (the M44 completion-audit refinement). A run preceded by `@`-with-a-package-name is a
/// package-qualified mention (`lodash@1.0.0` → [`VersionMention::Packaged`], the M44
/// report-split); a run at any other word boundary is [`VersionMention::Bare`].
/// Dash-separated dates (`2021-06-01`) carry no `.` and so never match. Heuristic only
/// — see [`dropped_release_versions`].
fn scan_version_mentions(text: &str) -> Vec<VersionMention> {
    let bytes = text.as_bytes();
    let mut mentions = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            let token = text[start..i].trim_end_matches('.');
            let dotted = token.contains('.') && !token.starts_with('.') && !token.contains("..");
            if !dotted {
                continue;
            }
            let prev = if start > 0 {
                Some(bytes[start - 1])
            } else {
                None
            };
            match prev {
                // Package-qualified: the run is preceded by `@` and a package name
                // precedes that `@` (the M44 report-split). Carries the binding a bare
                // scan drops.
                Some(b'@') => {
                    let at = start - 1;
                    let mut j = at;
                    while j > 0 && is_pkg_name_byte(bytes[j - 1]) {
                        j -= 1;
                    }
                    if j < at {
                        mentions.push(VersionMention::Packaged(
                            text[j..i].trim_end_matches('.').to_string(),
                        ));
                    } else {
                        // A bare `@` with no package name — an ordinary bare version.
                        mentions.push(VersionMention::Bare(token.to_string()));
                    }
                }
                // Version-marker exception (the M44 completion-audit refinement): a lone
                // `v`/`V` immediately before the run is the conventional version prefix
                // (`v1.0.0`), not a slug — accept it as a bare version, but only when the
                // marker is itself at a word boundary (start-of-string or a non-alphanumeric
                // byte before it), so `dev2.0` — where `v` is mid-word — still rejects. Must
                // precede the general word-boundary guard below.
                Some(b'v') | Some(b'V')
                    if start == 1 || !bytes[start - 2].is_ascii_alphanumeric() =>
                {
                    mentions.push(VersionMention::Bare(token.to_string()));
                }
                // Word-boundary guard (the M44 calibration fix): a dotted run glued to a
                // preceding ASCII-alphanumeric or `-` byte is a slug/identifier fragment
                // (`project-alpha-2.0`, `dev2.0`), not a version — reject it, don't cry wolf on
                // the operator's most-checked advisory surface.
                Some(b) if b.is_ascii_alphanumeric() || b == b'-' => {}
                // A run at start-of-string or after any other byte (whitespace,
                // punctuation) is a genuine word boundary — a bare version token.
                _ => mentions.push(VersionMention::Bare(token.to_string())),
            }
        } else {
            i += 1;
        }
    }
    mentions
}

/// The bare dotted-numeric version tokens in `text` — the [`VersionMention::Bare`]
/// projection of [`scan_version_mentions`]. Retained as the unit-tested entry point for
/// the M44 word-boundary guard (see `render_scan_version_tokens_word_boundary_guard`).
#[cfg(test)]
fn scan_version_tokens(text: &str) -> Vec<String> {
    scan_version_mentions(text)
        .into_iter()
        .filter_map(|m| match m {
            VersionMention::Bare(token) => Some(token),
            VersionMention::Packaged(_) => None,
        })
        .collect()
}

/// Flatten a blocking [`Finding`] into an `anyhow::Error` carrying the **whole** finding
/// surface — `severity · code — message`, the locus, and the route — so a verb whose only
/// failure channel is the operational funnel ([`operational_error`]) still refuses with an
/// identity and a recovery. Since M49 Increment 11 / T4 it carries the finding **itself**
/// ([`BlockedFinding`]) and not only its rendering, so a dispatch handler can log the
/// identity it prints ([`blocked_finding`]); the emitted bytes are unchanged.
///
/// The shape is the findings surface's own ([`finding_line`]): one funnel must not
/// describe a break in fewer facts than another (M49 Increment 8 / T4). Three verbs
/// already flattened findings this way with a private copy each (`config`, `describe`,
/// `milestone`); this is the shared one, minted when M49 Increment 11 / T1 needed a fourth
/// — the unknown-doctype block at `jigc migrate --as` / `relocate` / `rename` /
/// `task bind`, four doors that carried no finding code between them.
///
/// **Declared bound:** flattening puts the block in the *message*, so under
/// `--format json` these doors carry it inside the `{"error": …}` envelope rather than as
/// the structured finding projection — the posture the three earlier copies already ship.
/// It is the right posture for a code the contract lists under **no** target form, and the
/// wrong one for a code it lists under one: that listing promises a driver `(code, target)`
/// resolves, and a code inside a message is not a key. Those refusals take
/// [`envelope_finding_error`] instead (M51 Increment 6 / T1).
pub fn finding_error(finding: &Finding) -> anyhow::Error {
    anyhow::Error::new(BlockedFinding {
        finding: finding.clone(),
        envelope: false,
        beside: Vec::new(),
    })
}

/// The sibling of [`finding_error`] for a refusal the contract declares as a finding that
/// **projects a key** — same carrier, same printed identity, but the `--format json` arm
/// is the **findings envelope** (`findings` + `schema_version`) rather than the flattened
/// single-key `{"error": …}` (M51 Increment 6 / T1).
///
/// **Why the two forms coexist, and how a door chooses.** Flattening is the surface's
/// default and stays so: it costs a driver the envelope *key* and nothing else, which is a
/// price `design/command-output-contract.md` → *the complement* declares and accepts. What
/// it may **not** do is contradict the contract — and a code the contract lists under a
/// declared **target form** is one it promises a driver can key on, so that code's doors
/// owe the envelope. `finalize.no-task` is listed under the work-unit form, which is why
/// the unknown-work-unit doors take this constructor. Which arm each `(verb, arm)` pair
/// ships is a declared row of [`ENVELOPE_ARMS`], not a per-door judgment.
///
/// The printed (agent / human) surface is the house findings render either way; the two
/// differ only in the machine arm, and the routing footer the envelope render carries.
pub fn envelope_finding_error(finding: &Finding) -> anyhow::Error {
    anyhow::Error::new(BlockedFinding {
        finding: finding.clone(),
        envelope: true,
        beside: Vec::new(),
    })
}

/// The sibling of [`envelope_finding_error`] for a refusal that **carries other findings the
/// same reject document owes** — M52 Increment 5 / T8.
///
/// The contract's selection rule is *a reject that carries a finding takes the findings arm,
/// with the operational error itself as a finding*, and *on the arm whose document is
/// stderr's, nothing else is written there*
/// (`design/command-output-contract.md` → The two reject arms / Stream discipline). A door
/// whose transaction rolled back has two facts to report — what failed, and every path the
/// rollback could not put back — and printing the second beside the first is exactly the
/// shape M52 Increment 1 closed at the committing doors. `beside` is that second set: the
/// one funnel renders it *inside* the document and records every code in the invocation log.
///
/// `finding` stays the carrier's [`Display`](std::fmt::Display) and the head of the rendered
/// report, so a door with nothing beside it is byte-identical to
/// [`envelope_finding_error`].
pub fn envelope_finding_error_beside(finding: &Finding, beside: Vec<Finding>) -> anyhow::Error {
    anyhow::Error::new(BlockedFinding {
        finding: finding.clone(),
        envelope: true,
        beside,
    })
}

/// The [`Finding`] a [`finding_error`] was built from, when `err` is one — the read half
/// of the carrier, so a dispatch handler can name the refusal in the invocation log
/// (`Outcome::with_findings`) instead of logging one more anonymous exit 1.
///
/// Answers for **both** constructors: the log record is owed the identity whichever arm
/// the machine surface takes.
pub fn blocked_finding(err: &anyhow::Error) -> Option<&Finding> {
    err.downcast_ref::<BlockedFinding>()
        .map(|blocked| &blocked.finding)
}

/// The [`Finding`] an [`envelope_finding_error`] was built from — `None` for a flattened
/// one, so the one funnel that renders these
/// ([`crate::invocation_log::operational_failure`]) can ask which arm this refusal
/// declared instead of guessing from its code.
pub fn envelope_projecting_finding(err: &anyhow::Error) -> Option<&Finding> {
    err.downcast_ref::<BlockedFinding>()
        .filter(|blocked| blocked.envelope)
        .map(|blocked| &blocked.finding)
}

/// **Every finding an envelope-projecting refusal puts in its one document** — the refusal
/// itself, then whatever the door carried [`beside`](BlockedFinding::beside) it (M52
/// Increment 5 / T8).
///
/// The funnel reads this rather than [`envelope_projecting_finding`] so a door cannot report
/// the failure and drop the conflicts its own rollback raised; for every producer that
/// carries nothing beside, the two return the same one finding.
pub fn envelope_projecting_findings(err: &anyhow::Error) -> Option<Vec<Finding>> {
    err.downcast_ref::<BlockedFinding>()
        .filter(|blocked| blocked.envelope)
        .map(|blocked| {
            let mut findings = vec![blocked.finding.clone()];
            findings.extend(blocked.beside.iter().cloned());
            findings
        })
}

/// A blocking [`Finding`] travelling as an `anyhow::Error` — the carrier
/// [`finding_error`] builds and [`blocked_finding`] reads back.
///
/// It exists because a flattened finding lost something on the way out: the printed
/// surface kept the code, the locus and the route, but the **invocation log** saw only
/// `Outcome::failure()` — exit 1, `finding_codes: []`, `error_code: null` — so every
/// refusal a verb raised this way was indistinguishable there from every other
/// (M49 Increment 11 / T4). Carrying the finding rather than only its rendering costs one
/// clone and lets the dispatch log the identity it already printed.
///
/// [`Display`](std::fmt::Display) is the findings surface's own shape — `severity · code
/// — message`, the locus, the route ([`finding_line`]) — so `{err:#}` emits the bytes the
/// four doors already shipped and nothing on the printed side moves.
#[derive(Debug)]
pub struct BlockedFinding {
    /// The refusal itself — printed, logged, and (on the envelope arm) serialized.
    pub finding: Finding,
    /// Whether the `--format json` arm is the **findings envelope** rather than the
    /// flattened `{"error": …}` — set by [`envelope_finding_error`], and read at the one
    /// funnel by [`envelope_projecting_finding`]. It rides the carrier rather than being
    /// re-derived from the code at the funnel, so the door that raises the refusal is the
    /// one that declares its machine shape.
    pub envelope: bool,
    /// Findings the **same reject document** owes, rendered after `finding` and logged with
    /// it — the rollback conflicts a failed transaction raised, whose only other home would
    /// be bytes printed beside the envelope that a driver has to drop
    /// ([`envelope_finding_error_beside`]). Empty for every other producer.
    pub beside: Vec<Finding>,
}

impl std::fmt::Display for BlockedFinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The **house** agent-text finding line, minus the trailing newline the error
        // funnels' own `eprintln!` supplies. Delegating rather than re-deriving is what
        // makes this a carrier and not a second renderer: the flattened refusal cannot
        // drift from every other rendered finding, and `located_finding_text`'s sweep
        // keeps one row for one site instead of two rows for one shape.
        f.write_str(finding_line(&self.finding, false).trim_end_matches('\n'))
    }
}

impl std::error::Error for BlockedFinding {}

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
/// footer (tooling-consumed). **It parses, and the help says so** (M47 Inc 10 / T6):
/// the arm is *unpinned*, not *unparseable* — nothing versions it and any pack edit
/// may move it, which is what non-contractual means here; `jigc doc schema` is the
/// versioned structural read. The prose-hostility above is the prose arm's lever, and
/// the help no longer forbids a parse this arm serves.
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
                out.push_str(
                    "The workflows you can compose here. To read a task-minting one's full step text before you commit to running it, run `jigc workflow <id> --preview`, which composes the steps without minting a task; a workflow that mints nothing has no preview, and `jigc start --workflow <id>` composes it directly without minting either. Neither reaches a workflow whose line below says it is reached only through a verb: that verb binds what its steps read, so it is the one door that composes it, and both of these refuse it by name. ",
                );
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
                // The id alone does not identify an entry under a composed pack-set —
                // two packs may declare the same id with different hints, and both are
                // reachable — so each sentence names its declaring pack. A pack that
                // declares no `pack-id` has no attribution to print, and prints none.
                let sentences: Vec<String> = description
                    .commands
                    .iter()
                    .map(|c| {
                        if c.pack.is_empty() {
                            format!("{} {}", c.id, c.hint)
                        } else {
                            format!("{} ({} pack) {}", c.id, c.pack, c.hint)
                        }
                    })
                    .collect();
                out.push_str(&sentences.join(" "));
                out.push_str("\n\n");
            }

            out.push_str(ROUTING_FOOTER);
            out
        }
    }
}

// ───────────────────── the pinned-envelope registry (M51 Inc 5 / T7) ─────────────────────

/// Where a registry row's **arm** came from — the provenance half of D5's claim that the
/// enumeration is *derived where an enum can derive it*.
///
/// `clap` enumerates **syntax**, never runtime result variants, which is why the recipe
/// table's one-per-verb bijection had to be reshaped to `(path, arm)` in the first place
/// (`completions/artifacts/M51/settle-record.md` → §7). A verb's arms therefore come from
/// one of exactly three places, and the row says which — with the last two making a
/// *checkable* claim about how many rows the verb's path owns.
pub enum ArmOrigin {
    /// A **variant of a production result enum**, named here. The enum's own arm-name
    /// table ([`DOC_ACK_ARMS`] and its three siblings, or [`ConfigAck::ALL`]) is the only
    /// producer of the string, and a new variant cannot compile until it joins that table
    /// — at which point the registry fence reddens until the variant has a row.
    Variant {
        /// The production enum, spelled as the path a reader can open.
        of: &'static str,
    },
    /// **The verb answers with exactly one key set**, so nothing discriminates and nothing
    /// needs to: its several run-modes (`--dry-run` vs committed, hit vs miss, empty vs
    /// populated) move *values*, never keys — which is the shape a registry row should make
    /// visible. A `Sole` row is the only row its path has, and the fence checks that.
    Sole,
    /// **No enum can generate it**: the verb has several arms and a *dispatch branch*
    /// chooses among them. Carries the stated reason, which is what the Settle owes for each
    /// hand-enumerated row. A `Dispatch` row shares its path with at least one sibling, and
    /// the fence checks that too.
    Dispatch(&'static str),
}

/// The JSON **shape** an arm's document takes at its root. Six members, because four of
/// them carry no declarable top-level key list at all — the addressed document's own data,
/// or a bare scalar, decides the keys — a shape space a `keys: &[&str]` column alone would
/// have had to lie about.
///
/// **Driven, not reasoned** (`completions/artifacts/M52/baseline-contracts.md` §2.3):
/// `jigc doc show` answers with **seven** distinct root shapes over a stock corpus, against
/// the four rows the registry carried before M52 Increment 1 — which is what falsified both
/// sentences this enum used to state about its own shape space, and why
/// `crates/cli/tests/format_json_success_axis.rs` now fences those claims against the
/// registry instead of leaving them to a reader.
pub enum ArmShape {
    /// A JSON **object**, whose top-level key set is exactly these names, sorted.
    Object(&'static [&'static str]),
    /// A top-level **array** whose every element is an object with exactly these keys,
    /// sorted. `jigc task list` is the one array whose element keys are declarable,
    /// declared rather than reshaped
    /// (`completions/artifacts/M51/envelope-key-census.md` §4.1) — which is why the two
    /// facts an object would carry, `schema_version` and `op`, are absent here by
    /// construction rather than by omission. The surface's **other** arrays are not this
    /// shape: `jigc doc show` answers with [`ArmShape::ArrayOfDataKeyed`] and
    /// [`ArmShape::ArrayOfScalars`], whose elements no row can enumerate.
    ArrayOf(&'static [&'static str]),
    /// A bare JSON **scalar** — no keys at all. It is not only a slot section's prose:
    /// `jigc doc show` answers this way for every scalar leaf the address can reach — a
    /// slot section, a scalar field leaf, an item's own slot, and the `id-from` leaf that
    /// carries the item's heading.
    Scalar,
    /// A JSON object whose keys are **the document's own data**, so no key set can be
    /// declared. The shape is pinned; the key set is the document's. Two projections of
    /// `jigc doc show` are this shape: a fields-only section's field map, whose members
    /// follow the addressed doctype and which of its optional fields are populated, and a
    /// repeatable **item**'s own object, whose members follow its block.
    DataKeyed,
    /// A top-level **array** whose every element is a [`ArmShape::DataKeyed`] object — the
    /// shape pinned, the element keys the document's. `jigc doc show` is the only verb that
    /// answers with an array of data-keyed objects: a repeatable section addressed without
    /// an item hop (`#<section>`), and a declared nested block reached through the
    /// write-grammar chain, both serve that section's item array, whose members follow the
    /// addressed doctype's block rather than any set a row could name.
    ArrayOfDataKeyed,
    /// A top-level **array of bare scalars** — no keys at any level. A **list**-cardinality
    /// field leaf projects its elements as JSON strings (`#<section>/<leaf>` on a
    /// `card: "0..*"` field — `adr.supersedes` and `vision.grounded-in` are the shipped
    /// ones), which is the shape `design/doc-read-surface.md` already declares for a list
    /// field and the registry did not.
    ArrayOfScalars,
}

/// Whether a driver may build on the arm's key set. The two doors are **asymmetric**,
/// which is the whole reason D5 mints this before the 1.0 pin: `Unpinned(reason)` is
/// reversible — a later wave may pin it — while a blessed key is not, so an accident
/// shipped as `Pinned` costs a 2.0 to remove.
pub enum ArmStatus {
    /// A driver may build on the key set; it evolves only by a versioned extension
    /// (`design/command-output-contract.md` → Evolution posture).
    Pinned,
    /// Declared **as** unpinned, with the reason — so an absent promise is a decision on
    /// the record rather than an oversight.
    Unpinned {
        /// Why this arm's key set is not a promise.
        reason: &'static str,
        /// The keys that **are** pinned on it anyway — an unpinned *envelope* may still
        /// carry a key another contract pins (`describe` keeps the result contract's
        /// `schema_version`; the five milestone write acks keep `hook_output`). A subset of
        /// the declared key set.
        still_pinned: &'static [&'static str],
    },
}

/// Which stream carries the arm's document, and at what exit — the shape the driver's
/// discrimination predicate reads (`design/command-output-contract.md` → Stream
/// discipline: *parse stdout; if stdout is empty, parse stderr*).
pub enum ArmOutcome {
    /// The document rides **stdout** and stderr carries no JSON document, at exit **0** in
    /// the state the arm is driven in.
    ///
    /// **Not a claim that the verb can never exit non-zero.** Three rows' verbs flip their
    /// exit on a *corpus condition* rather than on an arm change, and in every such cell the
    /// declared key set is still exact on stdout with stderr at 0 bytes
    /// (`completions/artifacts/M52/baseline-contracts.md` §2.4): a store-scope condition
    /// drives `jigc validate`'s own `StoreSweep` arm to exit 1 — [`STORE_EXIT_FLIPS`] is
    /// that membership — `jigc migrate-corpus --dry-run`'s `Report` exits 1 over an
    /// ahead-of-current stamp, and `jigc task validate`'s `Report` exits 3 on a blocking
    /// preview. The **exit-code taxonomy is unaffected and deliberately not reworded**
    /// (`design/command-output-contract.md` → the exit taxonomy): what the stream discipline
    /// discriminates on is the *stream*, and that holds in all three.
    Success,
    /// A non-zero **adjudication**: the gate ran and reported, so the document still rides
    /// stdout, at the named exit (`task finalize` blocked at 3, the migration review hold
    /// at 4). Not a reject — nothing failed to run.
    Adjudicated(i32),
    /// A **reject**: stdout empty, the document on stderr, exit non-zero. The exit varies
    /// by door (1 for an operational error, 1 or 3 for a blocking finding), so the row
    /// names the stream and not a code.
    Reject,
}

/// Whether the envelope is rooted in a value **the result contract versions** — the
/// partition `schema_version` rides, fenced against this registry rather than asserted in
/// prose (D5: *typed result values carry it, ad-hoc `json!` envelopes do not*, and the
/// *"add it to ~40 envelopes"* reading is refused on the record as gold-plating).
pub enum ArmRoot {
    /// The root is an `engine` result value serialized whole — possibly extended in place
    /// with keys the text also prints — and `engine::result::SCHEMA_VERSION` is the integer
    /// that versions it. It reaches the wire either as the root type's own field or, at
    /// [`milestone_join`], as the one explicit insert made at the same seam.
    ResultContract(&'static str),
    /// A `json!` literal or a CLI-local `Serialize` the result contract does **not**
    /// version. It carries no `schema_version`, and adding one would be a claim the
    /// contract does not back.
    AdHoc(&'static str),
}

/// One `(path, arm)` row of the pinned-envelope registry: a leaf verb's argv path, the arm
/// of that verb whose `--format json` document this row declares, and the declaration.
///
/// See [`ENVELOPE_ARMS`] for what the table is and what fences it.
pub struct EnvelopeArm {
    /// The leaf verb's argv path as the clap tree spells it — **empty** for the two
    /// cross-cutting reject arms, which no single leaf owns.
    pub path: &'static [&'static str],
    /// The arm, unique within `path`. For a [`ArmOrigin::Variant`] row it is the production
    /// enum variant's own name.
    pub arm: &'static str,
    /// Where the arm came from — derived from an enum, or dispatch-chosen with a reason.
    pub origin: ArmOrigin,
    /// The document's root shape, and its declared key set where it has one.
    pub shape: ArmShape,
    /// Whether a driver may build on that key set.
    pub status: ArmStatus,
    /// Which stream carries it, and at what exit.
    pub outcome: ArmOutcome,
    /// Whether the result contract versions the root — the `schema_version` partition.
    pub root: ArmRoot,
}

/// The arm names of every [`DocAck`] variant, in declaration order — the **only**
/// producer of a `DocAck` row's `arm` string in [`ENVELOPE_ARMS`].
///
/// This array plus [`doc_ack_arm`]'s exhaustive match is the compile-time half of proof 2:
/// a tenth variant cannot compile until the match names it, and the name it must be given
/// comes from here — a tenth match arm reading `DOC_ACK_ARMS[9]` is a **hard compile
/// error** (`unconditional_panic`) until the array grows, at which point the registry
/// fence reddens until the variant has a row. That chain is what makes the nine `doc` rows
/// *derived* rather than a hand list that happens to be right today.
pub const DOC_ACK_ARMS: [&str; 9] = [
    "DocAck::Field",
    "DocAck::UnsetField",
    "DocAck::Slot",
    "DocAck::RemovedItem",
    "DocAck::Renamed",
    "DocAck::RetitledItem",
    "DocAck::Created",
    "DocAck::AddedItem",
    "DocAck::Authored",
];

/// The [`ENVELOPE_ARMS`] arm name of one [`DocAck`] — see [`DOC_ACK_ARMS`] for why the
/// names are read out of an array rather than written as literals here.
pub fn doc_ack_arm(ack: &DocAck) -> &'static str {
    match ack {
        DocAck::Field { .. } => DOC_ACK_ARMS[0],
        DocAck::UnsetField { .. } => DOC_ACK_ARMS[1],
        DocAck::Slot { .. } => DOC_ACK_ARMS[2],
        DocAck::RemovedItem { .. } => DOC_ACK_ARMS[3],
        DocAck::Renamed { .. } => DOC_ACK_ARMS[4],
        DocAck::RetitledItem { .. } => DOC_ACK_ARMS[5],
        DocAck::Created { .. } => DOC_ACK_ARMS[6],
        DocAck::AddedItem { .. } => DOC_ACK_ARMS[7],
        DocAck::Authored { .. } => DOC_ACK_ARMS[8],
    }
}

/// The arm names of every [`TaskAck`] variant, in declaration order. [`DOC_ACK_ARMS`]'
/// chain, applied to the task-state acks.
pub const TASK_ACK_ARMS: [&str; 2] = ["TaskAck::Bound", "TaskAck::Discarded"];

/// The [`ENVELOPE_ARMS`] arm name of one [`TaskAck`].
pub fn task_ack_arm(ack: &TaskAck) -> &'static str {
    match ack {
        TaskAck::Bound { .. } => TASK_ACK_ARMS[0],
        TaskAck::Discarded { .. } => TASK_ACK_ARMS[1],
    }
}

/// The arm names of every [`ConfigAck`] variant, in declaration order. [`DOC_ACK_ARMS`]'
/// chain, applied to the cascade-authoring acks — which already ship a witness table,
/// [`ConfigAck::ALL`], so the `config` rows are fenced from **both** sides: the witnesses
/// map through [`config_ack_arm`] to these names, and each name's row must answer to that
/// witness's own `verb`.
pub const CONFIG_ACK_ARMS: [&str; 6] = [
    "ConfigAck::Set",
    "ConfigAck::InsertStep",
    "ConfigAck::ReplaceStep",
    "ConfigAck::RemoveStep",
    "ConfigAck::Fill",
    "ConfigAck::Fork",
];

/// The [`ENVELOPE_ARMS`] arm name of one [`ConfigAck`].
pub fn config_ack_arm(ack: &ConfigAck) -> &'static str {
    match ack {
        ConfigAck::Set { .. } => CONFIG_ACK_ARMS[0],
        ConfigAck::InsertStep { .. } => CONFIG_ACK_ARMS[1],
        ConfigAck::ReplaceStep { .. } => CONFIG_ACK_ARMS[2],
        ConfigAck::RemoveStep { .. } => CONFIG_ACK_ARMS[3],
        ConfigAck::Fill { .. } => CONFIG_ACK_ARMS[4],
        ConfigAck::Fork { .. } => CONFIG_ACK_ARMS[5],
    }
}

/// The arm names of every [`OrientationView`] variant, in declaration order — the three
/// orientation states `jigc start` answers with when it orients rather than composes.
/// [`DOC_ACK_ARMS`]' chain, applied to the engine's own result sum.
pub const ORIENTATION_ARMS: [&str; 3] = [
    "OrientationView::UnsetProject",
    "OrientationView::Clean",
    "OrientationView::ActiveTask",
];

/// The [`ENVELOPE_ARMS`] arm name of one [`OrientationView`].
pub fn orientation_arm(view: &OrientationView) -> &'static str {
    match view {
        OrientationView::UnsetProject { .. } => ORIENTATION_ARMS[0],
        OrientationView::Clean { .. } => ORIENTATION_ARMS[1],
        OrientationView::ActiveTask { .. } => ORIENTATION_ARMS[2],
    }
}

/// The shared dispatch reason of the eight `jigc doc show` rows — one fact, one home.
///
/// The **address** picks the projection, never a result variant, and the root it answers
/// with follows: an object, an array, or a bare scalar. Each row's own `arm` and `shape`
/// name which projection it is, so this string states only what they share.
const DOC_SHOW_DISPATCH: &str = "the ADDRESS picks the projection — whole doc, fields-only section, slot section, \
     repeatable section, item, field leaf — and the root it answers with is an object, an \
     array or a bare scalar depending on which, so no result enum could model the set";

/// **The pinned-envelope registry**: every leaf verb × every arm that produces a distinct
/// `--format json` top-level key set, each row `Pinned` or `Unpinned(<reason>)`
/// (`completions/artifacts/M51/settle-record.md` → D5, amended by §7; the table is
/// [envelope-key-census.md](../../../completions/artifacts/M51/envelope-key-census.md),
/// adopted as decided).
///
/// **Why it exists.** `design/command-output-contract.md` states the rule *"an undeclared
/// key on a pinned envelope is a defect, not an addition, whichever wave mints it"* — and
/// before this table the rule quantified over a set that existed nowhere, so on 1.0.0 day
/// it was unanswerable in both directions. The contract doc now **names this list** rather
/// than quantifying over an imagined one, which is why the registry is production-side: a
/// locked 1.0 doc may not point at a test fixture, and the binary can read what it declares.
///
/// **What fences it** (`crates/cli/tests/format_json_success_axis.rs`, the one suite that
/// already drives every leaf to a real success through the real binary — a suite that
/// renders a witness cannot see an arm the *dispatch* chooses):
///
///   1. every clap leaf verb has **≥ 1** row;
///   2. every production arm has **exactly one** row — `(path, arm)` is unique, and each
///      result enum's rows equal that enum's own arm-name table;
///   3. every row is **driven** through the real binary;
///   4. the **driven** top-level key set equals the **declared** one — and, the partition
///      D5 asks for, `schema_version` rides a row **iff** its [`ArmRoot`] is
///      [`ArmRoot::ResultContract`].
///
/// Beside the four proofs, one guard on the table's own **prose**: every shape claim this
/// module states about the registry is checked against the registry, so a sentence the
/// surface has outgrown reddens rather than being left for a reader to catch. It is what
/// M52 Increment 1 / T4 added, having found two such sentences false — the shape space was
/// declared once and never re-driven.
///
/// **Declared bound — arm completeness is bounded by driving.** An arm exists here iff a
/// driven invocation produced a distinct key set. A run-mode that produces the *same* key
/// set is one row with its modes named in the origin's reason (the mode is then a *value*
/// — `dry_run`, `existed`, `already_absent` — which is the shape a registry row should make
/// visible); a key set reachable only under a state the census did not build would not
/// appear. Proof 1's `⇔` against the clap tree is what keeps that bound survivable: a verb
/// can be missing an *arm*, never a *row*.
///
/// `schema-version` and `contract-version` (hyphenated) are **not** the partition's key:
/// they are the addressed document's own stamp and `doc schema`'s separately-versioned
/// projection, and the fence keys on the exact name `schema_version`.
pub const ENVELOPE_ARMS: &[EnvelopeArm] = &[
    EnvelopeArm {
        path: &["start"],
        arm: "OrientationView::UnsetProject",
        origin: ArmOrigin::Variant {
            of: "OrientationView",
        },
        shape: ArmShape::Object(&["schema_version", "state"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::OrientationView"),
    },
    EnvelopeArm {
        path: &["start"],
        arm: "OrientationView::Clean",
        origin: ArmOrigin::Variant {
            of: "OrientationView",
        },
        shape: ArmShape::Object(&[
            "header",
            "next_steps",
            "schema_version",
            "state",
            "workflows",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::OrientationView"),
    },
    EnvelopeArm {
        path: &["start"],
        arm: "OrientationView::ActiveTask",
        origin: ArmOrigin::Variant {
            of: "OrientationView",
        },
        shape: ArmShape::Object(&[
            "header",
            "next_steps",
            "schema_version",
            "state",
            "tasks",
            "workflows",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::OrientationView"),
    },
    EnvelopeArm {
        path: &["start"],
        arm: "Composed",
        origin: ArmOrigin::Dispatch(
            "`start.rs` chooses compose-or-orient on the argv — an intent, `--workflow` or \
             `--task` composes, a bare `start` orients — and no result type spans the two",
        ),
        shape: ArmShape::Object(&["task", "text"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::start::Composition, via `render::composed`"),
    },
    EnvelopeArm {
        path: &["workflow"],
        arm: "Composed",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["task", "text"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::start::Composition, via `render::composed`"),
    },
    EnvelopeArm {
        path: &["setup"],
        arm: "Installed",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&[
            "allowlist_file",
            "findings",
            "guide_file",
            "hook_committed",
            "hook_file",
            "install_commit",
            "line_file",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::setup::SetupSummary"),
    },
    EnvelopeArm {
        path: &["uninstall"],
        arm: "TornDown",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["allowlist_file", "findings", "line_file", "removed"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::setup::UninstallSummary"),
    },
    EnvelopeArm {
        path: &["upgrade"],
        arm: "Swept",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["checked", "findings", "guide", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &["ingest"],
        arm: "Triaged",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["findings", "rows", "summary"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::ingest::IngestReport"),
    },
    EnvelopeArm {
        path: &["migrate"],
        arm: "Composed",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["task", "text"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::start::Composition, via `render::composed`"),
    },
    EnvelopeArm {
        path: &["migrate-corpus"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&[
            "already_current",
            "blocked",
            "commit",
            "dry_run",
            "hook_output",
            "migrated",
            "unadopted",
            "unfilled",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::corpus_migration`'s `json!`"),
    },
    EnvelopeArm {
        path: &["unmanage"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["dropped", "identity", "path"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::unmanage`'s `json!`"),
    },
    EnvelopeArm {
        path: &["rename"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&[
            "commit",
            "findings",
            "from",
            "hook_output",
            "new_path",
            "old_path",
            "prose_mentions",
            "referrers",
            "title",
            "to",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::rename`'s `json!`"),
    },
    EnvelopeArm {
        path: &["relocate"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["blocked", "displaced", "moved"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::freeze_exempt_relocation`'s `json!`"),
    },
    EnvelopeArm {
        path: &["describe"],
        arm: "Menu",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["commands", "definitions", "schema_version"]),
        status: ArmStatus::Unpinned {
            reason: "a composed-prose menu — `design/introspection.md` declares the json arm \
                     *unpinned rather than unparseable*: nothing versions it and any pack \
                     edit may move it, which is what non-contractual means here",
            still_pinned: &["schema_version"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::introspect::Description"),
    },
    EnvelopeArm {
        path: &["validate"],
        arm: "StoreSweep",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&[
            "blocking_probes",
            "findings",
            "report_only",
            "schema_version",
            "scope",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &["doc", "create"],
        arm: "DocAck::Created",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "existed", "findings", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "add-item"],
        arm: "DocAck::AddedItem",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "findings", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "remove-item"],
        arm: "DocAck::RemovedItem",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "findings", "op", "removed", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "retitle-item"],
        arm: "DocAck::RetitledItem",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "findings", "op", "target", "title"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "rename"],
        arm: "DocAck::Renamed",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&[
            "committed_identity",
            "copied_in",
            "findings",
            "from",
            "op",
            "reslugged",
            "target",
            "title",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "set-field"],
        arm: "DocAck::Field",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "findings", "op", "target", "value"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "set-field"],
        arm: "DocAck::UnsetField",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&[
            "already_absent",
            "copied_in",
            "findings",
            "op",
            "target",
            "unset",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "set-slot"],
        arm: "DocAck::Slot",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["chars", "copied_in", "findings", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "author"],
        arm: "DocAck::Authored",
        origin: ArmOrigin::Variant { of: "DocAck" },
        shape: ArmShape::Object(&["copied_in", "findings", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::doc_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "WholeDoc::Committed",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::Object(&[
            "fields",
            "item-count",
            "schema-version",
            "sections",
            "slug",
            "type",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "WholeDoc::Staged",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::Object(&[
            "fields",
            "item-count",
            "schema-version",
            "sections",
            "slug",
            "staged",
            "type",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "FieldsGroupSlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::DataKeyed,
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "SlotSlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::Scalar,
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "ItemArraySlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::ArrayOfDataKeyed,
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "ItemSlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::DataKeyed,
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "ListFieldSlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::ArrayOfScalars,
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "show"],
        arm: "CompoundFieldSlice",
        origin: ArmOrigin::Dispatch(DOC_SHOW_DISPATCH),
        shape: ArmShape::Object(&["sha", "short"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc show` projection"),
    },
    EnvelopeArm {
        path: &["doc", "schema"],
        arm: "Projection",
        origin: ArmOrigin::Sole,
        // `home` and `identity` join at `contract-version` 7 (M52 Increment 6 / T7):
        // the projection now names which addresses a doctype's instances have and
        // where they live, from `engine::schema::Schema::projection`.
        shape: ArmShape::Object(&[
            "contract-version",
            "fields",
            "home",
            "identity",
            "schema-version",
            "sections",
            "type",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc::SchemaContract`"),
    },
    EnvelopeArm {
        path: &["doc", "list"],
        arm: "Index",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["docs"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`cli::doc`'s `doc list` index projection"),
    },
    EnvelopeArm {
        path: &["task", "list"],
        arm: "Rows",
        origin: ArmOrigin::Sole,
        shape: ArmShape::ArrayOf(&["id", "intent", "workflow"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`[cli::task::TaskListRow]` — a bare array, not an object"),
    },
    EnvelopeArm {
        path: &["task", "diff"],
        arm: "Ack",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["base", "code_diff", "findings", "op", "staged_docs", "task"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::task_diff`'s `json!`"),
    },
    EnvelopeArm {
        path: &["task", "validate"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["findings", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &["task", "discard"],
        arm: "TaskAck::Discarded",
        origin: ArmOrigin::Variant { of: "TaskAck" },
        shape: ArmShape::Object(&["commit", "dropped", "findings", "op", "task"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::task_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["task", "bind"],
        arm: "TaskAck::Bound",
        origin: ArmOrigin::Variant { of: "TaskAck" },
        shape: ArmShape::Object(&["findings", "op", "role", "target", "task"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::task_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["task", "finalize"],
        arm: "Landed",
        origin: ArmOrigin::Dispatch(
            "four separate `render::` calls chosen by `task.rs`'s finalize dispatch — no \
             `FinalizeOutcome` enum exists, and the three non-landed arms carry three \
             different exits",
        ),
        shape: ArmShape::Object(&["committed", "findings", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &["task", "finalize"],
        arm: "Forecast",
        origin: ArmOrigin::Dispatch(
            "four separate `render::` calls chosen by `task.rs`'s finalize dispatch — no \
             `FinalizeOutcome` enum exists, and the three non-landed arms carry three \
             different exits",
        ),
        shape: ArmShape::Object(&["dry_run", "findings", "left_out", "manifest", "subject"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::finalize_manifest`'s `json!`"),
    },
    EnvelopeArm {
        path: &["task", "finalize"],
        arm: "Blocked",
        origin: ArmOrigin::Dispatch(
            "four separate `render::` calls chosen by `task.rs`'s finalize dispatch — no \
             `FinalizeOutcome` enum exists, and the three non-landed arms carry three \
             different exits",
        ),
        shape: ArmShape::Object(&["findings", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Adjudicated(3),
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &["task", "finalize"],
        arm: "MigrationReviewHold",
        origin: ArmOrigin::Dispatch(
            "four separate `render::` calls chosen by `task.rs`'s finalize dispatch — no \
             `FinalizeOutcome` enum exists, and the three non-landed arms carry three \
             different exits",
        ),
        shape: ArmShape::Object(&["retires", "rewrites", "source", "task"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Adjudicated(4),
        root: ArmRoot::AdHoc("`render::migration_review`'s `json!`"),
    },
    EnvelopeArm {
        path: &["config", "set"],
        arm: "ConfigAck::Set",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["committed", "key", "op", "relocated", "value"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "insert-step"],
        arm: "ConfigAck::InsertStep",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["anchor", "committed", "op", "side", "step", "workflow"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "replace-step"],
        arm: "ConfigAck::ReplaceStep",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["committed", "op", "step", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "remove-step"],
        arm: "ConfigAck::RemoveStep",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["committed", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "fill"],
        arm: "ConfigAck::Fill",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["committed", "op", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "fork"],
        arm: "ConfigAck::Fork",
        origin: ArmOrigin::Variant { of: "ConfigAck" },
        shape: ArmShape::Object(&["base", "committed", "op", "path", "target"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_ack`'s per-variant `json!`"),
    },
    EnvelopeArm {
        path: &["config", "get"],
        arm: "Reading",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["key", "layer", "op", "rejected", "value"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_get`'s `KnobReading` projection"),
    },
    EnvelopeArm {
        path: &["config", "list"],
        arm: "Readings",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["knobs", "op"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::config_list`'s `KnobReading` projection"),
    },
    EnvelopeArm {
        path: &["milestone", "create"],
        arm: "RecordOnlyAck",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["hook_output", "text"]),
        status: ArmStatus::Unpinned {
            reason: "`text` is composed prose — the record-only ack the judgment tier censused \
                     as such — so a driver reads the record, never this string",
            still_pinned: &["hook_output"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "add-task"],
        arm: "RecordOnlyAck",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["hook_output", "text"]),
        status: ArmStatus::Unpinned {
            reason: "`text` is composed prose — the record-only ack the judgment tier censused \
                     as such — so a driver reads the record, never this string",
            still_pinned: &["hook_output"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "add-from-spec"],
        arm: "RecordOnlyAck",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["hook_output", "text"]),
        status: ArmStatus::Unpinned {
            reason: "`text` is composed prose — the record-only ack the judgment tier censused \
                     as such — so a driver reads the record, never this string",
            still_pinned: &["hook_output"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "provision"],
        arm: "RecordOnlyAck",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["hook_output", "text"]),
        status: ArmStatus::Unpinned {
            reason: "`text` is composed prose — the record-only ack the judgment tier censused \
                     as such — so a driver reads the record, never this string",
            still_pinned: &["hook_output"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "discard"],
        arm: "RecordOnlyAck",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["hook_output", "text"]),
        status: ArmStatus::Unpinned {
            reason: "`text` is composed prose — the record-only ack the judgment tier censused \
                     as such — so a driver reads the record, never this string",
            still_pinned: &["hook_output"],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "list-tasks"],
        arm: "Listing",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["text"]),
        status: ArmStatus::Unpinned {
            reason: "the same composed prose as its five write siblings, minus `hook_output`: a \
                     `VerbKind::Read` verb commits nothing, so no hook can ever speak into it \
                     (M51 Increment 5 / T3)",
            still_pinned: &[],
        },
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "execute"],
        arm: "Composed",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&["task", "text"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("cli::start::Composition, via `render::composed`"),
    },
    EnvelopeArm {
        path: &["milestone", "join"],
        arm: "Report",
        origin: ArmOrigin::Sole,
        shape: ArmShape::Object(&[
            "findings",
            "milestone",
            "no_docs_from",
            "overlay",
            "schema_version",
        ]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::ResultContract("engine::milestone::JoinOutcome"),
    },
    EnvelopeArm {
        path: &["milestone", "finalize"],
        arm: "Landed",
        origin: ArmOrigin::Dispatch(
            "`milestone.rs`'s finalize dispatch picks the landed ack or the block; the two \
             share no type, and only one of them carries `findings`",
        ),
        shape: ArmShape::Object(&["committed"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Success,
        root: ArmRoot::AdHoc("`render::milestone_finalized`'s `json!`"),
    },
    EnvelopeArm {
        path: &["milestone", "finalize"],
        arm: "Blocked",
        origin: ArmOrigin::Dispatch(
            "`milestone.rs`'s finalize dispatch picks the landed ack or the block; the two \
             share no type, and only one of them carries `findings`",
        ),
        shape: ArmShape::Object(&["findings", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Adjudicated(3),
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
    EnvelopeArm {
        path: &[],
        arm: "Reject::Error",
        origin: ArmOrigin::Dispatch(
            "cross-cutting: which reject funnel a door enters is decided by the failure it \
             caught, not by the verb — a bare `anyhow` chain takes the first, a refusal \
             carrying a `Finding` the second. M52 Increment 1 / T1 moved the ten \
             `COMMITTING_DOORS` rejections across that line: a hook-refused commit used to \
             flatten its whole frame into this arm's `error` string, and now emits its \
             door's own `*.commit-rejected` identity — plus any rollback conflict — on the \
             findings arm below (the pre-pin reshape rule, `command-output-contract.md` -> \
             Evolution posture). This arm keeps every bare-`anyhow` reject and is unchanged",
        ),
        shape: ArmShape::Object(&["error"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Reject,
        root: ArmRoot::AdHoc("`render::operational_error`'s single-key `json!`"),
    },
    EnvelopeArm {
        path: &[],
        arm: "Reject::Findings",
        origin: ArmOrigin::Dispatch(
            "cross-cutting: which reject funnel a door enters is decided by the failure it \
             caught, not by the verb — a bare `anyhow` chain takes the first, a refusal \
             carrying a `Finding` the second. The doors that moved here: M51 Increment 6's \
             25 unknown-work-unit cells; M52 Increment 1 / T1's ten `COMMITTING_DOORS` \
             rejections, whose document also carries every `finalize.rollback-conflict` the \
             transaction raised — the one stream rule, so a reject's stream parses whole; \
             and M52 Increment 1 / T2's `setup` / `uninstall`, which until then answered on \
             a THIRD root shape no row here described (the bare `Finding` serialized whole, \
             through the deleted `render::setup_block`); and M52 Increment 9 / T2's \
             `workflow.verb-routed`, whose subject is a pack resource the contract keys \
             under the pack-resource target form, so the flattened arm could not carry it. \
             With them this arm and the one \
             above are the whole reject surface, which is what makes the two of them a \
             declaration a driver can discriminate on rather than a pair of common cases",
        ),
        shape: ArmShape::Object(&["findings", "schema_version"]),
        status: ArmStatus::Pinned,
        outcome: ArmOutcome::Reject,
        root: ArmRoot::ResultContract("engine::result::ValidationReport"),
    },
];

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
                copied_in: false,
            },
        );
    }

    /// The `setup` / `uninstall` reject seam (M52 Increment 1 / T2), both arms at once — the
    /// **named regression risk** of deleting `setup_block` and routing those two doors onto
    /// the shared reject funnel, fenced where the bytes are composed.
    ///
    /// The risk was one-directional and precise: the funnel's text side renders a
    /// [`ValidationReport`] through [`validation`], and if that render were anything other
    /// than [`finding_line`] plus [`ROUTING_FOOTER`] the doors would have lost their footer
    /// silently. So the first half asserts the **exact frame** against a literal composed
    /// here, not against the renderer's own output — a comparison of a function with itself
    /// proves nothing.
    ///
    /// The second half keeps what the deleted function's own test pinned: the membership
    /// check on the `--format json` arm. Every `setup.*` / `uninstall.*` code is a **declared
    /// singleton** (both doors are fail-fast `Result<_, Finding>`: at most one per
    /// invocation), so it keys at `target: null` by the pin rather than by omission
    /// (`command-output-contract.md` → The declared singleton exception) — and it now does so
    /// **inside** the `{findings, schema_version}` envelope, where at HEAD it was the root.
    #[test]
    fn the_setup_reject_renders_the_same_frame_on_the_declared_arm() {
        let singleton = Finding::block(
            "setup.repo-root",
            "`jigc setup` must run inside a git repository",
            "run `git init` first",
        );
        let resolved = crate::cascade_util::no_delta_resolved().expect("the no-delta cascade");
        let report = ValidationReport::new(vec![singleton], &resolved);

        assert_eq!(
            validation(Format::Agent, &report),
            format!(
                "blocking · setup.repo-root — `jigc setup` must run inside a git \
                 repository\n  route: run `git init` first\n{ROUTING_FOOTER}"
            ),
            "the doors' agent-text frame is the house finding line plus the routing footer \
             — the bytes the deleted `setup_block` emitted, unmoved",
        );

        let value: serde_json::Value =
            serde_json::from_str(&validation(Format::Json, &report)).expect("parses");
        assert!(
            value.get("code").is_none(),
            "the reject root is the envelope, never the finding itself",
        );
        assert_eq!(value["schema_version"], engine::result::SCHEMA_VERSION);
        assert_eq!(value["findings"][0]["key"]["code"], "setup.repo-root");
        assert_eq!(
            value["findings"][0]["key"]["target"],
            serde_json::Value::Null,
            "a declared singleton keys at null and passes the seam",
        );
    }

    /// T12 (M45 Inc 10) — describe names `jigc workflow <id> --preview` as the way to
    /// read a workflow's step text, gated on the workflows group being present. The
    /// pull-tier `--preview` (M44) shipped with nothing routing to it (findings §69);
    /// describe's workflow tour is where it earns its route (`introspection.md` →
    /// Command surface; `surface-contract.md` → law 2: nothing hides). The omitting
    /// context — a describe with no workflows — names no preview route (inert).
    #[test]
    fn render_describe_routes_to_workflow_preview() {
        use engine::introspect::{DefinitionKind, DefinitionProse, Description};
        use engine::result::SCHEMA_VERSION;

        let with_workflow = Description {
            schema_version: SCHEMA_VERSION,
            definitions: vec![DefinitionProse {
                kind: DefinitionKind::Workflow,
                id: "single-task".to_string(),
                prose: "single-task is one end-to-end scoped change. Reach for it when the work is small enough to hold in your head.".to_string(),
                router_hidden: None,
                origin_pack: None,
            }],
            commands: vec![],
        };
        let out = describe(Format::Agent, &with_workflow);
        assert!(
            out.contains("`jigc workflow <id> --preview`"),
            "describe must name the preview route as the way to read a workflow's step text; got:\n{out}",
        );

        // Omitting context: a describe with NO workflows (only a doctype) names no
        // preview route — there is nothing to preview, so the routing stays inert.
        let no_workflow = Description {
            schema_version: SCHEMA_VERSION,
            definitions: vec![DefinitionProse {
                kind: DefinitionKind::Doctype,
                id: "adr".to_string(),
                prose: "adr is a dated architectural decision record. Reach for it when a choice is worth preserving.".to_string(),
                router_hidden: None,
                origin_pack: None,
            }],
            commands: vec![],
        };
        let out = describe(Format::Agent, &no_workflow);
        assert!(
            !out.contains("--preview"),
            "a describe with no workflows names no preview route (inert omit-context); got:\n{out}",
        );
    }

    /// M47 Inc 10 / T6 (D1) — **both** advertising surfaces scope `--preview` to the
    /// workflows it actually serves. The verb refuses a `creates-task: false` member
    /// ("mints no task, so there is nothing to preview — run it directly"), and that
    /// refusal is *correct*: for such a workflow the direct run **is** the preview
    /// (it composes and mints nothing). The lie was in the advertising, which
    /// quantified over the whole catalog — orientation's bare `<id>` and describe's
    /// *"any one's"* — while nothing in either catalog marks which members mint, so a
    /// reader could not predict which ids the verb would refuse (law 1; the correctly
    /// scoped statement existed exactly once, in `jigc workflow --help`). Both lines
    /// now name the scope **and** the fallback for the members outside it.
    #[test]
    fn render_preview_advertising_is_scoped_to_the_minting_workflows() {
        use engine::introspect::{DefinitionKind, DefinitionProse, Description};
        use engine::result::SCHEMA_VERSION;

        let orientation = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &fixture(),
            &[],
        );
        let describe_out = describe(
            Format::Agent,
            &Description {
                schema_version: SCHEMA_VERSION,
                definitions: vec![DefinitionProse {
                    kind: DefinitionKind::Workflow,
                    id: "single-task".to_string(),
                    prose: "single-task is one end-to-end scoped change. Reach for it when the work is small enough to hold in your head.".to_string(),
                    router_hidden: None,
                    origin_pack: None,
                }],
                commands: vec![],
            },
        );

        for (surface, text) in [("orientation", &orientation), ("describe", &describe_out)] {
            assert!(
                text.contains("task-minting"),
                "the {surface} preview line must scope the affordance to the workflows \
                 the verb serves; got:\n{text}",
            );
            assert!(
                text.contains("`jigc start --workflow <id>`"),
                "and name the direct run as the fallback for a workflow that mints no \
                 task ({surface}); got:\n{text}",
            );
        }

        // describe's unscoped quantifier is gone — it is the exact byte-form of the lie.
        assert!(
            !describe_out.contains("any one's"),
            "describe must not quantify the preview over the whole catalog; got:\n{describe_out}",
        );
    }

    /// T12 (M45 Inc 10) — orientation names `jigc workflow <id> --preview` as the way
    /// to read a workflow's step text before minting, gated on a non-empty catalog. A
    /// bare-`start` reader — the most common entry — finds the preview surface here
    /// (findings §69: the pull-tier fix itself not pull-discoverable). The omitting
    /// context — an empty catalog — names no preview route (inert).
    #[test]
    fn render_orientation_clean_routes_to_workflow_preview() {
        let text = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &fixture(),
            &[],
        );
        assert!(
            text.contains("Preview: `jigc workflow <id> --preview`"),
            "orientation must route to the preview surface; got:\n{text}",
        );

        // Omitting context: an empty catalog has no workflow to preview, so no route.
        let empty = Orientation::new(Catalog::new(vec![]));
        let text = orientation_clean(
            "Pack: dev/v0.3.0 · Project config: .jigc/config",
            &empty,
            &[],
        );
        assert!(
            !text.contains("--preview"),
            "an empty catalog names no preview route (inert omit-context); got:\n{text}",
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
        Preview: `jigc workflow <id> --preview`   — read a task-minting workflow's step text without minting a task; a workflow that mints nothing has no preview — `jigc start --workflow <id>` composes it directly, and mints nothing either
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
            sub_task_of: None,
            also_open: Vec::new(),
        };

        let agent = composed(Format::Agent, &granting);
        assert!(agent.ends_with(ROUTING_FOOTER));
        assert!(agent.contains("Run: `jigc task finalize add-rate-limiter`"));
        assert!(
            agent.contains(
                "create-gates: adr, changelog   — the doc-types this task is allowed to \
                 create; any other type is refused\n— jigc"
            ),
            "the gates line sits immediately before the footer; got:\n{agent}",
        );

        // Human renders identically to agent in the MVP (TUI is post-MVP).
        assert_eq!(composed(Format::Human, &granting), agent);

        // A gate-less workflow renders no line at all — omitted, never `none`.
        let gateless = Composition {
            view: view.clone(),
            gates: Vec::new(),
            minted: true,
            sub_task_of: None,
            also_open: Vec::new(),
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

    /// M47 Inc 10 / T6 — the `create-gates:` line says what a gate **is**. M42 shipped
    /// the list (`create-gates: adr, changelog`) with the noun defined on no compose,
    /// `describe`, or help surface: the only place the binary ever explained it was the
    /// `create.gate-blocked` refusal, i.e. by tripping one (law 2 — the M42 fix moved the
    /// *list* off the refusal and left the *meaning* on it). The line now carries the one
    /// clause that defines it, and the gate-less context still renders no bytes.
    #[test]
    fn create_gates_line_defines_the_gate_it_lists() {
        let line = create_gates_line(&["adr".to_string(), "changelog".to_string()]);
        assert!(
            line.starts_with("create-gates: adr, changelog   — "),
            "the list keeps its shape and the definition follows it; got:\n{line}",
        );
        assert!(
            line.contains("this task is allowed to create"),
            "the clause must say what a gate grants — the in-task create permission; \
             got:\n{line}",
        );
        // …and it stays a *permission*, never a command-shaped fragment: a `jigc doc …`
        // string here would carry no `--task <id>` on a surface where every composed
        // `jigc doc` line is task-disambiguated by contract.
        assert!(
            !line.contains("jigc doc"),
            "the definition must not read as a runnable command; got:\n{line}",
        );
        assert!(
            line.contains("refused"),
            "and what happens outside it (the `create.gate-blocked` half); got:\n{line}",
        );
        assert!(line.ends_with('\n'), "one line; got:\n{line}");

        // The omitting context stays inert: a gate-less workflow renders no bytes at all,
        // definition included — never `create-gates: none`.
        assert!(create_gates_line(&[]).is_empty());
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
            sub_task_of: None,
            also_open: Vec::new(),
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
            sub_task_of: None,
            also_open: Vec::new(),
        };
        let re = composed(Format::Agent, &resumed);
        assert!(!re.contains("task minted"), "got:\n{re}");
        assert!(re.starts_with("Reason about the change.\n"), "got:\n{re}");

        // The `creates-task: false` (router) arm mints nothing and has no id: inert.
        let router = Composition {
            view: ComposedWorkflow { task: None, text },
            gates: Vec::new(),
            minted: false,
            sub_task_of: None,
            also_open: Vec::new(),
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
            sub_task_of: None,
            also_open: Vec::new(),
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
        // The parallelize affordance (findings §71): several open tasks are legal,
        // each addressed by its own `--task` — the default is not a one-task limit.
        assert!(
            scope.contains("several open tasks are legal") && scope.contains("parallel"),
            "the B3 statement also names the deliberate-parallelize affordance; \
             got:\n{scope}",
        );
        // …and the affordance is SCOPED, not flat (M47 Inc 8 / T2, law 1): the condition
        // it holds under, and the block that follows when it does not. The behaviour
        // behind both halves is driven end-to-end in `start_compose.rs`.
        assert!(
            scope.contains("disjoint")
                && scope.contains("blocks")
                && scope.contains("overlapping paths"),
            "the parallel claim names its condition and the overlap that blocks; \
             got:\n{scope}",
        );
        // Stack order: text, then the task-state lines, then gates, then footer.
        assert!(
            agent.contains(
                "task scope: `jigc doc` writes default to the single active task; \
                 `--task add-rate-limiter` is the explicit override and wins when \
                 several are active — several open tasks are legal, each addressed by \
                 its own `--task`, so you can run them in parallel while their work \
                 stays disjoint; once a sibling task commits a path this one also \
                 touches, resuming or finalizing here blocks and names the overlapping \
                 paths\ncreate-gates: adr   — the doc-types this task is allowed to \
                 create; any other type is refused\n— jigc"
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
            sub_task_of: None,
            also_open: Vec::new(),
        };
        let re = composed(Format::Agent, &resumed);
        assert!(!re.contains("task minted"), "got:\n{re}");
        assert!(
            re.contains("what's-left: `jigc task validate add-rate-limiter`"),
            "got:\n{re}",
        );

        // The **other unit kind** on the same axis (M47 Inc 8 / N7, law 1): on a milestone
        // sub-task the clause above is a lie — `jigc task finalize <sub>` refuses outright
        // and the milestone door blocks on *any* moved history, not on overlap — so the
        // sub-task compose names the milestone's commit boundary instead. The end-to-end
        // proof through the real binary is `start_resume.rs`; this is the seam fence.
        let sub = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: false,
            sub_task_of: Some(crate::start::SubTaskOf {
                milestone: "rework".to_string(),
                workflow: "sub-task".to_string(),
            }),
            also_open: Vec::new(),
        };
        let sub_agent = composed(Format::Agent, &sub);
        let sub_scope = sub_agent
            .lines()
            .find(|l| l.starts_with("task scope:"))
            .unwrap_or_else(|| panic!("a task-scope line renders; got:\n{sub_agent}"));
        assert!(
            !sub_scope
                .contains("resuming or finalizing here blocks and names the overlapping paths"),
            "a sub-task must not carry the top-level overlap claim; got:\n{sub_scope}",
        );
        assert!(
            sub_scope.contains("sub-task of milestone `rework`")
                && sub_scope.contains("jigc milestone finalize rework")
                && sub_scope.contains("only commit boundary"),
            "a sub-task names the milestone commit boundary that gates it; got:\n{sub_scope}",
        );
        // The shared half is unchanged — only the divergence clause is unit-kind-scoped.
        assert!(
            sub_scope.contains("several open tasks are legal"),
            "the parallelize affordance still holds for a sub-task; got:\n{sub_scope}",
        );

        // The **second** surface on the same discriminator (M51 Inc 9 / T1, law 1): the
        // `resume:` line names the door that provisions a sub-task's write-ready area —
        // the re-entry door under the task's own recorded workflow — and the worktree it
        // runs in, never `jigc start --task <sub>`, which composes and provisions nothing.
        // The end-to-end proof through the real binary is `compose_statefulness.rs`; this
        // is the seam fence, and it holds the top-level arm unchanged beside it.
        let sub_resume = sub_agent
            .lines()
            .find(|l| l.starts_with("resume:"))
            .unwrap_or_else(|| panic!("a resume line renders; got:\n{sub_agent}"));
        assert_eq!(
            sub_resume,
            "resume: `jigc workflow sub-task --task add-rate-limiter`   — re-composes this \
             workflow and provisions this sub-task's write-ready docs area on first entry; \
             run it from this sub-task's own worktree at `.jigc/worktrees/add-rate-limiter`, \
             where its work happens",
            "a sub-task's resume names the provisioning door and where it runs",
        );
        let top_resume = re
            .lines()
            .find(|l| l.starts_with("resume:"))
            .unwrap_or_else(|| panic!("a resume line renders; got:\n{re}"));
        assert_eq!(
            top_resume,
            "resume: `jigc start --task add-rate-limiter`   — re-composes this workflow if \
             context is lost",
            "a top-level task's area was provisioned at its mint — its resume line is \
             unchanged",
        );

        // The id-less router renders no bytes — the omitting context stays inert.
        let router = Composition {
            view: ComposedWorkflow {
                task: None,
                text: text.clone(),
            },
            gates: Vec::new(),
            minted: false,
            sub_task_of: None,
            also_open: Vec::new(),
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

    /// M47 Inc 4 T4 (`surface-contract.md` → law 1): the `what's-left:` line is the
    /// **highest-traffic** promise surface — it renders on every id-carrying compose —
    /// and until M47 it claimed `jigc task validate` "previews the findings finalize
    /// will gate on", flat. It does not: the preview covers the
    /// [`crate::gate_coverage::Tier::Previewed`] members and no more, while the staged
    /// set (`empty-commit`/`nothing-staged`), promotion, `stage-failed`, the untracked
    /// owner-artifact cause and the commit / hook rejection are decided only at
    /// `finalize` ([baseline](../../../completions/artifacts/M47/baseline.md) §4b).
    ///
    /// This assertion drives the **emitted** line — sliced out of the rendered agent
    /// text, not rebuilt here — so the fence is on the bytes an agent reads: the
    /// scoping clause and each covered family must be named, and the retired unscoped
    /// promise must not return anywhere in the composed surface. The id-less context
    /// (the router) stays inert: it emits no such line at all.
    #[test]
    fn whats_left_line_scopes_the_preview_to_what_validate_covers() {
        let minted = Composition {
            view: ComposedWorkflow {
                task: Some("add-rate-limiter".to_string()),
                text: "Reason about the change.\n".to_string(),
            },
            gates: Vec::new(),
            minted: true,
            sub_task_of: None,
            also_open: Vec::new(),
        };
        let agent = composed(Format::Agent, &minted);
        let line = agent
            .lines()
            .find(|l| l.starts_with("what's-left:"))
            .unwrap_or_else(|| panic!("a what's-left line renders; got:\n{agent}"));

        // It states that the preview is a *part* of the gate, never the whole of it.
        assert!(
            line.contains("previews part of the finalize gate"),
            "the line must scope its claim, not re-assert the unscoped promise; \
             got:\n{line}",
        );
        // It names each family the preview genuinely covers — read from the coverage
        // table, never re-listed here: a hand-list beside the generated line is the
        // ninth enumeration M46 Inc 6 / T2 exists to delete.
        for covered in crate::gate_coverage::members(crate::gate_coverage::Tier::Previewed) {
            assert!(
                line.contains(covered.token),
                "the line must name `{}` as covered; got:\n{line}",
                covered.id,
            );
        }
        // …and where the rest is decided.
        assert!(
            line.contains("the staged set, promotion and the commit surface at finalize"),
            "the line must say where the un-previewed rest surfaces; got:\n{line}",
        );
        // The retired promise cannot silently return — anywhere in the emitted surface.
        assert!(
            !agent.contains("previews the findings finalize will gate on"),
            "the unscoped promise is retired; got:\n{agent}",
        );

        // The omitting context (no id — the router) renders no line to scope.
        let router = Composition {
            view: ComposedWorkflow {
                task: None,
                text: "Pick a workflow.\n".to_string(),
            },
            gates: Vec::new(),
            minted: false,
            sub_task_of: None,
            also_open: Vec::new(),
        };
        let routed = composed(Format::Agent, &router);
        assert!(!routed.contains("what's-left:"), "got:\n{routed}");
        assert!(
            !routed.contains("previews part of the finalize gate"),
            "got:\n{routed}",
        );
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
    /// adjudicated top-level cross-pack collision per id-space (the `knobs` file; the
    /// `commit` doctype), the agent-text body renders **one line per
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
                    collision: "config:knobs".to_string(),
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
            agent.contains("collision: config:knobs → won by methodology/0.1.0"),
            "the knob-file collision must name the winning pack; got:\n{agent}",
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
        assert!(json_out.contains("\"collision\": \"config:knobs\""));
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
                collision: "config:knobs".to_string(),
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
            .find("collision: config:knobs")
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

        // The milestone's full live sub-task set, each id paired with whether its own area
        // staged anything — `area-idle` staged nothing, so the ack must name it as doc-less
        // (C3) rather than silently crediting it. The bit is the caller's disk read, never a
        // re-derivation from the overlay (M51 Increment 9 / T6).
        let sub_tasks: BTreeMap<String, bool> = [
            ("area-idle", false),
            ("area-low", true),
            ("area-zed", true),
            ("evict-stale-keys", true),
        ]
        .iter()
        .map(|(id, staged)| (id.to_string(), *staged))
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
                    provisioned: true,
                    worktree_unreadable: false,
                    // M47 Inc 3 (c) — the two reportable cells of the index-column
                    // partition. The wholly-staged path (`lru.py`, in the manifest above)
                    // is deliberately absent: it landed.
                    discarded: vec![
                        DiscardedWork {
                            path: "README.md".to_string(),
                            state: DiscardState::PartlyStaged,
                        },
                        DiscardedWork {
                            path: "notes/scratch.py".to_string(),
                            state: DiscardState::NeverStaged,
                        },
                    ],
                    // M50 Inc 11 / N12 — the `squash: false` chain minted this sub-task
                    // its own commit, so the ack names it instead of leaving the reader
                    // to attribute `lru.py` to the aggregate that does not contain it.
                    hash: Some("41c0de1".to_string()),
                },
                // The never-provisioned sub-task: no worktree ever existed, so its
                // `code_files: 0` is structural, and the label says so (M47 Inc 3 (b)(ii)).
                // Nothing to discard either — it never had a worktree to lose.
                SubTaskContribution {
                    id: "wire-cache-metrics-into".to_string(),
                    docs: 0,
                    code_files: 0,
                    provisioned: false,
                    worktree_unreadable: false,
                    discarded: Vec::new(),
                    // No worktree, no code, no commit of its own to name.
                    hash: None,
                },
            ],
            commits: vec![
                LandedCommit {
                    hash: "41c0de1".to_string(),
                    subject: "feat: implement LRU eviction".to_string(),
                    paths: vec!["lru.py".to_string()],
                },
                LandedCommit {
                    hash: "b546ca8".to_string(),
                    subject: "Finalize milestone cache-rework (2 sub-tasks)".to_string(),
                    paths: vec![
                        "docs/milestone-records/cache-rework.md".to_string(),
                        "docs/decisions/eviction-policy.md".to_string(),
                    ],
                },
            ],
            hook_output: "hook: fmt clean".to_string(),
            // The ordinary boundary: every sub-task area held only jigc's own files, so the
            // key is present and empty and the text says nothing (M52 Inc 4 / T4).
            displaced: Vec::new(),
        };

        let agent = milestone_finalized(Format::Agent, &landed);
        insta::assert_snapshot!(agent, @r"
        finalized b546ca8 — Finalize milestone cache-rework (2 sub-tasks)
          modified docs/milestone-records/cache-rework.md
          promoted docs/decisions/eviction-policy.md
          added lru.py
          3 files committed
          commits (oldest first, each with the paths it landed):
            41c0de1 feat: implement LRU eviction
              lru.py
            b546ca8 Finalize milestone cache-rework (2 sub-tasks)
              docs/milestone-records/cache-rework.md
              docs/decisions/eviction-policy.md
          sub-tasks: implement-lru-eviction: 1 doc, 1 code file, committed as 41c0de1 · wire-cache-metrics-into: nothing staged, no worktree provisioned
          discarded with the fan-out worktrees (not committed, not recoverable):
            implement-lru-eviction: README.md (staged only in part) · notes/scratch.py (never staged)
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
        // M50 Inc 11 / N12 — the attribution channel is data on the machine surface, and
        // the per-sub-task sha discriminates: the code-carrying sub-task names its own
        // commit, the one that minted none reads `null` rather than borrowing the
        // aggregate's sha.
        assert_eq!(value["committed"]["commits"][0]["hash"], "41c0de1");
        assert_eq!(
            value["committed"]["commits"][0]["paths"],
            serde_json::json!(["lru.py"]),
        );
        assert_eq!(value["committed"]["commits"][1]["hash"], "b546ca8");
        assert_eq!(value["committed"]["sub_tasks"][0]["hash"], "41c0de1");
        assert!(value["committed"]["sub_tasks"][1]["hash"].is_null());
        // M47 Inc 3 (b)(ii) — the degrade is data on the machine surface too, not only prose.
        assert_eq!(value["committed"]["sub_tasks"][0]["provisioned"], true);
        assert_eq!(value["committed"]["sub_tasks"][1]["provisioned"], false);
        // M47 Inc 3 (c) — the teardown's loss is data on the machine surface too, carrying
        // the same two-cell partition the prose block renders (and nothing else).
        assert_eq!(
            value["committed"]["sub_tasks"][0]["discarded"],
            serde_json::json!([
                { "path": "README.md", "state": "partly-staged" },
                { "path": "notes/scratch.py", "state": "never-staged" },
            ])
        );
        assert_eq!(
            value["committed"]["sub_tasks"][1]["discarded"],
            serde_json::json!([])
        );
        // M45 — the additive `hook_output` key rides the `committed` object, carrying the
        // captured boundary-commit hook string (`design/command-output-contract.md` →
        // Stream discipline); the agent-text arm relays it separately and omits it here.
        assert_eq!(value["committed"]["hook_output"], "hook: fmt clean");
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
            findings: Vec::new().into(),
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
          at: decisions/auth-choice.md
          route: reconcile decisions/auth-choice.md against the `adr` schema
        adoptable decisions/rate-limit.md → adr  (adopted — indexed + baselined, no file moved)
          adopted — structurally empty: 0 milestones
          adopted — 2 surplus trailing sections
        unmanaged docs/ — 1 file(s) parse against no schema (left untouched — fine to stay plain)

        What the verdicts above mean, and what to do next:
          adoptable — conformant at its managed location; adopted register-only (indexed + baselined, the file stays in place).
          needs-reconcile — parses as the named type but conflicts — in its content, its location, or its name; fix it per the row's route, then re-run `jigc ingest`.
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
            findings: Vec::new().into(),
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
            findings: Vec::new().into(),
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

        let agent = ingest(
            Format::Agent,
            &IngestReport {
                rows: Vec::new(),
                findings: Vec::new().into(),
            },
        );
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
            findings: Vec::new().into(),
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
            findings: Vec::new().into(),
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

    /// T4 (M44 Inc 7): the `--format json` projection carries a `summary` block beside
    /// `rows` — the three verdict-class counts + the adopted sub-count + the
    /// per-directory unmanaged collapse (a `BTreeMap`, so the breakdown is sorted +
    /// order-invariant). `rows` stays the full per-row projection (unchanged). The
    /// summary is a pure projection of the rows, so it is **byte-identical** under a
    /// reordered row set (id-order and its reverse), while `rows` reflects input order.
    #[test]
    fn render_ingest_json_carries_verdict_summary() {
        use crate::ingest::{IngestReport, TriageRow};
        use engine::finding::{Finding, Location, Severity};

        let unmanaged = |file: &str| TriageRow {
            file: file.to_string(),
            best_match: None,
            verdict: "unmanaged",
            finding: None,
            adopted: false,
            annotations: Vec::new(),
        };
        let adoptable = TriageRow {
            file: "decisions/keep.md".to_string(),
            best_match: Some("adr".to_string()),
            verdict: "adoptable",
            finding: None,
            adopted: true,
            annotations: Vec::new(),
        };
        let needs_reconcile = TriageRow {
            file: "decisions/auth.md".to_string(),
            best_match: Some("adr".to_string()),
            verdict: "needs-reconcile",
            finding: Some(Finding::graded(
                Severity::Blocking,
                "conformance.section-missing",
                "required section heading `## context` is missing",
                Some(Location::addressed("decisions/auth.md", 1, 1)),
                Some("reconcile decisions/auth.md against the `adr` schema".into()),
            )),
            adopted: false,
            annotations: Vec::new(),
        };
        let report = IngestReport {
            findings: Vec::new().into(),
            rows: vec![
                adoptable.clone(),
                needs_reconcile.clone(),
                unmanaged("docs/a.md"),
                unmanaged("docs/b.md"),
                unmanaged("src/x.md"),
                unmanaged("root.md"),
            ],
        };

        let json_out = ingest(Format::Json, &report);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("ingest json parses");
        // `rows` is unchanged — every row still projected, in input order.
        assert_eq!(
            value["rows"].as_array().expect("a rows array").len(),
            6,
            "rows stays the full per-row projection; got:\n{json_out}",
        );
        // The summary rolls the verdict classes up + the per-directory unmanaged collapse.
        assert_eq!(
            value["summary"],
            serde_json::json!({
                "adoptable": 1,
                "adopted": 1,
                "needs_reconcile": 1,
                "unmanaged": 4,
                "unmanaged_by_directory": { "./": 1, "docs/": 2, "src/": 1 },
            }),
            "the summary block projects verdict counts + the per-directory unmanaged collapse; got:\n{json_out}",
        );

        // Order-invariant: the same rows fed with the unmanaged rows reversed render a
        // **byte-identical** summary (the per-directory collapse is keyed on the
        // directory via a BTreeMap, never row-encounter order).
        let reversed = IngestReport {
            findings: Vec::new().into(),
            rows: vec![
                needs_reconcile,
                adoptable,
                unmanaged("root.md"),
                unmanaged("src/x.md"),
                unmanaged("docs/b.md"),
                unmanaged("docs/a.md"),
            ],
        };
        let reversed_value: serde_json::Value =
            serde_json::from_str(&ingest(Format::Json, &reversed)).expect("ingest json parses");
        assert_eq!(
            reversed_value["summary"], value["summary"],
            "the summary block must be order-invariant",
        );
    }

    /// The successful-setup summary names each installed target **and what it is for**
    /// (#9c — less-terse setup output), ending with the routing footer; the JSON shape
    /// carries the same facts as keys (`installed`/`line_file`/`allowlist_file`/
    /// `hook_file`/`hook_committed`/`guide_file`), with none of the explanatory prose —
    /// regression watch.
    #[test]
    fn render_setup_success_names_what_was_installed() {
        let summary = SetupSummary {
            line_file: "CLAUDE.md".to_string(),
            allowlist_file: ".claude/settings.json".to_string(),
            // Deliberately NOT the `.git/hooks` literal: the line carries the hooks dir
            // git resolved (D4), so a `core.hooksPath` install names where the hook
            // really landed. The prior hardcoded literal pinned that lie here.
            hook_file: "my-hooks/pre-commit".to_string(),
            // …and this witness is the shape where the hook DID ride the install commit,
            // so the hook line stands alone. Its sibling below is the other half.
            hook_committed: true,
            guide_file: Some(".claude/skills/jigc/SKILL.md".to_string()),
            findings: Vec::new().into(),
            install_commit: InstallCommit::Skipped,
        };

        let agent = setup_success(Format::Agent, &summary);
        insta::assert_snapshot!(agent, @r"
        jigc setup — adapter installed

        jigc is now wired into this project; setup installed:
          - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
          - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
          - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
          - pre-commit hook → my-hooks/pre-commit   (warn-only doc↔code drift backstop)
          - jigc guides → .claude/skills/jigc/SKILL.md   (jigc's own quickstart + migration notes, stamped with this build)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");
        assert!(agent.ends_with(ROUTING_FOOTER));
        // Human renders identically to agent in the MVP.
        assert_eq!(setup_success(Format::Human, &summary), agent);

        // JSON carries the stable keys and none of the explanatory prose — including
        // `hook_file`, which the agent line above names and the envelope used to
        // withhold (M48 Inc 7 / T2).
        let json_out = setup_success(Format::Json, &summary);
        assert!(!json_out.contains(ROUTING_FOOTER));
        assert!(!json_out.contains("wired into this project"));
        assert!(!json_out.contains("orients your assistant"));
        // …and NOT `installed`, whose literal `true` duplicated exit 0 (M51 Inc 5 / T2).
        // Asserted here as well as at the door so the renderer's own witness cannot
        // re-bless the key a behavioural sweep would then have to catch.
        assert!(!json_out.contains("\"installed\""));
        assert!(json_out.contains("\"line_file\": \"CLAUDE.md\""));
        assert!(json_out.contains("\"allowlist_file\": \".claude/settings.json\""));
        assert!(json_out.contains("\"hook_file\": \"my-hooks/pre-commit\""));
        assert!(json_out.contains("\"hook_committed\": true"));
        assert!(json_out.contains("\"guide_file\": \".claude/skills/jigc/SKILL.md\""));
    }

    /// The other half of the hooks-dir axis at the renderer: a hook the install commit
    /// could not carry — the default `.git/hooks`, the shape almost every adopter is on —
    /// is still listed under "setup installed:" (it *is* installed, locally) and now says
    /// what that costs. Red before the fix: the two shapes printed byte-identical lines,
    /// so the only shape most adopters hit was the one the door stayed silent about.
    #[test]
    fn render_setup_success_says_a_hook_the_commit_could_not_carry_is_local_only() {
        let summary = SetupSummary {
            line_file: "CLAUDE.md".to_string(),
            allowlist_file: ".claude/settings.json".to_string(),
            hook_file: ".git/hooks/pre-commit".to_string(),
            hook_committed: false,
            guide_file: None,
            findings: Vec::new().into(),
            install_commit: InstallCommit::Committed("a1b2c3d".to_string()),
        };

        let agent = setup_success(Format::Agent, &summary);
        insta::assert_snapshot!(agent, @r"
        jigc setup — adapter installed

        jigc is now wired into this project; setup installed:
          - bootstrap reference → CLAUDE.md   (orients your assistant to `jigc start` each session)
          - jigc allowlist → .claude/settings.json   (pre-approves the `jigc` commands the agent runs)
          - SessionStart hook → .claude/settings.json   (runs `jigc start` to orient your assistant each session)
          - pre-commit hook → .git/hooks/pre-commit   (warn-only doc↔code drift backstop)
            local to this checkout — git cannot track this path, so the hook is not in the install commit; a clone gets no drift backstop until `jigc setup` runs there
          - install commit → a1b2c3d   (setup's install files are committed on their own, off your first feature commit)
        — jigc · run `jigc start` for orientation; all writes through `jigc`.
        ");

        // The clause hangs off the hook's own line, not off the install commit's: an
        // install commit was made here, and it is the hook that is missing from it.
        let json_out = setup_success(Format::Json, &summary);
        assert!(json_out.contains("\"hook_committed\": false"));
        assert!(json_out.contains("\"install_commit\": \"a1b2c3d\""));
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
                    router_hidden: None,
                    origin_pack: None,
                },
                DefinitionProse {
                    kind: DefinitionKind::Doctype,
                    id: "adr".to_string(),
                    prose: "adr is a dated architectural decision record. Reach for it when a choice is worth preserving with its rationale.".to_string(),
                    router_hidden: None,
                    origin_pack: None,
                },
            ],
            commands: vec![CommandHint {
                id: "finalize".to_string(),
                pack: "dev".to_string(),
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
            agent.contains("finalize (dev pack) Validate, render the commit, and commit the task."),
            "the command-ref hint is carried verbatim, under the id and the pack that declares \
             it; got:\n{agent}",
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
            [
                ("narrated", &narrated, None),
                ("silent-workflow", &silent, None),
            ],
            std::iter::empty(),
            [("dev", &catalog)],
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
          "schema_version": 3,
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
                    // Routed as the production gate block routes it (the route floor) — and
                    // **substituted**, as P6 route-followability requires (M47 Inc 6 T3):
                    // the shipped route carries the finding's own `key.target`, never the
                    // `<address>` placeholder this fixture used to demonstrate as if it
                    // were production.
                    Some(
                        "`jigc doc set-slot commit:x#summary --from-file -` to fill the empty slot"
                            .into(),
                    ),
                ),
            ],
            &resolved,
        );

        let agent = validation(Format::Agent, &report);
        insta::assert_snapshot!(agent, @"
        blocking · file-state.hash-matches — on-disk content of `decisions/x.md` differs
          at: decisions/x.md
          route: reconcile decisions/x.md
        blocking · schema-conformance.required-slot-present — required slot in section `summary` is empty
          at: commit:x#summary
          route: `jigc doc set-slot commit:x#summary --from-file -` to fill the empty slot
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
          at: adr:cache#status/cites-code
          route: update the citation, or restore the cited symbol
        1 finding(s) — report-only at store scope (exit 0); these gate at `jigc task validate` / `jigc task finalize` / `jigc milestone finalize`.
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

    /// M47 Inc 10 / T6 (D12) — **the severity label says whose grading it is.** The trial
    /// read two `advisory · …` rows in one store report, one of which the trailer counted
    /// as gating: the severity vocabulary carries no gate information at all, and the
    /// trailer was the only discriminator. M47 inc-8 fixed the trailer's *count*; the
    /// residue is the label itself, and the fix lands **where the label prints** — a
    /// one-line note above the rows, so the reader meets it before the labels rather than
    /// inferring the scope-relativity from a trailer three lines down (law 1 + law 3).
    ///
    /// Keyed on an `advisory` row being present: `advisory` is the label whose scope the
    /// finding is about, so a report without one has nothing to clarify and renders no
    /// bytes (the omitting-context floor). The **task** view never renders it — there the
    /// severity token *is* the verdict for the transaction in hand.
    #[test]
    fn render_validation_store_says_the_severity_label_is_scope_relative() {
        use engine::finding::{Finding, Location, Severity};

        let resolved = crate::cascade_util::no_delta_resolved().expect("resolves");
        let graded = |severity: Severity, code: &str| {
            Finding::graded(
                severity,
                code,
                format!("a store-scope finding under `{code}`"),
                Some(Location::addressed("decisions/cache.md", 1, 1)),
                Some("follow the finding's own route".into()),
            )
        };

        let with_advisory = ValidationReport::new(
            vec![graded(
                Severity::Advisory,
                "schema-conformance.repeatable-populated",
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &with_advisory, &BTreeSet::new());
        assert!(
            agent.contains("severity is scope-relative"),
            "a store report carrying an `advisory` row must state whose grading the label \
             is; got:\n{agent}",
        );
        let note_at = agent
            .find("severity is scope-relative")
            .expect("note present");
        let row_at = agent.find("advisory · ").expect("the advisory row");
        assert!(
            note_at < row_at,
            "the note lands where the label prints — above the rows, not after them; \
             got:\n{agent}",
        );
        assert!(
            agent.contains("the trailer below says which"),
            "and hands the reader to the surface that does carry gate information; \
             got:\n{agent}",
        );
        // It quotes neither a command nor the per-row `(gates at finalize)` marker: this
        // report carries no marked row, and printing the literal in a sweep-wide lead
        // would re-mint the false-gate-claim class M42 closed.
        assert!(
            !agent.contains("(gates at finalize)"),
            "a report with no gating row must not print the marker at all; got:\n{agent}",
        );

        // Omitting context 1: a store report with no `advisory` row has no label to
        // clarify — no bytes, the surface stays byte-identical to before this note.
        let blocking_only = ValidationReport::new(
            vec![graded(Severity::Blocking, "doc-code.symbol-exists")],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &blocking_only, &BTreeSet::new());
        assert!(
            !agent.contains("scope-relative"),
            "no advisory row, no note (inert omit-context); got:\n{agent}",
        );

        // Omitting context 2: the **task** view — the severity token there is the verdict
        // for the transaction in hand, so the note would be false, not merely noise.
        let task = validation(Format::Agent, &with_advisory);
        assert!(
            !task.contains("scope-relative"),
            "the task view renders no store-scope note; got:\n{task}",
        );
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
        //
        // **Split on the exit axis, which is a different axis (M46 Inc 3 / T1).**
        // `schema-conformance.unadopted-instance` belongs to both: it gates nowhere *and* it is
        // `STORE_EXIT_FLIPS`' foreign-squatter member, so a report carrying it renders that
        // member's closing line and never reaches the report-only branch. Feeding it to the
        // fixture below would prove only that an unreachable branch stays silent, so it is held
        // out and driven on its own — against **both** claims — after it.
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
            !agent.contains("jigc task validate")
                && !agent.contains("jigc task finalize")
                && !agent.contains("jigc milestone finalize"),
            "every finding here gates nowhere — the trailer must not name a gate: {agent}",
        );
        assert!(
            agent.contains("report-only at store scope (exit 0)"),
            "it is still report-only, and still says so: {agent}",
        );
        assert!(
            agent.contains("8 finding(s)") && agent.contains("gates nowhere"),
            "and it states the truth — none of them gates anywhere: {agent}",
        );
        // The exit contract is untouched: gating nowhere is not the same as flipping the exit.
        assert!(!validation_store_exit_flips(&gate_nowhere));

        // **The ninth code, held out above — both claims at once (M46 Inc 3 / T1).** It gates
        // nowhere, so no door may be named; and it flips the sweep's exit, so the report-only
        // sentence — which would print `exit 0` beside a non-zero exit — must not render. The
        // two are independent: flipping the *sweep's* exit is not granting the *finding* a gate.
        let squatter = ValidationReport::new(vec![advisory(UNADOPTED_INSTANCE_CODE)], &resolved);
        let agent = validation_store(Format::Agent, &squatter, &BTreeSet::new());
        assert!(
            !agent.contains("jigc task validate")
                && !agent.contains("jigc task finalize")
                && !agent.contains("jigc milestone finalize"),
            "the adoption advisory gates nowhere — no closing line may name a door: {agent}",
        );
        assert!(
            validation_store_exit_flips(&squatter),
            "and it flips the sweep's exit — a member of the axis: {agent}",
        );
        assert!(
            !agent.contains("report-only at store scope (exit 0)")
                && agent.contains(UNADOPTED_SQUATTER_CAUSE)
                && agent.contains(STORE_EXIT_FLIP_PHRASE),
            "so the flip's own closing line renders, naming the condition and the exit: {agent}",
        );

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
            agent.contains(
                "these gate at `jigc task validate` / `jigc task finalize` / \
                 `jigc milestone finalize`."
            ),
            "a doc-code break DOES gate at the task boundary — say so, at every door that \
             carries it (M47 inc-8 / T4 added the milestone one): {agent}",
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
            agent.contains(
                "1 of them gate at `jigc task validate` / `jigc task finalize` / \
                 `jigc milestone finalize`"
            ),
            "the claim covers the gating finding only: {agent}",
        );
        assert!(
            agent.contains("the rest are store-scope advisories that gate nowhere"),
            "and disowns the gate for the store-scope-only one: {agent}",
        );

        // **Severity, not probe family** (M47 inc-8 / T4, N10 arm 1). `doc-code` is a genuine
        // task-scope family and `doc-code.title-names-symbol` is deliberately NOT a
        // `GATES_NOWHERE` member — yet it is advisory by pack default, and nothing anywhere
        // stops on an advisory. The shipped criterion counted it as gating, so the row printed
        // `advisory · …` and the trailer *"these gate at …"* two lines apart.
        let advisory_only =
            ValidationReport::new(vec![advisory("doc-code.title-names-symbol")], &resolved);
        let agent = validation_store(Format::Agent, &advisory_only, &BTreeSet::new());
        assert!(
            !agent.contains("jigc task validate")
                && !agent.contains("jigc task finalize")
                && !agent.contains("jigc milestone finalize"),
            "an advisory `doc-code` finding gates NOWHERE — the trailer must not claim a gate \
             its own row disowns: {agent}",
        );
        assert!(
            agent.contains("gates nowhere"),
            "and states what is true of it: {agent}",
        );

        // **The compose-scoped family keeps its own door** (M47 inc-8 / T4, the red-step
        // assumption that proved false). `validate_task` runs no workflow↔refs pass, so neither
        // task door nor the milestone gate can see this finding — but a hard gate does exist,
        // at compose, so the claim is re-doored rather than withdrawn.
        let compose_only = ValidationReport::new(
            vec![Finding::graded(
                Severity::Blocking,
                "workflow-refs.include-resolves",
                "include `step:not-a-step` resolves to no step file in the cascade",
                None,
                None,
            )],
            &resolved,
        );
        let agent = validation_store(Format::Agent, &compose_only, &BTreeSet::new());
        assert!(
            !agent.contains("jigc task validate")
                && !agent.contains("jigc task finalize")
                && !agent.contains("jigc milestone finalize"),
            "no task-scope path emits `workflow-refs.*`: {agent}",
        );
        assert!(
            agent.contains(
                "these gate at compose (`jigc start`), never at the task or \
                            milestone boundary."
            ),
            "the door it really has is named: {agent}",
        );
        assert!(
            !agent.contains("(gates at finalize)"),
            "and the row label shares the predicate — it must not claim a finalize block \
             either: {agent}",
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
            !agent.contains("jigc task validate")
                && !agent.contains("jigc task finalize")
                && !agent.contains("jigc milestone finalize"),
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
            agent.contains(
                "1 of them gate at `jigc task validate` / `jigc task finalize` / \
                 `jigc milestone finalize`"
            ),
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

    /// One forecast finding — the value the `findings` key carries (M51 Inc 5 / T5).
    /// Built by its production constructor, so it carries the declared `key.target` the
    /// membership seam asserts on serialization rather than a hand-shaped stand-in.
    fn forecast_finding() -> Findings {
        Findings::from(vec![engine::file_state::staged_copy_finding(
            "docs/decisions/x.md",
        )])
    }

    /// The dry-run manifest renders a titled block — the `would commit — <subject>`
    /// forecast line (M50 Inc 12 / F-7), then each entry by kind, an untracked sweep
    /// flagged distinctly — with no trailing newline; JSON carries `dry_run: true`, the
    /// same `subject`, a `manifest[]` of `{path,kind}` (kebab-case kinds), and the
    /// `findings` the forecast door reports (M51 Inc 5 / T5).
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

        let subject = "feat(cache): forecast the subject";
        let findings = forecast_finding();

        let agent = finalize_manifest(Format::Agent, subject, &included, &left_out, &findings);
        insta::assert_snapshot!(agent, @r"
        finalize --dry-run — pre-commit manifest (nothing committed)
        would commit — feat(cache): forecast the subject
          promoted docs/decisions/x.md
          left-out (unstaged/untracked — git add to include):
            scratch.txt");
        assert!(
            !agent.ends_with('\n'),
            "no trailing newline — the caller closes it"
        );
        assert_eq!(
            finalize_manifest(Format::Human, subject, &included, &left_out, &findings),
            agent
        );
        // The declared bound of the M51 rider: `findings` is a JSON-only key, so the text
        // surface is byte-identical with and without one.
        assert_eq!(
            finalize_manifest(
                Format::Agent,
                subject,
                &included,
                &left_out,
                &Findings::default()
            ),
            agent,
            "`findings` is a JSON-only key — the agent forecast keeps the shape it has",
        );

        let json_out = finalize_manifest(Format::Json, subject, &included, &left_out, &findings);
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["dry_run"], serde_json::Value::Bool(true));
        assert_eq!(
            value["subject"], subject,
            "the text's forecast subject reaches the envelope (the parity rule)"
        );
        let reported = value["findings"].as_array().expect("findings array");
        assert_eq!(
            reported.len(),
            1,
            "the forecast carries the findings the door it forecasts reports (EC-20)"
        );
        assert_eq!(reported[0]["code"], "file-state.staged-copy");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&finalize_manifest(
                Format::Json,
                subject,
                &included,
                &left_out,
                &Findings::default()
            ))
            .expect("valid JSON")["findings"],
            serde_json::json!([]),
            "nothing to report is an EMPTY array, never an absent key — a driver reads the \
             two differently",
        );
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

        let agent = finalize_manifest(
            Format::Agent,
            "feat: add it",
            &included,
            &[],
            &Findings::default(),
        );
        assert!(
            agent.contains("  added src/feature.rs"),
            "a staged new file renders `added`; agent:\n{agent}",
        );
        assert!(
            !agent.contains("swept"),
            "the retired `swept` wording must not appear on the included path; agent:\n{agent}",
        );

        let json_out = finalize_manifest(
            Format::Json,
            "feat: add it",
            &included,
            &[],
            &Findings::default(),
        );
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

        let agent = finalize_manifest(
            Format::Agent,
            "feat: carry it",
            &carried,
            &[],
            &Findings::default(),
        );
        assert!(
            agent.contains("  carried-over foreign-a.txt"),
            "a carried entry renders `carried-over`; agent:\n{agent}",
        );
        let json_out = finalize_manifest(
            Format::Json,
            "feat: carry it",
            &carried,
            &[],
            &Findings::default(),
        );
        let value: serde_json::Value = serde_json::from_str(&json_out).expect("valid JSON");
        assert_eq!(value["manifest"][0]["kind"], "carried-over");

        let advisory = carried_over_advisory(&carried);
        insta::assert_snapshot!(advisory, @r"
        finalize — about to commit the index; carrying over (staged before this task existed — declared with `--carry-staged`):
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
            hook_output: "hook: doc-code backstop clean".to_string(),
            displaced: Vec::new(),
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
        // M45 — the additive `hook_output` key rides the `committed` object, carrying the
        // captured commit-hook string the stderr relay mirrors (`design/command-output-
        // contract.md` → Stream discipline). The agent-text arm relays it to a delimited
        // section instead, so the summary above does not name it.
        assert_eq!(committed["hook_output"], "hook: doc-code backstop clean");
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

        let out = migration_review(Format::Agent, "task-1", Some(foreign), &rewrites, &[]);
        let summary = out
            .lines()
            .find(|l| l.contains("version-like token absent from the rewrite"))
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

        let out = migration_review(Format::Agent, "task-1", Some(foreign), &rewrites, &[]);
        let summary = out
            .lines()
            .find(|l| l.contains("version-like token absent from the rewrite"))
            .unwrap_or_else(|| panic!("the review must render the fidelity summary; got:\n{out}"));
        assert!(
            summary.trim_end().ends_with(": 1.5"),
            "the dropped 1.5 is reported without the sentence dot; got:\n{summary}",
        );
    }

    /// The version-scan is word-boundary-guarded (Hardening #5, the M44 calibration
    /// fix): a dotted-numeric run whose preceding byte is ASCII-alphanumeric or `-` is
    /// **not** a version token (the `project-alpha-2.0` → `2.0` false alarm), while a run at
    /// a genuine word boundary — start-of-string, after whitespace, a trailing sentence
    /// dot — still yields its token.
    #[test]
    fn render_scan_version_tokens_word_boundary_guard() {
        // Glued to `-` (the project-alpha-2.0 false alarm) yields nothing.
        assert!(
            scan_version_tokens("project-alpha-2.0").is_empty(),
            "a dotted run glued to `-` is a slug fragment, not a version",
        );
        // Glued to a mid-word letter likewise yields nothing (`v` is part of `dev`).
        assert!(
            scan_version_tokens("dev2.0").is_empty(),
            "a dotted run glued to a mid-word letter is not a bare version token",
        );
        // A lone `v`/`V` version marker at a word boundary IS the conventional prefix —
        // the run is a bare version (the M44 completion-audit refinement).
        assert_eq!(scan_version_tokens("v2.0"), vec!["2.0".to_string()]);
        assert_eq!(
            scan_version_tokens("since v1.0.0"),
            vec!["1.0.0".to_string()]
        );
        assert_eq!(scan_version_tokens("V3.4"), vec!["3.4".to_string()]);
        // A run at start-of-string is a version.
        assert_eq!(scan_version_tokens("1.0.0"), vec!["1.0.0".to_string()]);
        // After whitespace is a boundary.
        assert_eq!(
            scan_version_tokens("since 1.0.0"),
            vec!["1.0.0".to_string()]
        );
        // A trailing sentence dot still trims to its token.
        assert_eq!(scan_version_tokens("since 1.5."), vec!["1.5".to_string()]);
    }

    /// The fidelity summary is **report-split** (Hardening #5, the M44 report-split): a
    /// package-qualified `pkg@version` and a bare version token render under two
    /// **distinct** labels, so the reviewer sees the package binding a bare scan drops.
    /// Advisory / display-only / no gate (Framing A) — unchanged.
    #[test]
    fn render_migration_review_fidelity_report_split_packaged_vs_bare() {
        let foreign = "# Deps\n\nUses lodash@1.0.0 and targets the 3.4 platform.\n";
        let rewrites = vec![(
            "docs/prd/deps.md".to_string(),
            "# Deps PRD\n\n## Context\n\nDependencies to be decided.\n".to_string(),
        )];

        let out = migration_review(Format::Agent, "task-1", Some(foreign), &rewrites, &[]);

        let pkg_line = out
            .lines()
            .find(|l| l.contains("package@version absent from the rewrite"))
            .unwrap_or_else(|| {
                panic!("the review must render the package@version label; got:\n{out}")
            });
        assert!(
            pkg_line.contains("lodash@1.0.0"),
            "the dropped package@version must name lodash@1.0.0; got:\n{pkg_line}",
        );

        let bare_line = out
            .lines()
            .find(|l| l.contains("version-like token absent from the rewrite"))
            .unwrap_or_else(|| {
                panic!("the review must render the version-like token label; got:\n{out}")
            });
        assert!(
            bare_line.contains("3.4"),
            "the dropped bare version must name 3.4; got:\n{bare_line}",
        );

        // The two categories render on DISTINCT lines — the package binding is not
        // collapsed into the bare scan (`lodash@1.0.0` must not surface as a bare `1.0.0`).
        assert_ne!(
            pkg_line, bare_line,
            "the two categories render on distinct lines"
        );
        assert!(
            !bare_line.contains("1.0.0"),
            "the packaged version must not double-count as a bare token; got:\n{bare_line}",
        );

        // The advisory framing survives (Framing A — never a second structural authority).
        let lower = out.to_lowercase();
        assert!(
            lower.contains("fuzzy") || lower.contains("heuristic"),
            "the fidelity summary must carry the fuzzy/heuristic label; got:\n{out}",
        );
    }

    /// A conventional `v`-prefixed release dropped by the rewrite surfaces as a dropped
    /// version-like token (the M44 completion-audit refinement — the word-boundary guard
    /// no longer swallows the `v`/`V` version marker on both scan sides), while a
    /// slug-glued dotted run (`project-alpha-2.0`) stays unreported (the guard still holds).
    /// Advisory / display-only / no gate (Framing A) — unchanged.
    #[test]
    fn render_migration_review_reports_dropped_v_prefixed_version() {
        let foreign = "# Changelog\n\n## v1.0.0\n\nInitial cut of project-alpha-2.0.\n";
        let rewrites = vec![(
            "CHANGELOG.md".to_string(),
            "# Changelog\n\n## Unreleased\n\nNothing yet.\n".to_string(),
        )];

        let out = migration_review(Format::Agent, "task-1", Some(foreign), &rewrites, &[]);

        let token_line = out
            .lines()
            .find(|l| l.contains("version-like token absent from the rewrite"))
            .unwrap_or_else(|| {
                panic!("the review must render the version-like token label; got:\n{out}")
            });
        // The dropped `v1.0.0` release surfaces (as its bare `1.0.0` token).
        assert!(
            token_line.contains("1.0.0"),
            "a dropped v-prefixed version must surface as a dropped token; got:\n{token_line}",
        );
        // The slug-glued `project-alpha-2.0` run stays unreported (the guard holds).
        assert!(
            !token_line.contains("2.0"),
            "a slug-glued dotted run must not surface as a dropped token; got:\n{token_line}",
        );
    }
}
