# The cost of enforcement — making the block→recovery loop cheap

**Status:** shaped, unscheduled. Surfaced by the long-horizon study
([completions/artifacts/long-horizon-study/results.md](../completions/artifacts/long-horizon-study/results.md)
→ Cost), which measured a real ceremony tax — small on a cheap model, large on a capable
one. Flagged by the user (2026-06-23) as something to take care of, not just note.

## The finding (measured)

Mean USD per 8-edit sequence:

| | plain | static | jigc |
|---|--:|--:|--:|
| **Sonnet** | 2.77 | 3.87–4.68 | 4.52 (≈ static) |
| **Opus** | — | 8.71 | 27.64 pre-title-fix → ≈50.7 post-fix |

So jigc's cost is **acceptable on Sonnet (≈ static) but 3–6× static on Opus**, and the
`title-names-symbol` fix *raised* it (more checks → more block cycles). The win and the
cost have the **same root**: the block→recovery loop. We want to keep the win
(salience-independent enforcement) and shed the cost.

## The diagnosis: cost = discovery-via-block + re-orientation, not the check

Each blocked commit makes the agent: (1) read the rejection, (2) re-orient (figure out
what to do), (3) fix, (4) retry. The check itself is ~free (a static parse). The cost is
steps 1–2, and it explodes on a capable model:

- **Opus thrashed** — before fixing, it *investigated jigc* (it explored
  `validate`/`describe`/`status`/`ingest`/`unmanage` verbs), spending turns understanding
  the tool rather than applying a one-line edit.
- **Every arm-A-Opus edit blocked (8/8)** — the loop ran every edit; nothing was
  front-loaded, so the expensive path was the *only* path.
- **Sonnet recovered in 1–2 turns** and often without touching jigc at all — so its
  overhead stayed near static.

The expensive case is therefore: *capable model + discovers the problem only at the gate +
re-explores the tool every time.* That's what to attack.

## The strategy: decouple the win from the cost

The guarantee should stay hook-driven (salience-independent). The *common case* should
stop paying for it. Shift from **"block → re-orient → recover"** (expensive, every edit)
to **"proactively correct → hook validates silently"** (cheap), with the block as a
rarely-firing backstop on the residual. Two complementary lever families:

### A. Cheaper recovery — make a block a 1-turn mechanical fix

1. **Prescriptive findings, not just diagnostic.** The finding already names the symbol;
   go further — name the *exact edit* (`change \`### UIDoc\` → \`### DocEngine\` at
   docs/…:12`). The less the agent has to infer, the fewer turns. (doc↔code + title checks
   both have the inputs to do this.)
2. **Tell the agent not to investigate.** The block message (and/or the bootstrap) should
   say a doc-code block is a **mechanical** fix — *edit the named file/line to match the
   named symbol; do not inspect jigc internals.* This directly curbs the Opus thrash.
3. **A deterministic one-command fix for the structural cases.** A stale component *title*
   that must name its anchored symbol is a **structural** edit the CLI can own (structure
   to the CLI — within the determinism boundary; not prose, so not the "never auto-author"
   non-goal). A `jigc fix` / `--fix` that rewrites the title to the anchored symbol turns a
   block into one deterministic command, ~0 agent turns. (Scope strictly to deterministic
   structural syncs; never prose.)

### B. Fewer blocks — front-load the fix so the hook rarely fires

4. **A sharper bootstrap nudge.** "When you rename/move/remove a documented symbol, update
   its arch-doc title + anchor in the *same* change." The study showed the bootstrap *alone*
   drove drift 88% → 19%; a sharper, more salient nudge could make proactive doc-updates the
   norm, so the costly block path becomes the exception. **Safe by construction:** this is a
   cost optimization, not a correctness mechanism — if the nudge fails (salience decay), the
   hook still catches the residual. It only reduces *how often* the expensive recovery runs.

### C. Harness/operational levers (outside jigc)

5. **Cheap-model recovery.** The mechanical fix is cheap on Sonnet; a multi-model harness
   could route the *recovery* turn to a cheaper tier. (Not a jigc change.)

## How we'd know it worked (instrumentation)

The eval harness already records turns + cost per edit. Add **block-cycle counting**
(blocks per edit, turns-per-recovery) so each lever is measured, not asserted. Target: get
Opus's $/edit from ~$6 toward static's ~$1 **without** losing the 0-drift outcome — i.e.
drive block-rate down (front-loading) and turns-per-block down (prescriptive findings /
one-command fix), and watch the curve.

## Honest tensions

- Levers in (B) reintroduce *some* instruction-dependence — fine, because correctness still
  rests on the hook; the nudge only shifts the *cost distribution*. Keep the hook as the
  guarantee; never let a nudge become the guarantee.
- Lever (A3) must stay strictly inside the determinism boundary — auto-sync only the
  *structural* title↔symbol relationship the CLI can own deterministically, never prose.
- The cheapest outcome (front-load so nothing ever blocks) makes jigc's *enforcement*
  invisible in the happy path — which is the point: enforcement you only pay for when it
  actually catches something.

## Next step when scheduled

Pick the highest-leverage pair — likely **(2) "don't investigate" block message +
(4) sharper bootstrap nudge** (both cheap, no determinism risk) — implement, and re-run the
Opus arm-A cost measurement (reuse the long-horizon harness) to confirm $/edit drops while
drift stays 0. (A3 `--fix` is the bigger, higher-payoff follow-on.)
