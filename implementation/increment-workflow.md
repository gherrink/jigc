# Increment workflow (plan → execute → validate → fix)

The loop for **building one increment** of the MVP — how a whole [roadmap](roadmap.md) increment goes from "not started" to "validated and committed." It is the **outer** loop that orchestrates the [dev workflow](dev-workflow.md) (the inner, per-task TDD loop) across every task of an increment, then **independently validates** the result against the roadmap and **fixes until clean**.

Five loops, five homes: [design-workflow](design-workflow.md) governs *decisions* (discuss → converge → write); [dev-workflow](dev-workflow.md) governs a *single code task* (scope → red → green → refactor → gate → commit); the [milestone-planning workflow](milestone-planning-workflow.md) *opens* a milestone (scope → detect gaps → settle → review → decompose); **this doc** governs a *whole increment* (plan → execute → validate → fix); and the [milestone-completion workflow](milestone-completion-workflow.md) is the outer one that, once all of a milestone's increments are built, independently audits the whole assembled milestone and remediates before it ships. Like the dev loop, it is shaped as named, ordered phases on purpose — it maps onto the planned `milestone > increment > task` work-units ([structural-grammar.md](../design/structural-grammar.md) → Work-units): the increment is "the unit `milestone-planning` decomposes a milestone into," so running this by hand now (and its [planning bookend](milestone-planning-workflow.md)) is the dogfood for the `milestone-planning` workflow we will eventually compose.

## Precondition: settle the gates first

**Before an increment runs, the genuine design decisions it forces are decided and logged.** [decisions-pending.md](decisions-pending.md) is the forward look — each `(D)` it carries for the increment (and any cross-cutting one due before it) is resolved *up front*, recorded in [DECISIONS.md](../DECISIONS.md), and graduated out of the pending list. Only then is execution autonomous.

This is the discipline that lets the loop run unattended without violating [CLAUDE.md](../CLAUDE.md)'s rule that **design decisions are surfaced to the human, never silently picked**. A gate decided in advance is a gate the executor *implements*; an undecided gate hit mid-run is a **halt** (below). The complement holds too: *doc-elaboration pins* — the concrete on-disk form a locked doc leaves illustrative (canonical byte layout, task working-area files, edge-index format, injection markers) — are **made and logged by the executor**, not halted on, because they are elaborations *within* a settled spec, not open forks.

## The loop (one increment)

1. **Plan** — cut the increment into ordered, **single-concern tasks** straight from the [roadmap](roadmap.md) increment (its *Deliverable* / *Grouped scope* / *Proves*) and the design docs it names. Each task carries an **observable done-criterion** (a named test that passes, or a command that exits clean with asserted output) — the same bar the dev loop's Scope step demands, set here so every task has a real red step. Keep the cut **minimal**: the smallest set of tasks that delivers the increment's Deliverable, no creep into later increments. *Halt* if planning surfaces a genuinely **new fork** not covered by the settled gates and not resolvable from the locked docs.

2. **Execute** — run each task through the [dev workflow](dev-workflow.md) verbatim (scope → red → green → refactor → gate → commit). Tasks run **strictly serially**: they share one working tree and each ends in one commit, and the spine is linear (each task builds on the last). One task = one focused concern = one conventional commit to `main`. *Halt* on a blocked task (a real ambiguity, a new fork, or a green gate unreachable without overstepping scope).

3. **Validate** — an **independent, read-only** check of the increment against the roadmap, run by an agent that **did not build it and cannot edit or commit** (so it cannot certify its own work). It verifies, with concrete evidence:
   - **gate green** from a clean state (`fmt --check` · `clippy -D warnings` · `test` · `build`);
   - **deliverable holds** end-to-end — by *running the real binary / commands and observing output*, not trusting the tests alone;
   - **proves** holds — the increment proves what the roadmap says it proves;
   - **invariants** honored ([CLAUDE.md](../CLAUDE.md) — engine makes no LLM calls / ships empty / presentation-free; CLI-locates / engine-resolves; result types are the Serialize contract; slots vs placeholders distinct; splice round-trip discipline; deterministic composition; transactional finalize);
   - **scope honest** — *the critical check*: no inert/dead feature (a flag parsed but ignored), no stub presented as done, no `todo!()` masquerading as complete, no creep into a later increment, no tautological test that asserts nothing real.

   It returns **findings**, each *blocking* (the deliverable/proves/invariants/scope are not genuinely met) or *advisory* (real but non-blocking).

4. **Fix until valid** — for each **blocking** finding, a [dev-workflow](dev-workflow.md) fix task (red that fails *because of the defect*, minimal green, gate, commit) → then **re-validate**. Bounded to **3 rounds**. *Halt* if blocking findings remain after the cap — thrash is a signal for a human, not for a fourth round. Advisories are surfaced, not gated.

The increment is **done** when validation passes (zero blocking findings, gate green, deliverable genuinely holds). The next increment then begins on top of it.

## Validation hardening (audit-derived)

An external, adversarial audit of the first autonomous run (increments 2–6) found two real issues the per-increment validator had passed over — a reachable parser panic on out-of-band input, and an MVP-scope component built and unit-tested but never wired to a command. Both are fixed; the lessons fold back here as **mandatory validator checks**, because they are the exact blind spots an automated validator inherits:

1. **Every grouped-scope bullet, not just the headline.** "Deliverable holds" must be asserted for *each* bullet of the roadmap increment's *Grouped scope*, every one **exercised through the binary** — not only the marquee acceptance path. A passing headline flow can hide a sibling sub-deliverable that has no command surface.
2. **`pub`-but-test-only is a scope finding.** A `pub fn` whose only callers live under `#[cfg(test)]` is dead in production — and it slips past a `#[allow(dead_code)]` grep *precisely because it is public*. The validator must trace that every shipped capability is reachable from a command path, and flag any public surface invoked only from tests.
3. **Parsers get a hostile-input pass, not only round-trip-on-conformant.** Property/round-trip tests over *conformant* input prove losslessness but never exercise the malformed-input contract ("every mismatch is a located finding, never a panic"). Any parser or reader must be fuzzed on hostile/foreign input — non-ASCII, BOM, truncation, mixed EOL — as its own check.

These turn the scope-honesty check from "grep for dead-code markers" into "prove each capability is reachable through the binary, and each parser is panic-free on garbage."

## Halt and resume

The loop **stops and surfaces to the human** at exactly three points: a **new fork** at Plan, a **blocked task** at Execute, **still-blocking after 3 rounds** at Validate. A halt returns a structured report — which increment, which phase, the precise reason — and is **resumable**: prior committed work stands, completed steps replay from cache, and the run continues once the human clears the blocker. Nothing is guessed past a point the design says is the human's.

## Orchestration

Run by hand, the phases above are steps one person walks. Run as an orchestrated workflow, **each phase is its own agent**, so the loop's independence is structural rather than self-policed:

- **Plan** — one planning agent per increment (just-in-time, after the prior increment lands), emitting the ordered task list.
- **Execute** — one agent **per task**, each running the whole [dev workflow](dev-workflow.md) loop to a single commit; tasks run serially over the shared working tree.
- **Validate** — one **independent, read-only** agent (it cannot edit or commit).
- **Fix** — one agent **per blocking finding**, each a dev-workflow fix task, then re-validate; bounded to 3 rounds.

Halts (above) surface to the human; nothing is auto-resolved past a fork.

## Why this shape

Framing notes, not extra phases:

- **Independent validation is the payoff.** The builder cannot grade itself; a fresh, read-only, adversarial pass against the roadmap is what catches *scope dishonesty* — the inert flag, the stub-as-done, the test that asserts nothing — which a self-check structurally misses. (This is precisely the class of gap a by-hand review caught once and this phase now catches every time.)
- **Gates-first is what makes it autonomous.** Deciding the forced `(D)`s up front converts "stop and ask mid-run" into "implement a settled decision," so the loop honors the surface-decisions guardrail *and* runs unattended.
- **It is the dev workflow's wrapper.** Execute delegates to [dev-workflow](dev-workflow.md) unchanged; this doc owns only the orchestration around it — the cut, the independent gate, the bounded fix loop. Cross-reference, never restate.
- **Serial by necessity.** Shared working tree, one commit per task, a linear spine — the loop is sequential by construction, the same property that makes each commit a clean, reviewable unit.
