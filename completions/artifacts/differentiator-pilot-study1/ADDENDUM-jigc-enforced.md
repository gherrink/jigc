# Addendum — folding the learnings back into jigc (arm A′)

**Written 2026-06-21** (arm A′), **extended 2026-06-22** (arm A″). Question: *if we
fold Study-1's diagnosed fixes into jigc and re-run the jigc arm, does it recover
the loss?* First read (A′): *yes, to parity with static (4/4).* **Corrected read
after A″ + instrumentation: the apparent recovery is largely run-to-run variance at
n=1 — the catch mechanism is sound but rarely fired, and proactive routing did not
raise engagement.** Read the [follow-on section](#follow-on-arm-a-proactive-bootstrap--and-the-variance-caveat-that-tempers-the-parity-claim)
at the end before trusting the A′ "parity" framing below. The A′ sections are kept
as originally written, with this banner as the correction.

## The intervention (light — no binary rebuild)

Study 1's #1 failure was **the gate is bypassable**: jigc validated only at
`finalize`, which the agents never reached (Sonnet edited + stopped; even Opus
committed via plain git). So jigc's detection — which *works* — never fired.

Arm **A′ (jigc-enforced)** = arm A + an **always-on `Stop` hook**: when the agent
tries to finish, a hook runs `jigc validate`; if a managed doc's code-anchor
dangles it **blocks the stop** and feeds the finding back, instructing the agent to
fix every doc reference (architecture doc + README headings/prose/cross-refs/
examples) before finishing (capped at 4 re-prompts to avoid loops). This is jigc's
**own detection**, only the *activation point* changed — finalize-gated →
always-on. It's a hook + settings change in the twin (`harness/jigc-stop-hook.js`,
`harness/jigc-enforced-settings.json`); **the pinned binary is unchanged.**

## Result (authoritative measure: `jigc validate` for the anchor + README/title/prose grep)

| task | model | A jigc (orig) | A′ jigc-enforced |
|---|---|---|---|
| task1 | sonnet | DRIFT (bypassed; README+title) | **clean** |
| task1 | opus | DRIFT (prose title) | **clean** |
| task2 | sonnet | clean | clean |
| task2 | opus | clean | clean |
| **clean / 4** | **2/4** | **4/4** |

A′ matches arm C (static) at **4/4**. The hook recovered both original drifts:
the Sonnet **bypass** (it now can't finish while the anchor dangles) and the Opus
**prose-title** blind spot.

> **Measurement note:** A′ task2-opus initially *looked* like a regression under
> the raw old-name grep (the anchor read `MarkdownDescriptionParser.ts#MarkdownDocParser`).
> It is **not** drift — the agent renamed the *class* but not the *file*, so the
> file still exists and the symbol resolves; `jigc validate` correctly reports
> clean. The authoritative measure (anchor-resolves + README/title/prose) scores it
> clean. The raw grep over-counts an old word in a *valid* path; the original
> 16-cell verdict is unaffected (every original DRIFT cell had real README or title
> drift, not a path false-positive).

## Honest caveats — this is parity, not superiority, and it's a hybrid

1. **Parity, not a win.** A′ reaches 4/4 = static (C). It removes jigc's
   *self-inflicted* losses; it does **not** beat the static file on this task. To
   *beat* static you still need the regime where static degrades — the
   **long-horizon many-edit** test. This addendum closes the "jigc lost to its own
   bypass" gap; it does not clear the value gate.
2. **The win is a hybrid (mechanical trigger + targeted instruction).** `jigc
   validate` mechanically detects only the **anchor** (the probe's prose blind spot
   is unchanged). A′ caught the README/title drift because the hook's feedback
   *told* the agent to sweep those surfaces — i.e. a mechanical catch that *wakes
   up* a broad instruction at the right moment. That is a legitimate and effective
   design (detection you can't forget, firing exactly when relevant), but the
   prose breadth still rides on instruction, not on the probe. Closing the probe's
   prose blind spot (a heavier binary change) would make the mechanical coverage
   match the instruction's breadth.
3. **Ceremony cost rises.** Re-prompting adds turns/tokens (task1-opus: 57 turns /
   $2.27 vs the original 44 / $1.61). Enforcement isn't free.
4. **Same bounds as the main study** — n=1/cell, one small task, doc↔code only.

## Takeaway

The biggest Study-1 failure (bypass) is **fixable and cheap** — wiring jigc's
validation to fire always-on (Stop hook), the way the warn-only pre-commit backstop
*should* behave, brings jigc to **parity with a static `CLAUDE.md`** on this task.
The remaining gap to *superiority* is the long-horizon regime, and the remaining
purely-mechanical gap is the probe's prose blind spot. Both are now precisely
scoped follow-ups, not open questions. The product implication: **make jigc's
doc↔code enforcement always-on (not finalize-gated), and broaden the probe beyond
anchors** — then re-run the long-horizon study to test for superiority.

---

## Follow-on (arm A″, proactive bootstrap) — and the variance caveat that TEMPERS the parity claim

Per the catch-*and*-route direction, **arm A″** = arm A′ (always-on catch) + a
**sharpened `.jigc/AGENT.md`** that proactively routes the agent to engage jigc up
front for code changes (framed about the *tool*, not the doc-rule — to avoid
conflating with arm C). The catch hook was instrumented to log each block.

**Result — A″ is 4/4 clean, but the intervention did not work as hoped, and it
exposes a variance problem that walks back the strong reading of A′:**

| metric (totals over 4 cells) | A (orig) | A′ (catch) | A″ (proactive+catch) |
|---|---|---|---|
| clean / 4 | 2/4 | 4/4 | 4/4 |
| jigc-verbs (agent engagement) | 3 | 5 | **3** (down) |
| catches that fired (logged) | n/a | unlogged | **1** |
| total cost | $3.82 | $4.45 | $4.33 |

1. **Proactive routing did NOT raise engagement.** Sonnet used **0 jigc-verbs** in
   both A″ cells — it bypassed jigc despite the sharpened instruction; total verbs
   fell (5→3) and Sonnet's turns ballooned (46 vs 17–20). The instruction bought
   ceremony, not routing. (Opus engaged slightly — one `finalize`.)
2. **The catch fired exactly once** across A″ (logged: task2-opus blocked→fixed);
   every other clean cell logged `ALLOW`-only (no drift at stop).
3. **The recovery is confounded by run-to-run variance.** task1-sonnet across the
   three runs: orig **drift**, A′ **clean**, A″ **clean** — *all three with 0
   jigc-verbs*, A″ with *0 catches*. The "recovered" cells were clean because the
   agent did a complete rename **on its own that run**, not because of jigc. The
   orig drift sits within the normal variance of a bypassing agent.

**Corrected reading.** The mechanism is **sound** — it works deterministically when
it fires (the seeded instrument check, the host linchpin test, and A″ task2-opus
all confirm). But across these runs it **rarely fired**, and the apparent
**2/4 → 4/4 "recovery" is not safely attributable to the fix** at n=1 — variance
dominates. A′'s "parity" should be read as *"jigc-with-catch was clean in this one
run set,"* not *"the catch closed the gap."* The proactive bootstrap, specifically,
**failed to increase routing** for the weaker model.

**The real methodological lesson:** n=1 per cell cannot separate mechanism from
noise. The honest next step is **replication** (e.g. 5–10 runs per cell) to measure
*base drift-rate by arm* and the *catch's marginal contribution* — not more
single-shot arms. And to get jigc's value to actually show, the agent has to engage
it: proactive prose instruction did not achieve that here, which pushes the design
toward **harder routing** (e.g. the validation gate wired so it cannot be skipped)
over softer instruction — consistent with jigc's *enforce-don't-instruct* thesis.
