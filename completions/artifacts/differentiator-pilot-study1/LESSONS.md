# Lessons + improvements — differentiator pilot Study 1

What this pilot taught us, the confounds that bound the result (especially the one
that tilted it toward arm C), the honest reframe of *what jigc's value actually is*,
and concrete ways to beat or draw the static-`CLAUDE.md` arm next time.

> **Honesty banner.** This was **one small task** (a public-API class rename), n=1
> per cell, doc↔code only. It cannot speak to jigc's value over the **long horizon
> with many edits and many rules** — which, after this run, looks like exactly where
> the value (if any) lives. Read every conclusion below through that lens.

---

## 1. What the pilot proved (independent of the verdict)

- **The harness is sound + reusable.** Container-per-arm isolation defeats the M17
  global-`CLAUDE.md` leak; OAuth auth works in-container; `--model` is honored;
  the seeded instrument check fires (clean → validates clean; planted dangling
  anchor → `blocking · doc-code.symbol-exists`). We can re-run this any time (see
  RUNBOOK.md).
- **The doc↔code differentiator genuinely engages** on a real TS project via the
  shipped `arch-doc` doctype — a rename dangles the anchor and jigc detects it.
- **Unguided agents do ship doc↔code drift** at the Sonnet tier (plain + GSD both
  drifted both tasks) — the failure mode the differentiator targets is *real*, not
  hypothetical. Opus didn't drift (capability covers it on a small task).

## 2. What it found (the verdict, in one line)

On this task, **jigc ≈ plain ≈ GSD (2/4 clean) < static-methodology (4/4)**. jigc
didn't beat the static file. Three diagnosed mechanisms: **(a) bypass** — the
weaker model never routed through jigc; **(b) prose blind spot** — jigc's anchor
check ignores titles/prose; **(c) friction** — when bypassed, jigc only blocks a
correct out-of-band edit. (Full detail in VERDICT.md.)

## 3. The dilution confound — tested (arm E), did NOT materialize

Arm C's file was a possible best-case: **42 lines, ~100% on the doc↔code rule.**
Real `CLAUDE.md` files carry many competing rules, so we **tested the confound
directly** with **arm E**: the *same* doc-rule embedded **verbatim** in a
realistic **157-line file** (1 of 14 genuine sections — build, style, testing,
commits, etc. — buried mid-file at ~23%).

**Result: arm E = 4/4 clean, identical to arm C** (both models, both tasks). The
model attended to the buried rule and followed it. **Moderate file-size dilution
did not degrade single-task adherence** — which makes C's win *more* robust, not
less, and removes "best-case file" as an excuse for jigc's loss.

What this does and does NOT settle:
- **Settles:** at realistic file size (~150 lines, rule at ~23%), a static rule is
  not diluted into ineffectiveness on a single task. Static instruction is more
  robust than the confound assumed.
- **Does NOT settle (jigc's remaining case):**
  1. **Heavy dilution** — a rule at ~5% of a 400–600-line file. Untested.
  2. **Long-horizon cumulative degradation** — the real structural claim: across
     many edits / a long session, does the static rule fade from attention or get
     overridden while **mechanical enforcement holds** (it fires identically on
     edit #1 or #5,000, with 1 rule or 100)? A single edit cannot probe this.

So jigc's structural case is **not refuted**, but it has **narrowed**: it no longer
rests on file-size dilution (disproven here) — only on heavy dilution and, mainly,
**long-horizon cumulative** effects. That is now the one test that matters.

## 4. The reframe: jigc's value is cumulative + dilution-proof, not single-task

The pilot's most useful output isn't the scoreline — it's the **relocation of the
claim** (echoing how M17 relocated the thesis). The live hypothesis is now:

> jigc beats a static `CLAUDE.md` **over a long horizon with many edits and many
> competing rules**, because enforcement holds its strength as instruction's
> degrades. On a single edit with a single rule, they tie — or static wins on
> ergonomics.

This is testable, and it's the test that decides the productive-go question.

## 5. How to beat or draw C next time

### A. Experiment-design changes (test the *real* claim, fairly)

1. ~~**Realistic `CLAUDE.md` for C.**~~ **DONE (arm E)** — a 157-line file with the
   rule at ~23% scored **4/4, same as C**. Prediction (drift rises) was **wrong** at
   this scale. Next step is *heavier* dilution: a 400–600-line file with the rule at
   ~5%, to find the dilution threshold (if any) where static adherence breaks.
2. **Long edit sequences (the headline test).** Run 15–50 sequential edits in one
   session (renames, signature changes, moves, deletions) and measure *cumulative*
   doc↔code drift. *Prediction:* static drift accumulates as attention fades and
   the rule is forgotten; jigc's per-edit gate holds. This is the regime where the
   value should appear if it exists.
3. **Many-rule competition.** Add competing/again-and-again-violated rules so the
   model must triage; see whether the doc rule survives the crowding (static) vs
   stays enforced (jigc).
4. **Routing realism.** Tasks vary in how obviously they "need" a workflow. Measure
   how often agents route through jigc for a task they read as a trivial edit — the
   bypass rate *is* the core-bet metric.

### B. jigc product changes (so it actually wins)

1. **Close the prose blind spot.** Let `arch-doc` anchor component **titles**, or
   add a managed-doc **prose-mention check** (flag any prose naming a symbol absent
   from code). This directly removes the only Opus drift in the matrix — jigc's own.
2. **Make enforcement actually enforce (the bypass fix) — TESTED, it works.** The
   biggest loss was the agent never using jigc. The git `pre-commit` hook is
   warn-only *and* only fires on commit (which the bypassing agents never did), so
   the real fix is **always-on**: a `Stop` hook running `jigc validate` that blocks
   the agent from finishing while a doc-anchor dangles. Implemented as **arm A′
   (jigc-enforced)** and re-run: jigc went **2/4 → 4/4, parity with static** — it
   recovered both original drifts. See [ADDENDUM-jigc-enforced.md](ADDENDUM-jigc-enforced.md).
   Caveat: parity, not superiority; and the README/title breadth rode on the hook's
   *instruction*, since `jigc validate` mechanically catches only the anchor.
3. **Absorb conformant out-of-band edits (the friction fix).** task2-jigc-Sonnet was
   a *correct* rename that jigc would still block (`file-state.hash-matches`). The
   reconciliation design says *absorb* conformant OOB edits — align store/finalize
   behavior so a correct direct edit isn't pure friction. Otherwise jigc penalizes
   agents that bypass it even when they're right.
4. **Lower the activation threshold.** jigc's value is gated behind the agent
   *choosing* the workflow. A lighter-weight path that engages validation on any
   edit (not only via `finalize`) would shrink the bypass gap.

### C. The bar for a real win

A future run **draws** C if jigc matches 4/4 on the realistic-file + long-sequence
regime; it **beats** C if, as the file grows and edits accumulate, C's drift climbs
while jigc's stays at zero — i.e. the curves cross. That crossing, if it exists, is
the whole product thesis. If it *doesn't* exist even over a long horizon, the honest
conclusion is that a well-maintained static file is enough and jigc isn't worth its
lock-in — which is exactly what the value gate is for.

## 6. Honest bounds (unchanged, restated)

n=1 per cell; one small task; doc↔code only; cross-model judge (Codex) was
quota-blocked (same-family blind judge corroborated the objective grep); arm C's
file was unrealistically focused; the long-horizon / many-rule regime — the one that
matters — was **not** tested here. This pilot bought a real, reversible signal and,
more usefully, told us the next test to run.
