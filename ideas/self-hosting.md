# Self-hosting — distill the harness into a jigc pack, dogfood it on a fresh project

**Status: parked, exploratory.** A direction, not a locked design. Recorded 2026-06-04 after the M6 run; see [VISION.md](../VISION.md#open-questions) for the index entry. This is the project's designed terminus — the workflow docs already call the hand-run loops *"the dogfood for the product workflow we will eventually compose."* Notation illustrative.

## The idea

After the current roadmap is built, **distill the accumulated harness** — decisions, the hand-run workflows ([milestone-planning](../implementation/milestone-planning-workflow.md) / [increment](../implementation/increment-workflow.md) / [dev](../implementation/dev-workflow.md) / [milestone-completion](../implementation/milestone-completion-workflow.md)), and the learnings folded into them — **into a jigc *methodology pack***, then **bootstrap a brand-new project with jigc driving that pack**, measure how it performs against the hand-run baseline, and improve from there. The ultimate test of jigc is whether it can run its own development methodology on a fresh codebase.

Three moves:
1. **Distill** — graduate the harness from "prose + agents we run by hand" into a clean, portable, project-agnostic form (the "harness graduation" pass, below).
2. **Encode as a pack** — express the workflows as jigc workflow-definitions + the gates as probes/steps, so jigc *composes* the methodology instead of an operator hand-running it.
3. **Dogfood + measure + improve** — start a real new project through jigc, track the same build-health metrics we already use (halts · fix-rounds · audit-findings), compare to the hand-run runs (M1–M6), and feed the deltas back.

## Why this is on-thesis, not a side-quest

- **It's the designed finish line.** [milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) says running the loop by hand *is* the dogfood for the product `milestone-planning` workflow. This idea is that sentence cashed in.
- **Distilling-into-jigc is a forcing function on our own lessons.** Every operator-discipline lesson (the fragile prose ones — "spike the acceptance flow," "pin a check's scope," "don't price an unspiked fork") hits a binary when you try to encode it as a workflow step or probe: it either **becomes a mechanical step/probe** (now enforced, no longer fragile) or it is revealed as **irreducible LLM judgment** that stays the agent's job. Both outcomes are wins — the exercise *sorts our harness into mechanizable vs irreducible*, which is precisely the [determinism boundary](../VISION.md) the product exists to draw. The dogfood doesn't just test jigc; it audits which of our habits are real structure vs tribal.

## The honest caveat — distilling-into-jigc ≠ automating the judgment

The determinism boundary holds: the semantic gates — gap-detection, design-review, reuse-spiking, "is this design sound" — are **LLM work by definition** and stay agent-authored. jigc would own the **structure** (compose the workflow, place every write, walk the deterministic gates, resolve the cascade), leaving the agent the prose and the judgment calls. So "the methodology becomes a pack" means *jigc orchestrates the loop and the agent fills the judgment slots* — the thesis, not a compromise. A workflow step that says "spawn N gap-detectors and settle the forks with the human" is jigc composing the *structure* of a judgment step, not automating the judgment.

## The "harness graduation" pass (the Distill move, in detail)

A one-time pass over the accumulated harness, four parts (surfaced in the post-M6 retro, 2026-06-04):

1. **Portability** — confirm each hardening reads *principle-first with jigc as example*, not jigc-coupled. Several are welded to concrete ids (`slot-fill-orphan`, `override-default`, `{{include:}}`); fine as illustration, risky if the concrete case is the whole statement. A check that only fires on jigc-shaped defects won't transfer.
2. **Rationale index** — a short *"each gate ↔ the failure class it prevents"* digest. Today the rationale lives in chronological [DECISIONS.md](../DECISIONS.md) history; a check whose motivating failure isn't legible is one a future team deletes as ceremony. (Cheapest, highest-value piece — mostly distillation of what's already in DECISIONS.)
3. **Enforced vs trusted** — mark each hardening *mechanically-enforced* (the gate, the validator/audit agents — robust) vs *operator-discipline* (the prose lessons — fragile, lapse silently for a fresh or different operator). The fragile ones are exactly what encoding-as-jigc-steps would harden.
4. **One forward-looking adversarial review of the *process itself*** — the [design-reviewer](../implementation/milestone-planning-workflow.md) move, aimed at the harness: *"what class of defect could our gates still pass that we haven't personally hit?"* The single highest-leverage item, because **all our hardening to date is reactive** — every check is a post-mortem of a failure we already ate. This is the only step that breaks the reactive cycle before a fresh project re-learns a 7th failure class the hard way.

## What the M7 run added (2026-06-04) — the pass, exercised early

M7 (the first off-substrate milestone since M3) ran part 4 *ahead of the build* as a Settle hedge, and the result sharpens this idea concretely — including a near-literal confirmation: part 4 predicted "a fresh project re-learns a **7th failure class** the hard way," and M7's forward review named exactly that — a 7th class, *single-execution determinism trust*, now [increment-workflow.md](../implementation/increment-workflow.md)'s second principle.

- **Part 4 is repeatable per-novel-milestone, not a one-time graduation pass.** It paid off *before dogfooding*, as a conditional Settle phase keyed on novelty — it found hardening #7 *before* the build, the first gate this project added ahead of its failure rather than as a post-mortem. The distilled methodology should compose it as a conditional step (off-substrate → run it; substrate-riding → skip), not a single pre-dogfood event. (Now wired into [milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) → Settle.)
- **#7 is this idea's first clean *mechanizable vs. judgment* data point.** Its obligations — ≥2 divergent orders incl. reverse, no *unsorted* hash-iteration reaching output, forced-overlap fixtures, `read_dir`≠id-order — are **mechanizable** (a determinism-test helper + a clippy-style lint). The review that *found* the class is irreducible **judgment**. The forcing function works exactly as claimed: encoding #7 sorts cleanly into a probe/lint, while the forward review stays the agent's. Use it as the worked example when distilling.
- **Two fragile operator-disciplines surfaced as mechanization candidates** (part 3's *enforced-vs-trusted* split):
  - **Halt→resume is hand-edited script surgery.** Resuming a halted build means editing the workflow script to force exactly one cache-miss on the halted call (the rest replays from cache). It worked but is error-prone operator-discipline; in a jigc-composed methodology, "re-run the halted phase once the human clears the blocker" should be a **structural** primitive (it is the same machinery the standing [progress/resumption](../design/workflow-dialect.md#open-questions) open question needs).
  - **Deferrals evaporate unless resurfaced at a trigger.** "We'll settle X before milestone N" lived only in scattered DECISIONS prose and was forgotten — so [decisions-pending.md](../implementation/decisions-pending.md) gained a **milestone-keyed "Before planning" ledger**, consulted at planning Scope. That ledger is itself a **mechanizable structural artifact**: jigc could surface "topics due before milestone N" automatically at plan time, rather than relying on a human to consult a doc — a concrete instance of the methodology-as-composed-structure the idea bets on. (The trigger-keying is the missing piece VISION → Open questions never had.)

## What the M8 run added (2026-06-05) — the determinism boundary applies to the harness itself

M8 (the control-plane half of milestone-execution — the second off-substrate milestone) ran part 4 again at Settle and it paid off again, but the sharpest contribution is a **new altitude for the mechanizable-vs-judgment sort**: M8's headline proof is, *by the determinism boundary the product exists to draw*, outside what any automated gate can run — so the boundary recurses onto the methodology pack.

- **The genuine-spawn proof is the harness's own non-deterministic, assistant-owned step.** M8's acceptance needed a real concurrent Task-tool spawn, which a headless build/Workflow subagent *cannot* perform (it can't spawn the Task tool). The resolution was **not** "write a probe" — it was a **named owner (the orchestrator/main session) + a blocking completion artifact + the automated gate declaring "I did not run this."** This is the third sort-outcome the idea must model, beside *mechanizable step* and *irreducible judgment*: an **owner-assigned, recorded artifact** for a proof that is real but un-automatable because it lives on the LLM/assistant side of the boundary. A distilled methodology pack must be able to compose a gate of the form "owner O runs this, records artifact A, and the automated half flags that it didn't" — not just probes and judgment slots. (Wired: [milestone-completion-workflow.md](../implementation/milestone-completion-workflow.md) → the spawn-class artifact; the e2e-tester declares its headless limit.)
- **Two more clean mechanizable lints surfaced** (both audit-discovered, both candidates the forcing function sorts to *structure*):
  - **Acceptance-flow *commands* are parse-against-the-grammar checks.** A `$ jigc …` line in a worked-example acceptance flow is a CLI-surface claim; checking each against the built clap surface (verb exists, flags combine) is a mechanizable lint. M8's inc-5 halt was exactly this gap, applied to outputs but not invocations — now widened in [milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) → Settle, and a clean "this becomes a lint" data point.
  - **A golden widened to *admit* output is a masking test at the snapshot layer** — mechanizable as a snapshot-diff lint (output-*adding* diff → flag; the *same* addition across N goldens → stronger flag, a propagated leak). M8's `sub-task` catalog leak hid in a routing test + three production goldens, all green; the per-increment gate passed it, the audit caught it. Now named in the scope-honesty face.
- **The fork-refusal that produced the inc-5 halt is the judgment side, and it worked.** The build-planner *refused to silently pick* between two contradicting locked docs and halted — exactly the irreducible-judgment behaviour the boundary protects (a model-free composer cannot resolve a genuine design fork). The mechanizable lint (above) would have moved the catch earlier *to Settle*, but the *refusal to guess* stays agent/owner judgment. A distilled pack must preserve the halt-on-fork, not lint it away.

The M8 net for the idea: the sort is **three-way, not two-way** — mechanizable step · irreducible judgment · owner-assigned recorded artifact — and the third bucket is forced by the determinism boundary turning on the harness's own proofs.

## Honest risks this idea has to carry

- **Reactive-only hardening.** See part 4 above. A new project inherits our 6+ post-mortems but still learns *its* novel failure the hard way unless we probe forward first.
- **A clean run is ambiguous evidence.** M6 scored best-yet, but that could mean the process is excellent, the milestone was easy, *or* the gates are now tuned to the failure shapes we've seen and blind to one we haven't. The dogfood's metrics must be read with this in mind — "jigc ran it clean" is not "jigc ran it well" without a hard milestone to stress it.
- **The final net is a single fallible operator.** After the audit, the last line is the human-facing reviewer — fallible (the M6 retro caught it erring twice in one session: a propagated stale count, a mis-scoped option). Encoding gates as jigc steps moves load off that net, but the irreducible-judgment slots still rest on it.
- **Capability prerequisites.** Self-hosting presupposes machinery not yet built: at least milestone execution (fan-out/join, [M7](../implementation/roadmap.md)) and project setup (new + existing, [M8](../implementation/roadmap.md)), plus **workflow-as-managed-artifact** support deep enough to express multi-phase, human-gated, subagent-fan-out workflows as jigc definitions — likely new doctypes/dialect surface beyond the current MVP. This is why it parks *after* the roadmap, not during.

## What "done / good" would look like

The dogfood succeeds when a fresh project, driven by jigc composing the methodology pack, **matches or beats the hand-run build-health trend** (M3 = 3 halts → M6 = 0 halts / 0 fix-rounds / audit-clean) on a milestone of comparable difficulty — *and* the irreducible-judgment slots (gap-detection, design-review, settle) are still genuinely exercised by the agent, not hollowed. The improvement loop then runs on the gap between jigc-driven and hand-driven.

## Open threads

- **Where the line falls** between "jigc composes the structure of a judgment step" and "the step is pure agent judgment jigc only points at" — the encode-as-pack move will surface concrete cases.
- **What new doctype/dialect surface** a multi-phase, human-gated, fan-out workflow needs (progress/resumption is already a [workflow-dialect open question](../design/workflow-dialect.md#open-questions)).
- **Which new project** to dogfood on — ideally one unlike jigc, to stress the portability pass (a jigc-shaped project would flatter jigc-shaped gates).
- **Candidate home when de-parked:** a `design/self-hosting.md` (or fold into the bootstrap / milestone-execution docs), once the prerequisites land and the direction locks.
