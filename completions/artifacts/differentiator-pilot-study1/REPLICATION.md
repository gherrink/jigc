# Replication — separating mechanism from variance (n=16/arm)

**Written 2026-06-22.** The A′/A″ single-shot runs couldn't tell the gate's effect
from run-to-run noise. This replication fixes that: **4 arms × Sonnet × 2 tasks × 8
reps = 64 independent runs** (fresh `--rm` container each, no cross-run memory).
Sonnet only — Opus was clean everywhere (too capable to drift on this small task).
Drift = old class name in README **or** in the arch-doc title/prose (the anchor-path
false-positive that bit A″ is excluded). Raw capture: `cells-rep/` is not exported
(64 runs); the rates below are reproducible via `RUNBOOK.md` + `run-replication.sh`.

## Result

| arm | drift-rate | agent engaged jigc | gate fired |
|---|---|---|---|
| **plain** (no doc instruction) | **88%** (14/16) | — | — |
| **static** (explicit doc-consistency rule) | **0%** (0/16) | — | — |
| **jigc** (bootstrap `AGENT.md`, bypassable) | **19%** (3/16) | **0/16** | — |
| **jigc-gate** (bootstrap + always-on `Stop` validate) | **0%** (0/16) | **0/16** | **0/16** |

## What replication settled

1. **Unguided agents drift heavily — base rate 88%.** Doc↔code drift is the *norm*
   without help, not an edge case. (The n=1 "plain 2/4" was a lucky-looking sample;
   the true rate is ~88%.)
2. **Any doc-aware instruction largely fixes it** — and a **direct static rule is
   best (0%)**. This is robust now, not a single run.
3. **jigc's measured benefit is its bootstrap-as-instruction, NOT its mechanism.**
   The decisive number: **the agent ran a jigc command in 0 of 32 jigc/jigc-gate
   runs.** It read the `AGENT.md` bootstrap (loaded via `CLAUDE.md`) and edited files
   directly — every time. Yet drift fell 88% → 19%. So the bootstrap *prose* did the
   work; the *tool* never engaged.
4. **The mechanical gate contributed nothing measurable.** Across 16 jic-gate runs
   the gate fired **0 times** (the bootstrap already got agents clean enough that no
   drift survived to stop-time). jigc-gate 0% vs jigc 19% is **within noise**, not a
   proven gate effect. The gate remains *sound but un-triggered* — its value lives in
   the tail (residual failures), which this regime didn't generate.

## The honest reading (it sharpens the negative)

On this task, **jigc collapses to "an instruction,"** exactly the risk flagged
earlier — and a purpose-built static instruction beats jigc's bootstrap (0% vs 19%)
at a fraction of the complexity. jigc's *differentiating* layers — the agent routing
through the tool, and the enforcement gate — **never activated**:

- **0/32 tool engagement.** Sonnet structurally will not route a "rename a class"
  task through jigc, regardless of bootstrap wording (the A″ proactive bootstrap
  didn't move it either). So "enforce, don't instruct" is **moot** here: there was no
  enforcement, only the bootstrap instruction.
- **0/16 gate fires.** Enforcement that never triggers can't differentiate.

**Implication.** For jigc's enforcement to ever matter, routing must be **forced,
not instructed** — the agent has to be unable to bypass it (e.g. managed files
read-only to direct edits, or an unconditional commit gate), *or* the failure rate
must be high enough that the gate's tail-coverage shows — which points, again, at the
**long-horizon many-edit regime** where instruction degrades and the residual drift
climbs into the gate's territory. A single small task generates no such tail.

## Bounds

n=16/arm, Sonnet only, one task family (symbol rename), doc↔code only. 0 errors,
no rate-limiting (concurrency 3). The gate's catch is proven *sound* elsewhere (the
seeded instrument check, the host linchpin, the one A″ fire) — this study shows it
*rarely triggers* in a regime an instruction already covers, not that it's broken.
