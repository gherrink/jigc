# Results — the maintenance / drift-over-time thesis test (M18)

**Run 2026-06-13**, after the pre-registration + appendix were frozen and the binary re-pinned
(`jigc` sha `00e67f0f…`). Four blind subagents (2 per arm) ran the identical 5-step code-evolution
sequence over byte-identical seed twins, one commit per step. Scored mechanically by the orchestrator
**and** independently by two blind review agents over de-identified bundles. **The headline is a tie
on the structural-drift metric; a notable *observational* signal appears in prose quality.**

## The arms (de-anonymized after scoring)

| Twin | Arm | Judge bundle label |
|---|---|---|
| `mdt-J1` | jigc (has `jigc validate`) | set-delta |
| `mdt-J2` | jigc | set-bravo |
| `mdt-S1` | static (manual diligence) | set-alpha |
| `mdt-S2` | static | set-charlie |

## Primary metric A — final stale-citation count

**0 stale citations in all four arms.** Every `implemented-by` / `cites-code` / `maps-to-test`
citation in every arm's final docs resolves to a real, *defined* symbol in that arm's final code.
Confirmed three ways: orchestrator grep-for-definition over the final trees; the two independent
review agents (each, blind, returned 0 per set); and `jigc validate` reported "no findings" at the
J twins' HEAD.

| Arm | final stale citations | over-editing (control step 5) |
|---|---|---|
| J1 | **0** | none — docs untouched at step 5 |
| J2 | **0** | none |
| S1 | **0** | none |
| S2 | **0** | none |

## Primary metric B — the per-step caught-vs-shipped timeline

**No arm ever shipped a stale citation at any commit.** Reconstructed per-commit (does each
`path#symbol` resolve at that commit?) over all four twins' git history: every commit — including the
four that renamed / deleted / moved / renamed-test a cited symbol — is **CLEAN**. Each arm repaired
the broken citation(s) **in the same commit** as the code change. The file-change pattern is
*identical* across all four arms:

| Step (code change) | code touched | docs fixed in the same commit |
|---|---|---|
| 1 · rename `refill`→`replenish` | `src/bucket.rs` | arch-doc citation → `#replenish` |
| 2 · delete `try_acquire`, inline | `src/bucket.rs` | arch-doc "Acquire path" → `#tick_and_take` |
| 3 · move `Clock` to `src/time.rs` | 5 files | arch-doc **and** adr citations → `src/time.rs#Clock` (both) |
| 4 · rename test | `tests/burst.rs` | spec citation → `#rejects_sustained_burst` |
| 5 · add `capacity()` (control) | `src/bucket.rs` | **none** (correctly — no documented symbol touched) |

The step-3 two-document break (one symbol cited by both the arch-doc and an ADR) was caught in full
by every arm. Zero drift accumulated; zero false flags on the control step.

## The repair *mechanism* (jigc arms) — operable; but `validate` never fired in anger

Both jigc arms re-pointed **committed-doc** code-anchor fields **surgically** (e.g.
`implemented-by: src/bucket.rs#refill` → `#replenish`), committed atomically with the code change,
and `jigc validate` reported clean at every commit. The M17-surfaced "no committed-field-edit verb"
concern did **not** block the repair here — the agents re-pointed the citation through jigc within the
task and finalized, so the repair *mechanism* is operable.

**Important caveat (do not overclaim):** the data does **not** show `jigc validate` *detecting* a stale
citation and *causing* a repair. Every repair was **proactive** — the agent fixed the citation in the
same commit as the code change, the same way the static arms did by hand — and `validate` merely
*confirmed clean* afterward. The detector never caught drift the agent had missed; the backstop never
fired in anger (because, under compression, the agent never let drift through). So this run evidences
that the repair path *works*, not that the *detector* added value.

## The honest signal — the behavioral blind spot, made concrete (the tiebreak)

On the primary metric the arms are perfectly tied. **Both independent review agents, unprompted,
surfaced the same single differentiator** — and it splits cleanly along arm lines:

- The **static** arms (S1, S2) re-pointed the deleted-symbol's "Acquire path" component to
  `tick_and_take` **and kept the description faithful**: *"Advances the bucket one tick, then attempts
  to take n tokens…"* — which matches the code (`tick_and_take` calls `clock.now()` → `replenish()` →
  take).
- Both **jigc** arms (J1, J2) re-pointed to the same valid symbol but **trimmed the prose** to
  *"Attempts to take n tokens…"*, dropping the tick-advance clause the code still performs.
  Structurally valid (the symbol resolves; `jigc validate` is green) — but **behaviorally less
  accurate**.

This is **exactly the bound the pre-registration carried in**: `symbol-exists` certifies that a
citation points at a *real symbol*, never that the *prose still describes what the code does*. The
jigc arms' green `validate` co-existed with a prose description that quietly degraded — the behavioral
blind spot, observed in the wild rather than asserted. It is **not** a jigc win and **not** a clean
static win: n = 2 per arm, prose quality is not what jigc claims to guarantee, and the split may be an
artifact of the jigc authoring flow drawing attention to the mechanical field-fix and away from the
prose slot. It is *suggestive* — a caution that the detector's "green" can mask prose rot — not a
demonstrated effect.

## Verdict — H0: no difference on the over-time structural-drift metric

**The thesis test does not demonstrate that jigc makes a coding agent's documentation more correct
over a maintenance sequence than a static doc set + manual diligence.** Both arms ended with **zero**
stale citations, repaired every break in-step, and avoided over-editing — a flat tie on the metric
M18's detector exists to move.

**Why this is the expected, honest outcome — and what it does and does not say:**

1. **It is H0 under the pre-registered session-compression bound, not a refutation of the over-time
   claim.** This test compressed "over time" into one focused subagent session, with a task list that
   *telegraphed* exactly which symbols change. That is the **most favorable case for the static arm**:
   the code change and the doc that cites it are in the same working memory, so diligence doesn't have
   to fight forgetting. `jigc validate` is **designed as a backstop for when diligence fails** — and a
   capable single session's diligence didn't fail, so the backstop never fired. The real over-time
   effect (drift because code changes and docs are separated by weeks and attention) is suppressed by
   construction. **A tie here cannot refute the over-time claim; a jigc win would have been strong.**
   We got the tie.
2. **It restates the native-grain lesson one grain up.** M17's native-grain test found machine-
   guaranteed integrity *ties* capable diligence at moderate authoring scale. M18 finds the same at
   *maintenance* scale under compression: the static arm's diligence matched the tool's guarantee.
3. **The positive findings are narrow and stand on their own.** What's demonstrated: jigc's store-wide
   structural validation runs and reports clean, and jigc-managed docs were kept structurally current
   through the repair path (the build-half audit + this run's clean-throughout `jigc validate` + the
   surgical committed-doc repair). What is **not** demonstrated: any *comparative* maintenance value at
   this grain/scale, or the *detector* itself adding value (it never caught drift the agent missed).
4. **The behavioral blind spot was observed here, not just theorized.** In this run both jigc arms
   passed `validate` green while their prose drifted from the code — concrete (n = 2) evidence that
   structural-anchor honesty (what jigc guarantees) is a different, narrower thing than documentation
   honesty (what a reader wants). Stated as observed-here, not as a general jigc tendency; any future
   claim must respect that line.

**Where the over-time advantage could still show (owed, not run here):** a test with genuine
cross-session separation (the agent does step 1 in one session, step 4 in another, having forgotten
the docs), or a longer/denser sequence that overruns a single session's attention — the regime where
the static arm's diligence *actually fails* and the backstop earns its keep. This test deliberately
did not stress that, and says so.

## Bounds (carried into the verdict)

- n = 2 per arm; one toy project; subagents ≈ not = human sessions; orchestrator built both environments.
- Session-compression suppresses the over-time effect (the load-bearing bound — above).
- Structural drift only; behavioral drift out of scope (and observed leaking in, § tiebreak).
- Judges are same-model-family review agents (blind, de-identified) — acceptable because the primary
  metric is a mechanical symbol-resolution check; both judges independently returned 0/arm.
- Non-Rust silent-pass not exercised (Rust seed, every citation genuinely checkable).

## Artifacts

- Pre-registration + appendix (frozen): `../maintenance-test/pre-registration.md`, `appendix-sequence.md`.
- Seed twins: `/home/maurice/jigc-dogfood/mdt-seed-{J,S}`; arm twins: `mdt-{J,S}{1,2}`.
- De-identified judge bundles: `/home/maurice/jigc-dogfood/mdt-judge-bundles/` (+ `.deanon-key.json`).
- Two independent judge reports + the orchestrator's per-step timeline are reproducible from the twin
  git histories (the resolver: does each cited `path#symbol` resolve at commit N).
