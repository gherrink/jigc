# Long-horizon many-edit study — verdict

**Written 2026-06-23**, after the full pre-registered 7-cell matrix (5 arms × Sonnet +
2 arms × Opus, 8 sequential edits each, 19 sequences / 152 cold-agent runs) ran through
isolated evolving-twin containers, measured mechanically (the real `doc-code` probe +
dead-symbol grep on each committed HEAD) and independently confirmed by a **blind
cross-model judge (Codex)** — the judge the three prior studies and the pilot never got
(quota-blocked). This is the ruling on the question jigc has never answered: **does it
make a coding agent measurably more correct over many edits than a static `CLAUDE.md`?**

## One-paragraph verdict

**Qualified yes — the founding empirical claim is demonstrated for the first time, but
only for the weaker model, and the same study refutes it for the stronger one.** In the
long-horizon regime the prior ties (M17–M19) and the pilot all excluded — 8 sequential
edits, fresh cold agent each, evolving repo — **jigc's blocking enforcement strictly
beats every static `CLAUDE.md` for Sonnet**: it holds doc↔code drift flat at **zero**
(anchors *and* prose, blind-judge 3/3 clean) while plain compounds to 5.0 dangling and
every static arm ships **residual** drift the instruction structurally cannot catch.
But for **Opus the result inverts**: jigc's hook guards only anchors, Opus surgically
clears the gate and leaves stale prose, so jigc ends **dirtier than a plain instruction**
(judge 0/2 vs static 2/2) at **~3× the cost**. The thesis is no longer untested — it is
**proven where instruction degrades and disproven where the enforced surface is
narrower than the instructed one.**

## The gate (from the pre-registration): PASSED, narrowly and conditionally

> Superiority = arm A ends with strictly fewer live dangling anchors than ≥1 static arm
> at the same model, **attributable to the hook** (a block in the causal chain), with A
> flat while a static arm rises.

**Met for Sonnet, unambiguously.** A-Sonnet final dangling **0** vs C160 **1.0**,
C550 **0.67**, C40 **0.33** — every static arm leaves residual drift; A does not. It is
hook-attributable: the hook **blocked 3/3 sequences** and drove repair (sometimes via
the agent's direct edit, sometimes via jigc) on exactly the edits that dangled. The
curve is flat (A) vs rising (P) / stepped-and-stuck (static). **This is the first
controlled, blind-confirmed jigc-beats-static result in the project's history.**

**Failed for Opus** — and informatively: the gate's "fewer dangling anchors" is met
(both 0), but the *honest* outcome (prose included) makes jigc worse, so claiming the
gate here would be gaming the metric jigc happens to govern. Reported as a loss.

## Why Sonnet won — two mechanisms, both predicted

1. **Salience-independent enforcement.** The hook fires on every `git commit`
   regardless of rules-file size or whether the agent attends to (or even uses) jigc.
   A-Sonnet recovered from blocks on edits where it ran **zero jigc commands** — it
   edited directly, was blocked, fixed, recommitted. Static is salience-*dependent*; the
   dilution ladder shows the cost (clean rule 0.33 residual → 157-line 1.0), though the
   dominant static failure was not dilution but —
2. **The move blind spot of name-based instruction.** Every static arm's residual
   dangling anchor was **the file *move*** (edit 3): the symbol name is unchanged, so
   "search for the old name" finds nothing and the doc *looks* right, but the recorded
   path is dead. **6/9 Sonnet static reps shipped it; jigc 0/3.** jigc resolves the full
   `path#symbol` against the tree — it validates against reality, not memory. This is the
   "integration advantage" the design always claimed, now measured.

## Why Opus lost — the prose blind spot, made decisive

jigc's hook governs **anchors**, not **prose**. Opus, capable enough to satisfy the gate
minimally, fixed each `implemented-by` anchor and committed — leaving the component
**title** (`### CommentBlockParser`) naming the dead class. Stale prose compounded to 3.0
mean / 5 worst (judge 0/2). Static-Opus, following the broad written rule ("update every
documentation reference, **including headings**"), stayed perfectly clean (0/0, 2/2) — at
1/3 the cost. **For a capable model, a well-written instruction covers more of the doc
than jigc's enforced surface does, and costs far less.** This is the M18/pilot prose
blind spot, previously a footnote, shown to be **outcome-determining**.

## What this licenses (and what it does not)

- **Licenses:** "In a long-horizon, many-edit regime with a weaker model, jigc's
  blocking doc↔code enforcement makes the agent measurably and verifiably more correct
  than any static `CLAUDE.md` — including catching move-induced drift that a name-based
  convention cannot see." A real, diagnosed, blind-confirmed win on a neutral baseline.
- **Does NOT license:** "jigc beats static, period." For a capable model on this task it
  loses on total doc honesty and badly on cost. The win is **regime- and model-bound.**
- **The clear, cheap fix the data points to:** close the prose blind spot — let the
  arch-doc anchor its component **titles** (or add a prose-mention check to the hook).
  That single change would plausibly flip Opus from loss to win, since jigc's anchors
  were already perfect; the only leak was the unenforced title. **This is the highest-
  value next increment** the study surfaces, and it is squarely within the existing
  `doc-code` mechanism.
  - **DONE + re-tested (2026-06-23, commit `9d99924`).** Implemented as the
    pack-declared `title-names-symbol` check (a `doc-code` finding the floor/hook
    already catch). Opus re-test: stale prose **3.0 → 0.5**, rep2 clean across all 8
    edits — the fix works. Full result, bounds, and the one now-closed heuristic
    residual: [title-fix-retest.md](title-fix-retest.md).

## Implication for the productive-go decision

The **value gate is now partially cleared** — for the first time there is a controlled,
blind-confirmed regime where jigc strictly wins. But it is **not** the unconditional win
the thesis hoped for: the advantage is concentrated at (weaker model × long horizon ×
the move/path-drift class), and a capable model exposes a real coverage gap. The honest
operational read: **the mechanism's value is proven and located; the prose-coverage gap
is the gating defect between "wins in a regime" and "wins broadly."** Recommendation
unchanged on corpus commitment (M33/M34 still gate it), but the *reason* has shifted from
"value unproven" to **"value proven-in-regime; close the prose gap, then re-test Opus."**

## Honesty bounds

n = 2–3 per cell (structured study, not powered statistics — the per-edit curve and the
ladder are the signal). One task family (rename/move/delete), doc↔code only. The blocking
hook is a 1-line delta from jigc's shipped warn-only default (not out-of-box behavior).
Agents had no `node_modules` (no build/test; uniform across arms). Opus n=2 and the
first Opus pass was truncated by an auth-token expiry, then cleanly re-run (Sonnet
headline unaffected; see results.md → Data integrity). The blind judge is genuinely
cross-model (Codex) and agreed with the objective oracle on every cell except the
move-path case, where the two correctly measure different surfaces (anchors vs names).
