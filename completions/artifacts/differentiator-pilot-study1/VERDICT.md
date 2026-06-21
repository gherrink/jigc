# Differentiator pilot — Study 1 verdict

**Written 2026-06-21**, after the full pre-registered 4-arm × 2-model × 2-task
matrix (16 cells) ran headless through isolated container twins, the doc↔code
outcome was measured mechanically (grep of the final docs) **and** independently
confirmed by a blind judge. This is the honest ruling on the value gate the
[readiness assessment](../../../READINESS-ASSESSMENT.md) named: *does jigc beat a
static `CLAUDE.md` when a differentiator is actually engaged?*

## One-sentence verdict

**No — and the static `CLAUDE.md` arm beat jigc.** With the doc↔code
differentiator genuinely engaged (the M17 gap closed), jigc **tied the unguided
baselines** (plain, GSD) at 2/4 clean and **lost to the static-methodology arm**
(4/4 clean) — the very "static `CLAUDE.md`" the thesis claims to beat. The thesis
is **not supported** by this study; the result extends M17's tie into a loss.

## The result (mechanical grep + blind judge agree exactly — 6 drift cells)

Outcome = did the final state ship a doc↔code drift (any document naming the
renamed, now-absent class)?

| Arm | Clean / 4 | Drift cells | Reading |
|---|---|---|---|
| **C · static-methodology** | **4/4** | — | the doc-rule as a flat `CLAUDE.md` reliably kept docs honest, **both models** |
| D · plain | 2/4 | task1-Sonnet, task2-Sonnet | Sonnet drifts, Opus clean (capability) |
| B · GSD | 2/4 | task1-Sonnet, task2-Sonnet | **GSD never activated** on a bare ticket → behaved exactly as plain |
| **A · jigc** | 2/4 | task1-Sonnet, task1-Opus | tied plain; see the two failure modes below |

A same-context **blind judge** (arm labels hidden) independently reproduced this
table cell-for-cell. *(The pre-registered cross-model judge, Codex, was
**quota-blocked**; recorded as a limitation — but the headline measure is
objective grep, model-independent, so the blind same-family judge is corroboration,
not the basis.)*

## Why jigc did not win — three diagnosed mechanisms

1. **The adapter was bypassed by the weaker model.** On **both** Sonnet tasks the
   jigc-arm agent used **zero jigc verbs** — it edited files directly and (task1)
   shipped drift identical to plain. jigc's gate only bites if the agent routes the
   task through `jigc task finalize`; for a task it reads as "just rename a class,"
   Sonnet didn't. This is the *enforced-not-sandboxed* core-bet risk, made real.
   **Fairness check:** the SessionStart `jigc start` hook is installed and Opus
   *did* engage jigc from the identical image — so jigc was discoverable; Sonnet's
   bypass was a choice, not a harness artifact.
2. **jigc's check has a prose blind spot.** When jigc **was** engaged (Opus task1),
   it fixed the `implemented-by` **anchor** (`jigc validate` → clean) but the
   component **title** `### CommentBlockParser` still named the dead symbol — and
   the agent committed it. The *only* Opus drift in the entire matrix is the jigc
   arm's own doc, on exactly the surface jigc doesn't police (anchors, not prose/titles).
3. **GSD is inert without explicit invocation.** All 8 GSD cells ran with the
   methodology **dormant** — a bare refactor prompt triggered no `gsd-*` skill, so
   arm B is indistinguishable from plain. (Honest about its ergonomics; not a rig.)

**A sharper instance of the bypass cost (task2-jigc-Sonnet):** the agent produced
a *perfect, fully consistent* rename (file + class + anchor + README all updated)
— but because it edited the managed arch-doc **out-of-band**, `jigc validate`
blocks on `file-state.hash-matches` ("on-disk content differs from recorded
state"). So jigc would have **blocked `finalize` on a correct change** and forced
re-authoring through the tool — pure friction, not a caught defect. When the agent
routes around jigc, jigc's only contribution is a block that demands rework of
already-correct output.

## The structural insight (why static beat enforcement here)

The static rule lives in `CLAUDE.md` — **always in context**, shaping *every*
task. jigc's enforcement lives behind a gate the agent must *choose* to walk
through (`finalize`). For a task that doesn't announce a need for jigc, the agent
skips the gate and jigc adds nothing; the always-present instruction does not get
skipped. On this task, **instruction-in-context beat enforcement-on-finalize** —
and even when enforcement ran, it under-covered the doc (prose blind spot).

## Pilot gate

**Not passed in jigc's favor.** The gate required ≥1 non-seeded run where the
differentiator arm (A) ends consistent while a no-differentiator arm ships drift
on the same task+model *because of the differentiator*. The one raw outcome
difference (jigc clean / plain drift on task2-Sonnet) is **not attributable to
jigc** — the jigc agent bypassed the tool and updated docs by its own diligence.
**No cell shows jigc's mechanism producing a better outcome than plain.** The
seeded instrument check passed (clean→validates clean; planted dangling
anchor→`blocking · doc-code.symbol-exists`) — the apparatus is sound; it measured
a real null/negative.

## Ceremony cost

jigc-Opus averaged **$1.63/run** vs plain-Opus $0.62 (~2.6×) — the highest cost in
the matrix, for an equal-or-worse outcome. static added ~15–20% over plain.

## What this does and does not license

- **Licenses:** "on a doc↔code symbol-rename, jigc ≈ plain ≈ GSD < static-CLAUDE.md."
  A real, diagnosed, blind-confirmed result on a neutral baseline.
- **Does NOT license:** "jigc is worthless." jigc's *detection* works (validate +
  the seeded gate); the failures are (a) ergonomic routing of a weaker model and
  (b) a fixable prose blind spot — not a broken mechanism. The other
  differentiators (drift/reconciliation, supersession, cross-ref integrity at
  scale, multi-doc composition) were **not** tested.
- **Bounds:** n=1 per cell (2 tasks × 2 models — a structured pilot, not
  statistics); doc↔code only; the JSON transcript doesn't capture hook output
  (Opus engagement is the evidence the hook reached the agent); the judge is
  same-family (Codex quota-blocked).
- **The dilution confound — tested (arm E), did NOT materialize.** Arm C's file
  was 42 lines, ~100% on the doc rule — a possible best-case. To test it we added
  **arm E**: the *same* doc-rule embedded verbatim in a realistic **157-line
  `CLAUDE.md`** (1 of 14 sections, ~23%, buried mid-file). Result: **arm E = 4/4
  clean, identical to arm C.** Moderate file-size dilution did **not** degrade
  single-task adherence — the model attended to the buried rule and followed it on
  both tiers. This **strengthens** the negative reading: static instruction is
  robust to realistic dilution, so "C beat A" is not merely a best-case-file
  artifact. **What remains untested** is *heavy* dilution (a rule at ~5% of a
  400–600-line file) and — the stronger, more important claim — **long-horizon
  cumulative degradation** over many edits/a long session, which a single edit
  cannot probe. jigc's structural case now rests entirely there. See
  [LESSONS.md](LESSONS.md) §3.

## Implication for the productive-go decision

The **value gate is not cleared by this study.** The signal leans negative, but the
regime was tilted toward static (one small task, a single-rule file) — so this is
**not a clean refutation** either; the honest read is *"unproven, with the burden
now on a long-horizon / many-rule test."* Combined with the readiness assessment
(schemas still moving, no corpus-migration path), the operational recommendation is
unchanged: do **not** commit a productive corpus or self-host yet — but the *reason*
is "value still unproven," not "value disproven." The reversible twin pilot did its
job — it bought a real signal without lock-in, and told us precisely which test to
run next (see [LESSONS.md](LESSONS.md)).

## Follow-ups that would change the picture (not part of this verdict)

- **Close the prose blind spot** — let arch-doc anchor component titles, or add a
  prose-mention check; re-run. (Would jigc then ≥ static?)
- **Strengthen routing** — does a workflow that *forces* finalize (or a harness
  that runs `jigc validate` as a gate) recover the win the bypass cost?
- **Engage a differentiator a static file *cannot* replicate** — supersession,
  cross-doc forward-ref integrity at scale, multi-pack composition. doc↔code was
  the "most likely measurable win" and it lost to instruction; the harder claim is
  whether *any* differentiator beats a well-written static file.
- **Re-run the cross-model judge** when Codex quota resets, for full independence.
