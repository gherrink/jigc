# M17 verdict — dogfood + measurement

**Written 2026-06-13**, after the pilot's three-arm single-task comparison (flow 22) and the
self-hosting dogfood (flow 21) both ran live through the pinned binary, and **revised the same day
after an adversarial cross-model (Codex) pass** that caught the first draft still flattering the
project (its findings are folded in below — the revision moves the claims *down*, toward what the
evidence actually carries). This is the milestone's honest ruling: prose over the mechanized facts,
the verdict itself a judgment slot, never an engine opinion ([measurement.md](../../../design/measurement.md)
→ The three-way cut). M17 set out to prove the founding thesis — *jigc makes a coding agent
measurably more correct/effective than a static `CLAUDE.md`* — on real projects.

## The one-sentence verdict

**M17 did not demonstrate the comparative thesis.** The one head-to-head jigc-vs-static comparison
**tied**; the self-hosting run is an **uncontrolled existence proof** (jigc's mechanisms work
end-to-end at native grain) with no static control, so it cannot establish "more correct/effective
than a static `CLAUDE.md`." What M17 earned is a **motivated hypothesis, not a conclusion**: jigc's
differentiating value, if it exists, lives at the methodology's native grain with a fitting domain
pack — the regime the comparison never reached. That hypothesis is **still untested** (the cases
that would test it were not run). The thesis is neither confirmed nor refuted; it is **relocated**
to where it must now be earned.

## What the two runs established

### 1 · Foreign-project single task: jigc ≈ static methodology (flow 22)

Three arms on `gherrink-galey` (a real TS task: heading `maxLevel` consistency), bare intent issued
identically, arm order B→A→C, pre-registered:

- **All three landed a correct fix (<5 min, A1–A6 pass).**
- **A (jigc) ≈ C (static methodology).** Both produced **test-first** changes (A deliberately, via
  the composed `implement` step; C reactively — its test then *caught a bug* A and B didn't make)
  and quality, correctly-scoped commits; both flagged the pre-existing lint debt. jigc's adapter
  routed the agent **unprompted** on this session (one data point, not a rate). jigc carried
  slightly more ceremony (9 invocations, one exit-1 CLI fumble).
- **B (control = GSD, the incumbent method — *not* "no method")** was quickest via its fast skill but
  **skipped the test** — correct fix, uncovered regression. So A/C were *more test-disciplined than
  GSD's fast path on this task*; this is not a "beats no-method" result.

**Reading:** for a small, spec-less code fix the *methodology content* drives the quality, not jigc's
dynamic machinery. jigc **matched** the same methodology written as a flat file; it did not beat it.
**This is the only controlled jigc-vs-static evidence M17 produced, and it is a tie.**

### 2 · Why this comparison can't reach the differentiators (the structural finding — bounded)

The `dev-task` workflow used in the comparison has **no `allows-create`**: it produces only a
transient `commit` and touches **none** of jigc's differentiators (managed docs, validation over
them, drift/reconciliation, forward-ref integrity, supersession). So *this comparison shape*
validates the adapter + methodology layer + capture apparatus but **cannot** test the
differentiators. **What this does and does not license:** it licenses "a lone `dev-task` ties" — it
does **not** license "any code task ties" or "a foreign spine comparison is impossible." A code task
that adopted existing managed docs, adjudicated doc↔code, or ran under a project-fitted pack *could*
engage the differentiators; those comparisons were simply **not run**, not proven impossible. The
honest obstacle is pragmatic-plus-structural: no off-the-shelf foreign project ships a jigc domain
pack, and authoring one per comparison is real work M17 did not do.

### 3 · Self-hosting at native grain: an existence proof, not comparative evidence (flow 21)

Run on a twin of jigc, building **the deferred increment-level workflow itself** (the grain gap the
pilot surfaced): jigc dogfooded its methodology *by building the workflow that would have helped*.

- **`decided-task`** (a `creates-task:true` dev-task variant with `allows-create:[{decisions-log}]`)
  was built **test-first** (compose test red→green, gate green), then **used to author a managed,
  schema-validated `decisions-log`** one commit after it existed. The run also produced a managed
  `completion-record`, and **one non-seeded finalize block** fired (the owner-artifact presence gate
  caught an untracked artifact; fixed, re-finalized).
- **Record: green** — adapter-writes 5 · oob-edits 0 · drift-caught 1 · validate-blocks 1 ·
  halts 2/1 · fix-rounds 2 · audit-findings 1; both **seeds registered once** (instrument validated).
  Raw capture + hash manifest under [`self-hosting/`](self-hosting/).

**What this is worth, honestly:** it proves the route — jigc's managed-doc / validation / drift /
promote machinery *runs* end-to-end and produces correct, committed, human-editable artifacts. It is
**not** comparative evidence. There was **no static-methodology control at the same grain**, the run
was **inline and self-judged** (the same agent built the workflow, ran it, recorded the facts, and
graded green), and `decided-task` was *built to produce the very managed artifact that scores as
differentiator engagement* — so "the differentiator engaged" is true by construction of the run.
The green says *the mechanism works in a friendly setting*, not *the agent was more
correct/effective than it would have been under a native-grain static methodology file*. That
comparison does not exist yet.

## Named bounds (what this verdict does *not* claim)

- **The comparative thesis is undemonstrated.** One head-to-head, and it tied. Everything beyond that
  is hypothesis.
- **The self-hosting run is uncontrolled, circular, n=1, single-judge.** No static control at grain;
  built-ran-judged by one agent; inline (the Write|Edit OOB hook was inactive, so the organic OOB
  denominator is ~0 *by construction* — adherence "held" only in the trivial sense that nothing
  invited a bypass). It is an existence proof, weighted as one.
- **The "domain-pack-bound" hypothesis is untested outside self-hosting.** P2 (lacon existing-docs,
  the one case where doc↔code genuinely adjudicates) and P3 (greenfield) were **not run**. Skipping
  them was a pragmatic call (a foreign project needs an authored pack to reach the differentiators),
  but it **removes the planned evidence most likely to test the reframe** — so the reframe stays a
  hypothesis, and the verdict does not claim those runs "would not add signal."
- **adapter-adherence is "routing worked in these sessions,"** not a validated core bet — the
  denominator was ~1 (foreign) and ~0-by-construction (self-hosting).
- **doc↔code was gated off** in both runs (galey non-Rust; no probe for the self-hosting docs) — the
  differentiator most likely to show a measurable correctness win went **unexercised**.
- **Arm B's home carried `lacon` + `repowise-augment`** A/C lacked — a conservative bias toward the
  control. Noted, not replicated.
- **`case` enum predates self-hosting** — the record uses `pilot` as a stand-in; a `self-hosting`
  enum member is owed.

## What M17 proved, demonstrated, and left open

- **Proved (mechanism + instrument):** the capture apparatus works end-to-end on real sessions (the
  no-exit-code harness contingency handled by jrun); the seeded-failure instrument is sound (both
  seeds registered exactly once); jigc's managed-doc/validation/drift/promote machinery runs
  end-to-end and produced correct artifacts **in one self-hosting run**.
- **Demonstrated once (not generalized):** the differentiators *can be* engaged at native grain; the
  adapter *can* route unprompted. Both are single observations, self-referential or n=1.
- **Did NOT prove (the thesis):** that jigc makes an agent **more correct/effective than a static
  `CLAUDE.md`**. The only controlled test tied; the positive run had no control.
- **Revised understanding:** the founding thesis as a *universal* claim is not supported by M17. The
  live hypothesis going forward is **scope-bound** — value, if any, concentrates where structural
  operations (placement, cross-refs, validation, drift, promotion) are in play, i.e. at native grain
  with a fitting pack. Earning it requires the comparison M17 could not run: jigc-vs-static **at that
  grain, with a control, judged independently**.

## Disposition

M17 **closes the roadmap spine** on honest terms: a working instrument, a green self-hosting
existence proof, a tied foreign comparison, and a relocated-but-unearned thesis. Owed follow-ups
(not part of this verdict): a native-grain jigc-vs-static comparison with an independent judge (the
real thesis test); the `lacon` doc↔code case; propagating the "thesis is scope-bound and
undemonstrated" finding into VISION's thesis framing rather than leaving VISION claiming a proven
win.
