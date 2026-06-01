# Milestone completion workflow (audit → plan → fix → re-verify)

The loop for **finishing a milestone** — the *completion* half of the milestone lifecycle, the bookend to [milestone planning](milestone-planning-workflow.md). **Planning** a milestone (detecting gaps, then decomposing it into ordered increments) is the [milestone planning workflow](milestone-planning-workflow.md); building those increments is the [increment workflow](increment-workflow.md). This doc owns only what comes *after* the last increment lands: once a milestone's increments are all built and individually validated, independently **audit the whole assembled milestone** and remediate before calling it shipped.

It is the outermost *execution* loop, completing the set: [design](design-workflow.md) governs a *decision*, [dev](dev-workflow.md) a *task*, [milestone planning](milestone-planning-workflow.md) the *opening of a milestone*, [increment](increment-workflow.md) an *increment*, and this one a milestone's *close* — the `milestone > increment > task` hierarchy ([structural-grammar.md](../design/structural-grammar.md) → Work-units), bracketed end to end.

## Why a separate audit (the per-increment validate isn't enough)

The increment workflow already validates each increment. So why audit again at the milestone boundary? Two reasons, both learned the hard way:

- **In-line validators share the builders' blind spots.** A validator spawned inside the build loop — even an independent one — inherits the same assumptions about what "done" means. It checks the increment's *headline* deliverable; it does not adversarially attack the finished whole.
- **Per-increment validation checks one slice.** Integration defects that span increments, latent failures on hostile input, and emergent gaps only appear when the *entire assembled system* is attacked end to end.

The evidence: the first milestone's external audit found a reachable parser panic and an MVP-scope differentiator that was built and unit-tested but **never wired to a command** — issues that five in-line increment validators and 177 passing tests had all passed over. An external, adversarial pass over the finished artifact is **not optional**; it is the milestone's acceptance gate.

## The loop (one milestone)

1. **Audit** — two **independent, adversarial** passes, run in parallel:
   - **Code review** — a read-only agent that **did not build the milestone and cannot edit or commit** (so it cannot certify its own work), reviewing the whole milestone diff for correctness bugs, invariant violations, and *scope honesty* (inert/dead features, stubs-as-done, `pub`-but-only-called-from-tests, tautological tests, creep). Severity-ranked findings with file:line evidence.
   - **End-to-end tests** — an agent that drives the **real binary** through the milestone's [worked-example](../design/worked-examples.md) flows in throwaway repos — *not* trusting the builders' own tests. Per-scenario pass/fail with concrete repro for each failure.

   Run them concurrently (the reviewer reads committed history; the e2e author writes test files but does not commit — no contention). Output: ranked findings + a green/red verdict on whether the milestone's deliverable genuinely holds.

2. **Plan (triage)** — *the human-in-the-loop step.* For each finding: **verify it is real** (reproduce it — a red test, a source trace — because an audit is a hypothesis generator, not an oracle), severity-rank it, and decide the remediation scope. **Surface genuine scope and judgment calls** rather than silently deciding them: must-fix-now vs. track-as-known; and whether a "fix" is warranted at all — a finding's right resolution is sometimes to *document a deliberate coupling* rather than build machinery (generality for a single-use thing is itself a defect — [PRINCIPLES](../PRINCIPLES.md) → Keep it minimal). The human owns this gate.

3. **Fix** — each confirmed finding through the [dev workflow](dev-workflow.md) (red → green → refactor → gate → commit), **one finding per commit**. The red test *is the finding reproduced*; a finding that a capability is unwired flips its e2e scenario from documenting-the-gap to a **regression test**. Small fixes run inline; larger ones (and anything design-sensitive, with a scope go/no-go gate) are **delegated to an executor agent** on the same loop. Strictly serial — shared working tree, one commit at a time.

4. **Re-verify** — the full gate green (`fmt` · `clippy -D warnings` · `test` · `build`) **and** re-run the affected audit slice to confirm the finding is actually closed. **Loop to clean:** fixes regress and audits miss, so repeat audit-relevant checks until the slice and the gate are both green.

The milestone is **done** when every confirmed finding is closed (or consciously tracked), the gate is green, and the deliverable holds under the re-run end-to-end flows.

## Why this shape

- **External adversarial audit is the acceptance gate.** Only a fresh pass over the finished whole — by something that did not build it and cannot fix it — catches the integration, latent, and emergent issues that in-line, per-slice validation structurally misses.
- **Verify before fixing.** The audit generates hypotheses; reproduce each as a red test so you fix a real thing and prove the fix. A wrong finding fixed is churn.
- **The human owns triage.** Scope ("now or later") and judgment ("is this fix over-engineering?") are decisions, not mechanics — they surface, never get silently picked.
- **It is the dev workflow's customer.** Every fix is a dev-workflow task; this doc owns only the audit, the triage, and the loop around them. Cross-reference, never restate.
- **Loop to clean, don't one-shot.** Re-verification against the audit *and* the gate is what makes "milestone complete" a verifiable claim rather than a hopeful one.
- **Orchestration.** Run as a workflow, the two audit passes are two parallel agents and each confirmed fix is its own agent (a dev-workflow task) — the agent-per-phase mapping the [increment workflow](increment-workflow.md) → Orchestration states.
